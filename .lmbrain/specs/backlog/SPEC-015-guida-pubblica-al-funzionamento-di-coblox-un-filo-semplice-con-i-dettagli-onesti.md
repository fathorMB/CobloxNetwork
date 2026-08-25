---
id: SPEC-015
# Note: Quote the title if it contains a colon
title: "Guida pubblica al funzionamento di Coblox: un filo semplice con i dettagli onesti"
status: backlog
kind: feature
priority: medium
area: design
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-006
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-007, ADR-009, ADR-012, ADR-014, ADR-015]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [design-system, documentation, security]
---

# Guida pubblica al funzionamento di Coblox: un filo semplice con i dettagli onesti

## Objective

Una **guida pubblica** che spiega come funziona Coblox a chi non ha mai letto una specifica di protocollo, e che **resta vera nel tempo per costruzione e non per buona volontà**.

Serve due scopi che l'operatore ha chiesto insieme, ed è la loro tensione a definire il lavoro: **onboarding** — chi accende un nodo deve capire cosa sta facendo — e **trasparenza** — chi entra deve sapere cosa il sistema espone di lui, *prima* di esporlo.

## Context

**Perché una pagina semplice è l'artefatto più pericoloso che questo progetto possa scrivere.** Ogni semplificazione è un'affermazione, e `.lmbrain/knowledge/recurring-defects.md` conta **sette occorrenze** di artefatti pubblicati che dicevano cose che le regole non sostenevano. Questa pagina ha tre aggravanti che nessuno degli altri sette aveva: la leggono persone che **non possono verificare**; è scritta per **semplificare**, quindi per dire meno di quanto la regola dica; e sarà **mantenuta nel tempo**, cioè avrà molte occasioni di restare indietro.

«Prevediamo di mantenerla aggiornata» è precisamente la forma di promessa che [ADR-012] esiste per non accettare più: *una prassi si dimentica esattamente nelle passate in cui l'attenzione è altrove.*

**Il meccanismo però esiste già.** `sim/tools/published_artifacts.toml` porta 19 **probe** che ancorano passaggi normativi in prosa e falliscono se spariscono o cambiano forma, e la CI le esegue a ogni push. È lo strumento nato da [SPEC-010] e provato in negativo su dieci classi di difetto.

**I tre impegni scomodi che il progetto ha già preso**, e che una guida onesta deve portare:

- La rete è robusta contro la falsificazione ma **non resistente ai Sybil per via crittografica** ([ADR-007]).
- Chi si abbona a un'app lo scrive **in un registro pubblico e permanente**, accanto a un identificatore stabile ([ADR-014]).
- Il design è **pseudonimo, non anonimo**, e la pseudonimia è stabile quindi debole (TM-28, **ancora aperto**).

## Scope

### Included

- Un pacchetto di design autoconsistente in `.lmbrain/design/<slug>/`, sorella di `coblox-design-system/`, che ne riusa token, CSS e strumenti.
- Una pagina sola: **filo principale semplice**, con blocchi apribili per i dettagli onesti.
- I diagrammi che il filo richiede, costruiti con i token del design system.
- Le **probe di ancoraggio** in `published_artifacts.toml`, una per ogni affermazione di proprietà del filo.

### Excluded

- **Il testo pubblico che attua [ADR-014]**, cioè la dichiarazione formale sugli abbonamenti. È un documento di sicurezza proprio, con un proprio `GATE-SECREVIEW`, e questa guida vi **rimanda** invece di sostituirlo. Scriverli insieme produrrebbe due copie che divergono, che è la famiglia 2.
- Localizzazione. `PROJECT.md` la dichiara fuori scope, ma le scelte non devono precluderla.
- Qualunque modifica a `docs/protocol/`, agli ADR o alle regole. Questa guida **descrive**, non decide. Se scrivendola emergesse che una regola è ambigua o assente, **è un finding da riportare** — è il modo in cui questo progetto ha trovato metà dei propri difetti.

## Existing-project analysis

**Verificato dal Lead il 2026-08-25.**

