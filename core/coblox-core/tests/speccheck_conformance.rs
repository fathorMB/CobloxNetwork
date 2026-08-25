//! Conformance tests for consensus-critical Ed25519 signature verification
//! against the `novifinancial/ed25519-speccheck` oracle vectors.
//!
//! # Verification Gate Fulfillment
//!
//! - `GATE-SPECCHECK`: Executes upstream vectors 0–11 **and** the seven Coblox
//!   extension vectors, printing the line-by-line comparison between observed
//!   outcomes and the published ones. The published column is **parsed from
//!   `docs/protocol/README.md` at build time**, never transcribed: see
//!   [`published_row_from_document`] for why that distinction is the whole
//!   content of this gate.
//!
//!   The extension vectors exist because [REVIEW-019] established that the
//!   upstream twelve exercise only half of rule 1: none of their twenty-four
//!   point encodings has a masked `y >= 2^255-19`, so an implementation that
//!   rejects such an encoding — as RFC 8032 §5.1.3 step 2 requires and as most
//!   non-ZIP-215 libraries do — passes all twelve while diverging from Coblox on
//!   signatures any key holder can construct. That divergence is not asserted
//!   here, it is executed, by
//!   [`strict_y_decoding_agrees_on_the_twelve_and_diverges_on_the_extension`].
//! - `GATE-COFACTOR`: Demonstrates that Vector 4 produces distinct outcomes under
//!   cofactored vs. cofactorless verification, and that [`ConsensusVerifier`]
//!   follows the cofactored equation `[8][S]B = [8]R + [8][k]A`.
//! - `GATE-DEPENDENCY`: Verified via `cargo deny check` with `curve25519-dalek` v5.

use coblox_core::SignatureVerifier;
use coblox_core::encoding::{hex_lower, hex_lower_decode};
use coblox_core::hash::{ChainId, Digest32, Domain};
use coblox_core::registry::{SigningPreimage, signing_preimage};
use coblox_core::verifier::{ConsensusVerifier, verify_consensus_ed25519};
use curve25519_dalek::digest::Digest;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::IsIdentity;
use sha2::{Sha256, Sha512};

/// The published protocol document, embedded at build time.
///
/// `include_str!` and not a runtime read: the file becomes a compile-time input
/// of this test binary, so editing the table forces a rebuild and a re-parse.
/// There is no transcription of the table anywhere in this crate.
const PROTOCOL_README: &str = include_str!("../../../docs/protocol/README.md");

/// The fixture provenance document, embedded for the same reason.
///
/// The upstream digest recorded there is the only copy; this suite reads it
/// rather than carrying a constant that could age away from the prose beside it.
const FIXTURE_README: &str = include_str!("fixtures/README.md");

/// The upstream `cases.json`, verbatim, embedded so the provenance check runs
/// offline and on every build rather than once, by hand, against a branch name.
const UPSTREAM_CASES: &str = include_str!("fixtures/ed25519_speccheck_upstream_cases.json");

/// Heading of the section that owns the upstream table.
const TABLE_SECTION: &str = "### Consensus-critical Ed25519 verification";

/// Heading of the subsection that owns the Coblox extension table.
const EXTENSION_TABLE_SECTION: &str = "#### Coblox extension vectors";

/// Leading cell of the row that carries the outcomes, in both tables. The two
/// are told apart by their section, never by their label.
const TABLE_ROW_LABEL: &str = "| Coblox v0 |";

/// Marker of the line in `fixtures/README.md` that records the upstream digest.
const UPSTREAM_DIGEST_MARKER: &str = "**SHA-256 of `ed25519_speccheck_upstream_cases.json`**:";

