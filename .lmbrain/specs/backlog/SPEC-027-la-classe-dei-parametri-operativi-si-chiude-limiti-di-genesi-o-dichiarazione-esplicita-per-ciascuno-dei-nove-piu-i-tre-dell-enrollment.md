---
id: SPEC-027
# Note: Quote the title if it contains a colon
title: "La classe dei parametri operativi si chiude: limiti di genesi o dichiarazione esplicita, per ciascuno dei nove piu' i tre dell'enrollment"
status: backlog
kind: feature
priority: high
area: governance
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: sol
thinking_level: maximum
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-010, ADR-012, ADR-017]
links: []
created: 2026-08-27
updated: 2026-08-27
tags: [conformance, economy]
activity:
  - date: 2026-08-27
    action: "created"
  - date: 2026-08-27
    action: "set tags"
---
# La classe dei parametri operativi si chiude

## Objective

Chiudere la classe che [DEBT-044] descrive: ogni parametro operativo ha un limite
di magnitudine ancorato nell'ancora di fiducia di genesi, **oppure** e' dichiarato
aperto nella lista DRAFT dei parametri di lancio con la ragione per cui resta
aperto. Nessuno resta ne' deciso ne' registrato come da decidere.

Questa spec e' anche il contenitore della **taratura** che l'operatore ha
rimandato il 2026-08-27 in attesa dell'analisi di [SPEC-023]: i valori si fissano
qui, sull'analisi, e non prima.

## Context

`ConsensusParametersBody` ha venti campi. I dieci di elezione hanno un limite di
genesi **e** sono dichiarati aperti. **Nove** degli altri dieci non hanno ne'
l'uno ne' l'altra — [DEBT-044] — e valgono tutti `1` in albero, che e' il valore
delle fixture. Il quorum sedente li firma senza che alcun limite lo trattenga.

Il decimo, `min_revocation_effective_delay_blocks`, e' uscito dalla classe il
2026-08-27 perche' [SPEC-022] ha attuato la parte 2 di [ADR-017]. E' il motivo per
cui [DEBT-036] e' stato superseduto da [DEBT-044]: la classe si e' ristretta per
una consegna, non per una cifra cambiata.

Tre campi di `EnrollmentParametersBody` hanno la stessa forma — [DEBT-037] — e
`election_epoch` dipende da un parametro governato senza che alcun documento dica
**quale** documento governi un'epoca — [DEBT-028].

**L'analisi esiste ed e' rivista.** Il documento
`knowledge/analisi-dieci-parametri-operativi-consensus.md` e' passato da quattro
esecuzioni di `GATE-SECREVIEW` ed e' stato accettato con [REVIEW-043]. Porta pero'
due avvertenze che vincolano questa spec: e' utilizzabile come **base** e non
copiabile come **testo**, e la sua tabella non e' autosufficiente per sei celle. E
[DEBT-043] registra che **quattro righe su dieci — la 2, la 4, la 5 e la 6 — non
hanno mai avuto un attacco nel merito.**

## Scope

### Included

- L'attacco nel merito alle righe 2, 4, 5 e 6, che chiude [DEBT-043]. Va fatto
  **prima** di portarle in una decisione, non dopo.
- L'ADR che fissa, per ciascuno dei nove piu' i tre dell'enrollment, la via
  scelta: limite di genesi con il suo valore, oppure dichiarazione DRAFT con la
  ragione.
- La regola che dice quale documento di parametri governa un'epoca di elezione,
  in forma che un verificatore che rigioca la catena possa applicare
  ([DEBT-028]).
- L'attuazione in `core/coblox-core/src/params.rs`, nel blocco dei vincoli di
  `docs/protocol/ledger.md`, e nella lista DRAFT di `docs/protocol/README.md`.
- La passata di [ADR-012] su tutti gli artefatti pubblicati, con lo strumento
  versionato.

### Excluded

- **Il beacon di casualita' dedicato in se'**, che l'operatore ha collocato in
  M-03 il 2026-08-27: il suo consumatore e' l'esito di quella milestone e non di
  questa.

  Due righe di `docs/protocol/ledger.md` rientrano pero' **dentro** il perimetro
  di questa spec, perche' vivono nel file su cui la passata di [ADR-012] gira
  comunque:

  - il rimando della sezione *"Challenge evidence"*, che oggi punta a [DEBT-005],
    risolto e con un altro oggetto, e va fatto puntare alla voce di M-03
    ([DEBT-038]);
  - la frase *"Two reductions are available and are not taken in v0"*, che non
    qualifica il proprio ambito: la seconda riduzione **e' presa** dal seme
    dell'elezione via `election_entropy_blocks` ([DEBT-046]).

  Una nota di provenienza, perche' non si ripeta: fino al 2026-08-27 queste due
  righe erano descritte come una **contraddizione** del documento. Non lo sono —
  le due frasi stanno in sezioni diverse e parlano di due consumatori di
  casualita' diversi. [DEBT-041] portava quella descrizione ed e' stato
  superseduto da [DEBT-046].
