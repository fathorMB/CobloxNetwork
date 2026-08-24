---
id: ADR-001
# Note: Quote the title if it contains a colon
title: "Ledger dei token su federazione BFT con validatori a rotazione"
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
# Ledger dei token su federazione BFT con validatori a rotazione

## Context

Coblox Network richiede un registro condiviso di saldi, accrediti (reddito di esistenza + lavoro svolto) e spese (hosting, abbonamenti). Il token non è convertibile in denaro, quindi l'incentivo economico ad attaccare il ledger è strutturalmente basso; restano però i requisiti "super-sicura" (niente double-spend, niente falsificazione dei saldi) e "verificabile in tempo reale" (ogni utente deve poter vedere i propri accrediti mentre avvengono). I nodi includono dispositivi Android e macchine headless piccole: il costo del consenso deve restare marginale per un nodo qualsiasi.

## Decision

Il ledger è mantenuto da una **federazione BFT**: un insieme ristretto (ordine di grandezza: 20–100) di nodi validatori esegue un consenso Byzantine Fault Tolerant della famiglia Tendermint/HotStuff. I validatori sono **eletti e ruotati periodicamente** tra i nodi con la migliore combinazione di reputazione e uptime dimostrato ([ADR-002]); nessun validatore è permanente. Tutti gli altri nodi sono light client: verificano le firme dei blocchi e le prove Merkle dei propri saldi senza partecipare al consenso.

## Alternatives considered

- **Blockchain permissionless PoS (stile Ethereum):** massima decentralizzazione, ma complessità e overhead sproporzionati per difendersi da attacchi economici che qui non hanno ricompensa economica.
- **DAG + gossip (crediti locali, stile IOTA/Holochain):** leggerissimo, ma finalità debole e double-spend difficile da escludere formalmente; in attrito con il requisito di sicurezza.
- **Coordinatore centrale temporaneo:** time-to-market rapido, ma tradisce la promessa distribuita fin dal design e rende dolorosa la migrazione successiva.

## Consequences

- Serve progettare il meccanismo di elezione/rotazione dei validatori come componente di prima classe (anti-collusione, slashing reputazionale, bootstrap iniziale con validatori seed del progetto).
- Finalità rapida (secondi) → la dashboard "quanto sto guadagnando adesso" è realizzabile davvero in tempo reale.
- I nodi piccoli non pagano il costo del consenso: partecipano come light client.
- Il set dei validatori è il perimetro di fiducia della rete: la sua salute va monitorata e resa pubblica.

## Review conditions

Rivedere se: la rotazione dei validatori si dimostra manipolabile in simulazione o in rete di test; il token acquisisse in futuro qualunque forma di convertibilità (cambierebbe il threat model); il numero di nodi rendesse il light-client protocol un collo di bottiglia.
