//! The canonical JSON subset the protocol signs and hashes.
//!
//! `docs/protocol/README.md#common-representation` defines Coblox objects as
//! UTF-8 JSON in the I-JSON subset (RFC 7493), canonicalized with the JSON
//! Canonicalization Scheme (JCS, RFC 8785), plus seven additional restrictions.
//! This module makes the canonical form **the only form a value can take**:
//!
//! * [`Json`] has no number and no null variant. Restrictions 2, 4 and 5 —
//!   integers as shortest-form `u64` strings, timestamps as `u64` strings, and
//!   the ban on floating point and `null` — are therefore not runtime checks
//!   but statements about which programs compile. Because no JSON number can
//!   ever be produced, the ES6/`ryu` number-formatting half of RFC 8785 is
//!   unreachable and is deliberately not implemented; a number encountered
//!   while parsing is rejected rather than converted.
//! * [`JsonObject`] keys are validated lower `snake_case` ASCII (restriction 1)
//!   and live in a `BTreeMap`, so duplicates are impossible and ordering is not
//!   a caller's choice. For ASCII keys, byte order and the UTF-16 code-unit
//!   order RFC 8785 specifies coincide, which is why sorting the map by bytes
//!   is exact rather than approximate here.
//! * [`ObjectBuilder`] is the ergonomic construction path and each of its
//!   typed setters emits the canonical spelling of its field: `uint` the
//!   shortest decimal, `digest` the `sha256:` + 64 lowercase hex form, `bytes`
//!   unpadded base64url.
//! * [`JsonObject::parse_canonical`] parses and then requires the input bytes
//!   to equal the serialization of what was parsed. Non-canonical bytes are
//!   rejected; they are never silently normalized.
//!
//! The bytes that are hashed or signed are always [`JsonObject::to_jcs`],
//! without a byte order mark and without a trailing newline.

use std::collections::BTreeMap;

use crate::encoding::{base64url_encode, uint_to_string};
use crate::error::{Error, JsonError, Result};
use crate::hash::Digest32;

/// A value of the Coblox JSON subset.
///
/// There is deliberately no `Number` and no `Null` variant. See the module
/// documentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Json {
    /// A JSON boolean.
    Bool(bool),
    /// A JSON string, holding Unicode scalar values.
    Str(String),
    /// A JSON array. Order is significant and is preserved.
    Array(Vec<Json>),
    /// A JSON object with validated keys in canonical order.
    Object(JsonObject),
}

impl Json {
    /// A string value.
    #[must_use]
    pub fn str(value: impl Into<String>) -> Self {
        Self::Str(value.into())
    }

    /// A protocol `u64`, rendered as its shortest unsigned base-10 string.
    #[must_use]
    pub fn uint(value: u64) -> Self {
        Self::Str(uint_to_string(value))
    }

    /// A hash, rendered as `sha256:` plus 64 lowercase hexadecimal digits.
    #[must_use]
    pub fn digest(value: &Digest32) -> Self {
        Self::Str(value.to_prefixed())
    }

    /// A byte string, rendered as unpadded RFC 4648 base64url.
    #[must_use]
    pub fn bytes(value: &[u8]) -> Self {
        Self::Str(base64url_encode(value))
    }

    fn write_jcs(&self, out: &mut String) {
        match self {
            Self::Bool(true) => out.push_str("true"),
            Self::Bool(false) => out.push_str("false"),
            Self::Str(text) => write_json_string(text, out),
            Self::Array(items) => {
                out.push('[');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(',');
                    }
                    item.write_jcs(out);
                }
                out.push(']');
            }
            Self::Object(object) => object.write_jcs(out),
        }
    }
}

/// A JSON object whose keys are validated and canonically ordered.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct JsonObject {
    fields: BTreeMap<String, Json>,
}

impl JsonObject {
    /// An empty object.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts a builder. This is the intended construction path.
    #[must_use]
    pub fn builder() -> ObjectBuilder {
        ObjectBuilder::new()
    }

    /// Inserts a field, rejecting an invalid key or a duplicate.
    pub fn insert(&mut self, key: &str, value: Json) -> Result<()> {
        validate_key(key)?;
        if self.fields.contains_key(key) {
            return Err(JsonError::DuplicateKey(key.to_owned()).into());
        }
        self.fields.insert(key.to_owned(), value);
        Ok(())
    }

