---
id: HANDOFF-002
title: "Handoff di sessione — notte del 26 agosto: tre spec chiuse, cinque debiti, e una review che lodava senza attaccare"
status: superseded
from_role: AGENT-LEAD
to_role: AGENT-LEAD
created: 2026-08-26
updated: 2026-08-26
related_specs: [SPEC-014, SPEC-015, SPEC-016, SPEC-017, SPEC-018, SPEC-019, SPEC-020]
related_reviews: [REVIEW-025, REVIEW-026, REVIEW-027]
related_decisions: [ADR-010, ADR-012, ADR-013]
links: [DEBT-013, DEBT-014, DEBT-017, DEBT-018, DEBT-019, DEBT-020, DEBT-021, DEBT-022, DEBT-023, DEBT-024, DEBT-025]
tags: [session-handoff]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned ready -> superseded"
---
# Project Lead session handoff

## Purpose of this handoff

Consegnare lo stato dopo la sessione autonoma della notte fra il 25 e il 26 agosto 2026. Mandato dell'operatore: *«portiamo avanti l'analisi sui debt e lavoriamo entrambe le spec in backlog. io non ci sarò per approvazioni, dispaccia le spec una alla volta, attendi il completamento e fai i giri di review sullo stesso sub-agent. se emergono nuove spec, procedi anche con quelle.»* Più due interventi successivi: chiudere [SPEC-015], e **una sola spec per volta** per non sforare i limiti di consumo.

**Verificare le affermazioni di questo documento prima di agire**: sono uno scatto al momento della scrittura, e questa sessione ha prodotto tre casi in cui un'affermazione scritta con cura era falsa.

## Executive project state

M-02 in corso. **Diciotto spec redatte, sedici `done`, due in `backlog` più una in `working` senza nessuno che ci lavori.** 151 test, `clippy -D warnings` e `fmt` puliti, gate di [ADR-012] `PASS` con 103 probe C10 e 8 C11, prova in negativo su 15 mutazioni **più tutte e 103 le probe individualmente**.

## Work completed in this session

**Recuperata l'implementazione di [SPEC-014]**, accettata il giorno prima e **mai committata**: sette file di `core/` erano nell'albero di lavoro e in nessun commit. Le 126 prove verdi riportate all'operatore giravano su quei file. Commit `dfe44fe`.

**[SPEC-015] chiusa** con `GATE-SECREVIEW` **derogata**, su richiesta dell'operatore. L'obbligo non è sparito con la spec: è passato su **[DEBT-023]**, che porta il nome della gate derogata come [DEBT-001] portava `GATE-CI-GREEN`, e ha per innesco **la pubblicazione e non una data**.

**Le tre valutazioni di AGENT-007** su [DEBT-022], [DEBT-017] e [DEBT-018], che hanno corretto l'impostazione del Lead su due debiti su tre. Poi le correzioni del Lead sui propri artefatti, in un commit separato perché l'attribuzione resti leggibile.

**[SPEC-016] chiusa**, e con lei [DEBT-013], [DEBT-014] e [DEBT-019]. Due giri di review, [REVIEW-025] del Lead e [REVIEW-027] di AGENT-007.

**[SPEC-018] chiusa**, e con lei [DEBT-018]. Un giro di remediation, [REVIEW-026].

**Tre spec redatte**: [SPEC-018], [SPEC-019], [SPEC-020].

**Quattro debiti aperti**: [DEBT-023], [DEBT-024], [DEBT-025], più le riclassificazioni di [DEBT-017] e [DEBT-022].

## Le tre cose che il Lead entrante deve leggere prima delle altre

Riguardano il **modo** in cui il progetto trova i propri difetti, non il loro contenuto.

**1. Una review che loda senza attaccare non ha verificato.** [REVIEW-025], del Lead, ha lodato l'asimmetria fuori banda di [SPEC-016] come *«la parte migliore del lavoro»* e non l'ha attaccata. [REVIEW-027] vi ha trovato un finding **high**: l'argomento *«la misura è distorta verso il basso e solo verso il basso»* valeva per il numeratore, mentre il denominatore aveva una distorsione propria e opposta — `issued_at_ms` è quando il checkpoint è **prodotto**, non quando l'altezza che nomina è raggiunta, quindi i blocchi della latenza di rilascio erano contati **senza il loro tempo**, e la lettura era spinta verso il lato su cui il client fallisce chiuso, **da qualcosa che non è la catena**.

