---
id: REVIEW-005
# Note: Quote the title if it contains a colon
title: "Review di SPEC-002 — Scheletro del workspace Rust e CI multipiattaforma"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-002
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-008
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [test-quality]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-005-EVENT-001"
    timestamp: "2026-08-25T02:03:15.418327300+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-XXX"
  - schema_version: "1"
    id: "REVIEW-005-EVENT-002"
    timestamp: "2026-08-25T02:04:16.433034800+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "I tre difetti (script build-android.sh, wrapper Gradle, .gitignore di root) sono chiusi e verificati indipendentemente dal Lead: flag --platform corretto, gradle-wrapper.jar autentico da 43583 byte con distribuzione pinnata a 8.11.1, gitignore che copre target, node_modules, dist e src-tauri/gen con albero pulito e schema tolti dall'indice. Il Lead ha rieseguito fmt, clippy strict, cargo deny e i test del workspace: tutti verdi. La remediation ha inoltre scoperto sei problemi mai visti prima eseguendo realmente il runbook, incluso un flag Tauri inesistente che il Lead stesso aveva erroneamente citato come prova di conformita senza eseguirlo. GATE-LOCAL-REPRO soddisfatto con transcript reale; GATE-CI-GREEN derogato e coperto da DEBT-001. Un solo finding di severita bassa sulla scarsita dei test, conforme a quanto la spec chiedeva e non bloccante."
    evidence_refs: ["SPEC-002", "DEBT-001", ".lmbrain/knowledge/build-toolchain.md", ".gitignore"]
    implementation_agent: "AGENT-008"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> accepted"
---
# Review

## Outcome

**Accettata.** I tre difetti sono chiusi e verificati indipendentemente dal Lead. La remediation ha inoltre scoperto e corretto **sei problemi che nessuno aveva visto**, tutti dovuti al fatto che questa è la prima volta che la pipeline viene eseguita davvero invece che letta.

## Correzione di un errore del Lead

Va detto per primo, perché riguarda me. Nella nota che avevo scritto in questa spec avevo citato `npm run tauri -- build --bundles none` come **prova** che la pipeline fosse già conforme al vincolo "nessun installer". Non l'avevo eseguito: l'avevo dedotto leggendo il workflow.

Quel comando **non funziona**. La CLI Tauri v2 rifiuta `none` come nome di bundle (`invalid value 'none'`); il flag reale è `--no-bundle`. Verificato dal Lead in modo indipendente su `tauri build --help`:

```text
  -b, --bundles [<BUNDLES>...]   Space or comma separated list of bundles to package
      --no-bundle
```

L'implementatore ha corretto il flag in `ci.yml` e nel runbook. La conclusione che avevo tratto era giusta — la pipeline non produce installer — ma **la prova che avevo portato era falsa**, e se la CI avesse potuto girare quel job sarebbe fallito. È esattamente l'errore che ho contestato all'implementatore nel giro precedente: dedurre invece di eseguire.

## Acceptance-criteria compliance

| Criterio | Esito | Verifica del Lead |
| --- | --- | --- |
| `cargo build`/`test` sul workspace | Pass (derogato in CI, verificato in locale) | Eseguiti dal Lead: `cargo test --locked --workspace` verde, 2 test reali |
| Libreria Android e bindings Kotlin, test che chiama `core_version()` | Pass (derogato in CI, verificato in locale) | Il Lead aveva già prodotto il `.so` arm64 in autonomia; l'implementatore ha eseguito il test JVM sui bindings |
| App Tauri che mostra la versione dal core | Pass (derogato in CI, verificato in locale) | Transcript nell'evidenza: `Coblox desktop core version: 0.1.0` |
| `clippy -D warnings`, `rustfmt --check`, `cargo deny check` bloccanti | Pass | **Rieseguiti dal Lead**: fmt exit 0, clippy exit 0, `advisories ok, bans ok, licenses ok, sources ok` |
| Runbook riproducibile da un altro agente | Pass | 125 righe, aggiornato con `./gradlew`, `--platform 26` e `--no-bundle`, e con la spiegazione del perché `-p 26` fallisce |
| Pipeline < 20 minuti con cache calda | Derogato | Non misurabile senza una run: [DEBT-001] |
| DIFETTO-A chiuso | Pass | `scripts/build-android.sh` usa `--platform 26` |
| DIFETTO-B chiuso | Pass | Wrapper autentico: `gradle-wrapper.jar` è un archivio ZIP reale di 43.583 byte, distribuzione pinnata a Gradle 8.11.1 |
| DIFETTO-C chiuso | Pass | `.gitignore` di root copre `target/`, `**/target/`, `node_modules/`, `dist/`, output Android e `src-tauri/gen/`; l'albero è pulito e i 4 schema già committati sono stati tolti dall'indice |

