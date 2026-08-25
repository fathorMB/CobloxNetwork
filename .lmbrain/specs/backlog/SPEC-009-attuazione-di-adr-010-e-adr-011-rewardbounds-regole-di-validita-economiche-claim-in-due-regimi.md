---
id: SPEC-009
# Note: Quote the title if it contains a colon
title: "Attuazione di ADR-010 e ADR-011: RewardBounds, regole di validita economiche, claim in due regimi"
status: backlog
kind: feature
priority: high
area: token-economy
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
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
related_decisions: [ADR-005, ADR-007, ADR-010, ADR-011]
links: [SPEC-006, SPEC-007, SPEC-008]
created: 2026-08-25
updated: 2026-08-25
tags: [governance, sybil, conformance]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "set effort"
  - date: 2026-08-25
    action: "set tags"
---
# Attuazione di ADR-010 e ADR-011: RewardBounds, regole di validità economiche, claim in due regimi

## Objective

Scrivere nei documenti di protocollo le quattro disposizioni di [ADR-010] e [ADR-011], trasformando in **regole di validità** ciò che oggi sono valori scelti bene: `RewardBounds` nell'ancora di fiducia della genesi, la tariffa di availability a zero, il vincolo `3 · validator_min_set_size >= 2 * V`, e il dimensionamento del fondo alla genesi con la dichiarazione del claim in due regimi.

## Context

Le due ADR si attuano **insieme e non in sequenza**, perché `RewardBounds` deve esprimere un tetto che il fondo di genesi rispetta e che la crescita successiva non viola: separarle produrrebbe due documenti che si contraddicono a vicenda.

Il principio che le tiene, ed è di AGENT-007 in [REVIEW-011]:

> Ciò che è governato senza limiti di magnitudine non è un parametro, è una preferenza.

**È la terza istanza di uno schema che il progetto ha già risolto due volte.** Il pavimento di costo dell'Argon2id è una regola di validità perché un insieme di parametri governato avrebbe potuto rimuoverlo restando conforme e senza traccia on-chain; `ElectionBounds` è nella genesi per la stessa ragione. Le difese economiche erano rimaste indietro, e questa spec le allinea. Quando scrivi la motivazione nei documenti, **cita il precedente**: è ciò che rende la regola difendibile invece che arbitraria.

Da [ADR-011] viene la struttura che nessun documento aveva prima: **`α ≈ 1` durante l'avviamento è strutturale e nessun valore del fondo lo cambia**, perché `α` è un rapporto il cui denominatore al lancio è vuoto. Rimpicciolire il fondo non abbassa `α` durante la rampa, abbassa il fondo. Ciò che il fondo governa è l'importo assoluto a rischio, `F · N/(N+H)`, che **non contiene l'uso**. Due strumenti separati per due problemi separati.

## Scope
### Included

- `RewardBounds` nell'ancora di fiducia della genesi, simmetrico a `ElectionBounds`: magnitudini, rapporto massimo di variazione, spaziatura minima in altezze di catena.
- `availability_microtokens_per_unit = 0` come regola di validità, rifiutata in accettazione.
- `3 * validator_min_set_size >= 2 * V` nel blocco di vincoli dei parametri di consenso.
- Il dimensionamento del fondo alla genesi sulla popolazione attesa al lancio.
- La riformulazione del claim in due regimi: garanzia assoluta per la rampa, banda su `α` per il regime maturo con la soglia reale.
- Le fixture di conformità corrispondenti, e l'aggiornamento di quelle che le nuove regole invalidano.

### Excluded

- **L'implementazione in Rust.** `coblox-core` dovrà validare `RewardBounds` con lo stesso trattamento già dato a `ValidatedConsensusParameters` — nessun costruttore diverso dalla validazione — ma è spec conseguente, e appartiene a chi ha scritto quel crate.
- Qualunque modifica alla regola di elezione, alla derivazione o al pavimento di contrazione: [SPEC-006] ha attraversato quattro giri di review e questa spec **aggiunge un vincolo**, non ne tocca alcuno.
- La ritaratura dei valori di [SPEC-007]. Restano quelli, salvo dove una nuova regola li renda inammissibili — nel qual caso **fermati e segnala**, non ritarare.

## Existing-project analysis

**Il fixture `PD-0` fallisce il nuovo vincolo, e va corretto.** Verificato dal Lead: il documento di conformità dei parametri di consenso usa `validator_min_set_size = 1` con `V = 12`, e `3 · 1 = 3` non è `>= 2 · 12 = 24`. Il minimo ammissibile per `V = 12` è `min_set = 8`, che il Lead ha verificato compatibile con tutto il resto del blocco. **`consensus_parameters_hash` cambierà per la terza volta**, ed è la terza volta che una regola nuova rivela un fixture che insegnava una forma inammissibile.

