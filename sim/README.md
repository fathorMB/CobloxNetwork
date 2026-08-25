# Coblox economic simulator

The agent-based model behind [SPEC-007]: it fixes `alpha`, the shape of the
existence-income fund, the value `X` of [ADR-007], and the twenty-two election
and eligibility parameters that [SPEC-006] deliberately left symbolic.

## Running it

Python 3.11 or newer. No dependencies, no virtual environment, no build step.

```sh
cd sim
python -m coblox_sim            # the full report
python -m coblox_sim gates      # only the two before-submit gates
python -m unittest discover -s tests -v
```

The process exits non-zero if either gate fails.

## Why it lives here and not in the Cargo workspace

The root `Cargo.toml` and everything under `core/` are off limits to this spec —
[SPEC-008] is being implemented there in parallel — and adding a crate would mean
editing the workspace manifest. The simulator is also not shipped: it produces
numbers for a decision, it is not part of the node. `sim/` at the repository root
keeps it outside the node build path entirely, which is what [SPEC-007] asked
for, and matches the `sim/` entry already declared in the AGENT-002 profile.

Python was chosen over a standalone Rust crate for the same reason: a model whose
whole purpose is to be re-run and argued with by someone else should not need a
toolchain to run.

## Determinism

Every draw is `SHA-256(seed | stream | index)`, never `random`. That makes the
figures reproducible across Python versions, platforms, and interpreter builds,
not merely within one process. The seed is printed at the top of every run.

The election model uses the protocol's own preimages — `election_ticket` is the
tagged SHA-256 that `ledger.md` specifies — so the candidate ordering is the
protocol's ordering rather than a stand-in.

## What each module is

| File | What it holds |
| --- | --- |
| `params.py` | The parameter containers and the **constraint block** of `ledger.md`, evaluated rule by rule with the text of each rule attached |
| `emission.py` | Mint accounting: the capped existence fund (`F // E`, remainder never minted), work compensation, publisher reward, and the reputation margin of `threat-model.md` §6.3 |
| `election.py` | The election derivation: retain, commit, fill pool, seed, rank, fill under the cap, contraction floor, minimum set size, cooldown |
| `population.py` | Deterministic synthetic populations with a heavy-tailed contribution distribution |
| `recommended.py` | The recommended combination and the assumptions behind it |
| `scenarios.py` | The experiments |
| `__main__.py` | The report renderer |

## What it does not model

Declared so nobody reads more into a figure than is in it.

- **No network layer.** No blocks, no signatures, no propagation, no partitions.
  It models the *derivation* and the *arithmetic*, which is what the parameters
  are tuned against.
- **No price discovery.** Subscription and hosting prices are inputs, not
  outputs; the report sweeps them instead of predicting them.
- **No behavioural model.** Nodes do not decide whether to stand for election
  based on their income; willingness is a swept parameter.
- **The block interval is an assumption** (5 s), declared in `recommended.py`,
  because no protocol document fixes one. Change it and the block-count
  parameters must be rescaled and the constraint block re-run.
- **The attacker in AT-10 configuration 1 is modelled with reduced pools.** The
  measured quantity is the ratio `N/H` and the grinding delta, neither of which
  depends on absolute pool size.

## Changing a value

Edit `recommended.py` and re-run. The constraint block is checked mechanically,
so a combination that does not hold together fails loudly. That is the intended
way to disagree with the recommendation.
