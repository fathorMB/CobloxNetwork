---
id: SPEC-025
# Note: Quote the title if it contains a colon
title: "Il motore di consenso: prevoto, blocco, e una catena finalizzata da quattro validatori"
status: backlog
kind: feature
priority: high
area: consensus
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: sol
thinking_level: maximum
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: [SKILL-001, SKILL-002, SKILL-003]
verification_gates: []
related_decisions: [ADR-018, ADR-001, ADR-012]
links: [DEBT-035]
created: 2026-08-27
updated: 2026-08-27
tags: [rust, ledger, conformance]
activity:
  - date: 2026-08-27
    action: "created"
  - date: 2026-08-27
    action: "set tags"
  - date: 2026-08-27
    action: "set recommended_agent"
---
# Il motore di consenso: prevoto, blocco, e una catena finalizzata da quattro validatori

## Objective

Attuare [ADR-018]: dare al progetto **il motore che raggiunge il consenso**, e dimostrarlo producendo **una catena di blocchi finalizzati da quattro validatori**.

Oggi `coblox-core` sa **verificare** un quorum e non sa **raggiungerlo**. Alla fine di questa spec una catena esiste, i blocchi hanno certificati veri, e i casi in cui il consenso deve sopravvivere — proponente muto, round fallito, validatore che vota due volte — sono esercitati e passano.

**Questa spec non consegna una devnet, e va detto in apertura.** Consegna il motore e lo fa girare **in un processo solo, su un trasporto in memoria**. Rete vera, persistenza e ciclo di vita del nodo sono la spec successiva. Le ragioni del taglio sono nei *Risks*.

## Context

[ADR-018] ha stabilito che il protocollo aveva già deciso quasi tutto senza saperlo. I vincoli che contano, verificati sull'albero:

- **un blocco porta il proprio certificato di quorum**, quindi un `Block` sul filo è già finalizzato e non è una proposta;
- **esiste un solo dominio di firma per i voti**, `coblox-block-vote-v0`, su quaranta;
- il predicato di quorum è **stretto** — `signed_power * 3 > total_power * 2` — e vale per ogni quorum di v0;
- **i round esistono già** nel `BlockHeader` e nella preimmagine del voto, senza che alcuna regola li produca.

Da lì il crux di [ADR-018]: con un solo voto firmato un protocollo è sicuro **oppure** vivo, mai entrambi. Il voto esistente è **un precommit**, e ciò che manca è la prima fase.

**Nulla di pubblicato cambia.** Si aggiunge un dominio, tre messaggi, la regola di chi propone, tre timeout.

## Scope
### Included

- **`coblox-block-prevote-v0`**, dominio nuovo, stessa forma di preimmagine del voto esistente.
- **I tre messaggi in `wire.md`**: proposta di blocco, prevoto, precommit, sulla busta firmata già specificata.
- **La regola di chi propone**: round-robin deterministico sul set attivo, indicizzato da `(height, round)` e pesato per potere di voto.
- **Il motore di consenso in `coblox-core`**, come **macchina a stati deterministica e senza I/O**: prende messaggi ed eventi di tempo, restituisce azioni. Nessuna socket, nessun orologio interno, nessun file.
- **La regola di blocco** di [ADR-018] parte 2.
- **I tre timeout** come parametri **locali** e dichiarati tali.
- **Un banco di prova** che istanzia quattro motori in un processo, li collega con un trasporto in memoria che il test controlla, e produce **una catena reale** di blocchi finalizzati con certificati veri.
- **La passata di [ADR-012]** sul contenuto normativo nuovo.

### Excluded

- **Rete vera, scoperta dei peer, persistenza, ciclo di vita del nodo.** Sono la spec successiva. `wire.md` specifica già lo stack e non ne è implementato niente.
- **Il mempool e la selezione delle transazioni.** Il proponente di questa spec propone blocchi con l'insieme di transazioni che il banco di prova gli dà, anche vuoto.
- **Ogni modifica a `BlockHeader`, `QuorumCertificate`, al predicato di quorum, o alla preimmagine del voto esistente.** [ADR-018] dichiara che nulla di pubblicato cambia: **se durante l'implementazione sembrasse necessario cambiarne uno, fermarsi e riportarlo al Lead**, perché sarebbe una premessa dell'ADR che cade e non un dettaglio.
- La taratura dei valori dei timeout.

## Existing-project analysis

