---
id: ADR-017
# Note: Quote the title if it contains a colon
title: "La revoca ha due lavori, e nessuno dei lavori del pavimento riguarda il saldo"
status: accepted
decision_date: 2026-08-26
decider: OPERATOR
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: [ADR-010, ADR-015, ADR-012, DEBT-040]
tags: [consensus, security, identity]
created: 2026-08-26
updated: 2026-08-27
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned proposed -> accepted"
  - date: 2026-08-27
    action: "corretta la parte 2 su REVIEW-042 RF-001: pavimento di G ancorato in genesi come relazione, decisione dell'operatore"
  - date: 2026-08-27
    action: "correzione della parte 2 approvata dall'operatore dopo lettura"
  - date: 2026-08-27
    action: "seconda correzione: punti 1, 2 e 5 approvati dopo REVIEW-046, punto 3 riaperto"
---
# La revoca ha due lavori, e nessuno dei lavori del pavimento riguarda il saldo

> Proposta dal Lead e **decisa dall'operatore il 2026-08-26**, per chiudere [DEBT-033].
>
> **Seconda stesura, ed è quella decisa.** La prima è stata sottoposta a critica avversariale da AGENT-007 su richiesta dell'operatore, prima della decisione: [REVIEW-036], dieci finding e cinque errori fattuali. La parte 1 è sopravvissuta a ogni attacco ed è conservata. La parte 2 è stata **rifatta**. La parte 3, che aggiungeva un campo di altezza a `challenge_evidence`, è stata **tolta**: la critica ha stabilito che non chiude la superficie che diceva di chiudere, e la sua sostanza è passata su [DEBT-034]. Il titolo stesso è cambiato, perché la premessa della prima stesura era falsa.

## Context

[SPEC-019] ha fissato una definizione unica di *enrolled, unrevoked* per autorizzare una transazione, ancorandola a `effective_height`, e ha scartato la lettura *«revoca finalizzata sotto `h`»* con un argomento che regge: quella lettura dipende da quali certificati un nodo ha raccolto, quindi **non è una regola stretta con margine largo, è un fork con una specifica**.

Chiudendola, [SPEC-019] ha spostato peso su `effective_height`. Prima quel campo governava la transizione del set di validatori; ora governa **anche** se una chiave revocata possa svuotare un saldo. E il documento lo ha dichiarato invece di tacerlo: `ledger.md` dice in chiaro che *nulla in v0 limita `effective_height` dall'alto*, e che finché quel campo non è limitato **quanto una revoca protegga un saldo lo sceglie il quorum che revoca**.

Lo stesso documento nomina le tre domande rimaste aperte — quanto debba essere corta la finestra, se il pavimento debba dipendere da `reason`, cosa limiti `effective_height` dall'alto — e le classifica come *«meccanica della revoca»*, aggiungendo che quel lavoro **è lavoro che nessuno ha fatto, non una cosa che non si possa fare**. Questo ADR è quel lavoro.

### I fatti verificati, con il perimetro su cui valgono

Verificati sull'albero a `3f1bef7`. **Tre di questi sono correzioni a errori della prima stesura**, trovati da [REVIEW-036] e riverificati dal Lead in modo indipendente.

