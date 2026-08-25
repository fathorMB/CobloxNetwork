---
id: SPEC-005
# Note: Quote the title if it contains a colon
title: "Applicazione di ADR-009 al design system: unità credit e font deciso"
status: ready
kind: feature
priority: medium
area: design
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-006
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: terra
thinking_level: standard
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-009]
links: [SPEC-003]
created: 2026-08-25
updated: 2026-08-25
tags: [design-system, typography, naming]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "set effort"
  - date: 2026-08-25
    action: "set tags"
  - date: 2026-08-25
    action: "transitioned backlog -> ready"
---
# Applicazione di ADR-009 al design system: unità credit e font deciso

## Objective

Rimuovere dal pacchetto di design ogni traccia del segnaposto dell'unità e portarlo alla forma decisa da [ADR-009]: unità `credit`/`credits`, forma compatta `cr` **posposta** al numero. Ritirare il glifo `◇` e la classe `.cbx-unit--provisional`, e registrare in `PRINCIPLES.md` la regola con la sua ragione, non solo il suo esito.

## Context

[SPEC-003] aveva consegnato il design system con il nome dell'unità dichiaratamente indeciso, e con il punto di modifica isolato apposta per rendere la decisione a costo quasi nullo. [ADR-009] ha preso quella decisione ed è `accepted`.

Il ragionamento che la governa va conosciuto da chi applica, perché determina *come* si applica e non solo *cosa*: il vincolo permanente di [[PROJECT]] è che il token non acquisisca valore monetario neanche di fatto, e la resa tipografica è una delle superfici su cui quel vincolo si difende. Un glifo che precede il numero è la grammatica del denaro (`$50`); un'abbreviazione che lo segue è la grammatica della misura (`50 kg`). Chi applica questa spec deve poter riconoscere, in ogni punto che tocca, quale delle due sta scrivendo.

Il font monospace è deciso — **JetBrains Mono** — ma è già primo nello stack dichiarato in `tokens.json`, quindi sui token non c'è nulla da cambiare: cambia lo stato della questione, da aperta a decisa, e va riflesso dove il pacchetto la dichiara ancora aperta.

## Scope
### Included

- Ritiro del glifo `◇` e della classe `.cbx-unit--provisional` da tutte le superfici del pacchetto.
- Adozione della forma `credits` estesa e `cr` compatta, sempre posposta al numero.
- Aggiornamento di `PRINCIPLES.md` §4.1 e di `$meta` in `tokens.json`.
- Rimozione delle avvertenze «il nome dell'unità non è ancora deciso» dai mockup e dall'anteprima.
- Chiusura della questione font nella documentazione del pacchetto.

### Excluded

- **Incorporare i file del font nel bundle Tauri.** Resta lavoro di un'altra spec, come [SPEC-003] già annotava. Con SIL OFL 1.1 andrà inclusa la licenza del font accanto all'`Apache-2.0` del progetto.
- Qualunque modifica a palette, spaziature, componenti o layout.
- Qualunque modifica al codice dell'applicazione: il pacchetto di design è l'unico perimetro.
- Traduzione in inglese dell'intero pacchetto. Vedi *Risks and open decisions*.

## Existing-project analysis

Verificato dal Lead prima della stesura. Due fatti cambiano la forma del lavoro rispetto a come lo si immaginerebbe.

**Gli HTML sono generati, non scritti.** `preview/index.html` e i tre mockup sono prodotti da `tools/build-preview.mjs` e `tools/build-mockups.mjs`, come dichiara `README.md`. Modificarli a mano è l'errore da non fare: verrebbero sovrascritti alla prima rigenerazione e il difetto ricomparirebbe senza che nessuno capisca perché. **La sorgente sono i generatori.** Entrambi definiscono l'unità in una singola costante `UNIT` — `build-mockups.mjs:23`, `build-preview.mjs:29` — quindi il grosso della sostituzione è due righe. Esistono però occorrenze anche fuori da quella costante: `build-mockups.mjs:918`, `build-preview.mjs:499` e `:502` contengono le frasi di avvertenza e un'icona `◇`.

**La regola esiste già, ed è il token a farle eccezione.** `PRINCIPLES.md` §7 impone da sempre lo spazio unificatore fra numero e unità per ogni altra unità del prodotto — `512 GB`, `340 ms` — e il token era l'unico valore reso con un glifo anteposto. [ADR-009] non introduce quindi una regola nuova: **rimuove un'eccezione**. Va detto in §4.1, perché rende la regola molto più difficile da infrangere di quanto sarebbe una prescrizione isolata.

**Formato numerico.** `PRINCIPLES.md` §4.2 impone separatore delle migliaia **spazio stretto unificatore** (U+202F) e separatore decimale il punto. La forma corretta è `1 240 cr`, **non** `1,240 cr`. Gli esempi della prima stesura di [ADR-009] usavano la virgola e sono stati corretti dal Lead; se ne trovi altri, sono errori da correggere e non precedenti da seguire.

## Technical proposal

