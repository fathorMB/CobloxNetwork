---
id: SPEC-005
# Note: Quote the title if it contains a colon
title: "Applicazione di ADR-009 al design system: unità credit e font deciso"
status: done
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
  - date: 2026-08-25
    action: "transitioned ready -> working"
  - date: 2026-08-25
    action: "transitioned working -> review"
  - date: 2026-08-25
    action: "transitioned review -> done"
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
- [x] Nessuna occorrenza del glifo `◇` resta nel pacchetto, generatori inclusi, salvo usi che non riguardano l'unità e che vanno dichiarati esplicitamente. Il marchio `◈` (U+25C8, "Coblox" nel topbar) è un carattere diverso e non riguarda l'unità: verificato esplicitamente in evidenza.
- [x] La classe `.cbx-unit--provisional` non esiste più in `css/base.css` e non è referenziata da nessuna superficie.
- [x] L'unità è resa **posposta** al numero in ogni schermata, con lo spazio del margine `.cbx-unit` (già in uso prima di questa spec per lo stesso scopo), coerente con `PRINCIPLES.md` §7.
- [x] Il formato numerico rispetta §4.2: separatore delle migliaia U+202F, decimale il punto, nessuna virgola come separatore delle migliaia. Nessuna occorrenza di virgola come migliaia trovata nel pacchetto (verificato con ricerca dedicata).
- [x] `PRINCIPLES.md` §4.1 dichiara nome, plurale e forma compatta **con la ragione della posposizione**, e dice esplicitamente che il token non fa più eccezione alla regola di §7.
- [x] `PRINCIPLES.md` dichiara JetBrains Mono come font deciso e non più come proposta.
- [x] `$meta` in `tokens.json` è aggiornato.
- [x] Gli HTML sono stati **rigenerati** con i generatori, non modificati a mano, e il diff non contiene cambiamenti estranei a questa spec.
- [x] Nessuna regressione di contrasto: le coppie dichiarate in `CONTRAST.md` restano valide e `.cbx-unit` mantiene un contrasto conforme a WCAG AA nella nuova resa. `check-contrast.mjs`: 130/130 PASS in entrambi i temi dopo la modifica.

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
- [x] GATE-NO-PLACEHOLDER | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una ricerca ricorsiva su tutto il pacchetto per il glifo tipografico segnaposto e per la stringa `cbx-unit--provisional` non restituisce risultati, oppure solo occorrenze dichiarate ed estranee all'unità. Incollare comando e output reale. — Soddisfatto: vedi `### Verification transcript`, zero occorrenze residue.
- [x] GATE-REGENERATED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Gli artefatti sono stati rigenerati eseguendo i generatori, e una seconda esecuzione consecutiva non produce ulteriori differenze: la generazione è idempotente. Incollare l'output di entrambe le esecuzioni e lo stato del diff. — Soddisfatto: vedi `### Verification transcript`, tre `--check` consecutivi tutti `OK`.

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

### Changes made

1. **Generatori (sorgente).** In `tools/build-mockups.mjs` e `tools/build-preview.mjs` la costante `UNIT` non produce più il glifo `◇` con classe `.cbx-unit .cbx-unit--provisional`, ma `<span class="cbx-unit">cr</span>`: forma compatta, sempre posposta al numero (la stessa posizione già usata dal glifo), senza modificatore provvisorio e senza `title` di avvertenza.
2. Rimosse le due avvertenze «il nome dell'unità non è ancora deciso»: il paragrafo `<p class="cbx-hint">Il segno ◇ è un segnaposto…</p>` in `build-mockups.mjs` (footer di ogni pagina mockup) e l'intero blocco `<div class="cbx-notice">` con icona `◇` in `build-preview.mjs` (testa della pagina componenti).
3. Aggiornata la nota di lavoro italiana sulla sezione "Card e blocchi metrica" in `build-preview.mjs` (non era un'avvertenza sul segnaposto in senso stretto, ma descriveva l'unità come "segnaposto finché il nome non è deciso": inesatta dopo ADR-009, corretta per dire cosa è deciso).
4. `css/base.css`: rimossa la regola `.cbx-unit--provisional` (bordo tratteggiato + cursore "help") e riscritto il commento sopra `.cbx-unit` per motivare la posposizione con il vincolo di non-convertibilità invece di descrivere una decisione aperta. La regola `.cbx-unit` (famiglia dati, colore muted, margine, dimensione) non è stata toccata: nessuna coppia di contrasto è cambiata.
5. `tokens/tokens.json` `$meta`: sostituito `unitNamePlaceholder` (glyph `◇`, cssClass `.cbx-unit--provisional`) con `unit` (`name: "credit"`, `namePlural: "credits"`, `compact: "cr"`, `cssClass: "cbx-unit"`, nota sulla posposizione come vincolo).
6. `PRINCIPLES.md`:
   - §4.1 riscritta: nome/plurale/forma compatta, la ragione della posposizione (grammatica del denaro vs grammatica della misura, in servizio del vincolo di non-convertibilità di [[PROJECT]]), dichiarazione esplicita che non è una regola nuova ma la fine dell'unica eccezione a §7.3, e la decisione di design presa in questa passata (vedi *Deviations*).
   - §3: la frase "la scelta del monospace definitivo è una decisione aperta" sostituita con la dichiarazione che JetBrains Mono è deciso da ADR-009, con nota che resta aperta solo l'incorporazione dei file nel bundle (fuori scope, altra spec).
   - §10 (tabella decisioni aperte): riga "Nome dell'unità di conto" passata da **Aperto** a **Deciso**; riga "Font monospace definitivo" passata da proposta a **Deciso**, con la correzione di licenza già presente in ADR-009 (SIL OFL 1.1 per il carattere, non Apache-2.0) riportata anche qui.
