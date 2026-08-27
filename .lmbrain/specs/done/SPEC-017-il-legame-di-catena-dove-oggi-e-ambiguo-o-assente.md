---
id: SPEC-017
# Note: Quote the title if it contains a colon
title: "Il legame di catena dove oggi e ambiguo o assente"
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
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012]
links: []
created: 2026-08-26
updated: 2026-08-27
tags: [conformance, ledger, rust, security]
activity:
  - date: 2026-08-26
    action: "transitioned backlog -> ready"
  - date: 2026-08-26
    action: "transitioned ready -> working"
  - date: 2026-08-26
    action: "transitioned working -> review"
  - date: 2026-08-26
    action: "attested verification GATE-SECREVIEW by lead"
  - date: 2026-08-26
    action: "transitioned review -> done"
  - date: 2026-08-27
    action: "waived acceptance criterion 5 against DEBT-029"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "ecd7fc8ce2498f279587a146a237baa8673acace312e58012c13e8d9c5a618a0"
    evidence_ref: "REVIEW-029"
    id: "SPEC-017-ATTEST-001"
    requirement_digest: "a9c47c07926b8cf49ac42b821250d0fc098d01605ab4df1427bee90e293e0f6e"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-26T11:21:25.281707200+02:00"
mutation_overrides:
  - schema_version: "1"
    id: "SPEC-017-OVERRIDE-001"
    actor_role: "project-lead"
    timestamp: "2026-08-26T11:21:57.740102200+02:00"
    from: "review"
    to: "done"
    reason: "Forzato per una sola casella non spuntata, e la mancata spunta e' corretta e voluta.\n\nIl criterio e' condizionale: \"Se la forma scelta e' di compilazione, il caso sbagliato non compila\". La forma scelta e' a runtime - PreimageContext confrontato da binds() dentro verify_in_context - quindi l'antecedente e' falso e il criterio e' vacuamente soddisfatto. L'implementatore ha lasciato la casella vuota invece di spuntarla, annotando la ragione in linea e nelle Deviations.\n\nE' la scelta giusta e il Lead l'ha gia' lodata in REVIEW-028: spuntare un condizionale con antecedente falso significa marcare come verificata una clausola che nessun oracolo esercita, cioe' la famiglia 4 del censimento commessa dentro il gesto di chiusura. La lifecycle chiede tutte le caselle spuntate; qui la casella dice qualcosa di piu' preciso restando vuota, e il forzamento registra quel di piu' invece di cancellarlo.\n\nIl ramo alternativo dello stesso criterio e' soddisfatto e provato: GATE-WRONG-CONTEXT-REJECTED nel ramo \"e' rifiutata\", con matrice 4x4 - sedici celle, quattro accettazioni, nessuna grandezza tenuta costante - e prova in negativo che rende binds() sempre true e osserva cadere tre test.\n\nTutto il resto e' spuntato con la sua evidenza, e le cinque gate sono soddisfatte, GATE-SECREVIEW compresa con l'attestazione del Lead su REVIEW-029."
    unmet_invariant: "a done spec requires its acceptance criteria checked and evidence recorded"
---
# Il legame di catena dove oggi e ambiguo o assente

## Objective

Chiudere [DEBT-020] e [DEBT-021], che sono la stessa questione a due altezze: **`chain_id` lega quasi tutto il protocollo, e in due punti quel legame non tiene.**

Alla genesi `chain_id` è **ambiguo**, perché la sua derivazione è circolare e nessuna regola dice come si rompe. Nel verificatore è **assente**, perché `SigningPreimage` garantisce da dove vengono i byte e non quale contesto rappresentino.

## Context

**[DEBT-020].** `chain_id` ← `genesis_block_id` ← intestazione di genesi ← `validator_set_hash`. La fixture `HASH-0` usa 32 byte a zero, ma è una fixture e non una regola. Due implementazioni possono derivare **due `chain_id` diversi dalla stessa distribuzione di genesi**, e poiché `chain_id` entra in quasi tutte le preimmagini a dominio separato, **non concorderebbero su nulla**. Colpisce anche l'ancora del light client, il cui passo 1 impone `chain_id` uguale al configurato: se il valore corretto è ambiguo, non esiste un solo valore configurabile giusto.

È la stessa forma di [DEBT-012], chiuso da [SPEC-010]: un valore che entra in una preimmagine e che nessun documento fissa, **invisibile a ogni test di questa base di codice** perché una sola implementazione è internamente coerente.

**[DEBT-021].** `SigningPreimage`, introdotto da [SPEC-014], garantisce che i byte siano stati prodotti da `signing_preimage`, e **nulla su quali byte siano**: il tipo non trasporta il `Domain` né il `chain_id`, che `signing_preimage` impasta nel prefisso e il tipo poi dimentica. Un chiamante che costruisse la preimmagine con il dominio sbagliato, o con il `chain_id` di un'altra catena, otterrebbe **un valore ben tipato e semanticamente falso**, e il verificatore lo accetterebbe.

È un fallimento **più difficile da notare** di quello che [SPEC-014] ha chiuso: là il prefisso spariva del tutto, qui c'è ma può essere quello sbagliato. E la separazione di dominio esiste precisamente per impedire che una firma valida in un contesto lo sia in un altro.

**Perché insieme e perché ora.** Entrambi hanno la stessa scadenza — **prima del primo chiamante del verificatore e prima che una devnet accumuli storia** — ed entrambi toccano la stessa grandezza. Chiuderne uno solo lascerebbe il legame di catena difeso a metà.

## Scope

### Included

- La regola normativa che rompe la circolarità di `chain_id` alla genesi, con la fixture pubblicata corrispondente.
- La generalizzazione che [DEBT-020] pone: quali altri valori entrano in una preimmagine **senza essere derivabili in un solo modo**.
- Il legame fra `SigningPreimage` e il contesto per cui è stato costruito, o la dimostrazione motivata che non serve.

### Excluded

- Qualunque modifica alla logica di verifica delle firme. [SPEC-012] l'ha chiusa e [REVIEW-019] l'ha verificata con tre oracoli indipendenti: **non si tocca**.
- Il contenimento della via non-consensus, chiuso da [SPEC-014] e verificato in entrambi i sensi.
- [DEBT-022], che è di AGENT-007 e attende la sua valutazione.

## Existing-project analysis

**Verificato dal Lead il 2026-08-26.** Le quattro produttrici di preimmagini dell'albero — tre in `registry.rs`, una in `validator_set.rs` — restituiscono tutte `SigningPreimage`, e nessuna resta a `Vec<u8>`: la conversione è completa, ed è la proprietà su cui questa spec può appoggiarsi per imporre il contesto **in un punto solo** invece che su ogni chiamante.

