---
id: SPEC-025
# Note: Quote the title if it contains a colon
title: "Il motore di consenso: prevoto, blocco, e una catena finalizzata da quattro validatori"
status: done
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
  - date: 2026-08-27
    action: "transitioned backlog -> ready"
  - date: 2026-08-27
    action: "transitioned ready -> working"
  - date: 2026-08-27
    action: "transitioned working -> review"
  - date: 2026-08-27
    action: "attested verification GATE-CI-GREEN by lead"
  - date: 2026-08-27
    action: "attested verification GATE-LEAD-REPRO by lead"
  - date: 2026-08-27
    action: "attested verification GATE-CI-GREEN by lead"
  - date: 2026-08-27
    action: "attested verification GATE-SECREVIEW by lead"
  - date: 2026-08-27
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "e1bdc40618ab8ac93136c8b1c9865bab5ca7170cb2b82934eb92c0dfc9afa8b7"
    evidence_ref: "Run GitHub Actions 33086611370 sul commit 76b5bd3 di main, che porta la consegna: sei job su sei verdi — Rust (ubuntu-latest), Rust (windows-latest), Tauri desktop (ubuntu-latest), Tauri desktop (windows-latest), Android arm64 + Kotlin bindings, Protocol document guards. Verificato dal Lead interrogando i job della run, non il solo esito complessivo.\n\nVerificato inoltre che i tre passi nuovi introdotti da questa consegna abbiano davvero girato, e non solo di essere stati aggiunti al file: lo sweep esteso in release — cargo test --release --locked -p coblox-core --test consensus_devnet -- --ignored --nocapture — risulta success su Linux e skipped su Windows, che e' il comportamento voluto dalla sua condizione runner.os == 'Linux'; \"The consensus engine has no I/O\" success; \"That lint proved in the negative, three defect classes\" success.\n\nIl controllo sui passi nuovi conta piu' dell'esito complessivo: un passo aggiunto a un workflow che non venga mai eseguito produce una pipeline verde e una guardia inesistente, ed e' la forma di difetto che questa consegna dichiarava di voler evitare quando ha portato lo sweep in CI invece di lasciarne il numero in una trascrizione."
    id: "SPEC-025-ATTEST-001"
    requirement_digest: "f8e43a5fdd0ae9021563670cf903511159d3c6e03749a1f62b79d01126d324fd"
    requirement_id: "GATE-CI-GREEN"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-27T17:26:29.979625900+02:00"
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "8f1b18a180623661258f92e19e45402e21b61d183ba4f311d10c80de00c169a8"
    evidence_ref: "Riprodotta dal Lead sull'albero dopo la remediation, in tre parti.\n\nPROPONENTE MUTO. Eseguito `cargo test -p coblox-core --test consensus_devnet -- --nocapture` e letta la trascrizione reale: \"proposer of (height 1, round 0) is val-001, silenced. Height 1 finalized at round 1 proposed by val-002, after 46 scheduled events and 300 ms of virtual clock.\" E' il caso che ADR-018 dichiara fatale per l'alternativa a una fase, quindi il caso che giustifica l'architettura scelta, e finalizza.\n\nESECUZIONE AVVERSA. Dalla stessa esecuzione: \"8 conflicting precommits injected under val-000's real key across 4 rounds; chain length 10, no height with two block IDs, no certificate with a repeated signer\". Piu' la catena piena: \"4 validators, 375 scheduled events, virtual clock 101 ms, 320 messages admitted through the boundary, 40 certificates verified\".\n\nMUTAZIONE, che e' la parte indipendente. Il Lead ha disattivato il legame fra carico e blocco introdotto dalla remediation di RF-001 - la guardia `computed_root != proposal.header.transactions_root` in `consensus/messages.rs` - e ha eseguito le due suite di consenso. Falliscono esattamente i due test che quella regola esiste per tenere e nessun altro: `one_header_with_two_payloads_does_not_produce_two_blocks`, cioe' E5 invertito nel banco a quattro nodi, e `a_proposal_whose_payload_does_not_reproduce_its_root_is_refused`, cioe' il controllo al confine. Fuori da quelli, 24 passati su 25 in consensus_rules e 9 su 10 in consensus_devnet. Albero ripristinato con git checkout dal commit 31669eb e riverificato pulito.\n\nContorno rieseguito dal Lead e dichiarato per quello che e' - riesecuzione, non riproduzione indipendente: 230 test verdi 0 falliti, clippy --workspace --all-features --all-targets -D warnings pulito, cargo fmt --all --check pulito, nove strumenti di progetto a exit 0, probe da 172 a 180."
    id: "SPEC-025-ATTEST-002"
    requirement_digest: "8b9cd405aa8b58392ee05c084d4f0336ad506a94518d39d3e911c2a5fa99e882"
    requirement_id: "GATE-LEAD-REPRO"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-27T18:17:24.243203200+02:00"
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "ebfbf0bd7d7a5181d6279b2089c5c139374962c77aaf2f013217b4da07693d38"
    evidence_ref: "RIATTESTAZIONE dopo la remediation di REVIEW-047, che sostituisce SPEC-025-ATTEST-001. Quella attestava la run 33086611370 sul commit 76b5bd3; il codice e' cambiato con 31669eb, quindi accettare su quella significherebbe attestare su un albero che non esiste piu'.\n\nRun GitHub Actions 33092387240 sul commit 437f4c4 di main, che porta la remediation e l'attestazione di GATE-LEAD-REPRO: sei job su sei verdi — Rust (ubuntu-latest), Rust (windows-latest), Tauri desktop (ubuntu-latest), Tauri desktop (windows-latest), Android arm64 + Kotlin bindings, Protocol document guards. Verificato dal Lead interrogando i job, non il solo esito complessivo.\n\nVerificato inoltre che i passi introdotti da questa spec abbiano girato su QUESTO commit e non solo sul precedente: lo sweep esteso in release risulta success su Linux e skipped su Windows, che e' il comportamento voluto dalla condizione runner.os == 'Linux'; \"The consensus engine has no I/O\" success; \"That lint proved in the negative, three defect classes\" success.\n\nIl controllo sui passi conta piu' dell'esito complessivo: un passo che non venga eseguito produce una pipeline verde e una guardia inesistente, ed e' la ragione per cui questa consegna ha portato lo sweep in CI invece di lasciarne il numero in una trascrizione."
    id: "SPEC-025-ATTEST-003"
    requirement_digest: "f8e43a5fdd0ae9021563670cf903511159d3c6e03749a1f62b79d01126d324fd"
    requirement_id: "GATE-CI-GREEN"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-27T18:26:09.905190200+02:00"
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "b1728c3745d68f7bc58b4bc281d7b8979f946e4986d4da0b4484ab9b32bb5237"
    evidence_ref: "REVIEW-048, accepted, di AGENT-007: seconda esecuzione della gate sull'albero dopo la remediation. La prima passata, REVIEW-047, aveva chiesto modifiche su tre bloccanti; questa li dichiara tutti e tre chiusi e lo stabilisce mutando l'albero invece di leggere l'evidenza.\n\nLa gate e' stata rieseguita e non attestata sulla passata precedente, perche' il codice e' cambiato con il commit 31669eb: accettare su REVIEW-047 avrebbe attestato un albero che non esiste piu'.\n\nIl controllo piu' stringente e' su RF-001: allargare di un solo campo la rimozione in `transactions_root_of` fa fallire tre test, quindi la rimozione e' esattamente quella che `ledger.md` definisce per `tx_id` — un errore li' avrebbe fatto rifiutare proposte oneste o accettare carichi divergenti, e su un percorso di consenso e' un fork. La reviewer ha inoltre verificato che il motivo scritto per RF-002 sia vero e non solo plausibile: `block_id` copre davvero l'header, `round` incluso, e la POL a `vr` e' verificata contro il log proprio.\n\nSette rilievi nuovi, nessuno bloccante. I due che contano sono promossi a debito con un bersaglio e un criterio invece di aprire un terzo giro: DEBT-047 sul carico slegato in `FinalizedBlock::verify`, instradato su SPEC-029 perche' e' la consegna che rende il buco raggiungibile portando disco e rete; DEBT-048 sull'ordine della radice non provato, instradato sulla stessa spec perche' vi aggiungera' un secondo sito che ricalcola la radice.\n\nLa gate e' soddisfatta con il residuo dichiarato e tracciato, non con il residuo taciuto."
    id: "SPEC-025-ATTEST-004"
    requirement_digest: "c177f49122193d8c7aaf6fb04646b1aefb94019ba93c89d9a5f4e3d7b25d809e"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-27T18:43:03.876765200+02:00"
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

