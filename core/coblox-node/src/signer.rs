//! Ed25519 signer for `coblox-node`.
//!
//! Implements RFC 8032 section 5.1.6 Ed25519 signing for consensus votes,
//! proposals, and wire envelopes.

use core::fmt;

use curve25519_dalek::{edwards::EdwardsPoint, scalar::Scalar};
use sha2::{Digest, Sha512};

use coblox_core::hash::{ChainId, Digest32, Domain};
use coblox_core::registry::{block_prevote_preimage, block_vote_preimage, signing_preimage};

/// An Ed25519 signing key, held by secret scalar and prefix.
///
/// Not `Debug`-derived, not `Copy`, and not `PartialEq`, and each omission
/// answers something [REVIEW-049] RF-009 executed:
///
/// - the derived `Debug` printed the secret scalar and the prefix in clear, and
///   `NodeConfig` derives `Debug` and holds one of these, so any `{:?}` on a
///   configuration — in a log line or a panic message — published the key;
/// - `Copy` scattered silent copies of secret material through memory with
///   nothing to zero;
/// - the derived `PartialEq` compared secret bytes in variable time.
///
/// The scalar and the prefix are zeroed on `Drop`. That closes the lifetime of
/// *this* copy and not every copy the allocator may have made, which is the
/// honest limit of doing it without a dedicated crate.
#[derive(Clone)]
pub struct SigningKey {
    scalar: Scalar,
    prefix: [u8; 32],
    public_key: [u8; 32],
}

// `clippy::missing_fields_in_debug` wants every field printed, and printing the
// two it names is the whole defect this impl exists to close.
#[allow(clippy::missing_fields_in_debug)]
impl fmt::Debug for SigningKey {
    /// Prints the public key and says the rest is withheld.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SigningKey")
            .field("public_key", &hex_lower(&self.public_key))
            .field("secret", &"<redacted>")
            .finish()
    }
}

impl Drop for SigningKey {
    fn drop(&mut self) {
        self.scalar = Scalar::ZERO;
        self.prefix.fill(0);
    }
}

fn hex_lower(bytes: &[u8; 32]) -> String {
    let mut out = String::with_capacity(64);
    for byte in bytes {
        out.push(char::from_digit(u32::from(byte >> 4), 16).unwrap_or('0'));
        out.push(char::from_digit(u32::from(byte & 0x0f), 16).unwrap_or('0'));
    }
    out
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

    /// Signs a prevote preimage for `(height, round, block_id)` under `chain_id`.
    #[must_use]
    pub fn sign_prevote(
        &self,
        chain_id: &ChainId,
        height: u64,
        round: u64,
        block_id: &Digest32,
    ) -> [u8; 64] {
        let preimage = block_prevote_preimage(chain_id, height, round, block_id);
        self.sign(preimage.as_bytes())
    }

    /// Signs a precommit preimage for `(height, round, block_id)` under `chain_id`.
    #[must_use]
    pub fn sign_precommit(
        &self,
        chain_id: &ChainId,
        height: u64,
        round: u64,
        block_id: &Digest32,
    ) -> [u8; 64] {
        let preimage = block_vote_preimage(chain_id, height, round, block_id);
        self.sign(preimage.as_bytes())
    }

    /// Signs a wire envelope payload under `chain_id`.
    #[must_use]
    pub fn sign_envelope(&self, chain_id: &ChainId, unsigned_envelope_jcs: &[u8]) -> [u8; 64] {
        let preimage = signing_preimage(Domain::SIG_WIRE_ENVELOPE, chain_id, unsigned_envelope_jcs);
        self.sign(preimage.as_bytes())
    }
}