Sostituire la costante `UNIT` nei due generatori con una resa che accosti valore e unità posposta, separati da spazio unificatore, riusando la classe `.cbx-unit` senza il modificatore provvisorio. Eliminare `.cbx-unit--provisional` da `css/base.css` insieme al commento che ne spiegava la provvisorietà. Rimuovere le frasi di avvertenza e l'icona segnaposto. Rigenerare gli artefatti e verificare che il diff degli HTML contenga solo i cambiamenti attesi.

Se `cr` vada usata ovunque o se le cifre "hero" delle schermate principali meritino `credits` per esteso è **giudizio di design e spetta a te**: la spec impone la grammatica, non la densità. Qualunque sia la scelta, va scritta in `PRINCIPLES.md` come regola e non lasciata implicita nei mockup.

## Files and areas involved

- `.lmbrain/design/coblox-design-system/tools/build-mockups.mjs` — costante `UNIT` alla riga 23, avvertenza alla riga 918.
- `.lmbrain/design/coblox-design-system/tools/build-preview.mjs` — costante `UNIT` alla riga 29, icona e avvertenza alle righe 499 e 502.
- `.lmbrain/design/coblox-design-system/css/base.css` — classe `.cbx-unit--provisional` alla riga 175 e il commento che la precede.
- `.lmbrain/design/coblox-design-system/PRINCIPLES.md` — §4.1 da riscrivere; verificare coerenza con §4.2 e §7.
- `.lmbrain/design/coblox-design-system/tokens/tokens.json` — `$meta`.
- Rigenerati, da non modificare a mano: `preview/index.html`, `mockups/dashboard.html`, `mockups/attivita.html`, `mockups/onboarding.html`, `mockups/index.html`.

## Acceptance criteria
- [ ] Nessuna occorrenza del glifo `◇` resta nel pacchetto, generatori inclusi, salvo usi che non riguardano l'unità e che vanno dichiarati esplicitamente.
- [ ] La classe `.cbx-unit--provisional` non esiste più in `css/base.css` e non è referenziata da nessuna superficie.
- [ ] L'unità è resa **posposta** al numero in ogni schermata, con spazio unificatore, coerente con `PRINCIPLES.md` §7.
- [ ] Il formato numerico rispetta §4.2: separatore delle migliaia U+202F, decimale il punto, nessuna virgola come separatore delle migliaia.
- [ ] `PRINCIPLES.md` §4.1 dichiara nome, plurale e forma compatta **con la ragione della posposizione**, e dice esplicitamente che il token non fa più eccezione alla regola di §7.
- [ ] `PRINCIPLES.md` dichiara JetBrains Mono come font deciso e non più come proposta.
- [ ] `$meta` in `tokens.json` è aggiornato.
- [ ] Gli HTML sono stati **rigenerati** con i generatori, non modificati a mano, e il diff non contiene cambiamenti estranei a questa spec.
- [ ] Nessuna regressione di contrasto: le coppie dichiarate in `CONTRAST.md` restano valide e `.cbx-unit` mantiene un contrasto conforme a WCAG AA nella nuova resa.

## Implementation plan
1. Leggere [ADR-009], in particolare *Decision* e *Consequences*: la ragione della posposizione determina come applicare, non solo cosa.
2. Sostituire la costante `UNIT` nei due generatori; eliminare avvertenze e icona segnaposto.
3. Rimuovere `.cbx-unit--provisional` e il suo commento da `css/base.css`.
4. Riscrivere `PRINCIPLES.md` §4.1 e chiudere la questione font.
5. Aggiornare `$meta` in `tokens.json`.
6. Rigenerare tutti gli artefatti con i generatori e ispezionare il diff.
7. Verificare contrasto e assenza di residui del segnaposto.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-NO-PLACEHOLDER | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una ricerca ricorsiva su tutto il pacchetto per il glifo tipografico segnaposto e per la stringa `cbx-unit--provisional` non restituisce risultati, oppure solo occorrenze dichiarate ed estranee all'unità. Incollare comando e output reale.
- [ ] GATE-REGENERATED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Gli artefatti sono stati rigenerati eseguendo i generatori, e una seconda esecuzione consecutiva non produce ulteriori differenze: la generazione è idempotente. Incollare l'output di entrambe le esecuzioni e lo stato del diff.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio principale è modificare gli HTML invece dei generatori.** Sembrerebbe funzionare, e il difetto tornerebbe alla prima rigenerazione. `GATE-REGENERATED` esiste per intercettarlo: se hai modificato gli HTML a mano, la seconda esecuzione consecutiva dei generatori produrrà differenze e il gate fallirà.
- **Aperto, e ti riguarda solo in parte.** Le avvertenze e alcune frasi dei mockup sono in italiano, mentre il vincolo di lingua di [[PROJECT]] vuole in inglese tutto ciò che vede l'utente finale. Le avvertenze spariscono con questa spec, quindi il problema si riduce da sé. **Non tradurre il resto del pacchetto in questa passata**: sarebbe un cambiamento ampio, non richiesto qui, e va deciso a parte. Se durante il lavoro incontri altro testo italiano su superfici di prodotto, segnalalo nell'evidenza invece di correggerlo.
- **Aperto, e spetta a te**: se le cifre "hero" debbano portare `credits` per esteso o la forma compatta. Vedi *Technical proposal*.

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
