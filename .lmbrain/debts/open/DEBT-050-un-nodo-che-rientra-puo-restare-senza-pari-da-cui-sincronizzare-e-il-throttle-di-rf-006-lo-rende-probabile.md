---
id: DEBT-050
title: "Un nodo che rientra puo' restare senza pari da cui sincronizzare, e il throttle di RF-006 lo rende probabile"
status: open
category: "correctness"
severity: "high"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-029"]
related_reviews: ["REVIEW-049"]
related_decisions: []
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: []
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-050-EVENT-001"
    timestamp: "2026-08-27T22:18:38.628288+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Accertato dal Lead alla terza esecuzione, dopo due diagnosi sbagliate. Non corretto dentro questa sessione perche' l'operatore l'ha chiusa, e perche' il rimedio e' una scelta di progetto fra due strade e non una sistemazione meccanica: appartiene a chi ha scritto il throttle, con le tre trascrizioni gia' in mano."
    evidence_refs: []
---
# Un nodo che rientra puo' restare senza pari da cui sincronizzare, e il throttle di RF-006 lo rende probabile

## Statement

Il recupero di un nodo che riparte indietro dipende dalla presenza dei pari, e nulla la garantisce. Con il throttle introdotto da [REVIEW-049] RF-006 — `MAX_BLOCKS_PER_SYNC_RESPONSE = 8` e `MIN_MS_BETWEEN_SYNC_ANSWERS = 1000` — il recupero costa secondi di presenza altrui, mentre un nodo che raggiunge il proprio `--target-height` esce senza riguardo per chi e' rimasto indietro. In devnet il bersaglio e' esplicito; su una rete vera l'equivalente e' un pari che si spegne, e la forma del difetto non cambia.

## Evidence and provenance

Tre esecuzioni di CI sul job `Rust (ubuntu-latest)`, tutte con `--test-threads=1` e in release: run 33110165893 `Counts: [8, 8, 8, 3]` con scadenza 20s; suo rilancio `Counts: [8, 8, 7, 8]` con scadenza 20s; run 33111841066 `Counts: [8, 8, 8, 5]` con scadenza 45s, finita in 46,51s. Il terzo dato e' decisivo: con piu' del doppio del tempo il nodo riavviato passa da 3 a 5 invece che a 8, quindi la scadenza non era la causa. Il meccanismo e' in `core/coblox-node/src/node.rs`, nel ramo che restituisce `Ok(true)` quando `blk_height >= target`: il nodo esce. Il test passa in locale su Windows perche' li' il recupero rientra nella finestra in cui i pari sono ancora vivi.

## Impact and scope boundary

`GATE-CI-GREEN` di [SPEC-029] non e' verde, e `validator_crash_and_restart_recovers_without_equivocation` — cioe' il test che sostiene `GATE-RESTART-NO-EQUIVOCATION` — fallisce in pipeline su Linux. Il difetto tocca inoltre la proprieta' che la spec esiste per dimostrare: un validatore che rientra e non recupera non e' un validatore recuperato. Aggravante: due tentativi del Lead di attribuirlo altrove (prima al throttle senza prove, poi alla lentezza della macchina) hanno prodotto un commento falso dentro il test, poi corretto.

## Decision log

Created by project-lead: Accertato dal Lead alla terza esecuzione, dopo due diagnosi sbagliate. Non corretto dentro questa sessione perche' l'operatore l'ha chiusa, e perche' il rimedio e' una scelta di progetto fra due strade e non una sistemazione meccanica: appartiene a chi ha scritto il throttle, con le tre trascrizioni gia' in mano.

## Resolution criteria

Il test `validator_crash_and_restart_recovers_without_equivocation` passa su `Rust (ubuntu-latest)` in tre esecuzioni consecutive, senza che alcuna scadenza sia stata allentata per ottenerlo. La correzione deve nominare quale delle due strade prende: i pari restano disponibili finche' un ritardatario noto ha recuperato, oppure il recupero smette di dipendere da un throttle tarato sull'abuso. La distinzione fra un pari che tace per abuso e uno che tace perche' sta recuperando va scritta, non lasciata implicita.

## Resolution evidence

