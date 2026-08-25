---
id: SPEC-015
# Note: Quote the title if it contains a colon
title: "Guida pubblica al funzionamento di Coblox: un filo semplice con i dettagli onesti"
status: review
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
updated: 2026-08-26
tags: [design-system, documentation, security]
activity:
  - date: 2026-08-25
    action: "transitioned backlog -> ready"
  - date: 2026-08-25
    action: "transitioned ready -> working"
  - date: 2026-08-26
    action: "transitioned working -> review"
  - date: 2026-08-26
    action: "attested verification GATE-OPERATOR-LOOK by operator"
  - date: 2026-08-26
    action: "attested verification GATE-OPERATOR-LOOK by operator (out-of-band, recorded by AGENT-LEAD via conversation)"
verification_attestations:
  - actor: "operator"
    actor_role: "operator"
    evidence_digest: "c0e17f2b1b6f7aa562fe9974f642b55ba0a984ccffe133b01746cd6247dab92c"
    evidence_ref: "like it, approved"
    id: "SPEC-015-ATTEST-001"
    requirement_digest: "ed00ac596273dd4fd541f2d5dd14e33b8cc6628ef0650d51cb6cbe3a1856c772"
    requirement_id: "GATE-OPERATOR-LOOK"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-26T00:24:48.873546700+02:00"
  - actor: "Moreno Bruschi"
    actor_role: "operator"
    delegated_by: "AGENT-LEAD"
    delegation_authorization: "attestata, mi piace per il momento"
    delegation_channel: "conversation"
    evidence_digest: "0a4511e60f70269631000e0244b596dd8c2b7b8bc6b223fc01d2c5936af51d11"
    evidence_ref: "L'operatore ha letto la guida aperta nel pannello di anteprima dal file .lmbrain/design/coblox-public-guide/index.html e ha attestato la gate in conversazione. Il giudizio e sul tono ed e suo: la gate esiste perche nessuna verifica meccanica se ne accorge, e ne il Lead ne l'implementatrice possono darlo al posto suo. La formula usata, per il momento, e registrata cosi com'e: e un'attestazione sullo stato attuale della pagina e non un giudizio definitivo su una guida che per sua natura sara riscritta nel tempo."
    id: "SPEC-015-ATTEST-002"
    requirement_digest: "ed00ac596273dd4fd541f2d5dd14e33b8cc6628ef0650d51cb6cbe3a1856c772"
    requirement_id: "GATE-OPERATOR-LOOK"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-26T00:25:29.998809600+02:00"
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

- [x] Esiste un pacchetto di design autoconsistente, senza risorse esterne, che riusa i token esistenti senza modificarli.
- [x] La pagina è **in inglese**, l'unità è `credits`/`cr` posposta, il separatore delle migliaia è U+202F, e il glifo `◇` non compare.
- [x] Il filo principale risponde alle sette domande elencate, nell'ordine in cui una persona se le pone.
- [x] **Leggendo il filo con tutti i blocchi chiusi**, le tre cose scomode di *Context* sono già dette.
- [x] **Ogni affermazione di proprietà del filo ha una probe** in `published_artifacts.toml` verso la regola che la tiene, e l'elenco affermazione → regola è scritto nell'evidenza.
- [x] Nessuna affermazione priva di regola è rimasta: quelle non ancorabili sono state tolte o riscritte, e l'elenco di ciò che è stato tolto è riportato.
- [x] I diagrammi usano i token, non hanno risorse esterne, e rispettano la barra di contrasto di [SPEC-005].
- [x] La guida **rimanda** al testo pubblico di [ADR-014] invece di duplicarlo. *(La guida rimanda e non duplica. Il testo pubblico però **non esiste ancora**: vedi finding F1. La pagina lo dice esplicitamente invece di collegare un documento introvabile.)*
- [x] `published_artifacts.py` e la sua prova in negativo passano.

## Implementation plan

1. Ricavare dalle fonti l'elenco delle affermazioni che il filo vuole fare, **prima di scrivere la pagina**, e per ciascuna la regola che la tiene. Ciò che resta senza regola non entra.
2. Scrivere il filo, tenendo i punti scomodi nel filo e non negli apribili.
3. Costruire i diagrammi sui token esistenti.
4. Scrivere le probe e provarle in negativo.
5. Rileggere il filo con tutti i blocchi chiusi e verificare il criterio delle tre cose scomode.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-CLAIMS-ANCHORED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Ogni affermazione di proprietà del filo ha la propria probe, l'elenco affermazione → regola è nell'evidenza, e **almeno una probe è provata in negativo** cambiando la regola a cui punta e osservando la guida diventare rossa. Una probe che non si è mai vista fallire non è un ancoraggio: è un commento.
- [x] GATE-HARD-TRUTHS-IN-THE-THREAD | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il filo principale, letto con **tutti i blocchi apribili chiusi**, dice già le tre cose scomode di *Context*. Incollare il testo del solo filo. È il criterio che distingue questa guida da un testo di marketing, e non è sostituibile da un conteggio di menzioni.
- [x] GATE-NOTHING-INVENTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | L'elenco delle affermazioni **tolte o riscritte** perché nessuna regola le teneva è riportato, anche se vuoto. Un elenco vuoto è un'informazione; un elenco assente è una domanda non posta.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha riletto le affermazioni di sicurezza nella loro forma semplificata e il Lead ha accettato la review. Una posizione di sicurezza detta in parole semplici è la forma in cui è più facile prometterla più forte di com'è, ed è la sua materia da quattro spec.
- [x] GATE-OPERATOR-LOOK | kind=operator | owner=operator | phase=before-done | evidence=observation | L'operatore ha letto la guida e giudicato il tono. È un giudizio che non spetta né al Lead né all'implementatrice: la pagina esiste per parlare a chi entra, e se il tono è sbagliato nessuna verifica meccanica se ne accorge. Il precedente è [SPEC-003], dove questa gate ha funzionato come doveva.

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

**Il pacchetto.** `.lmbrain/design/coblox-public-guide/`, sorella di
`coblox-design-system/`, che ne carica token, CSS e componenti per percorso
relativo senza copiarne né modificarne un byte. Una pagina sola, in inglese,
senza framework, senza script, senza una sola richiesta di rete: si apre da
disco con un doppio clic, come le altre superfici del design system.

**Il filo.** Sette sezioni, una per domanda, nell'ordine proposto dal Lead. Ogni
sezione è prosa continua; i dettagli esatti stanno in otto blocchi
`<details>` chiusi di default. La regola di [SPEC-015] §2 è stata applicata
letteralmente: **il punto scomodo sta nel filo, in una frase semplice;
l'apribile porta l'esattezza, mai la notizia.**

