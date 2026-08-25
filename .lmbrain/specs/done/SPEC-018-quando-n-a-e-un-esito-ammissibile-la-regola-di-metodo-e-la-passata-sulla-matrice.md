---
id: SPEC-018
# Note: Quote the title if it contains a colon
title: "Quando n/a e un esito ammissibile: la regola di metodo e la passata sulla matrice"
status: done
kind: chore
priority: high
area: security
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-007
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: []
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [threat-model, documentation]
activity:
  - date: 2026-08-26
    action: "transitioned backlog -> ready"
  - date: 2026-08-26
    action: "transitioned ready -> working"
  - date: 2026-08-26
    action: "transitioned working -> review"
  - date: 2026-08-26
    action: "transitioned review -> done"
---
# Quando n/a e un esito ammissibile: la regola di metodo e la passata sulla matrice

## Objective

Chiudere [DEBT-018] nella sua ampiezza reale, che è **circa tre volte quella che il debito dichiarava**: non tre celle, ma **venticinque delle trentuno celle `n/a`** della matrice di `.lmbrain/knowledge/threat-model.md`, più un secondo difetto che nessuno aveva censito.

E chiuderlo nella **forma** che quell'ampiezza impone: una **regola di metodo** su quando `n/a` sia un esito ammissibile, **più una passata** che la applichi a ogni cella. Non venticinque correzioni.

## Context

**Una cella `n/a` dice al lettore successivo di non cercare lì.** È la ragione per cui questo difetto è più grave della manutenzione documentale che sembra: una `n/a` sbagliata non è un'informazione mancante, è **un'istruzione a smettere di cercare**, ed è la forma peggiore della famiglia 2 del censimento — l'impossibilità dichiarata a torto.

**L'ampiezza, accertata.** La valutazione di AGENT-007 del 2026-08-26 ha stabilito che **25 delle 31 celle `n/a`** poggiano sull'argomento del percorso di scrittura — *«non può scrivere, quindi n/a»* — e ne ha falsificate tre. Il denominatore è stato riverificato dal Lead enumerando la tabella: **31 celle `n/a` esatte su 91 celle totali**.

**Il titolo del debito copre la maggioranza dei casi ma non tutti, e la cella più grave è fra le eccezioni.** Su `A-09` × `T-07` il difetto non è la confusione fra falsificazione e perdita: è una **contraddizione piatta dentro lo stesso documento**. La cella (riga 135) dice *«l'insider agisce su parametri e liste, non sul runtime»*; la definizione di `T-07` (riga 112) gli attribuisce *«la distribuzione dei trust anchor e dei binari»*, e TM-36 nomina build, installer e store. **Chi controlla il binario controlla il runtime che quel binario contiene.** Ventitré righe separano l'affermazione dalla sua smentita, nello stesso file.

**Un secondo difetto, mai censito, che questo debito non copriva.** Quattro celle della colonna `T-01` si giustificano col **movente** invece che con la capacità, in violazione del metodo che il documento si impone alla riga 101: *«Ogni attore è descritto per capacità e budget, non per intenzione: la difesa si progetta sul primo, non sulla seconda.»* AGENT-007 non afferma che quelle celle siano false; afferma che **i loro motivi non sono del tipo ammesso**. La distinzione è quella giusta e va conservata: una conclusione corretta raggiunta con un argomento inammissibile va comunque riscritta, **perché è l'argomento che il lettore successivo riuserà**.

**Perché questa spec viene prima delle altre due.** L'uscita per TM-37 è stata sciolta da AGENT-007 fra le tre che il debito lasciava aperte: **attore nuovo `T-08`, compromissione dell'endpoint** — tredici celle nuove più tre `n/a` da riesaminare. È la più cara delle tre uscite, ed è la ragione per cui è quella giusta. L'attaccante di [DEBT-022], che spende con una chiave revocata, **è** un attaccante di endpoint e oggi non ha né cella né attore dove essere registrato: `T-08` è la casella che gli serve. E la contromisura (a) di TM-37 va riscritta prima che [DEBT-017] la tocchi.

**Chi scrive è chi ha censito il difetto, ed è il rischio principale di questa spec.** `A-09` × `T-07` è il difetto già scritto e non guardato nella sua forma più pura, ed è dentro il documento della specialista che lo ha trovato. La valutazione che la spec esegue è **un'autovalutazione**, e per questo la review è del Lead e non sua.

## Scope

### Included

