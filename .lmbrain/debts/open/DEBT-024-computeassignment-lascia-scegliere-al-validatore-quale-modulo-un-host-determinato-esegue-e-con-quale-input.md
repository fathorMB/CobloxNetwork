---
id: DEBT-024
title: "ComputeAssignment lascia scegliere al validatore quale modulo un host determinato esegue, e con quale input"
status: open
category: "security"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-06"
owner: "AGENT-007"
origin_artifact: "SPEC-018"
origin_ref: "TM-42, perimetro dichiarato"
related_specs: ["SPEC-018"]
related_reviews: []
related_decisions: ["ADR-004","ADR-006"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-26
tags: ["compute","security","wire"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-024-EVENT-001"
    timestamp: "2026-08-26T01:35:08.541108100+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead su segnalazione esplicita di AGENT-007, che ha rispettato il perimetro di [SPEC-018] invece di correggere `wire.md` fuori scopo — la scelta giusta, perche' quella spec non ha la gate di [ADR-012] e correggere li' un documento di protocollo l'avrebbe scavalcata.\n\nIl debito esiste perche' TM-42 registra lo **scenario** e non l'**obbligo**: uno scenario nel threat model dice che il pericolo e' stato visto, non che qualcuno lo chiudera'. Senza questo artefatto la conoscenza sarebbe rimasta in un documento di analisi, dove nessuna lifecycle la fa maturare."
    evidence_refs: []
---
# ComputeAssignment lascia scegliere al validatore quale modulo un host determinato esegue, e con quale input

## Statement

`ComputeAssignment` porta un campo `input` scelto **verbatim dall'emittente**. Ne segue che un validatore sceglie **quale modulo un host determinato esegue e con quale input**, senza alcun tetto dichiarato sul numero di assegnazioni ne' sulla taglia dell'input. Trovato da AGENT-007 durante la passata di [SPEC-018] ed emerso come scenario TM-42, la cui contromisura toccherebbe `docs/protocol/wire.md` — fuori dallo scopo di quella spec, che per costruzione non ha la gate di [ADR-012].

## Evidence and provenance

Registrato come scenario TM-42 in `.lmbrain/knowledge/threat-model.md` con nota di perimetro esplicita: la spec che lo ha scoperto escludeva ogni modifica ai documenti di protocollo, e correggerlo li' avrebbe scavalcato la gate di [ADR-012]. Il campo e la sua provenienza sono nella definizione di `ComputeAssignment`; la mancanza di un tetto e' l'assenza di una regola e non una regola permissiva, che e' la forma piu' difficile da vedere rileggendo.

## Impact and scope boundary

Il bersaglio e' l'asset dell'isolamento della sandbox e, per il tramite, la macchina di un partecipante. La superficie non e' l'esecuzione di codice ostile in se', che [ADR-004] mette in conto: e' la **direzionalita'**. Un avversario che sceglie insieme il modulo, l'input e l'host non sta pubblicando codice e aspettando che qualcuno lo esegua — sta puntando un host determinato, che e' una capacita' diversa e piu' forte, e che [ADR-006] aveva deliberatamente tolto al publisher scartando la scelta degli host.

Senza tetto sul numero e sulla taglia, la stessa capacita' e' anche un canale di esaurimento risorse verso un bersaglio scelto, e non richiede alcuna violazione di regola perche' nessuna regola e' scritta.

Severita' `medium` e non `high` per due ragioni dichiarate: richiede il ruolo di validatore, che e' gia' la soglia oltre la quale altre proprieta' sono in discussione; e il livello compute e' M-06, quindi nessuna riga di codice lo implementa oggi. E' la classe piu' economica da chiudere adesso, quando e' un paragrafo, e la piu' cara quando sara' un formato di messaggio con implementazioni al seguito.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead su segnalazione esplicita di AGENT-007, che ha rispettato il perimetro di [SPEC-018] invece di correggere `wire.md` fuori scopo — la scelta giusta, perche' quella spec non ha la gate di [ADR-012] e correggere li' un documento di protocollo l'avrebbe scavalcata.

Il debito esiste perche' TM-42 registra lo **scenario** e non l'**obbligo**: uno scenario nel threat model dice che il pericolo e' stato visto, non che qualcuno lo chiudera'. Senza questo artefatto la conoscenza sarebbe rimasta in un documento di analisi, dove nessuna lifecycle la fa maturare.

## Resolution criteria

Stabilire se la scelta congiunta di modulo, input e host da parte dell'emittente sia **deliberata** — e in tal caso scritta accanto al messaggio con la sua ragione, perche' una capacita' forte non dichiarata si legge come una dimenticanza — **oppure** vincolata.

Se vincolata, la chiusura deve pronunciarsi separatamente su tre cose, perche' hanno rimedi diversi: chi sceglie l'host; se l'input debba essere un riferimento a contenuto gia' pubblicato invece che byte verbatim; e i tetti su numero e taglia.

**Il rimedio apparente da non adottare:** un tetto sulla sola taglia dell'input. Vincola la grandezza nominata e non quella da cui la proprieta' dipende, che e' la direzionalita': un avversario che punta un host determinato non ha bisogno di input grandi. E' la famiglia 3, e va nominata qui perche' e' il rimedio che sembra ovvio guardando il campo invece della capacita'.

Da chiudere **prima che il livello compute abbia un formato di messaggio implementato**, cioe' dentro M-06 e non alla sua fine.

## Resolution evidence

