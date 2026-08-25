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
resistance is treated as an economic property governed by the fraction of
emission that flows through the existence income, not as a cryptographic
guarantee. See `ADR-007` in `.lmbrain/decisions/`. Three things are
specifically not guaranteed: enrollment availability under sustained attack,
cryptographic Sybil resistance, and independent verification of validator
eligibility.

**The validator set is self-perpetuating in the current v0 protocol.** The
protocol authenticates a validator-set transition but does not yet constrain
who may enter the next set, so a quorum reached once can commit itself
indefinitely without a light client noticing. The election rule is unwritten
and is the first work of M-02. Tracked as `DEBT-005`.

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
`.lmbrain/knowledge/threat-model.md`: 36 scenarios, 24 numbered security
requirements mapped to milestones, and 15 attack tests. If you are looking
for where the sharp edges are, start there rather than here. It is written in
Italian, the project's internal working language.
