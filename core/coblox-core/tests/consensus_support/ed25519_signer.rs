//! An Ed25519 signer for the conformance suite, and nothing else.
//!
//! `coblox-core` ships a **verifier** and no signer, deliberately: a library of
//! deterministic rules has no business holding a private key. [SPEC-025] needs
//! one anyway, because a consensus engine that produces a real chain has to be
//! driven by validators that really sign, and a chain signed by a stub would
//! prove nothing about the certificates it carries.
//!
//! # The source, named
//!
//! RFC 8032, *Edwards-Curve Digital Signature Algorithm (`EdDSA`)*, section 5.1.6
//! *"Sign"*, over the base point and order of §5.1. Nothing here is invented:
//!
//! 1. `h = SHA-512(seed)`; `s = clamp(h[0..32])`; `prefix = h[32..64]`;
//! 2. `A = [s]B`, and `A_enc` is its compression;
//! 3. `r = SHA-512(prefix || M) mod L`;
//! 4. `R = [r]B`, and `R_enc` is its compression;
//! 5. `k = SHA-512(R_enc || A_enc || M) mod L`;
//! 6. `S = (r + k * s) mod L`;
//! 7. the signature is `R_enc || S_enc`.
//!
//! # Two oracles, because one implementation is consistent with itself
//!
//! [`rfc8032_vectors_reproduce`] runs the three Ed25519 vectors of RFC 8032
//! §7.1 — the ones published as `TEST 1`, `TEST 2` and `TEST 3` — and requires
//! this code to reproduce the public key and the signature **byte for byte**
//! from the secret key alone. That is the first oracle and it is external.
//!
//! The second is [`coblox_core::verify_consensus_ed25519`], the shipped
//! consensus verifier: it is a separate reading of the same curve, written
//! against ZIP-215 rather than against §5.1.6, and every signature this file
//! produces is verified by it before any test relies on it. A signer checked
//! only against the verifier that will accept it is the defect
//! [[recurring-defects]] family 1 records as "the test compared the
//! implementation with itself through two copies".

#![allow(dead_code)]

use curve25519_dalek::{edwards::EdwardsPoint, scalar::Scalar};
use sha2::{Digest, Sha512};

/// An Ed25519 key pair, held by seed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigningKey {
    scalar: Scalar,
    prefix: [u8; 32],
    public_key: [u8; 32],
}

fn sha512(parts: &[&[u8]]) -> [u8; 64] {
    let mut hasher = Sha512::new();
    for part in parts {
        hasher.update(part);
    }
    hasher.finalize().into()
}

impl SigningKey {
    /// Derives a key pair from a 32-byte seed, per RFC 8032 §5.1.5.
    #[must_use]
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let h = sha512(&[seed]);
        let mut clamped = [0u8; 32];
        clamped.copy_from_slice(&h[..32]);
        // RFC 8032 §5.1.5 step 2: "Prune the buffer".
        clamped[0] &= 0b1111_1000;
        clamped[31] &= 0b0111_1111;
        clamped[31] |= 0b0100_0000;
        let scalar = Scalar::from_bytes_mod_order(clamped);
        let mut prefix = [0u8; 32];
        prefix.copy_from_slice(&h[32..]);
        let public_key = EdwardsPoint::mul_base(&scalar).compress().to_bytes();
        Self {
            scalar,
            prefix,
            public_key,
        }
    }

    /// The 32-byte public key.
    #[must_use]
    pub const fn public_key(&self) -> [u8; 32] {
        self.public_key
    }

    /// Signs `message`, per RFC 8032 §5.1.6.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        let r = Scalar::from_bytes_mod_order_wide(&sha512(&[&self.prefix, message]));
        let r_enc = EdwardsPoint::mul_base(&r).compress().to_bytes();
        let k = Scalar::from_bytes_mod_order_wide(&sha512(&[&r_enc, &self.public_key, message]));
        let s = r + k * self.scalar;
        let mut signature = [0u8; 64];
        signature[..32].copy_from_slice(&r_enc);
        signature[32..].copy_from_slice(&s.to_bytes());
        signature
    }
}

/// One RFC 8032 §7.1 vector: secret key, public key, message, signature.
struct Rfc8032Vector {
    name: &'static str,
    secret_key: &'static str,
    public_key: &'static str,
    message: &'static str,
    signature: &'static str,
}

/// `TEST 1`, `TEST 2` and `TEST 3` of RFC 8032 §7.1, transcribed with their
/// line breaks removed and nothing else changed.
const RFC8032_VECTORS: [Rfc8032Vector; 3] = [
    Rfc8032Vector {
        name: "TEST 1",
        secret_key: "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
        public_key: "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
        message: "",
        signature: "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b",
    },
    Rfc8032Vector {
        name: "TEST 2",
        secret_key: "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb",
        public_key: "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c",
        message: "72",
        signature: "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00",
    },
    Rfc8032Vector {
        name: "TEST 3",
        secret_key: "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7",
        public_key: "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025",
        message: "af82",
        signature: "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a",
    },
];

fn from_hex(text: &str) -> Vec<u8> {
    assert!(text.len().is_multiple_of(2), "hex has an odd length");
    (0..text.len() / 2)
        .map(|index| {
            u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("hex digit pair")
        })
        .collect()
}

fn to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::new(), |mut out, byte| {
        let _ = write!(out, "{byte:02x}");
        out
    })
}

/// The first oracle: this signer reproduces RFC 8032 §7.1, byte for byte.
///
/// Returns a transcript line per vector, so the evidence of `SPEC-025` can show
/// what was compared rather than assert that it was.
///
/// # Panics
///
/// Panics on the first vector that does not reproduce, which is the intended
/// failure mode: every consensus test in this suite is built on this signer, so
/// a signer that is wrong makes the rest of the suite meaningless rather than
/// merely failing.
#[must_use]
pub fn rfc8032_vectors_reproduce() -> Vec<String> {
    let mut transcript = Vec::new();
    for vector in &RFC8032_VECTORS {
        let seed_bytes = from_hex(vector.secret_key);
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&seed_bytes);
        let key = SigningKey::from_seed(&seed);
        let message = from_hex(vector.message);
        let signature = key.sign(&message);
        assert_eq!(
            to_hex(&key.public_key()),
            vector.public_key,
            "RFC 8032 §7.1 {}: public key does not reproduce",
            vector.name
        );
        assert_eq!(
            to_hex(&signature),
            vector.signature,
            "RFC 8032 §7.1 {}: signature does not reproduce",
            vector.name
        );
        transcript.push(format!(
            "RFC 8032 7.1 {}: public key and signature reproduce ({} message byte(s))",
            vector.name,
            message.len()
        ));
    }
    transcript
}