`signing_preimage(domain, chain_id, payload)` compone `dominio || 0x00 || chain_id_32 || payload`: **entrambe le grandezze sono già nei byte**, e il problema non è produrle ma conservarle nel tipo.

L'inventario di [SPEC-010] conta le preimmagini prive di fixture pubblicata, ma **non verifica che ogni valore che vi entra sia derivabile in un solo modo**. È la lacuna che [DEBT-020] indica, ed è la classe generale a cui questa spec deve rispondere con un elenco.

## Technical proposal

### 1. La circolarità, rotta da una regola

Una regola normativa che dice **come** si rompe alla genesi, con la fixture pubblicata corrispondente, così che due implementazioni indipendenti derivino lo stesso `chain_id` dalla stessa distribuzione. Il valore a 32 byte zero di `HASH-0` può essere la risposta giusta: **ciò che manca non è un valore, è che sia una regola.**

Nella stessa passata, la generalizzazione: **quali altri valori entrano in una preimmagine senza essere derivabili in un solo modo.** È l'esercizio che [SPEC-010] ha fatto per le codifiche simboliche e che va rifatto per le derivazioni — e come là, **la risposta è un elenco e non una rassicurazione.**

### 2. Il contesto, portato dal tipo

Il tipo deve rendere impossibile — o almeno rilevabile — usare una preimmagine costruita per un contesto in un altro. Tre forme sono plausibili e la scelta è dell'implementatore, con l'argomento:

- un tipo **parametrizzato sul dominio**, che sposta il controllo alla compilazione;
- **campi conservati** e confrontati in verifica;
- una funzione di verifica che **prende dominio e `chain_id` attesi** e li confronta.

**Il criterio che le distingue:** l'ergonomia dei chiamanti che ancora non esistono. Una forma che rende scomodo il caso corretto verrà aggirata dal primo che ha fretta — ed è la ragione per cui questa decisione si prende **ora** che i chiamanti si possono immaginare, e non dopo che sono scritti.

**Se la conclusione è che il legame non serve**, va scritta accanto al tipo con la sua ragione, non lasciata implicita.

## Files and areas involved

- `docs/protocol/README.md`, `ledger.md` — la regola di genesi e la fixture.
- `core/coblox-core/src/registry.rs` — il tipo e le produttrici.
- `core/coblox-core/src/lib.rs`, `verifier.rs` — solo la firma, **mai la logica**.
- `core/coblox-core/tests/`, `sim/tools/` — fixture, gate di [ADR-012], eventuale elenco delle derivazioni.

## Acceptance criteria

- [x] Una regola normativa dice come si rompe la circolarità di `chain_id` alla genesi, e una fixture pubblicata la esercita.
- [x] Due derivazioni indipendenti dalla stessa distribuzione di genesi producono lo stesso `chain_id`, e la seconda è fatta **senza riusare il codice della prima**.
- [x] Esiste l'elenco dei valori che entrano in una preimmagine **senza essere derivabili in un solo modo**, anche se vuoto.
- [x] Una preimmagine costruita per un dominio o una catena non è utilizzabile in un altro senza che qualcosa lo dica, **oppure** la ragione per cui non serve è scritta accanto al tipo.
- [~] Se la forma scelta è di compilazione, il caso sbagliato **non compila**, e la trascrizione riporta l'errore. La forma scelta è a runtime: `GATE-WRONG-CONTEXT-REJECTED` è soddisfatta nel ramo *«e rifiutata»*, e il recinto a compilazione resta non imposto. Vedi *Deviations* 5 e la trascrizione del rifiuto in *Verification transcript*. | waived=DEBT-029
- [x] La logica di verifica delle firme è **invariata**: i dodici vettori upstream e i sette di estensione danno gli stessi esiti di prima.
- [x] La gate di [ADR-012] è eseguita e la trascrizione allegata.

## Implementation plan

1. Stabilire come si rompe la circolarità e con quale fixture, **prima** di toccare il tipo: è l'unica delle due che cambia artefatti pubblicati.
2. Produrre l'elenco delle derivazioni non univoche.
3. Scegliere la forma del legame di contesto, motivandola sull'ergonomia dei chiamanti futuri.
4. Verificare che gli esiti dei vettori Ed25519 non si siano mossi.
5. Eseguire la gate di [ADR-012] e le prove in negativo.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-TWO-DERIVATIONS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il `chain_id` di genesi è derivato **due volte per due strade che non condividono codice**, e i due valori coincidono. Una regola che rompe una circolarità è verificabile solo così: un'implementazione sola è internamente coerente per costruzione, ed è precisamente il motivo per cui [DEBT-012] è rimasto invisibile fino a [SPEC-010].
- [x] GATE-WRONG-CONTEXT-REJECTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una preimmagine costruita per un dominio o un `chain_id` e usata per un altro **è rifiutata o non compila**, e la trascrizione lo mostra. Se la conclusione è che il legame non serve, questa gate è sostituita dalla ragione scritta accanto al tipo e la sostituzione è dichiarata.
- [x] GATE-VERIFIER-UNCHANGED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | I dodici vettori upstream e i sette di estensione producono gli stessi esiti di prima, e l'oracolo indipendente concorda. Questa spec tocca la **forma** e non il **comportamento**: se un esito si muove, l'ha capita male.
- [x] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato e la trascrizione allegata. Questa spec aggiunge una regola di genesi e una fixture: è della classe che quella ADR governa.
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto entrambe le chiusure e il Lead ha accettato la review. La separazione di dominio è la difesa che impedisce a una firma di valere in due contesti, ed è materia sua.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è una seconda derivazione che non è indipendente.** Rileggere il codice della prima e riscriverlo in un altro linguaggio non è una seconda strada: è la stessa strada con un'altra sintassi. La derivazione va rifatta **dal documento**, come il Lead ha fatto per `ER-0` e per i vettori Ed25519.
- **Il rischio secondario è la forma scomoda.** Un legame di contesto che rende scomodo il caso corretto verrà aggirato dal primo chiamante che ha fretta, e il tipo diventerà un ostacolo invece che una difesa. È il criterio esplicito con cui va scelta la forma.
- **La circolarità potrebbe non essere l'unica.** L'elenco delle derivazioni non univoche è la parte da cui il Lead si aspetta di più, esattamente come l'elenco delle preimmagini scoperte in [SPEC-010] valeva più della fixture che quella spec doveva aggiungere.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Fermarsi e riportare è un esito previsto**, e in questa spec vale in particolare se la rottura della circolarità richiedesse di cambiare una preimmagine già pubblicata: è una decisione del Lead e apre la propria passata.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

