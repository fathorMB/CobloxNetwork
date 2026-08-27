---
id: SPEC-026
# Note: Quote the title if it contains a colon
title: "Tre discipline invisibili agli strumenti: il ciclo di vita delle review, la provenienza degli argomenti nelle probe, e le liste dichiarate nel crate"
status: backlog
kind: feature
priority: high
area: tooling
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-008
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012]
links: []
created: 2026-08-27
updated: 2026-08-27
tags: [conformance, ci, governance]
activity:
  - date: 2026-08-27
    action: "created"
---

# Tre discipline invisibili agli strumenti

## Objective

Rendere eseguibili tre discipline che oggi esistono come prosa. Le prime due il
Project Lead le ha violate entrambe nella stessa giornata. Nessuna delle tre e' vista da
`lmbrain_validate`, da `spec_done` o da alcuna gate: sono visibili solo a un
essere umano che guardi la board o legga una probe, che e' il posto peggiore per
un difetto.

## Context

**Difetto A, rilevato dall'operatore due volte.** Il 2026-08-25 l'operatore ha
notato dalla board tre review ferme in `changes-requested` mentre [SPEC-001] era
`done`. Ne e' nato `knowledge/review-lifecycle-discipline.md`, che ne scrive la
causa e prescrive il controllo da eseguire prima di chiudere una spec, con il
comando. Il 2026-08-27 l'operatore ha rilevato **lo stesso difetto** su
[SPEC-023]: [REVIEW-038], [REVIEW-040] e [REVIEW-041] ferme in
`changes-requested` su una spec chiusa un'ora prima, piu' [REVIEW-042] superata
da [REVIEW-044]. Il Lead non aveva eseguito il comando che era gia' scritto.

La causa e' sempre la stessa: durante le remediation si crea una review nuova a
ogni giro invece di ri-esprimere il verdetto su quella esistente, e ogni review a
monte resta con l'ultimo verdetto emesso.

**Difetto B, rilevato da [REVIEW-045] RF-008.** Il Lead aveva scritto in
[ADR-017] la regola *«nessun argomento diventa normativo prima di essere
attaccato»*. La review ha stabilito che non e' decidibile, non ha proprietario,
vive in un ADR che nessuno strumento legge, ed e' scritta dalla parte che deve
rispettarla. Ne ha proposto la forma verificabile che questa spec attua.

Il caso reale: la probe `revocation-grace-floor-is-one-rotation-of-the-minimum-set`
pinnava un'affermazione di sicurezza **falsa**, e `published_artifacts.py` era
verde su di essa. La probe teneva ferma la frase invece di verificarla, ed e' la
terza occorrenza della stessa forma nella sola [SPEC-022].

**Difetto C, [DEBT-031], aggiunto il 2026-08-27.** La passata di [ADR-012] legge
`docs/protocol/`, `sim/tools/`, i test, la guida, `recurring-defects.md` e
`SECURITY.md`. **Non legge `core/coblox-core/src/`.** Ma i commenti di modulo
fanno affermazioni della stessa natura: `lib.rs` dichiara che i parametri sono
configurazione validata e poi **enumera** i portatori. Il repository e' pubblico e
`cargo doc` le rende, e nessuna probe le tiene.

Il criterio, che il debito ha derivato e l'operatore ha adottato: **un'affermazione
che enumera e' una lista dichiarata**, ed e' la forma che si rompe in silenzio.
Quelle vanno tenute, e nel verso giusto — non pinnando il testo, ma verificando
che l'insieme dichiarato coincida con quello osservabile dal codice, come
`C6-ORPHAN` e `C11-CLAIMDOC` gia' fanno altrove. Il danno e' proprio di questa
superficie: la documentazione di modulo e' cio' che il prossimo implementatore
legge per sapere dove mettere una cosa nuova, quindi una lista incompleta non
produce un digest sbagliato — produce **un membro che non viene aggiunto**.

## Scope

### Included

- Un controllo che fallisce quando una spec in `specs/done/` ha una review che la
  nomina in `pending`, `changes-requested` o `blocked`.
- Un controllo che fallisce quando una `[[probe]]` di
  `sim/tools/published_artifacts.toml` porta nel proprio `why` un argomento di
  sicurezza e **non nomina l'ID della review che lo ha attaccato**.
- Un controllo che sorveglia le **affermazioni che enumerano** nei commenti di
  modulo di `core/coblox-core/src/`, aggiunto il 2026-08-27 su decisione
  dell'operatore per chiudere [DEBT-031]. Vedi *Difetto C* sotto.
- L'esecuzione di tutti e tre in CI, nel job che gia' ospita i controlli sui
  documenti.
- La passata su `published_artifacts.toml` per portare le probe esistenti alla
  forma nuova, o per dichiarare quali non portano argomenti di sicurezza.

### Excluded

- Modifiche a `.lmbrain/` che non siano artefatti di questa spec: il kit e' di
  proprieta' dell'applicazione e non si tocca da qui.