- [x] `coblox-block-prevote-v0` esiste come dominio, con la preimmagine di [ADR-018], e un test lo esercita.
- [x] I tre messaggi sono in `wire.md`, nella forma del catalogo esistente.
- [x] La regola di chi propone è scritta, è deterministica, e **due nodi con lo stesso `validator_set_hash` calcolano lo stesso proponente senza scambiare messaggi** — dimostrato da un test.
- [x] Il motore è **senza I/O**: nessuna socket, nessun orologio, nessun file. Dimostrato dalla forma della sua interfaccia, non da un'affermazione.
- [x] **Quattro validatori producono una catena di almeno dieci blocchi finalizzati**, con certificati veri che il verificatore esistente accetta.
- [x] **Sicurezza sotto partizione**: sotto uno scheduler avverso che riordina, ritarda e partiziona i messaggi, **non finalizzano mai due blocchi diversi alla stessa altezza**. Il test dichiara **quante esecuzioni** ha percorso.
- [x] **Vivacità dopo un proponente muto**: se il proponente del round `r` non invia nulla, l'altezza **finalizza comunque** a un round successivo. È il caso che [ADR-018] dichiara fatale per l'alternativa a una fase.
- [x] **Equivocazione rifiutata**: un validatore che invia due precommit diversi allo stesso round non fa finalizzare due blocchi, e il caso è esercitato.
- [x] **Determinismo**: la stessa sequenza di eventi produce la stessa catena, byte per byte.
- [x] La regola di blocco è **confrontata con la fonte da cui è presa**, e la trascrizione dichiara quale.
- [x] **Nessuna modifica** a `BlockHeader`, `QuorumCertificate`, predicato di quorum, preimmagine del voto esistente. Verificato dal diff.
- [x] Passata di [ADR-012] eseguita e trascritta; `published_artifacts.py` `PASS`, prova in negativo compresa.
- [x] `cargo test --workspace --all-features`, `clippy -D warnings`, `fmt --check` puliti.

## Implementation plan

1. Leggere [ADR-018] per intero, e la sezione §*"What validators sign"* di `ledger.md`.
2. **Scrivere prima i documenti** — dominio, messaggi, regola del proponente, regola di blocco — poi il codice dal testo dei documenti.
3. Il motore come macchina a stati, con l'interfaccia che rende impossibile l'I/O.
4. Il banco a quattro validatori con lo scheduler controllato dal test.
5. I casi avversi: partizione, proponente muto, equivocazione.
6. Passata di [ADR-012] ([SKILL-002]) e prova in negativo ([SKILL-001]).

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-CHAIN-EXISTS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Quattro validatori producono almeno dieci blocchi finalizzati, e la trascrizione mostra altezze, round e il fatto che il verificatore esistente accetta i certificati.
- [x] GATE-SAFETY-UNDER-ADVERSARY | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Sotto scheduler avverso non finalizzano due blocchi diversi alla stessa altezza. **La trascrizione dichiara quante esecuzioni sono state percorse**: un numero che non c'è è una prova che non c'è.
- [x] GATE-LIVENESS-AFTER-SILENCE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Con il proponente del round `r` muto, l'altezza finalizza a un round successivo. È il caso che distingue questa architettura da quella rifiutata.
- [x] GATE-LOCKING-FROM-SOURCE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La regola di blocco è confrontata riga per riga con la fonte da cui è presa, che la trascrizione nomina. **Una regola di blocco derivata da capo va respinta anche se i test passano.**
- [x] GATE-NO-IO | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il motore non ha I/O, dimostrato dalla propria interfaccia e non da un'affermazione.
- [x] GATE-NOTHING-PUBLISHED-CHANGED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il diff non tocca `BlockHeader`, `QuorumCertificate`, il predicato di quorum, né la preimmagine del voto esistente. È la premessa di [ADR-018] e si verifica guardando il diff.
- [x] GATE-ADR012-PASS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Passata eseguita, `published_artifacts.py` `PASS`, probe nuove nella prova in negativo.
- [x] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=transcript | Pipeline reale verde, con numero di run e commit.
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | Review di AGENT-007. **È la superficie di sicurezza più grande che il progetto abbia prodotto finora**: finora il rischio stava nelle regole, da qui sta in un protocollo distribuito con stati, timeout e avversari che tacciono.
- [x] GATE-LEAD-REPRO | kind=manual | owner=lead | phase=before-done | evidence=transcript | Il Lead riesegue in modo indipendente il caso del proponente muto e almeno una esecuzione avversa, invece di prenderli dall'evidenza.

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
> Compilata da AGENT-002 il 2026-08-27.

### Changes made

**Il motore.** `core/coblox-core/src/consensus/`, modulo nuovo in cinque file:

- `mod.rs` — la documentazione portante: il confronto **riga per riga** con la
  fonte della regola di blocco, le **quattro divergenze** dichiarate, l'argomento
  del no-I/O, e la nota sui due `round`.
- `engine.rs` — la macchina a stati. `Engine::step_event(Event) -> Result<Vec<Action>>`.
  Ogni regola porta in commento il numero di riga dell'Algorithm 1 che attua.
- `messages.rs` — i tre messaggi, `VotePhase`, `Validity`, e il **confine**:
  `verify_proposal` e `verify_vote` sono i soli costruttori di `VerifiedMessage`,
  il cui campo interno è privato, quindi al motore non può arrivare un messaggio
  di cui nessuno ha controllato la firma.
- `proposer.rs` — la regola di chi propone.
- `certificate.rs` — `QuorumCertificate`, `CertificateSignature`, `FinalizedBlock`
  (il `Block` pubblicato) e i loro verificatori.

**Il dominio nuovo.** `Domain::SIG_BLOCK_PREVOTE = "coblox-block-prevote-v0"` in
`hash.rs`; `registry::block_prevote_preimage` in `registry.rs`. Le due preimmagini
condividono `vote_payload`, che è il motivo per cui non possono divergere su una
larghezza di campo. Il dominio esistente `coblox-block-vote-v0` **non cambia**: i
byte prodotti sono identici, e la prova è nella sezione `GATE-NOTHING-PUBLISHED-CHANGED`.

**I documenti.** Solo `docs/protocol/wire.md`: il topic gossip `consensus`,
l'estensione dell'enum `message_type` con `block_proposal`/`prevote`/`precommit`,
la sezione *«The three consensus messages»* con la regola del proponente, le tre
forme di payload, la preimmagine del prevoto, la regola del non-doppio-precommit,
la dichiarazione che **non esiste voto nil**, la dichiarazione che i **tre timeout
sono locali**, e la riga di validazione gossip. **`ledger.md` non e' stato toccato**
— zero righe di diff — e nemmeno `README.md` o `identity.md`.

**Le guardie.** `sim/tools/consensus_no_io.py`, versionata, con `--negative`, e
cinque `[[probe]]` nuove nel manifest di [ADR-012]. Entrambe in CI.

### Files changed

| file | stato | cosa |
| --- | --- | --- |
| `core/coblox-core/src/consensus/mod.rs` | nuovo, 220 righe | il confronto con la fonte, le divergenze, l'argomento no-I/O |
| `core/coblox-core/src/consensus/engine.rs` | nuovo, 882 righe | la macchina a stati |
| `core/coblox-core/src/consensus/messages.rs` | nuovo, 369 righe | i tre messaggi e il confine |
| `core/coblox-core/src/consensus/certificate.rs` | nuovo, 330 righe | `QuorumCertificate`, `FinalizedBlock` |
| `core/coblox-core/src/consensus/proposer.rs` | nuovo, 86 righe | la regola di chi propone |
| `core/coblox-core/src/hash.rs` | +18/-1 | il dominio `coblox-block-prevote-v0` |
| `core/coblox-core/src/registry.rs` | +49/-1 | `block_prevote_preimage`, `vote_payload` |
| `core/coblox-core/src/error.rs` | +71 | `ConsensusError`, dodici varianti |
| `core/coblox-core/src/lib.rs` | +2 | `pub mod consensus` e la riga di tabella |
| `core/coblox-core/Cargo.toml` | +15 | `curve25519-dalek` e `sha2` come **dev**-dependency |
| `docs/protocol/wire.md` | +161/-2 | i tre messaggi, il topic, la regola del proponente, i timeout locali |
| `sim/tools/published_artifacts.toml` | +43/-2 | il dominio nuovo e cinque probe |
| `sim/tools/consensus_no_io.py` | nuovo, 352 righe | la guardia del no-I/O, con prova in negativo |
| `.github/workflows/ci.yml` | +18 | tre passi nuovi |
| `core/coblox-core/tests/consensus_rules.rs` | nuovo, 730 righe | 20 test di regola |
| `core/coblox-core/tests/consensus_devnet.rs` | nuovo, 472 righe | 10 test di rete |
| `core/coblox-core/tests/consensus_support/` | nuovo, 1003 righe | il firmatario RFC 8032 e la devnet a quattro |

