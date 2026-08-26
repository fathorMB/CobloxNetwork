# Security Policy

## Project status

Coblox is in early development. There is no production network, no public
devnet, and no released binary. Milestone M-01 covers the protocol on paper
and the skeleton of the Rust core; the network described in the protocol
documents is not yet implemented. Nothing here is deployed anywhere that a
vulnerability could currently harm a user.

This does not make reports unwelcome. A design flaw found now costs a
paragraph to fix and would cost a migration later.

## Reporting a vulnerability

Use GitHub's private vulnerability reporting: open the **Security** tab of
this repository and choose **Report a vulnerability**. The report stays
private between you and the maintainers until a fix is published.

Please do not open a public issue for a security problem, and please do not
disclose it elsewhere before we have had a chance to respond.

There is no email address here on purpose: a published address is a spam
target and a single point of failure, and GitHub's channel gives you a
tracked, private thread instead.

### What helps

- Which document or file the problem is in. Most of the attack surface today
  is specification, not code: `docs/protocol/` and
  `.lmbrain/knowledge/threat-model.md`.
- What an attacker gains, concretely. "This lets an unelected node commit a
  ledger transition" is actionable; "this looks weak" is hard to act on.
- Whether the problem is already covered by a known limitation below.

### What to expect

This is a small project. We aim to acknowledge a report within a week and to
tell you plainly whether we consider it a real problem, a known limitation,
or out of scope. We will not leave a report unanswered.

There is no bug bounty and no payment of any kind. The project's token is
permanently non-convertible to money and is not offered as a reward.

We are glad to credit you in the fix and in the advisory unless you would
rather stay anonymous.

## Scope

**In scope.** The protocol specification, the Rust core, the node and FFI
crates, the desktop and Android shells, the build and CI configuration, and
the published design of the ledger, consensus, and token economy.

**Out of scope.** Anything requiring physical access to a device already
controlled by the attacker; findings that reduce to "a user ran malicious
software"; automated scanner output without an analysis of impact; and the
known limitations below, unless your report shows the limitation is worse
than we have described it — which is genuinely useful and welcome.

## Known limitations

We would rather write these down than have them found and reported as
surprises. Each is documented in depth in the linked artifact.

**The network is not cryptographically Sybil-resistant.** It is robust
against *forgery* — balances, signatures, double spending — but it does not
distinguish `N` emulated nodes on one host from `N` real devices. Sybil
resistance is treated as an economic property, not as a cryptographic
guarantee, and since `SPEC-009` one half of it is held by a rule rather than by
a well-chosen value. **A fleet of `N` emulated nodes cannot enlarge what the
network pays out in a reward epoch.** Existence income is a fund divided among
eligible nodes, not an amount per node; the fund has a ceiling fixed in the
genesis trust anchor and outside on-chain governance (`RewardBounds`); the one
channel that would have paid per node without an aggregate ceiling is required
by a validity rule to be zero, so a policy document that sets it positive is
rejected on acceptance rather than discouraged. What a fleet buys is a larger
**share** of a fixed fund — dilution of honest nodes, not inflation.

**This bound is per reward epoch, and not per unit of real time.** The epoch
index is paced by block height, so a validator quorum that compresses the real
cadence multiplies real issuance whatever the fleet does; see *How fast the
chain runs is measured, not enforced* below. The two limitations are adjacent
and only one of them is held by a rule.

See `ADR-007` and `ADR-010` in `.lmbrain/decisions/`. Three things are
specifically not guaranteed: enrollment availability under sustained attack,
cryptographic Sybil resistance, and independent verification of validator
eligibility.

**Owning the validator set and controlling it are different thresholds.** The
election and rotation rule is specified in
[`docs/protocol/ledger.md`](docs/protocol/ledger.md); a quorum can no longer
commit itself indefinitely, and `DEBT-005` is closed. What remains is narrower
and is stated here because it is easy to overstate the fix: the relational rule
`3 * validator_min_set_size >= 2 * V` prevents a coalition below two thirds from
*owning* the set, not from obtaining its *quorum*. A coalition can censor
selectively, drive a lawful contraction, and reach a quorum from roughly **four
ninths** of the set. The gain over the naive threshold is real and is claimed
for what it is, not for more.

**How fast the chain runs is measured, not enforced.** Every clock inside the
chain is written by the validators themselves, so no internal validity rule can
bound real time — and a rule on the distance between consecutive block
timestamps is rejected rather than merely absent, because it would oblige a set
to *write* a cadence and not to *produce* one. Block timestamps are constrained
only to increase and not to run ahead of the receiver's clock. A validator set
can therefore move the real production rate while the chain stays live and every
block stays valid.

**The two directions cost different things, do not substitute for each other,
and are not bought at the same price.** **Stretching** lengthens, in real time,
everything the protocol denominates in blocks: validator incumbency, and the
effective delay of a revocation. It requires only a **blocking third**, which
simply withholds the quorum. **Compressing** multiplies real issuance, because
the reward-epoch index is derived from block height — an epoch that may be
settled after a fixed number of blocks is settled sooner in real time when
blocks arrive sooner. But compressing requires a **quorum**, because every block
carries a quorum certificate: no minority can make a block exist. The cheaper
attack is the one on incumbency.

What v0 has is a measurement, not a prohibition. The only external clock the
protocol has is the signed weak-subjectivity checkpoint, whose `issued_at_ms` no
validator writes; a light client and the checkpoint release process each compare
the observed rate against a two-sided band fixed in the genesis distribution.

**The measurement has a declared error, and it is declared because getting this
wrong once already cost a false positive on an honest chain.** A light client
counts blocks from the checkpoint's height but counts time from the moment the
checkpoint was *produced*, which is later — so blocks produced while the
checkpoint was being released are counted without their time, and the reading is
pushed toward "too fast" by something that is not the chain. A client clock that
is behind, or a release clock that is ahead, does the same. The genesis band
therefore carries an explicit tolerance for that shortfall, and the release
process is bound by the same number. Past the tolerance a fast reading has no
innocent explanation, since nothing honest makes blocks appear; a slow reading is
indistinguishable from the client's own sync lag at any magnitude, which is why
the client reports it rather than rejecting on it — and why the cheaper of the
two attacks is the one this protocol only reports on.

The manoeuvre is not prevented. It is made visible, against a threshold declared
before anyone had a reason to argue about it — which for a defect whose severity
is its invisibility is the part that counts, and is less than the word
"prevented" would claim. The analysis is recorded as `DEBT-013`, and the
specification is `docs/protocol/README.md` and `docs/protocol/ledger.md`.

**Some dependency advisories are knowingly derogated.** The desktop shell
depends on Tauri, which on Linux sits on the unmaintained GTK3 stack. Those
advisories have no resolvable upgrade at the pinned version; each derogation
is listed individually, with its reason and its review condition, in
`apps/desktop/src-tauri/deny.toml`. They are not silent: CI fails on any
advisory that is not in that file.

## Supply chain

Every third-party GitHub Action used by CI is pinned to a commit SHA rather
than a mutable tag, and refreshed through grouped Dependabot pull requests
that are merged only against a green pipeline. Rust and npm dependencies are
built from committed lockfiles (`cargo build --locked`, `npm ci`), and both
dependency graphs -- the workspace and the desktop shell -- are gated by
`cargo-deny` for advisories and licences.

## Threat model

The project maintains a threat model at
`.lmbrain/knowledge/threat-model.md`: 43 scenarios, 26 numbered security
requirements mapped to milestones, and 15 attack tests. If you are looking
for where the sharp edges are, start there rather than here. It is written in
Italian, the project's internal working language.
