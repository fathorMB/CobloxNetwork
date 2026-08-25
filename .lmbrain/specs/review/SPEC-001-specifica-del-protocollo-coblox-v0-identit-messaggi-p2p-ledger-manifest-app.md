---
id: SPEC-001
# Note: Quote the title if it contains a colon
title: "Specifica del protocollo Coblox v0 (identità, messaggi P2P, ledger, manifest app)"
status: review
kind: feature
priority: high
area: core
milestone: M-01
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: sol
thinking_level: extended
effort_observations:
  - timestamp: "2026-08-25"
    actor: "AGENT-001"
    observed_tier: "sol"
    recommended_tier: "sol"
    note: "Production protocol specification required five interdependent documents spanning identity cryptography, libp2p networking, BFT ledger/light-client proofs, and WASM packaging, plus cross-domain coherence verification; observed scope matches Sol."
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-001, ADR-002, ADR-003, ADR-004]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [documentation, identity, p2p, ledger]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "set recommended_agent"
  - date: 2026-08-25
    action: "set effort"
  - date: 2026-08-25
    action: "set tags"
  - date: 2026-08-25
    action: "transitioned backlog -> ready"
  - date: 2026-08-25
    action: "transitioned ready -> working"
  - date: 2026-08-25
    action: "record effort observation"
  - date: 2026-08-25
    action: "transitioned working -> review"
---
# Specifica del protocollo Coblox v0

## Objective

Produrre la specifica scritta e versionata del protocollo Coblox v0: identità dei nodi, wire protocol P2P, formati del ledger e manifest delle app. È il documento fondante da cui dipendono core, ledger e runtime: deve essere abbastanza preciso da permettere a un implementatore di scrivere codice senza inventare formati.

## Context

Le decisioni architetturali sono fissate in [ADR-001] (federazione BFT), [ADR-002] (challenge crittografici), [ADR-003] (core Rust), [ADR-004] (WASM/WASI). Il repository non contiene ancora codice: questa spec produce documenti in `docs/protocol/`. I formati del ledger vanno definiti in coordinamento con il dominio di AGENT-002 (che li implementerà in M-02): dove esiste incertezza, marcare la sezione come `DRAFT` con le alternative, invece di decidere in silenzio.

## Scope
### Included
- `docs/protocol/identity.md` — identità dei nodi: coppia di chiavi Ed25519, derivazione del node ID, formato dei certificati/firme, procedura di enrollment con il costo di ingresso anti-Sybil (proof-of-work una tantum alla registrazione: parametri e verifica).
- `docs/protocol/wire.md` — trasporto e messaggi P2P su libp2p: protocolli/stream utilizzati (discovery, gossip, challenge, sync ledger), envelope firmato, versioning dei messaggi, serializzazione scelta e motivata.
- `docs/protocol/ledger.md` — formato di blocchi e transazioni: tipi `mint` (reddito di esistenza, compenso lavoro), `burn` (hosting, abbonamento), `challenge-evidence`; struttura Merkle per le prove dei light client; cosa firma un validatore.
- `docs/protocol/app-manifest.md` — manifest delle app WASM: capability richieste, limiti di risorse, prezzo in token, formato di distribuzione del modulo.
- `docs/protocol/README.md` — indice, glossario, convenzioni di versioning del protocollo.

### Excluded
- Implementazione in codice di qualsiasi parte (M-01/M-02 successive).
- Algoritmo di elezione/rotazione dei validatori (spec dedicata in M-02, qui solo il segnaposto e i vincoli).
- Parametri economici numerici definitivi (arrivano dal simulatore, [ADR-005]).

## Existing-project analysis

Repository vuoto salvo `.lmbrain/`. Nessun vincolo di codice esistente; i vincoli sono gli ADR accettati e le esclusioni permanenti di [[PROJECT]] (nessuna convertibilità monetaria, nessun trasferimento utente→utente in v0).

## Technical proposal

