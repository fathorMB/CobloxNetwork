---
id: ADR-009
# Note: Quote the title if it contains a colon
title: "L'unità del token si chiama credit e si scrive come una misura, non come una valuta"
status: accepted
decision_date: 2026-08-25
decider: AGENT-LEAD
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: [ADR-005, ADR-006, ADR-007]
tags: [design, architecture]
created: 2026-08-25
updated: 2026-08-25
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned proposed -> accepted"
---
# L'unità del token si chiama credit e si scrive come una misura, non come una valuta

> Decisa dall'operatore il 2026-08-25, in sessione, dopo istruttoria del Lead.

## Context

[SPEC-003] ha consegnato il design system con un segnaposto dichiarato: il glifo `◇`, classe `.cbx-unit--provisional` con sottolineatura tratteggiata, e forma scritta "Coblox token". AGENT-006 aveva isolato il punto di modifica — una classe CSS più `$meta` in `tokens.json` — proprio per non far costare nulla la decisione quando fosse arrivata. Restava aperto anche il font monospace, con tre candidati a licenza libera.

Il vincolo che governa la scelta è il più duro del progetto: il token non deve poter acquisire valore monetario **neanche di fatto** ([[PROJECT]]), esclusione permanente e di principio, con il divieto di trasferimento diretto utente→utente reso strutturalmente inesprimibile nel formato del ledger anziché soltanto vietato a parole.

Il Lead ha osservato che quel vincolo ha una conseguenza sul naming che il riflesso naturale contraddice. Un nome non è un'etichetta neutra: è ciò attorno a cui si costruisce un'aspettativa, ed è il nome che comparirebbe in un annuncio "vendo 5.000 ___". Un nome proprio, corto e coniabile è la materia prima di un mercato grigio, perché la speculazione ha bisogno di un brand su cui aggrapparsi; un termine descrittivo e generico è molto più difficile da feticizzare. **Il vincolo non spinge quindi solo verso un nome non-monetario: spinge verso un nome poco brandizzabile** — l'opposto di ciò che si cerca istintivamente per un progetto pubblico.

## Decision

**L'unità si chiama `credit`, al plurale `credits`.** Forma estesa nelle interfacce: `1,240 credits`.

**La forma compatta è `cr`, posposta al numero:** `1,240 cr`. Il glifo `◇` e la classe `.cbx-unit--provisional` sono ritirati.

La posizione non è un dettaglio tipografico ma il portatore del significato. Un glifo che precede il numero — `$50`, `€50`, `◇50` — è la grammatica del denaro. Un'abbreviazione che segue il numero — `50 kg`, `50 ms`, `1240 cr` — è la grammatica della misura. Adottando la seconda, la tipografia lavora **a favore** del vincolo di non convertibilità a ogni schermata, invece di contraddirlo mille volte al giorno mentre il documento di progetto lo afferma una volta sola.

**Font monospace per dati, numeri e identificativi: JetBrains Mono**, con lo stack di fallback già dichiarato in `tokens.json`. La motivazione è di leggibilità e non di gusto: zero barrato, distinzione netta fra `1`, `l` e `I` e fra `0` e `O`, altezza-x generosa. Su un'interfaccia densa di hash, chiavi pubbliche e identificativi di nodo, quella distinzione è la differenza fra leggere un identificativo e sbagliarlo.

**Correzione a un dato di [SPEC-003].** La proposta di AGENT-006 motivava JetBrains Mono anche con la licenza Apache-2.0. È inesatto, ed è stato verificato alla fonte: Apache-2.0 copre il *codice sorgente* del repository JetBrains/JetBrainsMono, mentre il **carattere** è sotto SIL Open Font License 1.1. Anche IBM Plex Mono e Fira Code sono OFL 1.1, quindi la licenza non distingueva i tre candidati e l'argomento va rimosso dalla motivazione. Non cambia la fattibilità: OFL 1.1 permette la ridistribuzione dentro un'applicazione, con l'obbligo che la licenza viaggi col font e che il Reserved Font Name non sia usato per versioni modificate.

