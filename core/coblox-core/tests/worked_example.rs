//! The epoch-3 worked example of `ledger.md#worked-example-of-the-derivation`,
//! reproduced in full.
//!
//! **Provenance rule.** Every `EXPECTED_*` constant below is copied from the
//! `text` blocks and tables of that section. The example publishes five leaves,
//! the empty leaf, six internal nodes, the root, the entropy, the seed, three
//! tickets, the resulting order, the `fills` minimum and the final table of
//! four seats; all of them are asserted here, individually, against the
//! document.
//!
//! "The example is normative in form and not in values." Its parameters are
//! therefore instantiated in `common::worked_example_parameters`, are validated
//! against the constraint block before use, and are not promoted to constants
//! anywhere in `coblox-core`.

mod common;

use coblox_core::election::{self, CandidateFacts};
use coblox_core::error::{ElectionError, Error};
use coblox_core::hash::{AccountKey, Digest32};
use coblox_core::merkle::{self, TaggedTree};
use coblox_core::validator_set::{ElectionRecord, ValidatorEntry, ValidatorSet};

use common::{worked_example_parameters, zero_chain_id};

// `candidate_leaf = H(0x40 || u64be(3) || account_key_32)`
const EXPECTED_LEAF_02: &str = "cd3950bac9c60b73523a0f8157e5da8384515d2e9c4ef4127be2d910b878158c";
const EXPECTED_LEAF_04: &str = "004e3b02570032774d181a20dbb4d5e23f6fe83092374635189f42c513db97d9";
const EXPECTED_LEAF_05: &str = "154a73c742c2f580aafaf97401ef5633898862d761fc13ed230db7817b0111fd";
const EXPECTED_LEAF_06: &str = "9ecce0184a6b8d6d3a1257add0af3a16cc64809fe2cf723cfe91b570abe07f44";
const EXPECTED_LEAF_08: &str = "c9e67f59e0d4a6c08cc588bb906b17c8bddba657def06d6c2f111c64420796ca";
const EXPECTED_CANDIDATE_EMPTY: &str =
    "df7e70e5021544f4834bbee64a9e3789febc4be81470df629cad6ddb03320a5c";

// level 1
const EXPECTED_N1_0: &str = "00d36c1eb2fc336cb635735bbae46cf3e2a32322f4761db3d29a297644e45bb2";
const EXPECTED_N1_1: &str = "db1bd3372cea52438df608862c3dcb3c0a2b1c16b152c296c2a8effe9005bc20";
const EXPECTED_N1_2: &str = "b65c2a34ededd92d98721c11a620cc6f16e87a03f512e54fd35b35cd70c3bf39";
const EXPECTED_N1_3: &str = "a7ee32ed571f99897d698653a14d292cfea8e8633f95a9db217ea993c7773e91";
// level 2
const EXPECTED_N2_0: &str = "a2bb9ac490112abad3c2af7197fdf6efe62af01b7c251f8d3443b20fa145a060";
const EXPECTED_N2_1: &str = "5d426fa91db2b6acb77c0980837dea9e54380afc25eede062e05e2cab2dc6c2b";
const EXPECTED_CANDIDATE_ROOT: &str =
    "42e4f6b1f01af3b69ba154aa464738829635a3ed7facf65e652d9712b461924a";

const EXPECTED_ENTROPY: &str = "29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42";
const EXPECTED_SEED: &str = "9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85";

const EXPECTED_TICKET_05: &str = "a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21";
const EXPECTED_TICKET_06: &str = "547132161f56f2361faf3caae90224e1b5c04ae1986ab871703b7003693c5fdd";
const EXPECTED_TICKET_08: &str = "9d04ef2f66f5403d275fa1f80819ce40a17af81931d263d3c390e0291e0b19c9";

fn expect(hex: &str) -> Digest32 {
    Digest32::parse_hex(hex).expect("an expected value from the worked example")
}

fn key(byte: u8) -> AccountKey {
    AccountKey::from_bytes([byte; 32])
}

