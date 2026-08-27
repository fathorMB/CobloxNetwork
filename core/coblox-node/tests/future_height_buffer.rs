//! The buffer that holds messages of heights the node has not reached.
//!
//! The acceptance criterion of [SPEC-029] names this test by its behaviour —
//! *«un messaggio di un'altezza futura arriva presto, viene trattenuto, e viene
//! consumato quando l'altezza comincia»* — and was marked satisfied while
//! `buffer.rs` had no test at all and no test file named `FutureHeightBuffer`.
//! [REVIEW-049] RF-004.

use coblox_core::hash::{ChainId, Digest32};
use coblox_core::json::JsonObject;
use coblox_node::buffer::FutureHeightBuffer;
use coblox_node::envelope::{SignedEnvelope, fresh_nonce};
use coblox_node::signer::SigningKey;

fn envelope(marker: u64) -> SignedEnvelope {
    let key = SigningKey::from_seed(&[0x01; 32]);
    let payload = JsonObject::builder()
        .uint("marker", marker)
        .build()
        .expect("payload");
    SignedEnvelope::build_and_sign(
        &ChainId::from_digest(Digest32::repeated(0x7a)),
        "coblox-devnet-0",
        "prevote",
        "val-000",
        1_787_654_400_000,
        30_000,
        fresh_nonce().expect("system entropy"),
        payload,
        &key,
    )
    .expect("envelope")
}

fn marker_of(envelope: &SignedEnvelope) -> u64 {
    envelope.payload.uint("marker").expect("marker")
}

#[test]
fn a_message_that_arrives_early_is_held_and_then_consumed_at_its_height() {
    let mut buffer = FutureHeightBuffer::with_defaults();
    let current_height = 4;

    // 1. It arrives early.
    buffer.insert(current_height, 7, envelope(70));
    assert_eq!(buffer.len(), 1);

    // 2. It is held: the height it belongs to is not the height we are at, and
    //    draining any other height yields nothing.
    assert!(buffer.drain_height(current_height).is_empty());
    assert!(buffer.drain_height(6).is_empty());
    assert_eq!(buffer.len(), 1, "it is still held");

    // 3. It is consumed when that height begins, exactly once.
    let drained = buffer.drain_height(7);
    assert_eq!(drained.len(), 1);
    assert_eq!(marker_of(&drained[0]), 70);
    assert!(buffer.is_empty());
    assert!(buffer.drain_height(7).is_empty(), "and not a second time");
}

#[test]
fn messages_of_the_current_height_or_below_are_not_buffered() {
    let mut buffer = FutureHeightBuffer::with_defaults();
    buffer.insert(10, 10, envelope(1));
    buffer.insert(10, 9, envelope(2));
    buffer.insert(10, 0, envelope(3));
    assert!(
        buffer.is_empty(),
        "a message for a height already reached has nothing to wait for"
    );
}

#[test]
fn messages_beyond_the_lookahead_window_are_dropped() {
    let mut buffer = FutureHeightBuffer::new(3, 100);
    buffer.insert(10, 13, envelope(1)); // the last height inside the window
    buffer.insert(10, 14, envelope(2)); // one past it
    assert_eq!(buffer.len(), 1);
    assert_eq!(marker_of(&buffer.drain_height(13)[0]), 1);
    assert!(buffer.drain_height(14).is_empty());
}

#[test]
fn a_height_holds_no_more_than_its_cap() {
    let mut buffer = FutureHeightBuffer::new(20, 2);
    for marker in 0..5 {
        buffer.insert(1, 2, envelope(marker));
    }
    assert_eq!(buffer.len(), 2, "the third insert onwards is dropped");
    let drained = buffer.drain_height(2);
    assert_eq!(
        drained.iter().map(marker_of).collect::<Vec<_>>(),
        vec![0, 1],
        "the cap keeps the earliest and refuses the rest, rather than evicting"
    );
}

#[test]
fn skipped_heights_do_not_accumulate() {
    // [REVIEW-049] RF-010: `drain_height` removes one exact height, and the
    // `finalized_block` path skips heights by construction, so without
    // `prune_before` every skipped height kept its entry for the life of the
    // process. `prune_before` had no caller at all.
    let mut buffer = FutureHeightBuffer::with_defaults();
    for height in 2..=11 {
        buffer.insert(1, height, envelope(height));
    }
    assert_eq!(buffer.height_count(), 10);

    // The node jumps from height 1 to height 12 on a finalized block.
    buffer.prune_before(12);

    assert_eq!(
        buffer.height_count(),
        0,
        "ten skipped heights leave nothing"
    );
    assert!(buffer.is_empty());
}

#[test]
fn pruning_keeps_the_current_height() {
    let mut buffer = FutureHeightBuffer::with_defaults();
    buffer.insert(1, 5, envelope(50));
    buffer.insert(1, 6, envelope(60));
    buffer.prune_before(6);
    assert_eq!(buffer.height_count(), 1);
    assert_eq!(marker_of(&buffer.drain_height(6)[0]), 60);
}
