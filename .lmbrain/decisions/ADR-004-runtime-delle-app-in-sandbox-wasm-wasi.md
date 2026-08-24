---
id: ADR-004
# Note: Quote the title if it contains a colon
title: "Runtime delle app in sandbox WASM/WASI"
status: accepted
decision_date: 2026-08-25
decider: AGENT-LEAD
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: []
tags: [architecture]
created: 2026-08-25
updated: 2026-08-25
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned proposed -> accepted"
---
# Runtime delle app in sandbox WASM/WASI

## Context

Le "app" sono il lato domanda dell'economia: chi le pubblica spende token per farle hostare dai nodi. I nodi eseguono quindi codice di sconosciuti su macchine personali — la sandbox è un requisito di sicurezza assoluto. Vincolo aggiuntivo: i container OCI non girano su Android, e vogliamo che i telefoni possano offrire anche compute, non solo storage.

## Decision

Le app sono **moduli WebAssembly (WASI)** eseguiti in **wasmtime**, embedded nel core Rust ([ADR-003]). Capability esplicite (niente accesso a filesystem/rete se non concesso dal manifest dell'app), metering di CPU/memoria/fuel per la tariffazione in token, ed esecuzione deterministica — che rende la verifica a campione dei risultati ([ADR-002]) un semplice confronto di output.

## Alternatives considered

- **Container OCI (solo desktop/headless):** qualunque stack esistente, ma sandbox più debole, niente compute mobile, orchestrazione pesante.
- **Ibrido WASM + container sui nodi grossi:** buona evoluzione di fase 2, ma due runtime e due modelli di sicurezza al lancio sono complessità prematura.
- **Solo storage+availability al lancio:** taglierebbe metà complessità ma priverebbe l'economia del suo lato domanda.

## Consequences

- Gli sviluppatori di app devono usare linguaggi che compilano a WASM (Rust, Go, AssemblyScript, C/C++); serve un SDK e almeno un'app dimostrativa fatta da noi.
- Determinismo → il confronto dei risultati tra nodi è la base della verifica del compute.
- Il manifest dell'app (capability richieste, risorse massime, prezzo) è un'interfaccia di prima classe da specificare presto.
- L'opzione container resta aperta come tier futuro per nodi headless potenti, senza vincolare il design attuale.

## Review conditions

Rivedere se: l'ecosistema WASI (threads, networking) non copre i bisogni delle prime app reali; il metering di wasmtime si rivela troppo grossolano per una tariffazione equa.
