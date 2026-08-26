---
id: REVIEW-029
# Note: Quote the title if it contains a colon
title: "Security review of SPEC-017 (GATE-SECREVIEW): il segnaposto di genesi, il contesto della preimmagine e la finestra in cui le due meta si annullano"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-017
reviewer: AGENT-007
review_requested_by: AGENT-LEAD
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-029-EVENT-001"
    timestamp: "2026-08-26T11:20:26.356295+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Security review di AGENT-007 su SPEC-017, GATE-SECREVIEW. Tre medium, tre low, nessun high. Il Lead registra il verdetto raccomandato dalla reviewer e lo condivide dopo aver riverificato i due portanti sui file.\n\nRF-002 e' il finding che porta il peso e non sta in nessuna delle due meta': sta nel fatto che comporle apre una finestra che nessuna delle due guarda. DEBT-020 chiude la circolarita' rendendo chain_id una costante nota e uguale su ogni catena nella genesi; DEBT-021 definisce il contesto come la coppia dominio-chain_id. Composti, dentro la finestra di genesi binds() degrada a un controllo di solo dominio. Verificato dal Lead: consensus_key_binding_preimage prende il chain_id e lo passa, ma alla genesi quel chain_id e' il segnaposto identico ovunque, e il corpo dell'oggetto non porta network_id. Due reti con la stessa voce firmata producono la preimmagine byte per byte identica, e si perde l'attribuzione del consenso del validatore, che e' la sola cosa che quella firma esiste per provare.\n\nRF-001 verificato: verify_in_context ha un solo chiamante in src/, ed e' identity.rs su un oggetto non di consenso, mentre verify_consensus_ed25519 e' riesportata alla radice e SignatureVerifier::verify e' pubblica. E' la forma di REVIEW-022, aggravata dal fatto che il rimedio e' gia' nello stesso codice applicato alla scappatoia gemella. Non chiuso qui perche' il recinto giusto dipende dal primo chiamante di consenso, che non esiste: aperto come DEBT-029.\n\nRF-003: il paragrafo What the placeholder does not buy e' falso in due versi, e GEN-0/GEN-1 pubblicate dodici righe sopra dalla remediation precedente sono il controesempio.\n\nRF-004, RF-005, RF-006 low, tutti reali. RF-006 in particolare: il numero 51 e' onesto ma il perimetro no, perche' le dodici preimmagini di firma - cioe' la popolazione di DEBT-021 - non erano censite, e RF-002 e' la prova che il buco costa.\n\nConfermato che GEN-1 non e' famiglia 1 e che il rifiuto del file terzo non pubblicato era corretto. Confermato DEBT-028 high, con la condizione di chiusura contestata dalla reviewer e corretta dal Lead."
    evidence_refs: ["SPEC-017", "DEBT-029"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-029-EVENT-002"
    timestamp: "2026-08-26T11:20:33.162304900+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Remediation dei sei finding consegnata da AGENT-001. RF-002 chiuso legando network_id al payload del key binding a ogni altezza e non solo alla genesi; RF-003 riscritto e il rifiuto rimotivato sull'enumerazione dei dodici domini di firma; RF-004, RF-005, RF-006 chiusi; RF-001 nominato e puntato a DEBT-029. Da verificare dal Lead."
    evidence_refs: ["SPEC-017"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-029-EVENT-003"
    timestamp: "2026-08-26T11:20:55.192491300+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificata dal Lead rieseguendo: 165 test da 151 di baseline, published_artifacts.py PASS con 116 candidati C10, prova in negativo PASS con 15 mutazioni su 11 classi piu' tutte e 116 le probe individualmente, protocol_hashes.py PASS senza valori mossi, verifier.rs con zero rimozioni.\n\nRF-002 chiuso col rimedio piu' caro dei tre, dopo aver verificato la condizione di stop prima di toccare il codice: nessun valore pubblicato si muove, perche' key_binding_signature non compare in alcuna riga di valore del manifesto. Il network_id entra nell'oggetto a ogni altezza e non solo alla genesi, con la ragione giusta - una forma che cambia a un'altezza e' una forma da sbagliare - e non e' campo del ValidatorSet: il verificatore lo prende dalla stessa ancora di fiducia da cui prende chain_id. La prova in negativo riproduce il finding in albero e i 32 byte zero del segnaposto sono visibili nel payload che coincideva.\n\nRF-003: il rifiuto del segnaposto derivato regge ma per una ragione nuova, e l'implementatore ha registrato il cambio invece di riscrivere in silenzio, perche' e' il paragrafo che dichiara un residuo di sicurezza. L'enumerazione dei dodici domini di firma e' fatta contando e non a impressione, e coblox-consensus-key-binding-v0 era l'unico vuoto. Il residuo e' scritto per quello che e': l'attribuzione e' al livello del nome di rete, ed e' un soffitto e non un'omissione, perche' ogni candidato migliore sarebbe o il chain_id che si sta derivando o un secondo nome.\n\nRF-005 chiuso scegliendo la regola invece del caveat - il blocco di altezza 0 non porta transazioni - con la motivazione che una regola che vale per tre generi su cinque non e' una regola.\n\nDue cose vanno registrate perche' l'implementatore poteva tacerle.\n\nIl lint di contenimento lo ha colto: la prima stesura di RF-001 nominava il costruttore gemello e non_consensus_containment.py ha fallito. Ha riformulato la prosa invece di allargare la guardia, e ha scritto nel commento che la frase non puo' nominarlo come dimostrazione stessa. Allentare un recinto che funziona per far stare un paragrafo piu' bello sarebbe stato lo scambio sbagliato.\n\nE ha introdotto un byte NUL in README.md riscrivendo RF-003, una escape interpretata invece che scritta, trovato perche' grep ha risposto \"Binary file matches\". Corretto, e il Lead ha verificato indipendentemente ogni file toccato e ogni file non tracciato: zero NUL. Lo ha riportato perche' nessuna gate lo avrebbe visto - published_artifacts.py legge il file come testo e tutte le probe hanno continuato a corrispondere. E' esattamente la classe che questo progetto censisce: il difetto era in albero e nessuno lo stava guardando.\n\nImprecisione minore senza conseguenze, registrata perche' su questo progetto i numeri contano: il rapporto dice git diff --numstat su verifier.rs uguale a 55 0, la misura del Lead da' 59 0. La parte portante - zero rimozioni - regge in entrambe."
    evidence_refs: ["SPEC-017", "REVIEW-029"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-029-EVENT-004"
    timestamp: "2026-08-26T11:21:13.112785100+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "GATE-SECREVIEW soddisfatta. Sei finding, tutti chiusi o trasferiti su debito, e verificati dal Lead rieseguendo.\n\nIl contributo che vale oltre la spec e' il modo in cui RF-002 e' stato trovato. Non sta in DEBT-020 ne' in DEBT-021: sta nel fatto che comporli apre una finestra che nessuna delle due meta' guarda. E' una classe di difetto che questo progetto non aveva ancora censito - non un artefatto che insegna una forma inammissibile, non una pretesa rimasta indietro, non una grandezza sbagliata, non una clausola non esercitata, ma due chiusure corrette la cui composizione degrada una difesa. Vale la pena tenerla a mente per recurring-defects.md.\n\nRegistrata anche una deviazione dichiarata e accettata: l'implementatore ha modificato .github/workflows/ci.yml, che non era nella lista dei file della spec, per cablare la seconda strada di GATE-TWO-DERIVATIONS nella pipeline. Il Lead l'ha letta e l'accetta con la ragione che l'implementatore da': uno strumento che gira solo quando qualcuno se lo ricorda e' una trascrizione e non una guardia. E' la stessa cosa che SKILL-004 scrive al passo 3.\n\nCosa la reviewer ha attaccato senza romperlo, e va registrato perche' e' informazione: il checkpoint ad altezza 0, cioe' il caso che l'implementatore dichiarava ingannevole, che e' risultato la clausola meglio difesa delle tre; il conflitto fra il segnaposto e il chain_id a zero di HASH-0, che il documento aveva gia' anticipato invece di difendere dopo; state_root come quinto canale mancante, ipotesi falsa perche' le foglie sono separate per tag byte; un mint di genesi come rottura pulita della circolarita', impossibile per il pavimento di liquidazione; una divergenza fra binds() e i byte, impossibile perche' stessi argomenti nella stessa espressione su campo privato; e la matrice 4x4 cercandovi la costante di SKILL-001 passo 4 - il file la aspettava, dichiara che nessuna grandezza e' tenuta costante, ed e' la gate meglio costruita della spec."
    evidence_refs: ["SPEC-017", "DEBT-020", "DEBT-021", "DEBT-028", "DEBT-029"]
    implementation_agent: "AGENT-001"
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [review, security]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-26
    action: "recorded review remediation"
  - date: 2026-08-26
    action: "recorded review remediation-verification"
  - date: 2026-08-26
    action: "transitioned changes-requested -> accepted"
---
# Review

## Outcome

**Raccomandazione: changes requested. Nessun finding `high`; tre `medium` e tre `low`.**

Il lavoro è solido e in due punti è migliore di quanto la spec chiedesse: la regola di genesi è formulata sul **criterio** invece che sull'elenco, e il censimento delle derivazioni non univoche è un ritrovamento vero. La remediation di [REVIEW-028] RF-001 — pubblicare le righe **a zero** e far nominare a ciascuna strada il proprio valore contro il documento sbagliato — è il metodo migliore che io abbia visto usare su questo progetto per provare un accordo invece di asserirlo.

Il peso di questa passata sta però in un punto che nessuna delle due metà guarda, **perché è quello in cui le due metà si incontrano**. [DEBT-020] chiude la circolarità rendendo `chain_id` **una costante nota su ogni catena** dentro la finestra di genesi. [DEBT-021] definisce il contesto di una preimmagine come **la coppia (dominio, `chain_id`)**. Le due chiusure sono corrette una per una e, composte, producono una finestra in cui `PreimageContext` **non separa più due catene**: dentro la genesi la metà «catena» del contesto è `32 byte zero` per tutti, e `binds()` degrada a un controllo di solo dominio. Una firma di genesi di una rete è **byte per byte utilizzabile** sulla genesi di un'altra, e il criterio di accettazione — *«una preimmagine costruita per un dominio o una catena non è utilizzabile in un altro»* — è falso in quella finestra.

Il documento **sa** che esiste un residuo e lo dichiara, il che è merito. Ma lo descrive con una premessa più stretta del vero, e la coppia `GEN-0`/`GEN-1` che lo stesso documento pubblica nella remediation è il controesempio che la smentisce.

## Acceptance-criteria compliance

Non ho rieseguito le gate: il Lead le ha rieseguite due volte e [REVIEW-028] ne porta le trascrizioni. Ho fatto ciò che `lead-claims-discipline.md` chiede a chi rivede dopo: **attaccare invece di rieseguire**. Ciò che ho verificato sui file, e non sulle trascrizioni:

- `verify_consensus_ed25519` è **invariata**: `verifier.rs` contiene solo `verify_in_context` in aggiunta, sopra `ConsensusVerifier`, e nessuna riga della funzione di verifica è toccata. `GATE-VERIFIER-UNCHANGED` regge.
- `signing_preimage` scrive il contesto **dagli stessi argomenti** con cui compone i byte, quindi contesto e byte non possono divergere per costruzione. La forma è corretta.
- Il rifiuto del parametro di tipo sul dominio è motivato bene e lo confermo nel merito: `chain_id` è un valore, sposterebbe metà del controllo, e renderebbe `SignatureVerifier` non più `dyn`.
- I 51 nomi della tabella `preimage` di `published_artifacts.toml` sono effettivamente 51 e il censimento li copre uno per uno. Il numero non è gonfiato. Il **criterio** ha un problema, e il **perimetro** un altro: RF-006.

Il criterio di accettazione che **non** considero soddisfatto è il quarto, per RF-002.

## Domanda 1 — Il segnaposto a 32 byte zero è sicuro? La domanda è davvero meccanica?

**Sì come domanda, non ancora come enumerazione.**

Il confine — *questo valore è un ingresso di `genesis_block_id`, direttamente o attraverso un hash che l'intestazione di altezza 0 porta?* — è decidibile senza giudizio per **qualunque valore già dato**. Ho enumerato i campi dell'intestazione che sono hash: `previous_block_id` (fissato a zero da `ledger.md`, quindi non un canale), `transactions_root`, `state_root`, `validator_set_hash`, `next_validator_set_hash`, `consensus_parameters_hash`. Il verso «col segnaposto» dell'elenco copre quattro di questi cinque canali.

**Il quinto è `state_root`, e l'omissione è corretta ma non detta.** Le foglie dell'albero degli account — `node_leaf`, `app_leaf`, i rami, i vuoti — sono separate per **tag byte** e non portano `chain_id`, quindi nulla sotto `state_root` può entrare in conflitto col segnaposto. Il censimento lo dice (*«lo stato di genesi non è derivato affatto: è dichiarato»*); il documento normativo no. È corretto e va bene così: non è un finding, è la ragione per cui l'enumerazione è più corta di quanto la mia lettura si aspettasse.

**L'attacco al caso che l'implementatore dice inganni riesce a metà, e non nel verso che lui teme.** Il checkpoint di soggettività debole emesso ad altezza 0 **non** è materiale di genesi, e la ragione data è meccanica e verificabile: nessun campo dell'intestazione lo nomina, non passa per `transactions_root`, non passa per `state_root`. Ho cercato una via per cui un checkpoint diventi ingresso di `genesis_block_id` e non ne esiste. La clausola regge, e il documento la difende anche dal verso opposto — l'osservazione che leggere le righe del registro come la genesi di una catena renderebbe `WSC-0` inammissibile è esattamente la domanda di famiglia 1 (*questo stesso valore compare in due ruoli che il protocollo richiede distinti?*) fatta prima che qualcuno la ponesse. **Non si è rotta.**

**Ciò che si rompe è il verso «col `chain_id` derivato», e attraverso le transazioni.** Vedi RF-005.

## Domanda 2 — `PreimageContext` chiude davvero [DEBT-021]?

**Chiude la parte che dichiara di chiudere, e la chiude bene. Non è imposto da nulla.**

`verify_in_context` è una funzione libera e non un metodo di default, e la ragione scritta accanto è corretta e non retorica: un default si sovrascrive, e chi lo sovrascrivesse toglierebbe il controllo conservandone il nome. **Nulla che un implementatore scriva può indebolire quella funzione.** Confermo.

Ma la domanda che il Lead pone è l'altra, ed è la giusta: *esiste un percorso che raggiunga `verify_consensus_ed25519` senza passare da `verify_in_context`?* **Sì, e non uno solo.**

```
core/coblox-core/src/lib.rs:125
  pub use verifier::{ConsensusVerifier, verify_consensus_ed25519, verify_in_context};
```

`verify_consensus_ed25519` è `pub` **e riesportata alla radice del crate**. `SignatureVerifier::verify` è un metodo pubblico di un trait pubblico che `ConsensusVerifier` implementa pubblicamente. Un chiamante di consenso futuro raggiunge la verifica per **due** vie che non toccano il contesto, e nessuna guardia lo dice: nessun `feature` gate, nessun confine di compilazione, nessuna guardia testuale in CI, nessun test che asserisca chi usa cosa.

**È la forma esatta che [REVIEW-022] ha trovato con `pub(crate)`: un legame nominale.** Ed è aggravata dal fatto che **lo stesso file ha già il rimedio, applicato all'altra scappatoia.** `SigningPreimage::from_raw_bytes_non_consensus` è dietro una feature non di default **e** dietro `sim/tools/non_consensus_containment.py`, che fallisce se il nome compare fuori da `registry.rs` e `tests/`. Le due scappatoie sono sorelle — una lascia entrare byte senza contesto, l'altra lascia uscire una verifica senza contesto — e una ha due recinti mentre l'altra ne ha zero. RF-001.

Va aggiunto un dato che pesa sulla severità nei due versi. Oggi `verify_in_context` ha **un solo chiamante in tutta la base di codice**, `identity.rs:232`, ed è un'attestazione di **chiave di trasporto**, cioè un oggetto **non** di consenso; `light_client.rs:119` dichiara che questo crate non spedisce alcun verificatore di firme. Quindi: nessun percorso di consenso oggi *bypassa* il punto d'ingresso verificato — perché nessun percorso di consenso esiste ancora. La disciplina è una convenzione con un esempio, e l'esempio è fuori dal consenso. Il commento a `identity.rs:230` dice la cosa giusta (*«la forma che un lettore copia è la forma che avrà il prossimo chiamante»*) e per questo la convenzione va meccanizzata **adesso**, che costa dieci righe di Python, e non dopo il primo chiamante che ha fretta — che è il criterio con cui la spec stessa ha scelto la forma del legame.

## Domanda 3 — La regola di genesi apre una superficie nuova?

**Sì, e questa è la parte che porta il peso.**

Il documento pone la domanda e risponde, in *What the placeholder does not buy*. La risposta è nella direzione giusta e **il perimetro è sbagliato in entrambi i versi**.

Il documento dice:

> two networks whose genesis material is identical byte for byte and which differ only in `network_id` produce the same `genesis_block_id` and the same genesis signatures

**Primo verso: la premessa non può realizzarsi, ed è la remediation stessa a dimostrarlo.** `network_id` è un campo dell'intestazione di altezza 0 (`ledger.md:603`). Due reti che differiscono in `network_id` hanno intestazioni di genesi **diverse** e `genesis_block_id` **diversi**. `GEN-0` e `GEN-1` sono letteralmente quelle due reti — *«`GEN-1` è `GEN-0` con un campo cambiato: `network_id`»* — e i loro `block_id` di genesi pubblicati sono `sha256:1334f536…` e `sha256:6b625392…`. Il paragrafo normativo e la coppia di fixture pubblicata dodici righe sopra si contraddicono.

**Secondo verso, ed è quello che conta: la condizione vera è molto più larga, non più stretta.** Non serve che il materiale di genesi coincida. Serve soltanto che **il payload firmato** coincida. Il `key_binding_signature` firma (`ledger.md:774-776`):

```
"coblox-consensus-key-binding-v0" || 0x00 || chain_id_32
|| JCS({activation_height, consensus_public_key, node_id, validator_id})
```

Alla genesi `chain_id_32` è la costante, e le quattro chiavi dell'oggetto **non contengono né `network_id` né alcun identificatore di catena**. Il `ValidatorSet` stesso non ha campo `network_id` (`ledger.md:709-720`). Quindi: **il `key_binding_signature` di un validatore alla genesi è valido su ogni rete la cui genesi seggia quello stesso validatore**, quali che siano il resto della coorte, i parametri, lo stato e il nome della rete.

Riproduzione, eseguita fuori dal repository:

```text
$ python <scratchpad>/rf_genesis_replay.py
network A genesis key_binding preimage sha256: 5aff496fa9a55b1e0d4a33bcab6cb051d38ed02a44693477b7ae819279757aa8
network B genesis key_binding preimage sha256: 5aff496fa9a55b1e0d4a33bcab6cb051d38ed02a44693477b7ae819279757aa8
byte-identical: True

post-genesis, chain A: c1c629cc971ed93217e942fbc20f2efcf89a2630208188bc36c50f9ed568ff94
post-genesis, chain B: 1b46b0b17823ffea3185ec348c1630199a990ea5add0a2fa2a1d799b1aa4f673
byte-identical: False
```

(Due reti, **materiale di genesi diverso**, stessa voce di validatore. Il contrasto con le due righe di sotto è la misura esatta di ciò che il segnaposto costa: da altezza 1 in poi il legame c'è.)

**La conseguenza sulla metà [DEBT-021] della spec è quella che nessuno dei due documenti nomina.** `binds()` confronta `(dominio, chain_id)`. Dentro la finestra di genesi `chain_id` è `ChainId::GENESIS_PLACEHOLDER` per **tutte** le catene, quindi:

```rust
let p = signing_preimage(Domain::SIG_CONSENSUS_KEY_BINDING, &ChainId::GENESIS_PLACEHOLDER, payload);
p.binds(Domain::SIG_CONSENSUS_KEY_BINDING, &ChainId::GENESIS_PLACEHOLDER)  // true, su qualunque rete
```

Il contesto degrada a **solo dominio**. Il tipo che la spec ha introdotto per impedire che una preimmagine di una catena valga in un'altra non lo impedisce esattamente nella finestra che l'altra metà della stessa spec ha creato. RF-002.

**Cosa questo non è.** Non è una rottura del consenso e non produce una catena falsa: il set di genesi arriva da un canale di distribuzione autenticato, e il documento lo dice. Ciò che si perde è la **attribuzione del consenso del validatore**. Il `key_binding_signature` esiste per provare che la chiave d'identità di V ha acconsentito a legare la propria chiave di consenso; alla genesi quella prova **non nomina la catena**, quindi un distributore di rete B può insediare V nella propria genesi riusando la firma pubblicata di V nella genesi di A, senza che V abbia mai visto B. Il paragrafo del documento dice *«non è evidenza di quale catena»* — vero — ma lo motiva con una condizione (materiale di genesi identico) che fa sembrare il caso teorico, mentre è a costo zero.

**La chiusura che il documento respinge resta respinta, e ho controllato l'argomento.** Il segnaposto derivato dalla rete — `H("coblox-chain-id-v0\0" || u32be(len) || network_id_utf8 || 32 byte zero)` — chiuderebbe RF-002 alla radice, e la ragione del rifiuto è buona: due grafie di *non c'è ancora tale valore* dentro un oggetto solo, accanto a `previous_block_id`, sono una cosa da sbagliare. **Non contesto il rifiuto.** Contesto che sia motivato contro un perimetro sbagliato: *«ciò che la seconda grafia comprerebbe è limitato dal paragrafo sopra»*, e il paragrafo sopra sottostima il residuo. Il rifiuto va **riconfermato contro il perimetro vero**, e se regge — credo regga — il residuo va scritto per quello che è.

## Domanda 4 — `GEN-1` insegna qualcosa di inammissibile?

**No. L'argomento dell'implementatore regge e lo confermo senza riserve.**

Ho applicato la domanda di famiglia 1 — *quale artefatto pubblicato questa regola nuova rende inammissibile, fra quelli che non sto toccando?* — e la sua variante più affilata, *questo stesso valore compare in due ruoli che il protocollo richiede distinti?*

`GEN-1` è `GEN-0` con una sostituzione di stringa. Non introduce alcuna forma nuova: stessi `schema_version`, stesso corpo di `PD-0`, stessi `previous_block_id`, `transactions_root`, `state_root`, `validator_set_hash`, `next_validator_set_hash`. Non c'è configurazione che un lettore possa copiare da `GEN-1` e non da `GEN-0`. **Non è famiglia 1**, e la ragione per cui esiste — esercitare `u32be(len(network_id_utf8))` e mostrare che il nome entra sia nell'intestazione sia nella preimmagine di `chain_id` — è quella giusta, dichiarata nel documento e tenuta ferma da una probe C10.

Confermo anche il rifiuto del file terzo non pubblicato: un oracolo fuori dal registro è un oracolo che nessuna implementazione indipendente riceve, cioè il difetto che questa spec esiste per non commettere.

**Una cosa `GEN-1` insegna, e non è inammissibile ma è utile che sia detta.** È la prima coppia pubblicata di **due reti distinte che nominano lo stesso set di validatori di genesi** (entrambe `validator_set_hash = dd…dd`). Sotto la regola del segnaposto quella coincidenza implica che ogni `key_binding_signature` di quel set sarebbe identica sulle due reti. Non è un difetto della fixture — sono letterali dichiarati, non un `ValidatorSet` vero — ma è l'istanza pubblicata di RF-002, e vale la pena che il documento la nomini quando corregge il paragrafo.

## Domanda 5 — [DEBT-028] è classificato bene?

**Confermo il difetto, confermo `high`, e contesto la condizione di chiusura.**

**Il difetto è reale e l'ho verificato indipendentemente sui file.** `ledger.md:947-955` dà `election_boundary_height(e) = e * L` con `L = election_epoch_blocks` *«from the active consensus parameters»*, e nessuna clausola dice attivi a quale altezza. `election_epoch` entra in `election_entropy` (`registry.rs:174`), `election_seed` (`:207`) ed `election_ticket` (`:217`), tutte e tre a dominio separato e legate a `chain_id`. Il contrasto che rende la diagnosi certa è nella stessa base di codice ed è quello che il Lead cita: [SPEC-016] ha chiuso la forma identica per `reward_epoch` **nominando il documento**, e nessun oggetto dell'elezione porta quella cucitura.

E il documento **contempla esplicitamente che `L` cambi**: `ledger.md:1742` discute *«a sitting set publishes a document with `election_epoch_blocks` set to `2^60`»*, e il blocco di vincoli lo limita senza congelarlo. Non è uno scenario ipotetico costruito dalla review: è un atto di governance che il protocollo prevede.

**`high` è la severità giusta.** Il bersaglio è chi può firmare i blocchi, non una spesa; non serve alcun attaccante; e il costo di correzione cresce di ordini di grandezza dopo la prima catena con storia. Non `critical` perché nessuna rete esiste. Concordo con la motivazione del Lead parola per parola.

**Contesto la condizione di chiusura, ed è l'unico punto in cui il debito è formulato più debolmente del vero.** Il criterio scritto è *«prima che esista una seconda implementazione»*, per analogia con [DEBT-022]. **L'analogia non tiene, perché questo difetto colpisce anche con una sola implementazione.** Se *«i parametri attivi»* si legge come *attivi ora* — la lettura naturale, e quella che il documento non esclude — allora dopo un cambio di `L`:

- un nodo online da prima del cambio ha attribuito l'altezza 5000 all'epoca 50 e ha derivato i suoi tre semi sotto `L = 100`;
- un nodo che **rigioca la catena dalla genesi dopo il cambio** attribuisce la stessa altezza all'epoca 25 e deriva semi diversi;

e i due nodi eseguono **lo stesso binario**. È una divergenza fra due nodi della stessa implementazione, non fra due implementazioni. La condizione di chiusura corretta è quindi la **congiunzione**: prima che esista una seconda implementazione **e** prima che una qualunque rete accumuli storia sufficiente a rendere possibile un cambio di `L` — cioè, in pratica, prima della prima devnet che faccia governance sui parametri di elezione. Raccomando di aggiornare `Resolution criteria` di [DEBT-028] su questo punto: è una restrizione, non un allargamento, e rende la scadenza più vicina di quanto il debito dichiari.

Il resto del debito lo confermo, incluso il *rimedio apparente da non adottare* (rendere `election_epoch_blocks` non governato): sarebbe curare il sintomo cambiando un parametro, [ADR-010].

## Domanda 6 — L'elenco delle cinquantuno derivazioni è completo?

**Il numero è onesto. Il criterio ha un cedimento, e il perimetro un buco dichiarabile in una riga.**

**Il numero, prima di tutto.** Non me ne sono fidato: `grep -c '^\[\[preimage\]\]' sim/tools/published_artifacts.toml` dà **51**, e i 51 identificatori corrispondono uno a uno alle voci che il censimento dichiara di aver classificato. Nessun gonfiaggio.

**Il cedimento del criterio è la classe T.** Il metodo la definisce così: *«univoca **dopo** la trasmissione: la domanda non si pone, perché non c'è derivazione da sbagliare»*. Presa alla lettera, quella frase assorbe qualunque grandezza che un oggetto porti con sé — che è la classe più grande di tutte, e la classe in cui un elenco lungo diventa un elenco vuoto. Il controllo che lo dimostra è dentro la pagina stessa: **le due voci aperte sono entrambe valori trasmessi.** `election_epoch` è un campo dell'`ElectionRecord`; `reward_epoch` è un campo del corpo del mint. Il metodo *come è scritto* le avrebbe classificate T e chiuse; sono state classificate A e T-dichiarata solo perché l'implementatore ha applicato una domanda in più che il metodo **non enuncia**: *esiste una regola di validità che leghi il valore trasmesso a qualcosa di derivabile, e quella regola nomina il documento che fissa il proprio denominatore?* Il giudizio è stato quello giusto; il criterio scritto non lo cattura, e la pagina porta il metodo proprio perché sia riusata da chi non ha fatto questa passata. RF-006(a).

**Il perimetro.** I 51 sono le preimmagini di **hash** del manifesto. Le preimmagini di **firma** — i dodici domini `SIG_*` di `hash.rs:147-169`, fra cui `coblox-consensus-key-binding-v0`, `coblox-ledger-transaction-v0`, `coblox-block-vote-v0`, `coblox-protocol-document-v0` — non compaiono fra i 51 e non sono nella popolazione censita. Non è un'omissione grave in sé, ma è **la popolazione a cui appartiene [DEBT-021]**, cioè metà di questa spec, e la voce 4 dell'elenco (*con quale `chain_id` firma il `key_binding_signature` del set di genesi*) è una preimmagine di firma **trovata di lato**, arrivandoci dall'angolo della genesi anziché dal metodo. Se il metodo avesse coperto le preimmagini di firma, RF-002 sarebbe stato trovato in questa passata invece che in questa review: la domanda *«è derivabile in un solo modo?»* applicata al payload del `key_binding_signature` dà «sì, e nello **stesso** modo su ogni catena», che è precisamente la forma sorella dell'ambiguità. RF-006(b).

La sezione *Ciò che è stato guardato e trovato univoco* è invece esemplare e va lodata: un censimento che elenca solo i propri ritrovamenti non dice dove ha guardato, e questa dice anche dove.

## Review findings

### RF-001 — `medium` — Il legame di contesto non è imposto da nulla, ed è la forma di [REVIEW-022] con il rimedio già presente nello stesso file

`verify_in_context` è il punto d'ingresso verificato **per convenzione**. Due percorsi pubblici raggiungono la verifica saltandolo, e uno dei due è riesportato alla radice del crate:

- `coblox_core::verify_consensus_ed25519(pk, &preimage, &sig)` — `lib.rs:125`;
- `<qualunque V: SignatureVerifier>::verify(pk, &preimage, &sig)` — `lib.rs:145`, implementato da `ConsensusVerifier` a `verifier.rs:95`.

Nessuna feature gate, nessuna guardia testuale, nessun test che asserisca chi usa cosa. La doc di `verify_in_context` dice *«This is the entry point a consensus caller **should** use»*, e *should* è la parola esatta: il legame è nominale, come `pub(crate)` in [REVIEW-022].

**Perché è dello stesso peso della scappatoia sorella.** `from_raw_bytes_non_consensus` è la scappatoia che fa **entrare** byte senza contesto e ha **due** recinti: la feature `conformance-testing` non di default, e `sim/tools/non_consensus_containment.py` che fallisce se il nome compare fuori da `registry.rs` e `tests/`. `verify_consensus_ed25519` è la scappatoia che fa **uscire** una verifica senza contesto e non ne ha nessuno. La spec ha usato il precedente per un pericolo e non per il suo gemello, nello stesso file, senza dirlo.

**Riproduzione.**
```text
$ grep -n "pub use verifier" core/coblox-core/src/lib.rs
125:pub use verifier::{ConsensusVerifier, verify_consensus_ed25519, verify_in_context};

$ grep -rn "verify_in_context" core/coblox-core/src/ | grep -v verifier.rs | grep -v registry.rs
core/coblox-core/src/identity.rs:232:        if !crate::verifier::verify_in_context(
```
Un solo chiamante in tutto `src/`, ed è un'attestazione di chiave di **trasporto**, cioè un oggetto non di consenso. `light_client.rs:119` dichiara che il crate non spedisce alcun verificatore di firme: **non esiste oggi alcun chiamante di consenso**, quindi la convenzione non è nemmeno esemplificata sul percorso che deve proteggere. Per verificare che il bypass compili, in una copia dell'albero fuori dal repository basta un file in `tests/` che chiami `coblox_core::verify_consensus_ed25519` su una preimmagine di dominio arbitrario: compila, passa, e nessuna guardia lo nota.

**Rimedio suggerito, e non è la sola forma possibile.** Estendere `non_consensus_containment.py` con un secondo simbolo: `verify_consensus_ed25519` e `SignatureVerifier::verify` nominati solo in `verifier.rs`, `tests/` e nei documenti che ne parlano; ogni altro sito è un fallimento con la classe nominata. Costa la stessa forma di guardia che il progetto ha già scritto e provato in negativo. In alternativa, o in aggiunta, una nota accanto a `verify_consensus_ed25519` che dica esplicitamente che **non** è il punto d'ingresso di consenso — oggi la sua doc non lo dice, e la doc di `verify_in_context` dichiara che nulla può indebolire *quella funzione*, che è vero e non è la stessa cosa di *nulla può evitarla*.

### RF-002 — `medium` — Dentro la finestra di genesi `PreimageContext` degrada a un controllo di solo dominio, e il quarto criterio di accettazione è falso lì

`binds()` confronta `(dominio, chain_id)`. La regola di [DEBT-020] rende `chain_id` la costante `32 byte zero` **su ogni catena** per tutti i valori che sono ingresso di `genesis_block_id` e per ogni firma su di essi. Quindi la metà «catena» del contesto è la stessa per tutte le reti, e il legame introdotto da [DEBT-021] non separa due catene esattamente nella finestra che [DEBT-020] ha appena definito.

Il caso concreto è il `key_binding_signature`, perché il suo payload non porta alcun identificatore di catena:

```
"coblox-consensus-key-binding-v0" || 0x00 || 00…00 (32)
|| JCS({activation_height, consensus_public_key, node_id, validator_id})
```

**Riproduzione** (fuori dal repository; lo script è in scratchpad, `rf_genesis_replay.py`):

```text
network A genesis key_binding preimage sha256: 5aff496fa9a55b1e0d4a33bcab6cb051d38ed02a44693477b7ae819279757aa8
network B genesis key_binding preimage sha256: 5aff496fa9a55b1e0d4a33bcab6cb051d38ed02a44693477b7ae819279757aa8
byte-identical: True
```

Le due reti hanno `network_id` diversi, `genesis_block_id` diversi e `chain_id` diversi. Condividono **solo la voce di validatore**, e tanto basta. Lato Rust la stessa cosa si osserva senza chiavi:

```rust
let p = signing_preimage(Domain::SIG_CONSENSUS_KEY_BINDING, &ChainId::GENESIS_PLACEHOLDER, payload);
assert!(p.binds(Domain::SIG_CONSENSUS_KEY_BINDING, &ChainId::GENESIS_PLACEHOLDER)); // vero ovunque
```

**Cosa si perde davvero.** Non la sicurezza del consenso: il set di genesi arriva da un canale di distribuzione autenticato. Si perde l'**attribuzione del consenso del validatore**, che è la sola cosa che il `key_binding_signature` esiste per provare. Un distributore della rete B può insediare V nella propria genesi riusando la firma pubblicata di V nella genesi di A. Costo: zero. Prerequisito: che V sia seduto alla genesi di una rete la cui distribuzione è pubblica — cioè la condizione normale.

**Nota di corredo, dello stesso ceppo.** `hash.rs:289-292` dice *«This is a constant and not a `ChainId` a caller may keep»*, ma `GENESIS_PLACEHOLDER` è un `pub const` di un tipo `Copy`: un chiamante **può** tenerlo, e nulla al livello del tipo distingue il segnaposto da un `chain_id` derivato. È la stessa affermazione di prosa smentita dal tipo che [REVIEW-023] ha già visto una volta su questo tipo. Vale come nota, non come finding separato.

**Rimedio.** Almeno uno dei tre, e la scelta è del Lead:
1. accettare il residuo e **scriverlo per quello che è** (vedi RF-003), rendendo esplicito che dentro la finestra di genesi `binds()` è un controllo di dominio e che il documento lo sa;
2. riaprire, contro il perimetro corretto, la scelta del segnaposto derivato dalla rete che il documento respinge — con l'avvertenza che il motivo del rifiuto (una seconda grafia di *non c'è ancora tale valore* dentro un oggetto solo) resta valido e che io **non** raccomando di ribaltarlo alla leggera;
3. portare `network_id` nel payload del `key_binding_signature` di genesi, che chiude il caso concreto senza toccare la forma del segnaposto — e che è comunque una regola di validità nuova, quindi una passata di [ADR-012] propria.

Se la scelta è la 1, il quarto criterio di accettazione va riformulato con la sua eccezione dichiarata invece di restare spuntato come è.

### RF-003 — `medium` — Il paragrafo *What the placeholder does not buy* poggia su una premessa falsa, e la coppia `GEN-0`/`GEN-1` dello stesso documento la smentisce

`docs/protocol/README.md:197-208`:

> two networks whose genesis material is identical byte for byte and which differ only in `network_id` produce the same `genesis_block_id` and the same genesis signatures

Due difetti in una frase, in versi opposti.

**(a) La premessa non è realizzabile.** `network_id` è un campo dell'intestazione di altezza 0 (`ledger.md:603`), quindi due reti che differiscono in `network_id` non hanno materiale di genesi identico e **non** producono lo stesso `genesis_block_id`. La prova è nella tabella dello stesso file: `GEN-0` e `GEN-1` differiscono **solo** in `network_id` (README:604) e i loro `block_id` di genesi sono `sha256:1334f536…` e `sha256:6b625392…`. Anche la trascrizione della spec lo asserisce due volte: *«`network_id` enters the header, so `genesis_block_id` moves with it»*.

**(b) La condizione vera è più larga.** Perché una firma di genesi valga su due catene non serve che coincida il materiale di genesi: basta che coincida il **payload firmato**. Per il `key_binding_signature` il payload è una quaterna che non contiene né `network_id` né `chain_id`. Vedi RF-002.

**Riproduzione:** puramente documentale. `README.md:604-614` più le righe di tabella `block_id (genesis)` e `block_id (genesis, GEN-1)` contraddicono `README.md:198-201` senza eseguire nulla.

**Perché è `medium` e non `low`.** È il paragrafo che dichiara il residuo di sicurezza di una regola normativa nuova, ed è l'argomento su cui poggia il **rifiuto** dell'alternativa (il segnaposto derivato dalla rete): *«ciò che la seconda grafia comprerebbe è limitato dal paragrafo sopra»*. Un rifiuto motivato contro un perimetro sbagliato va rimotivato, non solo riscritto. È famiglia 2 — l'affermazione rimasta indietro rispetto alla regola — su un'affermazione **di sicurezza**, e la stessa passata che ha introdotto la regola ha pubblicato il controesempio.

### RF-004 — `low` — La clausola `key_binding_signature` non è espressa in nessun punto del codice, ed è l'unica clausola priva sia di fixture sia di espressione

Il documento dichiara che nessun valore pubblicato esercita questa clausola e ne dà la ragione (`README.md:595-597`): pubblicare un `ValidatorSet` di genesi significherebbe pubblicare una coorte che il blocco di vincoli governa. **Accetto la ragione**, e considero corretto averla dichiarata come famiglia 4 invece di nasconderla. Il difetto è un altro: la clausola non è dichiarata **nemmeno dove i byte vengono costruiti**.

```text
core/coblox-core/src/validator_set.rs:114
pub fn consensus_key_binding_preimage(
    chain_id: &ChainId, activation_height: u64, ...
```

Nessuna nota, nessun caso di genesi, nessun riferimento alla sezione nuova. Il confronto interno lo rende visibile: `registry::genesis_derivation` porta una doc che spiega perché l'intestazione si calcola sotto il segnaposto, `ChainId::GENESIS_PLACEHOLDER` porta la regola per esteso, e `validator_set_hash` porta tre paragrafi sul perché **non** ha un legame di catena — mentre l'unica funzione dell'albero che un chiamante di genesi userà **con il segnaposto** non dice che esiste un segnaposto. È il punto esatto in cui il prossimo implementatore passerà il `chain_id` derivato per un set di genesi, e la conseguenza di quell'errore non è un digest sbagliato che una suite di conformità nota: è un `genesis_block_id` diverso, cioè [DEBT-020] che si riapre invisibilmente.

**Riproduzione:** `grep -n "GENESIS_PLACEHOLDER\|genesis" core/coblox-core/src/validator_set.rs` — zero occorrenze nella regione 82-132.

**Rimedio:** una doc su `consensus_key_binding_preimage` che nomini la clausola e la sezione, e — se non costa una coorte pubblicata — un test in `tests/genesis_derivation.rs` che costruisca in memoria una voce di genesi e asserisca che la preimmagine porta il segnaposto. Non pubblica nulla e toglie la clausola dalla condizione di *normativa sulla sola forza del proprio testo*.

### RF-005 — `low` — L'enumerazione nei due versi non è chiusa: il verso «derivato» contiene hash che una transazione di altezza 0 può nominare

L'enumerazione dichiara normativi entrambi i versi. Il verso «col segnaposto» include *«ogni `tx_id` e ogni firma di autorizzazione di transazione del blocco di altezza 0, e i voti di finalità del suo certificato di quorum se ne porta uno»* — quindi il documento **contempla** che il blocco di genesi porti transazioni. Il verso «col `chain_id` derivato» include *«gli altri tre documenti di protocollo firmati»*, motivato con *«nessun campo dell'intestazione li nomina»*.

Le due clausole si toccano. `transactions_root` **è** un campo dell'intestazione, e alcuni corpi di transazione nominano quei documenti per hash: un `burn` porta `pricing_hash`, cioè `hosting_rate_card_hash` (`ledger.md:394-405`); i corpi `challenge_commitment` e `challenge_evidence` portano valori legati a `chain_id` per costruzione. Un blocco di altezza 0 che portasse una tale transazione renderebbe quel documento un ingresso di `genesis_block_id`: sotto il **criterio** va calcolato col segnaposto, sotto l'**enumerazione** col derivato — e sotto il derivato la circolarità si riapre. Due implementazioni conformi, due `chain_id`.

**Alcune vie sono chiuse per altra strada, e le ho verificate.** Un `mint` è impossibile ad altezza 0, perché il pavimento di insediamento `(e + 1) * reward_epoch_blocks <= h` non è soddisfacibile per `h = 0`. Un `validator_candidacy` è impossibile, perché richiede una finestra di entropia di blocchi precedenti. Restano `burn` e i due di sfida, che nessuna regola vieta e che una distribuzione insolita potrebbe includere.

**La fixture assume il caso via, senza che la regola lo dica.** `GEN-0` e `GEN-1` hanno `transactions_root = H(0x03)`, la radice vuota: il blocco di genesi **non** porta transazioni. È la scelta giusta ed è anche la risposta — ma vive in una fixture e non in una clausola.

**Riproduzione:** documentale. `README.md:169-176` (le due voci) contro `ledger.md:394-405` (`pricing_hash` nel corpo di `burn`) e `ledger.md:600-613` (`transactions_root` campo dell'intestazione).

**Rimedio, una riga:** *«Il blocco di altezza 0 non porta transazioni»* — che è ciò che entrambe le fixture già assumono — oppure, se le transazioni di genesi vanno ammesse, la clausola che dica quali documenti una tale transazione può nominare.

### RF-006 — `low` — Il criterio del censimento: la classe T è enunciata troppo largamente, e il perimetro dei 51 esclude proprio le preimmagini di firma

**(a) La classe T.** *«Univoca dopo la trasmissione: la domanda non si pone»* assorbe qualunque valore che un oggetto porti con sé, che è la classe più numerosa. Il controllo è interno alla pagina: **entrambe** le voci non chiuse — `election_epoch` e `reward_epoch` — sono valori trasmessi, e il metodo *come è scritto* le avrebbe classificate T. Sono state classificate correttamente solo grazie a una domanda che il metodo non enuncia: *esiste una regola di validità che leghi il valore trasmesso a un derivato, e nomina il documento che ne fissa il denominatore?* Il giudizio è stato giusto; il criterio scritto non lo cattura, e la pagina esiste per essere riusata da chi non ha fatto questa passata.

**(b) Il perimetro.** I 51 sono le righe `preimage` del manifesto, cioè le preimmagini di **hash**. I dodici domini `SIG_*` di `hash.rs:147-169` — le preimmagini di **firma**, la popolazione a cui appartiene [DEBT-021] — non sono nella popolazione censita e non è detto che non lo siano. La voce 4 dell'elenco è una preimmagine di firma trovata **di lato**, arrivandoci dalla genesi e non dal metodo. RF-002 è la prova che il buco costa: applicare la domanda del censimento al payload del `key_binding_signature` dà *«derivabile in un solo modo, e nello stesso modo su ogni catena»*, cioè la forma sorella dell'ambiguità, e nessuno l'ha applicata perché quella preimmagine non era nella lista.

**Riproduzione:** `grep -c '^\[\[preimage\]\]' sim/tools/published_artifacts.toml` → `51`; i 51 `id` non contengono alcun dominio `SIG_*`.

**Rimedio:** aggiungere alla classe T la condizione che la rende vera (*un valore trasmesso è univoco solo se una regola di validità lo lega a un derivato e nomina il documento che lo governa; altrimenti è A*), e dichiarare in *Ciò che questa pagina non copre* che il perimetro è quello delle preimmagini di hash del manifesto — oppure estenderlo alle preimmagini di firma, che sono dodici e che questa spec ha appena reso di prima classe nel tipo.

## Ciò che ho attaccato senza riuscire a romperlo

La regola è in `lead-claims-discipline.md` e in [SKILL-001], e vale anche per chi rivede: la sezione che [REVIEW-025] non aveva è quella che è costata un `high` su [SPEC-016]. Riporto gli attacchi che **non** hanno prodotto un finding, perché sapere dove ho guardato è informazione quanto sapere cosa ho trovato.

**Il checkpoint di soggettività debole ad altezza 0, cioè il caso che l'implementatore dice inganni.** È il primo che ho attaccato, come richiesto. Ho cercato una via per cui un checkpoint diventi ingresso di `genesis_block_id`: non è un campo dell'intestazione, non passa per `transactions_root`, non passa per `state_root`, e nessun oggetto che l'intestazione nomina lo nomina a sua volta. La clausola *«è emesso dopo che la catena esiste e non è mai materiale di genesi»* è decidibile senza giudizio, e il verso opposto — un checkpoint che portasse il segnaposto — è chiuso dal documento che dichiara `WSC-0` inammissibile sotto quella lettura. **Non si è rotto**, ed è la clausola meglio difesa delle tre.

**Che il segnaposto fosse in conflitto con il `chain_id` a zero di `HASH-0`.** È la domanda di famiglia 1 sui due ruoli di uno stesso valore, e il documento la aveva già posta e risolta (`README.md:567-574`): `HASH-0` fissa zero **per dichiarazione**, il segnaposto zero **per regola**, i valori coincidono e i significati no, e leggere le righe del registro come la genesi di una catena renderebbe `WSC-0` inammissibile. Ho verificato che le due letture non si tocchino altrove nella tabella. **Non si è rotto** — e va detto che è un'anticipazione, non una difesa costruita dopo un finding.

**`state_root` come quinto canale dell'enumerazione.** È l'unico campo-hash dell'intestazione che l'elenco non nomina, e la mia ipotesi era che l'elenco fosse incompleto lì. È falsa: tutte le foglie dell'albero degli account sono separate per **tag byte** e nessuna porta `chain_id`, quindi nulla sotto `state_root` può essere in conflitto col segnaposto. L'esclusione è corretta. **Non si è rotto.**

**Che il verso «derivato» dell'enumerazione ammettesse un `mint` di genesi**, che sarebbe stata la rottura pulita della circolarità: `policy_hash` è nel corpo di un mint ed è sul lato derivato. Non è possibile — il pavimento `(e + 1) * reward_epoch_blocks <= h` non è soddisfacibile a `h = 0` — e per la stessa ragione cade `validator_candidacy`, che richiede una finestra di entropia di blocchi precedenti. **Il ramo forte dell'attacco non si è rotto**; ciò che resta è il ramo debole di RF-005, con `burn` e le due di sfida, che è `low` proprio per questo.

**Che `binds()` potesse divergere dai byte della preimmagine.** Se il contesto conservato e il prefisso scritto potessero non concordare, il tipo mentirebbe. Non possono: `signing_preimage` (`registry.rs:502-516`) compone i byte e costruisce il contesto **dagli stessi tre argomenti** nella stessa espressione, il campo `bytes` è privato e non `pub(crate)`, e nessun costruttore accetta contesto e byte separatamente. Il fallimento chiuso su `context: None` per `from_raw_bytes_non_consensus` è nel verso giusto e il test lo asserisce su tutta la matrice. **Non si è rotto.**

**La matrice 4×4, cercandovi la grandezza costante che [SKILL-001] passo 4 impone di cercare.** L'ho cercata perché è il difetto che ha prodotto il `high` di [REVIEW-027], e il file me la aspettava: la sua intestazione dichiara *«No quantity is held constant»* e incrocia due domini con due `chain_id` su tutte e sedici le celle, contando le accettazioni per escludere una guardia che rifiuti tutto. Ho verificato che la costante residua — lo stesso `payload` in tutte le celle — non sia sotto test e non possa mascherare nulla, perché dominio e catena stanno nel **prefisso**. Il verificatore stub che accetta tutto è la disposizione più forte e non la più debole, per la ragione scritta. **Non si è rotto**, ed è la gate meglio costruita di questa spec.

**Che la convergenza delle due strade fosse costruita invece che osservata**, cioè che il «terzo» fosse il documento dell'implementatore e quindi non un terzo. La procedura della remediation regge e la regge in un modo che non mi aspettavo: le righe pubblicate **a zero**, le due strade fatte fallire separatamente contro il documento sbagliato, e `6ba582b4…` nominato da entrambe le trascrizioni prima che il documento lo contenesse. Nessuna delle due poteva copiarlo dall'altra. **Non si è rotto.** Osservazione senza severità: quella prova è una **trascrizione** e non una guardia, quindi se un giorno gli ingressi di `GEN-1` cambiassero, l'accordo andrebbe riosservato con la stessa procedura a mano e nulla lo ricorderà. Vale la pena che il metodo stia scritto accanto alla fixture, non solo nella review.

**`GATE-VERIFIER-UNCHANGED`, dal lato dei file e non del conteggio.** `verifier.rs` contiene `verify_in_context` sopra `ConsensusVerifier` e la funzione di verifica sotto, invariata riga per riga: la matematica delle quattro regole ZIP-215, l'ordine dei controlli, `is_small_order` prima della decodifica dello scalare, `k` sulle codifiche originali. **Non si è rotto.**

**Che `GEN-1` fosse famiglia 1.** È la domanda 4 e le ho dedicato una sezione: l'argomento dell'implementatore regge e lo confermo. **Non si è rotto.**

## Required follow-up

**All'implementatore, dentro il giro di review:** RF-001, RF-004, RF-005, RF-006. Sono tutti e quattro chiudibili senza toccare un valore pubblicato.

**RF-002 e RF-003 sono una decisione, non una correzione, e la decisione è del Lead.** RF-003 (riscrivere il paragrafo con il perimetro vero) è dovuto in ogni caso. RF-002 dipende da quale delle tre vie il Lead sceglie, e due delle tre sono regole di validità nuove con la loro passata di [ADR-012]. **La mia raccomandazione è la via 1** — accettare il residuo e scriverlo per quello che è, incluso il fatto che dentro la finestra di genesi `binds()` è un controllo di dominio — con il quarto criterio di accettazione riformulato con la sua eccezione dichiarata. Il rifiuto del segnaposto derivato dalla rete resta a mio avviso corretto, ma va **rimotivato contro il perimetro vero** invece di appoggiarsi al paragrafo che RF-003 corregge.

**Su [DEBT-028]:** confermato nel merito e nella severità; `Resolution criteria` va aggiornato perché la condizione di chiusura è più stretta di come è scritta — la divergenza non richiede una seconda implementazione, basta un nodo che rigioca la catena dopo un cambio di `L`. È lavoro del Lead sul debito, non dell'implementatore su questa spec.

## Final decision

**Changes requested.** Tre `medium`, tre `low`, nessun `high`.

Il lavoro fa ciò che dichiara e in due punti lo fa meglio del richiesto. Il finding che porta il peso non sta in nessuna delle due metà: sta nel fatto che **comporle apre una finestra che nessuna delle due guarda**, perché ciascuna è stata verificata contro il proprio debito e la composizione contro nessuno. È la quarta volta su questo progetto che il difetto era già scritto e non guardato — qui letteralmente, perché il paragrafo che descrive il residuo e la coppia di fixture che lo smentisce distano dodici righe nello stesso file.

Va registrato in positivo, e non come cortesia: la sezione *Ciò che è stato guardato e trovato univoco* del censimento è la disposizione che ha reso questa review più veloce, perché mi ha detto dove l'implementatore **non** aveva guardato senza che dovessi dedurlo; e la procedura delle righe a zero della remediation è un contributo di metodo che vale oltre questa spec, e che raccomando di promuovere a skill.
