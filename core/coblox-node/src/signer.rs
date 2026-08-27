//! Ed25519 signer for `coblox-node`.
//!
//! Implements RFC 8032 section 5.1.6 Ed25519 signing for consensus votes,
//! proposals, and wire envelopes.

use curve25519_dalek::{edwards::EdwardsPoint, scalar::Scalar};
use sha2::{Digest, Sha512};

use coblox_core::hash::{ChainId, Digest32, Domain};
use coblox_core::registry::{block_prevote_preimage, block_vote_preimage, signing_preimage};

/// An Ed25519 signing key, held by secret scalar and prefix.
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