/// Parse the published outcome table out of [`PROTOCOL_README`].
///
/// # Why this is parsed and not transcribed
///
/// [REVIEW-018] found that a hand-written `PUBLISHED_OUTCOMES` constant here was
/// *labelled* as the document but in fact transcribed what the implementation
/// does, so `GATE-SPECCHECK` compared the implementation with itself through two
/// copies and could not have detected that the document disagreed — which it
/// did, at vector 8. A constant that claims to be a document without being
/// derived from it is the aged copy [ADR-012] cites as its own precedent.
///
/// Consequence of parsing instead: if the document changes and this test does
/// not, the test fails. That is the intended failure mode, not a nuisance.
///
/// # Panics
///
/// Deliberately, and loudly, on every shape the document could take that is not
/// the table this gate is about: a missing section, a missing row, the wrong
/// number of cells, or a cell that is neither `accept` nor `reject`. A parser
/// that silently returned a default on a reshaped document would reintroduce
/// exactly the defect it exists to close.
fn published_row_from_document(section_heading: &str, expected_cells: usize) -> Vec<bool> {
    let section_start = PROTOCOL_README
        .find(section_heading)
        .unwrap_or_else(|| panic!("docs/protocol/README.md must contain `{section_heading}`"));
    let section = &PROTOCOL_README[section_start + section_heading.len()..];
    // Stop at the next heading of the same or higher level so the row cannot be
    // picked up from an unrelated section further down the document.
    let section_end = section
        .match_indices("\n#")
        .map(|(i, _)| i)
        .find(|&i| {
            let rest = &section[i + 2..];
            rest.starts_with('#') || rest.starts_with(' ')
        })
        .unwrap_or(section.len());
    let section = &section[..section_end];

    let mut rows = section
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with(TABLE_ROW_LABEL));
    let row = rows.next().unwrap_or_else(|| {
        panic!("section `{section_heading}` must contain a row starting with `{TABLE_ROW_LABEL}`")
    });
    assert!(
        rows.next().is_none(),
        "section `{section_heading}` must contain exactly one `{TABLE_ROW_LABEL}` row; \
         two rows means the published table has been duplicated and this gate \
         can no longer say which one is normative"
    );

    let cells: Vec<&str> = row
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .skip(1) // the "Coblox v0" label cell
        .collect();
    assert_eq!(
        cells.len(),
        expected_cells,
        "the published table in `{section_heading}` must carry exactly \
         {expected_cells} outcome cells, one per vector; found {}: {row}",
        cells.len()
    );

    cells
        .iter()
        .enumerate()
        .map(|(i, cell)| match *cell {
            "accept" => true,
            "reject" => false,
            other => panic!(
                "vector {i} of the published table in `{section_heading}` reads \
                 `{other}`; the only conformant values are `accept` and `reject`"
            ),
        })
        .collect()
}

/// The twelve upstream `ed25519-speccheck` outcomes, as published.
fn published_outcomes_from_document() -> Vec<bool> {
    published_row_from_document(TABLE_SECTION, 12)
}

/// The seven Coblox extension outcomes, as published.
fn published_extension_outcomes_from_document() -> Vec<bool> {
    published_row_from_document(EXTENSION_TABLE_SECTION, 7)
}

/// Decode a lower-case hex string of any length.
///
/// The extension vectors sign a whole finality-vote preimage, which is 101 bytes
/// and not 32, so the fixed-width [`hex_lower_decode`] cannot read them.
fn hex_to_vec(text: &str) -> Vec<u8> {
    assert!(
        text.len().is_multiple_of(2),
        "hex string must have even length: {text}"
    );
    (0..text.len() / 2)
        .map(|i| {
            u8::from_str_radix(&text[i * 2..i * 2 + 2], 16)
                .unwrap_or_else(|_| panic!("invalid hex byte at {i} in {text}"))
        })
        .collect()
}

struct TestVectorEntry {
    index: usize,
    comment: &'static str,
    /// Not `[u8; 32]`: the extension vectors sign a whole finality-vote preimage.
    message: Vec<u8>,
    pub_key: [u8; 32],
    signature: [u8; 64],
    /// The fixture's own `expected_coblox` field — *not* the published table.
    /// The two are brought together only by
    /// [`fixture_expectations_agree_with_the_published_table`].
    expected: bool,
}

/// The twelve upstream vectors, with the Coblox annotations added.
fn load_speccheck_vectors() -> Vec<TestVectorEntry> {
    load_vector_file(
        include_str!("fixtures/ed25519_speccheck.json"),
        "ed25519_speccheck.json",
        12,
    )
}

/// The seven Coblox extension vectors for the `y >= 2^255-19` half of rule 1.
fn load_extension_vectors() -> Vec<TestVectorEntry> {
    load_vector_file(
        include_str!("fixtures/ed25519_coblox_extension.json"),
        "ed25519_coblox_extension.json",
        7,
    )
}

fn load_vector_file(json_bytes: &str, name: &str, expected: usize) -> Vec<TestVectorEntry> {
    let json: serde_json::Value =
        serde_json::from_str(json_bytes).unwrap_or_else(|_| panic!("{name} must be valid JSON"));

    let array = json.as_array().expect("vectors file must be a JSON array");
    assert_eq!(
        array.len(),
        expected,
        "{name} must contain exactly {expected} vectors"
    );

    array
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let index =
                usize::try_from(v["index"].as_u64().expect("index")).expect("valid usize index");
            assert_eq!(index, i);
            let comment = v["comment"].as_str().expect("comment");
            let message = hex_to_vec(v["message"].as_str().expect("message"));
            let pub_key_hex = v["pub_key"].as_str().expect("pub_key");
            let pub_key = hex_lower_decode::<32>(pub_key_hex).expect("valid hex pub_key");
            let sig_hex = v["signature"].as_str().expect("signature");
            let signature = hex_lower_decode::<64>(sig_hex).expect("valid hex signature");

            let expected_str = v["expected_coblox"].as_str().expect("expected_coblox");
            let expected = match expected_str {
                "accept" => true,
                "reject" => false,
                other => panic!("unknown expected outcome: {other}"),
            };

            TestVectorEntry {
                index,
                comment: Box::leak(comment.to_string().into_boxed_str()),
                message,
                pub_key,
                signature,
                expected,
            }
        })
        .collect()
}

