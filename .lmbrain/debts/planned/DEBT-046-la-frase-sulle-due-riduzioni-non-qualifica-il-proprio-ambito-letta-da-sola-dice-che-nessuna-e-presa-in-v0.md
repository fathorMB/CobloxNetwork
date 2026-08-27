---
id: DEBT-046
title: "La frase sulle due riduzioni non qualifica il proprio ambito: letta da sola dice che nessuna e' presa in v0"
status: planned
category: "documentation"
severity: "low"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-023"]
related_reviews: ["REVIEW-041"]
related_decisions: ["ADR-012"]
target_specs: ["SPEC-027"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: ["documentation","conformance","consensus"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: SPEC-027 esegue una passata di ADR-012 su tutti gli artefatti pubblicati e tocca gia' `docs/protocol/ledger.md` per i limiti dei parametri. Qualificare l'ambito di una frase e metterle una probe e' lavoro della stessa passata, sullo stesso file, con lo stesso strumento: aprire una spec propria per una riga di documentazione sarebbe sproporzionato.\n\nVa corretta anche l'esclusione dichiarata nello Scope di SPEC-027, che rimanda a DEBT-041 come dipendenza in sospeso: quella dipendenza non esiste piu'."
debt_events:
  - schema_version: "1"
    id: "DEBT-046-EVENT-001"
    timestamp: "2026-08-27T15:11:04.643144800+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto come successore invece di correggere DEBT-041 in luogo perche' il difetto cambia natura e non solo grado: da `correctness` a `documentation`, da `medium` a `low`, e da \"il documento si contraddice\" a \"una frase non dichiara il proprio ambito\". Il titolo di DEBT-041 portava la parola contraddice e sarebbe rimasto falso in ogni digest, esattamente come il numero nel titolo di DEBT-036.\n\nLa supersessione serve anche a lasciare leggibile l'errore del Lead: DEBT-041 fu aperto verificando che le due frasi esistessero, senza verificare che parlassero della stessa cosa. E' la quarta verifica sotto-specificata del Lead nella stessa giornata, e la prima che abbia prodotto un artefatto governato con un contenuto falso."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-046-EVENT-002"
    timestamp: "2026-08-27T15:11:28.096951+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "SPEC-027 esegue una passata di ADR-012 su tutti gli artefatti pubblicati e tocca gia' `docs/protocol/ledger.md` per i limiti dei parametri. Qualificare l'ambito di una frase e metterle una probe e' lavoro della stessa passata, sullo stesso file, con lo stesso strumento: aprire una spec propria per una riga di documentazione sarebbe sproporzionato.\n\nVa corretta anche l'esclusione dichiarata nello Scope di SPEC-027, che rimanda a DEBT-041 come dipendenza in sospeso: quella dipendenza non esiste piu'."
    evidence_refs: ["SPEC-027"]
---
# La frase sulle due riduzioni non qualifica il proprio ambito: letta da sola dice che nessuna e' presa in v0

## Statement

Successore di DEBT-041, che descriveva il difetto come una contraddizione e non lo era. In `docs/protocol/ledger.md`, sezione *"Challenge evidence"*, la frase "Two reductions are available and are not taken in v0" non qualifica il proprio ambito. Le due riduzioni non sono prese **per l'evidenza di sfida**, ma la seconda — derivare materiale del beacon dai `block_id` di `K` blocchi consecutivi — **e' presa per il seme dell'elezione**, dove `election_entropy_blocks` la attua. Chi legge solo quella sezione conclude che la riduzione non sia da nessuna parte in v0.

## Evidence and provenance

Verificato dal Lead il 2026-08-27 individuando la sezione che contiene ciascuna frase, che e' il controllo che era mancato all'apertura di DEBT-041. La frase "Two reductions are available and are not taken in v0" sta sotto `### Challenge evidence`. La frase "the reduction this document deferred to \"the dedicated randomness beacon\" and takes here" sta sotto `### The seed, and why the rule does not depend on it`, e riconosce esplicitamente la tensione invece di ignorarla: dichiara che il documento l'aveva rimandata e che qui la prende.

I due siti parlano quindi di due consumatori di casualita' diversi — l'assegnazione emittente/soggetto e il seme dell'elezione — e non si contraddicono. Resta vera solo la mancata qualificazione dell'ambito nel primo.

## Impact and scope boundary

Un implementatore che legga la sola sezione dell'evidenza di sfida crede che l'aggregazione su `K` blocchi non esista in v0, e non cerca `election_entropy_blocks`. Non produce divergenza di consenso, perche' nessuna regola di validita' dipende da quella frase: e' orientamento sbagliato per il lettore successivo, che e' la stessa forma di danno di una cella `n/a` posata a torto, ma senza il potere di far divergere due implementazioni. Da qui la severita' low invece della medium con cui DEBT-041 era nato.

## Decision log

Created by AGENT-LEAD: Aperto come successore invece di correggere DEBT-041 in luogo perche' il difetto cambia natura e non solo grado: da `correctness` a `documentation`, da `medium` a `low`, e da "il documento si contraddice" a "una frase non dichiara il proprio ambito". Il titolo di DEBT-041 portava la parola contraddice e sarebbe rimasto falso in ogni digest, esattamente come il numero nel titolo di DEBT-036.

La supersessione serve anche a lasciare leggibile l'errore del Lead: DEBT-041 fu aperto verificando che le due frasi esistessero, senza verificare che parlassero della stessa cosa. E' la quarta verifica sotto-specificata del Lead nella stessa giornata, e la prima che abbia prodotto un artefatto governato con un contenuto falso.

## Resolution criteria

La frase della sezione *"Challenge evidence"* qualifica il proprio ambito: dice che le due riduzioni non sono prese **per questo consumatore**, e nomina il fatto che la seconda e' presa per il seme dell'elezione, con un rimando alla sezione che la attua. Una probe pinna la forma qualificata, cosi' che togliere la qualificazione faccia fallire la passata.

## Resolution evidence

