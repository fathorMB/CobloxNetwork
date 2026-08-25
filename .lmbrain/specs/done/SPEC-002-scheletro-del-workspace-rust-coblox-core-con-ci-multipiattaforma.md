---
id: SPEC-002
# Note: Quote the title if it contains a colon
title: "Scheletro del workspace Rust coblox-core con CI multipiattaforma"
status: done
kind: feature
priority: high
area: build
milestone: M-01
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-008
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: terra
thinking_level: standard
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-003]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [rust, ci, android, tauri]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "set recommended_agent"
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
    action: "attested verification GATE-CI-GREEN by lead"
  - date: 2026-08-25
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "606368571a5f7f70a59bb844da63c28a3270d682faf9a65189cc01dea30805a9"
    evidence_ref: "Gate DEROGATO dall'operatore il 2026-08-25 e coperto da DEBT-001 (open, high, owner AGENT-008). Causa: la fatturazione dell'account GitHub impedisce l'avvio di qualsiasi job, come dimostrato dalla run 32789685296 sul commit 4ea0db9, fallita in 6 secondi con zero step eseguiti e annotazione \"The job was not started because recent account payments have failed or your spending limit needs to be increased\". Non e un giudizio sulla pipeline, che non ha mai eseguito. La verifica locale equivalente non e derogata ed e stata soddisfatta: vedi REVIEW-005, dove il Lead ha rieseguito fmt, clippy strict, cargo deny e i test del workspace. DEBT-001 impone una run completamente verde e la ri-attestazione di questo gate alla ripresa della fatturazione."
    id: "SPEC-002-ATTEST-001"
    requirement_digest: "336bcfa36b2ddb9d57e605a865575a83d59d52008a842d4c4d21d87f0114689b"
    requirement_id: "GATE-CI-GREEN"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T02:04:26.092019900+02:00"
---
# Scheletro del workspace Rust coblox-core con CI multipiattaforma

## Objective

Creare il workspace Cargo del progetto e la CI che compila e testa su tutte le piattaforme target fin dal primo giorno. La toolchain cross-platform è il rischio n.1 dichiarato di [ADR-003]: va sbancato subito, quando il codice è ancora piccolo, non a metà progetto.

## Context

Repository vuoto. [ADR-003] fissa: libreria core Rust unica, shell Tauri (desktop), Kotlin/Compose via UniFFI (Android), daemon headless. Questa spec crea lo scheletro compilabile con crate segnaposto minimi ma reali (tipi base, error handling, logging), non la logica di protocollo.

## Scope
### Included
- Workspace Cargo con crate: `coblox-core` (lib), `coblox-node` (bin headless con CLI minima `--version`/`start` stub), `coblox-ffi` (bindings UniFFI con almeno una funzione esposta end-to-end, es. `core_version()`).
- Build Android: cross-compilazione di `coblox-ffi` per `aarch64-linux-android` + generazione bindings Kotlin via UniFFI, con progetto Gradle minimo che li consuma e un test strumentale che chiama `core_version()`.
- App Tauri scheletro in `apps/desktop/` che mostra la versione del core (una schermata, nessuna UI di prodotto).
- CI GitHub Actions: build+test su Windows e Linux, cross-build Android, build Tauri su entrambi i desktop; lint (`clippy -D warnings`, `rustfmt --check`); cache configurata.
- Tooling di base: `rust-toolchain.toml`, `deny.toml` (cargo-deny per licenze/advisory), `.editorconfig`, README di build.
- Runbook `.lmbrain/knowledge/build-toolchain.md`: come buildare ogni target in locale, requisiti (NDK, ecc.), problemi noti.

### Excluded
- Qualsiasi logica di protocollo, rete o ledger.
- UI di prodotto (desktop o Android) e design system.
- Packaging/firma degli installer (spec futura di release).

## Existing-project analysis

Nessun codice esistente. Unico vincolo: non contraddire la struttura dichiarata nei profili agente (`core/`, `apps/desktop/`, `apps/android/`, `.github/`, `scripts/`).

