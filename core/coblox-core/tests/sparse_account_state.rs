//! The depth-256 sparse account state tree and the proof rules of steps 8 to
//! 10 of the light-client algorithm.

mod common;

use coblox_core::hash::{AccountKey, Digest32, NodeId};
use coblox_core::json::JsonObject;
use coblox_core::merkle::{
    AccountProof, AccountState, AppLifecycle, SparseAccountTree, constant_time_eq,
};

/// The canonical proof example of `ledger.md#sparse-merkle-account-state`: "an
/// absent account with all-default siblings".
const CANONICAL_ABSENT_PROOF: &str = r#"{"account_key":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8","account_kind":"node","account_nonce":"0","balance_microtokens":"0","present":false,"sibling_bitmap":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","siblings":[],"subject_id":"cblx1absentfixture"}"#;

fn fixture_account_key() -> AccountKey {
    let mut bytes = [0u8; 32];
    for (index, slot) in bytes.iter_mut().enumerate() {
        *slot = u8::try_from(index).unwrap();
    }
    AccountKey::from_bytes(bytes)
}

#[test]
fn the_default_subtree_chain_is_the_one_the_specification_defines() {
    // `empty[256] = H(0x12)`
    assert_eq!(
        SparseAccountTree::empty_at(256),
        coblox_core::hash::tagged_hash(SparseAccountTree::EMPTY_LEAF_TAG, &[])
    );
    // `empty[d] = H(0x11 || empty[d+1] || empty[d+1])` for d = 255 down to 0
    for depth in (0..256).rev() {
        let child = SparseAccountTree::empty_at(depth + 1);
        assert_eq!(
            SparseAccountTree::empty_at(depth),
            SparseAccountTree::branch(&child, &child)
        );
    }
    // Every level is distinct, so a proof cannot substitute one depth's default
    // for another's.
    let mut seen = std::collections::BTreeSet::new();
    for depth in 0..=256 {
        assert!(seen.insert(SparseAccountTree::empty_at(depth)));
    }
}

#[test]
fn the_published_absent_proof_example_is_canonical_and_rebuilds_the_empty_root() {
    let object = JsonObject::parse_canonical(CANONICAL_ABSENT_PROOF.as_bytes())
        .expect("the published example is canonical");
    assert_eq!(object.to_jcs(), CANONICAL_ABSENT_PROOF.as_bytes());
    assert!(!object.boolean("present").unwrap());

    let proof = AccountProof {
        account_key: fixture_account_key(),
        state: AccountState::Absent,
        sibling_bitmap: [0u8; 32],
        siblings: Vec::new(),
    };
    // An all-default proof for an absent account reconstructs the root of an
    // entirely empty tree, whatever the key is.
    assert_eq!(
        proof.compute_root().unwrap(),
        SparseAccountTree::empty_at(0)
    );
    assert!(proof.verify(&SparseAccountTree::empty_at(0)));
    assert!(!proof.verify(&Digest32::repeated(0x01)));
}

/// "Negative fixture `SMT-1`: all-default absent proof with bit 0 set to 1 and
/// `siblings:[empty[1]]` rejects."
///
/// It rejects **even though it reconstructs the root**, which is the whole
/// point of the rule: an explicitly supplied default is a second encoding of a
/// proof that already has one.
#[test]
fn smt_1_rejects_an_explicitly_supplied_default_sibling() {
    let mut bitmap = [0u8; 32];
    bitmap[0] = 0b1000_0000; // bit 0, most significant bit first
    let proof = AccountProof {
        account_key: fixture_account_key(),
        state: AccountState::Absent,
        sibling_bitmap: bitmap,
        siblings: vec![SparseAccountTree::empty_at(1)],
    };
    assert!(proof.compute_root().is_err());
    assert!(!proof.verify(&SparseAccountTree::empty_at(0)));

    // The same proof with the bit cleared and the sibling omitted is the
    // canonical one, and it does reconstruct the root.
    let canonical = AccountProof {
        sibling_bitmap: [0u8; 32],
        siblings: Vec::new(),
        ..proof
    };
    assert_eq!(
        canonical.compute_root().unwrap(),
        SparseAccountTree::empty_at(0)
    );
}

#[test]
fn a_sibling_count_that_disagrees_with_the_bitmap_is_rejected() {
    let mut bitmap = [0u8; 32];
    bitmap[0] = 0b1100_0000; // two bits set
    let proof = AccountProof {
        account_key: fixture_account_key(),
        state: AccountState::Absent,
        sibling_bitmap: bitmap,
        siblings: vec![Digest32::repeated(0x01)],
    };
    assert!(proof.compute_root().is_err());
}