**Non toccati, ed e' la premessa di [ADR-018]:** `block.rs`, `quorum.rs`,
`validator_set.rs`, `light_client.rs`, `verifier.rs`, `json.rs`, `merkle.rs`,
`params.rs`, `docs/protocol/ledger.md`, `docs/protocol/README.md`,
`docs/protocol/identity.md`, `Cargo.lock`.

### Verification performed

Tutto eseguito su questo albero, sul commit `7c95267` piu' le modifiche sopra,
`rustc 1.96.0`, Windows 11.

| gate | come | esito |
| --- | --- | --- |
| GATE-CHAIN-EXISTS | `consensus_devnet::four_validators_finalize_a_chain_of_ten_blocks` | dieci blocchi, quaranta certificati verificati |
| GATE-SAFETY-UNDER-ADVERSARY | due sweep, **30** e **500** esecuzioni | nessuna altezza con due blocchi |
| GATE-LIVENESS-AFTER-SILENCE | `a_height_survives_a_proposer_that_says_nothing` e la variante a due proponenti muti | finalizza al round successivo |
| GATE-LOCKING-FROM-SOURCE | confronto riga per riga con arXiv:1807.04938 Algorithm 1 | quattro divergenze dichiarate |
| GATE-NO-IO | forma dell'interfaccia + `consensus_no_io.py` con `--negative` | PASS |
| GATE-NOTHING-PUBLISHED-CHANGED | diff + un test che riproduce una fixture anteriore alla spec | 7 preimmagini riprodotte byte per byte |
| GATE-ADR012-PASS | `published_artifacts.py` e la prova in negativo | PASS, 177 probe provate singolarmente (**180** dopo la remediation di [REVIEW-047]) |

### Verification transcript

#### GATE-LOCKING-FROM-SOURCE — la fonte, e come e' stata verificata

La regola di blocco **non e' derivata**. E' presa da:

> Ethan Buchman, Jae Kwon, Zarko Milosevic, **"The latest gossip on BFT
> consensus"**, arXiv:1807.04938, **Algorithm 1 — Tendermint consensus algorithm**.

Il confronto e' stato fatto sul **sorgente LaTeX dell'e-print**, non su una
descrizione e non a memoria:

```text
$ curl -sL -o tm.tar.gz https://arxiv.org/e-print/1807.04938 && sha256sum tm.tar.gz
138b688f2c8e4dee0ee89b7574aafa7cd99d43bbb8fdca3fc4cba9ee17bbc29f *tm.tar.gz
$ tar xzf tm.tar.gz && sha256sum consensus.tex
7fa4253844ac93c4ef23a3ffeaf4c1fd36c6e2f5e04aec8fbfbbbca4c09f8d3f *consensus.tex
$ grep -n "Tendermint consensus algorithm" consensus.tex
172:    \end{algorithmic} \caption{Tendermint consensus algorithm}
```

La tabella riga-per-riga completa e' in `core/coblox-core/src/consensus/mod.rs`
sezione *The locking rule, line by line* — venti righe di corrispondenza — e ogni
regola in `engine.rs` porta in commento il numero di riga che attua. Le tre righe
su cui poggia la sicurezza, citate dal `.tex` e affiancate al codice:

```text
riga 23  \IF{$valid(v) \wedge (lockedRound_p = -1  \vee lockedValue_p = v$)}
   -->   None => match &self.locked { None => true,
             Some((_, locked)) => locked.block_id == block_id }

riga 29  \IF{$valid(v) \wedge (lockedRound_p \le vr \vee lockedValue_p = v)$}
   -->   Some((locked_round, locked)) =>
             *locked_round < valid_round || locked.block_id == block_id

righe 38-41  $lockedValue_p \assign v$ / $lockedRound_p \assign round_p$ /
             \Broadcast $\li{\Precommit,h_p,round_p,id(v))}$ / $step_p \assign \precommit$
   -->   if self.step == Step::Prevote {
             self.locked = Some((round, value.clone()));
             actions.push(... Precommit ...);
             self.enter_precommit(actions);
         }
```

**Le quattro divergenze, dichiarate e non nascoste.** Ognuna e' argomentata per
esteso in `mod.rs`; qui la sostanza.

1. **Non esistono voti nil.** L'Algorithm 1 li trasmette alle righe 26, 32, 45,
   59, 63 e li conta alle righe 34, 44, 47. La preimmagine pubblicata di
   `ledger.md` sezione *What validators sign* porta `raw_32_bytes(block_id)` e
   **non ha una codifica del nil**; scrivere nil come 32 byte zero avrebbe dato a
   una preimmagine pubblicata un secondo significato, che e' la premessa di
   [ADR-018] che cade. I voti nil nell'Algorithm 1 non decidono e non bloccano
   nulla: servono ad **armare il timer successivo** (righe 35 e 48). Qui ogni
   timer e' armato al cambio di passo, che e' un **soprainsieme** — arma in ogni
   stato in cui l'Algorithm 1 armerebbe, e anche in stati in cui l'Algorithm 1
   resterebbe fermo. Non tocca la sicurezza perche' nessuna regola che blocca,
   precommitta o decide legge un timer. Costo: un round fallito si abbandona a
   scadenza invece che a due terzi di nil, cioe' latenza.
2. **Anche il proponente arma `OnTimeoutPropose`.** La riga 21 lo fa solo nel
   ramo `else`. Qui in entrambi. La guardia interna e' `step_p = propose`, quindi
   il timer in piu' scatta solo per un proponente il cui valore non e' mai
   arrivato, dove l'Algorithm 1 resterebbe appeso. Soprainsieme, stessa ragione.
3. **`f+1` e' una quota di potere, non un conteggio di processi**, perche' questo
   protocollo pesa per `voting_power`. E' `one_correct_threshold`,
   `signed_power * 3 > total_power`. **Non e' in `quorum.rs`**, la cui
   documentazione dichiara un solo predicato senza varianti: questo non e' un
   quorum, autorizza solo il salto di round, e nessuna regola di validita' lo
   confronta con niente.
4. **Il confronto di sblocco e' stretto, e lo chiede [ADR-018].** La riga 29 e'
   `lockedRound_p <= vr`; [ADR-018] parte 2 dice *«a un round **maggiore** di
   quello del proprio blocco»*, e il motore attua l'ADR:
   `*locked_round < valid_round`. Il caso di differenza e' `lockedRound = vr` con
   `v != lockedValue`, che **non puo' accadere** con meno di un terzo del potere
   guasto — due quorum di prevoto allo stesso round per blocchi diversi si
   sovrapporrebbero in piu' di un terzo del potere, e ogni processo nella
   sovrapposizione avrebbe prevotato due volte in un round. Sotto l'ipotesi di
   guasto le due scritture coincidono; fuori da essa quella stretta e' la **piu'
   restrittiva** delle due, e sbloccare e' l'unica mossa di questa regola che puo'
   costare sicurezza. **E' riportata come discrepanza fra [ADR-018] parte 2 e la
   riga 29, non risolta dall'implementatore.**

#### GATE-CHAIN-EXISTS

