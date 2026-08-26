//! Consensus-critical Ed25519 verification.
//!
//! # Specification rule
//!
//! `docs/protocol/README.md#consensus-critical-ed25519-verification` requires
//! one identical ZIP-215-derived rule across all Coblox implementations. Given
//! 32-byte encodings `A_enc` and `R_enc`, scalar bytes `S_enc`, message `M`,
//! base point `B`, subgroup order `L`, and `k = SHA-512(R_enc || A_enc || M) mod L`:
//!
//! 1. decode `A_enc` and `R_enc` as points `A` and `R` on the complete Ed25519
//!    twisted Edwards curve; non-canonical y-coordinate encodings are accepted
//!    and reduced modulo `2^255-19` as required by ZIP-215;
//! 2. interpret `S_enc` as little-endian and require `0 <= S < L`;
//! 3. require `[8]A != identity` so an identity/validator key has no small order;
//! 4. accept if and only if `[8][S]B = [8]R + [8][k]A`.
//!
//! The cofactorless equation `[S]B = R + [k]A` MUST NOT be used. Implementations
//! MUST NOT substitute `ed25519-dalek::verify_strict`, legacy-compatibility modes,
//! or a library default whose edge-case acceptance has not been shown equivalent
//! to these four rules. The hash for `k` uses the original encodings, not
//! re-encoded points.
//!
//! # Primitive library choice
//!
//! Rather than hand-rolling curve arithmetic (where field arithmetic mistakes
//! cause invisible and catastrophic security failures), this implementation
//! composes on the primitive crate [`curve25519-dalek`] (see `Cargo.toml` for
//! the version-level audit provenance note):
//! - [`CompressedEdwardsY::decompress`] performs complete twisted Edwards curve
//!   decompression with non-canonical y-coordinate reduction modulo `2^255-19`
//!   (satisfying rule 1);
//! - [`Scalar::from_canonical_bytes`] enforces `0 <= S < L` (satisfying rule 2);
//! - [`EdwardsPoint::is_small_order`] enforces `[8]A != identity` (satisfying rule 3);
//! - [`Sha512`] hashes the raw input byte slices `R_enc || A_enc || M` before
//!   wide reduction `mod L` (satisfying the original encodings requirement);
//! - [`EdwardsPoint::vartime_double_scalar_mul_basepoint`] and
//!   [`EdwardsPoint::mul_by_cofactor`] implement `[8][S]B = [8]R + [8][k]A`
//!   via `[8](R - ([S]B - [k]A)) == 0` (satisfying rule 4).

use curve25519_dalek::{
    digest::Digest,
    edwards::{CompressedEdwardsY, EdwardsPoint},
    scalar::Scalar,
    traits::IsIdentity,
};
use sha2::Sha512;

use crate::SignatureVerifier;
use crate::hash::{ChainId, Domain};
use crate::registry::SigningPreimage;

/// Verifies `signature` **and** that `preimage` was built for `domain` and
/// `chain_id`.
///
/// This is the entry point a consensus caller should use.
/// [`SignatureVerifier::verify`] answers "were these bytes signed by this key",
/// which is a different question from "were these the right bytes to sign": a
/// preimage built with the wrong domain, or with another chain's `chain_id`, is
/// a well-typed value that `verify` accepts, and domain separation exists
/// precisely so that a signature valid in one context is not valid in another
/// ([DEBT-021]).
///
/// It is a free function rather than a method on [`SignatureVerifier`] on
/// purpose: a defaulted trait method can be overridden, and an implementor who
/// overrode this one would remove the check while keeping its name. Nothing an
/// implementor writes can weaken this function.
///
/// The context check runs **before** the signature check, so a wrong-context
/// preimage costs no curve arithmetic. The verification logic itself is
/// untouched — this function calls it and does not reimplement it.
///
/// # Nothing makes a caller come through here, and that is not yet closed
///
/// Using this function is a **convention**, not a boundary. Two public paths
/// reach signature verification without any context check:
/// [`SignatureVerifier::verify`], which this function itself calls, and
/// [`verify_consensus_ed25519`], re-exported at the crate root. Neither is
/// behind a feature gate or a versioned lint.
///
/// That is the shape [REVIEW-022] found in `pub(crate)`: a guarantee held by a
/// name. It is sharper here because **the sibling escape hatch in this crate has
/// two fences and this one has none** — the raw-bytes constructor on
/// [`SigningPreimage`], the one this file must not name, is behind the
/// non-default `conformance-testing` feature *and* behind a versioned lint in
/// `sim/tools/`. The two are twins: one lets bytes in without a context, the
/// other lets a verification out without one.
///
/// *That the sentence above cannot name it is itself the demonstration.* The
/// first draft did name it, and the lint failed the build — a guard doing
/// exactly what it was written to do. Loosening it so that a doc comment could
/// spell the name would have traded a working fence for a nicer paragraph.
///
/// It is named and not closed, deliberately. `light_client` documents that this
/// crate ships no verifier of its own, so there is no consensus caller to fence
/// today, and the right fence depends on how the first one is built — which is
/// another spec's work ([REVIEW-029] RF-001, tracked as [DEBT-029]). A convention its own file does not state is not a convention, so it is
/// stated here.
#[must_use]
pub fn verify_in_context<V: SignatureVerifier + ?Sized>(
    verifier: &V,
    domain: Domain,
    chain_id: &ChainId,
    public_key: &[u8; 32],
    preimage: &SigningPreimage,
    signature: &[u8; 64],
) -> bool {
    preimage.binds(domain, chain_id) && verifier.verify(public_key, preimage, signature)
}

