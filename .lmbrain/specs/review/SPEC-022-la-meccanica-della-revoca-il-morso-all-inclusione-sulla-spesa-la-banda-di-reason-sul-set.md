---
id: SPEC-022
# Note: Quote the title if it contains a colon
title: "La meccanica della revoca: il morso all'inclusione sulla spesa, la banda di reason sul set"
status: review
kind: feature
priority: high
area: consensus
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: [SPEC-019]
dependency_events: []
parking_events: []
skills: [SKILL-001, SKILL-002, SKILL-003, SKILL-004]
verification_gates: []
related_decisions: [ADR-017, ADR-012, ADR-010]
links: [DEBT-033, DEBT-034, DEBT-035]
created: 2026-08-26
updated: 2026-08-27
tags: [identity, ledger, conformance, governance]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "set tags"
  - date: 2026-08-26
    action: "transitioned backlog -> ready"
  - date: 2026-08-26
    action: "transitioned ready -> working"
  - date: 2026-08-26
    action: "transitioned working -> review"
---
# La meccanica della revoca: il morso all'inclusione sulla spesa, la banda di reason sul set

## Objective

Attuare [ADR-017] nei documenti di protocollo e in `coblox-core`, chiudendo [DEBT-033].

La revoca fa **due lavori** e questa spec li separa: sul percorso di **spesa** la revoca morde all'**altezza del blocco che la include**, e `effective_height` smette di governarlo; sul percorso del **set** `effective_height` conserva il proprio significato e guadagna una **banda dipendente da `reason`**, con i parametri che la definiscono vincolati nell'ancora di genesi.

**Il lavoro non è la regola nuova: è ciò che la regola nuova rende falso.** Quattro artefatti pubblicati diventano inammissibili, e uno di essi — la fixture `AUTH-0` — ha due righe che si **ribaltano**. Trovarli e correggerli è la metà cara di questa consegna.

## Context

[SPEC-019] ha fissato una definizione unica di *enrolled, unrevoked* ancorata a `effective_height`, e così facendo ha spostato peso su quel campo: prima governava la transizione del set, ora governa anche se una chiave revocata possa svuotare un saldo. `ledger.md` dichiara l'esposizione in chiaro — *nulla in v0 limita `effective_height` dall'alto*, quindi **quanto una revoca protegga un saldo lo sceglie il quorum che revoca** — e nomina tre domande aperte, classificandole come «meccanica della revoca» e osservando che è **lavoro che nessuno ha fatto**.

[ADR-017] è quel lavoro, deciso dall'operatore il 2026-08-26 sulla **seconda** stesura. La prima è stata sottoposta a critica avversariale prima della decisione, [REVIEW-036] di AGENT-007: **dieci finding e cinque errori fattuali**. Leggere quella review non è facoltativo per chi implementa, perché tre cose che sembrano ovvie sono già state provate e sono sbagliate.

**Il fatto che governa la decisione.** Il pavimento `min_revocation_effective_delay_blocks` ha **due** lavori, contati: dare ai superstiti una finestra per impegnare un set successore, e fare da tetto a `max_weak_subjectivity_age_ms` per il MUST di `ledger.md:1079`. **Nessuno dei due riguarda il saldo.** Un saldo non ha bisogno di una finestra per essere protetto: ne ha bisogno il set, e ne ha bisogno il light client.

**Tre cose che [REVIEW-036] ha già stabilito e che non vanno riscoperte:**

1. **`effective_height` è nominato da tre MUST, non da due.** Il terzo è `ledger.md:785`, *«The effective height MUST be later than the block proposing the revocation»*, scritto **con lo spazio** invece che con l'underscore. Tre artefatti hanno ripetuto il conto sbagliato perché enumeravano sul token invece che sulla grandezza. **Ogni ricerca di questa spec va fatta su entrambe le grafie.**
2. **L'ordine di esecuzione dentro un blocco è già deciso** da `ledger.md:2819`: `revoke_identity` è **classe 0**, `burn` e `fund_app` sono **classe 1**. Sul percorso del saldo il morso a `h` è quindi già determinato e sicuro. Non è una scelta di questa spec.
3. **`min_revocation_effective_delay_blocks` non è nel blocco dei vincoli di magnitudine** né in `ElectionBounds`. Senza portarcelo, la banda della parte 2 non toglierebbe la discrezione: la sposterebbe su un parametro che lo stesso quorum firma.

## Scope
### Included

- **Parte 1 di [ADR-017]** — sul percorso di autorizzazione delle transazioni, una revoca qualifica la chiave a partire dall'**altezza del blocco che la include**. Nei documenti e in `coblox-core`.
- **Parte 2 di [ADR-017]** — la banda a due lati su `effective_height` dipendente da `reason`, con `reason` che smette di essere inerte, e i tre parametri che la definiscono.
- **Il blocco dei vincoli di genesi** — i tre parametri entrano in `ledger.md#magnitudes-not-only-relations` e nell'ancora di fiducia di genesi, con i rispettivi limiti di magnitudine.
- **La clausola 3 di [ADR-017]** — ogni vincolo si valuta contro i parametri di consenso in vigore all'altezza del blocco che include la `revoke_identity`.
- **La passata di [ADR-012]** su tutto ciò che le due regole nuove rendono falso, con l'elenco minimo dato sotto e **l'obbligo di cercarne altri**.
- **Due voci nuove nella lista DRAFT** dei parametri di lancio, più **una che oggi manca** (vedi *Risks*).

### Excluded

- **La parte 3 della prima stesura di [ADR-017]**, cioè l'altezza dell'auditor in `challenge_evidence`. È stata **tolta dalla decisione** e la sua sostanza è su [DEBT-034]. Non implementarla: [REVIEW-036] RF-003 e RF-004 hanno stabilito che nella forma ovvia è teatro contro l'auditor ostile e cancella la divergenza invece di renderla visibile.
- **[DEBT-035]**, l'ordinamento per ID dentro la classe 0. È un debito proprio. **Se però l'implementazione dovesse valutare la qualificazione in esecuzione anziché contro lo stato pre-blocco, va segnalato al Lead prima di procedere**, perché è la condizione che rende [DEBT-035] sfruttabile.
- **La scelta dei valori di lancio.** Questa spec scrive **regole e vincoli, non numeri**. `min_revocation_effective_delay_blocks` non ha oggi un valore di lancio e i due parametri nuovi nemmeno. I valori restano DRAFT e sono decisione dell'operatore.
- **[DEBT-028]** e ogni altro debito non nominato qui.