- **`effective_height` è nominato da tre MUST, non da due.** La prima stesura diceva due, e l'errore era ereditato da [DEBT-033] e prima ancora da [REVIEW-033] RF-001: **tre artefatti che ripetevano lo stesso conto**. La ragione è che l'enumerazione era fatta sul **token** `effective_height`, mentre la grandezza ha una seconda grafia. Contando entrambe le grafie: `ledger.md:1033` (clausola 4, un **pavimento**), `ledger.md:1064` (light client), e **`ledger.md:785`** — *«The effective height MUST be later than the block proposing the revocation»* — che è la regola che impedisce a una revoca di mordere nel proprio blocco, cioè la riga che questo ADR tocca più da vicino. *Perimetro: `docs/protocol/`; trenta occorrenze del token in `ledger.md`, di cui tre su righe con `MUST` e una di quelle è prosa sui MUST, più due righe che usano la grafia a spazio.*
- **Un riferimento lo limita verso l'alto, ed è la clausola 8** della regola di contrazione del set (`ledger.md:1107`, `is at most that activation_height`). La prima stesura scriveva *«la sola»* e l'enumerazione portata non lo sosteneva: `ledger.md:107`, la clausola 2 della definizione di [SPEC-019], ha la stessa forma sintattica (`carries an effective_height at most h`). La conclusione non cambia — **nessuna delle due è un tetto sul campo**, perché entrambe condizionano la validità di *altro* e lasciano valida una revoca con efficacia assurda — ma il superlativo è stato tolto.
- **`reason` è inerte.** Quattro occorrenze fuori dal brain: lo schema a `ledger.md:778`, la fixture canonica a `793`, la sua copia in `canonical_serialization.rs`, la probe dell'inventario di [ADR-012]. **Nessuna regola lo legge, nessun codice lo ramifica.** È però già impegnato nell'ID della transazione, quindi renderlo letto non cambia il formato. *Enumerazione rieseguita in modo indipendente da [REVIEW-036] e confermata.*
- **`min_revocation_effective_delay_blocks` è governato, senza valore di lancio e senza limite di genesi.** Diciassette occorrenze su `docs/` + `core/` + `sim/` escludendo gli artefatti di build, di cui **tre soltanto gli assegnano un valore** (`tests/common/mod.rs:242`, `:516`, `protocol_hashes.py:142`), tutte e tre a `1`, tutte e tre fixture. Non è nella lista DRAFT dei parametri di lancio. E — questo è il fatto che ha rifatto la parte 2 — **non è nel blocco dei vincoli di magnitudine** di `ledger.md#magnitudes-not-only-relations`, né in `ElectionBounds`. Quel blocco esiste proprio per impedire a un set seduto di camminare un parametro governato fino a un valore assurdo, e questo parametro ne è fuori. *Verificato leggendo il blocco per intero.*
- **L'ordine di esecuzione dentro un blocco è già deciso.** `ledger.md:2819` mette `revoke_identity` in **classe 0** e `burn`/`fund_app` in **classe 1**. La prima stesura dichiarava aperta la scelta fra mordere a `h` o a `h+1`: era chiusa. Sul percorso del saldo una revoca inclusa in `h` esegue prima di ogni spesa dello stesso blocco, per ogni verificatore.

### Il fatto che governa la decisione

**Il pavimento ha due lavori, contati, e nessuno dei due riguarda il saldo.**

La prima stesura diceva *«ha una giustificazione, e quella esiste su un percorso solo»*. Era falso, e [REVIEW-036] lo ha stabilito enumerando:

1. `ledger.md:1033-1036` — dare ai validatori superstiti una finestra dichiarata in cui impegnare un set successore conforme, così che la catena non si fermi. È **liveness del set**.
2. `ledger.md:1075-1080` — fare da tetto a `max_weak_subjectivity_age_ms` per MUST, così che un checkpoint ancora accettato non sia mai più vecchio della finestra concessa. È **freschezza dell'ancora di fiducia del light client**.

La premessa corretta è più debole della prima e basta lo stesso: **nessuno dei due lavori è una ragione per mettere un ritardo sul saldo.** Un saldo non ha bisogno di una finestra per essere protetto — ne ha bisogno il set, e ne ha bisogno il light client. Il pavimento è quindi giustificato su due percorsi e arbitrario su un terzo, ed è proprio sul terzo che [SPEC-019] ha appena messo il peso.

### La cosa che va guardata prima di scegliere

[DEBT-033] scarta la via *«far mordere la revoca sul percorso di spesa a `min(effective_height, proponente + pavimento)`»* perché *«reintrodurrebbe due significati di "revocata" alla stessa altezza, cioè esattamente ciò contro cui [SPEC-019] ha argomentato»*.

**Quella lettura eredita la conclusione di [SPEC-019] senza il suo confine.** L'argomento aveva due parti, e solo una era fatale:

1. la lettura era **dipendente dal verificatore** — due nodi con certificati diversi danno verdetti diversi. **Questa è la parte fatale**, ed è un fork.
2. dava **due significati di *revocata* alla stessa altezza**. Questa è leggibilità, non correttezza.

L'opzione innesca la **(2)** e non la **(1)**: entrambe le grandezze sono nel corpo della transazione, lette dagli stessi byte da ogni verificatore, e la funzione è totale sul blocco e i suoi antenati e monotona in `h`. Non è un fork. [REVIEW-036] ha attaccato questa affermazione da sei direzioni — riorganizzazioni, mempool, inclusione condizionale, non retroattività, revoche multiple, censura — e non l'ha rotta.

