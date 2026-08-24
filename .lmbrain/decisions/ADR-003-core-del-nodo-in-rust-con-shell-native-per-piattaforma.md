---
id: ADR-003
# Note: Quote the title if it contains a colon
title: "Core del nodo in Rust con shell native per piattaforma"
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
# Core del nodo in Rust con shell native per piattaforma

## Context

Lo stesso motore di nodo deve girare su Android, Windows/Linux desktop e Windows/Linux headless, gestendo crittografia, networking P2P, client del ledger, storage engine e sandbox di compute. Duplicare questa logica per piattaforma sarebbe insostenibile e pericoloso (la parte critica per la sicurezza esisterebbe in N versioni).

## Decision

Un'unica libreria core in **Rust** (`coblox-core`) contiene tutta la logica: P2P su **libp2p**, crittografia, light client del ledger, storage engine, runtime WASM, motore dei challenge. Le shell per piattaforma sono sottili:

- **Desktop (Windows/Linux):** app **Tauri** — il frontend web-tech ospita il design system "hacker" e parla col core via comandi Tauri.
- **Android:** UI **Kotlin/Jetpack Compose**, core integrato via **UniFFI**; esecuzione in foreground service per l'uptime.
- **Headless (Windows/Linux):** stesso binario del core come servizio/daemon con CLI e API locale (la stessa API usata dalle UI).

## Alternatives considered

- **Go + gomobile/Wails:** sviluppo più rapido e go-libp2p maturissimo, ma gomobile fragile, binari grossi e GC subottimale su mobile.
- **Kotlin Multiplatform:** ottimo per Android ma privo di uno stack P2P/crypto maturo; JVM pesante su nodi headless piccoli.
- **Node/Electron:** prototipazione veloce ma footprint enorme e sandbox difficile; incoerente con il claim di sicurezza.

## Consequences

- La logica critica per la sicurezza esiste in un solo posto, memory-safe.
- Il design system HTML/CSS del desktop (Tauri) è riusabile per sito e documentazione; Android richiede una resa nativa dei token di design.
- La toolchain di build cross-platform (NDK Android, UniFFI, CI multipiattaforma) va impostata presto: è un rischio di attrito noto.
- Il team di agenti deve includere competenza Rust forte; la curva di apprendimento è accettata come costo.

## Review conditions

Rivedere se: UniFFI o la build Android generano attrito ingestibile; le prestazioni di libp2p su reti mobili (NAT, cambio rete) si rivelano inadeguate.
