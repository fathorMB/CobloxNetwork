---
id: DEBT-016
title: "Il verificatore consensus-critical accetta una fetta di byte dove il contratto impone un messaggio"
status: resolved
category: "security"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: "REVIEW-019"
origin_ref: "RF-003"
related_specs: ["SPEC-012","SPEC-011"]
related_reviews: ["REVIEW-019"]
related_decisions: ["ADR-003"]
target_specs: []
blocked_by: []
resolution_refs: ["SPEC-014","REVIEW-022","REVIEW-023"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["rust","api","security","conformance"]
links: []
activity:
  - date: 2026-08-25
    action: "resolved: Risolto da SPEC-014, accettata con REVIEW-022 e con GATE-SECREVIEW attestato su REVIEW-023. Chiuso prima del primo chiamante del verificatore, che era la scadenza reale del debito e la ragione per cui e stato raggruppato con DEBT-015 invece di essere rimandato."
debt_events:
  - schema_version: "1"
    id: "DEBT-016-EVENT-001"
    timestamp: "2026-08-25T22:09:31.437988100+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-012. Registrato come debito e non chiuso in remediation perche la chiusura richiede di modificare verifier.rs, che il Lead aveva escluso dal perimetro, e perche estendere una terza volta una spec gia passata per due giri di review e il modo in cui una spec non chiude mai. AGENT-001 si e fermato e ha riportato invece di forzare, che e il comportamento corretto e va registrato come tale. Owner AGENT-001 perche e l'autore della cucitura e del crate."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-016-EVENT-002"
    timestamp: "2026-08-25T23:57:40.381897900+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Risolto da SPEC-014, accettata con REVIEW-022 e con GATE-SECREVIEW attestato su REVIEW-023. Chiuso prima del primo chiamante del verificatore, che era la scadenza reale del debito e la ragione per cui e stato raggruppato con DEBT-015 invece di essere rimandato."
    evidence_refs: ["SPEC-014", "REVIEW-022", "REVIEW-023"]
---
# Il verificatore consensus-critical accetta una fetta di byte dove il contratto impone un messaggio

## Statement

SignatureVerifier::verify e verify_consensus_ed25519 accettano message come fetta di byte, mentre il contratto del protocollo impone che quel valore sia la preimmagine integrale prodotta da registry::signing_preimage e mai un suo digest. Digest32::as_bytes coercisce a una fetta, quindi un chiamante che passasse un digest compilerebbe e passerebbe ogni test esistente, e il legame a dominio separato e a chain_id che la preimmagine porta decadrebbe in silenzio. Il verificatore continuerebbe a verificare qualcosa, e non cio che si crede.

E l'unica cucitura consensus-critical in cui la separazione strutturale dichiarata in lib.rs non e applicata dal tipo. Oggi nessun chiamante esiste in src, quindi il contratto ha zero applicazione e il primo chiamante ne fissera la convenzione.

## Evidence and provenance

RF-003 di REVIEW-019, review di sicurezza di AGENT-007 su SPEC-012, verdetto changes-requested. AGENT-001 ha tentato la chiusura in remediation e si e fermato riportando, come il mandato del Lead gli imponeva: non esiste chiusura che non tocchi verifier.rs, perche il tipo va imposto sulla firma sia di SignatureVerifier::verify in lib.rs sia di verify_consensus_ed25519 in verifier.rs, e il Lead aveva escluso quel file dalla remediation avendo stabilito che l'implementazione era corretta.

La sua osservazione decisiva, che il Lead condivide: un newtype introdotto nel solo registry.rs lascerebbe message come fetta di byte sulla firma del verificatore e sembrerebbe la chiusura senza esserlo. E la forma di difetto che questa stessa spec ha gia commesso una volta con PUBLISHED_OUTCOMES, cioe un artefatto che dice di essere una cosa senza esserlo.

Comprende OSS-001 di REVIEW-019: la locuzione audited primitive crate nell'intestazione di verifier.rs va resa esatta nella stessa passata, perche correggerla e modificare quel file.

## Impact and scope boundary

Nessun impatto oggi, ed e precisamente la ragione per cui va chiuso adesso: non esiste alcun chiamante, quindi il cambio di firma non rompe nulla e non costera mai meno di ora. Ogni chiamante scritto prima della chiusura rende il cambiamento breaking sul serio, e il primo di essi sara un percorso di consenso.

Il danno potenziale e della classe che questo componente ha per natura: un verificatore che verifica la cosa sbagliata non fallisce, accetta in silenzio. Severita medium e non high perche richiede l'errore di un chiamante futuro e non un difetto presente, ma la finestra si chiude al primo chiamante e non alla devnet.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-012. Registrato come debito e non chiuso in remediation perche la chiusura richiede di modificare verifier.rs, che il Lead aveva escluso dal perimetro, e perche estendere una terza volta una spec gia passata per due giri di review e il modo in cui una spec non chiude mai. AGENT-001 si e fermato e ha riportato invece di forzare, che e il comportamento corretto e va registrato come tale. Owner AGENT-001 perche e l'autore della cucitura e del crate.

## Resolution criteria

Un tipo distinto rappresenta la preimmagine di firma e compare nella firma di SignatureVerifier::verify e di verify_consensus_ed25519, cosi che passare un digest non compili. Il tipo non deve essere costruibile da byte arbitrari senza passare per registry::signing_preimage, altrimenti la garanzia e nominale. Nella stessa passata va resa esatta la locuzione audited primitive crate nell'intestazione di verifier.rs.

Va chiuso insieme a DEBT-015, che chiede l'altro cambiamento breaking dell'API pubblica di coblox-core, in una sola spec e prima del primo chiamante del verificatore. Raggrupparli e la disposizione che DEBT-015 gia prevede, e farne due passate separate raddoppierebbe il costo per gli stessi consumatori.

## Resolution evidence

Il tipo SigningPreimage compare ora nella firma sia di SignatureVerifier::verify sia di verify_consensus_ed25519, e passare un Digest32 o una fetta di byte arbitraria non compila. Verificato dal Lead con una sonda: due errori E0308, expected &amp;SigningPreimage, su entrambi i punti d'ingresso.

Il criterio che il debito poneva, cioe che il tipo non sia costruibile da byte arbitrari se non attraverso una via nominata, e soddisfatto in forma piu forte di come era stato scritto. Il campo e privato, e la via nominata from_raw_bytes_non_consensus e sotto una feature non-default abilitata dalla sola dev-dependency che il crate dichiara su se stesso: non e quindi solo nominata, e inaccessibile alle build di produzione dei crate dipendenti. Verificato dal Lead con una sonda dentro coblox-node, che fallisce con E0599 in build di produzione.

Il limite residuo e dichiarato e misurato, non taciuto: la feature unification riabilita la via sotto cargo test --workspace, verificato dal Lead compilando la stessa sonda in quel profilo. E coperto da una guardia d'albero versionata, sim/tools/non_consensus_containment.py, eseguita dalla CI e provata in negativo su quattro classi, fra cui un dipendente che abilita la feature per se con una riga in un manifesto.

La conversione e completa: tutte e quattro le produttrici di preimmagini dell'albero restituiscono SigningPreimage e nessuna resta a Vec di byte. E la proprieta che rende la chiusura reale, perche una sola lasciata indietro avrebbe costretto il primo chiamante di consenso a usare la via d'uscita per fare il ponte.

Chiusa anche OSS-001 di REVIEW-019, che il debito comprendeva: la locuzione audited primitive crate in verifier.rs non c'e piu e rimanda a Cargo.toml per la provenienza degli audit.
