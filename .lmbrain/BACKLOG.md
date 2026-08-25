---
title: Product and technical backlog
updated: 2026-08-25
---

# Backlog

This is a concise, prioritized index of opportunities and work areas. Implementation handoffs (specs) live under `specs/`.

> Riallineato il 2026-08-25. La versione precedente era ferma a M-01: dava per aperti quattro debiti chiusi e per "Now" tre spec già `done`. Un indice che invecchia in silenzio è peggio di nessun indice, perché si legge come se fosse aggiornato.

## Now — M-02, nell'ordine

**[SPEC-010] … [SPEC-013] sono tutte `done` dal 2026-08-25**, e con esse il lavoro che doveva precedere la devnet. Nessuna spec redatta in attesa.

Il prossimo lavoro, da redigere:

1. Una spec piccola che raggruppi i due cambiamenti breaking dell'API di `coblox-core`, [DEBT-015] e [DEBT-016], **prima del primo chiamante del verificatore**: oggi non ne esiste alcuno e non costera mai meno di adesso.
2. Poi devnet BFT, light client con prove Merkle e mint & burn — dipendono dalle API fissate da [SPEC-008].

## Debiti aperti

- [DEBT-013] Nessuna regola impone il passo di produzione dei blocchi — medium, owner AGENT-007, M-02.
- [DEBT-014] `validator_set_hash` e l'unica preimmagine a dominio separato non legata a `chain_id` — medium, owner AGENT-007, M-02. Trovato da AGENT-001 costruendo l'inventario di [SPEC-010].
- [DEBT-017] La finestra di esposizione dell'attestazione e tolleranza piu durata, e solo la durata e limitata — medium, owner AGENT-007, M-02.
- [DEBT-018] Nella matrice del threat model l'argomento non puo scrivere quindi n/a confonde falsificazione e perdita — medium, owner AGENT-007, M-02.
- [DEBT-016] Il verificatore accetta una fetta di byte dove il contratto impone un messaggio — medium, owner AGENT-001, M-02. **Da chiudere con [DEBT-015] in una sola spec**, prima del primo chiamante: sono due cambiamenti breaking della stessa API.
- [DEBT-015] I sotto-controlli della reward policy sono pubblici e invocabili al posto della validazione — low, owner AGENT-001, M-02. Cambiamento breaking, da raggruppare con altri.

**Da portare in una spec, non ancora registrato come debito:** nessun artefatto propone un valore di genesi per `F_max`, per i due pavimenti tariffari o per `validator_eligibility_threshold_units_min` — `recommended.py` ha un `ElectionBounds` e nessun `RewardBounds`. E RF-004 di [REVIEW-017]. Inoltre l'oracolo Python **non copre il rapporto di variazione sul lato elezione**, quindi per quel gemello `GATE-TWO-ORACLES` si applica a vuoto.

**Nessun debito `critical` ne `high` aperto.** I due `medium` hanno entrambi owner AGENT-007 e la stessa forma: un'osservazione che chi l'ha fatta non deve valutare da se.

Differito: [DEBT-010] a M-07, con la dimostrazione del tetto di genesi come criterio di una spec di M-02.

Risolti: [DEBT-001], [DEBT-005] (l'unico `critical`), [DEBT-006], [DEBT-007], [DEBT-008], [DEBT-009], [DEBT-012].

## Decisioni di prodotto

**Nessuna aperta.**

**Rimandata deliberatamente: la forma del monitoraggio di rete.** Decisione dell'operatore del 2026-08-25, presa dopo aver visto le alternative e il loro costo. Resta a M-08 come la roadmap prevede, con la motivazione che senza una devnet non si sa quali grandezze servano davvero e deciderle a tavolino rischia di progettare per problemi immaginari.

**Il perimetro del rinvio, perche non venga letto piu largo di com'e.** Riguarda la sola *telemetria di rete*, cioe nodi che riportano a qualcuno. Non riguarda la dashboard locale di M-03, che e gia prevista e non crea alcuna superficie nuova.

**La condizione che il rinvio porta con se, ed e la ragione per cui e registrato invece che lasciato in conversazione.** Quando M-08 arrivera, *«telemetria di salute della rete»* e una riga sola, e le righe sole diventano collettori centrali per inerzia. La forma va decisa **con un ADR** e non scoperta come dettaglio implementativo, perche telemetria di rete significa nodi che riportano a qualcuno: la superficie di correlazione che [ADR-015] ha tolto e la raccolta che [ADR-014] impegna a dichiarare. Le tre forme e il loro costo sono in questa nota: derivata dalla catena, auto-riportata, campionata e aggregata.

**Un innesco piu vicino di M-08, da riconoscere quando arriva.** La spec che chiudera [DEBT-013] costruira la misura della cadenza reale dal checkpoint di soggettivita debole, lato light client. E monitoraggio derivato dalla catena che entra da un'altra porta: quando accadra, buona parte della prima opzione sara gia costruita, e conviene accorgersene invece di riprogettarla.

**Cosa la catena gia da senza decidere nulla:** altezza e cadenza reale, composizione del set e ogni transizione, emissione per epoca, e `eligible_node_count` con `eligible_set_root` in ogni mint di reddito di esistenza. `ledger.md` dichiara gia che la deriva di composizione del set e *«a light-client-computable quantity rather than an operator dashboard»*, e lo e per scelta.

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
