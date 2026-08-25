---
id: SPEC-011
# Note: Quote the title if it contains a colon
title: "RewardBounds e le regole di validita economiche in coblox-core"
status: backlog
kind: feature
priority: high
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
capability_tier: sol
thinking_level: standard
effort_observations: []
depends_on: [SPEC-010]
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-010, ADR-011, ADR-012]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [conformance, ledger, rust, sybil]
---

# RewardBounds e le regole di validita economiche in coblox-core

## Objective

Chiudere il divario fra ciò che i documenti del protocollo dichiarano e ciò che l'unica implementazione esistente applica. [SPEC-009] ha introdotto `RewardBounds` e tre regole di validità **nei documenti e non nel codice**: `coblox-core` riproduce le fixture nuove e non applica nessuna delle regole nuove.

La conseguenza non è teorica. **Quattro casi di frontiera dichiarati `invalid` in `docs/protocol/README.md` oggi non sono rifiutati da nulla**, e il registro di conformità è per costruzione l'oracolo su cui un'implementazione indipendente si misura.

## Context

Il commit `eadba2d` di [SPEC-009] tocca `core/coblox-core/tests/` — fixture e costanti — e **non tocca `core/coblox-core/src/` in nessuna riga**. Non fu una svista dell'implementatrice: la spec delegava i documenti, e il crate era di un'altra spec. Il difetto è di cucitura fra le due, ed è la ragione per cui questa spec esiste separatamente invece di essere una remediation.

È anche la dimostrazione più chiara del motivo per cui [ADR-012] esiste: la regola nuova non invalida gli artefatti che la spec tocca, ma **quelli che nessuno sta guardando** — e questa volta l'artefatto che nessuno guardava era l'implementazione.

## Scope

### Included

- Il tipo `RewardBounds` in `coblox-core` e la sua validazione, con lo stesso trattamento già dato a `ValidatedConsensusParameters`.
- Le tre regole di validità di [ADR-010] e [ADR-011] applicate in accettazione.
- I casi di frontiera pubblicati come test, ciascuno con il proprio verdetto atteso.
- Il valore del fondo di genesi deciso dall'operatore, dove effettivamente appartiene.
- La correzione della divergenza in `sim/coblox_sim/recommended.py` descritta più sotto.

### Excluded

- **`docs/protocol/`.** Le regole sono già scritte e sono corrette; questa spec le implementa e non le riscrive. Se l'implementazione rivelasse che una regola scritta è ambigua o impossibile, **è un finding da riportare, non da correggere in autonomia** — è ciò che ha prodotto i risultati migliori di [SPEC-008].
- Il verificatore Ed25519 ([SPEC-012]) e la separazione della chiave di trasporto ([SPEC-013]).
- Qualunque logica di consenso, blocco o transazione oltre l'accettazione dei documenti governati.

## Existing-project analysis

**Verificato dal Lead il 2026-08-25 leggendo il codice**, non ricordato:

- **`RewardBounds` non esiste come tipo.** `params.rs` definisce `ElectionBounds`, non il suo gemello. Nessun tetto sul fondo, nessun rapporto di variazione, nessun gap di attivazione per la reward policy.
- **`RewardPolicyConstraints::from_body` non legge `availability_microtokens_per_unit`.** Il campo non compare nella struttura, quindi il crate accetta la tariffa positiva che `ledger.md` dichiara *rejected on acceptance*. `validate()` ha sei regole e nessuna è quella nuova.
- **`check_relations` non contiene `3 * validator_min_set_size >= 2 * V`.** Contiene `0 < min_set <= V <= max_set`, `3c < V` e `3cm <= V`, ma non il vincolo su cui poggia la sezione *«Owning the set and controlling it are different thresholds»*. Il solo limite su `min_set` è `>= validator_min_set_size_min` da `ElectionBounds`, che è una grandezza diversa.
- Le fixture di frontiera pubblicate sono in `README.md` §*reward policy* e §*RewardBounds*, e in `ledger.md` §`3 * validator_min_set_size >= 2 * V`. Sono già scritte con il verdetto atteso per ciascun caso: **sono una suite di test scritta prima del codice**, come lo fu il registro di conformità per [SPEC-008].
- `sim/tools/reward_rules.py` implementa già queste regole in Python, con i casi mirroring delle tabelle pubblicate. **È un oracolo indipendente e va usato come tale**, non riscritto: due implementazioni che concordano su una tabella pubblicata sono un'evidenza che una sola non dà.

