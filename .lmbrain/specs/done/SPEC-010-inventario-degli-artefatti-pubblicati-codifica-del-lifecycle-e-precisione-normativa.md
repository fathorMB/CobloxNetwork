---
id: SPEC-010
# Note: Quote the title if it contains a colon
title: "Inventario degli artefatti pubblicati, codifica del lifecycle e precisione normativa"
status: done
kind: feature
priority: high
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012, ADR-013]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [conformance, ledger, documentation]
activity:
  - date: 2026-08-25
    action: "transitioned backlog -> ready"
  - date: 2026-08-25
    action: "transitioned ready -> working"
  - date: 2026-08-25
    action: "transitioned working -> review"
  - date: 2026-08-25
    action: "attested verification GATE-INVENTORY-ANSWER by lead"
  - date: 2026-08-25
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "bf315f1de073b698d5956f196cbe46d632b70546d1675ba38d68c8cd77808c76"
    evidence_ref: "REVIEW-015. La domanda generale di DEBT-012 ha una risposta enumerata e non una rassicurazione: 51 preimmagini, 18 nel registro, 8 pubblicate altrove, 25 senza alcun valore pubblicato ciascuna con la ragione scritta, e un solo byte simbolico su 51, app_leaf.lifecycle_u8, con la spiegazione strutturale del perche sia uno solo. Verificato eseguendo published_artifacts.py --uncovered. Il Lead ha inoltre ricalcolato in modo indipendente entrambi gli hash nuovi, validando prima il metodo su due fixture non modificate: object_id e input_hash su 00 01 02 riprodotti, poi account_key app a881e2e0907aa86b225aaa2a2e1898afda1ce4733bd6d9cb390475ded4737e9d e app_leaf 2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515697 riprodotti da preimmagini ricostruite dai documenti, entrambi al primo tentativo."
    id: "SPEC-010-ATTEST-001"
    requirement_digest: "a23a840f1d65cdc2118c6995c4c9e1b2c37ddd8472c46c7b4874e4535a990342"
    requirement_id: "GATE-INVENTORY-ANSWER"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T19:38:15.157179+02:00"
---
# Inventario degli artefatti pubblicati, codifica del lifecycle e precisione normativa

## Objective

Rendere **eseguibile** la gate di [ADR-012], che oggi non lo è, e chiudere nella stessa passata i due difetti di specifica che quella gate avrebbe intercettato se fosse esistita: la codifica mancante di `lifecycle_u8` ([DEBT-012]) e le due affermazioni sovradimensionate del protocollo v0 ([DEBT-008]).

[ADR-012] impone a ogni spec che introduca o modifichi una regola di validità una passata su **tutti gli artefatti pubblicati**. La ADR lo scrive di sé stessa nelle proprie conseguenze: *«Chi scrive una spec deve sapere quali siano gli artefatti pubblicati, il che è un esercizio utile in sé: la quarta occorrenza esiste perché quell'inventario non era mai stato fatto.»* Finché l'elenco non esiste, la gate è un buon proposito con un nome da meccanismo.

## Context

Quattro volte, in tre spec diverse, un artefatto pubblicato ha insegnato una forma che le regole del protocollo rendono inammissibile. La tabella completa è in [ADR-012]. Ogni volta è emerso per caso.

[DEBT-012] ne indica la generalizzazione, e questa spec la raccoglie come oggetto primario invece che come nota a margine:

> Colpisce inoltre una superficie che il registro di conformità non copre, il che indica una lacuna più generale da valutare nella stessa occasione: **quante altre preimmagini non hanno una fixture pubblicata, e fra queste quante contengono campi la cui codifica non è fissata altrove.**

Il caso concreto che l'ha rivelata: la preimmagine di `app_leaf` in `docs/protocol/ledger.md` commette un campo `lifecycle_u8`, ma nessuno dei quattro documenti assegna un valore numerico agli stati `active`, `grace`, `suspended`. Due implementazioni conformi calcolano `app_leaf` diverse per lo stesso stato, quindi `state_root` diverse, quindi una divisione della catena al primo conto di app che non sia `active`. `coblox-core` usa una codifica provvisoria `0/1/2` documentata come tale e bloccata da un test che dichiara di **non** essere una prova di correttezza.

Questa spec porta anche due contenuti normativi decisi dall'operatore il 2026-08-25 e non ancora scritti nei documenti: l'intervallo di blocco di [ADR-013] e la conseguenza che quella ADR dichiara insieme al valore.

## Scope

### Included

- L'**inventario degli artefatti pubblicati**, in forma versionata e leggibile da uno strumento.
- Lo **strumento versionato** che [ADR-012] richiede, con la sua prova in negativo.
- La codifica numerica del ciclo di vita del conto di app, con la fixture che oggi manca ([DEBT-012]).
- Le due riformulazioni di [DEBT-008].
- L'intervallo di blocco di [ADR-013] nei documenti di protocollo, con la sua non-imposizione dichiarata.
- L'allineamento di `coblox-core` dove la codifica provvisoria diventa definitiva.

### Excluded

- **`RewardBounds` e le tre regole di validità di [SPEC-009] in `coblox-core`.** Sono [SPEC-011] e non vanno anticipate qui: questa spec deve produrre l'inventario *prima* che qualcuno lo usi, altrimenti si autocertifica.
- **Il verificatore Ed25519** ([SPEC-012]) e **la separazione della chiave di trasporto** ([SPEC-013]).
- Qualunque modifica alla **regola** di elezione o alla reward policy. Qui si scrivono codifiche mancanti e si correggono frasi, non si cambiano regole — con la sola eccezione discussa in *Risks and open decisions*.

## Existing-project analysis

**Verificato dal Lead il 2026-08-25.** Ciò che segue è stato controllato nei file, non ricordato; dove il Lead non ha verificato, è detto.