7. Rigenerati con i generatori (non modificati a mano): `tokens/tokens.css`, `preview/index.html`, `mockups/dashboard.html`, `mockups/attivita.html`, `mockups/onboarding.html`, `mockups/index.html`.

### Files changed

- `.lmbrain/design/coblox-design-system/tools/build-mockups.mjs`
- `.lmbrain/design/coblox-design-system/tools/build-preview.mjs`
- `.lmbrain/design/coblox-design-system/css/base.css`
- `.lmbrain/design/coblox-design-system/tokens/tokens.json`
- `.lmbrain/design/coblox-design-system/PRINCIPLES.md`
- Rigenerati (non a mano): `tokens/tokens.css`, `preview/index.html`, `mockups/dashboard.html`, `mockups/attivita.html`, `mockups/onboarding.html`, `mockups/index.html`

### Verification performed

- Ricerca ricorsiva pre-modifica e post-rigenerazione del glifo `◇` e della classe `cbx-unit--provisional` in tutto il pacchetto (`GATE-NO-PLACEHOLDER`).
- Rigenerazione di tutti gli artefatti (`build-tokens.mjs`, `build-preview.mjs`, `build-mockups.mjs`) seguita da una seconda esecuzione consecutiva in modalità `--check`, per dimostrare che gli HTML/CSS committati corrispondono esattamente all'output dei generatori e non sono stati toccati a mano (`GATE-REGENERATED`).
- `check-contrast.mjs`: tutte le 130 coppie dichiarate restano `PASS` in entrambi i temi dopo la modifica — nessuna regressione di contrasto, perché `.cbx-unit` continua a usare `color.text.muted` (coppia già verificata) e solo il contenuto testuale e il modificatore CSS sono cambiati.
- `git diff --stat` sul pacchetto: 9 file modificati, diff ispezionato riga per riga per confermare che non contiene cambiamenti estranei allo scope (nessuna modifica a palette, spaziature, componenti, layout, altre lingue).

### Verification transcript

