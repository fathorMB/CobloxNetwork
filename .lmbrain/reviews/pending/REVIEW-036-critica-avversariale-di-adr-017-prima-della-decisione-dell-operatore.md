---
id: REVIEW-036
# Note: Quote the title if it contains a colon
title: "Critica avversariale di ADR-017 prima della decisione dell'operatore"
status: pending
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-019
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-LEAD
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security, correctness, documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-036-EVENT-001"
    timestamp: "2026-08-26T22:11:09.329896400+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-XXX"
links: [DEBT-033, DEBT-034]
created: 2026-08-26
updated: 2026-08-26
tags: [security, review, consensus]
related_decisions: [ADR-017]
activity:
  - date: 2026-08-26
    action: "created"
---
# Review

> **L'oggetto di questa review è [ADR-017], non l'implementazione di [SPEC-019].** Il campo `spec` punta a [SPEC-019] perché lo schema di review lo richiede e perché è la spec da cui [DEBT-033] e [DEBT-034] nascono, non perché sia il codice sotto esame. Richiesta dall'operatore il 2026-08-26: *«voglio una critica avversariale prima»*, cioè prima di decidere l'ADR. L'ADR era in stato `proposed` e non è mai stato accettato nella forma qui criticata.

## Outcome

**L'ADR regge con modifiche.**

