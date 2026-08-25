---
id: SPEC-011
# Note: Quote the title if it contains a colon
title: "RewardBounds e le regole di validita economiche in coblox-core"
status: done
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
verification_gates:
  - id: GATE-INVALID-REJECTED
    status: passed
  - id: GATE-DIRECTION
    status: passed
  - id: GATE-TWO-ORACLES
    status: passed
  - id: GATE-ADR012
    status: passed
  - id: GATE-SECREVIEW
    status: pending
related_decisions: [ADR-010, ADR-011, ADR-012]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [conformance, ledger, rust, sybil]
activity:
  - date: 2026-08-25
    action: "transitioned backlog -> ready"
  - date: 2026-08-25
    action: "transitioned ready -> working"
  - date: 2026-08-25
    action: "transitioned working -> review"
  - date: 2026-08-25
    action: "attested verification GATE-SECREVIEW by lead"
  - date: 2026-08-25
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "67ef65f1bd67274a26dd5c0ed43bb0948632fb69a5b8a6c12415e7bd44a878e2"
    evidence_ref: "REVIEW-017, accettata. AGENT-007 ha rivisto l'implementazione delle tre regole con una campagna di mutazione su copia isolata, verificando 19 regole di check_internal e check_magnitudes, tutti e 11 i confronti di direzione riga per riga contro README compresi i tre contro-intuitivi, e la regola relazionale rimossa e invertita. Verdetto changes-requested con tre finding medium, nessuno critical o high, tutti e tre chiusi in remediation e verificati dal Lead in modo indipendente. Il Lead ha riprodotto entrambe le guardie nuove in negativo: mutazione sul loop REWARD_PARAMETERS, test FAILED; mutazione sul loop ELECTION_PARAMETERS, test gemello FAILED. Albero ripristinato e verificato integro, 113 test passati, clippy zero warning, fmt pulito, quattro strumenti versionati OK, nessun hash pubblicato mosso."
    id: "SPEC-011-ATTEST-001"
    requirement_digest: "91a93686ea9c2e00e0ed7aa8e14440c00a9101b09f3e1ba46e711fb625c5b08e"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T20:53:31.332892900+02:00"
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

- [x] `RewardBounds` esiste come tipo, con la stessa disciplina di `ElectionBounds`: configurazione, mai stato di catena, limiti presi dai bound e mai dal documento in valutazione.
- [x] `availability_microtokens_per_unit` è letto e una tariffa positiva è **rifiutata**.
- [x] `3 * validator_min_set_size >= 2 * V` è nel blocco relazionale ed è rifiutata la violazione.
- [x] I limiti di magnitudine, il rapporto di variazione e il gap di attivazione della reward policy sono applicati.
- [x] **Ogni riga delle tabelle di frontiera pubblicate** ha un caso di prova con il proprio verdetto, e i casi `invalid` sono rifiutati.
- [x] Per ogni limite nuovo, un test dimostra il rifiuto **nella direzione giusta**: un caso che viola verso l'alto per i tetti e verso il basso per i pavimenti, e un caso che rispetta il limite al valore esatto.
- [x] L'aritmetica usa intermedi verificati e l'overflow rifiuta invece di troncare, con un caso che lo dimostra.
- [x] `coblox-core` e `sim/tools/reward_rules.py` **concordano su ogni caso pubblicato**, e la trascrizione mostra entrambe le esecuzioni.
- [x] `sim/coblox_sim/recommended.py` non porta più un fondo di genesi dimensionato sulla rete matura; il valore è 300 000 000 µt e la ragione è scritta accanto.
- [x] Nessun hash pubblicato cambia. Se qualcosa lo facesse, è un finding da riportare prima di procedere.
- [x] La gate di [ADR-012] è eseguita con lo strumento di [SPEC-010], e la trascrizione è allegata.

## Implementation plan

1. Leggere le tabelle di frontiera pubblicate ed enumerare i casi con il loro verdetto, **prima** di scrivere codice. Sono la specifica.
2. Introdurre `RewardBounds` seguendo `ElectionBounds`, con la stessa separazione fra bound e documento.
3. Applicare le tre regole, ciascuna con il proprio errore distinto.
4. Scrivere i casi di frontiera e verificarli contro `reward_rules.py`.
5. Correggere `recommended.py` e verificare la coerenza fra i due artefatti del simulatore.
6. Eseguire la gate di [ADR-012] con lo strumento di [SPEC-010].

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-INVALID-REJECTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | **Ogni** caso dichiarato `invalid` nelle tabelle pubblicate è rifiutato, e la trascrizione lo mostra caso per caso. Una suite di soli casi validi la passa anche un validatore che accetta tutto: è la ragione per cui questa gate esiste e non è sostituibile da un conteggio di test verdi.
- [x] GATE-DIRECTION | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Per ogni limite nuovo, la trascrizione mostra il rifiuto **nella direzione del pericolo** e l'accettazione al valore esatto del limite. Un limite implementato al contrario passa tutti i test positivi, ed è il punto che [REVIEW-014] indica come facile e invisibile.
- [x] GATE-TWO-ORACLES | kind=manual | owner=agent | phase=before-submit | evidence=transcript | `coblox-core` e `sim/tools/reward_rules.py` producono lo stesso verdetto su ogni caso pubblicato. Due implementazioni indipendenti che concordano su una tabella pubblicata sono un'evidenza che una sola non dà.
- [x] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato di [SPEC-010] e la trascrizione è allegata, **anche se non trova nulla**. [ADR-012] lo dice esplicitamente: una passata che non trova nulla è il caso previsto e non è evidenza che la gate sia inutile.
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto l'implementazione delle regole e il Lead ha accettato la review. Le tre regole nascono da suoi finding critici, e chiuderle senza la sua verifica sarebbe incoerente con il modo in cui sono state aperte.

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
1. **`RewardBounds` struct & validation (`core/coblox-core/src/params.rs`):**
   - Implemented 17-field `RewardBounds` configuration struct.
   - Added `RewardBounds::validate(&self, configured_chain_id: &ChainId) -> Result<()>` checking chain identity match, positive change denominator, numerator > denominator, positive activation gap, positive minimum epoch, and minimum epoch <= maximum epoch.
