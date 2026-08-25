---
id: DEBT-006
title: "La quota al creatore obbliga a pubblicare chi e abbonato a cosa"
status: resolved
category: "privacy"
severity: "high"
origin_severity: "high"
area: "core"
milestone: "M-06"
owner: "AGENT-LEAD"
origin_artifact: "SPEC-004"
origin_ref: "TM-26"
related_specs: ["SPEC-004"]
related_reviews: ["REVIEW-003"]
related_decisions: ["ADR-006"]
target_specs: []
blocked_by: []
resolution_refs: ["ADR-014","ADR-015"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["privacy","ledger","governance"]
links: []
activity:
  - date: 2026-08-25
    action: "resolved: Risolto dalla decisione dell'operatore del 2026-08-25, che e esattamente la forma che il criterio di risoluzione richiedeva. Chiuso ora e non alla scrittura del testo pubblico perche il debito chiedeva la decisione, non la sua esecuzione, e lasciarlo aperto fino all'esecuzione confonderebbe due cose che hanno owner e scadenze diverse."
debt_events:
  - schema_version: "1"
    id: "DEBT-006-EVENT-001"
    timestamp: "2026-08-25T01:50:43.166126400+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Promosso a debito dal Lead durante la review di SPEC-004, su mandato dell'operatore di salvare tutti i debiti emergenti. Sopravvive alla spec che l'ha scoperto e nessuna spec aperta lo copre."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-006-EVENT-002"
    timestamp: "2026-08-25T18:50:43.071395+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Risolto dalla decisione dell'operatore del 2026-08-25, che e esattamente la forma che il criterio di risoluzione richiedeva. Chiuso ora e non alla scrittura del testo pubblico perche il debito chiedeva la decisione, non la sua esecuzione, e lasciarlo aperto fino all'esecuzione confonderebbe due cose che hanno owner e scadenze diverse."
    evidence_refs: ["ADR-014", "ADR-015"]
---
# La quota al creatore obbliga a pubblicare chi e abbonato a cosa

## Statement

ADR-006 fa dipendere la ricompensa al publisher dal numero di abbonati attivi, e ADR-001 richiede che i validatori possano ricalcolare quel numero dai burn finalizzati. Ne consegue che il ledger pubblico deve contenere, in forma ricalcolabile, quali identita sono abbonate a quale app, per sempre, accanto a uno pseudonimo stabile per tutta la vita della chiave. Questo conflitto strutturale fra la ricompensa al creatore e la privacy degli abbonati non e dichiarato in alcun documento del progetto.

## Evidence and provenance

SPEC-004 scenari TM-26 e TM-29, con l'analisi di threat-model.md §6.3. Confermato dalla struttura di docs/protocol/ledger.md, dove il mint publisher_reward porta active_subscriber_count e active_subscription_root ricalcolabili dai burn di abbonamento finalizzati. AGENT-007 segnala che la privacy e l'unica superficie del threat model priva di un ADR alle spalle.

## Impact and scope boundary

Un osservatore ricostruisce i consumi di ogni utente della rete e li correla a un identificatore stabile e a un indirizzo IP. Per un progetto che si presenta come alternativa indipendente alle piattaforme centralizzate, e una proprieta che contraddice la promessa implicita, e che gli utenti scoprirebbero dopo aver gia pubblicato i propri dati in modo irreversibile. Non esiste una remediation retroattiva: cio che finisce su un ledger immutabile non si toglie.

## Decision log

Created by project-lead: Promosso a debito dal Lead durante la review di SPEC-004, su mandato dell'operatore di salvare tutti i debiti emergenti. Sopravvive alla spec che l'ha scoperto e nessuna spec aperta lo copre.

## Resolution criteria

Una ADR dedicata alla privacy che dichiari esplicitamente cosa e pubblico e correlabile nella rete, e che scelga fra: accettare la trasparenza dichiarandola nella documentazione pubblica prima del lancio (SEC-REQ-22); ridurre la granularita del conteggio abbonati; oppure sostituire il conteggio per identita con una prova aggregata che non esponga l'insieme. La scelta va presa prima che una rete pubblica accumuli abbonamenti reali.

## Resolution evidence

Il criterio di risoluzione chiedeva una ADR dedicata alla privacy che dichiarasse esplicitamente cosa e pubblico e correlabile nella rete, e che scegliesse fra le tre opzioni, prima che una rete pubblica accumulasse abbonamenti reali. ADR-014, accettata dall'operatore il 2026-08-25, sceglie l'opzione (a): il conteggio per identita resta e la proprieta che ne discende e dichiarata pubblicamente prima del primo partecipante esterno, con SEC-REQ-22 come obbligo con scadenza e non come nota. Le altre due opzioni sono registrate con il motivo del rifiuto, e la prova aggregata e nominata come ricerca e non come promessa, perche annunciare una privacy futura incassa oggi la fiducia che quella privacy meriterebbe domani.

La decisione e presa avendo prima stabilito dove sta davvero la fuga, che il debito non nominava: non nel conteggio ma nel burn, che porta payer_node_id perche e la firma del pagatore ad autorizzare l'addebito. Ne consegue che togliere la quota al creatore toglie la ragione di contare e lascia la lista intatta, e che la grandezza da cui la proprieta dipende e l'invariante un pagatore un voto. Questo e il motivo per cui l'opzione (b) del debito e stata rifiutata sul merito e non sul costo.

Il debito nominava una sola meta della superficie. L'altra — il legame fra pseudonimo e indirizzo IP, TM-28, alta severita e senza alcun ADR alle spalle — e stata affrontata nella stessa passata con ADR-015, che separa la chiave di trasporto dalla chiave di identita.

Resta lavoro a valle e non e risoluzione di questo debito: il testo pubblico va scritto una volta sola e citato, ed e deliverable di una spec con GATE-SECREVIEW perche e un documento di sicurezza rivolto all'utente.
