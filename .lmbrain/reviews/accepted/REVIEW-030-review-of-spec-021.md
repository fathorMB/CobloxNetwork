---
id: REVIEW-030
# Note: Quote the title if it contains a colon
title: "Review of SPEC-021"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-021
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-030-EVENT-001"
    timestamp: "2026-08-26T11:53:36.386332400+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Un finding low, non a carico dell'implementazione. Riverificato dal Lead rieseguendo: 167 test da 165, 126 probe C10 da 116 ciascuna osservata fallire da sola, protocol_hashes senza valori mossi, lista DRAFT da 4 a 3 con le tre superstiti giuste.\n\nRF-001: lib.rs:52-56 dichiara che i parametri sono configurazione validata e non costanti compilate, e poi enumera cinque portatori. CadenceBand e' il sesto, introdotto da SPEC-016, ed e' quello i cui valori questa spec ha appena scritto; in lib.rs compare zero volte. Terza occorrenza stanotte della stessa forma - lista dichiarata invece che osservata, ultimo arrivato non aggiunto - dopo SECURITY.md fuori dall'inventario di ADR-012 e le due liste senza lato disco. Il rimedio e' una decisione e non un'aggiunta: il paragrafo chiude dicendo che in produzione quei valori arrivano dentro un documento firmato da un quorum, e per CadenceBand e' falso e deliberatamente, perche' ADR-016 stabilisce che nessun documento on-chain possa cambiarla.\n\nIl Lead ha attaccato quattro cose e nessuna si e' rotta: che i valori di lancio fossero compilati nel crate (cadence.rs:490 e' sotto #[cfg(test)] alla riga 476, e max_ms_per_block vale li' 10_000 e non 20_000, conferma indipendente); GATE-RENUNCIATION-INTACT dal lato della posizione e non della presenza, con la frase a riga 114 e i due hunk a 1378 e 1555; che la banda fosse raggiungibile da un documento di consenso; e il conteggio della lista DRAFT. Il finding e' emerso mentre attaccava il primo, non cercandolo.\n\nDue difetti sono della spec, quindi del Lead. \"Al posto della voce DRAFT\" preso alla lettera avrebbe messo cinque valori di un'ancora di fiducia in una sezione che il documento dichiara non normativa; l'implementatrice ha letto le due voci di scopo come partenza e arrivo dello stesso trasferimento, che e' cio' che il Lead intendeva. E \"Files and areas involved\" indicava core/coblox-core/ come sede dell'ancora di genesi, che non esiste: l'implementatrice ha rifiutato di inventare un file per far quadrare la spec, esito che SKILL-003 prevede.\n\nRegistrato che SKILL-001 passo 4 ha prodotto un caso di prova che senza di essa non sarebbe esistito: il test preesistente sulla relazione dello slack provava entrambi i lati della frontiera tenendo costanti min_measured_blocks e block_interval_ms, quindi non distingueva una regola relazionale da una soglia costante, e la mutazione che sostituisce la finestra con la costante fallisce solo sul caso nuovo."
    evidence_refs: ["SPEC-021", "ADR-016", "SKILL-001"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-030-EVENT-002"
    timestamp: "2026-08-26T11:58:09.286725300+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Remediation di RF-001 consegnata da AGENT-002. L'elenco dei portatori e' stato riscritto in tre classi invece di ricevere un sesto membro, perche' aggiungere CadenceBand sotto una clausola falsa per essa avrebbe scambiato un difetto di omissione con uno di asserzione. Da verificare dal Lead."
    evidence_refs: ["SPEC-021"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-030-EVENT-003"
    timestamp: "2026-08-26T11:58:26.444892300+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificata dal Lead rieseguendo: 167 test invariati, published_artifacts.py PASS con 126 candidati C10 e ogni probe osservata fallire da sola, protocol_hashes.py PASS. La remediation tocca solo documentazione di modulo: nessuna firma, nessuna regola, nessun valore, e il conteggio delle probe non doveva salire e non e' salito.\n\nLa soluzione e' migliore dell'aggiunta che il finding suggeriva. Invece di un sesto membro, l'elenco e' stato riscritto in tre classi, tagliate su un criterio che non e' \"e' configurazione\" - lo sono tutte e sei, ed e' per questo che la lista le confondeva - ma su cosa significhi rifiutare l'oggetto: documenti governati, dove il rifiuto e' operazione ordinaria di protocollo e la clausola contestata e' vera; bound di genesi, che limitano cio' che un documento governato puo' portare; e CadenceBand, che viaggia nella distribuzione firmata come una bound ma non limita nulla che un documento porti.\n\nIl Lead ha attaccato la citazione su cui poggia la terza classe, perche' se fosse stata inventata la soluzione sarebbe stata un'uscita dall'impaccio invece di un ancoraggio. Regge: README.md:1327 porta il paragrafo \"How the band differs from the two bounds objects, and it is not a detail\", e la frase \"The cadence band bounds nothing any document carries\" e' verbatim. Il commento di modulo aveva quindi perso una distinzione che il documento normativo faceva gia', ed e' la ragione per cui la lista a cinque poteva sembrare completa: con un membro in meno la clausola di chiusura era vera per tutti.\n\nIl testo nuovo nomina la falsita' invece di aggirarla, e mette il divieto dove lo cerca chi legge il commento per capire dove infilare la banda.\n\nUna riga oltre il perimetro, dichiarata invece di essere fatta trovare: il paragrafo successivo aveva lo stesso difetto sullo stesso oggetto, enumerando due punti di ingresso composti e omettendo la cadenza, mentre CadenceBand::validate e' gia' chiamata come primo atto di entrambe le misure. Il Lead l'accetta: lasciarlo sarebbe stato consegnare meta' rimedio sulla stessa lista dichiarata."
    evidence_refs: ["SPEC-021", "REVIEW-030"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-030-EVENT-004"
    timestamp: "2026-08-26T11:58:39.510762600+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata dopo un giro di remediation. Tutti i criteri soddisfatti e le cinque gate spuntate con la loro evidenza.\n\nL'osservazione che chiude il rapporto vale piu' del finding che l'ha prodotta, ed e' dell'implementatrice: il conteggio delle probe non doveva salire e non e' salito perche' published_artifacts.py copre docs/protocol/, sim/tools/ e core/coblox-core/tests/, quindi src/lib.rs non e' letto da alcuna gate di ADR-012. RF-001 stava in un punto cieco della passata - la stessa famiglia che l'ha prodotto. Aperto come DEBT-031.\n\nRegistrato che SKILL-001 passo 4 ha prodotto un caso di prova che senza di essa non sarebbe esistito, e che le altre due skill hanno ricevuto due critiche di taratura, entrambe fondate e gia' applicate: SKILL-002 non diceva cosa scrivere in assenza di valori variati, SKILL-003 portava convenzioni di unita' dentro una skill che si legge a ogni dispatch. Sono difetti che sarebbero rimasti invisibili se il dispatch avesse chiesto un'esecuzione invece di un giudizio.\n\nDue difetti della spec sono del Lead e sono registrati come tali: \"al posto della voce DRAFT\", che preso alla lettera avrebbe reso non normativi cinque valori di un'ancora di fiducia, e l'indicazione di core/coblox-core/ come sede dell'ancora di genesi, che non esiste. L'implementatrice ha rifiutato di inventare un file per far quadrare la spec."
    evidence_refs: ["SPEC-021", "ADR-016", "SKILL-001", "DEBT-031"]
    implementation_agent: "AGENT-002"
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [review]
activity:
  - date: 2026-08-26
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-26
    action: "recorded review remediation"
  - date: 2026-08-26
    action: "recorded review remediation-verification"
  - date: 2026-08-26
    action: "transitioned changes-requested -> accepted"
---
# Review

## Outcome

**Changes requested, un finding `low`**, e non è a carico dell'implementazione: è una lista dichiarata invece che osservata, nel commento di modulo del crate, che questa spec ha reso incompleta senza esserne la causa.

**Due difetti della spec sono dell'autrice della spec, cioè del Lead**, e l'implementatrice li ha trovati leggendo invece di eseguire.

## Acceptance-criteria compliance

Riverificato dal Lead rieseguendo: **167 test** da 165. `published_artifacts.py` `PASS` con **126 candidati C10** da 116. `published_artifacts_negative.py` `PASS` con 15 mutazioni su 11 classi **più ogni probe individualmente**, 126 su 126. `protocol_hashes.py` `PASS`, nessun valore pubblicato cambiato.

I tre vincoli reggono sui valori scritti — `2500 ≤ 5000 ≤ 20000`, tutti positivi, `600000 < 3 600 000` — e non sono stati riscritti ma **applicati alla banda di genesi da un test**.

**Il lato lento è scritto come numero in due punti indipendenti**, che è più di quanto la gate chiedesse: l'intestazione porta `4 * block_interval_ms` e la prosa porta *«can stretch its own epochs to four times their declared real-time length»*. Due probe distinte li tengono, con la ragione giusta — il multiplo e la conseguenza si possono cancellare **indipendentemente**.

**La prosa istruttiva della voce DRAFT è rientrata come giustificazione invece di essere cancellata**, che era il rischio secondario nominato dalla spec. Compreso l'avvertimento sui `2 ×`, arricchito della conseguenza che [ADR-016] ne trae e che la voce DRAFT non conteneva.

## Cosa ho attaccato senza riuscire a romperlo

**Che i valori di lancio fossero finiti compilati nel crate.** È l'attacco che pagava di più, perché contraddirebbe una dichiarazione che `lib.rs` fa su sé stesso. `grep` su `core/coblox-core/src/` trova `min_ms_per_block: 2_500` a `cadence.rs:490` — ma la riga `476` è `#[cfg(test)]` e la `477` è `mod tests`, quindi è dentro il modulo di prova; il commento accanto dice *«A test input, not a launch value»*, e `max_ms_per_block` vale lì `10_000` e non `20_000`, che è la conferma indipendente che non sono i valori di lancio. **Non si è rotto.**

**`GATE-RENUNCIATION-INTACT` dal lato della posizione, non della presenza.** La probe che pinna la frase dimostra che c'è, non che il testo attorno non la contraddica. Verificato: la frase è alla riga **114**, i due hunk del diff di `README.md` sono a **1378** e **1555**. Mille e duecento righe di distanza, nessun contatto. **Non si è rotto.**

**Che la banda fosse diventata raggiungibile da un documento di consenso**, cioè il rischio che la spec nominava come «mossa proposta in buona fede». Nessun campo nuovo, nessuno schema, nessuna regola: `min_ms_per_block` e `max_external_clock_slack_ms` non compaiono in alcun contesto di `ConsensusParametersBody`. **Non si è rotto.**

**Il conteggio della lista DRAFT.** Quattro prima, tre dopo, e le tre superstiti sono le stesse tre. **Non si è rotto.**

Ciò che si è rotto l'ho trovato **mentre** attaccavo il primo, non cercandolo.

## Review findings

**RF-001 — `low` — il commento di modulo enumera i portatori di configurazione e `CadenceBand` non c'è, e per essa una delle clausole è falsa.**

`core/coblox-core/src/lib.rs:52-56` dichiara il principio — *«Parameters are validated configuration, never compiled constants. No launch value appears in this crate»* — e poi **enumera** i portatori: `ConsensusParameters`, `ElectionBounds`, `RewardPolicy`, `RewardBounds`, `EnrollmentParameters`. Cinque.

`CadenceBand` è il sesto, introdotto da [SPEC-016], ed è **quello i cui valori questa spec ha appena scritto**. In `lib.rs` compare **zero volte**.

È la terza volta stanotte che compare la stessa forma — `SECURITY.md` fuori dall'inventario di [ADR-012], `CLAIM_DOCS` e `MIRROR_FILES` senza lato disco — e cioè **una lista dichiarata invece che osservata, alla quale l'ultimo arrivato non è stato aggiunto**.

Ma qui c'è un secondo strato che rende il rimedio una decisione e non un'aggiunta. Il paragrafo chiude con: *«in production these values arrive inside a document a validator quorum signed»*. **Per `CadenceBand` è falso, e deliberatamente**: [ADR-016] stabilisce che vive nella distribuzione firmata e che **nessun documento on-chain può cambiarla**, perché una banda che un quorum seduto potesse allargare sarebbe una tolleranza sotto l'unica misura che il protocollo ha di quel quorum.

Quindi aggiungerla all'elenco senza altro la porterebbe sotto una frase che la descrive al contrario. Serve la distinzione, non l'aggiunta.

## Required follow-up

RF-001 all'implementatrice: è materia sua, perché la classificazione di `CadenceBand` rispetto agli altri portatori è una scelta di progetto che ha fatto lei in [SPEC-016].

**Due cose sono difetti della spec, quindi del Lead**, e vanno corrette nelle prossime invece che qui.

Lo scopo diceva «i cinque valori in `README.md`, **al posto della voce DRAFT**». Preso alla lettera sarebbe stato un difetto: la riga 25 del documento stabilisce che *«Unless a section is explicitly headed DRAFT, it is normative for v0»*, quindi i valori sarebbero finiti in una sezione che il documento stesso dichiara aperta — **cinque valori di un'ancora di fiducia dichiarati non normativi**. L'implementatrice ha letto le due voci di scopo come partenza e arrivo dello stesso trasferimento, che è ciò che intendevo, e i valori sono nella sezione normativa. **La formulazione ammetteva l'altra lettura ed è un difetto mio.**

E «Files and areas involved» indicava `core/coblox-core/` come sede dell'ancora di genesi. **Non esiste** un artefatto di ancora con valori: `lib.rs:53` dichiara che nessun valore di lancio appare nel crate, e l'ancora di fiducia **è** la sezione `## Trust anchors` di `README.md`. L'implementatrice ha scritto i valori lì e nel crate li ha messi in documentazione, che è il trattamento che `block_interval_ms` riceveva già. **Ha rifiutato di inventare un file per far quadrare la mia spec**, ed è l'esito che [SKILL-003] prevede.

Resta al Lead anche la riconciliazione di `ROADMAP.md`, che non elenca [SPEC-021] sotto M-02.

## Final decision

**Changes requested su RF-001.**

Va registrato che [SKILL-001] ha prodotto un caso di prova che senza di essa non sarebbe esistito, e l'implementatrice lo documenta: il test preesistente sulla relazione dello slack provava entrambi i lati della frontiera **tenendo costanti `min_measured_blocks` e `block_interval_ms`**, quindi non distingueva una regola relazionale da una soglia costante. Il passo 4 — *cosa hanno in comune tutti i casi?* — le ha fatto aggiungere il quarto caso a finestra dimezzata, e la mutazione che sostituisce la finestra con la costante `3_600_000` fallisce **solo** su quello. È la stessa forma del difetto di `GATE-MEASURE-BINDS`, intercettata prima di uscire invece che due review dopo.