La frase falsa era deducibile da `README.md`, nella **stessa sezione che l'implementatrice stava citando mentre la scriveva**. È il tratto comune delle quattro famiglie del censimento — il difetto era già scritto e non guardato — applicato due volte nello stesso punto: da chi ha scritto e da chi ha revisionato.

È la **quinta** volta su questo progetto che una misura risulta puntata sulla grandezza sbagliata, e la **seconda** dentro un lavoro già accettato dal Lead.

**2. Una gate che esercita solo il regime nominale non ha mai visto lo scenario che la rompe.** `GATE-MEASURE-BINDS` provava tre catene, **tutte a latenza zero**. Vale come criterio per ogni gate futura: se tutti i casi di prova hanno lo stesso valore su una grandezza che non è quella sotto test, quella grandezza è la prossima da variare.

**3. La guardia ha funzionato meglio del Lead.** `C11-CLAIMDOC`, costruita durante la notte, ha catturato **entro pochi minuti dall'esistere** una deriva reale prodotta da un altro agente sullo stesso albero: AGENT-007 aggiungeva scenari al threat model mentre `SECURITY.md` ne dichiarava un numero fermo. Il Lead non si è ricordato di aggiornare quel numero — **gliel'ha chiesto la gate**, nominando la fonte e il valore.

## Active work and current position

**Nessun agente in esecuzione.** L'albero è pulito e allineato a `origin/main`.

**[SPEC-017] è in `working` senza che nessuno ci lavori.** Era stata dispacciata ad AGENT-001, che ha fatto in tempo solo a chiamare `spec_start` prima che il Lead lo fermasse per rispettare il limite di una spec per volta. **Chi la riprende deve saltare `spec_start`.** Lo stato non è stato riportato indietro perché la spec *è* assegnata, solo in pausa, e forzare la lifecycle avanti e indietro sarebbe stato peggio del disallineamento.

## Ready for manual handoff

Tre spec, in ordine di **dipendenza e non di preferenza**:

1. **[SPEC-017]** — AGENT-001, `sol`/`standard`, già in `working`. Chiude [DEBT-020] e [DEBT-021]. Gate portante `GATE-TWO-DERIVATIONS`.
2. **[SPEC-019]** — AGENT-002, `sol`/`extended`, `backlog`. Chiude [DEBT-022]. Dipendenza su [SPEC-018] **soddisfatta**.
3. **[SPEC-020]** — AGENT-001, `sol`/`extended`, `backlog`. Chiude [DEBT-017]. Dipendenze su [SPEC-016] e [SPEC-018] **soddisfatte**.

## Pending review or evidence to inspect

Nulla in attesa di review. Tutte le review di questa sessione sono `accepted`.

**Due cose attendono l'operatore**, verificate enumerando i gate con `owner=operator` aperti e le decisioni in stato `proposed`: l'accettazione di [ADR-016], e la pubblicazione della guida che [DEBT-023] blocca — quest'ultima senza urgenza finché la guida resta interna. La prima riguarda: i valori della banda di cadenza — `min_ms_per_block`, `max_ms_per_block`, `min_measured_blocks`, `max_external_clock_slack_ms`. Sono una scelta di prodotto come `alpha`, e AGENT-002 ha fatto la cosa giusta a non prenderla. Istruiti nella lista DRAFT di `docs/protocol/README.md`, **coi due lati che non si scambiano fra loro**: il lato lento è l'incumbency, il lato veloce è l'emissione.

## Decisions, assumptions, and constraints

**[ADR-013] è stata annotata**, e l'annotazione era dovuta: la parte 3 resta vera e diventa **incompleta**. Fino a [SPEC-016] il pericolo aveva un verso solo — il rallentamento — perché `reward_epoch` non era derivato da nulla. Derivandolo da `height`, **accelerare moltiplica l'emissione reale**.

**Le due soglie non sono la stessa**, ed è la distinzione di AGENT-007: rallentare basta a un **terzo bloccante**, accelerare richiede un **quorum**, perché ogni blocco porta un certificato di quorum. Il guadagno sul lato veloce è inoltre *pro quota*, non ha negabilità, ed è osservabile da chiunque. **Il movente dominante resta il rallentamento — ed è il lato su cui il protocollo si limita a segnalare.** Il progetto fallisce chiuso sul lato più caro da attaccare. È la scelta giusta per la ragione che [SPEC-016] scrive, ma va saputa.

