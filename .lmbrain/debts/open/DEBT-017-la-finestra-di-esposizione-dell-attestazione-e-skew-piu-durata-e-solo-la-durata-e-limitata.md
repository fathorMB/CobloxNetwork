---
id: DEBT-017
title: "La finestra di esposizione dell'attestazione e' skew piu' durata, e solo la durata e' limitata"
status: open
category: "security"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "SPEC-013"
origin_ref: "segnalazione dell'implementatore in remediation di REVIEW-021, punto 2"
related_specs: ["SPEC-013"]
related_reviews: ["REVIEW-021"]
related_decisions: ["ADR-015"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["security","privacy","consensus"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-017-EVENT-001"
    timestamp: "2026-08-25T23:04:18.150331100+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-013, su segnalazione dell'implementatore che si e fermato e ha riportato invece di decidere, come il mandato gli imponeva. Registrato come debito e non come remediation perche introdurre un vincolo relazionale e una regola di validita nuova, quindi ricade sotto ADR-012 e apre la propria passata, e perche estendere una terza volta una spec gia passata per una remediation e il modo in cui una spec non chiude mai. Owner AGENT-007 e non il Lead ne l'implementatore: e un'osservazione che chi l'ha fatta non deve valutare da se, ed e la regola che questo progetto ha applicato a DEBT-013 e DEBT-014."
    evidence_refs: []
---
# La finestra di esposizione dell'attestazione e' skew piu' durata, e solo la durata e' limitata

## Statement

SPEC-013 introduce due parametri che governano l'accettazione di una TransportKeyAttestation: max_transport_attestation_validity_ms, che limita expires_at_ms meno created_at_ms, e max_transport_attestation_future_skew_ms, che tollera un created_at_ms nel futuro rispetto all'orologio del verificatore. Nessuna regola li mette in relazione, ne li mette in relazione con max_envelope_validity_ms.

La grandezza da cui dipende la proprieta che RF-002 esisteva per difendere non e la durata dichiarata dall'attestazione ma la finestra in cui un verificatore la accetta, e quella finestra e la somma dei due: un'attestazione datata nel futuro entro la tolleranza resta accettabile fino a skew piu durata. Limitare solo la durata limita cio che il documento dichiara, non cio che l'avversario ottiene.

## Evidence and provenance

Segnalato dall'implementatore AGENT-001 al termine della remediation di REVIEW-021 come punto 2 delle questioni riportate al Lead, con la motivazione corretta: aggiungere un vincolo relazionale sarebbe una regola di validita oltre a quelle che i finding impongono, e il mandato del Lead gli imponeva di fermarsi e riportare invece di decidere. Il Lead conferma che fermarsi era il comportamento giusto.

L'osservazione non e stata sottoposta a valutazione adversariale: e il Lead a ritenerla della famiglia 3 di recurring-defects.md, cioe vincolata la grandezza nominata e non quella da cui la proprieta dipende, ma e esattamente la superficie su cui il Lead ha gia sbagliato piu volte in questa sessione e la sua valutazione non deve essere l'ultima parola.

## Impact and scope boundary

Da stabilire, ed e il lavoro. La direzione del pericolo e verso l'alto sulla somma: piu la tolleranza e generosa, piu la finestra reale eccede quella dichiarata, e il punto 2 della motivazione scritta in identity.md — una chiave di trasporto compromessa smette di valere da sola — vale sulla durata e non sulla finestra reale.

Va valutato separatamente il rapporto con max_envelope_validity_ms, perche l'attestazione e la porta di tutti i protocolli protetti mentre l'envelope e cio che vi transita, e non e detto che le due finestre debbano avere lo stesso ordine di grandezza ne quale delle due debba contenere l'altra.

Severita medium e non high perche richiede una taratura generosa della tolleranza, che oggi non e fissata da alcun documento di genesi, e perche nessuna rete esiste ancora; sarebbe high su una rete viva con una tolleranza gia scelta male.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-013, su segnalazione dell'implementatore che si e fermato e ha riportato invece di decidere, come il mandato gli imponeva. Registrato come debito e non come remediation perche introdurre un vincolo relazionale e una regola di validita nuova, quindi ricade sotto ADR-012 e apre la propria passata, e perche estendere una terza volta una spec gia passata per una remediation e il modo in cui una spec non chiude mai. Owner AGENT-007 e non il Lead ne l'implementatore: e un'osservazione che chi l'ha fatta non deve valutare da se, ed e la regola che questo progetto ha applicato a DEBT-013 e DEBT-014.

## Resolution criteria

Una valutazione adversariale che stabilisca se la finestra reale vada vincolata, e in quale forma, pronunciandosi separatamente sul rapporto fra i due parametri dell'attestazione e sul rapporto con max_envelope_validity_ms. Gli esiti ammissibili sono due e vanno distinti: un vincolo relazionale nel blocco di validita dei consensus_parameters, con la sua fixture di frontiera e la prova in negativo; oppure il rifiuto motivato, con la somma dichiarata nel documento accanto ai due parametri invece che lasciata al lettore. Una terza uscita apparente, tarare stretta la tolleranza nella genesi senza vincolarla, va rifiutata per la ragione che ADR-010 ha gia stabilito: un valore scelto bene non e una proprieta, e una preferenza.

Da chiudere prima che una devnet emetta attestazioni, per la stessa ragione per cui SPEC-013 doveva atterrare prima del primo certificato.

## Resolution evidence

