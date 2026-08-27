# Devnet runbook: four seed validators on one machine

This starts a four-validator Coblox devnet on a single host, watches the chain
grow, then kills a validator and restarts it.

Written by the Project Lead on 2026-08-27 under an operator-authorized corrective
takeover, because [SPEC-029] marked this criterion satisfied while no runbook
existed. Every command below was executed before this file was committed: the
transcripts at the end are real output, not illustrations.

## What this is not

This is a **devnet**, not a node deployment. It uses the deterministic
four-validator set from `coblox_node::config::devnet_4_validator_set` and derives
each signing key from `--seed-index`, so **the keys are reproducible by anyone
who reads this file**. That is deliberate for a devnet and disqualifying for
anything else.

The transport is the declared devnet subset — TCP with Noise and Yamux, plus
GossipSub — and not the full v0 interoperability baseline of
[`wire.md`](protocol/wire.md). There is no NAT traversal, no peer discovery, and
no relay: the four nodes find each other because you tell each one where the
others are.

## Prerequisites

A Rust toolchain, and the binary built once:

```bash
cargo build -p coblox-node
```

The binary lands at `target/debug/coblox-node` (`.exe` on Windows).

**On Windows, in Git Bash, export `MSYS_NO_PATHCONV=1` first.**

```bash
export MSYS_NO_PATHCONV=1
```

Without it, MSYS rewrites any argument that looks like a Unix path into a Windows
one, so `/ip4/127.0.0.1/tcp/19100` reaches the node as `C:/Program Files/Git/ip4/...`
and it exits with:

```text
Error: Protocol("invalid listen multiaddr: invalid multiaddr")
```

That is the shell, not the node. This runbook failed on its first run for exactly
this reason.

## Starting the devnet

Four terminals, one command each. The `--seed-peers` list is **the same in all
four**: it names every node including the one you are starting, and a node
ignores itself.

```bash
# terminal 1
./target/debug/coblox-node start \
  --validator-id val-000 --seed-index 0 \
  --data-dir ./data/val-000 \
  --listen-addr /ip4/127.0.0.1/tcp/19100 \
  --seed-peers /ip4/127.0.0.1/tcp/19100,/ip4/127.0.0.1/tcp/19101,/ip4/127.0.0.1/tcp/19102,/ip4/127.0.0.1/tcp/19103
```

```bash
# terminal 2
./target/debug/coblox-node start \
  --validator-id val-001 --seed-index 1 \
  --data-dir ./data/val-001 \
  --listen-addr /ip4/127.0.0.1/tcp/19101 \
  --seed-peers /ip4/127.0.0.1/tcp/19100,/ip4/127.0.0.1/tcp/19101,/ip4/127.0.0.1/tcp/19102,/ip4/127.0.0.1/tcp/19103
```

```bash
# terminal 3
./target/debug/coblox-node start \
  --validator-id val-002 --seed-index 2 \
  --data-dir ./data/val-002 \
  --listen-addr /ip4/127.0.0.1/tcp/19102 \
  --seed-peers /ip4/127.0.0.1/tcp/19100,/ip4/127.0.0.1/tcp/19101,/ip4/127.0.0.1/tcp/19102,/ip4/127.0.0.1/tcp/19103
```

```bash
# terminal 4
./target/debug/coblox-node start \
  --validator-id val-003 --seed-index 3 \
  --data-dir ./data/val-003 \
  --listen-addr /ip4/127.0.0.1/tcp/19103 \
  --seed-peers /ip4/127.0.0.1/tcp/19100,/ip4/127.0.0.1/tcp/19101,/ip4/127.0.0.1/tcp/19102,/ip4/127.0.0.1/tcp/19103
```

Add `--target-height 10` to every command if you want each node to exit once it
has finalized height 10 instead of running until you stop it. That is what the
automated test does.

**`--validator-id` and `--seed-index` must agree.** The validator ID selects
which member of the set the node claims to be; the seed index derives the key it
signs with. Start `val-000` with `--seed-index 1` and its signatures will not
verify against the set — the other three will reject every vote it sends, and the
node will look alive and be ignored.

## Watching the chain grow

Each node writes two files under its `--data-dir`:

| File | What it holds |
| --- | --- |
| `wal.jsonl` | The write-ahead log of votes. **Written and flushed before a vote is sent.** |
| `blocks.jsonl` | The finalized blocks, one per line |

So the chain height of a node is the line count of its block log:

```bash
wc -l ./data/val-000/blocks.jsonl
```

Watch all four together:

```bash
for i in 0 1 2 3; do printf "val-00%s " "$i"; wc -l < "./data/val-00$i/blocks.jsonl"; done
```

The four counts should stay within a block or two of each other. A node stuck at
a fixed height while the others advance is not participating — check that its
port is free and that its `--seed-peers` list is the same as everyone else's.

## Killing a validator and restarting it

This is the part worth doing, because it is the one the consensus depends on.

**1. Let the chain grow past a few heights**, then kill one node — terminal 4,
or:

```bash
kill -9 $(pgrep -f "validator-id val-003")
```

`kill -9` on purpose: a graceful shutdown would let the node flush anything it
was holding, and the whole point is to test what happens when it does not get
the chance.

**2. The other three keep going.** Four validators need three signatures for a
quorum, so the chain survives one loss. Watch the remaining three advance while
`val-003` stays where it died.

**3. Restart it with the same `--data-dir`.**

```bash
./target/debug/coblox-node start \
  --validator-id val-003 --seed-index 3 \
  --data-dir ./data/val-003 \
  --listen-addr /ip4/127.0.0.1/tcp/19103 \
  --seed-peers /ip4/127.0.0.1/tcp/19100,/ip4/127.0.0.1/tcp/19101,/ip4/127.0.0.1/tcp/19102,/ip4/127.0.0.1/tcp/19103
```

**The data directory is the whole point.** On restart the node re-reads
`wal.jsonl`, which maps every `(height, round, phase)` it has already voted in to
the block it voted for, and `Wal::can_vote` refuses a second, *different* vote for
any of them. A validator that came back with an empty log would sign that second
vote — **equivocating without being malicious** — and the quorum-intersection
argument that every safety proof in [ADR-018] rests on would no longer hold. Start
it with a fresh `--data-dir` and you have not restarted a validator, you have added
a faulty one.

**The restart restores the lock too, and it says so on its first line.** The
paragraph that stood here said the opposite, and it was true when it was written:
`Engine::start` built `locked: None` every time, so a validator that was locked on
a value when it died came back **unlocked** and could prevote a different value in
a later round of the same height. That is not equivocation — the WAL is right not
to call it one — but it violates the locking rule, and [REVIEW-049] RF-002 added
the part the Lead had not measured: with n=4 and f=1, one `kill -9` spent the
entire fault budget with no adversary present. The round that makes it reachable
is not hypothetical either: on this devnet heights routinely finalize at `round=1`
and `round=2`.

It is closed now. `EngineConfig` carries `locked_round` and `locked_block_id`, and
the node fills them from `wal.jsonl`: the highest round it precommitted in at the
height it is resuming **is** the round it was locked at, because Algorithm 1 lines
38-40 lock and precommit in the same step. Nothing new is written to disk — the
fact was already in the log; nothing read it back. A restart that finds a lock
prints it:

```text
Starting coblox-node validator=val-003 pid=19472
LOCK_RESTORED node=val-003 height=59 round=0 block_id=Digest32([140, 77, 243, 136, ...])
```

A restart at a height the node never precommitted in prints nothing and starts
unlocked, which is Algorithm 1's `lockedRound_p = -1`.


`wal.jsonl` is append-only. If a **complete** line in it is unreadable, the node
**fails to start** rather than skipping it: skipping would mean coming back up
without knowing what it had already signed, which is the one thing it must never
do.

There is exactly one exception, and it is the case this whole exercise is about.
A `kill -9` can land between the write and the `fsync` and leave a final line with
no newline on it — an interrupted append, not a corrupt record. A vote whose line
is incomplete never left the process, because `record_vote` returns before the
send. So the node discards that tail, truncates the file back to the last complete
record, prints what it did, and starts. Any malformed line that is **not** the last
one is still fatal: that one has no benign explanation. [REVIEW-049] RF-008.

## Cleaning up

```bash
rm -rf ./data ./data-val*.log
```