Applica il metodo che il progetto ha già usato due volte e che il Lead rifarà: **valida il tuo procedimento su una fixture non modificata prima di ricalcolare quelle che cambiano.** Se il procedimento non riproduce un valore che non hai toccato, non è evidenza per quelli che hai toccato.

**I valori raccomandati passano con uguaglianza esatta.** `V = 27` e `min_set = 18` danno `54 >= 54`. Il vincolo è quindi a costo zero sui valori scelti, ed è la ragione per cui [ADR-010] lo adotta: non chiede di cambiare nulla, impedisce che il rapporto si eroda.

**La forma da specchiare esiste già.** `ElectionBounds` è nell'ancora di fiducia della genesi con magnitudini, rapporto di variazione e spaziatura minima in altezze di catena, e ha una ragione scritta per ciascuna delle tre. Segui quella forma: la simmetria non è estetica, è ciò che consente a un implementatore di validare i due insiemi con lo stesso codice.

**Attenzione a `ledger.md`.** L'affermazione «soglia effettiva appena sopra un terzo» è oggi **la cifra giusta nel caso peggiore governabile**, come AGENT-007 ha stabilito, ed è corretta finché il vincolo su `min_set` non esiste. Quando lo avrai scritto, **quella frase va aggiornata nella stessa passata**: il merito che [SPEC-007] attribuiva a `min_set_size` diventa rivendicabile solo ora che `min_set_size` è vincolato. Non lasciarla indietro.

## Technical proposal

Su `RewardBounds`, la domanda da porsi per ogni grandezza della reward policy non è «serve un limite?» ma **«questa grandezza sostiene una proprietà di sicurezza dichiarata?»**. Se sì, va vincolata; se no, dichiara perché no. Il tetto di `existence_fund_microtokens_per_epoch` vi appartiene di sicuro.

Sul dimensionamento del fondo, `RewardBounds` e il valore di genesi sono **due cose distinte che devono essere coerenti**: il tetto è statico e dimensionato sulla rete matura, il valore di genesi è dimensionato sul lancio, e il limite di variazione è ciò che porta il secondo verso il primo. Scrivi anche il **costo operativo**: la crescita richiede una successione di documenti con la spaziatura minima, cioè governance attiva, e va dichiarato invece che scoperto.

Sul claim, la formulazione **non è** «la metrica vale sopra una soglia» ma «la metrica è una proprietà della rete a regime, e durante l'avviamento la garanzia è un'altra». Enuncia la garanzia della rampa in **termini assoluti**, perché è la forma in cui è vera.

**Un rimedio apparente da rifiutare esplicitamente nel documento**, perché qualcuno lo proporrà: un tetto del fondo **proporzionale al numero di eleggibili** sarebbe un tetto che una flotta alza gonfiando il denominatore, e riaprirebbe il criterio (a) di [ADR-007]. È la stessa trappola della ripartizione pesata raggiunta da un'altra strada. Scriverlo come rifiutato costa due righe e risparmia un giro di review.

## Files and areas involved

- `docs/protocol/README.md` — schema di `RewardBounds`, ancora di fiducia della genesi, registro delle preimmagini e registro di conformità.
- `docs/protocol/ledger.md` — blocco di vincoli, regola di validità sulla tariffa di availability, e l'affermazione sulla soglia effettiva da aggiornare.
- `.lmbrain/knowledge/economic-simulation-report.md` — la dichiarazione in due regimi e il costo operativo della crescita del fondo.
- `.lmbrain/knowledge/threat-model.md` — `SEC-REQ-16`, `SEC-REQ-18` e le note dei test di attacco. È documento di AGENT-007: segui le sue convenzioni.

