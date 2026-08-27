---
id: DEBT-047
title: "FinalizedBlock::verify accetta un Block il cui carico l'header non impegna: il legame e' al confine della proposta e non alla verifica"
status: planned
category: "security-boundary"
severity: "high"
origin_severity: "high"
area: "consensus"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: "REVIEW-048"
origin_ref: "RF-001"
related_specs: ["SPEC-025","SPEC-029"]
related_reviews: ["REVIEW-048","REVIEW-047"]
related_decisions: []
target_specs: ["SPEC-029"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: ["consensus","security","conformance"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: SPEC-029 e' la consegna che rende il buco raggiungibile, e per questo e' il bersaglio giusto e non un ripiego. SPEC-025 ha messo il legame al confine della proposta; finche' non esistono rete e persistenza nessun `Block` arriva da un altro percorso. SPEC-029 introduce entrambi i percorsi che lo portano — un blocco letto da disco al riavvio e uno ricevuto da un pari in sincronizzazione — quindi il difetto passa da irraggiungibile a vivo esattamente li'.\n\nLa spec porta ora un criterio di accettazione proprio che lo nomina, con il test che osserva il rifiuto su un blocco con certificato genuino e carico divergente. Il debito non poggia quindi sulla memoria di chi scrivera' quella spec: e' gia' scritto dentro i suoi criteri.\n\nNon chiuso con un terzo giro su SPEC-025 perche' il rilievo non blocca alcun criterio ne' alcuna gate, e la 5.1.0 vieta di aprire un giro per rilievi che non bloccano. La reviewer dichiara che il rimedio e' sei righe gia' scritte nella review, quindi il costo e' noto e non stimato."
debt_events:
  - schema_version: "1"
    id: "DEBT-047-EVENT-001"
    timestamp: "2026-08-27T18:41:15.373612300+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead invece di aprire un terzo giro di remediation su SPEC-025. La spec e' al secondo giro, chiude il primo esito di M-02, e il rilievo non blocca alcun criterio di accettazione ne' alcuna gate dichiarata: la 5.1.0 vieta di aprire un giro per rilievi che non bloccano.\n\nInstradato su SPEC-029 e non lasciato senza bersaglio perche' quella spec e' esattamente cio' che rende il buco raggiungibile — porta rete e persistenza — e la spec porta ora un criterio proprio che lo nomina. Chiuderlo li' e' piu' forte che chiuderlo qui: lo lega alla consegna in cui diventa vivo, invece di correggerlo mentre non lo e'."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-047-EVENT-002"
    timestamp: "2026-08-27T18:42:11.984753600+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "SPEC-029 e' la consegna che rende il buco raggiungibile, e per questo e' il bersaglio giusto e non un ripiego. SPEC-025 ha messo il legame al confine della proposta; finche' non esistono rete e persistenza nessun `Block` arriva da un altro percorso. SPEC-029 introduce entrambi i percorsi che lo portano — un blocco letto da disco al riavvio e uno ricevuto da un pari in sincronizzazione — quindi il difetto passa da irraggiungibile a vivo esattamente li'.\n\nLa spec porta ora un criterio di accettazione proprio che lo nomina, con il test che osserva il rifiuto su un blocco con certificato genuino e carico divergente. Il debito non poggia quindi sulla memoria di chi scrivera' quella spec: e' gia' scritto dentro i suoi criteri.\n\nNon chiuso con un terzo giro su SPEC-025 perche' il rilievo non blocca alcun criterio ne' alcuna gate, e la 5.1.0 vieta di aprire un giro per rilievi che non bloccano. La reviewer dichiara che il rimedio e' sei righe gia' scritte nella review, quindi il costo e' noto e non stimato."
    evidence_refs: ["SPEC-029"]
---
# FinalizedBlock::verify accetta un Block il cui carico l'header non impegna: il legame e' al confine della proposta e non alla verifica

## Statement

La remediation di REVIEW-047 RF-001 ha legato il carico al blocco **al confine della proposta**, in `verify_proposal`, e non alla verifica del blocco finalizzato. `FinalizedBlock::verify` continua a controllare tre cose — che il certificato punti al `block_id` calcolato dall'header, che le altezze coincidano, e che le firme verifichino — e **non ricalcola `transactions_root` da `transactions`**. Un `Block` che arrivi da un percorso diverso dalla proposta e' quindi accettato con qualunque carico.

## Evidence and provenance

Rilevato da AGENT-007 in REVIEW-048 RF-001 e misurato: certificato genuino piu' tre carichi divergenti su tre danno `Ok(())`. Verificato dal Lead il 2026-08-27 contando le occorrenze di `transactions_root` nell'`impl FinalizedBlock` di `core/coblox-core/src/consensus/certificate.rs`: **zero**. Il legame vive in `core/coblox-core/src/consensus/messages.rs:376`, dentro `verify_proposal`.

La reviewer osserva inoltre che questa passata ha reso il difetto **piu' difficile da vedere**, perche' ha scritto il legame accanto al campo gemello: chi legge `certificate.rs` vede una struttura `Block` con tre campi e una verifica che ne controlla due, senza che nulla segnali che il terzo e' impegnato altrove.

## Impact and scope boundary

Oggi il danno e' nullo, e va detto con la stessa precisione con cui si dice il resto: nessun percorso porta un `Block` da fuori, perche' non esiste rete e non esiste persistenza. **SPEC-029 introduce entrambe.** Da quel momento un blocco letto da disco al riavvio, o ricevuto da un pari in sincronizzazione, entra nello stato senza che il suo carico sia impegnato dall'header che il quorum ha firmato — ed e' la stessa classe di divergenza che REVIEW-047 RF-001 ha giudicato bloccante sul percorso della proposta.

E' inoltre la forma di argomento — "il danno e' interamente sul futuro" — che REVIEW-047 RF-007 ha rifiutato altrove nella stessa giornata, quando `verifier.rs` la usava per lasciare aperta una scappatoia. Registrarlo come debito con un bersaglio, invece di accettarlo come nota, e' cio' che impedisce di ripetere quell'errore.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead invece di aprire un terzo giro di remediation su SPEC-025. La spec e' al secondo giro, chiude il primo esito di M-02, e il rilievo non blocca alcun criterio di accettazione ne' alcuna gate dichiarata: la 5.1.0 vieta di aprire un giro per rilievi che non bloccano.

Instradato su SPEC-029 e non lasciato senza bersaglio perche' quella spec e' esattamente cio' che rende il buco raggiungibile — porta rete e persistenza — e la spec porta ora un criterio proprio che lo nomina. Chiuderlo li' e' piu' forte che chiuderlo qui: lo lega alla consegna in cui diventa vivo, invece di correggerlo mentre non lo e'.

## Resolution criteria

`FinalizedBlock::verify` ricalcola `transactions_root` dai `transactions` portati e rifiuta se non riproduce `header.transactions_root`, con la stessa definizione di `tx_id` che il confine della proposta gia' usa. Un test osserva il rifiuto su un `Block` con certificato genuino e carico divergente. La reviewer dichiara che sono **sei righe gia' scritte** nella review, quindi il costo e' noto e non stimato.

## Resolution evidence

