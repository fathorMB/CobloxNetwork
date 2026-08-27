---
id: DEBT-039
title: "glib 0.18.5 resta vulnerabile perche' javascriptcore-rs la pinna esattamente: nessun percorso di aggiornamento fino a wry"
status: accepted-risk
category: "security"
severity: "low"
origin_severity: null
area: "build"
milestone: "M-04"
owner: "AGENT-008"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-002"]
related_reviews: []
related_decisions: []
target_specs: []
blocked_by: []
resolution_refs: ["SPEC-002"]
superseded_by: null
revisit_condition: "`wry` o `javascriptcore-rs` rilasciano una versione che ammette `glib >= 0.20`, e l'aggiornamento diventa risolvibile. In alternativa, e con urgenza invece che in attesa: se il progetto arrivasse a usare direttamente API `glib::Variant`, l'esposizione verificata decadrebbe e il rischio accettato andrebbe rivalutato subito."
created: 2026-08-27
updated: 2026-08-27
tags: ["security","build","dependencies"]
links: []
activity:
  - date: 2026-08-27
    action: "accepted-risk: Rischio accettato dall'operatore il 2026-08-27, dopo la verifica di esposizione che l'operatore stesso ha chiesto di fare prima di accettare. Il debito era rimasto in `open` per un errore del Lead: l'accettazione era gia' stata data e registrata nella motivazione, ma lo stato dell'artefatto non la portava, quindi il debito compariva fra quelli da lavorare invece che fra quelli decisi.\n\nI due fatti che sostengono l'accettazione sono accertati eseguendo, non dedotti. Non esiste percorso di aggiornamento: `cargo update --dry-run -p glib@0.18.5 --precise 0.20.0` fallisce la risoluzione perche' `javascriptcore-rs = \"=1.1.2\"`, dentro `wry 0.55` e quindi `tauri 2.11.5`, impone `glib = \"^0.18.0\"` con un pin esatto. E il codice di progetto non raggiunge la superficie vulnerabile: `apps/desktop/src-tauri/src/` contiene un solo file di venti righe, senza alcuna occorrenza di `VariantStrIter`, `glib::` o `Variant`. Lo stack gtk/javascriptcore e' il backend Linux e la build desktop su Windows non lo esercita a runtime.\n\nL'alert Dependabot e' stato chiuso come `tolerable_risk` citando questo debito."
debt_events:
  - schema_version: "1"
    id: "DEBT-039-EVENT-001"
    timestamp: "2026-08-27T10:11:43.217786200+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Rischio accettato su decisione esplicita dell'operatore del 2026-08-27, dopo la verifica di esposizione che l'operatore ha chiesto di fare prima di accettare. L'alternativa scartata era accettare senza verifica: avrebbe prodotto un'affermazione della forma \"probabilmente non lo usiamo\" invece di \"verificato che non lo usiamo\", che e' la classe di difetto censita in knowledge/recurring-defects.md. L'aggiornamento non e' fra le opzioni perche' non esiste: e' stato accertato eseguendo il risolutore, non leggendo il grafo."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-039-EVENT-002"
    timestamp: "2026-08-27T15:02:28.634121800+02:00"
    action: "accepted-risk"
    from_status: "open"
    to_status: "accepted-risk"
    actor_role: "operator"
    actor: "AGENT-LEAD"
    rationale: "Rischio accettato dall'operatore il 2026-08-27, dopo la verifica di esposizione che l'operatore stesso ha chiesto di fare prima di accettare. Il debito era rimasto in `open` per un errore del Lead: l'accettazione era gia' stata data e registrata nella motivazione, ma lo stato dell'artefatto non la portava, quindi il debito compariva fra quelli da lavorare invece che fra quelli decisi.\n\nI due fatti che sostengono l'accettazione sono accertati eseguendo, non dedotti. Non esiste percorso di aggiornamento: `cargo update --dry-run -p glib@0.18.5 --precise 0.20.0` fallisce la risoluzione perche' `javascriptcore-rs = \"=1.1.2\"`, dentro `wry 0.55` e quindi `tauri 2.11.5`, impone `glib = \"^0.18.0\"` con un pin esatto. E il codice di progetto non raggiunge la superficie vulnerabile: `apps/desktop/src-tauri/src/` contiene un solo file di venti righe, senza alcuna occorrenza di `VariantStrIter`, `glib::` o `Variant`. Lo stack gtk/javascriptcore e' il backend Linux e la build desktop su Windows non lo esercita a runtime.\n\nL'alert Dependabot e' stato chiuso come `tolerable_risk` citando questo debito."
    evidence_refs: ["SPEC-002"]
---
# glib 0.18.5 resta vulnerabile perche' javascriptcore-rs la pinna esattamente: nessun percorso di aggiornamento fino a wry

