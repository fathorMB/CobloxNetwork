---
id: DEBT-022
title: "L'autorizzazione del burn di abbonamento non richiede che la chiave sia non revocata"
status: open
category: "security"
severity: "high"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "SPEC-015"
origin_ref: "F4 dell'evidenza di AGENT-006"
related_specs: ["SPEC-001","SPEC-015"]
related_reviews: ["REVIEW-024"]
related_decisions: ["ADR-006"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-26
tags: ["ledger","security","identity"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-022-EVENT-001"
    timestamp: "2026-08-26T00:26:52.057323400+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-015, su segnalazione di AGENT-006 che ha riportato invece di scrivere attorno al buco, come il dispatch le imponeva. Owner AGENT-007 e non AGENT-006 ne il Lead: e una questione di sicurezza sulla spesa e va valutata da chi ha mandato di attaccarla, ed e la stessa regola applicata a DEBT-013, DEBT-014 e DEBT-017."
    evidence_refs: []
---
# L'autorizzazione del burn di abbonamento non richiede che la chiave sia non revocata

## Statement

La regola di autorizzazione del burn di abbonamento in docs/protocol/ledger.md riga 347 dice soltanto che la chiave MUST derive payer_node_id. Le tre regole sorelle dello stesso documento dicono tutte the enrolled, unrevoked: riga 312 per fund_app, riga 398 per challenge_commitment, riga 871 per validator_candidacy. L'autorizzazione dell'abbonamento e l'unica priva di quella qualificazione.

Ne segue che una chiave revocata, tipicamente revocata perche compromessa, puo apparentemente ancora autorizzare addebiti sul saldo del nodo. La revoca esiste per fermare esattamente questo, e per il burn di abbonamento non lo dice.

## Evidence and provenance

Trovato da AGENT-006 durante SPEC-015, provando a scrivere con precisione cosa succede quando una chiave viene revocata. E la forma in cui questo progetto ha trovato meta dei propri difetti: qualcuno prova a dire una cosa con precisione e scopre che non e scritta da nessuna parte. Di conseguenza la guida pubblica non afferma che la revoca fermi la spesa.

Verificato dal Lead prima di promuoverlo a debito, e la verifica era necessaria: la riga 312 nomina anch'essa payer_node_id e sembrerebbe una regola generale che copre anche il burn. Non lo e. Appartiene a FundAppAuthorization, che e una transazione diversa con una propria struttura di autorizzazione; il burn di abbonamento ha la propria, SubscriptionBurnAuthorization, dichiarata alla riga 338, e la propria regola alla 347.

Non e stato stabilito se l'omissione sia deliberata. Il Lead non vede una ragione per cui un abbonamento dovrebbe accettare una chiave revocata mentre il finanziamento di un'app non lo fa, ma non ha svolto istruttoria e non lo afferma.

## Impact and scope boundary

Da stabilire, ed e il lavoro. La superficie e la spesa dal saldo di un nodo la cui chiave e stata revocata: se l'omissione e reale e non coperta altrove, chi ha rubato una chiave puo continuare a svuotare il saldo dopo che il legittimo proprietario ha ottenuto la revoca, che e il momento in cui il protocollo dovrebbe averlo fermato.

Va valutato separatamente cosa accada agli abbonamenti gia attivi al momento della revoca, perche e una questione diversa dall'aprirne di nuovi, e cosa comporti per la ricompensa al creatore, che conta gli abbonati attivi e li deriva dai burn finalizzati.

Severita high e non critical perche richiede una chiave gia compromessa, quindi un fallimento a monte, e perche nessuna rete esiste; ma e una regola di validita mancante su un percorso di spesa, ed e la classe piu economica da correggere adesso e piu cara dopo.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-015, su segnalazione di AGENT-006 che ha riportato invece di scrivere attorno al buco, come il dispatch le imponeva. Owner AGENT-007 e non AGENT-006 ne il Lead: e una questione di sicurezza sulla spesa e va valutata da chi ha mandato di attaccarla, ed e la stessa regola applicata a DEBT-013, DEBT-014 e DEBT-017.

## Resolution criteria

Una valutazione che stabilisca se l'omissione sia deliberata o accidentale, pronunciandosi separatamente sull'apertura di nuovi abbonamenti e sugli abbonamenti gia attivi al momento della revoca. Gli esiti ammissibili sono due: allineare la regola alle tre sorelle con la qualificazione enrolled, unrevoked, che e una modifica a una regola di validita e fa quindi scattare la gate di ADR-012 con la sua passata; oppure il rifiuto motivato, con la ragione dell'asimmetria scritta accanto alla regola invece che lasciata implicita, perche un'eccezione non scritta si legge come una dimenticanza.

Va inoltre stabilito nella stessa occasione se altre regole di autorizzazione del protocollo omettano la stessa qualificazione, perche il difetto e nell'asimmetria e non nella singola riga.

Da chiudere prima che una devnet accumuli saldi reali.

## Resolution evidence

