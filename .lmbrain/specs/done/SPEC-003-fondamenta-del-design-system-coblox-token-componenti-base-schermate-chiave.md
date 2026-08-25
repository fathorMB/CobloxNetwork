---
id: SPEC-003
# Note: Quote the title if it contains a colon
title: "Fondamenta del design system Coblox (token, componenti base, schermate chiave)"
status: done
kind: feature
priority: medium
area: design
milestone: M-01
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
related_decisions: [ADR-003]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [design-system, mockups, accessibility]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "set recommended_agent"
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
    action: "attested verification GATE-OPERATOR-LOOK by operator"
  - date: 2026-08-25
    action: "transitioned review -> done"
verification_attestations:
  - actor: "operator"
    actor_role: "operator"
    evidence_digest: "49fcfc2a61832158d0de3e1ca2308f2e71e62ae7a497a3976f16a36fb2f53379"
    evidence_ref: "looks fine"
    id: "SPEC-003-ATTEST-001"
    requirement_digest: "ee98a78df10617e77f7102f3502cc086467237f9e8257507b04b1ea03d8b13ff"
    requirement_id: "GATE-OPERATOR-LOOK"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T12:04:48.315684800+02:00"
---
# Fondamenta del design system Coblox

## Objective

Definire l'identità visiva di Coblox — "hackerosa ma usabile" — come design system concreto: token di design, componenti base e mockup delle tre schermate chiave del client desktop. È il riferimento vincolante per ogni superficie futura (Tauri, Android, sito).

## Context

L'estetica richiesta dall'operatore: look "hacker" figo ma usabile. Direzione già condivisa nel profilo AGENT-006: dark-first, monospace per i dati, accenti fosforescenti, densità da terminale — con leggibilità e accessibilità da prodotto vero. La superficie primaria è l'app desktop Tauri ([ADR-003]), quindi il design system nasce come HTML/CSS; il mapping Compose per Android arriverà in una spec dedicata.

## Scope
### Included
- **Token di design** in `design/tokens/` come CSS custom properties + un JSON sorgente: palette (dark primario + variante light degradabile), scala tipografica (una monospace per dati/numeri e una sans per il testo, con fallback), spaziature, raggi, ombre/glow, durate di animazione.
- **Componenti base** specificati e mostrati in una pagina demo statica `design/preview/index.html`: bottoni, input, card, tabella dati, badge di stato (online/offline/validating), tooltip, notifica/toast, grafico sparkline placeholder, barra di progresso "token".
- **Mockup** (HTML statico o immagini, in `design/mockups/`) di tre schermate desktop: (1) dashboard nodo — guadagni in tempo reale, risorse offerte, eventi; (2) onboarding — creazione identità/chiavi in 3 passi comprensibili a un non tecnico; (3) dettaglio attività — chi sta usando il nodo, quanto frutta, storico mint/burn. Ogni schermata con stati: vuoto, caricamento, errore, offline.
- **Principi** in `design/PRINCIPLES.md`: quando usare il monospace, come si mostrano i numeri di token, lingua e tono del copy, regole di accessibilità.