Le tre cose scomode non sono raccolte in §06 e basta: ciascuna è detta **nel
momento in cui diventa rilevante**, e §06 le ricapitola sapendo di ricapitolare.
Sybil in §01, dove un lettore forma l'idea di «rete sicura»; saldo pubblico e
pseudonimo stabile in §03, dove per la prima volta possiede qualcosa; abbonamenti
pubblici e permanenti in §04, prima della frase che descrive come ci si abbona.
La forma è un blocco `.plainly` — riga verticale marcata, micro-etichetta
*Plainly*, nessun colore che porti significato da solo.

**I diagrammi.** Due SVG inline, costruiti sui soli token semantici, senza
risorse esterne:

1. *Where a credit comes from and where it goes* — due frecce e una terza
   tracciata **barrata**, perché il trasferimento fra persone non esiste come
   forma nel registro. Il tratto porta anche uno stile di linea proprio (pieno,
   tratteggiato, punteggiato) e ogni nodo la sua etichetta scritta: in scala di
   grigi il diagramma resta leggibile.
2. *What the shared register holds about you* — la riga del registro pubblico
   con le sue cinque colonne, e sotto **due** riquadri: ciò che non viene mai
   chiesto, e ciò che non è scritto ma è comunque visto. Il secondo esiste
   perché è la cosa che si sbaglia di solito.

**L'ancoraggio, nelle due direzioni.** 65 probe nuove in
`sim/tools/published_artifacts.toml`, in coda e senza toccare nulla di
esistente, una per ogni affermazione di proprietà. Ogni probe porta anche il
campo `claims` con la frase della guida che sostiene.

- **regola → pagina**: `published_artifacts.py` esce ≠ 0 nominando la frase
  della guida quando la regola cambia. Gira già in CI.
- **pagina → regola**: `check-guide-pairs.mjs` esce ≠ 0 quando la frase cambia e
  la probe resta orfana. È il verso che il manifesto da solo non copre, ed è la
  differenza fra un ancoraggio e un commento.

**Lo strumento del pacchetto.** `tools/check-guide-pairs.mjs`, sei classi, tutte
provate in negativo: nessun colore letterale, nessun token inventato, nessun
accostamento fuori dalle 130 coppie verificate di [SPEC-005], le regole di
[ADR-009] (unità posposta, separatore U+202F, glifo `◇` assente), assenza di
rete e di script, e la verifica `claims` di cui sopra. Non ricalcola i rapporti
di contrasto: quello resta il mestiere di `check-contrast.mjs`, e riscriverne la
formula avrebbe prodotto la seconda copia della famiglia 2.

### Files changed

Nuovi:

- `.lmbrain/design/coblox-public-guide/index.html` — la guida
- `.lmbrain/design/coblox-public-guide/guide.css` — il solo strato di pagina
- `.lmbrain/design/coblox-public-guide/used-pairs.json` — gli accostamenti impegnati
- `.lmbrain/design/coblox-public-guide/tools/check-guide-pairs.mjs` — le sei verifiche
- `.lmbrain/design/coblox-public-guide/README.md` — note di lavoro (italiano)

Modificati:

- `sim/tools/published_artifacts.toml` — **solo aggiunte in coda**: un blocco di
  commento e 65 `[[probe]]`, +533 righe, 0 righe rimosse o riordinate. Verificato
  con `git diff --stat`.

In sola lettura e verificati intatti (`git status --porcelain` vuoto):
`docs/protocol/`, `.lmbrain/decisions/`, `.lmbrain/design/coblox-design-system/`.

### Verification performed

| Verifica | Esito |
| --- | --- |
| `check-guide-pairs.mjs` — 6 classi, 131 candidati | PASS |
| ogni classe di `check-guide-pairs.mjs` provata in negativo | 6/6 osservate fallire |
| `published_artifacts.py` — C10 passa da 19 a 84 probe | PASS |
| tre regole mutate, la guida diventa rossa nominando le frasi | 3/3 osservate fallire |
| `published_artifacts_negative.py` — le 10 classi storiche | PASS |
| `check-contrast.mjs` del design system | 130/130 WCAG AA |
| generatori del design system in `--check` | 3/3 in sync |
| filo con tutti i blocchi chiusi, tre cose scomode | 3/3 presenti |
| resa: token risolti, temi dark e light, `details` chiusi di default | verificato su stile calcolato |
| geometria: nessun testo esce dal proprio riquadro o dal `viewBox` | 0 sconfinamenti |
| nessuno scorrimento orizzontale di pagina a 192, 375 e 1280 px | 0 |

**Nota sul modo.** La verifica visiva è stata fatta leggendo lo **stile
calcolato** e la **geometria misurata** nel browser, non guardando uno
screenshot: il riquadro di anteprima di questa sessione non compone i fotogrammi
e ogni tentativo di cattura è scaduto. È una verifica più forte per ciò che
misura — un token risolto o un `getBBox()` non sono opinioni — e più debole per
il colpo d'occhio, che è esattamente ciò che `GATE-OPERATOR-LOOK` esiste per
coprire. La pagina è stata servita su `http://127.0.0.1:8731` da un server
statico temporaneo, spento a fine lavoro; nessun file di progetto lo riguarda.

Due difetti di resa sono stati trovati così e corretti: la legenda del primo
diagramma usciva dal `viewBox` di 20 px, e tre sotto-etichette uscivano dal
proprio riquadro. Inoltre i diagrammi, scalati a 293 px su uno schermo da
telefono, riducevano il proprio testo a 4,9 px: la figura ora **scorre** sotto i
39 rem invece di scalare le etichette fino a renderle inutili, dentro un
contenitore focalizzabile perché una regione che scorre deve scorrere anche da
tastiera (WCAG 2.1.1).

### GATE-HARD-TRUTHS-IN-THE-THREAD — il filo con tutti i blocchi chiusi

Testo estratto meccanicamente rimuovendo ogni `.detail__body`, cioè esattamente
ciò che un lettore vede senza aprire nulla. I diagrammi sono segnati
`[DIAGRAM]`; i titoli dei blocchi apribili restano perché restano visibili.

Le tre cose scomode, e dove cadono:

1. **Non resistente ai Sybil per via crittografica** — §01, blocco *Plainly*:
   «It cannot tell a thousand devices apart from one computer pretending to be a
   thousand devices. That is a deliberate and permanent property of this version,
   not a gap waiting for a patch.»
2. **Abbonamenti pubblici e permanenti** — §04, blocco *Plainly*: «A subscription
   is a public and permanent fact. […] The register is immutable, so there is no
   way to take it back later.»
3. **Pseudonimo e non anonimo** — §03, blocco *Plainly*: «The design is
   pseudonymous, not anonymous. A stable pseudonym is a weak one.»

Nessuna delle tre compare per la prima volta in §06. §06 le raccoglie e lo
dichiara: *«The three uncomfortable facts on this page have already been said».*

