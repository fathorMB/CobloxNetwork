---
id: ADR-002
# Note: Quote the title if it contains a colon
title: "Proof of contribution tramite challenge crittografici"
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
# Proof of contribution tramite challenge crittografici

## Context

Il reddito di esistenza e i guadagni per storage/compute funzionano solo se la rete può provare che un nodo offre davvero le risorse dichiarate. Senza prove robuste, un attaccante può emulare migliaia di nodi virtuali e stampare token (attacco Sybil), svuotando di significato l'intera economia. Le tre risorse richiedono tecniche diverse: uptime, integrità dello storage, correttezza del compute.

## Decision

L'accredito dei token è condizionato a **challenge crittografici** orchestrati dai validatori ([ADR-001]) e da peer selezionati casualmente:

- **Availability:** ping firmati con nonce imprevedibile a intervalli casuali; la risposta firmata entro la finestra prova la presenza online.
- **Storage:** proof-of-retrievability su campioni casuali dei blocchi custoditi (il nodo deve restituire il dato o una prova Merkle del possesso).
- **Compute:** ri-esecuzione a campione dei task WASM ([ADR-004], il determinismo del runtime rende il confronto dei risultati banale) con penalità reputazionale per i risultati errati.

Le risposte alle sfide, firmate, sono l'evidenza registrata sul ledger che sblocca l'accredito. Il rollout è incrementale: prima l'uptime, poi storage, poi compute.

## Alternatives considered

- **Attestazione hardware (TPM/Play Integrity) + challenge:** anti-Sybil più forte ma esclude VM e vecchi dispositivi e crea dipendenza da Google/Microsoft; riconsiderabile come tier opzionale "nodo certificato".
- **Reputazione + peer audit senza crittografia:** semplice ma banalmente colludibile; il reddito di esistenza diventerebbe stampabile.
- **Self-report firmato (fiducia iniziale):** MVP rapido ma economia falsificabile dal giorno uno; pessimo precedente culturale.

## Consequences

- Il design del campionamento (frequenza, casualità, chi sfida chi) è un componente critico di sicurezza e va specificato e simulato prima dell'implementazione.
- Il reddito di esistenza è di fatto un "reddito di presenza dimostrata": un nodo spento non accumula.
- Serve una strategia anti-Sybil complementare a livello di identità (costo di ingresso per nodo, es. proof-of-work una tantum alla registrazione o rate-limit sull'onboarding).
- Ogni challenge/risposta produce dati verificabili che alimentano direttamente la dashboard "in tempo reale" dell'utente.

## Review conditions

Rivedere se: le simulazioni mostrano che il campionamento è aggirabile con probabilità non trascurabile; il traffico delle sfide pesa troppo su nodi mobili (batteria/dati); emergono attacchi di collusione tra sfidanti e sfidati.
