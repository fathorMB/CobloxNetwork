---
id: DEBT-005
title: "Il set di validatori e auto-perpetuante: manca la regola di elezione"
status: open
category: "security"
severity: "critical"
origin_severity: "critical"
area: "core"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: "SPEC-004"
origin_ref: "TM-18"
related_specs: ["SPEC-004","SPEC-001"]
related_reviews: ["REVIEW-003"]
related_decisions: ["ADR-001","ADR-007"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["consensus","governance","sybil"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-005-EVENT-001"
    timestamp: "2026-08-25T01:50:03.498792200+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Promosso a debito dal Lead durante la review di SPEC-004: e una questione che sopravvive alla spec che l'ha scoperta e che nessuna spec attualmente aperta copre. Registrato su mandato dell'operatore di salvare tutti i debiti emergenti."
    evidence_refs: []
---
# Il set di validatori e auto-perpetuante: manca la regola di elezione

## Statement

La regola di continuita del validator set in ledger.md autentica in modo sicuro la transizione da un set al successivo, ma non vincola in alcun modo CHI possa finire nel set successivo: il documento dichiara esplicitamente che non specifica come i membri siano eletti o ruotati. In quello spazio vuoto il set corrente e l'unico soggetto che scrive il set successivo, quindi un quorum raggiunto una sola volta puo impegnare un successore composto interamente da se stesso, all'infinito. Il light client non puo accorgersene perche la continuita e formalmente valida a ogni passo.

## Evidence and provenance

SPEC-004 scenario TM-18, con l'analisi quantitativa di threat-model.md §6.1. Citazione diretta da docs/protocol/ledger.md: "This continuity rule specifies safe authentication but not how members are elected or rotated". Requisito derivato SEC-REQ-13, indicato da AGENT-007 come uno dei tre irrinunciabili.

## Impact and scope boundary

Una rete che accumuli storia sotto questa regola mancante puo diventare permanentemente chiusa senza che nessuno se ne accorga, e la chiusura non e reversibile a posteriori perche il set insediato controlla ogni transizione futura. La roadmap colloca la rotazione automatica in M-07, il che e ragionevole per l'automazione ma non per l'invariante: l'invariante serve prima che esista storia. E inoltre intrecciato con ADR-007, che vincola l'eleggibilita a lavoro difficile da falsificare e non al solo uptime.

## Decision log

Created by project-lead: Promosso a debito dal Lead durante la review di SPEC-004: e una questione che sopravvive alla spec che l'ha scoperta e che nessuna spec attualmente aperta copre. Registrato su mandato dell'operatore di salvare tutti i debiti emergenti.

## Resolution criteria

La regola di elezione e scritta nei documenti di protocollo: deterministica a partire da casualita finalizzata, su un insieme di eleggibili calcolabile da chiunque, con tetto di rotazione per epoca e impegno nel header del blocco che ne consenta il ricalcolo a posteriori. I test di attacco AT-09 e AT-10 di SPEC-004 passano. Fino ad allora nessuna devnet deve accumulare storia che si intenda conservare.

## Resolution evidence

