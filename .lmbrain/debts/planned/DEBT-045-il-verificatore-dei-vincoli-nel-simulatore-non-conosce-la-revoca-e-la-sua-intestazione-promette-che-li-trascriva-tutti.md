---
id: DEBT-045
title: "Il verificatore dei vincoli nel simulatore non conosce la revoca, e la sua intestazione promette che li trascriva tutti"
status: planned
category: "correctness"
severity: "medium"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-022","SPEC-007"]
related_reviews: ["REVIEW-042"]
related_decisions: ["ADR-017","ADR-010"]
target_specs: ["SPEC-027"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: ["consensus","conformance","simulation"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: Instradato su SPEC-027 e non lasciato senza casa, e l'argomento non e' la comodita': **SPEC-027 aggiunge limiti al blocco dei vincoli**, quindi lasciare il simulatore fuori lo farebbe divergere di piu' di quanto gia' diverge. Il debito nasce cosi' — la prima consegna di SPEC-022 porto' i tre parametri della revoca nel crate e nei documenti e non nel simulatore — e la sua remediation lo ha allargato aggiungendo un pavimento di genesi e portando l'elenco del rapporto di variazione da dieci a tredici nomi. Ogni modifica futura a quel blocco lo allarga ancora finche' resta aperto.\n\nChiuderlo dentro SPEC-027 costa una passata sullo stesso blocco che si sta gia' cambiando; chiuderlo dopo costa rifare il confronto da capo su uno stato diverso.\n\nAggiunto ai criteri anche cio' che il debito chiede e che si perde piu' facilmente: una prova in negativo che mostri il **simulatore rifiutare** un documento che il crate rifiuta. Allineare due implementazioni a vista non dimostra che concordino; il progetto ha gia' pagato questa distinzione con DEBT-012, rimasto invisibile finche' non e' esistita una seconda derivazione."
debt_events:
  - schema_version: "1"
    id: "DEBT-045-EVENT-001"
    timestamp: "2026-08-27T11:19:50.919268500+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead perche' la scoperta e' fuori dal perimetro della spec che l'ha prodotta e AGENT-002 ha correttamente riportato invece di correggere, seguendo il precedente di DEBT-041 nella stessa sessione. E' un debito e non un rilievo locale perche' attraversa piu' consegne: nasce dalla prima SPEC-022, si allarga con la sua remediation, e ogni futura modifica al blocco dei vincoli lo allarghera' ancora finche' resta aperto. Appartiene alla famiglia gia' censita in knowledge/derivazioni-non-univoche.md."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-045-EVENT-002"
    timestamp: "2026-08-27T15:38:27.668924100+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Instradato su SPEC-027 e non lasciato senza casa, e l'argomento non e' la comodita': **SPEC-027 aggiunge limiti al blocco dei vincoli**, quindi lasciare il simulatore fuori lo farebbe divergere di piu' di quanto gia' diverge. Il debito nasce cosi' — la prima consegna di SPEC-022 porto' i tre parametri della revoca nel crate e nei documenti e non nel simulatore — e la sua remediation lo ha allargato aggiungendo un pavimento di genesi e portando l'elenco del rapporto di variazione da dieci a tredici nomi. Ogni modifica futura a quel blocco lo allarga ancora finche' resta aperto.\n\nChiuderlo dentro SPEC-027 costa una passata sullo stesso blocco che si sta gia' cambiando; chiuderlo dopo costa rifare il confronto da capo su uno stato diverso.\n\nAggiunto ai criteri anche cio' che il debito chiede e che si perde piu' facilmente: una prova in negativo che mostri il **simulatore rifiutare** un documento che il crate rifiuta. Allineare due implementazioni a vista non dimostra che concordino; il progetto ha gia' pagato questa distinzione con DEBT-012, rimasto invisibile finche' non e' esistita una seconda derivazione."
    evidence_refs: ["SPEC-027"]
---
# Il verificatore dei vincoli nel simulatore non conosce la revoca, e la sua intestazione promette che li trascriva tutti

## Statement

`sim/coblox_sim/params.py` e' una seconda implementazione del blocco dei vincoli di `docs/protocol/ledger.md`, ed e' muta sulla revoca. Il suo `ConsensusParameters` non porta `min_revocation_effective_delay_blocks`, `revocation_effective_grace_blocks` ne' `max_planned_revocation_delay_blocks`; il suo `ElectionBounds` non porta i tre tetti corrispondenti ne' `revocation_effective_grace_blocks_min`; il suo elenco dei parametri soggetti al rapporto di variazione resta a dieci nomi mentre `ledger.md` ora ne dichiara tredici. La stringa `revocation` non compare nel file: zero occorrenze.

## Evidence and provenance

Verificato dal Lead il 2026-08-27: `grep -c revocation sim/coblox_sim/params.py` restituisce 0. L'intestazione del file, righe 13-14, promette il contrario: "every rule carries the exact text it transcribes, so a reviewer can diff the two side by side". Un revisore che facesse quel diff fianco a fianco troverebbe le regole della revoca in `ledger.md` e non nel file che dichiara di trascriverle, senza che nulla segnali l'assenza. Trovato da AGENT-002 durante la remediation di SPEC-022 su REVIEW-042 e riportato invece di essere corretto, perche' fuori dal perimetro dichiarato di quella spec.

## Impact and scope boundary

E' una derivazione non univoca fra due implementazioni dello stesso blocco di vincoli: `coblox-core` rifiuta documenti che il simulatore accetta. Il difetto precede questa remediation — nasce con la consegna originale di SPEC-022, che porto' i tre parametri nel crate e nei documenti e non nel simulatore — e la remediation lo ha allargato, perche' ha aggiunto un pavimento di genesi e portato l'elenco del rapporto di variazione da dieci a tredici nomi. Il rischio non e' di consenso, perche' il simulatore non valida catene: e' che una taratura fatta col simulatore possa proporre parametri che il crate rifiuta, e che la promessa di trascrizione fedele nell'intestazione renda l'assenza invisibile invece che evidente.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead perche' la scoperta e' fuori dal perimetro della spec che l'ha prodotta e AGENT-002 ha correttamente riportato invece di correggere, seguendo il precedente di DEBT-041 nella stessa sessione. E' un debito e non un rilievo locale perche' attraversa piu' consegne: nasce dalla prima SPEC-022, si allarga con la sua remediation, e ogni futura modifica al blocco dei vincoli lo allarghera' ancora finche' resta aperto. Appartiene alla famiglia gia' censita in knowledge/derivazioni-non-univoche.md.

## Resolution criteria

`sim/coblox_sim/params.py` porta i tre parametri della revoca, i loro tetti di genesi, `revocation_effective_grace_blocks_min` con la relazione verso `validator_min_set_size_min`, e l'elenco del rapporto di variazione allineato a tredici nomi — ciascuno con il testo esatto che trascrive, come la sua intestazione promette. In alternativa: l'intestazione dichiara per iscritto quale sottoinsieme del blocco il file copre, cosi' che la promessa smetta di essere piu' ampia del contenuto. Una prova in negativo mostra il simulatore rifiutare un documento che il crate rifiuta.

## Resolution evidence

