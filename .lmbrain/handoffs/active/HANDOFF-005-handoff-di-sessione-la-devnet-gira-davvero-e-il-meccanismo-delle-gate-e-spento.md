---
id: HANDOFF-005
title: "Handoff di sessione — la devnet gira davvero, e il meccanismo delle gate è spento"
status: ready
from_role: AGENT-LEAD
to_role: AGENT-LEAD
created: 2026-08-27
updated: 2026-08-27
related_specs: [SPEC-022, SPEC-025, SPEC-029]
related_reviews: [REVIEW-044, REVIEW-047, REVIEW-049]
related_decisions: [ADR-017, ADR-018]
links: [DEBT-049, DEBT-050]
tags: [session-handoff]
activity:
  - date: 2026-08-27
    action: "created"
---
# Project Lead session handoff

## Leggi prima questo

**Il meccanismo delle gate di verifica è spento, e lo è sempre stato.**

`verification_gates` è vuoto nel frontmatter di **28 spec su 29**, e `.lmbrain/verification.toml` non esiste. Ne segue che `spec_attest_lead` **non è utilizzabile**: ha rifiutato l'attestazione di `GATE-LEAD-REPRO` su [SPEC-029] con *«verification requirement does not exist»*. Il meccanismo funziona — [SPEC-011] ha le gate registrate con `id`/`status` — semplicemente non lo usa nessun altro.

Finché resta così, **ogni gate è una casella markdown scritta e spuntata dalla stessa parte che deve rispettarla.**

Non è teoria. Questa sessione ha trovato **cinque caselle `[x]` che asserivano il falso**, tutte su [SPEC-029]: il criterio del runbook (spuntato mentre `docs/devnet-runbook.md` non esisteva), `GATE-DURABLE-BEFORE-SEND` (il test che il suo testo nomina non esisteva), il criterio sul buffering (nessun test nominava `FutureHeightBuffer`), il criterio sul riavvio senza equivocazione (spuntato su una proprietà più debole del proprio testo), e `GATE-SUBSET-DECLARED`. Più `GATE-TWO-ORACLES` su [SPEC-022] dalla sessione precedente.

> Sei istanze non sono una serie di sviste. Sono un meccanismo che non gira. **Questa è la prima cosa che il Lead entrante dovrebbe portare all'operatore**, perché è una decisione di governo e non una spec da dispacciare.

## Il risultato che vale della sessione

**La devnet esiste e gira.** Quattro processi validatore separati, su rete vera con TCP + Noise + Yamux + GossipSub, che finalizzano blocchi e sopravvivono all'uccisione di un nodo. Il Lead l'ha avviata di persona dal runbook: quattro nodi a 35, `kill -9` su `val-003`, i tre superstiti a 67 mentre il morto restava a 36, riavvio sulla stessa `--data-dir` e tutti e quattro a 132.

HANDOFF-004 diceva: *«Abbiamo un validatore di regole, non una rete. Due macchine oggi non potrebbero parlarsi, né produrre un blocco, né salvare niente.»* **Non è più vero.** [SPEC-025] ha portato il motore di consenso e [SPEC-029] la rete, la persistenza e il nodo eseguibile. `coblox-node` non è più ventun righe.

Questo è **un esito di M-02 su quattro** — la devnet BFT. Restano light client con prove Merkle, e mint & burn.

## Lo stato, in numeri

| | |
| --- | --- |
| Righe di Rust in `core/` | 25 719 (erano ~16 000) |
| Test | 262 verdi, 3 ignorati, 0 falliti in locale |
| Spec | 23 done, 2 in review, 4 in backlog |
| Review | 40 accettate, 3 changes-requested, 0 pending |
| Debiti | 1 open, 18 planned, 20 resolved |

## Cosa è aperto, per nome

**[SPEC-029] è in review e non è chiudibile oggi.** `GATE-CI-GREEN` è **rossa** e non è mai stata verde in tutta la vita della spec. Quattro rilievi di [REVIEW-049] sono dichiarati non chiusi da AGENT-001 e vanno portati a debito prima della chiusura: RF-006 (la sincronizzazione resta pubblicazione sul topic invece che request/response su `ledger-sync`), RF-010 (nessun tetto in byte sul buffer), RF-016 (il WAL non copre `(h, r) -> proposta`), e la coda di RF-009 (`--seed-hex` ancora su `argv`).

**[SPEC-022] è in review** con [REVIEW-044] in changes-requested. **[REVIEW-047] su [SPEC-025]** ha quattro rilievi non bloccanti mai triati: RF-004, RF-006, RF-009, RF-010.

**[DEBT-050] è il debito che blocca la CI**, ed è descritto sotto.

## Il difetto vivo, e le due diagnosi sbagliate che l'hanno preceduto

[DEBT-050]: **un nodo che rientra può restare senza pari da cui sincronizzare.** Un nodo che raggiunge il proprio `--target-height` esce senza riguardo per chi è rimasto indietro, e col throttle di [REVIEW-049] RF-006 — otto blocchi per risposta, una risposta al secondo per richiedente — il recupero costa più secondi di *presenza altrui* di quanti gliene vengano concessi.