/// Verification helper that implements cofactorless equation `[S]B = R + [k]A`
/// for direct differential testing.
fn verify_cofactorless_differential(
    public_key: &[u8; 32],
    message: &[u8],
    signature: &[u8; 64],
) -> bool {
    let Some(a_point) = CompressedEdwardsY(*public_key).decompress() else {
        return false;
    };
    if a_point.is_small_order() {
        return false;
    }

    let mut r_bytes = [0u8; 32];
    r_bytes.copy_from_slice(&signature[..32]);

    let mut s_bytes = [0u8; 32];
    s_bytes.copy_from_slice(&signature[32..]);

    let Some(s_scalar) = Option::<Scalar>::from(Scalar::from_canonical_bytes(s_bytes)) else {
        return false;
    };
    let Some(r_point) = CompressedEdwardsY(r_bytes).decompress() else {
        return false;
    };

    let mut hasher = Sha512::new();
    hasher.update(r_bytes);
    hasher.update(public_key);
    hasher.update(message);
    let k_output: [u8; 64] = hasher.finalize().into();
    let k = Scalar::from_bytes_mod_order_wide(&k_output);

    let r_prime = EdwardsPoint::vartime_double_scalar_mul_basepoint(&k, &-a_point, &s_scalar);
    // Cofactorless check: R == R' (without multiplying by 8)
    (r_point - r_prime).is_identity()
}

/// Verification helper that computes `k` over recompressed/reduced point bytes
/// instead of raw input encodings.
fn verify_with_recompressed_points_hash(
    public_key: &[u8; 32],
    message: &[u8],
    signature: &[u8; 64],
) -> bool {
    let Some(a_point) = CompressedEdwardsY(*public_key).decompress() else {
        return false;
    };
    if a_point.is_small_order() {
        return false;
    }

    let mut r_bytes = [0u8; 32];
    r_bytes.copy_from_slice(&signature[..32]);

    let mut s_bytes = [0u8; 32];
    s_bytes.copy_from_slice(&signature[32..]);

    let Some(s_scalar) = Option::<Scalar>::from(Scalar::from_canonical_bytes(s_bytes)) else {
        return false;
    };
    let Some(r_point) = CompressedEdwardsY(r_bytes).decompress() else {
        return false;
    };

    // Wrong: recompress points to canonical representation before hashing
    let recompressed_r = r_point.compress().to_bytes();
    let recompressed_a = a_point.compress().to_bytes();

    let mut hasher = Sha512::new();
    hasher.update(recompressed_r);
    hasher.update(recompressed_a);
    hasher.update(message);
    let k_output: [u8; 64] = hasher.finalize().into();
    let k = Scalar::from_bytes_mod_order_wide(&k_output);

    let r_prime = EdwardsPoint::vartime_double_scalar_mul_basepoint(&k, &-a_point, &s_scalar);
    (r_point - r_prime).mul_by_cofactor().is_identity()
}