- La **regola di metodo**: quando `n/a` è un esito ammissibile in questa matrice, e quale forma deve avere l'argomento che lo sostiene. Scritta nel documento, non in una spec.
- La **passata** su **tutte e trentuno** le celle `n/a`, non sulle venticinque selezionate.
- La chiusura della contraddizione `A-09` × `T-07`, in un verso o nell'altro, con il verso motivato.
- Le quattro celle di `T-01` giustificate col movente.
- L'attore nuovo `T-08` — compromissione dell'endpoint — con le sue tredici celle e le tre `n/a` da riesaminare, e la riscrittura della collocazione di TM-37.
- L'elenco asset di TM-31, completato con `A-02`.

### Excluded

- Qualunque modifica ai documenti di protocollo. Se la passata scoprisse un difetto **del protocollo** e non della matrice, si apre un debito e **non si corregge qui**: questa spec cambia un documento di analisi, e la disciplina di [ADR-012] non le si applica proprio perché non introduce alcuna regola di validità. Estenderla al protocollo le farebbe scavalcare quella gate.
- [DEBT-022] e [DEBT-017], che hanno le proprie spec e dipendono da questa.
- La ritaratura di qualunque parametro.

## Existing-project analysis

La matrice ha 91 celle, 31 delle quali `n/a`. Il documento si impone alla riga 101 un metodo — capacità e budget, non intenzione — che **non applica a sé stesso in almeno quattro celle**, e contraddice la propria definizione di `T-07` in almeno una.

Il precedente che dà la misura del rischio è nel debito stesso: la cella `A-02` è rimasta `n/a` dal 2026-08-25 fino a quando **una spec di tutt'altro oggetto non l'ha incrociata per caso**, e la via più semplice per falsificarla era già scritta nel documento due sezioni più in là. Non è stata trovata guardando: è stata trovata inciampandoci.

## Technical proposal

**Primo la regola, poi la passata.** La regola di metodo va scritta e collocata nel documento **prima** che una sola cella sia toccata, e ogni cella riesaminata deve **citarla**. L'ordine non è pedanteria: una passata senza regola produce venticinque giudizi indipendenti che nessuno può verificare insieme, e lascia il difetto libero di rientrare dalla prossima cella scritta.

La regola deve pronunciarsi almeno su: cosa distingue **non può fare** da **non guadagna nulla a farlo** (la confusione che il titolo del debito nomina); cosa distingue **falsificazione** da **perdita**; e perché il **movente** non è mai un argomento ammissibile per una cella, coerentemente con la riga 101.

**La passata copre trentuno celle e non venticinque.** La selezione delle venticinque è essa stessa un giudizio, ed è il giudizio che questa spec deve dimostrare, non assumere. Le sei celle escluse vanno guardate e dichiarate escluse **con la regola in mano**.

**Su `A-09` × `T-07` il verso della chiusura va motivato, e un verso è sospetto.** La contraddizione si chiude o falsificando la cella, o restringendo la definizione di `T-07`. **Restringere la definizione per salvare la cella sarebbe la mossa sbagliata** e va nominata qui perché è la più comoda: la definizione è stata scritta per prima e con cura, la cella dopo e in fretta, e sarebbe il documento che si adatta alla propria svista invece del contrario.

## Files and areas involved

- `.lmbrain/knowledge/threat-model.md` — la regola di metodo, la passata, `T-08`, TM-37, TM-31.
- `.lmbrain/knowledge/recurring-defects.md` — **solo se** la passata mostrasse che una delle quattro famiglie va estesa o che ne esiste una quinta. Non altrimenti.

## Acceptance criteria

- [x] Una **regola di metodo** su quando `n/a` è ammissibile è scritta nel documento, e si pronuncia su *non può* contro *non guadagna*, su falsificazione contro perdita, e sull'inammissibilità del movente.
- [x] **Tutte e trentuno** le celle `n/a` sono state risottoposte alla regola, non venticinque. Le celle escluse sono dichiarate escluse con la regola in mano.
- [x] Ogni cella `n/a` sopravvissuta **cita l'argomento** che la sostiene, e quell'argomento è del tipo che la regola ammette.
- [x] La contraddizione `A-09` × `T-07` è chiusa, e il **verso** della chiusura è motivato.
- [x] Le quattro celle di `T-01` giustificate col movente sono riscritte per capacità, oppure dichiarate false.
- [x] `T-08` esiste con le sue tredici celle, le tre `n/a` da riesaminare sono riesaminate, e la collocazione di TM-37 è riscritta.
- [x] L'elenco asset di TM-31 comprende `A-02`.
- [x] Il conteggio finale delle celle e delle `n/a` è riportato, e la differenza rispetto a 91 e 31 è spiegata.