```text
How Coblox works

Coblox · public guide

How Coblox works

Coblox is a network made of ordinary devices. This page explains what it does, what your

device does inside it, and what it records about you — without assuming you have

read a protocol specification.

Read it straight through and you will have the whole shape of it. The blocks marked

+ open on the exact version of something the paragraph above has

already told you. They are for people who want the mechanism; nothing that matters

to a decision is hidden inside one.

01

What is Coblox?

It is a network of ordinary machines — laptops, desktops, spare computers,

Android phones — that keep pieces of each other's data and run small programs

for each other. No company owns it. There is no central server that everything passes

through; if you switch your device off, the network is smaller and nothing else

changes.

The devices share one register of who did what. A rotating group of them, called

validators, write the register; everybody else can read it and check the writing.

The programs that run on the network are called apps, and anybody can publish one.

Plainly

Coblox is good at stopping forgery and not at stopping crowds. It can prove that a

balance was not invented, that a signature is genuine, that nothing was spent twice.

It cannot tell a thousand devices apart from one computer pretending to be a

thousand devices. That is a deliberate and permanent property of this version, not a

gap waiting for a patch.

Why the crowd problem is not solved, and what is used instead

02

What does your device actually do?

It stays on and connected, and it answers questions it cannot see coming. Another node

asks it to prove it is reachable, or to prove it still holds a piece of data it agreed

to keep, or to run a small computation and hand back the result. Each question has a

deadline. An answer that does not arrive in time counts as an answer that did not

arrive.

You do not choose which questions you get, and neither does the node asking them. The

asker has to lock in its secret before the randomness that picks the question exists,

so it cannot go looking for a question you happen to be able to answer.

On a phone this is meant to be survivable: the work is sampled rather than continuous,

and the network is not allowed to reward a device for simply burning more electricity

than its neighbour.

How a question is made unpredictable, and where that stops

03

What do you get for it?

Credits. They are written the way a measurement is written —

1 240 cr , with the unit

after the number, the way you write 512 GB — because that is what they are. A credit

records that the network used your machine. There is no transaction in Coblox that

takes credits from one person and gives them to another: that shape does not exist in

the register, so credits cannot be sent, and they are not a currency the network can

move around.

You are paid in two ways. Some of it is for work that was checked: storage you proved

you still hold, computation you actually ran. Some of it is for being present and

reachable at all, and that part is a fixed pot for each period, divided among everyone

who qualified.

Plainly

Your balance is public. It sits in a register that anyone can read, next to your node

identifier, permanently. Nobody can take credits away from you — the only

entries that reduce your balance are ones you signed yourself — but

“nobody can take them” and “nobody can see them” are

different sentences, and only the first one is true.

And that identifier is a pseudonym, not a disguise. It carries no name, no email

address and no postal address, but it never changes for as long as you keep the same

key, so everything it ever does stays tied together under it. The design is

pseudonymous, not anonymous. A stable pseudonym is a weak one.

[DIAGRAM]

Credits only ever appear against something the network verified, and only ever

disappear when they are spent. There is no third arrow. A burn destroys the credits

rather than handing them to anyone: whoever provided the service is paid separately,

by the protocol, against proof.

What “a fixed pot, divided” means for the amount you see

Why “nobody can take your credits” is exactly, and only, true

04

What can you do with them?

You subscribe to apps. An app declares its own subscription price; you pay it, and the

credits are destroyed rather than handed over. The people whose machines actually host

the app are paid separately by the protocol, once they have proved they did the work.

Plainly

A subscription is a public and permanent fact. Paying for one writes your node

identifier, the app, the amount and the dates into the shared register, where they

stay for ever and can be read by anyone, and joined up with everything else that

identifier ever did. The register is immutable, so there is no way to take it back

later. Credits having no monetary value does not change this: what is exposed is not

money, it is what you use.

Publishing an app is the other direction. You fund an account attached to the app, and

the network bills that account for hosting at a price the validators set — a

publisher cannot set or lower its own hosting price. If the account runs dry the app

goes into a grace period, then is suspended; it is never silently deleted, and funding

it brings it back. If people subscribe to your app, you are paid for that separately,

capped so that you cannot profitably subscribe to yourself.

An app you install gets nothing by default. It can only touch what its manifest asks

for and you agreed to: a private folder of its own, a monotonic clock, a fixed list of

HTTPS addresses. Anything not asked for is denied, and asking is not the same as

getting.

The full statement of what the network publishes about you

05

Who keeps the books?

A group of nodes called validators. A block of entries counts as settled when more than

two thirds of their voting power has signed it — strictly more, not two thirds

exactly.

They are not a permanent committee. Every seat carries an expiry stamped on it when it

is filled, so a set with V members and terms of

T periods gives up at least

V divided by T seats every

period, whatever anybody intends. A departing member sits out a cooling-off period and

then competes for re-entry like anyone else. Nobody can be made a validator without

standing for it, and the sitting validators cannot name their successors: they can

narrow the field of candidates, but the composition comes out of a derivation from

randomness they cannot choose.

Getting in is a threshold, not a league table. You qualify by having proved storage and

computation — not by having been switched on, which is what a rented server in a

data centre beats a real phone at. Above the threshold, more storage buys nothing: no

seat, no better odds, no extra weight. Every validator counts as exactly one.

Why the threshold is deliberately not a ranking

06

What does the network know about you?

The three uncomfortable facts on this page have already been said, and they belong

together, so here they are in one place: the network cannot tell a crowd of fake

devices from real ones; your balance and your subscriptions are public and permanent;

and your identifier is a stable pseudonym, which is a weak one. None of the three is a

defect being worked on. They are properties of the design, and the reason they are on

this page is that you cannot decide about them after you have joined.

One thing that is not in the register is worth stating precisely, because it is easy to

over-promise. Your network address is never written into the shared register, and the

key your device uses to make connections is deliberately a different key from the one

that identifies you, so that nobody can work out one from the other by reading the

register alone. But every node you actually talk to sees the address you are connecting

from, and your messages carry your node identifier in the clear. The register does not

publish the link. Participating exposes it.

[DIAGRAM]

The distinction that matters is not between secret and public but between what the

register publishes and what taking part reveals. The second box is the one people

usually get wrong.

What the register does not let you undo

07

What happens if somebody cheats?

Mostly, the cheating simply does not count. Evidence of work that fails any of its

checks cannot pay anybody, and every reader can run those checks, not only validators.

A block that contains a single entry which does not execute is rejected whole: there

are no half-applied blocks.

There is no punishment that takes credits away. The register has a closed list of entry

types and none of them confiscates. Cheating costs you what you would have been paid,

and a validator that misbehaves can have its identity revoked by the others and loses

its seat — then has to sit out the cooling-off period like any departing member.

The kind of cheating the design does not stop is the kind described in the first

section: not forging anything, just being many. That is why the pot for presence income

is capped and shared, and why becoming a validator is anchored to work that is hard to

fake rather than to being switched on.

What a device with very little power can check for itself

About this page

Every sentence on this page that says the system guarantees something is tied to

the rule in the protocol specification that holds it, by an automated check that runs on

every change. If a rule is changed or removed, the check fails and names this page. That

is the whole of the maintenance promise: there is no promise to remember.

Where a rule holds less than a simple sentence would suggest, this page says the smaller

thing. The test used throughout: if the simple version is stronger than the exact one,

the simple version is wrong.

The protocol specification is in docs/protocol/ .

Known security limitations are in SECURITY.md , which is

also where to report one you have found.
```