- `docs/protocol/` conta 4 566 righe su quattro documenti: `README.md` 941, `ledger.md` 2 382, `identity.md` 447, `wire.md` 442, `app-manifest.md` 354.
- Il registro di conformità in `README.md` elenca dieci valori di hash attesi con la loro fixture. `app_leaf` **non è fra questi**, e nessuna fixture pubblicata l'avrebbe intercettata.
- `sim/tools/protocol_hashes.py` e `sim/tools/reward_rules.py` esistono e sono versionati. Sono i due strumenti che [ADR-012] cita come precedente, e **hanno già trovato un difetto ciascuno**. Sono il modello da seguire, non da riscrivere.
- `core/coblox-core/tests/conformance_registry.rs` e `light_client_perimeter.rs` incorporano gli hash attesi come costanti. Sono una terza copia degli stessi valori, dopo il documento e lo strumento Python.
- La ricerca di `lifecycle_u8` su tutti i documenti restituisce **una sola riga**, quella che lo usa. Nessuna che lo definisca.

**Un'osservazione del Lead che l'implementatore deve contestare se la giudica sbagliata.** Gli stessi valori di hash vivono oggi in almeno tre sedi indipendenti — documento, strumento Python, test Rust — allineate a mano. È la forma in cui un artefatto pubblicato invecchia in silenzio, ed è plausibile che l'inventario debba affrontare anche questo. Il Lead **non** prescrive la soluzione: prescrive che la questione sia affrontata esplicitamente e che la scelta sia motivata.

## Technical proposal

### 1. L'inventario non è un elenco scritto a mano

Un elenco compilato a mano ha lo stesso difetto degli artefatti che deve sorvegliare: invecchia in silenzio, e si legge come se fosse aggiornato. L'inventario deve quindi avere **due metà**, e la seconda è quella che conta.

**a. Un manifesto versionato** che enumera gli artefatti pubblicati con, per ciascuno, la sede, il tipo e ciò che il documento asserisce su di esso.

**b. Una verifica di completezza del manifesto stesso**, che enumera meccanicamente i candidati presenti nei documenti — i blocchi di esempio canonici, le righe delle tabelle di conformità, i valori dichiarati attesi — e **fallisce se un candidato non compare nel manifesto**. Senza questa metà, il manifesto è una dichiarazione di intenti con un formato.

Il perimetro è quello di [ADR-012]: registro di conformità, esempi normativi e ogni valore che i documenti espongono come atteso; non gli artefatti interni al brain.

### 2. Lo strumento, e la sua prova in negativo

Lo strumento versionato esegue l'inventario e verifica ciò che l'inventario dichiara verificabile. [ADR-012] pone tre condizioni, e la terza è quella che il progetto ha pagato: **lo strumento deve saper fallire, e va verificato in negativo**, reintroducendo il difetto e osservando la guardia fallire. Una guardia che non sa fallire non è una guardia, e un falso positivo insegna a non fidarsi — [SPEC-009] ha creato il precedente con una costante di confronto invecchiata che gridava al lupo su un allineamento corretto.

### 3. La codifica del ciclo di vita

Assegnare valori numerici espliciti agli stati del conto di app, con due vincoli di forma:

- La codifica va **dichiarata nel documento di protocollo**, non solo nel codice, perché il difetto è di interoperabilità e vive fra due implementazioni.
- **Un valore sconosciuto è invalido, non predefinito.** Un default è il modo in cui la divergenza rientra: due implementazioni che sbagliano allo stesso modo su un valore noto si accorgono del problema, due che applicano default diversi a un valore ignoto no.

La scelta dei numeri è dell'implementatore ed è libera, perché nessuna fixture pubblicata la vincola. **Va però motivata**, e la tensione va nominata: se `0` sia lo stato attivo — l'ordine di elencazione, che è ciò che tre implementatori sceglierebbero — oppure un valore riservato che intercetta lo stato non inizializzato. Il Lead non ha una preferenza verificata e non ne impone una.

Serve poi la **fixture pubblicata di `app_leaf`**, che oggi manca, e l'allineamento di `coblox-core` dove la codifica provvisoria diventa definitiva — compresa la rimozione del test che dichiara di non essere una prova.

### 4. Le due frasi di [DEBT-008]

- **RF-109.** La frase secondo cui il pavimento Argon2id rifiuta tutto ciò che è più debole di entrambe le raccomandazioni RFC è più ampia del vero: la forma ad area ammette una banda con `iterations = 1` sotto i 2 GiB quando `memory_kib >= 196608`, che non corrisponde ad alcuna delle due raccomandazioni.
- **RF-110.** La conseguenza secondo cui lo scudo di ammissione costa all'attaccante un indirizzo raggiungibile per ogni slot concorrente vale **solo se** l'emissione dei nonce è conteggiata contro il limite per sorgente del primo passo; il carattere monouso del nonce limita il riuso, non il volume.

Per entrambe la scelta è la stessa e va presa nel merito: **riformulare la frase** perché descriva esattamente ciò che le regole impongono, **oppure cambiare la regola** perché imponga ciò che la frase promette. Vedi *Risks and open decisions* per la conseguenza della seconda strada.

### 5. L'intervallo di blocco

Scrivere nei documenti di protocollo la costante di [ADR-013] — **5 secondi** — come costante di genesi, non come parametro governato, e con essa la parte che una scrittura compiacente ometterebbe: **v0 dichiara la cadenza e non la impone**, perché il solo vincolo su `timestamp_ms` è la mediana degli undici precedenti. La conseguenza è già registrata come [DEBT-013] e **non va risolta qui**: va nominata nel documento e collegata al debito.

## Files and areas involved

