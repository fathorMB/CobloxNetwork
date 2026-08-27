---
id: SPEC-025
# Note: Quote the title if it contains a colon
title: "Il motore di consenso: prevoto, blocco, e una catena finalizzata da quattro validatori"
status: review
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
| GATE-ADR012-PASS | `published_artifacts.py` e la prova in negativo | PASS, 177 probe provate singolarmente |

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
