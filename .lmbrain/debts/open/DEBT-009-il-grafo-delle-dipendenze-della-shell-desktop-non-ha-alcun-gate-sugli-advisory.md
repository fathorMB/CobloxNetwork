---
id: DEBT-009
title: "Il grafo delle dipendenze della shell desktop non ha alcun gate sugli advisory"
status: open
category: "security"
severity: "high"
origin_severity: null
area: "build"
milestone: "M-02"
owner: "AGENT-008"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-002"]
related_reviews: []
related_decisions: ["ADR-003"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["ci","supply-chain","verification-gap","cargo-deny"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-009-EVENT-001"
    timestamp: "2026-08-25T09:42:11.335941700+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Scoperto durante la remediation di DEBT-001 e l'hardening del repository dopo il passaggio a pubblico. Registrato come debito separato e non come lavoro immediato perche la correzione ha due parti che non hanno la stessa urgenza ne lo stesso decisore: aggiungere lo step e meccanico, ma la sorte dell'advisory su glib e una decisione di deroga che va motivata per iscritto. Severita high e non medium nonostante l'advisory in se sia medium: il difetto non e la vulnerabilita trovata, e il fatto che il gate riporti verde su un grafo che non ha mai controllato, quindi la prossima vulnerabilita di qualunque severita sarebbe ugualmente invisibile. Deliberatamente senza origin_artifact: un primo tentativo lo aveva ancorato a SPEC-002/GATE-CI-GREEN e il contratto lo ha respinto perche quel gate e gia promosso da DEBT-001. Il rifiuto era corretto nel merito e non solo nella forma: questo debito non nasce da un gate non soddisfatto, nasce dal perimetro su cui il gate si pronuncia."
    evidence_refs: []
---
# Il grafo delle dipendenze della shell desktop non ha alcun gate sugli advisory

## Statement

Il crate apps/desktop/src-tauri e escluso dal workspace Cargo radice, e cargo deny check in CI opera sul grafo del workspace. Ne consegue che l'intero albero di dipendenze della shell desktop — che e il piu grande del progetto, perche trascina Tauri, WebKitGTK e lo stack GTK — non e coperto da nessun gate sugli advisory di sicurezza ne sulle licenze. Il workflow compensa gia questa esclusione per fmt e clippy con step dedicati su src-tauri/Cargo.toml, ma non lo fa per cargo-deny: la copertura del lint e stata ripristinata, quella della supply chain no. Questo debito non riguarda il gate GATE-CI-GREEN, che e stato soddisfatto e chiuso con DEBT-001: riguarda cosa quel gate misura, cioe il perimetro su cui la pipeline verde si pronuncia.

## Evidence and provenance

Il 2026-08-25, entro pochi minuti dall'abilitazione di Dependabot sul repository, e stato aperto l'alert numero 1: GHSA-wrw7-89jp-8q8g, severita medium, unsoundness negli impl di Iterator e DoubleEndedIterator per glib::VariantStrIter, versioni vulnerabili >= 0.15.0 < 0.20.0, manifest apps/desktop/src-tauri/Cargo.lock. La CI era verde sulla run 32821923135 dello stesso commit, con lo step EmbarkStudios/cargo-deny-action@v2 riuscito: il gate non ha visto l'advisory perche il crate non e nel suo grafo. La prova che l'esclusione dal workspace e la causa e nel workflow stesso, .github/workflows/ci.yml, dove il job desktop porta un commento esplicito sul fatto che src-tauri non viene lintato dal job rust e aggiunge per questo step fmt e clippy dedicati con --manifest-path src-tauri/Cargo.toml, senza un equivalente per cargo-deny. Il tentativo automatico di Dependabot di rimediare e fallito, run 32821326909, con security_update_not_possible: latest-resolvable-version 0.18.5 contro lowest-non-vulnerable-version 0.20.0, cioe la versione corretta non e raggiungibile senza far avanzare lo stack GTK di Tauri.

## Impact and scope boundary

Una vulnerabilita nelle dipendenze della shell desktop passa il gate di CI senza essere vista. La copertura e stata scoperta mancante solo perche il repository e diventato pubblico e Dependabot e stato attivato: senza quell'evento la CI avrebbe continuato a riportare verde su un grafo mai controllato. Il rischio si estende oltre gli advisory al controllo delle licenze, che deny.toml governa con una allow list chiusa: qualunque licenza incompatibile entrata dallo stack Tauri e oggi invisibile. Il progetto dichiara la sicurezza come proprieta portante e ora lo fa su un repository pubblico, dove la disciplina della supply chain e osservabile da chiunque.

## Decision log

Created by project-lead: Scoperto durante la remediation di DEBT-001 e l'hardening del repository dopo il passaggio a pubblico. Registrato come debito separato e non come lavoro immediato perche la correzione ha due parti che non hanno la stessa urgenza ne lo stesso decisore: aggiungere lo step e meccanico, ma la sorte dell'advisory su glib e una decisione di deroga che va motivata per iscritto. Severita high e non medium nonostante l'advisory in se sia medium: il difetto non e la vulnerabilita trovata, e il fatto che il gate riporti verde su un grafo che non ha mai controllato, quindi la prossima vulnerabilita di qualunque severita sarebbe ugualmente invisibile. Deliberatamente senza origin_artifact: un primo tentativo lo aveva ancorato a SPEC-002/GATE-CI-GREEN e il contratto lo ha respinto perche quel gate e gia promosso da DEBT-001. Il rifiuto era corretto nel merito e non solo nella forma: questo debito non nasce da un gate non soddisfatto, nasce dal perimetro su cui il gate si pronuncia.

## Resolution criteria

Il workflow esegue cargo-deny anche su apps/desktop/src-tauri, con la stessa configurazione deny.toml e la stessa capacita bloccante del gate del workspace, e una run verde dimostra che lo step e stato realmente eseguito su quel grafo. Separatamente va presa una posizione esplicita su GHSA-wrw7-89jp-8q8g: se la versione corretta di glib resta irraggiungibile perche vincolata dallo stack GTK di Tauri, la deroga va scritta nella sezione advisories.ignore di deny.toml con la motivazione e la condizione di riesame, non lasciata implicita nella mancanza di copertura. Da valutare nella stessa occasione se l'esclusione di src-tauri dal workspace, decisa per ragioni di build, continui a valere il costo di dover replicare a mano ogni gate.

## Resolution evidence