- `docs/protocol/README.md` — registro di conformità, fixture di `app_leaf`, intervallo di blocco.
- `docs/protocol/ledger.md` — codifica del ciclo di vita, preimmagine di `app_leaf`, cadenza dichiarata.
- `docs/protocol/identity.md` — le due frasi di [DEBT-008].
- `sim/tools/` — lo strumento versionato dell'inventario, accanto ai due esistenti.
- Sede dell'inventario, da proporre: `docs/protocol/` se è contenuto normativo, `sim/tools/` se è materiale di verifica. La scelta va motivata.
- `core/coblox-core/src/` e `tests/` — allineamento della codifica del ciclo di vita e della fixture nuova.

## Acceptance criteria

- [x] L'inventario degli artefatti pubblicati esiste, è versionato, ed enumera il perimetro di [ADR-012].
- [x] La verifica di **completezza** del manifesto esiste e fallisce quando un artefatto presente nei documenti non vi compare. Dimostrato reintroducendo l'omissione.
- [x] Lo strumento è versionato nel repository, eseguibile in un comando, e la sua esecuzione è nella trascrizione.
- [x] Lo strumento è **provato in negativo** su ogni classe di difetto che dichiara di intercettare.
- [x] Gli stati del ciclo di vita del conto di app hanno valori numerici dichiarati nel documento di protocollo, con la motivazione della scelta.
- [x] Un valore di ciclo di vita sconosciuto è **rifiutato**, e la trascrizione mostra il rifiuto.
- [x] Esiste una fixture pubblicata di `app_leaf` nel registro di conformità, riprodotta da `coblox-core`.
- [x] `coblox-core` non contiene più codifiche dichiarate provvisorie per il ciclo di vita, né il test che dichiara di non essere una prova.
- [x] Le due frasi di [DEBT-008] corrispondono esattamente a ciò che le regole impongono, per riformulazione o per modifica della regola, con la scelta motivata nel merito.
- [x] L'intervallo di blocco di [ADR-013] è nei documenti di protocollo come costante di genesi, insieme alla dichiarazione esplicita che v0 non ne impone il rispetto, con rimando a [DEBT-013].
- [x] La domanda generale di [DEBT-012] ha una risposta scritta: **quali preimmagini non hanno una fixture pubblicata, e quali fra queste contengono campi la cui codifica non è fissata altrove.** Un elenco, non una rassicurazione.
- [x] Nessun hash pubblicato esistente cambia senza che il cambiamento sia dichiarato e ricalcolato dai byte effettivamente scritti.

## Implementation plan

1. Enumerare gli artefatti pubblicati leggendo i quattro documenti, e produrre l'elenco delle preimmagini prive di fixture — cioè rispondere prima alla domanda generale di [DEBT-012], perché è ciò che dimensiona tutto il resto.
2. Decidere la sede e la forma del manifesto, motivando la scelta.
3. Scrivere lo strumento e la verifica di completezza; provarli in negativo su un difetto reintrodotto per ciascuna classe.
4. Fissare la codifica del ciclo di vita nel documento, aggiungere la fixture di `app_leaf`, allineare `coblox-core`.
5. Riformulare le due frasi di [DEBT-008], oppure cambiare la regola, dichiarando quale delle due strade e perché.
6. Scrivere l'intervallo di blocco e la sua non-imposizione.
7. Rieseguire l'intera verifica e incollare le trascrizioni.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-TOOL-NEGATIVE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Per **ogni** classe di difetto che lo strumento dichiara di intercettare, il difetto è reintrodotto e la trascrizione mostra la guardia fallire, poi passare a difetto rimosso. Include l'omissione di un artefatto dal manifesto. È la precisazione n.3 di [ADR-012] e non è negoziabile: uno strumento che non si è mai visto fallire non è evidenza.
- [x] GATE-FIXTURES-RECOMPUTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il metodo di calcolo è validato riproducendo almeno una fixture **non modificata** prima di calcolare quelle nuove, e ogni hash è calcolato dai byte effettivamente scritti. Incollare entrambe le esecuzioni. È il metodo che il progetto ha già usato tre volte e che il Lead rifarà in modo indipendente.
- [x] GATE-LIFECYCLE-REJECT | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Un valore di ciclo di vita non assegnato viene **rifiutato**, non interpretato come predefinito, e la trascrizione lo mostra. Un default è il modo in cui la divergenza che questa spec chiude rientrerebbe dalla finestra.
- [x] GATE-INVENTORY-ANSWER | kind=manual | owner=lead | phase=before-done | evidence=artifact | Il Lead verifica che la domanda generale di [DEBT-012] abbia una risposta enumerata e non una rassicurazione, e ricalcola in modo indipendente almeno un hash nuovo. Le altre due volte in cui il Lead ha verificato così ha trovato qualcosa.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Questa spec potrebbe cambiare una regola di validità, e in tal caso ricade sotto [ADR-012] che essa stessa rende eseguibile.** Accade se per RF-109 si sceglie di restringere il pavimento Argon2id invece di descriverlo, o se per RF-110 si conteggia l'emissione dei nonce contro il limite per sorgente. Non è un paradosso da evitare ma una sequenza da rispettare: l'inventario si costruisce **prima**, poi lo si esegue sulla modifica. Se si sceglie quella strada, la passata va fatta e la trascrizione allegata.
- **Il rischio principale di questa spec è la falsa completezza.** Un inventario che dichiara di coprire tutto e ne copre il novanta per cento è peggio dell'assenza di inventario, perché la gate successiva restituirà un verde più stretto del vero — che è precisamente il difetto per cui [ADR-012] esiste. Se una classe di artefatti non è enumerabile meccanicamente, **va dichiarata come non coperta** e non inclusa in silenzio. Vale la regola *no silent caps*: ciò che resta fuori si scrive.
- **La triplice sede degli hash attesi** — documento, strumento Python, test Rust — è segnalata in *Existing-project analysis* come osservazione del Lead non verificata a fondo. Va affrontata esplicitamente, in un senso o nell'altro, non ignorata.
- **La codifica del ciclo di vita è irreversibile una volta pubblicata una fixture.** È il motivo per cui la scelta va motivata e non presa per ordine di elencazione.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- **Contestare le formulazioni del Lead fa parte del mandato, e vale anche quando l'assunzione arriva dal Lead.** In questa spec sono marcate come tali l'osservazione sulla triplice sede degli hash e l'assenza di preferenza sulla codifica. Tre errori del Lead in questo progetto sono stati trovati dagli specialisti e nessuno da lui; vedi `.lmbrain/knowledge/recurring-defects.md`.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence

### Changes made

**1. The inventory, in two halves.**

`sim/tools/published_artifacts.toml` is the manifest: every domain-separation
string (39), tree tag byte (24), named fixture (15), digest literal (51) and
base64url presentation (10) the four protocol documents carry, plus the 51 hash
preimages with their coverage state, the equalities an inline example must
satisfy, and eleven hand-pinned normative passages.

`sim/tools/published_artifacts.py` is the half that matters. It re-derives the
candidate set mechanically from `docs/protocol/*.md` and fails when the manifest
and the documents disagree **in either direction**, so neither the manifest nor
the tool can age in silence. Ten defect classes, C1 to C10. Its module docstring
also states what the sweep does **not** cover, and `meta.not_covered` in the
manifest carries the same list, which check C6 keeps honest.

*Seat, and why.* The manifest sits in `sim/tools/`, not in `docs/protocol/`.
Putting it under `docs/protocol/` would make it a fifth published artifact — one
that asserts expected values, can go stale, and would then need a guard of its
own. It is an index over the normative documents, not normative content, and the
documents stay the only oracle.

**2. The negative proof, versioned rather than pasted once.**

`sim/tools/published_artifacts_negative.py` copies the tree to a temporary
directory, reintroduces exactly one defect per class, and requires the tool to
exit non-zero **naming that class**; then it requires a clean pass on the
unmutated copy. The working tree is never touched. A pasted transcript proves
the guard failed on the day somebody ran it; this proves it on every run, and it
is wired into CI.

**3. `lifecycle_u8`.** `ledger.md` now assigns `active = 0x01`, `grace = 0x02`,
`suspended = 0x03`, reserves `0x00`, and declares every other value invalid with
no default. `coblox-core` follows: `AppLifecycle::from_u8` is fallible with no
default arm, and the test that declared itself not to be a proof is gone,
replaced by one that asserts rejection over the whole `u8` range plus the
published `APP-0` fixture that asserts the encoding itself.

**4. `APP-0`**, the app account in state `suspended` — deliberately not `active`,
since a fixture in the state whose byte an implementer would guess correctly
proves nothing about the encoding. Two registry rows: `account_key` (app) and
`app_leaf`.

**5. The two sentences of [DEBT-008].** Both reformulated; one also carries a
rule change. Reasoning per sentence in *Deviations*.

**6. The block interval.** `README.md` gains a `Genesis constants` section with
`block_interval_seconds = 5`, its non-governed status, and the explicit statement
that v0 does not enforce the cadence, linked to [DEBT-013].
`ledger.md#block-format` repeats the non-imposition where an implementer looks
for the timestamp rules. [DEBT-013] is named, not closed.

### Files changed

| File | What |
| --- | --- |
| `sim/tools/published_artifacts.py` | new — the inventory tool, ten defect classes |
| `sim/tools/published_artifacts.toml` | new — the manifest |
| `sim/tools/published_artifacts_negative.py` | new — the negative proof of all ten |
| `sim/tools/protocol_hashes.py` | reads the registry table instead of copying it; adds the tagged-tree section and `APP-0` |
| `docs/protocol/README.md` | genesis constants; `APP-0`; RF-109; "inline examples are not oracles"; two boundary rows |
| `docs/protocol/ledger.md` | `lifecycle_u8` encoding; block-interval non-imposition; challenge-evidence example corrected |
| `docs/protocol/identity.md` | RF-110: the nonce-volume rule and the reformulated consequence |
| `core/coblox-core/src/merkle.rs` | normative `AppLifecycle` encoding, `from_u8`, `RESERVED_U8` |
| `core/coblox-core/tests/sparse_account_state.rs` | the "not a proof" test replaced by a rejection test |
| `core/coblox-core/tests/conformance_registry.rs` | `APP-0` rows; row count 16 to 18 |
| `core/coblox-core/tests/canonical_serialization.rs` | the corrected `request_hash` of the challenge-evidence example |
| `.github/workflows/ci.yml` | new `protocol-docs` job running the four Python guards |

Nothing under `.lmbrain/` was modified except this evidence section.

### Verification performed

- `python sim/tools/published_artifacts.py` — the inventory over 51 preimages
  and 184 candidates across the ten classes.
- `python sim/tools/published_artifacts_negative.py` — **GATE-TOOL-NEGATIVE**:
  ten defects reintroduced one at a time, each observed failing, plus the
  control run on the unmutated copy.
- `python sim/tools/protocol_hashes.py` — **GATE-FIXTURES-RECOMPUTED**: the six
  values this pass did not change reproduced first, then the two it added.
- `cargo test -p coblox-core` with a default arm reintroduced into
  `AppLifecycle::from_u8`, then removed — **GATE-LIFECYCLE-REJECT**.
- `cargo test -p coblox-core` with the provisional `0/1/2` encoding
  reintroduced, then removed — the `APP-0` fixture proved in the negative.
- `cargo test --locked --workspace`, `cargo fmt --all -- --check`,
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- `python sim/tools/reward_rules.py` — the pre-existing guard re-run, because
  the RF-110 remediation changes a normative MUST and [ADR-012] applies to this
  spec as it does to any other.