- La banda di `key_compromise` e tutto cio' che vive nella parte 2 di [ADR-017]:
  e' [SPEC-022], in revisione aperta.
- `sim/coblox_sim/params.py`, che e' [DEBT-045] e non ha ancora una casa.

## Technical proposal

Tre passi, nell'ordine, e il primo non e' saltabile.

**1. Attaccare le quattro righe non riviste.** Classificazione, danno massimo,
vincolo proposto. Se l'attacco ne cambia una, l'ADR nasce su quella corretta.

**2. Decidere via per via.** Per ciascuno dei dodici parametri: limite di genesi o
dichiarazione aperta. La scelta va motivata sul **danno massimo**, non sulla
comodita', e per ogni limite il valore va **derivato da una grandezza dichiarata**,
non scelto. Dove la grandezza manca, si dichiara aperto: e' la via onesta, non la
resa.

**3. Attuare e provare in negativo.** Ogni limite nuovo va osservato rifiutare un
documento che lo viola.

## Acceptance criteria

- [ ] Le righe 2, 4, 5 e 6 hanno un attacco nel merito registrato, e la
      trascrizione dice per ciascuna se l'attacco ne ha cambiato la
      classificazione, il danno massimo o il vincolo proposto. Chiude [DEBT-043].
- [ ] Ciascuno dei nove parametri di [DEBT-044] ha una via scelta e motivata:
      limite di genesi con valore derivato, oppure dichiarazione DRAFT con la
      ragione. Nessuno resta senza.
- [ ] Lo stesso per i tre campi di `EnrollmentParametersBody` ([DEBT-037]).
- [ ] Esiste una regola che dice quale documento di parametri governa un'epoca di
      elezione, applicabile da un verificatore che rigioca la catena ([DEBT-028]).
- [ ] `consensus_parameters_closure.py` e' verde sul nuovo stato, e la sua prova
      in negativo continua a cogliere sia il campo di schema fuori da entrambe le
      liste sia la voce di lista senza campo corrispondente.
- [ ] Ogni limite nuovo e' stato **osservato rifiutare** un documento che lo
      viola, una mutazione per limite, con la trascrizione di ciascun rifiuto.
- [ ] La passata di [ADR-012] e' eseguita e la trascrizione allegata.
- [ ] Nessun valore e' stato copiato dall'analisi come testo: per ciascuno la
      trascrizione nomina la grandezza da cui e' derivato.

## Verification gates

- [ ] GATE-FOUR-ROWS-ATTACKED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Le righe 2, 4, 5 e 6 sono state attaccate nel merito **prima** che l'ADR le usasse, e la trascrizione lo mostra nell'ordine.
- [ ] GATE-NEGATIVE-PROOF | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Ogni limite nuovo osservato rifiutare un documento che lo viola, una mutazione per limite.
- [ ] GATE-DERIVED-NOT-CHOSEN | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Per ogni valore fissato la trascrizione nomina la grandezza da cui e' derivato. Un numero senza derivazione va dichiarato aperto invece che scritto.
- [ ] GATE-ADR012-PASS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Passata eseguita con lo strumento versionato, trascrizione allegata.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | Review di AGENT-007. **Non e' facoltativa**: questa spec fissa i limiti che il quorum sedente non potra' superare.
- [ ] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=transcript | La pipeline reale e' verde su tutti i job, con numero di run e commit.

## Instructions for the assigned specialist

- La spec e' in `backlog`. Se passa a `ready`, esegui `spec_start` come prima
  azione e `spec_submit` a implementazione completa.
- **I valori di lancio sono una decisione dell'operatore.** Proponi con la
  derivazione e il costo; non fissarli da solo.
- L'analisi e' una **base**, non un testo da copiare. Le sue avvertenze sono in
  [REVIEW-043].
- Se trovi un difetto fuori perimetro, **aprilo come rilievo e non correggerlo**.
  Precedenti: [DEBT-041], [DEBT-045].
- Niente commit ne' push: il push su `main` e' del Lead.

## Implementation evidence

> Compilata dallo specialista a lavoro concluso.

### Changes made

### Files changed

### Verification performed

### Verification transcript
