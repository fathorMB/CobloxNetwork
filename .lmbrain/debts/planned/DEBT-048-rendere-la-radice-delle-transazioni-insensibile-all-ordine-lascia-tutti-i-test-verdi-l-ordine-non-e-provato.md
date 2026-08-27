---
id: DEBT-048
title: "Rendere la radice delle transazioni insensibile all'ordine lascia tutti i test verdi: l'ordine non e' provato"
status: planned
category: "test-quality"
severity: "medium"
origin_severity: "medium"
area: "consensus"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: "REVIEW-048"
origin_ref: "RF-003"
related_specs: ["SPEC-025"]
related_reviews: ["REVIEW-048"]
related_decisions: []
target_specs: ["SPEC-029"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: ["consensus","conformance"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: Instradato su SPEC-029 perche' e' la prossima consegna che tocca lo stesso percorso: aggiunge il ricalcolo della radice in `FinalizedBlock::verify` per DEBT-047, quindi scrive un secondo sito che dipende dalla sensibilita' all'ordine. Aggiungere li' il test che la prova costa una passata sullo stesso codice che si sta gia' scrivendo, e chiuderlo altrove significherebbe tornare due volte sulla stessa funzione.\n\nC'e' inoltre una ragione di merito per non rimandarlo oltre: con due siti che ricalcolano la radice invece di uno, una mutazione che ordini le foglie resterebbe verde in entrambi, e la lacuna di prova varrebbe il doppio."
debt_events:
  - schema_version: "1"
    id: "DEBT-048-EVENT-001"
    timestamp: "2026-08-27T18:41:48.091647600+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead contestualmente all'accettazione di REVIEW-048, invece di aprire un terzo giro di remediation su SPEC-025: il rilievo non blocca alcun criterio ne' alcuna gate, e la 5.1.0 vieta di aprire un giro per rilievi che non bloccano.\n\nOwner AGENT-002 perche' e' chi ha scritto sia la radice sia i test, e perche' la lacuna e' nella prova e non nella regola: chiuderla e' aggiungere un test, non correggere una consegna."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-048-EVENT-002"
    timestamp: "2026-08-27T18:42:25.063836300+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Instradato su SPEC-029 perche' e' la prossima consegna che tocca lo stesso percorso: aggiunge il ricalcolo della radice in `FinalizedBlock::verify` per DEBT-047, quindi scrive un secondo sito che dipende dalla sensibilita' all'ordine. Aggiungere li' il test che la prova costa una passata sullo stesso codice che si sta gia' scrivendo, e chiuderlo altrove significherebbe tornare due volte sulla stessa funzione.\n\nC'e' inoltre una ragione di merito per non rimandarlo oltre: con due siti che ricalcolano la radice invece di uno, una mutazione che ordini le foglie resterebbe verde in entrambi, e la lacuna di prova varrebbe il doppio."
    evidence_refs: ["SPEC-029"]
---
# Rendere la radice delle transazioni insensibile all'ordine lascia tutti i test verdi: l'ordine non e' provato

## Statement

Nessun test del progetto distingue una radice delle transazioni **sensibile all'ordine** da una insensibile. Mutando `transactions_root_of` perche' ordini le foglie prima di costruire l'albero — cioe' perche' produca lo stesso digest per qualunque permutazione dello stesso insieme — tutti i **230** test restano verdi.

## Evidence and provenance

Rilevato e misurato da AGENT-007 in REVIEW-048 RF-003, mutando l'albero e ripristinandolo da copia presa prima. L'ordine e' la proprieta' per cui l'albero delle transazioni e' diverso dagli altri cinque alberi del progetto: `ledger.md` definisce `transactions` come portate "in canonical execution order", e l'ordine di esecuzione e' cio' che decide l'esito quando due transazioni della stessa altezza toccano lo stesso saldo.

## Impact and scope boundary

Una regola di consenso e' protetta da una prova che non la esercita. Il rischio non e' teorico: una modifica futura che introduca un ordinamento delle foglie — per esempio per rendere deterministico un percorso che oggi dipende dall'ordine di inserimento in una mappa — passerebbe la suite intera, e due implementazioni che ordinano diversamente produrrebbero lo stesso `block_id` per esiti di esecuzione diversi. E' la classe di difetto che il progetto ha gia' pagato con DEBT-012, rimasto invisibile finche' non e' esistita una seconda derivazione.

Non e' bloccante perche' il codice consegnato e' corretto: e' la sua **prova** a essere incompleta.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead contestualmente all'accettazione di REVIEW-048, invece di aprire un terzo giro di remediation su SPEC-025: il rilievo non blocca alcun criterio ne' alcuna gate, e la 5.1.0 vieta di aprire un giro per rilievi che non bloccano.

Owner AGENT-002 perche' e' chi ha scritto sia la radice sia i test, e perche' la lacuna e' nella prova e non nella regola: chiuderla e' aggiungere un test, non correggere una consegna.

## Resolution criteria

Esiste un test che fallisce se la radice diventa insensibile all'ordine: due permutazioni dello stesso insieme di transazioni producono radici diverse, e la trascrizione mostra la mutazione osservata fallire. Il test nomina `ledger.md` e la frase che definisce l'ordine canonico di esecuzione, cosi' che chi lo legge sappia quale proprieta' sta tenendo.

## Resolution evidence