```text
$ cargo test --test consensus_devnet four_validators_finalize -- --nocapture

running 1 test
--- GATE-CHAIN-EXISTS ---
4 validators, 375 scheduled events, virtual clock 101 ms, 320 messages admitted through the boundary, 40 certificates verified
  height  1 header.round 0 qc.round 0 signatures 3 block_id sha256:d50a809a5ad3fff3 verified true
  height  2 header.round 0 qc.round 0 signatures 3 block_id sha256:6767660cbf44b8cd verified true
  height  3 header.round 0 qc.round 0 signatures 3 block_id sha256:197df01e7d676f66 verified true
  height  4 header.round 0 qc.round 0 signatures 3 block_id sha256:e65129aee48baffd verified true
  height  5 header.round 0 qc.round 0 signatures 3 block_id sha256:83cb79afa4e7c257 verified true
  height  6 header.round 0 qc.round 0 signatures 3 block_id sha256:2a1119b5c82f8e3b verified true
  height  7 header.round 0 qc.round 0 signatures 3 block_id sha256:b8d9be13e0171799 verified true
  height  8 header.round 0 qc.round 0 signatures 3 block_id sha256:82320256f77212b2 verified true
  height  9 header.round 0 qc.round 0 signatures 3 block_id sha256:ffb4a1f47f641320 verified true
  height 10 header.round 0 qc.round 0 signatures 3 block_id sha256:5ea661c292e044eb verified true
test four_validators_finalize_a_chain_of_ten_blocks ... ok
```

`verified true` e' **il verificatore gia' spedito**: `FinalizedBlock::verify`
ricalcola `block_id` dall'header, controlla che il certificato nomini quel blocco
e quella altezza, e poi chiama `QuorumCertificate::verify`, che passa da
`verify_in_context(&ConsensusVerifier, Domain::SIG_BLOCK_VOTE, ...)` firma per
firma e chiude su `quorum::quorum`. `Devnet::finalize` fa questa verifica
**prima** di accettare qualunque blocco in catena, quindi una catena esiste solo
se ogni certificato e' passato. Le firme sono tre su quattro perche' il
certificato e' assemblato nell'istante in cui il quorum e' raggiunto:
`3*3 > 4*2` e' vero, `2*3 > 4*2` e' falso.

#### GATE-SAFETY-UNDER-ADVERSARY — **il numero e' dichiarato, ed e' due numeri**

Sweep sempre attivo, dentro `cargo test --workspace`:

```text
--- GATE-SAFETY-UNDER-ADVERSARY (always-on) ---
executions percorse: 30
  event budget per execution: 500; executions with at least one finalized block: 12; executions with a directed partition: 30; total finalized blocks across all nodes and executions: 156
test no_two_blocks_are_ever_finalized_at_one_height ... ok
```

Sweep esteso, `#[ignore]` sotto `cargo test` ed eseguito in CI in profilo release:

```text
$ cargo test --release --locked -p coblox-core --test consensus_devnet -- --ignored --nocapture

running 1 test
--- GATE-SAFETY-UNDER-ADVERSARY (extended) ---
executions percorse: 500
  event budget per execution: 8000; executions with at least one finalized block: 283; executions with a directed partition: 484; total finalized blocks across all nodes and executions: 51228
test the_extended_adversarial_sweep ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 89.20s
```

**Perche' due numeri e non uno.** Ogni messaggio di ogni esecuzione attraversa una
verifica Ed25519 reale al confine, e in profilo debug quella verifica costa circa
cento volte quello che costa in release: lo sweep da 500 esecuzioni misurato qui
e' 89 s in release e sarebbe oltre due ore in debug — misurato, non stimato: 120
esecuzioni x 1500 eventi hanno impiegato 591 s in debug. Renderlo economico
facendo passare i messaggi senza verifica avrebbe scambiato un numero con la
misura di niente. Quindi: **30 esecuzioni x 500 eventi** su ogni `cargo test`, e
**500 esecuzioni x 8000 eventi** in un passo release di CI, aggiunto in `ci.yml`.
Le due meta' sono dichiarate perche' presentare solo la seconda rivendicherebbe
una guardia che la pipeline non avrebbe.

Lo scheduler avverso, per esecuzione: ritardo di consegna estratto in
`1..=max_delay_ms` con `max_delay_ms` da 1 a 400; **partizione diretta**, ognuna
delle dodici coppie ordinate distinte tagliata con probabilita' 1/4 (quindi anche
i casi asimmetrici, un validatore che invia e non riceve); duplicazione di ogni
messaggio con probabilita' 1/2. 484 delle 500 esecuzioni avevano almeno una coppia
tagliata. L'asserzione e' `assert_no_conflicting_finality` piu'
`assert_chains_agree` piu' `assert_all_certificates_verify`, su **ogni** esecuzione.

Il test rifiuta anche di essere vacuo: se meno di un quinto delle esecuzioni
finalizzasse qualcosa, fallisce, perche' una suite di sicurezza in cui nulla
finalizza dimostra solo che un protocollo che non decide non decide due volte.

E la controprova nell'altro verso — una partizione che nega il quorum a entrambi
i lati **stalla e non forka**:

```text
--- partition without a quorum ---
4000 events, virtual clock 2806421 ms, node rounds [210, 210, 210, 210], nothing finalized
test a_split_that_denies_both_sides_a_quorum_finalizes_nothing ... ok
```

#### GATE-LIVENESS-AFTER-SILENCE

```text
--- GATE-LIVENESS-AFTER-SILENCE ---
proposer of (height 1, round 0) is val-001, silenced. Height 1 finalized at round 1 proposed by val-002, after 46 scheduled events and 300 ms of virtual clock.
  height  1 header.round 1 qc.round 1 signatures 3 block_id sha256:9b622a28e83bf29e verified true
test a_height_survives_a_proposer_that_says_nothing ... ok
test a_height_survives_two_consecutive_mute_proposers ... ok
test one_validator_that_never_speaks_does_not_stop_the_chain ... ok
```

Il nodo muto e' **per il resto interamente corretto**: prevota, precommitta, e' e
resta contato in ogni quorum. Solo la sua proposta di quel round non lascia il
nodo. Modellare il silenzio come un messaggio scartato invece che come un motore
modificato e' cio' che tiene onesto il motore. La variante a due proponenti muti
consecutivi finalizza al round >= 2; la variante a un validatore completamente
muto produce cinque blocchi con gli altri tre.

Il test `consecutive_rounds_visit_every_member_before_repeating` chiude l'altro
capo: la regola del proponente **non puo'** ripetere un proponente in quattro
round consecutivi della stessa altezza, che e' l'obbligo su cui questa vivacita'
poggia.

#### GATE-NO-IO — dalla forma dell'interfaccia

La dimostrazione primaria e' la firma, non un commento:

```rust
pub fn step_event(&mut self, event: Event) -> Result<Vec<Action>>
```

`Event` ed `Action` sono enum di soli dati. `Engine` non e' generico, non ha
parametri di lifetime, non contiene trait object, closure, `Rc`, `Arc`, `Cell`,
`Mutex` ne' puntatori grezzi. Le tre cose per cui un motore di consenso uscirebbe
sono **invertite**: il tempo entra come `Event::Timeout` ed esce come
`Action::ScheduleTimeout`; il valore da proporre entra come `Event::Value` dopo
`Action::RequestValue`; **la chiave di firma non entra mai**, perche'
`Action::Broadcast` emette i voti **non firmati** e li firma il chiamante. La
verifica di firma sta al confine, in `messages.rs`, quindi il motore non e'
generico neanche su `SignatureVerifier`.

La seconda meta' e' versionata ed e' in CI:

```text
$ python3 sim/tools/consensus_no_io.py
  N1-IO-PATH       1869 candidate(s) checked
  N2-ENGINE-SEAM   879 candidate(s) checked
  N3-BOUND         3 candidate(s) checked

consensus engine no-I/O lint: PASS

$ python3 sim/tools/consensus_no_io.py --negative

=== N1-IO-PATH ===
defect reintroduced: the engine learns the time by itself, which is the single change that would make every adversarial schedule in the suite unreproducible while leaving the happy path green
  FAIL N1-IO-PATH: core/coblox-core/src/consensus/engine.rs:493 names 'std::time'. ...
  names N1-IO-PATH: True

=== N2-ENGINE-SEAM ===
defect reintroduced: the engine takes a callback for the value to propose instead of asking for it and being told - the shape in which a mempool, and with it a socket, arrives inside the state machine
  FAIL N2-ENGINE-SEAM: core/coblox-core/src/consensus/engine.rs:335 introduces a trait object: 'pub fn with_value_source(&mut self, _source: Box<dyn Iterator<Item = u64>>) {}'. ...
  names N2-ENGINE-SEAM: True

=== N3-BOUND ===
defect reintroduced: a second seam appears at the message boundary, so the module is generic over a trait nobody decided on
  FAIL N3-BOUND: core/coblox-core/src/consensus/messages.rs is generic over 'Clone'. ...
  names N3-BOUND: True

negative proof: PASS - 3 mutations across 3 defect classes, each observed failing
```

