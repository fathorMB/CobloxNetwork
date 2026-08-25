---
id: SPEC-019
# Note: Quote the title if it contains a colon
title: "Cosa significa non revocata per autorizzare una spesa, e la riga che non lo chiede"
status: backlog
kind: bugfix
priority: high
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: [SPEC-018]
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012, ADR-006]
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [security, identity, conformance]
---

# Cosa significa non revocata per autorizzare una spesa, e la riga che non lo chiede

## Objective

Chiudere [DEBT-022]. La riga 407 di `docs/protocol/ledger.md` dice *«For a subscription, the key MUST derive `payer_node_id`»*, mentre le tre righe sorelle — 372, 458, 968 — dicono *«the **enrolled, unrevoked** key»*.

Ma la chiusura **non è l'allineamento**, e questa spec esiste per impedire che lo sia. La valutazione di AGENT-007 ha stabilito che **`unrevoked` non è definito da nessuna parte** per l'autorizzazione delle transazioni, e che il documento ne usa **due letture diverse**. Allineare la riga alle sorelle chiude l'asimmetria e **lascia il buco aperto**. La definizione viene prima.

## Context

**L'omissione è una svista e non una deroga, ed è accertato.** Il Lead aveva chiesto se esistesse una ragione per cui un'autorizzazione ricorrente debba sopravvivere alla revoca — per esempio perché l'addebito discende da un consenso passato. La risposta è no: **non esiste in questo protocollo alcun oggetto «abbonamento» con durata** da cui possa discendere alcunché. Ogni addebito è una transazione nuova, con firma fresca e nonce nuovo. Non c'è nulla da onorare.

**L'esito peggiore non è quello che il debito immaginava.** Non è l'addebito indebito né il drenaggio: è un **fork**. Chi legge la riga 407 alla lettera accetta il burn di una chiave revocata; chi generalizza dalle tre sorelle lo rifiuta; e il disaccordo è **sulla validità di un blocco**. Non richiede alcuna chiave rubata. È la famiglia 4 del censimento — clausola normativa che nessun oracolo esercita — nella sua forma peggiore, perché il costo non è un difetto ma una **divergenza di catena**.

Ne discendono due correzioni già registrate nel debito. La motivazione di severità è stata sostituita: fondarla sulla chiave rubata la faceva poggiare su un fallimento a monte, e una gravità che poggia su un fallimento altrui si lascia sempre declassare. E la condizione di chiusura è passata da *«prima che una devnet accumuli saldi reali»* a **«prima che esista una seconda implementazione»**: la grandezza da cui il pericolo dipende non è il valore in gioco, è il numero di lettori del documento.

**Il buco vero: `unrevoked` non ha una definizione.** Il documento usa due letture, e il divario fra loro non è teorico:

- `identity.md` la lega a una revoca **finalizzata**;
- `ledger.md` la lega a *«enrolled and not revoked **as of** una certa altezza»*, cioè a una revoca **efficace**;
- e `min_revocation_effective_delay_blocks` è **dichiaratamente scelto lungo** (`ledger.md` §*revoca*).

Fra «finalizzata» ed «efficace» c'è quindi un intervallo **deliberatamente ampio**, durante il quale le due letture divergono su ogni autorizzazione. Allineare la riga 407 alle sorelle sposta la riga dentro l'ambiguità invece di toglierla: **è un rimedio che sembra completo e non lo è.**

**Il drenaggio non dipende dalla finestra.** Il Lead aveva chiesto di stabilire *su quale finestra temporale* un attaccante operi. La domanda era mal posta: prezzo e periodo del burn sono scelti da chi attacca, e **una sola transazione può azzerare il saldo**. La finestra non è la grandezza da cui la perdita dipende — è la terza domanda della famiglia 3, posta e risposta.

## Scope

### Included

- La **definizione normativa** di ciò che `unrevoked` significa per autorizzare una transazione: quale delle due letture vale, e la stessa in tutti i punti.
- L'allineamento della riga 407, **dopo** la definizione.
- La **passata su tutte le regole di autorizzazione del protocollo**, non sulle quattro nominate: il difetto è nell'asimmetria, e un'asimmetria si censisce enumerando.
- La fixture di conformità che esercita il caso in cui le due letture divergono.
- La gate di [ADR-012], perché questa spec modifica una regola di validità.

### Excluded

- Qualunque modifica a `min_revocation_effective_delay_blocks`. È lungo per una ragione dichiarata, e accorciarlo per ridurre l'intervallo di ambiguità sarebbe **curare il sintomo cambiando un parametro invece di scrivere una regola** — [ADR-010], e la famiglia 3.
- La revoca stessa, la sua propagazione e la sua meccanica.
- [DEBT-017], che ha la propria spec.

## Existing-project analysis

Le quattro righe di `docs/protocol/ledger.md`: 372 (`payer_node_id`, con qualificazione), 407 (**abbonamento, senza**), 458 (`issuer_node_id`, con), 968 (`node_id`, con). Le righe si muoveranno con l'atterraggio di [SPEC-016]: sono citate anche per il testo della clausola.

Il termine `enrolled and not revoked as of ...` compare inoltre a 728 (set di validatori) e 1027 (eleggibilità), entrambi **ancorati a un'altezza**, che è la seconda lettura.

## Technical proposal

