---
id: REVIEW-025
# Note: Quote the title if it contains a colon
title: "Review of SPEC-016"
status: changes-requested
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-016
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-025-EVENT-001"
    timestamp: "2026-08-26T01:12:42.401196400+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Changes requested con due finding, nessuno a carico dell'implementazione. Il lavoro e' corretto in tutto cio' che il Lead ha potuto riverificare rieseguendo: 147 test da una baseline di 126, published_artifacts.py PASS su 91 probe, prova in negativo su dieci classi, ogni valore pubblicato riprodotto e nessuno mosso.\n\nRF-001 medium: SECURITY.md porta la dichiarazione che un terzo bloccante puo' allungare la durata reale di un'epoca, e nomina una sola direzione. Con reward_epoch derivato da height, comprimere moltiplica l'emissione reale, quindi il paragrafo e' ora incompleto nella direzione che questo stesso lavoro ha aperto. Il secondo difetto e' quello grave: l'inventario di published_artifacts.toml copre i cinque documenti di protocollo, la guida e recurring-defects.md, e SECURITY.md non c'e'. La gate e' passata verde su 91 probe mentre l'artefatto che GitHub espone dalla scheda Security restava fuori dal suo insieme. E' la famiglia 3 applicata alla gate stessa, ed e' la terza volta su questo progetto che accade a una gate; le altre due sono del Lead.\n\nRF-002 low: la riga di DEBT-013 che dice che la direzione del pericolo e' il rallentamento e non l'accelerazione e' ora falsa e non incompleta. L'implementatrice l'ha segnalata invece di correggerla in silenzio; la correzione e' del Lead perche' il debito e' di AGENT-007.\n\nDue cose vanno registrate a carico del Lead. Il rifiuto del punto 3 e' corretto e la spec aveva torto: un limite di mandato in millisecondi di catena si evade scrivendo incrementi di timestamp piccoli a piacere, ed e' la famiglia 3 commessa dentro la spec che esisteva per non commetterla. Quarta volta che un agente ha ragione contro il Lead. E la conseguenza nuova - accelerare moltiplica l'emissione - non e' una correzione della valutazione di AGENT-007, che era vera quando l'ha scritta: e' la chiusura di DEBT-019 a renderla falsa.\n\nCorretta anche l'asimmetria fuori banda, che non era chiesta ed e' la parte migliore: il client fallisce chiuso sul lato veloce e segnala sul lato lento perche' un client non allineato conta meno blocchi, quindi la sua misura e' distorta verso il basso e solo verso il basso."
    evidence_refs: ["SPEC-016", "ADR-012", "ADR-013", "DEBT-013", "DEBT-014", "DEBT-019"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [review]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned pending -> changes-requested"
---
# Review

## Outcome

**Changes requested, con due finding, e nessuno dei due è a carico dell'implementazione.** Il lavoro consegnato è corretto in tutto ciò che ho potuto riverificare, e in due punti **corregge il Lead**. I finding riguardano un artefatto pubblicato che questa spec ha reso incompleto e che la gate di [ADR-012] **non era in grado di vedere**.

Il risultato che vale più della chiusura dei tre debiti è che la gate di [ADR-012], eseguita e verde su 91 probe, è passata mentre `SECURITY.md` — l'artefatto che GitHub espone dalla scheda *Security*, il primo che un ricercatore esterno legge — restava fuori dal suo inventario.

## Acceptance-criteria compliance

Riverificato dal Lead rieseguendo, non letto dall'evidenza:

- `cargo test --workspace --all-features`: **147 passati, 0 falliti**, da una baseline di 126.
- `published_artifacts.py`: **PASS**, 91 candidati C10-PROBE.
- `published_artifacts_negative.py`: **PASS**, dieci classi di difetto ciascuna osservata fallire.
- `protocol_hashes.py`: **PASS**, ogni valore pubblicato riprodotto. Nessun valore pubblicato è cambiato in questa passata, il che è coerente: non sono state introdotte preimmagini né toccate fixture, e `CadenceBand` non entra in alcun hash.

**La chiusura di [DEBT-013] ha la forma che [ADR-013] lascia disponibile e nessun'altra.** `README.md` dice esplicitamente *«No rule of this protocol prevents that, and none can»*, ed è ora una probe della gate. Da nessuna parte compare la parola «risolto» o «impedito».

**L'asimmetria del comportamento fuori banda è la parte migliore del lavoro, e non era chiesta.** Il client fallisce chiuso sul lato veloce e segnala sul lato lento, perché un client non allineato conta meno blocchi di quanti la catena ne abbia prodotti: la sua misura è distorta verso il basso *e solo verso il basso*. Una lettura lenta non è quindi attribuibile alla catena da quella posizione; una veloce sì, perché il ritardo di sync non fabbrica blocchi. Il processo di rilascio dei checkpoint, che non ha ritardo di sync e può aspettare, fallisce chiuso in entrambi i versi. È un ragionamento sulla **posizione dell'osservatore** che la spec non conteneva.

**[DEBT-019] si chiude con l'esito forte.** `reward_epoch` è derivato da `height` — verificato in `cadence.rs`: `check_mint_reward_epoch` impone `(e+1) * reward_epoch_blocks <= h`, e `reward_epoch_blocks` è `reward_epoch_ms.div_ceil(block_interval_ms)`, costante di genesi. `height` è `previous + 1`, l'unica grandezza di questa catena che un validatore non scrive liberamente. Il limite che ne discende è enunciato **stretto quanto è vero**: per blocco, non per millisecondo reale.

## Code observations

**La correzione del superlativo di [DEBT-014] è esatta, e il superlativo l'aveva scritto il Lead.** Il debito diceva *«l'unica preimmagine a dominio separato non legata a `chain_id`»*, e la formulazione viene dall'inventario di [SPEC-010]. È falsa. Riverificata la correzione per esaurimento: `dht_namespace_key` esiste in `core/coblox-core/src/registry.rs:318` e prende **solo** `genesis_block_id`; `object_id` e `input_hash` omettono `chain_id` per necessità, perché sono indirizzi di contenuto e devono avere lo stesso nome ovunque; `account_key` è in `ledger.md:2329-2330`. Il documento scrive ora la **classe vera** — *una preimmagine su un oggetto di consenso specifico della catena che altri oggetti di consenso nominano per hash* — ed enumera i diciotto membri che portano `chain_id_32`.

**L'argomento falso non compare.** Il Lead aveva scritto nel debito che l'eccezione si giustificava perché *«è una lista di chiavi»*. Al suo posto c'è l'argomento corretto: un `ValidatorSet` è già legato alla propria catena **dai propri byte**, tre volte, e ogni oggetto che nomina un set per hash ha contenuti che differiscono fra catene.

**Un'aggiunta non richiesta che chiude il difetto di [REVIEW-017] tornato disponibile:** le due misure validano la `CadenceBand` come primo atto. Una banda con `min_ms_per_block = 0` ammetterebbe qualunque ritmo — esattamente come `RewardBounds::validate`, che esisteva e che nessuno chiamava.

## Tests and verification

`GATE-MEASURE-BINDS`, `GATE-NO-TIMESTAMP-RULE`, `GATE-BOTH-DIRECTIONS` e `GATE-ADR012` sono soddisfatte, e tutte e quattro sono state **provate in negativo**.

Due cose meritano di essere registrate.

**La gate di [ADR-012] non è passata alla prima esecuzione**, e la trascrizione lo riporta invece di nasconderlo. Aveva trovato che il collegamento a [DEBT-013] pinnato da una probe era stato rimosso da `README.md`. La sostituzione è migliore dell'originale: la probe nuova pinna **la rinuncia** invece del debito, perché è quella che deve sopravvivere — un lettore che trova la cadenza misurata e non trova quella frase legge la misura come applicazione.

**`GATE-NO-TIMESTAMP-RULE` è stata trasformata da divieto in prosa a guardia eseguibile.** Il test `the_cadence_module_never_reads_a_chain_written_clock` legge il sorgente del modulo, ne toglie i commenti, e fallisce se `timestamp_ms` compare nel codice eseguibile. Provata in negativo introducendo l'orologio della catena senza cambiare la firma della funzione. Un divieto che nessuno esercita è la famiglia 4; questo lo esercita.

## Review findings

**RF-001 — medium — `SECURITY.md` porta una pretesa che questa spec ha reso incompleta, e la gate di [ADR-012] non può vederlo.**

Sono due difetti e il secondo è quello grave.

`SECURITY.md` §*Known limitations* dice che un terzo bloccante può **allungare** la durata reale di un'epoca, e nomina come conseguenze l'incumbency e il ritardo effettivo di una revoca. Nomina **una sola direzione**. Con `reward_epoch` derivato da `height`, **comprimere moltiplica l'emissione reale**: il paragrafo è ora incompleto nella direzione che questo stesso lavoro ha aperto, e cita un debito che questa spec chiude.

Il secondo difetto è la ragione per cui questo è un finding e non una svista. L'inventario di `sim/tools/published_artifacts.toml` copre `README.md`, `identity.md`, `ledger.md`, `wire.md`, `app-manifest.md`, l'`index.html` della guida e `recurring-defects.md`. **`SECURITY.md` non c'è.** La gate ha quindi misurato l'insieme sbagliato: è la **famiglia 3 applicata alla gate stessa**, e su questo progetto è la **terza volta** che accade a una gate. Le altre due sono del Lead.

**RF-002 — low — la riga di [DEBT-013] che questo lavoro ha reso falsa.**

[DEBT-013] dice: *«La direzione del pericolo è verso il rallentamento, non verso l'accelerazione: […] blocchi più veloci accorciano tutto e favoriscono il ricambio.»* Dopo la chiusura di [DEBT-019] quella frase non è incompleta, è **falsa**. L'implementatrice l'ha segnalata invece di correggerla in silenzio, che è la scelta giusta: il debito è di AGENT-007 e la correzione spetta al Lead, con conferma in `GATE-SECREVIEW`.

## Required follow-up

RF-001 è rimandata all'implementatrice come remediation dentro il giro di review: aggiungere `SECURITY.md` all'inventario con le probe che pinnano le sue dichiarazioni portanti e **la prova in negativo per ciascuna**, correggere il paragrafo sulla cadenza perché nomini entrambe le direzioni, e **verificare il resto del file** — nessuna riga di `SECURITY.md` ha mai avuto una prova, quindi non si può presumere che la cadenza sia l'unica imprecisione.

RF-002 è del Lead.

**Tre residui dichiarati dall'implementatrice e accettati come fuori scopo:** `BLOCK_INTERVAL_SECONDS = 5 # assumption` in `sim/coblox_sim/recommended.py`; le due liste chiuse di `what a light client can establish`, che trattano fatti di composizione e non grandezze calcolate; e i **valori della banda**, che sono una decisione dell'operatore come `α` ed è corretto che non siano stati scelti da un agente.

## Final decision

**Changes requested su RF-001.** Nulla è a carico dell'implementazione.

Due cose vanno registrate perché riguardano il Lead e non lei.

**Il rifiuto del punto 3 è corretto e la spec aveva torto.** La spec chiedeva un secondo limite in millisecondi di catena accanto a quello in blocchi. I millisecondi di catena sono `timestamp_ms`, scritto dai validatori con la sola mediana degli undici: un limite di mandato denominato lì si evade scrivendo incrementi piccoli a piacere. È la famiglia 3 commessa **dentro la spec che esisteva per non commetterla**, e la spec la registrava come «rischio secondario» quando era la conclusione. È la quarta volta su questo progetto che un agente ha ragione contro il Lead.

**La conseguenza nuova non era prevista da nessuno**, e questo va detto con precisione: non è una correzione della valutazione di AGENT-007, che era vera quando l'ha scritta. È la chiusura di [DEBT-019] a renderla falsa. Il pericolo aveva una direzione sola finché `reward_epoch` non era derivato; derivandolo da `height` ne acquista due, e le due conseguenze **non si scambiano fra loro** — il lato lento è l'incumbency, il lato veloce è l'emissione. È la ragione per cui la banda è a due lati, e va portata ad AGENT-007 prima di `GATE-SECREVIEW`.