/// A present account: the leaf is type-specific and its proof rebuilds a root
/// that differs from the empty one, so the presence of a leaf is visible in the
/// root rather than only in the proof's own `present` flag.
#[test]
fn a_present_node_leaf_produces_a_root_of_its_own() {
    let key = AccountKey::for_node(&NodeId::from_string("cblx1presentfixture".to_owned()));
    let present = AccountProof {
        account_key: key,
        state: AccountState::Node {
            balance_microtokens: 250_000,
            account_nonce: 8,
        },
        sibling_bitmap: [0u8; 32],
        siblings: Vec::new(),
    };
    let absent = AccountProof {
        state: AccountState::Absent,
        ..present.clone()
    };
    assert_ne!(
        present.compute_root().unwrap(),
        absent.compute_root().unwrap()
    );

    // "A present zero-balance account still has a leaf so its spend nonce
    // remains committed."
    let zero_balance = AccountProof {
        state: AccountState::Node {
            balance_microtokens: 0,
            account_nonce: 3,
        },
        ..present.clone()
    };
    assert_ne!(
        zero_balance.compute_root().unwrap(),
        absent.compute_root().unwrap()
    );
    // And the nonce is part of that commitment.
    let other_nonce = AccountProof {
        state: AccountState::Node {
            balance_microtokens: 0,
            account_nonce: 4,
        },
        ..present
    };
    assert_ne!(
        zero_balance.compute_root().unwrap(),
        other_nonce.compute_root().unwrap()
    );
}

/// Node and app account keys are domain-separated by the `0x00`/`0x01` byte,
/// and node and app leaves by their `0x10`/`0x13` tags.
#[test]
fn node_and_app_accounts_are_separated_at_both_levels() {
    let node_key = AccountKey::for_node(&NodeId::from_string("cblx1fixture".to_owned()));
    let app_key = AccountKey::for_app(&Digest32::repeated(0x77));
    assert_ne!(node_key, app_key);

    let key = node_key;
    let node_leaf = AccountState::Node {
        balance_microtokens: 1,
        account_nonce: 2,
    }
    .leaf(&key);
    let app_leaf = AccountState::App {
        balance_microtokens: 1,
        account_nonce: 2,
        lifecycle: AppLifecycle::Active,
        suspension_effective_epoch: 0,
    }
    .leaf(&key);
    assert_ne!(node_leaf, app_leaf);
}

/// Every `lifecycle_u8` value the specification does not assign is rejected,
/// and nothing is defaulted.
///
/// `ledger.md#lifecycle_u8-and-why-zero-is-not-active` reserves `0x00` and
/// assigns `0x01`, `0x02`, `0x03`. The reserved zero is the point of the test:
/// it is the byte an uninitialized record produces, and a decoder that mapped
/// it to `Active` would turn a truncated record into a serving app.
///
/// The positive direction — that the encoding this crate emits is the one the
/// document publishes — is asserted against the published `APP-0` fixture in
/// `conformance_registry.rs`, not here. This test would pass under any
/// injective mapping; that one would not.
#[test]
fn an_unassigned_lifecycle_value_is_rejected_and_never_defaulted() {
    assert_eq!(AppLifecycle::from_u8(0x01).unwrap(), AppLifecycle::Active);
    assert_eq!(AppLifecycle::from_u8(0x02).unwrap(), AppLifecycle::Grace);
    assert_eq!(
        AppLifecycle::from_u8(0x03).unwrap(),
        AppLifecycle::Suspended
    );

    assert!(
        AppLifecycle::from_u8(AppLifecycle::RESERVED_U8).is_err(),
        "0x00 is reserved and MUST NOT decode to a state"
    );
    for byte in 4u8..=255 {
        assert!(
            AppLifecycle::from_u8(byte).is_err(),
            "0x{byte:02x} is unassigned and MUST be rejected"
        );
    }

    // The textual spelling is rejected on the same terms.
    assert_eq!(AppLifecycle::parse("grace").unwrap(), AppLifecycle::Grace);
    assert!(AppLifecycle::parse("retired").is_err());
    assert!(AppLifecycle::parse("").is_err());

    // Round-tripping is total over the assigned values and only those.
    for state in [
        AppLifecycle::Active,
        AppLifecycle::Grace,
        AppLifecycle::Suspended,
    ] {
        assert_eq!(AppLifecycle::from_u8(state.as_u8()).unwrap(), state);
    }

    // The three states produce three different leaves.
    let key = AccountKey::for_app(&Digest32::repeated(0x77));
    let leaf = |lifecycle| {
        AccountState::App {
            balance_microtokens: 1,
            account_nonce: 1,
            lifecycle,
            suspension_effective_epoch: 1,
        }
        .leaf(&key)
    };
    assert_ne!(leaf(AppLifecycle::Active), leaf(AppLifecycle::Grace));
    assert_ne!(leaf(AppLifecycle::Grace), leaf(AppLifecycle::Suspended));
}

#[test]
fn the_final_comparison_is_constant_time() {
    let left = [0x42u8; 32];
    let mut right = left;
    assert!(constant_time_eq(&left, &right));
    right[31] ^= 1;
    assert!(!constant_time_eq(&left, &right));
    right[31] ^= 1;
    right[0] ^= 1;
    assert!(!constant_time_eq(&left, &right));
}
