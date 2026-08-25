---
id: SPEC-014
# Note: Quote the title if it contains a colon
title: "I due cambiamenti breaking dell'API di coblox-core, prima del primo chiamante"
status: done
kind: refactor
priority: high
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
capability_tier: sol
thinking_level: standard
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-003, ADR-010]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [rust, api, security]
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
    evidence_digest: "eea5e38357fd0a0a70047b32d784ca022c462a593ae2ea29786ee5eb7e80b777"
    evidence_ref: "REVIEW-023, accettata. AGENT-007 ha rivisto la forma della garanzia con verdetto changes-requested, un finding medium e due low, e ha invertito correttamente il ragionamento del finding che il Lead aveva registrato in REVIEW-022. Il medium: la via non-consensus era nominata ma non contenuta, perche registry e un pub mod, il costruttore era pub e il crate non aveva alcuna sezione features, quindi era raggiungibile da coblox-node, coblox-ffi e dalla shell Tauri in build di produzione. La prova era gia in albero e nessuno l'aveva letta come tale: la suite di conformita e un test di integrazione, cioe un crate esterno, e le sue otto chiamate dimostravano la raggiungibilita dall'esterno e non l'assenza di chiamanti di consenso.\n\nChiuso con entrambe le strade proposte, con ruoli distinti: una feature non-default come confine di compilazione portante, e uno strumento versionato come guardia d'albero eseguito dalla CI. Il Lead ha verificato scrivendo la sonda dentro coblox-node nella forma esatta dello scenario: la build di produzione fallisce con E0599, e la stessa sonda compila sotto cargo test --workspace, cioe il limite di feature unification che l'implementatore ha misurato e dichiarato invece di tacere. La prova in negativo della guardia copre quattro classi, fra cui un dipendente che abilita la feature per se con una riga in un manifesto, classe che nessuna delle due opzioni della review nominava.\n\n126 test passati, clippy zero warning con --all-features, fmt pulito, nessun valore pubblicato mosso, nessun comportamento del verificatore cambiato."
    id: "SPEC-014-ATTEST-001"
    requirement_digest: "b61865906dbbfa3ea419dd5e88dac839d26acc4430b6c540a737ca1152cec62a"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T23:57:17.441874500+02:00"
---
# I due cambiamenti breaking dell'API di coblox-core, prima del primo chiamante

## Objective

Chiudere [DEBT-016] e [DEBT-015] in una sola passata, perché sono **due cambiamenti breaking della stessa API pubblica** e farne due raddoppierebbe il costo per gli stessi consumatori.

Entrambi hanno la stessa forma: **una convenzione che il progetto dichiara e che il tipo non impone.** E entrambi hanno la stessa scadenza — **prima che esista un chiamante** — perché oggi non ne esiste alcuno e non costeranno mai meno di adesso.

## Context

Il primo è il più grave e riguarda la cucitura consensus-critical. `SignatureVerifier::verify` e `verify_consensus_ed25519` accettano `message: &[u8]`, mentre il contratto impone che quel valore sia la preimmagine integrale prodotta da `registry::signing_preimage` e **mai un suo digest**. `Digest32::as_bytes()` coercisce a una fetta, quindi un chiamante che passasse un digest **compilerebbe e passerebbe ogni test**, e il legame a dominio separato e a `chain_id` che la preimmagine porta decadrebbe in silenzio. Il verificatore continuerebbe a verificare qualcosa, e non ciò che si crede.

Il secondo è la stessa forma su un'altra superficie: i sotto-controlli della reward policy sono pubblici mentre i gemelli del lato consenso sono privati, quindi un chiamante può invocarne uno solo e ricevere un `Ok` che non significa ciò che sembra.

**La ragione per cui questa spec esiste ora e non dopo.** [DEBT-016] è stato aperto perché AGENT-001 si è fermato invece di forzare, avendo il Lead escluso `verifier.rs` dal perimetro della remediation di [SPEC-012]. La sua osservazione decisiva vale per entrambi i debiti: *un newtype introdotto nel solo `registry.rs` lascerebbe `message: &[u8]` sulla firma del verificatore e sembrerebbe la chiusura senza esserlo* — che è la forma di difetto che [SPEC-012] aveva già commesso una volta con `PUBLISHED_OUTCOMES`.

## Scope

### Included

- Un tipo distinto per la preimmagine di firma, imposto sulla firma di **entrambi** i punti d'ingresso.
- Il ritorno dei sotto-controlli della reward policy alla visibilità privata dei loro gemelli.
- La riscrittura dei chiamanti nei test sull'API pubblica, **senza perdita di copertura**.
- La correzione della locuzione «audited primitive crate» in `verifier.rs` (OSS-001 di [REVIEW-019]).

### Excluded

- **Qualunque modifica alla logica di verifica**, alle regole di validità o ai valori pubblicati. Questa spec cambia **forme di tipo e visibilità**, non comportamenti. Se un hash pubblicato si muovesse, è un finding da riportare prima di procedere, non da normalizzare.
- [DEBT-017] e [DEBT-018], che sono di AGENT-007 e riguardano una regola nuova e un documento di analisi.
- L'aggiunta di chiamanti al verificatore: questa spec prepara la cucitura, non la usa.

## Existing-project analysis

**Verificato dal Lead il 2026-08-25 leggendo i file**, non ricordato. Le due volte in cui il Lead ha scritto un'analisi dell'esistente a memoria, in questa sessione, è stato corretto dall'implementatore.

