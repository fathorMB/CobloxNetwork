---
id: ADR-005
# Note: Quote the title if it contains a colon
title: "Economia del token a mint & burn"
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
# Economia del token a mint & burn

## Context

Il token di Coblox non sarà mai collegato al valore di valute reali o crypto: è un'unità di misura dell'uso della rete, non ricchezza. Ma se tutti i nodi guadagnano un reddito di esistenza continuo, l'offerta cresce senza limite e i prezzi in token perdono significato (iperinflazione). Serve un meccanismo che chiuda il ciclo mantenendo la promessa "guadagni anche se i tuoi nodi non vengono usati".

## Decision

Modello **mint & burn**:

- Ogni guadagno è **emesso dal protocollo**: reddito di esistenza (per presenza dimostrata, [ADR-002]) + compensi per storage/compute/availability effettivamente forniti.
- Ogni spesa (hosting di app, abbonamenti ai servizi delle app) è **bruciata**: i token spesi vengono distrutti, e il fornitore della risorsa è compensato con nuova emissione proporzionale al lavoro provato.
- L'offerta si autoregola: rete molto usata → molto burn; rete ferma → resta solo l'emissione base, contenuta.

Le curve (tasso base, compensi per risorsa, prezzi minimi) sono parametri di protocollo da tarare con **simulazioni economiche prima del lancio** e regolabili poi via governance dei validatori.

## Alternatives considered

- **Circolazione chiusa (le spese vanno al fornitore):** intuitivo ma senza pozzi per l'emissione base → inflazione perpetua e accumulo dominante — ricrea le dinamiche monetarie che il progetto rifiuta.
- **Mint & burn + demurrage (decadimento dei saldi inattivi):** filosoficamente coerentissimo, rimandato a fase 2 quando avremo dati reali; da comunicare con grande cura.
- **Offerta fissa con tesoreria redistributiva:** niente inflazione ma il reddito di esistenza diventerebbe dipendente dalla spesa altrui, rompendo la promessa.

## Consequences

- Serve un simulatore dell'economia (agent-based) come deliverable di progetto prima della taratura dei parametri.
- Il disaccoppiamento spesa→compenso (bruci qui, eminti là) elimina l'incentivo a "comprarsi da soli" per gonfiare i guadagni.
- Il demurrage resta un'estensione candidata (ADR futuro) come misura anti-accumulo.
- Ogni evento mint/burn è un record del ledger: la dashboard utente può mostrare guadagni e cause in tempo reale.

## Review conditions

Rivedere se: le simulazioni mostrano instabilità dei prezzi (spirali deflattive o inflattive); l'assenza di trasferimenti diretti utente→utente si rivela un limite per casi d'uso legittimi (da decidere esplicitamente: oggi i trasferimenti P2P non sono previsti).
