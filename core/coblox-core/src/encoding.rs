//! Strict byte-string encodings used by the protocol.
//!
//! Every decoder here rejects non-canonical spellings instead of accepting and
//! normalizing them. That is a protocol requirement rather than strictness for
//! its own sake: "Decoders MUST reject non-canonical encodings before signature
//! verification, so a logical object has one signing representation"
//! (`docs/protocol/README.md`).

use crate::error::{Error, Result};

const B64URL: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
const B32_LOWER: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";

/// Encodes bytes as unpadded RFC 4648 base64url.
#[must_use]
pub fn base64url_encode(input: &[u8]) -> String {
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let b0 = u32::from(chunk[0]);
        let b1 = chunk.get(1).copied().map_or(0, u32::from);
        let b2 = chunk.get(2).copied().map_or(0, u32::from);
        let packed = (b0 << 16) | (b1 << 8) | b2;
        let indices = [
            (packed >> 18) & 0x3f,
            (packed >> 12) & 0x3f,
            (packed >> 6) & 0x3f,
            packed & 0x3f,
        ];
        let emit = chunk.len() + 1;
        for &index in indices.iter().take(emit) {
            out.push(char::from(B64URL[index as usize]));
        }
    }
    out
}

/// Decodes unpadded RFC 4648 base64url.
///
/// Padding characters, whitespace, the standard-alphabet `+` and `/`, a
/// length congruent to 1 modulo 4, and non-zero trailing bits are all
/// rejected: each of them is a second spelling of a byte string that already
/// has one.
pub fn base64url_decode(input: &str, context: &'static str) -> Result<Vec<u8>> {
    let bytes = input.as_bytes();
    if bytes.len() % 4 == 1 {
        return Err(Error::Base64Url(context));
    }
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);
    for chunk in bytes.chunks(4) {
        let mut packed: u32 = 0;
        for &byte in chunk {
            let value = decode_b64url_symbol(byte).ok_or(Error::Base64Url(context))?;
            packed = (packed << 6) | u32::from(value);
        }
        match chunk.len() {
            4 => {
                out.push(
                    u8::try_from((packed >> 16) & 0xff).map_err(|_| Error::Base64Url(context))?,
                );
                out.push(
                    u8::try_from((packed >> 8) & 0xff).map_err(|_| Error::Base64Url(context))?,
                );
                out.push(u8::try_from(packed & 0xff).map_err(|_| Error::Base64Url(context))?);
            }
            3 => {
                // 18 bits carry 2 bytes; the low 2 bits must be zero.
                if packed & 0x3 != 0 {
                    return Err(Error::Base64Url(context));
                }
                let packed = packed >> 2;
                out.push(
                    u8::try_from((packed >> 8) & 0xff).map_err(|_| Error::Base64Url(context))?,
                );
                out.push(u8::try_from(packed & 0xff).map_err(|_| Error::Base64Url(context))?);
            }
            2 => {
                // 12 bits carry 1 byte; the low 4 bits must be zero.
                if packed & 0xf != 0 {
                    return Err(Error::Base64Url(context));
                }
                out.push(
                    u8::try_from((packed >> 4) & 0xff).map_err(|_| Error::Base64Url(context))?,
                );
            }
            _ => return Err(Error::Base64Url(context)),
        }
    }
    Ok(out)
}

/// Decodes unpadded base64url and requires an exact decoded length.
pub fn base64url_decode_fixed<const N: usize>(
    input: &str,
    context: &'static str,
) -> Result<[u8; N]> {
    let bytes = base64url_decode(input, context)?;
    <[u8; N]>::try_from(bytes.as_slice()).map_err(|_| Error::Base64Url(context))
}

fn decode_b64url_symbol(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'-' => Some(62),
        b'_' => Some(63),
        _ => None,
    }
}

/// Encodes bytes as lowercase unpadded RFC 4648 base32.
///
/// This is the `node_id` body encoding of
/// `docs/protocol/README.md#identifiers-and-cryptographic-conventions`.
#[must_use]
pub fn base32_lower_encode(input: &[u8]) -> String {
    let mut out = String::with_capacity(input.len().div_ceil(5) * 8);
    let mut buffer: u32 = 0;
    let mut bits: u32 = 0;
    for &byte in input {
        buffer = (buffer << 8) | u32::from(byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            let index = ((buffer >> bits) & 0x1f) as usize;
            out.push(char::from(B32_LOWER[index]));
        }
        buffer &= (1u32 << bits) - 1;
    }
    if bits > 0 {
        let index = ((buffer << (5 - bits)) & 0x1f) as usize;
        out.push(char::from(B32_LOWER[index]));
    }
    out
}

