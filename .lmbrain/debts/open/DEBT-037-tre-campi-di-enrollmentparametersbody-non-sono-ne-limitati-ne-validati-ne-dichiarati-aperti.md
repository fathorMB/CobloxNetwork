---
id: DEBT-037
title: "Tre campi di EnrollmentParametersBody non sono ne limitati, ne validati, ne dichiarati aperti"
status: open
category: "security"
severity: "high"
origin_severity: null
area: "governance"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "REVIEW-038"
origin_ref: "RF-003"
related_specs: ["SPEC-023"]
related_reviews: ["REVIEW-038"]
related_decisions: ["ADR-007","ADR-010","ADR-012"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-26
tags: ["security","governance","identity"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-037-EVENT-001"
    timestamp: "2026-08-26T23:23:02.144758800+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead su finding di AGENT-007 in [REVIEW-038] RF-003, e tenuto separato da [DEBT-036] invece che fuso in esso, per due ragioni.\n\nLa prima e' pratica: [DEBT-036] ha gia' una spec che ne chiude la prima meta', e allargargli il perimetro a lavoro in corso renderebbe ambiguo cosa quella spec abbia chiuso.\n\nLa seconda e' che la forma va registrata da sola. **[DEBT-036] e' stato aperto guardando la classe invece dell'occorrenza, e la classe che ha guardato era ancora troppo stretta.** Il Lead aveva enumerato i venti campi di `ConsensusParametersBody` e concluso che quello fosse l'insieme; AGENT-007 ha chiesto se l'insieme dei **corpi firmati** fosse a sua volta enumerato, e non lo era.\n\nE' la settima occorrenza della famiglia dell'insieme dichiarato, ed e' la prima in cui **il difetto sta nell'insieme di insiemi**. La lezione che ne discende, e che vale oltre questo debito: **guardare la classe invece dell'occorrenza non basta se la classe e' a sua volta un'occorrenza.** Chi enumera deve dichiarare anche il perimetro dentro cui ha enumerato, che e' la stessa disciplina gia' scritta per le dimostrazioni."
    evidence_refs: []
---
# Tre campi di EnrollmentParametersBody non sono ne limitati, ne validati, ne dichiarati aperti

## Statement

`EnrollmentParametersBody` ha nove campi. Sei sono coperti da una regola di validita' o da un blocco di limiti: `pow_algorithm`, `lanes`, `tag_length_bytes`, `memory_kib`, `iterations`, `difficulty_bits`.

**Tre non lo sono**: `max_request_age_ms`, `max_future_skew_ms`, `recent_block_window`.

Ciascuno dei tre compare **una volta sola** in `docs/protocol/README.md` — la riga di schema — **zero volte** in `docs/protocol/ledger.md`, e **zero volte** nella lista DRAFT dei parametri di lancio. `EnrollmentParameters::validate()` li **legge in `from_body` e non li controlla mai**.

Sono della **stessa famiglia dei dieci di [DEBT-036]** — eta' di una richiesta, tolleranza di skew in avanti, finestra di freschezza — e non della famiglia economica. `identity.md` lo dichiara apertamente: `max_transport_attestation_future_skew_ms` e' stato modellato *«sul modello del `max_future_skew_ms` che la finestra di enrollment gia' usa»*.

**Il perimetro di [DEBT-036] e' quindi piu' stretto della classe che nomina**, e lo strumento `consensus_parameters_closure.py` non li vede per costruzione: la sua docstring dichiara il perimetro `ConsensusParametersBody`. **Lo strumento non e' in difetto; il perimetro della chiusura lo e'.**

## Evidence and provenance

Trovato da AGENT-007 in [REVIEW-038] RF-003, eseguendo `GATE-SECREVIEW` su [SPEC-023]. La reviewer aveva enumerato tutti e tredici i campi di `RewardPolicyBody` e tutti e cinque quelli di `HostingRateCardBody` **senza trovare nulla di scoperto**, e li ha trovati in un quarto corpo firmato che nessuno aveva guardato.

**Riverificato dal Lead in modo indipendente contro lo stato committato `065760f`**, e non contro l'albero di lavoro, perche' [SPEC-022] lo stava modificando. Per ciascuno dei tre: **una** occorrenza in `README.md`, **zero** in `ledger.md`, **zero** nella sezione DRAFT. In `params.rs`, la ricerca dentro `validate()` restituisce solo dichiarazioni di campo e chiamate `body.uint(...)` in `from_body`: **nessun controllo**.

Lo schema e' a `docs/protocol/README.md`, blocco `EnrollmentParametersBody`, nove campi.

## Impact and scope boundary

La superficie e' la difesa anti-Sybil, cioe' quella su cui [ADR-007] poggia.

**Lo scenario piu' concreto e' `recent_block_window`.** La prova Argon2id di enrollment e' ancorata a `recent_block_id`, e `recent_block_height` non puo' stare piu' di `recent_block_window` dietro l'ultima altezza finalizzata. Un quorum sedente che voglia diluire la difesa pubblica un `enrollment_parameters` con quella finestra enorme: nessuna regola lo rifiuta, ne' nei documenti ne' in codice. Un attaccante puo' allora **precalcolare** tag Argon2id contro un blocco vecchio per l'intera durata della finestra e poi riversare N enrollment in un colpo. **Il costo per identita' non cambia; il costo di picco per una flotta crolla**, ed e' il costo di picco la proprieta' che l'ancoraggio a un blocco recente esiste per imporre.

Simmetricamente, portato a zero **nessun enrollment e' piu' costruibile** e la rete si chiude alle identita' nuove: e' un pavimento mancante, della stessa forma che [REVIEW-038] RF-007 osserva sugli altri dieci.

`max_future_skew_ms` ha lo stesso profilo di rischio che l'analisi di [SPEC-023] descrive per il proprio discendente sul trasporto, con un'aggravante: al minimo, il nodo escluso e' quello **appena installato**, che per `identity.md` non ha ancora alcun canale da cui correggere il proprio orologio.

`high` per la stessa ragione di [DEBT-036]: nessuno sa che vanno decisi, e finche' non sono dichiarati aperti il fatto che non siano nemmeno limitati non ha modo di farsi notare.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead su finding di AGENT-007 in [REVIEW-038] RF-003, e tenuto separato da [DEBT-036] invece che fuso in esso, per due ragioni.

La prima e' pratica: [DEBT-036] ha gia' una spec che ne chiude la prima meta', e allargargli il perimetro a lavoro in corso renderebbe ambiguo cosa quella spec abbia chiuso.

La seconda e' che la forma va registrata da sola. **[DEBT-036] e' stato aperto guardando la classe invece dell'occorrenza, e la classe che ha guardato era ancora troppo stretta.** Il Lead aveva enumerato i venti campi di `ConsensusParametersBody` e concluso che quello fosse l'insieme; AGENT-007 ha chiesto se l'insieme dei **corpi firmati** fosse a sua volta enumerato, e non lo era.

E' la settima occorrenza della famiglia dell'insieme dichiarato, ed e' la prima in cui **il difetto sta nell'insieme di insiemi**. La lezione che ne discende, e che vale oltre questo debito: **guardare la classe invece dell'occorrenza non basta se la classe e' a sua volta un'occorrenza.** Chi enumera deve dichiarare anche il perimetro dentro cui ha enumerato, che e' la stessa disciplina gia' scritta per le dimostrazioni.

## Resolution criteria

Tre lavori, e il terzo e' quello che chiude la forma invece dell'occorrenza.

**1. L'analisi delle quattro domande per i tre campi**, nello stesso formato dell'analisi dei dieci: cosa governa, cosa ottiene un quorum ai due estremi, cosa gia' lo vincola, e **da quale grandezza dipende la proprieta' voluta**. Su `recent_block_window` la domanda 4 e' quella che conta: la proprieta' e' il **costo di picco per una flotta**, non la freschezza in se'.

**2. La copertura**: DRAFT, limite di genesi, o regola di validita' in `params.rs`. Non necessariamente la stessa per i tre — e' la stessa avvertenza di [DEBT-036], dove aggiungere tetti in blocco sarebbe famiglia 3.

**3. L'estensione del perimetro della chiusura.** `consensus_parameters_closure.py` va esteso a `EnrollmentParametersBody`, oppure affiancato da uno strumento gemello. La chiusura richiede che **la prova in negativo dello strumento esteso catturi un campo di `EnrollmentParametersBody` scoperto**, osservata fallire.

**Va inoltre deciso se la classe sia chiusa a quattro corpi firmati.** [REVIEW-038] ha enumerato `RewardPolicyBody`, `HostingRateCardBody`, `ConsensusParametersBody` ed `EnrollmentParametersBody`, ma la domanda giusta e' se un quinto corpo firmato possa nascere fuori da ogni lista, come questo e' rimasto fuori.

**Rilievo di forma da chiudere qui**, segnalato da [REVIEW-038]: la lista DRAFT scrive la terna Argon2id come `memory_kib`, `lanes`, `passes`. Il campo si chiama **`iterations`**; `passes` e' il termine della RFC 9106. E' una grandezza con due grafie, ed e' la stessa forma per cui `max_clock_drift_ms` non e' stato trovato in `ledger.md`.

Da chiudere **prima della devnet**, per la stessa ragione di [DEBT-036].

## Resolution evidence

