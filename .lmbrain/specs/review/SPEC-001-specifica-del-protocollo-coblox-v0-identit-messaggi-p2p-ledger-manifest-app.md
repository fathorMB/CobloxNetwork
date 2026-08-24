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
- Specified a depth-256 sparse Merkle state tree, compressed balance proof, and
  a seven-step light-client verification procedure from genesis trust anchor to
  authenticated balance.
- Specified the signed WASM manifest, capability sandbox, resource limits,
  pricing, deterministic `.cobloxapp` byte layout, installation validation, and
  distribution integrity.
- Remediated REVIEW-001 against accepted ADR-006: added the
  `publisher_reward` mint with deterministic active-subscription eligibility,
  added signed desired-replica requirements to the manifest, and made hosting
  prices exclusively validator-governed protocol parameters.
- Expanded beyond the compact context pack only for direct requirements:
  ADR-005 was read in full for economic-flow reconciliation, and current
  official libp2p specifications were consulted for Identify, Kademlia,
  AutoNAT, Circuit Relay v2, DCUtR, GossipSub, and hole punching.

### Files changed

- `docs/protocol/README.md`
- `docs/protocol/identity.md`
- `docs/protocol/wire.md`
- `docs/protocol/ledger.md`
- `docs/protocol/app-manifest.md`
- `.lmbrain/specs/working/SPEC-001-specifica-del-protocollo-coblox-v0-identit-messaggi-p2p-ledger-manifest-app.md` (acceptance checklist and implementation evidence only)

### Verification performed

- Parsed every fenced canonical JSON example and verified compact sorted-key
  serialization; checked exactly five deliverable files, non-empty sections,
  and local document targets.
- Reconciled the complete wire enum against message schema headings, all four
  ADR-005 flows, absence of a transfer transaction kind, anti-Sybil cost and
  proof binding, the seven light-client steps, all local anchors, and ownership
  in every `DRAFT` section.
- Reconciled RF-001/RF-002/RF-003 against ADR-006: verified publisher-reward
  schema/evidence/uniqueness, replica bounds and host-selection ownership, the
  absence of publisher-declared hosting unit prices, and hosting burn binding
  to a signed protocol rate card.
- Reviewed official current libp2p primary sources; the v0 baseline deliberately
  uses AutoNAT v1 while treating AutoNAT v2 as optional because its official
  specification is still a working draft.
- `GATE-SECREVIEW` is not an implementer claim. Owner: AGENT-LEAD; scheduled
  after `spec_submit` and before `spec_done`; requested reviewer: AGENT-007.

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
```

### Deviations from the specification

None. Final numeric economic values, launch enrollment difficulty, and validator
election remain explicitly `DRAFT` as required by the scope exclusions; every
draft lists bounded alternatives and its decision owner. No quality-policy
exception was used. ADR-006 was accepted after initial implementation and has
now been incorporated through REVIEW-001 remediation without changing the
original acceptance criteria or spec lifecycle state.

### Handoff status
- [x] Ready for Project Lead review
