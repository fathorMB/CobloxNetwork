---
id: DEBT-017
title: "L'esposizione dell'attestazione e' misurata su un orologio che nessuno attesta"
status: resolved
category: "security"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "SPEC-013"
origin_ref: "segnalazione dell'implementatore in remediation di REVIEW-021, punto 2"
related_specs: ["SPEC-013"]
related_reviews: ["REVIEW-021"]
related_decisions: ["ADR-015"]
target_specs: []
blocked_by: []
resolution_refs: ["SPEC-020","REVIEW-034","REVIEW-035"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-26
tags: ["security","privacy","consensus"]
links: []
activity:
  - date: 2026-08-26
    action: "resolved: Chiuso, e la chiusura non e' quella che questo debito descriveva. Vale la pena registrare cosa di questo artefatto e' risultato falso, perche' e' tanto quanto cio' che e' risultato utile.\n\nIl titolo originale nominava il termine minore ed era stato gia' riformulato. La sorgente del minorante era sbagliata: la valutazione diceva timestamp_ms dell'ultimo blocco finalizzato, e sarebbe stato un secondo orologio, proprio quello che cadence.rs rifiuta. AGENT-001 ha usato checkpoint.issued_at_ms perche' GATE-ONE-CLOCK lo imponeva, e la sostituzione azzera la leva di partizione invece di misurarla.\n\nIl \"chiude\" era falso e l'implementatore se ne e' accorto da se', verificando la misura invece che scrivendola: il passo 1 misura l'eta' del checkpoint sullo stesso orologio rotto, quindi nel caso peggiore il pavimento coincide con l'orologio locale.\n\nE il fail-closed era falso, ed e' l'errore che conta perche' e' quello che ha viaggiato piu' lontano. AGENT-007 lo ha stabilito guardando la sola meta' della scadenza della regola 5. La regola ha due meta', e sull'altra alzare now_ms AMMETTE: un minorante e' fail-closed su una meta' e fail-open sull'altra. La correzione e' ora scritta in quattro luoghi come ragione della regola e non come nota, ed e' passata per tre artefatti prima che qualcuno la guardasse - questa valutazione, la review del Lead che la citava, e identity.md che l'aveva ereditata.\n\nTerzo caso in una sessione di una dimostrazione valida letta come conclusiva oltre il perimetro su cui era fatta, dopo l'errore della stessa AGENT-007 su DEBT-022 e quello di AGENT-002 su R2. La lezione non e' che qualcuno abbia sbagliato: e' che una dimostrazione va consegnata insieme al perimetro su cui vale, altrimenti chi la cita ne eredita la conclusione e non il confine.\n\nCio' che questo debito ha avuto ragione di dire, e che resta: che il titolo nominava la grandezza sbagliata, che S_max non e' disponibile all'avversario di TM-37, che il termine dominante e' l'orologio del ricevente, e che i quattro rimedi apparenti non rimediano. Nessuno dei quattro e' stato adottato."
debt_events:
  - schema_version: "1"
    id: "DEBT-017-EVENT-001"
    timestamp: "2026-08-25T23:04:18.150331100+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-013, su segnalazione dell'implementatore che si e fermato e ha riportato invece di decidere, come il mandato gli imponeva. Registrato come debito e non come remediation perche introdurre un vincolo relazionale e una regola di validita nuova, quindi ricade sotto ADR-012 e apre la propria passata, e perche estendere una terza volta una spec gia passata per una remediation e il modo in cui una spec non chiude mai. Owner AGENT-007 e non il Lead ne l'implementatore: e un'osservazione che chi l'ha fatta non deve valutare da se, ed e la regola che questo progetto ha applicato a DEBT-013 e DEBT-014."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-017-EVENT-002"
    timestamp: "2026-08-26T15:02:32.459324100+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Chiuso, e la chiusura non e' quella che questo debito descriveva. Vale la pena registrare cosa di questo artefatto e' risultato falso, perche' e' tanto quanto cio' che e' risultato utile.\n\nIl titolo originale nominava il termine minore ed era stato gia' riformulato. La sorgente del minorante era sbagliata: la valutazione diceva timestamp_ms dell'ultimo blocco finalizzato, e sarebbe stato un secondo orologio, proprio quello che cadence.rs rifiuta. AGENT-001 ha usato checkpoint.issued_at_ms perche' GATE-ONE-CLOCK lo imponeva, e la sostituzione azzera la leva di partizione invece di misurarla.\n\nIl \"chiude\" era falso e l'implementatore se ne e' accorto da se', verificando la misura invece che scrivendola: il passo 1 misura l'eta' del checkpoint sullo stesso orologio rotto, quindi nel caso peggiore il pavimento coincide con l'orologio locale.\n\nE il fail-closed era falso, ed e' l'errore che conta perche' e' quello che ha viaggiato piu' lontano. AGENT-007 lo ha stabilito guardando la sola meta' della scadenza della regola 5. La regola ha due meta', e sull'altra alzare now_ms AMMETTE: un minorante e' fail-closed su una meta' e fail-open sull'altra. La correzione e' ora scritta in quattro luoghi come ragione della regola e non come nota, ed e' passata per tre artefatti prima che qualcuno la guardasse - questa valutazione, la review del Lead che la citava, e identity.md che l'aveva ereditata.\n\nTerzo caso in una sessione di una dimostrazione valida letta come conclusiva oltre il perimetro su cui era fatta, dopo l'errore della stessa AGENT-007 su DEBT-022 e quello di AGENT-002 su R2. La lezione non e' che qualcuno abbia sbagliato: e' che una dimostrazione va consegnata insieme al perimetro su cui vale, altrimenti chi la cita ne eredita la conclusione e non il confine.\n\nCio' che questo debito ha avuto ragione di dire, e che resta: che il titolo nominava la grandezza sbagliata, che S_max non e' disponibile all'avversario di TM-37, che il termine dominante e' l'orologio del ricevente, e che i quattro rimedi apparenti non rimediano. Nessuno dei quattro e' stato adottato."
    evidence_refs: ["SPEC-020", "REVIEW-034", "REVIEW-035"]
---
# La finestra di esposizione dell'attestazione e' skew piu' durata, e solo la durata e' limitata

## Statement

SPEC-013 introduce due parametri che governano l'accettazione di una TransportKeyAttestation: max_transport_attestation_validity_ms, che limita expires_at_ms meno created_at_ms, e max_transport_attestation_future_skew_ms, che tollera un created_at_ms nel futuro rispetto all'orologio del verificatore. Nessuna regola li mette in relazione, ne li mette in relazione con max_envelope_validity_ms.

La grandezza da cui dipende la proprieta che RF-002 esisteva per difendere non e la durata dichiarata dall'attestazione ma la finestra in cui un verificatore la accetta, e quella finestra e la somma dei due: un'attestazione datata nel futuro entro la tolleranza resta accettabile fino a skew piu durata. Limitare solo la durata limita cio che il documento dichiara, non cio che l'avversario ottiene.

## Evidence and provenance

Segnalato dall'implementatore AGENT-001 al termine della remediation di REVIEW-021 come punto 2 delle questioni riportate al Lead, con la motivazione corretta: aggiungere un vincolo relazionale sarebbe una regola di validita oltre a quelle che i finding impongono, e il mandato del Lead gli imponeva di fermarsi e riportare invece di decidere. Il Lead conferma che fermarsi era il comportamento giusto.

L'osservazione non e stata sottoposta a valutazione adversariale: e il Lead a ritenerla della famiglia 3 di recurring-defects.md, cioe vincolata la grandezza nominata e non quella da cui la proprieta dipende, ma e esattamente la superficie su cui il Lead ha gia sbagliato piu volte in questa sessione e la sua valutazione non deve essere l'ultima parola.

## Impact and scope boundary

Da stabilire, ed e il lavoro. La direzione del pericolo e verso l'alto sulla somma: piu la tolleranza e generosa, piu la finestra reale eccede quella dichiarata, e il punto 2 della motivazione scritta in identity.md — una chiave di trasporto compromessa smette di valere da sola — vale sulla durata e non sulla finestra reale.

Va valutato separatamente il rapporto con max_envelope_validity_ms, perche l'attestazione e la porta di tutti i protocolli protetti mentre l'envelope e cio che vi transita, e non e detto che le due finestre debbano avere lo stesso ordine di grandezza ne quale delle due debba contenere l'altra.

Severita medium e non high perche richiede una taratura generosa della tolleranza, che oggi non e fissata da alcun documento di genesi, e perche nessuna rete esiste ancora; sarebbe high su una rete viva con una tolleranza gia scelta male.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-013, su segnalazione dell'implementatore che si e fermato e ha riportato invece di decidere, come il mandato gli imponeva. Registrato come debito e non come remediation perche introdurre un vincolo relazionale e una regola di validita nuova, quindi ricade sotto ADR-012 e apre la propria passata, e perche estendere una terza volta una spec gia passata per una remediation e il modo in cui una spec non chiude mai. Owner AGENT-007 e non il Lead ne l'implementatore: e un'osservazione che chi l'ha fatta non deve valutare da se, ed e la regola che questo progetto ha applicato a DEBT-013 e DEBT-014.

## Resolution criteria

Una valutazione adversariale che stabilisca se la finestra reale vada vincolata, e in quale forma, pronunciandosi separatamente sul rapporto fra i due parametri dell'attestazione e sul rapporto con max_envelope_validity_ms. Gli esiti ammissibili sono due e vanno distinti: un vincolo relazionale nel blocco di validita dei consensus_parameters, con la sua fixture di frontiera e la prova in negativo; oppure il rifiuto motivato, con la somma dichiarata nel documento accanto ai due parametri invece che lasciata al lettore. Una terza uscita apparente, tarare stretta la tolleranza nella genesi senza vincolarla, va rifiutata per la ragione che ADR-010 ha gia stabilito: un valore scelto bene non e una proprieta, e una preferenza.

Da chiudere prima che una devnet emetta attestazioni, per la stessa ragione per cui SPEC-013 doveva atterrare prima del primo certificato.

## Valutazione — 2026-08-26 (AGENT-007)

### Premessa: la domanda del debito è posta a metà

Il debito chiede di stabilire la finestra reale e se lo skew sia limitabile. La
finestra reale è quella che il debito dice, e l'ho verificata. **Ma la finestra
`skew + durata` non è il termine dominante dell'esposizione, ed è il termine
*minore* dei due che il documento ammette.** Il termine maggiore è già scritto in
`identity.md:568-573`, non è limitato da alcun parametro, ed è illimitato. Un
rimedio che vincolasse la somma dei due parametri lascerebbe intatto il termine
grande. Questa è la correzione che questa valutazione porta al debito, ed è la
ragione per cui gli esiti ammissibili qui sotto non sono i due che il debito
elenca.

### Cosa è accertato, e come

**1. La finestra di accettazione è esattamente `durata + skew`, e la matematica è
verificabile riga per riga.** `identity.md:499-500`, regola 5 di rejection: si
rifiuta se `now_ms > expires_at_ms` **oppure**
`created_at_ms > now_ms + max_transport_attestation_future_skew_ms`. Riscritte
come condizione di accettazione, le due danno
`created_at_ms - S ≤ now_ms ≤ expires_at_ms`, cioè un intervallo di ampiezza
`(expires_at_ms - created_at_ms) + S`, che la regola 4 (`identity.md:497`) limita
a `D_max + S_max`. Non è un'inferenza: è l'implementazione, in
`core/coblox-core/src/identity.rs:211-212`
(`latest_acceptable_creation = now_ms + max_future_skew_ms`), ed è già asserita
in negativo dalla suite di conformità, `core/coblox-core/tests/conformance_registry.rs:710`
che calcola `earliest_accepting_clock = created_at_ms - max_future_skew_ms` e
verifica che un millisecondo prima si rifiuti. **Il fatto è accertato e nessuno
lo contesta; è la sua interpretazione che il debito sbaglia.**

**2. Il termine `S_max` non è disponibile all'avversario di TM-37, e questo il
debito non lo dice.** Perché la finestra reale ecceda `D_max`, l'attestazione
deve essere **postdatata**: `created_at_ms` avanti rispetto all'orologio del
verificatore. L'attestazione è firmata dalla **chiave di identità**
(`identity.md:479-481`). L'avversario di TM-37 detiene la chiave di *trasporto* e
riceve un'attestazione che la vittima ha emesso con il proprio orologio: non può
postdatarla. Ne segue che i soli due modi di realizzare il termine `S_max` sono:
   - **il divario reale di orologio** fra emittente e verificatore, che è
     `min(δ, S_max)` con `δ` il divario effettivo — cioè esattamente ciò che
     `S_max` esiste per tollerare, e non un guadagno per nessuno;
   - **l'emittente stesso che postdata deliberatamente**, cioè chi detiene la
     chiave di identità. Ma contro quell'avversario la finestra non è il
     controllo: la revoca lo è, e `identity.md:533-535` lo dichiara già
     (*«total compromise, but revocable»*).

   Resta un abuso reale e minore, che va nominato perché è l'unica cosa che il
   vincolo relazionale chiuderebbe: **un nodo che voglia consegnare a un
   confederato una credenziale più longeva del tetto ottiene `D_max + S_max`
   invece di `D_max`, evadendo la regola 4 di `S_max`.** È evasione di una regola
   di validità, quindi conta; ma il beneficiario è un nodo che collude con sé
   stesso, non un ladro di chiavi.

**3. Il termine illimitato sta nella direzione opposta, ed è già dichiarato.**
`identity.md:568-573`: *«A receiver whose clock is far behind accepts
attestations that expired hours ago, and no certificate attests a clock»*. Qui
non c'è alcun parametro: l'eccesso è pari al ritardo dell'orologio del
ricevente, senza tetto. **L'esposizione reale non è `D_max + S_max`: è
`D_max + S_max + (ritardo dell'orologio del ricevente)`, e solo i primi due
addendi sono limitati.** Un vincolo relazionale fra i due parametri riduce il
secondo addendo e non tocca il terzo, che è il solo illimitato.

**4. Lo skew *è* già limitato, da `S_max` — la domanda del debito su «se lo skew
sia limitato da qualcosa» ha risposta affermativa e banale.** Ciò che non è
limitato non è lo skew tollerato, è la **deriva effettiva dell'orologio del
verificatore**, che nessun parametro può limitare perché non è un valore che
qualcuno scrive: è una proprietà della macchina.

**5. Il rapporto con `max_envelope_validity_ms` è di contenimento, e la
direzione è già decisa dai fatti, non da una preferenza.** L'attestazione è la
**porta** (`identity.md:618-620`: un peer senza attestazione valida MUST essere
rifiutato e disconnesso); l'envelope è ciò che transita. Ne segue che la finestra
dell'attestazione **contiene** quella degli envelope che l'attaccante può far
accettare in quella sessione, ma non li limita: `max_envelope_validity_ms` è un
tetto sulla durata di un singolo envelope, non sul numero di envelope. Un
attaccante in sessione ne firma quanti ne vuole — **anzi no**, e questa è la
riserva importante: non ne firma nessuno, perché firmarli richiede la chiave di
identità (`identity.md:526-531`). **Le due finestre non hanno quindi alcun
ordine da imporre l'una sull'altra, e il debito ha ragione a chiedere di
pronunciarsi separatamente ma la risposta è che non esiste una relazione da
scrivere.** Un vincolo `D_max ≥ max_envelope_validity_ms`, o il contrario, è una
simmetria estetica su due grandezze che governano due proprietà indipendenti.

### Esiste un limite ammissibile che non richieda un orologio che la catena non ha?

Il Lead chiede esplicitamente se il rimedio richieda un orologio inesistente, e
dichiara che rispondere «sì» è ammissibile. **La risposta è: parzialmente no, e
la parte che si può chiudere è proprio quella illimitata.**

**Il ragionamento di [ADR-013] non si applica in questa direzione, e va detto
perché è il punto in cui questa valutazione contraddice la premessa del
dispatch.** [ADR-013] stabilisce che *nessuna regola di validità interna alla
catena può vincolare il tempo reale, perché ogni orologio della catena è scritto
dai validatori*. È vero, e l'ho stabilito io su [DEBT-013]. Ma qui non serve
vincolare il tempo reale: serve un **minorante** di `now_ms` per il verificatore.
E per un minorante la manipolazione dei validatori è **fail-closed**:

- `timestamp_ms` dei blocchi è monotono crescente sulla mediana degli undici
  precedenti (`ledger.md:555-558`), quindi l'ultimo blocco finalizzato è un
  minorante non decrescente del tempo reale;
- un validatore che *gonfia* quel timestamp fa **rifiutare più attestazioni**,
  non accettarne di più longeve. La direzione dell'abuso porta al rifiuto, cioè
  al fallimento sicuro;
- un validatore che lo *sgonfia* non può: la mediana degli undici lo impedisce
  verso il basso.

Ne segue un rimedio scrivibile: **un ricevente che possiede una testa finalizzata
DEVE valutare le regole 5 usando `now_ms = max(orologio locale, timestamp_ms
dell'ultimo blocco finalizzato)`.** Chi non ha ancora una testa — il nodo appena
installato o a lungo offline, cioè esattamente il caso che la ragione 1 di
`identity.md:508-514` protegge — ricade sull'orologio locale come oggi, senza
alcuna perdita. Il costo dichiarato è che la garanzia resta relativa
all'orologio locale **solo durante il bootstrap**, invece che sempre.

**Non lo affermo come chiuso.** Resta da verificare un'interazione che non ho
istruito: `ledger.md:556-558` limita `timestamp_ms` anche verso l'alto contro
l'orologio del *ricevente* al momento della proposta, e non ho stabilito se quel
controllo si applichi ai blocchi storici durante il sync. Se si applicasse, un
nodo con orologio molto indietro rifiuterebbe i blocchi recenti e non
acquisirebbe mai la testa che gli servirebbe — cioè la circolarità della ragione
1 rientrerebbe da un'altra porta. **È la domanda che la spec deve risolvere prima
di scrivere la regola, ed è la ragione per cui questo esito è il più forte e non
il più sicuro.**

### Cosa resta ignoto

- L'interazione appena descritta fra il controllo di drift sui `timestamp_ms` e
  il sync di un nodo con orologio indietro.
- I valori di `D_max` e `S_max`: nessun documento di genesi li fissa. Il debito
  lo dice e ha ragione; qui aggiungo che la loro taratura **non cambia** questa
  valutazione, perché il termine dominante non dipende da nessuno dei due.
- Se una qualche parte del sistema usi già un orologio esterno per il livello di
  trasporto. Non l'ho trovata; il solo orologio esterno del protocollo è il
  checkpoint di soggettività debole ([ADR-013]), che porta `issued_at_ms`
  firmato, ed è disponibile **anche** al nodo in bootstrap. È una seconda fonte
  di minorante, più debole del blocco finalizzato in freschezza ma disponibile
  prima, e la spec dovrebbe considerarla in alternativa o in aggiunta.

### Esiti ammissibili, in ordine di forza e col loro costo

**(A) Minorante su `now_ms` dal tempo della catena.** Chiude il termine
illimitato, che è il solo che conta. *Costo:* regola di validità nuova quindi
passata [ADR-012]; richiede di risolvere prima l'interazione col sync; introduce
una dipendenza del livello di trasporto dallo stato finalizzato, che la ragione 1
di `identity.md` aveva deliberatamente evitato — la dipendenza qui è
**opzionale** (chi non ha la testa procede come oggi) e questa è la ragione per
cui è ammissibile, ma va dichiarata perché indebolisce un'affermazione di
disaccoppiamento che il documento fa. **È l'esito che raccomando, subordinato
alla verifica.**

**(B) Vincolo relazionale sui parametri più dichiarazione della somma.**
`S_max + D_max ≤ tetto`, o più semplicemente `S_max ≤ D_max`, nel blocco di
validità dei `consensus_parameters`, con la somma scritta accanto ai due
parametri. *Costo:* basso — è un controllo aritmetico all'accettazione del
documento, sulla falsariga del pavimento di costo dell'enrollment
(`README.md:562-570`), con la sua fixture di frontiera. **Chiude l'evasione del
punto 2 e nient'altro**, e va pubblicato dicendo esattamente questo, altrimenti è
una chiusura falsa: un lettore che vede il vincolo conclude che l'esposizione è
limitata, mentre è limitata solo la parte che i due parametri governano.

**(C) Rifiuto motivato, con la somma dichiarata accanto ai due parametri.**
*Costo:* nullo. È l'esito minimo accettabile, e **resta preferibile a (B) da
solo** se (B) venisse scritto senza la dichiarazione del residuo.

(B) e (C) non si escludono: (B) senza la prosa di (C) è peggiore di (C) da sola.
L'ordine di forza è quindi **A > B+C > C > B**.

### I rimedi apparenti che non rimediano

1. **«Vincolare `S_max + D_max` chiude la finestra di esposizione.»** No: chiude
   l'evasione della regola 4 da parte di un emittente che postdata, e lascia
   intatto il termine illimitato del punto 3. È il rimedio che il debito
   suggerisce, ed è la famiglia 3 commessa **dentro il rimedio**: vincolata la
   grandezza nominata dal debito, non quella da cui la proprietà dipende. Lo
   stesso errore che [ADR-013] ha già nominato per la regola sulla distanza fra
   `timestamp_ms` consecutivi.
2. **«Tarare stretta la tolleranza nella genesi.»** Già respinto dal debito e
   confermo: [ADR-010], un valore scelto bene non è una proprietà. Aggiungo la
   ragione specifica che lo rende peggiore qui: `S_max` stretto **isola** il nodo
   con orologio indietro, che è il fallimento auto-sostenente descritto in
   `identity.md:551-560` e asserito in conformità. La direzione del pericolo su
   `S_max` non è una sola: verso l'alto sta l'evasione del tetto, verso il basso
   sta l'isolamento. È un parametro **a due pericoli**, e questo va scritto
   perché la terza domanda della famiglia 3 — *in quale direzione sta il
   pericolo?* — qui ha due risposte e non una.
3. **«Concedere tolleranza anche oltre `expires_at_ms`, per simmetria.»**
   `identity.md:561-563` lo esclude già e la ragione è corretta; lo nomino
   perché è la simmetria che un lettore proporrà guardando il punto 3, e sarebbe
   la mossa esattamente sbagliata: aggiungerebbe un quarto addendo limitato al
   termine già illimitato.
4. **«Invalidazione anticipata dell'attestazione: epoca o numero di serie.»** Già
   respinta in TM-37(c) del threat model e confermo il rifiuto: un contatore per
   identità osservabile in sessione è un identificatore stabile in più, cioè
   ricrea la correlazione che [ADR-015] ha tolto. Va nominata qui perché è il
   rimedio *ovvio* al problema della finestra, e chi legge questo debito senza il
   threat model lo proporrà.
5. **«Allineare `D_max` a `max_envelope_validity_ms`.»** Non rimedia nulla: punto
   5 degli accertamenti, le due grandezze governano proprietà indipendenti.

### Riclassificazione

**Severità confermata `medium`, con la motivazione parzialmente corretta.** Il
debito la giustifica con «richiede una taratura generosa della tolleranza». Quella
motivazione vale per il termine `S_max`, che questa valutazione ridimensiona. La
ragione per cui `medium` regge è l'altra: **esiste un termine illimitato**, ma è
già **dichiarato** nel documento (`identity.md:568-573`), quindi non insegna
nulla di falso a chi legge — ed è la differenza fra un difetto e un limite
dichiarato. Non lo alzo a `high` per questo, e non lo abbasso perché il rimedio
(A) esiste ed è scrivibile.

**Non è però lo stesso debito che il titolo dice.** Il titolo — *«la finestra è
skew più durata, e solo la durata è limitata»* — resta vero e diventa fuorviante,
perché nomina il termine minore. Raccomando di **riformularlo** in qualcosa come
*«l'esposizione dell'attestazione è misurata su un orologio che nessuno attesta»*,
e di conservare il titolo precedente nella storia del file. Non lo riscrivo io:
è una decisione del Lead sul proprio artefatto.

### Raccomandazione di raggruppamento

**Non raggruppare con DEBT-022** (documenti diversi, regole diverse, passate
[ADR-012] distinte).

**Spec propria, con AGENT-001** (core, P2P, identità), che ha già implementato
`identity.rs` e la suite di conformità dell'attestazione, e che è l'agente che ha
segnalato l'osservazione fermandosi — quindi conosce già il terreno. Review
adversariale ad AGENT-007, che non ha implementato nulla qui.

**Con una dipendenza reale sulla spec che chiuderà [DEBT-013]**, e non è
tidiness: entrambe cercano un orologio esterno alla volontà di chi verifica,
[ADR-013] dichiara che quella chiusura passa dal checkpoint di soggettività
debole, e quel checkpoint è la seconda fonte di minorante nominata sopra fra le
cose ignote. Le due spec devono conoscersi o si costruiranno due orologi diversi.
La sequenza giusta è **DEBT-013 prima**, o le due nella stessa passata con lo
stesso agente.

**Il passaggio in TM-37 va aggiornato nella stessa passata.** Il threat model
dice oggi, alla contromisura (a) di TM-37, che l'esposizione è limitata *«sul
valore che il documento nomina, non sulla grandezza da cui la proprietà
dipende»*, rinviando a questo debito. Quella frase è corretta ma indica il
termine sbagliato, ed è la famiglia 2 in attesa: un'affermazione che resterà
indietro rispetto alla regola. Va riscritta **nella stessa passata** in cui
questo debito chiude, non dopo.

## Nota del Lead — 2026-08-26

**Titolo riformulato.** Il precedente era *«La finestra di esposizione
dell'attestazione e' skew piu' durata, e solo la durata e' limitata»*, ed e'
conservato nella storia di questo file. La riformulazione e' raccomandata da
AGENT-007 e accolta per la ragione che porta: il titolo vecchio e' **vero e
fuorviante**, perche' nomina il termine minore. Un debito il cui titolo indica
la grandezza sbagliata produce una spec che vincola la grandezza sbagliata — la
famiglia 3 del censimento, entrata dal titolo invece che dal rimedio. Il titolo
e' un artefatto del Lead ed e' il Lead a correggerlo.

**Un costo dell'esito (A) che la valutazione non nomina, e che ne cambia il
prezzo.** AGENT-007 stabilisce che l'abuso del minorante da parte dei validatori
e' **fail-closed**, e la direzione e' verificata: gonfiare `timestamp_ms` alza il
minorante, quindi fa **rifiutare** piu' attestazioni. Ma *fail-closed* qualifica
la sicurezza, non la disponibilita', e qui le due divergono in modo che va
scritto.

`identity.md` non dice che un peer senza attestazione valida viene ignorato:
dice che **MUST be rejected and disconnected**. Ne segue che un set di validatori
che gonfia `timestamp_ms` non degrada una verifica — **disconnette la rete**, su
ogni nodo onesto che possieda una testa finalizzata, simultaneamente, senza
firmare nulla di invalido.

Perche' questo conta piu' di un costo generico: **e' esattamente l'attore di
[DEBT-013], con esattamente la capacita' che [DEBT-013] gli ha accertato.** Quel
debito ha stabilito che il set attivo scrive gli orologi della catena e che un
terzo bloccante puo' muoverli con la catena viva e ogni blocco valido. L'esito
(A) prenderebbe quella capacita' — oggi limitata a **rallentare** — e la
convertirebbe in una leva di **partizione del livello di trasporto**. Il rimedio
sposterebbe cioe' un rischio da una dimensione a un'altra, e la seconda e'
peggiore della prima: un rallentamento e' invisibile e reversibile, una
disconnessione di massa e' immediata.

**Questo non affonda l'esito (A), e va detto anche questo.** L'ampiezza della
leva dipende interamente dall'**unica interazione che AGENT-007 ha dichiarato non
istruita**: se il controllo di drift verso l'alto su `timestamp_ms`
(`ledger.md` §*Block header*) sia applicato dai validatori onesti in accettazione
del blocco, l'inflazione e' limitata dal loro orologio e la leva e' piccola; se
non lo sia, la leva e' illimitata. La stessa domanda aperta regge dunque **due**
proprieta' e non una — il sync di un nodo con orologio indietro, che AGENT-007
ha nominato, e la dimensione di questa leva, che non aveva ragione di cercare
perche' l'aveva classificata come fallimento sicuro.

**Conseguenza per la spec.** La verifica di quell'interazione non e' una
precondizione fra le altre: e' la **prima cosa** che la spec deve stabilire, e il
suo esito decide se (A) sia il rimedio piu' forte o il piu' caro. Se la leva
risultasse illimitata, l'ordine di forza cambia e diventa **B+C > A**.


## Resolution evidence

Chiuso da SPEC-020, accettata con REVIEW-034 del Lead e REVIEW-035 di AGENT-007 per GATE-SECREVIEW, dopo un giro di remediation su un finding high. Verificato dal Lead rieseguendo: 181 test da 177 di baseline, 150 probe C10 da 146 ciascuna osservata fallire da sola, protocol_hashes.py senza valori mossi, e git diff --numstat su light_client.rs vuoto, cioe' checkpoint_is_fresh invariata riga per riga.

La chiusura: un tipo AttestationClock in cadence.rs con due costruttori e campi privati. now_ms = max(orologio locale, checkpoint.issued_at_ms) per chi possiede un checkpoint verificato; orologio locale nudo in bootstrap, con floor_ms() zero per costruzione.

La regola 5 e' divisa sui due orologi, che e' il rimedio di RF-002: la meta' della scadenza legge il valore pavimentato, la meta' dell'ammissione legge l'orologio nudo. Senza la divisione il pavimento avrebbe dato alla chiave di rilascio una capacita' di ammissione sul trasporto che non possedeva.

Il termine illimitato NON e' chiuso, ed e' scritto cosi' in quattro artefatti. Cio' che il pavimento cambia e' la grandezza da cui il residuo dipende: prima l'errore dell'orologio, non osservabile ne' correggibile senza un riferimento esterno; ora l'eta' di un artefatto che l'operatore ottiene fuori banda senza possedere un orologio giusto. Un residuo piccolo e' ottenibile, non garantito.

GATE-DRIFT-ANSWERED-FIRST soddisfatta nell'ordine e nella sostanza, su tre fonti indipendenti che legano il controllo di drift alla ricezione della proposta, piu' due che confermano e nessuna che contraddica.

step_one_and_the_attestation_floor_do_not_compose, in tests/light_client_perimeter.rs, asserisce la composizione invece di ragionarla: e' il test che fallisce il giorno in cui qualcuno subordina il pavimento al passo 1, mossa che altrimenti non romperebbe nulla e lo renderebbe inerte per sempre.</resolution_evidence>
</invoke>
