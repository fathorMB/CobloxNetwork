---
id: DEBT-022
title: "L'autorizzazione del burn di abbonamento non richiede che la chiave sia non revocata"
status: open
category: "security"
severity: "high"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "SPEC-015"
origin_ref: "F4 dell'evidenza di AGENT-006"
related_specs: ["SPEC-001","SPEC-015"]
related_reviews: ["REVIEW-024"]
related_decisions: ["ADR-006"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-26
tags: ["ledger","security","identity"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-022-EVENT-001"
    timestamp: "2026-08-26T00:26:52.057323400+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-015, su segnalazione di AGENT-006 che ha riportato invece di scrivere attorno al buco, come il dispatch le imponeva. Owner AGENT-007 e non AGENT-006 ne il Lead: e una questione di sicurezza sulla spesa e va valutata da chi ha mandato di attaccarla, ed e la stessa regola applicata a DEBT-013, DEBT-014 e DEBT-017."
    evidence_refs: []
---
# L'autorizzazione del burn di abbonamento non richiede che la chiave sia non revocata

## Statement

La regola di autorizzazione del burn di abbonamento in docs/protocol/ledger.md riga 347 dice soltanto che la chiave MUST derive payer_node_id. Le tre regole sorelle dello stesso documento dicono tutte the enrolled, unrevoked: riga 312 per fund_app, riga 398 per challenge_commitment, riga 871 per validator_candidacy. L'autorizzazione dell'abbonamento e l'unica priva di quella qualificazione.

Ne segue che una chiave revocata, tipicamente revocata perche compromessa, puo apparentemente ancora autorizzare addebiti sul saldo del nodo. La revoca esiste per fermare esattamente questo, e per il burn di abbonamento non lo dice.

## Evidence and provenance

Trovato da AGENT-006 durante SPEC-015, provando a scrivere con precisione cosa succede quando una chiave viene revocata. E la forma in cui questo progetto ha trovato meta dei propri difetti: qualcuno prova a dire una cosa con precisione e scopre che non e scritta da nessuna parte. Di conseguenza la guida pubblica non afferma che la revoca fermi la spesa.

Verificato dal Lead prima di promuoverlo a debito, e la verifica era necessaria: la riga 312 nomina anch'essa payer_node_id e sembrerebbe una regola generale che copre anche il burn. Non lo e. Appartiene a FundAppAuthorization, che e una transazione diversa con una propria struttura di autorizzazione; il burn di abbonamento ha la propria, SubscriptionBurnAuthorization, dichiarata alla riga 338, e la propria regola alla 347.

Non e stato stabilito se l'omissione sia deliberata. Il Lead non vede una ragione per cui un abbonamento dovrebbe accettare una chiave revocata mentre il finanziamento di un'app non lo fa, ma non ha svolto istruttoria e non lo afferma.

## Impact and scope boundary

Da stabilire, ed e il lavoro. La superficie e la spesa dal saldo di un nodo la cui chiave e stata revocata: se l'omissione e reale e non coperta altrove, chi ha rubato una chiave puo continuare a svuotare il saldo dopo che il legittimo proprietario ha ottenuto la revoca, che e il momento in cui il protocollo dovrebbe averlo fermato.

Va valutato separatamente cosa accada agli abbonamenti gia attivi al momento della revoca, perche e una questione diversa dall'aprirne di nuovi, e cosa comporti per la ricompensa al creatore, che conta gli abbonati attivi e li deriva dai burn finalizzati.

Severita high e non critical perche richiede una chiave gia compromessa, quindi un fallimento a monte, e perche nessuna rete esiste; ma e una regola di validita mancante su un percorso di spesa, ed e la classe piu economica da correggere adesso e piu cara dopo.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-015, su segnalazione di AGENT-006 che ha riportato invece di scrivere attorno al buco, come il dispatch le imponeva. Owner AGENT-007 e non AGENT-006 ne il Lead: e una questione di sicurezza sulla spesa e va valutata da chi ha mandato di attaccarla, ed e la stessa regola applicata a DEBT-013, DEBT-014 e DEBT-017.

## Resolution criteria

Una valutazione che stabilisca se l'omissione sia deliberata o accidentale, pronunciandosi separatamente sull'apertura di nuovi abbonamenti e sugli abbonamenti gia attivi al momento della revoca. Gli esiti ammissibili sono due: allineare la regola alle tre sorelle con la qualificazione enrolled, unrevoked, che e una modifica a una regola di validita e fa quindi scattare la gate di ADR-012 con la sua passata; oppure il rifiuto motivato, con la ragione dell'asimmetria scritta accanto alla regola invece che lasciata implicita, perche un'eccezione non scritta si legge come una dimenticanza.

Va inoltre stabilito nella stessa occasione se altre regole di autorizzazione del protocollo omettano la stessa qualificazione, perche il difetto e nell'asimmetria e non nella singola riga.

Da chiudere prima che una devnet accumuli saldi reali.

## Valutazione — 2026-08-26 (AGENT-007)

### Cosa è accertato, e come

**1. L'asimmetria è reale e il censimento è completo.** Le autorizzazioni a
chiave singola di `docs/protocol/ledger.md` sono quattro, e le ho contate invece
di ricordarle: riga 312 (`FundAppAuthorization`), riga 347
(`SubscriptionBurnAuthorization`), riga 398
(`ChallengeCommitmentAuthorization`), riga 871
(`ValidatorCandidacyAuthorization`). Tre dicono *«the enrolled, unrevoked»*; la
347 dice *«the key MUST derive `payer_node_id`»* e basta. Fuori da `ledger.md`
la stessa qualificazione compare in `docs/protocol/app-manifest.md:64-65` per la
chiave del publisher (*«a finalized, unrevoked enrollment certificate»*). Tutte
le altre autorizzazioni del protocollo — `mint`, burn `app_hosting`,
`revoke_identity`, `challenge_evidence` — sono a certificato di quorum e non
hanno la questione. **L'omissione è unica in tutto `docs/protocol/`**: non c'è
una seconda occorrenza, e questa è la risposta alla domanda che il debito pone
in coda alle *Resolution criteria*.

**2. `unrevoked` non è definito da nessuna parte per l'autorizzazione delle
transazioni, e il documento usa due formulazioni diverse.** È il fatto più
importante di questa valutazione e non era nel debito. La revoca ha
un'`effective_height` che `ledger.md:689` obbliga a stare almeno
`min_revocation_effective_delay_blocks` sopra il blocco che la propone, e
`ledger.md:731` dichiara che quel parametro **è scelto lungo** e nomina per
esteso *«l'intervallo durante il quale una revoca è finalizzata ma non ancora
efficace»*. In quell'intervallo, «revocata» ha due letture:

- *finalizzata* — è la lettura di `identity.md:614`, «no revocation exists at
  the receiver's finalized height», usata per accettare una connessione;
- *efficace* — è la lettura di `ledger.md:1027`, «enrolled and not revoked **as
  of** `candidacy_close_height(e)`», usata per l'eleggibilità.

Le tre regole sorelle non scelgono. **Ne segue che allineare la riga 347 alle
sorelle, e basta, produce una regola ancora indecidibile in senso proprio nella
sola finestra in cui serve**: quella fra la finalizzazione della revoca e la sua
efficacia, che il protocollo dichiara di volere lunga.

**3. La vittima non può revocare da sé.** `ledger.md:522` e `identity.md:636`:
la revoca richiede un certificato di quorum dei validatori, e
*«a node's self-signature alone cannot erase evidence»*. La latenza fra il furto
e la finalizzazione della revoca non è quindi un tempo tecnico ma un tempo di
governance, e nessun documento la limita.

**4. L'attacco non è distribuito nel tempo: sta in una transazione sola.** Ho
cercato un tetto per addebito e non c'è. `ledger.md:347-360` impone che
`amount_microtokens` uguagli l'addebito quotato in modo deterministico dal
`pricing_hash`, che *commits to the app manifest's publisher-declared
subscription and invocation pricing*, sul `service_period` dichiarato dal
pagatore stesso. Prezzo e periodo sono entrambi scelti da chi attacca, se
l'attaccante pubblica la propria app. **Un solo `burn` con periodo lungo e
prezzo alto può portare il saldo a zero.** La finestra temporale, che il Lead mi
chiede di stabilire, quindi *non è la grandezza da cui la perdita dipende*: la
perdita dipende dal saldo, e il tempo necessario è quello di una transazione.

**5. Gli abbonamenti già finalizzati continuano a contare dopo la revoca, e
nessuna regola può fermarli senza rendere la revoca retroattiva.**
`ledger.md:195-199`: per ogni epoca i validatori selezionano i burn
`app_subscription` *«whose half-open paid service period contains the entire
reward epoch»*. Un burn finalizzato prima della revoca continua quindi a contare
il nodo revocato come abbonato attivo per tutte le epoche che il periodo copre,
gonfiando `active_subscriber_count` e `counted_subscription_burn_microtokens`
del publisher. **Questo non è un difetto**: `identity.md:638` stabilisce che
*«revocation is not retroactive: historical signatures remain valid at heights
before the effective height»*, e il conteggio poggia su una transazione già
valida quando fu finalizzata. È la risposta alla seconda domanda del debito, e
va scritta *come conseguenza dichiarata*, non corretta.

**6. Il canale di recupero esiste ma è strutturalmente in perdita.** Ho
verificato se il drenaggio possa diventare furto: l'attaccante che pubblica
l'app riceve `publisher_reward`. `ledger.md:200-204` esclude il node ID del
publisher stesso e **trattiene un solo burn per pagatore per epoca** (il
`raw_32_bytes(subscription_burn_tx_id)` più basso); `ledger.md:228` impone
`amount * kd <= kn * counted_burn` con `kn < kd` stretto. Quindi al più
`kn/kd` di *un* addebito per epoca torna all'attaccante. **Il drenaggio è quasi
interamente distruzione di valore, non trasferimento.** Lo scrivo perché
l'ipotesi opposta — «la riga 347 riapre il trasferimento che il design ha
vietato» — è la prima che viene in mente ed è falsa: il divieto strutturale di
trasferimento regge.

### L'omissione è una svista, non una scelta

Il Lead mi chiede di stabilirlo e non di supporlo. **È una svista**, e le ragioni
sono tre, in ordine di forza.

1. **Non esiste il consenso passato che giustificherebbe l'eccezione.** L'ipotesi
   nobile — «l'addebito ricorrente sopravvive alla revoca perché è già autorizzato
   da un consenso precedente» — richiederebbe che in questo protocollo esista un
   oggetto *abbonamento* con una durata. Non esiste. `ledger.md:338-347` mostra
   che ogni addebito è **una transazione nuova, firmata ora, con un
   `account_nonce` nuovo**: non c'è mandato permanente da onorare, c'è una firma
   fresca da verificare. L'unica cosa che sopravvive legittimamente alla revoca è
   il punto 5 qui sopra, cioè un burn *già finalizzato* — e quello sopravvive per
   la regola generale di non retroattività, senza bisogno di alcuna eccezione
   scritta alla riga 347.
2. **La forma della frase è quella del taglia-e-incolla, non della deroga.** Le
   tre sorelle sono frasi autonome che aprono il paragrafo; la 347 è una
   subordinata dentro un periodo che comprime tre regole diverse
   (*«the key MUST derive `payer_node_id`; the signature is required and the node
   balance is debited»*). L'omissione sta nel punto in cui il testo era già
   denso, che è dove le omissioni stanno.
3. **Nessun documento la nomina.** Ho cercato una motivazione dell'asimmetria in
   `ledger.md`, `identity.md`, [ADR-006] e [ADR-014] e non c'è. Per la regola di
   forma di questo progetto, un'eccezione non scritta si legge come una
   dimenticanza — e qui lo è.

**Ne segue che la chiusura è un allineamento, e va detto che è un allineamento**,
come il dispatch chiede esplicitamente. Non c'è una regola *diversa* da scrivere.

### Cosa ottiene l'attaccante, in ordine di gravità

Il Lead chiede se l'esito peggiore sia l'addebito indebito, il drenaggio, o
qualcos'altro. **È qualcos'altro, e non è nell'elenco: è una divergenza di
consenso.**

1. **Fork.** Due implementazioni conformi divergono sulla validità di un blocco.
   Chi legge la riga 347 alla lettera accetta il burn di una chiave revocata; chi
   generalizza dalla 312 lo rifiuta. Il burn è una transazione dentro un blocco,
   quindi il disaccordo non è su una politica locale ma sulla **validità del
   blocco**, e la conseguenza è una partizione della catena. È la famiglia 4 di
   `recurring-defects.md` nella sua forma peggiore: una clausola che nessun
   oracolo esercita e su cui due letture diligenti danno verdetto opposto. Il
   costo per innescarla è quello di una transazione.
2. **Drenaggio completo del saldo, in una transazione, senza limite superiore
   nel tempo.** Sotto il testo attuale non esiste alcun istante dopo il quale la
   chiave revocata smetta di poter spendere: la finestra è **illimitata**, non
   lunga. È il punto 4 accertato sopra unito al fatto che la 347 non nomina la
   revoca affatto.
3. **Recupero parziale in perdita** verso un'app dell'attaccante, limitato a
   `kn/kd` di un addebito per epoca (punto 6).
4. **Addebito indebito** nel senso ordinario, cioè il caso in cui l'attaccante
   si limiti a sottoscrivere app di terzi. È il meno grave dei quattro.

### Cosa resta ignoto

- **Se esista una regola di unicità per `(payer_node_id, app_id,
  service_period)`.** Non l'ho trovata, e la sua assenza permetterebbe addebiti
  ripetuti sullo stesso periodo. **Non affermo che manchi**: ho letto la sezione
  burn e la sezione mint, non l'intero documento con quella domanda, e una
  regola di unicità potrebbe stare nella sezione di esecuzione delle
  transazioni. È una domanda separata da questo debito e va posta a chi lo
  chiuderà, non risolta qui per comodità.
- **La latenza reale fra furto, rilevazione e quorum di revoca.** Non è
  stabilita da alcun documento e non è stabilibile prima che esista una rete.
- **Il valore di `min_revocation_effective_delay_blocks`.** Non è tarato. La
  taratura non cambia questa valutazione, perché il punto 4 la rende
  irrilevante per la perdita massima.

### Esiti ammissibili, in ordine di forza e col loro costo

**(A) Allineamento più definizione di `unrevoked` a un'altezza, su tutte e
quattro le regole.** La riga 347 acquista *«the enrolled, unrevoked»*, e il
documento dice una volta sola, in un punto unico, rispetto a quale altezza la
qualificazione si valuta — e sceglie **la finalizzazione**, non l'efficacia,
perché per l'autorizzazione di una spesa il pericolo sta verso l'alto sulla
durata dell'esposizione e la scelta sicura è quella che chiude prima.
*Costo:* è una regola di validità nuova su quattro percorsi invece che su uno,
quindi una passata [ADR-012] più larga; tocca le fixture canoniche dei quattro
oggetti; richiede una fixture di frontiera all'altezza esatta e la prova in
negativo. **È l'esito che raccomando**, perché è il solo che chiude la finestra
che il protocollo dichiara di volere lunga.

**(B) Allineamento secco della sola riga 347.** *Costo:* minimo, una passata
[ADR-012] su una regola. **Ma lascia aperta la finestra fra finalizzazione ed
efficacia su tutte e quattro le regole**, cioè chiude l'asimmetria e non chiude
il buco. Ammissibile solo se dichiarato come tale, con il residuo scritto accanto
alla regola e un debito nuovo aperto nella stessa passata. Se viene scelto senza
quella dichiarazione, è una chiusura falsa.

**(C) Rifiuto motivato, con l'asimmetria scritta accanto alla regola.** *Costo:*
nullo in ingegneria, alto in credibilità. **Lo respingo**: l'istruttoria del
paragrafo precedente non ha trovato la ragione che lo sosterrebbe, e scrivere una
motivazione che non si ha è la forma dell'«impossibilità dichiarata a torto»
censita nella famiglia 2.

### I rimedi apparenti che non rimediano

Nominati perché non vengano adottati.

1. **«La riga 312 è la regola generale e copre anche il burn.»** Falso, e il Lead
   lo aveva già verificato: la 312 appartiene a `FundAppAuthorization`, una
   struttura di autorizzazione diversa dichiarata alla riga 306, mentre il burn
   di abbonamento ha la propria alla 338. La verifica va conservata scritta,
   perché è l'errore che chiunque rilegga queste due righe rifarà.
2. **«Tanto un nodo revocato viene disconnesso, quindi non può inviare la
   transazione.»** È il rimedio apparente più insidioso, perché è *vero al
   livello sbagliato*. `identity.md:625-631` obbliga davvero a chiudere le
   connessioni verso un peer revocato. Ma una transazione è un oggetto firmato
   che viaggia in gossip: **l'attaccante non deve consegnarla lui**. La consegna
   a un terzo qualsiasi, o a un relay non conforme, la immette in rete, e da
   quel momento la sua validità la decide `ledger.md:347` e nient'altro. È
   esattamente la forma già censita nella famiglia 3 applicata alle gate:
   *l'irraggiungibilità di un peer non è l'invalidità di un oggetto che ha
   firmato*.
3. **«Tarare corto `min_revocation_effective_delay_blocks`.»** Non rimedia per
   due ragioni indipendenti: la perdita massima si ottiene prima della
   finalizzazione (punto 4), e `ledger.md:731-735` dichiara che accorciare quel
   parametro **aggrava** il rischio di stallo e va scelto insieme a
   `max_weak_subjectivity_age_ms`. Sarebbe un valore scelto bene al posto di una
   proprietà — [ADR-010].
4. **«Rendere la revoca retroattiva sugli abbonamenti attivi.»** Non rimedia e
   rompe: contraddice `identity.md:638`, e trasformerebbe una `revoke_identity`
   in uno strumento capace di riscrivere il `publisher_reward` di epoche già
   finalizzate, cioè in una leva di `T-07` sull'emissione. Il punto 5 va
   *dichiarato*, non chiuso.
5. **«Il cap creator-share protegge già il saldo.»** No: il cap protegge
   l'**emissione** dal ciclo di stampa, non il **saldo** della vittima dal
   burn. Sono due asset diversi (`A-02` e `A-01`), e confonderli è la stessa
   confusione fra falsificazione e perdita che [DEBT-018] censisce sulla
   matrice.

### Riclassificazione

**Severità confermata `high`, ma la motivazione scritta nel debito va
sostituita.** Il debito la giustifica con «richiede una chiave già compromessa,
quindi un fallimento a monte». Quella motivazione è debole e sarebbe la ragione
per declassare. La ragione per cui `high` regge è un'altra e non richiede alcuna
chiave rubata: **la regola è ambigua fra due letture conformi che danno verdetto
opposto sulla validità di un blocco** (esito 1 sopra). Il difetto è di
interoperabilità del consenso prima ancora che di sicurezza della spesa, e su
quel piano non c'è alcun «fallimento a monte» a mitigarlo. La condizione di
chiusura «prima che una devnet accumuli saldi reali» va corretta in **prima che
esista una seconda implementazione**, che è la condizione che morde per prima.

### Raccomandazione di raggruppamento

**Non raggruppare DEBT-022 con DEBT-017.** Documenti diversi, famiglie di regola
diverse, e soprattutto la passata [ADR-012] è per regola di validità: metterle
insieme non ne risparmia una.

**Spec propria, con AGENT-002** (ledger, consenso, token economy) come
implementatore e AGENT-007 in review adversariale. La ragione della scelta
dell'agente è che l'esito (A) tocca quattro regole di autorizzazione e la
semantica di un'altezza, cioè materia di `ledger.md`, non di sicurezza
applicativa.

**Con una dipendenza di sequenza su DEBT-018, e non è ordine ma sostanza:**
l'attacco qui valutato — chi detiene la chiave di identità di un nodo dopo la
revoca — **non ha oggi alcuna cella nella matrice del threat model né alcun
attore che lo rappresenti**, per la stessa lacuna che [DEBT-018] registra su
TM-37. Chiudere DEBT-022 prima significa scrivere una regola la cui ragione non
ha posto dove essere registrata. DEBT-018 va chiuso prima, e nella sua chiusura
va previsto lo scenario che questa valutazione descrive.

## Resolution evidence