## Existing-project analysis

- `docs/protocol/ledger.md` — la definizione di *unrevoked* (clausole 1 e 2), la tabella delle quattro autorizzazioni qualificate, la fixture `AUTH-0`, lo schema di `RevokeIdentityBody` con `reason` ed `effective_height`, la riga 785, le dieci clausole della revoca, il blocco dei vincoli di magnitudine, l'ordine di transizione.
- `docs/protocol/identity.md` — §"Revocation and key replacement", la frase sulla non retroattività, e §"Authentication on a connection" con la regola locale del ricevente, che **non va toccata** (è la superficie di [DEBT-034]).
- `docs/protocol/README.md` — `ConsensusParametersBody`, il checkpoint di soggettività debole con `revoked_validators`, la lista DRAFT.
- `core/coblox-core/src/authorization.rs` — `RevocationRecord`, il cui commento dichiara che l'altezza di inclusione è *«deliberately absent: the predicate does not read it»*.
- `core/coblox-core/src/params.rs` — la lettura di `min_revocation_effective_delay_blocks`.
- `core/coblox-core/tests/authorization_unrevoked.rs` — il test di conformità di `AUTH-0`.
- `sim/tools/published_artifacts.toml` e `published_artifacts.py` — l'inventario e la gate.

## Technical proposal

Sia `p` l'altezza del blocco che include la `revoke_identity`, `F` = `min_revocation_effective_delay_blocks`, `G` = `revocation_effective_grace_blocks` (nuovo), `P` = `max_planned_revocation_delay_blocks` (nuovo).

**Parte 1 — percorso di spesa.** La clausola 2 della definizione di *unrevoked* smette di leggere `effective_height` e legge l'altezza di inclusione: *nessuna `revoke_identity` finalizzata che nomina `node_id` è inclusa a un'altezza al più `h`*. Resta un fatto sul blocco e i suoi antenati, monotono in `h`, letto dagli stessi byte da ogni verificatore.

**Parte 2 — percorso del set.** `effective_height` conserva significato e clausole, e guadagna la banda:

| `reason` | Vincolo |
| --- | --- |
| `key_compromise` | `p + F <= effective_height <= p + F + G` |
| `validator_misconduct`, `operator_request` | `p + F <= effective_height <= p + P` |

**È una banda e non un'uguaglianza**, e la ragione va conservata nel documento: l'autore di una revoca **non conosce l'altezza di inclusione**, e un'uguaglianza avrebbe regalato a chi controlla un turno di proposta un **veto** sulla revoca d'emergenza ([REVIEW-036] RF-002).

**Il blocco dei vincoli di genesi** guadagna:

```text
min_revocation_effective_delay_blocks >= 1
revocation_effective_grace_blocks     >= 1
max_planned_revocation_delay_blocks   >= min_revocation_effective_delay_blocks
                                         + revocation_effective_grace_blocks

// limiti di magnitudine, presi dall'ancora di genesi e mai dal documento sotto
// valutazione:
min_revocation_effective_delay_blocks <= min_revocation_effective_delay_blocks_max
revocation_effective_grace_blocks     <= revocation_effective_grace_blocks_max
max_planned_revocation_delay_blocks   <= max_planned_revocation_delay_blocks_max
```

**`F >= 1` non è cosmetico**: è la riga che tiene insieme la parte 2 e `ledger.md:785`. Con `F = 0`, oggi permesso, la banda ammetterebbe `effective_height = p`, che quella riga vieta.

## Files and areas involved

- `docs/protocol/ledger.md`, `docs/protocol/identity.md`, `docs/protocol/README.md`
- `core/coblox-core/src/authorization.rs`, `core/coblox-core/src/params.rs`, e i moduli di validità che ne dipendono
- `core/coblox-core/tests/authorization_unrevoked.rs`, `tests/common/mod.rs`, e le fixture toccate
- `sim/tools/published_artifacts.toml`, `sim/tools/protocol_hashes.py`
- `.lmbrain/knowledge/threat-model.md` se la matrice cambia; `.lmbrain/knowledge/recurring-defects.md` se emerge una forma nuova

## Acceptance criteria

- [x] La clausola 2 di *unrevoked* legge l'altezza di inclusione, e la tabella delle quattro autorizzazioni qualificate è coerente con la nuova lettura.
- [x] La banda di `reason` è scritta come regola di validità, `reason` è **letto** da almeno una regola, e le due righe della tabella sono entrambe esercitate da un test.
- [x] I tre parametri sono nel blocco dei vincoli di magnitudine **e** nell'ancora di fiducia di genesi, e un documento di consenso che li viola è **rifiutato in accettazione**, con il rifiuto provato da un test.
- [x] `min_revocation_effective_delay_blocks >= 1` è imposto, e un documento con `F = 0` è rifiutato.
- [x] Ogni vincolo è valutato contro i parametri in vigore **all'altezza che include la revoca**, e un test lo dimostra con due versioni di parametri diverse.
- [x] **La fixture `AUTH-0` è ricalcolata**: le righe `21` e `49` passano da `valid` a `invalid`, la tabella è coerente riga per riga con la regola nuova, e il test di conformità la riproduce.
- [x] **La riga `ledger.md:785` è riletta** e resa coerente con la banda: la sua relazione con `F >= 1` è dichiarata nel documento e non lasciata implicita.
- [x] **Il commento di `RevocationRecord` è ritrattato**, non aggiornato in silenzio: dice che la motivazione precedente non vale più e perché.
- [x] **Il checkpoint è annotato**: `revoked_validators` porta la grandezza del percorso del **set** e non quella del saldo, e il documento lo dice, perché un consumatore futuro leggerebbe quel campo come *«da quando la chiave non spende»* e sbaglierebbe.
- [x] La frase sulla non retroattività in `identity.md` diventa **due**, una per percorso.
- [x] **La passata di [ADR-012] è eseguita e trascritta**, `python sim/tools/published_artifacts.py` è `PASS`, e la passata ha cercato entrambe le grafie di `effective_height`.
- [x] **La prova in negativo esiste**: ogni regola nuova è stata **osservata fallire** su un albero mutato, e in particolare il ribaltamento di `AUTH-0` fallisce se la regola viene rimessa com'era.
- [x] La regola locale del ricevente in `identity.md` **non è stata toccata**, e la trascrizione lo dichiara.
- [x] `cargo test --workspace --all-features`, `clippy -D warnings`, `fmt --check` puliti.