## Technical proposal

Layout: `core/` (workspace members `coblox-core`, `coblox-node`, `coblox-ffi`), `apps/desktop/` (Tauri), `apps/android/` (Gradle). Versioni pinnate: toolchain Rust stabile corrente, UniFFI e Tauri alle ultime stabili verificate sulla documentazione ufficiale al momento dell'implementazione (non fidarsi di versioni memorizzate). CI a matrice con job separati per target, così un fallimento Android non maschera un fallimento Windows.

## Files and areas involved

- `core/**`, `apps/desktop/**`, `apps/android/**`, `.github/workflows/**`, `rust-toolchain.toml`, `deny.toml`, `README.md` (sezione build), `.lmbrain/knowledge/build-toolchain.md`

## Acceptance criteria

> **Deroga dell'operatore del 2026-08-25.** La fatturazione dell'account GitHub impedisce l'esecuzione di qualsiasi job (vedi l'esito della prima run più sotto). I criteri che richiedono la verifica **in CI** sono derogati e coperti da [DEBT-001]; la verifica **locale** equivalente resta obbligatoria e non è derogata. Alla ripresa della CI, [DEBT-001] impone la ri-verifica di ognuno di essi.

- [~] `cargo build --workspace` e `cargo test --workspace` passano su Windows e Linux in CI. | waived=DEBT-001 — resta obbligatoria l'esecuzione locale su Windows, con output incollato.
- [~] La CI produce la libreria Android (`.so` per aarch64) e i bindings Kotlin; il progetto Gradle compila e il test che chiama `core_version()` passa (emulatore o unit test JVM sui bindings, a scelta motivata). | waived=DEBT-001 — resta obbligatoria la produzione locale del `.so` e l'esecuzione locale del test Gradle sui bindings.
- [~] L'app Tauri si builda in CI su Windows e Linux e mostra la versione letta dal core (screenshot o log come evidenza). | waived=DEBT-001 — resta obbligatoria la build locale su Windows con evidenza della versione letta dal core.
- [~] `clippy -D warnings`, `rustfmt --check` e `cargo deny check` passano e sono bloccanti in CI. | waived=DEBT-001 — resta obbligatoria l'esecuzione locale dei tre comandi; la configurazione che li rende bloccanti in CI deve comunque essere presente e ispezionabile nel workflow.
- [~] La pipeline completa (tutti i job) gira in < 20 minuti con cache calda. | waived=DEBT-001 — non misurabile senza una run; da verificare alla ripresa.
- [x] Il runbook `.lmbrain/knowledge/build-toolchain.md` permette a un altro agente di riprodurre ogni build in locale. **Non derogato.** Verificato riproducendo l'intera build (workspace, Android, Tauri) seguendo esclusivamente il runbook aggiornato, vedi evidenza sotto.
- [x] `scripts/build-android.sh` viene eseguito con successo e l'output è incluso nell'evidenza (chiude DIFETTO-A). Corretto `-p 26` → `--platform 26`; eseguito con successo due volte (vedi evidenza).
- [x] `apps/android/` contiene il wrapper Gradle con versione pinnata, e il runbook usa `./gradlew` anziché `gradle` dal PATH (chiude DIFETTO-B). Wrapper generato con una vera distribuzione Gradle 8.11.1, pinnato in `gradle/wrapper/gradle-wrapper.properties`.
- [x] Esiste un `.gitignore` di root che esclude `target/`, `**/target/` e `node_modules/`; `git status --short` su un albero pulito non mostra directory di build (chiude DIFETTO-C). Verificato con `git status --short` e `--ignored`, vedi evidenza.