## Acceptance criteria
- [ ] `RewardBounds` è definito nell'ancora di fiducia della genesi con magnitudini, rapporto di variazione e spaziatura minima, **con una ragione scritta per ciascuna delle tre**, e simmetrico a `ElectionBounds` nella forma.
- [ ] Per ogni grandezza della reward policy è dichiarato se sostiene una proprietà di sicurezza, e quindi se è vincolata o perché non lo è.
- [ ] `availability_microtokens_per_unit` diverso da zero rende il documento **invalido in accettazione**, e la ragione strutturale è scritta accanto alla regola: è l'unico canale che paga per nodo senza tetto aggregato.
- [ ] `3 * validator_min_set_size >= 2 * V` è nel blocco di vincoli, e il documento dichiara che cosa quel vincolo impedisce.
- [ ] Il fixture `PD-0` dei parametri di consenso soddisfa il nuovo vincolo, e `consensus_parameters_hash` è ricalcolato con il metodo validato su una fixture non modificata.
- [ ] Il valore di genesi del fondo è dimensionato sulla popolazione attesa al lancio, coerente con il tetto di `RewardBounds`, e il **costo operativo della crescita** è dichiarato.
- [ ] Il claim è enunciato in **due regimi**: garanzia assoluta per la rampa, banda su `α` con la soglia reale per il regime maturo. Nessuna formulazione lascia intendere che la banda valga durante l'avviamento.
- [ ] L'affermazione di `ledger.md` sulla soglia effettiva è aggiornata nella stessa passata in cui il vincolo su `min_set` viene scritto.
- [ ] Il tetto proporzionale agli eleggibili è **esplicitamente rifiutato** nel documento, con la ragione.
- [ ] Le fixture di conformità coprono le regole nuove, incluso un documento rifiutato per tariffa di availability positiva.
- [ ] Nessuna regola di [SPEC-006] è modificata, e nessun valore di [SPEC-007] è ritarato.

## Implementation plan
1. Leggere [ADR-010] e [ADR-011] per intero, e la sezione di `README.md` che motiva `ElectionBounds` e il pavimento Argon2id: sono la forma da specchiare e il precedente da citare.
2. Definire `RewardBounds` e collocarlo nell'ancora di fiducia della genesi.
3. Scrivere le due regole di validità: tariffa di availability, e vincolo su `min_set` nel blocco.
4. Correggere il fixture `PD-0`, ricalcolare l'hash con il metodo validato, aggiornare il registro.
5. Dimensionare il fondo di genesi e dichiarare il costo operativo della crescita.
6. Riformulare il claim in due regimi e aggiornare l'affermazione di `ledger.md`.
7. Aggiungere le fixture delle regole nuove e aggiornare il threat model.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-FIXTURES-RECOMPUTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il metodo di calcolo è validato riproducendo almeno una fixture **non modificata** prima di ricalcolare quelle che cambiano, e ogni hash nuovo è ricalcolato dai byte effettivamente scritti. Incollare entrambe le esecuzioni. È il metodo che il progetto ha già usato due volte e che il Lead rifarà.
- [ ] GATE-RULES-REJECT | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Per ciascuna delle tre regole di validità nuove esiste un caso che **viene rifiutato**, e la trascrizione lo mostra. Una regola di validità di cui non si esibisce il rifiuto è una raccomandazione con un nome diverso.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto le regole nuove e la riformulazione del claim, e il Lead ha accettato la review. Le regole nascono da un suo finding critico e la formulazione del claim in due regimi è materia sua: chiuderle senza la sua verifica sarebbe incoerente con il modo in cui sono state aperte.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Aperto, e non lo decidi tu: la popolazione attesa al lancio.** È il numero su cui il fondo di genesi va dimensionato, ed è una decisione di prodotto dell'operatore. Istruiscila come hai istruito `α`: mostra cosa comporta ciascun ordine di grandezza in termini di importo assoluto a rischio e di numero di documenti necessari per arrivare a scala di riferimento, e raccomanda. **Non scegliere il numero da sola.**
- **Il rischio principale è vincolare solo ciò che è comodo vincolare.** `RewardBounds` è utile quanto è completo: se una grandezza che sostiene una proprietà di sicurezza resta fuori, la superficie non è chiusa, è ristretta — e una superficie ristretta dichiarata chiusa è peggio di una aperta dichiarata tale. Per ogni grandezza lasciata fuori, scrivi perché.
- **Rischio di scrivere regole senza rifiuti.** `GATE-RULES-REJECT` esiste per questo: una regola di validità di cui nessuno esibisce il caso rifiutato è indistinguibile da una raccomandazione, e il progetto ha già stabilito due volte che la differenza è tutta lì.
- **Il tetto proporzionale agli eleggibili tornerà.** È il rimedio che sembra ovvio e che una flotta alza gonfiando il denominatore. Scriverlo come rifiutato nel documento, con la ragione, è più efficace che confidare che nessuno lo riproponga.
- **Se una regola nuova rende inammissibile un valore di [SPEC-007]**, fermati e segnala invece di ritarare: la taratura ha attraversato una review di sicurezza e non si tocca di lato.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

### Files changed

### Verification performed

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

```text

```

### Deviations from the specification

### Handoff status
- [ ] Ready for Project Lead review
