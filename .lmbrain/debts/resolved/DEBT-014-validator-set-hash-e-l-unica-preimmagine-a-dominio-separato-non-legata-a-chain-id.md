---
id: DEBT-014
title: "validator_set_hash e l'unica preimmagine a dominio separato non legata a chain_id"
status: resolved
category: "security"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "SPEC-010"
origin_ref: "segnalazione fuori ambito di AGENT-001, registrata in REVIEW-015"
related_specs: ["SPEC-001","SPEC-006","SPEC-010"]
related_reviews: ["REVIEW-015"]
related_decisions: ["ADR-001"]
target_specs: []
blocked_by: []
resolution_refs: ["SPEC-016","REVIEW-025","REVIEW-027"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-26
tags: ["consensus","conformance","replay"]
links: []
activity:
  - date: 2026-08-26
    action: "resolved: Chiuso come rifiuto motivato: l'omissione di chain_id da validator_set_hash e' deliberata e resta, ed e' ora dichiarata invece che implicita. Un'eccezione non scritta si legge come una dimenticanza, e un lettore che l'assumesse la chiuderebbe muovendo ogni valore pubblicato che ne dipende, per nulla.\n\nLa parte che vale piu' della chiusura e' che il debito conteneva due affermazioni del Lead e nessuna delle due ha retto.\n\nLa prima era l'argomento - \"e' una lista di chiavi\" - falso, e il Lead lo aveva gia' saputo prima di dispacciare, tanto da vietarne l'uso: un'eccezione motivata con la ragione sbagliata diventa un precedente, e il precedente e' peggio del difetto. L'argomento vero e' che un ValidatorSet e' legato alla propria catena dai propri byte, e che ogni oggetto che nomina un set per hash ha contenuti che differiscono fra catene.\n\nLa seconda era il titolo stesso di questo debito. Il superlativo \"l'unica preimmagine a dominio separato non legata a chain_id\" e' falso, e proviene dall'inventario di SPEC-010: sei altre preimmagini omettono chain_id, e per object_id e input_hash l'indipendenza dalla catena e' richiesta perche' sono indirizzi di contenuto. Un superlativo non verificato dentro un'eccezione dichiarata sarebbe stato ereditato da chiunque avesse letto il documento.\n\nE la classe sostitutiva ha richiesto tre stesure prima di essere vera: enunciata senza \"a dominio separato\" era di nuovo falsa, perche' le preimmagini ad albero sono controesempi. Tre giri sulla stessa frase, tutti e tre trovati da qualcun altro, sono la misura giusta di quanto sia difficile scrivere un'eccezione con la sua classe esatta - e la ragione per cui vale la pena farlo invece di scrivere \"l'unica\"."
debt_events:
  - schema_version: "1"
    id: "DEBT-014-EVENT-001"
    timestamp: "2026-08-25T19:40:11.165444400+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-010, su segnalazione fuori ambito di AGENT-001. Registrato come debito e non come remediation perche la spec lo dichiarava fuori perimetro e perche non e noto se sia un difetto: promuoverlo a finding senza valutazione sarebbe affermare piu di quanto si sappia, che e la famiglia 2 di recurring-defects.md. Owner AGENT-007 perche serve la valutazione di chi ha mandato di attaccarla, e non di chi l'ha osservata ne del Lead che l'ha registrata."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-014-EVENT-002"
    timestamp: "2026-08-26T02:31:12.552268400+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Chiuso come rifiuto motivato: l'omissione di chain_id da validator_set_hash e' deliberata e resta, ed e' ora dichiarata invece che implicita. Un'eccezione non scritta si legge come una dimenticanza, e un lettore che l'assumesse la chiuderebbe muovendo ogni valore pubblicato che ne dipende, per nulla.\n\nLa parte che vale piu' della chiusura e' che il debito conteneva due affermazioni del Lead e nessuna delle due ha retto.\n\nLa prima era l'argomento - \"e' una lista di chiavi\" - falso, e il Lead lo aveva gia' saputo prima di dispacciare, tanto da vietarne l'uso: un'eccezione motivata con la ragione sbagliata diventa un precedente, e il precedente e' peggio del difetto. L'argomento vero e' che un ValidatorSet e' legato alla propria catena dai propri byte, e che ogni oggetto che nomina un set per hash ha contenuti che differiscono fra catene.\n\nLa seconda era il titolo stesso di questo debito. Il superlativo \"l'unica preimmagine a dominio separato non legata a chain_id\" e' falso, e proviene dall'inventario di SPEC-010: sei altre preimmagini omettono chain_id, e per object_id e input_hash l'indipendenza dalla catena e' richiesta perche' sono indirizzi di contenuto. Un superlativo non verificato dentro un'eccezione dichiarata sarebbe stato ereditato da chiunque avesse letto il documento.\n\nE la classe sostitutiva ha richiesto tre stesure prima di essere vera: enunciata senza \"a dominio separato\" era di nuovo falsa, perche' le preimmagini ad albero sono controesempi. Tre giri sulla stessa frase, tutti e tre trovati da qualcun altro, sono la misura giusta di quanto sia difficile scrivere un'eccezione con la sua classe esatta - e la ragione per cui vale la pena farlo invece di scrivere \"l'unica\"."
    evidence_refs: ["SPEC-016", "REVIEW-025", "REVIEW-027"]
---
# validator_set_hash e l'unica preimmagine a dominio separato non legata a chain_id

## Statement

Fra tutte le preimmagini a dominio separato del protocollo, validator_set_hash e l'unica che non lega chain_id. Ogni altra preimmagine della stessa classe lo include, e l'inclusione e cio che impedisce a un oggetto valido su una catena di essere valido su un'altra. Un insieme di validatori identico su due catene diverse produce quindi lo stesso validator_set_hash, e ogni oggetto che si riferisce a quel set per hash — certificati di quorum, intestazioni di blocco, checkpoint di soggettivita debole, transizioni di set — vi si riferisce con lo stesso valore su entrambe.

## Evidence and provenance

Segnalato da AGENT-001 durante SPEC-010 come osservazione fuori ambito, emersa costruendo l'inventario delle preimmagini pubblicate: enumerando i domini di separazione uno per uno, questo e l'unico della sua classe privo del legame. Registrato in REVIEW-015 fra le tre segnalazioni fuori ambito.

Non e stato verificato ne dal Lead ne da una review di sicurezza se l'asimmetria sia un difetto o una scelta. Esiste un argomento plausibile in entrambe le direzioni, e nessuno dei due e stato messo alla prova: che sia deliberata, perche un insieme di validatori e una lista di chiavi e non un oggetto di catena, e legarla alla catena impedirebbe di riusare la stessa lista in una genesi nuova; oppure che sia un'omissione, perche ogni altro oggetto della stessa classe lo lega e la conseguenza e che un riferimento per hash non distingue le catene.

## Impact and scope boundary

Da stabilire, ed e il lavoro. La superficie da esaminare e il riuso fra catene: due reti Coblox che condividono un insieme di validatori — una devnet e una rete di prova, oppure una rete e il suo fork — e cosa comporta che gli oggetti che nominano il set per hash non distinguano quale delle due. Va valutato separatamente per i certificati di quorum, per i checkpoint di soggettivita debole che un light client usa come ancora, e per le transizioni di set.

Il Lead non attribuisce una gravita nel merito e la registra come medium in via cautelativa: e un'asimmetria in una preimmagine consensuale, che e la classe di difetto piu costosa da correggere dopo che una rete ha storia, e la piu economica prima.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-010, su segnalazione fuori ambito di AGENT-001. Registrato come debito e non come remediation perche la spec lo dichiarava fuori perimetro e perche non e noto se sia un difetto: promuoverlo a finding senza valutazione sarebbe affermare piu di quanto si sappia, che e la famiglia 2 di recurring-defects.md. Owner AGENT-007 perche serve la valutazione di chi ha mandato di attaccarla, e non di chi l'ha osservata ne del Lead che l'ha registrata.

## Resolution criteria

Una valutazione adversariale che stabilisca se l'assenza di chain_id in validator_set_hash sia deliberata o un'omissione, pronunciandosi separatamente sui certificati di quorum, sui checkpoint di soggettivita debole e sulle transizioni di set. Gli esiti ammissibili sono due e vanno distinti: legare chain_id come le altre preimmagini della stessa classe, con la conseguenza che ogni valore pubblicato che ne dipende va ricalcolato; oppure il rifiuto motivato, con l'asimmetria dichiarata nel documento accanto alla definizione invece che lasciata implicita, perche un'eccezione non scritta si legge come una dimenticanza.

Da chiudere prima che una devnet accumuli storia conservabile, per la stessa ragione per cui DEBT-005 e DEBT-012 non potevano essere rimandati: e una preimmagine consensuale.

## Resolution evidence

Chiuso da SPEC-016 come rifiuto motivato, in due paragrafi in docs/protocol/README.md piu' i commenti in registry.rs e hash.rs.

Il documento enuncia la classe vera - una preimmagine a dominio separato su un oggetto di consenso specifico della catena che altri oggetti di consenso nominano per hash - ed enumera i diciotto membri che portano chain_id_32 e i sei controesempi ad albero. L'argomento e' che un ValidatorSet e' gia' legato alla propria catena dai propri byte, tre volte: election_seed e ogni election_ticket sono derivati attraverso chain_id_32, e ogni key_binding_signature e' presa sulla procedura di firma legata alla catena.

Verificato dal Lead per esaurimento: dht_namespace_key esiste in core/coblox-core/src/registry.rs e prende solo genesis_block_id; account_key e' in ledger.md; l'inventario di SPEC-010 conferma il resto. L'argomento falso non compare in alcun artefatto.

AGENT-007 ha attaccato il legame di catena su tutte e tre le superfici del debito, genesi compresa, senza romperlo: il rifiuto e' corretto nel merito. RF-003 di REVIEW-027 riguardava solo come era motivato ed e' stato chiuso, con la pronuncia separata sulle tre superfici scritta in entrambi i documenti e con la nota che sul set di genesi l'argomento per byte poggia su chain_id, aperto come DEBT-020.</resolution_evidence>
</invoke>