### Verification transcript

#### The inventory

```text
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
```

#### GATE-TOOL-NEGATIVE — every defect class observed failing

```text
$ python sim/tools/published_artifacts_negative.py
=== control: the unmutated copy ===
published-artifact inventory: PASS

=== C1-DOMAIN ===
defect reintroduced: a new domain-separation string is added to a document and nobody records it as a published artifact
  FAIL C1-DOMAIN: domain string 'coblox-brand-new-v0' occurs in wire.md but is absent from the manifest
  exit=1 names C1-DOMAIN: True

=== C2-TAG ===
defect reintroduced: a new tagged tree is introduced with a tag byte the inventory has never seen
  FAIL C2-TAG: tree tag byte '0x50' occurs in ledger.md but is absent from the manifest
  exit=1 names C2-TAG: True

=== C3-FIXTURE-ID ===
defect reintroduced: a document names a conformance fixture that is in no inventory
  FAIL C3-FIXTURE-ID: fixture identifier 'NEW-9' occurs in README.md but is absent from the manifest
  exit=1 names C3-FIXTURE-ID: True

=== C4-VALUE ===
defect reintroduced: a published digest is edited, which is the shape of a fixture that silently stops matching what it claims
  FAIL C4-VALUE: digest 2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515698 occurs in README.md but is not classified in the manifest
  FAIL C6-ORPHAN: digest 2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515697 is in the manifest but occurs in no document
  exit=1 names C4-VALUE: True

=== C5-MIRROR ===
defect reintroduced: the transcription in the coblox-core conformance suite drifts away from the document it transcribes
  FAIL C5-MIRROR: registry digest a881e2e0907aa86b225aaa2a2e1898afda1ce4733bd6d9cb390475ded4737e9d (account_key (app) | APP-0) is declared mirrored in core/coblox-core/tests/conformance_registry.rs but does not appear there
  exit=1 names C5-MIRROR: True

=== C6-ORPHAN ===
defect reintroduced: a published artifact is deleted from the documents while the inventory keeps asserting it - the tool going stale, which is the false positive of [SPEC-009]
  FAIL C6-ORPHAN: tree tag byte '0x26' is in the manifest but occurs in no document
  exit=1 names C6-ORPHAN: True

=== C7-COVERAGE ===
defect reintroduced: a preimage is declared covered by a fixture that does not exist
  FAIL C7-COVERAGE: preimage 'app_leaf' names fixture 'APP-9', which is not a declared fixture identifier
  exit=1 names C7-COVERAGE: True

=== C8-ENCODING ===
defect reintroduced: the lifecycle_u8 encoding table is removed from the document while the preimage still commits the byte - [DEBT-012] exactly
  FAIL C8-ENCODING: preimage 'app_leaf' commits the symbolic byte 'lifecycle_u8', whose enumeration is declared in ledger.md but no longer matches '\\| `active` \\| `0x01` \\|'. An undeclared symbolic byte is DEBT-012.
  exit=1 names C8-ENCODING: True

=== C9-EXAMPLE ===
defect reintroduced: an inline example stops satisfying an equality the specification states between its own fields - the defect this pass found
  FAIL C4-VALUE: digest e14d4c02c41a950c9f4f4464e9f98a6652c64e6c992efc36c97f01d2f4ca2dc2 occurs in ledger.md but is not classified in the manifest
  FAIL C9-EXAMPLE: invariant 'challenge-evidence-request-hash': README.md#hash-preimage-registry: `challenge_id` MUST equal `request_hash` - the example in ledger.md carries body.challenge_id=sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21, body.request_hash=sha256:e14d4c02c41a950c9f4f4464e9f98a6652c64e6c992efc36c97f01d2f4ca2dc2
  exit=1 names C9-EXAMPLE: True

=== C10-PROBE ===
defect reintroduced: the declaration that v0 does not enforce the block interval is quietly dropped, leaving a cadence that reads as enforced
  FAIL C10-PROBE: probe 'block-interval-not-enforced' expected 1 match(es) of '\\*\\*`block_interval_seconds = 5` is declared, not enforced\\.\\*\\*' in README.md, found 0. [ADR-013] part 3 is a renunciation, and a declared cadence reads as an enforced one unless the difference is written.
  exit=1 names C10-PROBE: True

negative proof: PASS - 10 defect classes, each observed failing
```

#### GATE-FIXTURES-RECOMPUTED

```text
$ python sim/tools/protocol_hashes.py
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
```

#### GATE-LIFECYCLE-REJECT

First with the default arm the rule forbids put back into
`AppLifecycle::from_u8` (`_ => Ok(Self::Active)` in place of the error):

```text
$ cargo test -p coblox-core --test sparse_account_state an_unassigned
   Compiling coblox-core v0.1.0 (E:\Git\CobloxNetwork\core\coblox-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.83s
     Running tests\sparse_account_state.rs
running 1 test
test an_unassigned_lifecycle_value_is_rejected_and_never_defaulted ... FAILED
failures:
---- an_unassigned_lifecycle_value_is_rejected_and_never_defaulted stdout ----
thread 'an_unassigned_lifecycle_value_is_rejected_and_never_defaulted' (32040)
panicked at core\coblox-core\tests\sparse_account_state.rs:211:5:
0x00 is reserved and MUST NOT decode to a state
failures:
    an_unassigned_lifecycle_value_is_rejected_and_never_defaulted
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 7 filtered out
error: test failed, to rerun pass `-p coblox-core --test sparse_account_state`
```

Then with the default arm removed again:

```text
$ cargo test -p coblox-core --test sparse_account_state an_unassigned
   Compiling coblox-core v0.1.0 (E:\Git\CobloxNetwork\core\coblox-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.81s
     Running tests\sparse_account_state.rs
running 1 test
test an_unassigned_lifecycle_value_is_rejected_and_never_defaulted ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out
```

