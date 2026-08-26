---
id: DEBT-032
title: "Le probe della guida vedono la pagina allontanarsi da se stessa, non dal protocollo"
status: open
category: "verification"
severity: "medium"
origin_severity: null
area: "design"
milestone: "M-08"
owner: "AGENT-006"
origin_artifact: "REVIEW-031"
origin_ref: "RF-002"
related_specs: ["SPEC-015","SPEC-016","SPEC-021"]
related_reviews: ["REVIEW-031"]
related_decisions: ["ADR-012"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-26
tags: ["verification-gap","documentation"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-032-EVENT-001"
    timestamp: "2026-08-26T13:03:16.563387800+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead chiudendo [DEBT-023], perche' quel debito conteneva due cose e ne discharge una sola.\n\nL'obbligo - la review prima della pubblicazione - e' soddisfatto. La clausola \"con la versione definitiva del protocollo sotto mano\" **non lo e' alla lettera**, perche' il protocollo non e' definitivo, e non lo sara' per parecchie milestone. Chiudere [DEBT-023] senza registrare quella meta' avrebbe fatto sparire con esso il rischio che quella clausola intercettava, che e' il gesto che questo progetto censisce come famiglia 2.\n\nIl rischio non e' pero' lo stesso e non ha lo stesso rimedio: [DEBT-023] chiedeva **un evento** che e' avvenuto, questo chiede **un meccanismo** che non esiste. Tenerli in un debito solo li avrebbe fatti chiudere insieme al primo dei due."
    evidence_refs: []
---
# Le probe della guida vedono la pagina allontanarsi da se stessa, non dal protocollo

## Statement

Le 137 probe della guida pubblica falliscono quando **la pagina cambia** e la frase pinnata sparisce. Non falliscono quando **il protocollo cambia** e la pagina resta ferma.

Ne segue che la guida puo' diventare falsa senza che nulla lo dica, ed e' precisamente cio' che e' successo: fra il 25 e il 26 agosto 2026 [SPEC-016], [SPEC-017] e [SPEC-021] hanno mosso il protocollo, la pagina non e' stata toccata, **tutte le probe sono rimaste verdi**, e la security review vi ha trovato **tre affermazioni `high` diventate false**.

## Evidence and provenance

[REVIEW-031], nove finding di cui tre `high`, su una pagina i cui due strumenti passavano entrambi al momento della review: `published_artifacts.py` verde su 126 probe e `check-guide-pairs.mjs` verde su 65 `claims`, in entrambi i versi.

Nessuno dei tre `high` era falso quando AGENT-006 ha scritto la pagina, e la reviewer lo dichiara: una review fatta il 25 agosto avrebbe trovato due `low` e avrebbe assolto la guida, firmandola. L'intervallo fra la scrittura e la falsita' e' stato di **ventiquattro ore**.

I tre casi sono verificabili uno per uno: la promessa di rotazione era vera in blocchi e lo e' rimasta, ma [SPEC-016] ha reso misurabile - e quindi dicibile - che in tempo reale non lo e'; il tetto sull'emissione era limitato in modo indeterminato finche' `reward_epoch` non e' stato derivato da `height`, e da allora e' limitato **per epoca**; la soglia dei due terzi era l'unica scritta finche' i quattro noni non sono entrati in `SECURITY.md`.

## Impact and scope boundary

La guida e' l'artefatto che piu' di ogni altro **insegna** il sistema, ed e' scritto per l'on-boarding e per la trasparenza. Cio' che afferma verra' creduto e ripetuto da chi non ha modo di verificarlo.

Il difetto non e' che la pagina contenga errori: e' che **il meccanismo che dovrebbe accorgersene guarda dalla parte sbagliata**. Una probe lega una frase della pagina a una regola del protocollo e verifica che la frase ci sia ancora. Nessuna verifica che la **regola** sia ancora quella. La direzione non sorvegliata e' esattamente quella in cui il protocollo si muove.

E la superficie cresce: ogni spec che tocca `docs/protocol/` o `SECURITY.md` puo' invecchiare la guida, e M-02 non e' finita.

`medium` e non `high` perche' la guida non e' ancora pubblicata su un canale permanente e perche' la review appena fatta l'ha riportata in pari; `medium` e non `low` perche' il ciclo si e' gia' chiuso una volta in ventiquattro ore e non c'e' ragione di credere che non si richiuda.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead chiudendo [DEBT-023], perche' quel debito conteneva due cose e ne discharge una sola.

L'obbligo - la review prima della pubblicazione - e' soddisfatto. La clausola "con la versione definitiva del protocollo sotto mano" **non lo e' alla lettera**, perche' il protocollo non e' definitivo, e non lo sara' per parecchie milestone. Chiudere [DEBT-023] senza registrare quella meta' avrebbe fatto sparire con esso il rischio che quella clausola intercettava, che e' il gesto che questo progetto censisce come famiglia 2.

Il rischio non e' pero' lo stesso e non ha lo stesso rimedio: [DEBT-023] chiedeva **un evento** che e' avvenuto, questo chiede **un meccanismo** che non esiste. Tenerli in un debito solo li avrebbe fatti chiudere insieme al primo dei due.

## Resolution criteria

Serve un controllo che fallisca quando **la regola citata da una probe si muove**, non quando la frase sparisce.

La forma da esplorare per prima, perche' il progetto ha gia' l'apparato: ogni `claims` porta gia' il riferimento alla regola che la tiene. Se quel riferimento portasse anche un'**impronta del testo della regola** al momento in cui la probe e' stata scritta, un cambiamento della regola renderebbe l'impronta stale e il controllo rosso — e chi ha mosso la regola sarebbe la persona giusta per riguardare la frase, perche' e' l'unica che sa cosa e' cambiato.

**Il rimedio apparente da non adottare:** una revisione periodica a calendario. Non e' una guardia, e' un promemoria; fallisce esattamente come e' fallito qui, cioe' nessuno se ne accorge finche' qualcuno non guarda. E la lezione della deroga di [DEBT-023] e' l'opposto: la condizione utile non era una data ma un evento.

Da chiudere **prima che la guida sia pubblicata su un canale permanente**, cioe' prima di M-08. Finche' resta un file che si manda a mano, il ciclo di invecchiamento e' visibile a chi lo manda.

## Resolution evidence

## Nota del Lead — 2026-08-26

**L'operatore ha deciso di non pubblicare la guida su GitHub Pages**, e questo debito e' la ragione per cui.

La domanda e' stata posta subito dopo la chiusura di [DEBT-023], con la guida gia' corretta e pubblicabile. La scelta e' stata di restare al **file autoconsistente mandato a mano**, per l'argomento che questo debito porta: finche' e' un file che qualcuno manda, **chi lo manda sa quanto e' vecchio**; su un canale permanente e indicizzabile non lo sa nessuno.

E' registrato perche' e' la seconda volta che l'operatore sceglie di **aspettare invece di derogare** su questa pagina, e la prima volta ha pagato in modo misurabile: il rinvio della security review le ha fatto trovare tre `high` che il giorno prima non esistevano.

**L'innesco di questo debito resta quindi armato e non derogato.**