Lo strumento dichiara i propri limiti nella propria intestazione: legge testo,
non prova che i tipi nominati nelle firme siano a loro volta privi di I/O, non
esegue il compilatore, ed **e' un lint e non un confine**.

#### GATE-NOTHING-PUBLISHED-CHANGED

Dal diff, la lista dei file **non toccati** e' verificata e non dichiarata:

```text
$ git diff --stat -- core/coblox-core/src/block.rs core/coblox-core/src/quorum.rs \
      core/coblox-core/src/validator_set.rs docs/protocol/ledger.md docs/protocol/README.md
(nessun output: zero righe di diff)
```

`BlockHeader`, lo schema di `QuorumCertificate`, il predicato di quorum e la
sezione *What validators sign* di `ledger.md` sono quindi letteralmente immutati.
`registry.rs` **e'** toccato — la preimmagine del prevoto nasce accanto a quella
del precommit e le due condividono ora `vote_payload` — quindi la domanda che un
revisore ha diritto di fare e' se quel payload condiviso sia il payload che
c'era prima. La risposta non e' un'opinione:

```text
$ cargo test --test consensus_rules the_finality_vote_preimage -- --nocapture
GATE-NOTHING-PUBLISHED-CHANGED: 7 pre-existing finality-vote preimages reproduced byte for byte by the refactored block_vote_preimage
test the_finality_vote_preimage_still_reproduces_a_fixture_older_than_this_spec ... ok
```

`tests/fixtures/ed25519_coblox_extension.json` porta sette vettori il cui campo
`message` e' una **preimmagine di voto di finalita' intera**, generata da
`sim/tools/ed25519_coblox_extension_vectors.py`, pubblicata da [SPEC-012] e non
modificata da questa spec. Il test la ri-analizza in
`(chain_id, height, round, block_id)` e pretende che `block_vote_preimage` la
ricostruisca byte per byte. E' la seconda strada: la fixture viene da un
generatore Python che non condivide codice con questo crate, e quel generatore
continua a riprodurla:

```text
$ python3 sim/tools/ed25519_coblox_extension_vectors.py
core/coblox-core/tests/fixtures/ed25519_coblox_extension.json: reproduces byte for byte
```

#### GATE-ADR012-PASS

```text
$ python3 sim/tools/published_artifacts.py
  C1-DOMAIN         41 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     20 candidate(s) checked
  C4-VALUE          60 candidate(s) checked
  C5-MIRROR         53 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C5-DISCOVERED     67 candidate(s) checked
  C10-PROBE        177 candidate(s) checked
  C11-CLAIMDOC       8 candidate(s) checked

published-artifact inventory: PASS
```

`C1-DOMAIN` passa da 40 a **41**: il conteggio che [ADR-018] fa dei domini
esistenti — quaranta — e' confermato dallo strumento, e il quarantunesimo e'
quello che questa spec aggiunge. `C10-PROBE` passa da 172 a **177**: cinque probe
nuove, `consensus-prevote-preimage`, `consensus-proposer-index-is-not-chosen`,
`consensus-header-round-not-rewritten`, `consensus-has-no-nil-vote`,
`consensus-timeouts-are-local`.

La prova in negativo le esercita **una per una**, perche' `prove_every_probe`
cancella dal documento il passaggio che ogni probe dichiara di fissare e pretende
che lo strumento fallisca nominando quella probe:

```text
$ python3 sim/tools/published_artifacts_negative.py
=== C10-PROBE, every probe individually ===
deleting each probe's own pinned passage from its own document, 177 case(s)
  every one of the 177 probes was observed failing
...
negative proof: PASS - 17 mutations across 11 defect classes, plus every probe individually, each observed failing
```

#### Equivocazione e determinismo

```text
--- equivocation ---
8 conflicting precommits injected under val-000's real key across 4 rounds; chain length 10, no height with two block IDs, no certificate with a repeated signer
test a_validator_that_precommits_twice_finalizes_only_one_block ... ok
test one_validator_cannot_reach_a_quorum_by_voting_many_times ... ok

--- determinism ---
two runs of seed 8675309: 8190 chain bytes per node, identical on all four nodes
test the_same_schedule_produces_the_same_chain_byte_for_byte ... ok
```

I voti in conflitto sono firmati con la **chiave vera** dell'equivocatore, quindi
passano il confine: il test riguarda cosa il motore fa con due firme valide, non
il controllo di firma. La difesa non e' una regola di rilevamento — e'
`precommit_of`, che tiene il **primo** precommit di ogni `(round, validator)` e
scarta un secondo diverso, cosi' che il potere di un validatore raggiunga al piu'
un `block_id` per round nel conteggio di qualunque nodo onesto, che
l'equivocazione sia notata o no. Il test verifica anche che nessun certificato
prodotto conti due volte lo stesso firmatario.

Il determinismo e' affermato **con il perimetro su cui vale**: la stessa sequenza
di eventi sullo stesso nodo produce gli stessi byte di `Block`, certificato
compreso. Fra nodi diversi i **blocchi** sono identici e i **certificati** possono
differire nel numero di firme, perche' un certificato e' assemblato con i
precommit che quel nodo aveva quando il quorum si e' chiuso; `block_id` non copre
il certificato, quindi questo e' normale e non e' un difetto. Il test controlla
anche che un seme diverso produca una linea temporale diversa, altrimenti
l'uguaglianza sopra sarebbe vera di tutto.

#### Il firmatario dei test, e i suoi due oracoli

Il crate spedisce un verificatore e nessun firmatario. Una catena firmata da uno
stub non direbbe niente sui certificati che porta, quindi la suite ha un
firmatario, scritto da RFC 8032 sezione 5.1.6 in
`tests/consensus_support/ed25519_signer.rs`, e **provato prima di essere usato**:

```text
$ cargo test --test consensus_rules the_test_signer -- --nocapture
RFC 8032 7.1 TEST 1: public key and signature reproduce (0 message byte(s))
RFC 8032 7.1 TEST 2: public key and signature reproduce (1 message byte(s))
RFC 8032 7.1 TEST 3: public key and signature reproduce (2 message byte(s))
test the_test_signer_reproduces_rfc_8032_section_7_1 ... ok
```

I vettori sono trascritti da `https://www.rfc-editor.org/rfc/rfc8032.txt`
(`sha256 ed63657ff389301282b169b0abde9b5dd2c7e4d524fdfa5da6ff3094fc93c4c3`),
sezione 7.1, `TEST 1`/`TEST 2`/`TEST 3`, con le sole interruzioni di riga
rimosse. Il secondo oracolo e' il `ConsensusVerifier` spedito, che e' una lettura
indipendente della stessa curva — ZIP-215 invece di 5.1.6 — e che accetta ogni
firma prodotta qui e rifiuta un suo bit invertito. Un firmatario provato solo
contro il verificatore che lo accettera' e' il difetto che [[recurring-defects]]
famiglia 1 registra come *«il test confrontava l'implementazione con se' stessa
attraverso due copie»*.

Nessun package nuovo entra nel grafo: `curve25519-dalek` e `sha2` sono i due che
la libreria gia' usa, ri-dichiarati come **dev**-dependency, e `Cargo.lock` non
cambia.

#### Le tre passate di progetto

```text
$ cargo test --workspace --all-features
     Running unittests src\lib.rs           35 passed; 0 failed
     tests\authorization_unrevoked.rs        12 passed; 0 failed
     tests\cadence_and_reward_epoch.rs       19 passed; 0 failed
     tests\canonical_serialization.rs         6 passed; 0 failed
     tests\conformance_registry.rs           26 passed; 0 failed
     tests\consensus_devnet.rs                9 passed; 0 failed; 1 ignored
     tests\consensus_rules.rs                20 passed; 0 failed
     tests\constraint_block.rs               24 passed; 0 failed
     tests\election_degenerate.rs            12 passed; 0 failed
     tests\genesis_derivation.rs              9 passed; 0 failed
     tests\identity_revocation.rs             7 passed; 0 failed
     tests\light_client_perimeter.rs         14 passed; 0 failed
     tests\preimage_context.rs                5 passed; 0 failed
     tests\sparse_account_state.rs            8 passed; 0 failed
     tests\speccheck_conformance.rs          11 passed; 0 failed
     tests\worked_example.rs                  6 passed; 0 failed
     coblox_ffi                               1 passed; 0 failed

$ cargo clippy --workspace --all-features --all-targets -- -D warnings
(nessuna diagnostica; exit 0)

$ cargo fmt --all --check
(nessun output; exit 0)

$ cargo build --locked --workspace
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.65s
```

