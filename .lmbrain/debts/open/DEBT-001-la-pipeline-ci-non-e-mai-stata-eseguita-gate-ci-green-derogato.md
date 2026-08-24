---
id: DEBT-001
title: "La pipeline CI non e mai stata eseguita: GATE-CI-GREEN derogato"
status: open
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
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["ci","verification-gap","toolchain"]
links: []
activity: []
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

