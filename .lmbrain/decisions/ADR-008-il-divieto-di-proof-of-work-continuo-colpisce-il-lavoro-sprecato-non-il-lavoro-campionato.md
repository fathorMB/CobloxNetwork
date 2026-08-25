---
id: ADR-008
# Note: Quote the title if it contains a colon
title: "Il divieto di proof-of-work continuo colpisce il lavoro sprecato, non il lavoro campionato"
status: accepted
decision_date: 2026-08-25
decider: AGENT-LEAD
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: [ADR-002, ADR-004, ADR-005, ADR-007]
tags: [architecture, security]
created: 2026-08-25
updated: 2026-08-25
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned proposed -> accepted"
---
# Il divieto di proof-of-work continuo colpisce il lavoro sprecato, non il lavoro campionato

> Riapertura chiesta dall'operatore il 2026-08-25, contestualmente alla conferma di [ADR-007]. Questa ADR **non abroga** l'esclusione: la rende precisa e verificabile.

## Context

`PROJECT.md` esclude, in una riga secca in fondo all'elenco delle esclusioni, «Mining/proof-of-work continuo di qualsiasi tipo». A differenza dell'esclusione sulla convertibilità in denaro, questa non porta la qualifica «permanente, di principio» e non compare fra i vincoli: ha il peso di un'esclusione di scope.

La riga è entrata in conflitto con sé stessa nel momento in cui [ADR-007] ha ancorato l'eleggibilità a validatore a **storage e compute dimostrati**, esplicitamente mai al solo uptime. Dimostrare storage e compute in modo continuativo *è* lavoro continuo. [ADR-002] lo prescrive già nel dettaglio: proof-of-retrievability su campioni casuali dei blocchi custoditi, ri-esecuzione a campione dei task WASM. La formula «di qualsiasi tipo» non distingue quel lavoro dal mining, e quindi, letta alla lettera, il protocollo che il progetto sta costruendo viola una propria esclusione dichiarata.

La contraddizione non è mai stata scritta perché nessuno l'ha messa alla prova: l'esclusione è stata redatta pensando a Bitcoin, non alle challenge di [ADR-002]. Ma un'esclusione che il progetto viola di fatto non protegge nulla, e peggio: rende impossibile respingere una proposta futura che davvero la violi, perché il precedente è già in casa.

Il vincolo confinante di `PROJECT.md` — «i nodi Android devono rispettare batteria/dati» — indica dove sta la preoccupazione reale. Il threat model l'ha già incontrata e risolta in forma verificabile: `TM-05` documenta che l'imprevedibilità delle challenge impedisce a un nodo Android di programmare i risvegli e «costa batteria per davvero», e prescrive come composizione ragionevole «una finestra di risposta ampia (minuti, non secondi) con istante di emissione imprevedibile, così il dispositivo può accorpare i risvegli senza poterli anticipare» (`SEC-REQ-17`).

## Decision

La riga «Mining/proof-of-work continuo di qualsiasi tipo» è sostituita in `PROJECT.md` da una formulazione che dichiara **cosa** l'esclusione colpisce, con un test in tre parti che decide i casi nuovi senza doverli enumerare.

**Il principio.** L'esclusione colpisce il lavoro **il cui costo è il meccanismo**: quello in cui bruciare risorse *è* l'evidenza, per cui il partecipante razionale scala la spesa senza limite e il budget di sicurezza della rete coincide con la sua dissipazione. Non colpisce il lavoro **il cui prodotto è il meccanismo**: quello in cui la risorsa sarebbe stata spesa comunque per servire la rete — custodire un blocco, eseguire un task WASM — e la challenge si limita a *campionarlo*. Nel primo caso il costo è il mezzo; nel secondo è una conseguenza.

**Il test.** Una proposta di lavoro ricade nell'esclusione se soddisfa anche una sola delle prime due condizioni, e non è ammissibile su nodi mobili se non soddisfa la terza.

1. **Limite.** Esiste un tetto posto dal fabbisogno reale della rete oltre il quale spendere di più non fa guadagnare di più? I blocchi da custodire e i task da eseguire sono in numero finito e determinato dalla domanda; gli hash da calcolare no. **Se un nodo può guadagnare di più spendendo di più senza che la rete abbia bisogno di più, è mining.**
2. **Spreco.** Se quel lavoro smettesse di essere svolto, un servizio visibile all'utente si degraderebbe? Se la risposta è no, il lavoro era il fine e non il mezzo, e ricade nell'esclusione.
3. **Batteria.** Il lavoro è campionabile entro una finestra di risposta ampia, che consenta a un dispositivo mobile di accorpare i risvegli senza poter anticipare l'istante di emissione? È `SEC-REQ-17`, già verificabile con un test statistico sul log delle challenge di una devnet. Un lavoro che pretende presenza continua a bassa latenza non è ammissibile su nodi mobili, indipendentemente dall'esito dei punti 1 e 2.

