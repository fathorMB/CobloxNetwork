---
id: DEBT-020
title: "La circolarita' di chain_id alla genesi non e' risolta da alcuna regola"
status: resolved
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
resolution_refs: ["SPEC-017","REVIEW-028","REVIEW-029"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-26
tags: ["conformance","ledger","interoperability"]
links: []
activity:
  - date: 2026-08-26
    action: "resolved: Chiuso con una regola formulata sul criterio invece che sull'elenco, ed e' la ragione per cui regge: l'implementatore ha contraddetto l'analisi della spec, che diceva che la circolarita' passa per validator_set_hash, e aveva ragione. Gli ingressi circolari dell'intestazione di genesi sono tre e non uno. Una regola scritta sull'elenco della spec sarebbe stata troppo stretta, e nessuna gate lo avrebbe rilevato: e' la sesta volta su questo progetto che un agente ha ragione contro il Lead.\n\nIl valore che eccede la chiusura e' l'elenco delle derivazioni non univoche, che la spec chiedeva \"anche se vuoto\" e che non lo era: 51 preimmagini di hash piu' 12 di firma, cinque ambiguita' chiuse dentro la spec e una lasciata aperta, diventata DEBT-028 - la terza porta sulla stessa famiglia dopo DEBT-012 e questo.\n\nVa infine registrata la classe di difetto che la chiusura ha aperto e che il progetto non aveva ancora censito. RF-002 di REVIEW-029 non stava in questo debito ne' in DEBT-021: stava nel fatto che comporli degrada binds() a un controllo di solo dominio dentro la finestra di genesi, perche' questa chiusura rende chain_id una costante nota e uguale su ogni catena proprio li'. Non e' un artefatto che insegna una forma inammissibile, ne' una pretesa rimasta indietro, ne' una grandezza sbagliata, ne' una clausola non esercitata: sono due chiusure corrette la cui composizione toglie una difesa."
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
  - schema_version: "1"
    id: "DEBT-020-EVENT-002"
    timestamp: "2026-08-26T11:22:19.649107100+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Chiuso con una regola formulata sul criterio invece che sull'elenco, ed e' la ragione per cui regge: l'implementatore ha contraddetto l'analisi della spec, che diceva che la circolarita' passa per validator_set_hash, e aveva ragione. Gli ingressi circolari dell'intestazione di genesi sono tre e non uno. Una regola scritta sull'elenco della spec sarebbe stata troppo stretta, e nessuna gate lo avrebbe rilevato: e' la sesta volta su questo progetto che un agente ha ragione contro il Lead.\n\nIl valore che eccede la chiusura e' l'elenco delle derivazioni non univoche, che la spec chiedeva \"anche se vuoto\" e che non lo era: 51 preimmagini di hash piu' 12 di firma, cinque ambiguita' chiuse dentro la spec e una lasciata aperta, diventata DEBT-028 - la terza porta sulla stessa famiglia dopo DEBT-012 e questo.\n\nVa infine registrata la classe di difetto che la chiusura ha aperto e che il progetto non aveva ancora censito. RF-002 di REVIEW-029 non stava in questo debito ne' in DEBT-021: stava nel fatto che comporli degrada binds() a un controllo di solo dominio dentro la finestra di genesi, perche' questa chiusura rende chain_id una costante nota e uguale su ogni catena proprio li'. Non e' un artefatto che insegna una forma inammissibile, ne' una pretesa rimasta indietro, ne' una grandezza sbagliata, ne' una clausola non esercitata: sono due chiusure corrette la cui composizione toglie una difesa."
    evidence_refs: ["SPEC-017", "REVIEW-028", "REVIEW-029"]
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

Chiuso da SPEC-017, accettata con REVIEW-028 e REVIEW-029 (GATE-SECREVIEW) dopo due giri di remediation. Verificato dal Lead rieseguendo: 165 test da 151 di baseline, published_artifacts.py PASS con 116 probe C10, prova in negativo su ciascuna individualmente, protocol_hashes.py senza valori mossi.

La regola: il segnaposto di genesi e' 32 byte zero, e un valore che sia ingresso di genesis_block_id - e ogni firma su un tale valore - si calcola con il segnaposto al posto di chain_id_32. Il confine e' una domanda meccanica e non un elenco, enumerata nei due versi.

GATE-TWO-DERIVATIONS: nove valori su due genesi, derivati da due strade che non condividono codice, entrambe confrontate con la tabella pubblicata di README.md e nessuna con l'altra. La convergenza e' dimostrata e non costruita, con la procedura delle righe pubblicate a zero: 0x6ba582b4 nominato da entrambe le strade prima che il documento lo contenesse. Il metodo e' diventato SKILL-004.

Il segnaposto e' provato in negativo: sotto un chain_id di 32 byte ff entrambi i valori derivati si muovono.</resolution_evidence>
</invoke>
