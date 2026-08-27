---
id: DEBT-028
title: "election_epoch dipende da un parametro governato senza che il documento dica quale versione valga"
status: planned
category: "correctness"
severity: "high"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: "SPEC-017"
origin_ref: "elenco delle derivazioni non univoche, voce aperta"
related_specs: ["SPEC-016","SPEC-017","SPEC-006"]
related_reviews: []
related_decisions: ["ADR-012"]
target_specs: ["SPEC-027"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-27
tags: ["consensus","governance","light-client"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: Il debito chiede che il documento dica quale documento di parametri governa un'epoca di elezione, con una regola che un verificatore che rigioca la catena possa applicare. E' la stessa domanda che SPEC-027 deve risolvere per poter fissare i limiti: senza sapere quale versione dei parametri vale a una data altezza, un limite di genesi non ha un oggetto su cui mordere.\n\nInstradato qui e non su SPEC-025 perche' e' governance dei parametri e non meccanica del consenso: SPEC-025 usa la risposta, non la produce."
debt_events:
  - schema_version: "1"
    id: "DEBT-028-EVENT-001"
    timestamp: "2026-08-26T10:43:49.844637600+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead su segnalazione di AGENT-001, che lo ha trovato facendo il lavoro che la spec chiedeva — l'elenco delle derivazioni non univoche — e non il lavoro che la spec nominava.\n\nVale la pena registrarlo come conferma di una scommessa: [SPEC-017] chiedeva quell'elenco «anche se vuoto», e il Lead aveva scritto di aspettarsene piu' che dalla fixture, sulla base di [SPEC-010], dove l'elenco delle preimmagini scoperte valse piu' della fixture che quella spec doveva aggiungere. E' andata di nuovo cosi': la fixture di `chain_id` era il lavoro, questo debito e' il ritrovamento.\n\nE' inoltre la **terza porta** sullo stesso difetto: [DEBT-012] era un valore che entra in una preimmagine e che nessun documento fissa, [DEBT-020] era la circolarita' di `chain_id`, e questo e' un denominatore di epoca che dipende da quando lo si legge. La famiglia e' sempre quella: **una preimmagine i cui ingressi non sono derivabili in un solo modo**."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-028-EVENT-002"
    timestamp: "2026-08-27T15:05:00.468508400+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Il debito chiede che il documento dica quale documento di parametri governa un'epoca di elezione, con una regola che un verificatore che rigioca la catena possa applicare. E' la stessa domanda che SPEC-027 deve risolvere per poter fissare i limiti: senza sapere quale versione dei parametri vale a una data altezza, un limite di genesi non ha un oggetto su cui mordere.\n\nInstradato qui e non su SPEC-025 perche' e' governance dei parametri e non meccanica del consenso: SPEC-025 usa la risposta, non la produce."
    evidence_refs: ["SPEC-027"]
---
# election_epoch dipende da un parametro governato senza che il documento dica quale versione valga

## Statement

`election_boundary_height(e) = e * L`, dove `L` e' `election_epoch_blocks` preso «dai parametri di consenso attivi». **Il documento non dice attivi a quale altezza**, ne' quale documento di parametri valga per un'epoca passata.

`L` e' un parametro governato. Portandolo da 100 a 200, l'altezza 5000 e' epoca 50 sotto un documento e epoca 25 sotto l'altro. E `election_epoch` entra in `election_entropy`, `election_seed` ed `election_ticket`: due verificatori che leggono documenti di parametri diversi derivano preimmagini diverse per gli stessi oggetti, e il passo 2 del light client da' verdetti opposti sullo stesso set di validatori.

Trovato da AGENT-001 durante [SPEC-017], costruendo l'elenco delle derivazioni non univoche che quella spec chiedeva. E' l'unica voce dell'elenco rimasta aperta.

## Evidence and provenance

`.lmbrain/knowledge/derivazioni-non-univoche.md`, prodotto da [SPEC-017]: cinquantuno preimmagini classificate per ingresso, di cui cinque chiuse dentro quella spec e questa lasciata aperta con la ragione.

**La forma e' gia' stata riconosciuta e chiusa altrove, ed e' questo che rende il difetto accertato invece che sospetto.** [SPEC-016] ha chiuso lo stesso problema per `reward_epoch` **nominando il documento**: un mint dice quale `reward_policy` lo governa attraverso il proprio `policy_hash`, quindi il denominatore dell'epoca non e' ambiguo. **Nessun oggetto dell'elezione porta quella cucitura.**

AGENT-001 si e' fermato invece di chiuderlo, correttamente: sono tre regole di validita' nuove sull'elezione, quindi una passata di [ADR-012] propria e una decisione del Lead.

## Impact and scope boundary

Il bersaglio e' l'integrita' del set di validatori e la sua verificabilita' da parte di un light client, cioe' due degli asset piu' alti del threat model.

**La gravita' non e' nella spesa ma nel disaccordo**: due implementazioni entrambe conformi al documento come e' scritto oggi possono attribuire lo stesso blocco a epoche diverse, derivare `election_seed` diversi, e concludere in modo opposto sulla validita' di un set. E' la stessa forma di [DEBT-022] — clausola che ammette due letture, divergenza sulla validita' — applicata al livello che decide **chi puo' firmare i blocchi** invece che a una spesa.

Non richiede alcun attaccante. Basta che `election_epoch_blocks` sia cambiato **una volta** nella storia della catena, che e' un atto di governance previsto e legittimo, perche' ogni epoca precedente al cambio diventi ambigua.

`high` e non `critical` perche' nessuna rete esiste e nessun documento di parametri e' mai stato modificato in produzione; ma e' la classe piu' economica da correggere adesso, quando e' una clausola, e la piu' cara dopo, quando sara' una catena con una storia e due implementazioni che non concordano su di essa.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead su segnalazione di AGENT-001, che lo ha trovato facendo il lavoro che la spec chiedeva — l'elenco delle derivazioni non univoche — e non il lavoro che la spec nominava.

Vale la pena registrarlo come conferma di una scommessa: [SPEC-017] chiedeva quell'elenco «anche se vuoto», e il Lead aveva scritto di aspettarsene piu' che dalla fixture, sulla base di [SPEC-010], dove l'elenco delle preimmagini scoperte valse piu' della fixture che quella spec doveva aggiungere. E' andata di nuovo cosi': la fixture di `chain_id` era il lavoro, questo debito e' il ritrovamento.

E' inoltre la **terza porta** sullo stesso difetto: [DEBT-012] era un valore che entra in una preimmagine e che nessun documento fissa, [DEBT-020] era la circolarita' di `chain_id`, e questo e' un denominatore di epoca che dipende da quando lo si legge. La famiglia e' sempre quella: **una preimmagine i cui ingressi non sono derivabili in un solo modo**.

## Resolution criteria

Il documento deve dire **quale documento di parametri governa un'epoca di elezione**, con una regola che un verificatore che rigioca la catena possa applicare **senza giudizio e senza stato esterno**, a ogni altezza e per ogni epoca passata.

La forma da imitare e' quella che [SPEC-016] ha usato per `reward_epoch`: **l'oggetto nomina il documento che lo governa**, invece di rimandare a un «attivo» che dipende da quando lo si legge. Va stabilito se la cucitura vada sull'`election`, sul `validator_candidacy`, o su entrambi.

Va inoltre stabilito **cosa accade alle epoche a cavallo di un cambio di `L`**: se il confine si ricalcoli, se le epoche passate restino congelate sotto il documento che le governava, o se un cambio di `L` sia ammesso solo a un confine.

Il rimedio richiede tre regole di validita' nuove sull'elezione, quindi **fa scattare la gate di [ADR-012]** con la sua passata.

**Il rimedio apparente da non adottare:** rendere `election_epoch_blocks` non governato. Chiuderebbe l'ambiguita' togliendo la governance invece di scrivere la regola, e sposterebbe il problema su ogni altro parametro dell'elezione che entra in una derivazione — che e' curare il sintomo cambiando un parametro, [ADR-010].

~~Da chiudere prima che esista una seconda implementazione, per la stessa ragione di [DEBT-022].~~

**Corretto dal Lead il 2026-08-26, su contestazione di AGENT-007 in [REVIEW-029], e la contestazione e' fondata.** La condizione precedente era **troppo debole**, perche' importava da [DEBT-022] una ragione che qui non vale. La' il pericolo era la divergenza fra **due letture** dello stesso testo, quindi serviva un secondo lettore. Qui la divergenza colpisce **anche con una sola implementazione**: un nodo che rigioca la catena dopo un cambio di `L` attribuisce la stessa altezza a un'epoca diversa da un nodo rimasto online, **con lo stesso identico binario**. Non c'e' un secondo lettore da aspettare, e la condizione va letta come: **da chiudere prima che `election_epoch_blocks` possa cambiare su una catena che qualcuno rigiochera'**, cioe' prima della devnet.

Vale la pena registrare la forma dell'errore, perche' e' sottile: il Lead ha **importato una condizione di chiusura da un debito della stessa famiglia** senza verificare che la ragione che la sosteneva fosse la stessa. Due difetti della stessa famiglia possono avere inneschi diversi, e la famiglia non e' l'innesco. `ledger.md:1742` contempla esplicitamente che `L` cambi, il che rende l'innesco non ipotetico.

## Resolution evidence