Le altre guardie di progetto, tutte eseguite su questo albero:

```text
== published_artifacts ==             exit=0  published-artifact inventory: PASS
== protocol_hashes ==                 exit=0  every published value reproduced: PASS
== reward_rules ==                    exit=0  cases: 58, mismatches: 0 / GATE-RULES-REJECT: PASS
== genesis_chain_id ==                exit=0  ok
== non_consensus_containment ==       exit=0  ok
== consensus_parameters_closure ==    exit=0  PASS: all 22 fields covered
== consensus_no_io ==                 exit=0  consensus engine no-I/O lint: PASS
== ed25519_coblox_extension_vectors ==exit=0  reproduces byte for byte
== ed25519_speccheck_oracle ==        exit=0
== threat_model_matrix_coherence ==   exit=0  celle: 104 coperte: 97 n/a: 7 scenari: 43
== lead_claims_check ==               exit=0  lead-claims: PASS
```

### Known limitations

Elencate perche' un criterio lasciato aperto e dichiarato vale piu' di uno
dichiarato chiuso e non chiuso.

1. **Non esiste ancora una devnet.** La spec lo dice in apertura e resta vero:
   quattro motori in un processo su un trasporto in memoria. Rete, persistenza e
   ciclo di vita del nodo sono la spec successiva, e `coblox-node/src/main.rs`
   non e' stato toccato.
2. **Nessun motore bizantino e' esercitato.** L'avversario e' lo *scheduler* —
   riordina, ritarda, duplica, partiziona — piu' due comportamenti scorretti
   iniettati: un proponente silenziato e un doppio precommit firmato con la
   chiave vera. Un nodo le cui **regole** sono sbagliate non e' provato.
3. **L'equivocazione non e' provabile a terzi per le proposte.** Una proposta non
   porta firma propria: la sua autenticita' e' quella della busta. Un proponente
   che manda due proposte diverse in un round e' rilevabile da chi le riceve
   entrambe e **non attribuibile** da un payload isolato. Aggiungere un quarto
   dominio di firma sarebbe superficie pubblicata che [ADR-018] non porta, quindi
   non e' stato fatto; il residuo e' nominato in `messages.rs` e in `wire.md`.
4. **Il buffering fra altezze e' del chiamante.** Il motore tiene il log
   dell'altezza che esegue e scarta il resto, che e' lo scoping `h_p`
   dell'Algorithm 1. La devnet ri-consegna; in produzione lo fa il ledger sync di
   `wire.md`. Tenere dentro il motore un numero illimitato di altezze future
   sarebbe una memoria che un peer remoto fa crescere.
5. **`valid(v)` oltre altezza e legame col genitore e' del chiamante.** Il motore
   verifica `header.height` e `header.previous_block_id`; il `state_root` richiede
   un esecutore, che non e' materia di consenso e non e' in questo crate.
6. **I valori dei timeout non sono tarati** — la spec lo escludeva. Quelli del
   banco di prova sono scelti perche' una prova non aspetti, e nessun test
   asserisce una magnitudine.
7. **Nessun esempio JSON canonico** e' stato aggiunto in `wire.md` per i tre
   messaggi. Il catalogo esistente ne ha per alcuni messaggi e non per altri
   (`EnrollmentResponse` non ne ha), quindi la forma e' ammessa; un esempio
   avrebbe pero' introdotto letterali di digest da inventariare in
   `published_artifacts.toml`, e nessuno di essi sarebbe stato derivato da una
   catena reale. E' un'aggiunta possibile per una passata successiva.
8. **`consensus_no_io.py` e' un lint, non un confine.** Legge testo. Un percorso
   di I/O raggiunto tramite un alias passerebbe.
9. **La sicurezza e' provata per esplorazione, non per dimostrazione.** 530
   esecuzioni avverse in totale sono una ricerca, non una prova di modello. Una
   verifica formale della regola di blocco non e' stata fatta e non era in scope.

---

## Remediation di REVIEW-047
> Compilata da AGENT-002 il 2026-08-27, sui tre rilievi bloccanti piu' RF-005.
> La spec resta in `review`.

### Che cosa e' cambiato, rilievo per rilievo

**RF-001 (high) — il carico e' ora legato al blocco.** `verify_proposal` ricalcola
`transactions_root` dai `transactions` portati e rifiuta la proposta che non
riproduce `header.transactions_root`. Il controllo e' il **quinto** dell'elenco
del doc-comment ed e' fatto **prima** che qualunque regola possa prevotare, cioe'
nella stessa classe di `links_to_the_chain` e non in quella di `state_root`: non
serve un esecutore, perche' `registry::tx_id` e `merkle::transactions_root` sono
gia' in questo crate. La radice e' presa sull'oggetto **senza `authorization`**,
come `ledger.md#unsigned-transaction-and-authorization` definisce `tx_id`; la
rimozione e' fatta nel confine e non assunta del chiamante, perche' un ricevente
che hashasse l'oggetto firmato rifiuterebbe *ogni* proposta onesta — che e' il
modo peggiore in cui una regola di rifiuto puo' sbagliare, ed e' pinnato da
`the_boundary_computes_the_root_over_the_unsigned_transaction`.

Variante d'errore nuova: `ConsensusError::ProposalTransactionsRootMismatch`, che
porta **entrambe** le radici perche' un log che dicesse solo «radice sbagliata»
non distinguerebbe un payload troncato da un proponente che ha mandato un header
con due carichi.

La frase falsa di `messages.rs` — *«il valore su cui si accorda e' `block_id`, e
`block_id` li copre attraverso `transactions_root`»* — e' corretta: copriva
l'hash e nessuno lo confrontava, e ora il doc-comment del campo dice quale delle
due cose fa il confine.

`wire.md#block_proposal` porta il MUST del ricevente, e la riga di validazione
gossip lo ripete nella forma in cui un ricevente la applica. Probe di [ADR-012]:
`consensus-proposal-payload-reproduces-its-root`.

**RF-002 (high) — `header.round` e' imposto dove va imposto.** Nel ramo
`valid_round: None` il confine esige `header.round == proposal.round`. Nel ramo
`Some(vr)` **non** c'e' alcun confronto, ed e' scritto perche' non c'e' invece di
essere lasciato dedurre: una ri-proposta porta l'header del round di prima
proposta, `block_id` copre ogni byte dell'header, e il ricevente agisce su una
ri-proposta solo dopo aver visto **nel proprio log** oltre due terzi di prevoti
per quello stesso `block_id` a `vr` — un quorum che il proponente non puo'
fabbricare. Il punto 3 del doc-comment, che dichiarava un controllo inesistente,
e' riscritto in due punti (3 e 4) che descrivono i controlli che esistono.
`wire.md` porta la regola del ricevente **e** l'esenzione, perche' il
rafforzamento ingenuo — confrontare i round su ogni proposta — rifiuterebbe ogni
ri-proposta e stallerebbe ogni altezza a due round. Probe:
`consensus-first-hand-proposal-carries-its-own-round`.

**RF-003 (medium) — l'affermazione pubblicata e' ristretta al fatto.** Presa la
strada **(a)** della review: l'indice **non** e' cambiato, perche' cambiarlo e'
una modifica di [ADR-018] §3 e non e' dell'implementatrice. `wire.md` e
`proposer.rs` dicono ora che la proprieta' vale **a potere uniforme** — cio' che
`check_elected_shape` impone a un set eletto — e dichiarano accanto che a potere
pesato un membro di potere `w` propone in `w` round consecutivi, con la
conseguenza sulla vivacita' (attesa quadratica in `w`, perche' il timeout cresce
col numero di round) e con la nota che la **sicurezza non e' toccata**: la regola
autorizza a proporre, e una proposta da sola non decide niente. Probe:
`consensus-proposer-property-holds-at-uniform-power`.

**RF-005 (medium, documentazione) — le tre frasi della Divergenza 1.** Nessuna
modifica di codice.

