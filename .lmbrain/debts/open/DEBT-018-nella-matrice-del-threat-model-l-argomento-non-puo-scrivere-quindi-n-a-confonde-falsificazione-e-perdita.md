---
id: DEBT-018
title: "Nella matrice del threat model l'argomento «non puo' scrivere, quindi n/a» confonde falsificazione e perdita"
status: open
category: "documentation"
severity: "medium"
origin_severity: null
area: "security"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "REVIEW-021"
origin_ref: "RF-005"
related_specs: ["SPEC-004","SPEC-013"]
related_reviews: ["REVIEW-021"]
related_decisions: ["ADR-015"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["threat-model","security"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-018-EVENT-001"
    timestamp: "2026-08-25T23:11:21.763527900+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-013 su segnalazione di AGENT-007, che lo indica come il piu utile dei tre residui da non perdere e precisa che e manutenzione del proprio documento e non della spec. Registrato come debito e non chiuso nella remediation per quella ragione: allargare SPEC-013 al threat model sarebbe stato scambiare il perimetro di una spec con quello di un documento di analisi. Owner AGENT-007 perche il documento e suo e perche la valutazione di A-04 richiede di leggere le regole di eleggibilita, che lei dichiara di non aver letto abbastanza a fondo per pronunciarsi."
    evidence_refs: []
---
# Nella matrice del threat model l'argomento «non puo' scrivere, quindi n/a» confonde falsificazione e perdita

## Statement

La matrice di threat-model.md marca alcune celle come n/a con l'argomento che l'attore non ha un percorso di scrittura verso l'asset. L'argomento confonde due cose diverse: non poter falsificare un asset e non poter causare una perdita su quell'asset. La cella A-02 per T-06 e caduta su questo, e AGENT-007 segnala che non e stata resa falsa da SPEC-013 ma era gia falsa da SPEC-004, perche T-06 comprende per definizione l'ISP o censore e TM-31 descrive l'isolamento di nodi: un nodo isolato non riceve i propri challenge_request e subisce emissione mancata senza alcun furto di chiave. TM-37 e quindi il secondo falsificatore e non il primo.

Tre conseguenze da trattare insieme. La cella A-04, set dei validatori, poggia sullo stesso argomento nella forma nessun potere sulla composizione, e chi isola dei candidati ne fa fallire le challenge e quindi ne altera l'eleggibilita. L'elenco asset di TM-31 omette A-02, quindi la cella corretta punta a TM-37 e la via piu semplice resta non tracciata. E TM-37 e collocato sotto un attore che non lo copre, perche T-06 e definito come osservazione passiva, peer enrollato che si connette a molti, oppure ISP o censore, e chi esfiltra una chiave privata da un dispositivo non e nessuna delle tre.

## Evidence and provenance

Segnalato da AGENT-007 nell'addendum di verifica mirata a REVIEW-021, dopo che la remediation di SPEC-013 aveva falsificato e corretto la cella A-02 per T-06. AGENT-007 dichiara di aver ripassato le sei celle n/a di quella colonna con lo stesso metro: A-01, A-09 e A-13 reggono, A-08 regge al limite, e A-04 e la sola che segnala. Dichiara esplicitamente di non affermare che A-04 sia falsa, non avendo letto le regole di eleggibilita abbastanza a fondo, ma che poggia sull'argomento appena caduto e va sottoposta allo stesso test.

Sull'attore mancante AGENT-007 nomina tre uscite senza imporne una: allargare esplicitamente T-06, dichiarare TM-37 trasversale, oppure aggiungere un attore per la compromissione dell'endpoint, che giudica la piu onesta e la piu cara.

## Impact and scope boundary

Nessun impatto sul protocollo: e manutenzione di un documento di analisi. L'impatto e sulla fiducia che si puo riporre nella matrice, che e lo strumento con cui il progetto decide dove guardare. Una cella n/a dice al lettore successivo di non cercare li, quindi una n/a sbagliata ha la stessa forma dell'impossibilita dichiarata a torto gia censita nella famiglia 2 di recurring-defects.md: e la forma peggiore, perche dice di smettere di cercare.

Il caso di A-02 lo dimostra: la cella e rimasta n/a dal 2026-08-25 fino a quando una spec di tutt'altro oggetto non l'ha incrociata per caso, e la via piu semplice per falsificarla era gia scritta nel documento due sezioni piu in la.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-013 su segnalazione di AGENT-007, che lo indica come il piu utile dei tre residui da non perdere e precisa che e manutenzione del proprio documento e non della spec. Registrato come debito e non chiuso nella remediation per quella ragione: allargare SPEC-013 al threat model sarebbe stato scambiare il perimetro di una spec con quello di un documento di analisi. Owner AGENT-007 perche il documento e suo e perche la valutazione di A-04 richiede di leggere le regole di eleggibilita, che lei dichiara di non aver letto abbastanza a fondo per pronunciarsi.

## Resolution criteria

La cella A-04 per T-06 e sottoposta allo stesso test e confermata o corretta, con la ragione scritta accanto in entrambi i casi. L'elenco asset di TM-31 e completato con A-02, cosi che la via piu semplice sia tracciata. E la collocazione di TM-37 e risolta scegliendo fra le tre uscite nominate, con la scelta motivata invece che presa per comodita.

Va inoltre stabilito se l'argomento non puo scrivere quindi n/a compaia altrove nella matrice fuori dalla colonna T-06, perche il difetto e nell'argomento e non nella colonna. Se ne compare, ogni occorrenza va risottoposta.

## Resolution evidence

