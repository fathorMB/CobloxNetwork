---
id: SPEC-004
# Note: Quote the title if it contains a colon
title: "Threat model iniziale della rete Coblox"
status: working
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
- [ ] Tutti gli attori elencati nello scope sono coperti con almeno uno scenario concreto ciascuno.
- [ ] I due punti caldi (elezione validatori, economia Sybil) hanno un'analisi dedicata con numeri d'ordine di grandezza, non solo qualitativa.
- [ ] La superficie di [ADR-006] è coperta: abbonati fittizi per lucrare la quota al creatore, e abuso della lista di blocco di rete.
- [ ] Ogni scenario ha severità, contromisura e stato; nessuno scenario è lasciato senza disposizione.
- [ ] I requisiti `SEC-REQ-NN` sono verificabili (un test o una review può dire pass/fail) e mappati a milestone.
- [ ] La lista dei test di attacco per M-02/M-03 è definita in modo che AGENT-002/AGENT-001 possano implementarli senza reinterpretare.
- [ ] Nessun requisito contraddice le esclusioni permanenti di [[PROJECT]] (in particolare: nessuna contromisura può introdurre convertibilità o valore monetario del token).

## Implementation plan
1. Inventario asset e attori dagli ADR e da PROJECT.md.
2. Scenari per attore, con severità e contromisure.
3. Analisi quantitativa dei due punti caldi.
4. Derivazione dei SEC-REQ e dei test di attacco; passata finale di coerenza.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-COVERAGE | kind=manual | owner=agent | phase=before-submit | evidence=artifact | Matrice attori × asset completa: ogni cella o ha uno scenario o è marcata esplicitamente non applicabile con motivo.
- [ ] GATE-LEAD-MAP | kind=manual | owner=lead | phase=before-done | evidence=observation | Il Lead ha verificato che ogni SEC-REQ è mappato a una milestone esistente della roadmap.

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
> Filled in by the specialist after completion.

### Changes made

### Files changed

### Verification performed

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

```text

```

### Deviations from the specification

### Handoff status
- [ ] Ready for Project Lead review
