---
id: SPEC-028
# Note: Quote the title if it contains a colon
title: "L'host di un ComputeAssignment si deriva, non si sceglie: togliere la direzionalita' e dichiarare i due tetti"
status: backlog
kind: feature
priority: medium
area: compute
milestone: M-06
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-003
capability_tier: terra
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-004, ADR-006, ADR-012]
links: []
created: 2026-08-27
updated: 2026-08-27
tags: [sandbox, threat-model]
activity:
  - date: 2026-08-27
    action: "created"
  - date: 2026-08-27
    action: "set tags"
---
# L'host di un ComputeAssignment si deriva, non si sceglie

## Objective

Chiudere [DEBT-024] nel verso che l'operatore ha deciso il 2026-08-27: **la
scelta congiunta di modulo, input e host da parte dell'emittente non e'
deliberata**, e va tolta. L'host viene derivato, come gia' avviene per le sfide.
Piu' i due tetti che oggi non esistono.

## Context

`ComputeAssignment` porta un campo `input` scelto **verbatim dall'emittente**, e
ne segue che un validatore sceglie quale modulo un host determinato esegue e con
quale input, senza alcun tetto dichiarato sul numero di assegnazioni ne' sulla
taglia dell'input. Scenario TM-42 del threat model.

**Il protocollo ha gia' il modello giusto e lo usa altrove.** Per le sfide,
`docs/protocol/ledger.md` impone che la coppia sia *"the pair the epoch's
assignment function produces from that beacon"* — derivata da un beacon, non
scelta da chi emette. Compute e sfide seguono oggi modelli **opposti** nello
stesso protocollo, e nessun documento dice perche'.

**E la capacita' era gia' stata negata una volta.** [ADR-006] tolse
deliberatamente al publisher la scelta degli host. Un validatore che ce l'ha per
il compute e' la stessa capacita' rientrata da un'altra porta: non e'
l'esecuzione di codice ostile, che [ADR-004] mette in conto, ma la
**direzionalita'** — puntare un host determinato invece di pubblicare e
aspettare.

**Perche' adesso e non a M-06.** Il livello compute e' M-06 e oggi nessuna riga
di codice lo implementa: la correzione e' un paragrafo. A M-06 sara' un formato
di messaggio con implementazioni al seguito, e la stessa correzione costera'
molto di piu'. E' l'argomento che [DEBT-024] fa da se'.

## Scope

### Included

- La regola che **deriva l'host** di un `ComputeAssignment` invece di lasciarlo
  scegliere all'emittente, nella forma che le sfide gia' usano.
- Un **tetto dichiarato sul numero** di assegnazioni che un emittente puo'
  produrre per epoca.
- Un **tetto dichiarato sulla taglia** del campo `input`.
- L'aggiornamento di `docs/protocol/wire.md` e la passata di [ADR-012].
- L'aggiornamento della cella TM-42 in `knowledge/threat-model.md`, che oggi
  descrive la superficie come aperta.

### Excluded

- L'implementazione del livello compute, che resta M-06. Questa spec cambia una
  regola scritta, non costruisce il runtime.
- Il modello di sandbox di [ADR-004], che non e' in discussione: la superficie
  qui e' la direzionalita', non l'esecuzione.

## Technical proposal

L'assegnazione compute adotta la forma dell'assegnazione delle sfide: la coppia
`(emittente, host)` e' prodotta da una funzione dell'epoca a partire da un
beacon, e l'emittente non la sceglie. Il modulo e l'input restano suoi, perche'
e' cio' che l'assegnazione **e'**; cio' che gli si toglie e' il bersaglio.

I due tetti vanno **derivati da una grandezza dichiarata** e non scelti. Dove la
grandezza manca, il tetto va dichiarato aperto nella lista DRAFT con la ragione,
seguendo la disciplina che [SPEC-027] stabilisce per la classe dei parametri
operativi.

## Acceptance criteria

- [ ] `docs/protocol/wire.md` dichiara che l'host di un `ComputeAssignment` e'
      derivato e non scelto, e nomina la funzione che lo produce.
- [ ] Esiste un tetto dichiarato sul numero di assegnazioni per emittente per
      epoca, con la grandezza da cui e' derivato oppure la dichiarazione che
      resta aperto e perche'.
- [ ] Esiste un tetto dichiarato sulla taglia di `input`, alle stesse condizioni.
- [ ] La cella TM-42 di `knowledge/threat-model.md` e' aggiornata e non descrive
      piu' la superficie come aperta.
- [ ] La passata di [ADR-012] e' eseguita e la trascrizione allegata.
- [ ] Una probe pinna la frase che dichiara la derivazione, cosi' che toglierla
      faccia fallire la passata.

## Verification gates

- [ ] GATE-ADR012-PASS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Passata eseguita con lo strumento versionato, trascrizione allegata. Questa spec cambia una regola in un documento pubblicato: e' della classe che quella ADR governa.
- [ ] GATE-DERIVED-NOT-CHOSEN | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Per ciascuno dei due tetti la trascrizione nomina la grandezza da cui e' derivato, oppure dichiara che resta aperto e perche'. Un numero senza derivazione va dichiarato aperto invece che scritto.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | Review di AGENT-007, che ha trovato la superficie durante [SPEC-018] e ne ha scritto lo scenario.
- [ ] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=transcript | La pipeline reale e' verde su tutti i job, con numero di run e commit.

## Instructions for the assigned specialist

- La spec e' in `backlog`. Se passa a `ready`, esegui `spec_start` come prima
  azione e `spec_submit` a implementazione completa.
- **Non costruire il livello compute.** Questa spec cambia una regola scritta.
- Se la funzione di assegnazione delle sfide non si presta al compute per una
  ragione tecnica, **fermati e riportala**: significa che la decisione
  dell'operatore poggiava su un'analogia che non regge, ed e' un fatto che va
  portato a lui e non aggirato.
- Se trovi un difetto fuori perimetro, **aprilo come rilievo e non correggerlo**.
  Precedenti: [DEBT-045], [DEBT-046].
- Niente commit ne' push: il push su `main` e' del Lead.

## Implementation evidence

> Compilata dallo specialista a lavoro concluso.

### Changes made

### Files changed

### Verification performed

### Verification transcript
