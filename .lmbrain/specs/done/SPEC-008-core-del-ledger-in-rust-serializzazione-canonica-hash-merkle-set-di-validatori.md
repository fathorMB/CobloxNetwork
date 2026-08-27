---
id: SPEC-008
# Note: Quote the title if it contains a colon
title: "Core del ledger in Rust: serializzazione canonica, hash, Merkle, set di validatori"
status: done
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
updated: 2026-08-27
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
  - date: 2026-08-25
    action: "transitioned ready -> working"
  - date: 2026-08-25
    action: "transitioned working -> review"
  - date: 2026-08-25
    action: "attested verification GATE-LEAD-REPRO by lead"
  - date: 2026-08-25
    action: "transitioned review -> done"
  - date: 2026-08-27
    action: "attested verification GATE-CI-GREEN by lead"
mutation_overrides:
  - schema_version: "1"
    id: "SPEC-008-OVERRIDE-001"
    actor_role: "implementation-specialist"
    timestamp: "2026-08-25T15:54:16.108466500+02:00"
    from: "working"
    to: "review"
    reason: "GATE-CI-GREEN resta non spuntato di proposito e non per deroga. Il gate chiede la pipeline verde \"sul commit consegnato\", mentre il mandato di questa spec vieta esplicitamente qualunque commit e push: non esiste un commit consegnato, quindi la pipeline non puo' avere girato e spuntare il gate sarebbe un'affermazione falsa — esattamente il costo che DEBT-001 ha gia' pagato una volta. In sostituzione ho eseguito localmente ogni comando dei cinque job che questa macchina puo' eseguire, con trascrizione reale nella evidence: tutti e cinque i passi del job `rust` (build --locked, test --locked, fmt --check, clippy -D warnings, cargo deny check: advisories/bans/licenses/sources ok), la compilazione di coblox-ffi per aarch64-linux-android del job `android`, e del job `desktop` fmt, clippy -D warnings, cargo deny check sul manifesto separato di src-tauri e il test core_version. Non eseguibili qui: il link della .so Android (serve l'NDK 28.2), npm ci/build e tauri build, e la matrice Linux (macchina Windows). Il gate va soddisfatto dal Lead sulla pipeline reale dopo il commit, oppure riformulato per un agente a cui e' vietato committare. GATE-FIXTURES e' spuntato con la copertura completa delle 16 righe del registro e le fixture non coperte dichiarate con la ragione."
    unmet_invariant: "before-submit verification blocked: GATE-CI-GREEN (owner=agent): checklist item is unchecked"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "e9b364fa92f2fbac380ef863c929c5e39a253eef44c001af49e6f3ae8142777d"
    evidence_ref: "Il Lead ha rieseguito cargo test --locked --workspace: 103 test verdi su sette file, 26 nel crate piu 77 nei test di integrazione. Provenienza degli attesi verificata per campione su cinque valori che il Lead aveva ricalcolato in modo indipendente durante SPEC-006 e che quindi conosce come corretti: policy_hash fbc7493a, consensus_parameters_hash 840dd6a9, parameter_set_hash a2553f36, candidate_root 42e4f6b1 ed election_seed 9e2aa262. Tutti e cinque compaiono come costanti letterali nei file di test e coincidono con i valori pubblicati nei documenti. Il file conformance_registry.rs dichiara in testa la propria regola di provenienza e nomina esplicitamente la modalita di fallimento che la spec aveva previsto, cioe il test che genera l'attesa dall'implementazione e poi la confronta con se stessa."
    id: "SPEC-008-ATTEST-001"
    requirement_digest: "4fce5c195d25ac1c8e30e6e8572835b852cf4113e73496440e412fe05e898dc5"
    requirement_id: "GATE-LEAD-REPRO"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T16:06:49.945880600+02:00"
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "4f061328d0d6510069082d901712aaf44ab94864d56427cbc43d4a34a4aad153"
    evidence_ref: "Soddisfatta, non derogata. Run 33051858034 sul commit 43801e9 di main, che contiene il codice di questa spec: sei job su sei verdi (Rust windows/ubuntu, Tauri desktop windows/ubuntu, Android arm64 + Kotlin bindings, Protocol document guards). Sei e non cinque perche' Protocol document guards e' successivo alla consegna. Attestata dal Lead: la gate diceva owner=agent | before-submit e chiedeva la pipeline verde sul commit consegnato, ma il mandato della spec vietava ogni commit, quindi era insoddisfacibile per costruzione. Riformulata a owner=lead | before-done, la forma che SPEC-002 usa gia' per questa gate. DEBT-001, citato nel testo, riguarda SPEC-002 ed e' resolved."
    id: "SPEC-008-ATTEST-002"
    requirement_digest: "924ed20f5c5a9f6df84640964ff33246225b76e1cce75cfc309a873171d6cc9f"
    requirement_id: "GATE-CI-GREEN"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-27T10:02:05.542059900+02:00"
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
- [x] **Ogni valore del registro di conformità di `README.md` è riprodotto da un test**, con il valore atteso preso dal documento e non dall'implementazione.
- [x] L'esempio numerico dell'elezione dell'epoca 3 è riprodotto per intero come test: foglie, foglia vuota, nodi interni, `candidate_root`, entropia, seme, biglietti, ordinamento e insediamento.
- [x] La serializzazione canonica è verificata **in entrambe le direzioni**: i byte prodotti sono canonici, e i byte non canonici sono rifiutati e non normalizzati in silenzio.
- [x] La derivazione dell'elezione è deterministica e testata sui casi degeneri che la specifica elenca: eleggibili insufficienti, parità, coorte intera in scadenza, interazione con la revoca.
- [x] Il pavimento di contrazione, il tetto di riempimento e i timbri di scadenza sono implementati e testati, inclusi i due arresti che [SPEC-006] ha scoperto: genesi sincronizzata e limite di mandato decrescente.
- [x] Il blocco di vincoli è validato per intero, e il test dimostra che `T <= 3` è rifiutato.
- [x] I controlli normativi del light client sono funzioni pure testate, e **nessun controllo della lista delle incapacità è implementato come se fosse una capacità**.
- [x] I parametri sono input di configurazione validati, non costanti compilate.
- [x] La suite valida i propri fixture di parametri contro il blocco di vincoli prima di usarli.
- [x] `cargo build`, `cargo test`, `cargo fmt --check`, `clippy -D warnings` e `cargo-deny` passano; `unsafe_code = "forbid"` resta attivo.
- [x] Nessun file in `docs/protocol/` è stato modificato.

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
- [x] GATE-FIXTURES | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Un test riproduce **ogni** valore del registro di conformità, con l'atteso citato dal documento. Incollare l'esecuzione reale e il conteggio. Una fixture non coperta va dichiarata con la ragione, non omessa in silenzio.
- [x] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=transcript | La pipeline è verde su tutti e cinque i job sul commit consegnato, con `clippy -D warnings` e `cargo-deny` eseguiti. Il progetto ha già pagato una volta il prezzo di un gate di CI derogato ([DEBT-001]): qui non si deroga.
- [x] GATE-LEAD-REPRO | kind=manual | owner=lead | phase=before-done | evidence=transcript | Il Lead riesegue la suite e verifica per campione che gli attesi provengano dal documento e non dall'implementazione. Un test che confronta il codice con sé stesso passa sempre ed è la modalità di fallimento specifica di questa spec.

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