/// The example writes account keys as a byte repeated 32 times and identifies
/// nodes by them. Node identifiers are opaque strings in the protocol, so the
/// example's `01`..`08` become these; their order matches the account-key order,
/// which is what the assembly step sorts on.
fn node(byte: u8) -> String {
    format!("cblx1node{byte:02x}")
}

fn entropy_block_ids() -> Vec<Digest32> {
    // "The entropy window holds block IDs `aa`x32 at height 297, `bb`x32 at 298
    // and `cc`x32 at 299".
    vec![
        Digest32::repeated(0xaa),
        Digest32::repeated(0xbb),
        Digest32::repeated(0xcc),
    ]
}

fn entry(byte: u8, seated_since_epoch: u64, term_expiry_epoch: u64) -> ValidatorEntry {
    ValidatorEntry {
        validator_id: node(byte),
        node_id: node(byte),
        consensus_public_key: [byte; 32],
        key_binding_signature: [byte; 64],
        seated_since_epoch,
        term_expiry_epoch,
        voting_power: 1,
    }
}

fn facts(byte: u8, eligible: bool) -> CandidateFacts {
    CandidateFacts {
        node_id: node(byte),
        account_key: key(byte),
        consensus_public_key: [byte; 32],
        key_binding_signature: [byte; 64],
        eligible,
    }
}

/// `P`, the previous set active at height 299.
///
/// "| `01`x32 | 0 | 3 | ... | `02`x32 | 2 | 6 | ... | `03`x32 | 2 | 6 | ...
/// | `04`x32 | 1 | 5 | ...". Its own election record is the epoch-2 one it must
/// have carried; the example does not publish it and no assertion depends on
/// its contents beyond the set hash it produces.
fn previous_set() -> ValidatorSet {
    let validators = vec![
        entry(0x01, 0, 3),
        entry(0x02, 2, 6),
        entry(0x03, 2, 6),
        entry(0x04, 1, 5),
    ];
    ValidatorSet {
        schema_version: "0.1".to_owned(),
        activation_height: 200,
        election: Some(ElectionRecord {
            election_epoch: 2,
            previous_validator_set_hash: Digest32::repeated(0xee),
            candidate_root: Digest32::repeated(0xdd),
            candidate_count: 4,
            entropy_first_height: 197,
            entropy_block_ids: vec![
                Digest32::repeated(0x0a),
                Digest32::repeated(0x0b),
                Digest32::repeated(0x0c),
            ],
            election_seed: Digest32::repeated(0xcc),
            retained_count: 2,
            filled_count: 2,
            member_count: 4,
        }),
        validators,
    }
}

/// The epoch-3 facts of the retention table and the paragraph after it.
///
/// "`03` filed no candidacy for epoch 3: voluntary exit. [...] Node `07` filed
/// a candidacy but its `contribution_score` is below
/// `validator_eligibility_threshold_units`; node `01` is in cooldown; node `03`
/// filed nothing."
fn epoch_three_facts() -> Vec<CandidateFacts> {
    vec![
        facts(0x01, false),
        facts(0x02, true),
        facts(0x03, false),
        facts(0x04, true),
        facts(0x05, true),
        facts(0x06, true),
        facts(0x07, false),
        facts(0x08, true),
    ]
}

#[test]
fn the_five_candidate_leaves_and_the_empty_leaf() {
    assert_eq!(
        merkle::candidate_leaf(3, &key(0x02)),
        expect(EXPECTED_LEAF_02)
    );
    assert_eq!(
        merkle::candidate_leaf(3, &key(0x04)),
        expect(EXPECTED_LEAF_04)
    );
    assert_eq!(
        merkle::candidate_leaf(3, &key(0x05)),
        expect(EXPECTED_LEAF_05)
    );
    assert_eq!(
        merkle::candidate_leaf(3, &key(0x06)),
        expect(EXPECTED_LEAF_06)
    );
    assert_eq!(
        merkle::candidate_leaf(3, &key(0x08)),
        expect(EXPECTED_LEAF_08)
    );
    assert_eq!(
        TaggedTree::CANDIDATES.empty_leaf(),
        expect(EXPECTED_CANDIDATE_EMPTY)
    );
}