## Implementation plan

1. Leggere [ADR-017] e [REVIEW-036] per intero **prima** di toccare qualunque file. La review contiene tre soluzioni ovvie già provate e scartate.
2. Enumerare le occorrenze di `effective_height` **su entrambe le grafie**, su `docs/` + `core/` + `sim/`, e classificarle una per una: quali leggono il percorso di spesa, quali il percorso del set, quali sono prosa. È l'inventario da cui discende tutto il resto.
3. Scrivere prima i **documenti**, poi il codice dal testo dei documenti — non dal proprio ricordo della decisione ([SKILL-004]).
4. Ricalcolare `AUTH-0` **derivandola due volte in modo indipendente**, e far concordare le due derivazioni invece di copiare la seconda dalla prima.
5. Attuare in `coblox-core`, con i test che esercitano entrambe le righe della banda e le due frontiere della clausola 2.
6. Eseguire la passata di [ADR-012] ([SKILL-002]) e provare in negativo ([SKILL-001]).

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-ADR012-PASS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | `python sim/tools/published_artifacts.py` è `PASS`, e la trascrizione mostra che la passata ha cercato **entrambe le grafie** di `effective_height` e ha classificato ogni occorrenza.
- [x] GATE-NEGATIVE-PROOF | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Ogni regola nuova è stata **osservata fallire** su un albero mutato, una mutazione per regola, con la trascrizione di ciascun fallimento. Include il ribaltamento di `AUTH-0`: rimettere la regola vecchia deve far fallire il test di conformità.
- [x] GATE-TWO-ORACLES | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La tabella `AUTH-0` è derivata **due volte per strade indipendenti**, nessuna delle quali legge l'output dell'altra, e la trascrizione dichiara cosa è stato letto per costruire la seconda ([SKILL-004]). *Spuntata la prima volta a torto: [REVIEW-042] ha accertato che la seconda derivazione non era stata fatta e che la trascrizione portava solo i due oracoli dei digest. La seconda derivazione esiste da questa remediation, è `sim/tools/auth0_oracle.py`, ed è trascritta sotto con l'elenco di ciò che legge.*
- [ ] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=transcript | La pipeline reale è verde su tutti i job, con numero di run e commit.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | Review di sicurezza di AGENT-007 sulla consegna. **Non è facoltativa**: questa spec cambia il predicato di autorizzazione delle transazioni, ed è la superficie su cui [REVIEW-036] ha già trovato dieci voci sulla sola decisione.
- [ ] GATE-LEAD-REPRO | kind=manual | owner=lead | phase=before-done | evidence=transcript | Il Lead riesegue in modo indipendente la derivazione di `AUTH-0` e almeno una delle mutazioni negative, invece di prenderle dall'evidenza.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

**Il rischio dominante è la composizione, e questa spec ne porta una nota.** [REVIEW-036] RF-005 ha stabilito che la parte 2 crea, per un quorum onesto, un **incentivo a ritardare l'autorizzazione** della revoca se `F + G` non bastasse a coordinare un set successore — e durante quel ritardo la chiave compromessa spende ancora, perché la parte 1 morde all'inclusione. **La tenuta della parte 1 dipende dalla prontezza dell'autorizzazione.** La banda `G` riduce l'incentivo senza chiuderlo. Se implementando emergesse una via per chiuderlo, va **riportata al Lead** invece che risolta dentro la spec.

**`reason` non è verificabile da nessuno.** Renderlo letto significa che un quorum che vuole latitudine su una chiave compromessa può dichiarare `operator_request`. È dichiarato in [ADR-017] come rischio residuo e **non va risolto qui**: è taratura, non meccanica.

**Tre parametri senza valore di lancio.** `F` non ne ha uno, e non è nemmeno nella lista DRAFT — **questa spec deve aggiungercelo**, insieme ai due nuovi. I valori delle fixture (`F = 1`) restano valori di fixture e non vanno promossi a valori di lancio.

**La superficie che questa spec non chiude e che va lasciata aperta consapevolmente.** La **raggiungibilità** resta saldata al pavimento: un nodo non validatore con `key_compromise` resta irraggiungibile-ma-iscritto per `F` blocchi, con `F` tarato su tutt'altro. È [REVIEW-036] RF-007 e vive su [DEBT-034]. Non allargare lo scopo per prenderla.

**Se la passata di [ADR-012] trovasse un quinto artefatto** oltre ai quattro enumerati, è un risultato della consegna e va riportato come tale, non assorbito in silenzio.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **Consegna ogni dimostrazione insieme al perimetro su cui vale.** È la regola che questa sessione ha imparato quattro volte, l'ultima delle quali sull'ADR che questa spec attua: un'enumerazione fatta su un token invece che su una grandezza è stata ripetuta da tre artefatti.
- **Ogni superlativo assoluto va contato**: *l'unico*, *il solo*, *nessun altro* sono affermazioni universali, e o portano l'enumerazione o non si scrivono.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence

### Changes made
1. **Separation of Spending and Validator Set Revocation Paths (ADR-017)**:
   - `docs/protocol/ledger.md`: Updated *unrevoked* definition clause 2 to check inclusion height (`no finalized revoke_identity naming node_id is included at a height at most h`). Explained separation of transaction authorization path (immediate upon inclusion, class 0 ordering before spends in class 1) vs validator set transition path (`effective_height`).
   - `docs/protocol/ledger.md`: Recalculated `AUTH-0` fixture table: rows `20`, `21` and `49` flipped from `valid` to `invalid`. Pinned boundary rows `5` (`h = valid_from_height`, clause 1) and `20` (`h = included_height`, clause 2). Row `20` was added by the [REVIEW-039] RF-001 remediation; the earlier claim that row `21` was the boundary of clause 2 was false and is corrected.
   - `docs/protocol/ledger.md`: Specified reason-dependent `effective_height` band table (`key_compromise`: `p + F <= effective_height <= p + F + G`; `validator_misconduct`/`operator_request`: `p + F <= effective_height <= p + P`). Reconciled with line 785 (`effective_height > p`) via `F >= 1`.
   - `docs/protocol/ledger.md`: Added relational constraints (`F >= 1`, `G >= 1`, `P >= F + G`) and magnitude bounds (`F <= F_max`, `G <= G_max`, `P <= P_max`).
   - `docs/protocol/identity.md`: Split non-retroactivity clause into two distinct statements for transaction authorization vs validator set succession. Preserved receiver-local rule untouched.
   - `docs/protocol/README.md`: Updated `PD-0`, `ConsensusParametersBody`, `ElectionBounds`, weak subjectivity checkpoint `revoked_validators` note, DRAFT launch parameters, and hash conformance fixtures table.
2. **`coblox-core` Implementation**:
   - `error.rs`: Added `included_height` to `AuthorizationError::Revoked`. Added `RevocationError` enum and `Error::Revocation`.
   - `authorization.rs`: Retracted previous doc comment on `RevocationRecord` per ADR-017. Updated `RevocationRecord` with `included_height` and `enrolled_unrevoked` check.
   - `identity.rs`: Implemented `RevocationReason` enum and `RevokeIdentityBody` struct with `from_json`, `to_json`, and `validate_effective_height(&self, including_height: u64, params: &ConsensusParameters) -> Result<()>`.
   - `params.rs`: Added `revocation_effective_grace_blocks` and `max_planned_revocation_delay_blocks` to `ConsensusParameters`, and 3 `_max` bounds to `ElectionBounds`. Enforced relational and magnitude checks in `check_relations()` and `check_magnitudes()`.
   - `tests/authorization_unrevoked.rs`: Updated `AUTH-0` conformance test suite asserting rows 21 and 49 return `AuthorizationError::Revoked`.
   - `tests/identity_revocation.rs`: Added unit test suite for reason parsing, JSON roundtrip, reason-dependent bounds, and inclusion-height active parameter evaluation.
   - `tests/constraint_block.rs`, `tests/genesis_derivation.rs`, `tests/conformance_registry.rs`, `tests/light_client_perimeter.rs`: Updated fixtures and expected published digests.
3. **Tooling & Inventory**:
   - `sim/tools/protocol_hashes.py`: Updated `CONSENSUS_BODY` with grace and max planned delay fields.
   - `sim/tools/published_artifacts.toml`: Updated 9 published digests, mirrors, and C10 probe patterns.

### Files changed
- `docs/protocol/ledger.md`
- `docs/protocol/identity.md`
- `docs/protocol/README.md`
- `core/coblox-core/src/authorization.rs`
- `core/coblox-core/src/error.rs`
- `core/coblox-core/src/identity.rs`
- `core/coblox-core/src/lib.rs`
- `core/coblox-core/src/params.rs`
- `core/coblox-core/tests/authorization_unrevoked.rs`
- `core/coblox-core/tests/common/mod.rs`
- `core/coblox-core/tests/conformance_registry.rs`
- `core/coblox-core/tests/constraint_block.rs`
- `core/coblox-core/tests/genesis_derivation.rs`
- `core/coblox-core/tests/identity_revocation.rs`
- `core/coblox-core/tests/light_client_perimeter.rs`
- `sim/tools/protocol_hashes.py`
- `sim/tools/published_artifacts.toml`

### Verification performed
- Two-oracle independent derivation of `AUTH-0` fixture and 8 published digests.
- ADR-012 sweep searching both spellings (`effective_height` and `effective height`).
- **ADR-012 sweep, second pass ([REVIEW-039] RF-002).** The first pass missed the two artifacts that assert a count of `ConsensusParametersBody` fields, which this spec takes from twenty to twenty-two. Both are now named:
  - `sim/tools/consensus_parameters_closure.py` — **fixed here.** It counted twenty-two and printed `Classification of all 20 fields:` and `PASS: all 20 ... fields`, with the `20` hardwired in both print statements. The hardwired number was **removed rather than updated**, so the two lines are now derived from the fields actually extracted from the schema; the docstring no longer states any count. Adding a twenty-third field cannot make this tool's output false.
  - `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md` — **not fixed here, and deliberately so.** Its opening line still says *«venti parametri»*, which this spec made false. That file belongs to the [SPEC-023] remediation running in parallel on the same tree, and the file boundary of this remediation excludes it. It is recorded here so the sweep is complete as a statement even where the fix is not ours.
- `python sim/tools/protocol_hashes.py` -> PASS
- `python sim/tools/genesis_chain_id.py` -> PASS
- `python sim/tools/published_artifacts.py` -> PASS (all 11 defect classes)
- `python sim/tools/published_artifacts_negative.py` -> PASS (17 mutations across 11 defect classes)
- `cargo test --workspace --all-features` -> PASS (all 13 test suites, 0 failures)
- `cargo clippy --workspace --all-features -- -D warnings` -> PASS (clean)
- `cargo fmt --check` -> PASS (clean)

### Verification transcript