- Qualunque cambiamento a `lmbrain_validate` o ai verbi MCP, che sono kit-owned.
- Il risolutore delle citazioni, che e' [SPEC-024] e resta separato.

## Technical proposal

Tre controlli nella famiglia di `sim/tools/`, accanto a `published_artifacts.py`
e `lead_claims_check.py`, che sono il precedente per forma e per collocazione.

**Controllo A.** Enumera `specs/done/`, enumera `reviews/pending`,
`reviews/changes-requested`, `reviews/blocked`, e fallisce nominando ogni coppia
in cui una review non terminale punta a una spec chiusa. Il criterio di
appartenenza e' il campo `spec:` del frontmatter della review.

**Controllo C.** Per ogni affermazione che **enumera** in un commento di modulo
di `core/coblox-core/src/`, confronta l'insieme dichiarato con quello osservabile
dal codice e fallisce nominando il membro mancante. Il debito avverte che la
classe giusta e' **probabilmente una terza** e non un'estensione delle classi
esistenti: aggiungere `src/` a quelle di scoperta produrrebbe falsi positivi il
primo giorno, ed e' il modo in cui `SECURITY.md` ha dovuto avere una classe
propria.

**Controllo B.** Per ogni `[[probe]]` il cui `why` contenga un argomento di
sicurezza, richiede un riferimento `REVIEW-nnn` che lo abbia attaccato. La
definizione di *argomento di sicurezza* e' il punto delicato e va decisa
dall'implementatore con una regola dichiarata e falsificabile, non con un elenco
di parole chiave che chiunque puo' aggirare riformulando. La regola scelta va
scritta accanto al controllo.

## Acceptance criteria

- [ ] Il controllo A e' stato **osservato fallire** sullo stato reale del
      2026-08-27, ricostruito: [SPEC-023] `done` con [REVIEW-038], [REVIEW-040] e
      [REVIEW-041] non terminali. La trascrizione mostra il fallimento e nomina
      le tre coppie.
- [ ] Il controllo A e' verde sull'albero corrente, dove quelle review sono
      state superate.
- [ ] Il controllo B e' stato **osservato fallire** sulla probe
      `revocation-grace-floor-is-one-rotation-of-the-minimum-set` nella forma in
      cui e' entrata, cioe' senza riferimento a una review. E' il caso reale che
      motiva il controllo, e un controllo che non lo coglie non serve.
- [ ] La regola che definisce *argomento di sicurezza* e' scritta accanto al
      controllo, ed e' **falsificabile**: la trascrizione mostra almeno un `why`
      che la regola classifica come non-sicurezza e la ragione.
- [ ] Il controllo C e' stato **osservato fallire** su una lista dichiarata resa
      incompleta ad arte in un commento di modulo di `core/coblox-core/src/`, e
      nomina il membro mancante invece di segnalare genericamente il file.
- [ ] Il controllo C verifica **nel verso giusto**: confronta l'insieme
      dichiarato con quello osservabile dal codice, e non pinna il testo. La
      trascrizione mostra che una riformulazione della frase che **conservi**
      l'insieme non lo fa fallire.
- [ ] La classe di scoperta del controllo C e' **propria** e non un'estensione
      delle classi esistenti, e la trascrizione dichiara quanti falsi positivi
      produrrebbe l'alternativa scartata su `src/`.
- [ ] Tutti e tre i controlli girano in CI e la pipeline e' verde.
- [ ] Nessuna probe esistente e' stata cancellata per far passare il controllo B:
      quelle senza riferimento sono state corrette o dichiarate, e la
      trascrizione elenca quale delle due per ciascuna.

## Verification gates

- [ ] GATE-SEEN-IT-FAIL-FIRST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Entrambi i controlli sono stati eseguiti **prima** della correzione e la trascrizione mostra che nominavano i casi reali. Una gate che nasce verde non ha mai dimostrato di vedere.
- [ ] GATE-NEGATIVE-PROOF | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Ogni classe di difetto e' stata osservata fallire su un albero mutato, una mutazione per classe.
- [ ] GATE-NO-KIT-WRITES | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il diff non tocca `.lmbrain/` fuori dagli artefatti di questa spec. Verificabile guardando il diff.
- [ ] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=transcript | La pipeline reale e' verde su tutti i job, con numero di run e commit.

## Instructions for the assigned specialist

- La spec e' in `backlog`. Se passa a `ready`, esegui `spec_start` come prima
  azione e `spec_submit` a implementazione completa.
- Implementa solo lo scope dichiarato.
- Codice di produzione, niente segnaposto. Ogni regola nuova va **osservata
  fallire**.
- Non toccare `.lmbrain/` fuori dagli artefatti di questa spec, e non fare commit
  ne' push: il push su `main` e' del Lead.
- Se trovi un difetto fuori perimetro, **aprilo come rilievo e non correggerlo**.
  Precedenti nella stessa sessione: [DEBT-041] e [DEBT-045].

## Implementation evidence

> Compilata dallo specialista a lavoro concluso.

### Changes made

### Files changed

### Verification performed

### Verification transcript