- `SignatureVerifier::verify` è a `lib.rs:142`, `verify_consensus_ed25519` a `verifier.rs:71`. **Entrambi** portano `message: &[u8]`.
- `registry::signing_preimage(domain, chain_id, payload)` restituisce `Vec<u8>` (`registry.rs:321`). È il solo produttore legittimo del valore che il verificatore deve ricevere.
- `RewardPolicy::check_internal` (`params.rs:720`) e `check_magnitudes` (`params.rs:767`) sono `pub`. I gemelli del lato consenso — `ConsensusParameters::check_relations` (335) e `check_magnitudes` (390) — sono **privati**. `check_against_active` è già privato su entrambi i lati.
- I chiamanti diretti nei test sono **tre**, tutti in `constraint_block.rs`: righe 540, 548, 580. Un quarto punto, riga 1638, è un commento che spiega perché un caso è escluso da una passata proprio a causa di `check_internal`: **va letto prima di toccare quel test**, perché descrive una precedenza fra controlli.
- La locuzione è a `verifier.rs:27`: «composes on the audited primitive crate». RF-005 di [REVIEW-019] ha stabilito che nessun audit della 5.x è citabile, e `Cargo.toml` lo dice già; la frase nel modulo non è stata allineata perché correggerla significava modificare `verifier.rs`, escluso da quella remediation.

## Technical proposal

### 1. Il tipo, e la tensione che ne governa la forma

Il tipo deve rendere **impossibile da compilare** il passaggio di un digest, e non deve essere costruibile da byte arbitrari senza passare per `signing_preimage`, altrimenti la garanzia è nominale.

**C'è però una tensione reale, ed è il punto in cui questa spec può fallire in silenzio.** La suite di conformità `ed25519-speccheck` verifica firme su **messaggi arbitrari** che non sono preimmagini Coblox: i vettori upstream portano un campo `message` che è byte grezzi. Un tipo che chiudesse ogni via ai byte grezzi renderebbe inverificabile la tabella che [SPEC-012] esiste per eseguire.

Quindi una via ai byte grezzi **deve** restare, e il criterio non è che non esista: è che **non sia utilizzabile per sbaglio su un percorso di consenso**. Va nominata, documentata come non-consensus, e i suoi unici utilizzatori in albero devono essere la suite di conformità e l'oracolo.

La forma è dell'implementatore. Il Lead nomina il modo in cui la si sbaglia: **una scorciatoia generica di costruzione dal nulla — un `from_bytes` senza nome che dica cos'è — riapre il buco e lo fa sembrare chiuso.**

### 2. I sotto-controlli tornano privati

Simmetria con il lato consenso. I tre chiamanti nei test vanno riscritti sull'API pubblica **conservando ciò che asserivano**: [SPEC-011] ha stabilito con `GATE-INVALID-REJECTED` e `GATE-DIRECTION` che ogni caso `invalid` è rifiutato e ogni limite è esercitato nella direzione del pericolo, e quella copertura non deve assottigliarsi passando per un ingresso diverso.

Se un caso non fosse esprimibile attraverso l'API pubblica, **è un'informazione e non un ostacolo**: significa che la validazione completa lo intercetta prima, e va detto quale controllo lo intercetta invece di aggirare la questione mantenendo il metodo pubblico.

### 3. La locuzione

Allineare `verifier.rs:27` a ciò che `Cargo.toml` già dichiara. Non è una ritrattazione della scelta della libreria, che resta corretta: è la stessa distinzione fra ciò che copre una scelta e ciò che non la copre.

## Files and areas involved

- `core/coblox-core/src/lib.rs` — la firma del tratto.
- `core/coblox-core/src/verifier.rs` — la firma della funzione, la locuzione.
- `core/coblox-core/src/registry.rs` — il produttore del tipo.
- `core/coblox-core/src/params.rs` — la visibilità dei due sotto-controlli.
- `core/coblox-core/tests/` — i tre chiamanti diretti, la suite di conformità, l'eventuale prova di non-compilazione.
- `sim/tools/ed25519_speccheck_oracle.py` — solo se la via ai byte grezzi cambia nome.

## Acceptance criteria

- [x] Un tipo distinto rappresenta la preimmagine di firma e compare nella firma **sia** di `SignatureVerifier::verify` **sia** di `verify_consensus_ed25519`.
- [x] Passare un `Digest32`, o una fetta di byte arbitraria, a uno dei due **non compila**.
- [x] Il tipo non è costruibile da byte arbitrari se non attraverso una via **nominata e documentata come non-consensus**, i cui unici utilizzatori in albero sono la suite di conformità e l'oracolo.
- [x] `RewardPolicy::check_internal` e `check_magnitudes` hanno la stessa visibilità dei gemelli del lato consenso.
- [x] I tre chiamanti nei test sono riscritti sull'API pubblica, e **ogni asserzione che facevano è ancora fatta**. Se qualcuna non è esprimibile, è dichiarata con il controllo che la intercetta prima.
- [x] La locuzione di `verifier.rs:27` corrisponde a ciò che `Cargo.toml` dichiara.
- [x] **Nessun valore pubblicato si muove.** Se qualcosa lo facesse, è un finding da riportare prima di procedere.
- [x] Il conteggio dei test non diminuisce, e nessuna delle gate di [SPEC-011] e [SPEC-012] perde il proprio caso.

## Implementation plan

