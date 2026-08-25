---
id: SPEC-008
# Note: Quote the title if it contains a colon
title: "Core del ledger in Rust: serializzazione canonica, hash, Merkle, set di validatori"
status: ready
kind: feature
priority: high
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
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
related_decisions: [ADR-001, ADR-003, ADR-007]
links: [SPEC-001, SPEC-002, SPEC-006]
created: 2026-08-25
updated: 2026-08-25
tags: [rust, ledger, conformance, merkle]
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
# Core del ledger in Rust: serializzazione canonica, hash, Merkle, set di validatori

## Objective

Implementare in `coblox-core` lo strato deterministico del protocollo: serializzazione canonica, registro delle preimmagini di hash, alberi di Merkle, `ValidatorSet` con `ElectionRecord` e derivazione dell'elezione, verifica dell'header di blocco e predicato di quorum, blocco di vincoli dei parametri di consenso e `ElectionBounds`. Il criterio di accettazione è insolitamente netto: **riprodurre ogni valore del registro di conformità pubblicato**.

## Context

Questa è **la prima implementazione reale del progetto**. Oggi `coblox-core` contiene sessanta righe: una funzione che restituisce la versione. Tutto ciò che esiste è specifica — e la specifica è densa: `ledger.md` e `identity.md` sono passati per tre giri di remediation di sicurezza su [SPEC-001] e quattro su [SPEC-006], per un totale di venticinque finding chiusi.

Ne discende una responsabilità che va detta esplicitamente: **le convenzioni che stabilisci qui le eredita tutto il resto**. Il nodo headless, la shell Tauri, il binding Android via UniFFI e il light client consumeranno questi tipi. Un'astrazione sbagliata qui non è un difetto locale, è una tassa su ogni spec successiva di M-02 e oltre.

**Il dividendo delle fixture.** I documenti di protocollo pubblicano un registro di conformità con i valori attesi di ogni hash — `ER-0`, i quattro `PD-0`, `CMT-0`, `RND-0`, `REQ-0`, `RESP-0`, `ADM-0`, `ELEC-0`, `WSC-0`, `HASH-0`. Il Lead li ha ricalcolati in modo indipendente quattro volte nel corso di [SPEC-006], validando ogni volta il metodo su una fixture non modificata prima di fidarsi delle altre. **Sono una suite di test che esiste già, scritta prima del codice.** Non li trattare come documentazione: sono l'oracolo.

## Scope
### Included

- Serializzazione canonica JCS e il registro delle preimmagini di hash, con la separazione di dominio come la specifica la definisce.
- Tipi del ledger: envelope delle transazioni, `MintBody`, `BlockHeader`, `ValidatorSet`, `ElectionRecord`, documenti di protocollo firmati.
- Alberi di Merkle: stato sparso dei conti, insieme candidato, insieme eleggibile, radice di revoca — ciascuno con la propria separazione di dominio per foglie e nodi interni.
- Derivazione dell'elezione: `election_entropy`, `election_seed`, `election_ticket`, ordinamento, tetto di riempimento, pavimento di contrazione, timbri di scadenza.
- Verifica: continuità del set, `key_binding_signature`, predicato di quorum, regola di confine, transizioni di sola rimozione.
- I controlli normativi del light client sulla composizione del set, come funzioni pure verificabili.
- Validazione del blocco di vincoli dei parametri di consenso e di `ElectionBounds`, inclusi rapporto di variazione e spaziatura.

### Excluded

- **Rete e consenso BFT.** Nessun libp2p, nessuna produzione di blocchi, nessuna devnet: sono la spec successiva, e dipendono dalla forma delle API che fissi qui.
- Storage engine e persistenza.
- Runtime WASM, challenge, e qualunque cosa oltre il ledger.
- **La scelta dei valori dei parametri**, che è di [SPEC-007] e corre in parallelo. Il core li tratta come **input di configurazione validati**, mai come costanti compilate.

## Existing-project analysis

**Lo stato reale del codice**, verificato dal Lead: `core/coblox-core/src/lib.rs` espone solo `core_version()`; `coblox-ffi` e `coblox-node` sono altrettanto vuoti. Il workspace, la CI su cinque job, il pinning delle action e i gate di lint e `cargo-deny` sono invece **completi e verdi** da [SPEC-002], quindi non devi costruire impalcatura: `unsafe_code = "forbid"` è già attivo su `coblox-core`, e `clippy` con `pedantic` è bloccante in CI.

**Il registro di conformità è normativo, e ha un obbligo che ti riguarda.** [SPEC-006] ha reso normativo che una suite di conformità **validi i propri fixture di parametri contro il blocco di vincoli prima di usarli**. La ragione è concreta e costata un giro di review: il fixture `PD-0` del progetto usava `T = 3`, che il blocco rende inammissibile a **ogni** dimensione del set. Un caso di prova costruito su parametri inammissibili asserisce un comportamento per uno stato che nessuna rete conforme può raggiungere. Applica quell'obbligo al tuo stesso codice di test.

**Due punti dove la specifica è più sottile di quanto sembri.**

L'esempio numerico dell'elezione in `ledger.md` è **normativo nella forma e non nei valori**, e i suoi parametri sono illustrativi. Riproducilo come test, ma non dedurne costanti.

I controlli del light client sono **due liste chiuse**: ciò che può stabilire e ciò che non può. La seconda lista è parte della specifica quanto la prima. Se implementi un controllo che la specifica dichiara impossibile per un light client, non hai aggiunto una garanzia: hai introdotto un'assunzione che il protocollo non autorizza.

## Technical proposal

