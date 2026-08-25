---
id: SPEC-001
# Note: Quote the title if it contains a colon
title: "Specifica del protocollo Coblox v0 (identità, messaggi P2P, ledger, manifest app)"
status: done
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
  - date: 2026-08-25
    action: "attested verification GATE-SECREVIEW by lead"
  - date: 2026-08-25
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "dec1f4d22dd5313a77c6f2efcaefec25fefad57e61437e3337aa11d37db9891e"
    evidence_ref: "Il Lead ha richiesto e ottenuto tre review di sicurezza da AGENT-007 su identita, enrollment e verifica del light client. REVIEW-002 (changes-requested, 18 finding di cui 8 gravi), REVIEW-006 (changes-requested dopo i Lotti A e B, 15 finding chiusi su 18 ma 4 gravi residui fra cui il pavimento Argon2id azzerabile che invalidava il punto 3 di ADR-007), e infine REVIEW-007 (accepted): AGENT-007 attesta che GATE-SECREVIEW e superato. Le due contestazioni di AGENT-001 sono state confermate corrette dalla reviewer e in due casi migliori della sua stessa condizione di chiusura: il pavimento memory-hard imposto come area piu memoria minima invece di iterations>=3, che avrebbe rifiutato il profilo RFC 9106 piu forte, e lo scudo di ammissione adattivo con validazione della sorgente invece di un puzzle fisso. Restano due finding low non bloccanti, promossi a DEBT-008 per M-02. Un terzo giro di remediation e stato dichiarato esplicitamente non giustificato dalla reviewer."
    id: "SPEC-001-ATTEST-001"
    requirement_digest: "f70c43edf348535a5f1ae4d98441e9f325f5e75a54d11e49e79f700ef4a3fe91"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T02:53:08.741879800+02:00"
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
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=observation | Il Lead ha richiesto e ottenuto una review di sicurezza di AGENT-007 sulla specifica (identità, enrollment, verifica light client) prima della chiusura.

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
  Coblox application catalog (14 after the two enrollment-admission messages
  added by the [REVIEW-006] remediation) with framing, signed envelopes, schemas, replay
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

#### Remediation di [REVIEW-006] (2026-08-25) — AGENT-001

Chiusi i 4 finding high, i 2 medium e i 2 low della ri-verifica di sicurezza.

- **RF-101 — il pavimento memory-hard non è più revocabile.** I bound di RFC 9106
  non sono più presentati come vincoli di sicurezza. Il costo minimo è ora una
  **regola di validità sull'accettazione** del documento
  `enrollment_parameters`, con lo stesso schema del tetto `kn < kd`:
  `memory_kib >= 65536`, `iterations >= 1`, e `memory_kib * iterations >= 196608`
  in `u128` controllato, più `lanes` in 1–16 e `tag_length_bytes == 32`. Tabella
  di fixture al confine con cinque righe. `memory_kib` e `iterations` sono
  dichiarati parametri **di sicurezza**, il pavimento è legato al dispositivo di
  riferimento dichiarato, e la governance può alzarlo ma non abbassarlo.
- **RF-102 — la revoca è visibile al light client.** La frase «A light client
  needs no new field to see this» è rimossa e sostituita dalla spiegazione del
  perché era falsa e dall'attacco che consentiva. Il checkpoint di soggettività
  debole porta ora `revoked_validators` e `revocation_root`, e il passo 4
  dell'algoritmo light-client applica le regole 1 e 2 della transizione forzata
  con quei dati. Dichiarata la tensione fra
  `min_revocation_effective_delay_blocks` e `max_weak_subjectivity_age_ms`, con
  il vincolo normativo che il secondo non superi la durata attesa del primo.
  Nessun campo aggiunto a `BlockHeader`.
- **RF-103 — il checkpoint è specificato.** Schema normativo
  `WeakSubjectivityCheckpoint` in un unico punto, dominio di firma dedicato
  `coblox-weak-subjectivity-signature-v0`, voce nel registro delle preimmagini
  con `chain_id_32`, fixture `WSC-0` nella tabella di conformità, primitive
  `revocation_leaf`/`node`/`empty` con fixture `REVL-0` e radice vuota. Definita
  la network-release trust key: provenienza, pluralità, rotazione a due release
  sovrapposte, recupero fuori banda e limite dichiarato. Risolta la circolarità
  di `max_weak_subjectivity_age_ms`, che il passo 1 legge dal checkpoint firmato.
