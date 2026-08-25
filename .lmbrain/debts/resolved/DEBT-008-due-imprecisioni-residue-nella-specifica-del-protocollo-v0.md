---
id: DEBT-008
title: "Due imprecisioni residue nella specifica del protocollo v0"
status: resolved
category: "documentation"
severity: "low"
origin_severity: "low"
area: "core"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: "SPEC-001"
origin_ref: "REVIEW-007 RF-109, RF-110"
related_specs: ["SPEC-001"]
related_reviews: ["REVIEW-007"]
related_decisions: ["ADR-007"]
target_specs: []
blocked_by: []
resolution_refs: ["SPEC-010","REVIEW-015"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["documentation","argon2id","enrollment"]
links: []
activity:
  - date: 2026-08-25
    action: "resolved: Risolto da SPEC-010 nella milestone M-02, come il debito prevedeva. La stima di una riga ciascuna era per difetto su RF-110, dove la correzione corretta era una regola nuova e non una riformulazione, ed e un miglioramento di sostanza rispetto a cio che il debito chiedeva."
debt_events:
  - schema_version: "1"
    id: "DEBT-008-EVENT-001"
    timestamp: "2026-08-25T02:52:57.527877100+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Promosso a debito dal Lead alla chiusura di SPEC-001, su mandato dell'operatore di salvare ogni questione emergente invece di lasciarla in una review chiusa. Senza questo, i due finding sparirebbero con l'archiviazione della spec."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-008-EVENT-002"
    timestamp: "2026-08-25T19:38:51.335566700+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Risolto da SPEC-010 nella milestone M-02, come il debito prevedeva. La stima di una riga ciascuna era per difetto su RF-110, dove la correzione corretta era una regola nuova e non una riformulazione, ed e un miglioramento di sostanza rispetto a cio che il debito chiedeva."
    evidence_refs: ["SPEC-010", "REVIEW-015"]
---
# Due imprecisioni residue nella specifica del protocollo v0

## Statement

Due affermazioni della specifica del protocollo sono leggermente piu forti di quanto la regola scritta garantisca. RF-109: la frase secondo cui il pavimento Argon2id rifiuta tutto cio che e piu debole di entrambe le raccomandazioni RFC e sovradimensionata, perche la forma ad area ammette una banda stretta di configurazioni con iterations=1 sotto i 2 GiB quando memory_kib e almeno 196608, che non corrisponde a nessuna delle due raccomandazioni. RF-110: la conseguenza secondo cui lo scudo di ammissione costa all'attaccante un indirizzo raggiungibile per ogni slot concorrente vale solo se l'emissione dei nonce e conteggiata contro il limite per sorgente del primo passo; il carattere monouso del nonce limita il riuso, non il volume.

## Evidence and provenance

REVIEW-007 di AGENT-007, verifica finale di sicurezza su SPEC-001 con GATE-SECREVIEW attestato superato. Entrambi i finding sono di severita low e dichiarati non bloccanti dalla reviewer, che ha giudicato esplicitamente non giustificato un terzo giro di remediation. Sul primo punto AGENT-007 quantifica il degrado come un piccolo fattore costante e non un ordine di grandezza, incomparabile con il fattore circa 8000 che aveva motivato RF-101.

## Impact and scope boundary

Nessun impatto sulla sicurezza effettiva del protocollo v0: entrambe le proprieta sottostanti valgono, e sono le frasi che le descrivono a essere leggermente piu ampie del vero. L'impatto e sulla precisione della specifica come contratto di implementazione: chi implementa leggendo quelle due frasi potrebbe assumere una garanzia marginalmente piu forte di quella imposta dalle regole di validita. Va corretto prima che la specifica diventi riferimento pubblico per sviluppatori terzi.

## Decision log

Created by project-lead: Promosso a debito dal Lead alla chiusura di SPEC-001, su mandato dell'operatore di salvare ogni questione emergente invece di lasciarla in una review chiusa. Senza questo, i due finding sparirebbero con l'archiviazione della spec.

## Resolution criteria

Le due frasi sono riformulate in modo da corrispondere esattamente a cio che le regole di validita impongono: per RF-109 dichiarando la banda ammessa con iterations=1 sopra i 196608 KiB, oppure restringendo la regola se si decide che quella banda non debba essere ammessa; per RF-110 conteggiando l'emissione dei nonce contro il limite per sorgente, oppure indebolendo la conseguenza dichiarata. Correzione attesa in M-02, una riga ciascuna.

## Resolution evidence

RF-109 e stata riformulata e non ristretta. Il documento dichiara ora la banda ammessa, iterations uguale 1 con memory_kib fra 196608 e 2097152 escluso, con due righe di frontiera nuove nella tabella di conformita, e nomina il costo della scelta: quella banda ha le stesse KiB-passate della seconda raccomandazione RFC e piu memoria, quindi non e piu debole per nessuna delle due grandezze che le regole misurano, ma la RFC non la raccomanda e il documento non pretende che sia equivalente. Restringere la regola alle due configurazioni nominate e stato rifiutato con la motivazione che una regola che enumera le raccomandazioni correnti di una RFC e una whitelist che invecchia alla prima revisione di quella RFC.

RF-110 e stata chiusa prendendo entrambe le strade e non una. L'implementatore ha contestato l'oppure offerto dalla spec, stabilendo che la sola riformulazione lascerebbe il primo passo dello scudo di ammissione a costare all'attaccante un giro, in contraddizione con l'argomento della sezione stessa. L'emissione dei nonce e ora conteggiata contro il limite per sorgente del primo passo, con un tetto dichiarato k sui nonce in sospeso, e la frase e corretta di conseguenza: con un tetto k il costo e un indirizzo ogni k slot concorrenti, non uno per slot.

Il Lead ha verificato che la frase di RF-109 sta in README.md e non in identity.md come la spec affermava, correzione dell'implementatore registrata come RF-202 di REVIEW-015.