Documenti Markdown in `docs/protocol/`, in inglese (saranno la referenza pubblica per sviluppatori terzi), con esempi serializzati per ogni formato. Ogni scelta non ovvia (curva crittografica, formato di serializzazione, schema dei topic gossip) deve citare l'alternativa scartata in una riga. Consultare la documentazione ufficiale corrente di libp2p per i nomi di protocollo e le capacità reali (NAT traversal, relay) prima di specificare il trasporto.

## Files and areas involved

- `docs/protocol/README.md`, `identity.md`, `wire.md`, `ledger.md`, `app-manifest.md` (nuovi)

## Acceptance criteria
- [x] I cinque documenti esistono e coprono le sezioni elencate nello scope, senza sezioni vuote non marcate `DRAFT`.
- [x] Ogni formato (messaggi, transazioni, manifest) ha almeno un esempio concreto serializzato.
- [x] L'enrollment anti-Sybil è specificato: costo, verifica, e perché non è aggirabile creando N identità gratis.
- [x] Le transazioni mint/burn coprono tutti i flussi economici di [ADR-005] (reddito di esistenza, compenso lavoro, spesa hosting, spesa abbonamento) senza permettere trasferimenti diretti utente→utente.
- [x] Un light client può verificare il proprio saldo con le sole informazioni specificate (prova Merkle + firme dei validatori): il percorso di verifica è descritto passo-passo.
- [x] Le sezioni marcate `DRAFT` elencano le alternative aperte e chi deve decidere.

## Implementation plan
1. Studiare gli ADR e la documentazione libp2p corrente; fissare le convenzioni (serializzazione, versioning).
2. Scrivere `identity.md` e `wire.md` (dominio proprio).
3. Scrivere `ledger.md` e `app-manifest.md` marcando `DRAFT` i punti che appartengono ai domini di AGENT-002/AGENT-003.
4. Passata finale di coerenza incrociata + esempi serializzati.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-COHERENCE | kind=manual | owner=agent | phase=before-submit | evidence=artifact | Verifica incrociata: ogni tipo di messaggio citato in wire.md ha il formato definito, ogni transazione in ledger.md copre un flusso di ADR-005, nessun riferimento pendente tra i documenti.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=observation | Il Lead ha richiesto e ottenuto una review di sicurezza di AGENT-007 sulla specifica (identità, enrollment, verifica light client) prima della chiusura.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- Rischio: specificare il trasporto oltre le capacità reali di libp2p su reti mobili → mitigazione: verificare la documentazione corrente prima di fissare i protocolli.
- Aperto: formato di serializzazione (candidati: protobuf, CBOR/DAG-CBOR, borsh) — l'implementatore propone con motivazione.
- Aperto: parametri del proof-of-work di enrollment — proposti qui, tarati poi con AGENT-007.

## Triage del Lead sui finding di [REVIEW-002] (2026-08-25)

`GATE-SECREVIEW` non è superato: 18 finding, 8 gravi. Il Lead ha verificato indipendentemente RF-002 e lo conferma (con potere di voto totale 101 le due regole danno 67 e 68; due quorum da 67 si sovrappongono in 33 unità, pari agli `f` bizantini tollerati, quindi due certificati in conflitto possono essere entrambi validi).

L'operatore ha deciso che **la posizione del progetto sull'anti-Sybil si stabilisce dopo il threat model** ([SPEC-004], avviata). Di conseguenza la seconda remediation si divide in due lotti.

### Lotto A — rimediabile subito, indipendente dalla decisione anti-Sybil

