---
id: DEBT-019
title: "reward_epoch non ha una regola di derivazione dal tempo"
status: open
category: "correctness"
severity: "high"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: "SPEC-013"
origin_ref: "osservazione adiacente di AGENT-007 nella valutazione di DEBT-013 e DEBT-014"
related_specs: ["SPEC-009","SPEC-011"]
related_reviews: ["REVIEW-014"]
related_decisions: ["ADR-010","ADR-011"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["ledger","economy","conformance"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-019-EVENT-001"
    timestamp: "2026-08-25T23:31:36.819325200+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead su osservazione adiacente di AGENT-007, che ha scelto di riportarla invece di crearne il debito da se, dichiarando il limite della propria istruttoria. Owner AGENT-002 perche possiede la superficie economica e del ledger, ed e l'autrice di SPEC-009 da cui questa questione discende direttamente."
    evidence_refs: []
---
# reward_epoch non ha una regola di derivazione dal tempo

## Statement

Nessun documento deriva reward_epoch da alcunche. Il campo compare nei corpi di mint, nelle foglie Merkle e in una regola di unicita, ma non esiste una regola che lo leghi a timestamp_ms, a reward_epoch_ms o ad altro. Ne segue che reward_epoch_ms_min vincola la durata dichiarata in un documento firmato e non la velocita con cui gli indici avanzano nei mint: un quorum conforme che incrementa reward_epoch a ogni blocco resta valido, e il pavimento che SPEC-009 ha introdotto per impedire che accorciare l'epoca moltiplichi l'emissione reale non morde su quel percorso.

Se la lettura e corretta, e famiglia 3 un livello sotto RF-002 di REVIEW-014: quel finding aveva vincolato la durata dichiarata, e questa e la grandezza da cui l'emissione reale dipende davvero.

## Evidence and provenance

Osservazione di AGENT-007 riportata al Lead al termine della valutazione di DEBT-013 e DEBT-014, con la propria confidenza dichiarata: ricerca documentale esaustiva su docs/protocol/ e core/coblox-core, nessuna istruttoria oltre, e la richiesta esplicita che non sia affermata come certa prima di essere attribuita a chi possiede quella superficie.

Verificato dal Lead in modo indipendente: reward_epoch compare diciannove volte fra ledger.md e README.md, e l'unico MUST che lo nomina riguarda i limiti su reward_epoch_ms, non la derivazione dell'indice. Nessuna occorrenza lo deriva.

## Impact and scope boundary

Da stabilire, ed e il lavoro. La direzione del pericolo e verso l'alto sulla velocita di avanzamento dell'indice: piu rapidamente avanza, piu fondi di esistenza vengono emessi per unita di tempo reale, che e esattamente il fallimento che ADR-010 e SPEC-009 esistono per impedire.

Va valutato anche il verso opposto, cioe un indice che non avanza affatto, perche congelerebbe l'emissione senza violare alcuna regola: e il gemello del caso che README gia dichiara invalido per reward_epoch_ms sopra il tetto.

Severita high e non critical perche nessuna rete esiste e la correzione e una regola di derivazione, non una riprogettazione; ma tocca l'emissione, che e la superficie su cui il progetto ha speso due spec e quattro giri di review.

## Decision log

Created by project-lead: Aperto dal Lead su osservazione adiacente di AGENT-007, che ha scelto di riportarla invece di crearne il debito da se, dichiarando il limite della propria istruttoria. Owner AGENT-002 perche possiede la superficie economica e del ledger, ed e l'autrice di SPEC-009 da cui questa questione discende direttamente.

## Resolution criteria

Una regola normativa che deriva reward_epoch da una grandezza che i validatori non scrivono liberamente, oppure la dimostrazione che una tale regola non e ottenibile dentro la catena, nel qual caso vale la proposizione generale che AGENT-007 ha stabilito su DEBT-013 e la chiusura ha la stessa forma: rendere l'avanzamento misurabile da fuori invece che vincolabile da dentro. Con la sua fixture di frontiera e la prova in negativo.

Da chiudere prima che una devnet emetta reddito di esistenza.

## Resolution evidence