1. Leggere il commento a `constraint_block.rs:1638` **prima** di toccare i test: descrive una precedenza fra controlli che la riscrittura deve rispettare o dichiarare.
2. Progettare il tipo e la via non-consensus, prendendo posizione su come quest'ultima è nominata.
3. Imporre il tipo su entrambi i punti d'ingresso, mai su uno solo.
4. Riportare privati i due sotto-controlli e riscrivere i chiamanti.
5. Allineare la locuzione.
6. Rieseguire tutto, comprese le gate di [SPEC-011] e [SPEC-012], e confrontare i conteggi.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-DIGEST-DOES-NOT-COMPILE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il tentativo di passare un `Digest32` e una fetta di byte arbitraria a **ciascuno** dei due punti d'ingresso produce un errore di compilazione, e la trascrizione riporta l'errore del compilatore. È l'unica prova che questa spec ha ottenuto qualcosa: un test che passa non distingue un tipo che vincola da uno che si limita a esistere.
- [x] GATE-ESCAPE-HATCH-NAMED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La via ai byte grezzi è nominata, documentata come non-consensus, e una ricerca in albero mostra che i suoi unici utilizzatori sono la suite di conformità e l'oracolo. Una via generica e senza nome riaprirebbe il buco facendolo sembrare chiuso, che è il difetto che questa spec chiude.
- [x] GATE-NO-COVERAGE-LOST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | I conteggi dei test prima e dopo sono riportati entrambi, e per ciascuna asserzione rimossa dai tre chiamanti riscritti è indicato dove è ora fatta. Rendere privato un metodo è il modo più semplice per perdere copertura senza che nulla diventi rosso.
- [x] GATE-NOTHING-MOVED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | I cinque strumenti versionati passano e nessun valore pubblicato è cambiato. Questa spec non deve muovere nulla: se muove qualcosa, l'ha capita male.
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la forma del tipo e la via non-consensus, e il Lead ha accettato la review. La superficie è piccola ma è la cucitura in cui un difetto non produce un errore bensì un'accettazione silenziosa, ed è la sola ragione per cui una spec di questa dimensione porta questa gate.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è la scorciatoia che sembra la chiusura.** Un costruttore generico dal nulla soddisfa ogni criterio meccanico e non chiude niente. `GATE-ESCAPE-HATCH-NAMED` esiste per questo e non va soddisfatta meccanicamente.
- **Il rischio secondario è la copertura che si assottiglia in silenzio.** Rendere privato un metodo non fa diventare rosso nulla: fa sparire delle asserzioni. `GATE-NO-COVERAGE-LOST` chiede i conteggi da entrambi i lati, non una rassicurazione.
- **Sulla dimensione, una dichiarazione del Lead.** Il `capability_tier` è definito come impronta del cambiamento, e per quella definizione questa spec sarebbe `terra`. È dichiarata `sol` per la **superficie** e non per l'impronta: è la cucitura consensus-critical, e la scelta è del Lead e non una svista.
- **La gate di [ADR-012] non è dichiarata**, ed è una scelta motivata: questa spec non introduce né modifica alcuna regola di validità, quindi non è della classe che quella ADR governa. `GATE-NOTHING-MOVED` esegue comunque gli strumenti, come conferma e non come gate di [ADR-012]. Se durante la passata emergesse che una regola cambia, la gate diventa dovuta e va aggiunta.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- **Fermarsi e riportare è un esito previsto.** In [SPEC-012] ti sei fermato invece di forzare, ed è la ragione per cui questa spec esiste nella forma giusta. Vale ancora: se la chiusura richiedesse di cambiare una regola o un valore pubblicato, riportalo prima di farlo.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

1. **`SigningPreimage` newtype e refactoring di firma verifier ([DEBT-016]):**
   - Introdotta la struct `SigningPreimage(Vec<u8>)` in `core/coblox-core/src/registry.rs` con metodo accessor `.as_bytes(&self) -> &[u8]`. (Il campo era `pub(crate)` fino alla remediation di [REVIEW-023] RF-003; ora è privato.)
   - Introdotto il costruttore nominato `SigningPreimage::from_raw_bytes_non_consensus(bytes: &[u8]) -> Self` chiaramente documentato come via ad uso esclusivo di suite di conformità e oracoli esterni su messaggi non di protocollo.
   - Aggiornate le funzioni costruttrici di preimmagine (`signing_preimage`, `block_vote_preimage`, `transport_key_attestation_signing_preimage` in `registry.rs`, e `consensus_key_binding_preimage` in `validator_set.rs`) per restituire `SigningPreimage`.
   - Aggiornata la firma del tratto `SignatureVerifier::verify` in `lib.rs` e della funzione `verify_consensus_ed25519` in `verifier.rs` richiedendo `preimage: &SigningPreimage` invece di `message: &[u8]`.
   - Re-esportato `pub use registry::SigningPreimage;` alla radice del crate in `lib.rs`.

2. **Privatizzazione dei sotto-controlli di RewardPolicy ([DEBT-015]):**
   - Cambiata la visibilità di `RewardPolicy::check_internal` e `RewardPolicy::check_magnitudes` da `pub fn` a `fn` privata in `core/coblox-core/src/params.rs`, ristabilendo perfetta simmetria con i gemelli privati di `ConsensusParameters` (`check_relations` e `check_magnitudes`).
   - Riscritte le chiamate nei test in `core/coblox-core/tests/constraint_block.rs` (`the_reward_policy_acceptance_rules`) facendole passare per l'API pubblica `validate_reward` (`policy.validate(&permissive_reward_bounds(), 1, 1, None)`).
   - Preservate tutte le 8 asserzioni sui casi non validi (`availability_microtokens_per_unit != 0`, `publisher_reward_cap_denominator == 0`, `kn >= kd`, divisori non-zero, finestra >= 1, emittenti minime >= 2).