RF-001 (equazione di verifica Ed25519 da scrivere esplicitamente), RF-002 (formula di quorum unica in aritmetica intera nei tre punti, con fixture al confine), RF-003 (ancoraggio di soggettività debole: checkpoint recente nei trust anchor, non regressione dell'altezza fidata, e correzione della frase che implica l'autosufficienza dei sette passi), RF-004 limitatamente a (a) e (b) — proof of possession della chiave di consenso e rimozione dell'autocontraddizione su chiave enrollata sì/no, RF-006 (legare la prova di enrollment a un `recent_block_id` finalizzato per impedirne la precomputazione), RF-008 (definire le otto preimmagini di hash mancanti; `policy_hash` è la più urgente perché senza di essa nessuno fuori dai validatori può ricalcolare un mint, e cade la promessa "verificabile in tempo reale"), RF-009, RF-010, RF-011, RF-012, RF-014, RF-016, RF-017 (il "saldo dell'app" di [ADR-006] non esiste ancora nel ledger — divergenza sfuggita a [REVIEW-001]), RF-018.

### Lotto B — subordinato a decisioni ancora aperte

- **RF-005** (il PoW SHA-256 non regge l'anti-Sybil su hardware commodity) e **RF-007** (limite normativo `k < 1` sul `publisher_reward`): attendono l'esito di [SPEC-004] e la conseguente decisione dell'operatore. RF-007 è tecnicamente scrivibile subito, ma il valore di `k` e la sua giustificazione dipendono dal quadro anti-Sybil.
- **RF-004(c)** (una revoca finalizzata deve forzare la transizione del validator set), **RF-013** (randomness delle challenge verificabile, es. commit-reveal) e **RF-015** (il consenso alle capability è incompatibile con host headless assegnati dal protocollo): toccano scelte di prodotto e vanno istruite dal threat model prima di essere fissate nel protocollo.

### Conseguenza sulla milestone

SPEC-001 **non può passare a `done`** finché il Lotto A non è rimediato e `GATE-SECREVIEW` non è ri-attestato. Il Lotto B potrà richiedere un ADR dedicato e, plausibilmente, una SPEC di aggiornamento del protocollo in M-02 anziché una terza remediation qui.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence

### Changes made

- Defined the Coblox v0 common canonical representation (RFC 8785 JCS),
  cryptographic domains, identifiers, limits, and version-negotiation rules.
- Specified Ed25519 identity/Peer-ID binding, signed enrollment certificates,
  exact one-time proof-of-work input/target verification, cost model, replay
  resistance, revocation, and the limits of proof of work as a Sybil defense.
- Specified the libp2p transport/discovery/NAT stack and the complete 12-message
  Coblox application catalog with framing, signed envelopes, schemas, replay
  handling, error codes, gossip validation, and backpressure.
- Specified ledger transactions, blocks, Ed25519 quorum certificates,
  validator-set continuity, mint/burn invariants covering all ADR-005 flows, and
  challenge evidence. Direct user-to-user transfer is not representable.
- Specified a depth-256 sparse Merkle account tree, canonical compressed proofs,
  and a nine-step light-client procedure anchored by a recent external weak-
  subjectivity checkpoint with persisted-height non-regression.
- Specified the signed WASM manifest, capability sandbox, resource limits,
  pricing, deterministic `.cobloxapp` byte layout, installation validation, and
  distribution integrity.
- Remediated REVIEW-001 against accepted ADR-006: added the
  `publisher_reward` mint with deterministic active-subscription eligibility,
  added signed desired-replica requirements to the manifest, and made hosting
  prices exclusively validator-governed protocol parameters.
- Remediated the authorized Lotto A of REVIEW-002: fixed one strict weighted-
  power quorum predicate and boundary fixtures; made Ed25519/ZIP-215 acceptance
  consensus-exact; bound every signature and core identifier to a genesis-
  derived chain ID; added consensus-key proof of possession; made enrollment
  PoW recent-block-bound; defined and fixture-tested all missing hash preimages
  and their retrievability; canonicalized Peer IDs/protobuf keys; made ledger
  ordering nonce-aware; bounded replay state fail-closed; required exact-height
  fresh balance responses; rejected redundant sparse-Merkle siblings; added
  ADR-006 app escrow/funding/suspension state; and specified DNS pinning against
  rebinding across redirects.
- Remediated the authorized Lotto B of REVIEW-002 under accepted ADR-007.
  RF-005: replaced SHA-256 with Argon2id (RFC 9106, version `0x13`) as the
  enrollment floor, with a deterministically derived 16-byte salt, the nonce in
  the password, the difficulty range recalibrated from 18–40 to 2–6, the cost
  profile carried in the signed parameter set and echoed in the request, a
  mandatory validation order that evaluates the memory-hard step **last**, and
  explicit per-key/global concurrency and queue bounds because ordering alone is
  necessary but not sufficient. Closed SEC-REQ-12 by stating that v0 does not
  distinguish `N` emulated nodes on one host from `N` real devices, together
  with the one-time-cost-versus-perpetual-flow limit and the statement that
  containment is economic rather than cryptographic.
  RF-007: made the creator share a validity rule —
  `amount * kd <= kn * counted_subscription_burn_microtokens` with `kn < kd`
  enforced when the reward policy document is accepted — added
  `counted_subscription_burn_microtokens` to the mint so the constraint is
  checkable from the transaction, and declared the residual reputation-buying
  channel the cap does not close.
  RF-004(c): a finalized revocation of a sitting validator now invalidates any
  set or block that still contains it at or after `effective_height`, gated by a
  new `min_revocation_effective_delay_blocks` consensus parameter, with the
  safety-over-liveness stall declared. No header field was added: forcing the
  set transition makes the revocation visible through the continuity path the
  light client already verifies.
  RF-013: added the `challenge_commitment` transaction kind, `randomness_source`
  / `issuer_commitment` / `issuer_signature` on the request, `issuer_reveal` on
  the evidence, and the registry preimages for `issuer_commitment` and
  `challenge_randomness`, so any observer recomputes the randomness; the
  commitment must be finalized strictly below the beacon height, and the
  proposer-side grinding residual is declared.
  RF-015: added `HostAcceptancePolicy` and split installation step 7 into two
  disjoint consent paths — operator grant for voluntary installation, automatic
  policy evaluation for protocol assignment, where outside the policy is a
  refusal and never a silent grant.
- Reflected the two ADR-007 consequences the formats touch: existence income is
  a share of a per-epoch capped fund (`existence_fund_microtokens_per_epoch`,
  integer division by a committed `eligible_node_count`, remainder not minted)
  rather than a fixed per-node amount, so a Sybil fleet can dilute but not
  increase emission; and validator eligibility is required to be anchored to
  finalized storage/compute work and never to uptime alone. The election
  algorithm itself was left untouched as M-02 work under DEBT-005.
- Expanded beyond the compact context pack only for direct requirements:
  ADR-005 was read in full for economic-flow reconciliation, and current
  official libp2p specifications were consulted for Identify, Kademlia,
  AutoNAT, Circuit Relay v2, DCUtR, GossipSub, and hole punching.
- For REVIEW-002 direct verification, read the complete security review and all
  five protocol documents and reconciled the assigned findings against RFC 8032,
  ZIP-215, the official libp2p Peer ID specification, and the CometBFT light-
  client trust model.
- For Lotto B, read ADR-007 in full and threat-model sections 6.2, 6.3, 7.4 and
  the section 9 SEC-REQ table, and consulted the current official RFC 9106 text
  for the Argon2id parameter profiles rather than relying on recall. The RFC's
  first recommended option (2 GiB, `t=1`, `p=4`) is confirmed unusable on
  Android and the second (64 MiB, `t=3`, `p=4`) is adopted as the starting
  profile, with the RFC's own bounds (`m >= 8p`, 128-bit salt, 256-bit tag)
  written into the governed document as validity constraints.

### Files changed

- `docs/protocol/README.md`
- `docs/protocol/identity.md`
- `docs/protocol/wire.md`
- `docs/protocol/ledger.md`
- `docs/protocol/app-manifest.md`
- `.lmbrain/specs/review/SPEC-001-specifica-del-protocollo-coblox-v0-identit-messaggi-p2p-ledger-manifest-app.md` (implementation evidence only; lifecycle unchanged)

### Verification performed

- Parsed every fenced canonical JSON example and verified compact sorted-key
  serialization; checked exactly five deliverable files, non-empty sections,
  and local document targets.
- Reconciled the complete wire enum against message schema headings, all ADR-005
  flows plus ADR-006 app escrow funding, absence of a direct transfer transaction
  kind, anti-Sybil cost/recent-block binding, the nine light-client steps, all local anchors, and ownership
  in every `DRAFT` section.
- Reconciled RF-001/RF-002/RF-003 against ADR-006: verified publisher-reward
  schema/evidence/uniqueness, replica bounds and host-selection ownership, the
  absence of publisher-declared hosting unit prices, and hosting burn binding
  to a signed protocol rate card.
- Reviewed official current libp2p primary sources; the v0 baseline deliberately
  uses AutoNAT v1 while treating AutoNAT v2 as optional because its official
  specification is still a working draft.
- Executed targeted Lotto A checks for six strict quorum boundaries, all nine
  registered hash vectors, the canonical Peer ID/public-key/node-ID fixture,
  canonical JSON, local links/anchors, nonce-aware ordering requirements, chain
  binding, bounded replay behavior, sparse-proof minimality, app escrow state,
  DNS pinning, and absence of the reserved Lotto B mechanisms.
- For Lotto B, recomputed **all eleven** hash-registry vectors from their
  preimage definitions — the eight pre-existing ones plus the two new challenge
  derivations and the wire-form randomness — and compared all 32 digest bytes
  against the README table. The four vectors whose preimages Lotto B did not
  touch (`hosting_rate_card_hash`, `object_id`, `input_hash`, `response_hash`)
  reproduce their previously published values exactly, which validates the
  canonicalizer used to regenerate the four that did change.
- Re-verified the Lotto A properties that Lotto B could have regressed: the
  single integer quorum formula and its `V=101` boundary row are intact, the
  ZIP-215 cofactored equation and the ban on the cofactorless one are intact,
  and the `node_id` of the identity fixture still derives from its public key by
  the documented rule.
- Re-parsed every canonical JSON example after the edits, including the four
  regenerated lines (enrollment request, existence mint, publisher mint,
  challenge evidence), the rewritten wire challenge request, and the new
  challenge-commitment example: 20 examples, all byte-identical to the JCS of
  their own parse. Re-checked all internal Markdown targets and anchors,
  including the two new ones.
- `GATE-SECREVIEW` is not an implementer claim. Owner: AGENT-LEAD; scheduled
  for re-review of this remediation and before `spec_done`; requested reviewer:
  AGENT-007.

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

```text
$ python -  # parse examples; check five files, headings, and local targets
PASS files=5 json_examples=16 empty_sections=0 missing_local_files=0
$ python -  # GATE-COHERENCE semantic cross-document verifier
PASS messages=12 economic_flows=4 light_client_steps=7 draft_owners=4 canonical_json_examples=16 local_anchors=ok
$ lmbrain_validate
PASS unique_ids=true; lifecycle/contract errors=0 (informational workspace diagnostics only)
$ python -  # REVIEW-001 targeted ADR-006 remediation verifier
PASS json_examples=17 local_anchors=ok RF-001=pass RF-002=pass RF-003=pass
$ python -  # REVIEW-002 Lotto A conformance verifier
PASS files=5 json_examples=18 quorum_boundaries=6 peer_id=ok node_id=ok hash_vectors=9 lotto_b_guard=ok
$ python -  # local Markdown target and anchor verifier
PASS local_links_and_anchors=ok
$ lmbrain_validate
PASS unique_ids=true; lifecycle/contract errors=0; one unrelated SPEC-002 verification-policy warning and informational tag diagnostics only
$ python lottob_verify.py   # REVIEW-002 Lotto B conformance verifier
hash_vectors: 11 recomputed, 11 match, 0 mismatch
randomness_wire_b64url: jOvkrYkL1B6MN7h62XatkrjvNaoyhMRB2GaRz9qtiNc in_doc
canonical_json_examples: 20 checked, 0 non-canonical
internal_links: 22 checked, 0 broken
node_id_derivation: ok cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq
quorum_predicate_intact: ok
cofactorless_ban_intact: ok
RF-005 argon2id primitive          pass
RF-005 pow checked last            pass
RF-005 difficulty 2-6              pass
SEC-REQ-12 declaration             pass
RF-007 k cap validity rule         pass
RF-004c revocation transition      pass
RF-013 commitment tx               pass
RF-013 reveal + recompute          pass
RF-013 two issuers per epoch       pass
RF-015 host policy                 pass
RF-015 refuse not grant            pass
ADR-007 capped existence fund      pass
ADR-007 eligibility not uptime     pass
lotto_b_markers: 13/13 pass
$ lmbrain_validate
PASS unique_ids=true; lifecycle/contract errors=0; 11 informational unknown-spec-tag diagnostics only (SPEC-001/002/003/004 vocabulary, unrelated to this change)
```

Recomputed hash-registry values after the Lotto B schema changes, all confirmed
against the README table by the run above:

```text
enrollment_request_hash   ER-0  sha256:cb1245f681d732aba57064face8872cd2104a185916ff1f0ac2d2e0651e7fb7f
parameter_set_hash        PD-0  sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
policy_hash               PD-0  sha256:1a4139ed0204a94efd654d324a859af913a351dea191a6c6839f8fddeee17075
consensus_parameters_hash PD-0  sha256:1a947f60bada6a4974ae55411f404216ffd4093ebf5add0ed34cb95ba20c6a92
issuer_commitment         CMT-0 sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5
challenge_randomness      RND-0 sha256:8cebe4ad890bd41e8c37b87ad976ad92b8ef35aa3284c441d86691cfdaad88d7
request_hash              REQ-0 sha256:8beb98273d89ed31dd62803506e6739fc83ccf3bbca9c20d1028b998fa033360
unchanged (canonicalizer control): hosting_rate_card_hash, object_id,
input_hash, response_hash all reproduce their previously published digests
```

### Deviations from the specification

None within the authorized Lotto A. Final numeric economic values, launch enrollment difficulty, and validator
election remain explicitly `DRAFT` as required by the scope exclusions; every
draft lists bounded alternatives and its decision owner. No quality-policy
exception was used. ADR-006 was accepted after initial implementation and has
now been incorporated through REVIEW-001 remediation. REVIEW-002 Lotto B
(RF-005, RF-007, RF-004(c), RF-013, RF-015) is now remediated under accepted
ADR-007. The acceptance criteria and spec lifecycle state were not changed.

Three deliberate departures from the literal remediation text, each an addition
rather than a contradiction, flagged for the Lead:

1. **Ordering the proof of work last is not sufficient on its own**, so the
   specification also requires bounded concurrency for the memory-hard step. An
   attacker holding a key can build a request that passes every cheap check and
   fails only the Argon2id evaluation, so ordering alone still lets one anonymous
   peer spend 64 MiB and hundreds of milliseconds of a validator per request.
   ADR-007 and RF-005 both stop at the ordering; implementing only that would
   have left the denial-of-service vector open.
2. **Argon2id cost parameters are governed alongside `difficulty_bits`.** With a
   memory-hard primitive, the per-evaluation cost and the expected number of
   evaluations are independent, and a 2–6 bit range gives governance only five
   settings spanning a factor of sixteen. Carrying `memory_kib`, `iterations`,
   `lanes`, and `tag_length_bytes` in the signed parameter set makes the real
   knob governable and bounded rather than frozen at one profile.
3. **RF-004(c) was closed without adding a header field.** The finding offered a
   `BlockHeader` revocation commitment or set re-commitment; forcing the set
   transition already re-commits the set, so the light client sees the change
   through the continuity and key-binding checks it already performs. This costs
   nothing on the wire, at the price of a declared safety-over-liveness stall if
   no compliant successor set is committed within the delay window.

### Handoff status
- [x] Ready for Project Lead review