## Implementation plan

1. Scrivere la regola di metodo e collocarla. Nessuna cella si tocca prima.
2. Definire `T-08` e la sua colonna, perché cambia il denominatore della passata.
3. Passata su tutte le celle `n/a`, regola alla mano, una per una.
4. Le due eccezioni che la regola non copre: `A-09` × `T-07` e le quattro di `T-01`.
5. TM-37, TM-31, e il conteggio finale.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-METHOD-BEFORE-PASS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La regola di metodo è scritta **prima** che una cella sia toccata, e ogni cella riesaminata la cita. Una passata senza regola produce giudizi indipendenti che nessuno può verificare insieme, e lascia il difetto libero di rientrare dalla prossima cella scritta.
- [x] GATE-ALL-NA-RESUBMITTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Le celle risottoposte sono **trentuno**, non venticinque, e la trascrizione le conta. La selezione delle venticinque è essa stessa un giudizio, ed è il giudizio che questa spec deve **dimostrare** e non assumere: escludere sei celle sulla base della stessa lettura che ha prodotto il difetto sarebbe l'errore commesso dentro il rimedio.
- [x] GATE-NO-SELF-CONFIRMATION | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Nessuna cella è confermata **col medesimo argomento che l'aveva prodotta**. Chi esegue la passata è chi ha scritto la matrice: una conferma che ripete l'argomento originale non è un riesame, è una rilettura. Per ogni cella confermata la trascrizione mostra l'argomento **nuovo**.
- [x] GATE-A09-DIRECTION | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La contraddizione `A-09` × `T-07` è chiusa e il **verso** è motivato. Se il verso scelto è restringere la definizione di `T-07` invece di falsificare la cella, la motivazione deve reggere l'obiezione che la definizione è stata scritta prima e con cura e la cella dopo e in fretta.
- [x] GATE-T08-COMPLETE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | `T-08` ha tutte e tredici le celle compilate e le tre `n/a` preesistenti riesaminate. Un attore nuovo con celle vuote è peggio di nessun attore: dichiara una copertura che non c'è.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è l'autovalutazione.** Chi esegue la passata è chi ha scritto la matrice, e il difetto più grave che la spec deve chiudere è nel suo documento. Per questo la review è **del Lead** e non di AGENT-007, e per questo esiste `GATE-NO-SELF-CONFIRMATION`. Non è sfiducia: è la stessa ragione per cui [DEBT-013] fu fatto valutare a lei invece che al Lead, e quella scelta corresse due affermazioni del Lead.
- **Il rischio secondario è che la passata scopra un difetto del protocollo e non della matrice.** È un esito probabile — `A-02` era esattamente questo. Va aperto un debito e **non corretto qui**: correggerlo significherebbe cambiare una regola di validità dentro una spec che non ha la gate di [ADR-012].
- **Un rischio terzo, e va nominato perché è invisibile:** una regola di metodo scritta troppo permissiva salva tutte e trentuno le celle e sembra un successo. Se la passata non falsifica **nessuna** cella oltre alle tre già note, la regola va sospettata prima delle celle.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Non scrivere codice.** Questa spec cambia un documento di analisi.
- **Fermarsi e riportare è un esito previsto**, e qui vale in particolare se la regola di metodo, una volta scritta, falsificasse un numero di celle tale da cambiare le conclusioni di una milestone: è una decisione del Lead.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

