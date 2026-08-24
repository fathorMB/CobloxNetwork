---
id: DEBT-007
title: "La forma del reddito di esistenza non e decisa e determina alpha"
status: open
category: "design"
severity: "high"
origin_severity: "high"
area: "core"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: "SPEC-004"
origin_ref: "TM-08"
related_specs: ["SPEC-004"]
related_reviews: ["REVIEW-003"]
related_decisions: ["ADR-005","ADR-007"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["economy","simulation","sybil"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-007-EVENT-001"
    timestamp: "2026-08-25T01:51:01.942934400+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Promosso a debito dal Lead durante la review di SPEC-004 e alla luce di ADR-007, su mandato dell'operatore di salvare tutti i debiti emergenti. E il ponte fra la decisione anti-Sybil appena presa e la taratura economica di M-02."
    evidence_refs: []
---
# La forma del reddito di esistenza non e decisa e determina alpha

## Statement

Nessun documento del progetto dichiara se il reddito di esistenza sia un importo fisso per nodo oppure un fondo a tetto per epoca ripartito fra i nodi presenti. ADR-007 adotta la seconda forma come conseguenza della decisione anti-Sybil, ma i parametri concreti (tetto del fondo, frazione alpha dell'emissione che vi transita, criterio di ripartizione) restano da fissare, e alpha determina direttamente quale quota di emissione una flotta di identita emulate puo catturare.

## Evidence and provenance

threat-model.md §6.2.4 e §7, con la raccomandazione esplicita di AGENT-007 di prendere questa decisione prima di tarare ADR-005. Calcolo verificato indipendentemente dal Lead: con alpha=1 una flotta di 10.000 identita emulate contro 1.000 nodi onesti cattura il 90,9% dell'emissione, con alpha=0,1 ne cattura il 9,1%. Requisiti derivati SEC-REQ-16 e SEC-REQ-18.

## Impact and scope boundary

Alpha e il parametro piu importante dell'economia della rete e oggi non esiste da nessuna parte. Senza una decisione esplicita il simulatore economico di M-02 non ha un modello da simulare, e la metrica di successo riformulata da ADR-007 non ha un valore di X da verificare. C'e inoltre una conseguenza di prodotto da comunicare: con il fondo a tetto il reddito di esistenza diventa una quota variabile e non un importo garantito, il che contraddice l'intuizione comune della parola reddito.

## Decision log

Created by project-lead: Promosso a debito dal Lead durante la review di SPEC-004 e alla luce di ADR-007, su mandato dell'operatore di salvare tutti i debiti emergenti. E il ponte fra la decisione anti-Sybil appena presa e la taratura economica di M-02.

## Resolution criteria

Fissati e documentati: forma del reddito di esistenza (adottato il fondo a tetto in ADR-007), tetto per epoca, criterio di ripartizione, valore iniziale di alpha e il suo intervallo di sorveglianza, e il valore X della metrica riformulata. Il rapporto del simulatore di M-02 espone le tre grandezze richieste da SEC-REQ-16. La comunicazione all'utente del fatto che il reddito e una quota variabile e presente nel design del prodotto.

## Resolution evidence

