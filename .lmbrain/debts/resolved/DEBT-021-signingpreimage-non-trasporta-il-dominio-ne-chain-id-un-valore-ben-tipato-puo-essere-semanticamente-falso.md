---
id: DEBT-021
title: "SigningPreimage non trasporta il dominio ne' chain_id: un valore ben tipato puo' essere semanticamente falso"
status: resolved
category: "design"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: "REVIEW-023"
origin_ref: "RF-002"
related_specs: ["SPEC-012","SPEC-014"]
related_reviews: ["REVIEW-023"]
related_decisions: []
target_specs: []
blocked_by: []
resolution_refs: ["SPEC-017","REVIEW-028","REVIEW-029","DEBT-029"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-26
tags: ["rust","api","security","conformance"]
links: []
activity:
  - date: 2026-08-26
    action: "resolved: Chiuso nella forma scelta sull'ergonomia dei chiamanti futuri, che era il criterio esplicito del debito: un legame che rende scomodo il caso corretto verrebbe aggirato dal primo chiamante che ha fretta.\n\nIl rifiuto del parametro di tipo sul dominio e' motivato meglio di quanto il debito chiedesse, e con un caso concreto invece che a parole: sposterebbe a compilazione meta' del controllo, perche' chain_id e' un valore e non un tipo, coprendo l'errore facile - dominio sbagliato in una funzione che nomina il dominio - e non quello difficile, dominio giusto e catena altrui. E renderebbe SignatureVerifier generico, quindi non piu' dyn, obbligando chi tiene preimmagini di domini diversi in una collezione a introdurre un enum.\n\nDue cose restano aperte e non sono residui nascosti, sono debiti con un proprietario. DEBT-029: il legame e' una convenzione e non un confine, perche' due percorsi pubblici raggiungono la verifica saltando verify_in_context. Non chiuso qui perche' il recinto giusto dipende dal primo chiamante di consenso, che non esiste: sceglierne la forma contro un chiamante immaginario sarebbe un valore ben scelto e non una proprieta'. Cio' che e' stato fatto subito e' nominare la scappatoia nel commento, perche' una convenzione che il proprio file non esemplifica non e' una convenzione.\n\nE il difetto di composizione con DEBT-020, che nessuno dei due debiti conteneva: alla genesi il chain_id e' una costante uguale ovunque, quindi il contesto degradava a un controllo di solo dominio. Chiuso legando network_id al payload del key binding a ogni altezza, non solo alla genesi, perche' una forma che cambia a un'altezza e' una forma da sbagliare."
debt_events:
  - schema_version: "1"
    id: "DEBT-021-EVENT-001"
    timestamp: "2026-08-25T23:58:25.124034300+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Aperto dal Lead alla chiusura di SPEC-014, su raccomandazione esplicita di AGENT-007 in RF-002 di REVIEW-023. Registrato come debito e non chiuso nella remediation perche e fuori dallo scope dei due debiti che SPEC-014 raggruppava, e perche allargare una spec gia passata per una remediation e il modo in cui una spec non chiude mai. Owner AGENT-001 perche e l'autore del tipo e della cucitura."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-021-EVENT-002"
    timestamp: "2026-08-26T11:22:43.217839900+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Chiuso nella forma scelta sull'ergonomia dei chiamanti futuri, che era il criterio esplicito del debito: un legame che rende scomodo il caso corretto verrebbe aggirato dal primo chiamante che ha fretta.\n\nIl rifiuto del parametro di tipo sul dominio e' motivato meglio di quanto il debito chiedesse, e con un caso concreto invece che a parole: sposterebbe a compilazione meta' del controllo, perche' chain_id e' un valore e non un tipo, coprendo l'errore facile - dominio sbagliato in una funzione che nomina il dominio - e non quello difficile, dominio giusto e catena altrui. E renderebbe SignatureVerifier generico, quindi non piu' dyn, obbligando chi tiene preimmagini di domini diversi in una collezione a introdurre un enum.\n\nDue cose restano aperte e non sono residui nascosti, sono debiti con un proprietario. DEBT-029: il legame e' una convenzione e non un confine, perche' due percorsi pubblici raggiungono la verifica saltando verify_in_context. Non chiuso qui perche' il recinto giusto dipende dal primo chiamante di consenso, che non esiste: sceglierne la forma contro un chiamante immaginario sarebbe un valore ben scelto e non una proprieta'. Cio' che e' stato fatto subito e' nominare la scappatoia nel commento, perche' una convenzione che il proprio file non esemplifica non e' una convenzione.\n\nE il difetto di composizione con DEBT-020, che nessuno dei due debiti conteneva: alla genesi il chain_id e' una costante uguale ovunque, quindi il contesto degradava a un controllo di solo dominio. Chiuso legando network_id al payload del key binding a ogni altezza, non solo alla genesi, perche' una forma che cambia a un'altezza e' una forma da sbagliare."
    evidence_refs: ["SPEC-017", "REVIEW-028", "REVIEW-029", "DEBT-029"]
---
# SigningPreimage non trasporta il dominio ne' chain_id: un valore ben tipato puo' essere semanticamente falso

## Statement

SigningPreimage garantisce che i byte passati al verificatore siano stati prodotti da signing_preimage, e non garantisce nulla su quali byte siano. Il tipo non trasporta il Domain ne il chain_id: signing_preimage li impasta nel prefisso e poi il tipo li dimentica. Un chiamante che costruisse la preimmagine con il dominio sbagliato, o con il chain_id di un'altra catena, otterrebbe un valore ben tipato e semanticamente falso, e il verificatore lo accetterebbe.

SPEC-014 ha chiuso il salto da byte grezzi a preimmagine. Questa e la stessa domanda un livello sopra: la separazione di dominio esiste per impedire che una firma valida in un contesto sia valida in un altro, e oggi e tenuta dalla correttezza del chiamante e non dal tipo.

## Evidence and provenance

RF-002 di REVIEW-023, review di sicurezza di AGENT-007 su SPEC-014, dichiarato low e fuori dallo scope dei due debiti che quella spec chiudeva, con la raccomandazione di promuoverlo a debito proprio invece di allargare la spec.

La domanda era stata posta ad AGENT-007 dal Lead nel dispatch, come superficie da guardare oltre il finding gia registrato. La sua risposta e che non e un regresso introdotto da SPEC-014 ma una proprieta che quella spec non aveva mandato di chiudere.

Il Lead alza la severita da low a medium rispetto alla review, e dichiara la ragione: il primo chiamante di consenso non esiste ancora, quindi oggi nessuno puo sbagliare, ma la finestra in cui la correzione e gratuita e la stessa che DEBT-016 ha appena usato ed e la stessa che si chiude allo stesso evento. Un debito che vale low finche nessuno usa l'API e high il giorno dopo e mal classificato come low.

## Impact and scope boundary

Nessun impatto oggi: non esiste alcun chiamante del verificatore. Il danno potenziale e della classe che questo componente ha per natura, cioe un'accettazione silenziosa invece di un errore, ed e la stessa forma del replay cross-chain che RF-001 di REVIEW-023 descriveva: una firma legata a un contesto accettata in un altro.

La differenza rispetto a RF-001 e che li il prefisso spariva del tutto, qui il prefisso c'e ma puo essere quello sbagliato. E un fallimento piu difficile da notare, perche il valore ha la forma giusta.

## Decision log

Created by project-lead: Aperto dal Lead alla chiusura di SPEC-014, su raccomandazione esplicita di AGENT-007 in RF-002 di REVIEW-023. Registrato come debito e non chiuso nella remediation perche e fuori dallo scope dei due debiti che SPEC-014 raggruppava, e perche allargare una spec gia passata per una remediation e il modo in cui una spec non chiude mai. Owner AGENT-001 perche e l'autore del tipo e della cucitura.

## Resolution criteria

Il tipo trasporta il dominio e il chain_id con cui e stato costruito, e il verificatore o il chiamante possono verificarli contro cio che si aspettano, cosi che una preimmagine costruita per un contesto non sia utilizzabile in un altro senza che qualcosa lo dica. Oppure la dimostrazione motivata che il legame non serve, con la ragione scritta accanto al tipo invece che lasciata implicita.

Va valutato se la forma giusta sia un tipo parametrizzato sul dominio, un campo confrontato in verifica, o una funzione di verifica che prende dominio e chain_id attesi. La scelta ha conseguenze sull'ergonomia dei chiamanti che ancora non esistono, il che e un argomento per decidere ora e non dopo.

Da chiudere prima del primo chiamante del verificatore, per la stessa ragione di DEBT-016.

## Resolution evidence

Chiuso da SPEC-017. SigningPreimage porta ora un PreimageContext, binds() lo confronta con l'attesa, e verify_in_context e' il punto d'ingresso verificato - funzione libera e non metodo di default del trait, perche' un default si puo' sovrascrivere e chi lo sovrascrivesse toglierebbe il controllo lasciandone il nome.

GATE-WRONG-CONTEXT-REJECTED soddisfatta nel ramo "e' rifiutata", con matrice 4x4: sedici celle, quattro accettazioni, e il file dichiara esplicitamente che nessuna grandezza e' tenuta costante - la domanda del passo 4 di SKILL-001, anticipata invece che subita. Provata in negativo rendendo binds() sempre true: tre test cadono con l'asserzione stampata.

AGENT-007 ha attaccato in REVIEW-029 una divergenza fra binds() e i byte effettivi: impossibile, stessi tre argomenti nella stessa espressione su campo privato.

Il difetto di composizione trovato in REVIEW-029 RF-002 e' chiuso: l'oggetto del key binding porta ora network_id, e la prova in negativo mostra i 32 byte zero del segnaposto nel payload che prima faceva coincidere due reti diverse. Verificato dal Lead: nessun valore pubblicato mosso, perche' key_binding_signature non compare in alcuna riga di valore del manifesto.</resolution_evidence>
</invoke>