#[test]
fn gate_speccheck_table_conformance_vector_by_vector() {
    let vectors = load_speccheck_vectors();
    let published = published_outcomes_from_document();
    let verifier = ConsensusVerifier;

    println!(
        "\n========================================================================================================="
    );
    println!(" Coblox v0 Ed25519 Speccheck Conformance Table (GATE-SPECCHECK)");
    println!(
        " Published column parsed from docs/protocol/README.md {TABLE_SECTION:?}, row {TABLE_ROW_LABEL:?}"
    );
    println!(
        "========================================================================================================="
    );
    println!(
        "| {:<6} | {:<9} | {:<8} | {:<8} | {:<8} | {:<48} |",
        "Vector", "Published", "Observed", "Fixture", "Status", "Comment"
    );
    println!(
        "|--------+-----------+----------+----------+----------+--------------------------------------------------|"
    );

    let mut all_match = true;

    for tv in &vectors {
        let published_outcome = published[tv.index];
        let published_str = if published_outcome {
            "accept"
        } else {
            "reject"
        };
        let msg_preimage = SigningPreimage::from_raw_bytes_non_consensus(&tv.message);
        let observed_fn = verify_consensus_ed25519(&tv.pub_key, &msg_preimage, &tv.signature);
        let observed_trait = verifier.verify(&tv.pub_key, &msg_preimage, &tv.signature);
        assert_eq!(
            observed_fn, observed_trait,
            "standalone function and trait implementation must agree"
        );

        let observed_str = if observed_fn { "accept" } else { "reject" };
        let matches = observed_fn == published_outcome;
        if !matches {
            all_match = false;
        }

        let status_str = if matches { "MATCH" } else { "MISMATCH" };

        let short_comment = if tv.comment.len() > 48 {
            &tv.comment[..48]
        } else {
            tv.comment
        };

        let fixture_str = if tv.expected { "accept" } else { "reject" };

        println!(
            "| Vector {:<1} | {:<9} | {:<8} | {:<8} | {:<8} | {:<48} |",
            tv.index, published_str, observed_str, fixture_str, status_str, short_comment
        );
    }
    println!(
        "=========================================================================================================\n"
    );

    assert!(
        all_match,
        "every observed outcome must match the published Coblox v0 table in \
         docs/protocol/README.md. A MISMATCH row above is not necessarily an \
         implementation defect: it means the document and a conformant \
         implementation disagree, and which of the two is wrong has to be \
         settled by derivation before either is changed."
    );
}

/// The fixture file carries its own `expected_coblox` field, which is convenient
/// and is also a second copy of the published table. This test is the only place
/// the copy is allowed to exist: it must agree with the document, vector by
/// vector, or the suite fails.
#[test]
fn fixture_expectations_agree_with_the_published_table() {
    let vectors = load_speccheck_vectors();
    let published = published_outcomes_from_document();

    let divergent: Vec<String> = vectors
        .iter()
        .filter(|tv| tv.expected != published[tv.index])
        .map(|tv| {
            format!(
                "vector {}: fixture says {}, document says {}",
                tv.index,
                if tv.expected { "accept" } else { "reject" },
                if published[tv.index] {
                    "accept"
                } else {
                    "reject"
                }
            )
        })
        .collect();

    assert!(
        divergent.is_empty(),
        "ed25519_speccheck.json disagrees with docs/protocol/README.md:\n  {}",
        divergent.join("\n  ")
    );
}

#[test]
fn gate_cofactor_differential_verification() {
    // GATE-COFACTOR requirement:
    // "Esiste almeno un caso in cui l'equazione con cofattore e quella senza danno esiti diversi,
    //  e la trascrizione mostra che l'implementazione segue quella con cofattore."
    let vectors = load_speccheck_vectors();

    // Vector 4 is specifically constructed such that:
    // - Cofactored verification passes: [8][S]B = [8]R + [8][k]A
    // - Cofactorless verification fails: [S]B != R + [k]A
    let v4 = &vectors[4];
    assert_eq!(v4.index, 4);

    let v4_preimage = SigningPreimage::from_raw_bytes_non_consensus(&v4.message);
    let cofactored_result = verify_consensus_ed25519(&v4.pub_key, &v4_preimage, &v4.signature);
    let cofactorless_result =
        verify_cofactorless_differential(&v4.pub_key, &v4.message, &v4.signature);

    println!("\n=== GATE-COFACTOR: Differential Equation Verification ===");
    println!("Vector 4 (A mixed, R mixed):");
    println!(
        "  Cofactored equation   [8][S]B = [8]R + [8][k]A : {}",
        if cofactored_result {
            "ACCEPT (true)"
        } else {
            "REJECT (false)"
        }
    );
    println!(
        "  Cofactorless equation    [S]B = R + [k]A       : {}",
        if cofactorless_result {
            "ACCEPT (true)"
        } else {
            "REJECT (false)"
        }
    );

    assert!(
        cofactored_result,
        "ConsensusVerifier MUST accept Vector 4 using cofactored verification"
    );
    assert!(
        !cofactorless_result,
        "Vector 4 MUST fail under cofactorless equation"
    );
    assert_ne!(
        cofactored_result, cofactorless_result,
        "Vector 4 must produce different outcomes under cofactored vs cofactorless equations"
    );
}

