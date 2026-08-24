---
id: SPEC-004
# Note: Quote the title if it contains a colon
title: "Threat model iniziale della rete Coblox"
status: done
kind: feature
priority: high
area: security
milestone: M-01
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-007
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-001, ADR-002, ADR-004, ADR-005, ADR-006]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [threat-model, sybil, documentation]
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
    action: "transitioned working -> review"
  - date: 2026-08-25
    action: "attested verification GATE-LEAD-MAP by lead"
  - date: 2026-08-25
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "4ac86958b42fde14f6203ee9912f1814e08653374d3f19cd37cc5f7102b9513a"
    evidence_ref: "REVIEW-003: il Lead ha estratto i 24 SEC-REQ dalla tabella §9 di threat-model.md e confrontato le milestone citate con ROADMAP.md. Risultato: 24 requisiti su 24 mappati, zero riferimenti a milestone inesistenti (le valide sono M-01..M-08). Verificata inoltre in modo indipendente l'affermazione portante di §6.2.4 sulla leva alpha, che risulta corretta."
    id: "SPEC-004-ATTEST-001"
    requirement_digest: "a8d6183dd8b8dd0ac0eb714dc94802d1876ac6b18d59443d071b9ca817c0b907"
    requirement_id: "GATE-LEAD-MAP"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T01:51:29.981564300+02:00"
---
# Threat model iniziale della rete Coblox

## Objective

Produrre il threat model v1 della rete: attori ostili, superfici d'attacco, scenari concreti con severità, e i requisiti di sicurezza che ne derivano — da innestare come criteri nelle spec di M-02/M-03. "Super-sicura" smette di essere uno slogan e diventa una lista verificabile.

## Context

Le decisioni fissate creano superfici note: federazione BFT ed elezione dei validatori ([ADR-001]), challenge e accrediti ([ADR-002]), sandbox WASM ([ADR-004]), economia mint/burn ([ADR-005]), pubblicazione delle app e ricompensa al creatore ([ADR-006]). I due punti già segnalati come più delicati dal Lead: collusione nell'elezione dei validatori e resistenza Sybil dell'enrollment. Il deliverable è documentale e vive in `.lmbrain/knowledge/`.

## Scope
### Included
- `.lmbrain/knowledge/threat-model.md` con: attori (nodo egoista, botnet Sybil, validatore malevolo, cartello di validatori, sviluppatore di app ostile, osservatore di rete/privacy), asset da proteggere, e per ogni scenario: descrizione concreta dell'attacco, impatto, probabilità, contromisura esistente o richiesta, stato (mitigato / aperto / accettato).
- Analisi dedicata dei due punti caldi: (a) collusione/manipolazione dell'elezione dei validatori; (b) economia dell'attacco Sybil contro reddito di esistenza e challenge (quanto costa fingere N nodi vs quanto frutta).
- Analisi della superficie introdotta da [ADR-006]: un publisher che controlla abbonati fittizi per lucrare la quota al creatore (quanto costa fabbricare N abbonati vs quanto frutta la ricompensa), e l'abuso della lista di blocco di rete come strumento di censura o di pressione.
- Requisiti di sicurezza derivati, numerati (`SEC-REQ-NN`), formulati in modo verificabile, con il mapping verso le milestone/spec che dovranno soddisfarli.
- Lista dei test di attacco che le milestone M-02/M-03 dovranno superare (definizione, non esecuzione).