### Changes made

`coblox-core` passa da 15 righe (una funzione che restituisce la versione) a
uno strato deterministico completo del protocollo v0, con 8.097 righe fra
sorgenti e test e 103 test nel crate.

**Struttura scelta, e perché.** Dodici moduli che seguono la forma di
`docs/protocol/` dal basso verso l'alto, perché quello è l'ordine delle
dipendenze *e* l'ordine in cui un revisore con la specifica in mano cerca le
regole. La tabella di corrispondenza modulo/sezione è nella documentazione di
crate ed è parte della superficie pubblica.

| Modulo | Sezione di specifica |
| --- | --- |
| `encoding` | codifiche di stringhe di byte (base64url, base32, `u64`, hex) |
| `json` | rappresentazione comune, JCS |
| `hash` | identificatori e convenzioni crittografiche, separazione di dominio |
| `registry` | registro delle preimmagini di hash |
| `merkle` | primitive di hashing, i cinque alberi ordinati, stato sparso dei conti |
| `params` | documenti di protocollo firmati, `ElectionBounds`, blocco di vincoli |
| `quorum` | predicato di quorum |
| `block` | formato del blocco, continuità del set |
| `validator_set` | continuità del set, transizioni di sola rimozione |
| `election` | elezione e rotazione dei validatori |
| `light_client` | verifica del light client, composizione del set |
| `error` | tipi di errore recuperabili |