## Code observations

I sei problemi trovati eseguendo davvero il runbook meritano di essere elencati, perché sono la dimostrazione del valore di `GATE-LOCAL-REPRO` e la ragione per cui non l'ho derogato insieme agli altri:

1. Disallineamento del target JVM fra Kotlin (17) e Java (1.8) in `apps/android/core/build.gradle.kts`.
2. Dipendenza implicita non dichiarata fra due task Gradle, che la validazione del grafo rifiutava.
3. Il flag Tauri inesistente di cui sopra.
4. `tauri-build` richiede `icons/icon.ico` anche senza bundling, e la cartella non esisteva.
5. `deny.toml` aveva `"LLVM-exception"` come licenza autonoma, che non è SPDX valido: la forma corretta è `"Apache-2.0 WITH LLVM-exception"`.
6. `build.rs` di `src-tauri` non era mai stato lintato, perché quel crate è escluso dal workspace — lacuna colmata aggiungendo fmt e clippy dedicati sia in CI sia nel runbook.

Nessuno di questi era nei tre difetti che avevo nominato. Sono emersi solo perché la pipeline è stata guidata da capo a fondo.

Apprezzabile anche il metodo su due punti: il wrapper Gradle è stato generato scaricando ed eseguendo una distribuzione vera invece di scrivere a mano i file, e il problema delle dipendenze wildcard di `cargo deny` è stato risolto con `publish = false` sui crate interni più `allow-wildcard-paths`, motivato sulla documentazione, invece che con un bypass generico.

## Tests and verification

`GATE-LOCAL-REPRO` (owner: agent, before-submit, **non derogato**): soddisfatto, con transcript reale nell'evidenza. Il Lead ha rieseguito in proprio la parte verificabile in fretta:

```text
cargo fmt --all -- --check          -> exit 0
cargo clippy --workspace --all-targets --all-features -- -D warnings -> exit 0
cargo deny check                    -> advisories ok, bans ok, licenses ok, sources ok
cargo test --locked --workspace     -> verde, 2 test passati
tauri build --help                  -> conferma che --no-bundle esiste e --bundles none no
gradle-wrapper.jar                  -> Zip archive data, 43583 byte (wrapper autentico)
```

`GATE-CI-GREEN` (owner: lead, before-done): **derogato**, coperto da [DEBT-001]. La fatturazione GitHub blocca l'esecuzione di qualsiasi job. Alla ripresa, DEBT-001 impone una run verde e la ri-attestazione.

## Production quality and documentation compliance

Conforme a [[QUALITY]]. L'implementatore ha ritrattato esplicitamente nella spec la propria affermazione errata sul blocco NDK, invece di lasciarla agli atti. Confini rispettati: nessun commit né push, nessun job di release aggiunto, niente toccato in `docs/protocol/` o nel pacchetto di design.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

- RF-B01 | category=test-quality | severity=low | criterion=test unitari reali | remediation=Il workspace ha 2 test in tutto, uno in `coblox-core` e uno in `coblox-ffi`; `coblox-node` non ne ha alcuno. È conforme a quanto la spec chiedeva ("test banali ma reali" su crate segnaposto) e non blocca la chiusura, ma va registrato: da qui in avanti `cargo test --workspace` verde dirà molto poco finché la copertura non cresce con il codice vero. Non apro un debito perché il rimedio naturale è la crescita del codice in M-02, non un intervento separato.

## Required follow-up

1. **[DEBT-001]** resta aperto e vincolante: alla ripresa della fatturazione GitHub serve una run completamente verde, e solo allora `GATE-CI-GREEN` viene ri-attestato dal Lead. Fino a quel momento nessuna affermazione sulla tenuta cross-platform è dimostrata.
2. Nota per il Lead, valida per tutte le review future: **non citare come prova un comando che non si è eseguito.** Questa review ne contiene un esempio a mio carico.

## Final decision

Accettata. `GATE-LOCAL-REPRO` soddisfatto e verificato in parte dal Lead; `GATE-CI-GREEN` derogato con debito tracciato. SPEC-002 può passare a `done`.