2. **`RewardPolicy` & `ValidatedRewardPolicy` (`core/coblox-core/src/params.rs`):**
   - Expanded `RewardPolicy` (with `pub type RewardPolicyConstraints = RewardPolicy` for backward compatibility) to carry all 13 canonical fields parsed via `from_body` rejecting unknown keys.
   - Implemented `check_internal()` enforcing Rule 1 (`availability_microtokens_per_unit == 0`), creator-share cap denominator non-zero and numerator strictly less than denominator, non-zero divisors, window >= 1, and minimum issuers >= 2.
   - Implemented `check_magnitudes(&self, bounds: &RewardBounds)` enforcing all genesis magnitude floors and ceilings.
   - Implemented `check_against_active(...)` enforcing monotonically increasing sequence numbers, rate-of-change ratio (`new * den <= old * num` and `old * den <= new * num`) over all 13 parameters using checked `u128` products, and minimum activation gap in chain height.
   - Implemented `ValidatedRewardPolicy` wrapping `RewardPolicy`, returned only upon passing `RewardPolicy::validate`.
3. **Consensus Relational Rule (`core/coblox-core/src/params.rs`):**
   - Added `3 * validator_min_set_size >= 2 * V` relational check in `ConsensusParameters::check_relations` with checked `u128` multiplication.
4. **Simulator Alignment (`sim/coblox_sim/recommended.py`):**
   - Corrected `REWARD.existence_fund_microtokens_per_epoch` in `sim/coblox_sim/recommended.py` to `300_000_000` (µt) for the ~200 expected launch nodes population per ADR-011.
5. **Comprehensive Verification & Oracles:**
   - Implemented tests for all published boundary cases from `README.md` §*Reward bounds* (22 cases), rate-of-change (5 cases), and `ledger.md` (7 cases for `3 * min_set >= 2 * V`).
   - Implemented directional testing (`GATE-DIRECTION`) for all 13 economic limits and the 3 relational/rate-of-change rules.
   - Implemented checked arithmetic overflow tests demonstrating `Error::Arithmetic` is returned instead of panicking or truncating.
   - Executed ADR-012 published artifact verification tools (`protocol_hashes.py`, `published_artifacts.py`, `published_artifacts_negative.py`, `reward_rules.py`).

### Files changed
- `core/coblox-core/src/params.rs`
- `core/coblox-core/src/lib.rs`
- `core/coblox-core/tests/common/mod.rs`
- `core/coblox-core/tests/constraint_block.rs`
- `sim/coblox_sim/recommended.py`
- `sim/coblox_sim/scenarios.py`
- `sim/tests/test_simulator.py`

### Verification performed
- `cargo test --all-targets` (all 109 tests passed)
- `cargo clippy --all-targets -- -D warnings` (clean)
- `cargo fmt --check` (clean)
- `python sim/tools/reward_rules.py` (34/34 cases passing, 0 mismatches)
- `python sim/tools/protocol_hashes.py` (all published hashes reproduced)
- `python sim/tools/published_artifacts.py` (all C1-C10 invariant checks passing)
- `python sim/tools/published_artifacts_negative.py` (10/10 defect classes verified failing in negative proof)
- `$env:PYTHONPATH="sim"; pytest sim` (all 44 simulator unit tests passing)

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

