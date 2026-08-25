---
id: DEBT-009
title: "Il grafo delle dipendenze della shell desktop non ha alcun gate sugli advisory"
status: resolved
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
resolution_refs: ["SPEC-002","ADR-003"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["ci","supply-chain","verification-gap","cargo-deny"]
links: []
activity:
  - date: 2026-08-25
    action: "resolved: Chiuso su richiesta dell'operatore il 2026-08-25, senza attendere M-02. Eseguire il controllo ha rivelato che la stima del debito era per difetto: non falliva solo advisories, fallivano anche bans e licenses. Due dei difetti emersi non erano derogabili ed e giusto che non lo fossero, perche non erano condizioni irrisolvibili a monte ma errori nostri: coblox-desktop non dichiarava license ne publish, e il campo repository del workspace puntava a un repository inesistente. Sono stati corretti, non ignorati.\n\nSulla terza parte dei criteri di risoluzione, la valutazione se l'esclusione di src-tauri dal workspace valga ancora il costo: la posizione del Lead e che il costo sia ora pagato ma vada dichiarato. La replica manuale dei gate e completa (fmt, clippy, cargo-deny) e ogni step porta un commento che spiega perche esiste, quindi il difetto specifico e chiuso. Resta pero una tassa permanente: ogni gate futuro sul workspace andra replicato a mano su src-tauri, e nulla nel repository lo impone. Non se ne fa un debito nuovo perche non c'e oggi un difetto da riparare, ma e la prima cosa da riconsiderare se un quarto gate dovesse aggiungersi, o se Tauri rendesse un giorno possibile includere il crate nel workspace senza i problemi di build che ne avevano motivato l'esclusione."
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
  - schema_version: "1"
    id: "DEBT-009-EVENT-002"
    timestamp: "2026-08-25T11:51:09.200865100+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Chiuso su richiesta dell'operatore il 2026-08-25, senza attendere M-02. Eseguire il controllo ha rivelato che la stima del debito era per difetto: non falliva solo advisories, fallivano anche bans e licenses. Due dei difetti emersi non erano derogabili ed e giusto che non lo fossero, perche non erano condizioni irrisolvibili a monte ma errori nostri: coblox-desktop non dichiarava license ne publish, e il campo repository del workspace puntava a un repository inesistente. Sono stati corretti, non ignorati.\n\nSulla terza parte dei criteri di risoluzione, la valutazione se l'esclusione di src-tauri dal workspace valga ancora il costo: la posizione del Lead e che il costo sia ora pagato ma vada dichiarato. La replica manuale dei gate e completa (fmt, clippy, cargo-deny) e ogni step porta un commento che spiega perche esiste, quindi il difetto specifico e chiuso. Resta pero una tassa permanente: ogni gate futuro sul workspace andra replicato a mano su src-tauri, e nulla nel repository lo impone. Non se ne fa un debito nuovo perche non c'e oggi un difetto da riparare, ma e la prima cosa da riconsiderare se un quarto gate dovesse aggiungersi, o se Tauri rendesse un giorno possibile includere il crate nel workspace senza i problemi di build che ne avevano motivato l'esclusione."
    evidence_refs: ["SPEC-002", "ADR-003"]
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

Run GitHub Actions 32833295352 sul commit 7f5327d di main, conclusione success su tutti e cinque i job. Nel job Tauri desktop (ubuntu-latest) lo step EmbarkStudios/cargo-deny-action@3c6349835b2b7b196a839186cb8b78e02f7b5f25 con manifest-path apps/desktop/src-tauri/Cargo.toml risulta eseguito e riuscito, e il log riporta per esteso "advisories ok, bans ok, licenses ok, sources ok" sul grafo della shell desktop. La capacita bloccante e dimostrata dal fatto che la stessa combinazione di comando e configurazione, eseguita in locale prima delle correzioni, restituiva "advisories FAILED, bans FAILED, licenses FAILED" con exit non nullo: cio che la fa passare oggi e l'insieme delle correzioni piu le deroghe dichiarate, non una configurazione permissiva.

Quadro reale trovato eseguendo il controllo, piu ampio della stima del debito.

Corretti come difetti, non derogati.

1. coblox-desktop risultava unlicensed e senza campo license nel manifest, e faceva scattare un errore wildcard: senza publish = false cargo-deny lo tratta come crate pubblico e rifiuta la dipendenza per path su coblox-core che allow-wildcard-paths esiste per permettere. Causa: l'esclusione dal workspace radice gli impedisce di ereditare [workspace.package]. Aggiunti license = "Apache-2.0", repository e publish = false, allineati al workspace, con un commento che spiega perche sono ripetuti a mano.
2. Il campo repository di [workspace.package] dichiarava https://github.com/cobloxnetwork/coblox, che non esiste: verificato con l'API di GitHub, "Could not resolve to a Repository". Su un repository pubblico era un puntatore rotto nei metadati di ogni crate. Corretto su https://github.com/fathorMB/CobloxNetwork.

Derogati in apps/desktop/src-tauri/deny.toml, ciascuno con motivazione propria e condizione di riesame comune.

- GHSA-wrw7-89jp-8q8g, cioe RUSTSEC-2024-0429, unsoundness in glib::VariantStrIter. Raggiungibile solo chiamando quell'iteratore, cosa che ne coblox-desktop ne coblox-core fanno. Dependabot aveva gia dimostrato che non e risolvibile: security_update_not_possible, latest-resolvable-version 0.18.5 contro lowest-non-vulnerable-version 0.20.0.
- Dieci advisory sui binding GTK3 di gtk-rs, non piu mantenuti come famiglia. Tauri v2 vi dipende su Linux attraverso webkit2gtk e non esiste un percorso a GTK4 indipendente da Tauri: RUSTSEC-2024-0411 fino a 0420.
- Cinque advisory sui crate Unicode di open-i18n/rust-unic, non piu mantenuti come famiglia, che arrivano tramite urlpattern e tauri-utils: RUSTSEC-2025-0075, 0080, 0081, 0098, 0100.
- RUSTSEC-2024-0370, proc-macro-error non mantenuto, dipendenza transitiva di sola build che non contribuisce codice al binario distribuito.

Per tutti e diciassette cargo-deny riporta "No safe upgrade is available": sono deroghe di una condizione irrisolvibile alla versione pinnata, non di una condizione non esaminata. La condizione di riesame scritta nel file impone di rivedere l'intera lista, e non di estenderla per abitudine, quando Tauri pubblichera una release che abbandona lo stack GTK3 o quando una singola voce acquisira un percorso di aggiornamento risolvibile.

Aggiunte alla allow list delle licenze del solo grafo desktop: BSD-3-Clause, che arriva con alloc-no-stdlib, alloc-stdlib e brotli, e Zlib, che arriva con foldhash. Entrambe permissive, OSI approved e FSF Free/Libre, senza obblighi di copyleft e quindi senza questioni di compatibilita con l'Apache-2.0 del progetto.

La configurazione e deliberatamente separata da deny.toml radice. La sezione advisories.ignore di cargo-deny e indicizzata per ID di advisory e non permette di limitarne la portata a un crate: scrivere queste voci nel file condiviso sopprimerebbe in silenzio gli stessi advisory anche per il workspace il giorno in cui uno di quei crate comparisse nel suo grafo, che e esattamente il modo di fallire per cui questo debito era stato aperto. Il costo accettato e la duplicazione della allow list delle licenze, che puo divergere; la divergenza pero fallisce in modo rumoroso in uno dei due grafi, mentre una soppressione ereditata sarebbe invisibile. Il file documenta questo compromesso al proprio interno. Verificato inoltre che il gate del workspace radice resta invariato e verde: cargo deny check dalla radice riporta "advisories ok, bans ok, licenses ok, sources ok".
