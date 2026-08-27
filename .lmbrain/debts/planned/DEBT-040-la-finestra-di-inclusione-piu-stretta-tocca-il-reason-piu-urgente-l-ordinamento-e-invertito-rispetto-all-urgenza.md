---
id: DEBT-040
title: "La finestra di inclusione piu' stretta tocca il reason piu' urgente: l'ordinamento e' invertito rispetto all'urgenza"
status: planned
category: "security"
severity: "medium"
origin_severity: "high"
area: "consensus"
milestone: "M-03"
owner: "AGENT-007"
origin_artifact: "REVIEW-042"
origin_ref: "RF-001"
related_specs: ["SPEC-022"]
related_reviews: ["REVIEW-042","REVIEW-036"]
related_decisions: ["ADR-017"]
target_specs: ["SPEC-022"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: ["security","consensus","governance"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: Instradato su SPEC-022 perche' e' la consegna in cui la banda vive, ma con un'avvertenza che va letta prima di lavorarci: **la Statement di questo debito e' scritta su una premessa che REVIEW-045 ha accertato falsa**. Dice che la finestra piu' stretta e' la superficie che la censura attraversa; non lo e'. Chi ritarda l'inclusione spinge `p` verso l'alto e incontra il pavimento `e >= p + F`, cioe' la clausola 4 preesistente, non il tetto. Verificato dal Lead eseguendo: con `F=10, G=5, e=100` l'estremo superiore della finestra e' `p = 90` identico con e senza tetto.\n\nL'ordinamento invertito rispetto all'urgenza resta un fatto — `key_compromise` ha comunque il margine minore — ma la ragione per cui conta va riscritta quando questo debito viene lavorato, insieme alla bozza v2 di ADR-017. Chiuderlo sulla motivazione attuale propagherebbe l'errore una quarta volta."
debt_events:
  - schema_version: "1"
    id: "DEBT-040-EVENT-001"
    timestamp: "2026-08-27T10:15:01.544674200+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead il 2026-08-27 su decisione dell'operatore, contestualmente alla scelta di correggere ADR-017 con il solo pavimento relazionale. L'operatore ha scelto la correzione minima che chiude il high e mantiene la promessa gia' scritta in ADR-017; questo debito esiste perche' la seconda meta' del finding — l'ordinamento invertito — non e' chiusa da quella correzione e sparirebbe insieme al finding se non le si desse un artefatto proprio. La condizione di riesame non e' una data ma la disponibilita' della misura, ed e' la stessa che ADR-017 dichiara gia' nella propria sezione Revisit."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-040-EVENT-002"
    timestamp: "2026-08-27T15:02:10.178994800+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Instradato su SPEC-022 perche' e' la consegna in cui la banda vive, ma con un'avvertenza che va letta prima di lavorarci: **la Statement di questo debito e' scritta su una premessa che REVIEW-045 ha accertato falsa**. Dice che la finestra piu' stretta e' la superficie che la censura attraversa; non lo e'. Chi ritarda l'inclusione spinge `p` verso l'alto e incontra il pavimento `e >= p + F`, cioe' la clausola 4 preesistente, non il tetto. Verificato dal Lead eseguendo: con `F=10, G=5, e=100` l'estremo superiore della finestra e' `p = 90` identico con e senza tetto.\n\nL'ordinamento invertito rispetto all'urgenza resta un fatto — `key_compromise` ha comunque il margine minore — ma la ragione per cui conta va riscritta quando questo debito viene lavorato, insieme alla bozza v2 di ADR-017. Chiuderlo sulla motivazione attuale propagherebbe l'errore una quarta volta."
    evidence_refs: ["SPEC-022"]
---
# La finestra di inclusione piu' stretta tocca il reason piu' urgente: l'ordinamento e' invertito rispetto all'urgenza

## Statement

Nella banda di ADR-017 parte 2 le altezze di inclusione ammesse sono `[e-F-G, e-F]`, larghezza `G+1`, e la banda e' dichiarata dipendente da `reason`. Il risultato e' che `key_compromise` — il `reason` che porta l'urgenza crittografica — riceve la finestra piu' stretta, mentre i `reason` pianificati ricevono quella piu' larga. L'ordinamento delle finestre e' invertito rispetto all'ordinamento dell'urgenza.

## Evidence and provenance

REVIEW-042 RF-001, GATE-SECREVIEW di SPEC-022, severita' high: "Il reason con l'urgenza crittografica e' quello con la finestra piu' stretta: l'ordinamento e' invertito rispetto all'urgenza". Lo stesso finding accerta che il tetto e' nuovo di SPEC-022: la clausola 4 preesistente aveva pavimento e nessun tetto, quindi un ritardo di inclusione poteva solo rimandare una revoca, mentre da SPEC-022 puo' distruggerla. ADR-017 aveva corretto REVIEW-036 RF-002 — l'uguaglianza regalava un veto a un proponente per un turno — sostituendola con una banda che concede lo stesso veto a chi censura `G+1` blocchi: il veto e' stato reso piu' caro, non tolto.

## Impact and scope boundary

Chi vuole sopravvivere alla propria revoca d'emergenza ha bisogno di censurare la finestra piu' corta del sistema, invece della piu' lunga. La correzione decisa il 2026-08-27 — pavimento di `G` ancorato in genesi come relazione `G+1 >= validator_min_set_size_min` — toglie al set seduto la possibilita' di stringere la finestra, e quindi chiude il `high`, ma non riordina le larghezze fra i `reason`: dopo la correzione `key_compromise` resta il caso con il margine minore.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead il 2026-08-27 su decisione dell'operatore, contestualmente alla scelta di correggere ADR-017 con il solo pavimento relazionale. L'operatore ha scelto la correzione minima che chiude il high e mantiene la promessa gia' scritta in ADR-017; questo debito esiste perche' la seconda meta' del finding — l'ordinamento invertito — non e' chiusa da quella correzione e sparirebbe insieme al finding se non le si desse un artefatto proprio. La condizione di riesame non e' una data ma la disponibilita' della misura, ed e' la stessa che ADR-017 dichiara gia' nella propria sezione Revisit.

## Resolution criteria

Esiste una misura del tempo di coordinamento di un set successore — il numero che ADR-017 dichiara mancante nella propria sezione Revisit e che decide se `G` sia un margine o un alibi — e sulla sua base la tabella di `reason` e' rivista in modo che la larghezza della finestra cresca con l'urgenza invece di decrescere. In alternativa: e' dimostrato che l'ordinamento attuale e' corretto, e la dimostrazione e' scritta accanto alla tabella.

## Resolution evidence