```text
=== CARGO TEST --ALL-TARGETS ===
running 26 tests in src/lib.rs ... ok. 26 passed; 0 failed
running 5 tests in tests/canonical_serialization.rs ... ok. 5 passed; 0 failed
running 23 tests in tests/conformance_registry.rs ... ok. 23 passed; 0 failed
running 17 tests in tests/constraint_block.rs ... ok. 17 passed; 0 failed
running 12 tests in tests/election_degenerate.rs ... ok. 12 passed; 0 failed
running 12 tests in tests/light_client_perimeter.rs ... ok. 12 passed; 0 failed
running 8 tests in tests/sparse_account_state.rs ... ok. 8 passed; 0 failed
running 6 tests in tests/worked_example.rs ... ok. 6 passed; 0 failed
test result: ok. 109 passed; 0 failed; 0 ignored; 0 measured

=== CARGO CLIPPY --ALL-TARGETS ===
    Checking coblox-core v0.1.0 (E:\Git\CobloxNetwork\core\coblox-core)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s

=== REWARD RULES PYTHON ORACLE (sim/tools/reward_rules.py) ===
Rules 1 and 2 - reward_policy acceptance against RewardBounds
  case                                        expected       got  reason
  availability tariff 0                          valid     valid  accepted
  availability tariff 1                        INVALID   INVALID  availability tariff must be zero
  availability tariff 1000                     INVALID   INVALID  availability tariff must be zero
  creator cap 1/2                                valid     valid  accepted
  creator cap 2/2                              INVALID   INVALID  creator-share cap not strictly lossy
  creator cap 1/0                              INVALID   INVALID  creator-share cap not strictly lossy
  F exactly at the ceiling                       valid     valid  accepted
  F one above the ceiling                      INVALID   INVALID  above the existence fund ceiling
  epoch exactly at the floor                     valid     valid  accepted
  epoch one below the floor                    INVALID   INVALID  epoch below the floor inflates real issuance
  epoch of 86 400 ms (the x1000 attack)        INVALID   INVALID  epoch below the floor inflates real issuance
  epoch one above the ceiling                  INVALID   INVALID  epoch above the ceiling freezes issuance
  storage divisor at the ceiling                 valid     valid  accepted
  storage divisor x 10^6                       INVALID   INVALID  redenominates the eligibility unit
  compute divisor above the ceiling            INVALID   INVALID  redenominates the eligibility unit
  window at the ceiling                          valid     valid  accepted
  window of 3000 epochs                        INVALID   INVALID  window above the ceiling drives the required rate toward zero
  storage tariff at the floor                    valid     valid  accepted
  storage tariff zero                          INVALID   INVALID  empties the denominator of the surveilled ratio
  compute tariff zero                          INVALID   INVALID  empties the denominator of the surveilled ratio
  threshold at the floor                         valid     valid  accepted
  threshold below the floor                    INVALID   INVALID  eligibility threshold below the floor

Rule 3 - rate of change and activation spacing
  F at exactly 5/4                               valid     valid  accepted
  F one above 5/4                              INVALID   INVALID  rate of change exceeded on existence_fund_microtokens_per_epoch
  epoch 86 400 000 -> 86 400 in one document   INVALID   INVALID  epoch below the floor inflates real issuance
  activation exactly at the gap                  valid     valid  accepted
  activation one block short                   INVALID   INVALID  activation gap not respected

Relational rule on consensus_parameters - 3 * min_set >= 2 * V
  V=12   min_set=8    3*8=24    vs 2*12=24        valid     valid
  V=12   min_set=7    3*7=21    vs 2*12=24      INVALID   INVALID
  V=12   min_set=1    3*1=3     vs 2*12=24      INVALID   INVALID
  V=27   min_set=18   3*18=54    vs 2*27=54        valid     valid
  V=27   min_set=17   3*17=51    vs 2*27=54      INVALID   INVALID
  V=36   min_set=24   3*24=72    vs 2*36=72        valid     valid
  V=36   min_set=18   3*18=54    vs 2*36=72      INVALID   INVALID

cases: 34, mismatches: 0
GATE-RULES-REJECT: PASS

=== PROTOCOL HASHES (sim/tools/protocol_hashes.py) ===
Governed protocol documents. None of the four changed in this pass,
so all four are method validation:
  enrollment_parameters        MATCH
    published sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
    computed  sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
  hosting_rate_card            MATCH
    published sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8
    computed  sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8
  consensus_parameters         MATCH
    published sha256:628c66f9ca8ac1a3161a0159201f7b6c6bf4c7500b390bc89b9b65a6c50ccbe9
    computed  sha256:628c66f9ca8ac1a3161a0159201f7b6c6bf4c7500b390bc89b9b65a6c50ccbe9
  reward_policy                MATCH
    published sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48
    computed  sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48

The reward fixture with the pre-[ADR-010] availability tariff, for
comparison - this is the shape the validity rule of [ADR-010] forbids:
    availability=1 -> sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d

Tagged trees. Method validation on the two values published before
this pass and untouched by it:
  empty revocation_root H(0x33) MATCH
    published sha256:4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce
    computed  sha256:4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce
  revocation_leaf REVL-0       MATCH
    published sha256:7fb1f4024627c413cbf70b49a390b6d31778e667e86042864c4bed107cd52497
    computed  sha256:7fb1f4024627c413cbf70b49a390b6d31778e667e86042864c4bed107cd52497

The fixture this pass added, computed with that validated method
from the bytes the document now carries:
  account_key (app) APP-0      MATCH
    published sha256:a881e2e0907aa86b225aaa2a2e1898afda1ce4733bd6d9cb390475ded4737e9d
    computed  sha256:a881e2e0907aa86b225aaa2a2e1898afda1ce4733bd6d9cb390475ded4737e9d
  app_leaf APP-0               MATCH
    published sha256:2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515697
    computed  sha256:2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515697

The same leaf under the encodings the document does NOT use, so the
choice is visible as a choice and not as an accident:
    reserved 0x00 (invalid)                    sha256:562c066031560a5d6993ea7e911cb2124904768085f216c2db08e50e3a927c91
    provisional pre-[DEBT-012] suspended = 2   sha256:3f6992a4031a5bd162e5697f4dd2e7e8bb7f2d97db7fbf0dac3b2f4561c7983e

every published value reproduced: PASS

=== PUBLISHED ARTIFACTS INVENTORY (sim/tools/published_artifacts.py) ===
  C1-DOMAIN         39 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     15 candidate(s) checked
  C4-VALUE          51 candidate(s) checked
  C5-MIRROR         42 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C10-PROBE         11 candidate(s) checked

published-artifact inventory: PASS

=== PUBLISHED ARTIFACTS NEGATIVE PROOF (sim/tools/published_artifacts_negative.py) ===
=== control: the unmutated copy ===
published-artifact inventory: PASS
=== C1-DOMAIN ===
  exit=1 names C1-DOMAIN: True
=== C2-TAG ===
  exit=1 names C2-TAG: True
=== C3-FIXTURE-ID ===
  exit=1 names C3-FIXTURE-ID: True
=== C4-VALUE ===
  exit=1 names C4-VALUE: True
=== C5-MIRROR ===
  exit=1 names C5-MIRROR: True
=== C6-ORPHAN ===
  exit=1 names C6-ORPHAN: True
=== C7-COVERAGE ===
  exit=1 names C7-COVERAGE: True
=== C8-ENCODING ===
  exit=1 names C8-ENCODING: True
=== C9-EXAMPLE ===
  exit=1 names C9-EXAMPLE: True
=== C10-PROBE ===
  exit=1 names C10-PROBE: True
negative proof: PASS - 10 defect classes, each observed failing

=== SIM PYTEST (pytest sim) ===
============================= test session starts =============================
platform win32 -- Python 3.11.9, pytest-9.0.3, pluggy-1.6.0
collected 44 items
sim\tests\test_simulator.py ............................................ [100%]
============================= 44 passed in 0.23s ==============================
```