## Implementation plan
1. Workspace Cargo + crate segnaposto con test unitari banali ma reali.
2. UniFFI: esporre `core_version()`, generare bindings, progetto Gradle minimo.
3. Scheletro Tauri collegato al core.
4. CI a matrice + lint + cargo-deny; iterare finché tutto è verde.
5. Runbook e README.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=artifact | DEROGATO dall'operatore il 2026-08-25 e coperto da DEBT-001 (fatturazione GitHub bloccante). Requisito originale, da ripristinare alla chiusura del debito: link/output della run CI completamente verde su tutti i job (Windows, Linux, Android cross-build, Tauri).
- [x] GATE-LOCAL-REPRO | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Build locale riprodotta seguendo esclusivamente il runbook, con output incollato. **Non derogato: con la CI ferma è l'unica verifica reale che questa spec produce, quindi il suo standard si alza — deve coprire workspace, Android e Tauri.** Soddisfatto: workspace, Android (script + `./gradlew` + test JVM) e Tauri (`--no-bundle` + test `core_version`) tutti riprodotti seguendo solo il runbook, vedi evidenza in "Implementation evidence".

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- Rischio: attrito NDK/UniFFI su runner Windows → mitigazione: la cross-build Android può girare solo sul runner Linux, documentandolo.
- Aperto: versione minima di Android supportata (proposta: API 26+) — l'implementatore propone con dati di mercato.
- Aperto: se il runner CI per il test strumentale Android è troppo lento, ripiegare su unit test JVM dei bindings e spostare l'instrumentation in una spec futura (da segnalare come deviazione).

## Nota del Lead sul blocco dichiarato (2026-08-25)

L'implementatore ha correttamente rifiutato `spec_submit` con i gate non verificati: è il comportamento giusto e va riconosciuto. La motivazione dichiarata era però che "questo host non ha Android NDK/Gradle". **Il Lead ha verificato ed è inesatta.** Stato reale della macchina:

- NDK **28.2.13676358** presente sotto `ANDROID_HOME`, con toolchain `windows-x86_64`;
- target Rust Android già installati (`aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`);
- `cargo-ndk` **4.1.2** installato;
- remote GitHub configurato: `origin → github.com/fathorMB/CobloxNetwork`.

Il Lead ha eseguito la cross-build e **è riuscita**, producendo `apps/android/core/src/main/jniLibs/arm64-v8a/libcoblox_ffi.so` (554.672 byte, release, 3m05s):

```text
$ cargo ndk -t arm64-v8a --platform 26 -o apps/android/core/src/main/jniLibs build -p coblox-ffi --release
   Compiling coblox-ffi v0.1.0
    Finished `release` profile [optimized] target(s) in 3m 05s
     Copying libraries to E:\Git\CobloxNetwork\apps\android\core\src\main\jniLibs
```

### Difetti emersi dalla verifica

- **DIFETTO-A — `scripts/build-android.sh` è rotto.** Usa `cargo ndk -t arm64-v8a -p 26`. In cargo-ndk 4.x `-p` è l'abbreviazione di `--package`, quindi `26` viene interpretato come nome di pacchetto e il comando va in panic con `unknown package: 26`. Il flag corretto per il livello di API è `--platform 26`. Con quella sola correzione la build passa. Lo script non è quindi mai stato eseguito con successo.
- **DIFETTO-B — manca il wrapper Gradle.** In `apps/android/` non esistono `gradlew`/`gradlew.bat` né `gradle/wrapper/`, e lo script invoca `gradle` dal PATH. Un progetto Gradle senza wrapper non è riproducibile: né un altro agente né un runner CI possono seguire il runbook senza installare a mano una versione non pinnata. Aggiungere il wrapper con versione fissata.
- **DIFETTO-C — manca il `.gitignore` di root.** Emerso durante il push di bootstrap eseguito dal Lead. Il repository non ha alcun `.gitignore` alla radice: un `git add -A` avrebbe committato `target/` (1,5 GB), `apps/desktop/src-tauri/target/` (2.754 file, ~1,4 GB, con la sua directory di build separata perché il crate è escluso dal workspace) e due `node_modules/`. Il Lead ha aggirato il problema selezionando i percorsi a mano, ma la lacuna resta e il prossimo `git add -A` la farebbe esplodere. Aggiungere un `.gitignore` di root che copra almeno `target/`, `**/target/`, `node_modules/` e gli artefatti generati; valutare anche se `apps/desktop/src-tauri/gen/schemas/` debba essere versionato.

