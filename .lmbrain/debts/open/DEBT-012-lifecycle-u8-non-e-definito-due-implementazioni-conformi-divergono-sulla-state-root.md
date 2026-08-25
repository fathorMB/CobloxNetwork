---
id: DEBT-012
title: "lifecycle_u8 non e definito: due implementazioni conformi divergono sulla state_root"
status: open
category: "correctness"
severity: "high"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-001","SPEC-008"]
related_reviews: ["REVIEW-012"]
related_decisions: []
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["ledger","conformance","interoperability"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-012-EVENT-001"
    timestamp: "2026-08-25T16:07:53.453728400+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Registrato come debito e non come remediation di SPEC-008 perche la correzione sta in docs/protocol/, che quella spec dichiarava sola lettura, e perche tocca una preimmagine gia pubblicata: e lavoro di specifica con una fixture da aggiungere, non una modifica al crate. Owner AGENT-001 perche e l'autore del protocollo v0 e di coblox-core, quindi conosce entrambi i lati della correzione. Milestone M-02 perche va chiuso prima della devnet, coerentemente con la ragione per cui DEBT-005 non poteva essere rimandato."
    evidence_refs: []
---
# lifecycle_u8 non e definito: due implementazioni conformi divergono sulla state_root

## Statement

La preimmagine di app_leaf definita in docs/protocol/ledger.md commette un campo lifecycle_u8, ma nessuno dei quattro documenti di protocollo assegna un valore numerico agli stati del ciclo di vita di un conto di app. Gli stati compaiono ovunque solo come stringhe: active, grace, suspended. Due implementazioni conformi possono quindi scegliere codifiche diverse, calcolare app_leaf diverse per lo stesso identico stato, e da li state_root diverse. La conseguenza e una divisione della catena al primo conto di app che non sia active. Il registro di conformita non copre app_leaf, quindi nessuna fixture pubblicata avrebbe intercettato la divergenza.

## Evidence and provenance

Trovato da AGENT-001 durante l'implementazione di SPEC-008 e verificato in modo indipendente dal Lead. La preimmagine e a docs/protocol/ledger.md riga 2022 e include lifecycle_u8 fra i campi di app_leaf. Una ricerca su tutti i documenti di docs/protocol/ restituisce lifecycle solo come stringa: alla riga 341 nella descrizione del conto di app, alla riga 2042 come campo di schema con i tre valori stringa, e alla riga 2143 nella descrizione della visualizzazione. Nessuna occorrenza assegna un numero. La ricerca del termine lifecycle_u8 restituisce la sola riga 2022, cioe l'unico punto che lo usa e nessun punto che lo definisca.

Il difetto e emerso scrivendo il codice, che e l'unico modo in cui poteva emergere: la specifica si legge come completa perche il nome del campo suggerisce una codifica ovvia, e tre implementatori sceglierebbero probabilmente 0, 1, 2 nell'ordine di elencazione. Probabilmente non e certamente, e su una state_root la differenza fra i due avverbi e una divisione della catena.

Mitigazione in essere, non risoluzione: coblox-core usa una codifica provvisoria 0/1/2 documentata come tale sul tipo e bloccata da un test che dichiara esplicitamente di non essere una prova di correttezza. AGENT-001 ha segnalato senza correggere, perche docs/protocol/ era sola lettura per SPEC-008.

## Impact and scope boundary

E un difetto di interoperabilita che si manifesta solo quando esistono due implementazioni indipendenti, cioe esattamente quando il progetto avra successo. Fino ad allora una sola implementazione e internamente coerente e nulla si rompe, il che rende il difetto invisibile a ogni test di questa base di codice. La gravita e high e non critical perche non compromette una rete a implementazione singola e perche la correzione e una riga di specifica; ma va chiuso prima che esista una seconda implementazione o una devnet con storia conservabile, per la stessa ragione per cui DEBT-005 andava chiuso prima di accumulare storia.

Colpisce inoltre una superficie che il registro di conformita non copre, il che indica una lacuna piu generale da valutare nella stessa occasione: quante altre preimmagini non hanno una fixture pubblicata, e fra queste quante contengono campi la cui codifica non e fissata altrove.

## Decision log

Created by project-lead: Registrato come debito e non come remediation di SPEC-008 perche la correzione sta in docs/protocol/, che quella spec dichiarava sola lettura, e perche tocca una preimmagine gia pubblicata: e lavoro di specifica con una fixture da aggiungere, non una modifica al crate. Owner AGENT-001 perche e l'autore del protocollo v0 e di coblox-core, quindi conosce entrambi i lati della correzione. Milestone M-02 perche va chiuso prima della devnet, coerentemente con la ragione per cui DEBT-005 non poteva essere rimandato.

## Resolution criteria

docs/protocol/ledger.md assegna un valore numerico esplicito a ciascuno dei tre stati del ciclo di vita, nel punto in cui definisce app_leaf o nello schema del conto di app, e la scelta e stabile perche entra in una preimmagine. Viene aggiunta al registro di conformita una fixture che copra app_leaf con uno stato diverso da active, cosi che la codifica diventi verificabile e non solo dichiarata. La codifica provvisoria di coblox-core e sostituita da quella normativa, e il test che dichiara di non essere una prova di correttezza viene sostituito da uno che lo e. Da valutare nella stessa occasione l'estensione del registro alle altre preimmagini oggi prive di fixture.

## Resolution evidence