/// Decodes lowercase unpadded RFC 4648 base32.
///
/// Uppercase spellings, padding and non-zero trailing bits are rejected.
pub fn base32_lower_decode(input: &str, context: &'static str) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(input.len() * 5 / 8);
    let mut buffer: u32 = 0;
    let mut bits: u32 = 0;
    for byte in input.bytes() {
        let value = match byte {
            b'a'..=b'z' => byte - b'a',
            b'2'..=b'7' => byte - b'2' + 26,
            _ => return Err(Error::Base32(context)),
        };
        buffer = (buffer << 5) | u32::from(value);
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            let emitted = ((buffer >> bits) & 0xff) as u8;
            out.push(emitted);
        }
        buffer &= (1u32 << bits) - 1;
    }
    if bits > 0 && buffer != 0 {
        return Err(Error::Base32(context));
    }
    Ok(out)
}

/// Renders a `u64` in the shortest unsigned base-10 form the protocol requires.
#[must_use]
pub fn uint_to_string(value: u64) -> String {
    value.to_string()
}

/// Parses a protocol `u64` string, rejecting every non-shortest spelling.
///
/// `"0"` is the only accepted encoding of zero; `"00"`, `"+1"`, `"-0"`,
/// leading zeros and surrounding whitespace are all rejected.
pub fn uint_from_str(text: &str) -> Result<u64> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err(Error::NonCanonicalUint);
    }
    if text.len() > 1 && text.starts_with('0') {
        return Err(Error::NonCanonicalUint);
    }
    text.parse::<u64>().map_err(|_| Error::NonCanonicalUint)
}

/// Renders 32 bytes as 64 lowercase hexadecimal digits.
#[must_use]
pub fn hex_lower(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(char::from(hex_digit(byte >> 4)));
        out.push(char::from(hex_digit(byte & 0x0f)));
    }
    out
}

const fn hex_digit(nibble: u8) -> u8 {
    if nibble < 10 {
        b'0' + nibble
    } else {
        b'a' + (nibble - 10)
    }
}

/// Parses exactly `N` bytes of lowercase hexadecimal. Uppercase is rejected.
pub fn hex_lower_decode<const N: usize>(text: &str) -> Result<[u8; N]> {
    if text.len() != N * 2 {
        return Err(Error::DigestString);
    }
    let mut out = [0u8; N];
    let bytes = text.as_bytes();
    for (index, slot) in out.iter_mut().enumerate() {
        let high = hex_value(bytes[index * 2]).ok_or(Error::DigestString)?;
        let low = hex_value(bytes[index * 2 + 1]).ok_or(Error::DigestString)?;
        *slot = (high << 4) | low;
    }
    Ok(out)
}

const fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64url_round_trips_the_zero_signature() {
        let signature = [0u8; 64];
        let text = base64url_encode(&signature);
        assert_eq!(text.len(), 86);
        assert!(text.bytes().all(|b| b == b'A'));
        assert_eq!(base64url_decode(&text, "test").unwrap(), signature.to_vec());
    }

    #[test]
    fn base64url_rejects_padding_and_non_canonical_tails() {
        assert!(base64url_decode("AA==", "test").is_err());
        // "AB" decodes 1 byte but leaves non-zero trailing bits.
        assert!(base64url_decode("AB", "test").is_err());
        assert!(base64url_decode("AA", "test").is_ok());
        // Standard-alphabet characters are not base64url.
        assert!(base64url_decode("+/+/", "test").is_err());
        assert!(base64url_decode("A", "test").is_err());
    }

    #[test]
    fn uint_parsing_rejects_every_non_shortest_form() {
        assert_eq!(uint_from_str("0").unwrap(), 0);
        assert_eq!(uint_from_str("65536").unwrap(), 65536);
        for bad in ["", "00", "007", "+1", "-1", " 1", "1 ", "0x1"] {
            assert!(uint_from_str(bad).is_err(), "accepted {bad:?}");
        }
    }

    #[test]
    fn base32_rejects_uppercase_and_non_zero_tail_bits() {
        let encoded = base32_lower_encode(&[0xff; 32]);
        assert_eq!(encoded.len(), 52);
        assert!(base32_lower_decode(&encoded, "test").is_ok());
        assert!(base32_lower_decode("AAAA", "test").is_err());
    }

    #[test]
    fn hex_is_lowercase_only() {
        assert_eq!(hex_lower(&[0xde, 0xad]), "dead");
        assert!(hex_lower_decode::<2>("DEAD").is_err());
        assert_eq!(hex_lower_decode::<2>("dead").unwrap(), [0xde, 0xad]);
    }
}
