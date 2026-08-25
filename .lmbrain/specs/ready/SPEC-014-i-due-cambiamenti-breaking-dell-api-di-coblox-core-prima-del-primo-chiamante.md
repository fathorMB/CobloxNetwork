---
id: SPEC-014
# Note: Quote the title if it contains a colon
title: "I due cambiamenti breaking dell'API di coblox-core, prima del primo chiamante"
status: ready
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

- [ ] Un tipo distinto rappresenta la preimmagine di firma e compare nella firma **sia** di `SignatureVerifier::verify` **sia** di `verify_consensus_ed25519`.
- [ ] Passare un `Digest32`, o una fetta di byte arbitraria, a uno dei due **non compila**.
- [ ] Il tipo non è costruibile da byte arbitrari se non attraverso una via **nominata e documentata come non-consensus**, i cui unici utilizzatori in albero sono la suite di conformità e l'oracolo.
- [ ] `RewardPolicy::check_internal` e `check_magnitudes` hanno la stessa visibilità dei gemelli del lato consenso.
- [ ] I tre chiamanti nei test sono riscritti sull'API pubblica, e **ogni asserzione che facevano è ancora fatta**. Se qualcuna non è esprimibile, è dichiarata con il controllo che la intercetta prima.
- [ ] La locuzione di `verifier.rs:27` corrisponde a ciò che `Cargo.toml` dichiara.
- [ ] **Nessun valore pubblicato si muove.** Se qualcosa lo facesse, è un finding da riportare prima di procedere.
- [ ] Il conteggio dei test non diminuisce, e nessuna delle gate di [SPEC-011] e [SPEC-012] perde il proprio caso.

## Implementation plan

1. Leggere il commento a `constraint_block.rs:1638` **prima** di toccare i test: descrive una precedenza fra controlli che la riscrittura deve rispettare o dichiarare.
2. Progettare il tipo e la via non-consensus, prendendo posizione su come quest'ultima è nominata.
3. Imporre il tipo su entrambi i punti d'ingresso, mai su uno solo.
4. Riportare privati i due sotto-controlli e riscrivere i chiamanti.
5. Allineare la locuzione.
6. Rieseguire tutto, comprese le gate di [SPEC-011] e [SPEC-012], e confrontare i conteggi.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-DIGEST-DOES-NOT-COMPILE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il tentativo di passare un `Digest32` e una fetta di byte arbitraria a **ciascuno** dei due punti d'ingresso produce un errore di compilazione, e la trascrizione riporta l'errore del compilatore. È l'unica prova che questa spec ha ottenuto qualcosa: un test che passa non distingue un tipo che vincola da uno che si limita a esistere.
- [ ] GATE-ESCAPE-HATCH-NAMED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La via ai byte grezzi è nominata, documentata come non-consensus, e una ricerca in albero mostra che i suoi unici utilizzatori sono la suite di conformità e l'oracolo. Una via generica e senza nome riaprirebbe il buco facendolo sembrare chiuso, che è il difetto che questa spec chiude.
- [ ] GATE-NO-COVERAGE-LOST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | I conteggi dei test prima e dopo sono riportati entrambi, e per ciascuna asserzione rimossa dai tre chiamanti riscritti è indicato dove è ora fatta. Rendere privato un metodo è il modo più semplice per perdere copertura senza che nulla diventi rosso.
- [ ] GATE-NOTHING-MOVED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | I cinque strumenti versionati passano e nessun valore pubblicato è cambiato. Questa spec non deve muovere nulla: se muove qualcosa, l'ha capita male.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la forma del tipo e la via non-consensus, e il Lead ha accettato la review. La superficie è piccola ma è la cucitura in cui un difetto non produce un errore bensì un'accettazione silenziosa, ed è la sola ragione per cui una spec di questa dimensione porta questa gate.

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

### Files changed

### Verification performed

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

```text

```

### Deviations from the specification

### Handoff status
- [ ] Ready for Project Lead review
