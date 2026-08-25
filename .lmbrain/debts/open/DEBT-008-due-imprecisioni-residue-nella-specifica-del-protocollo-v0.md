---
id: DEBT-008
title: "Due imprecisioni residue nella specifica del protocollo v0"
status: open
category: "documentation"
severity: "low"
origin_severity: "low"
area: "core"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: "SPEC-001"
origin_ref: "REVIEW-007 RF-109, RF-110"
related_specs: ["SPEC-001"]
related_reviews: ["REVIEW-007"]
related_decisions: ["ADR-007"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["documentation","argon2id","enrollment"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-008-EVENT-001"
    timestamp: "2026-08-25T02:52:57.527877100+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Promosso a debito dal Lead alla chiusura di SPEC-001, su mandato dell'operatore di salvare ogni questione emergente invece di lasciarla in una review chiusa. Senza questo, i due finding sparirebbero con l'archiviazione della spec."
    evidence_refs: []
---
# Due imprecisioni residue nella specifica del protocollo v0

## Statement

Due affermazioni della specifica del protocollo sono leggermente piu forti di quanto la regola scritta garantisca. RF-109: la frase secondo cui il pavimento Argon2id rifiuta tutto cio che e piu debole di entrambe le raccomandazioni RFC e sovradimensionata, perche la forma ad area ammette una banda stretta di configurazioni con iterations=1 sotto i 2 GiB quando memory_kib e almeno 196608, che non corrisponde a nessuna delle due raccomandazioni. RF-110: la conseguenza secondo cui lo scudo di ammissione costa all'attaccante un indirizzo raggiungibile per ogni slot concorrente vale solo se l'emissione dei nonce e conteggiata contro il limite per sorgente del primo passo; il carattere monouso del nonce limita il riuso, non il volume.

## Evidence and provenance

REVIEW-007 di AGENT-007, verifica finale di sicurezza su SPEC-001 con GATE-SECREVIEW attestato superato. Entrambi i finding sono di severita low e dichiarati non bloccanti dalla reviewer, che ha giudicato esplicitamente non giustificato un terzo giro di remediation. Sul primo punto AGENT-007 quantifica il degrado come un piccolo fattore costante e non un ordine di grandezza, incomparabile con il fattore circa 8000 che aveva motivato RF-101.

## Impact and scope boundary

Nessun impatto sulla sicurezza effettiva del protocollo v0: entrambe le proprieta sottostanti valgono, e sono le frasi che le descrivono a essere leggermente piu ampie del vero. L'impatto e sulla precisione della specifica come contratto di implementazione: chi implementa leggendo quelle due frasi potrebbe assumere una garanzia marginalmente piu forte di quella imposta dalle regole di validita. Va corretto prima che la specifica diventi riferimento pubblico per sviluppatori terzi.

## Decision log

Created by project-lead: Promosso a debito dal Lead alla chiusura di SPEC-001, su mandato dell'operatore di salvare ogni questione emergente invece di lasciarla in una review chiusa. Senza questo, i due finding sparirebbero con l'archiviazione della spec.

## Resolution criteria

Le due frasi sono riformulate in modo da corrispondere esattamente a cio che le regole di validita impongono: per RF-109 dichiarando la banda ammessa con iterations=1 sopra i 196608 KiB, oppure restringendo la regola se si decide che quella banda non debba essere ammessa; per RF-110 conteggiando l'emissione dei nonce contro il limite per sorgente, oppure indebolendo la conseguenza dichiarata. Correzione attesa in M-02, una riga ciascuna.

## Resolution evidence