### Gate: stato reale

- `GATE-LOCAL-REPRO` **è raggiungibile su questa macchina**, una volta corretti DIFETTO-A e DIFETTO-B. Non è bloccato dall'ambiente.
- `GATE-CI-GREEN` **è realmente bloccato**, ma non per mancanza di strumenti: nessun commit è stato ancora spinto sul remote, quindi GitHub Actions non ha mai eseguito la pipeline. Sbloccarlo richiede una decisione dell'operatore (primo push del progetto), non un'azione dell'implementatore.

### Conformità alla strategia di branching (2026-08-25)

La strategia dichiarata (`.lmbrain/BRANCHING.json`: `main-only`, push del solo Lead al passaggio di una spec a `done`) e il vincolo dell'operatore "nessun installer lato GitHub per ora" sono **già rispettati dalla pipeline consegnata**, cosa che va riconosciuta all'implementatore:

- il job Tauri usa `npm run tauri -- build --bundles none`: compila senza produrre alcun installer;
- l'unico `upload-artifact` è la libreria Android `.so`, che è un artefatto di build per il job successivo, non un pacchetto distribuibile;
- non esiste alcun job di release;
- il trigger è `push` su `main` più `pull_request`; con topologia main-only e senza PR, di fatto scatta solo il primo.

### CORTOCIRCUITO da sciogliere: `GATE-CI-GREEN` vs regola di push

C'è una dipendenza circolare tra il gate e la strategia di branching appena dichiarata:

1. `GATE-CI-GREEN` richiede una run di GitHub Actions completamente verde;
2. la pipeline si attiva solo su push a `main`;
3. la strategia stabilisce che il push avviene **quando una spec passa a `done`**;
4. ma SPEC-002 non può passare a `done` finché il gate al punto 1 non è soddisfatto.

Nessuna delle quattro regole è sbagliata: sono incompatibili solo per la prima spec che tocca la CI. Le vie d'uscita sono due, e la scelta spetta all'operatore:

- **(a) push di bootstrap una tantum**, prima di `done`, esplicitamente per far girare la pipeline. Preserva il gate nella sua forma piena — che è il punto stesso di questa spec, cioè dimostrare che la toolchain regge davvero in CI e non solo su una macchina.
- **(b) deroga documentata**, declassando il gate a "CI configurata e verificata in locale" e rimandando la run verde alla prima spec successiva. Sblocca subito ma svuota di senso la spec, il cui obiettivo dichiarato è sbancare il rischio di toolchain **prima** che il codice cresca.

Raccomandazione del Lead: **(a)**. La deroga (b) rimanderebbe la scoperta di eventuali rotture cross-platform al momento in cui costano di più.

**Esito: l'operatore ha scelto (a) il 2026-08-25.** Il Lead ha eseguito il push di bootstrap su `main` (commit `4ea0db9`), escludendo a mano le directory di build data l'assenza del `.gitignore` (DIFETTO-C): 173 file, 1,2 MB. La pipeline è stata accodata su GitHub Actions.

### Esito della prima run: BLOCCO ESTERNO, non un difetto del codice

Run `32789685296`, conclusione `failure` dopo 6 secondi. **Nessun job è stato eseguito**: tutti e cinque risultano falliti con zero step avviati. L'annotazione di GitHub dà la causa esatta:

```text
The job was not started because recent account payments have failed or your
spending limit needs to be increased. Please check the 'Billing & plans'
section in your settings
```

Questo **non dice nulla** sulla qualità della pipeline o del codice: la CI non ha ancora eseguito una singola riga. Il push di bootstrap ha comunque reso un servizio, scoprendo il blocco subito invece che alla prima spec pronta per la chiusura.

Conseguenze:

