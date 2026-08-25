---
title: Product and technical backlog
updated: 2026-08-25
---

# Backlog

This is a concise, prioritized index of opportunities and work areas. Implementation handoffs (specs) live under `specs/`.

> Riallineato il 2026-08-25. La versione precedente era ferma a M-01: dava per aperti quattro debiti chiusi e per "Now" tre spec già `done`. Un indice che invecchia in silenzio è peggio di nessun indice, perché si legge come se fosse aggiornato.

## Now — M-02, nell'ordine

[SPEC-010] e `done` dal 2026-08-25. Le tre che seguono erano bloccate dalla sua dipendenza e sono in `ready`. Owner AGENT-001, tier `sol`.

1. **[SPEC-011] — `RewardBounds` e le regole di validita economiche in `coblox-core`.** Chiude il divario fra documenti e codice lasciato da [SPEC-009], che tocco `tests/` e non `src/`. Corregge anche il fondo di genesi in `recommended.py`, che oggi contraddice [ADR-011].
2. **[SPEC-012] — Verificatore Ed25519 con i vettori speccheck come oracolo.** Isolata, parallelizzabile, **prima di qualunque devnet**.
3. **[SPEC-013] — Separazione della chiave di trasporto**, attuazione di [ADR-015]. **Prima che la devnet emetta il primo certificato**: dopo e una migrazione, non una decisione.

Poi devnet BFT, light client con prove Merkle e mint & burn — dipendono dalle API fissate da [SPEC-008].

## Debiti aperti

- [DEBT-013] Nessuna regola impone il passo di produzione dei blocchi — medium, owner AGENT-007, M-02.
- [DEBT-014] `validator_set_hash` e l'unica preimmagine a dominio separato non legata a `chain_id` — medium, owner AGENT-007, M-02. Trovato da AGENT-001 costruendo l'inventario di [SPEC-010].

**Nessun debito `critical` ne `high` aperto.** Entrambi gli aperti hanno owner AGENT-007 e la stessa forma: un'osservazione che chi l'ha fatta non deve valutare da se.

Differito: [DEBT-010] a M-07, con la dimostrazione del tetto di genesi come criterio di una spec di M-02.

Risolti: [DEBT-001], [DEBT-005] (l'unico `critical`), [DEBT-006], [DEBT-007], [DEBT-008], [DEBT-009], [DEBT-012].

## Decisioni di prodotto

**Nessuna aperta.** Le cinque residue sono state chiuse il 2026-08-25: intervallo di blocco ([ADR-013]), popolazione al lancio (annotazione su [ADR-011]), privacy degli abbonati ([ADR-014]), identità di trasporto ([ADR-015]), disposizione di [DEBT-010].

[ADR-015] è stata letta e **accettata** dall'operatore il 2026-08-25. Nessuna azione dell'operatore in sospeso.

Due segnalazioni fuori ambito emerse da [SPEC-010] e da valutare: gli alberi Merkle delle transazioni, degli abbonamenti e dell'insieme eleggibile **non hanno un esempio lavorato pubblicato** mentre quello dei candidati sì; e le lacune più chiare fra le 25 preimmagini senza valore pubblicato sono `enrollment_pow_salt` e `node_leaf`.

## Next

- Il **testo pubblico** che attua [ADR-014]: scritto una volta sola e citato, con `GATE-SECREVIEW`, con scadenza al primo partecipante esterno. Due copie divergono, ed è la famiglia 2 di `recurring-defects.md`.
- Da [ADR-006]: entità "saldo dell'app" nel ledger con consumo per epoche (M-02, dominio AGENT-002).
- Il **Circuit Relay v2 obbligatorio per i nodi domestici**: è l'altra metà del rimedio a TM-28, quella che [ADR-015] non affronta. Decisione propria, da prendere con misure di latenza e carico che richiedono una devnet.
- Mapping Compose dei design token (dopo [SPEC-003], già chiusa).

## Later

- Da [ADR-006]: flusso di pubblicazione end-to-end e catalogo delle app (M-06).
- Da [ADR-006]: lista di rifiuto per nodo e lista di blocco di rete — funzionalità di prodotto, non dettagli implementativi (M-06/M-07).
- `HostingRateCardBody` è il terzo documento governato **senza alcun oggetto di limiti**, con un proprio denominatore nelle sue tariffe. È burn e non mint, quindi nessuna superficie Sybil, ma è integrità di addebito. Segnalato da AGENT-007 come residuo fuori ambito di [SPEC-009]; la gate di [ADR-012] vi si applicherà quando M-06 toccherà l'hosting.
- Ricerca sulla **prova aggregata degli abbonati** ([ADR-014]): candidato per M-08, senza data e senza promessa.
- Tier "nodo certificato" con attestazione hardware (idea parcheggiata da [ADR-002]).

## Parking lot

- Demurrage anti-accumulo ([ADR-005], fase 2).
- Tier container per nodi headless potenti ([ADR-004], fase 2).
- iOS/macOS.