**Conseguenza diretta.** Le challenge di [ADR-002] e l'ancoraggio a storage e compute di [ADR-007] sono ammessi, e lo sono ora per iscritto. Il mining resta escluso, e resta escluso *per una ragione dichiarata* invece che per enumerazione.

**Ciò che questa ADR non fa.** Non aumenta di per sé la resistenza ai Sybil. Il lavoro che il test ammette è limitato dal fabbisogno della rete, e un lavoro limitato non può prezzare un flusso perpetuo più di quanto potesse un costo una tantum: l'affermazione strutturale di [ADR-007] resta intatta. Il contributo alla leva anti-Sybil è indiretto ma reale: rende legittimo ancorare a storage e compute una **quota maggiore** dell'emissione, e la quota residua che passa dal reddito di esistenza è esattamente `α`, il parametro che [ADR-007] identifica come quello che governa la resistenza. Questa ADR non abbassa `α`: rimuove l'ostacolo formale che rendeva contraddittorio abbassarla.

## Alternatives considered

- **Lasciare la riga com'è.** Scartata: il progetto la viola già di fatto con [ADR-002] e [ADR-007]. Un'esclusione violata in casa non può essere opposta a nessuno.
- **Abrogare il divieto.** Sarebbe la massima leva anti-Sybil disponibile una volta escluso il proof-of-stake, perché il proof-of-work continuo è la difesa efficace contro un flusso perpetuo di identità. Scartata su indicazione dell'operatore, e comunque in conflitto frontale con il vincolo su batteria e dati dei nodi Android: una rete che chiede mining ai telefoni perde i telefoni, cioè perde il proprio parco nodi caratteristico.
- **Enumerare le eccezioni ammesse** («escluso il mining, ammessi proof-of-retrievability e ri-esecuzione WASM»). Scartata perché non decide i casi nuovi: ogni primitiva futura richiederebbe una nuova ADR per essere classificata, e l'elenco invecchierebbe in silenzio come ha fatto la riga originale.
- **Distinguere per nome della primitiva** anziché per proprietà. Scartata: la stessa primitiva crittografica può stare da entrambe le parti a seconda di come è impiegata. La proprietà che conta è se il costo è il meccanismo, non quale funzione hash lo produce.

## Consequences

- `PROJECT.md` cambia: la riga dell'elenco delle esclusioni va sostituita dal principio, e il test va riportato per esteso, perché un principio senza test è decorazione.
- Il test diventa un criterio di review vincolante. Ogni ADR o spec futura che introduca una nuova forma di lavoro remunerato deve dichiarare esplicitamente l'esito dei tre punti, ed è materia di [REVIEW] verificarlo.
- `SEC-REQ-17` acquista un secondo ruolo. Nato come contromisura a `TM-05`, diventa anche il criterio di ammissibilità su nodi mobili: non è più solo una proprietà desiderabile della schedulazione, è la condizione che separa il lavoro proponibile su Android da quello che non lo è.
- Il punto 1 del test vincola la specifica di elezione dei validatori di M-02, già gravata da [DEBT-005]: l'eleggibilità ancorata a storage e compute deve poter dimostrare di avere un tetto di fabbisogno, altrimenti reintroduce per la porta di servizio la corsa alla spesa che l'esclusione vieta.
- Il progetto acquista una risposta pubblica difendibile alla domanda «è proof-of-work?», che su un repository pubblico verrà posta. La risposta è: sì per campionamento di lavoro utile, no per dissipazione — e c'è un test scritto che chiunque può applicare invece di doversi fidare.

## Review conditions

Rivedere se: una primitiva utile viene respinta dal test pur non essendo dissipativa, il che indicherebbe che il test è formulato troppo stretto e non che la primitiva è sbagliata; il simulatore di M-02 mostra che il fabbisogno di storage e compute non basta a tenere `α` bassa, nel qual caso il nodo da riaprire non è questa ADR ma l'esclusione sulla convertibilità o la forma del reddito di esistenza in [DEBT-007]; oppure se il benchmark su parco dispositivi reale mostra che nemmeno una finestra di risposta ampia rende sostenibile per la batteria la partecipazione a storage e compute, che invaliderebbe il punto 3 come criterio praticabile invece che come criterio corretto.
