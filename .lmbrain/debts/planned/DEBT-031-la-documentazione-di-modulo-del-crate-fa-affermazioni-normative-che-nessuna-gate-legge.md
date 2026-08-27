---
id: DEBT-031
title: "La documentazione di modulo del crate fa affermazioni normative che nessuna gate legge"
status: planned
category: "verification"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-03"
owner: "AGENT-001"
origin_artifact: "REVIEW-030"
origin_ref: "RF-001"
related_specs: ["SPEC-016","SPEC-021"]
related_reviews: ["REVIEW-030"]
related_decisions: ["ADR-012"]
target_specs: ["SPEC-026"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-27
tags: ["verification-gap","documentation","rust"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: L'operatore ha adottato il 2026-08-27 il criterio che questo debito aveva gia' derivato da se': un'affermazione che **enumera** e' una lista dichiarata, ed e' la forma che si rompe in silenzio. Sorvegliare quelle, e non tutte le affermazioni di `src/`, e' cio' che impedisce alla gate di trasformare ogni refactor in una passata di ADR-012 e di essere quindi disattivata.\n\nInstradato su SPEC-026 perche' e' la spec dei controlli su discipline che nessuno strumento vede, stesso agente e stessa famiglia di `sim/tools/`. La spec diventa la terza guardia di quel gruppo e passa da `terra` a `sol`: e' il costo dichiarato dell'alternativa scelta, contro una spec propria che avrebbe lasciato due specifiche vicine da tenere allineate.\n\nPortati dentro i criteri i due avvertimenti del debito, perche' sono la parte che si perde piu' facilmente: il controllo deve verificare **nel verso giusto** — l'insieme dichiarato contro quello osservabile dal codice, non il testo pinnato — e la sua classe di scoperta deve essere **propria** e non un'estensione di quelle esistenti, con la trascrizione che dichiara quanti falsi positivi produrrebbe l'alternativa scartata."
debt_events:
  - schema_version: "1"
    id: "DEBT-031-EVENT-001"
    timestamp: "2026-08-26T11:59:20.395795500+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead su osservazione di AGENT-002, che l'ha fatta **dopo** aver chiuso il finding e non per giustificarne l'esistenza: ha notato che il conteggio delle probe non era salito, si e' chiesta perche', e ha stabilito che non poteva salire.\n\nVale la pena registrare la ricorrenza invece del singolo fatto. Tre volte in una sessione la stessa forma - una gate che misura un insieme dichiarato invece di uno osservato - e ogni volta il membro mancante era **l'ultimo arrivato**: `SECURITY.md` mai aggiunto all'inventario, i documenti di pretese confrontati con una costante Python, `CadenceBand` non aggiunta all'elenco dei portatori. La forma non e' «qualcuno ha dimenticato»: e' che **un insieme dichiarato non ha modo di accorgersi di un nuovo membro**, e chi lo aggiunge non ha modo di sapere che esiste una lista da aggiornare.\n\nMilestone M-03 perche' nessuno dei tre casi ha prodotto un difetto di comportamento, e anticiparlo sottrarrebbe tempo a [SPEC-019], che riguarda un fork."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-031-EVENT-002"
    timestamp: "2026-08-27T15:34:52.052699600+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "L'operatore ha adottato il 2026-08-27 il criterio che questo debito aveva gia' derivato da se': un'affermazione che **enumera** e' una lista dichiarata, ed e' la forma che si rompe in silenzio. Sorvegliare quelle, e non tutte le affermazioni di `src/`, e' cio' che impedisce alla gate di trasformare ogni refactor in una passata di ADR-012 e di essere quindi disattivata.\n\nInstradato su SPEC-026 perche' e' la spec dei controlli su discipline che nessuno strumento vede, stesso agente e stessa famiglia di `sim/tools/`. La spec diventa la terza guardia di quel gruppo e passa da `terra` a `sol`: e' il costo dichiarato dell'alternativa scelta, contro una spec propria che avrebbe lasciato due specifiche vicine da tenere allineate.\n\nPortati dentro i criteri i due avvertimenti del debito, perche' sono la parte che si perde piu' facilmente: il controllo deve verificare **nel verso giusto** — l'insieme dichiarato contro quello osservabile dal codice, non il testo pinnato — e la sua classe di scoperta deve essere **propria** e non un'estensione di quelle esistenti, con la trascrizione che dichiara quanti falsi positivi produrrebbe l'alternativa scartata."
    evidence_refs: ["SPEC-026"]
---
# La documentazione di modulo del crate fa affermazioni normative che nessuna gate legge

## Statement

La passata di [ADR-012] legge `docs/protocol/`, `sim/tools/`, `core/coblox-core/tests/`, la guida pubblica, `recurring-defects.md` e `SECURITY.md`. **Non legge `core/coblox-core/src/`.**

Ma i commenti di modulo di `src/` fanno affermazioni della stessa natura di quelle che la gate protegge altrove: `lib.rs` dichiara che *«Parameters are validated configuration, never compiled constants. No launch value appears in this crate»* e poi **enumera** i portatori. Sono affermazioni normative sul comportamento del sistema, pubblicate — il repository e' pubblico e `cargo doc` le rende — e **nessuna probe le tiene**.

## Evidence and provenance

Trovato da AGENT-002 alla fine della remediation di [REVIEW-030] RF-001, e riverificato dal Lead: lo scopo dei documenti di `published_artifacts.py` e' `docs/protocol/{name}`, e `src/` non compare in alcuna classe.

**La prova che il punto cieco costa e' il finding stesso.** `lib.rs:52-56` enumerava cinque portatori di configurazione; `CadenceBand`, introdotta da [SPEC-016], era il sesto e non c'era. La lista e' rimasta incompleta da quando `CadenceBand` esiste, e nessuna gate poteva vederlo: la remediation di [SPEC-021] non ha fatto salire il conteggio delle probe **perche' non poteva**.

**E' la terza volta nella stessa sessione che la gate misura l'insieme piu' piccolo**, dopo `SECURITY.md` fuori dall'inventario ([REVIEW-025] RF-001) e le due liste senza lato disco in `published_artifacts.py` ([REVIEW-027] RF-005).

## Impact and scope boundary

Il danno e' quello di ogni artefatto pubblicato non sorvegliato, e su `src/` ha una forma propria: **la documentazione di modulo e' cio' che il prossimo implementatore legge per sapere dove mettere una cosa nuova.** Una lista incompleta li' non produce un digest sbagliato, produce un membro che non viene aggiunto — e nessuno lo scopre finche' qualcuno non enumera a mano.

E' inoltre il luogo in cui una **clausola falsa** e' piu' difficile da vedere che in un documento di protocollo, perche' non c'e' una fixture accanto che la smentisca. Il paragrafo che questa remediation ha corretto affermava che quei valori arrivano dentro un documento firmato da un quorum: per `CadenceBand` e' falso e deliberatamente, e nulla lo segnalava.

`medium` e non `high` perche' nessuna regola di validita' e' scritta li' e nessun valore pubblicato ne dipende; `medium` e non `low` perche' la superficie cresce a ogni spec che tocca il core, e perche' e' gia' costata un finding.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead su osservazione di AGENT-002, che l'ha fatta **dopo** aver chiuso il finding e non per giustificarne l'esistenza: ha notato che il conteggio delle probe non era salito, si e' chiesta perche', e ha stabilito che non poteva salire.

Vale la pena registrare la ricorrenza invece del singolo fatto. Tre volte in una sessione la stessa forma - una gate che misura un insieme dichiarato invece di uno osservato - e ogni volta il membro mancante era **l'ultimo arrivato**: `SECURITY.md` mai aggiunto all'inventario, i documenti di pretese confrontati con una costante Python, `CadenceBand` non aggiunta all'elenco dei portatori. La forma non e' «qualcuno ha dimenticato»: e' che **un insieme dichiarato non ha modo di accorgersi di un nuovo membro**, e chi lo aggiunge non ha modo di sapere che esiste una lista da aggiornare.

Milestone M-03 perche' nessuno dei tre casi ha prodotto un difetto di comportamento, e anticiparlo sottrarrebbe tempo a [SPEC-019], che riguarda un fork.

## Resolution criteria

Stabilire **quali** affermazioni di `src/` sono normative e quali sono commento di implementazione, perche' sorvegliarle tutte trasformerebbe ogni refactor in una passata di [ADR-012] e la gate verrebbe disattivata.

Il criterio da cui partire, che questa sessione ha gia' prodotto due volte: un'affermazione che **enumera** — «i portatori sono questi cinque», «i chiamanti sono questi due» — e' una lista dichiarata, ed e' la forma che si rompe in silenzio. Quelle vanno tenute, e vanno tenute **nel verso giusto**: non pinnando il testo, ma verificando che l'insieme dichiarato coincida con quello osservabile dal codice, come `C6-ORPHAN` e `C11-CLAIMDOC` gia' fanno altrove.

**Il rimedio apparente da non adottare:** aggiungere `src/` alle classi di scoperta esistenti. Produrrebbe falsi positivi il primo giorno — `src/` e' pieno di identificatori che le regex di fixture riconoscono — ed e' esattamente il modo in cui `SECURITY.md` ha dovuto avere una classe propria invece di entrare fra i documenti. **La classe giusta e' probabilmente una terza**, e stabilirlo e' il lavoro.

Da chiudere prima che il crate cresca oltre la dimensione in cui una enumerazione a mano e' ancora possibile.

## Resolution evidence