### GATE-CLAIMS-ANCHORED — affermazione → regola

65 affermazioni di proprietà, 65 probe. La colonna *Guide sentence* è il campo
`claims` della probe, ed è la stessa stringa che `check-guide-pairs.mjs`
richiede di ritrovare nella pagina.

| # | Guide sentence (the probe's `claims` field) | Rule that holds it | Probe id |
| ---: | --- | --- | --- |
| 1 | It can prove that a balance was not invented, that a signature is genuine, that nothing was spent twice. | `identity.md` &mdash; `while not being Sybil-resistant by cryptographic means` | `guide-sybil-not-cryptographic` |
| 2 | It cannot tell a thousand devices apart from one computer pretending to be a thousand devices. | `identity.md` &mdash; `does not distinguish 'N' emulated nodes on one host from 'N'` | `guide-emulated-nodes-indistinguishable` |
| 3 | That is a deliberate and permanent property of this version, not a gap waiting for a patch. | `identity.md` &mdash; `MUST NOT be described as a temporary gap` | `guide-sybil-gap-is-permanent` |
| 4 | Answering "are you there?" proves that a key is online, not that a device exists. | `identity.md` &mdash; `proves that \*a key is online\*, not that \*a device exists\*` | `guide-availability-proves-a-key-not-a-device` |
| 5 | Joining costs a one-time proof of work: your device has to do a deliberately expensive memory-heavy calculation before it is enrolled. | `identity.md` &mdash; `The primitive is \*\*Argon2id\*\*` | `guide-enrolment-is-memory-hard` |
| 6 | A one-time cost cannot price something that pays out for ever. | `identity.md` &mdash; `\*\*A one-time cost cannot price a perpetual flow\.\*\*` | `guide-one-time-cost-cannot-price-a-flow` |
| 7 | the reward for merely existing is a fixed pot shared out among everyone who qualifies, not an amount per device | `ledger.md` &mdash; `Existence income is \*\*not\*\* a fixed amount per node` | `guide-existence-income-is-a-shared-pot` |
| 8 | The remainder is not minted at all, so the period's presence emission is capped by construction rather than by anyone's restraint. | `ledger.md` &mdash; `emission for an epoch is therefore at most 'F' by construction, not by` | `guide-existence-fund-capped-by-construction` |
| 9 | that total divided by their number, rounded down | `ledger.md` &mdash; `The remainder is \*\*not\*\* minted and is not carried forward` | `guide-remainder-is-not-minted` |
| 10 | Ten fake devices do not create more credits; they only take smaller shares of the same pot | `ledger.md` &mdash; `fleet can only dilute the share of honest nodes` | `guide-fleet-dilutes-rather-than-mints` |
| 11 | an inflated count can be contradicted by recomputation instead of merely disputed | `ledger.md` &mdash; `the root makes the count \*\*falsifiable\*\*` | `guide-eligible-count-is-falsifiable` |
| 12 | prove it still holds a piece of data it agreed to keep, or to run a small computation and hand back the result | `ledger.md` &mdash; `"kind":"availability"\\|"storage"\\|"compute"` | `guide-challenge-kinds` |
| 13 | An answer that does not arrive in time counts as an answer that did not arrive. | `ledger.md` &mdash; `"outcome":"passed"\\|"failed"\\|"late"\\|"no_response"` | `guide-challenge-outcomes-include-late` |
| 14 | The asker has to lock in its secret before the randomness that picks the question exists | `ledger.md` &mdash; `was fixed before the beacon existed` | `guide-secret-fixed-before-randomness` |
| 15 | reader — not only validators — recomputes all of it | `ledger.md` &mdash; `not only validators — MUST recompute` | `guide-every-verifier-recomputes` |
| 16 | evidence that fails any step cannot pay anybody | `ledger.md` &mdash; `Evidence failing any of these is invalid and cannot back a mint\.` | `guide-bad-evidence-pays-nobody` |
| 17 | the attack is reduced from "pass the question" to "pass one of two" | `ledger.md` &mdash; `to "pass one of two"` &times;2 | `guide-grinding-degrades-to-one-of-two` |
| 18 | the network is not allowed to reward a device for simply burning more electricity than its neighbour | `ledger.md` &mdash; `is mining wearing a different hat` | `guide-paying-more-for-more-is-mining` |
| 19 | There is no transaction in Coblox that takes credits from one person and gives them to another | `ledger.md` &mdash; `A direct user-to-user transfer is therefore unrepresentable` | `guide-no-user-to-user-transfer` |
| 20 | they are not a currency the network can move around | `ledger.md` &mdash; `It is not a transferable currency\.` | `guide-not-a-currency` |
| 21 | The register has a closed list of entry types and none of them confiscates. | `ledger.md` &mdash; `"kind":"mint"\\|"burn"\\|"fund_app"\\|"challenge_commitment"\\|"challenge_evidence"` | `guide-closed-transaction-kinds` |
| 22 | Credits only ever appear against something the network verified, and only ever disappear when they are spent. | `ledger.md` &mdash; `'mint' is the only supply-increasing transaction` | `guide-mint-and-burn-are-the-only-supply-changes` |
| 23 | Work the network checked | `ledger.md` &mdash; `every mint is linked to finalized, validator-verifiable eligibility evidence` | `guide-mint-needs-checkable-evidence` |
| 24 | Some of it is for work that was checked: storage you proved you still hold, computation you actually ran. | `ledger.md` &mdash; `evidence MUST establish the measured resource contribution` | `guide-work-mint-needs-measured-contribution` |
| 25 | the only entries that reduce your balance are ones you signed yourself | `ledger.md` &mdash; `the key MUST derive 'payer_node_id'; the signature is` | `guide-subscription-burn-needs-your-signature` |
| 26 | paying for an app subscription, and moving credits into an app's funding account | `ledger.md` &mdash; `The key MUST derive the enrolled, unrevoked 'payer_node_id'\.` | `guide-funding-an-app-needs-your-signature` |
| 27 | the validators spend them on that app's hosting on a schedule, without asking you again | `ledger.md` &mdash; `quorum authorizes the deterministic charge, and the app escrow is debited` | `guide-escrow-is-debited-without-asking-again` |
| 28 | There is no entry that confiscates, and there is no fee. | `ledger.md` &mdash; `transactions or fees in v0` | `guide-no-fees` |
| 29 | A burn destroys the credits rather than handing them to anyone: whoever provided the service is paid separately, by the protocol, against proof. | `ledger.md` &mdash; `the burn never names or credits a provider` | `guide-burn-pays-nobody` |
| 30 | Your balance is public. It sits in a register that anyone can read, next to your node identifier, permanently. | `wire.md` &mdash; `"account_kind":"node"\\|"app", "subject_id":string,` | `guide-anyone-can-ask-for-a-balance` |
| 31 | next to your node identifier, permanently | `ledger.md` &mdash; `account_key = H\("coblox-account-key-v0\\0" \\|\\| 0x00 \\|\\| node_id_utf8\)` | `guide-account-key-is-your-node-id` |
| 32 | It carries no name, no email address and no postal address, but it never changes for as long as you keep the same key | `identity.md` &mdash; `It is stable for the life of the key, case-sensitive, and contains no account or` | `guide-node-id-is-a-stable-pseudonym` |
| 33 | The design is pseudonymous, not anonymous. A stable pseudonym is a weak one. | `identity.md` &mdash; `the enrollment certificate publishes the` | `guide-identity-key-is-on-the-ledger` |
| 34 | Paying for one writes your node identifier, the app, the amount and the dates into the shared register | `ledger.md` &mdash; `"reason":"app_hosting"\\|"app_subscription"` | `guide-subscription-burn-names-the-payer` |
| 35 | and joined up with everything else that identifier ever did | `ledger.md` &mdash; `group them by payer node ID` | `guide-subscribers-are-grouped-by-identity` |
| 36 | The register is immutable, so there is no way to take it back later. | `ledger.md` &mdash; `cannot erase evidence or consensus history` | `guide-history-cannot-be-erased` |
| 37 | a publisher cannot set or lower its own hosting price | `ledger.md` &mdash; `publisher cannot supply or lower a hosting rate\.` | `guide-publisher-cannot-price-hosting` |
| 38 | it is never silently deleted, and funding it brings it back | `ledger.md` &mdash; `it is never silently deleted` | `guide-suspended-app-is-not-deleted` |
| 39 | capped so that you cannot profitably subscribe to yourself | `ledger.md` &mdash; `The publisher's own node ID is excluded\.` | `guide-publisher-cannot-subscribe-to-itself` |
| 40 | Anything not asked for is denied, and asking is not the same as getting. | `app-manifest.md` &mdash; `Absence means denial\.` | `guide-capability-absence-is-denial` |
| 41 | An app you install gets nothing by default. | `app-manifest.md` &mdash; `Neither path ever grants by default\.` | `guide-no-capability-by-default` |
| 42 | a private folder of its own, a monotonic clock, a fixed list of HTTPS addresses | `app-manifest.md` &mdash; `exposes only a per-app virtual directory` | `guide-app-storage-is-a-private-directory` |
| 43 | A block of entries counts as settled when more than two thirds of their voting power has signed it | `ledger.md` &mdash; `quorum\(signed_power, total_power\) := signed_power \* 3 > total_power \* 2` | `guide-quorum-is-strictly-more-than-two-thirds` |
| 44 | strictly more, not two thirds exactly | `ledger.md` &mdash; `This strict predicate is not '>='` | `guide-quorum-predicate-is-strict` |
| 45 | Every seat carries an expiry stamped on it when it is filled | `ledger.md` &mdash; `\*\*The floor is a term limit, stamped and not derived\.\*\*` | `guide-term-limit-is-stamped` |
| 46 | gives up at least | `ledger.md` &mdash; `Turnover is consequently not a target but an arithmetic certainty` | `guide-turnover-is-arithmetic` |
| 47 | A departing member sits out a cooling-off period and then competes for re-entry like anyone else. | `ledger.md` &mdash; `\*\*for any reason whatsoever\*\*` | `guide-cooldown-applies-for-any-reason` |
| 48 | Nobody can be made a validator without standing for it | `ledger.md` &mdash; `A node cannot be conscripted into the set` | `guide-no-conscription` |
| 49 | the sitting validators cannot name their successors: they can narrow the field of candidates | `ledger.md` &mdash; `\*\*cannot name its members\*\*` | `guide-quorum-cannot-name-its-successors` |
| 50 | not by having been switched on, which is what a rented server in a data centre beats a real phone at | `ledger.md` &mdash; `### Eligibility: demonstrated storage and compute, never availability` | `guide-uptime-counts-for-nothing` |
| 51 | You qualify by having proved storage and computation | `ledger.md` &mdash; `Evidence of kind 'availability' contributes` | `guide-availability-scores-zero` |
| 52 | Above the threshold, more storage buys nothing: no seat, no better odds, no extra weight. | `ledger.md` &mdash; `\*\*Eligibility is a predicate, not a ranking, and this is the whole design\.\*\*` | `guide-eligibility-is-not-a-ranking` |
| 53 | Every validator counts as exactly one. | `ledger.md` &mdash; `and uniform voting power\.\*\*` | `guide-uniform-voting-power` |
| 54 | qualifying is expensive to fake, not impossible to fake | `ledger.md` &mdash; `\*\*expensive to fake and not impossible to fake\*\*` | `guide-eligibility-is-expensive-not-impossible-to-fake` |
| 55 | The score must draw on several independent askers, which raises the price of faking it in proportion, and does not remove it. | `ledger.md` &mdash; `the score must draw` | `guide-score-needs-distinct-issuers` |
| 56 | Writing "cannot be faked without spending real resources" would claim more than the rules deliver. | `ledger.md` &mdash; `would be the overstated safety claim this document refuses to make\.` | `guide-overstated-safety-claim-refused` |
| 57 | Your network address is never written into the shared register | `identity.md` &mdash; `\*\*never published on the ledger\*\*` | `guide-transport-key-not-on-the-ledger` |
| 58 | the key your device uses to make connections is deliberately a different key from the one that identifies you | `identity.md` &mdash; `as its 'transport_public_key' MUST be rejected` | `guide-attestation-may-not-name-the-identity-key` |
| 59 | every node you actually talk to sees the address you are connecting from, and your messages carry your node identifier in the clear | `wire.md` &mdash; `in the signed cleartext` | `guide-envelopes-carry-your-node-id-in-clear` |
| 60 | a replacement identity inherits nothing: no balance, no history, no standing | `ledger.md` &mdash; `receives no balance, nonce, or privileges from the old` | `guide-revocation-transfers-nothing` |
| 61 | a validator that misbehaves can have its identity revoked by the others and loses its seat | `ledger.md` &mdash; `"reason":"key_compromise"\\|"validator_misconduct"\\|"operator_request"` | `guide-validator-misconduct-is-a-revocation-reason` |
| 62 | A block that contains a single entry which does not execute is rejected whole: there are no half-applied blocks. | `ledger.md` &mdash; `execution invalidates the entire proposed block` | `guide-no-partially-applied-blocks` |
| 63 | It holds a recent checkpoint signed by a key it already trusts | `ledger.md` &mdash; `\*\*Validate the external checkpoint\.\*\*` | `guide-light-client-validates-its-checkpoint` |
| 64 | It fails closed rather than open | `ledger.md` &mdash; `unknown-key checkpoints fail closed` | `guide-light-client-fails-closed` |
| 65 | it will not accept a lower height than the highest it has already trusted, nor a different block at a height it has already seen | `ledger.md` &mdash; `\*\*Enforce non-regression\.\*\*` | `guide-light-client-refuses-to-regress` |