**1. La regola che rompe la circolarita.** `docs/protocol/README.md` §*Genesis
derivation and the placeholder chain ID*: il **segnaposto di genesi** e 32 byte
zero, e lo usa ogni valore che sia ingresso di `genesis_block_id` e ogni firma
su un tale valore; tutto il resto della catena usa il `chain_id` derivato. Il
confine e meccanico — *questo valore e ingresso di `genesis_block_id`?* — ed e
enumerato per v0 nei due versi, compreso il caso che inganna: un checkpoint di
soggettivita debole emesso ad altezza 0 **non** e materiale di genesi e porta il
`chain_id` derivato.

Due clausole adiacenti, senza le quali la regola non determina `chain_id`:
`previous_block_id` dell'intestazione di genesi e 32 byte zero **e non e
configurabile** (`ledger.md`; la formulazione precedente, *«the configured
all-zero previous ID»*, ammetteva entrambe le letture), e il `network_id` che
entra in `chain_id` e quello dell'intestazione di altezza 0, che ogni altro
oggetto della catena DEVE portare identico byte per byte.

**2. La fixture `GEN-0`** (piu `DHT-0`), rete `genesis-fixture`: documento
`consensus_parameters` di genesi, intestazione di altezza 0, `genesis_block_id`,
`chain_id`, chiave di namespace DHT. Cinque valori pubblicati nuovi. `GEN-0` non
sta sulla rete `fixture` di proposito, e il documento dice perche: `HASH-0`
fissa `chain_id` a 32 byte zero **per dichiarazione**, il segnaposto lo fa **per
regola**, e leggere le righe del registro come la genesi di una catena renderebbe
`WSC-0` inammissibile. Copertura chiusa per quattro preimmagini finora scoperte:
`chain_id`, `block_id`, `dht_namespace_key`, `empty_transactions_root`.

**3. Il legame di contesto.** `SigningPreimage` porta ora un `PreimageContext`
(dominio + `chain_id`), `binds()` lo confronta con l'attesa del chiamante, e
`verifier::verify_in_context` e il punto d'ingresso verificato. Forma **a
runtime**, non di compilazione; le due alternative e le ragioni per cui hanno
perso sono scritte accanto al tipo. `identity.rs` e stato portato sul punto
d'ingresso verificato perche la forma che un lettore copia e la forma che avra
il prossimo chiamante.

**4. Il censimento** `.lmbrain/knowledge/derivazioni-non-univoche.md`: le 51
preimmagini, ogni ingresso classificato, cinque voci chiuse da questa spec e due
aperte — di cui **una e un difetto vero e non e chiusa qui**, `election_epoch`
(vedi *Deviations*).

### Files changed

- `docs/protocol/README.md` — la sezione normativa nuova, le tre clausole
  adiacenti, la definizione di `GEN-0`/`DHT-0`, cinque righe di tabella, e due
  affermazioni su `validator_set_hash` alla genesi che citavano [DEBT-020] come
  aperto.
- `docs/protocol/ledger.md` — `previous_block_id` di genesi reso regola; la
  stessa affermazione su `key_binding_signature` alla genesi.
- `core/coblox-core/src/hash.rs` — `ChainId::GENESIS_PLACEHOLDER`.
- `core/coblox-core/src/registry.rs` — `genesis_derivation`, `PreimageContext`,
  `SigningPreimage` con contesto e `binds`.
- `core/coblox-core/src/verifier.rs` — `verify_in_context`, **solo aggiunta**:
  la logica di verifica non e toccata di una riga.
- `core/coblox-core/src/identity.rs` — usa il punto d'ingresso verificato.
- `core/coblox-core/src/lib.rs` — riesportazioni.
- `core/coblox-core/tests/genesis_derivation.rs` — **nuovo**, 7 test.
- `core/coblox-core/tests/preimage_context.rs` — **nuovo**, 5 test.
- `sim/tools/genesis_chain_id.py` — **nuovo**, la seconda strada.
- `sim/tools/published_artifacts.toml` — 2 fixture, 5 valori, 4 coperture, 8
  probe C10, 1 mirror.
- `.github/workflows/ci.yml` — la seconda strada gira in CI.
- `.lmbrain/knowledge/derivazioni-non-univoche.md` — **nuovo**.

### Verification performed

- **GATE-TWO-DERIVATIONS** — due strade senza codice condiviso, concordi su
  **nove** valori attraverso **due** genesi diverse.
- **GATE-WRONG-CONTEXT-REJECTED** — matrice 4x4 e prova in negativo.
- **GATE-VERIFIER-UNCHANGED** — 12 vettori upstream + 7 di estensione, oracolo
  indipendente, e il diff di `verifier.rs` che e puramente additivo.
- **GATE-ADR012** — passata verde, prova in negativo con 111 probe.
- Test: **151 prima, 163 dopo**. `clippy -D warnings` e `fmt --check` puliti,
  `cargo build --locked --workspace` verde.

### Verification transcript