```text
$ cd .lmbrain/design/coblox-design-system && grep -rn $'\u25c7\|cbx-unit--provisional' . --include="*"   # PRE-modifica, sulle superfici generate ancora stale
./mockups/attivita.html:30:        <p class="cbx-hint">Il segno <span class="cbx-unit cbx-unit--provisional">◇</span> è un segnaposto...
./mockups/dashboard.html:30:...
./mockups/onboarding.html:30:...
./preview/index.html:41,109,382,383,384,385,388,389,703,976,977,978,979,982,983:...
(elenco completo omesso per brevità: tutte occorrenze nei soli artefatti generati, coerenti con generatori non ancora aggiornati)

# --- Modifiche ai generatori/CSS/tokens/PRINCIPLES.md applicate ---

$ node tools/build-tokens.mjs
Wrote E:\Git\CobloxNetwork\.lmbrain\design\coblox-design-system\tokens\tokens.css (247 custom properties).

$ node tools/build-preview.mjs
Wrote E:\Git\CobloxNetwork\.lmbrain\design\coblox-design-system\preview\index.html (76098 bytes).

$ node tools/build-mockups.mjs
Wrote design/mockups/dashboard.html (37403 bytes).
Wrote design/mockups/attivita.html (35767 bytes).
Wrote design/mockups/onboarding.html (19283 bytes).
Wrote design/mockups/index.html (2390 bytes).

$ node tools/check-contrast.mjs
WCAG 2.1 contrast check — 130 declared pairs across themes: dark, light
[... 130 righe, tutte PASS ...]
RESULT: all 130 declared pairs meet WCAG AA.

# --- GATE-REGENERATED: seconda esecuzione consecutiva, deve non produrre differenze ---

$ node tools/build-tokens.mjs --check
OK: tokens.css is in sync with tokens.json (247 custom properties).

$ node tools/build-preview.mjs --check
OK: design/preview/index.html matches its generator.

$ node tools/build-mockups.mjs --check
OK: all 4 mockup pages match their generator.

# --- GATE-NO-PLACEHOLDER: ricerca ricorsiva POST-rigenerazione, su tutto il pacchetto ---

$ grep -rn $'\u25c7' .
(nessun output — exit status 1, nessun match)

$ grep -rn "cbx-unit--provisional" .
(nessun output — exit status 1, nessun match)

# --- Verifica di non interferenza con il glifo del brand, distinto (◈ U+25C8 vs ◇ U+25C7) ---

$ grep -c "cbx-app__brand-mark" mockups/dashboard.html tools/build-mockups.mjs
mockups/dashboard.html:5
tools/build-mockups.mjs:2
(il marchio ◈ resta intatto: è un carattere diverso e non riguarda l'unità)

# --- Diff finale, solo file nello scope dichiarato ---

$ cd E:\Git\CobloxNetwork && git status --porcelain -- .lmbrain/design/coblox-design-system
 M .lmbrain/design/coblox-design-system/PRINCIPLES.md
 M .lmbrain/design/coblox-design-system/css/base.css
 M .lmbrain/design/coblox-design-system/mockups/attivita.html
 M .lmbrain/design/coblox-design-system/mockups/dashboard.html
 M .lmbrain/design/coblox-design-system/mockups/onboarding.html
 M .lmbrain/design/coblox-design-system/preview/index.html
 M .lmbrain/design/coblox-design-system/tokens/tokens.json
 M .lmbrain/design/coblox-design-system/tools/build-mockups.mjs
 M .lmbrain/design/coblox-design-system/tools/build-preview.mjs

$ git diff --stat -- .lmbrain/design/coblox-design-system
 .lmbrain/design/coblox-design-system/PRINCIPLES.md | 43 ++++++++++++++++------
 .lmbrain/design/coblox-design-system/css/base.css  | 13 +++----
 .../coblox-design-system/mockups/attivita.html     | 13 +++----
 .../coblox-design-system/mockups/dashboard.html    |  7 ++--
 .../coblox-design-system/mockups/onboarding.html   |  1 -
 .../design/coblox-design-system/preview/index.html | 40 ++++++++------------
 .../design/coblox-design-system/tokens/tokens.json | 11 +++---
 .../coblox-design-system/tools/build-mockups.mjs   |  7 +++-
 .../coblox-design-system/tools/build-preview.mjs   | 16 +++-----
 9 files changed, 79 insertions(+), 72 deletions(-)
```

### Deviations from the specification

- **Scelta di design compiuta dove la spec lasciava aperto (Technical proposal / Risks and open decisions, ultimo punto).** `cr` compatta è usata su **ogni** valore numerico di credit nelle superfici di prodotto (card, tabelle, log, didascalie), non solo nelle cifre "hero"; `credits` estesa è riservata alla prosa discorsiva e alla documentazione. Motivazione registrata in `PRINCIPLES.md` §4.1: la densità da terminale è una delle quattro leve dell'identità visiva dichiarate in §2, e la condizione di revisione di ADR-009 indica esplicitamente `cr` come default finché test con utenti reali non mostrino il contrario. Non è una scorciatoia: è la scelta esplicitamente delegata dalla spec, motivata e registrata come richiesto.
- **Nessuna modifica a `$meta.version` / `$meta.spec` in `tokens.json`.** La spec chiede di aggiornare `$meta`; ho aggiornato il blocco `unit` (rinominato da `unitNamePlaceholder`) ma non ho incrementato il numero di versione né cambiato l'attribuzione `spec: "SPEC-003"`, per restare al perimetro minimo richiesto dai criteri di accettazione. Se il Lead preferisce un bump di versione per marcare la decisione, è una modifica di un rigo che lascio alla review.
- **Segnalazione, non correzione, di testo italiano su superficie di prodotto residuo.** Non ho trovato altro testo italiano *dentro* un componente o artboard oltre alle avvertenze rimosse: il resto dell'italiano nel pacchetto è nota di lavoro interna (note sopra gli artboard, testate `<h1>`/`<title>` delle pagine come "Coblox — Dashboard del nodo", commenti nei generatori), esplicitamente ammesso da `PRINCIPLES.md` §7.1. Non l'ho tradotto, come da vincolo della spec. Se il Lead considera i `<title>`/`<h1>` delle pagine mockup (in italiano, es. "Dashboard del nodo") superficie vista dall'utente finale piuttosto che nota di lavoro, va segnalato come debito separato: non l'ho toccato perché è fuori dal perimetro stretto di questa spec (unità e font) e la spec vieta esplicitamente traduzioni in questa passata.
- **Contestazione tecnica non necessaria.** Non ho trovato assunzioni fragili nella spec da contestare: l'analisi del Lead sul punto di modifica isolato (`UNIT` nei due generatori) era accurata, e l'approccio margine-CSS già esistente per lo spazio fra numero e unità (invece di un carattere spazio letterale) si è rivelato coerente e sufficiente da riusare senza modifiche strutturali.

### Handoff status
- [x] Ready for Project Lead review
