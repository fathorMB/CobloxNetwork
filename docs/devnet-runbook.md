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

**What the restart does *not* restore, as of this writing: the lock.**
`ConsensusEngine` carries `locked: Option<(u64, Value)>` — `lockedValue` and
`lockedRound` of Algorithm 1 — and it is built as `None` every time
(`consensus/engine.rs:307`). The string `locked` does not appear anywhere in
`coblox-node/src`. So a validator that was locked on a value when it died comes
back **unlocked**, and nothing stops it from prevoting a different value in a
later round. That is not equivocation, and the WAL is right not to call it one:
it is a violation of the locking rule, which is a *different* safety argument, and
it is the one this runbook cannot currently demonstrate holding. The information
needed to rebuild the lock is in `wal.jsonl` — the highest round with a non-nil
precommit — but nothing reads it back. Verified by the Lead on 2026-08-27 while
executing this runbook, independently and before reading the review; [REVIEW-049]
RF-002 reached the same conclusion with an executed proof of concept, and adds the
part the Lead had not measured: **with n=4 and f=1, a single `kill -9` spends the
entire equivocation budget with no adversary present.**

And the round that makes this reachable is not hypothetical. In the Lead's own run
above, **height 1 finalizes at `round=1`** — the very first height of a healthy
four-node devnet already goes to a second round.

`wal.jsonl` is append-only and is never rewritten. If a line in it is
unreadable, the node **fails to start** rather than skipping it: skipping would
mean coming back up without knowing what it had already signed, which is the one
thing it must never do.

## Cleaning up

```bash
rm -rf ./data
```

Deleting a data directory discards that validator's memory of what it signed.
Harmless between devnet runs, and never appropriate on a node that has
participated in a chain you care about.

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

From the Lead's own run on 2026-08-27, driven from the commands above — not from
the automated test. The four nodes were started **without** `--target-height`, so
they were still running when the kill landed. An earlier attempt with
`--target-height 12` was discarded: the nodes reached 12 and exited on their own
before `kill -9` arrived, and it proved nothing.

**Growing, then `kill -9` on a live node:**

```text
prima del kill:
 val-000=51 val-001=51 val-002=51 val-003=52
>>> val-003 ucciso con kill -9 <<<
dopo il kill:
 val-000=98 val-001=99 val-002=99 val-003=52
```

Three validators carried the chain from 51 to 99 while the fourth stayed at the
52 it died on. Three of four is exactly the quorum, so there was no margin left:
a second loss would have stopped it.

**Restart on the same data directory:**

```text
voti nel WAL di val-003 prima del riavvio: 105
Starting coblox-node validator=val-003 pid=26140
SYNC_FINALIZED node=val-003 height=53 block_id=Digest32([53, 188, 149, 247, ...])
SYNC_FINALIZED node=val-003 height=54 block_id=Digest32([16, 137, 220, 84, ...])
dopo il riavvio:
 val-000=184 val-001=184 val-002=185 val-003=185
```

It came back with 105 votes already in its log, resumed at height 53 — the one
after the last it had finalized, not from genesis — and caught the other three.

**What these transcripts do not show.** The node is killed while the chain is
running, which is better than the automated test's kill at a height boundary, but
neither run observes the case that matters most: a node killed *after* it has
signed a precommit for a height and *before* that height finalizes, restarted, and
then asked to vote in a later round of the same height. That is where the WAL and
the missing lock restore would actually be exercised against each other. This
runbook can start that situation but cannot reliably *hit* it by hand, because the
window is milliseconds wide. It needs a fault injection point in the node, and
that does not exist yet.
