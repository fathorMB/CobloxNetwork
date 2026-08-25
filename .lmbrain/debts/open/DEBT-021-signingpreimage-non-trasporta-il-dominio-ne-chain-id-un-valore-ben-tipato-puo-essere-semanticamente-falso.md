---
id: DEBT-021
title: "SigningPreimage non trasporta il dominio ne' chain_id: un valore ben tipato puo' essere semanticamente falso"
status: open
category: "design"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: "REVIEW-023"
origin_ref: "RF-002"
related_specs: ["SPEC-012","SPEC-014"]
related_reviews: ["REVIEW-023"]
related_decisions: []
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["rust","api","security","conformance"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-021-EVENT-001"
    timestamp: "2026-08-25T23:58:25.124034300+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-014, su raccomandazione esplicita di AGENT-007 in RF-002 di REVIEW-023. Registrato come debito e non chiuso nella remediation perche e fuori dallo scope dei due debiti che SPEC-014 raggruppava, e perche allargare una spec gia passata per una remediation e il modo in cui una spec non chiude mai. Owner AGENT-001 perche e l'autore del tipo e della cucitura."
    evidence_refs: []
---
# SigningPreimage non trasporta il dominio ne' chain_id: un valore ben tipato puo' essere semanticamente falso

## Statement

SigningPreimage garantisce che i byte passati al verificatore siano stati prodotti da signing_preimage, e non garantisce nulla su quali byte siano. Il tipo non trasporta il Domain ne il chain_id: signing_preimage li impasta nel prefisso e poi il tipo li dimentica. Un chiamante che costruisse la preimmagine con il dominio sbagliato, o con il chain_id di un'altra catena, otterrebbe un valore ben tipato e semanticamente falso, e il verificatore lo accetterebbe.

SPEC-014 ha chiuso il salto da byte grezzi a preimmagine. Questa e la stessa domanda un livello sopra: la separazione di dominio esiste per impedire che una firma valida in un contesto sia valida in un altro, e oggi e tenuta dalla correttezza del chiamante e non dal tipo.

## Evidence and provenance

RF-002 di REVIEW-023, review di sicurezza di AGENT-007 su SPEC-014, dichiarato low e fuori dallo scope dei due debiti che quella spec chiudeva, con la raccomandazione di promuoverlo a debito proprio invece di allargare la spec.

La domanda era stata posta ad AGENT-007 dal Lead nel dispatch, come superficie da guardare oltre il finding gia registrato. La sua risposta e che non e un regresso introdotto da SPEC-014 ma una proprieta che quella spec non aveva mandato di chiudere.

Il Lead alza la severita da low a medium rispetto alla review, e dichiara la ragione: il primo chiamante di consenso non esiste ancora, quindi oggi nessuno puo sbagliare, ma la finestra in cui la correzione e gratuita e la stessa che DEBT-016 ha appena usato ed e la stessa che si chiude allo stesso evento. Un debito che vale low finche nessuno usa l'API e high il giorno dopo e mal classificato come low.

## Impact and scope boundary

Nessun impatto oggi: non esiste alcun chiamante del verificatore. Il danno potenziale e della classe che questo componente ha per natura, cioe un'accettazione silenziosa invece di un errore, ed e la stessa forma del replay cross-chain che RF-001 di REVIEW-023 descriveva: una firma legata a un contesto accettata in un altro.

La differenza rispetto a RF-001 e che li il prefisso spariva del tutto, qui il prefisso c'e ma puo essere quello sbagliato. E un fallimento piu difficile da notare, perche il valore ha la forma giusta.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-014, su raccomandazione esplicita di AGENT-007 in RF-002 di REVIEW-023. Registrato come debito e non chiuso nella remediation perche e fuori dallo scope dei due debiti che SPEC-014 raggruppava, e perche allargare una spec gia passata per una remediation e il modo in cui una spec non chiude mai. Owner AGENT-001 perche e l'autore del tipo e della cucitura.

## Resolution criteria

Il tipo trasporta il dominio e il chain_id con cui e stato costruito, e il verificatore o il chiamante possono verificarli contro cio che si aspettano, cosi che una preimmagine costruita per un contesto non sia utilizzabile in un altro senza che qualcosa lo dica. Oppure la dimostrazione motivata che il legame non serve, con la ragione scritta accanto al tipo invece che lasciata implicita.

Va valutato se la forma giusta sia un tipo parametrizzato sul dominio, un campo confrontato in verifica, o una funzione di verifica che prende dominio e chain_id attesi. La scelta ha conseguenze sull'ergonomia dei chiamanti che ancora non esistono, il che e un argomento per decidere ora e non dopo.

Da chiudere prima del primo chiamante del verificatore, per la stessa ragione di DEBT-016.

## Resolution evidence