**Ordine eseguito, ed è l'evidenza di `GATE-METHOD-BEFORE-PASS`.** La regola **R-NA**
è stata scritta in §4 di `threat-model.md` come prima modifica del documento, in un
edit che non tocca alcuna cella (il blocco `### R-NA — Quando n/a è un esito
ammissibile`, cinque condizioni più due corollari di forma più la regola sull'esito).
Solo dopo sono stati aggiunti `T-08` in §3 e la nuova matrice. Ogni `n/a`
sopravvissuta cita `R-NA` per numero di condizione; verificato meccanicamente: tutte
e dieci contengono la stringa `R-NA`.

**R-NA in breve.** Una `n/a` è ammessa solo se: (1) *entrambe* le domande — *può
falsificare l'asset?* e *può causare una perdita su quell'asset come §2 la
definisce?* — hanno risposta negativa, ed entrambe sono scritte; (2) il motivo è di
sola **capacità**, mai di movente, anche quando la conclusione sarebbe vera; (3) la
cella non contraddice la definizione dell'attore in §3 — in caso di conflitto cade la
cella; (4) **monotonia**: se l'attore possiede per definizione le capacità di un
attore più debole, la cella non può essere `n/a` dove quella del più debole porta uno
scenario; (5) l'`n/a` poggia su una regola citabile e **non sull'assenza di un attore
dal modello**. Più: ogni `n/a` cita la regola che la tiene, e nessuna `n/a` è un
superlativo. Più la regola sull'esito: una cella che contraddice un'altra parte del
documento **si falsifica**, non si restringe la parte contraddetta.

**Passata su trentuno celle — `GATE-ALL-NA-RESUBMITTED`.** Le trentuno enumerate:
`A-01`×`T-02`,`T-05`,`T-06`; `A-03`×`T-05`; `A-04`×`T-01`,`T-03`,`T-05`,`T-06`;
`A-05`×`T-01`,`T-05`; `A-06`×`T-05`; `A-07`×`T-01`,`T-03`,`T-05`;
`A-08`×`T-01`,`T-03`,`T-06`; `A-09`×`T-01`,`T-02`,`T-03`,`T-04`,`T-06`,`T-07`;
`A-10`×`T-01`; `A-11`×`T-01`; `A-12`×`T-01`,`T-05`;
`A-13`×`T-01`,`T-02`,`T-03`,`T-06`. Le sei che il debito escludeva sono state guardate
con la regola in mano e **non sono state assunte**: due di esse (`A-08`×`T-06`, la
cella su cui la valutazione dichiarava di non pronunciarsi, e `A-11`×`T-01`) sono
cadute, e questa è la dimostrazione che la selezione delle venticinque era essa stessa
un giudizio da non fidarsi.

**Esito: 21 falsificate, 10 confermate.**

*Falsificate (21):* `A-01`×`T-02`,`T-05`,`T-06` e `A-03`×`T-05` (R-NA.4: ogni attore
che §3 definisce con almeno un'identità enrollata possiede le capacità di `T-01`,
quindi TM-04); `A-04`×`T-03` (TM-13: chi non emette challenge verso una vittima ne
abbassa il `contribution_score` e la rende ineleggibile — *vero su chi entra, falso su
chi resta fuori*); `A-04`×`T-06` (TM-31, già nota); `A-05`×`T-05`, `A-06`×`T-05`,
`A-12`×`T-05` (TM-24: l'host saturato serve dati vecchi, fallisce le proprie challenge,
e le poche macchine capienti occupate escludono le altre app dall'hosting);
`A-08`×`T-01`, `A-10`×`T-01`, `A-12`×`T-01` (TM-04, metà «assenza di fee»: senza fee
e senza limite di ammissione per account una sola identità impone I/O a tutti e la
congestione è esclusione); `A-08`×`T-03` (TM-13: l'emittente sceglie `deadline_ms` e
`response_bytes`, cioè batteria e dati su un nodo mobile); `A-08`×`T-06` (TM-28: il
peer che si connette al maggior numero possibile impone connessioni e gossip su ogni
vittima — **è la cella su cui la valutazione non si era pronunciata, ed è decisa qui
per capacità e non per confine fra asset**); `A-09`×`T-03` e `A-09`×`T-04` (TM-42
nuovo: `ComputeAssignment` porta `input` scelto verbatim dall'emittente, quindi il
ruolo di validatore *è* un percorso verso il runtime di un host);
`A-09`×`T-07` (TM-36, già nota); `A-11`×`T-01` (TM-29: «non serve alcun attacco, è
lettura» — la cella diceva «solo dati pubblici», che è il **meccanismo** dello
scenario, non la sua esclusione); `A-13`×`T-02` (TM-40 nuovo: cattura delle repliche
di hosting e diniego di consegna); `A-13`×`T-03` (TM-15: un record che non entra in un
blocco non è un record alterato, è un record che non esiste); `A-13`×`T-06` (TM-31,
già nota).

*Confermate (10), ognuna con l'argomento nuovo — `GATE-NO-SELF-CONFIRMATION`:*

| Cella | Argomento originale | Argomento **nuovo** con cui è confermata |
| --- | --- | --- |
| `A-04`×`T-01` | «un singolo nodo egoista non altera la composizione del set» | Lato perdita (R-NA.1b), che l'originale non trattava: la perdita che §2 assegna ad `A-04` è la *cattura da parte di pochi*, non raggiungibile da uno. **Più un confine dichiarato:** che un `T-01` superi il pavimento di eleggibilità con la sola presenza di TM-01 è un difetto del *criterio*, contato su `A-06` |
| `A-04`×`T-05` | «nessun percorso dalla pubblicazione all'elezione» | Lato perdita: TM-24 esiste ma **non è dirigibile** su un candidato scelto, perché [ADR-006] ha tolto al publisher la scelta degli host. Confine: cade se una politica di deployment gli desse influenza sull'assegnazione |
| `A-05`×`T-01` | *movente* — «non ha vantaggio a falsificare» | Capacità, su entrambi i lati: non può forgiare una prova Merkle contro un'intestazione finalizzata; e la sola leva è il rifiuto di rispondere, che è una perdita solo quando i peer sono tutti suoi — cioè TM-10, che richiede la massa di `T-02` |
| `A-07`×`T-01` | *movente* — «abusa della propria identità, non ne attacca altre» | Capacità: `identity.md` §"Key hierarchy" impedisce di produrre una firma altrui; la sola leva su un'identità altrui è la correlazione in sessione, che è la capacità con cui §3 definisce `T-06` |
| `A-07`×`T-03` | «le chiavi non transitano mai» | Lato perdita: la leva del validatore è TM-13, la cui perdita cade su `A-06` e `A-12` e non arriva al possesso dell'identità. **E R-NA.5:** la cella regge ora contro `T-08` *dichiarato*, non più sull'assenza di quell'attore |
| `A-07`×`T-05` | «nessun percorso verso chiavi altrui» | Il residuo è un'**evasione** dalla sandbox, che ha una cella propria a `A-09`×`T-05`; un'evasione riuscita converte l'attore in `T-08`. È una *precondizione con cella propria*, non una perdita contata altrove |
| `A-09`×`T-01` | «l'egoista non pubblica moduli» | Lato perdita: il solo confine che indebolisce è quello del proprio host, e §2 definisce la perdita su `A-09` come la compromissione della macchina di *chi non ha scelto il codice* |
| `A-09`×`T-02` | «moltiplicare identità non aggiunge percorso» | Il confine della sandbox è una proprietà **locale** di ogni runtime, non una risorsa condivisa che una maggioranza di identità sposta: moltiplicare identità moltiplica le macchine che l'attaccante **possiede** |
| `A-09`×`T-06` | «nessun percorso verso il runtime» | Il **diniego di consegna** che falsifica `A-13`×`T-06` non arriva qui: un modulo non consegnato è un modulo non eseguito, e un confine si viola eseguendo |
| `A-13`×`T-01` | «nessun percorso di scrittura sul catalogo» | Lato perdita: la perdita su `A-13` è di *distribuzione*, e negarla richiede una quota del set di repliche tale che la riassegnazione di `app-manifest.md` non trovi sostituti — massa di identità, cioè `T-02` |

**`A-09` × `T-07` — verso della chiusura e motivazione (`GATE-A09-DIRECTION`).**
È stata **falsificata la cella**, non ristretta la definizione di `T-07`. Motivazione
scritta per esteso nella nota sotto TM-36, e regge l'obiezione in tre passi: (1) la
definizione è scritta prima e con cura, e non è isolata — TM-36 *esiste* perché la
distribuzione degli ancoraggi è nelle mani di `T-07`, quindi restringerla lascerebbe
TM-36 senza attore, che è lo stesso buco che TM-37 aveva sotto `T-06`; (2) è la regola
di esito di R-NA; (3) **e la ragione che chiude la questione da sola: la cella cade
anche sotto la lettura ristretta**, perché la politica di accettazione dell'host di
[RF-015] e i tetti di deployment di TM-24(a) *sono parametri firmati*, e TM-23 porta
`A-09` fra i propri asset da quando è stato scritto. Non esiste restrizione di `T-07`
che salvi la cella e lasci in piedi il resto del documento.

**`T-08` (`GATE-T08-COMPLETE`).** Attore definito in §3 con capacità, budget e
**confine esplicito** (un endpoint per volta; quando diventa campagna la capacità che
conta è la massa di identità, cioè `T-02`). Tutte e tredici le celle compilate,
**nessuna `n/a`**, e la ragione è dichiarata sotto la matrice: per R-NA.4 `T-08`
eredita ogni capacità della vittima e vi aggiunge il controllo locale, quindi una
`n/a` dovrebbe nominare qualcosa che la vittima non può fare ai propri asset. Le tre
`n/a` preesistenti che reggevano sull'assenza di questo attore sono state riesaminate:
`A-07`×`T-03` e `A-07`×`T-05` confermate **con il confine scritto nella cella**, e la
nota delle «proprietà di design conquistate» riscritta. Nuova sezione §5.8 con TM-37
ricollocato e TM-41 nuovo; nota di ricollocazione lasciata in §5.6 al posto di TM-37.

**Conteggio finale.** **104 celle (13 asset × 8 attori), 94 coperte, 10 `n/a`.** Prima:
91, 60, 31. Differenza: **+13** celle di `T-08`, **−21** `n/a` falsificate. Verificato
meccanicamente insieme a: ogni `TM-xx` citato in matrice esiste come scenario (42/42);
ogni scenario definito compare in almeno una cella (42/42, TM-39 incluso, che non era
mai stato collocato); ogni elenco asset di scenario coincide **esattamente** con le
righe in cui lo scenario compare; nessuna `n/a` senza `R-NA`; nessuna delle cinque
formule di movente sopravvissuta in una cella.

**Correzione della premessa di TM-38 dopo [SPEC-016].** La derivazione di
`reward_epoch` da `height` rende **falsa** la lettura «il pericolo sta verso il basso»:
l'emissione è ora limitata *per blocco* e non per unità di tempo reale, quindi
l'accelerazione della produzione dei blocchi moltiplica l'emissione. Nota datata
aggiunta a TM-38, con la verifica — **enumerata** — che **nessuna cella `n/a` poggiava
su quell'assunzione**: la riga `A-02` non ha `n/a`, e i soli attori capaci di
accelerare sono `T-03` e `T-04`, già coperti da TM-14 e TM-20. A rimanere indietro era
la prosa, non la griglia.

**Fuori scopo, rispettato.** Nessun file di `docs/protocol/`, `core/`, `sim/`,
`SECURITY.md` o `.lmbrain/debts/` è stato toccato. TM-42 registra uno scenario la cui
contromisura toccherebbe `wire.md`: è scritto **come scenario** con una nota di
perimetro esplicita, e la modifica è materia di un debito del Lead.

### Files changed

- `.lmbrain/knowledge/threat-model.md` — unico file modificato. §3 (`T-08`), §4
  (R-NA, matrice, conteggio, nota sulle proprietà conquistate), §5.2 (TM-40), §5.3
  (TM-42), §5.4 (nota su TM-38), §5.6 (asset di TM-31, nota di ricollocazione di
  TM-37), §5.7 (nota sul verso di `A-09`×`T-07` sotto TM-36), §5.8 nuova (`T-08`,
  TM-37 ricollocato, TM-41), elenchi asset di TM-04, TM-08, TM-13, TM-15, TM-19,
  TM-24, TM-28, TM-31, TM-36, TM-37, §14.
- `.lmbrain/knowledge/recurring-defects.md` — **non modificato**, e la spec lo
  ammetteva solo a condizione. La condizione non si è verificata: la passata non ha
  mostrato una quinta famiglia né richiesto di estenderne una. Il difetto chiuso qui è
  la **famiglia 2** nella sua forma peggiore già censita — l'impossibilità dichiarata
  a torto — e le tre falsificazioni principali sono tre conferme in più del tratto
  comune già scritto in quella pagina, non un tratto nuovo.

### Verification transcript

Le cinque gate sono `kind=manual`, quindi l'evidenza è la trascrizione qui sopra. Va
però detto che **una parte di questa spec è meccanizzabile e non era ovvio che lo
fosse**: la *coerenza* fra la matrice e gli scenari — non la correttezza semantica di
una cella, che [ADR-012] dichiara fuori dalla propria portata e che
`recurring-defects.md` classifica non meccanizzabile. Lo script sotto controlla ciò
che è controllabile, ed è la ragione per cui sono state trovate **sei** disallineamenti
preesistenti fra elenco asset di uno scenario e righe in cui lo scenario compare, più
uno scenario (TM-39) mai collocato in matrice da quando è stato scritto.

Script: `scratchpad/check.py` (13 righe, 8 colonne attese; ogni `n/a` deve contenere
`R-NA`; nessuna delle cinque formule di movente ammessa in una cella; ogni `TM-xx`
citato deve esistere; ogni scenario definito deve comparire; l'elenco asset dichiarato
di ogni scenario deve coincidere **esattamente** con l'insieme delle righe in cui
compare).

```text
$ python scratchpad/check.py
righe matrice: 13
celle: 104 coperte: 94 n/a: 10
scenari definiti: 42
citati in matrice ma non definiti: []
definiti ma non in matrice: []
```

Prima delle correzioni di allineamento la stessa esecuzione dava:

```text
$ python scratchpad/check.py
righe matrice: 13
celle: 104 coperte: 94 n/a: 10
scenari definiti: 42
citati in matrice ma non definiti: []
definiti ma non in matrice: ['TM-39']
DISALLINEATO TM-01 dichiarati-non-collocati: ['A-06'] collocati-non-dichiarati: []
DISALLINEATO TM-03 dichiarati-non-collocati: ['A-06'] collocati-non-dichiarati: []
DISALLINEATO TM-08 dichiarati-non-collocati: ['A-03'] collocati-non-dichiarati: ['A-06']
DISALLINEATO TM-27 dichiarati-non-collocati: ['A-02'] collocati-non-dichiarati: []
DISALLINEATO TM-32 dichiarati-non-collocati: ['A-05'] collocati-non-dichiarati: []
DISALLINEATO TM-37 dichiarati-non-collocati: [] collocati-non-dichiarati: ['A-07']
DISALLINEATO TM-39 dichiarati-non-collocati: ['A-04', 'A-05'] collocati-non-dichiarati: []
DISALLINEATO TM-42 dichiarati-non-collocati: ['A-08'] collocati-non-dichiarati: []
```

Sei di quegli otto erano **preesistenti** e nessuno li aveva visti: uno scenario che
dichiara di colpire `A-06` e non compare nella riga `A-06` è la matrice che promette
una copertura che la griglia non porta, cioè un buco nell'evidenza di `GATE-COVERAGE`.
Due erano miei, introdotti in questa stessa passata (TM-37 e TM-42), e sono la prova
che il controllo serviva anche a chi lo scriveva. Tutti e otto chiusi.

Assenza di modifiche fuori perimetro:

```text
$ git status --porcelain
 M .lmbrain/knowledge/threat-model.md
 D .lmbrain/specs/backlog/SPEC-017-il-legame-di-catena-dove-oggi-e-ambiguo-o-assente.md
 D .lmbrain/specs/backlog/SPEC-018-quando-n-a-e-un-esito-ammissibile-la-regola-di-metodo-e-la-passata-sulla-matrice.md
?? .lmbrain/specs/working/SPEC-017-il-legame-di-catena-dove-oggi-e-ambiguo-o-assente.md
?? .lmbrain/specs/working/SPEC-018-quando-n-a-e-un-esito-ammissibile-la-regola-di-metodo-e-la-passata-sulla-matrice.md
```

Le righe di SPEC-017 sono di un altro agente che lavora in parallelo sullo stesso
albero. L'unico file modificato da questa spec è `threat-model.md`. Nessun `git commit`,
`git push`, `git add -A` o `git add .` è stato eseguito.

**Raccomandazione al Lead, e non è meccanizzabile:** lo script sopra vive nello
scratchpad e sparirà. Se il controllo di coerenza fra matrice ed elenchi asset deve
sopravvivere a questa spec — e sei disallineamenti preesistenti dicono di sì — va
versionato, ed è una decisione del Lead perché tocca la strumentazione di [ADR-012].

---

## Remediation — 2026-08-26, su REVIEW del Lead (RF-001, RF-002)

La spec resta in `review`. Nessun ritorno a `working`.

### RF-001 — scelto: **la cella cade**

`A-09` × `T-01` è falsificata e acquista uno scenario, **TM-43**. Non ho aggiunto il
qualificatore a §2, e la ragione è che l'altro verso sarebbe stato **la mossa che
questa spec vieta, commessa una seconda volta e con l'asset invece che con l'attore**.
La regola sull'esito di `R-NA` dice che una cella che contraddice un'altra parte del
documento si falsifica, e non si restringe la parte contraddetta; §2 è scritto una
volta e per l'asset intero, la cella l'ha citato ampliandolo perché ne aveva bisogno.
Emendare §2 per salvare la cella sarebbe stato `A-09` × `T-07` di nuovo, dal lato
comodo.

**E il merito conferma la forma.** Tolto il qualificatore, la frase con cui la cella si
difendeva — *chi disattiva il proprio confine ha scelto* — è l'attacco. La macchina di
un `T-01` che allenta il contenitore custodisce chunk di storage di terzi, tiene la
chiave di identità del nodo, ed è un peer autenticato: la sua compromissione **non è
confinata a chi ha scelto**, e produce un `T-08` senza che nessuno abbia pagato il
costo di `T-08`. La conformità al contenitore è verificabile sugli **output** (TM-03) e
non sul modo dell'esecuzione, quindi nulla la rileva.

**Cascata dichiarata, e non l'avevi chiesta.** `R-NA.4` è mia e vale contro di me:
indebolire il proprio contenitore è una capacità di chiunque faccia girare un nodo,
quindi **`A-09` × `T-02` e `A-09` × `T-06` cadono anch'esse** ed ereditano TM-43.
`A-09` × `T-06` cade per il **peer enrollato** della definizione di `T-06`, non per la
via di percorso: quella resta chiusa — un modulo non consegnato è un modulo non
eseguito — e la cella lo dice ancora, perché era l'argomento giusto per la domanda
sbagliata.

*Come è stato trovato, e vale più della correzione.* L'argomento con cui avevo
confermato `A-09` × `T-02` — *moltiplicare identità moltiplica gli host che
l'attaccante **possiede*** — **conteneva la propria confutazione**: se possedere l'host
basta a rinunciare al confine, possederne molti è l'attacco. Il difetto era già scritto
nella cella, e non lo stavo guardando. È il tratto comune di [[recurring-defects]]
applicato a chi scriveva il rimedio, terza volta in questa spec.

**Esito sul modello, ed è la parte che conta più del conteggio:** la riga `A-09` ha ora
**zero `n/a`**. `A-09` non ha alcuna difesa indipendente dall'host — l'isolamento è
tenuto dal software che l'host esegue, e l'host può cambiarlo — e sei celle `n/a`
dicevano il contrario. Registrato in TM-43 e nella nota sotto la matrice.

### RF-002 — la riga aggiunta

Aggiunta dentro `R-NA.1`, senza toccare §2:

> **Si legge così: quando la colonna porta un caso peggiore, la domanda (b) si risolve
> contro la *classe* di perdita che quel caso esemplifica, non contro il caso.** Una
> perdita minore ma della stessa classe è una perdita, e una cella `n/a` che si
> appoggia a una riga di quel tipo **deve dichiarare quale classe sta negando**.

Le due celle di `A-04` — le sole `n/a` superstiti su una riga di caso peggiore — sono
state riscritte per dichiarare la classe negata: *l'alterazione della composizione da
parte di chi non dovrebbe comporla*. Entrambe reggono, e per la ragione che avevi
indicato: l'argomento portante è `(a)`, di capacità.

**Ho aggiunto una seconda riga che non avevi chiesto**, perché RF-001 è la prova che
serve, e senza di essa la regola resta aperta esattamente dove è stata aggirata: il
caso simmetrico di `R-NA.3` sull'**asset**. Una cella non può citare §2 aggiungendo né
togliendo un qualificatore alla perdita. È la mossa più difficile da vedere, perché la
cella *cita* §2 mentre lo sta cambiando.

### Conteggio dopo la remediation

**104 celle, 97 coperte, 7 `n/a`.** Erano 104 / 94 / 10. Sulle trentuno risottoposte:
**24 falsificate, 7 confermate**. Le sette superstiti: `A-04`×`T-01`, `A-04`×`T-05`,
`A-05`×`T-01`, `A-07`×`T-01`, `A-07`×`T-03`, `A-07`×`T-05`, `A-13`×`T-01`.

### Lo script, versionato e provato in negativo

`sim/tools/threat_model_matrix_coherence.py`, senza cablaggio in CI, come chiesto. Sette
controlli C1–C7, e l'intestazione dichiara **ciò che non fa**: non giudica mai la
correttezza semantica di una cella, che [ADR-012] esclude dalla propria portata e che
`recurring-defects.md` classifica non meccanizzabile. Verifica la coerenza fra **due
copie dello stesso fatto** — gli asset che uno scenario dichiara e le righe in cui la
matrice lo colloca.

```text
$ python sim/tools/threat_model_matrix_coherence.py
celle: 104  coperte: 97  n/a: 7  scenari: 43
OK: matrice e scenari coerenti
exit=0
```

**Prova in negativo**, eseguita e non trascritta: reintrodotte due delle occorrenze
reali — la vecchia `A-09`×`T-01` con un motivo di movente, e l'elenco asset di TM-08
riportato a com'era prima di questa spec — la guardia le trova entrambe e distingue le
tre cause.

```text
$ python sim/tools/threat_model_matrix_coherence.py
celle: 104  coperte: 96  n/a: 8  scenari: 43
FAIL C6: A-09 x T-01 e` n/a e non cita R-NA
FAIL C7: A-09 x T-01 argomenta dal movente: «non ha vantaggio»
FAIL C5: TM-08 dichiara ['A-02', 'A-03'] e compare in ['A-02', 'A-03', 'A-06']
exit=1
```

Documento ripristinato subito dopo e riverificato verde. Nessun `git commit`, nessun
`git add`. `.lmbrain/debts/` non toccato: DEBT-024 e DEBT-025 sono comparsi in albero
mentre lavoravo e non sono opera mia.
