---
id: DEBT-001
title: "La pipeline CI non e mai stata eseguita: GATE-CI-GREEN derogato"
status: resolved
category: "verification"
severity: "high"
origin_severity: null
area: "build"
milestone: "M-01"
owner: "AGENT-008"
origin_artifact: "SPEC-002"
origin_ref: "GATE-CI-GREEN"
related_specs: ["SPEC-002"]
related_reviews: []
related_decisions: ["ADR-003"]
target_specs: []
blocked_by: []
resolution_refs: ["SPEC-002","ADR-003"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["ci","verification-gap","toolchain"]
links: []
activity:
  - date: 2026-08-25
    action: "resolved: La fatturazione GitHub e stata sbloccata dall'operatore il 2026-08-25 e la pipeline ha eseguito per la prima volta. La prima run reale ha rivelato tre rotture, tutte corrette come remediation prima della chiusura come i criteri richiedevano; la run successiva e completamente verde su tutti e cinque i job. Il rischio numero uno di ADR-003 e sbancato con un esito preciso: nessuna delle tre rotture era un problema di toolchain cross-platform. Erano difetti di confezionamento del repository (due bit di esecuzione persi perche i file sono stati committati da Windows) e un ordine di step sbagliato nel workflow. Rust, NDK, cargo-ndk, UniFFI e Tauri hanno funzionato al primo tentativo utile su entrambi i sistemi operativi."
debt_events:
  - schema_version: "1"
    id: "DEBT-001-EVENT-001"
    timestamp: "2026-08-25T01:35:01.134765400+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Deroga richiesta esplicitamente dall'operatore il 2026-08-25 per non tenere bloccata M-01 su un impedimento amministrativo esterno al progetto. Registrata come debito e non come chiusura silenziosa, cosi che la verifica mancante resti visibile e recuperabile."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-001-EVENT-002"
    timestamp: "2026-08-25T09:40:15.262482900+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "La fatturazione GitHub e stata sbloccata dall'operatore il 2026-08-25 e la pipeline ha eseguito per la prima volta. La prima run reale ha rivelato tre rotture, tutte corrette come remediation prima della chiusura come i criteri richiedevano; la run successiva e completamente verde su tutti e cinque i job. Il rischio numero uno di ADR-003 e sbancato con un esito preciso: nessuna delle tre rotture era un problema di toolchain cross-platform. Erano difetti di confezionamento del repository (due bit di esecuzione persi perche i file sono stati committati da Windows) e un ordine di step sbagliato nel workflow. Rust, NDK, cargo-ndk, UniFFI e Tauri hanno funzionato al primo tentativo utile su entrambi i sistemi operativi."
    evidence_refs: ["SPEC-002", "ADR-003"]
---
# La pipeline CI non e mai stata eseguita: GATE-CI-GREEN derogato

## Statement

La pipeline GitHub Actions del progetto non ha mai eseguito una singola riga. Il gate GATE-CI-GREEN di SPEC-002 ("run CI completamente verde su tutti i job: Windows, Linux, cross-build Android, Tauri") e stato derogato dall'operatore per sbloccare la milestone, e con esso restano non verificati in ambiente CI i criteri di accettazione della spec che dipendono dall'esecuzione della pipeline. Tutte le verifiche corrispondenti esistono solo come esecuzione locale su una singola macchina Windows.

## Evidence and provenance

Run GitHub Actions 32789685296 sul commit 4ea0db9 (push di bootstrap eseguito dal Lead il 2026-08-25): conclusione failure dopo 6 secondi, cinque job tutti falliti con zero step avviati. Annotazione restituita da GitHub: "The job was not started because recent account payments have failed or your spending limit needs to be increased. Please check the 'Billing & plans' section in your settings". La causa e quindi la fatturazione dell'account GitHub, non il codice ne la configurazione della pipeline.

## Impact and scope boundary

Il rischio numero uno dichiarato in ADR-003 — l'attrito della toolchain cross-platform — resta non sbancato. Non sappiamo se il progetto compili su Linux, se la cross-build Android funzioni su un runner pulito, se la build Tauri regga su entrambi i desktop, ne se i lint siano davvero bloccanti. Una rottura cross-platform verrebbe scoperta quando il codice e molto piu grande e la correzione costa molto di piu, che e esattamente lo scenario che SPEC-002 doveva prevenire. Da notare che il Lead ha verificato in locale una parte del rischio Android: la cross-build per arm64 riesce su questa macchina.

## Decision log

Created by project-lead: Deroga richiesta esplicitamente dall'operatore il 2026-08-25 per non tenere bloccata M-01 su un impedimento amministrativo esterno al progetto. Registrata come debito e non come chiusura silenziosa, cosi che la verifica mancante resti visibile e recuperabile.

## Resolution criteria

Ripristinata la fatturazione dell'account GitHub, una run della pipeline su main deve concludersi completamente verde su tutti i job (Rust su Windows e Linux, Android arm64 con bindings Kotlin, Tauri su Windows e Linux), con lint e cargo-deny bloccanti effettivamente eseguiti. A quel punto AGENT-007 non e coinvolta: il Lead ri-attesta GATE-CI-GREEN su SPEC-002 e chiude questo debito con il link alla run verde. Se la run rivela rotture, ciascuna diventa lavoro di remediation prima della chiusura.

## Resolution evidence

Run GitHub Actions 32821923135 sul commit 6b9ad1f di main, conclusione success con tutti e cinque i job verdi: Rust (ubuntu-latest), Rust (windows-latest), Tauri desktop (ubuntu-latest), Tauri desktop (windows-latest), Android arm64 + Kotlin bindings. Nel job Rust (ubuntu-latest) i gate bloccanti richiesti dai criteri di risoluzione risultano eseguiti e riusciti come step distinti: cargo build --locked --workspace, cargo test --locked --workspace, cargo fmt --all -- --check, cargo clippy --workspace --all-targets --all-features -- -D warnings, EmbarkStudios/cargo-deny-action@v2. Il job Android ha prodotto e caricato come artifact libcoblox_ffi.so per arm64-v8a ed eseguito il test Kotlin sul binding generato.

Remediation eseguite prima della chiusura, ciascuna scoperta da una run reale.

1. Run 32820395450, job Android: exit 126 con "./scripts/build-android.sh: Permission denied". Lo script era tracciato con modo 100644 perche committato da Windows, che non preserva il bit di esecuzione. Corretto con git update-index --chmod=+x nel commit 23d3113.

2. Run 32820395450, job Tauri desktop (ubuntu-latest): panico del proc macro tauri::generate_context! con "The frontendDist configuration is set to ../dist but this path doesn't exist". Il gate clippy su src-tauri girava prima di npm run build, quindi la directory del bundle frontend non esisteva ancora. In locale il difetto era invisibile perche apps/desktop/dist era gia presente da build precedenti. Corretto nel commit 23d3113 spostando clippy dopo npm run build, con commento nel workflow che spiega perche lint e build non sono indipendenti in questo crate.

3. Run 32821304150, job Android: exit 126 con "./gradlew: Permission denied", stessa classe del punto 1 un livello piu in basso. La cross-compilazione arm64 via cargo-ndk era riuscita e aveva gia copiato la libreria. Corretto nel commit 6b9ad1f. Nello stesso commit ANDROID_NDK_ROOT e stato fissato allo stesso NDK di ANDROID_NDK_HOME: l'immagine del runner ne preinstalla uno piu vecchio e cargo-ndk emetteva un warning di mismatch, lasciando la toolchain effettivamente usata dipendente dall'ordine di lookup invece che dichiarata.
