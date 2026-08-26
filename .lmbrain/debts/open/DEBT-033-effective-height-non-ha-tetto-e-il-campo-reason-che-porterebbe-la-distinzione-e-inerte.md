---
id: DEBT-033
title: "effective_height non ha tetto, e il campo reason che porterebbe la distinzione e' inerte"
status: open
category: "security"
severity: "high"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: "REVIEW-033"
origin_ref: "RF-001"
related_specs: ["SPEC-019"]
related_reviews: ["REVIEW-033"]
related_decisions: ["ADR-010"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-26
tags: ["security","ledger","identity"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-033-EVENT-001"
    timestamp: "2026-08-26T13:59:29.971475600+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead perche' chiudere SPEC-019 senza registrarlo avrebbe fatto sparire la superficie insieme al debito che la nominava.\n\nVale la pena registrare la forma, perche' e' la seconda volta in poche ore che si presenta. In SPEC-017 il difetto stava fra le due meta': comporre DEBT-020 e DEBT-021 apriva una finestra che nessuno dei due guardava. Qui il difetto e' salito di un livello: chiudere l'asimmetria fra le quattro righe di autorizzazione ha spostato il peso su effective_height, e quella grandezza non era vincolata perche' fino a ieri non portava questo peso.\n\nNon e' un difetto introdotto da SPEC-019: e' un difetto che SPEC-019 ha reso visibile e rilevante. Una chiusura corretta puo' rendere critica una grandezza che prima non lo era, e nessuna gate lo cerca, perche' la grandezza non e' cambiata - e' cambiato cio' che vi poggia sopra."
    evidence_refs: []
---
# effective_height non ha tetto, e il campo reason che porterebbe la distinzione e' inerte

## Statement

Nessuna regola limita quanto in alto possa stare effective_height in un revoke_identity. Due soli MUST lo nominano: uno impone che stia almeno min_revocation_effective_delay_blocks sopra il blocco proponente, l'altro e' la regola del light client. Il solo riferimento verso l'alto, alla riga 1037, e' una condizione sulla validita' di una contrazione del set e non un tetto: una revoca con effective_height assurdo non puo' giustificare una contrazione, ma resta una revoca valida.

SPEC-019 ha cambiato il peso di quel campo. Prima governava la transizione del set di validatori; ora governa anche se una chiave revocata possa svuotare un saldo, e chi lo sceglie e' il quorum che revoca. Una revoca con effective_height a 2^60 soddisfa ogni MUST ed e' cosmetica.

E il campo che porterebbe la distinzione esiste gia' ed e' inerte: reason porta key_compromise, validator_misconduct, operator_request, e' impegnato nell'ID della transazione, e nessuna regola lo legge.

## Evidence and provenance

Trovato da AGENT-007 in REVIEW-033 RF-001, riverificato dal Lead due volte.

Il tetto: la ricerca su effective_height in ledger.md filtrata per vincoli superiori restituisce una sola riga, la 1037, e letta in contesto e' la clausola 8 della regola di contrazione del set. I MUST che lo nominano sono due, entrambi verso il basso o sul light client.

L'inerzia di reason: la ricerca di key_compromise, validator_misconduct e operator_request su docs/protocol/ e core/coblox-core/src/ restituisce due occorrenze in tutto il protocollo - la dichiarazione dello schema a ledger.md:778 e la fixture canonica a 793. Nessuna regola, nessun codice.

L'implementatrice ha inoltre riverificato che light_client.rs confronti effective_height senza alcun vincolo sul valore.

## Impact and scope boundary

La superficie e' il saldo di un nodo la cui chiave e' stata compromessa, ed e' esattamente cio' che DEBT-022 proteggeva, un livello piu' in alto. Quel debito ha chiuso l'asimmetria fra le quattro righe di autorizzazione; questo dice che la grandezza su cui ora tutte e quattro poggiano non e' vincolata.

Il danno concreto: quanto una revoca protegga un saldo lo sceglie chi revoca, e chi revoca e' un quorum. Contro un key_compromise - il caso per cui la revoca esiste - un quorum lento, distratto o ostile puo' emettere una revoca formalmente ineccepibile che non protegge nulla. Non serve malizia: serve un valore scelto male, e nessuna regola lo respinge.

Si compone con RF-002 della stessa review: il pavimento e' tarato su una ragione di liveness del set di validatori - dare ai superstiti una finestra dichiarata per impegnare un set successore conforme - e quella ragione sul percorso di spesa non esiste. Un saldo non ha bisogno di una finestra per essere protetto: ne ha bisogno il set. Il pavimento e' quindi giusto per un percorso e arbitrario per l'altro, e il tetto manca su entrambi.

high per la stessa ragione per cui lo era DEBT-022: la classe piu' economica da correggere adesso, quando e' una clausola, e la piu' cara quando sara' una catena con una storia.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead perche' chiudere SPEC-019 senza registrarlo avrebbe fatto sparire la superficie insieme al debito che la nominava.

Vale la pena registrare la forma, perche' e' la seconda volta in poche ore che si presenta. In SPEC-017 il difetto stava fra le due meta': comporre DEBT-020 e DEBT-021 apriva una finestra che nessuno dei due guardava. Qui il difetto e' salito di un livello: chiudere l'asimmetria fra le quattro righe di autorizzazione ha spostato il peso su effective_height, e quella grandezza non era vincolata perche' fino a ieri non portava questo peso.

Non e' un difetto introdotto da SPEC-019: e' un difetto che SPEC-019 ha reso visibile e rilevante. Una chiusura corretta puo' rendere critica una grandezza che prima non lo era, e nessuna gate lo cerca, perche' la grandezza non e' cambiata - e' cambiato cio' che vi poggia sopra.

## Resolution criteria

Stabilire quale grandezza vincolare, e la valutazione di AGENT-002 va usata come punto di partenza perche' ha gia' scartato la risposta ovvia.

Il tetto ovvio non e' il rimedio. Un max_revocation_effective_delay_blocks e' la famiglia 3 un'altra volta - vincola la grandezza nominata invece di quella da cui la proprieta' dipende - e per giunta e' inefficace, perche' un quorum ostile sceglierebbe semplicemente il massimo ammesso. Va nominato qui perche' e' la prima cosa che verra' proposta.

Le due vie che non sono famiglia 3 tolgono la discrezione invece di limitarla, e sono entrambe meccanica della revoca:

1. far mordere la revoca sul solo percorso di spesa a min(effective_height, proponente + pavimento). E' replayable, ma reintrodurrebbe due significati di "revocata" alla stessa altezza, cioe' esattamente cio' contro cui SPEC-019 ha argomentato per scartare la lettura "finalizzata". Da valutare con quella tensione in mano.

2. Legare il ritardo a reason. E' la piu' promettente, e il fatto che la rende azionabile e' che il campo esiste gia', e' gia' impegnato nell'ID della transazione, ed e' inerte: non va aggiunto nulla al formato, va solo reso letto. Un key_compromise non ha ragione di avere lo stesso pavimento di un operator_request.

Va inoltre deciso se il pavimento debba essere lo stesso sui due percorsi, dato che la ragione che lo giustifica esiste solo su uno.

Da chiudere prima che una devnet accumuli saldi reali, che qui e' la condizione giusta - a differenza di DEBT-022, dove il pericolo era il disaccordo fra letture e la condizione era la seconda implementazione. Qui non c'e' ambiguita': c'e' una discrezione non vincolata, e il danno ha bisogno di un saldo.

## Resolution evidence

