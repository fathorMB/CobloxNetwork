---
id: DEBT-013
title: "Nessuna regola impone il passo di produzione dei blocchi: il set attivo decide la durata reale delle proprie epoche"
status: open
category: "design"
severity: "medium"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-006","SPEC-007"]
related_reviews: []
related_decisions: ["ADR-013","ADR-001"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["consensus","governance","liveness","anti-capture"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-013-EVENT-001"
    timestamp: "2026-08-25T18:51:22.776840200+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead il 2026-08-25 contestualmente ad ADR-013, che lo nomina fra le proprie conseguenze. Registrato come debito e non come contenuto della ADR perche la ADR decide un valore ed e competenza dell'operatore, mentre questa e un'osservazione di sicurezza che richiede verifica indipendente. Owner AGENT-007 e non il Lead ne AGENT-002: e un'osservazione del Lead, e affidarne la valutazione a chi l'ha fatta e esattamente il difetto che .lmbrain/knowledge/recurring-defects.md registra nella sezione sulla review adversariale, dove nessuna delle occorrenze censite e stata trovata da chi aveva scritto la modifica. Milestone M-02 perche va chiuso prima della devnet."
    evidence_refs: []
---
# Nessuna regola impone il passo di produzione dei blocchi: il set attivo decide la durata reale delle proprie epoche

## Statement

ADR-013 fissa l'intervallo di blocco obiettivo a 5 secondi come costante di genesi dichiarata, e dichiara insieme che v0 non ha alcuna regola di validita che imponga quel passo. L'unico vincolo temporale su un blocco e che timestamp_ms superi la mediana degli undici precedenti, che impone monotonia e non cadenza. Ne consegue che il set di validatori attivo determina di fatto la durata in tempo reale delle proprie epoche, e quindi la propria incumbency: producendo blocchi piu lentamente allunga in giorni ogni quantita denominata in blocchi, fra cui election_epoch_blocks, candidacy_close_blocks, election_entropy_blocks, min_revocation_effective_delay_blocks e election_parameter_min_activation_gap_blocks. Le garanzie anti-cattura di SPEC-006 sono denominate in epoche e restano vere in epoche; e la loro traduzione in tempo reale a dipendere da chi le epoche le produce.

## Evidence and provenance

docs/protocol/ledger.md, sezione sull'intestazione di blocco: timestamp_ms e "constrained only to exceed the median of the previous 11". Una ricerca su tutti i documenti di docs/protocol/ non restituisce alcuna occorrenza di un intervallo di blocco, di un target di cadenza o di un limite superiore sulla distanza fra timestamp consecutivi. Il valore 5 s esisteva prima di ADR-013 in un solo punto del repository, sim/coblox_sim/recommended.py riga 21, dichiarato come assunzione.

Simmetria che rende l'omissione visibile: l'emissione e denominata in millisecondi (reward_epoch_ms) e SPEC-009 ne ha chiuso il denominatore con reward_epoch_ms_min e reward_epoch_ms_max in RewardBounds, sulla base dell'osservazione di REVIEW-014 che accorciare l'epoca moltiplica l'emissione reale senza violare alcun tetto. L'elezione e denominata in blocchi ed e limitata in blocchi da ElectionBounds. Un limite in blocchi non e un limite in tempo reale finche il blocco non ha una durata imposta. E la stessa domanda di REVIEW-014 — qual e il denominatore — applicata alla meta che quella review non guardava.

Osservazione del Lead durante la passata di decisioni di prodotto del 2026-08-25, non ancora sottoposta a review adversariale. Il Lead la registra come debito e non come finding proprio perche la superficie — il potere del set attivo — e quella su cui questo progetto ha gia sbagliato tre volte di seguito in SPEC-009, e la sua valutazione non deve essere l'ultima parola.

## Impact and scope boundary

Da stabilire, ed e precisamente questo il lavoro. La direzione del pericolo e verso il rallentamento, non verso l'accelerazione: blocchi piu lenti allungano i mandati in tempo reale e quindi l'incumbency, mentre blocchi piu veloci accorciano tutto e favoriscono il ricambio. Un set che rallenta lo fa senza violare alcuna regola e senza che un light client possa dire che sia dovuto, perche la mediana degli undici e rispettata.

Tre effetti da valutare separatamente invece che in blocco, perche hanno gravita diverse: l'allungamento dell'incumbency; l'allungamento del ritardo effettivo di revoca, che e denominato in blocchi e ha la stessa esposizione; e l'interazione con l'emissione, che e denominata in millisecondi e quindi non si muove — il che significa che un set che rallenta non guadagna nulla in crediti, e riduce l'ipotesi di movente a quella del mantenimento del seggio.

Non e classificato high perche il vettore richiede il controllo del set attivo, cioe di chi propone i blocchi, e a quella soglia altre proprieta sono gia in discussione; e non e classificato low perche non richiede alcun quorum ai due terzi ne alcuna violazione, solo lentezza.

## Decision log

Created by project-lead: Aperto dal Lead il 2026-08-25 contestualmente ad ADR-013, che lo nomina fra le proprie conseguenze. Registrato come debito e non come contenuto della ADR perche la ADR decide un valore ed e competenza dell'operatore, mentre questa e un'osservazione di sicurezza che richiede verifica indipendente. Owner AGENT-007 e non il Lead ne AGENT-002: e un'osservazione del Lead, e affidarne la valutazione a chi l'ha fatta e esattamente il difetto che .lmbrain/knowledge/recurring-defects.md registra nella sezione sulla review adversariale, dove nessuna delle occorrenze censite e stata trovata da chi aveva scritto la modifica. Milestone M-02 perche va chiuso prima della devnet.

## Resolution criteria

Una valutazione adversariale che stabilisca se la cadenza non imposta sia un difetto sfruttabile o una proprieta accettabile di una federazione BFT, e che si pronunci separatamente sui tre effetti elencati sopra. Gli esiti ammissibili sono tre e vanno distinti: una regola di validita che vincoli la distanza fra timestamp consecutivi, nel qual caso la parte 3 di ADR-013 va riscritta e non annotata; la denominazione in tempo reale delle quantita di elezione che ne hanno bisogno, come gia fatto per l'emissione; oppure il rifiuto motivato, con la proprieta dichiarata nei documenti di protocollo accanto alla costante di ADR-013 invece che lasciata implicita.

Va chiuso prima che una devnet accumuli storia conservabile, per la stessa ragione per cui DEBT-005 e DEBT-012 non potevano essere rimandati: e una proprieta del consenso, e le proprieta del consenso si correggono a rete ferma.

## Resolution evidence

