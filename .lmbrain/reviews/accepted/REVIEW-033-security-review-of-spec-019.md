---
id: REVIEW-033
# Note: Quote the title if it contains a colon
title: "Security review of SPEC-019 (GATE-SECREVIEW): la contraddizione tiene, e il costo della lettura scelta e' dichiarato per difetto"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-019
reviewer: AGENT-007
review_requested_by: AGENT-LEAD
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-033-EVENT-001"
    timestamp: "2026-08-26T13:41:53.692658700+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Security review di AGENT-007 su SPEC-019, GATE-SECREVIEW. Un high, cinque medium, tre low. Verdetto changes-requested con quattro voci dentro il perimetro e cinque debiti nuovi.\n\nLa reviewer RIBALTA IL PROPRIO ESITO. L'esito (A) della sua valutazione di DEBT-022 raccomandava la lettura \"finalizzata\"; conferma che AGENT-002 ha ragione e nomina l'errore per quello che e': ha motivato sul margine dove la proprieta' era disponibile. Ha attaccato la contraddizione piu' duramente di quanto avesse fatto il Lead, provando a rimettere la finalita' in catena attraverso il QuorumCertificate che il revoke_identity porta con se' - mossa che nessuno aveva provato - e non funziona, perche' quel certificato attesta che un quorum ha autorizzato la revoca, non a quale altezza un blocco sia diventato finale. Ha inoltre costruito il caso che avrebbe rotto la monotonia, una seconda revoca sullo stesso nodo con effective_height piu' basso, e non rompe perche' la definizione si ancora agli antenati del blocco e non alla testa.\n\nRF-001 high, riverificato dal Lead: non esiste alcun tetto su effective_height. Solo due MUST lo nominano - la 963 impone il minimo sopra il blocco proponente, la 994 e' la regola del light client - e nessuna riga limita quanto in alto possa stare. L'unico riferimento verso l'alto, alla riga 1037, e' una condizione sulla validita' di una contrazione del set: una revoca con effective_height assurdo non puo' giustificare una contrazione ma resta una revoca valida. Prima di questa spec quel campo governava la transizione del set di validatori; ora governa anche se una chiave revocata possa svuotare un saldo, e chi lo sceglie e' il quorum che revoca. Una revoca con effective_height a 2^60 soddisfa ogni MUST ed e' cosmetica. E' DEBT-022 spostato di un livello, dalla riga al campo su cui la riga ora poggia.\n\nRF-004 medium e' il finding che il Lead avrebbe dovuto trovare: la frontiera della clausola 1 non e' pinnata da nulla, e la mutazione valid_from_height minore-stretto lascia 176 test verdi, mentre la frontiera della clausola 2 e' pinnata due volte. La remediation deve dimostrarla in negativo.\n\nSulla regola di unicita' che la sua valutazione lasciava aperta: conferma che e' questione separata, ma l'ha cercata e non esiste, e quattro oggetti sorelle portano un \"at most one per\" che il burn di abbonamento non porta. Seconda cosa che quella clausola risulta non chiedere."
    evidence_refs: ["SPEC-019", "DEBT-022", "REVIEW-032"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-033-EVENT-002"
    timestamp: "2026-08-26T13:56:44.018494600+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Remediation consegnata da AGENT-002: quattro voci chiuse dentro il perimetro, RF-001 riportato invece che chiuso con la valutazione di quale grandezza vincolare. Da verificare dal Lead."
    evidence_refs: ["SPEC-019"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-033-EVENT-003"
    timestamp: "2026-08-26T13:57:07.765613900+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificata dal Lead rieseguendo: 177 test da 176, published_artifacts.py PASS con 146 candidati C10 da 142, prova in negativo PASS con ogni probe osservata fallire da sola, protocol_hashes.py PASS senza valori mossi.\n\nRF-004 chiuso con la diagnosi giusta, che vale piu' della correzione: fra le due clausole della definizione quella sotto test era la seconda, e la prima era la grandezza costante. E' il passo 4 di SKILL-001 applicato dentro la definizione invece che dentro la fixture, cioe' al livello a cui nessuno l'aveva applicato. La mutazione che sopravviveva a 176 test ora fallisce nominando il caso. L'implementatrice ha inoltre rieseguito la mutazione A sulla fixture allargata, perche' aggiungere righe e' il modo di indebolire una fixture senza accorgersene: continuano a fallire 21 e 49 e solo quelle.\n\nRF-003 chiuso con la forma dell'errore nominata peggiore di come era stata scritta: la probe era stata allargata credendo di rafforzarla, e la motivazione affermava una garanzia che dentro la finestra e' falsa, nel campo che lega una promessa pubblica a una riga di protocollo.\n\nRF-005 chiuso, e ha prodotto un residuo nuovo: la portata era dichiarata come stringa letterale mentre le formulazioni reali sono tre e nessuna e' quella. Ora e' dichiarata come perimetro. Riportato R3, ledger.md:928, dove l'ancoraggio non arriva per costruzione perche' un ValidatorSet non e' una transazione, e sei celle della tabella vi poggiavano.\n\nRF-001 riportato invece che chiuso, con la valutazione richiesta di quale grandezza vincolare. Il tetto ovvio - max_revocation_effective_delay_blocks - e' famiglia 3 un'altra volta e per giunta inefficace, perche' un quorum ostile sceglierebbe il massimo. Le due alternative che non sono famiglia 3 tolgono la discrezione invece di limitarla e sono entrambe meccanica della revoca, esclusa dal perimetro.\n\nLa seconda alternativa e' la piu' promettente e porta il fatto che rende il debito azionabile, riverificato dal Lead: il campo reason esiste gia', e' impegnato nell'ID della transazione, e compare in tutto il protocollo due volte - la dichiarazione dello schema e la fixture canonica. Nessuna regola lo legge, nessun codice lo legge. Il campo che porterebbe la distinzione fra key_compromise e operator_request e' gia' in catena e inerte.\n\nIl testo dichiara ora cio' che taceva: che l'intervallo non ha limite superiore, che una revoca con effective_height assurdo soddisfa ogni MUST ed e' cosmetica, e che finche' il campo non e' recintato quanto una revoca protegga un saldo lo sceglie il quorum che revoca. Ed e' stata ritirata una frase dell'evidenza precedente: il baratto non e' finestra dichiarata contro fork, e' finestra dichiarata contro ridefinire effective_height."
    evidence_refs: ["SPEC-019", "REVIEW-033"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-033-EVENT-004"
    timestamp: "2026-08-26T13:57:23.894769700+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "GATE-SECREVIEW soddisfatta. Nove finding: quattro chiusi dentro il perimetro, cinque riportati come debiti perche' richiedono di toccare la meccanica della revoca, che la spec esclude.\n\nQuesta review vale oltre la spec per due ragioni.\n\nLa prima e' che la reviewer ha ribaltato il proprio esito raccomandato e ha nominato il proprio errore: ha motivato sul margine dove la proprieta' era disponibile. Ha confermato la contraddizione attaccandola piu' duramente di quanto avesse fatto il Lead, provando a rimettere la finalita' in catena attraverso il QuorumCertificate che il revoke_identity porta con se' - mossa che nessuno aveva provato - e costruendo il caso che avrebbe rotto la monotonia.\n\nLa seconda e' RF-006, che l'implementatrice indica come il finding che le ha insegnato di piu' e il Lead concorda. La sua classificazione di R2 era corretta per il perimetro guardato: ha dimostrato che identity.md:614 non e' una regola di validita' su un blocco, e ha concluso che quindi non produce due verdetti. La seconda cosa non segue dalla prima, e la strada per cui non segue - un verdetto locale riciclato in catena attraverso una firma di quorum - e' una forma che il progetto non aveva mai visto.\n\nE' l'errore di forma di AGENT-007 rovesciato: una ha motivato sul margine dove la proprieta' era disponibile, l'altra ha dimostrato la proprieta' su un perimetro piu' stretto di quello vero e ha letto la dimostrazione come conclusiva. Vale la pena tenerle vicine perche' sono la stessa lezione da due lati: una dimostrazione e' conclusiva solo quanto il perimetro su cui e' fatta."
    evidence_refs: ["SPEC-019", "DEBT-022"]
    implementation_agent: "AGENT-002"
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [review, security, identity, ledger]
activity:
  - date: 2026-08-26
    action: "created pending"
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

**La contraddizione tiene. Ribalto il mio stesso esito (A) nella forma in cui l'avevo scritto**, e lo faccio sull'argomento di AGENT-002 e non per cortesia: la lettura «finalizzata», *come parola scritta in `identity.md`*, non e' una regola stretta con un margine largo, e' una regola il cui verdetto dipende da quali certificati un verificatore possiede. Su una transazione dentro un blocco quello e' un fork, ed e' esattamente l'esito che mi aveva fatto confermare `high` su [DEBT-022]. Raccomandare (A) con quella lettura significava chiudere una finestra reintroducendo il difetto peggiore che quella stessa valutazione aveva accertato. L'errore e' mio ed e' di forma nota: **ho motivato sul margine dove la proprieta' era disponibile**.

**Ma la contraddizione tiene sull'argomento, non sul costo.** Il mio esito (A) proteggeva una grandezza — la durata dell'esposizione — che questa consegna dichiara di lasciare aperta. La dichiarazione c'e' ed e' onesta. **E' incompleta**, e la parte mancante e' quella che conta: il documento dichiara il *pavimento* della finestra e tace il fatto che **non esiste alcun tetto**. Sotto la lettura scelta, tutta la forza di una revoca sulla spesa e' ora funzione di un campo `u64` illimitato scelto dal quorum che revoca. Nessuna riga del protocollo lo limita. E' il finding che questa review porta, ed e' il costo che la mia valutazione vedeva e che questa consegna non nomina.

**Verdetto raccomandato: `changes-requested`**, con una remediation piccola e circoscritta — quattro dei nove finding sono dentro il perimetro di questa spec e si chiudono con testo e una riga di fixture — e **tre debiti nuovi** per cio' che sta fuori. `GATE-SECREVIEW` e' attestabile dopo la remediation.

## Acceptance-criteria compliance

Riverificato eseguendo, non leggendo. `python sim/tools/published_artifacts.py` → `PASS`, C10-PROBE **142**, C3-FIXTURE-ID **20**: coincide con la trascrizione. `cargo test -p coblox-core --test authorization_unrevoked` → 9 verdi su una copia dell'albero in `.../scratchpad/rev033`, e la copia e' byte-identica a `core/coblox-core/src/authorization.rs` dell'albero. `cargo test --workspace` sulla copia → 176 passati.

`GATE-DEFINITION-FIRST`, `GATE-NO-PARAMETER-MOVED`, `GATE-ADR012`: soddisfatte, e la trascrizione mostra l'ordine invece di raccontarlo. `GATE-ALL-AUTHORIZATION-RULES` e `GATE-DIVERGENT-CASE` sono soddisfatte **ma incomplete** nei modi descritti in RF-004 e RF-005.

## Domanda 1 — La contraddizione tiene? Conferma o ribaltamento

**Confermo il ribaltamento, e nomino i due punti in cui l'argomento e' piu' forte di come e' scritto e il punto in cui e' piu' debole.**

**Dove e' forte, primo.** Ho provato a rompere il fondamento dove il Lead lo ha gia' attaccato, e non si e' rotto: nessun campo di `BlockHeader` registra la finalita' di un antenato. Ma l'argomento regge anche a un attacco che nessuno ha ancora provato, e che era il piu' promettente: **il `revoke_identity` porta lui stesso un `QuorumCertificate`** (`RevokeIdentityAuthorization`, `ledger.md:710`). Si potrebbe pensare che la finalita' sia quindi in catena, dentro la transazione. Non lo e': quel certificato attesta che **un quorum ha autorizzato la revoca**, non che **un blocco sia diventato finale**. La grandezza che la lettura «finalizzata» esige — *a quale altezza la revoca e' diventata finale* — continua a non esistere in catena. L'attacco piu' credibile all'argomento di AGENT-002 fallisce, e vale la pena scriverlo perche' e' il primo che chiunque rilegga queste righe provera'.

**Dove e' forte, secondo.** La scelta e' coerente con `identity.md:638` (*«revocation is not retroactive: historical signatures remain valid at heights before the effective height»*) invece di contraddirla. La mia (A) avrebbe creato due significati di *revocata* alla stessa altezza — uno per spendere, uno per validare — e il documento nuovo lo dice.

**Dove e' debole, ed e' RF-008.** L'evidenza della spec afferma che la lettura scelta e' *«l'unica che rende il predicato una funzione totale del blocco e dei suoi antenati»*. **Non e' l'unica.** Esiste una terza lettura — *nessun `revoke_identity` che nomina `node_id` e' incluso in un blocco ad altezza `<= h`* — che ha **tutte** le proprieta' che l'argomento invoca: e' un fatto del blocco e dei suoi antenati, e' monotona in `h`, ogni verificatore la legge dagli stessi byte, e non richiede alcun certificato fuori catena. AGENT-002 stessa scrive che un verificatore che rigioca *puo'* stabilire l'inclusione. **E quella lettura chiude la finestra**, cioe' fa esattamente cio' che la mia (A) voleva.

Non la raccomando, e la ragione va scritta: contraddice frontalmente `identity.md:638`, perche' invaliderebbe firme ad altezze **sotto** l'`effective_height`. Adottarla e' cambiare cosa `effective_height` significa, cioe' toccare la meccanica della revoca, che questa spec esclude a ragione. **Ma il documento non deve far credere che la scelta fosse forzata.** Il testo pubblicato e' corretto — non rivendica unicita' — mentre l'evidenza della spec la rivendica. Il vero baratto non e' *«finestra dichiarata contro fork»*: e' *«finestra dichiarata contro ridefinire `effective_height`»*, e il secondo termine e' un debito, non un'impossibilita'.

## Domanda 2 — La finestra: quanto vale davvero

**Vale, per un `key_compromise`, esattamente zero protezione del saldo, e la ragione non e' la lunghezza.** Tre fatti che ho verificato separatamente e che si compongono:

1. **La finestra e' garantita non nulla da un MUST.** `ledger.md:963`: `effective_height` MUST stare almeno `min_revocation_effective_delay_blocks` sopra il blocco che propone la revoca. Non e' un'impostazione, e' un pavimento normativo. Anche una governance istantanea non puo' portarlo a zero.
2. **Una sola transazione azzera il saldo** — punto 4 accertato in [DEBT-022] e non contestato da questa consegna. Prezzo e periodo sono scelti da chi attacca.
3. **Non esiste alcun tetto su `effective_height`.** `ledger.md:711` dice soltanto *«The effective height MUST be later than the block proposing the revocation»*; la 963 aggiunge il pavimento. Ho cercato un limite superiore in `ledger.md`, `identity.md` e `README.md` e **non c'e'**.

Da 1 e 2: chi detiene la chiave e guarda la catena drena il saldo **con certezza**, qualunque sia il valore del parametro. La revoca, per l'asset `A-01` e per il motivo `key_compromise`, non protegge nulla. Da 3: la protezione residua della revoca sulla spesa e' interamente delegata a un campo illimitato scelto dal quorum revocante — un `effective_height` assurdamente alto soddisfa ogni MUST del documento, viene registrato in catena, compare in `revoked_validators`, e non morde mai. Sotto la lettura precedente quel campo non aveva questo potere sull'autorizzazione della spesa; **la lettura scelta glielo ha dato, e nessuna riga lo recinta.**

C'e' un quarto fatto, ed e' quello che rende il pavimento non solo garantito ma **immotivato sul percorso di spesa**. La ragione per cui `min_revocation_effective_delay_blocks` e' lungo e' dichiarata e ha un solo oggetto: `ledger.md:963` lo giustifica *«so the surviving members have a bounded, declared window in which to commit a compliant successor set»*, e `ledger.md:1020` *«exists to make the window long enough that the choice [lo stallo] is rarely exercised»*. **E' una ragione di continuita' del set di validatori.** Per un nodo che non ha alcun seggio e la cui chiave e' stata rubata, quella ragione non esiste, e nessuna regola permette un `effective_height` piu' vicino per `reason = "key_compromise"`. La definizione nuova importa un parametro di liveness del consenso dentro una regola di spesa dove non ha giustificazione.

**Esposizione reale, quindi: totale sul saldo, non limitata superiormente nel tempo, e con un pavimento tarato da una grandezza che riguarda un'altra cosa.** Non e' accettabile come stato finale; **e' accettabile come stato dichiarato di questa spec**, perche' chiuderla richiede di toccare la meccanica della revoca. **Apre due debiti** (RF-001, RF-002).

## Domanda 3 — `AUTH-0` esercita il caso giusto?

**Sul caso divergente si', e la dimostrazione e' quella giusta**: le righe concordi restano verdi sotto la lettura sbagliata, e sono in tabella con scritto accanto che non provano nulla. Questo l'ho verificato invece di crederlo, rieseguendo la mutazione A sulla copia.

**Ma ho cercato la settima riga e l'ho trovata, e non e' dove il dispatch la immaginava.** Non e' una settima riga *divergente*: e' la frontiera della **clausola 1**, che la fixture non pubblica e la suite non pinna.

La definizione ha due clausole. La clausola 2 ha la sua frontiera pinnata due volte — riga `50` in tabella, `the_revocation_bites_exactly_at_its_effective_height` nel test — e il commento nel codice la spiega (*«la comparazione e' `<=`»*). La clausola 1 dice `valid_from_height <= h` e **la sua frontiera non e' esercitata da nulla**: i test toccano `h = 4` (sotto) e `h >= 19` (sopra), mai `h = 5`. `AUTH-0` non pubblica nemmeno una colonna `valid_from_height`.

Riprodotto su copia (`.../scratchpad/rev033`, fuori dal repository), **mutazione C**:

```
- .any(|record| record.node_id == node_id && record.valid_from_height <= including_height)
+ .any(|record| record.node_id == node_id && record.valid_from_height <  including_height)
```

```
$ cargo test -p coblox-core --test authorization_unrevoked
test result: ok. 9 passed; 0 failed
$ cargo test --workspace
176 passati, 0 falliti
```

**La mutazione sopravvive all'intera suite del workspace.** Ripristinata, verde riverificato, copia byte-identica all'albero. E' RF-004, ed e' precisamente la *«fixture di frontiera all'altezza esatta»* che l'esito (A) chiedeva: c'e' per una clausola e manca per l'altra.

**Le altre mutazioni che ho provato sono state catturate** — vedi la sezione finale.

Sulla precisione della frase: *«le righe 21 e 49 sono le uniche divergenti»* e' vera **della tabella** e falsa **delle altezze**. L'intervallo divergente e' `[20, 49]`: l'altezza `20` — quella di finalizzazione — e ogni altezza fino a `49` divergono. La tabella campiona il primo interno e l'ultimo. E' RF-007, `low`, e va corretto perche' e' il tipo di frase che il prossimo lettore prende per un'enumerazione.

## Domanda 4 — L'elenco dei tredici domini e' completo? Il perimetro e' quello giusto?

**L'elenco e' derivato bene e il metodo e' migliore del mio** — il registro dei domini di firma ha un lato disco e fallisce nei due versi, mentre la mia enumerazione era una lettura. Su questo la consegna mi corregge e ha ragione.

**Il perimetro pero' ha una crepa, ed e' dentro l'elenco stesso.** La mia valutazione di [DEBT-022] stabili' che il difetto era nell'asimmetria e non nella riga; l'asimmetria vera di questa spec e' *«la qualificazione senza l'altezza»*, ed e' quella che R1 nomina su `app-manifest.md`. **La stessa forma e' dentro `ledger.md`, in una riga che la tabella cita come copertura, e non e' riportata.**

`ledger.md:890`: *«Validators are sorted by ID, unique, enrolled and unrevoked»*. Porta la qualificazione, **non porta l'altezza**, ed e' una condizione di validita' su un oggetto `ValidatorSet` — che non e' una transazione, quindi la definizione nuova, che si ancora *«all'altezza del blocco che include la transazione»*, **per costruzione non la raggiunge**. Sei delle tredici celle della tabella (1e, 1f, 1g, 1h, 2, 4) sono segnate coperte *«via `ledger.md:890`»*. Sono coperte da una regola che ha lo stesso difetto di R1.

Aggravante di forma, ed e' il tipo di difetto che questo progetto censisce: la definizione dichiara di governare *«every occurrence of the words `enrolled, unrevoked`»*. Verificato con `grep`: `ledger.md:890` dice **`enrolled and unrevoked`**, `ledger.md:1204` dice **`enrolled and not revoked as of`**, `app-manifest.md:65` dice **`finalized, unrevoked`**. **Nessuna delle tre e' la stringa che la definizione nomina.** `app-manifest.md` e' salvata perche' la definizione la cita per nome; le altre due no. La portata autodichiarata e' una corrispondenza letterale che non copre le formulazioni su cui l'argomento di copertura poggia. E' RF-005.

**Il perimetro ha una seconda crepa, piu' larga e fuori scopo, e la segnalo senza pretendere che questa spec la chiuda.** L'enumerazione per domini di firma censisce **chi puo' firmare**. Non censisce **chi conta**. `ledger.md:391` seleziona i burn di abbonamento per il `publisher_reward` raggruppandoli per `payer_node_id`, e non chiede da nessuna parte che quel pagatore sia iscritto e non revocato: un'identita' revocata continua a gonfiare `active_subscriber_count` e `counted_subscription_burn_microtokens`, che sono grandezze di **emissione** (`A-02`). Nella mia valutazione l'ho accertato (punto 5) e l'ho dichiarato **conseguenza legittima** della non retroattivita', e lo confermo. Ma la conseguenza **non e' scritta in `ledger.md`**: un lettore della 391 non ha modo di saperlo, e la sezione nuova non la nomina. Una conseguenza accettata e non scritta si rilegge come una dimenticanza — che e' la regola di forma con cui questo debito e' nato. Va scritta, e appartiene alla stessa famiglia dei residui R1/R2.

## Domanda 5 — R1 e R2 sono classificati bene?

**R1 e' classificato bene** e correttamente non corretto: `app-manifest.md` non e' fra i file di questa spec, la definizione lo nomina e gli da' l'altezza, e correggerlo qui scavalcherebbe la sua gate. L'unica correzione e' che **R1 ha un gemello dentro `ledger.md`** (RF-005), dove il vincolo di perimetro non si applica.

**R2 e' il finding, e non e' chiuso.** La classificazione — *«regola di accettazione locale del ricevente, nessuno la rigioca, due riceventi non devono concordare»* — e' **vera della validita' di un blocco** e la consegna dimostra solo quello. Non e' vera degli **esiti**. Ho attaccato la convivenza e produce ancora due verdetti, per due strade indipendenti.

**Prima strada: la sfida.** `identity.md:614` obbliga il ricevente a rifiutare un peer quando una revoca esiste **alla propria altezza finalizzata**, e `identity.md:625-631` a chiudere le connessioni gia' aperte quando il proprio insieme di revoche finalizzate cambia. Le sfide viaggiano su uno stream protetto (`wire.md:56`, `/coblox/challenge/0.1.0`), e uno stream protetto esige proprio quel controllo. **Dentro la finestra**, due auditor con altezze finalizzate diverse ottengono esiti diversi sulla stessa sfida allo stesso soggetto: chi ha gia' visto la revoca deve rifiutare la connessione e registra `no_response` (`ledger.md:617`); chi e' indietro riceve la risposta e registra `passed`. Quell'esito entra in un `challenge_evidence` **firmato a quorum**, e da li' governa `contribution_score`, l'eleggibilita' (`ledger.md:1184`) e i mint `work_compensation`. **Il verdetto non diverge sulla validita' del blocco — diverge sul contenuto di un oggetto che finisce in catena.** La classificazione di R2 e' corretta sull'unica cosa che dimostra e ferma un passo prima di dove la convivenza morde.

**Seconda strada, e riguarda proprio la ragione per cui il parametro e' lungo.** Dentro la finestra un nodo revocato-non-ancora-efficace e' **simultaneamente**: autorizzato a spendere e a candidarsi (definizione nuova, clausola 2), contato con pieno peso di voto nel set attivo (le regole 1-3 di `ledger.md:956-958` mordono solo **da** `effective_height`), e **obbligatoriamente irraggiungibile** da ogni peer conforme (`identity.md:614` e `625-631`). Se il seggio revocato porta piu' di un terzo del potere, la catena si ferma **durante** la finestra — cioe' esattamente lo stallo che `min_revocation_effective_delay_blocks` e' lungo per rendere raro. Il parametro e' tarato contro uno stallo che un'altra regola dello stesso protocollo puo' produrre prima. E' precedente a questa spec e non e' colpa di questa consegna; **e' rilevante qui perche' la sezione nuova canonizza la convivenza in un paragrafo che un lettore prendera' per un via libera completo.** RF-006, e un debito.

## Domanda 6 — La regola di unicita' su `(payer_node_id, app_id, service_period)`

**Confermo che e' una questione separata da `unrevoked`, e confermo che non cercarla qui era la scelta giusta.** L'ho pero' cercata io, perche' la mia valutazione l'aveva lasciata aperta e lasciarla aperta due volte sarebbe stato un modo di non deciderla.

**Non c'e'.** `ledger.md:24-25` (invariante 6) impone che un ID di transazione occorra al piu' una volta e che i nonce di addebito siano strettamente consecutivi: questo ferma il **replay dello stesso oggetto**, non due burn distinti, con nonce diversi, sullo stesso `(payer_node_id, app_id, service_period)`. La sezione burn non lo vieta, e la sezione di esecuzione nemmeno.

**Perche' e' separata, e non e' la stessa da un'altra porta.** Tre ragioni indipendenti. Non ha due letture, quindi non produce due verdetti conformi: e' un'assenza, non un'ambiguita'. Non aumenta il drenaggio, perche' una sola transazione lo esaurisce gia'. E non aumenta il ricavo dell'attaccante, perche' `ledger.md:391-393` trattiene **un solo burn per pagatore per epoca**, il `subscription_burn_tx_id` piu' basso: i duplicati sono distruzione pura di valore. Su nessuno dei tre piani tocca `unrevoked`.

**Va comunque aperta come debito, per una ragione di forma che appartiene proprio a questo debito.** Tre oggetti sorelle portano il vincolo — `ledger.md:340` (*«at most one `existence_income` mint per…»*), `ledger.md:410` (*«at most one publisher-reward mint per `(app_id, reward_epoch)`»*), `ledger.md:593` (*«at most one commitment per `(issuer_node_id, commitment_epoch)`»*), `ledger.md:1152` (*«at most one candidacy per `(node_id, election_epoch)`»*) — e il burn di abbonamento no. **E' la stessa clausola, di nuovo l'unica di una famiglia priva del vincolo che le sorelle hanno.** Non e' lo stesso difetto; e' la stessa mano. Corrobora la conclusione di svista da taglia-e-incolla che sia la mia valutazione sia questa consegna hanno raggiunto, e questa e' la seconda volta che quella riga viene trovata mancante di qualcosa. RF-009, `low`, e un debito.

## Review findings

### RF-001 — `high` — Il costo dichiarato e' dichiarato per difetto: `effective_height` non ha un limite superiore, e la lettura scelta gli ha appena dato potere sull'autorizzazione della spesa

Il paragrafo *«The cost of this reading, declared»* (`ledger.md:147-156`) dichiara il **pavimento** della finestra — *«That interval is at least `min_revocation_effective_delay_blocks` blocks»* — e tace il fatto che **non esiste alcun tetto**. `ledger.md:711` chiede solo che `effective_height` sia *«later than the block proposing the revocation»*; `ledger.md:963` aggiunge il pavimento. Nessuna riga limita quanto in alto possa stare.

Prima di questa spec, quel campo governava la transizione del set di validatori (`ledger.md:956-958`). Dopo questa spec governa anche **se una chiave revocata puo' svuotare un saldo**. Una `revoke_identity` con `effective_height` a `2^60` soddisfa ogni MUST, e' registrata in catena, compare in `revoked_validators` del checkpoint, e non morde mai. La revoca diventa cosmetica senza che nulla la segnali, e chi sceglie il valore e' il quorum che revoca.

Il difetto non e' nella definizione, che e' la scelta giusta: **e' nel fatto che la definizione ha reso portante un campo non recintato, e il paragrafo che dichiara il costo non lo dice.** La frase finale del paragrafo — *«How short the window should be, and who sets `effective_height`, are questions about revocation mechanics and not about what the qualification means»* — parcheggia proprio la domanda che la scelta appena resa carica di peso.

**Come riprodurlo.**

```
$ grep -n "effective_height" docs/protocol/*.md | grep -iv "at least\|later than"
```
Nessuna occorrenza impone un limite superiore. Verificato anche su `README.md` (`revoked_validators`, `revocation_root`) e su `core/coblox-core/src/light_client.rs:281-292`, che confronta `header.height >= effective_height` senza alcun vincolo sul valore.

**Rimedio nel perimetro di questa spec (testo, non regola):** il paragrafo del costo deve dire che l'intervallo e' *at least* `min_revocation_effective_delay_blocks` **e non ha limite superiore**, e che finche' non ne ha uno la forza di una revoca sulla spesa e' scelta dal quorum revocante. **Fuori perimetro, e va a debito:** un tetto su `effective_height`, plausibilmente legato a `reason`.

### RF-002 — `medium` — Il pavimento della finestra e' tarato da una ragione di liveness del consenso che sul percorso di spesa non esiste

`min_revocation_effective_delay_blocks` e' lungo per un motivo dichiarato e unico: dare ai membri superstiti una finestra per impegnare un set successore conforme (`ledger.md:963`) e rendere raro lo stallo (`ledger.md:1020`). E' una ragione **di continuita' del set di validatori**.

La definizione nuova la applica a `SubscriptionBurnAuthorization` e `FundAppAuthorization`, cioe' a nodi che nella stragrande maggioranza dei casi non hanno alcun seggio. Per un `reason = "key_compromise"` su un nodo non validatore, quel pavimento e' un ritardo obbligatorio senza alcuna giustificazione locale, e non esiste alcuna regola che permetta un `effective_height` piu' vicino in funzione del `reason`.

Composto con il fatto — accertato in [DEBT-022] punto 4 e non contestato — che **una sola transazione azzera il saldo**, ne segue una conseguenza che va scritta e non solo saputa: **per il `key_compromise` la revoca non protegge il saldo in alcun grado**, qualunque taratura si scelga. Non e' un argomento per accorciare il parametro (sarebbe [ADR-010] e la famiglia 3, e la spec ha ragione a vietarlo): e' un argomento per **separare** il ritardo dovuto alla transizione del set dal ritardo imposto alla spesa, che sono due esigenze diverse su un solo campo.

**Come riprodurlo.** `ledger.md:963` e `ledger.md:1005-1020` per la ragione; `ledger.md:708` per l'enumerazione di `reason`, che non e' letta da alcuna regola sul ritardo; `ledger.md:542-556` per l'assenza di un tetto sull'importo. Va a debito, non a remediation.

### RF-003 — `medium` — La probe `guide-subscription-burn-needs-your-signature` e' stata allargata con una motivazione che la definizione non consegna

`sim/tools/published_artifacts.toml`, campo `why`: *«Since [SPEC-019] it also pins the qualification: a signature from a key the network has revoked is not a signature you gave, and the sentence would be false without it.»* La probe e' agganciata a una **claim della guida pubblica** (`claims = "Nobody can take credits away from you…"`), quindi la frase non e' un commento interno: e' la ragione per cui un'affermazione al pubblico e' considerata sostenuta.

Sotto la definizione appena adottata, **dentro la finestra la firma di una chiave che la rete ha revocato e' accettata**, e il saldo viene addebitato. La motivazione della probe afferma percio' una proprieta' piu' forte di quella che il protocollo garantisce, e lo fa nel punto esatto in cui il progetto tiene il legame fra documento e promessa. E' la forma dell'*«impossibilita' dichiarata a torto»* rovesciata: una garanzia dichiarata a torto.

**Come riprodurlo.** `grep -n "guide-subscription-burn-needs-your-signature" -A6 sim/tools/published_artifacts.toml`, e confrontare con `ledger.md:147-150`.

**Rimedio, dentro il perimetro:** riscrivere il `why` in modo che dica cio' che la qualificazione consegna davvero — la revoca ferma la spesa **da `effective_height`** — e che nomini la finestra. La probe come pattern va bene e va tenuta allargata; e' la motivazione a essere falsa.

### RF-004 — `medium` — La frontiera della clausola 1 non e' pinnata da nulla: mutazione riprodotta, 176 test verdi

La definizione ha due clausole. Quella su `effective_height` ha la sua frontiera esercitata (riga `50` di `AUTH-0`, `the_revocation_bites_exactly_at_its_effective_height`). Quella su `valid_from_height` **no**: i test toccano `h = 4` e `h >= 19`, mai `h = 5`, e `AUTH-0` non pubblica affatto la colonna dell'iscrizione.

**Come riprodurlo.** Copia dell'albero in `.../scratchpad/rev033`, fuori dal repository. Verde di partenza: 9 su 9.

```
- .any(|record| record.node_id == node_id && record.valid_from_height <= including_height)
+ .any(|record| record.node_id == node_id && record.valid_from_height <  including_height)
```

```
$ cargo test -p coblox-core --test authorization_unrevoked
test result: ok. 9 passed; 0 failed
$ cargo test --workspace
176 passati, 0 falliti
```

Ripristinata, verde riverificato, `diff` con l'albero vuoto.

**Perche' conta piu' di un test mancante.** L'esito (A) di [DEBT-022] chiedeva *«una fixture di frontiera all'altezza esatta e la prova in negativo»*. La consegna l'ha data per una clausola e non per l'altra, e il documento espone la seconda clausola con la stessa autorevolezza della prima. Un'implementazione che legge `valid_from_height` come *strettamente sotto* e' conforme a ogni fixture pubblicata e diverge dalla definizione a una altezza — che e' la stessa forma di difetto che questa spec esiste per chiudere, un'altezza piu' in la'.

**Rimedio, dentro il perimetro:** una riga in `AUTH-0` a `h = valid_from_height` con verdetto `valid`, e il test corrispondente.

### RF-005 — `medium` — R1 ha un gemello dentro `ledger.md`, in una riga su cui sei delle tredici celle poggiano, e non e' riportato

`ledger.md:890`: *«Validators are sorted by ID, unique, enrolled and unrevoked»*. La qualificazione c'e', **l'altezza no**. E' una condizione di validita' su un oggetto `ValidatorSet`, che non e' una transazione: la definizione nuova si ancora *«all'altezza del blocco che include la transazione»* e quindi **non la raggiunge**. Le celle `1e`, `1f`, `1g`, `1h`, `2` e `4` della tabella dei tredici domini sono segnate coperte *«via `ledger.md:890`»* — sei superfici su tredici, dichiarate coperte da una regola che ha esattamente il difetto di R1, in un file che questa spec era autorizzata a toccare.

Aggravante di forma. La definizione dichiara di governare *«every occurrence of the words `enrolled, unrevoked`»*. Le stringhe reali sono tre e nessuna e' quella:

```
$ grep -n "enrolled, unrevoked\|enrolled and unrevoked\|enrolled and not revoked\|finalized, unrevoked" docs/protocol/*.md
app-manifest.md:65:  ... finalized, unrevoked enrollment certificate.
ledger.md:890:      ... enrolled and unrevoked; voting power is
ledger.md:1204:  1. it is enrolled and not revoked as of `candidacy_close_height(e)`;
```

`app-manifest.md` e' coperto perche' la definizione lo cita per nome; `ledger.md:1204` e' salvo perche' porta gia' la propria altezza; `ledger.md:890` non e' ne' l'uno ne' l'altro.

**Come riprodurlo.** Il `grep` sopra, piu' `ledger.md:102-113` per la portata dichiarata e la tabella di `GATE-ALL-AUTHORIZATION-RULES` nella spec per le sei celle.

**Rimedio, dentro il perimetro:** allargare la portata dichiarata alle formulazioni equivalenti invece che alla stringa letterale, e riportare `ledger.md:890` come residuo **R3** con la sua raccomandazione di debito, esattamente come R1. Se l'ancoraggio giusto per un `ValidatorSet` e' `activation_height`, va detto li' o va detto che e' un'altra spec; non va lasciato come cella verde.

### RF-006 — `medium` — R2 e' classificato sulla sola validita' del blocco, e la convivenza produce ancora due verdetti su un oggetto che finisce in catena

La classificazione di R2 dimostra che `identity.md:614` non e' una regola di validita' su un blocco. E' vero. **Non e' vero che nessuno ne subisca il verdetto in catena.**

Le sfide viaggiano su stream protetti (`wire.md:56`); uno stream protetto esige il controllo di `identity.md:600-616`, che include *«no revocation exists at the receiver's finalized height»*; `identity.md:625-631` obbliga a chiudere anche le connessioni gia' aperte quando l'insieme finalizzato cambia. Dentro la finestra, due auditor con altezze finalizzate diverse producono esiti diversi sulla stessa sfida allo stesso soggetto — `no_response` contro `passed` (`ledger.md:617`) — e quell'esito entra in un `challenge_evidence` firmato a quorum che governa `contribution_score`, l'eleggibilita' (`ledger.md:1184`) e i mint `work_compensation`. **La lettura locale del ricevente e' laundered in catena attraverso una firma di quorum.**

E c'e' lo stato contraddittorio: dentro la finestra il nodo revocato e' autorizzato a spendere e a candidarsi (definizione nuova), conta con pieno peso di voto (le regole 1-3 di `ledger.md:956-958` mordono solo **da** `effective_height`), ed e' obbligatoriamente irraggiungibile da ogni peer conforme. Se porta piu' di un terzo del potere, la catena si ferma **dentro** la finestra che `min_revocation_effective_delay_blocks` e' lungo per rendere sicura.

Nulla di tutto questo e' stato introdotto da questa consegna, e non chiedo che venga chiuso qui. **Chiedo che il paragrafo *«One rule this definition does not govern»* non lo faccia sembrare chiuso.** Cosi' com'e' scritto, dice che quella regola e' libera di essere piu' stretta *«precisely because»* nessuno la rigioca — e la ragione e' vera per la validita' e falsa per gli esiti.

**Come riprodurlo.** `identity.md:600-631`, `wire.md:56` e `wire.md:204-315`, `ledger.md:604-620`, `ledger.md:1184`, `ledger.md:956-963`.

**Rimedio, dentro il perimetro:** una frase nel paragrafo che dice che la regola locale governa la **raggiungibilita'** e non la validita', e che dentro la finestra le due divergono in modo osservabile sul percorso della sfida. **Fuori perimetro, a debito:** l'interazione fra isolamento di trasporto e potere di voto contato fino a `effective_height`.

### RF-007 — `low` — «Le righe 21 e 49 sono le uniche divergenti» e' vera della tabella e falsa delle altezze

L'intervallo su cui le due letture divergono e' `[20, 49]`: l'altezza di finalizzazione compresa. `ledger.md:207-209` e il commento di modulo di `core/coblox-core/tests/authorization_unrevoked.rs:10-12` dicono *«the only ones»* senza il qualificatore *«fra le righe di questa tabella»*. Non induce alcuna implementazione in errore, ma e' un'enumerazione apparente in un punto in cui l'enumerazione e' la sostanza, e il prossimo lettore la prendera' per tale. Correggere in *«le uniche righe di questa tabella»*, e nominare l'intervallo.

### RF-008 — `low` — L'evidenza rivendica un'unicita' che il documento giustamente non rivendica, e il vero baratto resta non scritto

L'evidenza della spec: *«La lettura scelta e' quindi l'unica che rende il predicato una funzione totale del blocco e dei suoi antenati»*. Il documento pubblicato non lo afferma, e fa bene. Una terza lettura — *nessun `revoke_identity` che nomina `node_id` e' incluso ad altezza `<= h`* — e' un fatto del blocco e dei suoi antenati, monotona in `h`, letta dagli stessi byte da ogni verificatore, **e chiude la finestra**.

Non e' adottabile senza contraddire `identity.md:638`, cioe' senza ridefinire cosa `effective_height` significa — che e' meccanica della revoca ed e' correttamente fuori scopo. **Ma allora il baratto non e' «finestra dichiarata contro fork»: e' «finestra dichiarata contro ridefinire `effective_height`»**, e il secondo termine e' un debito e non un'impossibilita'. Scriverlo nel paragrafo del costo costa due righe e impedisce che la prossima persona che guarda questa finestra concluda che non c'era niente da fare.

### RF-009 — `low` — Nessuna regola di unicita' su `(payer_node_id, app_id, service_period)`, e la clausola priva del vincolo e' di nuovo quella dell'abbonamento

Questione separata da `unrevoked` — tre ragioni indipendenti nella Domanda 6 — e correttamente non risolta qui. Ma cercata e accertata: **non esiste**. `ledger.md:24-25` copre il replay dello stesso oggetto e i nonce consecutivi, non due burn distinti sullo stesso periodo. Quattro oggetti sorelle portano un *«at most one per …»* (`ledger.md:340`, `410`, `593`, `1152`); il burn di abbonamento no. E' la seconda cosa che quella clausola risulta non chiedere. A debito.

**Nota, non finding.** `authorize_single_key` e `enrolled_unrevoked` non hanno chiamanti fuori dai test: `grep -rn "authorize_single_key\|enrolled_unrevoked" core/ --include=*.rs` da' solo `src/authorization.rs` e `tests/authorization_unrevoked.rs`. E' coerente con lo stadio del crate — non esiste alcun percorso di esecuzione delle transazioni, e `election::CandidateFacts` ha la stessa forma — e la spec chiedeva una regola con la prova in negativo, non un cablaggio. Lo scrivo perche' un lettore futuro potrebbe leggere la presenza del modulo come la presenza di una guardia sul percorso di spesa, e oggi non lo e'.

## Cio' che ho attaccato senza riuscire a romperlo

**Che la finalita' fosse in catena dentro il `revoke_identity` stesso.** E' l'attacco piu' forte al fondamento di AGENT-002 e nessuno l'aveva provato: `RevokeIdentityAuthorization` **porta** un `QuorumCertificate` (`ledger.md:710`), quindi verrebbe da dire che la finalita' e' un fatto del blocco. Non lo e': quel certificato attesta che un quorum ha autorizzato la revoca, non a quale altezza un blocco sia diventato finale. La grandezza che la lettura «finalizzata» richiede continua a non esistere. **Non si e' rotto**, e il fondamento della contraddizione regge a un attacco piu' duro di quello che il Lead gli ha portato.

**Che il predicato non fosse monotono rispetto alla crescita della catena.** Se una seconda `revoke_identity` sullo stesso nodo potesse portare un `effective_height` **piu' basso** della prima, un blocco gia' valido diventerebbe invalido a una testa successiva, e la proprieta' su cui poggia tutta la scelta sarebbe falsa. Costruito il caso: revoca A proposta a `10` con `effective_height` `1000`, revoca B proposta a `20` con `effective_height` `30`, e un blocco a `h = 40`. Il predicato regge perche' la definizione dice *«against the finalized state that block builds on»*: B, inclusa a `20`, e' antenata di ogni blocco a `40`, quindi ogni verificatore la vede. L'ancoraggio agli antenati — e non alla testa — e' precisamente cio' che chiude questo attacco. **Non si e' rotto**, ed e' la parte migliore del disegno.

**Che esistesse un caso intra-blocco.** Un burn di abbonamento nello stesso blocco della `revoke_identity` che lo colpisce sarebbe il caso in cui l'ordine dentro il blocco decide la validita', cioe' un fork sull'ordinamento. Non esiste: `ledger.md:711` impone `effective_height` **strettamente oltre** il blocco che propone la revoca, e la 963 le mette in mezzo almeno `min_revocation_effective_delay_blocks`. Il caso e' vuoto per costruzione. **Non si e' rotto.**

**Che la regola potesse essere aggirata dal lato del filtro sul `node_id`.** Provate su copia le mutazioni che tolgono il filtro `record.node_id == node_id` dalle revoche e dalle iscrizioni: la prima fa fallire la riga di confronto a `51`, la seconda fa fallire `an_identity_no_certificate_names_is_not_enrolled_rather_than_revoked`. La sesta riga aggiunta da AGENT-002 fa esattamente il lavoro per cui e' stata aggiunta. **Non si e' rotto.**

**Che la mutazione A fosse catturata per caso.** Riprodotta: falliscono `21` e `49` e **solo** quelle; le tre righe concordi restano verdi sotto la lettura sbagliata. La fixture misura la divergenza e non la regola, che e' cio' che `GATE-DIVERGENT-CASE` chiedeva. **Non si e' rotto.**

**Che l'enumerazione dei domini fosse una lista compilata a mano.** `python sim/tools/published_artifacts.py` rieseguito: `PASS`, 142 candidati C10 e 20 fixture, coincidenti con la trascrizione. Il registro dei domini ha un lato disco e fallisce nei due versi. Il **metodo** e' migliore del mio, e lo scrivo perche' la mia valutazione contava righe leggendo. **Non si e' rotto** — la crepa che ho trovato (RF-005) e' nella copertura dichiarata di una cella, non nell'enumerazione.

**Che i valori pubblicati fossero cambiati senza dirlo.** `python sim/tools/protocol_hashes.py`: `PASS`, `REVL-0` invariato. Coerente: `AUTH-0` e' comportamentale. **Non si e' rotto.**

## Required follow-up

**Remediation dentro il perimetro di [SPEC-019]** — quattro voci, tutte in `ledger.md`, `sim/tools/published_artifacts.toml` e il file di test:

1. **RF-004** — riga di frontiera della clausola 1 in `AUTH-0` (`h = valid_from_height`, verdetto `valid`) e il test corrispondente, con la mutazione osservata fallire.
2. **RF-003** — riscrivere il `why` della probe `guide-subscription-burn-needs-your-signature` su cio' che la qualificazione consegna: la revoca ferma la spesa **da `effective_height`**.
3. **RF-005** — portata della definizione estesa alle formulazioni equivalenti; `ledger.md:890` riportato come residuo **R3** con raccomandazione di debito.
4. **RF-001 (parte testuale)**, **RF-006 (parte testuale)**, **RF-007**, **RF-008** — quattro correzioni di frase nel paragrafo del costo, nel paragrafo *«One rule this definition does not govern»* e nella nota sotto `AUTH-0`.

**Debiti nuovi raccomandati al Lead**, tutti fuori dal perimetro di questa spec:

- **`effective_height` senza tetto** (RF-001) — `high`. Condizione di chiusura: prima che esista una devnet con saldi, perche' qui la grandezza da cui il pericolo dipende **e'** il valore in gioco, a differenza di [DEBT-022].
- **Il pavimento del ritardo e il `reason`** (RF-002) — `medium`. Separare il ritardo dovuto alla transizione del set da quello imposto alla spesa.
- **Isolamento di trasporto contro potere di voto dentro la finestra** (RF-006) — `medium`, e va nella stessa passata di [DEBT-018] sul threat model, perche' l'attore non ha ancora una cella.
- **Unicita' su `(payer_node_id, app_id, service_period)`** (RF-009) — `low`.
- **La conseguenza non scritta sul conteggio degli abbonati** (Domanda 4, seconda crepa) — `low`. `ledger.md:391` non dice che un pagatore revocato continua a contare, e la non retroattivita' che lo giustifica sta in un altro file.

## Final decision

**`changes-requested`.** Nessuno dei nove finding tocca la **scelta**, che confermo contro la mia stessa raccomandazione precedente: l'ancoraggio a `effective_height` e' l'unico dei due che il documento poteva scrivere senza specificare un fork, e il ribaltamento e' corretto. Quattro finding stanno dentro il perimetro e si chiudono con testo e una riga di fixture; cinque sono debiti.

Il finding che porta il peso e' **RF-001**, ed e' la ragione per cui questa review non e' un'accettazione: la consegna dichiara il costo della lettura scelta e lo dichiara **per difetto**, perche' racconta il pavimento e tace che il tetto non c'e'. E' la stessa forma del difetto che [DEBT-022] censiva — una qualificazione scritta senza la grandezza rispetto a cui vale — spostata di un livello: dalla riga al campo su cui la riga ora poggia. Correggere due frasi la chiude dal lato del documento; recintare il campo e' un'altra spec.

`GATE-SECREVIEW` e' attestabile dopo la remediation delle quattro voci, e non prima: RF-003 e RF-004 sono l'unico modo in cui questa consegna puo' ancora consegnare meno di quello che dice.