```text
$ python sim/tools/genesis_chain_id.py          # GATE-TWO-DERIVATIONS, strada B
1. the method, on a value this pass did not change
  ok    consensus_parameters_hash / consensus PD-0
          computed  sha256:87dc1d92edcd94d5efe3837af9157a4bda604dbd7a658f509bd6fb864f86ada5
2. GEN-0, derived under the genesis placeholder rule
  ok    empty_transactions_root / H(0x03)
          computed  sha256:084fed08b978af4d7d196a7446a86b58009e636b611db16211b65a9aadff29c5
  ok    consensus_parameters_hash / GEN-0 document
          computed  sha256:bec637279b6dceb786a0758c8a48de508d6d08bff5878c0b71f844e48da0f275
  ok    block_id / GEN-0 genesis header
          computed  sha256:1334f5368141f78f23528624bf91973cb4cdf316c1e3452cb0e5470ff7145f92
  ok    chain_id / GEN-0
          computed  sha256:3004d71cffe8ea2cc07b254abcc65494c112c13b20a305910476860b6cc62847
  ok    dht_namespace_key / DHT-0
          computed  sha256:80c13c86cb480fe927e4aafe885b687d5fd2900a2d53e46de0460ee48f943b26
3. the second genesis, unpublished, for the Rust road to meet
          consensus_parameters_gen0  sha256:6ba582b42339763c4b79e7a41ff7d75f6283800a5a4b4d97176f318cb5f63c0d
          genesis_block_id           sha256:6b62539240dcbc9aedf3e47e32edef91d302cf0687865dad8904326d8f49c53d
          chain_id                   sha256:172fd2e8bbdffefecc8952c1e0b97b69275af0de9bc637c6735a09b872d5e033
          dht_namespace_key          sha256:e8ceaa4c9095078ae2347bb111484ed532e5c494e49341aba2f5b57312d72c7b
4. every clause, watched failing
  ok    network_id enters the header, so genesis_block_id moves with it
  ok    network_id enters chain_id twice over, so chain_id moves too
  ok    dropping u32be(len(network_id_utf8)) changes chain_id
          moved to sha256:c67358e4a3edffffeaddd3b7caf9aef4d6213af8ed0dca928ce3fd7d6d1f63e8
  ok    a placeholder of 32 ff bytes changes genesis_block_id
          moved to sha256:8f3382b16d3edce53c19e8e27916f6e061c76522429b3bdb3fe61dcb00abad72
  ok    and changes chain_id with it
          moved to sha256:5e80fa7c9fb67faf74405937cddabe1d174cd3e3b30d42406ca3f95f8de36269
ok

$ cargo test --all-features --test genesis_derivation -- --nocapture   # strada A
running 7 tests
test a_different_placeholder_moves_the_genesis_block_id_and_the_chain_id ... ok
test the_empty_transactions_root_is_the_one_gen0_publishes ... ok
test dropping_the_network_length_prefix_moves_the_chain_id ... ok
test gen0_derives_the_published_genesis_values ... ok
test the_method_reproduces_a_value_this_pass_did_not_change ... ok
test the_placeholder_is_thirty_two_zero_bytes ... ok
second consensus_parameters_hash sha256:6ba582b42339763c4b79e7a41ff7d75f6283800a5a4b4d97176f318cb5f63c0d
second genesis_block_id          sha256:6b62539240dcbc9aedf3e47e32edef91d302cf0687865dad8904326d8f49c53d
second chain_id                  sha256:172fd2e8bbdffefecc8952c1e0b97b69275af0de9bc637c6735a09b872d5e033
second dht_namespace_key         sha256:e8ceaa4c9095078ae2347bb111484ed532e5c494e49341aba2f5b57312d72c7b
test the_second_genesis_moves_every_derived_value ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

  Le due strade concordano su tutti e nove i valori. La strada B non hardcoda
  nulla: legge gli attesi dalla tabella di README.md. **Aggiornato dalla
  remediation di [REVIEW-028] RF-001:** alla consegna i quattro valori della
  seconda genesi erano solo stampati e l'accordo fra le due strade su di essi
  non era asserito da nulla. Ora sono pubblicati come `GEN-1`, ed entrambe le
  strade si confrontano con il documento su tutti e nove i valori invece che su
  cinque.

$ cargo test --all-features --test preimage_context   # GATE-WRONG-CONTEXT-REJECTED
running 5 tests
test a_preimage_binds_the_domain_and_chain_it_was_built_for ... ok
test only_the_matching_context_is_accepted ... ok
test block_vote_preimage_binds_the_block_vote_domain ... ok
test a_raw_non_consensus_preimage_binds_nothing ... ok
test the_consensus_verifier_rejects_a_wrong_context_before_any_curve_arithmetic ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

  E la stessa gate osservata fallire, con `binds` reso sempre `true`:

thread 'only_the_matching_context_is_accepted' panicked at preimage_context.rs:93:
assertion `left == right` failed: built for coblox-block-vote-v0/sha256:0101...01
  and offered as coblox-block-vote-v0/sha256:0202...02
  left: true
 right: false
thread 'block_vote_preimage_binds_the_block_vote_domain' panicked at :121:
assertion failed: !preimage.binds(Domain::SIG_BLOCK_VOTE, &other)
thread 'a_raw_non_consensus_preimage_binds_nothing' panicked at :139:
assertion failed: !raw.binds(domain, candidate)
test result: FAILED. 2 passed; 3 failed
  (albero ripristinato subito dopo; la suite torna a 5 passed)

$ cargo test --all-features --test speccheck_conformance   # GATE-VERIFIER-UNCHANGED
running 11 tests
test small_order_public_keys_are_strictly_rejected ... ok
test upstream_cases_file_matches_its_recorded_digest ... ok
test verifier_respects_signing_preimage_contract ... ok
test fixture_expectations_agree_with_the_published_table ... ok
test extension_fixture_expectations_agree_with_the_published_table ... ok
test derived_fixture_matches_upstream_cases_byte_for_byte ... ok
test gate_cofactor_differential_verification ... ok
test original_encodings_hash_differential ... ok
test gate_speccheck_extension_table_conformance_vector_by_vector ... ok
test gate_speccheck_table_conformance_vector_by_vector ... ok
test strict_y_decoding_agrees_on_the_twelve_and_diverges_on_the_extension ... ok
test result: ok. 11 passed; 0 failed

$ python sim/tools/ed25519_speccheck_oracle.py     # l'oracolo indipendente
 0..11  published/oracle  MATCH  (dodici righe, nessuna divergenza)
 0..6   extension         MATCH  (sette righe, nessuna divergenza)
decoder divergence: upstream 0-11 : 0 disagreement(s)
                    extension 0-6 : 4 disagreement(s) at [0, 1, 2, 3]
                    fully strict RFC 8032 decoder vs Coblox on the twelve: [9]
independent oracle: PASS

$ python sim/tools/ed25519_coblox_extension_vectors.py
core/coblox-core/tests/fixtures/ed25519_coblox_extension.json: reproduces byte for byte

$ git diff core/coblox-core/src/verifier.rs --stat
 core/coblox-core/src/verifier.rs | 32 ++++++++++
  Trentadue righe, tutte aggiunte, tutte sopra `ConsensusVerifier`. Nessuna riga
  di `verify_consensus_ed25519` e cambiata: il diff non ha alcuna riga di
  rimozione.

$ python sim/tools/published_artifacts.py          # GATE-ADR012
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     18 candidate(s) checked
  C4-VALUE          56 candidate(s) checked
  C5-MIRROR         49 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C5-DISCOVERED     63 candidate(s) checked
  C10-PROBE        111 candidate(s) checked
  C11-CLAIMDOC       8 candidate(s) checked
published-artifact inventory: PASS

$ python sim/tools/published_artifacts_negative.py
=== C10-PROBE, every probe individually ===
deleting each probe's own pinned passage from its own document, 111 case(s)
  every one of the 110 probes was observed failing
negative proof: PASS - 15 mutations across 11 defect classes, plus every probe
  individually, each observed failing

  111 probe, cioe le 103 di ieri piu le 8 nuove di questa spec, ciascuna
  osservata fallire da sola. (La riga di riepilogo dello strumento stampa "110":
  e un conteggio suo, non il numero di casi eseguiti, che e 111 nella riga
  sopra. Segnalato al Lead, non corretto qui: e un difetto di stampa di uno
  strumento che questa spec non ha mandato di toccare.)

$ python sim/tools/protocol_hashes.py      # gli altri strumenti versionati
every published value reproduced: PASS
$ python sim/tools/reward_rules.py
cases: 58, mismatches: 0 / GATE-RULES-REJECT: PASS
$ python sim/tools/non_consensus_containment.py ; ... --negative
ok  `from_raw_bytes_non_consensus` is named only in registry.rs and tests/
Every defect class is reachable and the guard names it.
$ python sim/tools/lead_claims_check.py
lead-claims: PASS
$ python sim/tools/threat_model_matrix_coherence.py
celle: 104  coperte: 97  n/a: 7  scenari: 43 / OK

$ cargo test --workspace --all-features      # prima: 151
TOTAL PASSED: 163
$ cargo clippy --workspace --all-targets --all-features -- -D warnings
    Finished
$ cargo fmt --all -- --check
FMT OK
$ cargo build --locked --workspace
    Finished
```