    /// Borrows a field.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Json> {
        self.fields.get(key)
    }

    /// Returns `true` when the object has no fields.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Number of fields.
    #[must_use]
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Iterates fields in canonical key order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Json)> {
        self.fields.iter()
    }

    /// Reads a string field.
    pub fn string(&self, key: &str) -> Result<&str> {
        match self.fields.get(key) {
            Some(Json::Str(text)) => Ok(text),
            _ => Err(JsonError::Field(key.to_owned()).into()),
        }
    }

    /// Reads a `u64` field, enforcing the shortest-form spelling.
    pub fn uint(&self, key: &str) -> Result<u64> {
        crate::encoding::uint_from_str(self.string(key)?)
    }

    /// Reads a boolean field.
    pub fn boolean(&self, key: &str) -> Result<bool> {
        match self.fields.get(key) {
            Some(Json::Bool(value)) => Ok(*value),
            _ => Err(JsonError::Field(key.to_owned()).into()),
        }
    }

    /// Reads a `sha256:`-prefixed hash field.
    pub fn digest(&self, key: &str) -> Result<Digest32> {
        Digest32::parse_prefixed(self.string(key)?)
    }

    /// Reads a nested object field.
    pub fn object(&self, key: &str) -> Result<&Self> {
        match self.fields.get(key) {
            Some(Json::Object(object)) => Ok(object),
            _ => Err(JsonError::Field(key.to_owned()).into()),
        }
    }

    /// Reads an array field.
    pub fn array(&self, key: &str) -> Result<&[Json]> {
        match self.fields.get(key) {
            Some(Json::Array(items)) => Ok(items),
            _ => Err(JsonError::Field(key.to_owned()).into()),
        }
    }

    /// Rejects any field outside `allowed`.
    ///
    /// Restriction 6 of the common representation: "Unknown fields are rejected
    /// unless they occur inside an explicitly defined `extensions` object. v0
    /// defines no consensus-relevant extensions."
    pub fn reject_unknown_fields(&self, allowed: &[&str]) -> Result<()> {
        for key in self.fields.keys() {
            if !allowed.contains(&key.as_str()) {
                return Err(JsonError::Field(key.clone()).into());
            }
        }
        Ok(())
    }

    /// The JCS bytes of this object: the exact bytes that are hashed or signed.
    #[must_use]
    pub fn to_jcs(&self) -> Vec<u8> {
        let mut out = String::new();
        self.write_jcs(&mut out);
        out.into_bytes()
    }

    /// The JCS serialization as text, for diagnostics and documentation.
    #[must_use]
    pub fn to_jcs_string(&self) -> String {
        let mut out = String::new();
        self.write_jcs(&mut out);
        out
    }

    fn write_jcs(&self, out: &mut String) {
        out.push('{');
        for (index, (key, value)) in self.fields.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            write_json_string(key, out);
            out.push(':');
            value.write_jcs(out);
        }
        out.push('}');
    }

    /// Parses canonical JCS bytes into an object.
    ///
    /// The input must already be canonical. After parsing, the value is
    /// re-serialized and the bytes are compared: any difference — key order,
    /// insignificant whitespace, a longer escape spelling for a character that
    /// has a shorter one, a byte order mark, a trailing newline — is reported
    /// as [`JsonError::NonCanonical`] rather than accepted and normalized.
    pub fn parse_canonical(input: &[u8]) -> Result<Self> {
        let text = core::str::from_utf8(input).map_err(|_| Error::Json(JsonError::NotUtf8))?;
        let mut parser = Parser {
            bytes: text.as_bytes(),
            position: 0,
        };
        let value = parser.parse_value()?;
        if parser.position != parser.bytes.len() {
            return Err(JsonError::TrailingBytes.into());
        }
        let Json::Object(object) = value else {
            return Err(JsonError::NotAnObject.into());
        };
        if object.to_jcs() != input {
            return Err(JsonError::NonCanonical.into());
        }
        Ok(object)
    }
}

/// Chaining builder for [`JsonObject`].
///
/// Errors are accumulated and surfaced by [`ObjectBuilder::build`], so a
/// fixture or a message construction reads as one expression while remaining
/// fallible.
#[derive(Debug)]
pub struct ObjectBuilder {
    state: Result<JsonObject>,
}

