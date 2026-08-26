---
id: DEBT-034
title: "Un verdetto locale del ricevente puo' entrare in catena attraverso una firma di quorum"
status: open
category: "security"
severity: "high"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "REVIEW-033"
origin_ref: "RF-006"
related_specs: ["SPEC-019","SPEC-013"]
related_reviews: ["REVIEW-033"]
related_decisions: []
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-26
tags: ["security","consensus","identity"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-034-EVENT-001"
    timestamp: "2026-08-26T14:00:11.310748200+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead su finding di AGENT-007, ma cio' che lo rende importante e' come e' stato trovato, e l'implementatrice lo ha riconosciuto per prima.\n\nLa sua classificazione di identity.md:614 era corretta per il perimetro guardato: ha dimostrato che quella regola non e' una regola di validita' su un blocco, e ne ha concluso che quindi non produce due verdetti. La seconda cosa non segue dalla prima, e la strada per cui non segue - un verdetto locale riciclato in catena attraverso una firma di quorum - e' una forma che questo progetto non aveva mai visto.\n\nVale la pena tenerlo accanto all'errore che AGENT-007 ha commesso e ammesso nella stessa spec, perche' sono la stessa lezione da due lati. Lei ha motivato sul margine dove la proprieta' era disponibile; AGENT-002 ha dimostrato la proprieta' su un perimetro piu' stretto di quello vero e ha letto la dimostrazione come conclusiva. In entrambi i casi il ragionamento era valido e la conclusione no, e in entrambi i casi il difetto stava nel confine dell'argomento e non nell'argomento.\n\nLa regola che ne discende, e che vale oltre questo debito: una dimostrazione e' conclusiva solo quanto il perimetro su cui e' fatta, e il perimetro va dichiarato insieme alla dimostrazione."
    evidence_refs: []
---
# Un verdetto locale del ricevente puo' entrare in catena attraverso una firma di quorum

## Statement

SPEC-019 ha stabilito una definizione unica di "enrolled, unrevoked" per autorizzare una transazione, ancorata all'altezza del blocco che la include, e ha lasciato in piedi la seconda lettura di identity.md:614 - la revoca valutata all'altezza finalizzata **del ricevente** - classificandola come regola di accettazione locale su una connessione, che nessuno rigioca.

La classificazione e' corretta sulla validita' del blocco e **incompleta**: quella regola governa la raggiungibilita', e un verdetto sulla raggiungibilita' puo' **rientrare in catena**. Sul percorso della sfida, due auditor con altezze finalizzate diverse registrano `no_response` contro `passed` sullo stesso peer, e quell'esito finisce in un `challenge_evidence` **firmato a quorum**.

Un giudizio che dipende dalla vista locale di chi lo emette diventa cosi' un fatto di consenso.

> **Correzione del puntatore, 2026-08-26.** Il riferimento `identity.md:614` e' **morto**: a quella riga oggi c'e' il rinvio alla numerazione delle regole di rifiuto, dentro il pavimento su `now_ms`. La regola vera — *«no revocation exists at the receiver's finalized height»* — sta a **`identity.md:814-815`**, con la regola sulle connessioni gia' stabilite a `:825-831`. Le righe sono scivolate di duecento quando [SPEC-019] e [SPEC-020] hanno riscritto il documento. Il difetto e' reale e sta dove questo debito dice: e' il puntatore a essere scaduto. Trovato dal Lead riverificando prima di redigere [ADR-017].

## Evidence and provenance

Trovato da AGENT-007 in REVIEW-033 RF-006, attaccando la classificazione di R2 che AGENT-002 aveva dichiarato chiusa.

La catena del difetto e' verificabile in tre passi: identity.md:614 ancora la revoca all'altezza finalizzata del ricevente, quindi due riceventi con teste diverse danno verdetto diverso sulla stessa connessione; un peer rifiutato non risponde a una sfida; l'assenza di risposta e' registrata come esito e confluisce in un challenge_evidence autorizzato a quorum.

Nessuno dei tre passi e' un difetto per se'. E' la loro composizione a portare un giudizio locale dentro il consenso, ed e' la stessa forma - due chiusure corrette la cui composizione toglie una proprieta' - gia' vista in SPEC-017 fra DEBT-020 e DEBT-021.

## Impact and scope boundary

Il bersaglio e' l'integrita' dell'evidenza di sfida e, per il suo tramite, la reputazione che decide l'eleggibilita' a validatore.

Il danno non e' un fork sulla validita' di un blocco: e' peggio da diagnosticare, perche' **ogni passo e' conforme**. Il blocco che contiene il challenge_evidence e' valido; il quorum che lo firma non ha violato nulla; l'auditor che ha registrato no_response ha applicato correttamente la propria regola locale. Cio' che e' falso e' il fatto registrato, e nulla nella catena dice che dipendeva da chi guardava.

La finestra e' quella fra finalizzazione ed efficacia della revoca, che DEBT-033 stabilisce non avere tetto: le due questioni si compongono, e la seconda allunga la prima a piacere di chi revoca.

high e non medium perche' produce un fatto di consenso falso senza che alcuna regola sia violata, e perche' la superficie - la reputazione - e' quella su cui poggia l'eleggibilita' del set.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead su finding di AGENT-007, ma cio' che lo rende importante e' come e' stato trovato, e l'implementatrice lo ha riconosciuto per prima.

La sua classificazione di identity.md:614 era corretta per il perimetro guardato: ha dimostrato che quella regola non e' una regola di validita' su un blocco, e ne ha concluso che quindi non produce due verdetti. La seconda cosa non segue dalla prima, e la strada per cui non segue - un verdetto locale riciclato in catena attraverso una firma di quorum - e' una forma che questo progetto non aveva mai visto.

Vale la pena tenerlo accanto all'errore che AGENT-007 ha commesso e ammesso nella stessa spec, perche' sono la stessa lezione da due lati. Lei ha motivato sul margine dove la proprieta' era disponibile; AGENT-002 ha dimostrato la proprieta' su un perimetro piu' stretto di quello vero e ha letto la dimostrazione come conclusiva. In entrambi i casi il ragionamento era valido e la conclusione no, e in entrambi i casi il difetto stava nel confine dell'argomento e non nell'argomento.

La regola che ne discende, e che vale oltre questo debito: una dimostrazione e' conclusiva solo quanto il perimetro su cui e' fatta, e il perimetro va dichiarato insieme alla dimostrazione.

## Resolution criteria

Stabilire quale delle due deve cedere, e sono entrambe difendibili oggi: la regola di raggiungibilita' ancorata alla vista locale, che e' legittimamente piu' stretta e serve a proteggere il ricevente, oppure la registrazione di un esito di sfida che dipende da quella vista.

La via da esplorare per prima, perche' non tocca nessuna delle due: **l'evidenza di sfida potrebbe portare l'altezza a cui l'auditor ha giudicato**, cosi' che un verdetto diventi verificabile invece che asserito. Non toglie la discrezione, la rende ricalcolabile - che e' la stessa forma con cui SPEC-019 ha scelto fra le due letture di unrevoked.

> **Quella via e' stata esplorata, e non basta. Aggiornamento del 2026-08-26.**
>
> [ADR-017] l'aveva adottata come propria parte 3. AGENT-007 l'ha smontata in [REVIEW-036] con due finding `high`, e il Lead ha tolto la parte 3 dall'ADR. Quello che segue e' cio' che quella esplorazione ha stabilito, e sostituisce la raccomandazione qui sopra.
>
> **Primo — un campo solo su N firmatari cancella la divergenza invece di renderla visibile** ([REVIEW-036] RF-004). `ChallengeEvidenceBody` porta `auditor_signatures` come **lista**, con una soglia di auditor indipendenti, e `outcome` come **scalare**. L'intero difetto di questo debito e' che auditor a teste diverse concludono cose diverse: un campo di altezza singolo costringe N-1 auditor a firmare un'altezza che non e' la loro. Tre auditor firmano un `no_response`; uno solo era sotto la finestra; l'evidenza risulta internamente coerente e registra un fatto che due dei tre non hanno osservato. **Se il campo si aggiunge, va aggiunto per firma**, dentro `ValidatorSignature` o in una struttura parallela indicizzata come `auditor_signatures`.
>
> **Secondo — il controllo non e' verificabile come MUST, e come MAY guarda dalla parte sbagliata** ([REVIEW-036] RF-003). Per stabilire che un `no_response` e' sostenuto, un validatore dovrebbe ricostruire le revoche **finalizzate** all'altezza dichiarata, e `ledger.md:139-148` dichiara che la catena non contiene alcuna altezza a cui una revoca sia divenuta finale. Il validatore ricostruisce l'**inclusione**, non la finalizzazione. Renderlo un MUST **reintrodurrebbe un livello piu' in basso la lettura verificatore-dipendente che [SPEC-019] ha scartato**: due validatori con certificati diversi, verdetti opposti su una transazione. Renderlo un MAY lascia il controllo con potere in **una direzione sola** — puo' smentire un `passed`, non un `no_response`, perche' la mancata risposta ha cause innocenti illimitate.
>
> **Terzo, ed e' la superficie che nessuna delle due letture di questo debito nominava.** La direzione ostile resta interamente scoperta: un validatore emette challenge verso un concorrente, non apre la connessione, registra `no_response`, dichiara un'altezza qualunque. L'evidenza passa ogni controllo di ricalcolabilita' ed e' falsa nel fatto. Il concorrente non accumula `contribution_score`, che somma **solo** gli `outcome:"passed"` di kind `storage` e `compute`, e fallisce l'eleggibilita'. **Guadagno: un seggio, senza violare alcuna regola.** Nulla lega l'altezza dichiarata a `completed_at_ms`, e nessuna regola in v0 mappa millisecondi ad altezze.
>
> **Quarto — la composizione con [DEBT-033] va ridimensionata** ([REVIEW-036] RF-007). Questo debito afferma che la finestra si allunga a piacere di chi revoca e che chiudere [DEBT-033] la accorcia. E' **vero nella direzione e sopravvalutato nell'ampiezza**: la parte 1 di [ADR-017] stacca il percorso di spesa, ma la **raggiungibilita' resta saldata al pavimento**, e per un nodo **non validatore** con `key_compromise` non esiste alcun set a cui dare una finestra. Quel nodo resta irraggiungibile-ma-iscritto per `min_revocation_effective_delay_blocks` blocchi, con un parametro tenuto lungo per progetto e tarato su tutt'altro.
>
> **Cosa serve davvero, quindi.** La chiusura di questo debito richiede che l'evidenza porti **cosa l'auditor ha osservato** e non solo quando, perche' e' l'unica forma che rende un verdetto **contestabile** invece che ricalcolabile — e la ricalcolabilita' non morde l'auditor ostile, che e' la superficie peggiore. Questo e' lavoro di progettazione del percorso di sfida, che **non esiste ancora in codice**: `no_response` e `auditor` non compaiono in `core/coblox-core/src/`. **La condizione giusta non e' una data ma l'esistenza di quel percorso**: il rimedio va deciso quando il percorso di sfida viene implementato, non prima, perche' prima si sceglierebbe una forma senza sapere cosa un auditor puo' onestamente osservare.

Il rimedio apparente da non adottare: allineare identity.md:614 alla definizione di SPEC-019. Renderebbe la regola di raggiungibilita' dipendente dagli antenati di un blocco che il ricevente potrebbe non avere ancora, cioe' toglierebbe al ricevente la capacita' di proteggersi in tempo reale - che e' la ragione per cui quella regola e' ancorata alla propria vista. Sarebbe una chiusura che sposta il danno sul livello di trasporto.

Da valutare insieme a DEBT-033, perche' la finestra che entrambi sfruttano e' la stessa.

## Resolution evidence