#### The `APP-0` fixture, proved in the negative

With the provisional pre-[DEBT-012] encoding put back (`suspended = 2`):

```text
$ cargo test -p coblox-core --test conformance_registry app0
     Running tests\conformance_registry.rs
running 1 test
test app0_account_key_and_app_leaf_match_the_registry ... FAILED
failures:
---- app0_account_key_and_app_leaf_match_the_registry stdout ----
thread 'app0_account_key_and_app_leaf_match_the_registry' (30940) panicked at
core\coblox-core\tests\conformance_registry.rs:340:5:
assertion `left == right` failed
  left: Digest32([63, 105, 146, 164, 3, 26, 91, 209, 98, 229, 105, 127, 77, 210,
        231, 232, 187, 127, 45, 151, 219, 127, 191, 13, 172, 59, 47, 69, 97,
        199, 152, 62])
 right: Digest32([46, 172, 139, 10, 121, 85, 167, 5, 67, 237, 223, 151, 88, 67,
        251, 142, 77, 223, 55, 125, 174, 240, 139, 97, 199, 184, 205, 228, 105,
        81, 86, 151])
failures:
    app0_account_key_and_app_leaf_match_the_registry
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 22 filtered out
error: test failed, to rerun pass `-p coblox-core --test conformance_registry`
```

The left value is `sha256:3f6992a4…`, which `protocol_hashes.py` prints above as
"provisional pre-[DEBT-012] suspended = 2". With the normative encoding restored:

```text
$ cargo test -p coblox-core --test conformance_registry app0
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.75s
     Running tests\conformance_registry.rs
running 1 test
test app0_account_key_and_app_leaf_match_the_registry ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out
```

#### The written answer to the general question of [DEBT-012]

```text
$ python sim/tools/published_artifacts.py --uncovered
Preimages in the v0 documents: 51
  in the conformance registry table: 18
  with a published value elsewhere:  8
  with no published value:           25

Preimages with no published fixture, and why:
  chain_id                     README.md
      Derived from a network_id and a genesis block ID, neither of which v0 fixes: a fixture would have to invent a genesis and would then be the only place in the documents asserting one. Covered indirectly, since every registry row binds the all-zero chain_id of HASH-0.
  enrollment_pow_salt          README.md
      Named in the preimage registry with no row of its own. It is a 16-byte truncation whose only consumer is the Argon2id salt, so an implementation that gets it wrong fails the ADM-0 and ER-0 rows for a different reason; a dedicated fixture would still be worth adding and is the clearest remaining gap.
  pow_password                 identity.md
      The Argon2id password. Verifying it end to end requires an Argon2id evaluation, which the documents deliberately keep out of the digest table; the enrollment example carries a placeholder proof-of-work nonce rather than a solved one.
  tx_id                        ledger.md
      JCS over a full unsigned transaction. The seven canonical transaction examples fix the JCS form, which is the part an implementation gets wrong, but none carries its own tx_id.
  block_id                     ledger.md
      JCS over a block header. The canonical header example fixes the JCS form; no published value is its digest.
  validator_set_hash           ledger.md
      JCS over a ValidatorSet. No canonical ValidatorSet example is published, so the JCS form of that object is fixed only by the schema. Note that this preimage, alone among the domain-separated ones, does not bind chain_id.
  app_id                       app-manifest.md
      JCS over an unsigned manifest. The canonical manifest example fixes the JCS form; its app_id is not published.
  message_id                   wire.md
      JCS over an envelope without message_id and signature. The canonical envelope example fixes the JCS form; the message_id it carries is a placeholder.
  dht_namespace_key            wire.md
      Derived from a genesis block ID, which v0 does not fix, for the same reason as chain_id. It is a discovery namespace, not a consensus commitment.
  tx_leaf                      ledger.md
      One tag byte over a tx_id. Uncovered together with the whole transaction Merkle tree; the tree's shape rules (order preserved, padded with H(0x02), empty root H(0x03)) are stated but no worked example computes a transactions_root.
  merkle_node                  ledger.md
      As tx_leaf: the transaction Merkle tree has no published worked example. The candidate-set tree of the election has one, and it exercises the same shape.
  tx_padding_leaf              ledger.md
      H(0x02), the transaction-tree padding leaf. No published value.
  empty_transactions_root      ledger.md
      H(0x03), the empty-block transactions root. No published value, although its revocation-tree counterpart H(0x33) has one.
  node_leaf                    ledger.md
      The node-account counterpart of app_leaf. It commits no symbolic byte � account_key, two u64be � so it carries none of the risk that made app_leaf urgent, but it is the clearest candidate for the next fixture after enrollment_pow_salt.
  subscription_leaf            ledger.md
      The active-subscription tree has no published worked example. Its root appears in the publisher-reward mint example only as a placeholder.
  subscription_node            ledger.md
      As subscription_leaf.
  subscription_empty           ledger.md
      As subscription_leaf.
  empty_subscription_root      ledger.md
      H(0x23). Stated to be unreachable in a valid publisher reward, and not published.
  eligible_leaf                ledger.md
      The eligible-set tree has no published worked example. Its root appears in the existence-income mint example only as a placeholder.
  eligible_node                ledger.md
      As eligible_leaf.
  eligible_empty               ledger.md
      As eligible_leaf.
  empty_eligible_root          ledger.md
      H(0x27). Stated to be unreachable in a valid mint, and not published.
  revocation_node              README.md
      REVL-0 is a single-entry tree, so no internal node is exercised by it. A two-entry fixture would cover this and is not published.
  revocation_empty             README.md
      H(0x32), the padding leaf. Not exercised by the single-entry REVL-0 and not published.
  empty_candidate_root         ledger.md
      H(0x43). Stated to be unreachable in a valid election, and not published.

The second half of the question of [DEBT-012]: which of these
commit a field whose encoding is not fixed elsewhere.

A preimage field is safe when its bytes are determined by the
formula alone: a fixed-width big-endian integer, raw digest bytes,
a UTF-8 string with its length, a literal tag, or a JCS object
whose schema enumerates its own spellings. A field is a SYMBOLIC
BYTE when a name has to be turned into a number, because the
number is the thing the formula does not carry.

Symbolic bytes across all 51 preimages: 1
  app_leaf.lifecycle_u8 - enumeration declared in ledger.md

That count is one, and the reason is structural rather than lucky:
v0 commits every other enumeration as a JCS string inside a JCS
object, so the committed bytes are the letters of the name and no
mapping exists to disagree about. `lifecycle_u8` is the only place
where a name is committed as a number, and it is exactly the place
where the mapping was missing. Check C8 fails if a preimage grows
another one without an enumeration to point at.

Not covered by this sweep at all:
  - Prose. A rule or value stated only in running text carries no domain string, tag byte, fixture identifier or digest literal and is invisible to the mechanical sweep. The C10 probe list pins the passages that matter today and is not claimed to be exhaustive.
  - base64url presentations outside the 43-character (32-byte) and 22-character (16-byte) unpadded forms, including the 86-character signature placeholders.
  - Semantic correctness of any digest. This tool never recomputes one; sim/tools/protocol_hashes.py and the coblox-core conformance suite do.
  - Numeric fixture values that are not digests, such as the parameter values of the PD-0 bodies. Those are the subject of sim/tools/reward_rules.py and of the constraint-block validation the registry section already requires of a suite.
```