impl Default for ObjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectBuilder {
    /// A builder over an empty object.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Ok(JsonObject::new()),
        }
    }

    fn with(mut self, key: &str, value: Json) -> Self {
        if let Ok(object) = self.state.as_mut()
            && let Err(error) = object.insert(key, value)
        {
            self.state = Err(error);
        }
        self
    }

    /// Adds a string field.
    #[must_use]
    pub fn str(self, key: &str, value: &str) -> Self {
        self.with(key, Json::str(value))
    }

    /// Adds a `u64` field in its shortest decimal spelling.
    #[must_use]
    pub fn uint(self, key: &str, value: u64) -> Self {
        self.with(key, Json::uint(value))
    }

    /// Adds a boolean field.
    #[must_use]
    pub fn boolean(self, key: &str, value: bool) -> Self {
        self.with(key, Json::Bool(value))
    }

    /// Adds a `sha256:`-prefixed hash field.
    #[must_use]
    pub fn digest(self, key: &str, value: &Digest32) -> Self {
        self.with(key, Json::digest(value))
    }

    /// Adds an unpadded base64url byte-string field.
    #[must_use]
    pub fn bytes(self, key: &str, value: &[u8]) -> Self {
        self.with(key, Json::bytes(value))
    }

    /// Adds a nested object field.
    #[must_use]
    pub fn object(self, key: &str, value: JsonObject) -> Self {
        self.with(key, Json::Object(value))
    }

    /// Adds an array field.
    #[must_use]
    pub fn array(self, key: &str, items: Vec<Json>) -> Self {
        self.with(key, Json::Array(items))
    }

    /// Adds an arbitrary value.
    #[must_use]
    pub fn value(self, key: &str, value: Json) -> Self {
        self.with(key, value)
    }

    /// Finishes the object, or reports the first rejected field.
    pub fn build(self) -> Result<JsonObject> {
        self.state
    }
}

/// Validates restriction 1: object keys are lower `snake_case` ASCII.
fn validate_key(key: &str) -> Result<()> {
    let bytes = key.as_bytes();
    let valid = !bytes.is_empty()
        && bytes[0].is_ascii_lowercase()
        && bytes
            .iter()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'_')
        && !key.ends_with('_')
        && !key.contains("__");
    if valid {
        Ok(())
    } else {
        Err(JsonError::InvalidKey(key.to_owned()).into())
    }
}