**Cosa non è nell'elenco, e perché.** Le frasi che descrivono un'esperienza e
non una garanzia non portano probe: «It stays on and connected», «You subscribe
to apps». Non portano probe nemmeno le regole **di forma** della pagina —
l'unità posposta, il separatore U+202F, il glifo ritirato — perché nessuna di
esse vive in `docs/protocol/`: sono tenute da [ADR-009] e verificate
meccanicamente dalla classe G4 di `check-guide-pairs.mjs`, che è l'ancoraggio
giusto per loro.

### GATE-NOTHING-INVENTED — ciò che è stato tolto o riscritto

L'elenco non è vuoto. Undici affermazioni sono state scritte, cercate nelle
regole, e non trovate nella forma in cui erano state scritte.

| # | Come sarebbe stata scritta | Perché non regge | Che cosa dice la pagina |
| ---: | --- | --- | --- |
| 1 | «Credits cannot be converted into money.» | **Nessuna regola in `docs/protocol/` la tiene.** È un'esclusione permanente di prodotto in `PROJECT.md` e [ADR-005], e `PROJECT.md` è un artefatto del brain, fuori dalla passata sugli artefatti pubblicati: non esiste probe che possa ancorarla. È la formulazione del Lead in *Technical proposal* §1 («non si convertono»), ed è contestata qui. | Dice solo ciò che il ledger dice di sé: non esiste transazione che tolga credits a una persona e li dia a un'altra, e non sono una valuta che la rete possa spostare. |
| 2 | «Your credits are safe.» | Più semplice e più falsa. Il caso che rompe non è ipotetico: i credits versati nell'escrow di un'app sono addebitati **dal quorum**, senza la firma di chi li ha versati. | «Nobody can take credits away from you — the only entries that reduce your balance are ones you signed yourself», e l'apribile porta subito l'eccezione dell'escrow, che è la parte che non conforta. |
| 3 | «The network is super-secure.» | Nessuna regola tiene una sicurezza non qualificata, e `PROJECT.md` impone di dichiararla in modo preciso. | La divisione di §01: robusta contro la falsificazione, non contro il numero. |
| 4 | «Your IP address is private.» | Il ledger non porta l'indirizzo (`identity.md`), ma le buste di gossip portano `sender_node_id` in chiaro (`wire.md`): partecipare espone il legame. La versione semplice è più forte del vero. | «The register does not publish the link. Participating exposes it.» |
| 5 | «Validators are chosen at random.» | L'invariante dice l'opposto a metà: un quorum **non può nominare** i successori ma **decide quali candidature sono finalizzate**, quindi può restringere il campo. | «They can narrow the field of candidates, but the composition comes out of a derivation from randomness they cannot choose.» |
| 6 | «Eligibility cannot be faked without spending real resources.» | È la frase che la specifica stessa ha **ritratto**, e la ritrattazione è pinnata da una probe preesistente. Riprodurla nella guida sarebbe stata la sua riabilitazione. | «Expensive to fake, not impossible to fake», con il prezzo lineare nel numero di issuer colludenti. |
| 7 | «Nobody can steer which question you are asked.» | Vero contro un proposer non colluso; falso contro un proposer colluso con l'issuer, e la specifica ne quantifica il costo (10³–10⁶ timestamp legali, un SHA-256 ciascuno). | Il filo dice l'impegno («lock in its secret before the randomness exists»), l'apribile dice il residuo e lo chiama riduzione, non correzione. |
| 8 | «Apps run in a sandbox, so your data is safe.» | Nessuna regola tiene una sicurezza generale della sandbox. Ciò che le regole tengono è un'enumerazione di capacità e due frasi di rifiuto. | «Anything not asked for is denied, and asking is not the same as getting», con le tre capacità nominate. |
| 9 | «Cheaters are punished.» | In v0 non esiste alcuno slashing e nessuna transazione confisca. «Punished» avrebbe suggerito una perdita che non può accadere. | «There is no punishment that takes credits away. […] Cheating costs you what you would have been paid.» |
| 10 | «A new block every five seconds.» | **Tolta del tutto.** `README.md` dichiara l'intervallo e nella riga successiva dice che nessuna regola di validità lo impone ([DEBT-013]). Scriverlo in una guida sarebbe stato precisamente il difetto contro cui quella sezione è scritta. La pagina **non dice nulla** sui tempi. | — |
| 11 | «If your identity is revoked you lose your credits.» | Nessuna regola dice cosa accade al saldo di un'identità revocata (finding F4). Ciò che le regole dicono è che il rimpiazzo non eredita nulla. | Solo questo: «a replacement identity inherits nothing: no balance, no history, no standing». |