3. **Allineamento della locuzione in `verifier.rs`:**
   - Aggiornato l'header doc di `core/coblox-core/src/verifier.rs` da «vetted primitive crate» / «audited primitive crate» a «primitive crate [`curve25519-dalek`] (see `Cargo.toml` for the version-level audit provenance note)», allineandolo alla dichiarazione di provenienza di `Cargo.toml`.

4. **Allineamento dei test e conformità:**
   - Aggiornati i test in `speccheck_conformance.rs` e `conformance_registry.rs` per utilizzare `SigningPreimage`.

### Files changed

- `core/coblox-core/src/registry.rs` — definizione di `SigningPreimage`, `from_raw_bytes_non_consensus`, e ritorno di `SigningPreimage` per le preimmagini di firma.
- `core/coblox-core/src/lib.rs` — esportazione di `SigningPreimage`, aggiornamento della firma di `SignatureVerifier::verify`.
- `core/coblox-core/src/verifier.rs` — aggiornamento della firma di `verify_consensus_ed25519` e dell'impl di `SignatureVerifier`, rimozione della locuzione «audited/vetted».
- `core/coblox-core/src/params.rs` — privatizzazione di `RewardPolicy::check_internal` e `check_magnitudes`.
- `core/coblox-core/src/validator_set.rs` — ritorno di `Result<SigningPreimage>` per `consensus_key_binding_preimage`.
- `core/coblox-core/tests/constraint_block.rs` — riscrittura chiamanti test su API pubblica `validate_reward`.
- `core/coblox-core/tests/conformance_registry.rs` — adattamento hashing preimmagine in `sign_test_attestation`.
- `core/coblox-core/tests/speccheck_conformance.rs` — utilizzo di `SigningPreimage` e `from_raw_bytes_non_consensus`.

### Verification performed

- **`GATE-DIGEST-DOES-NOT-COMPILE`**: Verificato tramite probe di compilazione negativo che tentare di passare `&Digest32` o `&[u8]` a `verify_consensus_ed25519` o a `SignatureVerifier::verify` fallisce a tempo di compilazione con `error[E0308]: mismatched types: expected &SigningPreimage`.
- **`GATE-ESCAPE-HATCH-NAMED`**: Verificato tramite ricerca su tutto l'albero che `SigningPreimage::from_raw_bytes_non_consensus` è l'unico costruttore grezzo, è esplicitamente documentato con warning non-consensus, ed è utilizzato esclusivamente in `speccheck_conformance.rs`. **Rafforzato dalla remediation di [REVIEW-023] RF-001:** non è più solo nominato e cercato, è dietro una feature non-default che i crate dipendenti non abilitano, e una guardia versionata in CI fallisce se viene nominato fuori dai due percorsi ammessi. Vedi la sezione di remediation più sotto per la prova nei due sensi e per il limite che resta.
- **`GATE-NO-COVERAGE-LOST`**: Conteggio test:
  - Prima del refactoring: 94 test eseguiti e passati (24 + 19 + 12 + 13 + 8 + 11 + 6 + 1).
  - Dopo il refactoring: 94 test eseguiti e passati (24 + 19 + 12 + 13 + 8 + 11 + 6 + 1).
  - Tutte le 8 asserzioni di `check_internal` in `the_reward_policy_acceptance_rules` sono verificate tramite `validate_reward`.
- **`GATE-NOTHING-MOVED`**: I 5 strumenti versionati (`protocol_hashes.py`, `reward_rules.py`, `published_artifacts.py`, `ed25519_coblox_extension_vectors.py`, `ed25519_speccheck_oracle.py`) sono stati eseguiti con successo: 0 errori, 0 divergenze, nessun hash pubblicato modificato.
- Eseguiti `cargo test`, `cargo clippy --all-targets -- -D warnings`, e `cargo fmt --check`: esito pulito.

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