- `docs/protocol/ledger.md` — §*"What validators sign"* (la preimmagine del voto), il `QuorumCertificate`, il `BlockHeader`, §*"Quorum predicate"*, §*"Validator-set continuity"*.
- `docs/protocol/wire.md` — la busta firmata, il framing, il catalogo dei messaggi, la validazione gossip e la contropressione.
- `core/coblox-core/src/` — `quorum.rs`, `validator_set.rs`, `block.rs`, `verifier.rs`, `hash.rs` (i domini di firma), `params.rs`.
- `core/coblox-node/src/main.rs` — ventun righe, `start` non fa nulla. **Questa spec non lo tocca.**

## Technical proposal

**Il motore è una funzione, non un servizio.** Prende `(stato, evento) → (stato', azioni)`, dove un evento è un messaggio ricevuto o un timeout scaduto, e un'azione è un messaggio da inviare o un blocco da finalizzare. **Non legge l'orologio, non apre socket, non scrive file.**

È la forma che rende esercitabili i casi che contano — partizione, proponente muto, round fallito, doppio voto — che su una rete vera sono difficili da produrre e impossibili da riprodurre. Ed è la stessa forma del resto di `coblox-core`, che è una libreria di regole senza I/O.

**Le tre fasi**, per [ADR-018]:

1. il proponente del round `(h, r)` invia una proposta;
2. chi la accetta invia un **prevoto**; chi vede oltre due terzi di prevoti per lo stesso blocco si **blocca** e invia il **precommit**;
3. chi vede oltre due terzi di precommit **finalizza**, assembla il `QuorumCertificate` e lo allega al blocco.

**La regola di blocco è il punto in cui questi protocolli si sbagliano**, e [ADR-018] lo dice: va **presa dalla letteratura e provata contro di essa**, non derivata da capo. La sicurezza dell'intero ledger poggia su tre righe.

**I timeout sono locali**, e vanno dichiarati tali nel documento: non sono portati da alcun documento firmato, quindi **nessuna regola di validità potrà mai confrontarli**. È il criterio di [[predicato-di-accettazione]], e nominarlo adesso costa una riga; scoprirlo dopo è costato tre passate di review sui parametri operativi.

## Files and areas involved

- `docs/protocol/ledger.md`, `docs/protocol/wire.md`
- `core/coblox-core/src/` — il modulo nuovo del motore, più `hash.rs` per il dominio
- `core/coblox-core/tests/` — le prove del motore e il banco a quattro validatori
- `sim/tools/published_artifacts.toml` — le probe del contenuto nuovo

## Acceptance criteria

- [ ] `coblox-block-prevote-v0` esiste come dominio, con la preimmagine di [ADR-018], e un test lo esercita.
- [ ] I tre messaggi sono in `wire.md`, nella forma del catalogo esistente.
- [ ] La regola di chi propone è scritta, è deterministica, e **due nodi con lo stesso `validator_set_hash` calcolano lo stesso proponente senza scambiare messaggi** — dimostrato da un test.
- [ ] Il motore è **senza I/O**: nessuna socket, nessun orologio, nessun file. Dimostrato dalla forma della sua interfaccia, non da un'affermazione.
- [ ] **Quattro validatori producono una catena di almeno dieci blocchi finalizzati**, con certificati veri che il verificatore esistente accetta.
- [ ] **Sicurezza sotto partizione**: sotto uno scheduler avverso che riordina, ritarda e partiziona i messaggi, **non finalizzano mai due blocchi diversi alla stessa altezza**. Il test dichiara **quante esecuzioni** ha percorso.
- [ ] **Vivacità dopo un proponente muto**: se il proponente del round `r` non invia nulla, l'altezza **finalizza comunque** a un round successivo. È il caso che [ADR-018] dichiara fatale per l'alternativa a una fase.
- [ ] **Equivocazione rifiutata**: un validatore che invia due precommit diversi allo stesso round non fa finalizzare due blocchi, e il caso è esercitato.
- [ ] **Determinismo**: la stessa sequenza di eventi produce la stessa catena, byte per byte.
- [ ] La regola di blocco è **confrontata con la fonte da cui è presa**, e la trascrizione dichiara quale.
- [ ] **Nessuna modifica** a `BlockHeader`, `QuorumCertificate`, predicato di quorum, preimmagine del voto esistente. Verificato dal diff.
- [ ] Passata di [ADR-012] eseguita e trascritta; `published_artifacts.py` `PASS`, prova in negativo compresa.
- [ ] `cargo test --workspace --all-features`, `clippy -D warnings`, `fmt --check` puliti.

## Implementation plan