#### The [ADR-012] pass over the rule this spec itself changes

The RF-110 remediation adds a normative MUST on validator behaviour, so this
spec falls under the ADR it makes executable. The inventory was built first and
run afterwards, in that order; the pre-existing rule guard was re-run too.

```text
$ python sim/tools/reward_rules.py
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
```

#### Workspace

```text
$ cargo fmt --all -- --check
(no output: clean)

$ cargo clippy --workspace --all-targets --all-features -- -D warnings
Checking coblox-core v0.1.0 (E:\Git\CobloxNetwork\core\coblox-core)
    Checking coblox-ffi v0.1.0 (E:\Git\CobloxNetwork\core\coblox-ffi)
    Checking coblox-node v0.1.0 (E:\Git\CobloxNetwork\core\coblox-node)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.87s
```

```text
$ cargo test --locked --workspace
   Compiling coblox-core v0.1.0 (E:\Git\CobloxNetwork\core\coblox-core)
   Compiling coblox-node v0.1.0 (E:\Git\CobloxNetwork\core\coblox-node)
   Compiling coblox-ffi v0.1.0 (E:\Git\CobloxNetwork\core\coblox-ffi)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.80s
     Running unittests src\lib.rs (target\debug\deps\coblox_core-b5fd208130b90801.exe)
running 26 tests
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
     Running tests\canonical_serialization.rs (target\debug\deps\canonical_serialization-20712e00c2dce33c.exe)
running 5 tests
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\conformance_registry.rs (target\debug\deps\conformance_registry-be1b63b18fcf3494.exe)
running 23 tests
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\constraint_block.rs (target\debug\deps\constraint_block-b03a13f95811055a.exe)
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\election_degenerate.rs (target\debug\deps\election_degenerate-65b85099a3cb8ec9.exe)
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\light_client_perimeter.rs (target\debug\deps\light_client_perimeter-a5643f417981de10.exe)
running 12 tests
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running tests\sparse_account_state.rs (target\debug\deps\sparse_account_state-d8046c81073ed252.exe)
running 8 tests
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
     Running tests\worked_example.rs (target\debug\deps\worked_example-ebe129c1b639d164.exe)
running 6 tests
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src\lib.rs (target\debug\deps\coblox_ffi-2bb3b2ae42f16959.exe)
running 1 test
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src\bin\uniffi-bindgen.rs (target\debug\deps\uniffi_bindgen-cfef90536b1d6158.exe)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
     Running unittests src\main.rs (target\debug\deps\coblox_node-a0511f3038ca2b70.exe)
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

(filtered to the per-binary headers and results; the 187-line full output
 lists every individual test as ok. Totals below.)

TOTAL: 105 passed, 0 failed across 13 test binaries
```

### Deviations from the specification

**1. The Lead's count of registry rows was wrong, and it mattered.**
*Existing-project analysis* states, as verified: "Il registro di conformità in
`README.md` elenca **dieci** valori di hash attesi". The table had **sixteen**
rows, and `core/coblox-core/tests/conformance_registry.rs` already carried
`REGISTRY_ROW_COUNT: usize = 16` — the correct number was in the tree, in the
file the same paragraph cites two bullets later. It is the tratto comune
`.lmbrain/knowledge/recurring-defects.md` names for all three families: the fact
was already written somewhere and nobody was looking at it. It mattered because
an inventory sized to ten rows would have declared itself complete over sixty
per cent of the table.

**2. The Lead's file attribution for [DEBT-008] was wrong.** *Files and areas
involved* assigns both sentences to `docs/protocol/identity.md`. RF-109's
sentence is in `README.md`, in *The enrollment cost floor is a validity rule*;
only RF-110's is in `identity.md`. Both were corrected where they actually are.

**3. The triple-site observation was right about the symptom and wrong about the
shape.** The Lead asked for it to be faced, not for a particular answer. The
three sites were never of equal standing:

- `docs/protocol/README.md` is the oracle.
- `sim/tools/protocol_hashes.py` **recomputes** values, so its `PUBLISHED` dict
  was pure duplication. It is gone: the tool now reads the registry table. That
  copy had already gone stale once, in [SPEC-009], and made the tool report a
  mismatch that did not exist — the false positive [ADR-012] cites when it
  requires the negative proof.
