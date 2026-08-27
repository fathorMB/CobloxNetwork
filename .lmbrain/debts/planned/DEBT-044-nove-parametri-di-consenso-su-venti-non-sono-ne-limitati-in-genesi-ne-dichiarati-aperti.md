---
id: DEBT-044
title: "Nove parametri di consenso su venti non sono ne' limitati in genesi ne' dichiarati aperti"
status: planned
category: "security"
severity: "high"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-023","SPEC-022"]
related_reviews: ["REVIEW-043"]
related_decisions: ["ADR-017","ADR-010"]
target_specs: ["SPEC-027"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: ["consensus","governance","security"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: SPEC-027 nasce per chiudere esattamente questa classe: per ciascuno dei nove, limite di genesi con valore derivato oppure dichiarazione aperta con la ragione. E' il primo debito di questo progetto a ricevere una spec bersaglio: fino al 2026-08-27 la cartella `planned` era vuota e nessun debito era mai stato instradato, quindi i diciotto aperti si chiudevano solo come effetto collaterale di lavoro fatto per altro.\n\nLa spec porta anche la taratura che l'operatore aveva rimandato in attesa dell'analisi di SPEC-023, perche' e' la stessa decisione: fissare un limite e fissare un valore di lancio sono lo stesso atto su questi parametri."
debt_events:
  - schema_version: "1"
    id: "DEBT-044-EVENT-001"
    timestamp: "2026-08-27T10:46:20.994934900+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Creato come successore invece di correggere DEBT-036 a mano perche' i debiti sono artefatti governati e la supersessione e' l'unica mutazione che il contratto ammette per cambiarne la portata: e' mutua e atomica, e lascia leggibile su entrambi i lati che il conteggio e' cambiato perche' un parametro e' uscito dalla classe, non perche' qualcuno abbia riscritto un numero. Il titolo di DEBT-036 diceva \"dieci\" e sarebbe rimasto falso in ogni digest."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-044-EVENT-002"
    timestamp: "2026-08-27T15:04:39.619667700+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "SPEC-027 nasce per chiudere esattamente questa classe: per ciascuno dei nove, limite di genesi con valore derivato oppure dichiarazione aperta con la ragione. E' il primo debito di questo progetto a ricevere una spec bersaglio: fino al 2026-08-27 la cartella `planned` era vuota e nessun debito era mai stato instradato, quindi i diciotto aperti si chiudevano solo come effetto collaterale di lavoro fatto per altro.\n\nLa spec porta anche la taratura che l'operatore aveva rimandato in attesa dell'analisi di SPEC-023, perche' e' la stessa decisione: fissare un limite e fissare un valore di lancio sono lo stesso atto su questi parametri."
    evidence_refs: ["SPEC-027"]
---
# Nove parametri di consenso su venti non sono ne' limitati in genesi ne' dichiarati aperti

## Statement

Successore di DEBT-036, corretto sul conteggio. `ConsensusParametersBody` ha venti campi. I dieci di elezione hanno un limite di magnitudine preso dall'ancora di genesi e sono dichiarati aperti nella lista DRAFT. **Nove** degli altri dieci non hanno ne' l'uno ne' l'altra: `max_clock_drift_ms`, `max_envelope_validity_ms`, `max_transport_attestation_validity_ms`, `max_transport_attestation_future_skew_ms`, `replay_cache_entries_per_peer`, `replay_cache_entries_global`, `max_weak_subjectivity_age_ms`, `max_current_balance_age_ms`, `app_suspension_notice_epochs`. Il decimo, `min_revocation_effective_delay_blocks`, e' uscito dalla classe: ha ora un tetto di genesi ed e' imposto. Tutti e nove valgono `1` in albero, che e' il valore delle fixture, e il quorum sedente li firma.

## Evidence and provenance

Il conteggio di DEBT-036 era corretto quando fu aperto e non lo e' piu'. `min_revocation_effective_delay_blocks_max` e' oggi in `ElectionBounds` (`core/coblox-core/src/params.rs:38`) ed e' imposto in `check_magnitudes` (`params.rs:589-590`, regola "min_revocation_effective_delay_blocks <= min_revocation_effective_delay_blocks_max"). Verificato dal Lead il 2026-08-27 leggendo entrambi i siti. La causa del cambiamento e' tracciabile: ADR-017 parte 2 imponeva che i tre parametri della revoca entrassero nel blocco dei vincoli di genesi — "e' la parte che manca oggi" — e SPEC-022 l'ha attuato. Rilevato da AGENT-007 in REVIEW-043 RF-004, che ne trae la conseguenza sulla riga 10 dell'analisi: l'affermazione "mantiene all'infinito" e' falsa.

## Impact and scope boundary

Identico a quello di DEBT-036 su nove parametri invece di dieci. Sono la meta' operativa e di sicurezza: orologi, finestre di validita', cache anti-replay, freschezza dell'ancora di fiducia. Nessun documento di genesi ne fissa uno e nessun documento li dichiara aperti, quindi non sono ne' decisi ne' registrati come da decidere, e il quorum sedente li firma senza che alcun limite lo trattenga. La correzione del conteggio riduce la classe di uno e non la natura del difetto.

## Decision log

Created by AGENT-LEAD: Creato come successore invece di correggere DEBT-036 a mano perche' i debiti sono artefatti governati e la supersessione e' l'unica mutazione che il contratto ammette per cambiarne la portata: e' mutua e atomica, e lascia leggibile su entrambi i lati che il conteggio e' cambiato perche' un parametro e' uscito dalla classe, non perche' qualcuno abbia riscritto un numero. Il titolo di DEBT-036 diceva "dieci" e sarebbe rimasto falso in ogni digest.

## Resolution criteria

Ciascuno dei nove ha un limite di magnitudine ancorato in genesi, oppure e' dichiarato aperto nella lista DRAFT dei parametri di lancio con la ragione per cui resta aperto. La gate che chiude la classe — `consensus_parameters_closure.py` — resta verde sul nuovo stato e la sua prova in negativo continua a cogliere sia il campo di schema fuori da entrambe le liste sia la voce di lista senza campo.

## Resolution evidence

