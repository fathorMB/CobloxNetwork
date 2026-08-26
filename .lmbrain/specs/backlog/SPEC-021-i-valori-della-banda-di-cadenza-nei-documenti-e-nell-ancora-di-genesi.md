---
id: SPEC-021
# Note: Quote the title if it contains a colon
title: "I valori della banda di cadenza nei documenti e nell'ancora di genesi"
status: backlog
kind: feature
priority: high
area: consensus
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
capability_tier: sol
thinking_level: standard
effort_observations: []
depends_on: [SPEC-016]
dependency_events: []
parking_events: []
skills: [SKILL-001, SKILL-002, SKILL-003]
verification_gates: []
related_decisions: [ADR-012, ADR-013, ADR-016]
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [governance, light-client]
---

# I valori della banda di cadenza nei documenti e nell'ancora di genesi

## Objective

Scrivere nei documenti di protocollo e nell'ancora di fiducia di genesi i valori della `CadenceBand` che [ADR-016] ha deciso, e togliere la voce corrispondente dalla lista DRAFT dei parametri di lancio.

**È la spec più piccola della coda e quella con la superficie di errore più alta**, perché ciò che scrive non è un meccanismo ma **cinque numeri e ciò che quei numeri costano**. I numeri sono già decisi: il lavoro è la seconda metà.

## Context

[SPEC-016] ha chiuso [DEBT-013] rendendo la cadenza reale **misurabile e dichiarata**, non impedita. La tolleranza di quella misura era l'unica cosa rimasta aperta ed è una decisione dell'operatore, presa il 2026-08-26 e registrata in [ADR-016]:

| Campo | Valore |
| --- | --- |
| `block_interval_ms` | `5000` |
| `min_ms_per_block` | `2500` |
| `max_ms_per_block` | `20000` |
| `min_measured_blocks` | `720` |
| `max_external_clock_slack_ms` | `600000` |

I vincoli sono già regole scritte in `docs/protocol/README.md` e vanno verificati, non riscritti: `min ≤ block_interval_ms ≤ max`, tutti positivi, e `max_external_clock_slack_ms < min_measured_blocks × block_interval_ms`.

**La banda vive nella distribuzione firmata e in nessun altro canale.** Non è modificabile da alcun documento on-chain, non va appresa da un peer, da un'intestazione o da un documento di protocollo. La ragione è scritta ed è la parte che non va persa: *una banda che un quorum seduto potesse allargare sarebbe una tolleranza sotto l'unica misura che il protocollo ha del comportamento di quel quorum.*

**Le conseguenze dei due lati non si scambiano fra loro**, e [ADR-016] le dichiara:

- il **lato lento**, `4 × interval`, significa che un set attivo può **stirare le proprie epoche fino a quattro volte** prima che qualcosa lo dica. Le garanzie anti-cattura di [SPEC-006] restano vere in **epoche**; la loro traduzione in giorni dipende da chi le epoche le produce;
- il **lato veloce**, `interval / 2`, obietta a un **raddoppio dell'emissione reale**, ed è il lato su cui il client **fallisce chiuso**.

## Scope

### Included

- I cinque valori in `docs/protocol/README.md`, al posto della voce DRAFT.
- I cinque valori nell'ancora di fiducia di genesi.
- **Le conseguenze dei due lati scritte accanto ai valori**, nella forma dichiarata da [ADR-016] e non riassunta.
- La rimozione della **sola** voce della banda dalla lista DRAFT, con le altre tre intatte.
- Una fixture di frontiera sulla regola relazionale `slack < min_measured_blocks × block_interval_ms`.
- La passata di [ADR-012].

### Excluded

- **Qualunque modifica alla misura, alla sua asimmetria o alla procedura di rilascio.** [SPEC-016] le ha chiuse e [REVIEW-027] le ha riviste dopo un finding `high`: qui si scrivono valori, non si tocca il meccanismo.
- **Qualunque modifica ai valori decisi.** Se un vincolo non fosse soddisfatto, è una decisione del Lead e dell'operatore, non una correzione da fare passando.
- Le altre tre voci della lista DRAFT.

## Existing-project analysis

`docs/protocol/README.md` porta lo schema di `CadenceBand`, le sue regole di validità, e la voce DRAFT che istruisce la scelta con il costo di ciascun ordine di grandezza **sui due lati**. Quella prosa istruttiva è ciò che ha permesso all'operatore di decidere, e **non va cancellata insieme alla voce**: va trasformata da istruzione a scelta in giustificazione della scelta fatta.

`core/coblox-core/src/params.rs` porta `CadenceBand` e la sua validazione, chiamata come primo atto da entrambe le misure di `cadence.rs`.

## Technical proposal

**Scrivere i valori è metà del lavoro. L'altra metà è ciò che i valori costano, e va scritta accanto a loro.**

