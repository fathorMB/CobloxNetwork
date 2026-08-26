---
id: DEBT-019
title: "reward_epoch non ha una regola di derivazione dal tempo"
status: resolved
category: "correctness"
severity: "high"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: "SPEC-013"
origin_ref: "osservazione adiacente di AGENT-007 nella valutazione di DEBT-013 e DEBT-014"
related_specs: ["SPEC-009","SPEC-011"]
related_reviews: ["REVIEW-014"]
related_decisions: ["ADR-010","ADR-011"]
target_specs: []
blocked_by: []
resolution_refs: ["SPEC-016","REVIEW-025","REVIEW-027"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-26
tags: ["ledger","economy","conformance"]
links: []
activity:
  - date: 2026-08-26
    action: "resolved: Chiuso con l'esito forte che il debito ammetteva come primo: la derivazione esiste, e non e' stata necessaria la dimostrazione del contrario. height e' l'unica grandezza di questa catena che un validatore non scrive liberamente, perche' e' previous + 1 e ricontrollabile da chiunque dalle sole intestazioni.\n\nIl limite che ne discende e' enunciato stretto quanto e' vero, ed e' la parte che vale: per blocco e non per millisecondo reale. Il residuo e' esattamente il residuo di DEBT-013, quindi i due debiti si chiudono l'uno dentro l'altro invece di rimandarsi.\n\nLa conseguenza che nessuno aveva previsto e che questa chiusura ha prodotto: legando l'epoca all'altezza, accelerare la produzione moltiplica l'emissione reale. Il pericolo acquista una seconda direzione, e le due conseguenze non si scambiano fra loro - il lato lento e' l'incumbency, il lato veloce e' l'emissione. E' la ragione per cui la banda di cadenza e' a due lati. Va registrato come modo di sbagliare che nessuna gate cerca: una chiusura che falsifica la descrizione del problema che chiudeva, e che lascia la descrizione in piedi perche' nessuno pensa a rileggerla. La descrizione e' stata corretta in DEBT-013 e in SECURITY.md.\n\nEntrambi i versi sono trattati, e il secondo non e' chiuso da una regola: nessuna regola interna puo' obbligare un quorum a mintare, perche' una regola rifiuta un atto e non ne impone uno. E' chiuso rendendo il ritardo ricalcolabile. Una scadenza di liquidazione e' stata valutata e rifiutata perche' non obbligherebbe comunque a mintare e trasformerebbe un'interruzione onesta in reddito perso per sempre."
debt_events:
  - schema_version: "1"
    id: "DEBT-019-EVENT-001"
    timestamp: "2026-08-25T23:31:36.819325200+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead su osservazione adiacente di AGENT-007, che ha scelto di riportarla invece di crearne il debito da se, dichiarando il limite della propria istruttoria. Owner AGENT-002 perche possiede la superficie economica e del ledger, ed e l'autrice di SPEC-009 da cui questa questione discende direttamente."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-019-EVENT-002"
    timestamp: "2026-08-26T02:30:47.855845200+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Chiuso con l'esito forte che il debito ammetteva come primo: la derivazione esiste, e non e' stata necessaria la dimostrazione del contrario. height e' l'unica grandezza di questa catena che un validatore non scrive liberamente, perche' e' previous + 1 e ricontrollabile da chiunque dalle sole intestazioni.\n\nIl limite che ne discende e' enunciato stretto quanto e' vero, ed e' la parte che vale: per blocco e non per millisecondo reale. Il residuo e' esattamente il residuo di DEBT-013, quindi i due debiti si chiudono l'uno dentro l'altro invece di rimandarsi.\n\nLa conseguenza che nessuno aveva previsto e che questa chiusura ha prodotto: legando l'epoca all'altezza, accelerare la produzione moltiplica l'emissione reale. Il pericolo acquista una seconda direzione, e le due conseguenze non si scambiano fra loro - il lato lento e' l'incumbency, il lato veloce e' l'emissione. E' la ragione per cui la banda di cadenza e' a due lati. Va registrato come modo di sbagliare che nessuna gate cerca: una chiusura che falsifica la descrizione del problema che chiudeva, e che lascia la descrizione in piedi perche' nessuno pensa a rileggerla. La descrizione e' stata corretta in DEBT-013 e in SECURITY.md.\n\nEntrambi i versi sono trattati, e il secondo non e' chiuso da una regola: nessuna regola interna puo' obbligare un quorum a mintare, perche' una regola rifiuta un atto e non ne impone uno. E' chiuso rendendo il ritardo ricalcolabile. Una scadenza di liquidazione e' stata valutata e rifiutata perche' non obbligherebbe comunque a mintare e trasformerebbe un'interruzione onesta in reddito perso per sempre."
    evidence_refs: ["SPEC-016", "REVIEW-025", "REVIEW-027"]
---
# reward_epoch non ha una regola di derivazione dal tempo

## Statement

Nessun documento deriva reward_epoch da alcunche. Il campo compare nei corpi di mint, nelle foglie Merkle e in una regola di unicita, ma non esiste una regola che lo leghi a timestamp_ms, a reward_epoch_ms o ad altro. Ne segue che reward_epoch_ms_min vincola la durata dichiarata in un documento firmato e non la velocita con cui gli indici avanzano nei mint: un quorum conforme che incrementa reward_epoch a ogni blocco resta valido, e il pavimento che SPEC-009 ha introdotto per impedire che accorciare l'epoca moltiplichi l'emissione reale non morde su quel percorso.

Se la lettura e corretta, e famiglia 3 un livello sotto RF-002 di REVIEW-014: quel finding aveva vincolato la durata dichiarata, e questa e la grandezza da cui l'emissione reale dipende davvero.

## Evidence and provenance

Osservazione di AGENT-007 riportata al Lead al termine della valutazione di DEBT-013 e DEBT-014, con la propria confidenza dichiarata: ricerca documentale esaustiva su docs/protocol/ e core/coblox-core, nessuna istruttoria oltre, e la richiesta esplicita che non sia affermata come certa prima di essere attribuita a chi possiede quella superficie.

Verificato dal Lead in modo indipendente: reward_epoch compare diciannove volte fra ledger.md e README.md, e l'unico MUST che lo nomina riguarda i limiti su reward_epoch_ms, non la derivazione dell'indice. Nessuna occorrenza lo deriva.

## Impact and scope boundary

Da stabilire, ed e il lavoro. La direzione del pericolo e verso l'alto sulla velocita di avanzamento dell'indice: piu rapidamente avanza, piu fondi di esistenza vengono emessi per unita di tempo reale, che e esattamente il fallimento che ADR-010 e SPEC-009 esistono per impedire.

Va valutato anche il verso opposto, cioe un indice che non avanza affatto, perche congelerebbe l'emissione senza violare alcuna regola: e il gemello del caso che README gia dichiara invalido per reward_epoch_ms sopra il tetto.

Severita high e non critical perche nessuna rete esiste e la correzione e una regola di derivazione, non una riprogettazione; ma tocca l'emissione, che e la superficie su cui il progetto ha speso due spec e quattro giri di review.

## Decision log

Created by project-lead: Aperto dal Lead su osservazione adiacente di AGENT-007, che ha scelto di riportarla invece di crearne il debito da se, dichiarando il limite della propria istruttoria. Owner AGENT-002 perche possiede la superficie economica e del ledger, ed e l'autrice di SPEC-009 da cui questa questione discende direttamente.

## Resolution criteria

Una regola normativa che deriva reward_epoch da una grandezza che i validatori non scrivono liberamente, oppure la dimostrazione che una tale regola non e ottenibile dentro la catena, nel qual caso vale la proposizione generale che AGENT-007 ha stabilito su DEBT-013 e la chiusura ha la stessa forma: rendere l'avanzamento misurabile da fuori invece che vincolabile da dentro. Con la sua fixture di frontiera e la prova in negativo.

Da chiudere prima che una devnet emetta reddito di esistenza.

## Resolution evidence

Chiuso da SPEC-016. check_mint_reward_epoch in core/coblox-core/src/cadence.rs impone che un mint che nomina reward_epoch e sia valido solo a un'altezza h con (e+1) * reward_epoch_blocks <= h, dove reward_epoch_blocks e' reward_epoch_ms.div_ceil(block_interval_ms), costante di genesi. Verificato dal Lead leggendo il codice.

GATE-BOTH-DIRECTIONS esercita entrambi i versi con un caso ciascuno: indice che avanza troppo in fretta - mint(reward_epoch=42) all'altezza 42 respinto, frontiera verificata da entrambi i lati a 311 040 e 311 039 - e indice fermo, con settleable_reward_epoch e reward_epoch_lag che rendono il ritardo ricalcolabile. Provata in negativo disattivando il pavimento: due test cadono.

AGENT-007 ha attaccato la derivazione dal lato dell'evasione in REVIEW-027 senza romperla: ceil piu' reward_epoch_ms_min piu' le due regole di unicita' reggono.</resolution_evidence>
</invoke>