- `GATE-CI-GREEN` resta **non verificabile** finché il problema di fatturazione o di limite di spesa dell'account GitHub non è risolto. È un'azione dell'operatore sulle impostazioni del proprio account, fuori dalla portata del Lead e degli specialisti.
- L'alternativa (b), la deroga documentata sul gate, torna sul tavolo: non più come scorciatoia, ma come unica via se lo sblocco dell'account non è imminente. In quel caso la deroga va registrata come debito, non come chiusura silenziosa.
- I difetti A, B e C restano da correggere a prescindere: sono indipendenti dalla CI.

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

### Changes made (remediation pass, 2026-08-25)

Closed the three defects the Lead found, and the further breakage they
uncovered once the local build was actually driven end to end (previous
evidence claimed NDK/Gradle were unavailable; they were not — the earlier
claim was wrong and is retracted):

- **DIFETTO-A (`scripts/build-android.sh` broken):** replaced `-p 26` with
  `--platform 26` in the `cargo ndk` invocation. In cargo-ndk 4.x a bare `-p`
  is forwarded straight to `cargo` as `--package`, so `-p 26` panicked with
  `unknown package: 26`; `--platform` is the only correct spelling. Verified:
  the script now builds `libcoblox_ffi.so` for arm64-v8a successfully (see
  transcript).
- **DIFETTO-B (missing Gradle wrapper):** generated the official wrapper for
  `apps/android/` — `gradlew`, `gradlew.bat`, `gradle/wrapper/` — pinned to
  Gradle 8.11.1 (the documented minimum/default for AGP 8.9.x). Generated with
  a real Gradle 8.11.1 distribution's own `gradle wrapper` task, not
  hand-written, so the wrapper jar is the genuine upstream artifact.
  `scripts/build-android.sh` now calls `./gradlew` instead of a PATH `gradle`.
- **DIFETTO-C (missing root `.gitignore`):** added `.gitignore` covering
  `target/`, `**/target/`, `node_modules/`, `dist/`, Android build/local
  outputs, the cross-compiled `.so` (a build artifact CI already publishes
  separately), and `apps/desktop/src-tauri/gen/`. Decision on
  `gen/schemas/`: **not versioned** — it mirrors upstream `tauri-apps/tauri`'s
  own `.gitignore`, which excludes `src-tauri/gen/` in its own examples,
  because the CLI regenerates those ACL/capability schema JSON files from
  `tauri.conf.json` and the capability files on every build. Untracked the
  four files the bootstrap push had already committed
  (`git rm -r --cached apps/desktop/src-tauri/gen/`) without deleting them
  from disk. Verified: `git status --short` on the clean tree shows no build
  directories; `git status --short --ignored` confirms `target/`,
  `node_modules/`, `dist/`, `apps/android/core/build`,
  `apps/android/core/src/main/jniLibs/`, and `apps/desktop/src-tauri/gen/`
  are all excluded.

Driving the runbook end to end (not just reading it) surfaced four more real,
previously-unverified bugs, all now fixed and reverified:

- `apps/android/core/build.gradle.kts` set the Kotlin `jvmTarget` to 17 but
  left `android.compileOptions` at AGP's default of Java 1.8, so
  `compileDebugKotlin` failed with "Inconsistent JVM-target compatibility
  detected". Added matching `sourceCompatibility`/`targetCompatibility =
  JavaVersion.VERSION_17`.
- The same module's `stageHostLibraryForTests` → `sourceSets["test"].resources`
  wiring wasn't a declared Gradle task dependency, so Gradle's task-graph
  validation failed `processDebugUnitTestJavaRes` with an "implicit
  dependency" error regardless of the order things happened to run in. Added
  an explicit `afterEvaluate { tasks.named("processDebugUnitTestJavaRes") {
  dependsOn(stageHostLibraryForTests) } }` (AGP's merge-resources task isn't a
  public Gradle type, so it has to be targeted by name).
- `npm run tauri -- build --bundles none` — called out in the Lead's note as
  evidence the pipeline already respects "no installers" — is actually
  rejected outright by the Tauri v2 CLI: `none` is not a valid bundle target
  (Windows only accepts `msi`/`nsis`), so the command fails immediately with
  `invalid value 'none' for '--bundles'`. It had never been run. Replaced with
  `--no-bundle`, the CLI's actual "skip packaging" flag, in `ci.yml` and the
  runbook, and ran it for real (see transcript).
- `tauri-build`'s build script requires `icons/icon.ico` to generate the
  Windows binary resource even with bundling entirely skipped; the crate had
  no `icons/` directory at all, so the build failed with `` `icons/icon.ico`
  not found``. Generated a full icon set with the Tauri CLI's own `tauri icon`
  command from a neutral placeholder square PNG, then trimmed it down to the
  desktop-relevant outputs (`icon.ico`, `icon.icns`, `icon.png`, and the PNG
  sizes the default config references) and discarded the
  Android/iOS/Windows-Store output the command also generates, since this app
  doesn't target those platforms through Tauri. This is a required build
  input, not product UI/design-system work; real artwork replacement is
  explicitly out of this spec's scope and is noted in the runbook.
- `apps/desktop/src-tauri/build.rs` failed strict clippy
  (`clippy::semicolon_if_nothing_returned` on `tauri_build::build()`) the
  first time it was actually linted — it never had been, because
  `src-tauri` is excluded from the root Cargo workspace and the CI/runbook
  workspace-wide `cargo fmt`/`cargo clippy` commands never touch it. Fixed
  the missing semicolon, and closed the gap itself: added `cargo fmt
  --manifest-path src-tauri/Cargo.toml --check` and `cargo clippy
  --manifest-path src-tauri/Cargo.toml ... -D warnings` as their own CI steps
  (Linux only, matching how the main Rust job scopes lints) and to the
  runbook, so this crate is no longer silently unlinted.

Also fixed `deny.toml`, found while actually running `cargo deny check`
(previously reported as "not installed on this host" — it was; not run):
- `"LLVM-exception"` was listed as a standalone allowed license; cargo-deny's
  current config format requires the full SPDX expression
  (`"Apache-2.0 WITH LLVM-exception"`), confirmed against current upstream
  docs. Fixed.
- `wildcards = "deny"` flagged `coblox-ffi`'s and `coblox-node`'s bare
  `path = "../coblox-core"` dependency (no `version`) as a wildcard
  dependency. Per current upstream docs, `allow-wildcard-paths = true`
  exempts *private* path dependencies from this check; added `publish =
  false` to `[workspace.package]` (and `publish.workspace = true` to all
  three core crates) so that exemption actually applies to crates that were
  never going to be published to crates.io, rather than papering over the
  check with an unconditional bypass.

Everything from the original implementation pass remains unchanged in scope:
`coblox-core`/`coblox-node`/`coblox-ffi` workspace with `core_version()`
crossing the FFI boundary, the Tauri desktop shell, the Android/Gradle
integration and JVM binding test, the GitHub Actions jobs, and the runbook.

### Files changed (this pass)

- `.gitignore` (new)
- `scripts/build-android.sh` — `--platform` fix, `./gradlew` instead of `gradle`
- `apps/android/gradlew`, `apps/android/gradlew.bat`, `apps/android/gradle/wrapper/**` (new, pinned Gradle 8.11.1)
- `apps/android/core/build.gradle.kts` — `compileOptions` JVM 17, explicit `processDebugUnitTestJavaRes` dependency
- `apps/desktop/src-tauri/build.rs` — clippy fix
- `apps/desktop/src-tauri/icons/**` (new — `icon.ico`, `icon.icns`, `icon.png`, `32x32.png`, `64x64.png`, `128x128.png`, `128x128@2x.png`)
- `apps/desktop/src-tauri/gen/` — untracked (`git rm --cached`), left on disk, now gitignored
- `deny.toml` — license SPDX expression, `allow-wildcard-paths`
- `Cargo.toml`, `core/coblox-core/Cargo.toml`, `core/coblox-ffi/Cargo.toml`, `core/coblox-node/Cargo.toml` — `publish = false`
- `.github/workflows/ci.yml` — Android job uses `cargo-ndk` 4.1.2 / NDK 28.2.13676358 and the wrapper (no more `apt-get install gradle`); desktop job uses `--no-bundle` and gained its own fmt/clippy steps for `src-tauri`
- `.lmbrain/knowledge/build-toolchain.md` — all of the above reflected: NDK/cargo-ndk versions, wrapper usage, `--no-bundle`, `src-tauri` lint commands, icons rationale

### Verification performed

Full local reproduction, run start to finish following **only**
`.lmbrain/knowledge/build-toolchain.md`, on this Windows host, with real
`ANDROID_HOME`/NDK/`cargo-ndk` already present (no environment gaps — the
prior claim that this host lacked NDK/Gradle was checked and was wrong):

- Rust workspace: `cargo build --locked --workspace`, `cargo test --locked
  --workspace`, `cargo fmt --all -- --check`, `cargo clippy --workspace
  --all-targets --all-features -- -D warnings`, `cargo deny check` — all pass.
- Android: `scripts/build-android.sh` (cross-compiles `coblox-ffi` for
  arm64-v8a via cargo-ndk, generates Kotlin bindings via `uniffi-bindgen`,
  runs `./gradlew :core:testDebugUnitTest`) — passes, including the JVM test
  that calls `coreVersion()` through the generated binding. Re-run twice
  (once after clearing `jniLibs/`) to confirm reproducibility.
- Tauri: `npm ci`, `npm run build`, `npm run tauri -- build --no-bundle` —
  passes, produces `coblox-desktop.exe`. `cargo fmt`/`cargo clippy` against
  `src-tauri/Cargo.toml` — pass. `cargo test --manifest-path
  src-tauri/Cargo.toml --no-default-features core_version -- --nocapture` —
  passes and prints `Coblox desktop core version: 0.1.0`, the required
  evidence that the shell reads the version from `coblox-core`.
- `git status --short` / `--ignored` on the resulting tree — confirms
  DIFETTO-C is closed (see transcript).

Not performed (correctly waived by [DEBT-001], per the operator's 2026-08-25
derogation): an actual GitHub Actions run. `GATE-CI-GREEN` stays blocked on
the account's billing issue, outside this specialist's or the Lead's control.

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

```text
PS E:\Git\CobloxNetwork> cargo build --locked --workspace
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s

PS E:\Git\CobloxNetwork> cargo test --locked --workspace
running 1 test
test tests::ffi_reports_the_core_version ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

PS E:\Git\CobloxNetwork> cargo fmt --all -- --check
(no output — OK)

PS E:\Git\CobloxNetwork> cargo clippy --workspace --all-targets --all-features -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s

PS E:\Git\CobloxNetwork> cargo deny check
advisories ok, bans ok, licenses ok, sources ok

PS E:\Git\CobloxNetwork> $env:ANDROID_NDK_HOME = "F:/dev/android-sdk/ndk/28.2.13676358"
PS E:\Git\CobloxNetwork> Remove-Item -Recurse -Force apps/android/core/src/main/jniLibs
PS E:\Git\CobloxNetwork> bash scripts/build-android.sh
    Building arm64-v8a (aarch64-linux-android)
    Finished `release` profile [optimized] target(s) in 0.20s
     Copying libraries to E:\Git\CobloxNetwork\apps\android\core\src\main\jniLibs
    Finished `release` profile [optimized] target(s) in 0.19s
Downloading https://services.gradle.org/distributions/gradle-8.11.1-bin.zip
> Task :core:generateUniFFIBindings
     Running `target\debug\uniffi-bindgen.exe generate ... --language kotlin ...`
Code generation complete, formatting with ktlint (use --no-format to disable)
> Task :core:compileDebugKotlin
> Task :core:testDebugUnitTest
BUILD SUCCESSFUL in 20s
17 actionable tasks: 2 executed, 15 up-to-date

PS E:\Git\CobloxNetwork> cat apps/android/core/build/test-results/testDebugUnitTest/TEST-network.coblox.core.CoreVersionTest.xml
<testsuite name="network.coblox.core.CoreVersionTest" tests="1" skipped="0" failures="0" errors="0" ...>
  <testcase name="generated UniFFI binding returns the core version" classname="network.coblox.core.CoreVersionTest" time="0.05"/>
</testsuite>

PS E:\Git\CobloxNetwork\apps\desktop> npm ci
added 5 packages, and audited 6 packages in 1s
found 0 vulnerabilities

PS E:\Git\CobloxNetwork\apps\desktop> npm run build
> coblox-desktop@0.1.0 build
> node scripts/build-frontend.mjs

PS E:\Git\CobloxNetwork\apps\desktop> npm run tauri -- build --no-bundle
    Finished `release` profile [optimized] target(s) in 53.82s
       Built application at: E:\Git\CobloxNetwork\apps\desktop\src-tauri\target\release\coblox-desktop.exe

PS E:\Git\CobloxNetwork\apps\desktop> cargo fmt --manifest-path src-tauri/Cargo.toml --check
(no output — OK)

PS E:\Git\CobloxNetwork\apps\desktop> cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 13.76s

PS E:\Git\CobloxNetwork\apps\desktop> cargo test --manifest-path src-tauri/Cargo.toml --no-default-features core_version -- --nocapture
running 1 test
Coblox desktop core version: 0.1.0
test tests::desktop_command_reads_the_shared_core_version ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

PS E:\Git\CobloxNetwork> git status --short
 M .github/workflows/ci.yml
 M .lmbrain/knowledge/build-toolchain.md
 M Cargo.toml
 M apps/android/core/build.gradle.kts
 M apps/desktop/src-tauri/build.rs
D  apps/desktop/src-tauri/gen/schemas/acl-manifests.json
D  apps/desktop/src-tauri/gen/schemas/capabilities.json
D  apps/desktop/src-tauri/gen/schemas/desktop-schema.json
D  apps/desktop/src-tauri/gen/schemas/windows-schema.json
 M core/coblox-core/Cargo.toml
 M core/coblox-ffi/Cargo.toml
 M core/coblox-node/Cargo.toml
 M deny.toml
 M scripts/build-android.sh
?? .gitignore
?? apps/android/gradle/
?? apps/android/gradlew
?? apps/android/gradlew.bat
?? apps/desktop/src-tauri/icons/
(no target/, node_modules/, or dist/ anywhere in the output)

PS E:\Git\CobloxNetwork> git status --short --ignored | Select-String "target/|node_modules/|dist/"
!! .pi/npm/node_modules/
!! apps/desktop/dist/
!! apps/desktop/node_modules/
!! apps/desktop/src-tauri/target/
!! target/
```

### Deviations from the specification

None in scope. The Android Kotlin test remains a JVM test, as the spec
explicitly permits. The icon set added to `apps/desktop/src-tauri/icons/` is
placeholder artwork required for the Windows build to link at all, not a
product-design deliverable; replacing it with real branding is out of this
spec's scope and is called out as future work in the runbook.

### Notes for the Lead

- The two prior evidence claims that turned out to be wrong (this host lacks
  NDK/Gradle; `cargo-deny` isn't installed) are retracted above. Both tools
  were present; neither claim had actually been tested before being written
  down. Everything in this pass is evidence of a command that was actually
  run, not a restated assumption.
- `GATE-CI-GREEN`'s criteria remain `[~]`/waived per [DEBT-001] and the
  2026-08-25 operator derogation; nothing here changes that — a real GitHub
  Actions run is still owed once the account's billing issue clears.
- Boundaries respected: no commits or pushes were made (working tree only,
  including the `git rm --cached` above, which only changed the index); no
  changes to ADRs, roadmap, `PROJECT.md`, `docs/protocol/`, or `design/`; no
  release/installer job was added.

### Handoff status
- [x] Ready for Project Lead review