### Verification transcript

```text
$ node .lmbrain/design/coblox-public-guide/tools/check-guide-pairs.mjs
  G1-NO-LITERAL-COLOUR     10 candidate(s) checked
  G2-KNOWN-TOKEN           41 candidate(s) checked
  G3-DECLARED-PAIR         13 candidate(s) checked
  G4-ADR-009                4 candidate(s) checked
  G5-SELF-CONTAINED         4 candidate(s) checked
  G6-CLAIM-STILL-MADE      65 candidate(s) checked

public-guide form check: PASS
exit=0

$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     16 candidate(s) checked
  C4-VALUE          51 candidate(s) checked
  C5-MIRROR         42 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C10-PROBE         84 candidate(s) checked

published-artifact inventory: PASS
exit=0

$ python sim/tools/published_artifacts_negative.py    # le dieci classi storiche, invariate

negative proof: PASS - 10 defect classes, each observed failing

$ node .lmbrain/design/coblox-design-system/tools/check-contrast.mjs   # SPEC-005, invariato
RESULT: all 130 declared pairs meet WCAG AA.
$ node .lmbrain/design/coblox-design-system/tools/build-tokens.mjs --check
OK: tokens.css is in sync with tokens.json (247 custom properties).
$ node .lmbrain/design/coblox-design-system/tools/build-preview.mjs --check
OK: design/preview/index.html matches its generator.
$ node .lmbrain/design/coblox-design-system/tools/build-mockups.mjs --check
OK: all 4 mockup pages match their generator.

$ git status --porcelain docs/protocol .lmbrain/decisions .lmbrain/design/coblox-design-system
(vuoto: le aree in sola lettura non sono state toccate)

$ git diff --stat sim/tools/published_artifacts.toml
 sim/tools/published_artifacts.toml | 533 +++++++++++++++++++++++++++++++++++++
 1 file changed, 533 insertions(+)
$ git diff --numstat sim/tools/published_artifacts.toml   # aggiunte, rimozioni
533	0	sim/tools/published_artifacts.toml
=== GATE-CLAIMS-ANCHORED: le probe della guida provate in negativo ===
Ogni mutazione avviene in una copia sotto la directory temporanea di sistema;
l'albero di lavoro non e mai modificato.

$ control: copia non mutata
published-artifact inventory: PASS

$ mutazione 1: la concessione sui Sybil e ammorbidita in identity.md
$ mutazione 2: "permanent property" diventa "a temporary gap" in identity.md
$ mutazione 3: l invariante del non-trasferimento sparisce da ledger.md

FAIL C10-PROBE: probe 'guide-sybil-not-cryptographic' expected 1 match(es) of 'while not being Sybil-resistant by cryptographic means' in identity.md, found 0. The guide's first uncomfortable sentence says the network is good at stopping forgery and not at stopping crowds. This is the passage that says both halves and requires them to be said together; if it softens, the guide is claiming a resistance the protocol does not have.
FAIL C10-PROBE: probe 'guide-sybil-gap-is-permanent' expected 1 match(es) of 'MUST NOT be described as a temporary gap' in identity.md, found 0. The guide says 'permanent', not 'not yet'. That word is the whole difference between a declared limit and a promise, and this is the rule that forbids the softer reading.
FAIL C10-PROBE: probe 'guide-no-user-to-user-transfer' expected 1 match(es) of 'A direct user-to-user transfer is therefore unrepresentable' in ledger.md, found 0. The strongest property claim on the page and the one the first diagram draws crossed out. It is an absence, so it is held by the invariant that makes the shape unrepresentable rather than by any rule forbidding an act.
published-artifact inventory: FAIL (3 finding(s))
=== check-guide-pairs.mjs: le sei classi provate in negativo ===
Ogni mutazione avviene in una copia; l'albero di lavoro non e mai modificato.

$ control
public-guide form check: PASS

$ G1  un colore letterale entra in guide.css
FAIL G1-NO-LITERAL-COLOUR: guide.css carries a hex colour: "#4FE3A3"

$ G2  la pagina cita un token che tokens.css non emette
FAIL G2-KNOWN-TOKEN: --cbx-color-text-whisper is used by the guide but is not emitted by tokens.css

$ G3a un accostamento non dichiarato dal design system
FAIL G3-DECLARED-PAIR: the guide uses color.text.muted on color.bg.hover but the design system does not declare it as legitimate. A pairing that is not in contrast-pairs.json has never been contrast-checked.

$ G3b la soglia dichiarata e quella piu debole
FAIL G3-DECLARED-PAIR: the guide records color.text.primary on color.bg.app as "non-text" but the design system declares it as "text". The two thresholds are not the same and the weaker reading would be the one that passes.

$ G4a l unita precede il numero (grammatica del denaro)
FAIL G4-ADR-009: 2 unit span(s) in index.html but 1 immediately follow a cbx-num span. An abbreviation that PRECEDES the number is the grammar of money; ADR-009 requires it to follow.

$ G4b il separatore delle migliaia diventa una virgola (il refuso del Lead)
FAIL G4-ADR-009: index.html writes "1,240" in visible prose. The thousands separator is U+202F (narrow no-break space), never a comma or a full stop.

$ G5  la pagina carica un foglio di stile dalla rete
FAIL G5-SELF-CONTAINED: index.html loads "https://cdn.example.com/x.css" from the network. The guide must open from disk.

$ G6  la frase della guida cambia e la probe resta orfana
FAIL G6-CLAIM-STILL-MADE: probe guide-identity-key-is-on-the-ledger claims to anchor "The design is pseudonymous, not anonymous. A stable pseudonym is a weak one.", which the guide no longer says. Either the sentence was edited and its anchor was left behind, or the sentence was dropped and the probe now defends nothing.
=== Resa e geometria, misurate nel browser ===
La pagina servita da un server statico temporaneo su http://127.0.0.1:8731,
spento a fine lavoro. Il riquadro di anteprima di questa sessione non compone i
fotogrammi, quindi nessuno screenshot: le espressioni sotto sono state valutate
nella pagina e il loro risultato e riportato tale e quale.

> stylesheets caricati, token risolti, temi
{ "theme": "dark",
  "sheets": [ {"href":"tokens.css","rules":5}, {"href":"base.css","rules":38},
              {"href":"components.css","rules":110}, {"href":"guide.css","rules":46} ],
  "tokenSample": "#0B100E",
  "bodyBg": "rgb(11, 16, 14)",      // color.bg.app
  "bodyColor": "rgb(227, 237, 232)", // color.text.primary
  "detailsOpen": false,
  "summaryMinHeight": "32px",        // layout.hit-target-min
  "numFont": "\"JetBrains Mono\", \"Fira Code\", ...",
  "numText": "1\u202f240", "unitText": "cr", "unitMarginStart": "4.2px" }

> il separatore delle migliaia, per punto di codice
{ "numCodepoints": ["31", "202f", "32", "34", "30"] }   // 1 U+202F 2 4 0

> tema light, stessa pagina
{ "bodyBg": "rgb(244, 247, 245)", "bodyColor": "rgb(11, 16, 14)",
  "detailBg": "rgb(255, 255, 255)", "plusColor": "rgb(10, 92, 62)",
  "boxStroke": "rgb(11, 124, 83)" }

> struttura e alternative testuali
{ "headings": ["H1 How Coblox works",
               "H2 01 What is Coblox?",
               "H2 02 What does your device actually do?",
               "H2 03 What do you get for it?",
               "H2 04 What can you do with them?",
               "H2 05 Who keeps the books?",
               "H2 06 What does the network know about you?",
               "H2 07 What happens if somebody cheats?"],
  "svgRoles": [ {"role":"img","labelled":"fig1-title fig1-desc"},
                {"role":"img","labelled":"fig2-title fig2-desc"} ],
  "detailsCount": 8 }

> il colore non e mai l'unico canale: ogni tratto porta anche uno stile di linea
{ "dg-edge dg-edge--mint": "none",       // pieno,        etichetta "mint"
  "dg-edge dg-edge--burn": "5px, 3px",   // tratteggiato, etichetta "burn"
  "dg-edge dg-edge--forbidden": "2px, 4px", // punteggiato + croce + didascalia
  "dg-box": "4px, 4px" }                 // il riquadro assente e tratteggiato

> geometria dei diagrammi, dopo la correzione
{ "escapes": "none",            // nessun testo esce dal proprio riquadro
  "viewBoxOverflow": "none",    // nessun testo esce dal viewBox
  "subFont": "12px" }

> nessuno scorrimento orizzontale di pagina, a tre larghezze
{ "viewport": 192,  "pageOverflow": false, "scrollersScrollable": [true, true] }
{ "viewport": 375,  "pageOverflow": false, "scrollersScrollable": [true, true] }
{ "viewport": 1280, "pageOverflow": false, "scrollersScrollable": [false, false],
  "svgRendered": [630, 630], "effectiveSubPx": 10.5 }

> le tre cose scomode, con ogni .detail__body rimosso dal DOM
{ "sybil": true, "subscriptions": true, "pseudonymous": true }

```