- Il pacchetto esistente è `.lmbrain/design/coblox-design-system/`, con `PRINCIPLES.md`, `tokens/`, `css/`, `mockups/`, `preview/`, `tools/` e un `manifest.json`. [SPEC-005] ha verificato **130 coppie di contrasto su 130** conformi a WCAG AA, e i tre generatori girano in `--check`: quella barra è la barra.
- `PROJECT.md:54` dichiara che **tutto ciò che vede l'utente finale, documentazione pubblica compresa, è in inglese**. Questa guida è superficie utente per definizione: **la pagina è in inglese**. Gli artefatti del brain restano in italiano.
- L'unità è `credits`, forma compatta `cr` **posposta** al numero, e il separatore delle migliaia è lo spazio stretto insecabile U+202F ([ADR-009], `PRINCIPLES.md` §4.2). Il glifo `◇` è **ritirato** e non va usato. Il Lead ha violato questa regola di persona scrivendo `1,240`, quindi la segnala.
- `published_artifacts.toml` ha 19 probe attive e il loro formato è già stabilito; non se ne inventa uno nuovo.

## Technical proposal

### 1. Il filo, e cosa deve contenere

Una sola pagina, che si legge d'un fiato al livello di un lettore di dieci anni. Le domande a cui deve rispondere, nell'ordine in cui una persona se le pone:

**Che cos'è** — una rete di dispositivi normali, senza un'azienda che la possiede. **Cosa fa il tuo dispositivo** — resta acceso, risponde a domande a sorpresa, custodisce pezzi di dati, esegue calcoli. **Cosa guadagni** — credits, che sono una **misura e non un denaro** e non si convertono. **Cosa ci fai** — abbonamenti alle app; pubblicarne una costa, ospitarla fa guadagnare. **Chi tiene i conti** — i validatori, che ruotano e sono eletti. **Cosa si sa di te.** **Cosa succede se qualcuno bara.**

### 2. La regola che governa i blocchi apribili, ed è la sola che conta

**Il punto scomodo sta nel filo principale, in una frase semplice. L'apribile porta l'esattezza, mai la notizia.**

Il criterio è verificabile e va verificato: **leggendo il filo con tutti i blocchi chiusi, le tre cose scomode devono essere già state dette.** Se una compare solo aprendo, l'apribile è diventato il posto dove si nasconde ciò che imbarazza — che è il modo in cui questa pagina fallisce, e produce marketing con l'etichetta della trasparenza.

### 3. L'ancoraggio, che è ciò che rende la guida manutenibile

**Ogni affermazione di proprietà del filo porta una probe** verso la regola che la tiene. Quando quella regola cambia, la CI diventa rossa **sulla guida**, non su un promemoria.

Un'affermazione di proprietà è una frase che dice che il sistema *garantisce* qualcosa — «nessuno può cancellare i tuoi credits», «i validatori cambiano nel tempo», «gli abbonamenti sono pubblici». Non lo è una frase che descrive un'esperienza — «il tuo computer resta acceso».

**Se un'affermazione non è ancorabile perché nessuna regola la tiene, non è una semplificazione: è un'invenzione**, e va tolta o riscritta finché non lo diventa. Questo è il valore principale dell'esercizio, e la ragione per cui la gate esiste.

### 4. I diagrammi

Costruiti con i token del design system, senza risorse esterne, leggibili in bianco e nero e con la stessa barra di contrasto di [SPEC-005]. Un diagramma è un'affermazione come una frase: **se mostra un flusso, quel flusso è ancorabile**.

## Files and areas involved

- `.lmbrain/design/<slug>/` — il pacchetto nuovo; lo slug è da proporre.
- `.lmbrain/design/coblox-design-system/` — **in sola lettura**: token, CSS e strumenti si riusano, non si modificano.
- `sim/tools/published_artifacts.toml` — le probe nuove.
- `docs/protocol/`, `.lmbrain/decisions/` — **in sola lettura**, sono le fonti da cui il filo deriva.

## Acceptance criteria

- [ ] Esiste un pacchetto di design autoconsistente, senza risorse esterne, che riusa i token esistenti senza modificarli.
- [ ] La pagina è **in inglese**, l'unità è `credits`/`cr` posposta, il separatore delle migliaia è U+202F, e il glifo `◇` non compare.
- [ ] Il filo principale risponde alle sette domande elencate, nell'ordine in cui una persona se le pone.
- [ ] **Leggendo il filo con tutti i blocchi chiusi**, le tre cose scomode di *Context* sono già dette.
- [ ] **Ogni affermazione di proprietà del filo ha una probe** in `published_artifacts.toml` verso la regola che la tiene, e l'elenco affermazione → regola è scritto nell'evidenza.
- [ ] Nessuna affermazione priva di regola è rimasta: quelle non ancorabili sono state tolte o riscritte, e l'elenco di ciò che è stato tolto è riportato.
- [ ] I diagrammi usano i token, non hanno risorse esterne, e rispettano la barra di contrasto di [SPEC-005].
- [ ] La guida **rimanda** al testo pubblico di [ADR-014] invece di duplicarlo.
- [ ] `published_artifacts.py` e la sua prova in negativo passano.