**Tre convenzioni che il resto di M-02 eredita**, tutte e tre strutturali e non
disciplinari:

1. **La canonicalizzazione è l'unica strada.** `Json` non ha varianti `Number` e
   `Null`: le restrizioni 2, 4 e 5 della rappresentazione comune non sono
   controlli a runtime ma affermazioni su quali programmi compilano. Ne discende
   che la metà "formattazione numerica ES6/ryu" di RFC 8785 è irraggiungibile e
   non è implementata. `JsonObject` tiene chiavi validate (`snake_case` ASCII) in
   una `BTreeMap`, quindi duplicati impossibili e ordine non a scelta del
   chiamante. `ObjectBuilder` emette la grafia canonica di ogni campo tipizzato.
   L'unico ingresso di decodifica, `JsonObject::parse_canonical`, riserializza e
   confronta i byte: i byte non canonici sono **rifiutati**, mai normalizzati.
2. **La separazione di dominio è strutturale.** Ogni dominio è una costante di
   `hash::Domain` e l'unico modo di iniziare una preimmagine è
   `PreimageWriter::new`, che scrive `domain || 0x00` e non può essere convinto a
   non farlo. I byte di tag dei Merkle stanno nelle costanti di
   `merkle::TaggedTree` per lo stesso motivo.
3. **I parametri sono configurazione validata.** Nel crate non compare **nessun**
   valore di lancio. La derivazione dell'elezione e i controlli del light client
   accettano solo `ValidatedConsensusParameters`, che non ha costruttore diverso
   da `ConsensusParameters::validate` contro il blocco di vincoli e i limiti di
   genesi. La validazione è un `Error` recuperabile, mai un panico.

`#![forbid(unsafe_code)]` è ora attivo a livello di crate su `coblox-core`
(vedi *Deviations*: non lo era).

Unica dipendenza aggiunta: `sha2 0.11.0` (MIT OR Apache-2.0, MSRV 1.85),
`default-features = false`.

### Files changed

Aggiunti:

- `core/coblox-core/src/{error,encoding,json,hash,registry,merkle,params,quorum,block,validator_set,election,light_client}.rs`
- `core/coblox-core/tests/common/mod.rs` — fixture di conformità
- `core/coblox-core/tests/conformance_registry.rs`
- `core/coblox-core/tests/worked_example.rs`
- `core/coblox-core/tests/canonical_serialization.rs`
- `core/coblox-core/tests/constraint_block.rs`
- `core/coblox-core/tests/election_degenerate.rs`
- `core/coblox-core/tests/sparse_account_state.rs`
- `core/coblox-core/tests/light_client_perimeter.rs`

Modificati:

- `core/coblox-core/src/lib.rs` — documentazione di crate, `#![forbid(unsafe_code)]`, moduli, `SignatureVerifier`
- `core/coblox-core/Cargo.toml` — dipendenza `sha2`
- `Cargo.lock` — 7 pacchetti aggiunti dal grafo di `sha2`