E la **(2)** è già lo stato del protocollo, in forma più forte: `ledger.md:222` dichiara che dentro la finestra un nodo è **contemporaneamente** autorizzato a spendere, contato a piena potenza di voto, e irraggiungibile da ogni peer conforme. Sono già **tre** risposte diverse alla stessa altezza. Il peccato non è averne una in più: è averne una **non dichiarata**.

## Decision

> **Decisa dall'operatore il 2026-08-26**, sulla seconda stesura, dopo la critica avversariale di [REVIEW-036].

La revoca fa due lavori — togliere una chiave dal set, e togliere a una chiave il potere di spendere — e questo ADR li separa lungo la linea su cui corrono le giustificazioni del pavimento.

### 1. Il percorso di spesa non ha pavimento: la revoca morde all'inclusione

Sul percorso di autorizzazione delle transazioni, una revoca qualifica la chiave a partire dall'**altezza del blocco che la include**, non da `effective_height`.

È la **terza lettura** che `ledger.md:186-193` descrive e certifica — *«un fatto sul blocco e i suoi antenati, monotono in `h`, letto dagli stessi byte da ogni verificatore»* — e **chiude la finestra**. Il documento non l'ha adottata perché adottarla significa ridefinire cos'è `effective_height`, cioè meccanica della revoca, e questo ADR è la sede in cui quella ridefinizione si fa.

**Morde a `h`, e non è una scelta di questo ADR**: `ledger.md:2819` la fa già, mettendo `revoke_identity` in classe 0 e la spesa in classe 1.

**Cosa chiude:** l'intera superficie di [DEBT-033] sul percorso di spesa. Non la limita, la toglie.

**Cosa resta aperto, e va aperto come debito proprio invece che rinviato alla spec:** dentro la **classe 0** l'ordine è per raw transaction ID, e l'ID è l'hash di un corpo che porta `created_at_ms`. Il revocante può enumerare millisecondi finché il proprio ID ordina prima o dopo quello di una `validator_candidacy` bersaglio. È deterministico, quindi non è un fork — ma è una discrezione, ed è [REVIEW-036] RF-006. Non tocca il saldo, perché la classe 1 sta sempre dopo.

### 2. Il percorso del set tiene il pavimento, e il tetto è una banda dichiarata dipendente da `reason`

Sul percorso della transizione del set, `effective_height` conserva il significato che ha oggi e il pavimento resta, perché lì le sue ragioni esistono.

**Il vincolo è una banda a due lati, non un'uguaglianza.** La prima stesura derivava `effective_height = proponente + F` esattamente su `key_compromise`. Era inapplicabile: **l'autore non conosce l'altezza di inclusione**, e un solo proponente ostile, per un solo turno, avrebbe invalidato la transazione — convertendo una censura di severità media in un **veto sulla revoca d'emergenza**, contro una transazione che porta `expires_at_ms`. È [REVIEW-036] RF-002, ed è il colpo che ha rifatto questa parte.

Sia `F` = `min_revocation_effective_delay_blocks`, `p` = altezza del blocco proponente, `G` = `revocation_effective_grace_blocks` (nuovo), `P` = `max_planned_revocation_delay_blocks` (nuovo).

| `reason` | Vincolo su `effective_height` | Perché |
| --- | --- | --- |
| `key_compromise` | `p + F <= effective_height <= p + F + G` | La chiave è in mano al nemico. Il set ha bisogno della sua finestra, più un margine dichiarato per assorbire il ritardo di inclusione. Nient'altro. |
| `validator_misconduct`, `operator_request` | `p + F <= effective_height <= p + P` | Uscita programmata o condotta da sanzionare senza urgenza crittografica. La latitudine è il punto, e va dichiarata come parametro invece che come assenza di regola. |

**Due righe e non tre**, ed è una correzione. La prima stesura dava a `validator_misconduct` un tetto di `2 × F`. Quel `2 ×` aveva **il denominatore sbagliato**: `F` è tarato su quanti blocchi servono ai superstiti per impegnare un set successore, e il margine legittimo di una cattiva condotta non ha relazione con quella durata. La conseguenza sarebbe stata un accoppiamento che nessuno sceglierebbe — alzare `F` per rendere lo stallo più raro **raddoppierebbe la latitudine sulla cattiva condotta**. È [REVIEW-036] RF-009.