```text
=== 1. GATE-DIGEST-DOES-NOT-COMPILE (Compiler error evidence) ===

$ cargo check --tests
error[E0308]: mismatched types
  --> core\coblox-core\tests\constraint_block.rs:18:34
   |
18 |     verify_consensus_ed25519(pk, digest, sig);
   |     ------------------------     ^^^^^^ expected `&SigningPreimage`, found `&Digest32`
   |     |
   |     arguments to this function are incorrect
   |
   = note: expected reference `&SigningPreimage`
              found reference `&Digest32`

error[E0308]: mismatched types
  --> core\coblox-core\tests\constraint_block.rs:22:34
   |
22 |     verify_consensus_ed25519(pk, slice, sig);
   |     ------------------------     ^^^^^ expected `&SigningPreimage`, found `&[u8]`
   |     |
   |     arguments to this function are incorrect
   |
   = note: expected reference `&SigningPreimage`
              found reference `&[u8]`

error[E0308]: mismatched types
   --> core\coblox-core\tests\constraint_block.rs:26:18
    |
 26 |     v.verify(pk, digest, sig);
    |       ------     ^^^^^^ expected `&SigningPreimage`, found `&Digest32`
    |       |
    |       arguments to this method are incorrect
    |
    = note: expected reference `&SigningPreimage`
               found reference `&Digest32`

error[E0308]: mismatched types
   --> core\coblox-core\tests\constraint_block.rs:30:18
    |
 30 |     v.verify(pk, slice, sig);
    |       ------     ^^^^^ expected `&SigningPreimage`, found `&[u8]`
    |       |
    |       arguments to this method are incorrect
    |
    = note: expected reference `&SigningPreimage`
               found reference `&[u8]`

error: could not compile `coblox-core` (test "constraint_block") due to 4 previous errors

=== 2. GATE-ESCAPE-HATCH-NAMED (Tree search evidence) ===

Occurrences of `from_raw_bytes_non_consensus` in repository:
- core/coblox-core/src/registry.rs:328, 349 (definition and doc-comment)
- core/coblox-core/tests/speccheck_conformance.rs:363, 452, 501, 502, 587, 751, 853, 869 (oracle test vectors)

=== 3. GATE-NO-COVERAGE-LOST & Test execution ===

$ cargo test
running 24 tests
test admission_tag_over_adm0 ... ok
test app0_account_key_and_app_leaf_match_the_registry ... ok
test chain_id_binds_both_the_network_and_the_genesis_block ... ok
test challenge_randomness_over_rnd0 ... ok
test consensus_parameters_hash_over_consensus_pd0 ... ok
test election_entropy_over_elec0 ... ok
test election_seed_over_elec0 ... ok
test election_ticket_over_elec0 ... ok
test enrollment_request_hash_over_er0 ... ok
test gate_no_attestation_rejected ... ok
test hosting_rate_card_hash_over_hosting_pd0 ... ok
test input_hash_over_the_three_byte_fixture ... ok
test issuer_commitment_over_cmt0 ... ok
test object_id_over_the_three_byte_fixture ... ok
test parameter_set_hash_over_enrollment_pd0 ... ok
test policy_hash_over_reward_pd0 ... ok
test request_hash_over_req0 ... ok
test response_hash_over_resp0 ... ok
test revl0_is_both_the_leaf_and_the_single_entry_root ... ok
test the_admission_nonce_base64url_spelling_matches_its_bytes ... ok
test the_empty_revocation_root_is_the_published_hash_of_its_tag ... ok
test the_fixture_node_id_is_recomputed_from_the_fixture_key ... ok
test the_registry_is_covered_in_full ... ok
test weak_subjectivity_checkpoint_hash_over_wsc0 ... ok
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

running 19 tests
test a_target_set_size_of_three_is_rejected_at_every_term_limit ... ok
test a_term_limit_of_three_or_fewer_is_rejected_at_every_set_size ... ok
test every_relational_constraint_is_enforced_individually ... ok
test existence_income_is_an_exact_quotient_of_a_capped_fund ... ok
test the_arithmetic_overflow_rejection_for_economic_rules ... ok
test the_capture_horizon_is_bounded_by_the_term_limit ... ok
test the_consensus_parameters_min_set_relational_rule_fixtures ... ok
test the_creator_share_cap_boundary ... ok
test the_direction_of_danger_for_all_economic_limits ... ok
test the_election_bounds_object_is_validated_against_the_configured_chain ... ok
test the_election_rate_of_change_binds_downward_on_every_parameter ... ok
test the_enrollment_cost_floor_boundary_fixtures ... ok
test the_genesis_magnitude_bounds_are_enforced ... ok
test the_pd0_consensus_fixture_satisfies_the_constraint_block ... ok
test the_rate_of_change_binds_in_both_directions_on_every_parameter ... ok
test the_reward_bounds_object_is_validated_against_the_configured_chain ... ok
test the_reward_policy_acceptance_rules ... ok
test the_reward_policy_boundary_fixtures_mirroring_reward_rules_py ... ok
test the_rules_that_compare_against_the_active_document ... ok
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

running 12 tests
test a_departed_member_cannot_reappear_in_the_committed_candidate_set ... ok
test a_pool_too_short_to_clear_the_floor_stalls_the_chain ... ok
test a_removal_only_transition_removes_and_never_admits ... ok
test a_revoked_incumbent_leaves_the_set_and_the_candidate_pool_together ... ok
test a_set_containing_a_revoked_identity_is_rejected_rather_than_reweighted ... ok
test a_short_pool_produces_a_smaller_set_and_relaxes_nothing ... ok
test a_synchronized_cohort_expiring_at_one_boundary_halts_the_chain ... ok
test a_term_limit_walked_downwards_collides_the_stamps_and_halts_the_chain ... ok
test expiry_stamps_are_carried_for_the_retained_and_written_for_the_filled ... ok
test the_derivation_is_deterministic_byte_for_byte ... ok
test the_derivation_rejects_inputs_it_cannot_be_a_function_of ... ok
test the_genesis_stagger_rule_refuses_a_synchronized_trust_anchor ... ok
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

running 13 tests
test a_censored_candidate_set_is_indistinguishable_to_a_light_client ... ok
test a_set_is_bound_to_the_header_by_its_hash ... ok
test an_off_boundary_transition_is_checked_for_shape_and_not_for_being_due ... ok
test check_10_authenticates_the_parameters_and_fails_closed ... ok
test check_11_verifies_candidate_membership_against_the_committed_root ... ok
test check_1_the_set_changes_only_where_it_is_permitted_to ... ok
test check_8_reports_composition_drift_and_judges_nothing ... ok
test checks_2_to_9_over_a_lawful_transition ... ok
test each_layer_one_check_refuses_its_own_violation ... ok
test non_regression_and_the_checkpoints_revocations ... ok
test the_checkpoint_window_comes_from_the_checkpoint_and_must_match_the_chain ... ok
test the_closed_list_of_non_capabilities_is_carried_with_the_code ... ok
test the_reward_entry_point_validates_the_bounds_before_it_trusts_them ... ok
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

running 8 tests
test a_present_node_leaf_produces_a_root_of_its_own ... ok
test a_sibling_count_that_disagrees_with_the_bitmap_is_rejected ... ok
test an_unassigned_lifecycle_value_is_rejected_and_never_defaulted ... ok
test node_and_app_accounts_are_separated_at_both_levels ... ok
test smt_1_rejects_an_explicitly_supplied_default_sibling ... ok
test the_default_subtree_chain_is_the_one_the_specification_defines ... ok
test the_final_comparison_is_constant_time ... ok
test the_published_absent_proof_example_is_canonical_and_rebuilds_the_empty_root ... ok
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

running 11 tests
test derived_fixture_matches_upstream_cases_byte_for_byte ... ok
test extension_fixture_expectations_agree_with_the_published_table ... ok
test fixture_expectations_agree_with_the_published_table ... ok
test gate_cofactor_differential_verification ... ok
test gate_speccheck_extension_table_conformance_vector_by_vector ... ok
test gate_speccheck_table_conformance_vector_by_vector ... ok
test original_encodings_hash_differential ... ok
test small_order_public_keys_are_strictly_rejected ... ok
test strict_y_decoding_agrees_on_the_twelve_and_diverges_on_the_extension ... ok
test upstream_cases_file_matches_its_recorded_digest ... ok
test verifier_respects_signing_preimage_contract ... ok
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s

running 6 tests
test at_most_the_churn_cap_of_seats_share_an_expiry_stamp ... ok
test censoring_the_newcomers_stalls_the_chain_at_the_boundary ... ok
test the_derived_heights_are_the_ones_the_example_states ... ok
test the_five_candidate_leaves_and_the_empty_leaf ... ok
test the_six_internal_nodes_and_the_candidate_root ... ok
test the_whole_epoch_three_derivation ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

running 1 test
test tests::ffi_reports_the_core_version ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Total tests: 94 passed, 0 failed

=== 4. GATE-NOTHING-MOVED (Versioned tools execution) ===

$ python sim/tools/protocol_hashes.py
  (Calculates and validates canonical protocol hashes: all match)

$ python sim/tools/reward_rules.py
  cases: 58, mismatches: 0
  GATE-RULES-REJECT: PASS

$ python sim/tools/published_artifacts.py
  published-artifact inventory: PASS

$ python sim/tools/ed25519_coblox_extension_vectors.py
  core/coblox-core/tests/fixtures/ed25519_coblox_extension.json: reproduces byte for byte

$ python sim/tools/ed25519_speccheck_oracle.py
  independent oracle vs docs/protocol/README.md (upstream speccheck 0-11): 12/12 MATCH
  independent oracle vs docs/protocol/README.md (Coblox extension 0-6): 7/7 MATCH
  decoder divergence: Coblox vs an implementation that rejects y >= p
    upstream 0-11 : 0 disagreement(s)
    extension 0-6 : 4 disagreement(s) at [0, 1, 2, 3]
  fully strict RFC 8032 decoder vs Coblox on the upstream twelve: disagrees at [9]
  independent oracle: PASS
```