### Deviations from the specification

**1. Nessuna preimmagine gia pubblicata e cambiata, e questo era il caso in cui
la spec chiedeva di fermarsi.** Il segnaposto a 32 byte zero rende la regola
compatibile con ogni valore gia in albero: `protocol_hashes.py` riproduce tutti
i valori preesistenti e la passata di [ADR-012] e verde senza toccarne alcuno.
Non c'e quindi nulla da fermare.

**2. `election_epoch` non e derivabile in un solo modo, e non l'ho chiuso.** E
la voce aperta del censimento e la trovo il ritrovamento piu importante di
questa passata. `election_boundary_height(e) = e * L` con `L =
election_epoch_blocks` **dai parametri di consenso attivi**, e il documento non
dice attivi a quale altezza ne quale documento valga per un'epoca passata. `L` e
governato: se passa da 100 a 200, l'altezza 5000 e epoca 50 sotto un documento e
epoca 25 sotto l'altro, e `election_epoch` entra in `election_entropy`,
`election_seed` ed `election_ticket`. Il passo 2 del light client darebbe
verdetti opposti sullo stesso set. [SPEC-016] ha chiuso la stessa forma per
`reward_epoch` **nominando il documento** — *il `reward_policy` che il mint nomina
attraverso il proprio `policy_hash`* — e nessun oggetto dell'elezione porta la
cucitura equivalente. Chiuderlo significa scegliere fra rinumerare le epoche,
vietare il cambio di `L` fuori da un confine, o far portare
all'`ElectionRecord` il `consensus_parameters_hash` sotto cui e derivato: tre
regole di validita nuove sulle regole di elezione, cioe una decisione del Lead
con la propria passata. **Raccomando un debito proprio.**

**3. Tre clausole normative aggiunte oltre alla regola principale**, perche
senza di esse la regola non determina `chain_id` e il criterio di accettazione
sarebbe falso: `previous_block_id` di genesi, l'unicita del `network_id`, e la
scelta esplicita del documento `consensus_parameters` che l'intestazione nomina.
La seconda e stata verificata contro tutti gli artefatti pubblicati per
enumerazione — tre soli nomi di rete nei documenti (`coblox-devnet-0` 15 volte,
`fixture`, `genesis-fixture`), nessun oggetto che ne mescoli due — e non per
assunzione.

**4. La clausola `key_binding_signature` non ha alcuna fixture**, ed e detto nel
documento con la ragione: pubblicare un `ValidatorSet` di genesi significherebbe
pubblicare una coorte che il blocco di vincoli governa per dimensione, stagger e
limite di mandato, e una coorte-fixture che li soddisfacesse tutti e un
artefatto piu grande di quello che serve qui — con il rischio di famiglia 1 che
[ADR-012] conta sette volte. E famiglia 4 di `recurring-defects.md` dichiarata
invece che nascosta.

**5. La forma del legame di contesto e a runtime**, quindi il criterio *«se la
forma scelta e di compilazione, il caso sbagliato non deve compilare»* non si
applica; `GATE-WRONG-CONTEXT-REJECTED` e soddisfatta nel ramo *«e rifiutata»*.
La ragione della scelta e l'ergonomia dei chiamanti futuri ed e scritta accanto
al tipo: un parametro di tipo sul dominio sposta a compilazione **meta** del
controllo — `chain_id` e un valore, non un tipo — rende `SignatureVerifier`
generico e quindi non piu `dyn`, e obbliga chiunque tenga preimmagini di domini
diversi in una collezione a introdurre un enum. Renderebbe scomodo il caso
corretto, che e esattamente il modo di sbagliare che la spec nomina.

**6. Un difetto di stampa in `published_artifacts_negative.py`**: la riga di
riepilogo dice «every one of the 110 probes» mentre i casi eseguiti sono 111.
Non toccato: e uno strumento fuori dal mandato di questa spec e correggerlo
significherebbe rieseguire la sua prova in negativo per una stringa.

## Remediation di [REVIEW-028] — RF-001

**Accolto senza obiezioni, e il costo che la review si chiedeva se avessi visto
non c'e.** `GEN-1` e `GEN-0` con **un solo campo diverso** — `network_id` a
`genesis-fixture-b`, 17 byte invece di 15 — e non introduce alcuna forma che
`GEN-0` non mostri gia. Il rischio di famiglia 1 che [ADR-012] conta sette volte
nasce da un artefatto che **insegna** una configurazione inammissibile; qui la
configurazione e la stessa, sostituita in una stringa. Il file terzo non
pubblicato sarebbe stato la scelta peggiore fra le due: un oracolo fuori dal
registro e un oracolo che nessuna implementazione indipendente riceve.

**La diagnosi era giusta e la registro come tale.** Il test vecchio dimostrava
che i valori *si muovono*, non che si muovono *insieme*: se una delle due strade
fosse derivata domani, nessun test sarebbe fallito. Era la forma che
[SKILL-001] intercetta, un livello piu in dentro di dove l'avevo cercata — la
gate della **varianza** era un calcolo, mentre quella della **derivazione** era
una guardia.

### Come ho fatto convergere le due strade

Non facendole incontrare fra loro, ma dando a entrambe lo stesso terzo. Quattro
valori di `GEN-1` — `consensus_parameters_hash`, `block_id`, `chain_id`,
`dht_namespace_key` — sono ora righe della tabella di `README.md`, e **ciascuna
strada si confronta con il documento**, mai con l'altra. `genesis_chain_id.py`
li legge dalla tabella con lo stesso estrattore che gia usava per `GEN-0`;
`tests/genesis_derivation.rs` li trascrive dalla tabella come ogni altro
`EXPECTED_*` del file, sotto la regola di provenienza che quel file dichiara in
testa.

