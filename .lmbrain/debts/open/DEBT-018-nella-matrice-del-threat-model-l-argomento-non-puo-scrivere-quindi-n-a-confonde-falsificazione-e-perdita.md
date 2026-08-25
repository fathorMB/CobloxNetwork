---
id: DEBT-018
title: "Nella matrice del threat model l'argomento «non puo' scrivere, quindi n/a» confonde falsificazione e perdita"
status: open
category: "documentation"
severity: "medium"
origin_severity: null
area: "security"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "REVIEW-021"
origin_ref: "RF-005"
related_specs: ["SPEC-004","SPEC-013"]
related_reviews: ["REVIEW-021"]
related_decisions: ["ADR-015"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["threat-model","security"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-018-EVENT-001"
    timestamp: "2026-08-25T23:11:21.763527900+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-013 su segnalazione di AGENT-007, che lo indica come il piu utile dei tre residui da non perdere e precisa che e manutenzione del proprio documento e non della spec. Registrato come debito e non chiuso nella remediation per quella ragione: allargare SPEC-013 al threat model sarebbe stato scambiare il perimetro di una spec con quello di un documento di analisi. Owner AGENT-007 perche il documento e suo e perche la valutazione di A-04 richiede di leggere le regole di eleggibilita, che lei dichiara di non aver letto abbastanza a fondo per pronunciarsi."
    evidence_refs: []
---
# Nella matrice del threat model l'argomento «non puo' scrivere, quindi n/a» confonde falsificazione e perdita

## Statement

La matrice di threat-model.md marca alcune celle come n/a con l'argomento che l'attore non ha un percorso di scrittura verso l'asset. L'argomento confonde due cose diverse: non poter falsificare un asset e non poter causare una perdita su quell'asset. La cella A-02 per T-06 e caduta su questo, e AGENT-007 segnala che non e stata resa falsa da SPEC-013 ma era gia falsa da SPEC-004, perche T-06 comprende per definizione l'ISP o censore e TM-31 descrive l'isolamento di nodi: un nodo isolato non riceve i propri challenge_request e subisce emissione mancata senza alcun furto di chiave. TM-37 e quindi il secondo falsificatore e non il primo.

Tre conseguenze da trattare insieme. La cella A-04, set dei validatori, poggia sullo stesso argomento nella forma nessun potere sulla composizione, e chi isola dei candidati ne fa fallire le challenge e quindi ne altera l'eleggibilita. L'elenco asset di TM-31 omette A-02, quindi la cella corretta punta a TM-37 e la via piu semplice resta non tracciata. E TM-37 e collocato sotto un attore che non lo copre, perche T-06 e definito come osservazione passiva, peer enrollato che si connette a molti, oppure ISP o censore, e chi esfiltra una chiave privata da un dispositivo non e nessuna delle tre.

## Evidence and provenance

Segnalato da AGENT-007 nell'addendum di verifica mirata a REVIEW-021, dopo che la remediation di SPEC-013 aveva falsificato e corretto la cella A-02 per T-06. AGENT-007 dichiara di aver ripassato le sei celle n/a di quella colonna con lo stesso metro: A-01, A-09 e A-13 reggono, A-08 regge al limite, e A-04 e la sola che segnala. Dichiara esplicitamente di non affermare che A-04 sia falsa, non avendo letto le regole di eleggibilita abbastanza a fondo, ma che poggia sull'argomento appena caduto e va sottoposta allo stesso test.

Sull'attore mancante AGENT-007 nomina tre uscite senza imporne una: allargare esplicitamente T-06, dichiarare TM-37 trasversale, oppure aggiungere un attore per la compromissione dell'endpoint, che giudica la piu onesta e la piu cara.

## Impact and scope boundary

Nessun impatto sul protocollo: e manutenzione di un documento di analisi. L'impatto e sulla fiducia che si puo riporre nella matrice, che e lo strumento con cui il progetto decide dove guardare. Una cella n/a dice al lettore successivo di non cercare li, quindi una n/a sbagliata ha la stessa forma dell'impossibilita dichiarata a torto gia censita nella famiglia 2 di recurring-defects.md: e la forma peggiore, perche dice di smettere di cercare.

Il caso di A-02 lo dimostra: la cella e rimasta n/a dal 2026-08-25 fino a quando una spec di tutt'altro oggetto non l'ha incrociata per caso, e la via piu semplice per falsificarla era gia scritta nel documento due sezioni piu in la.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-013 su segnalazione di AGENT-007, che lo indica come il piu utile dei tre residui da non perdere e precisa che e manutenzione del proprio documento e non della spec. Registrato come debito e non chiuso nella remediation per quella ragione: allargare SPEC-013 al threat model sarebbe stato scambiare il perimetro di una spec con quello di un documento di analisi. Owner AGENT-007 perche il documento e suo e perche la valutazione di A-04 richiede di leggere le regole di eleggibilita, che lei dichiara di non aver letto abbastanza a fondo per pronunciarsi.

## Resolution criteria

La cella A-04 per T-06 e sottoposta allo stesso test e confermata o corretta, con la ragione scritta accanto in entrambi i casi. L'elenco asset di TM-31 e completato con A-02, cosi che la via piu semplice sia tracciata. E la collocazione di TM-37 e risolta scegliendo fra le tre uscite nominate, con la scelta motivata invece che presa per comodita.

Va inoltre stabilito se l'argomento non puo scrivere quindi n/a compaia altrove nella matrice fuori dalla colonna T-06, perche il difetto e nell'argomento e non nella colonna. Se ne compare, ogni occorrenza va risottoposta.

## Valutazione — 2026-08-26 (AGENT-007)

Il documento è mio, quindi questa non è una review indipendente: è
un'autovalutazione, e va letta con quella riserva. Per compensarla ho fatto la
sola cosa che compensa qualcosa, cioè **contare invece di ricordare**: ho
riclassificato tutte e trentuno le celle `n/a` della matrice per l'argomento su
cui poggiano, e il conteggio è riproducibile da chiunque rilegga §4.

### Cosa è accertato, e come

**1. L'ampiezza reale è 25 celle su 31, non una.** Ho classificato ogni `n/a` di
`.lmbrain/knowledge/threat-model.md` §4 per la forma del motivo scritto nella
cella. Il totale torna con la frase di riga 141 (60 coperte + 31 `n/a` = 91).

- **Argomento «nessun percorso di scrittura / di falsificazione verso l'asset»
  — 25 celle.** `A-01`×`T-02`,`T-05`,`T-06`; `A-03`×`T-05`;
  `A-04`×`T-01`,`T-03`,`T-05`,`T-06`; `A-05`×`T-05`; `A-06`×`T-05`;
  `A-07`×`T-03`,`T-05`; `A-08`×`T-03`; `A-09`×`T-01`,`T-02`,`T-03`,`T-04`,
  `T-06`,`T-07`; `A-12`×`T-01`,`T-05`; `A-13`×`T-01`,`T-02`,`T-03`,`T-06`.
- **Argomento di *movente* invece che di capacità — 4 celle**, tutte in `T-01`:
  `A-05` («non ha vantaggio a falsificare»), `A-07` («abusa della propria
  identità, non ne attacca altre»), `A-08` («risparmia le proprie risorse»),
  `A-10` («vuole la rete viva, ci guadagna»).
- **Argomento di capacità osservativa — 2 celle:** `A-08`×`T-06`,
  `A-11`×`T-01`.

Il debito nomina una cella e ne implica poche. **Sono venticinque**, e la
correzione non può quindi avere la forma di tre modifiche puntuali. È il fatto
che cambia la scala del lavoro, ed è la ragione principale di questa
valutazione.

**2. `A-04` × `T-06` è falsa, e cade più semplicemente di come il debito
suppone.** Il debito ipotizza la via lunga — isolare dei candidati ne fa fallire
le challenge e ne abbassa il `contribution_score`. Quella via funziona
(`ledger.md:1030` condizione 3, e la 4 sui `validator_eligibility_min_issuers`
emittenti distinti, che un censore batte isolando il candidato da tutti gli
emittenti tranne uno). **Ma esiste una via più corta e più sicura, alla
condizione 2** (`ledger.md:1028-1029`): l'eleggibilità richiede una
`validator_candidacy` **finalizzata** strettamente sotto
`candidacy_close_height(e)`. Un censore che tenga il candidato offline per la
sola durata della finestra di candidatura impedisce che quella transazione sia
mai propagata, e il nodo è ineleggibile **senza che nulla di suo fallisca e
senza alcuna scrittura di nessun tipo**. Non c'è punteggio da abbassare: manca
proprio l'atto. Il motivo scritto nella cella — «nessun potere sulla
composizione» — è vero su *chi entra* e falso su *chi resta fuori*, ed è
esattamente la confusione che questo debito nomina. **Dichiaro `A-04` × `T-06`
falsificata**, e ritiro la riserva che avevo espresso nell'addendum a
[REVIEW-021] («non ho letto le regole di eleggibilità abbastanza a fondo»): ora
le ho lette, e la cella cade.

**3. Due celle in più cadono, e nessuno le aveva nominate.**

- **`A-13` × `T-06`.** Motivo scritto: «il trasporto non è fidato per costruzione
  (`app-manifest.md` §Deterministic container)». L'indirizzamento per contenuto
  impedisce la **sostituzione** di un modulo; non impedisce che un ISP ne
  impedisca la **consegna**. E `A-13` si chiama, testualmente,
  *«Integrità del catalogo e della **distribuzione** dei moduli»*: la
  distribuzione è nel nome dell'asset. Lo scenario esiste già, è TM-31, ed è
  nella stessa colonna. **È la stessa forma di `A-02` × `T-06`: la via più
  semplice era già scritta nel documento.**
- **`A-09` × `T-07`.** Motivo scritto: «l'insider agisce su parametri e liste,
  non sul runtime». **Contraddetto dalla definizione dell'attore nella stessa
  pagina** (riga 112, `T-07`): fra le sue capacità c'è *«la distribuzione dei
  trust anchor e dei binari»*, e TM-36 passo 2 nomina *«la build, l'installer, lo
  store»*. Chi controlla il binario controlla il runtime WASM che quel binario
  contiene: non ha un percorso *verso* la sandbox, ha la sandbox. Questa non è
  nemmeno confusione fra falsificazione e perdita — **è una `n/a` contraddetta
  dalla riga che definisce l'attore**, cioè il difetto già scritto e non
  guardato nella sua forma più pura. TM-36 va aggiunto in quella cella e `A-09`
  va aggiunto agli asset di TM-36 (oggi `A-05`, `A-13`, riga 1563).

**4. La nota sotto la matrice contiene un superlativo ora falso.** Righe 142-148
elencano quattro `n/a` che sarebbero *«proprietà di design conquistate»*, e fra
queste c'è **`A-13` × `T-06`** — che il punto 3 falsifica. La frase va corretta
nella stessa passata, non dopo: è la famiglia 2, e la quinta occorrenza di quella
famiglia nacque esattamente così, aggiornando una sola delle due frasi note.

**5. L'elenco asset di TM-31 va completato con `A-02`, `A-04` e `A-13`**, non
solo con `A-02`. Oggi è `A-03, A-05, A-10, A-12` (riga 1366). Le tre celle che
TM-31 copre e non dichiara sono quelle dei punti 2 e 3 più quella già corretta.

**6. La collocazione di TM-37 è sbagliata in tre celle, non in una.** TM-37
compare in `A-02`, `A-06` e `A-11` della colonna `T-06`. L'attore che lo esegue —
chi esfiltra una chiave privata da un dispositivo — non è nessuna delle tre
capacità con cui `T-06` è definito (riga 111: osservazione passiva; peer enrollato
che si connette a molti; ISP o censore). **L'effetto collaterale è peggiore
dell'errore di collocazione:** attribuendo TM-37 a `T-06` la matrice *gonfia*
`T-06` di una capacità che non ha, e **nasconde per intero l'attaccante che
compromette l'endpoint**, che nel modello non esiste. Ed è per quell'assenza che
`A-07` × `T-03`,`T-05` può dire «nessun percorso verso chiavi altrui» senza che
nessuno obietti: **è vero solo perché nessun attore del modello ruba chiavi.**

### Il difetto ha una seconda metà che il debito non nomina

Le quattro celle di movente (punto 1, colonna `T-01`) violano il **metodo
dichiarato dal documento stesso** a riga 101: *«Ogni attore è descritto per
capacità e budget, non per intenzione: la difesa si progetta sul primo, non sulla
seconda»*. Poi la colonna `T-01` archivia quattro celle sull'intenzione. È un
difetto **diverso** da quello del titolo di questo debito, ed è più insidioso,
perché un motivo di capacità sbagliato si falsifica leggendo una regola, mentre
un motivo di movente non si falsifica affatto: non è una proposizione sul
sistema. `A-10` × `T-01` — «l'egoista vuole la rete viva» — è l'esempio: un nodo
egoista che sbagli il calcolo, o che sia egoista *e* incompetente, danneggia la
disponibilità comunque. **Non affermo che le quattro celle siano false**; affermo
che i loro motivi non sono del tipo che il documento si è imposto e vanno
riscritti in termini di capacità, il che potrebbe farne cadere una o nessuna.

### Cosa resta ignoto

- **`A-08` × `T-06`** («l'osservatore non impone carico oltre il traffico
  ordinario») è la sola delle 31 su cui **non mi pronuncio**. `A-08` comprende
  *«reputazione dell'IP, esposizione legale»*, e un ISP o censore incide su
  entrambe; d'altra parte il costo dell'isolamento è già contato su `A-10` e
  `A-02`, e contarlo una terza volta su `A-08` diluirebbe l'asset. **È una
  questione di confine fra asset, non di percorso**, ed è il tipo di domanda che
  va decisa e scritta, non risolta di corsa. La segnalo come la sola cella
  residua.
- **Quante delle 25 cadano davvero.** Ne ho falsificate tre (`A-04`×`T-06`,
  `A-13`×`T-06`, `A-09`×`T-07`) più quella già corretta. Non ho sottoposto al
  test le altre 22 con la stessa profondità: farlo richiede rileggere per
  ciascuna la regola che la sostiene, ed è **il lavoro della spec, non della
  valutazione**. Dichiaro il metodo e non il risultato, perché affermare che le
  altre reggono senza averle istruite sarebbe la conferma di comodo che questo
  progetto ha imparato a riconoscere.

### Che forma deve avere la correzione

**Non tre modifiche. Una regola di forma sulla matrice, più una passata.**

1. **Una riga di metodo in §4**, che imponga a ogni `n/a` di rispondere a
   **due** domande distinte e di scriverle entrambe: *l'attore può falsificare
   l'asset?* e *l'attore può causare una perdita su quell'asset?* Un `n/a` è
   ammesso solo se entrambe hanno risposta negativa, e il motivo scritto deve
   dire quale delle due sta rispondendo. È la forma che rende il difetto
   **non ripetibile**, che è ciò che le tre correzioni puntuali non fanno.
2. **La passata sulle 25 celle** sotto quella regola, con l'esito scritto in
   ogni cella anche quando conferma.
3. **La riscrittura in termini di capacità delle 4 celle di movente**, con la
   riga 101 citata come il criterio che le condanna.
4. **La correzione della nota righe 142-148** (punto 4).
5. **Gli elenchi asset di TM-31 e TM-36** (punti 3 e 5).
6. **La collocazione di TM-37** (sotto).

### Le tre uscite per TM-37: la scelta, motivata

Avevo nominato tre uscite senza sceglierne una. **Scelgo la terza: un attore
nuovo, `T-08` — compromissione dell'endpoint.** Le altre due vanno respinte per
ragioni scritte, non per preferenza:

- **allargare `T-06`** unisce sotto un ID due avversari con budget
  incomparabili — chi guarda il filo e chi entra nel dispositivo — e il documento
  dichiara a riga 100-101 di descrivere gli attori *per capacità e budget*. È
  l'uscita che costa meno e che rende la colonna `T-06` inutilizzabile per
  decidere, che è a cosa la matrice serve;
- **dichiarare TM-37 trasversale** toglie lo scenario dalla matrice invece di
  collocarlo, e una matrice con eccezioni fuori griglia smette di essere
  l'evidenza di `GATE-COVERAGE`;
- **`T-08`** è la sola che dice la verità, ed è la più cara: **tredici celle
  nuove**, e almeno tre `n/a` esistenti da riesaminare perché reggono solo
  sull'assenza di questo attore (`A-07` × `T-03`, `A-07` × `T-05`, e la nota
  delle «proprietà di design conquistate» che cita `A-07` × `T-03`/`T-04`).
  **Il costo è la ragione per cui è giusta**: rende visibile che il modello non
  aveva un attaccante di endpoint, il che è un fatto sul modello e non sulla
  rete.

`T-08` è inoltre **necessario a [DEBT-022]**: l'attaccante che spende con una
chiave revocata è un attaccante di endpoint, e oggi non ha alcuna cella dove
essere registrato.

### I rimedi apparenti che non rimediano

1. **«Correggere le tre cose nominate dal debito e chiudere.»** È il rimedio che
   il debito stesso suggerisce e **non rimedia**: lascia 22 celle sullo stesso
   argomento e nessuna regola che impedisca la prossima. La chiusura di questa
   famiglia ha la forma della famiglia 1 rovesciata già usata per
   `ed25519-speccheck`: non correggere l'artefatto, **costruire il criterio che
   mancava**.
2. **«Sostituire gli `n/a` dubbi con “da valutare”.»** Peggiora: un `n/a` falso
   dice al lettore di smettere di cercare, un «da valutare» permanente dice la
   stessa cosa con l'aria di non dirla, e non ha nemmeno il pregio di essere
   falsificabile. Se una cella non è decisa, si scrive lo scenario o si scrive
   perché non lo si è deciso.
3. **«Meccanizzare la matrice con una gate.»** Non è meccanizzabile e va detto
   qui perché qualcuno lo proporrà: [ADR-012] verifica forme e coerenze fra
   copie, mai la correttezza semantica di una cella, e lo dichiara nella propria
   intestazione. Questa è la famiglia 2, che `recurring-defects.md` dichiara non
   meccanizzabile. L'unico presidio disponibile è la domanda scritta nel metodo,
   ed è per questo che il punto 1 della correzione è una riga di metodo.
4. **«L'isolamento è già coperto da `A-10`, quindi le celle su altri asset sono
   ridondanti.»** È l'obiezione che sembra sensata e che ha prodotto il difetto:
   ogni asset misura una perdita diversa, e l'emissione mancata (`A-02`),
   l'eleggibilità negata (`A-04`) e il modulo non consegnato (`A-13`) non sono la
   stessa perdita della connettività (`A-10`). Accorparle è precisamente
   **confondere il percorso con la perdita**, un livello più su.

### Riclassificazione

**Severità confermata `medium`; categoria confermata `documentation`; nessun
impatto sul protocollo.** Non la alzo, perché nessuna regola cambia e nessuna
rete esiste. **Ma l'ampiezza va corretta nel corpo del debito**: da «tre
conseguenze da trattare insieme» a «25 celle su 31 poggiano sull'argomento, 3
falsificate finora, più 4 celle con un difetto diverso e una regola di metodo da
scrivere». Un debito che dichiara un lavoro tre volte più piccolo di quello che è
verrà pianificato per quello che dichiara.

Aggiungo un rilievo sul **titolo del debito**, che è mio quanto il documento: dice
*«confonde falsificazione e perdita»*, e su `A-09` × `T-07` (punto 3) il difetto
non è quello — è una `n/a` contraddetta dalla definizione dell'attore. Il titolo
copre la maggioranza dei casi e non tutti. Non lo riscrivo: lo segnalo perché chi
chiuderà il debito non restringa il lavoro al titolo.

### Raccomandazione di raggruppamento

**Spec propria, e per prima delle tre.** L'agente è **AGENT-007**, perché il
documento è suo e perché `can_implement` copre esattamente questo tipo di
deliverable documentale. **La review spetta al Lead**, per il vincolo di profilo
che vieta ad AGENT-007 di rivedere il proprio lavoro — ed è un vincolo che qui
morde davvero, perché questa valutazione è già un'autovalutazione.

**Nessuna gate [ADR-012]**: non si tocca alcuna regola di validità e non si tocca
`docs/protocol/`.

**La sequenza è sostanziale e non estetica.** [DEBT-018] va chiuso **prima** di
[DEBT-022] e di [DEBT-017], per due ragioni indipendenti: (a) [DEBT-022] descrive
un attacco che oggi non ha né cella né attore, e `T-08` è la casella che gli
serve; (b) [DEBT-017] deve riscrivere la contromisura (a) di TM-37, e riscriverla
mentre la collocazione di TM-37 è in discussione significa scriverla due volte.

**Non raggruppare i tre in una spec sola.** Tre documenti diversi, tre agenti
diversi, e due delle tre passano per [ADR-012] mentre questa no: raggrupparle
imporrebbe a una manutenzione documentale la disciplina di una regola di validità
e a due regole di validità il perimetro di una passata sola. La forma giusta è
**tre spec in sequenza**, con questa in testa.

## Resolution evidence