- (a) L'argomento *«no rule that locks, precommits or decides reads a timer»* era
  falso e ora e' dichiarato falso nel testo che lo sostituisce:
  `try_lock_and_precommit` e' guardato da `self.step`, che `on_timeout` scrive.
  L'argomento vero e' la **direzione**: i due timer che questa divergenza arma in
  anticipo possono solo portare il passo **oltre** il punto in cui blocco e
  precommit del round sono ancora possibili, o abbandonare il round. Possono solo
  **sopprimere** un blocco o un precommit, mai causarne uno, e sopprimere e'
  sicuro per costruzione — chi blocca di meno puo' solo non aiutare un quorum a
  formarsi, mai aiutarne due — mentre un blocco gia' preso sopravvive al cambio
  di round, perche' `locked` si azzera solo su una decisione.
- (b) L'assenza del nil perde informazione che l'Algorithm 1 usa **alla riga 55**,
  la regola di salto di round `f+1`, e non alle tre elencate (34, 44, 47). Senza
  nil un round il cui proponente tace non produce **alcun** messaggio onesto,
  quindi un nodo in ritardo non riceve da quel round alcun segnale di salto e
  cammina i round uno alla volta pagando timeout crescenti; composto con RF-003 —
  un membro pesante muto che tiene `w` round — il recupero e' **quadratico**. E'
  un costo di recupero e non di sicurezza: `try_skip_round` non decide niente.
- (c) Armare `OnTimeoutPrevote` al cambio di passo e' un soprainsieme
  nell'**armare** e non nel **comportamento**, ed e' dichiarato come il costo di
  vivacita' che e': l'Algorithm 1 garantisce un `timeoutPrevote(round_p)` intero
  **dopo** il quorum di prevoti; qui il timer puo' scadere prima che arrivi un
  solo prevoto, e da `Precommit` il nodo non puo' piu' bloccarsi ne'
  precommittare quel round.

### Files changed nella remediation

| file | delta | cosa |
| --- | --- | --- |
| `core/coblox-core/src/consensus/messages.rs` | +85/-8 | il legame carico-blocco, `header.round` nel ramo di prima mano, `transactions_root_of`, i due doc-comment corretti |
| `core/coblox-core/src/consensus/mod.rs` | +58/-12 | le tre frasi della Divergenza 1 |
| `core/coblox-core/src/consensus/proposer.rs` | +34/-5 | la proprieta' ristretta al potere uniforme e il caso pesato |
| `core/coblox-core/src/error.rs` | +14 | `ProposalTransactionsRootMismatch` |
| `docs/protocol/wire.md` | +46/-6 | i due MUST del ricevente, la riga gossip, la frase del proponente ristretta |
| `sim/tools/published_artifacts.toml` | +21 | tre probe |
| `core/coblox-core/tests/consensus_rules.rs` | +241/-29 | cinque test nuovi, uno rinominato al vero |
| `core/coblox-core/tests/consensus_devnet.rs` | +163/-11 | E5 invertito |
| `core/coblox-core/tests/consensus_support/devnet.rs` | +90/-6 | `harness_transaction`, `harness_transactions_root`, `inject_to`, il contatore dei rifiuti |

**Ancora non toccati:** `ledger.md`, `README.md`, `identity.md`, `block.rs`,
`quorum.rs`, `registry.rs`, `hash.rs`, `merkle.rs`, `Cargo.lock`. La premessa di
[ADR-018] regge anche dopo questa passata.

### Verification transcript della remediation

#### Ogni regola nuova osservata fallire su un albero mutato

Le due regole nuove di `verify_proposal` sono state disattivate (`if false && ...`)
e i test sono stati eseguiti sull'albero mutato. L'albero e' stato ripristinato da
una copia presa **prima** della mutazione, in `scratchpad/messages.rs.bak`, non
con `git checkout`.

```text
$ cargo test -p coblox-core --test consensus_rules -- a_first_hand_proposal \
    a_proposal_whose_payload the_boundary_computes_the_root a_re_proposal
test a_first_hand_proposal_must_carry_its_own_round_in_the_header ... FAILED
test the_boundary_computes_the_root_over_the_unsigned_transaction ... ok
test a_re_proposal_keeps_the_round_the_value_was_first_proposed_at ... ok
test a_proposal_whose_payload_does_not_reproduce_its_root_is_refused ... FAILED

thread 'a_first_hand_proposal_must_carry_its_own_round_in_the_header' panicked at
consensus_rules.rs:374:5:
assertion failed: matches!(verify_proposal(&chain_id(), &set, &proposer, proposal,
thread 'a_proposal_whose_payload_does_not_reproduce_its_root_is_refused' panicked at
consensus_rules.rs:465:9:
a payload the header does not commit to was admitted

test result: FAILED. 2 passed; 2 failed
```

I due che restano verdi sono i due che **devono** restarlo, ed e' il controllo che
vale piu' dei due rossi: `a_re_proposal_...` e `the_boundary_computes_the_root_...`
sono le direzioni in cui una regola **troppo forte** romperebbe il protocollo, non
quelle in cui una regola assente lo lascia rotto.

#### E5 invertito — misurato prima e dopo

**Prima** (albero mutato, il legame carico-blocco disattivato): la costruzione
della review si riproduce esattamente. Un proponente muto, due proposte con lo
**stesso header** e due carichi diversi verso due nodi ciascuna, quorum di
prevoti e di precommit, e due `Block` diversi allo stesso `block_id`, entrambi
accettati da `FinalizedBlock::verify` col verificatore spedito — che e' cio' che
`Devnet::finalize` esige prima di ammettere un blocco in catena:

```text
thread 'one_header_with_two_payloads_does_not_produce_two_blocks' panicked at
consensus_devnet.rs:522:9:
node 0 and node 2 published different `Block` bytes for the same chain
(3113 and 3109 bytes).
Node 0's first block carries [{... "body":{"amount_microtokens":"250000",
  "pay_to":"an-ordinary-payee"} ...}];
node 2's carries [{... "body":{"amount_microtokens":"1000000",
  "pay_to":"the-attacker"} ...}].
```

Le due catene divergono al **primo** blocco; `assert_no_conflicting_finality` e
`assert_chains_agree` sono passate entrambe — perche' asseriscono su
`(height, block_id)` — e `assert_all_certificates_verify` e' passata anche lei.
E' esattamente il punto di RF-001: il criterio di sicurezza della spec resta vero
e l'artefatto pubblicato diverge.

**Dopo** (albero ripristinato):

```text
$ cargo test -p coblox-core --test consensus_devnet -- one_header_with_two_payloads --nocapture
--- E5 inverted: one header, two payloads ---
proposer of (height 1, round 0) is val-001, silenced; two proposals carrying the
identical header sha256:cdd9069e5ab89e41 injected, payload A of 1 transaction(s)
to nodes [1, 0], payload B (`the-attacker`, 1000000 microtokens) of 1
transaction(s) to nodes [2, 3]
  boundary refusals during injection: 2; published `Block` bytes per node:
  [2730, 2730, 2730, 2730]; identical: true
  height  1 header.round 1 qc.round 1 signatures 3 block_id sha256:9b622a28e83bf29e verified true
  height  2 header.round 0 qc.round 0 signatures 3 block_id sha256:2372bda384ece014 verified true
test one_header_with_two_payloads_does_not_produce_two_blocks ... ok
```

Entrambe le copie del carico che l'header non impegna sono respinte al confine; il
round 0 non raccoglie quorum e l'altezza riesce al round 1 con una proposta il cui
carico il suo header impegna; i quattro nodi pubblicano gli **stessi 2730 byte**.

#### E2 invertito

```text
$ cargo test -p coblox-core --test consensus_rules -- a_first_hand_proposal a_re_proposal
test a_first_hand_proposal_must_carry_its_own_round_in_the_header ... ok
test a_re_proposal_keeps_the_round_the_value_was_first_proposed_at ... ok
test result: ok. 2 passed; 0 failed
```

Il primo e' la proposta di prima mano con `header.round = 424242` al round 0,
respinta con `ProposalHeaderMismatch { field: "round" }`. Il secondo e' la meta'
necessaria: la ri-proposta ai round 1..4 che porta l'header del round 0 resta
**accettata**, a tutti e quattro i round, perche' un'implementazione che la
rifiutasse stallerebbe ogni altezza che richiede un secondo round.

#### RF-003 — E1 riprodotto come test