## Implementation plan

1. Ricavare dalle fonti l'elenco delle affermazioni che il filo vuole fare, **prima di scrivere la pagina**, e per ciascuna la regola che la tiene. Ciò che resta senza regola non entra.
2. Scrivere il filo, tenendo i punti scomodi nel filo e non negli apribili.
3. Costruire i diagrammi sui token esistenti.
4. Scrivere le probe e provarle in negativo.
5. Rileggere il filo con tutti i blocchi chiusi e verificare il criterio delle tre cose scomode.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-CLAIMS-ANCHORED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Ogni affermazione di proprietà del filo ha la propria probe, l'elenco affermazione → regola è nell'evidenza, e **almeno una probe è provata in negativo** cambiando la regola a cui punta e osservando la guida diventare rossa. Una probe che non si è mai vista fallire non è un ancoraggio: è un commento.
- [ ] GATE-HARD-TRUTHS-IN-THE-THREAD | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il filo principale, letto con **tutti i blocchi apribili chiusi**, dice già le tre cose scomode di *Context*. Incollare il testo del solo filo. È il criterio che distingue questa guida da un testo di marketing, e non è sostituibile da un conteggio di menzioni.
- [ ] GATE-NOTHING-INVENTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | L'elenco delle affermazioni **tolte o riscritte** perché nessuna regola le teneva è riportato, anche se vuoto. Un elenco vuoto è un'informazione; un elenco assente è una domanda non posta.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha riletto le affermazioni di sicurezza nella loro forma semplificata e il Lead ha accettato la review. Una posizione di sicurezza detta in parole semplici è la forma in cui è più facile prometterla più forte di com'è, ed è la sua materia da quattro spec.
- [ ] GATE-OPERATOR-LOOK | kind=operator | owner=operator | phase=before-done | evidence=observation | L'operatore ha letto la guida e giudicato il tono. È un giudizio che non spetta né al Lead né all'implementatrice: la pagina esiste per parlare a chi entra, e se il tono è sbagliato nessuna verifica meccanica se ne accorge. Il precedente è [SPEC-003], dove questa gate ha funzionato come doveva.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è l'apribile che diventa un nascondiglio.** È l'unico modo in cui questa pagina può sembrare riuscita ed essere fallita, perché soddisferebbe ogni criterio meccanico. `GATE-HARD-TRUTHS-IN-THE-THREAD` esiste per questo e chiede il testo, non un conteggio.
- **Il rischio secondario è la semplificazione che promette più del vero.** Dire «i tuoi credits sono al sicuro» è più semplice e più falso di «nessuno può toglierteli, ma chi ti guarda vede quanto ne hai». La regola: **se la versione semplice è più forte di quella esatta, la versione semplice è sbagliata.**
- **Una dichiarazione del Lead sulla dimensione.** Il `capability_tier` è definito come impronta del cambiamento, e per quella definizione questa spec sarebbe `terra`. È dichiarata `sol` per la **difficoltà**: la tensione fra semplicità e onestà è il lavoro, non la quantità di file. La deviazione è detta e non è una svista.
- **Sulla milestone.** Questa guida non appartiene all'esito di alcuna milestone, ed è assegnata a M-02 perché è la corrente. La ragione per iniziarla ora invece che a M-08: è un artefatto vivo, e cominciarlo tardi significa scriverlo su un sistema che nessuno ricorda più di aver progettato.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete content.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Se scrivendo il filo trovi che una regola è ambigua o assente, riportalo invece di aggirarlo scrivendo bene.** Metà dei difetti trovati in questo progetto sono emersi così: qualcuno ha provato a dire una cosa con precisione e ha scoperto che non era scritta da nessuna parte.
- **Contestare le formulazioni del Lead fa parte del mandato**, e in questa spec l'elenco delle sette domande è una proposta del Lead e non un vincolo: se l'ordine sbagliato è quello, dillo.
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
