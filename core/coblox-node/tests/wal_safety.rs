//! Unit and safety tests for WAL persistence and crash recovery.

use tempfile::tempdir;

use coblox_core::consensus::VotePhase;
use coblox_core::hash::Digest32;
use coblox_node::wal::Wal;

#[test]
fn wal_persists_votes_and_recovers_on_restart() {
    let dir = tempdir().expect("tempdir");
    let wal_path = dir.path().join("wal.jsonl");

    let block_1 = Digest32::repeated(0x11);
    let block_2 = Digest32::repeated(0x22);
    let sig = [0x77; 64];

    // Phase 1: Write initial votes
    {
        let mut wal = Wal::open(&wal_path).expect("open wal");
        assert!(wal.can_vote(1, 0, VotePhase::Prevote, &block_1));
        wal.record_vote(1, 0, VotePhase::Prevote, &block_1, "val-000", &sig)
            .expect("record prevote");
        wal.record_vote(1, 0, VotePhase::Precommit, &block_1, "val-000", &sig)
            .expect("record precommit");

        assert_eq!(wal.count(), 2);
        assert_eq!(wal.vote_of(1, 0, VotePhase::Prevote), Some(&block_1));
        assert_eq!(wal.vote_of(1, 0, VotePhase::Precommit), Some(&block_1));

        // Attempt to vote for different block at same (height, round, phase) is rejected
        assert!(!wal.can_vote(1, 0, VotePhase::Prevote, &block_2));
        assert!(
            wal.record_vote(1, 0, VotePhase::Prevote, &block_2, "val-000", &sig)
                .is_err(),
            "equivocation must be rejected by WAL"
        );
    }

    // Phase 2: Simulate crash and restart by reopening WAL
    {
        let mut wal = Wal::open(&wal_path).expect("reopen wal after crash");
        assert_eq!(wal.count(), 2, "recovered votes count must match");
        assert_eq!(wal.vote_of(1, 0, VotePhase::Prevote), Some(&block_1));
        assert_eq!(wal.vote_of(1, 0, VotePhase::Precommit), Some(&block_1));

        // Invariant: cannot vote for block_2 in recovered (height 1, round 0)
        assert!(!wal.can_vote(1, 0, VotePhase::Prevote, &block_2));
        assert!(
            wal.record_vote(1, 0, VotePhase::Prevote, &block_2, "val-000", &sig)
                .is_err(),
            "recovered WAL must prevent equivocation across restarts"
        );

        // Can vote for new height 2
        assert!(wal.can_vote(2, 0, VotePhase::Prevote, &block_2));
        wal.record_vote(2, 0, VotePhase::Prevote, &block_2, "val-000", &sig)
            .expect("vote at height 2");
        assert_eq!(wal.count(), 3);
    }
}