**Lingua dell'interfaccia: inglese.** Ogni testo visibile all'utente — etichette, bottoni, titoli, errori, stati vuoti, onboarding, microcopy — è in inglese, nella pagina demo e in tutti i mockup. La localizzazione non è in scope: va solo non ostacolata (niente stringhe concatenate a mano, layout con respiro per lingue più lunghe dell'inglese). *Correzione del Lead in corso d'opera, 2026-08-25: la formulazione originale diceva "tono del copy (it/en)" ed era ambigua.*

### Excluded
- Implementazione nella vera app Tauri (spec futura di AGENT-004) e mapping Compose/Android.
- Logo e branding del nome (dipende dalla decisione sul nome del token, aperta in [[STATUS]]).
- Sito pubblico.

## Existing-project analysis

Nessun asset esistente. Vincolo di coerenza futura: i token devono essere consumabili sia da CSS (Tauri) sia, concettualmente, da Compose (nomi semantici, non "colore-schermata-X").

## Technical proposal

Token con nomi semantici a due livelli (primitivi → semantici, es. `--green-500` → `--accent-earning`). La pagina demo non usa framework: HTML+CSS puri, così resta un artefatto di riferimento stabile. Contrasto minimo WCAG AA verificato per ogni coppia testo/sfondo dichiarata legittima. L'estetica "hacker" si gioca su densità, monospace e glow degli accenti — mai su testo a basso contrasto o animazioni che ostacolano la lettura.

## Files and areas involved

- `design/tokens/**`, `design/preview/index.html`, `design/mockups/**`, `design/PRINCIPLES.md`
- *Posizione effettiva dal 2026-08-25:* `.lmbrain/design/coblox-design-system/**` (vedi la nota del Lead sullo spostamento, più sotto).

## Acceptance criteria
- [x] I token esistono in JSON + CSS con nomi semantici; nessun colore hard-coded nella pagina demo fuori dai token.
- [x] La pagina demo mostra tutti i componenti elencati, in tema dark e nella variante light.
- [x] Le tre schermate chiave esistono con i quattro stati (vuoto, caricamento, errore, offline) ciascuna.
- [x] Ogni coppia testo/sfondo dichiarata passa il contrasto WCAG AA; le verifiche sono elencate (strumento + valori).
- [x] PRINCIPLES.md copre: uso del monospace, formattazione dei numeri di token, lingua e tono del copy, regole di accessibilità.
- [x] Tutto il testo visibile all'utente, nella pagina demo e nei mockup, è in inglese.
- [x] Un non-designer (il Lead) riesce a capire dalla dashboard mockup: quanto ho guadagnato oggi, chi mi sta usando ora, se il mio nodo è in salute.

## Implementation plan
1. Moodboard rapida + definizione token (palette, tipografia, spaziature).
2. Componenti base nella pagina demo.
3. Mockup delle tre schermate con stati.
4. PRINCIPLES.md e passata di verifica contrasto.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-CONTRAST | kind=manual | owner=agent | phase=before-submit | evidence=artifact | Tabella dei rapporti di contrasto per ogni coppia testo/sfondo legittima, tutti ≥ AA.
- [x] GATE-OPERATOR-LOOK | kind=operator | owner=operator | phase=before-done | evidence=observation | L'operatore ha visto la pagina demo e i mockup e approva la direzione estetica.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- Rischio: estetica "terminale" che degrada l'usabilità per utenti non tecnici → mitigazione: criterio di accettazione sul test di comprensione del Lead + gate operatore.
- Aperto: nome del token (placeholder "token Coblox" nei mockup, da sostituire quando deciso).
- Aperto: font monospace definitivo (candidati con licenza libera: JetBrains Mono, Fira Code, IBM Plex Mono) — la specialista propone.

## Nota del Lead: spostamento del pacchetto nel brain (2026-08-25)

Su richiesta dell'operatore, i deliverable sono stati spostati da `design/` alla radice del repository a **`.lmbrain/design/coblox-design-system/`**, così che siano visibili nell'app LMBrain. La convenzione di `.lmbrain/design/README.md` prevede una cartella per pacchetto con `index.html` come punto d'ingresso e `manifest.json` opzionale.

Lo spostamento non è una critica al lavoro consegnato: la spec indicava `design/` alla radice, e l'implementatrice ha seguito la spec. La convenzione del brain è emersa dopo.

Eseguito con `git mv` (storia preservata), più due file aggiunti dal Lead per soddisfare la convenzione: `manifest.json` e un `index.html` di pacchetto che rimanda a galleria dei componenti, mockup e principi. Quell'`index.html` è scritto a mano e **non** è prodotto dai generatori: AGENT-006 può assorbirlo in `tools/` quando tocca di nuovo il pacchetto.

Verifiche del Lead dopo lo spostamento:

```text
node tools/build-tokens.mjs   -> 247 custom properties, rigenerazione identica
node tools/check-contrast.mjs -> RESULT: all 130 declared pairs meet WCAG AA
link relativi                 -> file=11 riferimenti=31 rotti=0
classi CSS e variabili usate dal nuovo index.html -> tutte esistenti
```

Gli script continuano a funzionare perché risolvono i percorsi rispetto alla propria posizione (`import.meta.url`), non rispetto alla radice del repository: merito di come sono stati scritti.

### Finding aperto dallo spostamento

- **RF-D01 | category=documentation | severity=low** — Restano circa 54 occorrenze testuali del vecchio percorso `design/...` nel pacchetto: intestazioni generate, commenti d'uso degli script, prose in `README.md` (8), `preview/index.html` (6), i quattro mockup, `PRINCIPLES.md` e `CONTRAST.md`. Non rompono nulla, ma istruiscono il lettore a eseguire per esempio `node design/tools/build-tokens.mjs`, comando che oggi fallisce. La correzione va fatta **nei generatori** e poi rigenerando, non a mano sugli output: per questo è lavoro di AGENT-006 e non del Lead. Condizione di chiusura: nessuna occorrenza del vecchio percorso, e i comandi documentati eseguibili come scritti.
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

Design system completo sotto `design/`, dark-first, senza framework e senza script.

**1. Token (`design/tokens/`)**

- `tokens.json` è la **sorgente unica**: primitivi (`color.green.400`, `space.4`) →
  semantici di ruolo (`color.bg.app`, `color.flow.mint`, `space.card-padding`).
  247 custom properties generate.
- Nomi **neutri rispetto alla piattaforma**, così da poter essere ri-emessi per
  Jetpack Compose (`color.bg.app` → `ColorBgApp`) senza rinominare nulla. Nessun
  nome legato a una schermata.
- `tokens.css` è **generato** da `tools/build-tokens.mjs`. Il generatore impone la
  **parità fra temi** (`assertThemeParity`): se una chiave semantica manca in un
  tema, la build fallisce. Ha trovato un errore reale al primo giro.
- Tema dark di default su `:root`; il light è scoped su `[data-theme="light"]`, quindi
  una regione light può stare dentro un'app dark senza JavaScript. È così che la
  pagina demo mostra i due temi **nello stesso documento, staticamente**.

**2. Componenti (`design/css/` + `design/preview/index.html`)**

Bottoni (5 varianti + disabled), input (testo, dati, invalido, disabled, select),
card e blocchi metrica, tabella dati, badge di stato (online/verifying/degraded/
offline/error) e chip mint/burn, tooltip, toast e notice (4 livelli), sparkline,
barre di proporzione (semplice e mint/burn), scheletri di caricamento, stato vuoto,
flusso eventi, liste chiave/valore, stepper, griglia della frase di recupero,
specimen tipografico, swatch dei token. Tutti presenti **in entrambi i temi**.

**3. Mockup (`design/mockups/`)**

Tre schermate × **cinque** artboard ciascuna (i quattro stati richiesti + lo stato
nominale, necessario per il criterio di comprensione del Lead): 15 artboard totali.

**4. Documentazione**

`design/PRINCIPLES.md` (10 sezioni) e `design/README.md`.

### Files changed

Tutto nuovo, tutto dentro `design/` (più l'evidenza in questa spec). Nessun file
applicativo, nessuna app Tauri, nessun mapping Compose, nessun ADR toccato.

| File | Ruolo |
| --- | --- |
| `design/tokens/tokens.json` | **sorgente** dei token |
| `design/tokens/tokens.css` | generato (247 custom properties) |
| `design/tokens/contrast-pairs.json` | 65 coppie dichiarate legittime + 3 esenzioni motivate |
| `design/tokens/CONTRAST.md` | generato: tabella dei rapporti di contrasto |
| `design/css/base.css` | reset, tipografia, formattazione dati, utility |
| `design/css/components.css` | componenti |
| `design/css/app-shell.css` | telaio desktop, frame onboarding, chrome dei mockup |
| `design/preview/index.html` | generato: galleria componenti, dark + light |
| `design/preview/preview.css` | chrome della sola pagina di riferimento |
| `design/mockups/{index,dashboard,attivita,onboarding}.html` | generati: 15 artboard |
| `design/tools/build-tokens.mjs` | tokens.json → tokens.css (+ `--check`) |
| `design/tools/build-preview.mjs` | → preview/index.html (+ `--check`) |
| `design/tools/build-mockups.mjs` | → mockups/*.html (+ `--check`) |
| `design/tools/check-contrast.mjs` | verifica WCAG 2.1 (+ `--write`) |
| `design/README.md` | come si naviga e si rigenera |
| `design/PRINCIPLES.md` | regole vincolanti del sistema |

### Verification performed

**GATE-CONTRAST — tabella completa in `design/tokens/CONTRAST.md`.**

Strumento: `design/tools/check-contrast.mjs`, implementazione diretta della formula
WCAG 2.1 (luminanza relativa sRGB + `(L1+0.05)/(L2+0.05)`), sorgente
[W3C](https://www.w3.org/TR/WCAG21/#dfn-contrast-ratio). Il rapporto viene
arrotondato *prima* del confronto, così il numero stampato e il verdetto non
possono divergere. Exit code ≠ 0 su qualunque fallimento.

**Risultato: 130 coppie su 130 passano (65 coppie × 2 temi). 0 fallimenti.**

Soglie: testo normale 4,5:1 (WCAG 1.4.3 AA); componenti e grafica portatrice di
significato 3:1 (WCAG 1.4.11). Il sistema tiene **anche il testo "muted" a 4,5:1**,
rinunciando volutamente alla deroga per il testo grande.

Estratto (la tabella completa, con ogni valore esadecimale e ogni rapporto, è in
`design/tokens/CONTRAST.md`):

| Tema | Foreground | Background | Rapporto | Min |
| --- | --- | --- | ---: | ---: |
| dark | `text.primary` #E3EDE8 | `bg.app` #0B100E | 16.02:1 | 4.5 |
| dark | `text.muted` #879C93 | `bg.app` #0B100E | 7.02:1 | 4.5 |
| dark | `accent.primary` #4FE3A3 | `bg.surface` #111815 | 11.34:1 | 4.5 |
| dark | `text.on-accent` #071A13 | `accent.primary-solid` #1FC486 | 8.53:1 | 4.5 |
| dark | `flow.burn` #B39CFF | `flow.burn-quiet` #1E1938 | 6.71:1 | 4.5 |
| dark | `border.strong` #5F776D | `bg.surface` #111815 | 3.53:1 | 3.0 |
| light | `text.primary` #0B100E | `bg.surface` #FFFFFF | 18.72:1 | 4.5 |
| light | `text.muted` #485D54 | `bg.surface` #FFFFFF | 7.75:1 | 4.5 |
| light | `status.validating` #0C6A7A | `bg.inset` #E8EEEB | 5.31:1 | 4.5 |
| light | `text.link` #0A5C3E | `bg.inset` #E8EEEB | 6.83:1 | 4.5 |

Tre esenzioni **dichiarate e motivate** in `contrast-pairs.json`, non silenziose:
bordi decorativi e gridline (non portano informazione da soli, e dove un bordo è
portante si usa `border.strong`/`border.accent`/`focus.ring`, che *sono* verificati);
overlay traslucidi e ombre (non hanno una coppia fissa); anello di focus contro un
controllo pieno (l'anello ha `outline-offset: 2px`, quindi il colore adiacente è
sempre lo sfondo della pagina — ed è quella la coppia che WCAG 1.4.11 valuta, ed è
verificata).

**Verifiche eseguite oltre al gate richiesto**

1. **Contrasto sul DOM renderizzato**, non solo sui token: sweep in browser di tutti
   gli elementi con testo, risolvendo il background effettivo risalendo l'albero e
   applicando la soglia large-text dove dovuto. 475 elementi controllati sulla
   pagina demo. Ha trovato **un fallimento reale che la tabella dei token non
   copriva** (`flow.mint` su `bg.inset` nel tema light, 4.44:1 — il tag `mint` nel
   log). Corretto alzando il token, non abbassando la soglia; poi aggiunte le
   quattro coppie mancanti al file dichiarativo.
2. **Audit strutturale di accessibilità** sul DOM di tutte e 5 le pagine: id
   duplicati, controlli senza nome accessibile, tabelle senza `<caption>` o senza
   `th[scope]`, SVG informativi senza `aria-label`, SVG decorativi senza
   `aria-hidden`, salti di livello nelle intestazioni. Ha trovato **un bug reale**:
   i filtri di `attivita.html` ripetevano gli stessi `id` nei cinque artboard, quindi
   `label[for]` legava solo il primo. Corretto parametrizzando gli id per artboard.
   Esito finale: 0 problemi su tutte le pagine.
3. **Nessun colore hard-coded**: grep su tutti i CSS (escluso `tokens.css`) e su
   tutti gli attributi `style` dell'HTML generato → nessun literal esadecimale/rgb/hsl.
4. **Verifica visiva reale** in browser (server statico locale) di entrambi i temi e
   di tutti e 15 gli artboard, a più larghezze. Ha portato a due correzioni di
   layout: la colonna di dettaglio si schiacciava, e la tabella delle sessioni
   sforava in orizzontale anche a piena larghezza.
5. **Assenza di italiano nell'interfaccia**: sweep in browser sul testo renderizzato
   dentro `.cbx-app`, `.cbx-onboarding` e i 34 blocchi `.gallery__demo`, con un
   dizionario di marcatori italiani → 0 occorrenze su tutte e 5 le pagine.
6. **Non-regressione dei generati**: `--check` su tutti e tre i generatori.

### Verification transcript

```text
$ node design/tools/build-tokens.mjs --check
OK: tokens.css is in sync with tokens.json (247 custom properties).

$ node design/tools/build-preview.mjs --check
OK: design/preview/index.html matches its generator.

$ node design/tools/build-mockups.mjs --check
OK: all 4 mockup pages match their generator.

$ node design/tools/check-contrast.mjs
WCAG 2.1 contrast check — 130 declared pairs across themes: dark, light
[dark]
  PASS  16.02:1  (min 4.5)  color.text.primary #E3EDE8   on color.bg.app #0B100E        text
  PASS  7.02:1   (min 4.5)  color.text.muted #879C93     on color.bg.app #0B100E        text
  ...
[light]
  PASS  5.31:1   (min 4.5)  color.status.validating #0C6A7A  on color.bg.inset #E8EEEB  text
  PASS  6.83:1   (min 4.5)  color.text.link #0A5C3E          on color.bg.inset #E8EEEB  text
RESULT: all 130 declared pairs meet WCAG AA.
(exit code 0 — la tabella integrale e' in design/tokens/CONTRAST.md)

$ grep -nE '#[0-9a-fA-F]{3,8}|rgba?\(|hsl\(' design/css/*.css design/preview/preview.css | grep -v 'var(--cbx'
(none)

--- audit accessibilita' sul DOM (eseguito in browser, per pagina) ---
preview/index.html    {"problems":["OK"],"inputs":8,"tables":2,"svg":4}
mockups/dashboard     {"problems":["OK"],"tables":2,"inputs":0}
mockups/attivita      {"problems":["OK"],"inputs":10,"tables":3}
mockups/onboarding    {"problems":["OK"],"inputs":3}

--- contrasto sul DOM renderizzato (pagina demo, entrambi i temi) ---
{"checked":475,"failCount":0}
(prima della correzione: failCount 2 -> color.flow.mint #0B7C53 su bg.inset #E8EEEB = 4.44:1)

--- assenza di italiano nel testo di interfaccia ---
dashboard.html   {"scopes":5,"italianHits":[]}
attivita.html    {"scopes":5,"italianHits":[]}
onboarding.html  {"scopes":5,"italianHits":[]}
preview/index    {"scopes":34,"italianHits":[]}

--- copertura degli stati ---
dashboard:  Nominale Vuoto Caricamento Errore Offline
attivita:   Nominale Vuoto Caricamento Errore Offline
onboarding: Nominale Vuoto Caricamento Errore Offline
```

### Deviations from the specification

**Modifica di requisito richiesta dal Project Lead in corso d'opera (non una mia
deviazione).** La spec chiedeva "tono del copy (it/en)"; il Lead ha corretto il
requisito mentre il lavoro era in `working`: **la lingua dell'interfaccia è
l'inglese**. Applicato integralmente:

- tutto il testo visibile all'utente — etichette, bottoni, titoli, errori, stati
  vuoti, onboarding, microcopy, `aria-label` e testi per screen reader — è in inglese,
  nella pagina demo e in tutti e 15 gli artboard (verificato, vedi sopra);
- il formato dei numeri è stato adeguato di conseguenza: **separatore decimale
  punto** (`128.40`), mentre il separatore delle migliaia resta lo spazio stretto
  unificatore;
- anche i dati d'esempio sono stati rinominati in inglese (`app:photo-archive`,
  `app:indexer`, `app:open-maps`, `protocol`, "Loft laptop");
- `PRINCIPLES.md` §7 è stato riscritto: §7.1 dichiara l'inglese come lingua
  dell'interfaccia, §7.3 dà le convenzioni (sentence case, mai Title Case; maiuscolo
  solo via CSS così la stringa resta traducibile; punteggiatura; numeri; date ISO e
  orario 24 h; inglese britannico), §7.4 tratta la localizzazione futura;
- l'italiano resta **solo** nelle note di lavoro interne per il team (annotazioni
  attorno agli artboard, commenti nel codice, `PRINCIPLES.md`, `README.md`). Ogni
  pagina di riferimento porta in testa un avviso che dichiara questa separazione,
  così la revisione non la scambia per una svista.
- Localizzazione **non implementata**, come indicato: mi sono limitata a non
  ostacolarla (nessuna stringa concatenata a mano, nessun testo dentro le immagini,
  nessuna larghezza fissa su bottoni ed etichette, maiuscolo via CSS anziché in
  stringa). Nota in `PRINCIPLES.md` §7.4.

**Scelte di implementazione che vanno oltre la lettera della spec, con motivazione**

1. **Le pagine sono generate da script Node** (`design/tools/`), invece di essere
   scritte a mano. La spec impone che la pagina demo sia "HTML e CSS puri, un
   artefatto di riferimento stabile": **l'output committato lo è** — si apre con
   doppio clic, niente framework, niente script, niente `npm install`. I generatori
   servono solo a chi modifica il design system, e risolvono un problema concreto:
   la galleria va mostrata in due temi e 15 artboard devono condividere un solo
   telaio. Duplicare quel markup a mano garantisce che le copie divergano entro
   pochi giorni. Ogni generatore ha `--check` per rilevare il disallineamento.
   *Se il Lead preferisce artefatti scritti a mano, la decisione è reversibile: il
   costo è la duplicazione, e lo segnalo esplicitamente invece di darlo per scontato.*
2. **Quinto artboard "nominale" per ogni schermata**, oltre ai quattro stati
   richiesti. Il criterio di accettazione sulla comprensione del Lead riguarda una
   dashboard *piena*: senza stato nominale non sarebbe verificabile.
3. **Due token in più non previsti** (`feedback.danger-solid`, `text.on-danger`): il
   bottone distruttivo pieno non aveva una coppia che passasse AA senza inventare un
   accostamento non dichiarato.
4. **Container query** (non media query) per la griglia del pannello contenuti: il
   pannello è un riquadro ridimensionabile dentro la finestra, e nei mockup è un
   artboard dentro un documento — la larghezza del viewport non dice nulla di utile.
   Resta una media query come fallback.
5. **`design/css/` e `design/tools/`** non erano nominati nella spec, che elencava
   `tokens/`, `preview/index.html`, `mockups/`, `PRINCIPLES.md`. Sono comunque dentro
   `design/`. Il CSS dei componenti non poteva stare né nei token (che sono valori)
   né duplicato in quattro pagine.

**Scelte di design non ovvie**

- **Il burn è violetto, non rosso.** Spendere non è un errore né una perdita; il
  rosso in questo sistema significa "qualcosa non funziona". Simmetricamente il verde
  del mint significa "emissione", non "bene". Ogni importo colorato porta comunque un
  badge scritto (Minted/Burned): il colore non è mai l'unico canale.
- **Nessun "netto" da nessuna parte.** Emesso e bruciato sono due grandezze
  affiancate, con legenda scritta, più una nota che ricorda il modello mint & burn.
  Un utile netto reintrodurrebbe esattamente la lettura finanziaria che il progetto
  rifiuta ([ADR-005]).
- **`tabular-nums` obbligatorio ovunque.** I valori si aggiornano in tempo reale:
  con cifre proporzionali la riga "balla" mentre la si legge.
- **"Sconosciuto" non è "zero".** In errore i valori non leggibili sono `—` con
  `aria-label` e spiegazione. Mostrare `0.00` sarebbe una bugia, e mostrare un numero
  vecchio senza etichetta lo sarebbe di più.
- **Lo stato offline dichiara la conseguenza economica reale**: senza presenza
  dimostrata il reddito di esistenza è sospeso ([ADR-002]). Era l'occasione più facile
  per essere vaghi.
- **Lo stato vuoto della dashboard non è "vuoto di guadagni"**, ma "vuoto di
  sessioni": il reddito di esistenza matura comunque, e lo stato lo dice.
- **La staleness non si comunica attenuando il testo** (sarebbe una violazione di
  contrasto), ma con etichette d'orario esplicite su ogni cifra.
- **Il maiuscolo dei badge e delle micro-etichette è reso via CSS**, non scritto
  nella stringa: la stringa resta leggibile agli screen reader e traducibile.

### Domande aperte

- **Nome dell'unità**: non inventato, come concordato. Segnaposto `◇` con la classe
  `.cbx-unit--provisional` (sottolineatura tratteggiata) e forma scritta "Coblox
  token". Alla decisione si aggiorna **una sola classe** più `$meta` in `tokens.json`.
- **Font monospace**: propongo **JetBrains Mono** — Apache 2.0 (ridistribuibile con
  l'app), zero barrato, altezza-x generosa, distinzione netta `1/l/I` e `0/O`, che su
  hash e identificativi è decisiva. Alternative valide: IBM Plex Mono (SIL OFL, più
  sobrio); Fira Code (SIL OFL, ma le legature vanno disattivate perché falsano la
  lettura dei dati). **Decisione dell'operatore**: oggi il font è dichiarato con
  fallback di sistema e la pagina non scarica nulla dalla rete.
- **Incorporare i file dei font** nel bundle Tauri: necessario per un rendering
  identico su Windows e Linux, ma è lavoro di un'altra spec.

### Limiti noti

- I font sono **catene di fallback di sistema**: le proporzioni variano da macchina a
  macchina finché i file non vengono incorporati (vedi sopra).
- Il tema light è **completo e verificato** ma è la variante secondaria: ha ricevuto
  meno passate di rifinitura visiva del dark.
- I mockup sono **statici**: nessuna transizione fra stati, nessun comportamento
  interattivo oltre a hover, focus e tooltip in CSS. È l'artefatto di riferimento,
  non un prototipo cliccabile.
- La sparkline è un **placeholder deterministico** con dati sintetici, come previsto
  dalla spec. La specifica del grafico reale (aggregazione, finestra, densità) non
  rientra in questo scope.
- Verificato su un solo motore (Chromium). Il webview di Tauri su Linux è WebKitGTK:
  container query e `:focus-visible` sono supportati nelle versioni correnti, ma una
  verifica su WebKit va fatta quando l'app reale esisterà.
- **GATE-OPERATOR-LOOK non è chiuso**: è dell'operatore, per attestazione dopo la
  consegna. Punti di ingresso per la revisione: `design/preview/index.html` e
  `design/mockups/index.html`, apribili direttamente da disco.

### Handoff status
- [x] Ready for Project Lead review