## Statement

L'advisory Dependabot su `glib` (unsoundness negli impl di `Iterator` e `DoubleEndedIterator` per `glib::VariantStrIter`, severita' medium a monte) resta aperto sull'albero della shell desktop e non e' rimuovibile con un aggiornamento. La versione in lock e' 0.18.5, la prima corretta e' 0.20.0, e la catena delle dipendenze impone `glib = "^0.18.0"` con un pin esatto a monte.

## Evidence and provenance

Accertato per esecuzione, non per deduzione. `cargo update --dry-run -p glib@0.18.5 --precise 0.20.0` in `apps/desktop/src-tauri` fallisce la risoluzione: "failed to select a version for the requirement glib = ^0.18.0 ... required by package javascriptcore-rs v1.1.2 ... which satisfies dependency javascriptcore-rs = =1.1.2 of package wry v0.55.0 ... wry ^0.55.0 di tauri-runtime-wry v2.11.4 ... tauri ^2.11.5 di coblox-desktop". Il pin di `javascriptcore-rs` e' esatto (`=1.1.2`), quindi nemmeno una risoluzione permissiva apre la strada. `glib` non e' una dipendenza diretta: non compare in `apps/desktop/src-tauri/Cargo.toml`. Esposizione del codice di progetto verificata: `apps/desktop/src-tauri/src/` contiene un solo file, `main.rs`, venti righe, e nessuna occorrenza di `VariantStrIter`, `glib::` o `Variant`. Lo stack gtk/javascriptcore e' il backend Linux: la build desktop su Windows non lo esercita a runtime.

## Impact and scope boundary

L'unsoundness e' raggiungibile solo attraverso `VariantStrIter`, che il codice del progetto non usa in alcun punto. Il rischio residuo e' limitato a cio' che `wry` e `webkit2gtk` fanno internamente sulla piattaforma Linux, fuori dal controllo del progetto e non evitabile finche' quella catena non aggiorna. Il costo di tenerlo aperto senza artefatto sarebbe un alert acceso a tempo indeterminato su ogni push: il progetto ha gia' registrato che un segnale acceso troppo a lungo smette di essere letto.

## Decision log

Created by AGENT-LEAD: Rischio accettato su decisione esplicita dell'operatore del 2026-08-27, dopo la verifica di esposizione che l'operatore ha chiesto di fare prima di accettare. L'alternativa scartata era accettare senza verifica: avrebbe prodotto un'affermazione della forma "probabilmente non lo usiamo" invece di "verificato che non lo usiamo", che e' la classe di difetto censita in knowledge/recurring-defects.md. L'aggiornamento non e' fra le opzioni perche' non esiste: e' stato accertato eseguendo il risolutore, non leggendo il grafo.

## Resolution criteria

`wry` (o `javascriptcore-rs`) rilascia una versione che ammette `glib >= 0.20`, l'aggiornamento e' applicato e l'alert Dependabot si chiude da se'. In alternativa: se il progetto arrivasse a usare direttamente API `glib::Variant`, il rischio accettato decade e questo debito va rivalutato con urgenza invece che atteso.

## Resolution evidence

Risk accepted by AGENT-LEAD: Rischio accettato dall'operatore il 2026-08-27, dopo la verifica di esposizione che l'operatore stesso ha chiesto di fare prima di accettare. Il debito era rimasto in `open` per un errore del Lead: l'accettazione era gia' stata data e registrata nella motivazione, ma lo stato dell'artefatto non la portava, quindi il debito compariva fra quelli da lavorare invece che fra quelli decisi.

I due fatti che sostengono l'accettazione sono accertati eseguendo, non dedotti. Non esiste percorso di aggiornamento: `cargo update --dry-run -p glib@0.18.5 --precise 0.20.0` fallisce la risoluzione perche' `javascriptcore-rs = "=1.1.2"`, dentro `wry 0.55` e quindi `tauri 2.11.5`, impone `glib = "^0.18.0"` con un pin esatto. E il codice di progetto non raggiunge la superficie vulnerabile: `apps/desktop/src-tauri/src/` contiene un solo file di venti righe, senza alcuna occorrenza di `VariantStrIter`, `glib::` o `Variant`. Lo stack gtk/javascriptcore e' il backend Linux e la build desktop su Windows non lo esercita a runtime.

L'alert Dependabot e' stato chiuso come `tolerable_risk` citando questo debito.

Revisit: `wry` o `javascriptcore-rs` rilasciano una versione che ammette `glib >= 0.20`, e l'aggiornamento diventa risolvibile. In alternativa, e con urgenza invece che in attesa: se il progetto arrivasse a usare direttamente API `glib::Variant`, l'esposizione verificata decadrebbe e il rischio accettato andrebbe rivalutato subito.