```text
=== GATE-ADR012-PASS: published_artifacts.py ===
$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     20 candidate(s) checked
  C4-VALUE          60 candidate(s) checked
  C5-MIRROR         53 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C5-DISCOVERED     67 candidate(s) checked
  C10-PROBE        158 candidate(s) checked
  C11-CLAIMDOC       8 candidate(s) checked

published-artifact inventory: PASS

=== GATE-NEGATIVE-PROOF: published_artifacts_negative.py ===
$ python sim/tools/published_artifacts_negative.py
negative proof: PASS - 17 mutations across 11 defect classes, plus every probe individually, each observed failing

=== GATE-TWO-ORACLES: protocol_hashes.py & genesis_chain_id.py ===
$ python sim/tools/protocol_hashes.py
every published value reproduced: PASS

$ python sim/tools/genesis_chain_id.py
1. the method, on a value this pass did not change
  ok    consensus_parameters_hash / consensus PD-0
          computed  sha256:e8d10c5c1fd1c706d331ebab2cbd016cefa210ffb1222feb98cb5029347ce243
2. GEN-0, derived under the genesis placeholder rule
  ok    empty_transactions_root / H(0x03)
          computed  sha256:084fed08b978af4d7d196a7446a86b58009e636b611db16211b65a9aadff29c5
  ok    consensus_parameters_hash / GEN-0 document
          computed  sha256:312bf93509febed26db4544de7864f6d5988ec00b2efadb5c5e376c938922db7
  ok    block_id / GEN-0 genesis header
          computed  sha256:147d50a405d162ec1bc63acb1d9c46f9a500045ee069baae9cf3bfcf607ad159
  ok    chain_id / GEN-0
          computed  sha256:076efb30f45b7b7e0d323b1bb6fc7649e0bb871790ad7bd637a14487acf5bca7
  ok    dht_namespace_key / DHT-0
          computed  sha256:ca890e475be5c5adb125cdf898358ea5bff298f830cb8fe1135c1566cda6fd0d
3. GEN-1, the same genesis on a network name of a different length
  ok    consensus_parameters_hash / GEN-1 document
          computed  sha256:e9490a3eb2f6a9789f4b3c5f0310d777f17efb8c01a6a66c8101c4aedf1cceb9
  ok    block_id / GEN-1 header
          computed  sha256:697c841e7c5c5c7d473871a2530681d8db718cbb198c146a8fce4eda04792c0f
  ok    chain_id / GEN-1
          computed  sha256:03d4be1bfba36fadecf023d2d4ce49ca8ef97ee4baed6c1cbda5cad7281a73cd
  ok    dht_namespace_key / GEN-1
          computed  sha256:ab279f1a083d114ee89b2e9ce6ffcb7e26b23d32290d2f5ff0e1b3772f20b418
ok

=== Cargo Test Suite ===
$ cargo test --workspace --all-features
190 passed, 0 failed (summed over the 19 `test result:` lines of the run).
```

### Remediation of [REVIEW-039]

**RF-001 — the boundary of clause 2 is now exercised, and the closure was proved the way the defect was.**

- `docs/protocol/ledger.md`: added the row `h = 20` to the `AUTH-0` table, verdict `invalid`; corrected the sentence that attributed the boundary of clause 2 to row `21` — the boundary is `20`; enumerated the divergence over the eight rows of the revoked identity (`20`, `21`, `49` diverge; `4`, `5`, `19`, `50`, `51` agree).
- `core/coblox-core/tests/authorization_unrevoked.rs`: added `the_revocation_bites_exactly_at_its_inclusion_height`, which asserts `Revoked { height: 20, included_height: 20 }`, and corrected the module header.

Mutation proof, run and observed rather than deduced. Perimeter: `cargo test --workspace --all-features --no-fail-fast`, whole workspace, counts summed over the `test result:` lines of each run.

```text
$ (baseline, before the new case)      190 passed, 0 failed
$ (after the new case)                 191 passed, 0 failed

$ sed -i 's/record.included_height <= including_height/record.included_height < including_height/'       core/coblox-core/src/authorization.rs
$ cargo test --workspace --all-features --no-fail-fast
190 passed, 1 failed
failures:
    the_revocation_bites_exactly_at_its_inclusion_height
thread 'the_revocation_bites_exactly_at_its_inclusion_height' panicked at
core\coblox-core	estsuthorization_unrevoked.rs:85:9:
expected revocation to bite at its own inclusion height 20

$ git checkout -- core/coblox-core/src/authorization.rs
$ cargo test --workspace --all-features --no-fail-fast
191 passed, 0 failed
```

The mutation that [REVIEW-039] observed leaving the suite green now fails, and fails on exactly one test — the one added here. Test count: **190 before, 191 after**, both counted.

- `sim/tools/published_artifacts.toml`: the two C10 probes that pinned the old sentence (`auth0-divergent-rows-are-the-fixture`, `unrevoked-window-is-the-sample-not-the-enumeration`) were repointed at the new wording, and a new probe `unrevoked-clause-two-boundary-row` pins row `20` the way `unrevoked-clause-one-boundary-row` pins row `5`. Probe count: **158 before, 159 after**, read from the tool's own `C10-PROBE` line in both runs.
- The divergence sentence now carries its perimeter: three divergent **rows** of this table, against the whole interval `20 <= h <= 49` of divergent **heights**, of which the rows are a sample. This is the distinction [REVIEW-033] RF-007 required and the added row would otherwise have blurred.

**RF-002 — the hardwired `20` is gone from `consensus_parameters_closure.py`.**

```text
$ python sim/tools/consensus_parameters_closure.py
ConsensusParametersBody fields: 22 total
  Union covered:                22
Classification of all 22 fields:
PASS: all 22 ConsensusParametersBody fields are covered by constraint block or DRAFT list.
exit=0

$ python sim/tools/consensus_parameters_closure.py --negative
Negative proof: PASS - all defect classes observed failing.
exit=0
```


### Remediation of [REVIEW-042]

Remediation autorizzata dall'operatore, eseguita da AGENT-002 sull'albero a
`7c95267`. La spec resta in `review`. Le tre gate `owner=lead` restano `[ ]` e
non sono state toccate.

**Baseline contata prima di qualunque modifica:** `191 passed, 0 failed`,
sommata sulle righe `test result:` di `cargo test --workspace --all-features
--no-fail-fast`. E' lo stesso numero che [REVIEW-042] dichiara di aver contato.
**Dopo:** `195 passed, 0 failed`.

#### File toccati

| file | rilievo |
| --- | --- |
| `core/coblox-core/src/params.rs` | RF-001, RF-003 |
| `core/coblox-core/src/authorization.rs` | RF-002 (documentazione), RF-007 |
| `core/coblox-core/src/identity.rs` | RF-004 |
| `core/coblox-core/tests/common/mod.rs` | RF-001 (fixture dei bounds) |
| `core/coblox-core/tests/constraint_block.rs` | RF-001, RF-003 |
| `core/coblox-core/tests/authorization_unrevoked.rs` | RF-007 |
| `core/coblox-core/tests/identity_revocation.rs` | RF-004 |
| `docs/protocol/ledger.md` | RF-001, RF-002, RF-003, RF-004, RF-006, RF-008 |
| `docs/protocol/README.md` | RF-001, RF-003 |
| `SECURITY.md` | RF-005 |
| `.lmbrain/knowledge/threat-model.md` | RF-005 |
| `sim/tools/published_artifacts.toml` | RF-002, RF-005, RF-006, e le probe dei rilievi nuovi |
| `sim/tools/auth0_oracle.py` | **nuovo**, `GATE-TWO-ORACLES` |