**Primo la definizione, poi l'allineamento, poi la passata.** L'ordine è la sostanza di questa spec. Una definizione scritta dopo l'allineamento verrebbe scritta per giustificarlo.

La definizione deve scegliere fra le due letture **e motivare la scelta sulla proprietà che si vuole**, non sulla comodità di implementazione. La domanda da cui dipende: un verificatore che rigioca la catena deve poter stabilire la validità di quella transazione **senza giudizio e senza stato esterno**, e delle due letture solo una ha questa proprietà a ogni altezza.

**La fixture deve esercitare la divergenza, non la regola.** Una fixture che mostra che una chiave revocata-e-efficace viene rifiutata non prova nulla: entrambe le letture la rifiutano. Il caso che conta è quello **fra le due**: revoca finalizzata ma non ancora efficace. È lì che due implementazioni conformi divergono, ed è l'unico caso la cui fixture chiude il fork.

## Files and areas involved

- `docs/protocol/ledger.md` — la definizione, la riga 407, la passata sulle regole di autorizzazione.
- `docs/protocol/identity.md` — solo se la definizione scelta contraddice ciò che lì è scritto; in tal caso è l'altro punto a muoversi.
- `docs/protocol/README.md` — la fixture pubblicata e gli hash che ne discendono.
- `core/coblox-core/` — la regola e la sua prova in negativo.
- `sim/tools/` — la gate di [ADR-012].

## Acceptance criteria

- [ ] `unrevoked` ha **una** definizione normativa per l'autorizzazione delle transazioni, e la scelta fra le due letture è motivata sulla proprietà del verificatore che rigioca, non sulla comodità.
- [ ] La riga 407 è allineata, **dopo** che la definizione esiste.
- [ ] Esiste l'**elenco di tutte le regole di autorizzazione** del protocollo, con la qualificazione presente o assente segnata per ciascuna, anche se l'elenco non contiene altre omissioni.
- [ ] Una fixture pubblicata esercita il caso **fra le due letture** — revoca finalizzata e non ancora efficace — e le due letture ora concordano.
- [ ] Nessun parametro è stato mosso.
- [ ] Ogni valore pubblicato che cambia è ricalcolato con il metodo validato prima su un valore non modificato.
- [ ] La gate di [ADR-012] è eseguita e la trascrizione allegata.

## Implementation plan

1. Enumerare tutte le regole di autorizzazione e le due letture, prima di scrivere.
2. Scrivere la definizione, con la motivazione.
3. Allineare la riga 407 e ogni altra omissione che l'enumerazione abbia trovato.
4. La fixture del caso divergente, e la regola in `coblox-core` con la prova in negativo.
5. Ricalcolo degli hash pubblicati, e gate di [ADR-012].

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-DEFINITION-FIRST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La definizione di `unrevoked` è scritta **prima** dell'allineamento della riga 407, e la trascrizione mostra l'ordine. Una definizione scritta dopo verrebbe scritta per giustificare l'allineamento, e l'allineamento da solo sposta la riga **dentro** l'ambiguità invece di toglierla.
- [ ] GATE-DIVERGENT-CASE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La fixture esercita il caso **fra le due letture** — revoca finalizzata e non ancora efficace — e non il caso che entrambe già rifiutano. Una fixture sul caso concorde è verde oggi e sarebbe stata verde anche col difetto aperto: sarebbe un calcolo, non una guardia.
- [ ] GATE-ALL-AUTHORIZATION-RULES | kind=manual | owner=agent | phase=before-submit | evidence=transcript | L'elenco di **tutte** le regole di autorizzazione del protocollo è prodotto e allegato, con la qualificazione segnata presente o assente per ciascuna, **anche se non trova altre omissioni**. Il difetto è nell'asimmetria: correggere la sola riga nominata non dimostra che sia l'unica.
- [ ] GATE-NO-PARAMETER-MOVED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una ricerca sul diff mostra che **nessun parametro è stato mosso**, e in particolare non `min_revocation_effective_delay_blocks`. Accorciarlo ridurrebbe l'intervallo di ambiguità senza scrivere alcuna regola: è il rimedio che sembra economico ed è la famiglia 3 commessa dentro il rimedio.
- [ ] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato e la trascrizione allegata, **anche se non trova nulla**.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la chiusura e il Lead ha accettato la review. Il debito nasce dalla sua valutazione, ed è lei ad aver stabilito che l'allineamento da solo non chiude: è la persona che può dire se la definizione scelta chiuda davvero.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è la chiusura che sembra completa.** Allineare quattro righe è visibile, verificabile e sbagliato da solo. Se al termine di questa spec la riga 407 è allineata e `unrevoked` non ha una definizione, il fork è ancora lì e il documento **sembra** a posto — che è peggio di prima, perché nessuno tornerà a guardare.
- **Il rischio secondario è la scelta della lettura fatta per comodità.** La lettura «finalizzata» è più semplice da implementare e la «efficace» è più semplice da rigiocare. La scelta va motivata sulla seconda proprietà, e se le due confliggessero è una decisione del Lead.
- **Le citazioni di riga si muoveranno.** [SPEC-016] sta modificando `ledger.md`. Le righe qui citate sono da riverificare sul testo della clausola e non sul numero.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Fermarsi e riportare è un esito previsto**, e qui vale in particolare se le due letture non fossero riconciliabili senza cambiare la meccanica della revoca: quella è un'altra spec e una decisione del Lead.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

### Files changed