- `core/coblox-core/tests/conformance_registry.rs` **is the implementation under
  test**, and its hand transcription is deliberate and must stay: a suite whose
  expectation is generated by the code it checks asserts nothing. Its own header
  already says so.

So the answer is not "deduplicate". It is one oracle, one derived reader, one
deliberate transcription — and check C5 now compares the transcription against
the document on every run, which is what turns "aligned by hand" into "aligned
by hand, checked by machine".

**4. The lifecycle encoding is not declaration order, and `0x00` is reserved.**
The spec named the tension and imposed no preference. The choice was made on the
question `.lmbrain/knowledge/recurring-defects.md` gives for family 3 — *in quale
direzione sta il pericolo?* — and here the danger points one way. `app_leaf` is
rebuilt from stored state, and the zero byte is what a zero-filled, truncated or
uninitialized record yields for free in every language a node might be written
in. If `0x00` meant `active`, that accident would produce the **permissive**
state, and a leaf indistinguishable from a legitimately active one, so nothing
downstream could contradict it. With `0x00` reserved the same accident is a
rejection at the point it happens. The cost is one byte of intuition, and the
published `APP-0` fixture is what makes an implementer who assumed `0/1/2` find
out on the first run instead of on the first suspension.

**5. RF-109 was reformulated, not tightened.** The rules hold three properties:
no admitted configuration below 64 MiB; none below 196,608 KiB-passes; both RFC
profiles admitted. They do **not** hold "everything weaker than either
recommendation is rejected": the area form admits `iterations = 1` with
`196608 <= memory_kib < 2097152`, which matches neither recommendation.
Narrowing the rule to the two named profiles was rejected on the merits — a rule
that enumerates the current recommendations of one RFC is a whitelist that ages
the moment that RFC is revised, and this project already carries four
occurrences of a published value outliving the rule that made it true. The band
is now named with its cost, and two boundary rows exercise both of its edges.
The retraction of the superlative is itself pinned by a C10 probe, because
`recurring-defects.md` records what deleting a retraction cost here once.

**6. RF-110 needed both roads, not one.** The spec offered "reformulate **or**
change the rule". Reformulating alone would have left Part 1 of the admission
shield costing an attacker one round trip — which contradicts the section's own
argument that Part 1 is required and that the address cost is "the part of the
attack that does not scale with CPU". So the rule was strengthened: nonce
issuance is counted against the step-1 per-source rate limit, and a validator
declares a cap `k` on outstanding un-consumed nonces per source. But the rule
alone does not restore the original sentence either: with a cap `k` the cost is
one reachable address per **`k`** concurrent slots, not per slot. Both halves
were needed, and the sentence now states the bound in terms of the declared cap.
This is the rule change *Risks and open decisions* anticipated; the inventory was
built before it and run after it.

**7. A published example violated a stated validity rule, and this pass found
it.** README's preimage registry says "`challenge_id` MUST equal `request_hash`".
The canonical challenge-evidence example of `ledger.md` carried `challenge_id`
`sha256:3d56e5dd…` and `request_hash` `sha256:e14d4c02…`. It is a fifth
occurrence of family 1 in `recurring-defects.md`: a published artifact asserting
a state no conformant network can reach, mirrored into
`canonical_serialization.rs` where the crate pinned the inadmissible shape.
Corrected in both, declared in both documents, and now guarded by check C9.

**This is the one existing published hash literal that this pass changed**, and
it is a placeholder inside an inline example rather than a registry value — no
registry row changed, which `protocol_hashes.py` and the eighteen-row Rust suite
both demonstrate above. The value it now carries is the `challenge_id` the same
example already published; it is **not** a recomputation, because that example's
hash fields were never computed from anything. That fact is now written in the
documents, in *Inline examples are not conformance oracles*, instead of being
something a reader has to infer, and sixteen of the fifty-one published digests
are classified as placeholders in the manifest.

**8. A CI job was added, which the spec did not ask for.** The Python guards
were versioned and executed only when a person remembered to. [ADR-012] requires
a versioned tool and separately defers *continuous conformance over all
artifacts* as a not-yet-adopted alternative; running the guards themselves is
not that, and a guard nobody runs is the failure mode the ADR exists to close.
The new `protocol-docs` job runs the inventory, its negative proof,
`protocol_hashes.py` and `reward_rules.py`. No new action pin was introduced —
the job uses the runner's own `python3`.

**9. Acceptance criteria left unticked.** `CONTRACT.md` restricts a specialist
to implementation evidence on a spec. All twelve are believed satisfied by the
evidence above; the Lead ticks them.

**10. Scope held.** `RewardBounds` and the [SPEC-009] validity rules were not
touched ([SPEC-011]); no Ed25519 verifier ([SPEC-012]); no transport-key
separation ([SPEC-013]); no election or reward rule changed. The only rule
change is the RF-110 one, which *Risks and open decisions* authorized by name.

**Known limitations, stated rather than left to be discovered.** The sweep is
mechanical over four token classes plus one JSON-equality class. Prose is not
covered; the eleven C10 probes pin the passages that matter today, and the tool
says in its own docstring and in `meta.not_covered` that the probe list is not
claimed to be exhaustive. Twenty-five preimages still have no published fixture,
each with a written reason; the two clearest remaining gaps are
`enrollment_pow_salt` and `node_leaf`. Three preimages worth flagging to the
Lead beyond this spec: `validator_set_hash` is, alone among the
domain-separated preimages, **not** bound to `chain_id`; and neither the
transaction Merkle tree nor the subscription and eligible-set trees have a
published worked example, while the candidate-set tree does.

### Handoff status
- [x] Ready for Project Lead review