- **Parte 1** (il percorso di spesa perde il pavimento; la revoca morde all'altezza di inclusione): **sopravvive a ogni attacco portato**.
- **Parte 2** (tetto dipendente da `reason`; `effective_height` derivato su `key_compromise`): **va rifatta**. È inapplicabile come scritta e non toglie la discrezione, la sposta su un parametro che nessun ancoraggio di genesi limita.
- **Parte 3** (`challenge_evidence` porta l'altezza dell'auditor): **va declassata** da rimedio a dichiarazione.

## Acceptance-criteria compliance

Non applicabile: l'oggetto è una decisione proposta, non una consegna con criteri di accettazione.

## Code observations

**Il percorso di sfida non esiste in codice.** `no_response` e `auditor` non compaiono in `core/coblox-core/src/`; l'unica traccia è il dominio di firma in `hash.rs`. Ogni affermazione di questa review su inclusione, censura e ordinamento è **dedotta dai documenti e non verificata contro un'implementazione**. RF-002 e RF-006 in particolare descrivono comportamenti che nessun codice esibisce oggi.

`core/coblox-core/src/authorization.rs`: `RevocationRecord` dichiara nel proprio commento che l'altezza di **inclusione** è *«deliberately absent: the predicate does not read it»*. La parte 1 la renderebbe la sola grandezza che il predicato legge.

## Tests and verification

La reviewer **non ha eseguito** la suite né `published_artifacts.py`. Le affermazioni su `AUTH-0` sono lette dal documento e dal file di test, non ottenute facendo fallire un test.

**Il Lead ha riverificato in modo indipendente sei affermazioni fattuali di questa review**, tutte confermate: `ledger.md:785` come terzo MUST in grafia alternativa; `ledger.md:107` con la stessa forma della clausola 8; l'assenza di `min_revocation_effective_delay_blocks` dal blocco dei vincoli di genesi; il secondo lavoro del pavimento a `ledger.md:1075-1080`; l'ordine di transizione a `ledger.md:2819`; le righe `21` e `49` di `AUTH-0` marcate `valid`.

## Production quality and documentation compliance

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security | severity=high | criterion=ADR-017 parte 2 | remediation=portare pavimento e tetto nel blocco dei vincoli di `ledger.md#magnitudes-not-only-relations` con un limite preso dall'ancora di genesi
**La parte 2 non toglie la discrezione: la sposta su un parametro che lo stesso quorum firma.** `min_revocation_effective_delay_blocks` è in `ConsensusParametersBody` e **non** è nel blocco dei vincoli di magnitudine né in `ElectionBounds`. Scenario: un quorum oltre i due terzi che vuole latitudine su un `key_compromise` non tocca `effective_height` — la parte 2 glielo deriva — e pubblica un `consensus_parameters` con il pavimento a `2^40`. Ogni vincolo relazionale è soddisfatto, il documento è accettato, e da quel momento ogni `key_compromise` ha un'efficacia derivata, verificabile e cosmetica. **È la famiglia 3 alla lettera**: vincolata la grandezza nominata, non quella da cui la proprietà dipende. Aggravante: alzare il pavimento **autorizza** ad allungare `max_weak_subjectivity_age_ms`, quindi è una leva a due teste. *Perimetro: i parametri di consenso di v0 a `3f1bef7`.*

RF-002 | category=security | severity=high | criterion=ADR-017 parte 2 | remediation=disuguaglianza a due lati con margine dichiarato, oppure `effective_height` omesso dal corpo e calcolato all'inclusione
**`effective_height = proponente + pavimento` esattamente è inapplicabile: l'autore non conosce l'altezza di inclusione.** Oggi il vincolo è una disuguaglianza, quindi uno slittamento dell'inclusione è innocuo. Trasformarlo in uguaglianza rende la transazione invalida a ogni blocco di ritardo. Scenario: il proponente del turno — il validatore compromesso stesso, o un alleato, per **un solo turno** — omette la revoca; al blocco successivo `effective_height` non corrisponde più e il quorum deve riraccogliere oltre due terzi di firme, contro una transazione che porta `expires_at_ms`. **Converte TM-15, censura di severità media, in un veto sulla transazione d'emergenza.** *Perimetro: la sola riga `key_compromise`; le altre due sono tetti e non soffrono di questo.*

RF-003 | category=security | severity=high | criterion=ADR-017 parte 3 | remediation=dichiarare se il rifiuto è MUST o MAY, e dichiarare che la parte 3 non riduce la superficie dell'auditor ostile
**La parte 3 è teatro contro l'auditor ostile e fork contro quello onesto.** Se il rifiuto è un **MUST**: per stabilire che un `no_response` è sostenuto, un validatore deve ricostruire le revoche **finalizzate** all'altezza dichiarata, e `ledger.md:139-148` dichiara che la catena non contiene alcuna altezza a cui una revoca sia divenuta finale. Il validatore ricostruisce l'inclusione, non la finalizzazione — **quindi la regola reintrodurrebbe, un livello più in basso, la lettura verificatore-dipendente che [SPEC-019] ha scartato**. Se è un **MAY**: il controllo ha potere in una sola direzione, può smentire un `passed` e non un `no_response`, perché la mancata risposta ha cause innocenti illimitate. Scenario nella direzione scoperta: un validatore emette challenge verso un concorrente, non apre la connessione, registra `no_response`, dichiara la propria testa; l'evidenza è ricalcolabile su ogni controllo della parte 3 e falsa nel fatto; il concorrente non accumula `contribution_score` e fallisce l'eleggibilità. **Guadagno: un seggio, senza violare alcuna regola.**

RF-004 | category=correctness | severity=high | criterion=ADR-017 parte 3 | remediation=altezza per firma dentro `ValidatorSignature` o in struttura parallela, oppure rinunciare a chiamare il verdetto ricalcolabile
**La parte 3 porta un'altezza sola a un oggetto firmato da N auditor con viste diverse.** `ChallengeEvidenceBody` porta `auditor_signatures` come **lista** con una soglia di auditor indipendenti, e `outcome` come **scalare**. L'intero difetto di [DEBT-034] è che auditor a teste diverse concludono cose diverse: un campo singolo costringe N−1 auditor a firmare un'altezza che non è la loro. Scenario: tre auditor firmano un `no_response`; uno solo era sotto la finestra, gli altri due avrebbero visto `passed`; l'evidenza è internamente coerente e registra un fatto che due dei tre firmatari non hanno osservato. **La divergenza non diventa visibile nell'oggetto: viene cancellata da esso.**

RF-005 | category=security | severity=high | criterion=ADR-017, composizione delle parti 1 e 2 | remediation=ammettere un margine di programmazione su `key_compromise`, oppure dichiarare che si sceglie lo stallo sopra la latitudine
**La proprietà che cade dalla composizione: la parte 2 crea l'incentivo a ritardare l'autorizzazione, che è ciò che la parte 1 esisteva per impedire.** `ledger.md:1085-1091` dichiara che se i superstiti non impegnano un set conforme entro la finestra la catena si ferma. Oggi il quorum ha una leva per allargarla: `effective_height`. La parte 2 gliela toglie **proprio su `key_compromise`**, la sola riga in cui lo stallo è plausibile. Restano due mosse, entrambe cattive: autorizzare e accettare lo stallo, oppure **ritardare l'autorizzazione** per coordinare fuori catena — e durante il ritardo la chiave compromessa spende ancora, perché la parte 1 morde all'inclusione e inclusione non ce n'è. In alternativa il quorum dichiara `operator_request` per una chiave davvero compromessa. **Su `reason` la conseguenza è peggiore dell'inerzia:** l'ADR lo rende letto, quindi un `reason` misdichiarato diventa un fatto di consenso falso, e la regola premia la misdichiarazione. Contro un quorum pienamente ostile la questione è vuota — chi ha i due terzi non revoca e basta. **Contro il quorum onesto `reason` funziona oggi e l'ADR lo romperebbe: non è teatro, è un rimedio con l'incentivo invertito.**

RF-006 | category=security | severity=medium | criterion=ADR-017 parte 1, dettaglio rinviato alla spec | remediation=valutare la qualificazione contro lo stato pre-blocco per le transazioni di classe 0, oppure ordinare `revoke_identity` per tipo e non per ID
**Dentro la classe 0 l'ordine è per ID di transazione, che il revocante può macinare.** `ledger.md:2821-2824` mette `challenge_commitment`, `challenge_evidence`, `revoke_identity` e `validator_candidacy` in classe 0, ordinati per raw transaction ID. L'ID è l'hash del corpo, e il corpo porta `created_at_ms` ed `expires_at_ms`: il revocante può enumerare millisecondi finché il proprio ID ordina prima o dopo quello della candidatura bersaglio. Deterministico per ogni verificatore, quindi **non è un fork**, ma è una **scelta** del revocante — cioè la discrezione che la parte 1 dichiara di aver tolto, riapparsa come indice nel blocco. *Perimetro: non vale sul saldo, perché `burn` e `fund_app` sono classe 1 e stanno sempre dopo.*

RF-007 | category=correctness | severity=medium | criterion=ADR-017, argomento portante | remediation=dichiarare i tre percorsi e quale altezza governa ciascuno
**La parte 1 stacca due percorsi su tre: la raggiungibilità resta saldata al pavimento.** L'argomento *«il pavimento è arbitrario dove la sua ragione non esiste»* vale con la stessa forza sul terzo percorso e l'ADR non lo applica lì. Per un nodo **non validatore** con `key_compromise` non esiste alcun set a cui dare una finestra, eppure la parte 2 gli imporrebbe un'efficacia a `proponente + pavimento`, con un pavimento tenuto lungo per progetto: resta irraggiungibile-ma-iscritto per tutta la durata di un parametro tarato su altro. **L'affermazione dell'ADR che quella metà di [DEBT-034] si accorcia per composizione è vera nella direzione e sopravvalutata nell'ampiezza.**

RF-008 | category=documentation | severity=medium | criterion=ADR-012, enumerazione delle conseguenze | remediation=la passata di [ADR-012] sulla spec attuativa enumera `AUTH-0`, `authorization.rs`, il checkpoint e `ledger.md:785`
**L'enumerazione delle conseguenze omette gli artefatti pubblicati che la parte 1 rende falsi.** `AUTH-0` è il più caro: la revoca della fixture è finalizzata a `20` con efficacia `50`, e sotto la parte 1 le righe `21` e `49` **si ribaltano** da `valid` a `invalid` — precisamente le due righe che la fixture esiste per pinnare. È un artefatto pubblicato con un test di conformità dedicato, e l'ADR non lo nomina. Inoltre: il commento di `RevocationRecord` va **ritrattato** e non aggiornato in silenzio; e il checkpoint impegna `(node_id, effective_height)`, cioè dopo la parte 1 porta la grandezza del percorso del set e non quella del saldo — corretto per il light client, e va detto, perché un consumatore futuro leggerebbe quel campo come *«da quando la chiave non spende»* e sbaglierebbe.

RF-009 | category=correctness | severity=medium | criterion=ADR-017 parte 2, riga `validator_misconduct` | remediation=parametro proprio con denominatore dichiarato, oppure fondere le due righe
**Il `2 ×` non ha misura e ha il denominatore sbagliato.** È un multiplo di una grandezza tarata su tutt'altro: quanti blocchi servono ai superstiti per impegnare un set successore. Il margine di programmazione legittimo di una cattiva condotta non ha relazione con quella durata. Conseguenza: se la governance alza il pavimento per rendere lo stallo più raro, **raddoppia automaticamente la latitudine sulla cattiva condotta**.

RF-010 | category=correctness | severity=low | criterion=ADR-017 parte 2, campo derivato | remediation=togliere il campo se sopravvive l'uguaglianza; la domanda decade se si adotta la disuguaglianza di RF-002
**`effective_height` derivato non è una guardia, è una superficie.** Un campo derivato che resta nel corpo va ricalcolato **e** confrontato da ogni verificatore, cioè due modi di sbagliare invece di uno, ed è impegnato nell'ID della transazione. Come ridondanza non porta nulla: non c'è una seconda fonte, solo la stessa formula applicata due volte.

## Errori fattuali negli artefatti sotto esame

**E-01 — falso, e con conseguenza materiale.** *«`effective_height` è nominato da esattamente due MUST, per esaurimento delle trenta»*. Le trenta occorrenze sono reali e le tre righe con `MUST` sono `176`, `1033`, `1064`, di cui la `176` è prosa. **Ma l'enumerazione è fatta sul token e la grandezza ha una seconda grafia**: `ledger.md:785` recita *«The effective height MUST be later than the block proposing the revocation»*, ed è la sola regola che impedisce a una revoca di mordere nel proprio blocco — cioè la riga che la parte 1 tocca più da vicino. **Conseguenza:** se la governance fissa il pavimento a `0` — permesso, perché nessun vincolo lo impedisce — `key_compromise` deriverebbe `effective_height = proponente`, che **viola la riga 785**, e ogni revoca per compromissione diventerebbe incostruibile. *L'errore non nasce nell'ADR: è in [DEBT-033] e prima ancora in [REVIEW-033] RF-001, cioè in tre artefatti, e la riverifica ha usato lo stesso strumento che l'aveva prodotto.*

**E-02 — non sostenuto.** *«la clausola 8 è la sola che pone `effective_height` sotto un'altra grandezza»*. La classificazione lascia fuori `ledger.md:107`, che è la clausola 2 della definizione di [SPEC-019] e recita `carries an effective_height at most h`, sintatticamente la stessa forma. La **conclusione** sopravvive — nessuna delle due è un tetto sul campo — ma il superlativo non è sostenuto dall'enumerazione portata.

**E-03 — falso, e tocca la premessa portante.** *«Il pavimento ha una giustificazione, e quella esiste su un percorso solo»*. `ledger.md:1075-1080` gli assegna un **secondo** lavoro con un MUST: fare da tetto a `max_weak_subjectivity_age_ms`, cioè alla freschezza dell'ancora di fiducia del light client. La conclusione della parte 1 sopravvive — nessuno dei due lavori è una ragione per mettere il pavimento sul **saldo** — ma la premessa va riscritta come *«nessuno dei lavori del pavimento riguarda il saldo»*, che è enumerabile e vero. **Secondo ordine:** poiché la parte 2 rende il pavimento l'unica leva, e il pavimento è legato per MUST alla finestra del light client, l'ADR salderebbe la finestra di revoca alla finestra di esposizione del light client, che prima si muovevano separatamente.

**E-04 — questione dichiarata aperta che il documento aveva già chiuso.** L'ADR rinvia alla spec la scelta fra mordere a `h` o a `h+1`. `ledger.md:2819` **esiste già** e mette `revoke_identity` in classe 0 e `burn`/`fund_app` in classe 1: sul percorso del saldo l'ordinamento è già totale, già deterministico, e già favorevole a `h`. Dichiarare aperta una questione chiusa **distoglie dal punto in cui l'ordinamento è davvero indeterminato**, che è dentro la classe 0 (RF-006).

**E-05 — non materiale.** Sul perimetro dichiarato i match di `min_revocation_effective_delay_blocks` sono 18 e non 17: il diciottesimo è in `sim/tools/__pycache__/`. Artefatto di build. Riportato perché su questo progetto un'enumerazione si consegna con il perimetro, e quello dichiarato non è quello su cui il conto è stato fatto.

## Cosa la reviewer ha attaccato senza riuscire a romperlo

Obbligatorio su questo progetto, e qui è la parte che porta più informazione del verdetto.

1. **La parte 1 non è un fork.** L'inclusione è posizione in `transactions_root`, impegnato nell'header, quindi è un fatto del blocco. Le riorganizzazioni non lo rompono: una catena diversa è un blocco diverso. La mempool non entra nel predicato. Non esiste inclusione condizionale in v0.
2. **L'ordinamento intra-blocco sul saldo non è indeterminato** — ed era l'attacco che la reviewer riteneva migliore. `ledger.md:2819` mette la revoca in classe 0 e la spesa in classe 1: una revoca inclusa in `h` esegue prima di ogni spesa dello stesso blocco, per ogni verificatore.
3. **La non retroattività non si rompe.** `effective_height` MUST essere strettamente maggiore del blocco proponente, quindi l'inclusione è sempre **minore** dell'efficacia: la parte 1 anticipa il morso, non lo retrodata. Nessuna firma valida ieri diventa invalida oggi.
4. **La censura dell'inclusione non sostituisce la discrezione sul campo.** Oggi un quorum sceglie una finestra illimitata con una firma sola; con la parte 1 un censore guadagna **un blocco per turno di proposta che controlla**, cioè un danno per-turno invece che illimitato. *Perimetro: vale sul solo percorso di spesa; sul percorso del set l'attacco riesce, ed è RF-001.*
5. **La monotonia sotto revoche multiple.** Il predicato *«esiste una revoca inclusa a un'altezza al più `h`»* è monotono in `h` per costruzione, e più revoche possono solo anticipare la prima soglia.
6. **L'equivalenza `min(effective_height, proponente + 0) ≡ inclusione`**, dichiarata dall'ADR, è corretta: poiché l'efficacia è strettamente maggiore dell'inclusione, il `min` vale sempre l'inclusione.
7. **Le enumerazioni quantitative su `min_revocation_effective_delay_blocks` e su `reason`** — ripartizione per file e valori assegnati — sono state rieseguite e reggono, salvo la nota di perimetro E-05.

## Cosa la reviewer dichiara di non aver guardato

- Il codice di consenso, mempool e proposta: **non esiste**.
- `app-manifest.md`, che la definizione di [SPEC-019] raggiunge e che la parte 1 toccherebbe di conseguenza.
- Quali byte coprano le firme di auditor: RF-004 assume che un campo aggiunto al corpo entri nella firma.
- L'ipotesi che una revoca di massa renda insoddisfacibile la copertura a due issuer: **dedotta e non misurata**, segnalata come ipotesi e non come finding.
- **La clausola 4 dell'ADR** (quale versione del parametro governa). Dichiarazione testuale della reviewer: l'ha letta, le è parsa ben posta, e **non l'ha attaccata** — annotando che ciò che si loda è precisamente ciò che si smette di verificare.

## Required follow-up

- **Parte 1**: conservata. La premessa va riscritta secondo E-03, `ledger.md:785` va nell'elenco delle righe da riscrivere, e RF-008 fissa la passata di [ADR-012].
- **Parte 2**: rifatta secondo RF-001, RF-002, RF-005 e RF-009.
- **Parte 3**: tolta dall'ADR. RF-003, RF-004 e RF-007 diventano la sostanza riscritta di [DEBT-034].
- **RF-006**: non ha casa nell'ADR e va aperto come debito proprio.
- **E-01 va corretto anche in [DEBT-033] e segnalato su [REVIEW-033]**, dove nasce.
