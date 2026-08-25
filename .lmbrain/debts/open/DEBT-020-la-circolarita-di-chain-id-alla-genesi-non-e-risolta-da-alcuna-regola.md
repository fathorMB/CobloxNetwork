---
id: DEBT-020
title: "La circolarita' di chain_id alla genesi non e' risolta da alcuna regola"
status: open
category: "correctness"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: "SPEC-013"
origin_ref: "seconda osservazione adiacente di AGENT-007: circolarita di chain_id alla genesi"
related_specs: ["SPEC-001","SPEC-008","SPEC-010"]
related_reviews: ["REVIEW-021"]
related_decisions: []
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["conformance","ledger","interoperability"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-020-EVENT-001"
    timestamp: "2026-08-25T23:32:19.627259400+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead su osservazione adiacente di AGENT-007. Owner AGENT-001 perche e l'autore del protocollo v0, di coblox-core e dell'inventario degli artefatti pubblicati, quindi conosce le tre superfici che questa questione tocca."
    evidence_refs: []
---
# La circolarita' di chain_id alla genesi non e' risolta da alcuna regola

## Statement

chain_id deriva da genesis_block_id, che deriva dall'intestazione di genesi, che contiene validator_set_hash. La catena di derivazione e circolare alla genesi e nessuna regola dice come si rompe. La fixture HASH-0 usa 32 byte a zero per chain_id, ma e una fixture e non una regola: due implementazioni possono risolvere la circolarita in modi diversi e derivare due chain_id diversi dalla stessa distribuzione di genesi.

E una divergenza di conformita su un valore che tutto il resto lega: chain_id entra in quasi tutte le preimmagini a dominio separato del protocollo, quindi due implementazioni che lo derivano diversamente non concordano su nulla.

## Evidence and provenance

Osservazione di AGENT-007 riportata al Lead al termine della valutazione di DEBT-013 e DEBT-014, come seconda delle due questioni adiacenti per cui ha chiesto un debito proprio invece di crearlo da se.

E la stessa forma di DEBT-012, chiuso da SPEC-010: un valore che entra in una preimmagine e che nessun documento fissa, invisibile a ogni test di questa base di codice perche una sola implementazione e internamente coerente. L'inventario di SPEC-010 conta le preimmagini prive di fixture pubblicata ma non verifica che ogni valore che vi entra sia derivabile in un solo modo.

## Impact and scope boundary

Difetto di interoperabilita che si manifesta solo quando esistono due implementazioni indipendenti, cioe esattamente quando il progetto avra successo. Fino ad allora nulla si rompe, il che lo rende invisibile ai test.

Colpisce inoltre l'ancora di fiducia: se chain_id di genesi e ambiguo, lo sono anche il checkpoint di soggettivita debole e ogni oggetto che il light client usa per ancorarsi, perche il passo 1 impone che chain_id sia uguale al configurato e non esiste un solo valore configurabile corretto.

## Decision log

Created by project-lead: Aperto dal Lead su osservazione adiacente di AGENT-007. Owner AGENT-001 perche e l'autore del protocollo v0, di coblox-core e dell'inventario degli artefatti pubblicati, quindi conosce le tre superfici che questa questione tocca.

## Resolution criteria

Una regola normativa che dice come la circolarita si rompe alla genesi, con la fixture pubblicata corrispondente, cosi che due implementazioni indipendenti derivino lo stesso chain_id dalla stessa distribuzione. Va valutato nella stessa occasione se altri valori entrino in una preimmagine senza essere derivabili in un solo modo, che e la generalizzazione di questa questione e la stessa che DEBT-012 poneva per le codifiche simboliche.

Da chiudere prima che una devnet accumuli storia conservabile, per la ragione di DEBT-012.

## Resolution evidence