1. Leggere [ADR-018] per intero, e la sezione §*"What validators sign"* di `ledger.md`.
2. **Scrivere prima i documenti** — dominio, messaggi, regola del proponente, regola di blocco — poi il codice dal testo dei documenti.
3. Il motore come macchina a stati, con l'interfaccia che rende impossibile l'I/O.
4. Il banco a quattro validatori con lo scheduler controllato dal test.
5. I casi avversi: partizione, proponente muto, equivocazione.
6. Passata di [ADR-012] ([SKILL-002]) e prova in negativo ([SKILL-001]).

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-CHAIN-EXISTS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Quattro validatori producono almeno dieci blocchi finalizzati, e la trascrizione mostra altezze, round e il fatto che il verificatore esistente accetta i certificati.
- [ ] GATE-SAFETY-UNDER-ADVERSARY | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Sotto scheduler avverso non finalizzano due blocchi diversi alla stessa altezza. **La trascrizione dichiara quante esecuzioni sono state percorse**: un numero che non c'è è una prova che non c'è.
- [ ] GATE-LIVENESS-AFTER-SILENCE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Con il proponente del round `r` muto, l'altezza finalizza a un round successivo. È il caso che distingue questa architettura da quella rifiutata.
- [ ] GATE-LOCKING-FROM-SOURCE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La regola di blocco è confrontata riga per riga con la fonte da cui è presa, che la trascrizione nomina. **Una regola di blocco derivata da capo va respinta anche se i test passano.**
- [ ] GATE-NO-IO | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il motore non ha I/O, dimostrato dalla propria interfaccia e non da un'affermazione.
- [ ] GATE-NOTHING-PUBLISHED-CHANGED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il diff non tocca `BlockHeader`, `QuorumCertificate`, il predicato di quorum, né la preimmagine del voto esistente. È la premessa di [ADR-018] e si verifica guardando il diff.
- [ ] GATE-ADR012-PASS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Passata eseguita, `published_artifacts.py` `PASS`, probe nuove nella prova in negativo.
- [ ] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=transcript | Pipeline reale verde, con numero di run e commit.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | Review di AGENT-007. **È la superficie di sicurezza più grande che il progetto abbia prodotto finora**: finora il rischio stava nelle regole, da qui sta in un protocollo distribuito con stati, timeout e avversari che tacciono.
- [ ] GATE-LEAD-REPRO | kind=manual | owner=lead | phase=before-done | evidence=transcript | Il Lead riesegue in modo indipendente il caso del proponente muto e almeno una esecuzione avversa, invece di prenderli dall'evidenza.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

**Perché il taglio è questo, ed è una scelta del Lead che l'operatore può ribaltare.** Una spec che mettesse insieme rete, persistenza e consenso avrebbe una superficie che questa sessione ha già dimostrato di non saper rivedere bene. Il taglio mette **nel primo pezzo tutto il rischio di sicurezza** — la regola di blocco — e in una forma **esercitabile in modo esaustivo**: partizioni e proponenti muti si producono in memoria con tre righe e su una rete vera con giorni di lavoro, e non si riproducono. Il motore **non cambia** quando arriverà il trasporto vero.

**Il costo del taglio, dichiarato: alla fine di questa spec non esiste ancora una devnet.** Esiste una catena prodotta da quattro motori in un processo. È un passo reale e non è l'esito che [M-02] nomina.

**Il rischio dominante è la regola di blocco.** Tre righe su cui poggia la sicurezza dell'intero ledger, e la storia dei protocolli BFT è fatta di implementazioni che le hanno semplificate perdendo sicurezza in casi che i test ordinari non producono. `GATE-LOCKING-FROM-SOURCE` esiste per questo, e **una regola derivata da capo va respinta anche se i test passano**.

**Il secondo rischio è credere che i test verdi bastino.** Un protocollo BFT sbagliato passa il caso felice sempre. La prova sta nelle esecuzioni avverse e nel loro numero, ed è la ragione per cui `GATE-SAFETY-UNDER-ADVERSARY` chiede un conteggio.

**Se emergesse la necessità di cambiare un artefatto pubblicato**, è una premessa di [ADR-018] che cade: **fermarsi e riportarlo**, non aggirarlo.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **La regola di blocco si prende, non si inventa.** Nomina la fonte e confrontala riga per riga.
- **Ogni numero che scrivi va guardato**, e ogni superlativo assoluto va contato.
- **Consegna ogni dimostrazione insieme al perimetro su cui vale.**
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

### Files changed

### Verification performed

### Verification transcript