#[test]
fn original_encodings_hash_differential() {
    // Vectors 8 and 9 test non-canonical R encoding (y = 2^255 - 20, encoded as ecff..ff):
    // - Vector 8 was crafted with k computed over reduced R.
    // - Vector 9 was crafted with k computed over raw R_enc.
    // Coblox v0 and ZIP-215 mandate that k uses original bytes R_enc.
    let vectors = load_speccheck_vectors();
    let v8 = &vectors[8];
    let v9 = &vectors[9];

    // Under normative Coblox rule (raw R_enc in hash):
    let v8_preimage = SigningPreimage::from_raw_bytes_non_consensus(&v8.message);
    let v9_preimage = SigningPreimage::from_raw_bytes_non_consensus(&v9.message);
    let v8_normative = verify_consensus_ed25519(&v8.pub_key, &v8_preimage, &v8.signature);
    let v9_normative = verify_consensus_ed25519(&v9.pub_key, &v9_preimage, &v9.signature);

    // Under recompressed hash (wrong behavior):
    let v8_recompressed =
        verify_with_recompressed_points_hash(&v8.pub_key, &v8.message, &v8.signature);
    let v9_recompressed =
        verify_with_recompressed_points_hash(&v9.pub_key, &v9.message, &v9.signature);

    println!("\n=== Original Encodings vs. Re-encoded Points Hash Differential ===");
    println!("Vector 8 (k crafted over reduced R):");
    println!("  Normative (raw R_enc in hash)        : {v8_normative} (expected reject)");
    println!("  Recompressed (reduced R in hash)     : {v8_recompressed} (wrongly accepted)");
    println!("Vector 9 (k crafted over raw R_enc):");
    println!("  Normative (raw R_enc in hash)        : {v9_normative} (expected accept)");
    println!("  Recompressed (reduced R in hash)     : {v9_recompressed} (wrongly rejected)");

    assert!(
        !v8_normative,
        "Vector 8 must be rejected by normative verifier"
    );
    assert!(
        v9_normative,
        "Vector 9 must be accepted by normative verifier"
    );
    assert!(
        v8_recompressed,
        "Vector 8 passes only if points are erroneously re-encoded before hashing"
    );
    assert!(
        !v9_recompressed,
        "Vector 9 fails if points are erroneously re-encoded before hashing"
    );
}

#[test]
fn small_order_public_keys_are_strictly_rejected() {
    // Test the 8 small-order torsion points from curve25519 constants.
    // Even if a signature is syntactically valid or identity-derived,
    // [8]A == identity MUST result in immediate rejection.
    let small_order_points: [[u8; 32]; 8] = [
        // (0, 1), order 1
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
        // order 8
        [
            199, 23, 106, 112, 61, 77, 216, 79, 186, 60, 11, 118, 13, 16, 103, 15, 42, 32, 83, 250,
            44, 57, 204, 198, 78, 199, 253, 119, 146, 172, 3, 122,
        ],
        // order 4
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 128,
        ],
        // order 8
        [
            38, 232, 149, 143, 194, 178, 39, 176, 69, 195, 244, 137, 242, 239, 152, 240, 213, 223,
            172, 5, 211, 198, 51, 57, 177, 56, 2, 136, 109, 83, 252, 5,
        ],
        // (0, -1), order 2
        [
            236, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127,
        ],
        // order 8
        [
            38, 232, 149, 143, 194, 178, 39, 176, 69, 195, 244, 137, 242, 239, 152, 240, 213, 223,
            172, 5, 211, 198, 51, 57, 177, 56, 2, 136, 109, 83, 252, 133,
        ],
        // order 4
        [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
        // order 8
        [
            199, 23, 106, 112, 61, 77, 216, 79, 186, 60, 11, 118, 13, 16, 103, 15, 42, 32, 83, 250,
            44, 57, 204, 198, 78, 199, 253, 119, 146, 172, 3, 250,
        ],
    ];

    let message =
        SigningPreimage::from_raw_bytes_non_consensus(b"test message for small order pk rejection");
    let dummy_signature = [0u8; 64];

    for (i, pk) in small_order_points.iter().enumerate() {
        let result = verify_consensus_ed25519(pk, &message, &dummy_signature);
        assert!(
            !result,
            "small order point {i} must be rejected by ConsensusVerifier"
        );
    }
}

#[test]
fn verifier_respects_signing_preimage_contract() {
    let chain_id = ChainId::from_digest(Digest32::repeated(0x42));
    let payload = b"consensus payload to be signed";
    let preimage = signing_preimage(Domain::SIG_BLOCK_VOTE, &chain_id, payload);

    // The preimage starts with the domain string, zero byte, and chain_id bytes
    assert!(preimage.as_bytes().starts_with(b"coblox-block-vote-v0\0"));
    assert_eq!(
        &preimage.as_bytes()[Domain::SIG_BLOCK_VOTE.as_str().len() + 1
            ..Domain::SIG_BLOCK_VOTE.as_str().len() + 1 + 32],
        chain_id.as_digest().as_bytes()
    );

    // Verifier takes the typed SigningPreimage, not a 32-byte digest or arbitrary slice
    let verifier = ConsensusVerifier;
    let dummy_key = [0x11; 32];
    let dummy_sig = [0x22; 64];
    // Must execute cleanly (returning false on dummy data, without panicking)
    let _ = verifier.verify(&dummy_key, &preimage, &dummy_sig);
}