### Remediation of [REVIEW-017] — RF-001, RF-002, RF-003

La spec e rimasta in `review` per tutta la remediation. Nessun `spec_start`, nessun commit.

#### RF-001 — la regola era giusta e non era ricucita

La diagnosi del reviewer e riprodotta e accettata senza riserve: `RewardBounds::validate` non era
chiamata da nessuna riga di `src/`, quindi un oggetto bound degenere rendeva vacuo il limite di
variazione **senza errore**. Tre interventi, il primo strutturale e gli altri due nella stessa
famiglia che il finding chiedeva di chiudere insieme:

1. **`light_client::authenticate_reward_policy`** — il gemello di `authenticate_consensus_parameters`
   sul lato reward. Compone, nello stesso ordine: `bounds.validate(chain_id)`, il ricalcolo del
   `policy_hash` del documento e il confronto con il digest atteso, il controllo di `chain_id`
   dichiarato, `RewardPolicy::from_body`, `RewardPolicy::validate`.

   Una differenza rispetto al gemello, ed e della specifica e non una scorciatoia:
   `consensus_parameters_hash` e un campo di `BlockHeader`, quindi il gemello lo legge dall'header
   fidato. Per il `reward_policy` **non esiste alcun campo d'intestazione**: il documento e
   referenziato dal `policy_hash` che le transazioni `mint` firmate portano (`ledger.md#mint`).
   Il digest atteso e quindi un input del chiamante, che lo prende dall'oggetto firmato che nomina la
   policy — mai dal documento in valutazione. Ho verificato l'assenza del campo con
   `grep -rn "reward_policy_hash"` su `docs/`, `core/` e `sim/`: zero occorrenze.

2. **Un secondo presidio dentro la regola stessa.** `check_against_active` rifiuta ora esplicitamente
   un rapporto degenere (`denominator == 0`, oppure `numerator <= denominator`) e un gap di
   attivazione nullo, con `ParameterError::Bounds` e la regola nominata. L'entry point e la difesa
   strutturale; questo e il motivo per cui **la regola non puo diventare vacua su nessun percorso**,
   nemmeno per un chiamante che arriva a `RewardPolicy::validate` direttamente. E la differenza fra
   "non succede oggi perche nessuno chiama cosi" e "non puo succedere".

3. **`publisher_reward_within_cap` e ora un metodo su `ValidatedRewardPolicy`**, non su
   `RewardPolicy`. Il reviewer osservava che l'unico calcolo di ricompensa del crate girava su una
   policy mai validata mentre i suoi omologhi del lato consenso sono metodi su
   `ValidatedConsensusParameters`, e che `ValidatedRewardPolicy` non aveva consumatori. Ora ne ha uno,
   e l'affermazione di `lib.rs` e vera per il codice che esiste. **Questo e un cambiamento breaking
   dell'API pubblica del core**, dichiarato qui come il profilo AGENT-001 richiede: il tipo del
   ricevitore cambia da `RewardPolicy` a `ValidatedRewardPolicy`. Nessun consumatore fuori dai test
   esisteva, e il crate non e pubblicato.

4. **`lib.rs`** — il commento di modulo diceva che «the election derivation and reward validation
   accept only `ValidatedConsensusParameters` and `ValidatedRewardPolicy`», che era vero per il lato
   elezione e falso per il lato reward. Riscritto su cio che il codice fa, con un paragrafo nuovo sul
   fatto che un'ancora di fiducia e controllata prima di essere creduta.

#### RF-002 — la meta discendente non era vincolata da nessuno dei due oracoli

Il finding e accettato e riprodotto. La correzione **non** e un caso in piu preso dalla tabella
pubblicata: il reviewer avverte esplicitamente che due oracoli derivati dalla stessa tabella
concordano anche dove sono ciechi, quindi le due implementazioni derivano i casi nuovi **in due modi
diversi**, e nessuno dei due modi legge la tabella.

- **Rust** (`the_rate_of_change_binds_in_both_directions_on_every_parameter`): i casi sono derivati
  dalla struttura della regola. Una passata sui dodici parametri che il rapporto puo vincolare
  (tredici meno `availability_microtokens_per_unit`, fissato a zero da [ADR-010]: `0 -> 0` soddisfa
  entrambe le disuguaglianze e qualunque altro valore cade prima, su `check_internal`). Per ciascuno,
  i due estremi calcolati in forma chiusa — `old * 5 / 4` verso l'alto e `old * 4 / 5` verso il basso
  — piu un passo oltre ciascuno. **48 asserzioni.**
- **Python** (`rate_sweep()` in `sim/tools/reward_rules.py`): il punto di rottura non e calcolato ma
  **cercato**. La passata chiede al predicato stesso dove smette di accettare, per ricerca binaria, in
  ciascuna direzione, e poi verifica che la risposta sia quella che il rapporto implica e che il
  rifiuto un passo oltre **nomini la regola di variazione**. Se una direzione non e vincolata affatto,
  la ricerca corre fino in fondo all'intervallo e il caso fallisce — che e esattamente cio che prima
  non accadeva. **24 casi nuovi**, da 34 a 58.