Il percorso conta più della conclusione, perché è dove il Lead uscente ha sbagliato:

1. Primo fallimento `[8, 8, 8, 3]`: attribuito **al throttle**, senza prove.
2. Rilancio `[8, 8, 7, 8]`: il nodo in ritardo cambia, quindi il Lead **ritira** la prima diagnosi e attribuisce **alla lentezza della macchina**. Alza le scadenze a 45s e scrive nel test *«è la firma di una macchina lenta e non di un difetto»*.
3. Terzo fallimento `[8, 8, 8, 5]` a 46,51s: **con più del doppio del tempo il nodo riavviato passa da 3 a 5, non a 8.** La seconda diagnosi cade, e il commento scritto nel test era falso. È stato sostituito con le tre trascrizioni e il meccanismo vero.

> **La lezione ha un nome: due diagnosi su tre erano sbagliate, ed entrambe erano state scritte in un artefatto prima di essere provate.** Il dato che ha deciso non è arrivato da un ragionamento migliore, ma dal rilanciare l'esecuzione una terza volta.

## Cosa il Lead uscente ha sbagliato, per nome

- **Tre affermazioni false** introdotte nel codice durante la presa in carico correttiva della sessione precedente, tutte trovate da [REVIEW-049]: il commento che dichiarava di scartare buste di un'altra catena mentre il `chain_id` non era mai confrontato; quello che dichiarava di errare su una trasmissione che non parte mentre i cinque `try_send` sono tutti `let _ =`; e la ragione di `now_ms`, ripetuta anche in un messaggio di commit, che diceva *«il valore che arma ogni timeout di consenso»* mentre alimenta solo `created_at_ms`. Sul merito, `unwrap_or(u64::MAX)` falliva **aperto** e produceva una busta eterna: il verso sbagliato.
- **Il commento falso nel test**, descritto sopra.
- **`DEBT-049` aperto come fuori perimetro** quando RF-005 lo sussumeva già. Chiuso col rilievo.
- **Un `echo` letto come verifica**: `cargo clippy | tail -2 && echo "pulito"` stampa «pulito» anche quando clippy fallisce, perché in una pipeline l'esito è quello di `tail`. Clippy stava fallendo su tre `Duration`.

## Le due volte in cui la review aveva torto

Vanno lette insieme alle precedenti, perché la disciplina non è «credere al revisore»:

- **Il `chain_id`.** [REVIEW-049] RF-001 chiedeva di confrontarlo. La busta **non ha quel campo**: è legato dentro il preimage di `message_id`, quindi ricalcolarlo *è* il controllo di catena, ed è più forte. Corretto da AGENT-001.
- **La condizione di chiusura di RF-002.** Chiedeva che `can_vote` rifiutasse ogni valore diverso da quello lockato. Romperebbe la liveness: la riga 29 dell'Algoritmo 1 permette di sbloccarsi dopo una polka, e il WAL **non sa nulla delle polka**. Il Lead ha dato ragione allo specialista contro la review.

E una cosa che **nessuno dei due** aveva nominato, verificata dal Lead: restringere `locked` a `Digest32` non impedisce a un proposer lockato di riproporre, perché la ri-proposta passa da `self.valid` (`engine.rs:554`), che conserva il `Value` intero.

## Decisioni che aspettano l'operatore

1. **Il meccanismo delle gate**, sopra. È la più importante.
2. **Quattro vulnerabilità `high`** segnalate da Dependabot, tre dentro il sottoinsieme di trasporto dichiarato da [SPEC-029]: due su `libp2p-gossipsub` (crash remoto per overflow nel backoff dell'heartbeat) e una su `yamux` (panic remoto su frame malformato con `len = 262145`). Raggiungibili da qualunque pari.
3. **RF-008 di [REVIEW-042]**: `P = F + G` rende `reason` letto e inerte.
4. **Il punto 3 di [ADR-017]**, riaperto: serve una forma iniettiva che preservi le altezze distinte, o la prova che non esista.
5. **`GATE-ENGINE-UNCHANGED` su [SPEC-029]**, derogata dal Lead e ribaltabile: la clausola di eccezione nomina [REVIEW-047] e non [REVIEW-049].

## Cosa fare per primo

1. Portare all'operatore il meccanismo delle gate. Non dispacciare altro prima.
2. [DEBT-050] ad AGENT-001, che ha scritto il throttle e ha il contesto. Le tre trascrizioni sono nel debito e nel commento del test.
3. I quattro rilievi non chiusi di [REVIEW-049] a debito, poi chiudere [SPEC-029].
4. Solo dopo, il prossimo esito di M-02.

## Receiving Project Lead checklist

- [ ] Letti `CONTRACT.md`, `QUALITY.md`, `AGENT.md` e i moduli sotto `contract/`: nel kit 5.1.0 sono lettura obbligatoria
- [ ] Letto `.lmbrain/STATUS.md`
- [ ] Verificato che `GATE-CI-GREEN` di [SPEC-029] sia ancora rossa prima di toccare qualunque altra cosa
- [ ] Portata all'operatore la questione del meccanismo delle gate
- [ ] Consumato questo handoff con `handoff_consume`

## Handoff outcome

> Compilato dal Lead entrante a handoff consumato.