// ---------------------------------------------------------------------------
// The second implementation, and why it is in this file
// ---------------------------------------------------------------------------

/// `2^255-19` little-endian: the boundary clause 1a of rule 1 is about.
const P_LE: [u8; 32] = [
    0xed, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f,
];

/// `true` if the masked `y` of `encoding` is `< 2^255-19`.
///
/// RFC 8032 §5.1.3 step 2 requires this and rejects the encoding otherwise;
/// clause 1a of the Coblox rule reduces instead. The whole divergence this file
/// exists to exercise is one `if` over this predicate.
fn masked_y_is_canonical(encoding: &[u8; 32]) -> bool {
    let mut y = *encoding;
    y[31] &= 0x7f;
    for i in (0..32).rev() {
        if y[i] != P_LE[i] {
            return y[i] < P_LE[i];
        }
    }
    false // y == p is not canonical either
}

/// Decoding with the two RFC 8032 restrictions independently switchable.
///
/// `reject_non_canonical_y` is RFC 8032 §5.1.3 step 2; `reject_zero_x_with_sign`
/// is step 3. Coblox applies neither. Setting only the first gives the
/// implementation [REVIEW-019] identified as the dangerous class: ZIP-215 on the
/// sign bit, the RFC on canonicity, indistinguishable from Coblox on the
/// upstream twelve.
fn decompress_variant(
    encoding: &[u8; 32],
    reject_non_canonical_y: bool,
    reject_zero_x_with_sign: bool,
) -> Option<EdwardsPoint> {
    if reject_non_canonical_y && !masked_y_is_canonical(encoding) {
        return None;
    }
    let point = CompressedEdwardsY(*encoding).decompress()?;
    if reject_zero_x_with_sign
        && encoding[31] >> 7 == 1
        && point.compress().to_bytes()[31] >> 7 == 0
    {
        // The input asked for an odd `x` and the canonical re-encoding of the
        // decoded point has the sign bit clear, which for a point on this curve
        // happens only when `x = 0`.
        return None;
    }
    Some(point)
}

/// The Coblox rule with rule 1 replaced and rules 2-4 untouched.
///
/// A deliberate second implementation, not a helper: the question the extension
/// vectors settle is what a difference confined to decoding does to the verdict,
/// so everything else here mirrors `verifier.rs` exactly, including hashing the
/// original encodings.
fn verify_with_decoder_variant(
    public_key: &[u8; 32],
    message: &[u8],
    signature: &[u8; 64],
    reject_non_canonical_y: bool,
    reject_zero_x_with_sign: bool,
) -> bool {
    let Some(a_point) =
        decompress_variant(public_key, reject_non_canonical_y, reject_zero_x_with_sign)
    else {
        return false;
    };
    if a_point.is_small_order() {
        return false;
    }

    let mut r_bytes = [0u8; 32];
    r_bytes.copy_from_slice(&signature[..32]);
    let mut s_bytes = [0u8; 32];
    s_bytes.copy_from_slice(&signature[32..]);

    let Some(s_scalar) = Option::<Scalar>::from(Scalar::from_canonical_bytes(s_bytes)) else {
        return false;
    };
    let Some(r_point) =
        decompress_variant(&r_bytes, reject_non_canonical_y, reject_zero_x_with_sign)
    else {
        return false;
    };

    let mut hasher = Sha512::new();
    hasher.update(r_bytes);
    hasher.update(public_key);
    hasher.update(message);
    let k_output: [u8; 64] = hasher.finalize().into();
    let k = Scalar::from_bytes_mod_order_wide(&k_output);

    let r_prime = EdwardsPoint::vartime_double_scalar_mul_basepoint(&k, &-a_point, &s_scalar);
    (r_point - r_prime).mul_by_cofactor().is_identity()
}