**Una divergenza trovata dal Lead durante questa analisi, e non da una review.** `sim/coblox_sim/recommended.py` porta `existence_fund_microtokens_per_epoch = 15 882 352 941` — cioè `F_max`, il valore del regime maturo, **al tetto** — dentro un insieme di parametri chiamato `coblox-v0-genesis-candidate`. È precisamente ciò che il punto 1 di [ADR-011] vieta: un fondo di genesi dimensionato sulla rete che non c'è ancora. [SPEC-009] non toccò quel file, e `sim/tools/reward_rules.py` usa invece `300 000 000` come documento base. **I due artefatti versionati si contraddicono**, e quello che si chiama *genesis candidate* è quello sbagliato.

## Technical proposal

### 1. `RewardBounds`, sul modello di `ElectionBounds`

Il gemello esiste e va seguito, non reinventato: `ElectionBounds` è configurazione e non stato di catena, viaggia nell'ancora di fiducia di genesi, e i suoi limiti sono presi **dai bound e mai dal documento in valutazione** — la separazione che `check_magnitudes` già incarna. `RewardBounds` deve avere la stessa forma e la stessa disciplina.

Copre: il tetto sul fondo, il pavimento e il tetto sull'epoca di ricompensa, i limiti di eleggibilità, il rapporto di variazione e il gap minimo di attivazione fra sequenze consecutive.

### 2. Le tre regole, in accettazione

- `availability_microtokens_per_unit == 0`, rifiuto in accettazione se positivo.
- `3 * validator_min_set_size >= 2 * V`, nel blocco relazionale dei parametri di consenso.
- I limiti di magnitudine e di variazione della reward policy, presi da `RewardBounds`.

L'aritmetica segue la disciplina già stabilita: intermedi `u128` con moltiplicazioni verificate, e overflow che **rifiuta** invece di troncare.

### 3. I casi di frontiera, con il verdetto atteso

Ogni riga delle tabelle pubblicate diventa un caso di prova con il proprio verdetto. Il criterio che rende questa spec utile non è che i test passino: è che **ogni caso dichiarato `invalid` sia rifiutato**. Una suite di soli casi validi la passerebbe anche un validatore che accetta tutto — è la precisazione n.3 di [ADR-012] applicata a questa spec.

Attenzione alla direzione del pericolo, che [REVIEW-014] indica come il punto in cui l'errore sarebbe facile e invisibile: **tetti dove il pericolo è verso l'alto, pavimenti dove è verso il basso**. In [SPEC-009] tre dei sette limiti nuovi vanno nella direzione opposta a quella che l'intuizione suggerisce. Un limite implementato nel verso sbagliato **passa tutti i test positivi**.

### 4. Il fondo di genesi, dove appartiene davvero

L'operatore ha fissato la popolazione attesa al lancio a **~200 nodi**, quindi `existence_fund_microtokens_per_epoch` di genesi = **300 000 000 µt**, contro `F_max = 15 882 352 941`.

**Il valore non va nella fixture `PD-0`**, che è una fixture di hashing con `network_id: "fixture"` e ogni valore numerico a `1` salvo le eccezioni strutturali. Nessun hash pubblicato cambia per effetto di questa decisione. Il lavoro è invece:

- allineare `sim/coblox_sim/recommended.py`, che oggi contraddice [ADR-011];
- verificare che i due artefatti del simulatore concordino dopo la correzione;
- riportare la scelta nel rapporto economico se quel documento afferma diversamente.

## Files and areas involved

- `core/coblox-core/src/params.rs` — `RewardBounds`, `RewardPolicyConstraints`, `check_relations`.
- `core/coblox-core/src/error.rs` — le varianti di errore per le regole nuove.
- `core/coblox-core/tests/` — i casi di frontiera.
- `sim/coblox_sim/recommended.py` — la correzione del fondo di genesi.
- `.lmbrain/knowledge/economic-simulation-report.md` — solo se contiene affermazioni che la correzione rende false.

## Acceptance criteria

