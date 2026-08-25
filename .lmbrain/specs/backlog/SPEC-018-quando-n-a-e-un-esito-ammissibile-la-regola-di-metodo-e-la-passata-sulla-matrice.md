---
id: SPEC-018
# Note: Quote the title if it contains a colon
title: "Quando n/a e un esito ammissibile: la regola di metodo e la passata sulla matrice"
status: backlog
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

- [ ] Una **regola di metodo** su quando `n/a` è ammissibile è scritta nel documento, e si pronuncia su *non può* contro *non guadagna*, su falsificazione contro perdita, e sull'inammissibilità del movente.
- [ ] **Tutte e trentuno** le celle `n/a` sono state risottoposte alla regola, non venticinque. Le celle escluse sono dichiarate escluse con la regola in mano.
- [ ] Ogni cella `n/a` sopravvissuta **cita l'argomento** che la sostiene, e quell'argomento è del tipo che la regola ammette.
- [ ] La contraddizione `A-09` × `T-07` è chiusa, e il **verso** della chiusura è motivato.
- [ ] Le quattro celle di `T-01` giustificate col movente sono riscritte per capacità, oppure dichiarate false.
- [ ] `T-08` esiste con le sue tredici celle, le tre `n/a` da riesaminare sono riesaminate, e la collocazione di TM-37 è riscritta.
- [ ] L'elenco asset di TM-31 comprende `A-02`.
- [ ] Il conteggio finale delle celle e delle `n/a` è riportato, e la differenza rispetto a 91 e 31 è spiegata.

## Implementation plan

1. Scrivere la regola di metodo e collocarla. Nessuna cella si tocca prima.
2. Definire `T-08` e la sua colonna, perché cambia il denominatore della passata.
3. Passata su tutte le celle `n/a`, regola alla mano, una per una.
4. Le due eccezioni che la regola non copre: `A-09` × `T-07` e le quattro di `T-01`.
5. TM-37, TM-31, e il conteggio finale.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-METHOD-BEFORE-PASS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La regola di metodo è scritta **prima** che una cella sia toccata, e ogni cella riesaminata la cita. Una passata senza regola produce giudizi indipendenti che nessuno può verificare insieme, e lascia il difetto libero di rientrare dalla prossima cella scritta.
- [ ] GATE-ALL-NA-RESUBMITTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Le celle risottoposte sono **trentuno**, non venticinque, e la trascrizione le conta. La selezione delle venticinque è essa stessa un giudizio, ed è il giudizio che questa spec deve **dimostrare** e non assumere: escludere sei celle sulla base della stessa lettura che ha prodotto il difetto sarebbe l'errore commesso dentro il rimedio.
- [ ] GATE-NO-SELF-CONFIRMATION | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Nessuna cella è confermata **col medesimo argomento che l'aveva prodotta**. Chi esegue la passata è chi ha scritto la matrice: una conferma che ripete l'argomento originale non è un riesame, è una rilettura. Per ogni cella confermata la trascrizione mostra l'argomento **nuovo**.
- [ ] GATE-A09-DIRECTION | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La contraddizione `A-09` × `T-07` è chiusa e il **verso** è motivato. Se il verso scelto è restringere la definizione di `T-07` invece di falsificare la cella, la motivazione deve reggere l'obiezione che la definizione è stata scritta prima e con cura e la cella dopo e in fretta.
- [ ] GATE-T08-COMPLETE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | `T-08` ha tutte e tredici le celle compilate e le tre `n/a` preesistenti riesaminate. Un attore nuovo con celle vuote è peggio di nessun attore: dichiara una copertura che non c'è.

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

### Files changed