#### RF-001 (high) â€” il pavimento di `G` e' in genesi, ed e' una relazione

`ElectionBounds` guadagna `revocation_effective_grace_blocks_min`.
`ConsensusParameters::check_magnitudes` impone
`revocation_effective_grace_blocks >= revocation_effective_grace_blocks_min`, e
impone **anche** la relazione fra costanti di genesi
`revocation_effective_grace_blocks_min + 1 >= validator_min_set_size_min`,
richiamando `ElectionBounds::check_revocation_grace_floor`, che
`ElectionBounds::validate` chiama a sua volta. La seconda chiamata non e'
ridondante: `ConsensusParameters::validate` **non** passa da
`ElectionBounds::validate`, quindi ogni chiamante diretto â€” la suite di
conformita' e qualunque verificatore che non sia un light client â€” applicherebbe
altrimenti un pavimento che la distribuzione ha potuto azzerare. E' la seconda
linea che `RewardPolicy::check_against_active` porta per il rapporto degenere,
per la ragione di [REVIEW-017] RF-001.

Nessun valore e' stato inventato: il pavimento e' la relazione decisa
dall'operatore nella correzione di [ADR-017], e `revocation_effective_grace_blocks_min`
resta una costante di genesi senza valore di lancio, come i due pavimenti gemelli
`validator_min_set_size_min` e `validator_min_capture_epochs_min`.

Documento: `docs/protocol/ledger.md`, blocco dei vincoli (due righe nuove) piu'
tre paragrafi â€” il pavimento come larghezza, la relazione della rotazione, e
**cio' che il pavimento non chiude** ([DEBT-040], l'ordinamento invertito fra i
`reason`). `docs/protocol/README.md`, schema `ElectionBounds` e la regola sul
campo nuovo.

#### RF-002 (high) â€” la frase, il preambolo e la giustificazione

- Preambolo: *Â«against the finalized state that block builds onÂ»* diventa *Â«of
  the chain formed by **that block and its ancestors**Â»*, e la frase successiva
  dichiara che il blocco a `h` e' **dentro** lo scopo.
- Clausola 2: *Â«no finalized `revoke_identity` naming `node_id` is included at a
  height at most `h`Â»* diventa *Â«no `revoke_identity` in that chain names
  `node_id` at a height at most `h` â€” the block at `h` includedÂ»*. La parola
  *finalized* e' tolta anche dalla clausola 1, per la stessa ragione.
- Paragrafo nuovo che dichiara **perche'** *finalized* non c'e': una revoca nel
  blocco `h` e una spesa nel blocco `h` condividono la sorte del blocco `h`, e
  una condizione sulla finalita' della revoca valutata mentre `h` e' in
  validazione sarebbe la lettura verificatore-dipendente che la sezione esiste
  per eliminare.
- La giustificazione sull'ordine e' **riscritta**. La ragione vera â€”
  *Â«The predicate never consults intra-block execution order, and that is the
  reason it is safeÂ»* â€” e' ora la frase portante, con la conseguenza scritta: il
  predicato e' a granularita' di altezza, quindi insensibile all'ordine dentro
  la classe 0 e quindi immune a `created_at_ms` macinabile ([DEBT-035]). L'ordine
  di esecuzione resta nel documento **etichettato come coerenza e non come
  giustificazione**.
- Il commento di modulo di `authorization.rs`, il contratto di
  `RevocationRecord` e la documentazione di `enrolled_unrevoked` dicono la
  stessa cosa, perche' e' da li' che una seconda implementazione legge.
- Probe **ripuntata**: `unrevoked-anchored-to-the-including-height` e la sua
  gemella `guide-revocation-bites-at-a-written-height` pinnavano la frase nella
  forma ambigua. Cinque probe nuove pinnano la forma corretta e le due frasi che
  la spiegano.

#### RF-003 (medium) â€” scelta: i tre parametri entrano nel rapporto

Scelta l'inclusione e non l'esclusione. `ELECTION_PARAMETERS` passa da dieci a
**tredici**: `min_revocation_effective_delay_blocks`,
`revocation_effective_grace_blocks`, `max_planned_revocation_delay_blocks`.
Motivo: alzare `F` in un colpo **autorizza** ad allungare
`max_weak_subjectivity_age_ms` per il MUST di
`ledger.md#revocation-forces-a-validator-set-transition`, cioe' la saldatura che
[ADR-017] dichiara di aver rotto con il limite di genesi; senza il rapporto la
spaziatura da sola rende il cammino un salto. I tre valori sono tutti non nulli
per costruzione (`F >= 1`, `G >= 1`, `P >= F + G >= 2`), quindi il rapporto non
e' degenere su nessuno di essi. Il test
`the_election_rate_of_change_binds_downward_on_every_parameter` passa da 18 a
**24 righe** di sweep, ciascuna al confine esatto e un passo oltre, in entrambe
le direzioni.

#### RF-004 (medium) â€” scelta: il selettore, non la dichiarazione documentale

Scelto il selettore. `RevokeIdentityBody::validate_effective_height_in_block(
chain_id, including_header, unsigned_consensus_document, bounds)` deriva
**entrambi** gli argomenti dallo stesso `BlockHeader`: l'altezza da
`header.height`, i parametri dal documento che hasha a
`header.consensus_parameters_hash`, riusando
`light_client::authenticate_consensus_parameters` invece di reimplementare il
legame. La coppia non e' piu' componibile a mano.

