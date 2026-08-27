---
id: DEBT-041
title: "ledger.md si contraddice sull'aggregazione su K blocchi: dichiarata non presa in v0 in una sezione e presa in un'altra"
status: open
category: "correctness"
severity: "medium"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-023"]
related_reviews: ["REVIEW-041"]
related_decisions: ["ADR-012"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: ["consensus","conformance","documentation"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-041-EVENT-001"
    timestamp: "2026-08-27T10:29:29.773033900+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead perche' la scoperta e' fuori dal perimetro della spec che l'ha prodotta e SPEC-023 ha correttamente riportato invece di correggere: modificare `docs/protocol/` da dentro quella spec le avrebbe fatto scavalcare la gate di ADR-012, che e' la stessa ragione per cui SPEC-018 aveva escluso i documenti di protocollo. Il difetto e' durevole e attraversa piu' spec, quindi e' un debito e non un rilievo locale."
    evidence_refs: []
---
# ledger.md si contraddice sull'aggregazione su K blocchi: dichiarata non presa in v0 in una sezione e presa in un'altra

## Statement

`docs/protocol/ledger.md` afferma due cose incompatibili sulla stessa riduzione. La sezione *"Challenge evidence"* elenca l'aggregazione su `K` blocchi fra le riduzioni **non prese in v0**, rimandandole al beacon di casualita' dedicato. La sezione della regola di elezione dichiara invece di prendere qui proprio quella riduzione. Un implementatore che legga una sola delle due sezioni implementa un protocollo diverso da chi legge l'altra.

## Evidence and provenance

Verificato dal Lead il 2026-08-27 leggendo entrambe le sezioni. `docs/protocol/ledger.md:722` — "Two reductions are available and are not taken in v0: quantizing `timestamp_ms`" — e `:725` — "Both belong with the dedicated randomness beacon, which is M-02". Contro `docs/protocol/ledger.md:1400` — "reduction this document deferred to \"the dedicated randomness beacon\" and takes here". Le due frasi risolvono entrambe contro l'albero corrente, verificate con confronto normalizzato sugli spazi. Trovato da AGENT-002 durante la quarta passata di remediation di SPEC-023 e riportato invece di essere corretto, perche' `docs/protocol/` era fuori dal perimetro di quella spec.

## Impact and scope boundary

E' la famiglia 2 del censimento dei difetti ricorrenti — la pretesa rimasta avanti rispetto alla regola — dentro un documento di protocollo e non in un artefatto di analisi. Due implementazioni conformi a letture diverse dello stesso documento divergono su quale entropia entra nella regola di elezione, che e' materia di consenso. Non serve un avversario: serve una seconda implementazione. Tocca inoltre il perimetro di DEBT-038, che l'analisi di SPEC-023 propone di restringere alla sola quantizzazione allo slot proprio in conseguenza di questa contraddizione.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead perche' la scoperta e' fuori dal perimetro della spec che l'ha prodotta e SPEC-023 ha correttamente riportato invece di correggere: modificare `docs/protocol/` da dentro quella spec le avrebbe fatto scavalcare la gate di ADR-012, che e' la stessa ragione per cui SPEC-018 aveva escluso i documenti di protocollo. Il difetto e' durevole e attraversa piu' spec, quindi e' un debito e non un rilievo locale.

## Resolution criteria

Le due sezioni concordano: o l'aggregazione su `K` blocchi e' presa in v0 e la sezione *"Challenge evidence"* smette di elencarla fra le riduzioni non prese, o non e' presa e la regola di elezione smette di dichiarare di prenderla. Il verso e' motivato per iscritto. La passata di ADR-012 e' eseguita sulla correzione, e una probe pinna la frase nella forma decisa in entrambe le sezioni, non in una sola.

## Resolution evidence