Per non spuntare un accordo che non avevo osservato, ho pubblicato le righe
**a zero** e ho fatto fallire prima una strada e poi l'altra contro il documento
sbagliato, guardando quale valore ciascuna nominasse. Le due hanno nominato lo
stesso, separatamente, prima che il documento lo contenesse.

`gen1_moves_every_derived_value_away_from_gen0` resta come asserzione **propria**
e separata: due tabelle di digest che per caso coincidessero soddisferebbero il
confronto con il documento e non direbbero nulla sul nome di rete che entra
nella derivazione. E la ragione per cui `GEN-1` esiste, ed e ora asserita invece
che implicita.

`README.md` dice accanto a `GEN-1` perche non e un doppione, e una probe C10
nuova lo tiene li: una fixture che differisce per un campo verra cancellata dal
primo lettore che riordina, e con essa l'unico caso che esercita il prefisso di
lunghezza.

### Trascrizione

```text
# 1. le righe pubblicate a zero, e ciascuna strada che le contraddice da sola

$ python sim/tools/genesis_chain_id.py        # strada B, contro il documento a zero
3. GEN-1, the same genesis on a network name of a different length
  FAIL  consensus_parameters_hash / GEN-1 document
          computed  sha256:6ba582b42339763c4b79e7a41ff7d75f6283800a5a4b4d97176f318cb5f63c0d
          published sha256:0000000000000000000000000000000000000000000000000000000000000000
  FAIL  block_id / GEN-1 header
          computed  sha256:6b62539240dcbc9aedf3e47e32edef91d302cf0687865dad8904326d8f49c53d
  FAIL  chain_id / GEN-1
          computed  sha256:172fd2e8bbdffefecc8952c1e0b97b69275af0de9bc637c6735a09b872d5e033
  FAIL  dht_namespace_key / GEN-1
          computed  sha256:e8ceaa4c9095078ae2347bb111484ed532e5c494e49341aba2f5b57312d72c7b

$ cargo test --all-features --test genesis_derivation   # strada A, stesso documento a zero
test gen1_derives_the_published_genesis_values ... FAILED
thread 'gen1_derives_the_published_genesis_values' panicked at genesis_derivation.rs:180:
assertion `left == right` failed
  left: Digest32([107, 165, 130, 180, 35, 57, 118, 60, 75, 121, 231, 164, 31, 247,
                  215, 95, 98, 131, 128, 10, 90, 75, 77, 151, 23, 111, 49, 140,
                  181, 246, 60, 13])
 right: Digest32([0, 0, 0, ... 0])
test result: FAILED. 7 passed; 1 failed

  107,165,130,180,... e 0x6b,0xa5,0x82,0xb4,... cioe
  6ba582b42339763c4b79e7a41ff7d75f6283800a5a4b4d97176f318cb5f63c0d: lo stesso
  valore che la strada B ha nominato, prodotto senza che il documento lo
  contenesse e senza che nessuna delle due potesse copiarlo dall'altra.

# 2. le righe riempite, ed entrambe le strade verdi contro il documento

$ python sim/tools/genesis_chain_id.py
3. GEN-1, the same genesis on a network name of a different length
  ok    consensus_parameters_hash / GEN-1 document
          computed  sha256:6ba582b42339763c4b79e7a41ff7d75f6283800a5a4b4d97176f318cb5f63c0d
  ok    block_id / GEN-1 header
          computed  sha256:6b62539240dcbc9aedf3e47e32edef91d302cf0687865dad8904326d8f49c53d
  ok    chain_id / GEN-1
          computed  sha256:172fd2e8bbdffefecc8952c1e0b97b69275af0de9bc637c6735a09b872d5e033
  ok    dht_namespace_key / GEN-1
          computed  sha256:e8ceaa4c9095078ae2347bb111484ed532e5c494e49341aba2f5b57312d72c7b
4. every clause, watched failing
  ok    network_id enters the header, so genesis_block_id moves with it
  ok    network_id enters chain_id twice over, so chain_id moves too
  ok    dropping u32be(len(network_id_utf8)) changes chain_id
  ok    a placeholder of 32 ff bytes changes genesis_block_id
  ok    and changes chain_id with it
ok

$ cargo test --all-features --test genesis_derivation
running 8 tests
test the_empty_transactions_root_is_the_one_gen0_publishes ... ok
test a_different_placeholder_moves_the_genesis_block_id_and_the_chain_id ... ok
test the_placeholder_is_thirty_two_zero_bytes ... ok
test gen0_derives_the_published_genesis_values ... ok
test the_method_reproduces_a_value_this_pass_did_not_change ... ok
test gen1_moves_every_derived_value_away_from_gen0 ... ok
test dropping_the_network_length_prefix_moves_the_chain_id ... ok
test gen1_derives_the_published_genesis_values ... ok
test result: ok. 8 passed; 0 failed

# 3. GATE-ADR012 rieseguita: GEN-1 entra nell'inventario e nella prova in negativo

$ python sim/tools/published_artifacts.py
  C3-FIXTURE-ID     19 candidate(s) checked      (era 18: GEN-1)
  C4-VALUE          60 candidate(s) checked      (era 56: i quattro di GEN-1)
  C5-MIRROR         53 candidate(s) checked      (era 49)
  C5-DISCOVERED     67 candidate(s) checked      (era 63)
  C10-PROBE        112 candidate(s) checked      (era 111)
published-artifact inventory: PASS

$ python sim/tools/published_artifacts_negative.py
=== C10-PROBE, every probe individually ===
deleting each probe's own pinned passage from its own document, 112 case(s)
negative proof: PASS - 15 mutations across 11 defect classes, plus every probe
  individually, each observed failing

  La probe nuova, `gen1-exists-to-vary-the-network-length`, e nel conteggio ed e
  osservata fallire da sola come le altre 111.

# 4. nulla si e mosso altrove

$ python sim/tools/protocol_hashes.py
every published value reproduced: PASS
$ cargo test --all-features --test speccheck_conformance
test result: ok. 11 passed; 0 failed
$ python sim/tools/ed25519_speccheck_oracle.py
independent oracle: PASS
$ git diff --numstat core/coblox-core/src/verifier.rs
32      0       core/coblox-core/src/verifier.rs
$ cargo test --workspace --all-features
TOTAL PASSED: 164          (151 alla baseline, 163 alla consegna, +1 qui)
$ cargo clippy --workspace --all-targets --all-features -- -D warnings ; cargo fmt --all -- --check
    Finished / FMT OK
$ cargo build --locked --workspace
    Finished
```

### File toccati dalla remediation