### Deviations from the specification

**Nessuna deviazione di scope.** Tre scelte da dichiarare.

1. **Uno strumento in più nel pacchetto.** La spec elencava fra i file
   `published_artifacts.toml` e il pacchetto nuovo, e non prevedeva
   esplicitamente uno strumento di verifica dentro il pacchetto. Ne è stato
   scritto uno, `tools/check-guide-pairs.mjs`, perché senza di esso due criteri
   di accettazione sarebbero stati verificati solo a occhio (gli accostamenti di
   colore e le regole di [ADR-009]) e la direzione pagina → regola non sarebbe
   stata verificata affatto. Non modifica nulla in sola lettura.
2. **Il `claims` nel manifesto delle probe.** Campo nuovo sulle sole probe
   `guide-*`. `published_artifacts.py` non lo legge e non ne è disturbato
   (verificato: la passata resta verde con 84 probe). Sta lì e non accanto alla
   pagina perché la frase e la regola su cui poggia devono essere **un solo
   record**: separarli è il modo in cui divergono.
3. **`GATE-OPERATOR-LOOK` non è coperta da questa evidenza**, come da spec: è una
   gate `before-done` dell'operatore. La verifica visiva fatta qui misura stile e
   geometria, non giudica il tono.

### Findings — cose trovate scrivendo, non aggirate