**Un modo di sbagliare che nessuna gate cerca**, registrato in [DEBT-013] e in [ADR-013]: *una chiusura che falsifica la descrizione del problema che chiudeva, e che lascia la descrizione in piedi perché nessuno pensa a rileggerla.*

**Vincoli di processo confermati:** gli specialisti non committano né pushano, il Lead è l'unico che spinge; mai staging ampio mentre un agente lavora sullo stesso albero; testo di prodotto in inglese, artefatti del brain in italiano; unità `credits`/`cr` posposta con separatore U+202F.

## Risks, blockers, and unresolved questions

**Nessun blocker.**

**[DEBT-022] è il rischio aperto più alto.** L'esito peggiore non è l'addebito indebito: è un **fork**, perché due letture entrambe conformi divergono sulla validità di un blocco, e non richiede alcuna chiave rubata. La condizione di chiusura è stata corretta da *«prima che una devnet accumuli saldi reali»* a **«prima che esista una seconda implementazione»**: la grandezza da cui il pericolo dipende non è il valore in gioco, è il numero di lettori del documento.

**[DEBT-025] va chiuso prima che la sua strumentazione invecchi.** `sim/tools/threat_model_matrix_coherence.py` esiste ma **non è cablato in CI**, e gli otto disallineamenti noti vanno sanati prima del cablaggio: uno strumento che nasce rosso viene disattivato, ed è il modo in cui una guardia muore.

**[DEBT-023] blocca la pubblicazione della guida** e nient'altro. Finché resta in `.lmbrain/design/` non ha urgenza.

**Un residuo dichiarato di [SPEC-016]**, che non è un difetto ma va saputo: oltre la tolleranza di clock, un client non distingue una catena veloce da un checkpoint servito in ritardo. La procedura di rilascio vieta di firmare quel checkpoint; il residuo è nella trascrizione della gate.

**Due residui minori mai chiusi:** `BLOCK_INTERVAL_SECONDS = 5 # assumption` in `sim/coblox_sim/recommended.py`, dichiarato conseguenza di [ADR-013] e mai allineato; e le due liste chiuse di `what a light client can establish`, deliberatamente non toccate perché la cadenza è una grandezza calcolata e non un fatto di composizione.

## Documentation updated

`.lmbrain/STATUS.md` (*Current focus*, *Ready for handoff*, *In progress*), `.lmbrain/ROADMAP.md`, `SECURITY.md` (paragrafo anti-Sybil rafforzato, cadenza a due direzioni, conteggi ricalcolati dalla fonte), `docs/protocol/README.md`, `docs/protocol/ledger.md`, `.lmbrain/knowledge/threat-model.md`, [ADR-013].

## Recommended next actions

1. **Accettare [ADR-016]**, che registra i valori della banda di cadenza decisi dall'operatore e resta in `proposed`. [SPEC-020] li userà, e i valori non sono ancora nei documenti di protocollo: scriverli è contenuto normativo nuovo e fa scattare la gate di [ADR-012], quindi appartiene a una spec.
2. **Riprendere [SPEC-017]**, saltando `spec_start`.
3. Poi **[SPEC-019]**, che è il rischio più alto fra i tre.
4. Poi **[SPEC-020]**.
5. **Non dimenticare [DEBT-025]**: è l'unico debito la cui chiusura rende più difficile riaprire gli altri.

## Receiving Project Lead checklist

- [ ] Rieseguire `cargo test --workspace --all-features` e attendersi **151**.
- [ ] Rieseguire `python sim/tools/published_artifacts.py` e attendersi **PASS**, 103 C10 e 8 C11.
- [ ] Rieseguire `python sim/tools/published_artifacts_negative.py` e attendersi **15 mutazioni su 11 classi più tutte e 103 le probe**.
- [ ] Verificare che [SPEC-017] sia in `working` e che nessuno ci stia lavorando.
- [ ] Leggere le tre cose in cima a questo documento **prima** di scrivere una review.

## Handoff outcome

> Compilato dal Lead entrante.