## Alternatives considered

- **Un nome coniato legato a Coblox.** Dà identità e memorabilità, ed è la direzione istintiva. Scartata perché è precisamente ciò che rende un'unità trattabile come asset: crea l'oggetto con cui si costruisce un mercato. Due collisioni concrete rendevano lo spazio ancora più stretto: qualunque derivato di "blox" confligge con l'ecosistema Roblox, dove la valuta si chiama per giunta Robux; e qualunque suffisso in `-coin`, `-cash` o `-bit` promette esattamente ciò che il progetto rifiuta.
- **Un'unità di lavoro presa dalla fisica**, per esempio `erg`, unità CGS di lavoro. Aveva un merito reale: nessuna storia crypto, registro scientifico anziché finanziario, e dice letteralmente ciò che il token misura. Scartata perché resta un nome proprio distintivo, quindi brandizzabile, e paga quel rischio in cambio di un'eleganza che "credits" ottiene con meno superficie di attacco.
- **Restare senza nome proprio**, mantenendo la forma "Coblox token". Era la scelta conservativa e non bloccava nulla. Scartata perché non risolveva il problema pratico, cioè che le interfacce dense hanno bisogno di una forma breve, e il segnaposto `◇` che le serviva era proprio la resa peggiore rispetto al vincolo.
- **Mantenere un glifo come forma compatta.** La più distintiva e la più corta, ma imita la forma con cui si scrivono le valute. Scartata per la stessa ragione per cui `cr` è stata scelta.
- **Nessuna forma compatta**, scrivendo sempre `credits` per esteso. Massima chiarezza, ma costa spazio orizzontale esattamente dove il design system punta sulla densità da terminale.
- **IBM Plex Mono** come alternativa al font scelto: più sobrio, con una sans abbinata se un giorno servisse una sola famiglia. **Fira Code**: stessa base di Fira Mono più le legature, che però vanno disattivate perché falsano la lettura dei dati — preso per spegnerne la caratteristica distintiva, resta poco motivo per preferirlo.

## Consequences

- Il pacchetto di design in `.lmbrain/design/coblox-design-system/` va aggiornato: ritiro di `.cbx-unit--provisional` e del glifo `◇` da `css/base.css`, dai tre mockup e dalla pagina di anteprima, dove compaiono anche i paragrafi di avvertenza «il nome dell'unità non è ancora deciso», e aggiornamento di `$meta` in `tokens.json`. È lavoro di AGENT-006, non del Lead.
- `PRINCIPLES.md` deve dire come si formattano i numeri di credit, e in particolare che l'unità è **posposta** e mai anteposta, con la ragione: senza la ragione scritta, la regola verrà infranta dalla prima persona che troverà più bello il glifo davanti.
- La lingua di prodotto è l'inglese ([[PROJECT]]), quindi `credit`/`credits` è la forma canonica anche nella documentazione pubblica. Le avvertenze in italiano rimaste nei mockup sono debito di quella stessa passata.
- Incorporare i file del font nel bundle Tauri resta lavoro di un'altra spec, come [SPEC-003] già annotava: serve per un rendering identico su Windows e Linux. Con OFL 1.1 andrà inclusa la licenza del font accanto all'`Apache-2.0` del progetto.
- Il progetto acquista una risposta breve alla domanda «quanto vale un credit?», che su un repository pubblico verrà posta: non vale, misura. La resa tipografica è la prima cosa che lo dice, prima che qualcuno legga un documento.

## Review conditions

Rivedere se: emerge nonostante tutto un mercato grigio, il che indicherebbe che la difesa del naming è irrilevante rispetto alle leve economiche e che l'attenzione va spostata interamente su `α` e su [DEBT-007]; oppure se test con utenti reali mostrano che `cr` posposto non viene compreso come unità, nel qual caso la forma estesa `credits` va preferita anche a costo di densità. Non rivedere per ragioni di memorabilità o di brand: la scarsa brandizzabilità è l'obiettivo della decisione, non un suo effetto collaterale.
