---
id: DEBT-029
title: "Il legame di contesto della preimmagine non e' imposto da nulla: due percorsi pubblici saltano verify_in_context"
status: planned
category: "security"
severity: "medium"
origin_severity: null
area: "core"
milestone: "M-02"
owner: "AGENT-001"
origin_artifact: "REVIEW-029"
origin_ref: "RF-001"
related_specs: ["SPEC-014","SPEC-017"]
related_reviews: ["REVIEW-022","REVIEW-029"]
related_decisions: []
target_specs: ["SPEC-029"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-27
tags: ["security","api","rust"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: L'evento di apertura di questo debito dice perche' non fu chiuso dentro SPEC-017: \"il recinto giusto non e' determinabile oggi: non esiste un chiamante di consenso, e sceglierne la forma contro un chiamante immaginario e' la classe di errore che questo progetto ha gia' pagato\". SPEC-025 e' quel chiamante — il motore di consenso — quindi e' la consegna in cui la condizione diventa vera.\n\nI criteri chiedono che il caso corretto sia quello che un chiamante di consenso raggiunge senza sforzo e che la via corta gli costi qualcosa: e' un giudizio che si puo' dare solo avendo davanti il chiamante reale, non prima."
  - date: 2026-08-27
    action: "planned: Ri-instradato sullo stesso bersaglio per registrare che **la premessa dichiarata di questo debito e' cambiata**, come REVIEW-047 RF-007 ha accertato.\n\nIl debito diceva: \"Il danno oggi e' nullo, non esiste alcun chiamante di consenso da proteggere, il difetto e' interamente sul futuro\", con severita' medium motivata da \"la finestra si chiude quando il primo chiamante viene scritto\". **Quella finestra si e' chiusa con il commit 76b5bd3.** SPEC-025 e' il primo chiamante di consenso, e `verifier.rs:94` portava la frase che lo negava — era la giustificazione portante per lasciare aperta la scappatoia, non una descrizione. Il Lead l'ha corretta insieme alle altre tre del crate.\n\n**La meta' buona conta piu' della cattiva, e va registrata.** Il primo chiamante ha preso la strada giusta da se': `messages::verify_vote` e `QuorumCertificate::verify` passano entrambi da `verify_in_context` invece di aggirarlo. La convenzione ha ora l'esempio funzionante che il proprio file non poteva dare, ed e' esattamente cio' che i criteri di risoluzione di questo debito chiedevano di poter giudicare \"avendo davanti il chiamante reale\".\n\n**Cio' che non e' successo e' il recinto**: `verify_consensus_ed25519` resta riesportata alla radice senza feature gate ne' guardia. Il costo si e' quindi ri-tarato: non e' piu' un recinto attorno a terreno vuoto ma una **migrazione**, perche' i chiamanti esistono.\n\nIl giro di remediation aperto su SPEC-025 il 2026-08-27 **non copre questo debito**: il Lead ha escluso RF-007 dall'incarico dell'implementatrice perche' la review lo assegna a se stesso. Va quindi ri-instradato sulla spec della devnet quando esistera', ed e' la prima cosa da fare quando quella spec viene scritta: e' l'ultima consegna in cui il recinto costa poco."
  - date: 2026-08-27
    action: "planned: Ri-instradato da SPEC-025, ora `done`, a SPEC-029, come il Lead aveva scritto il 2026-08-27 nel momento in cui la premessa del debito e' cambiata: \"va ri-instradato sulla spec della devnet quando esistera', ed e' la prima cosa da fare li'\". Quella spec ora esiste e porta il recinto fra i propri Included.\n\nLa ragione per cui non e' stato chiuso dentro SPEC-025 resta valida e va ripetuta perche' non si perda: SPEC-025 e' il primo chiamante di consenso, e ha preso la strada recintata da se' — `verify_vote` e `QuorumCertificate::verify` passano entrambi da `verify_in_context`. La convenzione ha quindi l'esempio funzionante che i criteri di questo debito chiedevano di poter giudicare \"avendo davanti il chiamante reale\". Cio' che manca e' il recinto: `verify_consensus_ed25519` resta riesportata alla radice senza feature gate ne' guardia.\n\nSPEC-029 e' l'ultima consegna in cui il recinto costa poco. Dopo di essa i chiamanti si moltiplicano con la rete e la persistenza, e chiuderlo diventa una migrazione."
debt_events:
  - schema_version: "1"
    id: "DEBT-029-EVENT-001"
    timestamp: "2026-08-26T11:07:01.728293500+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead invece di essere chiuso dentro [SPEC-017], su indicazione esplicita all'implementatore. La ragione e' che il recinto giusto non e' determinabile oggi: non esiste un chiamante di consenso, e sceglierne la forma contro un chiamante immaginario e' la classe di errore che questo progetto ha gia' pagato.\n\nCio' che invece e' stato chiesto subito, perche' costa una riga e chiude la meta' peggiore del difetto, e' **nominare la scappatoia nel commento** accanto alla preimmagine. Una convenzione che il proprio file non esemplifica non e' una convenzione: e' una preferenza dell'autore che il lettore successivo non ha modo di conoscere."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-029-EVENT-002"
    timestamp: "2026-08-27T15:01:15.286117600+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "L'evento di apertura di questo debito dice perche' non fu chiuso dentro SPEC-017: \"il recinto giusto non e' determinabile oggi: non esiste un chiamante di consenso, e sceglierne la forma contro un chiamante immaginario e' la classe di errore che questo progetto ha gia' pagato\". SPEC-025 e' quel chiamante — il motore di consenso — quindi e' la consegna in cui la condizione diventa vera.\n\nI criteri chiedono che il caso corretto sia quello che un chiamante di consenso raggiunge senza sforzo e che la via corta gli costi qualcosa: e' un giudizio che si puo' dare solo avendo davanti il chiamante reale, non prima."
    evidence_refs: ["SPEC-025"]
  - schema_version: "1"
    id: "DEBT-029-EVENT-003"
    timestamp: "2026-08-27T17:47:11.867356300+02:00"
    action: "planned"
    from_status: "planned"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Ri-instradato sullo stesso bersaglio per registrare che **la premessa dichiarata di questo debito e' cambiata**, come REVIEW-047 RF-007 ha accertato.\n\nIl debito diceva: \"Il danno oggi e' nullo, non esiste alcun chiamante di consenso da proteggere, il difetto e' interamente sul futuro\", con severita' medium motivata da \"la finestra si chiude quando il primo chiamante viene scritto\". **Quella finestra si e' chiusa con il commit 76b5bd3.** SPEC-025 e' il primo chiamante di consenso, e `verifier.rs:94` portava la frase che lo negava — era la giustificazione portante per lasciare aperta la scappatoia, non una descrizione. Il Lead l'ha corretta insieme alle altre tre del crate.\n\n**La meta' buona conta piu' della cattiva, e va registrata.** Il primo chiamante ha preso la strada giusta da se': `messages::verify_vote` e `QuorumCertificate::verify` passano entrambi da `verify_in_context` invece di aggirarlo. La convenzione ha ora l'esempio funzionante che il proprio file non poteva dare, ed e' esattamente cio' che i criteri di risoluzione di questo debito chiedevano di poter giudicare \"avendo davanti il chiamante reale\".\n\n**Cio' che non e' successo e' il recinto**: `verify_consensus_ed25519` resta riesportata alla radice senza feature gate ne' guardia. Il costo si e' quindi ri-tarato: non e' piu' un recinto attorno a terreno vuoto ma una **migrazione**, perche' i chiamanti esistono.\n\nIl giro di remediation aperto su SPEC-025 il 2026-08-27 **non copre questo debito**: il Lead ha escluso RF-007 dall'incarico dell'implementatrice perche' la review lo assegna a se stesso. Va quindi ri-instradato sulla spec della devnet quando esistera', ed e' la prima cosa da fare quando quella spec viene scritta: e' l'ultima consegna in cui il recinto costa poco."
    evidence_refs: ["SPEC-025"]
  - schema_version: "1"
    id: "DEBT-029-EVENT-004"
    timestamp: "2026-08-27T18:44:24.529078400+02:00"
    action: "planned"
    from_status: "planned"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Ri-instradato da SPEC-025, ora `done`, a SPEC-029, come il Lead aveva scritto il 2026-08-27 nel momento in cui la premessa del debito e' cambiata: \"va ri-instradato sulla spec della devnet quando esistera', ed e' la prima cosa da fare li'\". Quella spec ora esiste e porta il recinto fra i propri Included.\n\nLa ragione per cui non e' stato chiuso dentro SPEC-025 resta valida e va ripetuta perche' non si perda: SPEC-025 e' il primo chiamante di consenso, e ha preso la strada recintata da se' — `verify_vote` e `QuorumCertificate::verify` passano entrambi da `verify_in_context`. La convenzione ha quindi l'esempio funzionante che i criteri di questo debito chiedevano di poter giudicare \"avendo davanti il chiamante reale\". Cio' che manca e' il recinto: `verify_consensus_ed25519` resta riesportata alla radice senza feature gate ne' guardia.\n\nSPEC-029 e' l'ultima consegna in cui il recinto costa poco. Dopo di essa i chiamanti si moltiplicano con la rete e la persistenza, e chiuderlo diventa una migrazione."
    evidence_refs: ["SPEC-029"]
---
# Il legame di contesto della preimmagine non e' imposto da nulla: due percorsi pubblici saltano verify_in_context

## Statement

[SPEC-017] ha introdotto `PreimageContext` e `verify_in_context` perche' una preimmagine costruita per il dominio o la catena sbagliata non sia utilizzabile in un altro contesto. **Ma nulla impone di passare da li'.**

`verify_consensus_ed25519` e' riesportata alla radice del crate (`core/coblox-core/src/lib.rs:125`) e `SignatureVerifier::verify` e' pubblica: due percorsi raggiungono la verifica saltando il controllo di contesto, senza feature gate, senza guardia in CI e senza test che lo vieti. La difesa e' disponibile, non imposta.

## Evidence and provenance

`grep -rn "verify_in_context" core/ --include=*.rs`: **un solo chiamante in `src/`**, ed e' `core/coblox-core/src/identity.rs:232`, cioe' un oggetto **non di consenso** — l'attestazione di chiave di trasporto. Gli altri riferimenti sono la definizione, la riesportazione, un commento e la suite `tests/preimage_context.rs`.

Trovato da AGENT-007 in [REVIEW-029] RF-001 e riverificato dal Lead.

**E' la stessa forma di [REVIEW-022] RF-001**, dove il campo di `SigningPreimage` era `pub(crate)` e la garanzia era nominale dentro il crate. Aggravata da un fatto che rende il difetto piu' visibile e non meno: **il rimedio e' gia' nello stesso codice, applicato alla scappatoia gemella.** `from_raw_bytes_non_consensus` porta la propria natura nel nome ed e' dietro `#[cfg(feature = "conformance-testing")]` — due recinti. Questa via ne ha zero.

## Impact and scope boundary

La superficie e' la separazione di dominio, cioe' la difesa che impedisce a una firma valida in un contesto di valere in un altro. Un chiamante di consenso che usasse la via corta accetterebbe una preimmagine ben tipata e semanticamente falsa: esattamente il difetto che [DEBT-021] descriveva e che questa spec doveva chiudere.

**Il danno oggi e' nullo e questo va detto**: `core/coblox-core/src/light_client.rs:119` dichiara che il crate non spedisce verificatori, quindi non esiste alcun chiamante di consenso da proteggere. Il difetto e' interamente sul **futuro**: il primo chiamante di consenso scrivera' cio' che trova, e cio' che trova oggi e' una convenzione che il proprio file non esemplifica — l'unico chiamante di `verify_in_context` e' su un oggetto non di consenso.

`medium` e non `high` perche' nessun percorso di consenso esiste; `medium` e non `low` perche' la finestra si chiude quando il primo chiamante viene scritto, e dopo costa una migrazione invece di un recinto.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead invece di essere chiuso dentro [SPEC-017], su indicazione esplicita all'implementatore. La ragione e' che il recinto giusto non e' determinabile oggi: non esiste un chiamante di consenso, e sceglierne la forma contro un chiamante immaginario e' la classe di errore che questo progetto ha gia' pagato.

Cio' che invece e' stato chiesto subito, perche' costa una riga e chiude la meta' peggiore del difetto, e' **nominare la scappatoia nel commento** accanto alla preimmagine. Una convenzione che il proprio file non esemplifica non e' una convenzione: e' una preferenza dell'autore che il lettore successivo non ha modo di conoscere.

## Resolution criteria

Il caso corretto deve essere quello che un chiamante di consenso raggiunge **senza sforzo**, e la via corta deve costargli qualcosa: un nome che dichiari la propria natura, un feature gate, o entrambi, sulla falsariga di `from_raw_bytes_non_consensus`.

**Da decidere insieme al primo chiamante di consenso e non prima**, perche' la forma giusta del recinto dipende da come quel chiamante e' fatto: un recinto scelto contro un chiamante immaginario e' un valore ben scelto e non una proprieta' ([ADR-010]), e rischia di rendere scomodo il caso corretto — che e' il criterio con cui [SPEC-017] ha scelto la forma del legame, e che il rimedio non deve violare.

Va inoltre esemplificata la convenzione **sul percorso che deve proteggere**: finche' l'unico chiamante di `verify_in_context` e' un oggetto non di consenso, il file insegna il contrario di cio' che dichiara.

**Il rimedio apparente da non adottare:** togliere `verify_consensus_ed25519` dalla riesportazione e basta. Chiuderebbe una delle due vie e lascerebbe `SignatureVerifier::verify`, che e' un trait pubblico e implementabile da chiunque. Vincolerebbe la via nominata invece di quella da cui la proprieta' dipende.

## Resolution evidence

