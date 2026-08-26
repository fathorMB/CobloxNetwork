---
id: DEBT-013
title: "Nessuna regola impone il passo di produzione dei blocchi: il set attivo decide la durata reale delle proprie epoche"
status: resolved
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
resolution_refs: ["SPEC-016","REVIEW-025","REVIEW-027","ADR-013"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-26
tags: ["consensus","governance","liveness","anti-capture"]
links: []
activity:
  - date: 2026-08-26
    action: "resolved: Chiuso nella forma che ADR-013 lascia disponibile e in nessuna altra: misurabile e dichiarato, non impedito. Nessuna regola interna alla catena poteva farlo, e nessun artefatto dice il contrario - README.md porta \"No rule of this protocol prevents that, and none can\" come probe della gate. Dire \"impedito\" avrebbe promesso piu' di quanto e' stato scritto.\n\nVa registrato che l'asimmetria del comportamento fuori banda, che il Lead aveva lodato come la parte migliore, poggiava su una premessa falsa scoperta da AGENT-007 in REVIEW-027: la distorsione della misura non e' in un verso solo, perche' issued_at_ms e' quando il checkpoint e' prodotto e non quando l'altezza che nomina e' raggiunta, e i blocchi della latenza di rilascio erano contati senza il loro tempo. L'asimmetria e' sopravvissuta ma su un criterio diverso e vero: cio' che separa le due direzioni e' cosa c'e' oltre la tolleranza, perche' nulla di onesto fa apparire blocchi mentre una lettura lenta e' indistinguibile dal ritardo del client a qualunque grandezza.\n\nLa correzione alla direzione del pericolo resta la parte piu' importante di questo debito: era vera quando AGENT-007 l'ha scritta ed e' la chiusura di DEBT-019 a renderla falsa. AGENT-007 la conferma e contesta la gerarchia dei moventi: il lato veloce costa un quorum contro un terzo, il guadagno e' pro quota e non esclusivo, non ha negabilita' ed e' osservabile senza banda. Il movente dominante resta il rallentamento, e il progetto fallisce chiuso sul lato debole - scelta giusta, ma da sapere.\n\nResta aperto per costruzione cio' che nessuna regola puo' chiudere: un set che rallenta non viola nulla. Cio' che cambia e' che ora si vede."
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
  - schema_version: "1"
    id: "DEBT-013-EVENT-002"
    timestamp: "2026-08-26T02:30:27.454714500+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Chiuso nella forma che ADR-013 lascia disponibile e in nessuna altra: misurabile e dichiarato, non impedito. Nessuna regola interna alla catena poteva farlo, e nessun artefatto dice il contrario - README.md porta \"No rule of this protocol prevents that, and none can\" come probe della gate. Dire \"impedito\" avrebbe promesso piu' di quanto e' stato scritto.\n\nVa registrato che l'asimmetria del comportamento fuori banda, che il Lead aveva lodato come la parte migliore, poggiava su una premessa falsa scoperta da AGENT-007 in REVIEW-027: la distorsione della misura non e' in un verso solo, perche' issued_at_ms e' quando il checkpoint e' prodotto e non quando l'altezza che nomina e' raggiunta, e i blocchi della latenza di rilascio erano contati senza il loro tempo. L'asimmetria e' sopravvissuta ma su un criterio diverso e vero: cio' che separa le due direzioni e' cosa c'e' oltre la tolleranza, perche' nulla di onesto fa apparire blocchi mentre una lettura lenta e' indistinguibile dal ritardo del client a qualunque grandezza.\n\nLa correzione alla direzione del pericolo resta la parte piu' importante di questo debito: era vera quando AGENT-007 l'ha scritta ed e' la chiusura di DEBT-019 a renderla falsa. AGENT-007 la conferma e contesta la gerarchia dei moventi: il lato veloce costa un quorum contro un terzo, il guadagno e' pro quota e non esclusivo, non ha negabilita' ed e' osservabile senza banda. Il movente dominante resta il rallentamento, e il progetto fallisce chiuso sul lato debole - scelta giusta, ma da sapere.\n\nResta aperto per costruzione cio' che nessuna regola puo' chiudere: un set che rallenta non viola nulla. Cio' che cambia e' che ora si vede."
    evidence_refs: ["SPEC-016", "REVIEW-025", "REVIEW-027", "ADR-013"]
---
# Nessuna regola impone il passo di produzione dei blocchi: il set attivo decide la durata reale delle proprie epoche

## Statement

ADR-013 fissa l'intervallo di blocco obiettivo a 5 secondi come costante di genesi dichiarata, e dichiara insieme che v0 non ha alcuna regola di validita che imponga quel passo. L'unico vincolo temporale su un blocco e che timestamp_ms superi la mediana degli undici precedenti, che impone monotonia e non cadenza. Ne consegue che il set di validatori attivo determina di fatto la durata in tempo reale delle proprie epoche, e quindi la propria incumbency: producendo blocchi piu lentamente allunga in giorni ogni quantita denominata in blocchi, fra cui election_epoch_blocks, candidacy_close_blocks, election_entropy_blocks, min_revocation_effective_delay_blocks e election_parameter_min_activation_gap_blocks. Le garanzie anti-cattura di SPEC-006 sono denominate in epoche e restano vere in epoche; e la loro traduzione in tempo reale a dipendere da chi le epoche le produce.

## Evidence and provenance

docs/protocol/ledger.md, sezione sull'intestazione di blocco: timestamp_ms e "constrained only to exceed the median of the previous 11". Una ricerca su tutti i documenti di docs/protocol/ non restituisce alcuna occorrenza di un intervallo di blocco, di un target di cadenza o di un limite superiore sulla distanza fra timestamp consecutivi. Il valore 5 s esisteva prima di ADR-013 in un solo punto del repository, sim/coblox_sim/recommended.py riga 21, dichiarato come assunzione.

Simmetria che rende l'omissione visibile: l'emissione e denominata in millisecondi (reward_epoch_ms) e SPEC-009 ne ha chiuso il denominatore con reward_epoch_ms_min e reward_epoch_ms_max in RewardBounds, sulla base dell'osservazione di REVIEW-014 che accorciare l'epoca moltiplica l'emissione reale senza violare alcun tetto. L'elezione e denominata in blocchi ed e limitata in blocchi da ElectionBounds. Un limite in blocchi non e un limite in tempo reale finche il blocco non ha una durata imposta. E la stessa domanda di REVIEW-014 — qual e il denominatore — applicata alla meta che quella review non guardava.

Osservazione del Lead durante la passata di decisioni di prodotto del 2026-08-25, non ancora sottoposta a review adversariale. Il Lead la registra come debito e non come finding proprio perche la superficie — il potere del set attivo — e quella su cui questo progetto ha gia sbagliato tre volte di seguito in SPEC-009, e la sua valutazione non deve essere l'ultima parola.

## Impact and scope boundary

~~Da stabilire, ed e precisamente questo il lavoro. La direzione del pericolo e verso il rallentamento, non verso l'accelerazione: blocchi piu lenti allungano i mandati in tempo reale e quindi l'incumbency, mentre blocchi piu veloci accorciano tutto e favoriscono il ricambio.~~

**Corretto dal Lead il 2026-08-26. La frase barrata sopra e' FALSA, non incompleta.** Era vera quando e' stata scritta, ed e' [SPEC-016] a renderla falsa chiudendo [DEBT-019]: derivando `reward_epoch` da `height`, **accelerare la produzione moltiplica l'emissione reale**, perche' l'indice dell'epoca avanza con l'altezza e non col tempo. Il pericolo ha ora **due direzioni**, e le due conseguenze **non si scambiano fra loro**: il lato lento e' l'incumbency, il lato veloce e' l'emissione. E' la ragione per cui la banda di cadenza introdotta da [SPEC-016] e' a due lati e per cui il light client fallisce chiuso proprio sul lato veloce.

La correzione e' stata portata da AGENT-002, che l'ha segnalata invece di correggerla in silenzio. **Non e' una correzione della valutazione di AGENT-007**, che su questo punto era esatta al momento in cui l'ha scritta: e' il rimedio ad aver cambiato il fatto. Vale la pena registrarlo, perche' e' un modo di sbagliare che nessuna gate cerca — **una chiusura che falsifica la descrizione del problema che chiudeva**, e che lascia la descrizione in piedi perche' nessuno pensa a rileggerla.

**Condizione di questa correzione.** [SPEC-016] e' in `review` e non ancora `done` al momento in cui questa nota e' scritta. Cio' che la rende gia' vera e' che la derivazione e' nell'albero e verificata dal Lead in `cadence.rs` — `check_mint_reward_epoch` impone `(e+1) * reward_epoch_blocks <= h`, e `reward_epoch_blocks` e' una costante di genesi. Se quella spec fosse abbandonata o la derivazione cambiata, **questa correzione va rivista e non conservata**: e' scritta contro un'implementazione, non contro una decisione. L'annotazione corrispondente su [ADR-013] e' deliberatamente rinviata a spec chiusa, perche' annotare una decisione accettata sulla base di codice in review sarebbe la pretesa scritta prima della regola.
 Un set che rallenta lo fa senza violare alcuna regola e senza che un light client possa dire che sia dovuto, perche la mediana degli undici e rispettata.

Tre effetti da valutare separatamente invece che in blocco, perche hanno gravita diverse: l'allungamento dell'incumbency; l'allungamento del ritardo effettivo di revoca, che e denominato in blocchi e ha la stessa esposizione; e l'interazione con l'emissione. ~~che e denominata in millisecondi e quindi non si muove — il che significa che un set che rallenta non guadagna nulla in crediti, e riduce l'ipotesi di movente a quella del mantenimento del seggio.~~ **Anche questa parte e' superata dal 2026-08-26.** La valutazione di AGENT-007 aveva gia' corretto il Lead una prima volta, stabilendo che l'emissione **si muove verso il basso** quando la catena rallenta, e che il costo e' esternalizzato perche' si perde l'emissione di tutta la rete conservando il seggio del solo cartello. [SPEC-016] aggiunge il verso opposto: accelerando, l'emissione **si muove verso l'alto e a beneficio di chi accelera**. Il movente non e' quindi ridotto al mantenimento del seggio ne' solo esternalizzato: sul lato veloce e' diretto.

Non e classificato high perche il vettore richiede il controllo del set attivo, cioe di chi propone i blocchi, e a quella soglia altre proprieta sono gia in discussione; e non e classificato low perche non richiede alcun quorum ai due terzi ne alcuna violazione, solo lentezza.

## Decision log

Created by project-lead: Aperto dal Lead il 2026-08-25 contestualmente ad ADR-013, che lo nomina fra le proprie conseguenze. Registrato come debito e non come contenuto della ADR perche la ADR decide un valore ed e competenza dell'operatore, mentre questa e un'osservazione di sicurezza che richiede verifica indipendente. Owner AGENT-007 e non il Lead ne AGENT-002: e un'osservazione del Lead, e affidarne la valutazione a chi l'ha fatta e esattamente il difetto che .lmbrain/knowledge/recurring-defects.md registra nella sezione sulla review adversariale, dove nessuna delle occorrenze censite e stata trovata da chi aveva scritto la modifica. Milestone M-02 perche va chiuso prima della devnet.

## Resolution criteria

Una valutazione adversariale che stabilisca se la cadenza non imposta sia un difetto sfruttabile o una proprieta accettabile di una federazione BFT, e che si pronunci separatamente sui tre effetti elencati sopra. Gli esiti ammissibili sono tre e vanno distinti: una regola di validita che vincoli la distanza fra timestamp consecutivi, nel qual caso la parte 3 di ADR-013 va riscritta e non annotata; la denominazione in tempo reale delle quantita di elezione che ne hanno bisogno, come gia fatto per l'emissione; oppure il rifiuto motivato, con la proprieta dichiarata nei documenti di protocollo accanto alla costante di ADR-013 invece che lasciata implicita.

Va chiuso prima che una devnet accumuli storia conservabile, per la stessa ragione per cui DEBT-005 e DEBT-012 non potevano essere rimandati: e una proprieta del consenso, e le proprieta del consenso si correggono a rete ferma.

## Resolution evidence

Chiuso da SPEC-016, accettata con REVIEW-025 e REVIEW-027 (GATE-SECREVIEW di AGENT-007) dopo due giri di remediation. Verificato dal Lead rieseguendo: 151 test da 126 di baseline, published_artifacts.py PASS con 103 probe C10 e 8 C11, prova in negativo con 15 mutazioni su 11 classi piu' tutte e 103 le probe individualmente, protocol_hashes.py senza valori mossi, clippy e fmt puliti.

Il light client misura la cadenza reale fra il checkpoint di soggettivita' debole che detiene e la testa autenticata, contando il tempo sul proprio orologio: nessuno dei due estremi e' un orologio della catena. La tolleranza e' la CadenceBand di genesi, con max_external_clock_slack_ms sul lato veloce. Comportamento asimmetrico: fallisce chiuso oltre la tolleranza sul lato veloce, segnala sul lato lento; il processo di rilascio fallisce chiuso in entrambi i versi e non puo' firmare un checkpoint il cui issued_at_ms sia oltre la tolleranza dopo la finalita' dell'altezza che nomina.

GATE-MEASURE-BINDS esercita ora anche il caso onesto con latenza di rilascio non nulla, che prima del rimedio dava Err(FasterThanBand). SECURITY.md porta il paragrafo con entrambe le direzioni e la distinzione fra terzo bloccante e quorum.</resolution_evidence>
</invoke>