#[test]
fn the_six_internal_nodes_and_the_candidate_root() {
    let leaves: Vec<Digest32> = [0x02u8, 0x04, 0x05, 0x06, 0x08]
        .into_iter()
        .map(|byte| merkle::candidate_leaf(3, &key(byte)))
        .collect();
    let levels = TaggedTree::CANDIDATES.levels(&leaves);

    // "Leaves stay in `account_key` order and pad to eight with
    // `candidate_empty`."
    assert_eq!(levels[0].len(), 8);
    assert_eq!(levels[0][5], TaggedTree::CANDIDATES.empty_leaf());

    assert_eq!(levels[1][0], expect(EXPECTED_N1_0));
    assert_eq!(levels[1][1], expect(EXPECTED_N1_1));
    assert_eq!(levels[1][2], expect(EXPECTED_N1_2));
    assert_eq!(levels[1][3], expect(EXPECTED_N1_3));
    assert_eq!(levels[2][0], expect(EXPECTED_N2_0));
    assert_eq!(levels[2][1], expect(EXPECTED_N2_1));

    let root = merkle::candidate_root(3, &[key(0x02), key(0x04), key(0x05), key(0x06), key(0x08)])
        .expect("candidate_root");
    assert_eq!(root, expect(EXPECTED_CANDIDATE_ROOT));
    assert_eq!(levels[3][0], root);
}

#[test]
fn the_derived_heights_are_the_ones_the_example_states() {
    let parameters = worked_example_parameters();
    // "The epoch under election is `e = 3`, so the boundary is height 300,
    // candidacy closed at height 290, and the entropy window is heights 297,
    // 298 and 299."
    assert_eq!(parameters.election_boundary_height(3).unwrap(), 300);
    assert_eq!(parameters.candidacy_close_height(3).unwrap(), 290);
    assert_eq!(parameters.entropy_window(3).unwrap(), (297, 299));
}