**I tre parametri entrano nel blocco dei vincoli di genesi**, ed è la parte che manca oggi:

```text
min_revocation_effective_delay_blocks >= 1
revocation_effective_grace_blocks     >= 1
max_planned_revocation_delay_blocks   >= min_revocation_effective_delay_blocks
                                         + revocation_effective_grace_blocks

// limiti di magnitudine, presi dall'ancora di fiducia di genesi e mai dal
// documento sotto valutazione:
min_revocation_effective_delay_blocks <= min_revocation_effective_delay_blocks_max
revocation_effective_grace_blocks     <= revocation_effective_grace_blocks_max
max_planned_revocation_delay_blocks   <= max_planned_revocation_delay_blocks_max

// correzione del 2026-08-27, [REVIEW-042] RF-001: il pavimento della banda
// deve stare nell'ancora di genesi, non fra i vincoli relazionali, perche'
// e' il lato da cui la banda difende:
revocation_effective_grace_blocks     >= revocation_effective_grace_blocks_min
revocation_effective_grace_blocks_min + 1 >= validator_min_set_size_min
```

> **Correzione del 2026-08-27, decisa e approvata dall'operatore.** [REVIEW-042]
> RF-001, `high`, contro questa stessa ADR. Le due righe qui sopra non c'erano.
> La forma della correzione e' stata scelta dall'operatore fra tre alternative;
> il testo e' stato redatto dal Lead e approvato dall'operatore dopo lettura,
> lo stesso giorno. Da qui e' vincolante per l'implementazione.

**Il pavimento era scritto dove il set seduto poteva portarlo a `1`.** Questa
decisione dichiarava, fra le alternative scartate, che la banda si paga «al
prezzo di `G`, che e' discrezione **dichiarata e limitata in genesi** invece che
illimitata». Non ha mantenuto la promessa: in `ElectionBounds` era ancorato solo
il **tetto** `revocation_effective_grace_blocks_max`, mentre il pavimento
`G >= 1` stava in `check_relations`, cioe' fra i vincoli che un
`consensus_parameters` governato soddisfa da se'. Un documento con `G = 1`
passava ogni controllo ed era indistinguibile da governance ordinaria: da quel
momento un `key_compromise` andava firmato a quorum e incluso entro una finestra
di **due blocchi**, predetta prima che il giro di firma cominciasse, e due
blocchi di censura o una riorganizzazione di profondita' due lo invalidavano.

**Il pavimento e' una relazione, non un numero.** `G+1` e' la larghezza della
finestra in blocchi, e la cadenza non e' imposta: la finestra non si converte in
tempo reale, quindi non puo' essere giustificata in secondi. La forma scelta e'
strutturale — **la finestra dura almeno una rotazione completa del set minimo**.
L'argomento si chiude da se': su una finestra lunga una rotazione ogni validatore
ha un turno di proposta dentro la finestra, e una coalizione capace di censurare
un'intera rotazione avrebbe gia' il quorum — a quel punto la revoca sarebbe vana
comunque, e questo parametro non e' piu' la difesa giusta. Cosi' il numero non va
scelto oggi contro la misura che la sezione *Revisit* dichiara mancante: arriva
con la taratura di `validator_min_set_size_min`.

> **SECONDA CORREZIONE, del 2026-08-27.** I punti 1, 2 e 5 sono **decisi e
> approvati** dall'operatore dopo che [REVIEW-046] li ha attaccati senza
> romperli. Il **punto 3 e' riaperto**: la regola che la v2 proponeva e' rotta,
> e il difetto e' scritto qui sotto invece che altrove.
>
> Percorso di questa parte 2, perche' sia leggibile: prima stesura rotta da
> [REVIEW-036], prima correzione rotta da [REVIEW-044], v1 della seconda rotta da
> [REVIEW-045], v2 rotta da [REVIEW-046]. Le prime due erano gia' normative
> quando sono cadute; le ultime due no, ed e' la differenza che la critica
> avversariale prima della decisione ha prodotto.

### 1. L'argomento della rotazione e' ritirato — DECISO

La prima correzione giustificava il pavimento di `G` dicendo che la finestra dura
almeno **una rotazione completa del set minimo**. L'argomento e' falso e la frase
e' ritirata.