Non toccati: `docs/protocol/` (sola lettura, verificato), `core/coblox-node/`,
`core/coblox-ffi/`, `Cargo.toml` di radice, `.lmbrain/knowledge/`, il simulatore.

### Verification performed

**GATE-FIXTURES — copertura del registro di conformità.**

Le 16 righe della tabella di `README.md#hash-conformance-fixtures` sono
riprodotte una per una, ciascuna con il proprio test e con l'atteso citato come
costante letterale dal documento. Nessun atteso è calcolato dall'implementazione;
il file `tests/conformance_registry.rs` porta la regola di provenienza in testa.

| # | Riga del registro | Fixture | Test |
| --- | --- | --- | --- |
| 1 | `enrollment_request_hash` | `ER-0` | `enrollment_request_hash_over_er0` |
| 2 | `parameter_set_hash` | enrollment `PD-0` | `parameter_set_hash_over_enrollment_pd0` |
| 3 | `policy_hash` | reward `PD-0` | `policy_hash_over_reward_pd0` |
| 4 | `hosting_rate_card_hash` | hosting `PD-0` | `hosting_rate_card_hash_over_hosting_pd0` |
| 5 | `consensus_parameters_hash` | consensus `PD-0` | `consensus_parameters_hash_over_consensus_pd0` |
| 6 | `object_id` | bytes `00 01 02` | `object_id_over_the_three_byte_fixture` |
| 7 | `input_hash` | bytes `00 01 02` | `input_hash_over_the_three_byte_fixture` |
| 8 | `issuer_commitment` | `CMT-0` | `issuer_commitment_over_cmt0` |
| 9 | `challenge_randomness` | `RND-0` | `challenge_randomness_over_rnd0` |
| 10 | `request_hash` | `REQ-0` | `request_hash_over_req0` |
| 11 | `response_hash` | `RESP-0` | `response_hash_over_resp0` |
| 12 | `admission_tag` | `ADM-0` | `admission_tag_over_adm0` |
| 13 | `election_entropy` | `ELEC-0` | `election_entropy_over_elec0` |
| 14 | `election_seed` | `ELEC-0` | `election_seed_over_elec0` |
| 15 | `election_ticket` | `ELEC-0` | `election_ticket_over_elec0` |
| 16 | `weak_subjectivity_checkpoint_hash` | `WSC-0` | `weak_subjectivity_checkpoint_hash_over_wsc0` |

Valori pubblicati fuori dalla tabella ma con lo stesso statuto, anch'essi
riprodotti da test contro la costante del documento:

| Valore | Documento | Test |
| --- | --- | --- |
| `revocation_root` di lista vuota, `H(0x33)` | `README.md#weak-subjectivity-checkpoint` | `the_empty_revocation_root_is_the_published_hash_of_its_tag` |
| `REVL-0` (foglia **e** radice a un'entrata) | idem | `revl0_is_both_the_leaf_and_the_single_entry_root` |
| base64url di `RND-0` (`jOvkrY…`) | `README.md#hash-conformance-fixtures` | `challenge_randomness_over_rnd0` |
| base64url di `admission_nonce` (`iIiIiI…`) | idem | `the_admission_nonce_base64url_spelling_matches_its_bytes` |
| `node_id` ricalcolato dalla chiave della fixture | `identity.md#node-identifier` | `the_fixture_node_id_is_recomputed_from_the_fixture_key` |
| esempio numerico dell'epoca 3: 5 foglie, foglia vuota, 6 nodi interni, `candidate_root`, entropia, seme, 3 biglietti, ordinamento, `fills`, tabella finale dei 4 seggi | `ledger.md#worked-example-of-the-derivation` | `worked_example.rs` (6 test) |
| fixture di confine del predicato di quorum (100/66-67, 101/67-68, 102/68-69) | `ledger.md#quorum-predicate` | `quorum::tests::published_boundary_fixtures` |
| tabella di confine del pavimento di costo di enrollment (5 righe) | `README.md#the-enrollment-cost-floor-…` | `the_enrollment_cost_floor_boundary_fixtures` |
| confine del tetto di quota del creatore (`floor(kn·B/kd)` valido, +1 invalido) | `ledger.md#creator-share-cap-…` | `the_creator_share_cap_boundary` |
| `SMT-1` (prova con default esplicito, rifiutata) | `ledger.md#sparse-merkle-account-state` | `smt_1_rejects_an_explicitly_supplied_default_sibling` |
| 17 serializzazioni canoniche pubblicate su una riga (ledger/identity/wire) | tutti e tre i documenti | `every_published_canonical_example_round_trips_byte_for_byte` |

**Fixture non coperte, dichiarate con la ragione** (nessuna omessa in silenzio):

1. **Vettori 0–11 di `novifinancial/ed25519-speccheck`** e la tabella degli esiti
   di `README.md#consensus-critical-ed25519-verification`. *Ragione:* questo
   crate **non spedisce un verificatore di firme**, per scelta motivata (vedi
   *Deviations*, punto 1). Un verificatore senza quei vettori come oracolo
   sarebbe esattamente il comportamento sui casi limite non dimostrato che la
   specifica vieta.
2. **Peer ID libp2p canonico** `12D3KooW…` e l'hex del protobuf canonico
   `080112202ffa…` di `identity.md#canonical-libp2p-peer-id`. *Ragione:*
   richiedono protobuf deterministico, multihash e base58btc, cioè lo strato di
   identità/enrollment, fuori dall'ambito dichiarato di questa spec. La stringa
   del Peer ID è comunque **usata** come campo opaco in `ER-0`, che è coperta per
   intero.
3. **`ORDER-1`** (ordine canonico di esecuzione nel blocco) di
   `ledger.md#state-transition-order`. *Ragione:* è esecuzione di transazioni con
   stato dei conti e nonce, esclusa dall'ambito ("niente consenso BFT, niente
   storage"); non esiste macchina a stati in questa spec.
4. **`ADM-1`** (burst di richieste di enrollment senza proof of work) di
   `identity.md#the-admission-shield-…`. *Ragione:* è un test di risorse e
   latenza su uno stream di rete di un validatore; richiede libp2p, escluso.
5. **`enrollment_pow_salt`, `tx_id`, `block_id`, `message_id`,
   `validator_set_hash`, `chain_id`, `account_key`, chiave DHT.** Sono nel
   registro delle preimmagini ma **non hanno un valore atteso pubblicato**:
   `README.md` non li mette in tabella, e gli esempi canonici che li contengono
   (`message_id` dell'envelope, `challenge_id` dell'evidenza) sono su
   `coblox-devnet-0`, il cui `chain_id` non è pubblicato. Sono implementati
   secondo la formula e testati per le proprietà che *sono* verificabili
   (separazione di dominio, binding di rete e blocco di genesi per `chain_id`,
   round-trip canonico); nessun test finge di avere un oracolo che non c'è.
6. **Argon2id `pow_tag`** di `identity.md#one-time-anti-sybil-proof-of-work`.
   *Ragione:* nessun vettore pubblicato, e la primitiva Argon2id appartiene allo
   strato di enrollment, fuori ambito. Il **pavimento di costo** che la governa è
   invece implementato e testato per intero (punto 5 della tabella sopra).

**Conteggio:** 16/16 righe del registro coperte; 103 test nel crate su 8 binari
di test; 104 test nel workspace.

**Altre verifiche.** Serializzazione canonica in entrambe le direzioni (17
esempi pubblicati riprodotti byte per byte; 18 grafie non canoniche rifiutate);
derivazione dell'elezione deterministica e testata sui casi degeneri elencati
(pool corto, parità, coorte sincronizzata, limite di mandato decrescente,
interazione con la revoca, transizioni di sola rimozione); blocco di vincoli
validato per intero con esaustione dello spazio dei parametri per `T <= 3` e per
`V = 3`; controlli del light client come funzioni pure, con la lista chiusa delle
incapacità portata nel codice come `light_client::CANNOT_ESTABLISH` e con due
test che *asseriscono* l'indistinguibilità dichiarata invece di negarla.

### Verification transcript

```text
$ cargo --version
cargo 1.96.0 (30a34c682 2026-05-25)
$ rustc --version
rustc 1.96.0 (ac68faa20 2026-05-25)

$ cargo build --locked --workspace
   Compiling coblox-node v0.1.0 (E:\Git\CobloxNetwork\core\coblox-node)
   Compiling coblox-ffi v0.1.0 (E:\Git\CobloxNetwork\core\coblox-ffi)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.02s

$ cargo test --locked --workspace
     Running unittests src\lib.rs (coblox_core)
running 26 tests
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
     Running tests\canonical_serialization.rs
running 5 tests
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\conformance_registry.rs
running 22 tests
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\constraint_block.rs
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\election_degenerate.rs
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\light_client_perimeter.rs
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\sparse_account_state.rs
running 8 tests
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
     Running tests\worked_example.rs
running 6 tests
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src\lib.rs (coblox_ffi)
running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src\bin\uniffi-bindgen.rs / src\main.rs (coblox_node)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   Doc-tests coblox_core / coblox_ffi
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo fmt --all -- --check
(nessun output: pulito)

$ cargo clippy --workspace --all-targets --all-features -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s

$ cargo deny --version
cargo-deny 0.18.9
$ cargo deny check
advisories ok, bans ok, licenses ok, sources ok

--- job `android`: compilazione per aarch64-linux-android (il link richiede l'NDK,
--- non disponibile su questa macchina)
$ cargo check -p coblox-ffi --target aarch64-linux-android
   Compiling coblox-ffi v0.1.0 (E:\Git\CobloxNetwork\core\coblox-ffi)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 14s

--- job `desktop`: i passi che toccano coblox-core
$ cargo fmt --manifest-path src-tauri/Cargo.toml --check
(nessun output: pulito)
$ cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
    Checking coblox-core v0.1.0 (E:\Git\CobloxNetwork\core\coblox-core)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 34.13s
$ cargo test --manifest-path src-tauri/Cargo.toml --no-default-features core_version -- --nocapture
Coblox desktop core version: 0.1.0
test tests::desktop_command_reads_the_shared_core_version ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cd apps/desktop/src-tauri && cargo deny check
advisories ok, bans ok, licenses ok, sources ok

--- vincolo di sola lettura su docs/protocol/
$ git status --porcelain docs/protocol
(nessun output)

--- unsafe_code
$ grep -n "forbid" core/coblox-core/src/lib.rs
78:#![forbid(unsafe_code)]
```

### Deviations from the specification

**1. Nessun verificatore Ed25519 in questa spec — decisione motivata, non una
scorciatoia.**

`README.md#consensus-critical-ed25519-verification` impone una regola derivata da
ZIP-215 con una quinta condizione propria (`[8]A != identity`) e vieta
esplicitamente di sostituirla con `ed25519-dalek::verify_strict`, con modalità di
compatibilità o con "un default di libreria il cui comportamento sui casi limite
non sia stato mostrato equivalente a queste quattro regole"; la conformità si
misura sui vettori 0–11 di `ed25519-speccheck`.

Spedire un verificatore senza quei vettori come oracolo sarebbe esattamente il
comportamento non dimostrato che la specifica vieta, e sarebbe
**indistinguibile** da uno corretto fino a una divisione della catena. Il crate
spedisce quindi le **preimmagini di firma** — che sono deterministiche, sono
l'oggetto di questa spec e sono testate: procedura globale legata alla catena,
oggetto di binding della chiave di consenso, byte del voto di finalità — e il
punto di innesto `SignatureVerifier` con il contratto scritto nella sua
documentazione. Nessuna funzione di questo crate afferma che una firma sia
valida. **Raccomandazione al Lead:** una spec dedicata, con la tabella dei
vettori 0–11 come proprio gate, prima di qualunque devnet.

**2. Difetto nella specifica: `lifecycle_u8` non è definito.** (Segnalato, non
corretto: `docs/protocol/` è sola lettura.)

`ledger.md#sparse-merkle-account-state` definisce

```text
app_leaf = H(0x13 || account_key || u64be(balance_microtokens)
                 || u64be(account_nonce) || lifecycle_u8
                 || u64be(suspension_effective_epoch))
```

ma **nessun documento in `docs/protocol/` assegna un valore numerico** ad
`active`, `grace` e `suspended`. Ho cercato in tutti e quattro i documenti: la
tripletta compare come stringhe in `AccountProof` e nel ciclo di vita dell'app,
mai come byte. `lifecycle_u8` è quindi indefinito, e due implementazioni
conformi possono calcolare `app_leaf` diversi per lo stesso stato — cioè
`state_root` diverse, cioè una divisione della catena, sul primo conto di app
che non sia `active`. La gravità è la stessa dei difetti che
`README.md#hash-conformance-fixtures` esiste per intercettare, e infatti la
fixture del registro non copre `app_leaf`.

`coblox-core` usa una codifica **provvisoria** (ordine di dichiarazione a partire
da zero: `active`=0, `grace`=1, `suspended`=2), documentata come tale sul tipo
`merkle::AppLifecycle` e bloccata da un test che dichiara esplicitamente di non
essere una prova di correttezza. Va sostituita con quanto il documento fisserà.

**3. `GATE-CI-GREEN` non è spuntato, e la ragione è un conflitto di vincoli, non
una deroga.**

Il gate chiede la pipeline verde "su tutti e cinque i job **sul commit
consegnato**". Il mandato vieta esplicitamente qualunque commit e qualunque push,
quindi non esiste un commit consegnato e la pipeline non può avere girato. Il
gate è strutturalmente non soddisfacibile da questo agente, e spuntarlo sarebbe
un'affermazione falsa — precisamente il tipo di deroga che [DEBT-001] è costato.

Quello che ho fatto invece: eseguire **localmente ogni comando dei cinque job che
questa macchina può eseguire**, con la trascrizione reale sopra. Coperto: tutti e
cinque i passi del job `rust` (build, test, fmt, clippy `-D warnings`,
`cargo deny check`); la compilazione di `coblox-ffi` per `aarch64-linux-android`
del job `android`; e del job `desktop` i passi `fmt`, `clippy -D warnings`,
`cargo deny check` sul manifesto separato di `src-tauri` e il test
`core_version`. Non coperto: il *link* della libreria Android (richiede l'NDK
28.2, assente qui), `npm ci` / `npm run build` / `tauri build` del job `desktop`,
e la matrice Linux — questa macchina è Windows. Nessuno dei tre dipende dal
codice di questa spec se non attraverso la compilazione di `coblox-core`, che è
verificata su entrambi i manifesti e su entrambi i target.

**Al Lead:** il gate va soddisfatto sulla pipeline reale dopo il commit, oppure
riformulato per un agente a cui è vietato committare.

**4. Un valore inventato in una fixture, dichiarato.** L'esempio numerico
dell'elezione non pubblica `validator_max_set_size`, che il blocco di vincoli
richiede. Il fixture di test lo pone uguale a `validator_target_set_size` (8), il
valore più piccolo che ammette il set dell'esempio; non entra in nessuna
asserzione. Documentato in `tests/common/mod.rs`.

**5. Nessuna deroga a [[QUALITY]] richiesta o presa.**

### Handoff status
- [x] Ready for Project Lead review