#[test]
fn the_whole_epoch_three_derivation() {
    let parameters = worked_example_parameters();
    let chain_id = zero_chain_id();
    let derivation = election::derive(
        &chain_id,
        &parameters,
        3,
        &previous_set(),
        &epoch_three_facts(),
        &entropy_block_ids(),
    )
    .expect("the epoch-3 derivation yields a valid set");

    // C = { 02, 04, 05, 06, 08 }, candidate_count = 5
    assert_eq!(
        derivation.candidates,
        vec![key(0x02), key(0x04), key(0x05), key(0x06), key(0x08)]
    );

    assert_eq!(derivation.entropy, expect(EXPECTED_ENTROPY));
    assert_eq!(derivation.seed, expect(EXPECTED_SEED));

    // Tickets over Nw = { 05, 06, 08 }.
    let ticket_of = |byte: u8| {
        derivation
            .ranked_pool
            .iter()
            .find(|(_, account_key)| *account_key == key(byte))
            .expect("the pool holds this candidate")
            .0
    };
    assert_eq!(ticket_of(0x05), expect(EXPECTED_TICKET_05));
    assert_eq!(ticket_of(0x06), expect(EXPECTED_TICKET_06));
    assert_eq!(ticket_of(0x08), expect(EXPECTED_TICKET_08));

    // "Ascending by ticket: `06` (`5471...`), `08` (`9d04...`), `05` (`a10e...`)."
    assert_eq!(
        derivation
            .ranked_pool
            .iter()
            .map(|(_, account_key)| *account_key)
            .collect::<Vec<_>>(),
        vec![key(0x06), key(0x08), key(0x05)]
    );

    // "fills = min( max(0, 8 - 2), 2, 3 ) = 2", "so the cap binds and `05` is
    // not seated this epoch".
    assert_eq!(derivation.fills, 2);

    // The elected set, sorted by validator_id, with its stamps.
    let expected_seats = [
        (0x02u8, 2u64, 6u64),
        (0x04, 1, 5),
        (0x06, 3, 7),
        (0x08, 3, 7),
    ];
    let seats: Vec<(String, u64, u64, u64)> = derivation
        .set
        .validators
        .iter()
        .map(|entry| {
            (
                entry.validator_id.clone(),
                entry.seated_since_epoch,
                entry.term_expiry_epoch,
                entry.voting_power,
            )
        })
        .collect();
    assert_eq!(
        seats,
        expected_seats
            .iter()
            .map(|(byte, seated, expiry)| (node(*byte), *seated, *expiry, 1))
            .collect::<Vec<_>>()
    );
    assert!(
        derivation.set.find(&node(0x05)).is_none(),
        "the cap binds, so 05 is not seated this epoch"
    );

    let record = derivation.set.election.as_ref().expect("election record");
    assert_eq!(record.election_epoch, 3);
    assert_eq!(record.candidate_root, expect(EXPECTED_CANDIDATE_ROOT));
    assert_eq!(record.candidate_count, 5);
    assert_eq!(record.entropy_first_height, 297);
    assert_eq!(record.election_seed, expect(EXPECTED_SEED));
    // "`member_count` 4, `retained_count` 2, `filled_count` 2, which is at the cap."
    assert_eq!(record.member_count, 4);
    assert_eq!(record.retained_count, 2);
    assert_eq!(record.filled_count, 2);
    assert_eq!(derivation.set.activation_height, 300);
    assert_eq!(
        record.previous_validator_set_hash,
        previous_set().hash().unwrap()
    );

    // The set is valid under the same Layer-1 checks a light client applies.
    derivation.set.check_elected_shape(&parameters).unwrap();
    derivation
        .set
        .check_election_record(&chain_id, &parameters)
        .unwrap();
    derivation
        .set
        .check_stamps_against_previous(&previous_set(), &parameters)
        .unwrap();
    derivation
        .set
        .check_contraction_floor(&previous_set())
        .unwrap();
}

/// "Had the two newcomers been censored out of the candidate window, the set
/// would have been `R` alone — two members — and `3 * 2 > 2 * 4` is false, so
/// **that set would have been invalid and the chain would have stalled**."
#[test]
fn censoring_the_newcomers_stalls_the_chain_at_the_boundary() {
    let parameters = worked_example_parameters();
    let censored: Vec<CandidateFacts> = epoch_three_facts()
        .into_iter()
        .map(|fact| {
            if fact.node_id == node(0x05)
                || fact.node_id == node(0x06)
                || fact.node_id == node(0x08)
            {
                CandidateFacts {
                    eligible: false,
                    ..fact
                }
            } else {
                fact
            }
        })
        .collect();
    let outcome = election::derive(
        &zero_chain_id(),
        &parameters,
        3,
        &previous_set(),
        &censored,
        &entropy_block_ids(),
    );
    assert_eq!(
        outcome.unwrap_err(),
        Error::Election(ElectionError::ContractionFloor {
            new: 2,
            previous: 4
        })
    );
}

/// "The two new entries share `term_expiry_epoch` 7, which equals
/// `validator_churn_cap_seats` and is therefore admissible: at most `c` seats
/// are stamped at any boundary, so at most `c` expire at any later one."
#[test]
fn at_most_the_churn_cap_of_seats_share_an_expiry_stamp() {
    let derivation = election::derive(
        &zero_chain_id(),
        &worked_example_parameters(),
        3,
        &previous_set(),
        &epoch_three_facts(),
        &entropy_block_ids(),
    )
    .expect("derivation");
    let sharing_seven = derivation
        .set
        .validators
        .iter()
        .filter(|entry| entry.term_expiry_epoch == 7)
        .count();
    assert_eq!(sharing_seven, 2);
    assert!(
        u64::try_from(sharing_seven).unwrap()
            <= worked_example_parameters().get().validator_churn_cap_seats
    );
}