```text
$ cargo test -p coblox-core --test consensus_rules -- at_weighted_power --nocapture
--- RF-003: the proposer rule at weighted power ---
powers [1, 1, 1, 7], height 1, rounds 0..12 -> ["val-001", "val-002", "val-003",
  "val-003", "val-003", "val-003", "val-003", "val-003", "val-003", "val-000",
  "val-001", "val-002"]
longest consecutive run by one proposer: 7
test at_weighted_power_a_member_proposes_in_as_many_consecutive_rounds_as_its_power ... ok
```

Il test asserisce tre cose e non una: che la corsa piu' lunga sia **esattamente**
il potere del membro pesante; che la corsa cominci **prima** che ogni membro abbia
proposto; e che quattro round consecutivi **non** nominino quattro membri
distinti, cioe' che il set pesato non si sia comportato come uno uniforme — senza
quest'ultima il test resterebbe verde anche se qualcuno rendesse uniforme il set.
Il test uniforme e' rinominato
`consecutive_rounds_visit_every_member_before_repeating_at_uniform_power` e
asserisce di essere su un set uniforme **prima** di asserire la proprieta'.

#### Le passate di progetto, rieseguite per intero

```text
$ cargo test --workspace
  coblox_core (unit)          35 passed
  authorization_unrevoked     12 passed
  cadence_and_reward_epoch    19 passed
  canonical_serialization      6 passed
  conformance_registry        26 passed
  consensus_devnet            10 passed, 1 ignored     (50.12s)
  consensus_rules             25 passed
  constraint_block            24 passed
  election_degenerate         12 passed
  genesis_derivation           9 passed
  identity_revocation          7 passed
  light_client_perimeter      14 passed
  preimage_context             5 passed
  sparse_account_state         8 passed
  speccheck_conformance       11 passed
  worked_example               6 passed
  coblox_ffi                   1 passed
  0 failed in every target

$ cargo clippy --workspace --all-features --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s)

$ cargo fmt --all --check
    (nessun diff)
```

```text
$ python sim/tools/consensus_no_io.py                  exit=0  consensus engine no-I/O lint: PASS
$ python sim/tools/consensus_parameters_closure.py     exit=0  PASS: all 22 fields covered
$ python sim/tools/published_artifacts.py              exit=0  published-artifact inventory: PASS
$ python sim/tools/published_artifacts_negative.py     exit=0  negative proof: PASS - 17 mutations
                                                               across 11 defect classes, plus every
                                                               probe individually, each observed failing
$ python sim/tools/protocol_hashes.py                  exit=0  every published value reproduced: PASS
$ python sim/tools/genesis_chain_id.py                 exit=0  ok
$ python sim/tools/non_consensus_containment.py        exit=0  ok
$ python sim/tools/threat_model_matrix_coherence.py    exit=0  OK: matrice e scenari coerenti
$ python sim/tools/lead_claims_check.py                exit=0  lead-claims: PASS
$ python sim/tools/reward_rules.py                     exit=0  GATE-RULES-REJECT: PASS
$ python sim/tools/auth0_oracle.py                     exit=0  AUTH-0 second derivation: PASS
$ python sim/tools/ed25519_coblox_extension_vectors.py exit=0  reproduces byte for byte
$ python sim/tools/ed25519_speccheck_oracle.py         exit=0
$ node .lmbrain/design/coblox-public-guide/tools/check-guide-pairs.mjs
                                                       exit=0  public-guide form check: PASS
```

`published_artifacts.py` conta ora **180** probe, tre in piu' delle 177 della
consegna, e la prova in negativo le esercita **una per una**: la riga «plus every
probe individually, each observed failing» e' cio' che chiude la condizione (c) di
RF-001 senza che io debba dichiararlo a parte.

### Stato delle gate `owner=agent` dopo la remediation

| gate | stato | perche' |
| --- | --- | --- |
| GATE-CHAIN-EXISTS | **vera** | rieseguita: dieci blocchi, certificati verificati |
| GATE-SAFETY-UNDER-ADVERSARY | **vera** | 30 esecuzioni sempre-attive rieseguite; nessuna altezza con due `block_id`, **e ora nessuna altezza con due `Block`** |
| GATE-LIVENESS-AFTER-SILENCE | **vera** | rieseguita; la proprieta' su cui poggia e' ora pubblicata nella forma vera (RF-003) |
| GATE-LOCKING-FROM-SOURCE | **vera** | la regola di blocco non e' cambiata; le **ragioni** della Divergenza 1 sono state corrette (RF-005), e la riga 55 e' ora contata fra quelle a cui i nil servono |
| GATE-NO-IO | **vera** | `consensus_no_io.py` PASS; il controllo nuovo e' in `messages.rs`, che e' il confine e non il motore, e non introduce ne' generici ne' `dyn` |
| GATE-NOTHING-PUBLISHED-CHANGED | **vera** | il diff della remediation non tocca `ledger.md`, `README.md`, `identity.md`, `registry.rs`, `hash.rs`, `block.rs`, `quorum.rs`, `merkle.rs`. `wire.md` cambia, ed e' il documento che questa spec pubblica |
| GATE-ADR012-PASS | **vera** | 180 probe, PASS, e la prova in negativo osserva fallire anche le tre nuove |

### Cio' che resta non verificato, e cio' che non e' stato toccato

1. **`Engine::on_value` non ricalcola la radice del valore che il chiamante gli
   porge.** Impone gia' `header.height` e `header.round`, ma non confronta
   `header.transactions_root` coi `transactions`. Non e' un buco raggiungibile da
   un pari — `on_value` risponde a una `RequestValue` che il motore stesso ha
   emesso — e la proposta che ne esce passa comunque dal confine di **ogni**
   ricevente, incluso il nodo stesso quando il trasporto gliela riconsegna: un
   chiamante che costruisse un valore incoerente vedrebbe la propria proposta
   rifiutata da tutti. La review chiede il legame **al confine** e li' e' stato
   messo; l'aggiunta simmetrica in `on_value` cambierebbe l'errore da «tutti
   rifiutano» a «il motore rifiuta subito» e non e' stata fatta perche' e' fuori
   dal rimedio richiesto. **Nominata qui, non aperta come debito.**
2. **Nessuna prova di equivocazione di proposta oltre quella di RF-001.** RF-006
   e' non bloccante e resta al triage del Lead. Il test di E5 invertito e' pero'
   la prima proposta equivocante che questa suite produca davvero — cosa su cui
   `messages.rs`, `mod.rs` e `wire.md` ragionavano senza esercitarla.
3. **Il costo del controllo nuovo non e' misurato.** `verify_proposal` fa ora una
   passata di SHA-256 sull'array per ogni proposta ricevuta. Sul banco di prova il
   carico e' di zero o una transazione e il tempo della suite non si e' mosso; su
   un blocco pieno (16.384 transazioni, il tetto di `ledger.md`) il costo e' reale
   e non e' stato profilato. Non e' una regressione di sicurezza — il tetto del
   Merkle e' imposto dalla stessa chiamata, quindi una proposta oltre il limite e'
   ora respinta **al confine** invece che accettata — ma e' una superficie di
   costo per messaggio che prima non c'era, e va detta.
4. **RF-004, RF-006, RF-009 e RF-010 non sono stati lavorati**, come da incarico:
   sono non bloccanti e il loro triage e' del Lead. **RF-007 e RF-008 sono del
   Lead** e non sono stati toccati.

### Il residuo di RF-003, per il Lead

**Nessun percorso di consenso chiama `ValidatorSet::check_elected_shape`.**
`Engine::start` e `proposer_at` chiamano `check_structure`, che ammette poteri
arbitrari, e [ADR-001] prevede un set pesato. La conseguenza e' che **il set di
genesi — cioe' la devnet di M-02 — non e' vincolato a potere uniforme**, e quindi
la proprieta' di vivacita' che `wire.md` pubblica non e' garantita da alcuna regola
su quel set: e' garantita solo dalla forma che un set **eletto** avra'. Questa
remediation ha ristretto l'affermazione al fatto, che e' la strada **(a)** della
review; la strada **(b)** — cambiare l'indice perche' round consecutivi non
ripetano un membro finche' ne resta uno non visitato, conservando la
proporzionalita' su un ciclo intero — e' una modifica di [ADR-018] §3 e non e'
dell'implementatrice. **Nessun debito e' stato aperto**, come da incarico: il
residuo e' riportato qui e la decisione e' del Lead.