Due dettagli deliberati, entrambi diretti al difetto che il reviewer ha misurato:

- I bound di entrambe le passate sono **larghi su ogni magnitudine**. Un rifiuto in questa passata
  deve venire dal rapporto e mai da un pavimento o da un tetto. E il difetto esatto della riga
  pubblicata `reward_epoch_ms | 86 400 000 -> 86 400`, che la tabella dichiara rifiutata dal rapporto
  e che l'implementazione rifiuta dal pavimento.
- Ogni rifiuto asserisce `ChangeRatio { parameter }` **nominando il parametro che si e mosso**, non
  `is_err()`. Un caso che rifiuta per il motivo sbagliato non passa piu.

E aggiunto lo scenario d'attacco concreto del finding: la tariffa storage tagliata a un decimo in un
solo documento, con ogni pavimento di magnitudine rispettato, rifiutata nominando
`storage_microtokens_per_byte_epoch`.

#### RF-003 — un artefatto versionato ed eseguibile che dichiara assenti le difese esistenti

Corretto `sim/coblox_sim/__main__.py`. Le quattro affermazioni che il reviewer elenca sono riscritte
al presente su cio che il protocollo applica oggi:

- la tariffa di availability e ora una **regola**, non un valore;
- `F` ha un tetto di genesi, e il salto «da 15 882 cr a 2^60 microtokens in un documento lecito» non
  e piu lecito;
- la disciplina 5/4 e ora una **regola**, estesa a tutti e tredici i parametri e bidirezionale;
- `RewardBounds` non e piu «una modifica da decidere e la ADR del Lead da aprire».

Cio che il rapporto continua a dichiarare non-regola e **il contenuto di genesi di `RewardBounds`
stesso**, che e configurazione e che nessuna regola on-chain vincola — con la nota che il crate
valida l'ancora prima di fidarsene. E il residuo onesto, ed e anche cio che RF-004 chiede al Lead di
assegnare.

Una quinta affermazione non elencata nel finding, trovata guardando lo stesso file: riga 254,
«F ... a governed value with no ceiling and no rate limit that any rule imposes». Falsa per le stesse
due ragioni. Corretta.

Il documento del brain (`.lmbrain/knowledge/economic-simulation-report.md`) porta la stessa forma
condizionale alle righe che il reviewer indica. Il reviewer scrive che «quella parte e del Lead e non
del riparatore»: **non l'ho toccato**.

#### RF-002-BIS — il gemello sul lato elezione, chiesto dal Lead

Il Lead ha trovato la stessa lacuna sul loop `ELECTION_PARAMETERS` mentre riproduceva RF-002:
disattivando `old_bounded` a `params.rs:416` la suite restava interamente verde. Non e codice di
questa consegna — viene da [SPEC-006] e [SPEC-008] — ma e lo stesso difetto, e i dieci parametri
dell'elezione ne erano privi. `the_rules_that_compare_against_the_active_document` esercita il
rapporto solo verso l'alto (12 -> 24), piu il gap, il cricchetto sul limite di mandato e la sequenza.

Nuovo test `the_election_rate_of_change_binds_downward_on_every_parameter`, con la stessa disciplina
adottata sul lato reward: bound larghi su ogni magnitudine, e `ChangeRatio { parameter }` asserito
**nominando il parametro** invece di `is_err()`. **18 righe** — nove parametri per due direzioni — piu
il decimo trattato a parte.

**La trappola che il Lead chiedeva di verificare esiste anche qui, ed e peggiore che sul lato reward.**
Sul lato reward il rifiuto anticipato arriva da un pavimento di magnitudine; qui arriva dal **blocco
relazionale**, che `validate` esegue *per primo*, prima ancora delle magnitudini. Un caso discendente
scritto su una base le cui relazioni sono tese nella direzione in prova viene rifiutato dalla
relazione, il verdetto coincide, e la regola che il caso doveva coprire non viene mai raggiunta — la
forma esatta di RF-002. Ho quindi scelto ogni base perche il blocco relazionale regga **al confine e
un passo oltre**, e ho messo la trappola *in evidenza* invece di dichiararla assente: l'ultima
asserzione del test mostra che la stessa discesa di `validator_target_set_size` che sulla base giusta
e rifiutata dal rapporto, sulla base sbagliata e rifiutata da
`0 < validator_min_set_size <= V <= validator_max_set_size`, un livello prima.

**Due informazioni, che riporto invece di forzare** — il Lead ha chiesto esattamente questo.

1. **`validator_min_set_size` e `validator_target_set_size` non possono essere spazzati in entrambe le
   direzioni dalla stessa base.** La regola `3 * validator_min_set_size >= 2 * V` li accoppia: perche
   `min_set` possa scendere serve `min_set >= (5/6) V`, perche possa salire serve `min_set <= (4/5) V`,
   e i due intervalli non si intersecano. Specularmente per `V`. Non e un limite del test: e una
   proprieta del blocco relazionale. Ho quindi due basi, identiche salvo `min_set` (20 000 e 14 000), e
   quale base serve quale direzione e dettato da quella regola, non da comodita.
2. **`validator_max_consecutive_terms` non ha alcun documento discendente lecito.** «On a live chain
   the term limit never decreases»: ogni discesa e rifiutata. Cio che vale la pena fissare non e *se*
   ma *quale regola* rifiuta, perche il ciclo del rapporto gira **prima** del cricchetto. Al confine
   discendente (1 000 -> 800) il rapporto e soddisfatto e a rifiutare e `TermLimitDecreased`; un passo
   piu giu (799) rifiuta il rapporto per primo, nominando il parametro. Entrambe le cose sono asserite.
   Verso l'alto il parametro si comporta come gli altri nove.