Progetta i tipi in modo che **la canonicalizzazione sia l'unica strada**. La classe di difetto più costosa in un ledger è che un valore ammetta due serializzazioni: la firma verifica su una e la controparte ne calcola l'altra. Se la struttura consente di scrivere byte non canonici, prima o poi qualcuno lo farà.

La separazione di dominio è pervasiva nella specifica e non decorativa: ogni preimmagine porta il proprio prefisso, e le foglie degli alberi si distinguono dai nodi interni per byte di tag. Rendila difficile da sbagliare — un errore di dominio produce un hash plausibile e sbagliato, cioè il difetto peggiore da diagnosticare.

Sui parametri: entrano come configurazione validata contro il blocco di vincoli, e la validazione è un errore recuperabile e non un panico, perché in esercizio arriva da un documento governato firmato da un quorum.

## Files and areas involved

- `core/coblox-core/` — il grosso del lavoro. Organizzazione interna a tua scelta motivata.
- `core/coblox-node/` e `core/coblox-ffi/` — solo se la superficie pubblica lo richiede; non ampliare la FFI in questa spec.
- `docs/protocol/` — **sola lettura**. Se trovi un difetto nella specifica, segnalalo: non correggerlo qui.

## Acceptance criteria
- [ ] **Ogni valore del registro di conformità di `README.md` è riprodotto da un test**, con il valore atteso preso dal documento e non dall'implementazione.
- [ ] L'esempio numerico dell'elezione dell'epoca 3 è riprodotto per intero come test: foglie, foglia vuota, nodi interni, `candidate_root`, entropia, seme, biglietti, ordinamento e insediamento.
- [ ] La serializzazione canonica è verificata **in entrambe le direzioni**: i byte prodotti sono canonici, e i byte non canonici sono rifiutati e non normalizzati in silenzio.
- [ ] La derivazione dell'elezione è deterministica e testata sui casi degeneri che la specifica elenca: eleggibili insufficienti, parità, coorte intera in scadenza, interazione con la revoca.
- [ ] Il pavimento di contrazione, il tetto di riempimento e i timbri di scadenza sono implementati e testati, inclusi i due arresti che [SPEC-006] ha scoperto: genesi sincronizzata e limite di mandato decrescente.
- [ ] Il blocco di vincoli è validato per intero, e il test dimostra che `T <= 3` è rifiutato.
- [ ] I controlli normativi del light client sono funzioni pure testate, e **nessun controllo della lista delle incapacità è implementato come se fosse una capacità**.
- [ ] I parametri sono input di configurazione validati, non costanti compilate.
- [ ] La suite valida i propri fixture di parametri contro il blocco di vincoli prima di usarli.
- [ ] `cargo build`, `cargo test`, `cargo fmt --check`, `clippy -D warnings` e `cargo-deny` passano; `unsafe_code = "forbid"` resta attivo.
- [ ] Nessun file in `docs/protocol/` è stato modificato.

## Implementation plan
1. Leggere `docs/protocol/README.md`, `ledger.md` e `identity.md`. Sono lunghi e vanno letti, non campionati: la specifica è l'oracolo.
2. Serializzazione canonica e registro delle preimmagini; portare a verde le fixture che non dipendono dagli alberi.
3. Alberi di Merkle con le rispettive separazioni di dominio.
4. `ValidatorSet`, `ElectionRecord`, derivazione dell'elezione; riprodurre `ELEC-0` e l'esempio numerico.
5. Tetto, pavimento, timbri, casi degeneri.
6. Verifica dell'header, predicato di quorum, regola di confine, transizioni di sola rimozione.
7. Controlli del light client e validazione dei parametri.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-FIXTURES | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Un test riproduce **ogni** valore del registro di conformità, con l'atteso citato dal documento. Incollare l'esecuzione reale e il conteggio. Una fixture non coperta va dichiarata con la ragione, non omessa in silenzio.
- [ ] GATE-CI-GREEN | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La pipeline è verde su tutti e cinque i job sul commit consegnato, con `clippy -D warnings` e `cargo-deny` eseguiti. Il progetto ha già pagato una volta il prezzo di un gate di CI derogato ([DEBT-001]): qui non si deroga.
- [ ] GATE-LEAD-REPRO | kind=manual | owner=lead | phase=before-done | evidence=transcript | Il Lead riesegue la suite e verifica per campione che gli attesi provengano dal documento e non dall'implementazione. Un test che confronta il codice con sé stesso passa sempre ed è la modalità di fallimento specifica di questa spec.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio più probabile è il test che confronta il codice con sé stesso.** Generare l'atteso dall'implementazione e verificare che coincida produce una suite verde che non dimostra nulla. Gli attesi vengono dal documento. `GATE-LEAD-REPRO` esiste per intercettarlo, ed è la ragione per cui è `owner=lead`.
- **Rischio di costanti compilate.** I valori dei parametri arrivano da [SPEC-007], che corre in parallelo e non ha ancora finito. Se ne cabli anche uno, quella spec diventerà una modifica al tuo codice invece che una configurazione, e il documento governato che li porta perderà senso.
- **Rischio di implementare una garanzia che il protocollo non dà.** L'elenco di ciò che un light client **non** può stabilire è specifica quanto l'altro. Il progetto ha già corretto due volte affermazioni di sicurezza sovrastimate in questi stessi documenti: non introdurne una terza in codice.
- **Aperto, e spetta a te**: l'organizzazione interna del crate e la forma della superficie pubblica. Non ho prescritto moduli perché la specifica non li impone e tu vedrai la struttura naturale meglio di me leggendola. Motiva la scelta: la eredita tutto il resto di M-02.
- **Se trovi un difetto nella specifica**, e dopo venticinque finding chiusi è meno probabile ma non impossibile, **segnalalo senza correggerlo**. `docs/protocol/` è sola lettura per questa spec.

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