- [ ] `RewardBounds` esiste come tipo, con la stessa disciplina di `ElectionBounds`: configurazione, mai stato di catena, limiti presi dai bound e mai dal documento in valutazione.
- [ ] `availability_microtokens_per_unit` è letto e una tariffa positiva è **rifiutata**.
- [ ] `3 * validator_min_set_size >= 2 * V` è nel blocco relazionale ed è rifiutata la violazione.
- [ ] I limiti di magnitudine, il rapporto di variazione e il gap di attivazione della reward policy sono applicati.
- [ ] **Ogni riga delle tabelle di frontiera pubblicate** ha un caso di prova con il proprio verdetto, e i casi `invalid` sono rifiutati.
- [ ] Per ogni limite nuovo, un test dimostra il rifiuto **nella direzione giusta**: un caso che viola verso l'alto per i tetti e verso il basso per i pavimenti, e un caso che rispetta il limite al valore esatto.
- [ ] L'aritmetica usa intermedi verificati e l'overflow rifiuta invece di troncare, con un caso che lo dimostra.
- [ ] `coblox-core` e `sim/tools/reward_rules.py` **concordano su ogni caso pubblicato**, e la trascrizione mostra entrambe le esecuzioni.
- [ ] `sim/coblox_sim/recommended.py` non porta più un fondo di genesi dimensionato sulla rete matura; il valore è 300 000 000 µt e la ragione è scritta accanto.
- [ ] Nessun hash pubblicato cambia. Se qualcosa lo facesse, è un finding da riportare prima di procedere.
- [ ] La gate di [ADR-012] è eseguita con lo strumento di [SPEC-010], e la trascrizione è allegata.

## Implementation plan

1. Leggere le tabelle di frontiera pubblicate ed enumerare i casi con il loro verdetto, **prima** di scrivere codice. Sono la specifica.
2. Introdurre `RewardBounds` seguendo `ElectionBounds`, con la stessa separazione fra bound e documento.
3. Applicare le tre regole, ciascuna con il proprio errore distinto.
4. Scrivere i casi di frontiera e verificarli contro `reward_rules.py`.
5. Correggere `recommended.py` e verificare la coerenza fra i due artefatti del simulatore.
6. Eseguire la gate di [ADR-012] con lo strumento di [SPEC-010].

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-INVALID-REJECTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | **Ogni** caso dichiarato `invalid` nelle tabelle pubblicate è rifiutato, e la trascrizione lo mostra caso per caso. Una suite di soli casi validi la passa anche un validatore che accetta tutto: è la ragione per cui questa gate esiste e non è sostituibile da un conteggio di test verdi.
- [ ] GATE-DIRECTION | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Per ogni limite nuovo, la trascrizione mostra il rifiuto **nella direzione del pericolo** e l'accettazione al valore esatto del limite. Un limite implementato al contrario passa tutti i test positivi, ed è il punto che [REVIEW-014] indica come facile e invisibile.
- [ ] GATE-TWO-ORACLES | kind=manual | owner=agent | phase=before-submit | evidence=transcript | `coblox-core` e `sim/tools/reward_rules.py` producono lo stesso verdetto su ogni caso pubblicato. Due implementazioni indipendenti che concordano su una tabella pubblicata sono un'evidenza che una sola non dà.
- [ ] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato di [SPEC-010] e la trascrizione è allegata, **anche se non trova nulla**. [ADR-012] lo dice esplicitamente: una passata che non trova nulla è il caso previsto e non è evidenza che la gate sia inutile.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto l'implementazione delle regole e il Lead ha accettato la review. Le tre regole nascono da suoi finding critici, e chiuderle senza la sua verifica sarebbe incoerente con il modo in cui sono state aperte.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è il limite implementato nel verso giusto per il test sbagliato.** Tre dei sette limiti di [SPEC-009] vanno nella direzione opposta all'intuizione. `GATE-DIRECTION` esiste per questo e non va soddisfatta meccanicamente.
- **La divergenza in `recommended.py` potrebbe non essere l'unica.** Il Lead l'ha trovata guardando un file che nessuna spec stava toccando, che è il modo in cui si trovano queste cose. Se ne emergessero altre, vanno riportate e non corrette in silenzio: sanno dire qualcosa sul perimetro dell'inventario di [SPEC-010].
- **Il perimetro di [ADR-012] su `sim/` non è deciso.** La ADR dice *artefatti pubblicati*, escludendo quelli interni al brain, ma `sim/` non è né l'uno né l'altro con chiarezza. Questa spec ne è un caso concreto e la sua esperienza va riportata a [SPEC-010], che decide il perimetro.
- Se l'implementazione rivelasse che una regola scritta è ambigua, **contestarla è parte del mandato**. In [SPEC-008] due contestazioni dell'implementatore risultarono migliori della condizione di chiusura del reviewer, e in questa stessa spec l'analisi dell'esistente del Lead contiene affermazioni verificate ma non esaustive.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- **Le superfici segnalate come rischiose in questa spec non sono il perimetro.** Sono i punti dove il Lead si aspetta di essere attaccato; tre volte in questo progetto la review le ha trovate solide e ha trovato i difetti altrove.
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