Un lettore che trova una banda conclude che la cadenza sia **limitata**. Non lo è: la banda la rende **misurabile**, e [SPEC-016] ha già dovuto scrivere in `README.md` che *«No rule of this protocol prevents that, and none can»*. Quella frase è una probe della gate e va lasciata dov'è; ciò che questa spec aggiunge non deve contraddirla né attenuarla.

La conseguenza del lato lento — **quattro volte** — va scritta come numero e non come qualità. «Il set può allungare le proprie epoche» è vero e non dice quanto; `4 ×` sì, e traduce le nove epoche del mandato massimo in una grandezza che un lettore può confrontare con la propria attesa.

## Files and areas involved

- `docs/protocol/README.md` — i valori, le conseguenze, la voce DRAFT, la fixture.
- `core/coblox-core/` — l'ancora di genesi e la fixture di frontiera.
- `sim/tools/` — la gate di [ADR-012] e le probe nuove.

## Acceptance criteria

- [ ] I cinque valori di [ADR-016] sono in `README.md` e nell'ancora di genesi, e coincidono con la ADR.
- [ ] I tre vincoli relazionali sono **verificati sui valori scritti**, non riscritti.
- [ ] **Le conseguenze dei due lati sono scritte accanto ai valori**, con il lato lento espresso come `4 ×` e il lato veloce come raddoppio dell'emissione.
- [ ] La frase *«No rule of this protocol prevents that, and none can»* è **ancora là e non attenuata**.
- [ ] La lista DRAFT ha perso **una sola** voce e ne conserva tre.
- [ ] Una fixture esercita la frontiera di `slack < min_measured_blocks × block_interval_ms` **da entrambi i lati**.
- [ ] Nessun valore pubblicato preesistente è cambiato, oppure ogni cambio è ricalcolato con il metodo validato prima su un valore non modificato.
- [ ] La gate di [ADR-012] è eseguita e la trascrizione allegata.

## Implementation plan

1. Verificare i tre vincoli sui valori decisi, **prima** di scrivere. Se uno non regge, fermarsi.
2. Scrivere valori e conseguenze insieme, mai i valori da soli.
3. L'ancora di genesi e la fixture di frontiera.
4. Togliere la voce DRAFT, conservando la prosa istruttiva come giustificazione.
5. Gate di [ADR-012] e probe nuove, ciascuna provata in negativo.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-COST-BESIDE-VALUE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Ogni valore scritto ha **accanto** ciò che costa, e il lato lento è espresso come `4 ×`. Una banda scritta senza il suo costo fa concludere a un lettore che la cadenza sia limitata, mentre è solo misurabile: è la famiglia 2, la pretesa avanti rispetto alla regola, su un documento che parla di ciò che il protocollo **non** impedisce.
- [ ] GATE-RENUNCIATION-INTACT | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La frase *«No rule of this protocol prevents that, and none can»* è ancora nel documento, **non attenuata e non spostata sotto i valori nuovi**. È già una probe di [ADR-012]: la gate qui verifica che il testo attorno non la contraddica, cosa che una probe non può vedere.
- [ ] GATE-DRAFT-MINUS-ONE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La lista DRAFT dei parametri di lancio ha perso **esattamente una** voce e ne conserva tre, contate prima e dopo. Una lista che si accorcia più del dovuto perde una decisione aperta senza che nessuno se ne accorga.
- [ ] GATE-RELATION-BOUNDARY | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La frontiera di `slack < min_measured_blocks × block_interval_ms` è esercitata **da entrambi i lati**: il valore massimo ammesso passa, il primo valore oltre è rifiutato. Una regola relazionale provata da un lato solo non distingue `<` da `≤`.
- [ ] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato e la trascrizione allegata, **anche se non trova nulla**. Ogni probe nuova entra nel conteggio individuale della prova in negativo.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è scrivere i numeri e non il loro costo.** È la metà facile e visibile; l'altra è quella per cui la spec esiste. Se al termine i cinque valori sono nel documento e un lettore ne conclude che la cadenza è limitata, questa spec ha fatto danno invece che lavoro.
- **Il rischio secondario è cancellare la prosa istruttiva insieme alla voce DRAFT.** Quella prosa è ciò che ha permesso la decisione, e diventa la sua giustificazione: si trasforma, non si toglie.
- **Un rischio terzo, e va nominato perché sembra un miglioramento:** rendere la banda leggibile da un documento di consenso «per comodità di configurazione». Sarebbe la cosa esatta che il divieto esiste per impedire, ed è la forma che un implementatore propone in buona fede guardando lo schema.
- `max_external_clock_slack_ms = 600000` è **una scelta sulla latenza attesa e non una misura**: [ADR-016] lo dichiara e va riverificato appena il processo di rilascio esiste. Scriverlo senza quella qualificazione lo trasformerebbe in un dato.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Fermarsi e riportare è un esito previsto**, e qui vale in particolare se uno dei tre vincoli relazionali non fosse soddisfatto dai valori decisi: è una decisione dell'operatore e non una correzione da fare passando.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

### Files changed