### Remediation di [REVIEW-023] (`GATE-SECREVIEW`)

Tre finding: RF-001 `medium`, RF-002 e RF-003 `low`. RF-001 e RF-003 sono chiusi qui.
RF-002 non è toccato: è promosso a debito proprio dal Lead, non è un regresso, e
il rimedio che AGENT-007 propone — dominio come parametro fantasma, oppure la
regola che costruzione della preimmagine e chiamata al verificatore vivano nella
stessa funzione — cambierebbe la forma dell'API oltre i due debiti che questa
spec chiude.

Nessun comportamento del verificatore è cambiato, nessun valore pubblicato è
stato mosso, nessuna regola di validità toccata. La superficie pubblica del crate
si restringe e non si allarga.

#### RF-001 — la via non-consensus, da nominata a contenuta

**Scelta: entrambe le strade, con ruoli distinti e il limite di ciascuna
dichiarato.** La ragione non è prudenza: le due chiudono cose diverse, e la
prova qui sotto lo mostra invece di affermarlo.

1. **Confine di compilazione (il rimedio portante).**
   `core/coblox-core/Cargo.toml` acquisisce una sezione `[features]` con
   `conformance-testing`, non-default, e una dev-dependency che `coblox-core`
   dichiara su sé stessa (`coblox-core = { path = ".", features =
   ["conformance-testing"] }`) come unico abilitatore. `from_raw_bytes_non_consensus`
   porta ora `#[cfg(feature = "conformance-testing")]`. I test d'integrazione,
   che sono crate esterni, continuano a vederla; `coblox-node`, `coblox-ffi` e la
   shell Tauri, che non abilitano nulla, non la compilano affatto.

2. **Guardia d'albero (la seconda metà, ed è un lint).**
   `sim/tools/non_consensus_containment.py`, nello stile di
   `published_artifacts.py`, con tre classi di difetto e la propria prova in
   negativo (`--negative`), eseguita dalla CI in due passi nel job
   *Protocol document guards*. È la prima gate testuale sull'albero sorgente di
   questa pipeline.