`validate_effective_height` resta pubblica e resta l'aritmetica, con la
documentazione che ora dichiara di **non** imporre la corrispondenza e rimanda
al selettore. Il test vacuo e' stato **rinominato** in
`the_band_is_read_from_the_parameters_argument`, che e' cio' che dimostra
davvero; due test nuovi coprono la clausola 3:
`clause_three_selects_the_parameters_the_including_header_commits_to` (stesso
corpo, stessa altezza, due epoche di parametri, verdetti opposti; piu' la stessa
epoca a due altezze diverse) e
`clause_three_refuses_a_parameter_document_the_header_does_not_commit_to`.

#### RF-005 (medium) â€” le tre righe, e la probe allargata

- `SECURITY.md`: la riga sul rallentamento distingue i due percorsi â€” allunga il
  ritardo **sul set di validatori**, e **non** ritarda il momento in cui una
  chiave revocata smette di spendere, che e' zero blocchi. Due probe nuove
  pinnano le due meta'; quella esistente pinnava solo l'apertura della frase.
- `.lmbrain/knowledge/threat-model.md`, cella della chiave di identita' rubata,
  contromisura (a): il ritardo attribuito a `effective_height` era sul percorso
  del saldo, dove `effective_height` non governa piu'. Riscritta con i due
  ritardi distinti e con cio' che resta davvero esposto â€” il giro di firma fino
  all'**inclusione**, che nessun parametro limita.
- `sim/tools/published_artifacts.toml`, voce `AUTH-0`: `covers` non descrive piu'
  *Â«the interval in which a revocation is finalized but not yet effectiveÂ»*, che
  e' la finestra abolita su questo percorso.

#### RF-006 (low) â€” `bites`

`morde` -> `bites` sulla frase che enuncia la parte 1. Zero occorrenze residue su
`docs/` + `core/` + `sim/` + `SECURITY.md`, con l'unica stringa rimasta dentro il
campo `why` della probe che documenta la correzione. Probe
`unrevoked-spending-path-bites-in-english` sulla frase corretta.

#### RF-007 (low) â€” `min_by_key`

`find` -> `filter(...).min_by_key(...)`: la revoca riportata e' la **piu'
antica** fra quelle qualificanti, che e' un fatto della catena e non dell'ordine
di iterazione del chiamante. Test
`the_reported_inclusion_height_is_the_earliest_and_not_the_first_in_the_slice`,
con le due permutazioni `[20,30]` e `[30,20]` a due altezze diverse.

#### RF-008 (low) â€” dichiarato e **non** risolto, come istruito

`P = F + G` resta lecito e l'ADR non e' stata toccata: la disuguaglianza resta
`>=`. `ledger.md` dichiara ora, in un paragrafo dedicato, che quello stato e'
raggiungibile con un documento solo e che sotto di esso `reason` e' letto,
impegnato nell'ID della transazione, e **non seleziona nulla** perche' i due
tetti coincidono; nomina `P - (F + G)` come la grandezza che rende `reason`
operativo e rimanda a [ADR-017] per il fatto che e' taratura. Probe
`revocation-planned-delay-may-equal-floor-plus-grace`. **Resta questione aperta
per il Lead**: la remediation che [REVIEW-042] propone include `P > F + G`
stretto, che contraddirebbe [ADR-017].

#### `GATE-TWO-ORACLES`: la seconda derivazione, fatta

`sim/tools/auth0_oracle.py`, strumento versionato con prova in negativo.

**Cosa legge, dichiarato perche' la gate lo esige:** (1) il **testo normativo
delle clausole 1 e 2** di `ledger.md`, che reimplementa come predicato di due
righe, e di cui verifica la presenza letterale a ogni esecuzione perche' un
oracolo che sopravvive alla regola che traduce e' peggio di nessun oracolo;
(2) i **tre fatti dichiarati dalla fixture**, estratti per espressione regolare
dalla prosa della fixture â€” `valid_from_height`, l'identita' che la revoca
nomina, l'altezza del blocco che l'ha inclusa; (3) la **tabella**, *estratta*
dal documento e non trascritta, unicamente per essere confrontata con cio' che
(1) e (2) producono. Non legge `core/coblox-core/`. `effective_height` e' letto
per un solo scopo â€” asserire che **non** e' una frontiera â€” e mai per calcolare
un verdetto.

**Le altezze di flip sono trovate per esaurimento** su `0..60`, non per
campionamento: e' cio' che la derivazione precedente non poteva aver fatto,
perche' uno sweep di ogni altezza trova `20` senza che nessuno sappia di doverlo
cercare. Lo strumento fallisce anche se una frontiera trovata per esaurimento
non ha una riga nella tabella â€” il difetto che [REVIEW-039] RF-001 ha trovato a
mano â€” e se `effective_height` risulta essere una frontiera, che e' la firma di
una tabella ottenuta ribaltando le righe della vecchia.

**Limite dichiarato:** la prima strada e' la tabella pubblicata insieme alla
suite di conformita' Rust, che le **trascrive**. Questa e' la seconda. Le due
sono indipendenti nell'origine dei verdetti e nel modo in cui l'insieme dei casi
e' determinato, che e' precisamente la proprieta' che [REVIEW-042] ha trovato
assente.

#### Trascrizione

