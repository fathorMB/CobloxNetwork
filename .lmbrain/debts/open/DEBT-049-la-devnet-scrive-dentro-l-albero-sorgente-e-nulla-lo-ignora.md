---
id: DEBT-049
title: "La devnet scrive dentro l'albero sorgente e nulla lo ignora"
status: open
category: "tooling"
severity: "low"
origin_severity: null
area: "tooling"
milestone: "M-02"
owner: null
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-029"]
related_reviews: ["REVIEW-049"]
related_decisions: []
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: []
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-049-EVENT-001"
    timestamp: "2026-08-27T20:24:04.889341700+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Dichiarato nel verdetto di REVIEW-049 come fuori perimetro della remediation: non e' un difetto di sicurezza del nodo e non va corretto dentro un ciclo che deve restare concentrato sui due critical. Aperto come debito perche' e' un fatto osservato eseguendo, non una preferenza."
    evidence_refs: []
---
# La devnet scrive dentro l'albero sorgente e nulla lo ignora

## Statement

`--data-dir` di `coblox-node` non ha ancoraggio fuori dall'albero e il runbook lo usa come `./data/val-00N`, quindi una devnet scrive WAL e blocchi dentro il working tree. `.gitignore` non ha alcuna voce per `data/`, percio' quei file compaiono come non tracciati a chiunque esegua la devnet.

## Evidence and provenance

Osservato dal Lead il 2026-08-27 eseguendo `docs/devnet-runbook.md`: dopo quattro nodi avviati, `git status --porcelain` mostrava `./data/` e `./data-val*.log` non tracciati, ripuliti a mano prima di ogni commit. Rilevato indipendentemente anche da AGENT-007 nelle note operative di REVIEW-049, ma non come rilievo numerato.

## Impact and scope boundary

Rischio di commit accidentale di stato di consenso in un repository pubblico, e rumore in `git status` che nasconde modifiche reali. Aggravante specifica: il Lead ha una disciplina che vieta `git add -A` mentre dei subagent lavorano sullo stesso albero, e file non tracciati generati da esecuzioni sono esattamente cio' che quella disciplina esiste per evitare di raccogliere.

## Decision log

Created by project-lead: Dichiarato nel verdetto di REVIEW-049 come fuori perimetro della remediation: non e' un difetto di sicurezza del nodo e non va corretto dentro un ciclo che deve restare concentrato sui due critical. Aperto come debito perche' e' un fatto osservato eseguendo, non una preferenza.

## Resolution criteria

`.gitignore` ha una voce che copre le directory dati della devnet e i log delle sue esecuzioni, oppure `--data-dir` acquisisce un default fuori dall'albero sorgente. In entrambi i casi una esecuzione completa del runbook lascia `git status --porcelain` vuoto.

## Resolution evidence