**F1 — Il testo pubblico di [ADR-014] non esiste.** `SEC-REQ-22` è un obbligo con
scadenza al primo partecipante esterno, ed è in `BACKLOG.md:54` con
`GATE-SECREVIEW`. La guida vi **rimanda** invece di duplicarlo, come chiede il
criterio, ma un rimando a un documento inesistente sarebbe un collegamento rotto
travestito da riferimento. La pagina dice perciò, in §04, che la dichiarazione è
dovuta e non ancora pubblicata, e che fino ad allora ciò che il lettore ha
davanti e `SECURITY.md` sono tutto ciò che il progetto ha detto. Un buco
dichiarato invece che nascosto dietro una scrittura elegante.

**F2 — `SECURITY.md` è stale e contraddice sia il protocollo sia questa guida.**
Dice: *«The validator set is self-perpetuating in the current v0 protocol […] The
election rule is unwritten and is the first work of M-02. Tracked as
`DEBT-005`.»* [DEBT-005] è **risolto**, e `ledger.md` specifica elezione,
rotazione, limite di mandato e pavimento di contrazione. §05 della guida
descrive quelle regole. Sono due copie pubbliche dello stesso fatto che hanno
divergiuto, cioè la famiglia 2 di `recurring-defects.md`, su una superficie che
un ricercatore di sicurezza legge per prima. `SECURITY.md` non è nel mio scope.

**F3 — La non-convertibilità del credit non è tenuta da nessuna regola
pubblicata.** Vive in `PROJECT.md` e in [ADR-005], che stanno fuori dalla passata
di [ADR-012]. Una pagina pubblica che dica «i credits non sono denaro» non ha
oggi alcuna probe disponibile: è l'unica affermazione centrale del prodotto per
cui il meccanismo di manutenzione non copre. Il ledger dice di sé *«It is not a
transferable currency»*, che è meno, ed è ciò che la pagina usa. Merita
probabilmente una riga normativa in `docs/protocol/README.md`, che è una
decisione del Lead e non mia.

**F4 — Il burn di abbonamento non richiede un firmatario `unrevoked`, e ogni
altra transazione autorizzata dall'utente sì.** `ledger.md:347` dice *«For a
subscription, the key MUST derive `payer_node_id`»* e si ferma lì. `fund_app`
(riga 312), `challenge_commitment` (398) e `validator_candidacy` (871) dicono
tutte *«the enrolled, unrevoked»*. Se l'omissione è deliberata non è detto
perché; se non lo è, una chiave revocata per compromissione può ancora
autorizzare addebiti sul saldo, che è il contrario del punto della revoca. È
emerso provando a scrivere con precisione «cosa succede quando una chiave viene
revocata», e la guida di conseguenza **non dice** che la revoca ferma la spesa.

**F5 — Le guardie di design non girano in CI.** `check-contrast.mjs`, i tre
generatori in `--check` e ora `check-guide-pairs.mjs` sono eseguiti solo a mano.
La barra 130/130 di [SPEC-005] è quindi una prassi, non un cancello — la stessa
forma di promessa che [ADR-012] esiste per non accettare più. La direzione che
conta di più per questa spec **è** in CI, perché `published_artifacts.py` lo è:
l'affermazione del colophon («an automated check that runs on every change»)
regge. Non regge per il verso inverso e per le verifiche di forma. Un job
`design-guards` di cinque righe accanto a *Protocol document guards* lo
chiuderebbe; la modifica alla CI non è delegata a questa spec.

### Contestazioni alle formulazioni del Lead

1. **«credits, che sono una misura e non un denaro e non si convertono»**
   (*Technical proposal* §1). L'ultima clausola non è ancorabile: vedi F1/F3.
   Riscritta, non tolta.
2. **L'ordine delle sette domande è stato tenuto**, ma con una precisazione che
   cambia il risultato: se le tre cose scomode fossero dette *nella* sezione 6,
   la pagina passerebbe la gate alla lettera e la tradirebbe nello spirito,
   perché arriverebbero dopo quattro sezioni in cui il lettore si è già
   convinto. Ciascuna è perciò detta **al primo momento in cui è rilevante** —
   §01, §03, §04 — e §06 le raccoglie dichiarando di raccoglierle. L'ordine
   proposto è quello della curiosità; il collocamento delle tre frasi non poteva
   seguirlo.
3. **Il Lead ha scritto `1,240` violando [ADR-009] e lo ha segnalato.** La classe
   G4 di `check-guide-pairs.mjs` ora fallisce esattamente su quel refuso, ed è
   stata provata in negativo proprio con quella stringa. Un errore che si fa una
   volta e che da adesso non passa.

### Handoff status
- [x] Ready for Project Lead review