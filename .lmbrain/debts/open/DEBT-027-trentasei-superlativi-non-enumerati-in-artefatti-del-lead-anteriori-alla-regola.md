---
id: DEBT-027
title: "Trentasei superlativi non enumerati in artefatti del Lead anteriori alla regola"
status: open
category: "documentation"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-03"
owner: "AGENT-LEAD"
origin_artifact: "REVIEW-025"
origin_ref: "RF-001"
related_specs: []
related_reviews: ["REVIEW-025","REVIEW-027"]
related_decisions: ["ADR-012"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-26
tags: ["documentation","verification-gap"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-027-EVENT-001"
    timestamp: "2026-08-26T10:02:08.381237500+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead su se stesso, il giorno in cui ha scritto lo strumento che lo ha contato.\n\nL'arretrato poteva restare invisibile scegliendo RULE_DATE e non guardando indietro. E' stato contato e stampato perche' una deroga silenziosa e' precisamente cio' che questo strumento cerca: la stessa notte SECURITY.md e' rimasto fuori dall'inventario di ADR-012 per tutta la vita di quello strumento senza che nulla lo dicesse, ed e' costato un finding medium e una classe di controllo nuova.\n\nMilestone M-03 e non M-02: nessuno dei falsi accertati blocca la devnet, e anticiparlo sottrarrebbe tempo a SPEC-019, che invece riguarda un fork."
    evidence_refs: []
---
# Trentasei superlativi non enumerati in artefatti del Lead anteriori alla regola

## Statement

sim/tools/lead_claims_check.py conta 36 superlativi assoluti non enumerati in artefatti scritti dal Lead e anteriori al 2026-08-26, distribuiti su ventidue file fra ADR, review, spec, debiti e handoff. Ciascuno e' un'affermazione universale - «l'unica», «il solo», «nessun altro» - priva della traccia di un conteggio che la sostenga. La regola che li vieta vincola in avanti e non retroattivamente, quindi questi non la violano; restano pero' affermazioni che il progetto sta facendo adesso, in documenti che vengono letti adesso.

## Evidence and provenance

Il conteggio e' stampato a ogni esecuzione dello strumento, sotto la voce «arretrato», e non e' quindi silenzioso.

Che non siano rumore e' accertato da tre casi su tre esaminati. Il superlativo di DEBT-014 - «l'unica preimmagine a dominio separato non legata a chain_id» - era falso: sei altre lo omettono, e per object_id e input_hash l'indipendenza dalla catena e' richiesta. Il superlativo su height in REVIEW-025 era falso: enumerando i dodici campi di BlockHeader, previous_block_id, state_root e validator_set_hash sono determinati esattamente quanto height. E «l'unica cosa che aspetta l'operatore» in HANDOFF-002 era falsa mentre veniva scritta, perche' ADR-016 era gia' in proposed.

Tre esaminati, tre falsi. Non e' un campione sufficiente a stimare i restanti trentatre, ed e' sufficiente a stabilire che non sono decorazione linguistica.

## Impact and scope boundary

Il danno non e' uniforme e va distinto, perche' il rimedio cambia.

Un superlativo falso in una ADR accettata o in un documento che una spec potrebbe citare e' contenuto normativo sbagliato che qualcuno erediteta': e' esattamente la strada per cui DEBT-014 ha portato avanti un'affermazione falsa per tre stesure, ciascuna scritta con cura da qualcuno che si fidava della precedente.

Un superlativo falso in una review chiusa o in un handoff consumato e' invece un errore storico: sbagliato, ma non piu' letto come istruzione. Vale meno e va sanato dopo.

Severita' medium e non high perche' nessuno dei tre falsi accertati sosteneva una regola di validita' o una difesa: erano affermazioni di contorno che rendevano un argomento piu' forte di quanto fosse. Ma il conteggio e' alto e la percentuale di falsi nel campione e' totale.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead su se stesso, il giorno in cui ha scritto lo strumento che lo ha contato.

L'arretrato poteva restare invisibile scegliendo RULE_DATE e non guardando indietro. E' stato contato e stampato perche' una deroga silenziosa e' precisamente cio' che questo strumento cerca: la stessa notte SECURITY.md e' rimasto fuori dall'inventario di ADR-012 per tutta la vita di quello strumento senza che nulla lo dicesse, ed e' costato un finding medium e una classe di controllo nuova.

Milestone M-03 e non M-02: nessuno dei falsi accertati blocca la devnet, e anticiparlo sottrarrebbe tempo a SPEC-019, che invece riguarda un fork.

## Resolution criteria

Passata sui trentasei, in ordine di ereditabilita' e non di numero: prima le ADR accettate e i documenti che una spec potrebbe citare, poi i debiti aperti, poi review e handoff chiusi.

Per ciascuno uno di tre esiti, tutti ammissibili: enumerato con la traccia scritta accanto; riformulato come congettura o come classe piu' stretta, che e' cio' che ha salvato l'eccezione di DEBT-014; oppure dichiarato falso con la correzione datata, senza cancellare l'originale.

Il conteggio stampato dallo strumento deve scendere a zero, e a quel punto RULE_DATE puo' essere rimossa e la regola vale su tutto l'albero. Finche' non scende, RULE_DATE non va rimossa: sarebbe la guardia disattivata perche' rossa, che e' il modo in cui una guardia muore.

Il rimedio apparente da non adottare: cancellare i superlativi. Toglierebbe l'affermazione senza stabilire se fosse vera, e un'affermazione tolta non insegna nulla a chi l'aveva creduta. Quella di DEBT-014 e' stata sostituita da una classe piu' stretta ed esatta, che vale piu' del silenzio.

## Resolution evidence

