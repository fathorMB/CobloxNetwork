---
id: REVIEW-035
# Note: Quote the title if it contains a colon
title: "Security review of SPEC-020"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-020
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, documentation, test-quality, maintainability]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-035-EVENT-001"
    timestamp: "2026-08-26T14:43:46.632854200+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Security review di AGENT-007 su SPEC-020, GATE-SECREVIEW. Un high, un medium, quattro low. Gate NON superata.\n\nRF-001 high, riverificato dal Lead meccanicamente e regge in modo netto: checkpoint_is_fresh (light_client.rs:68-70) fallisce con \"checkpoint issued in the future\" quando issued_at_ms supera now_ms; with_checkpoint_floor (cadence.rs:155) alza now_ms solo quando checkpoint_issued_at_ms supera local_clock_ms. Le due condizioni sono l'una la negazione dell'altra. Un checkpoint che produce un pavimento non nullo e' esattamente un checkpoint che il passo 1 rifiuta: o il rimedio e' inerte, o il passo 1 e' stato tacitamente derogato. Aggravante: delle tre funzioni in albero che consumano quel campo contro un orologio locale, due lo trattano come errore.\n\nRF-002 medium: la sussunzione e' falsa. Le capacita' preesistenti della chiave di rilascio sono di diniego o di ancoraggio sulla verifica di catena; quella aggiunta e' di AMMISSIONE sul trasporto. Con un'ancora avanti di delta, un ricevente con l'orologio esatto accetta un'attestazione postdatata che rifiutava, e la finestra reale diventa D_max + delta con delta scelto da chi firma il checkpoint. Il diniego non sussume l'ammissione.\n\nLa reviewer corregge inoltre se stessa su un punto che nessuno aveva nominato e che il Lead aveva citato come accertamento: aveva stabilito che il minorante e' fail-closed guardando la sola meta' della scadenza della regola 5. La regola ha due meta', e sull'altra alzare now_ms AMMETTE. Un minorante non e' fail-closed: e' fail-closed su una meta' e fail-open sull'altra. E' la forma di RF-001 di REVIEW-027 commessa da lei, ereditata da identity.md, e va corretta nel debito e non solo nella review.\n\nSul drift conferma le tre fonti verbatim, non trova una quarta contraria, ne trova due che confermano, e osserva che l'implementatore si e' sottovalutato: la lettura \"si applica in sync\" e' confutata anche documentalmente dall'elenco chiuso del passo 4 di ledger.md."
    evidence_refs: ["SPEC-020", "DEBT-017", "REVIEW-034"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-035-EVENT-002"
    timestamp: "2026-08-26T15:00:54.159225400+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Remediation dei sei finding consegnata da AGENT-001. RF-001 chiuso stabilendo che la freschezza e' precondizione per ancorare lo stato di catena e non per minorare il tempo reale; RF-002 accolto con la regola 5 divisa sui due orologi e la sussunzione ritirata; RF-003, RF-004, RF-006 chiusi; RF-005 curato a meta' e il residuo riportato. Da verificare dal Lead."
    evidence_refs: ["SPEC-020"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-035-EVENT-003"
    timestamp: "2026-08-26T15:01:15.807836100+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificata dal Lead rieseguendo: 181 test da 179, published_artifacts.py PASS con 150 candidati C10 da 148, prova in negativo PASS con ogni probe osservata fallire da sola, protocol_hashes.py PASS senza valori mossi. git diff --numstat su core/coblox-core/src/light_client.rs non restituisce nulla: checkpoint_is_fresh e' invariata riga per riga, come la remediation dichiara.\n\nRF-001 chiuso con una risposta migliore del rimedio, e registrata prima di toccare il codice. Il passo 1 valuta la freschezza sull'orologio locale nudo e non puo' fare altrimenti: valutata sull'orologio pavimentato l'eta' diventerebbe max(0, locale meno issued_at), quindi ogni checkpoint datato avanti leggerebbe eta' zero e la guardia sarebbe vacua. E proprio per questo il pavimento non e' subordinato al verdetto del passo 1: la deroga tacita non era che il passo 1 accetti checkpoint dal futuro, era una citazione sbagliata, perche' il punto 5 nominava il passo 1 come precondizione del pavimento importandone la freschezza, che il pavimento non richiede.\n\nLa ragione e' la sostanza: la freschezza e' precondizione per ancorare lo stato di catena, non per minorare il tempo reale. Un minorante ha bisogno di una sola proprieta' - che issued_at_ms nomini un istante realmente trascorso - e quella discende dalla firma, non dall'eta'. Un checkpoint vecchio e' un pavimento piu' debole, mai non sicuro.\n\nL'aggravante di AGENT-007 e' spiegata in modo ispezionabile invece che confutata: le due funzioni che trattano il campo come errore calcolano una differenza, quindi una durata negativa priva di senso; la terza calcola un massimo, per cui \"avanti al mio orologio\" e' l'unico caso in cui fa qualcosa. La discriminante e' l'operazione.\n\nE la composizione e' asserita e non ragionata: step_one_and_the_attestation_floor_do_not_compose, verificato esistere in tests/light_client_perimeter.rs:523, riproduce la forma a due cicli della review. E' il test che fallisce il giorno in cui qualcuno aggiusta il pavimento subordinandolo al passo 1 - mossa che altrimenti non romperebbe nulla e lo renderebbe inerte per sempre.\n\nRF-002 accolto e la sussunzione ritirata invece che riscritta in silenzio: la riga sbagliata resta nell'evidenza pre-review con un rinvio. La sussunzione diventa vera solo dopo la separazione, quindi e' conseguenza del rimedio e non premessa. Il test dimostra l'ammissione invece di descriverla.\n\nRF-003 chiuso in tre luoghi piu' un quarto che la review non nominava e che era sbagliato e non solo generoso: la documentazione di modulo motivava il residuo con la derivazione che RF-001 ha demolito. RF-005 curato a meta' e il residuo riportato con la ragione: la correzione vera sono due tipi distinti, cioe' un cambiamento di API oltre lo scopo."
    evidence_refs: ["SPEC-020", "REVIEW-035"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-035-EVENT-004"
    timestamp: "2026-08-26T15:01:33.428173+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "GATE-SECREVIEW soddisfatta. Sei finding, cinque chiusi e uno riportato come residuo con la ragione.\n\nQuesta review vale oltre la spec per il finding che l'ha aperta e per il modo in cui e' stato chiuso. RF-001 mostrava che il pavimento era non nullo solo per i checkpoint che il passo 1 rifiuta - due condizioni l'una la negazione dell'altra - quindi il rimedio era inerte oppure il passo 1 era stato tacitamente derogato. La risposta non e' stata alzare una soglia ne' scegliere fra i due corni: e' stata stabilire che la freschezza e' precondizione per ancorare lo stato di catena e non per minorare il tempo reale, e che un minorante ha bisogno solo che issued_at_ms nomini un istante realmente trascorso, proprieta' che discende dalla firma e non dall'eta'.\n\nVa registrato che la reviewer ha corretto se stessa su un punto che nessuno aveva nominato e che il Lead aveva citato come accertamento: aveva stabilito che il minorante e' fail-closed guardando la sola meta' della scadenza della regola 5. La regola ha due meta', e sull'altra alzare now_ms ammette. Un minorante non e' fail-closed: e' fail-closed su una meta' e fail-open sull'altra.\n\nE' il terzo caso di questa sessione in cui una dimostrazione valida e' stata letta come conclusiva oltre il perimetro su cui era fatta - dopo l'errore di AGENT-007 su DEBT-022 e quello di AGENT-002 su R2 - e il primo in cui l'errore ha attraversato tre artefatti prima che qualcuno lo guardasse: la valutazione del debito, la review del Lead che la citava, e identity.md che l'aveva ereditata. La correzione e' ora scritta in quattro luoghi come ragione della regola e non come nota."
    evidence_refs: ["SPEC-020", "DEBT-017"]
    implementation_agent: "AGENT-001"
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [review, security, identity, light-client]
activity:
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

**`GATE-SECREVIEW` non superata. Verdetto raccomandato: `changes-requested`.** Due
finding *high* e *medium* nel perimetro, quattro *low*, e una correzione alla mia
stessa valutazione di [DEBT-017] che va scritta prima di tutto il resto.

La sostituzione della sorgente del minorante — `checkpoint.issued_at_ms` al posto
del `timestamp_ms` dell'ultimo blocco finalizzato — è **corretta e migliore** sul
punto per cui è stata fatta, e la confermo riga per riga: la leva del set di
validatori è zero perché l'ingresso non esiste, non perché un attaccante sia
limitato. Il Lead ha ragione che *fail-closed* qualifica la sicurezza e non la
disponibilità, e la sostituzione azzera davvero la conversione «rallentamento →
partizione» **per i validatori**.

Ciò che nessuno ha scritto è che la stessa sostituzione **crea** la coppia di
problemi che questa review porta: il pavimento è operativo **solo** per i
checkpoint che il passo 1 dell'algoritmo light-client rifiuta (RF-001), e nella
regione in cui è operativo la chiave di rilascio guadagna una capacità di
**ammissione** che prima non aveva e che l'argomento della sussunzione non copre
(RF-002). I due finding sono i due corni della stessa dicotomia: **o il pavimento
è inerte, o la chiave di rilascio ha una leva nuova.** Come consegnato siamo sul
secondo corno, e nessuno dei due è dichiarato.

Entrambi sono riprodotti con test eseguiti su una copia dell'albero fuori dal
repository, e passano.

## La correzione alla mia valutazione di [DEBT-017], che viene prima

**La mia valutazione è superata da questa consegna in tre punti, e in due di essi
l'errore è mio e non dell'implementatore.**

1. **La sorgente.** Raccomandavo `max(orologio locale, timestamp_ms dell'ultimo
   blocco finalizzato)` e nominavo il checkpoint come *«seconda fonte, più debole
   in freschezza»*. L'ordine è rovesciato: il checkpoint è la fonte **giusta** e
   il blocco finalizzato è quella sbagliata, per le due ragioni che
   `identity.md` punto 5 ora scrive. Confermo la contraddizione alla spec e la
   trovo migliore della spec.
2. **«Chiude il termine illimitato».** L'ho scritto io nell'esito (A) ed è falso,
   e l'implementatore se n'è accorto da sé verificando la misura. La catena della
   sua correzione è **esatta** e l'ho riverificata (§*Punto 1* qui sotto).
3. **Ed ecco il punto che nessuno ha ancora nominato: la mia valutazione ha
   guardato un estremo solo.** Ho stabilito che l'abuso del minorante è
   *fail-closed* esaminando la sola metà **della scadenza** della regola di
   rifiuto 5 — `now_ms > expires_at_ms`, dove alzare `now_ms` rifiuta. La regola 5
   ha **due** metà, e sull'altra — `created_at_ms > now_ms + S_max` — alzare
   `now_ms` **ammette**. Un minorante non è fail-closed: è fail-closed su una
   metà e fail-**open** sull'altra. È esattamente la forma di RF-001 di
   [REVIEW-027], commessa da me, nell'artefatto che il Lead ha poi citato come
   accertamento. `identity.md` punto 5 eredita l'errore e lo rende esplicito
   scrivendo che il pavimento *«also loosens»* la metà dello skew e che
   *«both directions therefore move the way this section wants»*. La seconda
   direzione **non** muove nel verso che quella sezione vuole: allentare
   l'ammissione è l'evasione della regola 4 che il punto 3 dello stesso documento
   nomina come l'unico abuso reale del termine `S_max`. RF-002 è la
   quantificazione di questa frase.

## Punto 1 — Il pavimento non chiude il termine illimitato: la catena è esatta, la formulazione scritta è esatta in una direzione e muta nell'altra

**La catena regge.** Verificata sul codice e non sulla prosa:
`light_client::checkpoint_is_fresh(now_ms, issued_at_ms, W)` calcola
`age = now_ms - issued_at_ms` e confronta con `W`, e l'unico `now_ms` che un
ricevente possiede è il proprio orologio. Un ricevente indietro di `b` accetta
quindi un checkpoint di età vera fino a `W + b`; con `A` l'età vera, il terzo
addendo passa da `b` a `min(b, A)` e nel caso peggiore `A = b`, il pavimento
coincide con l'orologio locale e non si guadagna nulla. `min(b, A)` è **esatto**:
`now_ms = max(t - b, t - A) = t - min(b, A)`. La conclusione scritta — *«the
residue is therefore still unbounded by any rule of this protocol»* — è vera, ed
è la forma onesta.

**È generosa in un punto, e il punto non è piccolo.** La derivazione vale per
`A ≥ 0`, cioè per un'ancora **non successiva** al tempo reale. Se `issued_at_ms`
eccede il tempo reale di `Δ`, il terzo addendo non è `min(b, A)` con un valore
piccolo: è **negativo**, `now_ms` supera il tempo reale, e nella somma dichiarata
compare un **quarto addendo che il blocco a tre termini non porta**, sull'altra
metà della regola 5. La finestra di accettazione in tempo reale di
un'attestazione postdatata diventa `D_max + Δ` invece di `D_max + S_max`, e `Δ` è
scelto da chi firma il checkpoint. Il documento non ha una riga su questo caso, e
la frase *«both directions therefore move the way this section wants»* lo esclude
per affermazione.

**Verdetto sul punto 1:** la formulazione scritta è **esatta sul termine che
dichiara** e **incompleta sul dominio in cui vale**. Va aggiunta l'ipotesi
`issued_at_ms ≤ tempo reale`, oppure — meglio, ed è il rimedio di RF-002 — va
tolta la possibilità che il pavimento agisca sulla metà dell'ammissione.

## Punto 2 — La sorgente sposta la fiducia sulla chiave di rilascio, e la sussunzione è falsa

L'implementatore sostiene che la capacità aggiunta sia *«strettamente sussunta»*
da quella che la chiave di rilascio già detiene, quindi ad ampiezza marginale
zero. **Non lo è**, e la ragione è di genere e non di grado.

La capacità preesistente della chiave di rilascio è di **diniego e di ancoraggio
sulla verifica di catena**: può far fallire in chiuso un light client, o ancorarlo
a una catena che l'attaccante deve comunque produrre. La capacità che questa spec
aggiunge è di **ammissione sul livello di trasporto**: far accettare a un
ricevente **con l'orologio giusto** un'attestazione che quel ricevente rifiutava,
e che eccede il tetto della regola 4 di una quantità che nessuna regola limita.
Il diniego non sussume l'ammissione. Dettagli e riproduzione in RF-002.

Va inoltre detto ciò che il Lead ha aggiunto alla mia valutazione e che qui
**ritorna identico su un altro attore**: poiché un peer senza attestazione valida
è *«rejected and disconnected»*, chi gonfia `issued_at_ms` di `Δ > D_max` non
degrada una verifica — **disconnette il trasporto** di ogni nodo che detiene quel
checkpoint. Il documento usa questo argomento, testualmente, per **scartare**
`timestamp_ms`; non lo applica alla sorgente che adotta. La differenza reale fra i
due casi non è che l'una leva esista e l'altra no: è che il set di validatori
scrive un valore **condiviso e osservabile**, mentre la chiave di rilascio
distribuisce artefatti **fuori banda e per destinatario**, quindi la stessa leva è
in più **selettiva** e non osservabile dalla rete.

## Punto 3 — La risposta sul drift è istruita bene, e meglio di come è scritta

Le tre fonti dicono ciò che si sostiene, verbatim:

- `ledger.md:828-829` — *«Timestamps MUST be greater than the median of the
  previous 11 finalized blocks and no more than the active maximum clock drift
  after the proposal is received.»*
- `README.md:105-107` — *«not run ahead of the receiver's clock by more than the
  active maximum drift — monotonicity and a ceiling, not a step»*, che nomina
  l'orologio del ricevente.
- `params.rs:340-341` — *«Maximum accepted clock drift for a received
  proposal.»*

**Non esiste una quarta fonte che le contraddica, e ne esistono due che le
confermano**, entrambe non citate:

- `ledger.md:841` — *«an upper bound against the receiver's clock»*, nella stessa
  sezione che dichiara la cadenza non imposta;
- `ledger.md:737-741` — l'argomento sul grinding del proposer **dipende** dal
  soffitto: è quel soffitto a produrre la finestra di `10³–10⁶` valori legali di
  `timestamp_ms`. Se il controllo non fosse applicato, quella misura sarebbe
  illimitata. È una conferma per conseguenza, che è la forma più forte.

**E la confutazione della lettura «si applica anche in sync» non è solo
auto-evidente: è anche documentale, e l'implementatore si è sottovalutato.** Il
passo 4 di `ledger.md` §*Light-client balance verification* **enumera** i
controlli che un client applica a ogni header più recente — `block_id`, catena,
versione, altezza, ID precedente, continuità del set, certificato di quorum,
revoche del checkpoint — e `timestamp_ms` **non è nell'elenco**. Un elenco chiuso
che non lo contiene è una fonte, non un'evidenza di sé. `GATE-DRIFT-ANSWERED-FIRST`
è soddisfatta, e l'ordine è dimostrato dalla baseline a 177 test registrata
accanto alla risposta.

## Punto 4 — `AttestationClock` è una convenzione, dichiarata meglio della media, e più debole di quanto la sintesi dica

Confermo l'accertamento del Lead e lo porto un passo più in là, perché un passo
più in là c'è un fatto che cambia il giudizio.

Il tipo **non impone la provenienza**: `with_checkpoint_floor` prende due `u64`
grezzi. Il commento lo dichiara con precisione — *«An unverified checkpoint is an
attacker-chosen number and this constructor cannot tell»*, e in grassetto
*«`timestamp_ms` is not this parameter and must not be passed as it»*. È una
convenzione **documentata con il suo pericolo nominato**, che è più di quanto
[DEBT-029] descriva un livello più in là, dove il legame è affidato a un *should*.

**Ma la dichiarazione non basta, e la ragione è aritmetica.** `now_ms` è
`max(local_clock_ms, checkpoint_issued_at_ms)`, e `max` è **simmetrico**: due
argomenti dello stesso tipo, invertiti, producono **lo stesso `now_ms`** e quindi
lo stesso verdetto su ogni regola di rifiuto. L'unico osservabile che distingue lo
scambio è `floor_ms()`, che nessuna regola legge e che oggi compare solo nei test.
Uno scambio di argomenti, o un `timestamp_ms` passato al posto di
`issued_at_ms`, è quindi **invisibile nel comportamento**: non esiste un caso in
cui il difetto si manifesti come un rifiuto sbagliato che qualcuno noti. Una
convenzione la cui violazione non ha alcun sintomo osservabile è più debole di una
convenzione violabile ma rumorosa.

**E non esiste oggi alcun sito di chiamata di produzione.** `grep` su
`core/*/src/` non trova alcuna costruzione di `AttestationClock` fuori dai test,
e `identity.rs` non chiama `verify` da nessuna parte. La garanzia «`identity.rs`
non può costruire un secondo orologio» è **vera e vacua**: il sito in cui
l'obbligo di provenienza andrà davvero onorato è il livello di trasporto del nodo,
che è fuori da questo crate e non ancora scritto. È RF-005, ed è materia da
debito e non da blocco.

## Punto 5 — `GATE-BOOTSTRAP-UNCHANGED`, e la grandezza costante che nessuno ha variato

La gate è soddisfatta e la sua forma è la migliore delle tre spec recenti: la
proprietà è **strutturale prima che asserita** (`local_only` ha `floor_ms() == 0`
per costruzione, e l'altro costruttore richiede un dato che il nodo in bootstrap
non ha), e il caso che conta **riproduce** il limite dichiarato invece di
assumerlo. Ho cercato un percorso in cui un nodo senza checkpoint riceva un
pavimento: non esiste, i costruttori sono due e il campo è privato.

**Il passo 4 di [SKILL-001] produce però qualcosa anche qui, come nelle ultime
tre spec.** L'implementatore risponde che la grandezza costante nella gate
preesistente è *la forma dell'orologio*, e che nel caso nuovo la costante è
*l'attestazione*, deliberatamente. Entrambe le risposte sono vere e **nessuna
delle due è la grandezza che conta**. La grandezza costante in **tutti e dodici**
i casi — gli undici della gate preesistente e quello nuovo — è la **metà della
regola 5 che viene esercitata**: in ogni caso, `created_at_ms` è nel passato
rispetto a `now_ms`, e ciò che varia è sempre e solo la metà della **scadenza**.
I due «bordi dello skew» della gate preesistente variano l'orologio locale con
`local_only`, cioè con pavimento zero.

La cella «pavimento non nullo × metà dell'ammissione» della matrice è **vuota**, e
in quella cella vive RF-002. La direzione che il documento dichiara con *«it also
loosens … both directions therefore move the way this section wants»* è l'unica
affermazione del punto 5 che **nessun test tocca**.

## Punto 6 — Le tre correzioni a `identity.md`: una esatta, due corrette in una direzione sola

Le tre affermazioni che dicevano *«bounded only by the length of the window»*
sono state trovate tutte e tre, ed è la passata giusta. L'esito però non è
uniforme.

- **§*Authentication on a connection*: esatta.** Ridefinisce `now_ms` come la
  quantità pavimentata *«and not simply the local clock»*, con il rinvio. Nulla da
  dire.
- **Punto 2: corretta in una direzione.** *«…the sum point 3 writes out, offset by
  however far the receiver's clock is behind, which point 5 floors for a receiver
  holding a checkpoint and leaves unbounded for one that is not.»* Il lettore
  conclude che **con** un checkpoint il termine sia limitato. Il punto 5 conclude
  il contrario: con un checkpoint è `min(b, A)` e `A` non è limitato da alcuna
  regola. La correzione ha spostato la frase da «limitato dalla durata» a
  «limitato dalla somma più il pavimento», e il residuo è ancora fuori.
- **§*Anti-reuse property*: corretta in una direzione.** *«bounded only by the
  window in which a receiver accepts the attestation — the sum of points 3 and 5,
  not the declared duration alone.»* Stessa forma: *bounded* più *the sum*, dove
  il punto 5 non è un addendo ma il luogo in cui si dichiara che un addendo è
  illimitato.

È RF-004: la famiglia 2 non è stata commessa nel punto nuovo — dove il testo è
scrupoloso — ed è sopravvissuta nei due punti vecchi che la stessa passata è
andata a correggere.

## Code observations

`identity.rs::verify` è corretto nella meccanica: il pavimento entra come
`clock.now_ms()` e nient'altro cambia; il commento sull'asimmetria delle due
direzioni è nel posto giusto; `latest_acceptable_creation` usa `saturating_add`,
quindi `S_max` grande non avvolge. `floor_ms()` non può sottrarre in negativo per
costruzione. `AttestationClock` è `Copy` e `#[must_use]`, i campi sono privati, i
costruttori sono `const`.

Un fatto che nessuno ha scritto e che pesa su RF-001: **nel medesimo modulo, lo
stesso campo dello stesso oggetto firmato è trattato in due modi opposti.**
`measure_cadence_from_checkpoint` fa `now_ms.checked_sub(checkpoint_issued_at_ms)`
e restituisce `CadenceError::ClockRegression` quando `issued_at_ms` supera
l'orologio locale; `light_client::checkpoint_is_fresh` fa la stessa sottrazione e
restituisce `Error::Arithmetic("checkpoint issued in the future")`.
`with_checkpoint_floor` è la sola delle tre per cui quel medesimo caso non è un
errore ma **l'unico caso in cui la funzione fa qualcosa**. `GATE-ONE-CLOCK` è
soddisfatta sul campo e sul modulo, e **non** sul dominio ammissibile del campo:
[SPEC-016] e [SPEC-020] non concordano su quali valori di `issued_at_ms` siano
legittimi rispetto all'orologio locale.

## Tests and verification

Riverificato per campionamento sul contenuto, non rieseguendo la passata del Lead:
le due probe nuove pinnano ciò che dichiarano, e `transport-attestation-clock-floor`
pinna correttamente **il nome del campo** e non la sola presenza di un pavimento —
è la mutazione 3 della prova in negativo ed è la mutazione giusta da avere.
Il `count` di `transport-attestation-skew-tolerance` da 3 a 7 è giustificato voce
per voce nel `why`, che è la forma corretta.

Le prove in negativo sono quattro e ciascuna colpisce una cosa diversa; la
mutazione 2 (pavimento come sostituzione invece che come minorante) è quella che
mancava nelle spec precedenti ed è presente qui.

**Ciò che la suite non copre è nominato in RF-006**, ed è la cella individuata al
punto 5.

## Production quality and documentation compliance

Conforme, con l'eccezione di RF-003: la correzione di *«chiude»* è stata applicata
agli artefatti pubblicati e **non** ai due artefatti che nessuna gate legge.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

### RF-001 — `high` — `security-boundary` — Il pavimento è operativo solo per i checkpoint che il passo 1 rifiuta: o è inerte, o il passo 1 è stato tacitamente derogato

`identity.md` punto 5 qualifica il checkpoint ammissibile citando *«step 1 of
light-client balance verification»* e **nominandone due requisiti su tre**: firma
verificata sotto una trust key già posseduta, e `chain_id` uguale a quello
configurato. Il terzo requisito del passo 1 — `now - issued_at_ms ≤
max_weak_subjectivity_age_ms` — non è nominato **nella regola**, ma è invocato
**nell'analisi del residuo** dello stesso punto 5, che deriva `A ≤ W + b`
esattamente da esso.

Le due invocazioni non possono essere entrambe vere, e la ragione è banale una
volta scritta:

- il passo 1 è implementato da `checkpoint_is_fresh`, che calcola
  `now_ms.checked_sub(issued_at_ms)` e **fallisce** quando `issued_at_ms >
  now_ms` (`light_client.rs:69-70`, *«checkpoint issued in the future»*);
- `with_checkpoint_floor` alza `now_ms` **solo** quando `issued_at_ms >
  local_clock_ms` (`cadence.rs:155-159`).

Le due condizioni sono l'una la negazione dell'altra sullo stesso orologio. Ne
segue: **ogni checkpoint che soddisfa il passo 1 contro l'orologio del ricevente
ha `floor_ms() == 0`.** Il pavimento è non nullo esclusivamente per i checkpoint
che il passo 1 rifiuta — compreso quello del caso di conformità consegnato, che
usa `issued_at_ms = 5 000 000` contro un orologio locale a `1 500 000`.

Non è un cavillo, perché decide se [DEBT-017] sia chiuso:

- se il ricevente applica il passo 1 con il proprio orologio prima di usare il
  checkpoint come pavimento — cioè se la regola è quella che il punto 5 cita —
  **il rimedio è identicamente inerte** e l'esposizione è quella di prima di
  [SPEC-020];
- se non lo applica — cioè se il pavimento accetta ancore successive
  all'orologio locale, che è la lettura sotto cui la consegna funziona — allora
  la derivazione `A ≤ W + b` del punto 5 **non ha base**, `A` non è limitato da
  nulla in nessuna direzione, e vale RF-002.

Il documento afferma entrambe le cose in due paragrafi consecutivi e non sceglie.

**Aggravante di forma, dentro lo stesso modulo:** delle tre funzioni in albero che
consumano `checkpoint_issued_at_ms` contro un orologio locale, **due lo trattano
come errore** — `Error::Arithmetic("checkpoint issued in the future")` e
`CadenceError::ClockRegression` — e la terza è quella che ne fa il proprio caso
operativo. `GATE-ONE-CLOCK` verifica che il campo sia lo stesso; non verifica che
il suo dominio ammissibile lo sia, e non lo è.

**Riproduzione** (su copia dell'albero fuori dal repository,
`tests/rev035_attacks.rs`, eseguita e verde):

```rust
let local_clock_ms = 1_500_000u64;
// il checkpoint del caso di conformità consegnato
assert_eq!(
    AttestationClock::with_checkpoint_floor(local_clock_ms, 5_000_000).floor_ms(),
    3_500_000);
for window in [0u64, 1, 60_000, u64::MAX] {
    assert!(checkpoint_is_fresh(local_clock_ms, 5_000_000, window).is_err());
}
// e viceversa: ogni checkpoint che il passo 1 accetta lascia il pavimento a zero
for issued in [0u64, 1, local_clock_ms - 1, local_clock_ms] {
    assert!(checkpoint_is_fresh(local_clock_ms, issued, u64::MAX).unwrap());
    assert_eq!(
        AttestationClock::with_checkpoint_floor(local_clock_ms, issued).floor_ms(), 0);
}
```

```
running 2 tests
test rf001_step_one_and_the_floor_do_not_compose ... ok
```

**Rimedio richiesto.** Scegliere e **scrivere** quale delle due composizioni vale,
e ricalcolare l'esposizione dichiarata sotto la scelta.

- Se il pavimento ammette ancore successive all'orologio locale — che è l'unica
  scelta sotto cui il rimedio fa qualcosa — allora il punto 5 deve dire che il
  controllo di freschezza del passo 1 **non** si applica al checkpoint usato come
  pavimento, deve togliere la derivazione `A ≤ W + b` che vi poggia, e deve
  dichiarare che il valore di `issued_at_ms` accettato come pavimento **non è
  limitato verso l'alto da alcuna regola** — che è il presupposto di RF-002.
- La forma minima che rende la scelta sicura è quella di RF-002: pavimentare
  **solo** la metà della scadenza. Sotto quella forma il pavimento resta operativo
  e la sua ampiezza verso l'alto smette di essere una superficie.

### RF-002 — `medium` — `security-boundary` — La chiave di rilascio guadagna una capacità di ammissione, non solo di diniego: la sussunzione è falsa e l'ampiezza marginale non è zero

L'evidenza sostiene che la capacità aggiunta sia *«strettamente sussunta»* da
quella che la chiave di rilascio già detiene in quanto ancora di fiducia totale.
Le capacità preesistenti citate sono tutte di **diniego** o di **ancoraggio**:
firmare `height`, `block_id`, `validator_set_hash`, le revoche, e far fallire in
chiuso la verifica di catena. Nessuna di esse è una capacità di **far accettare a
un ricevente qualcosa che rifiutava**, e il diniego non sussume l'ammissione.

**L'attacco.** Un ricevente con l'orologio **esatto** e un checkpoint il cui
`issued_at_ms` eccede il tempo reale di `Δ`. La metà dell'ammissione della regola
5 è `created_at_ms ≤ now_ms + S_max`, e `now_ms` è ora `tempo reale + Δ`. Un
emittente che postdata — l'unico abuso reale di `S_max`, nominato dal punto 3 di
`identity.md` come evasione della regola 4 da parte di un nodo che collude con sé
stesso — ottiene quindi una credenziale accettata **per `D_max + Δ` di tempo
reale** invece di `D_max + S_max`, con `Δ` scelto da chi firma il checkpoint e
limitato da nessuna regola. Il tetto della regola 4 e la somma dichiarata al punto
3 sono entrambi evasi di una quantità arbitraria.

**Riproduzione** (stessa copia, eseguita e verde). Tempo reale `1 000 000`,
orologio del ricevente **corretto**, `max_validity_ms = 1 000 000`,
`max_future_skew_ms = 5 000`, `Δ = 10 000 000`:

```rust
// senza pavimento: rifiutata, ed è tutto ciò che la metà dello skew serve a fare
assert!(matches!(verify(AttestationClock::local_only(1_000_000)).unwrap_err(),
    Error::Attestation(AttestationError::Expired { .. })));
// con un'ancora avanti di Δ: lo stesso ricevente, lo stesso orologio, accetta
assert!(verify(AttestationClock::with_checkpoint_floor(1_000_000, 11_000_000)).is_ok());
// ampiezza reale della finestra di accettazione: Δ + D_max, non D_max + S_max
assert_eq!(widened, delta + bounds.max_validity_ms);
```

```
test rf002_the_floor_widens_the_accepted_window_in_the_skew_direction ... ok
```

**La seconda metà della capacità, che è di diniego ma resta nuova.** Con
`Δ > D_max` **ogni** attestazione in circolazione risulta scaduta, e poiché
§*Authentication on a connection* impone *«rejected and disconnected»*, il
detentore della chiave di rilascio partiziona il trasporto dei nodi che detengono
quel checkpoint. È **testualmente** l'argomento che il punto 5 usa per **scartare**
`timestamp_ms`, non applicato alla sorgente adottata. E rispetto al set di
validatori la leva è **peggiore in due modi**: i checkpoint sono distribuiti fuori
banda e per destinatario, quindi la leva è **selettiva**; e la misura di cadenza
che potrebbe accorgersene fallisce solo per `Δ` grandi rispetto alla finestra
misurata, quindi esiste un regime in cui la partizione avviene mentre
`measure_cadence_from_checkpoint` resta `WithinBand`.

**Perché `medium` e non `high`:** l'attore è l'ancora di fiducia totale del
client, la cui compromissione è già catastrofica per la verifica di catena, e la
metà di ammissione richiede in più una chiave di identità che collude. **Perché
non `low`:** la capacità agisce su un sottosistema **diverso** da quello che la
chiave di rilascio governava — il trasporto, che la ragione 1 di `identity.md`
aveva deliberatamente disaccoppiato dallo stato di catena — è di ampiezza non
limitata da alcuna regola, ed è **negata per iscritto** in due artefatti
(l'evidenza della spec, e `identity.md` con *«both directions therefore move the
way this section wants»*).

**Rimedio richiesto, ed è piccolo.** Pavimentare **solo** la metà della scadenza,
valutando la metà dell'ammissione sull'orologio locale non pavimentato:

```text
rifiuta se   now_floored > expires_at_ms
rifiuta se   created_at_ms > local_clock + max_transport_attestation_future_skew_ms
```

Sotto questa forma il pavimento è **monotonamente rifiutante**, l'affermazione
*«can only raise `now_ms` … and can never revive one»* diventa l'intera verità
invece che metà, la ragione 1 è protetta esattamente come oggi (la tolleranza di
skew resta sull'orologio locale, che è ciò che il nodo indietro possiede), e
l'ampiezza marginale della chiave di rilascio torna a essere di solo diniego. In
alternativa, se si vuole conservare l'allentamento, va dichiarato come quarto
addendo della somma con il suo attore e la sua ampiezza, e la frase sulle «due
direzioni» va ritirata.

### RF-003 — `low` — `documentation` — «Chiude» sopravvive nei due artefatti che nessuna gate legge

La correzione da *«chiude il termine»* a *«ne cambia la grandezza da cui
dipende»* è applicata a `identity.md` e a TM-37. Non è applicata dove nessuna
probe guarda:

1. `core/coblox-core/src/identity.rs:147-151`, doc di `verify`: *«of which only
   the first two terms are bounded by anything. **The third is closed by a
   floor** under `now_ms`»*. È l'affermazione che l'implementatore ha stabilito
   essere falsa, nel commento della funzione che applica la regola.
2. `sim/tools/published_artifacts.toml`, `why` della probe
   `transport-attestation-clock-floor`: *«This line is the floor that closes
   it»*; e `why` della probe `transport-attestation-residual-is-declared`, che
   dice che il terzo addendo *«stays unbounded»* **per il ricevente senza
   checkpoint**, implicando per differenza che con un checkpoint sia limitato.

È [DEBT-031] in atto — la documentazione di modulo del crate fa affermazioni
normative che nessuna gate legge — e qui la famiglia 2 non è stata introdotta ma
**lasciata indietro** dalla stessa passata che l'ha corretta altrove.

**Riproduzione:** `grep -n "closed by a floor" core/coblox-core/src/identity.rs`
e `grep -n "floor that closes it" sim/tools/published_artifacts.toml`.

**Rimedio:** allineare i tre testi alla formulazione del punto 5.

### RF-004 — `low` — `documentation` — Due delle tre correzioni sono esatte in una direzione e generose nell'altra

Dettaglio al punto 6 sopra. Il punto 2 e §*Anti-reuse property* dicono ora
*«bounded … the sum»*, dove il punto 5 dello stesso documento conclude che il
terzo addendo è illimitato **anche con un checkpoint**. Un lettore che si ferma a
una delle due frasi conclude che l'esposizione sia limitata.

**Rimedio:** in entrambe, sostituire *«bounded by»* con il rinvio esplicito al
punto 5 e alla parola che quel punto usa — `UNBOUNDED` — invece di *«the sum»*.

### RF-005 — `low` — `maintainability` — La provenienza è una convenzione la cui violazione non ha sintomi, e non esiste ancora un sito di chiamata di produzione

`with_checkpoint_floor(local_clock_ms, checkpoint_issued_at_ms)` prende due `u64`
nello stesso tipo. Poiché `now_ms = max(a, b)` è **simmetrico**, uno scambio dei
due argomenti produce lo stesso `now_ms` e quindi lo stesso verdetto su ogni
regola; l'unico osservabile che cambia è `floor_ms()`, che nessuna regola legge.
Lo stesso vale per un `timestamp_ms` passato al posto di `issued_at_ms` da un
chiamante che ne sia in possesso: il difetto che il commento vieta in grassetto è
**invisibile nel comportamento**.

Nessuna costruzione di `AttestationClock` esiste oggi fuori dai test, e
`identity.rs` non chiama `verify` da nessuna parte: l'obbligo di provenienza sarà
onorato per la prima volta al livello di trasporto del nodo, fuori da questo
crate.

**Rimedio suggerito, e non blocca questa spec:** un tipo che porti la provenienza
— `CheckpointIssuedAt(u64)` costruibile solo dal checkpoint verificato, o il
valore verificato stesso — al posto del secondo `u64`. È la forma che [DEBT-029]
chiede un livello più in là, e va aperta come debito proprio con questa
motivazione, non risolta qui a costo di un'API che nessuno chiama ancora.

### RF-006 — `low` — `test-quality` — La metà dell'ammissione della regola 5 non vede mai un pavimento non nullo

Passo 4 di [SKILL-001], applicato alla matrice invece che al singolo caso: in
tutti e dodici i casi — gli undici di `gate_no_attestation_rejected` e quello di
`the_external_clock_floor_closes_the_term_no_parameter_bounds` — `created_at_ms`
è nel passato rispetto a `now_ms`, e i due «bordi dello skew» usano `local_only`.
La cella «pavimento non nullo × `created_at_ms > now_ms`» è vuota, ed è l'unica
affermazione del punto 5 che nessun test tocca. È anche la cella in cui vive
RF-002.

**Rimedio:** un caso che vari il pavimento tenendo `created_at_ms` **avanti**
all'orologio locale, che asserisca la regola scelta nella remediation di RF-002.
Il test di RF-002 in questa review è quel caso scritto in negativo.

## Cosa ho attaccato senza riuscire a romperlo

**Che la leva del set di validatori sia davvero zero.** È l'affermazione da cui
dipende l'intera sostituzione della sorgente. Ho cercato un percorso qualsiasi da
`timestamp_ms` a `AttestationClock`: `grep` su `cadence.rs` trova `timestamp_ms`
in **dieci** righe e tutte e dieci sono commenti; il campo non è parametro di
alcuna funzione del modulo. Il minorante non ha ingressi scritti dai validatori.
**Non si è rotto**, ed è una proprietà di costruzione e non un limite su un
attaccante.

**Che il pavimento possa resuscitare un'attestazione scaduta.** È il modo in cui
un minorante si trasforma nel suo contrario. `now_ms = max(...)` non può abbassare
l'orologio locale, e la mutazione 2 della prova in negativo — il pavimento come
**sostituzione** invece che come minorante — è già in albero e fallisce quando
introdotta. Ho verificato la direzione anche sul bordo (`expires_at_ms` esatto
accettato, `expires_at_ms + 1` rifiutato). **Non si è rotto.**

**Che il caso di bootstrap possa ricevere un pavimento.** I costruttori sono due,
il campo è privato, e `local_only` ha `floor_ms() == 0` per costruzione. Non
esiste un percorso in cui un nodo senza checkpoint sia trattato diversamente da
come era trattato prima di [SPEC-020], e le undici asserzioni scritte prima della
spec valgono invariate. **Non si è rotto.**

**Che le tre fonti sul drift dicessero meno di quanto si sostiene.** È l'attacco
che ho fatto con più attesa di successo, perché una risposta a due rami istruita
su tre citazioni è la forma in cui una citazione viene tirata. Le tre dicono
esattamente ciò che l'evidenza riporta, ne esistono **due in più** che le
confermano, e la lettura opposta sul sync è confutata **anche documentalmente**
dall'elenco chiuso del passo 4. **Non si è rotto, e ne esce più forte di come è
scritto.**

**Che l'aritmetica di `min(b, A)` fosse sbagliata.** È la formula su cui poggia
tutta l'onestà della consegna. `max(t - b, t - A) = t - min(b, A)`: esatta.
**Non si è rotta** — ma vale per `A ≥ 0`, e il dominio è ciò che RF-002 attacca,
non l'aritmetica.

**Che `floor_ms()` potesse sottrarre in negativo, o che `S_max` grande potesse
avvolgere.** `now_ms ≥ local_clock_ms` per costruzione in entrambi i costruttori;
`latest_acceptable_creation` usa `saturating_add`. **Non si è rotto.**

## Required follow-up

1. **RF-001 e RF-002 nel perimetro di questa spec**, perché decidono se
   [DEBT-017] sia chiuso e con quale esposizione dichiarata. La remediation
   minima è una: pavimentare la sola metà della scadenza, e scrivere quale
   composizione con il passo 1 vale.
2. **RF-003 e RF-004 nel perimetro**, sono redazionali.
3. **RF-005 come debito nuovo**, della famiglia di [DEBT-029], con la
   motivazione che lo scambio dei due `u64` è invisibile nel comportamento.
4. **[DEBT-017] resta `open` fino alla remediation**, e la sua sezione
   *Valutazione* va corretta con il punto 3 della sezione di apertura di questa
   review: il minorante non è fail-closed, è fail-closed su una metà della regola
   5 e fail-open sull'altra. È la mia valutazione a essere stata scritta su un
   estremo solo, e va detto nel debito e non solo qui.
5. **`max_clock_drift_ms` non fissato da alcun documento di genesi**: confermo il
   fatto registrato dal Lead e confermo che non blocca questa spec, perché la
   formulazione che vi poggiava è stata scartata.

## Final decision

**`GATE-SECREVIEW` non superata; verdetto raccomandato `changes-requested`.**

I due punti su cui l'implementatore ha chiesto che mi pronunciassi sono i due
punti giusti, ed è la ragione per cui questa review ha trovato qualcosa: **su
entrambi la risposta è meno favorevole di quella scritta.** Il pavimento non
chiude il termine — l'implementatore lo dice, e la sua catena è esatta — ma la
formulazione scritta vale su un dominio che il documento non dichiara; e la
sorgente `issued_at_ms` **non** è a costo marginale zero, perché la capacità che
aggiunge è di genere diverso da quelle che la chiave di rilascio già detiene.

La consegna resta la migliore delle ultime tre per una ragione che va scritta
accanto ai finding: **è l'unica in cui l'implementatore ha corretto la spec e la
valutazione di sicurezza invece di trascriverle**, e la correzione era giusta
tutte e due le volte. I finding di questa review non contraddicono quella
correzione: la portano dove non è arrivata.