Regge una gamba sola, e basta da sola: **censura e quorum non sono la stessa
soglia.** Sotto il protocollo a due fasi di [ADR-018] oltre **un terzo** del
potere fa fallire ogni round trattenendo i precommit, mentre il quorum ne
richiede **due terzi**. Ne segue che nessuna larghezza di finestra difende sopra
un terzo: il pavimento proteggeva dalla minaccia sbagliata.

Le altre due gambe che la v1 elencava erano descritte male e non si usano:
citavano il punto 7 **di contesto** di [ADR-018] mentre la sua §3 assegna un
proponente e [SPEC-025] lo portera' in `docs/protocol/` — un fatto **con scadenza
programmata** usato per un ritiro permanente — e chiamavano «sorteggio» un
round-robin pesato.

Il vincolo era inoltre ancorato al **pavimento del pavimento**:
`permissive_bounds()` porta `revocation_effective_grace_blocks_min` e
`validator_min_set_size_min` entrambi a `1` con set massimo `1000`, quindi
soddisfa la relazione con `G = 1` — la finestra di due blocchi da cui
[REVIEW-042] era partita.

### 2. La rimozione del tetto e' ritirata — DECISO

La v1 la motivava con «un ritardo di inclusione torna a poter solo rimandare».
E' falso, verificato eseguendo: con `F=10, G=5, e=100` l'estremo **superiore**
della finestra di inclusione e' `p = 90` **identico con e senza tetto**, e a
`p = 91` la revoca e' invalida in entrambi i casi per il **pavimento**
`e >= p + F`, cioe' per la clausola 4 preesistente. Il tetto sposta solo
l'estremo **inferiore**, verso il basso, dove non c'e' avversario.

**Il tetto non e' il lato che la censura attraversa.** L'errore nasce in
[REVIEW-042] RF-001 ed e' stato ripetuto dal Lead in due artefatti senza
verificarne l'aritmetica.

Cadono con esso le due ragioni che sostenevano la rimozione. [REVIEW-045] RF-004
accerta che il tetto **non** toglieva il rimedio della diluizione — servono dieci
altezze distinte nel caso peggiore a `V = 81`, e una sola altezza di inclusione
ne ammette `G + 1`, cioe' diciotto con i bounds tarati — e che il rimedio mancava
solo sotto `G = 1`, lo stato che il pavimento di genesi ha proibito. E RF-003
accerta che toglierlo **aprirebbe** una buca nuova: resterebbe il solo
`e >= p + F`, e la chiave compromessa conserverebbe il pieno potere di voto per
`e − p` blocchi, illimitati e scelti da chi firma.

### 3. Cosa fa un ritardo di inclusione — RIAPERTO

**La v2 proponeva di derivare l'efficacia all'inclusione**, con
`e_eff = min(max(e, p + F), p + F + G)`, cosi' che un ritardo spostasse
l'efficacia invece di invalidare la transazione. La proposta era di AGENT-007,
adottata dal Lead, e [REVIEW-046] l'ha rotta — attaccando la propria stessa
proposta, che il Lead le aveva indicato come primo bersaglio proprio perche' era
entrata senza critica.

**Il clamp non e' iniettivo, e questo distrugge il rimedio che il documento
dichiara.** Verificato dal Lead eseguendo, con i bounds tarati `F = 10, G = 17`:
un lotto di `key_compromise` che un quorum onesto ha **correttamente diluito** su
altezze distinte `110, 115, 120, 125, 128` resta distinto se incluso a `p = 100`,
e **collassa interamente su `128`** se l'inclusione e' ritardata a `p = 118` —
una altezza distinta su cinque.

Il collasso e' peggiore del difetto che il clamp doveva curare. Sotto la banda
dura quelle transazioni sarebbero **rifiutate**: fallimento rumoroso e
recuperabile, si rifirma. Sotto il clamp vengono **accettate collassate**, e da
li' regola 8, regola 2 e regola 10 non ammettono alcun set valido — la catena si
ferma per sempre. E' [REVIEW-044] RF-002 **peggiorato** invece che risolto, con
l'innesco spostato dal difensore all'attaccante: basta un terzo bloccante che
ritardi l'inclusione.

**Il vincolo che una forma nuova deve rispettare, e che il clamp violava:
preservare le altezze distinte.** Il rimedio della diluizione vive esattamente
li'.

Restano da chiarire, e [REVIEW-046] li elenca come rilievi propri:

- Nessuna regola verrebbe **abrogata**: restano in piedi `ledger.md:795`, la
  clausola 4, la riga della tabella e il predicato spedito in `identity.rs`. Una
  correzione che aggiunge senza abrogare non cambia il comportamento.
- Sotto un clamp il pavimento di `G` **cambia segno**: da larghezza di una
  finestra difensiva a **latitudine garantita a chi firma**, diciassette blocchi
  sui bounds tarati.
- Il light client **non puo' calcolare** un'efficacia derivata, perche' non vede
  le transazioni: la grandezza operativa smetterebbe di essere impegnata da una
  firma.
- La formula non nominava ne' `reason` ne' `P`, mentre la banda e' dichiarata
  dipendente da `reason`.

### 4. Cosa questa correzione NON stabilisce

- **L'enumerazione degli artefatti resi falsi.** La v2 la assegnava allo
  strumento di [ADR-012]: **non puo' produrla**, e [REVIEW-046] RF-006 lo ha
  accertato eseguendolo. `C10` verifica che una frase **esista**, non che sia
  **vera**, e su questa domanda restituisce insieme vuoto. L'enumerazione e' in
  [REVIEW-046], prodotta a mano e per gruppi.
- **La sorte di `revocation_effective_grace_blocks_min`.** La v2 affermava che
  «ha cambiato mestiere». L'affermazione e' di [REVIEW-045] ma scritta nel ramo
  in cui il tetto **cade**; la v2 ritira quel ramo e ne trapianta la conclusione.
  Sotto il tetto che resta, la questione e' aperta.
- **Il tetto sui `reason` pianificati.** [REVIEW-045] accerta che **non** e' lo
  stesso difetto a severita' minore: il censore naturale e' il bersaglio stesso
  di `validator_misconduct`, gli basta un terzo bloccante, e «chi revoca sceglie
  il momento» non difende, perche' il momento sposta la **posizione** della
  finestra e non la sua **larghezza**.
- **RF-008 di [REVIEW-042]**, cioe' `P = F + G` che rende `reason` letto e
  inerte. Resta una decisione dell'operatore.

### 5. Disciplina sulla provenienza degli argomenti — DECISO

La v1 dichiarava che nessun argomento sarebbe diventato normativo prima di essere
attaccato. [REVIEW-045] RF-008 ha stabilito che quella regola non e' decidibile,
non ha proprietario, vive in un ADR che nessuno strumento legge, ed e' scritta
dalla parte che deve rispettarla. La sostituisce una forma verificabile: **ogni
`[[probe]]` il cui `why` porti un argomento di sicurezza nomina l'ID della review
che lo ha attaccato**, e lo strumento fallisce se manca. Sarebbe stata rossa
sulla probe della rotazione il giorno in cui e' entrata. E' [SPEC-026].


> **Paragrafo ritirato il 2026-08-27.** Qui stava una nota che attribuiva al
> **tetto** della banda la possibilita' che un ritardo di inclusione distrugga
> una revoca. [REVIEW-045] ha accertato che e' falso — il lato che la censura
> attraversa e' il **pavimento** `e >= p + F`, cioe' la clausola 4 preesistente —
> e il punto 2 qui sopra riporta l'aritmetica. L'affermazione falsa e' tolta
> invece che lasciata in coda, perche' era gia' diventata normativa una volta.
>
> Cio' che resta vero e' registrato su [DEBT-040]: `key_compromise` conserva il
> margine minore, cioe' l'urgenza piu' alta con la finestra piu' stretta. La sua
> *Statement* porta pero' la stessa premessa sbagliata e va riscritta quando il
> debito viene lavorato.

Senza questo la parte 2 **non toglierebbe la discrezione: la sposterebbe**. Un quorum che vuole latitudine su un `key_compromise` non toccherebbe `effective_height` — la banda glielo stringe — ma pubblicherebbe un `consensus_parameters` con `F` enorme, soddisfacendo ogni vincolo relazionale esistente. Sarebbe la **famiglia 3** alla lettera: vincolata la grandezza nominata, non quella da cui la proprietà dipende. È [REVIEW-036] RF-001, ed è l'obbligo che [ADR-010] impone e che la prima stesura non aveva assolto pur citando quell'ADR.

