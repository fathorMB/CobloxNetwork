---
id: DEBT-015
title: "I sotto-controlli della reward policy sono pubblici e invocabili al posto della validazione"
status: open
category: "design"
severity: "low"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: "SPEC-011"
origin_ref: "OSS-003 dell'evidenza di remediation"
related_specs: ["SPEC-011"]
related_reviews: ["REVIEW-017"]
related_decisions: ["ADR-010"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["rust","api","conformance"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-015-EVENT-001"
    timestamp: "2026-08-25T20:54:34.017093+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-011, su segnalazione dell'implementatore che ha scelto di riportarlo invece di chiuderlo. Registrato come debito e non come remediation perche l'argomento sul raggruppamento dei cambiamenti breaking e corretto, e perche chiuderlo nella stessa passata avrebbe esteso una spec gia in review per la seconda volta. Owner AGENT-001 perche e l'autore dell'API e conosce i chiamanti nei test."
    evidence_refs: []
---
# I sotto-controlli della reward policy sono pubblici e invocabili al posto della validazione

## Statement

RewardPolicy::check_internal e RewardPolicy::check_magnitudes sono pubblici, mentre i gemelli del lato consenso su ConsensusParameters sono privati. Un chiamante puo quindi invocare un solo sotto-controllo e credere di aver validato il documento, ottenendo un Ok che non significa cio che sembra. La disciplina che il progetto ha stabilito e che l'unico modo di ottenere un tipo validato sia passare per la validazione completa, ed e incarnata da ValidatedConsensusParameters e ora da ValidatedRewardPolicy, entrambi privi di costruttore alternativo; la visibilita dei sotto-controlli e una porta laterale attorno a quella disciplina.

## Evidence and provenance

Segnalato da AGENT-001 al termine della remediation di REVIEW-017 come OSS-003, con la raccomandazione esplicita di non chiuderlo in quella sede. L'argomento e stato accettato dal Lead nel merito e non per convenienza: renderli privati e un secondo cambiamento breaking dell'API pubblica del core, dopo quello gia dichiarato nella stessa remediation per lo spostamento di publisher_reward_within_cap su ValidatedRewardPolicy, e tocca test che oggi li chiamano direttamente. Non era condizione di chiusura di alcun finding di REVIEW-017.

L'asimmetria e verificabile confrontando le due meta di params.rs: i sotto-controlli del lato consenso non sono raggiungibili dall'esterno, quelli del lato reward si.

Registrato con origine sulla spec e non sulla review perche il contratto richiede a un debito di origine review di citare un finding numerato, e OSS-003 e un'osservazione dell'implementatore e non un finding del reviewer. La distinzione e corretta e va conservata.

## Impact and scope boundary

Nessun impatto sulla correttezza del percorso di accettazione oggi in uso: light_client::authenticate_reward_policy compone la validazione completa, e il presidio aggiunto dentro check_against_active rifiuta rapporto degenere e gap nullo su ogni percorso. Il danno e potenziale e riguarda chi implementera i chiamanti futuri: un Ok da un sotto-controllo si legge come una validazione, e la classe di difetto che ne discende e la stessa che SPEC-011 esiste per chiudere, cioe una regola scritta che nessuno applica sul percorso reale.

Severita low e non medium perche richiede che qualcuno scriva un chiamante nuovo usando l'API sbagliata, e perche l'API corretta ora esiste ed e nominata; sarebbe medium se il percorso di accettazione stesso vi si appoggiasse, e non lo fa.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-011, su segnalazione dell'implementatore che ha scelto di riportarlo invece di chiuderlo. Registrato come debito e non come remediation perche l'argomento sul raggruppamento dei cambiamenti breaking e corretto, e perche chiuderlo nella stessa passata avrebbe esteso una spec gia in review per la seconda volta. Owner AGENT-001 perche e l'autore dell'API e conosce i chiamanti nei test.

## Resolution criteria

I sotto-controlli della reward policy tornano privati come i gemelli del lato consenso, con i test che oggi li chiamano direttamente riscritti sull'API pubblica, oppure la loro visibilita e motivata per iscritto sul tipo con la ragione per cui l'asimmetria e voluta. Il cambiamento e breaking e va raggruppato con altri della stessa natura invece di essere fatto da solo; l'occasione naturale e la prossima spec che tocchi l'API pubblica di coblox-core.

## Resolution evidence