/// RFC 8785 section 3.2.2.2 string serialization.
fn write_json_string(text: &str, out: &mut String) {
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{0008}' => out.push_str("\\b"),
            '\u{0009}' => out.push_str("\\t"),
            '\u{000a}' => out.push_str("\\n"),
            '\u{000c}' => out.push_str("\\f"),
            '\u{000d}' => out.push_str("\\r"),
            c if u32::from(c) < 0x20 => {
                out.push_str("\\u00");
                let value = u32::from(c);
                out.push(char::from(hex_nibble(((value >> 4) & 0x0f) as u8)));
                out.push(char::from(hex_nibble((value & 0x0f) as u8)));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

const fn hex_nibble(nibble: u8) -> u8 {
    if nibble < 10 {
        b'0' + nibble
    } else {
        b'a' + (nibble - 10)
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Result<u8> {
        self.bytes
            .get(self.position)
            .copied()
            .ok_or_else(|| JsonError::UnexpectedEnd.into())
    }

    fn bump(&mut self) -> Result<u8> {
        let byte = self.peek()?;
        self.position += 1;
        Ok(byte)
    }

    fn expect(&mut self, expected: u8) -> Result<()> {
        let byte = self.bump()?;
        if byte == expected {
            Ok(())
        } else {
            Err(JsonError::UnexpectedByte(byte).into())
        }
    }

    /// Skips insignificant whitespace.
    ///
    /// The canonical form has none, so a document containing any is still
    /// rejected — by the byte comparison in
    /// [`JsonObject::parse_canonical`], with [`JsonError::NonCanonical`],
    /// which is a more useful diagnosis than a parse error at an arbitrary
    /// offset.
    fn skip_whitespace(&mut self) {
        while let Some(byte) = self.bytes.get(self.position) {
            if matches!(byte, b' ' | b'\t' | b'\n' | b'\r') {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn parse_value(&mut self) -> Result<Json> {
        self.skip_whitespace();
        match self.peek()? {
            b'{' => self.parse_object().map(Json::Object),
            b'[' => self.parse_array(),
            b'"' => self.parse_string().map(Json::Str),
            b't' => {
                self.parse_literal(b"true")?;
                Ok(Json::Bool(true))
            }
            b'f' => {
                self.parse_literal(b"false")?;
                Ok(Json::Bool(false))
            }
            b'n' => Err(JsonError::NullForbidden.into()),
            b'-' | b'0'..=b'9' => Err(JsonError::NumberForbidden.into()),
            other => Err(JsonError::UnexpectedByte(other).into()),
        }
    }

    fn parse_literal(&mut self, literal: &[u8]) -> Result<()> {
        for &expected in literal {
            self.expect(expected)?;
        }
        Ok(())
    }

    fn parse_array(&mut self) -> Result<Json> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_whitespace();
        if self.peek()? == b']' {
            self.position += 1;
            return Ok(Json::Array(items));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_whitespace();
            match self.bump()? {
                b',' => {}
                b']' => break,
                other => return Err(JsonError::UnexpectedByte(other).into()),
            }
        }
        Ok(Json::Array(items))
    }

    fn parse_object(&mut self) -> Result<JsonObject> {
        self.expect(b'{')?;
        let mut object = JsonObject::new();
        self.skip_whitespace();
        if self.peek()? == b'}' {
            self.position += 1;
            return Ok(object);
        }
        loop {
            self.skip_whitespace();
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect(b':')?;
            let value = self.parse_value()?;
            object.insert(&key, value)?;
            self.skip_whitespace();
            match self.bump()? {
                b',' => {}
                b'}' => break,
                other => return Err(JsonError::UnexpectedByte(other).into()),
            }
        }
        Ok(object)
    }

    fn parse_string(&mut self) -> Result<String> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            let byte = self.bump()?;
            match byte {
                b'"' => break,
                b'\\' => {
                    let escape = self.bump()?;
                    match escape {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{0008}'),
                        b'f' => out.push('\u{000c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => out.push(self.parse_unicode_escape()?),
                        _ => return Err(JsonError::InvalidEscape.into()),
                    }
                }
                b if b < 0x20 => return Err(JsonError::ControlCharacter(b).into()),
                b if b < 0x80 => out.push(char::from(b)),
                _ => {
                    // Multi-byte UTF-8: the whole input was validated as UTF-8
                    // already, so copy the scalar value across verbatim.
                    let start = self.position - 1;
                    let mut end = self.position;
                    while end < self.bytes.len() && (self.bytes[end] & 0xc0) == 0x80 {
                        end += 1;
                    }
                    let slice = core::str::from_utf8(&self.bytes[start..end])
                        .map_err(|_| Error::Json(JsonError::NotUtf8))?;
                    out.push_str(slice);
                    self.position = end;
                }
            }
        }
        Ok(out)
    }

    fn parse_unicode_escape(&mut self) -> Result<char> {
        let first = self.parse_hex4()?;
        if (0xd800..0xdc00).contains(&first) {
            self.expect(b'\\')?;
            self.expect(b'u')?;
            let second = self.parse_hex4()?;
            if !(0xdc00..0xe000).contains(&second) {
                return Err(JsonError::InvalidEscape.into());
            }
            let combined =
                0x1_0000 + ((u32::from(first) - 0xd800) << 10) + (u32::from(second) - 0xdc00);
            char::from_u32(combined).ok_or_else(|| JsonError::InvalidEscape.into())
        } else if (0xdc00..0xe000).contains(&first) {
            // Lone low surrogate: not a Unicode scalar value, rejected by I-JSON.
            Err(JsonError::InvalidEscape.into())
        } else {
            char::from_u32(u32::from(first)).ok_or_else(|| JsonError::InvalidEscape.into())
        }
    }

    fn parse_hex4(&mut self) -> Result<u16> {
        let mut value: u16 = 0;
        for _ in 0..4 {
            let byte = self.bump()?;
            let digit = match byte {
                b'0'..=b'9' => u16::from(byte - b'0'),
                b'a'..=b'f' => u16::from(byte - b'a' + 10),
                b'A'..=b'F' => u16::from(byte - b'A' + 10),
                _ => return Err(JsonError::InvalidEscape.into()),
            };
            value = (value << 4) | digit;
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_sorted_and_uints_are_shortest_form() {
        let object = JsonObject::builder()
            .uint("height", 42)
            .str("network_id", "coblox-devnet-0")
            .uint("round", 0)
            .build()
            .unwrap();
        assert_eq!(
            object.to_jcs_string(),
            r#"{"height":"42","network_id":"coblox-devnet-0","round":"0"}"#
        );
    }

    #[test]
    fn duplicate_and_malformed_keys_are_rejected() {
        let mut object = JsonObject::new();
        object.insert("height", Json::uint(1)).unwrap();
        assert!(object.insert("height", Json::uint(2)).is_err());
        assert!(object.insert("Height", Json::uint(2)).is_err());
        assert!(object.insert("height-ms", Json::uint(2)).is_err());
        assert!(object.insert("", Json::uint(2)).is_err());
    }

    #[test]
    fn numbers_and_null_are_unrepresentable_and_unparseable() {
        assert!(JsonObject::parse_canonical(br#"{"height":42}"#).is_err());
        assert!(JsonObject::parse_canonical(br#"{"height":null}"#).is_err());
        assert!(JsonObject::parse_canonical(br#"{"height":1.5}"#).is_err());
    }
}