- `docs/protocol/README.md` — la definizione di `GEN-1` con la ragione per cui
  non e un doppione, e quattro righe di tabella.
- `core/coblox-core/tests/genesis_derivation.rs` — `gen1_derives_the_published_genesis_values`
  (nuovo, asserisce il documento) e `gen1_moves_every_derived_value_away_from_gen0`
  (l'`assert_ne!` conservato come asserzione propria) al posto di
  `the_second_genesis_moves_every_derived_value`, che stampava.
- `sim/tools/genesis_chain_id.py` — la sezione 3 legge le quattro righe di
  `GEN-1` dalla tabella e le confronta invece di stamparle.
- `sim/tools/published_artifacts.toml` — 1 fixture, 4 valori, 1 probe.

## Remediation di [REVIEW-029] — GATE-SECREVIEW

Tre `medium` e tre `low`. Chiusi RF-002, RF-003, RF-004, RF-005, RF-006; RF-001
nominato e non chiuso, come il Lead ha deciso.

### RF-002 — chiuso col rimedio 3, e prima ho verificato la condizione di stop

**Nessun valore pubblicato si muove**, quindi il caso in cui dovevo fermarmi non
si presenta. Verificato prima di toccare il codice: `key_binding_signature` non
compare in alcuna riga `[[value]]` del manifesto, `validator_set_hash` e
`coverage = "uncovered"` perche nessun `ValidatorSet` canonico e pubblicato, e
`GEN-0`/`GEN-1` portano i due hash di set come letterali dichiarati `dd`. Dopo
la modifica `protocol_hashes.py` riproduce tutto e la passata di [ADR-012] e
verde: nulla si e mosso.

`network_id` entra nell'oggetto del `key_binding_signature`, che diventa
`{activation_height, consensus_public_key, network_id, node_id, validator_id}`.
**A ogni altezza e non solo alla genesi**, per la stessa ragione con cui il
documento respinge il segnaposto derivato: una forma che cambia a un'altezza e
una forma da sbagliare. Sopra la genesi e ridondante con `chain_id_32` e
innocua. `network_id` **non** e un campo del `ValidatorSet`: un verificatore lo
prende dalla stessa ancora di fiducia da cui prende `chain_id`.

**Cio che il rimedio non compra, ed e un soffitto e non un'omissione.** Prima
che `genesis_block_id` esista, l'unica grandezza distintiva disponibile e un
nome scelto dall'operatore: ogni altro candidato sarebbe o il `chain_id` che si
sta derivando, o un secondo nome. L'attribuzione dentro la finestra e quindi al
livello del **nome di rete**, e `README.md` dichiara che l'unicita di
`network_id` e una convenzione operativa e non un controllo di replay. Due
catene che condividono un `network_id` condividono ogni payload di genesi. E il
residuo, ed e ora scritto nel documento — cioe ho fatto anche il rimedio 1, non
al posto del 3 ma insieme.

Ho corretto anche la nota di corredo: la doc di `GENESIS_PLACEHOLDER` diceva
*«a constant and not a `ChainId` a caller may keep»* mentre il tipo e `Copy` e
la costante e `pub`. Era prosa smentita dal tipo, la forma che [REVIEW-023]
aveva gia trovato una volta su questo tipo. Ora la doc dice cio che regge
davvero, e dove verificarlo.

### RF-003 — riscritto, e il rifiuto **rimotivato** contro il perimetro vero

La premessa era falsa nei due versi e la correzione e **registrata invece che
fatta in silenzio**, perche e il paragrafo che dichiara un residuo di sicurezza.
Il testo nuovo dice la condizione giusta — una firma di genesi si replica quando
il **payload firmato** coincide, e nulla del materiale circostante entra nella
condizione — e la sostiene con l'**enumerazione dei dodici domini di firma**:
ciascuno o non e mai materiale di genesi, o porta byte che distinguono la rete
dentro il proprio payload.

**Il rifiuto del segnaposto derivato regge, contro il perimetro corretto, e la
ragione e cambiata.** Prima diceva *«cio che comprerebbe e limitato dal
paragrafo sopra»*, cioe si appoggiava alla premessa falsa. Ora: il segnaposto
derivato legherebbe ogni payload di genesi al nome di rete, e l'enumerazione
mostra che ogni payload di genesi porta gia il nome di rete nei propri byte —
**comprerebbe lo stesso soffitto due volte**. Contro questo aggiungerebbe una
seconda grafia di *non c'e ancora tale valore* dentro un oggetto solo, accanto a
`previous_block_id`, e il costo di quella seconda grafia e un'implementazione
che deriva il segnaposto correttamente per l'intestazione e sbagliato per il
set. Un legame che gia esiste non vale un secondo modo di sbagliarlo.

### RF-005 — chiuso con la regola, non con la clausola

`README.md` e `ledger.md` dicono ora che **il blocco di altezza 0 non porta
transazioni**, quindi `transactions_root` e `H(0x03)`. Ho scelto la regola e non
il caveat perche il caveat lascerebbe due implementazioni conformi con due
`chain_id`: e la scelta che entrambe le fixture gia facevano, promossa da
convenzione di fixture a clausola. AGENT-007 aveva gia verificato che `mint` e
`validator_candidacy` sono impossibili a `h = 0`; una regola che vale per tre
generi su cinque non e una regola.

### RF-004 — la clausola ora esiste nel codice e in un test

Doc estesa su `consensus_key_binding_preimage`: nomina il segnaposto, la sezione
normativa, e **la conseguenza dell'errore** — non un digest sbagliato che una
suite nota, ma un `genesis_block_id` diverso, cioe [DEBT-020] che si riapre
invisibilmente. Piu un test in memoria che non pubblica nulla, e la sua prova in
negativo riproduce esattamente il finding di AGENT-007.

### RF-006 — criterio e perimetro

**(a)** La classe T porta ora la condizione che la rende vera: *un valore
trasmesso e univoco solo se una regola di validita lo lega a una grandezza
derivabile e nomina il documento che ne fissa il denominatore; altrimenti e A.*
E la domanda che avevo applicato senza enunciarla, ed e cio che separa
`reward_epoch` da `election_epoch`.

**(b)** Il perimetro e ora **51 preimmagini di hash piu 12 di firma**, con una
tabella che censisce i dodici domini uno per uno. La colonna che conta e *byte
che distinguono la rete*: `coblox-consensus-key-binding-v0` era l'unico vuoto.

### RF-001 — nominato, non chiuso

Come deciso. `verify_in_context` dichiara ora che usarlo e una **convenzione e
non un confine**, nomina i due percorsi pubblici che la saltano
(`SignatureVerifier::verify` e `verify_consensus_ed25519` alla radice del
crate), dice che e la forma di [REVIEW-022], e dice perche non e chiusa qui.

### Trascrizione

```text
# RF-002, la prova in negativo: senza network_id i due payload coincidono
$ cargo test --all-features --test genesis_derivation a_genesis_key_binding
thread '...' panicked at genesis_derivation.rs:280:
assertion `left != right` failed: two networks must not share a genesis
  key-binding payload
  left:  [99,111,98,108,111,120,45,99,111,110,115,101,110,115,117,115,45,107,
          101,121,45,98,105,110,100,105,110,103,45,118,48, 0, 0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 123,34,97,99,116,...]
  right: [99,111,98,108,111,120,45,99,111,110,115,101,110,115,117,115,45,107,
          101,121,45,98,105,110,100,105,110,103,45,118,48, 0, 0,0,0,0,0,0,0,0,0,
          0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 123,34,97,99,116,...]
test result: FAILED. 0 passed; 1 failed

  I 32 byte zero del segnaposto sono visibili in mezzo, ed e per quelli che le
  due reti coincidevano. E il finding di [REVIEW-029] RF-002 riprodotto in
  albero. Ripristinato subito; il test torna verde.

$ cargo test --all-features --test genesis_derivation
running 9 tests
test a_genesis_key_binding_is_placeholder_bound_and_moves_with_the_network ... ok
test gen0_derives_the_published_genesis_values ... ok
test gen1_derives_the_published_genesis_values ... ok
test gen1_moves_every_derived_value_away_from_gen0 ... ok
test a_different_placeholder_moves_the_genesis_block_id_and_the_chain_id ... ok
test dropping_the_network_length_prefix_moves_the_chain_id ... ok
test the_empty_transactions_root_is_the_one_gen0_publishes ... ok
test the_placeholder_is_thirty_two_zero_bytes ... ok
test the_method_reproduces_a_value_this_pass_did_not_change ... ok
test result: ok. 9 passed; 0 failed

# la condizione di stop di RF-002, verificata prima di toccare il codice
$ grep -n "key_binding" sim/tools/published_artifacts.toml
2318:pattern = ... (una probe, non un valore)
2332:pattern = ... (una probe, non un valore)
  Nessuna riga [[value]]. Nessun digest pubblicato dipende da quella preimmagine.

$ python sim/tools/protocol_hashes.py     # dopo la modifica
every published value reproduced: PASS

# GATE-ADR012 rieseguita
$ python sim/tools/published_artifacts.py
  C10-PROBE        116 candidate(s) checked      (era 112: 4 nuove)
published-artifact inventory: PASS

$ python sim/tools/published_artifacts_negative.py
deleting each probe's own pinned passage from its own document, 116 case(s)
negative proof: PASS - 15 mutations across 11 defect classes, plus every probe
  individually, each observed failing

# GATE-TWO-DERIVATIONS e GATE-VERIFIER-UNCHANGED, invariate
$ python sim/tools/genesis_chain_id.py                       -> ok
$ cargo test --all-features --test speccheck_conformance     -> 11 passed
$ python sim/tools/ed25519_speccheck_oracle.py               -> PASS
$ git diff --numstat core/coblox-core/src/verifier.rs
55      0       core/coblox-core/src/verifier.rs   (solo aggiunte, ancora zero rimozioni)

# il lint di contenimento mi ha colto, e non l'ho allargato
$ python sim/tools/non_consensus_containment.py
N1-CALL-SITE: core/coblox-core/src/verifier.rs:83 names
  `from_raw_bytes_non_consensus`. ...
1 finding(s).
  La prima stesura del commento di RF-001 nominava il costruttore gemello.
  Riformulata la prosa invece di allargare la guardia: allentare un recinto che
  funziona per far stare un paragrafo piu bello e lo scambio sbagliato. Il fatto
  che la frase non possa nominarlo e ora scritto nel commento come la
  dimostrazione stessa.
$ python sim/tools/non_consensus_containment.py ; --negative
ok  ... named only in registry.rs and under tests/
Every defect class is reachable and the guard names it.

$ cargo test --workspace --all-features
TOTAL PASSED: 165        (151 baseline, 163 consegna, 164 REVIEW-028, 165 qui)
$ cargo clippy --workspace --all-targets --all-features -- -D warnings ; cargo fmt --all -- --check
    Finished / FMT OK
$ cargo build --locked --workspace
    Finished
$ python sim/tools/reward_rules.py ; lead_claims_check.py ; threat_model_matrix_coherence.py
GATE-RULES-REJECT: PASS / lead-claims: PASS / OK: matrice e scenari coerenti
```

### Un difetto mio, trovato e corretto in questa passata

Riscrivendo il paragrafo di RF-003 ho introdotto un **byte NUL** in
`docs/protocol/README.md`, dove il testo cita `"coblox-chain-id-v0\0"`: una
sequenza di escape interpretata invece che scritta. L'ho visto perche `grep`
ha risposto *«Binary file matches»* invece di stampare la riga. Corretto, e ho
poi verificato **ogni** file toccato da questa spec: zero byte NUL. Lo riporto
perche il difetto non sarebbe stato visto da alcuna gate — `published_artifacts.py`
legge il file come testo e le probe hanno continuato a corrispondere — ed e
esattamente la classe che questo progetto conta: il difetto era in albero e
nessuno lo stava guardando.

### File toccati dalla remediation

- `docs/protocol/README.md` — RF-003 riscritto con l'enumerazione dei dodici
  domini e il rifiuto rimotivato; RF-005 la regola sul blocco di altezza 0; le
  due affermazioni di corredo aggiornate; il NUL rimosso.
- `docs/protocol/ledger.md` — l'oggetto del key binding con `network_id` e la
  sua ragione; la regola sull'altezza 0; l'affermazione di corredo.
- `core/coblox-core/src/validator_set.rs` — `network_id` nell'oggetto e nella
  preimmagine, e la doc della clausola di genesi (RF-002, RF-004).
- `core/coblox-core/src/hash.rs` — la prosa smentita dal tipo, corretta.
- `core/coblox-core/src/verifier.rs` — la convenzione non imposta, nominata
  (RF-001).
- `core/coblox-core/tests/genesis_derivation.rs` — il test della clausola.
- `sim/tools/published_artifacts.toml` — 4 probe nuove, 2 ripuntate, e la
  riparazione di quattro pattern miei che erano sovra-scappati (`\\*` invece di
  `\*`): pinnavano il testo, ma non richiedevano davvero i marcatori di
  grassetto che dichiaravano.
- `.lmbrain/knowledge/derivazioni-non-univoche.md` — RF-006 (a) e (b).

### Handoff status
- [x] Ready for Project Lead review