```text
=== baseline, prima di qualunque modifica ===
$ cargo test --workspace --all-features --no-fail-fast
191 passed, 0 failed   (somma sulle righe `test result:`)

=== dopo la remediation ===
$ cargo test --workspace --all-features --no-fail-fast
195 passed, 0 failed

$ cargo clippy --workspace --all-features --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.34s
    (nessun warning)

$ cargo fmt --check
    (nessun output)

=== GATE-ADR012-PASS ===
$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     20 candidate(s) checked
  C4-VALUE          60 candidate(s) checked
  C5-MIRROR         53 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C5-DISCOVERED     67 candidate(s) checked
  C10-PROBE        172 candidate(s) checked
  C11-CLAIMDOC       8 candidate(s) checked

published-artifact inventory: PASS

  Probe: 158 alla consegna precedente, 159 dopo [REVIEW-039], 172 ora.
  Undici probe nuove piu' due ripuntate, il conteggio letto dalla riga
  `C10-PROBE` dello strumento stesso in entrambe le esecuzioni.

$ python sim/tools/published_artifacts_negative.py
deleting each probe's own pinned passage from its own document, 172 case(s)
negative proof: PASS - 17 mutations across 11 defect classes, plus every probe
individually, each observed failing

$ python sim/tools/protocol_hashes.py               -> exit 0
$ python sim/tools/genesis_chain_id.py              -> exit 0
$ python sim/tools/consensus_parameters_closure.py  -> exit 0
$ python sim/tools/threat_model_matrix_coherence.py -> exit 0
$ python sim/tools/non_consensus_containment.py     -> exit 0
$ python sim/tools/lead_claims_check.py             -> exit 0
$ python sim/tools/reward_rules.py                  -> exit 0

=== GATE-TWO-ORACLES: la seconda derivazione di AUTH-0 ===
$ python sim/tools/auth0_oracle.py
facts read from the fixture prose (not from the table):
  valid_from_height   = 5
  revoked identity    = cblx1revokedfixture
  included at height  = 20
  effective_height    = 50  (read, never used in a verdict)

verdicts derived from clauses 1 and 2, compared with the 9 table rows:
  ok    h=4   cblx1revokedfixture      table=invalid rule=invalid
  ok    h=5   cblx1revokedfixture      table=valid   rule=valid
  ok    h=19  cblx1revokedfixture      table=valid   rule=valid
  ok    h=20  cblx1revokedfixture      table=invalid rule=invalid
  ok    h=21  cblx1revokedfixture      table=invalid rule=invalid
  ok    h=49  cblx1revokedfixture      table=invalid rule=invalid
  ok    h=50  cblx1revokedfixture      table=invalid rule=invalid
  ok    h=51  cblx1revokedfixture      table=invalid rule=invalid
  ok    h=51  cblx1ci6q36gqm6u3spknxzr table=valid   rule=valid

flip heights over 0..60, by exhaustion and not by sampling: [5, 20]

AUTH-0 second derivation: PASS - 9 rows agree, boundaries [5, 20] found by
exhaustion, effective_height 50 is not a boundary
exit=0

$ python sim/tools/auth0_oracle.py --negative
=== mutation: clause 2 read as `<` instead of `at most`: the revocation would
    not bite at its own inclusion height ===
  FAIL row at h=20: table says invalid, the rule derives valid
  FAIL the verdict changes at [5, 21]; clauses 1 and 2 place the two changes
       at [5, 20]
exit=1 (must be non-zero)

=== mutation: clause 2 anchored to `effective_height` instead of the inclusion
    height: the reading ADR-017 part 1 replaced ===
  FAIL row at h=20 (cblx1revokedfixture, ledger.md:256): table says invalid,
       the rule derives valid
  FAIL row at h=21 (cblx1revokedfixture, ledger.md:257): table says invalid,
       the rule derives valid
  FAIL row at h=49 (cblx1revokedfixture, ledger.md:258): table says invalid,
       the rule derives valid
  FAIL the verdict changes at [5, 50]; clauses 1 and 2 place the two changes
       at [5, 20]
  FAIL `effective_height` 50 is a boundary of the verdict, which is the
       previous reading and not part 1 of ADR-017
exit=1 (must be non-zero)

negative proof: PASS - 2 mutations, each observed failing
exit=0

=== GATE-NEGATIVE-PROOF: cinque mutazioni sull'albero, ognuna osservata fallire ===
Perimetro: `cargo test --workspace --all-features --no-fail-fast`, intero
workspace, conteggi sommati sulle righe `test result:`. Ogni file e' ripristinato
da una copia presa prima della mutazione, mai con `git checkout`, perche'
l'albero porta modifiche non committate.

M1  rimossa da `check_magnitudes` la riga
    `revocation_effective_grace_blocks >= revocation_effective_grace_blocks_min`
    -> 194 passed, 1 failed
    failures: the_grace_floor_is_taken_from_genesis_and_not_from_the_document

M2  rimossa da `check_magnitudes` la chiamata
    `bounds.check_revocation_grace_floor()?`, cioe' la relazione fra costanti di
    genesi non e' piu' riverificata dove i bounds sono consumati
    -> 194 passed, 1 failed
    failures: the_grace_floor_is_taken_from_genesis_and_not_from_the_document

M3  i tre parametri di revoca tolti da `ELECTION_PARAMETERS` (13 -> 10)
    -> 194 passed, 1 failed
    failures: the_election_rate_of_change_binds_downward_on_every_parameter

M4  `validate_effective_height_in_block` smette di legare il documento a
    `consensus_parameters_hash` e legge il corpo direttamente
    -> 194 passed, 1 failed
    failures: clause_three_refuses_a_parameter_document_the_header_does_not_commit_to

M5  `min_by_key` rimesso a `find`
    -> 194 passed, 1 failed
    failures: the_reported_inclusion_height_is_the_earliest_and_not_the_first_in_the_slice

albero ripristinato -> 195 passed, 0 failed

Ogni mutazione fa fallire **esattamente un** test, e in tutti e cinque i casi
quello che la regola esiste per tenere.
```

#### Limiti noti di questa remediation

- **`GATE-CI-GREEN` non e' stata eseguita** e non e' mia: nessuna pipeline. Tutto
  quanto sopra e' locale, su un solo ambiente, Windows 11.
- **`sim/coblox_sim/params.py` non e' stato toccato**, ed e' fuori perimetro. E'
  il secondo verificatore del blocco dei vincoli in questo repository e la sua
  intestazione promette che *Â«every rule carries the exact text it
  transcribesÂ»*: non porta i tre parametri di revoca ne' alcuno dei vincoli che
  li riguardano, e il suo elenco del rapporto di variazione resta a dieci nomi
  mentre `ledger.md` ora ne dichiara tredici. Il difetto **precede** questa
  remediation. Riportato al Lead invece che corretto.
- **Il costo reale della rifirma sotto censura resta non misurato**, ed e' la
  stessa grandezza che [ADR-017] nomina nelle proprie condizioni di revisione.
  Il pavimento nuovo e' strutturale e non poggia su quella misura, ma non la
  sostituisce.
- **La composizione fra la banda e la regola di contrazione del set** non e'
  stata attaccata qui, come non lo era in [REVIEW-042].
- **Nessun valore di lancio** per `F`, `G`, `P`, ne' per
  `revocation_effective_grace_blocks_min`. Restano DRAFT e decisione
  dell'operatore.