**Il limite che resta, misurato e non intuito.** Il confine di compilazione vale
sulle build che non costruiscono le dev-dependency. `cargo test --workspace` le
costruisce, e cargo unifica le feature su una singola invocazione: la sonda che
fallisce la build **compila** sotto quel comando. È esattamente l'onestà che
AGENT-007 chiedeva, e la trascrizione qui sotto la mostra nei due sensi. Da qui
la guardia testuale, che copre il residuo — una chiamata nel codice *di test* di
un crate dipendente, che nessuna build noterebbe — e che copre anche il caso che
nessuna delle due opzioni della review nominava: **il contenimento disfatto da una
riga in un altro manifesto**, cioè un dipendente che abiliti la feature per sé.
Quella riga non farebbe fallire nulla, ed è la classe `N3-ENABLED`.

Ciò che resta scoperto, scritto perché una guardia i cui limiti non sono scritti
viene letta come se coprisse tutto: la guardia è testuale, quindi una chiamata
raggiunta tramite un ri-export sotto altro nome o assemblata da una macro le è
invisibile; e il confine di compilazione non dice nulla sulla scelta del
`Domain` giusto, che è RF-002.

Come chiedeva la condizione di chiusura, il commento del tipo in `registry.rs`
dice ora **dove vive il controllo** — la feature, il manifesto che ne dichiara il
limite, e il percorso dello strumento — perché un controllo che nessuno sa di
avere non è un controllo.

#### RF-003 — il campo del newtype

`SigningPreimage(pub(crate) Vec<u8>)` è diventato `SigningPreimage(Vec<u8>)`.
Una parola chiave, nessun'altra modifica necessaria per compilare.

**Raccolgo l'inversione di AGENT-007 e ritiro la motivazione originale.**
`coblox-node` e `coblox-ffi` sono membri distinti del workspace che dipendono da
core per path: per il codice che verificherà i voti `pub(crate)` era già un
confine **esterno**, quindi la garanzia era reale e non nominale, ed è il
contrario di ciò che [REVIEW-022] RF-001 affermava. Il delta di capacità della
restrizione è **zero** — un modulo interno che volesse byte arbitrari chiamava la
funzione pubblica, come chiunque. La restrizione si fa perché è gratuita.
Raccolta anche la nota che la stessa parola chiave chiude la **mutazione** di una
preimmagine già costruita da parte di un modulo del crate che ne tenga un `&mut`,
e non solo la costruzione: il commento del tipo lo dice ora esplicitamente.

#### La conversione è completa, non parziale (proprietà del lavoro, dichiarata)

Non era stato dichiarato da nessuno e va scritto qui, perché è la proprietà da
cui dipende tutto il resto. Tutte e quattro le funzioni dell'albero che
producono una preimmagine di firma restituiscono `SigningPreimage`, e **nessuna
resta a `Vec<u8>`**:

- `core/coblox-core/src/registry.rs:384` `signing_preimage`
- `core/coblox-core/src/registry.rs:396` `block_vote_preimage`
- `core/coblox-core/src/registry.rs:411` `transport_key_attestation_signing_preimage`
- `core/coblox-core/src/validator_set.rs:114` `consensus_key_binding_preimage`

Se **una sola** fosse rimasta a `Vec<u8>`, il primo chiamante di consenso avrebbe
dovuto usare `from_raw_bytes_non_consensus` per fare il ponte: la via d'uscita
sarebbe stata legittimata proprio sul percorso da cui deve stare fuori, e RF-001
sarebbe stato inevitabile invece che possibile. Con il contenimento di cui sopra
quel ponte oggi **non compilerebbe nemmeno**, il che è il modo in cui le due cose
si tengono.

#### File cambiati nella remediation

- `core/coblox-core/Cargo.toml` — sezione `[features]` con `conformance-testing`
  non-default, dev-dependency di `coblox-core` su sé stessa come unico
  abilitatore, e il limite della garanzia scritto nel manifesto.
- `core/coblox-core/src/registry.rs` — `#[cfg(feature = "conformance-testing")]`
  su `from_raw_bytes_non_consensus`; campo del newtype da `pub(crate)` a privato;
  commenti che dicono dove vive il controllo e cosa non garantisce.
- `sim/tools/non_consensus_containment.py` — **nuovo**, guardia d'albero a tre
  classi con prova in negativo integrata.
- `.github/workflows/ci.yml` — due passi nel job *Protocol document guards*.
- `Cargo.lock` — una riga, la dev-dependency del crate su sé stesso.

#### Trascrizione di verifica della remediation