### Excluded
- Esecuzione di test o scrittura di codice.
- Threat model delle app di terze parti (arriverà con l'SDK in M-06).
- Audit di implementazioni (non esiste ancora codice).

## Existing-project analysis

Nessun codice da analizzare: le fonti sono gli ADR accettati, [[PROJECT]] e la SPEC-001 (protocollo) se già disponibile in bozza — in tal caso i finding vanno riferiti alle sue sezioni.

## Technical proposal

Struttura per scenari (stile STRIDE adattato a reti P2P) piuttosto che per componenti, così ogni riga è un attacco raccontabile e contestabile. Ogni contromisura proposta cita il costo (complessità, UX, prestazioni): un threat model che ignora i costi produce requisiti che nessuno implementa. Consultare letteratura corrente su attacchi a reti BFT federate e a sistemi proof-of-X (Sybil economics) dove il comportamento è incerto.

## Files and areas involved

- `.lmbrain/knowledge/threat-model.md` (nuovo), eventuali appendici in `.lmbrain/knowledge/`

## Acceptance criteria
- [x] Tutti gli attori elencati nello scope sono coperti con almeno uno scenario concreto ciascuno.
- [x] I due punti caldi (elezione validatori, economia Sybil) hanno un'analisi dedicata con numeri d'ordine di grandezza, non solo qualitativa.
- [x] La superficie di [ADR-006] è coperta: abbonati fittizi per lucrare la quota al creatore, e abuso della lista di blocco di rete.
- [x] Ogni scenario ha severità, contromisura e stato; nessuno scenario è lasciato senza disposizione.
- [x] I requisiti `SEC-REQ-NN` sono verificabili (un test o una review può dire pass/fail) e mappati a milestone.
- [x] La lista dei test di attacco per M-02/M-03 è definita in modo che AGENT-002/AGENT-001 possano implementarli senza reinterpretare.
- [x] Nessun requisito contraddice le esclusioni permanenti di [[PROJECT]] (in particolare: nessuna contromisura può introdurre convertibilità o valore monetario del token).

## Implementation plan
1. Inventario asset e attori dagli ADR e da PROJECT.md.
2. Scenari per attore, con severità e contromisure.
3. Analisi quantitativa dei due punti caldi.
4. Derivazione dei SEC-REQ e dei test di attacco; passata finale di coerenza.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-COVERAGE | kind=manual | owner=agent | phase=before-submit | evidence=artifact | Matrice attori × asset completa: ogni cella o ha uno scenario o è marcata esplicitamente non applicabile con motivo.
- [x] GATE-LEAD-MAP | kind=manual | owner=lead | phase=before-done | evidence=observation | Il Lead ha verificato che ogni SEC-REQ è mappato a una milestone esistente della roadmap.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- Nota di processo: il profilo AGENT-007 ha `can_implement: false` (è un reviewer puro), ma questo deliverable è il suo dominio naturale. Serve la decisione dell'operatore: concedere ad AGENT-007 l'implementazione dei soli deliverable documentali di sicurezza, oppure co-assegnare un implementatore che scrive sotto la sua direzione.
- Rischio: threat model troppo teorico → mitigazione: ogni scenario deve essere un attacco raccontabile con passi concreti.
- Aperto: soglia di rischio accettato per la devnet vs rete pubblica (proposta attesa nel documento).

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

Prodotto `.lmbrain/knowledge/threat-model.md` (v1, 14 sezioni). Contenuto:

- **13 asset** (`A-01`…`A-13`) e **7 attori** (`T-01`…`T-07`). I sei attori dello
  scope sono tutti coperti; ho aggiunto `T-07` (insider di governance) perché lo scope
  richiede l'analisi dell'abuso della lista di blocco, che non è rappresentabile con
  nessuno dei sei — non è un validatore malevolo, perché agisce **con** il quorum.
- **Matrice attori × asset** di 91 celle: 59 con scenario, 32 marcate `n/a` con motivo
  esplicito. Nessuna cella vuota (`GATE-COVERAGE`).
- **36 scenari** (`TM-01`…`TM-36`), ciascuno con passi concreti, severità, impatto,
  contromisura, costo della contromisura e stato (`mitigato`/`aperto`/`accettato`).
- **Analisi quantitative dedicate ai due punti caldi**: §6.1 elezione dei validatori,
  §6.2 economia Sybil contro il reddito di esistenza, più §6.3 per la superficie di
  [ADR-006] richiesta dallo scope.
- **§7, quadro decisionale anti-Sybil**: le quattro opzioni con costo per l'attaccante,
  costo per l'utente onesto, rischio residuo e **una riformulazione candidata della
  metrica di [[PROJECT]] per ciascuna**, più tabella di confronto. Il documento
  istruisce la decisione e non la prende.
- **24 requisiti** `SEC-REQ-01`…`SEC-REQ-24`, ciascuno con metodo di verifica
  pass/fail, milestone esistente e agente owner.
- **15 test di attacco** `AT-01`…`AT-15` con preparazione, procedura e criterio di
  superamento binario.
- §11 verifica esplicita di conformità alle esclusioni permanenti; §12 raccomandazioni
  al Lead; §13 fonti.

I 18 finding di [REVIEW-002] non sono riscritti: gli scenari che vi corrispondono li
citano per ID e aggiungono solo ciò che la vista d'insieme rende visibile
(composizione fra attacchi, attori non nominati dalle spec, superfici che stanno fra
due documenti). Le sezioni interamente nuove rispetto a [REVIEW-002] sono §5.6
(privacy dell'osservatore di rete), §5.7 (lista di blocco), §6.1, §6.2.4, §6.3 e §7.

### Files changed

- `.lmbrain/knowledge/threat-model.md` — nuovo, 1930 righe.

Nessun altro file modificato: nessun codice, nessun documento in `docs/`, nessun ADR,
nessuna modifica a `ROADMAP.md`, `STATUS.md` o `PROJECT.md`.

### Verification performed

- **`GATE-COVERAGE`** (owner=agent, before-submit, evidence=artifact): verificato
  meccanicamente che la matrice di §4 abbia 91 celle, nessuna vuota, e che ogni
  scenario citato in matrice esista come sezione e viceversa. L'artefatto è §4 del
  documento; il transcript sotto è la verifica.
- **Metadati obbligatori**: verificato che tutti e 36 gli scenari dichiarino severità,
  stato e contromisura. La prima esecuzione ha rilevato **sei scenari** (`TM-06`,
  `TM-12`, `TM-14`, `TM-16`, `TM-21`, `TM-22`) che delegavano la contromisura al solo
  rinvio a un finding di [REVIEW-002]: corretti aggiungendo la contromisura esplicita
  con il proprio costo, così che nessuno scenario richieda di aprire un altro
  documento per conoscerne la disposizione.
- **Coerenza dei riferimenti**: verificato che tutte le milestone citate nella tabella
  `SEC-REQ` esistano in `ROADMAP.md`, e che ogni `AT-NN` citato come metodo di
  verifica sia definito in dettaglio in §10.
- **Esclusioni permanenti**: §11 verifica requisito per requisito. La verifica
  sostanziale riguarda "mining/proof-of-work continuo": §6.2.2 identifica il costo
  marginale ricorrente come la difesa anti-Sybil più efficace e la **scarta
  esplicitamente** per questa esclusione, invece di aggirarla.
- **Correzione rilevata dalla verifica**: la prima stesura affermava in prosa che le
  celle `n/a` fossero sedici; il conteggio meccanico ne ha misurate 32. Prosa
  corretta.
- **Documentazione primaria consultata** per §7.5, perché il comportamento delle
  piattaforme di attestazione cambia nel tempo e non andava ricostruito a memoria:
  Play Integrity API, Android key attestation, TPM su Windows (fonti in §13). Tre
  affermazioni non confermabili da fonte primaria sono dichiarate come tali in fondo a
  §7.5 anziché essere presentate come fatti.

### Verification transcript

```text
$ cd E:/Git/CobloxNetwork/.lmbrain/knowledge

### GATE-COVERAGE — matrice attori x asset
righe asset: 13   celle totali: 91   celle vuote: 0
celle n/a con motivo: 32

### Scenari
definiti: 36  in matrice: 36  disallineati: []
privi di severita/stato/contromisura: []

### SEC-REQ, milestone, test
SEC-REQ definiti: 24
AT dettagliati:   15
milestone citate non presenti in ROADMAP: []

### Esclusioni permanenti: termini monetari nelle sezioni 9-11
198:| Convertibilità del token in denaro, exchange, ponti verso crypto | **Nessun
`SEC-REQ` la introduce.** Nessuna contromisura di questo documento prevede depositi
cauzionali in valuta, stake acquistabile, prezzi di m...
(unica occorrenza: la tabella di conformità che ne asserisce l'assenza)

righe totali: 1930
```

Esecuzione precedente, prima delle correzioni, che ha rilevato i due difetti poi
chiusi:

```text
celle n/a con motivo: 32          <-- la prosa diceva "sedici", corretta
scenari privi di severita/stato/contromisura: [TM-06 TM-12 TM-14 TM-16 TM-21 TM-22 ]
```

### Deviations from the specification

1. **Attore aggiunto.** Lo scope elenca sei attori; ne ho usati sette. `T-07` (insider
   di governance) è necessario perché lo scope richiede esplicitamente l'analisi
   dell'abuso della lista di blocco di rete, e quella superficie non appartiene a
   nessuno dei sei. Tutti e sei gli attori dello scope restano coperti.
2. **Milestone oltre M-02/M-03 nei test.** Lo scope chiede i test per M-02/M-03.
   `AT-12` parte B (custode parziale) è mappato a M-05, perché la
   proof-of-retrievability non esiste prima di quella milestone e fingere il contrario
   avrebbe prodotto un test non implementabile. Tutti gli altri restano su M-02/M-03.
3. **Nessuna scorciatoia di qualità e nessuna eccezione richiesta.** Segnalo però un
   limite onesto: i numeri di §6 sono **ordini di grandezza** dichiarati come tali, e
   §7.8 elenca le tre informazioni che oggi non ho e che sposterebbero il confronto
   fra le opzioni. Il documento non nasconde questa incertezza dietro cifre precise.
4. **La questione di processo registrata nei rischi della spec** — `can_implement` di
   AGENT-007 — risulta già risolta dall'operatore: il profilo porta l'eccezione
   approvata il 2026-08-25 per i soli deliverable documentali di sicurezza. Questo
   lavoro rientra in quel perimetro; non ho scritto codice.
5. **Nessuna review di questo documento da parte mia.** Il vincolo di profilo lo
   vieta: la review spetta al Lead.

### Handoff status
- [x] Ready for Project Lead review