Il pavimento a `>= 1` non è cosmetico: **è la riga che tiene insieme la parte 2 e `ledger.md:785`.** Con `F = 0` — oggi permesso — la banda ammetterebbe `effective_height = p`, che quella riga vieta, e ogni revoca per compromissione diventerebbe incostruibile.

### 3. Quale versione del parametro governa

Ogni vincolo di cui sopra si valuta contro i **parametri di consenso in vigore all'altezza del blocco che include la `revoke_identity`**.

Questa clausola esiste per non aprire una porta ulteriore sulla famiglia di [DEBT-012], [DEBT-020] e [DEBT-028] — una regola che dipende da un parametro governato senza dire quale versione valga.

**[REVIEW-036] dichiara di avere letto questa clausola e di non averla attaccata**, annotando che è la parte dell'ADR che le è piaciuta di più e che ciò che si loda è precisamente ciò che si smette di verificare. **Resta quindi la parte meno provata di questa decisione, ed è scritto qui perché chi la citerà lo sappia.**

## Alternatives considered

- **`effective_height` derivato esattamente su `key_compromise`**, la forma della prima stesura. Rifiutata per RF-002: l'autore non conosce l'altezza di inclusione, e l'uguaglianza regala un veto a chi controlla un turno di proposta. La banda a due lati conserva l'intento — togliere la discrezione invece di limitarla — al prezzo di `G`, che è discrezione **dichiarata e limitata in genesi** invece che illimitata.
- **Solo un tetto secco `max_revocation_effective_delay_blocks`.** [DEBT-033] la dichiara *inefficace* perché un quorum ostile sceglierebbe il massimo. **Quella liquidazione è troppo netta**: un tetto converte un danno illimitato in un danno limitato, che non è nulla. Rifiutata perché **insufficiente da sola** — non distingue le ragioni e lascia intatta la discrezione sul caso che conta. La banda sopra la contiene e la rende dipendente da `reason`.
- **`min(effective_height, proponente + pavimento)` sul percorso di spesa.** Non è un fork. Rifiutata perché conserva un pavimento dove nessuno dei suoi due lavori si applica: proteggerebbe il saldo *meno* di quanto si potrebbe, in cambio di nessuna proprietà. La parte 1 è questa opzione con il pavimento a zero, ed è la stessa meccanica senza il residuo arbitrario — l'equivalenza è verificata, perché `ledger.md:785` impone che l'efficacia superi l'inclusione, quindi il `min` vale sempre l'inclusione.
- **Aggiungere a `challenge_evidence` l'altezza a cui l'auditor ha giudicato**, la parte 3 della prima stesura. **Tolta.** [REVIEW-036] RF-004 ha stabilito che `ChallengeEvidenceBody` porta `auditor_signatures` come **lista** e `outcome` come **scalare**: un campo di altezza singolo su N firmatari non rende il verdetto ricalcolabile, costringe N−1 auditor a firmare un'altezza che non è la loro, e **cancella la divergenza invece di renderla visibile**. RF-003 ha aggiunto che il controllo, se reso MUST, richiederebbe a un validatore di ricostruire le revoche *finalizzate* — cosa che `ledger.md:139-148` dichiara impossibile dalla sola catena — reintroducendo la lettura verificatore-dipendente un livello più in basso. La sostanza passa su [DEBT-034], che va riscritto.
- **Allineare la regola locale di `identity.md:814` alla definizione di [SPEC-019].** Rifiutata: renderebbe la raggiungibilità dipendente dagli antenati di un blocco che il ricevente potrebbe non avere ancora, togliendogli la capacità di proteggersi in tempo reale.
- **Non fare nulla e dichiarare la finestra.** È lo stato attuale, ed è onesto. Rifiutata perché la dichiarazione descrive un danno di cui **l'ampiezza la sceglie l'avversario**, e una dichiarazione del genere non è un limite.

## Consequences