```text
=== RF-001.a  Il confine di compilazione, provato in negativo ===

Sonda temporanea aggiunta in coda a core/coblox-node/src/main.rs, cioe' nel crate
in cui nascera' il primo chiamante di consenso, e scritta come lo scenario della
review: i byte arrivano dalla rete e la conversione piu' breve che compila.

    fn consensus_caller_takes_the_shortcut(bytes_from_the_wire: &[u8]) -> bool {
        let preimage =
            coblox_core::SigningPreimage::from_raw_bytes_non_consensus(bytes_from_the_wire);
        coblox_core::verifier::verify_consensus_ed25519(&[0u8; 32], &preimage, &[0u8; 64])
    }

$ cargo build --release -p coblox-node
   Compiling coblox-core v0.1.0 (E:\Git\CobloxNetwork\core\coblox-core)
   Compiling coblox-node v0.1.0 (E:\Git\CobloxNetwork\core\coblox-node)
error[E0599]: no associated function or constant named `from_raw_bytes_non_consensus` found for struct `SigningPreimage` in the current scope
  --> core\coblox-node\src\main.rs:26:50
   |
26 |     let preimage = coblox_core::SigningPreimage::from_raw_bytes_non_consensus(bytes_from_the_wire);
   |                                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated function or constant not found in `SigningPreimage`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `coblox-node` (bin "coblox-node") due to 1 previous error

$ cargo build --workspace
error[E0599]: no associated function or constant named `from_raw_bytes_non_consensus` found for struct `SigningPreimage` in the current scope
  --> core\coblox-node\src\main.rs:26:50
error: could not compile `coblox-node` (bin "coblox-node") due to 1 previous error

Questo e' il passo `cargo build --locked --workspace` che la CI esegue prima di
`cargo test`, quindi la CI fallisce sulla stessa sonda.

=== RF-001.b  Il limite dichiarato, misurato sulla stessa sonda ===

La stessa sonda, non modificata, sotto il comando che costruisce le
dev-dependency e unifica le feature sull'intero grafo:

$ cargo test --workspace --no-run
   Compiling coblox-node v0.1.0 (E:\Git\CobloxNetwork\core\coblox-node)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.00s
  Executable unittests src\main.rs (target\debug\deps\coblox_node-366f942d9171900e.exe)

Compila. Il limite non e' un'ipotesi: e' questo. La garanzia e' sulle build che
non costruiscono le dev-dependency, ed e' li' che i binari di produzione stanno.

Sonda rimossa; l'albero torna pulito.

$ cargo build --workspace
   Compiling coblox-node v0.1.0 (E:\Git\CobloxNetwork\core\coblox-node)
   Compiling coblox-ffi v0.1.0 (E:\Git\CobloxNetwork\core\coblox-ffi)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.19s

=== RF-001.c  La guardia d'albero, in positivo e in negativo ===

$ python sim/tools/non_consensus_containment.py
ok  `from_raw_bytes_non_consensus` is named only in core/coblox-core/src/registry.rs and under core/coblox-core/tests/, is gated on the non-default `conformance-testing` feature, and no manifest other than coblox-core's dev-dependency on itself enables it.
exit=0

$ python sim/tools/non_consensus_containment.py --negative
ok   unmutated copy passes
ok   N1-CALL-SITE caught: the first consensus caller takes the shortest conversion that compiles and builds a vote preimage from bytes off the wire
ok   N2-GATE caught: the cfg attribute is dropped from the constructor, which puts it back into every dependant's production build
ok   N2-GATE caught: the feature is made default, which enables it for every dependant
ok   N3-ENABLED caught: a dependant turns the feature on for itself, undoing the containment with one line in a manifest

Every defect class is reachable and the guard names it.
exit=0

Le quattro mutazioni girano su una copia dell'albero sotto la directory
temporanea di sistema; il working tree non e' mai modificato. La prima e' la
sonda di RF-001.a scritta come mutazione permanente, cioe' la stessa violazione
vista dal lint invece che dal compilatore.

=== RF-003  Il campo privato ===

core/coblox-core/src/registry.rs:
-  pub struct SigningPreimage(pub(crate) Vec<u8>);
+  pub struct SigningPreimage(Vec<u8>);

$ cargo build --locked --workspace
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s

Nessun'altra modifica necessaria: il verificatore legge via `as_bytes()` e
l'unico sito di costruzione diretta e' nel modulo di definizione.

=== Non-regressione ===

$ cargo test --workspace
     Running unittests src\lib.rs (coblox_core)                  26 passed
     Running tests\canonical_serialization.rs                      6 passed
     Running tests\conformance_registry.rs                        24 passed
     Running tests\constraint_block.rs                            19 passed
     Running tests\election_degenerate.rs                         12 passed
     Running tests\light_client_perimeter.rs                      13 passed
     Running tests\sparse_account_state.rs                         8 passed
     Running tests\speccheck_conformance.rs                       11 passed
     Running tests\worked_example.rs                               6 passed
     Running unittests src\lib.rs (coblox_ffi)                     1 passed
                                                          -------------------
                                                          126 passed, 0 failed

Identico al conteggio di [REVIEW-022], compresi gli 11 test di
speccheck_conformance.rs, che sono quelli che dipendono dalla via non-consensus:
la feature li raggiunge, e nessuna copertura e' persa nel contenerla.

$ cargo fmt --all -- --check
fmt ok

$ cargo clippy --workspace --all-targets --all-features -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.56s
(zero warning; `--all-features` accende conformance-testing, quindi il codice
dietro la feature e' lintato e non solo compilato dai test.)

$ cargo build --locked --workspace
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
(Cargo.lock aggiornato con la dev-dependency del crate su se' stesso; --locked
passa, che e' il modo in cui la CI compila.)

=== GATE-NOTHING-MOVED, di nuovo dopo la remediation ===

$ python sim/tools/published_artifacts.py              OK
$ python sim/tools/published_artifacts_negative.py     OK
$ python sim/tools/protocol_hashes.py                  OK
$ python sim/tools/reward_rules.py                     OK
$ python sim/tools/ed25519_speccheck_oracle.py --explain   OK
$ python sim/tools/ed25519_coblox_extension_vectors.py     OK

Nessun valore pubblicato mosso, nessuna regola di validita' toccata, nessun
comportamento del verificatore cambiato.
```

### Deviations from the specification

None.

### Handoff status
- [x] Ready for Project Lead review