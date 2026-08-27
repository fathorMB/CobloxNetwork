---
id: DEBT-035
title: "Dentro la classe 0 l'ordine e per ID di transazione, e il revocante puo macinare il proprio ID"
status: planned
category: "security"
severity: "medium"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "REVIEW-036"
origin_ref: "RF-006"
related_specs: ["SPEC-019"]
related_reviews: ["REVIEW-036"]
related_decisions: ["ADR-017"]
target_specs: ["SPEC-022"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-27
tags: ["security","consensus","ledger"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: Il debito e' gia' affrontato dalla consegna di SPEC-022 e chiude con la sua accettazione, non prima. REVIEW-042 aveva accertato che non e' sfruttabile perche' il predicato e' insensibile all'ordine intra-blocco — la terza opzione, che la spec non enumerava — e la remediation del 2026-08-27 ha scritto quella conseguenza dove serve: `core/coblox-core/src/authorization.rs:31` dichiara che DEBT-035 non e' sfruttabile attraverso questa regola, e la riscrittura di RF-002 ha portato la granularita' di altezza nel testo normativo come ragione, non come coincidenza.\n\nNon lo risolvo adesso perche' SPEC-022 e' in `review` con REVIEW-044 in changes-requested: chiudere un debito sulla base di lavoro non ancora accettato sarebbe la pretesa che corre avanti alla regola."
debt_events:
  - schema_version: "1"
    id: "DEBT-035-EVENT-001"
    timestamp: "2026-08-26T22:16:20.092951300+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead su finding di AGENT-007 in [REVIEW-036], perche' non ha casa dentro [ADR-017] e rinviarlo alla spec ripeterebbe l'errore che lo ha prodotto.\n\nLa prima stesura di [ADR-017] dichiarava aperta la scelta fra mordere a `h` o a `h+1` e diceva che l'implementatore doveva guardarla con il codice sotto mano. Quella questione era gia' chiusa da `ledger.md:2819`, e dichiararla aperta ha distolto lo sguardo dal punto in cui l'ordinamento e' davvero indeterminato — dentro la classe 0.\n\nVale la pena registrare la forma: **una questione dichiarata aperta a torto costa quanto una dichiarata chiusa a torto**, perche' manda chi legge a cercare dove non serve. E' la variante attenuata dell'impossibilita' dichiarata a torto che `recurring-defects.md` classifica come la peggiore della famiglia 2."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-035-EVENT-002"
    timestamp: "2026-08-27T15:01:46.945552200+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Il debito e' gia' affrontato dalla consegna di SPEC-022 e chiude con la sua accettazione, non prima. REVIEW-042 aveva accertato che non e' sfruttabile perche' il predicato e' insensibile all'ordine intra-blocco — la terza opzione, che la spec non enumerava — e la remediation del 2026-08-27 ha scritto quella conseguenza dove serve: `core/coblox-core/src/authorization.rs:31` dichiara che DEBT-035 non e' sfruttabile attraverso questa regola, e la riscrittura di RF-002 ha portato la granularita' di altezza nel testo normativo come ragione, non come coincidenza.\n\nNon lo risolvo adesso perche' SPEC-022 e' in `review` con REVIEW-044 in changes-requested: chiudere un debito sulla base di lavoro non ancora accettato sarebbe la pretesa che corre avanti alla regola."
    evidence_refs: ["SPEC-022"]
---
# Dentro la classe 0 l'ordine e per ID di transazione, e il revocante puo macinare il proprio ID

## Statement

`ledger.md:2821-2824` ordina le transazioni di **classe 0** — `challenge_commitment`, `challenge_evidence`, `revoke_identity` e `validator_candidacy` — per **raw transaction ID**. L'ID e' l'hash del corpo, e il corpo di una `revoke_identity` porta `created_at_ms` ed `expires_at_ms`.

Un revocante puo' quindi **enumerare millisecondi** finche' il proprio ID ordina prima o dopo quello di una `validator_candidacy` bersaglio che finisce nello stesso blocco, e scegliere cosi' se la revoca morda quella candidatura. Due delle autorizzazioni qualificate — `ChallengeCommitmentAuthorization` e `ValidatorCandidacyAuthorization` — sono in classe 0 insieme alla revoca.

L'esito e' **deterministico per ogni verificatore, quindi non e' un fork**. Ma e' una scelta del revocante, cioe' la stessa discrezione che [ADR-017] parte 1 dichiara di aver tolto, riapparsa come **indice nel blocco** invece che come valore in un campo.

## Evidence and provenance

Trovato da AGENT-007 in [REVIEW-036] RF-006, attaccando il dettaglio che la prima stesura di [ADR-017] rinviava alla spec.

`ledger.md:2819` §"State transition order", riverificato dal Lead leggendo le righe 2816-2828: *«(0) `challenge_commitment`, `challenge_evidence`, `revoke_identity`, and `validator_candidacy`, ordered by raw transaction ID; (1) `fund_app` and `burn`, ordered by `(account_kind, raw_account_key, debit_nonce, raw_tx_id)`»*.

La macinabilita' dell'ID poggia sulla stessa leva che `ledger.md:740` gia' quantifica per un altro percorso: da 10^3 a 10^6 valori legali di timestamp, una SHA-256 ciascuno.

**Non verificato contro codice**, perche' il codice non esiste: `core/coblox-core/src/` non contiene ne' esecutore di transazioni ne' costruttore. AGENT-007 lo dichiara esplicitamente nel proprio perimetro.

## Impact and scope boundary

La superficie e' l'eleggibilita' a validatore, non il saldo: `burn` e `fund_app` sono **classe 1** e stanno sempre dopo ogni transazione di classe 0, quindi il percorso della spesa non e' toccato e la parte 1 di [ADR-017] resta valida li' dove e' stata attaccata.

Il danno concreto e' che chi propone una revoca sceglie, macinando timestamp, se quella revoca morda le transazioni di classe 0 **dello stesso blocco** — fra cui la candidatura del soggetto. Non serve un quorum ostile per intero: serve chi compone il corpo della transazione.

`medium` e non `high` per tre ragioni dichiarate: l'esito e' deterministico e quindi non produce disaccordo fra verificatori; la finestra e' un blocco solo; e la leva richiede che revoca e candidatura finiscano nello stesso blocco, che non e' sotto il controllo di chi macina.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead su finding di AGENT-007 in [REVIEW-036], perche' non ha casa dentro [ADR-017] e rinviarlo alla spec ripeterebbe l'errore che lo ha prodotto.

La prima stesura di [ADR-017] dichiarava aperta la scelta fra mordere a `h` o a `h+1` e diceva che l'implementatore doveva guardarla con il codice sotto mano. Quella questione era gia' chiusa da `ledger.md:2819`, e dichiararla aperta ha distolto lo sguardo dal punto in cui l'ordinamento e' davvero indeterminato — dentro la classe 0.

Vale la pena registrare la forma: **una questione dichiarata aperta a torto costa quanto una dichiarata chiusa a torto**, perche' manda chi legge a cercare dove non serve. E' la variante attenuata dell'impossibilita' dichiarata a torto che `recurring-defects.md` classifica come la peggiore della famiglia 2.

## Resolution criteria

Due vie, e la prima e' la piu' economica perche' non tocca l'ordinamento:

1. **Dichiarare che il predicato di qualificazione si valuta contro lo stato pre-blocco** (i soli antenati) per le transazioni di classe 0. L'ordinamento resta quello che e', e la macinatura non compra nulla perche' nessuna transazione di classe 0 vede le altre.
2. Ordinare la classe 0 **per tipo prima che per ID**, mettendo `revoke_identity` davanti alle altre tre. Cambia l'ordinamento canonico, quindi tocca la serializzazione e i digest pubblicati.

In entrambi i casi la chiusura richiede una **fixture di conformita'** con una `revoke_identity` e una `validator_candidacy` nello stesso blocco, provata in **entrambi gli ordinamenti di ID**, che dia lo stesso esito. Senza quella fixture la regola sarebbe scritta e non esercitata, che e' la famiglia 1.

Da chiudere **insieme alla spec che attua [ADR-017]**, perche' e' quella spec a rendere la questione rilevante: finche' la revoca morde solo a `effective_height`, l'ordinamento intra-blocco non decide nulla.

## Resolution evidence

