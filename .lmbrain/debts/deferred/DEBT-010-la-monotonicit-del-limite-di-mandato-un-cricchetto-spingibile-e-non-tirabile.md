---
id: DEBT-010
title: "La monotonicità del limite di mandato è un cricchetto spingibile e non tirabile"
status: deferred
category: "design"
severity: "medium"
origin_severity: null
area: "consensus"
milestone: "M-07"
owner: "AGENT-002"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-006"]
related_reviews: ["REVIEW-010"]
related_decisions: ["ADR-001"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: "Riesaminare a M-07, oppure prima se la spec di M-02 che tocca i parametri di consenso produce la dimostrazione richiesta. La dimostrazione da produrre, come criterio di quella spec e verificata da chi ha mandato di attaccarla: che il tetto di genesi validator_max_consecutive_terms_max = 12 contro T = 9, con epoca di elezione di 120 960 blocchi a 5 s per blocco, renda irrilevante una spinta irreversibile del cricchetto — sia sul pavimento di ricambio ceil(V/T) <= c, sia sull'incumbency massima in tempo reale. Se la dimostrazione regge, chiudere come accepted-risk citandola. Se non regge, il debito torna attivo e la leva e il valore del tetto nell'ancora di genesi, non la regola di monotonicita, che SPEC-006 ha stabilito correttamente. Riaprire in ogni caso se i documenti governati acquisiranno un'attivazione condizionata allo stato della catena, nel qual caso la regola a finestra registrata nel debito sostituisce la monotonicita."
created: 2026-08-25
updated: 2026-08-25
tags: ["consensus","governance","liveness"]
links: []
activity:
  - date: 2026-08-25
    action: "deferred: Disposizione decisa dall'operatore il 2026-08-25 nella passata di chiusura delle decisioni di prodotto. Non chiuso come rischio accettato, benche i numeri tarati lo suggeriscano: con ADR-013 che fissa il blocco a 5 s, una spinta irreversibile porta l'incumbency massima da 63 a 84 giorni e il pavimento di ricambio non si muove, perche ceil(27/9) e ceil(27/12) valgono entrambi 3. Ma questa e aritmetica del Lead, non la dimostrazione che il debito stesso pone come condizione di chiusura, e accettare un rischio sulla base di un'affermazione non dimostrata e la forma di difetto censita in .lmbrain/knowledge/recurring-defects.md famiglia 2. Il differimento formale conserva la condizione invece di lasciare il debito aperto senza scadenza, che e il modo in cui tre review erano rimaste ferme sulla board."
debt_events:
  - schema_version: "1"
    id: "DEBT-010-EVENT-001"
    timestamp: "2026-08-25T14:48:52.263120900+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto su raccomandazione esplicita di AGENT-007 in REVIEW-010, giro 4, e non come riserva del Lead sulla decisione: la monotonicita e la scelta giusta per v0 e la review l'ha accettata. Registrato come debito separato invece che dentro SPEC-006 perche non e lavoro rimandato ma una condizione che diventa azionabile solo se cambia una proprieta del protocollo che oggi non esiste, l'attivazione condizionata dei documenti governati. Severita medium e non high: il vettore richiede un quorum ai due terzi, dove la safety e gia compromessa, quindi il debito riguarda la resilienza dopo un compromesso transitorio e non la difesa contro uno stabile. Milestone M-07 perche e li che la roadmap colloca rotazione automatica e hardening, ed e la prima occasione in cui l'attivazione condizionata potrebbe comparire."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-010-EVENT-002"
    timestamp: "2026-08-25T18:50:28.161182900+02:00"
    action: "deferred"
    from_status: "open"
    to_status: "deferred"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Disposizione decisa dall'operatore il 2026-08-25 nella passata di chiusura delle decisioni di prodotto. Non chiuso come rischio accettato, benche i numeri tarati lo suggeriscano: con ADR-013 che fissa il blocco a 5 s, una spinta irreversibile porta l'incumbency massima da 63 a 84 giorni e il pavimento di ricambio non si muove, perche ceil(27/9) e ceil(27/12) valgono entrambi 3. Ma questa e aritmetica del Lead, non la dimostrazione che il debito stesso pone come condizione di chiusura, e accettare un rischio sulla base di un'affermazione non dimostrata e la forma di difetto censita in .lmbrain/knowledge/recurring-defects.md famiglia 2. Il differimento formale conserva la condizione invece di lasciare il debito aperto senza scadenza, che e il modo in cui tre review erano rimaste ferme sulla board."
    evidence_refs: []
---
# La monotonicità del limite di mandato è un cricchetto spingibile e non tirabile

## Statement

[SPEC-006] chiude la risincronizzazione delle scadenze imponendo che `validator_max_consecutive_terms` sia monotono non decrescente su catena viva. La scelta e corretta e il suo costo e dichiarato onestamente nel documento come rifiuto sul costo e non come impossibilita, ma il costo ha una conseguenza che merita di essere sorvegliata: il cricchetto e spingibile da un avversario e non tirabile indietro da nessuno. Un quorum che tocchi i due terzi anche una sola volta porta il limite di mandato al tetto di genesi in modo permanente, e da li il pavimento di ricambio resta degradato per sempre. Esiste una regola piu permissiva che non ha questo costo, ed e nota: e valutata all'attivazione contro il set allora attivo, e richiede che nessuno dei valori della finestra di sovrapposizione sia occupato da un timbro esistente. Non e adottabile in v0 perche presuppone un documento governato la cui attivazione sia condizionata allo stato della catena, concetto che il protocollo non ha.

## Evidence and provenance

REVIEW-010, giro 4, valutazione di AGENT-007 sul costo della porta a senso unico, accettata dal Lead. La reviewer ha verificato indipendentemente la dimostrazione di AGENT-002 secondo cui la forma permissiva collassa sulla monotonicita quando valutata all'accettazione, e l'ha confermata: fra accettazione e attivazione puo cadere un confine, e una coorte insediata all'altezza precedente porta il timbro calcolato col limite vecchio, quindi la garanzia piu forte ottenibile all'accettazione e esattamente `T_new >= T_old`. Ha aggiunto due precisazioni. La prima: il collasso dipende dal fatto che un confine cada in mezzo, e se non ne cade nessuno l'insieme dei timbri e gia definitivo all'accettazione, quindi l'affermazione che la forma permissiva non sia valutabile in accettazione e vera nel caso che conta e non in generale. La seconda, che e la sostanza di questo debito: la regola permissiva che funzionerebbe all'attivazione non e quella scritta in REVIEW-010 al giro 3, perche il confronto con il solo massimo dei timbri e sufficiente ma inutilmente stretto. La condizione esatta e che nessuno dei valori della finestra `{e + T_new : e_a <= e <= e_a - 1 + T_old - T_new}` sia occupato, dato che la collisione si ripresenta a ogni confine della finestra e non solo al primo. Il documento di protocollo registra la disponibilita futura della regola permissiva ma non la sua forma corretta.

## Impact and scope boundary

Contro un avversario stabile sopra i due terzi la monotonicita non toglie nulla, perche a quella soglia la safety BFT e gia persa. Il danno riguarda i casi che il protocollo esiste per sopravvivere: un quorum transitorio sopra i due terzi, o un semplice errore dell'operatore, alzano il limite di mandato in modo irreversibile e lasciano il pavimento di ricambio degradato per il resto della vita della rete. Da quel momento il tetto di genesi `validator_max_consecutive_terms_max` diventa l'unico presidio residuo sulla velocita di rotazione, il che ha una conseguenza operativa immediata per la taratura di M-02: quel tetto va scelto stretto quanto la rete tollera, perche e cio che limita il danno di una spinta irreversibile. Il costo si somma inoltre a quello gia dichiarato del pavimento di contrazione, che converte un degrado in un arresto quando la rete perde piu di un terzo dei validatori vivi fra due confini.

## Decision log

Created by project-lead: Aperto su raccomandazione esplicita di AGENT-007 in REVIEW-010, giro 4, e non come riserva del Lead sulla decisione: la monotonicita e la scelta giusta per v0 e la review l'ha accettata. Registrato come debito separato invece che dentro SPEC-006 perche non e lavoro rimandato ma una condizione che diventa azionabile solo se cambia una proprieta del protocollo che oggi non esiste, l'attivazione condizionata dei documenti governati. Severita medium e non high: il vettore richiede un quorum ai due terzi, dove la safety e gia compromessa, quindi il debito riguarda la resilienza dopo un compromesso transitorio e non la difesa contro uno stabile. Milestone M-07 perche e li che la roadmap colloca rotazione automatica e hardening, ed e la prima occasione in cui l'attivazione condizionata potrebbe comparire.

## Resolution criteria

Da riaprire quando i documenti governati acquisiranno un'attivazione condizionata allo stato della catena, se mai accadra. A quel punto la regola a finestra sostituisce la monotonicita, va scritta nella forma esatta registrata qui e non nella forma con il solo massimo, e il rifiuto sul costo registrato in [SPEC-006] va superato con la motivazione aggiornata. Fino ad allora il debito e sorveglianza e non lavoro: il presidio e il valore di `validator_max_consecutive_terms_max` nell'ancora di fiducia della genesi, che il simulatore economico di M-02 deve tarare tenendo conto che e l'unico limite residuo dopo una spinta irreversibile. Chiudibile anche come accepted-risk se M-02 dimostrasse che un tetto di genesi sufficientemente stretto rende la spinta irrilevante.

## Resolution evidence