/// The canonical consensus-critical Ed25519 signature verifier.
///
/// Implements the four ZIP-215-derived rules and the small-order public-key
/// rejection defined in `docs/protocol/README.md#consensus-critical-ed25519-verification`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConsensusVerifier;

impl SignatureVerifier for ConsensusVerifier {
    /// Verifies `signature` over `preimage` under `public_key`.
    ///
    /// `preimage` is the chain-bound preimage produced by
    /// [`crate::registry::signing_preimage`], not a digest of it.
    fn verify(
        &self,
        public_key: &[u8; 32],
        preimage: &SigningPreimage,
        signature: &[u8; 64],
    ) -> bool {
        verify_consensus_ed25519(public_key, preimage, signature)
    }
}

/// Verifies an Ed25519 signature under the consensus-critical Coblox v0 rule.
///
/// Returns `true` if and only if all four protocol rules and the small-order
/// public-key rejection hold.
#[must_use]
pub fn verify_consensus_ed25519(
    public_key: &[u8; 32],
    preimage: &SigningPreimage,
    signature: &[u8; 64],
) -> bool {
    // 1. Decode A_enc as point A on the complete Ed25519 twisted Edwards curve;
    // non-canonical y-coordinate encodings are accepted and reduced mod 2^255-19.
    let Some(a_point) = CompressedEdwardsY(*public_key).decompress() else {
        return false;
    };

    // 3. Require [8]A != identity so an identity/validator key has no small order.
    if a_point.is_small_order() {
        return false;
    }

    let mut r_bytes = [0u8; 32];
    r_bytes.copy_from_slice(&signature[..32]);

    let mut s_bytes = [0u8; 32];
    s_bytes.copy_from_slice(&signature[32..]);

    // 2. Interpret S_enc as little-endian and require 0 <= S < L.
    let Some(s_scalar) = Option::<Scalar>::from(Scalar::from_canonical_bytes(s_bytes)) else {
        return false;
    };

    // 1 (cont). Decode R_enc as point R on the complete curve.
    let Some(r_point) = CompressedEdwardsY(r_bytes).decompress() else {
        return false;
    };

    // The hash for k uses the original encodings, not re-encoded points:
    // k = SHA-512(R_enc || A_enc || M) mod L
    let mut hasher = Sha512::new();
    hasher.update(r_bytes);
    hasher.update(public_key);
    hasher.update(preimage.as_bytes());
    let k_output: [u8; 64] = hasher.finalize().into();
    let k = Scalar::from_bytes_mod_order_wide(&k_output);

    // 4. Accept if and only if [8][S]B = [8]R + [8][k]A.
    //
    // Let R' = [S]B - [k]A = [S]B + [k](-A).
    // Then:
    //   [8][S]B = [8]R + [8][k]A
    //   <=> [8]R = [8][S]B - [8][k]A
    //   <=> [8](R - ([S]B - [k]A)) = identity
    //   <=> [8](R - R') = identity.
    let r_prime = EdwardsPoint::vartime_double_scalar_mul_basepoint(&k, &-a_point, &s_scalar);
    (r_point - r_prime).mul_by_cofactor().is_identity()
}
