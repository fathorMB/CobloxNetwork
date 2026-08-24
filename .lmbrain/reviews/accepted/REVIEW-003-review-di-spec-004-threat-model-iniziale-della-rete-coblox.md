---
id: REVIEW-003
# Note: Quote the title if it contains a colon
title: "Review di SPEC-004 — Threat model iniziale della rete Coblox"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-004
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-007
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-003-EVENT-001"
    timestamp: "2026-08-25T01:48:07.907551400+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-XXX"
  - schema_version: "1"
    id: "REVIEW-003-EVENT-002"
    timestamp: "2026-08-25T01:51:43.306912300+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Tutti i criteri di accettazione verificati meccanicamente dal Lead: 7 attori con 36 scenari, matrice 91 celle senza vuoti, 24 SEC-REQ tutti mappati a milestone esistenti (M-01..M-08, zero riferimenti inesistenti), 15 test di attacco, superficie di ADR-006 coperta, §11 dedicata alla conformita con le esclusioni permanenti di PROJECT. GATE-LEAD-MAP attestato. Verificata inoltre in modo indipendente l'affermazione portante di §6.2.4 sulla leva alpha, che risulta corretta e sposta la resistenza Sybil da problema crittografico irrisolvibile a parametro economico tarabile. Nessun finding a carico dell'implementatrice; le tre questioni durature sono state promosse a DEBT-005, DEBT-006 e DEBT-007."
    evidence_refs: ["SPEC-004", ".lmbrain/knowledge/threat-model.md", "ADR-007", "DEBT-005", "DEBT-006", "DEBT-007"]
    implementation_agent: "AGENT-007"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> accepted"
---
# Review

## Outcome

**Accettata.** Tutti i criteri di accettazione sono soddisfatti, verificati meccanicamente dal Lead e non sulla parola. Il documento fa quello che una spec di threat model può fare al meglio: trasforma un requisito di marketing ("super-sicura") in una lista verificabile, e istruisce una decisione di prodotto senza prenderla al posto dell'operatore.

## Acceptance-criteria compliance

| Criterio | Esito | Evidenza della verifica del Lead |
| --- | --- | --- |
| Tutti gli attori coperti con almeno uno scenario | Pass | 7 attori (`T-01`…`T-07`), 36 scenari `TM-01`…`TM-36` distribuiti sulle sette sezioni di §5 |
| Analisi dedicata dei due punti caldi, con numeri | Pass | §6.1 elezione dei validatori, §6.2 economia Sybil, entrambe quantitative |
| Ogni scenario ha severità, contromisura e stato | Pass | 36 scenari su 36 con severità nel raggio dell'intestazione; nessuno privo di disposizione |
| Superficie di [ADR-006] coperta (abbonati fittizi, abuso della lista di blocco) | Pass | §6.3 dedicata; `TM-22` per gli abbonati fittizi, `TM-33`/`TM-34` per la lista di blocco |
| `SEC-REQ-NN` verificabili e mappati a milestone esistenti | Pass | 24 requisiti in tabella §9, ciascuno con colonna "Come si verifica" e "Milestone"; nessuna milestone citata fuori da `M-01`…`M-08` |
| Test di attacco definiti per M-02/M-03 | Pass | 15 test `AT-01`…`AT-15` in §10 |
| Nessun requisito contraddice le esclusioni permanenti di [[PROJECT]] | Pass | §11 dedicata alla conformità; nessuna contromisura introduce convertibilità o valore monetario |

## Code observations

Nessun codice: deliverable documentale, 1930 righe in `.lmbrain/knowledge/threat-model.md`. I confini sono stati rispettati alla lettera — nessun file in `docs/`, nessun ADR toccato, nessuna modifica a `PROJECT.md` o `ROADMAP.md`, nessun commit.

Tre qualità che vanno oltre il mandato:

- **Riuso invece di duplicazione.** Tutti e 18 i finding di [REVIEW-002] sono riferiti per ID e non riscritti, come richiesto.
- **Autocritica sulla propria remediation.** `TM-36` documenta che l'ancoraggio di soggettività debole introdotto da lei stessa con [RF-003] sposta la fiducia sulla catena di distribuzione. Una difesa che nasconde la propria assunzione è peggiore di un rischio dichiarato: averlo scritto è esattamente il comportamento che si vuole da un reviewer di sicurezza.
- **Verifica su fonti ufficiali correnti invece che a memoria** sull'attestazione hardware, con esito che *contraddice* l'assunzione ottimistica con cui [ADR-002] aveva parcheggiato quell'opzione.

## Tests and verification

`GATE-COVERAGE` (owner: agent, before-submit): matrice attori × asset di 91 celle, 59 con scenario e 32 marcate non applicabile con motivo. Attestato dall'implementatrice, che dichiara anche di aver corretto due difetti trovati dalla propria passata meccanica.

`GATE-LEAD-MAP` (owner: lead, before-done): **verificato dal Lead**. Ho estratto i 24 `SEC-REQ` e le milestone citate nella tabella §9 e le ho confrontate con `ROADMAP.md`:

```text
milestone valide in ROADMAP: M-01 ... M-08
SEC-REQ totali=24  con milestone inesistente=0
```

Verifica indipendente aggiuntiva sull'affermazione portante di §6.2.4 — la leva `α`, cioè la frazione di emissione che passa dal canale del reddito di esistenza. Ricalcolata dal Lead:

```text
alpha=1.0 -> flotta di 10000 identita emulate contro 1000 nodi onesti cattura 90.9%
alpha=0.1 -> la stessa flotta cattura 9.1%
```

I numeri di Greta reggono. È il risultato più importante del documento: sposta la resistenza Sybil da problema crittografico irrisolvibile a parametro di design economico tarabile.

## Production quality and documentation compliance

Conforme a [[QUALITY]]. Ogni contromisura dichiara il proprio costo, come la spec imponeva: un requisito di sicurezza che ignora il costo non viene implementato, e il documento non cade in quella trappola. Nessuna eccezione di policy usata.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

Nessun finding a carico dell'implementatrice. Le questioni sollevate dal documento sono lavoro di progetto, non difetti del deliverable, e sono state promosse dal Lead a debiti di prima classe: vedi il seguito.

## Required follow-up

Il Lead ha promosso a debiti le tre questioni che sopravvivono a questa spec e che nessuna spec aperta copre:

1. **[DEBT-005]** — il set di validatori è auto-perpetuante per costruzione (`TM-18`): critico, blocca l'accumulo di storia su devnet.
2. **[DEBT-006]** — conflitto strutturale fra la quota al creatore di [ADR-006] e la privacy degli abbonati (`TM-26`/`TM-29`): è l'unica superficie del documento priva di un ADR alle spalle.
3. **[DEBT-007]** — il reddito di esistenza non ha una forma decisa (importo per nodo contro fondo a tetto ripartito), e la scelta determina `α` e quindi l'esito del simulatore di M-02.

La decisione anti-Sybil che questo documento doveva istruire è stata presa in [ADR-007], su delega esplicita dell'operatore.

## Final decision

Accettata. `GATE-LEAD-MAP` attestato dal Lead. SPEC-004 passa a `done`.