- **`effective_height` cambia significato e resta un campo solo.** Governa la transizione del set; non governa più la spesa.
- **Quattro artefatti pubblicati diventano falsi, e la prima stesura non li nominava** — famiglia 1, la classe che questo progetto ha già subito sette volte. È [REVIEW-036] RF-008. La passata di [ADR-012] sulla spec attuativa deve enumerare almeno:
  - **la fixture `AUTH-0`** (`ledger.md:242-283`), la cui revoca è finalizzata a `20` con efficacia `50`: sotto la parte 1 le righe `21` e `49` **si ribaltano** da `valid` a `invalid`, e sono precisamente le due righe che la fixture esiste per pinnare. Ha un test di conformità dedicato, `core/coblox-core/tests/authorization_unrevoked.rs`;
  - **`ledger.md:785`**, la riga che la parte 2 vincola dal basso;
  - **il commento di `RevocationRecord`** in `authorization.rs`, che dichiara l'altezza di inclusione *«deliberately absent: the predicate does not read it»*. Va **ritrattato**, non aggiornato in silenzio;
  - **il checkpoint di soggettività debole**, che impegna `(node_id, effective_height)`: dopo la parte 1 porta la grandezza del percorso del set e **non** quella del saldo. È corretto per il light client e va detto, perché un consumatore futuro leggerebbe quel campo come *«da quando la chiave non spende»* e sbaglierebbe.
- **La frase di `identity.md:839` sulla non retroattività va riscritta**, da una a due, una per percorso. La non retroattività non viene tolta: viene detta rispetto all'altezza che governa ciascun percorso. E non c'è un caso in cui si rompa, perché l'inclusione è sempre **precedente** all'efficacia: la parte 1 anticipa il morso, non lo retrodata.
- **Nascono due parametri governati** — `revocation_effective_grace_blocks` e `max_planned_revocation_delay_blocks` — e con essi l'obbligo di fissarli **insieme a `F`, che oggi non ha valore di lancio** e non è nella lista DRAFT. Va portato all'operatore con `max_clock_drift_ms` e `D_max`/`S_max`.
- **Il rischio residuo, dichiarato invece che taciuto.** `reason` **non è verificabile da nessuno**. Renderlo letto significa che un quorum che vuole latitudine su una chiave davvero compromessa può dichiarare `operator_request`. L'incentivo a farlo è proporzionale a `P − (F + G)`, quindi **è la taratura di `G` a comprarne la riduzione**, non una regola. Contro un quorum pienamente ostile la questione è vuota: chi ha i due terzi non revoca e basta. Contro un quorum onesto `reason` funziona. È [REVIEW-036] RF-005, ed è **la parte di quel finding che questa stesura riduce senza chiuderla**.
- **La metà di [REVIEW-036] RF-005 che questa stesura non chiude**: se `F + G` non bastasse ai superstiti per impegnare un set conforme, la mossa razionale di un quorum onesto resterebbe **ritardare l'autorizzazione**, e durante quel ritardo la chiave compromessa spende ancora, perché la parte 1 morde all'inclusione. **La tenuta della parte 1 dipende dalla prontezza dell'autorizzazione.** Se `G` sia abbastanza è una domanda di taratura che oggi **nessuna misura risponde**: non esiste una simulazione del tempo di coordinamento di un set successore.
- **La raggiungibilità resta saldata al pavimento**, ed è il terzo percorso che questo ADR non stacca ([REVIEW-036] RF-007). Per un nodo **non validatore** con `key_compromise` non esiste alcun set a cui dare una finestra, eppure resta irraggiungibile-ma-iscritto per `F` blocchi, con `F` tenuto lungo per progetto. Questo ADR dichiara che i percorsi sono **tre** e ne governa **due**.
- **`min_revocation_effective_delay_blocks` acquista un limite di genesi**, quindi smette di essere una leva a due teste: oggi alzarlo **autorizza** ad allungare `max_weak_subjectivity_age_ms` per il MUST di `ledger.md:1079`, e senza il limite la parte 2 avrebbe saldato la finestra di revoca alla finestra di esposizione del light client.

## Review conditions

Rivedere **la banda di `key_compromise`** quando esisterà una misura del tempo di coordinamento di un set successore. È il numero che oggi manca e che decide se `G` è un margine o un alibi.

Rivedere **la tabella di `reason`** se la devnet mostrasse `operator_request` usato per casi con l'urgenza di una compromissione: sarebbe la misura dell'incentivo dichiarato sopra, e direbbe che la banda larga va stretta.

Rivedere **la clausola 3** per prima in qualunque revisione futura, perché è la parte che la critica avversariale ha letto e non attaccato.

**Non rivedere** la parte 1 per riavvicinare i due percorsi. La loro asimmetria non è un difetto da sanare: è la conseguenza del fatto che i due lavori del pavimento riguardano il set e il light client, e riallinearli rimetterebbe sul saldo un ritardo che nessuna proprietà chiede.