Deleting a data directory discards that validator's memory of what it signed.
Harmless between devnet runs, and never appropriate on a node that has
participated in a chain you care about.

`data/` and `data-val*.log` are in `.gitignore`, and `--data-dir` has **no
default**: the node refuses to start without one. Both are [REVIEW-049] RF-005.
Before them, the default data directory was `./data/val-000` — inside the source
tree of a public repository — and a run left signed votes untracked in the working
tree, to be cleaned by hand before every commit.

## The automated version

`core/coblox-node/tests/devnet_multiprocess.rs` does all of the above without a
human: it spawns four processes, waits for ten heights, then kills one, checks
that the remaining three advance, restarts it, and checks that all four reach a
later height. Run it with:

```bash
cargo test -p coblox-node --test devnet_multiprocess -- --nocapture --test-threads=1
```

It is the same devnet. It is not a substitute for this file: a test proves the
sequence works, and a runbook lets a person drive it.

## Transcripts

Re-executed by AGENT-001 on 2026-08-27 during the remediation of [REVIEW-049],
from the commands above and not from the automated test. The earlier transcripts
were the Lead's and were replaced because the behaviour they showed has changed:
the restart now restores the lock, and that line did not exist when they were
taken. The four nodes were started **without** `--target-height`, so they were
still running when the kill landed.

On this machine `pgrep` is not available in Git Bash, so the kill used the PID the
node prints on its first line:

```text
prima del kill:
 val-000=57 val-001=57 val-002=57 val-003=58
>>> val-003 (pid=10704) ucciso con taskkill /F <<<
dopo il kill:
 val-000=91 val-001=91 val-002=91 val-003=58
voti nel WAL di val-003 prima del riavvio: 118
```

Three validators carried the chain from 57 to 91 while the fourth stayed at the 58
it died on. Three of four is exactly the quorum, so there was no margin left: a
second loss would have stopped it.

**Restart on the same data directory:**

```text
Starting coblox-node validator=val-003 pid=19472
LOCK_RESTORED node=val-003 height=59 round=0 block_id=Digest32([140, 77, 243, 136, ...])
PUBLISH_FAILED message_type=block_proposal: InsufficientPeers
PUBLISH_FAILED message_type=prevote: InsufficientPeers
SYNC_FINALIZED node=val-003 height=59 block_id=Digest32([140, 77, 243, 136, ...])
SYNC_FINALIZED node=val-003 height=60 block_id=Digest32([66, 192, 70, 218, ...])
t+10s: val-000=150 val-001=150 val-002=150 val-003=150
t+20s: val-000=191 val-001=192 val-002=192 val-003=192
t+30s: val-000=233 val-001=233 val-002=233 val-003=233
t+40s: val-000=274 val-001=274 val-002=275 val-003=275
```

Three things in that transcript are worth naming.

**`LOCK_RESTORED`** is the line that did not exist before. val-003 had
precommitted at height 59 round 0 and died before it finalized; it came back
locked on the same block, from its own log.

**The two `PUBLISH_FAILED ... InsufficientPeers`** are normal and are not an
error being swallowed. A node that has just started has no gossip mesh yet, and
its first proposal and prevote have nowhere to go. They are printed rather than
discarded because [REVIEW-049] RF-016 found every transmission written as
`let _ = try_send(...)`, with a full channel dropping an already-durable vote in
silence.

**It caught up within ten seconds** and then stayed level. Sync answers are
bounded at eight blocks and throttled to one per requester per second: without
the throttle the catch-up burst delayed live consensus messages past their own
expiry and stalled the chain for the whole duration of the sync. That
amplification was always there — the envelope expiry check of RF-001 is what made
it visible.

**What this transcript does not show.** The node is killed while the chain is
running, but the kill is not placed in the window between a vote's `fsync` and its
transmission: that window is milliseconds wide and cannot be hit by hand. It is
exercised instead by `core/coblox-node/tests/durable_before_send.rs`, which puts
the kill on an **instruction** — `std::process::abort()` between `Wal::record_vote`
and the send, behind `COBLOX_NODE_ABORT_AFTER_WAL_SYNC` — and then checks that the
vote is in the log while no `VOTE_SENT` line was ever printed.