- **RF-104 — l'ammissione non è più negabile a costo di un core.** Inserito uno
  **scudo di ammissione** fra il passo 7 e la valutazione memory-hard, che
  diventa il passo 9. Due parti obbligatorie: un `admission_nonce` effimero
  legato al Peer ID autenticato e all'indirizzo osservato, monouso e non
  trasferibile fra validatori; e un puzzle SHA-256 a verifica costante,
  `admission_tag`, con voce nel registro e fixture `ADM-0`. Formato fissato ora
  sul filo (`enrollment_admission_request`/`_challenge`, wrapper
  `EnrollmentSubmission`, codice `invalid_admission`) perché dopo sarebbe
  incompatibile. Dichiarato che la disponibilità dell'enrollment non è una
  garanzia di protocollo, con lo stesso standard di onestà del §"Declared limits".
- **RF-105 — il grinding è quantificato.** Riscritto il limite dichiarato: cosa
  il commit-reveal chiude davvero (il proposer non colluso non può macinare),
  cosa resta aperto con il suo ordine di grandezza (10³–10⁶ `timestamp_ms` legali
  a un SHA-256 ciascuno, sotto collusione emittente-proposer), e le due riduzioni
  non prese in v0. La copertura a due emittenti è dichiarata in `wire.md` come
  **la mitigazione del grinding**, non come ridondanza, con il rinvio incrociato.
- **RF-106 — `eligible_set_root` è fissato ora.** Campo richiesto in `MintBody`
  per `existence_income`, con primitive `eligible_leaf`/`node`/`empty` sulla
  falsariga dell'albero degli abbonamenti. I validatori full, che già ricalcolano
  `E` dalla stessa evidenza finalizzata, ricalcolano anche la radice: il conteggio
  passa da asserzione a fatto falsificabile. Il limite residuo per il light client
  resta dichiarato. Esempio canonico del mint di esistenza rigenerato.
- **RF-107** — `wire.md` nomina lo stream di enrollment per primo nei limiti di
  concorrenza, con il rinvio esplicito ai bound di `identity.md`.
- **RF-108** — `HostAcceptancePolicy` è dichiarata **oggetto locale**, non di
  rete: assegnazione ottimistica, rifiuto come canale di scoperta previsto, e il
  costo di liveness dichiarato.

Contestazioni motivate a [REVIEW-006], dettagliate in *Deviations* più sotto:
la formulazione del pavimento RF-101 come area anziché come `iterations >= 3`,
e la parametrizzazione dello scudo RF-104.

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

REVIEW-006 remediation run (`verify006.py`), output reale e integrale:

```text
$ python verify006.py
PASS node_id derives from fixture public key :: cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq
PASS enrollment_request_hash ER-0 unchanged :: sha256:cb1245f681d732aba57064face8872cd2104a185916ff1f0ac2d2e0651e7fb7f
PASS parameter_set_hash PD-0 unchanged :: sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
PASS object_id fixture unchanged :: sha256:fa67b77e3e686a4b3a2022fbe81edecd3e70a43a98d7e5aee2b76fdbdbe8a78c
PASS input_hash fixture unchanged :: sha256:66810b0847d6694ce6ac99a10db2f7339b89b10d3ed7817f6d27af832a6462c9
PASS admission_tag ADM-0 reproduces :: sha256:457915b8cd8816c5fe76651bdda0578983f8e393c7e4fe0b24376ca0bca22628
PASS weak_subjectivity_checkpoint_hash WSC-0 reproduces :: sha256:2bc543a3f8e4df60735e6431a6c1fb7293ed53047e98fe2e5bc1a879f200c71e
PASS empty revocation_root H(0x33) published :: sha256:4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce
PASS REVL-0 revocation_leaf published :: sha256:7fb1f4024627c413cbf70b49a390b6d31778e667e86042864c4bed107cd52497
PASS all canonical JSON examples are JCS-identical :: 19 checked, 0 bad []
PASS internal links and anchors resolve :: 40 checked, 0 broken []
PASS quorum predicate intact
PASS quorum boundary V=101 intact
PASS cofactorless ban intact
PASS ZIP-215 cofactored equation intact
PASS speccheck outcome table intact
PASS creator-share cap validity rule intact
PASS difficulty 2-6 intact
PASS argon2id primitive intact
PASS declared limits of mechanism intact
PASS two-issuer coverage intact
PASS capped existence fund intact
PASS revocation set-validity rules intact
PASS RF-101 cost-area floor is a validity rule
PASS RF-101 memory-hardness floor
PASS RF-101 boundary fixtures 65535/65536
PASS RF-101 RFC first profile stays valid
PASS RF-101 security-not-performance parameter
PASS RF-101 old RFC-limits-as-floor text removed
PASS RF-102 false light-client claim removed
PASS RF-102 checkpoint carries revocations
PASS RF-102 parameter tension declared
PASS RF-102 light-client step 4 applies revocations
PASS RF-103 checkpoint schema normative
PASS RF-103 dedicated signature domain
PASS RF-103 trust key provenance and rotation
PASS RF-103 circularity resolved
PASS RF-104 admission shield section
PASS RF-104 memory-hard step is now step 9
PASS RF-104 adaptive difficulty
PASS RF-104 availability declared limit
PASS RF-104 wire format fixed now
PASS RF-104 admission messages in enum
PASS RF-104 ADM-1 conformance fixture
PASS RF-105 grinding quantified
PASS RF-105 coverage named as mitigation
PASS RF-106 eligible_set_root reserved
PASS RF-106 eligible tree preimages
PASS RF-107 enrollment named in wire limits
PASS RF-108 host policy publication declared
PASS no empty sections :: []

RESULT: ALL PASS
```

Contabilità degli esempi canonici, per escludere una perdita silenziosa:

```text
$ git diff --stat -- docs/protocol
 docs/protocol/README.md       | 218 ++++++++++++++++++++++++++++++++++++++----
 docs/protocol/app-manifest.md |  16 ++++
 docs/protocol/identity.md     | 120 ++++++++++++++++++++---
 docs/protocol/ledger.md       | 154 ++++++++++++++++++++++++-----
 docs/protocol/wire.md         |  88 ++++++++++++++---
 5 files changed, 531 insertions(+), 65 deletions(-)
$ git diff -U0 -- docs/protocol | grep -c '^-{"'
1
$ git diff -U0 -- docs/protocol | grep -c '^+{"'
1
$ grep -c '^```json' docs/protocol/*.md
docs/protocol/README.md:0
docs/protocol/app-manifest.md:1
docs/protocol/identity.md:2
docs/protocol/ledger.md:11
docs/protocol/wire.md:5
```

Un solo esempio rimosso e uno aggiunto: il mint di esistenza, rigenerato per
`eligible_set_root`. 19 fence e 19 esempi verificati, nessuna perdita.

```text
$ lmbrain_validate
PASS unique_ids=true; lifecycle/contract errors=0; 11 informational unknown-spec-tag diagnostics only (identity/ledger/p2p/android/tauri/ci/rust/mockups/design-system/threat-model/sybil), invariate rispetto al giro precedente
```

Fonte primaria riconsultata per RF-101, [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html):
i quattro intervalli di parametri sono il dominio della funzione (`p` "from 1 to
2^(24)-1", `m` "from 8\*p to 2^(32)-1", `t` "from 1 to 2^(32)-1", `T` "from 4 to
2^(32)-1"); le raccomandazioni sono due configurazioni intere, la prima "a
uniformly safe option" con `t=1, p=4, m=2^(21)` (2 GiB) e la seconda con
`t=3, p=4, m=2^(16)` (64 MiB). La verifica A di AGENT-007 è confermata parola per
parola sul testo primario.

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

Due contestazioni motivate a [REVIEW-006], entrambe adottate nel testo. Le
analisi di AGENT-007 restano corrette nella diagnosi; il disaccordo è sulla
contromisura.

1. **RF-101 — il pavimento non può essere `iterations >= 3`.** La chiusura
   proposta è «un parameter set con `memory_kib < 65536`, oppure
   `iterations < 3`, oppure `lanes` fuori 1–16, è invalido all'accettazione».
   Presa alla lettera quella regola **rifiuta la prima configurazione
   raccomandata da RFC 9106** — `t=1, p=4, m=2^21`, 2 GiB, che la RFC chiama "a
   uniformly safe option" — cioè la più costosa delle due, e ammette solo la
   seconda. Un pavimento che esclude il profilo più forte è un difetto, non un
   pavimento. Ho quindi imposto il costo come **area**:
   `memory_kib * iterations >= 196608` (= 65536 × 3) in `u128` controllato,
   **più** `memory_kib >= 65536` come vincolo separato. I due vincoli servono
   scopi diversi e nessuno dei due basta da solo: l'area fissa la quantità di
   lavoro, il minimo di memoria impedisce di barattarla in passate su poca
   memoria — 8 KiB × 24.576 passate ha la stessa area sulla carta ed è
   compute-bound e perfettamente parallelizzabile su GPU, cioè esattamente la
   proprietà per cui [ADR-007] ha scartato SHA-256. Sotto questa forma entrambe
   le raccomandazioni della RFC sono valide e tutto ciò che è più debole di
   entrambe è invalido. Il valore numerico del pavimento resta quello
   raccomandato dal Lead e da AGENT-007 (profilo a 64 MiB); ciò che ho cambiato è
   la **forma** del vincolo, non la taratura.
2. **RF-104 — lo scudo è della classe giusta, ma "millisecondi, una volta" non
   regge, e da solo il puzzle non basta.** Accetto la struttura: puzzle
   interattivo, legato a un nonce effimero del validatore, non precomputabile né
   riusabile, e **SHA-256 di proposito** — l'ironia è corretta e l'ho scritta nel
   documento con la sua ragione, perché una funzione memory-hard costa al
   verificatore quanto al produttore ed è per questo inutile come scudo.
   Contesto due punti. **(a) La taratura.** Perché il puzzle assorba un
   attaccante a ~10¹⁰ H/s contro una capacità del validatore di poche decine di
   valutazioni al secondo servono ~2^28 tentativi; sul dispositivo di riferimento
   dichiarato sono decine di secondi, cioè **più della proof of work che lo scudo
   dovrebbe proteggere**, e pagati una volta **per validatore** raggiunto. Un
   puzzle fisso a quella difficoltà sostituisce l'attacco, non lo ferma, e
   reintroduce nello scudo esattamente il divario CPU/GPU che [ADR-007] esiste
   per evitare. Ho quindi reso `admission_difficulty_bits` **adattivo alla
   saturazione osservata**: zero sotto la soglia dichiarata — quindi onboarding
   ordinario a costo nullo oltre il round trip — crescente solo sotto attacco, e
   con un massimo normativo ancorato al dispositivo di riferimento, che non può
   superare il tempo che lo stesso dispositivo spende nella proof of work.
   **(b) Il puzzle da solo non sposta l'asimmetria giusta.** Poiché il
   certificato richiede un quorum, il richiedente onesto deve soddisfare lo scudo
   presso ~2/3 dei validatori mentre l'attaccante ne deve saturare ~1/3: il
   moltiplicatore di quorum penalizza l'onesto più dell'attaccante. Ho perciò
   affiancato al puzzle la **validazione della sorgente** — nonce legato al Peer
   ID autenticato e all'indirizzo osservato, monouso e a vita breve — che costa
   all'onesto un round trip e all'attaccante un indirizzo realmente
   raggiungibile per ogni slot concorrente, cioè la parte dell'attacco che non
   scala con la CPU. Conseguenza che ho dichiarato invece di nascondere: sotto
   attacco sostenuto l'enrollment **degrada** e i dispositivi lenti sono i primi
   a soffrirne. Lo scudo converte una chiusura permanente e gratuita in una
   degradazione che l'attaccante paga e non può ammortizzare fra validatori; non
   rende l'enrollment sempre disponibile, e il documento ora lo dice.

Una nota di merito, non una contestazione: sul tetto `k` seguo AGENT-007 e **non**
ho trattato il residuo di reputazione come finding, perché è finanziato dal
reddito di esistenza ed è quindi già limitato dallo stesso `α` sorvegliato da
[ADR-007]. Nessuna modifica ad ADR, roadmap o PROJECT; nessun commit; nessuna
modifica a `.lmbrain/design/` né al workspace Rust; l'algoritmo di elezione dei
validatori resta intatto e di competenza di M-02 sotto [DEBT-005].

### Handoff status
- [x] Ready for Project Lead review