#[test]
fn gate_speccheck_extension_table_conformance_vector_by_vector() {
    let vectors = load_extension_vectors();
    let published = published_extension_outcomes_from_document();
    let verifier = ConsensusVerifier;

    println!(
        "\n========================================================================================================="
    );
    println!(
        " Coblox v0 Ed25519 Extension Conformance Table (GATE-SPECCHECK, clause 1a of rule 1)"
    );
    println!(
        " Published column parsed from docs/protocol/README.md {EXTENSION_TABLE_SECTION:?}, row {TABLE_ROW_LABEL:?}"
    );
    println!(
        "========================================================================================================="
    );
    println!(
        "| {:<6} | {:<9} | {:<8} | {:<8} | {:<8} | {:<48} |",
        "Vector", "Published", "Observed", "Fixture", "Status", "Comment"
    );
    println!(
        "|--------+-----------+----------+----------+----------+--------------------------------------------------|"
    );

    let mut all_match = true;
    for tv in &vectors {
        let published_outcome = published[tv.index];
        let msg_preimage = SigningPreimage::from_raw_bytes_non_consensus(&tv.message);
        let observed_fn = verify_consensus_ed25519(&tv.pub_key, &msg_preimage, &tv.signature);
        let observed_trait = verifier.verify(&tv.pub_key, &msg_preimage, &tv.signature);
        assert_eq!(
            observed_fn, observed_trait,
            "standalone function and trait implementation must agree"
        );
        let matches = observed_fn == published_outcome;
        if !matches {
            all_match = false;
        }
        let short_comment = if tv.comment.len() > 48 {
            &tv.comment[..48]
        } else {
            tv.comment
        };
        println!(
            "| Vector {:<1} | {:<9} | {:<8} | {:<8} | {:<8} | {:<48} |",
            tv.index,
            if published_outcome {
                "accept"
            } else {
                "reject"
            },
            if observed_fn { "accept" } else { "reject" },
            if tv.expected { "accept" } else { "reject" },
            if matches { "MATCH" } else { "MISMATCH" },
            short_comment
        );
    }
    println!(
        "=========================================================================================================\n"
    );

    assert!(
        all_match,
        "every observed outcome must match the published Coblox extension table \
         in docs/protocol/README.md. A MISMATCH row above is not necessarily an \
         implementation defect: it means the document and a conformant \
         implementation disagree, and which of the two is wrong has to be \
         settled by derivation before either is changed."
    );
}

/// Same discipline as [`fixture_expectations_agree_with_the_published_table`],
/// applied to the extension file: its `expected_coblox` field is the only copy
/// of the published row that is allowed to exist, and it must agree with it.
#[test]
fn extension_fixture_expectations_agree_with_the_published_table() {
    let vectors = load_extension_vectors();
    let published = published_extension_outcomes_from_document();

    let divergent: Vec<String> = vectors
        .iter()
        .filter(|tv| tv.expected != published[tv.index])
        .map(|tv| {
            format!(
                "extension vector {}: fixture says {}, document says {}",
                tv.index,
                if tv.expected { "accept" } else { "reject" },
                if published[tv.index] {
                    "accept"
                } else {
                    "reject"
                }
            )
        })
        .collect();

    assert!(
        divergent.is_empty(),
        "ed25519_coblox_extension.json disagrees with docs/protocol/README.md:\n  {}",
        divergent.join("\n  ")
    );
}

/// The negative proof [REVIEW-019] RF-001 requires, executed rather than
/// transcribed.
///
/// It asserts three things, and the third is the sharpest:
///
/// 1. an implementation that rejects a masked `y >= 2^255-19` and is otherwise
///    identical returns **the same verdict on all twelve upstream vectors**, so
///    `GATE-SPECCHECK` on the twelve alone cannot tell it apart from Coblox;
/// 2. the same implementation **diverges on the extension vectors**, which is
///    what those vectors exist for. If a future change made the extension
///    vectors agree under both decoders, they would have quietly stopped being
///    evidence, and this assertion fails rather than letting that happen;
/// 3. the *fully* strict RFC 8032 decoder is already excluded by the upstream
///    twelve, and by exactly one of them — vector 9. That is why the dangerous
///    class is the intermediate implementation and not the strict one, and it is
///    checked here so the claim cannot age.
#[test]
fn strict_y_decoding_agrees_on_the_twelve_and_diverges_on_the_extension() {
    let upstream = load_speccheck_vectors();
    let extension = load_extension_vectors();

    println!("\n=== Decoder divergence: Coblox vs. an implementation that rejects y >= p ===");

    let mut upstream_disagreements = Vec::new();
    let mut rfc_disagreements = Vec::new();
    for tv in &upstream {
        let msg_preimage = SigningPreimage::from_raw_bytes_non_consensus(&tv.message);
        let coblox = verify_consensus_ed25519(&tv.pub_key, &msg_preimage, &tv.signature);
        let strict_y =
            verify_with_decoder_variant(&tv.pub_key, &tv.message, &tv.signature, true, false);
        let rfc8032 =
            verify_with_decoder_variant(&tv.pub_key, &tv.message, &tv.signature, true, true);
        if coblox != strict_y {
            upstream_disagreements.push(tv.index);
        }
        if coblox != rfc8032 {
            rfc_disagreements.push(tv.index);
        }
    }

    let mut extension_disagreements = Vec::new();
    for tv in &extension {
        let msg_preimage = SigningPreimage::from_raw_bytes_non_consensus(&tv.message);
        let coblox = verify_consensus_ed25519(&tv.pub_key, &msg_preimage, &tv.signature);
        let strict_y =
            verify_with_decoder_variant(&tv.pub_key, &tv.message, &tv.signature, true, false);
        println!(
            "  extension vector {}: Coblox {:<6}  y>=p-rejecting {:<6}  {}",
            tv.index,
            if coblox { "accept" } else { "reject" },
            if strict_y { "accept" } else { "reject" },
            if coblox == strict_y {
                "agree"
            } else {
                "DIVERGE"
            }
        );
        if coblox != strict_y {
            extension_disagreements.push(tv.index);
        }
    }

    println!("  upstream vectors 0-11, disagreements: {upstream_disagreements:?}");
    println!("  extension vectors 0-6,  disagreements: {extension_disagreements:?}");
    println!("  fully strict RFC 8032 decoder, disagreements on the twelve: {rfc_disagreements:?}");

    assert!(
        upstream_disagreements.is_empty(),
        "an implementation that rejects y >= p must agree with Coblox on all \
         twelve upstream vectors; that agreement is the finding, not a bug. \
         Disagreements: {upstream_disagreements:?}"
    );
    assert_eq!(
        extension_disagreements,
        vec![0, 1, 2, 3],
        "the four forgeable extension vectors must separate Coblox from a \
         y >= p-rejecting implementation; if they no longer do, the extension \
         table has stopped being evidence for clause 1a of rule 1"
    );
    assert_eq!(
        rfc_disagreements,
        vec![9],
        "the fully strict RFC 8032 decoder must be excluded by upstream vector 9 \
         and by nothing else: that is what makes the intermediate implementation, \
         and not the strict one, the class the extension vectors are aimed at"
    );
}

