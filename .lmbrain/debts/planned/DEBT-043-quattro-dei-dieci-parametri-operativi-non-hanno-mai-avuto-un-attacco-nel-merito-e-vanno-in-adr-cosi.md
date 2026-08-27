---
id: DEBT-043
title: "Quattro dei dieci parametri operativi non hanno mai avuto un attacco nel merito, e vanno in ADR cosi'"
status: planned
category: "correctness"
severity: "medium"
origin_severity: null
area: "governance"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-023"]
related_reviews: ["REVIEW-043","REVIEW-041","REVIEW-038"]
related_decisions: []
target_specs: ["SPEC-027"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: ["governance","conformance"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: I criteri di questo debito dicono che le quattro righe vanno attaccate nel merito \"prima o durante la stesura dell'ADR che fissera' i parametri\". SPEC-027 e' quell'ADR, e l'attacco e' il suo primo passo dichiarato non saltabile, con una gate propria — GATE-FOUR-ROWS-ATTACKED — che chiede alla trascrizione di mostrare l'ordine.\n\nE' il debito con l'innesco piu' preciso dei diciotto: non una data ma un evento, e quell'evento e' stato appena creato."
debt_events:
  - schema_version: "1"
    id: "DEBT-043-EVENT-001"
    timestamp: "2026-08-27T10:44:58.483081500+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto su decisione esplicita dell'operatore del 2026-08-27, contestualmente all'accettazione di REVIEW-043. L'alternativa scartata era accettare e basta, lasciando il buco registrato solo dentro la review: alla stesura dell'ADR nessun artefatto attivo avrebbe segnalato che quattro righe su dieci non erano state esaminate, e la dimenticanza sarebbe dipesa dalla memoria di chi scrive. L'altra alternativa scartata era una quinta passata prima di accettare: non e' un giro di remediation sullo stesso rilievo ma l'esame di righe mai esaminate, quindi sarebbe stata legittima, ed e' stata rifiutata per non tenere ferme SPEC-022 e SPEC-025 su superfici che la reviewer stessa giudica le piu' piccole. L'innesco non e' una data ma la stesura dell'ADR."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-043-EVENT-002"
    timestamp: "2026-08-27T15:05:10.312551800+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "I criteri di questo debito dicono che le quattro righe vanno attaccate nel merito \"prima o durante la stesura dell'ADR che fissera' i parametri\". SPEC-027 e' quell'ADR, e l'attacco e' il suo primo passo dichiarato non saltabile, con una gate propria — GATE-FOUR-ROWS-ATTACKED — che chiede alla trascrizione di mostrare l'ordine.\n\nE' il debito con l'innesco piu' preciso dei diciotto: non una data ma un evento, e quell'evento e' stato appena creato."
    evidence_refs: ["SPEC-027"]
---
# Quattro dei dieci parametri operativi non hanno mai avuto un attacco nel merito, e vanno in ADR cosi'

## Statement

Le righe 2, 4, 5 e 6 dell'analisi dei dieci parametri operativi — `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md` — non sono mai state esaminate nel merito da nessuna delle quattro passate di `GATE-SECREVIEW` su SPEC-023. Sono state toccate nella forma, allineate fra tabella e corpo, e verificate nelle citazioni, ma nessuna review ne ha attaccato la sostanza: la classificazione tassonomica, il danno massimo dichiarato e il vincolo proposto per quelle quattro righe non hanno alcuna verifica avversariale.

## Evidence and provenance

Dichiarato da AGENT-007 in REVIEW-043, accepted il 2026-08-27, come residuo strutturale rimesso al Lead e all'operatore: "quattro righe su dieci (2, 4, 5, 6) non hanno mai avuto un attacco nel merito da nessuna delle quattro passate. Accettare significa portarle in ADR cosi'". Le passate precedenti confermano il quadro: REVIEW-038 attaccava le righe 1, 3, 7 e la tassonomia; REVIEW-040 i difetti nati dalla correzione; REVIEW-041 le righe 1, 2, 5, 6 e 7 ma sulla forma e sull'ancoraggio, non sulla sostanza; REVIEW-043 le righe 7, 3, 8, 9 e 10 su bersagli dichiarati dal Lead. Le quattro righe non compaiono come oggetto di merito in nessuna.

## Impact and scope boundary

L'analisi e' l'artefatto su cui l'operatore decidera' i limiti dei parametri e da cui nascera' un ADR. Per sei righe su dieci quella decisione poggia su almeno una passata avversariale; per quattro poggia solo su cio' che l'autrice ha scritto. E' la stessa forma che REVIEW-042 ha censurato altrove: un criterio soddisfatto alla lettera e vuoto nella sostanza. AGENT-007 osserva che sono le superfici piu' piccole, il che rende il rischio proporzionato ma non nullo. REVIEW-043 aggiunge due avvertenze che valgono per l'intero artefatto: l'analisi e' utilizzabile come base e non copiabile come testo, e la tabella del paragrafo 3 non e' autosufficiente per sei celle e non per le tre che la sua didascalia nomina.

## Decision log

Created by AGENT-LEAD: Aperto su decisione esplicita dell'operatore del 2026-08-27, contestualmente all'accettazione di REVIEW-043. L'alternativa scartata era accettare e basta, lasciando il buco registrato solo dentro la review: alla stesura dell'ADR nessun artefatto attivo avrebbe segnalato che quattro righe su dieci non erano state esaminate, e la dimenticanza sarebbe dipesa dalla memoria di chi scrive. L'altra alternativa scartata era una quinta passata prima di accettare: non e' un giro di remediation sullo stesso rilievo ma l'esame di righe mai esaminate, quindi sarebbe stata legittima, ed e' stata rifiutata per non tenere ferme SPEC-022 e SPEC-025 su superfici che la reviewer stessa giudica le piu' piccole. L'innesco non e' una data ma la stesura dell'ADR.

## Resolution criteria

Le righe 2, 4, 5 e 6 ricevono un attacco nel merito — classificazione, danno massimo, vincolo proposto — prima o durante la stesura dell'ADR che fissera' i parametri, e l'esito e' registrato. In alternativa: l'ADR dichiara per iscritto quali sue righe poggiano su un'analisi non rivista in modo avversariale, cosi' che chi legge la decisione sappia quali parti hanno una verifica indipendente e quali no.

## Resolution evidence