L'oracolo Python **non copre il rapporto di variazione sul lato elezione** — `accept_consensus_min_set`
implementa la sola regola relazionale `3 * min_set >= 2 * V`. Non c'e quindi un secondo oracolo da cui
derivare i casi in modo indipendente, e non ne ho scritto uno: sarebbe stato fuori dal perimetro di
questa spec. Lo dico esplicitamente perche l'avvertenza del Lead sulla derivazione indipendente **non
si applica** qui, e perche l'assenza e essa stessa un'informazione per `GATE-TWO-ORACLES`.

### Verification transcript — remediation di [REVIEW-017]

Trascrizioni reali delle riesecuzioni, comprese le due che mostrano le guardie nuove **fallire** sul
difetto che chiudono.

```text
=== 1. LA GUARDIA DI RF-002 FALLISCE SUL DIFETTO CHE CHIUDE ===
Mutazione del reviewer, riapplicata alla lettera: in params.rs la condizione del
rapporto ridotta alla sola meta superiore (`if !new_bounded {`), e in
sim/tools/reward_rules.py `if not (new * den <= old * num):`.

$ cargo test --test constraint_block the_rate_of_change_binds
running 1 test
test the_rate_of_change_binds_in_both_directions_on_every_parameter ... FAILED

---- the_rate_of_change_binds_in_both_directions_on_every_parameter stdout ----
thread '...' panicked at core\coblox-core\tests\constraint_block.rs:1389:44:
called `Result::unwrap_err()` on an `Ok` value: ValidatedRewardPolicy(RewardPolicy {
  reward_epoch_ms: 3999, existence_fund_microtokens_per_epoch: 5000, ... })

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 17 filtered out

$ python sim/tools/reward_rules.py
  reward_epoch_ms upward                                 BOUND  stops accepting above 6,250 (rate of change exceeded on reward_epoch_ms)
  reward_epoch_ms downward                             UNBOUND  stops accepting below 1 (epoch below the floor inflates real issuance)   <-- MISMATCH
  existence_fund_microtokens_per_epoch downward        UNBOUND  stops accepting below 0 (accepted)   <-- MISMATCH
  storage_microtokens_per_byte_epoch downward          UNBOUND  stops accepting below 1 (empties the denominator of the surveilled ratio)   <-- MISMATCH
  compute_microtokens_per_million_fuel downward        UNBOUND  stops accepting below 1 (empties the denominator of the surveilled ratio)   <-- MISMATCH
  publisher_microtokens_per_active_subscriber downward   UNBOUND  stops accepting below 0 (accepted)   <-- MISMATCH
  publisher_reward_cap_numerator downward              UNBOUND  stops accepting below 0 (accepted)   <-- MISMATCH
  publisher_reward_cap_denominator downward            UNBOUND  stops accepting below 5,001 (creator-share cap not strictly lossy)   <-- MISMATCH
  storage_units_per_contribution_unit downward         UNBOUND  stops accepting below 0 (accepted)   <-- MISMATCH
  compute_units_per_contribution_unit downward         UNBOUND  stops accepting below 0 (accepted)   <-- MISMATCH
  validator_eligibility_threshold_units downward       UNBOUND  stops accepting below 1 (eligibility threshold below the floor)   <-- MISMATCH
  validator_eligibility_window_epochs downward         UNBOUND  stops accepting below 0 (accepted)   <-- MISMATCH
  validator_eligibility_min_issuers downward           UNBOUND  stops accepting below 2 (issuer diversity below the floor)   <-- MISMATCH

cases: 58, mismatches: 12
GATE-RULES-REJECT: FAIL
$ echo $?
1

Nota di lettura: la colonna del motivo dice PERCHE il buco era invisibile. Dove il
mutante sopravvive, chi rifiuta non e il rapporto ma un pavimento — «epoch below the
floor», «empties the denominator», «eligibility threshold below the floor» — o nessuno
(«accepted», con il parametro portato a zero). E la diagnosi di RF-002 riprodotta dallo
strumento invece che argomentata.


=== 2. LA GUARDIA DI RF-001 FALLISCE SUL DIFETTO CHE CHIUDE ===
Mutazione: rimossa `bounds.validate(chain_id)?` da authenticate_reward_policy e
rimosso il presidio sul rapporto degenere da check_against_active — cioe il crate
riportato allo stato che RF-001 descrive.

$ cargo test --test light_client_perimeter the_reward_entry_point
running 1 test
test the_reward_entry_point_validates_the_bounds_before_it_trusts_them ... FAILED

---- the_reward_entry_point_validates_the_bounds_before_it_trusts_them stdout ----
thread '...' panicked at core\coblox-core\tests\light_client_perimeter.rs:379:14:
called `Result::unwrap_err()` on an `Ok` value: ValidatedRewardPolicy(RewardPolicy {
  reward_epoch_ms: 1, existence_fund_microtokens_per_epoch: 1, ... })

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out


=== 2-BIS. LA GUARDIA DEL GEMELLO FALLISCE SUL DIFETTO REINTRODOTTO ===
Mutazione del Lead, riapplicata sul loop ELECTION_PARAMETERS di params.rs:416:
`if !new_bounded {`, cioe la meta discendente disattivata sul lato elezione.

$ cargo test --test constraint_block the_election_rate_of_change
running 1 test
test the_election_rate_of_change_binds_downward_on_every_parameter ... FAILED

---- the_election_rate_of_change_binds_downward_on_every_parameter stdout ----
thread '...' panicked at core\coblox-core\tests\constraint_block.rs:1511:18:
called `Result::unwrap_err()` on an `Ok` value: ValidatedConsensusParameters(
  ConsensusParameters { ... election_epoch_blocks: 7999, candidacy_close_blocks: 5000,
  election_entropy_blocks: 500, validator_min_set_size: 20000, ... })

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out

E la mutazione opposta, per mostrare che il gemello non ha barattato una meta
con l'altra — `if !old_bounded {`, cioe la meta ascendente disattivata:

$ cargo test --test constraint_block
test the_election_rate_of_change_binds_downward_on_every_parameter ... FAILED
test the_rules_that_compare_against_the_active_document ... FAILED
test result: FAILED. 17 passed; 2 failed; 0 ignored; 0 measured

Verso l'alto cadono due test: il gemello nuovo e quello preesistente, che la
meta ascendente gia vincolava. Verso il basso cade solo il gemello nuovo, e
prima della sua esistenza non cadeva nulla. E la misura del buco.


=== 3. ALBERO RIPRISTINATO — SUITE RUST ===
$ cargo test --all-targets
running 26 tests (src/lib.rs)                 test result: ok. 26 passed; 0 failed
running  5 tests (canonical_serialization.rs) test result: ok.  5 passed; 0 failed
running 23 tests (conformance_registry.rs)    test result: ok. 23 passed; 0 failed
running 19 tests (constraint_block.rs)        test result: ok. 19 passed; 0 failed
running 12 tests (election_degenerate.rs)     test result: ok. 12 passed; 0 failed
running 13 tests (light_client_perimeter.rs)  test result: ok. 13 passed; 0 failed
running  8 tests (sparse_account_state.rs)    test result: ok.  8 passed; 0 failed
running  6 tests (worked_example.rs)          test result: ok.  6 passed; 0 failed
running  1 test  (coblox-ffi)                 test result: ok.  1 passed; 0 failed
totale: 113 test, 0 falliti.  I 109 della consegna sono i primi otto binari
(26+5+23+17+12+12+8+6); questa remediation aggiunge due test a
constraint_block.rs (RF-002 e il gemello di elezione) e uno a
light_client_perimeter.rs (RF-001), quindi 112, piu l'unico test di
coblox-ffi che la trascrizione di consegna non aveva contato.

$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s)
(nessun errore, nessun warning)

$ cargo fmt --check
(nessuna differenza)


=== 4. ORACOLO PYTHON, PASSATA NUOVA IN ENTRAMBE LE DIREZIONI ===
$ python sim/tools/reward_rules.py
Rule 3 - both halves of the rate of change, per governed parameter
  (boundaries searched for, not transcribed; [REVIEW-017] RF-002)
  reward_epoch_ms upward                                 BOUND  stops accepting above 6,250 (rate of change exceeded on reward_epoch_ms)
  reward_epoch_ms downward                               BOUND  stops accepting below 4,000 (rate of change exceeded on reward_epoch_ms)
  existence_fund_microtokens_per_epoch upward            BOUND  stops accepting above 6,250 (rate of change exceeded on existence_fund_microtokens_per_epoch)
  existence_fund_microtokens_per_epoch downward          BOUND  stops accepting below 4,000 (rate of change exceeded on existence_fund_microtokens_per_epoch)
  storage_microtokens_per_byte_epoch upward              BOUND  stops accepting above 6,250 (rate of change exceeded on storage_microtokens_per_byte_epoch)
  storage_microtokens_per_byte_epoch downward            BOUND  stops accepting below 4,000 (rate of change exceeded on storage_microtokens_per_byte_epoch)
  compute_microtokens_per_million_fuel upward            BOUND  stops accepting above 6,250 (rate of change exceeded on compute_microtokens_per_million_fuel)
  compute_microtokens_per_million_fuel downward          BOUND  stops accepting below 4,000 (rate of change exceeded on compute_microtokens_per_million_fuel)
  publisher_microtokens_per_active_subscriber upward     BOUND  stops accepting above 6,250 (rate of change exceeded on publisher_microtokens_per_active_subscriber)
  publisher_microtokens_per_active_subscriber downward     BOUND  stops accepting below 4,000 (rate of change exceeded on publisher_microtokens_per_active_subscriber)
  publisher_reward_cap_numerator upward                  BOUND  stops accepting above 6,250 (rate of change exceeded on publisher_reward_cap_numerator)
  publisher_reward_cap_numerator downward                BOUND  stops accepting below 4,000 (rate of change exceeded on publisher_reward_cap_numerator)
  publisher_reward_cap_denominator upward                BOUND  stops accepting above 125,000 (rate of change exceeded on publisher_reward_cap_denominator)
  publisher_reward_cap_denominator downward              BOUND  stops accepting below 80,000 (rate of change exceeded on publisher_reward_cap_denominator)
  storage_units_per_contribution_unit upward             BOUND  stops accepting above 6,250 (rate of change exceeded on storage_units_per_contribution_unit)
  storage_units_per_contribution_unit downward           BOUND  stops accepting below 4,000 (rate of change exceeded on storage_units_per_contribution_unit)
  compute_units_per_contribution_unit upward             BOUND  stops accepting above 6,250 (rate of change exceeded on compute_units_per_contribution_unit)
  compute_units_per_contribution_unit downward           BOUND  stops accepting below 4,000 (rate of change exceeded on compute_units_per_contribution_unit)
  validator_eligibility_threshold_units upward           BOUND  stops accepting above 6,250 (rate of change exceeded on validator_eligibility_threshold_units)
  validator_eligibility_threshold_units downward         BOUND  stops accepting below 4,000 (rate of change exceeded on validator_eligibility_threshold_units)
  validator_eligibility_window_epochs upward             BOUND  stops accepting above 6,250 (rate of change exceeded on validator_eligibility_window_epochs)
  validator_eligibility_window_epochs downward           BOUND  stops accepting below 4,000 (rate of change exceeded on validator_eligibility_window_epochs)
  validator_eligibility_min_issuers upward               BOUND  stops accepting above 6,250 (rate of change exceeded on validator_eligibility_min_issuers)
  validator_eligibility_min_issuers downward             BOUND  stops accepting below 4,000 (rate of change exceeded on validator_eligibility_min_issuers)

cases: 58, mismatches: 0
GATE-RULES-REJECT: PASS

I due estremi sono trovati per ricerca, non trascritti: la colonna «stops accepting»
e la risposta del predicato, e il motivo fra parentesi e quello che il predicato da un
passo oltre. `publisher_reward_cap_denominator` parte da 100 000 invece che da 5 000
perche `kn < kd` deve valere in ogni punto della sua passata; i suoi estremi 125 000 e
80 000 sono gli stessi 5/4 e 4/5.


=== 5. GATE DI [ADR-012] E RESTO DEGLI STRUMENTI, RIESEGUITI ===
$ python sim/tools/protocol_hashes.py
  enrollment_parameters        MATCH
  hosting_rate_card            MATCH
  consensus_parameters         MATCH
  reward_policy                MATCH
  empty revocation_root H(0x33) MATCH
  revocation_leaf REVL-0       MATCH
  account_key (app) APP-0      MATCH
  app_leaf APP-0               MATCH
every published value reproduced: PASS

$ python sim/tools/published_artifacts.py
  C1-DOMAIN         39 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     15 candidate(s) checked
  C4-VALUE          51 candidate(s) checked
  C5-MIRROR         42 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C10-PROBE         11 candidate(s) checked
published-artifact inventory: PASS

$ python sim/tools/published_artifacts_negative.py
negative proof: PASS - 10 defect classes, each observed failing

$ PYTHONPATH=sim python -m pytest sim -q
44 passed in 0.19s

$ PYTHONPATH=sim python -m coblox_sim   # il rapporto economico, dopo RF-003
exit=0

Nessun hash pubblicato si e mosso: i quattro documenti governati e le quattro voci di
albero riproducono i valori pubblicati, come nella passata di consegna.
```

### File cambiati nella remediation

- `core/coblox-core/src/light_client.rs` — `authenticate_reward_policy` (nuovo).
- `core/coblox-core/src/params.rs` — presidio sul rapporto degenere in `check_against_active`;
  `publisher_reward_within_cap` spostato su `ValidatedRewardPolicy`.
- `core/coblox-core/src/lib.rs` — l'affermazione di modulo allineata al codice.
- `core/coblox-core/tests/light_client_perimeter.rs` — il test di RF-001.
- `core/coblox-core/tests/constraint_block.rs` — il test di RF-002 e il suo gemello sul lato
  elezione; il caso del creator-share cap passa ora da `validate`.
- `core/coblox-core/tests/common/mod.rs` — `reward_document_of`, per costruire un documento
  `reward_policy` da una policy arbitraria.
- `sim/tools/reward_rules.py` — la passata bidirezionale per ricerca.
- `sim/coblox_sim/__main__.py` — RF-003, piu la riga 254 non elencata nel finding.

### Deviations from the specification

Alla consegna: nessuna.

Dopo [REVIEW-017]:

- **Cambiamento breaking dell'API pubblica del core**, dichiarato come il profilo AGENT-001 richiede:
  `publisher_reward_within_cap` e ora un metodo su `ValidatedRewardPolicy` e non piu su
  `RewardPolicy`. Nessun consumatore esisteva fuori dai test.
- **API pubblica nuova**: `light_client::authenticate_reward_policy`.
- **Cosa non ho toccato, e perche.** **RF-004** e del Lead per esplicita assegnazione del reviewer e
  del mandato: nessun artefatto propone ancora i valori di lancio di `RewardBounds`, e non ne ho
  scritto uno. **OSS-001** (il commento sul perche il ramo d'errore dei prodotti `u128` e
  irraggiungibile) non e a mio carico per mandato e non l'ho aggiunto. **OSS-002** (il controllo
  ridondante su `publisher_reward_cap_denominator`) e lasciato dov'e: l'errore nominato vale piu del
  ramo risparmiato. **OSS-003** (`check_internal` e `check_magnitudes` sono `pub` mentre i gemelli del
  lato consenso sono privati) e reale e non l'ho chiuso: renderli privati e un secondo cambiamento
  breaking, tocca test che oggi li chiamano direttamente, e non e condizione di chiusura di questa
  review. Lo segnalo al Lead come candidato a un debito, non come lavoro svolto.
- **Nessun codice di produzione cambiato per il gemello di elezione.** La regola
  `ConsensusParameters::check_against_active` era gia corretta in entrambe le direzioni; mancava solo
  chi la vincolasse. L'aggiunta e interamente di test.
- **Nessuna contestazione.** Ho verificato i tre finding sul codice prima di ripararli e li ho trovati
  corretti tutti e tre, RF-002 in particolare: la meta discendente del rapporto non era esercitata da
  nessuna riga pubblicata, e l'unica riga che nomina una discesa e davvero rifiutata dal pavimento
  invece che dal rapporto. Il mandato invita a contestare quando serve; qui non serviva.

### Handoff status
- [x] Ready for Project Lead review