/// The versioned upstream copy still hashes to what its provenance claims.
///
/// The expected digest is read out of `fixtures/README.md`, not written here: a
/// constant in this file would be a second copy of the provenance and would age
/// away from it in silence, which is the [ADR-012] family of defect.
#[test]
fn upstream_cases_file_matches_its_recorded_digest() {
    let line = FIXTURE_README
        .lines()
        .find(|line| line.contains(UPSTREAM_DIGEST_MARKER))
        .unwrap_or_else(|| {
            panic!("fixtures/README.md must record `{UPSTREAM_DIGEST_MARKER} <sha256>`")
        });
    let recorded = line
        .rsplit('`')
        .nth(1)
        .expect("the digest must be the last backtick-quoted span on its line");
    assert_eq!(
        recorded.len(),
        64,
        "recorded upstream digest must be 64 hex characters, found `{recorded}`"
    );

    let mut hasher = Sha256::new();
    hasher.update(UPSTREAM_CASES.as_bytes());
    let digest: [u8; 32] = hasher.finalize().into();
    let observed = hex_lower(&digest);

    assert_eq!(
        observed, recorded,
        "the versioned copy of the upstream `cases.json` no longer hashes to the \
         digest recorded in fixtures/README.md; the provenance and the bytes have \
         parted company and neither is authoritative until that is explained"
    );
}

/// The annotated fixture carries the upstream bytes and nothing else.
///
/// This is the Lead's byte-for-byte check against upstream, made repeatable and
/// mechanical: [REVIEW-019] RF-004 recorded that it had been performed once,
/// against a moving branch reference, and so could not be repeated at all.
#[test]
fn derived_fixture_matches_upstream_cases_byte_for_byte() {
    let upstream: serde_json::Value =
        serde_json::from_str(UPSTREAM_CASES).expect("upstream cases.json must be valid JSON");
    let upstream = upstream
        .as_array()
        .expect("upstream cases must be an array");
    let derived = load_speccheck_vectors();
    assert_eq!(
        upstream.len(),
        derived.len(),
        "the derived fixture must carry exactly the upstream vectors, in order"
    );

    for (i, (up, tv)) in upstream.iter().zip(derived.iter()).enumerate() {
        assert_eq!(
            hex_to_vec(up["message"].as_str().expect("upstream message")),
            tv.message,
            "vector {i}: message differs from upstream"
        );
        assert_eq!(
            hex_lower_decode::<32>(up["pub_key"].as_str().expect("upstream pub_key"))
                .expect("valid upstream pub_key"),
            tv.pub_key,
            "vector {i}: pub_key differs from upstream"
        );
        assert_eq!(
            hex_lower_decode::<64>(up["signature"].as_str().expect("upstream signature"))
                .expect("valid upstream signature"),
            tv.signature,
            "vector {i}: signature differs from upstream"
        );
    }
}
