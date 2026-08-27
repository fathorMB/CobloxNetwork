---
id: REVIEW-049
# Note: Quote the title if it contains a colon
title: "GATE-SECREVIEW su SPEC-029: il confine della busta non esiste, e il WAL non ripristina il lock"
status: changes-requested
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-029
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, correctness, verification-integrity, schema-conformance, robustness, documentation, test-quality, provenance]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-049-EVENT-001"
    timestamp: "2026-08-27T20:16:45.095951200+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-049-EVENT-002"
    timestamp: "2026-08-27T20:22:49.662127500+02:00"
    action: "attribution-correction"
    from_status: "pending"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "attribution corrected from 'AGENT-002' to 'AGENT-001': Attribuzione errata alla creazione della review: risultava AGENT-002. SPEC-029 porta recommended_agent: AGENT-001 nel frontmatter, ed e' AGENT-001 che ha implementato il crate coblox-node. Correzione richiesta da AGENT-007 nelle note operative di REVIEW-049 e verificata dal Lead sul frontmatter della spec."
    implementation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-049-EVENT-003"
    timestamp: "2026-08-27T20:23:16.058644400+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "GATE-SECREVIEW non superata. Due critical con PoC eseguito, entrambi verificati dal Lead per conto proprio e non presi sulla parola.\n\nRF-001, il confine della busta non esiste. Verificato: le uniche due occorrenze di `.verify(` in coblox-node/src sono a node.rs:285 e node.rs:391 e sono entrambe su `FinalizedBlock`, non sulla busta. `SignedEnvelope::verify` non ha un solo chiamante nel workspace. `handle_envelope` confronta il solo `network_id` (node.rs:352); il `chain_id` che compare a node.rs:367 e' un argomento di `build_and_sign` per una busta in uscita, non un controllo su quella in ingresso. Una chiave estranea al set, in una busta scaduta dal 1970, ha fatto firmare e rendere durevole a un nodo onesto un prevoto su un blocco con `state_root` scelto dall'attaccante: le firme che ne derivano sono genuine e il certificato risultante e' valido. `wire.md:516` dice che l'autenticita' di una proposta e' quella della busta.\n\nRF-002, il WAL non ripristina il lock. Verificato indipendentemente dal Lead prima di leggere la review, mentre eseguiva GATE-LEAD-REPRO: `ConsensusEngine` nasce con `locked: None` (consensus/engine.rs:307) e la stringa `locked` non compare in alcun file di coblox-node/src. Il WAL persiste `(height, round, phase) -> block_id` e copre l'equivocazione nello stesso round, ma non `lockedValue`/`lockedRound`, su cui poggia la sicurezza di ADR-018. Un nodo ucciso mentre e' lockato torna su slockato e precommitta un valore diverso a un round successivo senza polka: il WAL non lo vede perche' la chiave del round e' diversa. Con n=4, f=1 un solo kill -9 consuma l'intero budget di equivocazione senza avversario. Il round > 0 non e' ipotetico: nel log del Lead l'altezza 1 finalizza a round=1.\n\nRF-003 e' accolto: GATE-WAL-BEFORE-SEND e' marcata [x] ma il test non uccide nulla, riapre il Wal nello stesso processo, e la devnet uccide fuori dalla finestra. La regola durevole-prima-di-trasmettere e' pero' attuata su ogni percorso e il fsync c'e': cio' che manca non e' la durabilita', e' cosa viene reso durevole.\n\nGATE-SUBSET-DECLARED non e' soddisfatta: topic `blocks` dichiarato in wire.md e non sottoscritto, `message_id_fn` su DefaultHasher dove wire.md impone l'ID verificato, `nonce` a [0;16] ovunque che rende inattuabile la cache anti-replay.\n\nIl Lead prende su di se' tre affermazioni false introdotte durante la presa in carico correttiva, tutte e tre verificate vere dalla review e riverificate dal Lead: il commento di `handle_envelope` dichiara di scartare buste di un'altra catena mentre il `chain_id` non e' mai confrontato; il commento di `run` dichiara di errare su una trasmissione che non parte mentre tutti e cinque i `try_send` sono `let _ =` (node.rs:131,160,226,313,378); e la ragione di `now_ms`, ripetuta nel messaggio di commit, dice che quel valore arma ogni timeout di consenso mentre le sue cinque chiamate lo passano solo come `created_at_ms` di una busta. Sul merito, la saturazione a `u64::MAX` e' nel verso sbagliato: con TTL separato (30_000, 60_000) produce una busta eterna, cioe' proprio il controllo che RF-001 chiede di accendere, mentre il ramo gemello `Duration::ZERO` fallisce chiuso. Vanno corretti nella stessa remediation, e la correzione del commento non basta: `now_ms` va rifatto nel verso che fallisce chiuso.\n\nRestano fuori remediation e vanno aperti come debito, non corretti qui: `.gitignore` senza voce per `data/` e `--data-dir` che punta di default dentro l'albero sorgente."
    evidence_refs: ["REVIEW-049", "SPEC-029", "core/coblox-node/src/node.rs", "core/coblox-core/src/consensus/engine.rs", "docs/devnet-runbook.md"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-049-EVENT-004"
    timestamp: "2026-08-27T21:30:43.671151700+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Remediation eseguita su venti rilievi. Chiusi i due critical: RF-001 porta `SignedEnvelope::verify` come prima operazione di `handle_envelope` su ogni message_type, con risoluzione di `sender_node_id` a un membro del set, scadenza, tetto di validita', nonce casuale e cache anti-replay, e il PoC-1 e' nella suite invertito con il gemello ben firmato. RF-002 ricostruisce `locked_round`/`locked_block_id` da `Wal::locked_at_height` e li passa a `Engine::start` con due campi nuovi di `EngineConfig`, la strada additiva che la review raccomandava. Chiusi RF-003, RF-004, RF-005, RF-007 e tutti i rilievi da RF-008 a RF-020, incluse le tre correzioni di provenienza Lead (RF-011, RF-012, RF-013), con `now_ms` rifatto nel merito e non solo nel commento. Dichiarati non chiusi, apertamente: RF-006 solo in parte (limitato a otto blocchi e regolato a una risposta al secondo per richiedente, ma la sincronizzazione resta pubblicazione sul topic invece che request/response su ledger-sync), RF-010 senza tetto in byte, RF-016 senza copertura WAL della proposta, e la coda di RF-009 con `--seed-hex` ancora su argv. Corrette e dichiarate false nel testo del criterio stesso le tre caselle marcate [x] senza il test che nominavano. Segnalati al Lead due esiti visibili solo eseguendo: accendere la scadenza ha reso l'amplificazione di RF-006 uno stallo di liveness durante il catch-up, e un difetto ventunesimo non censito (`UnsolicitedValue` da azioni di un motore sostituito da una dispatch annidata) e' stato corretto dentro il giro invece che soltanto riportato."
    evidence_refs: ["REVIEW-049", "SPEC-029", "core/coblox-node/tests/envelope_boundary.rs", "core/coblox-node/tests/wal_lock_restore.rs", "core/coblox-node/tests/durable_before_send.rs"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-049-EVENT-005"
    timestamp: "2026-08-27T21:31:03.567650500+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verifica del Lead eseguita per conto proprio, non sulle affermazioni dello specialista.\n\nEseguito `cargo test --workspace`: 264 passati, 0 falliti, cifra identica a quella dichiarata. Esistono i cinque file di test nuovi (`envelope_boundary.rs`, `wal_lock_restore.rs`, `durable_before_send.rs`, `future_height_buffer.rs`, `sync_response_bound.rs`) e il ripristino del lock e' reale (`node.rs:145` chiama `wal.locked_at_height(start_height)` e traccia `LOCK_RESTORED`).\n\nSul `chain_id`, lo specialista corregge la remediation di RF-001 e ha ragione: la busta non ha un campo `chain_id`, e `envelope.rs:281` ricalcola `message_id(chain_id, ...)` con il `chain_id` legato dentro il preimage. Ricalcolare e' il controllo di catena, ed e' piu' forte del confronto di campo che la review chiedeva.\n\nSulla seconda meta' della condizione di chiusura di RF-002, il Lead da' ragione allo specialista **contro la review**. La review chiedeva `can_vote(5, 2, Precommit, C) == false`. Sarebbe scorretto: la riga 29 dell'Algoritmo 1 permette di sbloccarsi dopo una polka a un `valid_round` adeguato, e `can_vote` interroga il WAL, che non sa nulla delle polka. Farlo rifiutare ogni `C` diverso dal blocco lockato bloccherebbe uno sblocco legittimo e romperebbe la liveness. Quel giudizio spetta al predicato del motore, che il lock restaurato ora alimenta. La condizione apparteneva all'altro rimedio, quello che la review stessa sconsigliava.\n\nVerificato inoltre un rischio che ne' la review ne' lo specialista avevano nominato: il restringimento di `locked` da `Option<(u64, Value)>` a `Option<(u64, Digest32)>` non impedisce a un proposer lockato di riproporre, perche' la ri-proposta segue le righe 15-19 passando da `self.valid` (`engine.rs:554`), che conserva il `Value` intero, mentre `locked` serve al solo predicato di sblocco, per il quale l'ID basta.\n\n`GATE-ENGINE-UNCHANGED` e' stata riportata al suo testo originale e derogata dal Lead: lo specialista l'aveva lasciata [x] argomentando l'intento, e ha dichiarato la scelta invece di nasconderla, ma la clausola di eccezione nomina [REVIEW-047] e non [REVIEW-049], quindi la gate come scritta e' falsa. Riscrivere il testo di una gate da parte di chi essa vincola resta una decisione del Lead. Nessun debito: non resta nulla di non fatto.\n\nI quattro rilievi dichiarati non chiusi restano aperti e vanno portati a debito prima della chiusura della spec."
    evidence_refs: ["REVIEW-049", "SPEC-029", "core/coblox-core/src/consensus/engine.rs", "core/coblox-node/src/envelope.rs", "core/coblox-node/src/node.rs"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
links: []
created: 2026-08-27
updated: 2026-08-27
tags: [review, security, devnet, wal, p2p]
related_decisions: [ADR-018, ADR-012, ADR-015, ADR-001]
activity:
  - date: 2026-08-27
    action: "created"
  - date: 2026-08-27
    action: "corrected implementation_agent AGENT-002 -> AGENT-001"
  - date: 2026-08-27
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-27
    action: "recorded review remediation"
  - date: 2026-08-27
    action: "recorded review remediation-verification"
---
# Review

> **`GATE-SECREVIEW` su [SPEC-029]**, albero al commit `fde4d6b`, piu' i file **non
> versionati** che quel commit non contiene e che ho trovato nell'albero di lavoro
> (`docs/devnet-runbook.md`, `data/`, `data-val{0,1,2,3}.log`). Vedi RF-005.
>
> Ogni affermazione dichiara se e' **ESEGUITA** o **LETTA**. Le prove eseguite sono
> quattro proof-of-concept scritti in `core/coblox-node/tests/greta_poc.rs`,
> compilati ed eseguiti contro `fde4d6b`, e **rimossi dopo l'esecuzione**: l'albero
> che il Lead trovera' e' quello che mi ha consegnato, meno nulla e piu' nulla.
> L'attribuzione `implementation_agent` e' stata allocata a `AGENT-002` dalla
> creazione dell'artefatto: se e' sbagliata la corregge il Lead con
> `review_set_implementation_agent`, non io a mano.

## Outcome

**Non passa.** Due difetti bloccanti, e nessuno dei due e' un caso limite.

Il primo e' quello che l'incarico chiamava «il difetto piu' grave possibile qui»:
**ogni messaggio in arrivo entra senza che la busta sia verificata**.
`SignedEnvelope::verify` esiste, e' scritta bene, e' documentata — e **non ha
un solo chiamante nell'intero workspace**. `verify_proposal` non fa alcuna
verifica crittografica: prende `sender_validator_id` sulla parola. Il nodo gli
passa `envelope.sender_node_id`, che nessuno ha controllato. `wire.md` dice
esattamente cosa costa: *«A proposal carries no signature of its own. Its
authenticity is the envelope's, which binds `sender_node_id`»* (riga 516). Quella
autenticita' non viene mai stabilita.

Il secondo e' quello che la spec ha nominato per prima e ha misurato con la regola
sbagliata. Il WAL impedisce di firmare due volte lo **stesso** `(altezza, round,
fase)`. La sicurezza di [ADR-018] non poggia su quello: poggia su `lockedValue` e
`lockedRound`, che il WAL **non scrive** e che `EngineConfig` **non ha un campo per
ripristinare** (`engine.rs:307`, `locked: None`). Un validatore onesto che riparte
perde il lock e puo' precommittare un blocco diverso a un round successivo senza
polka. Non serve un avversario: serve un `kill -9`, che e' la condizione che la
sezione *Risks* della spec aveva gia' scritto per esteso.

Il resto e' proporzionato: una gate dichiarata soddisfatta da un test che non
esiste, un sottoinsieme dichiarato che non e' quello attuato, e la solita coda.

**Sulla presa in carico del Lead: regge, e le due affermazioni portanti sono
vere.** Ho verificato i nove `if` collassati e le quindici sezioni `# Errors`.
Nessuno dei nove cambia comportamento. Le due affermazioni sul WAL che il Lead
ha dichiarato portanti — `Wal::open` fallisce su una riga illeggibile,
l'errore di `Wal::record_vote` non e' recuperabile ritrasmettendo — **sono
entrambe vere**, e la prima l'ho eseguita. Due *altre* delle quindici sono false
(RF-011), e la ragione scritta accanto a `now_ms` e ripetuta nel messaggio di
commit e' falsa (RF-013). La passata non ha rotto nulla; ha aggiunto tre
affermazioni non verificate a un crate pubblico, che e' la famiglia 2 del
censimento.

## Acceptance-criteria compliance

| Criterio | Marcato | Verificato |
| --- | --- | --- |
| `coblox-node start` carica chiave, configurazione, pari seed | `[x]` | **Sì** (LETTO `main.rs`, ESEGUITO il binario via i test) |
| Quattro processi separati finalizzano ≥ 10 blocchi, certificati accettati | `[x]` | **Sì** (ESEGUITO: `devnet_multiprocess`, quattro PID distinti, verifica con `ConsensusVerifier`) |
| Il riavvio non produce equivocazione | `[x]` | **No.** RF-002. Il test osserva che il nodo riavviato *non contraddice il proprio log*, che e' vero e non e' la proprieta'. Il lock non e' ripristinato. |
| Un voto non e' mai trasmesso prima di essere durevole, **dimostrato da un test che uccide il processo fra la scrittura e la trasmissione** | `[x]` | **No.** RF-003. Quel test non esiste. Nemmeno in forma probabilistica con una `sleep`: non c'e' alcuna finestra. |
| I blocchi finalizzati sopravvivono al riavvio | `[x]` | **Sì** (LETTO `store.rs`, ESEGUITO indirettamente dal test di riavvio) |
| `FinalizedBlock::verify` ricalcola `transactions_root` — chiude [DEBT-047] | `[x]` | **Sì** (LETTO `messages.rs:379`, `certificate.rs`; il test negativo esiste in `consensus_rules.rs`) |
| Buffering fra altezze e `valid(v)` implementati **e nominati: esiste un test** | `[x]` | **No.** RF-004. `buffer.rs` non ha alcun test, in nessun file. |
| I tre timeout derivati da una grandezza dichiarata | `[x]` | **Sì** (LETTO `config.rs`: `Delta_net = 50 ms` dichiarato, i quattro valori derivati). Nota: `prevote_ms`/`precommit_ms` valgono 150, non i 100 che il commento accanto deriva. RF-020. |
| Recinto di [DEBT-029] | `[x]` | **Sì** (ESEGUITO: `verify_consensus_ed25519` non e' piu' esportata dalla radice) |
| Trasporto TCP+Noise+Yamux+GossipSub e `wire.md` dichiara **quale sottoinsieme** | `[x]` | **No.** RF-007. Il testo dichiara un sottoinsieme che il codice non attua. |
| Il runbook avvia quattro nodi e la trascrizione mostra la catena crescere | `[x]` | **Parziale.** RF-005: il runbook esiste, e' buono, ed e' **non versionato**. Non e' in `fde4d6b` ne' in alcun commit. |
| Passata di [ADR-012] eseguita e trascritta | `[x]` | **Sì** (ESEGUITO: la probe `wire-devnet-transport-subset` esiste e passa; fissa il **testo**, non la conformita' del codice) |

## Code observations

### Il confine che non c'e'

`node.rs::handle_envelope` fa, in ordine: confronta `network_id`, poi instrada per
`message_type`. Non chiama `SignedEnvelope::verify`. Non confronta `chain_id`. Non
guarda `expires_at_ms`. Non guarda `nonce`. Non ha cache di replay.

Cio' che protegge il consenso e' allora solo la firma **interna** dei voti
(`verify_vote` → `ConsensusVerifier`). Le proposte non hanno firma interna per
disegno di `wire.md`, quindi non sono protette da nulla. Il percorso completo
dell'attacco e' stato **ESEGUITO** (PoC-1):

```text
PoC-1 handle_envelope -> Ok(false)
PoC-1 WAL votes before=0 after=1
PoC-1 DEMONSTRATED: honest node signed 1 vote(s) on a proposal forged by a
non-validator, with an envelope expired since 1970 and an attacker-chosen state_root
```

La chiave dell'attaccante (`SigningKey::from_seed(&[0xAA; 32])`) non appartiene
al set — il PoC lo asserisce prima di procedere. La busta dichiara
`created_at_ms = 0`, cioe' scaduta dal 1970. Il nodo onesto ha comunque firmato e
reso durevole un prevoto su un blocco con `state_root` scelto dall'attaccante.

### Il WAL, e la regola che misura

La regola dichiarata — *un voto e' trasmesso solo dopo essere stato reso
durevole* — **e' attuata correttamente su ogni percorso**, e questo va detto
prima delle critiche. In `node.rs`, ramo `Action::Broadcast(Outbound::Vote)`:
`can_vote` → firma → `self.wal.record_vote(...)?` → `try_send`. Il `?` propaga.
Non esiste ordine in cui il `try_send` preceda la scrittura, non c'e' un errore
ignorato su quel percorso, e il buffer non riordina perche' `outbound_tx` e' un
canale FIFO consumato da un solo lettore.

Il `fsync` **c'e' davvero**: `wal.rs` chiama `write_all` → `flush` → `sync_all`,
e `sync_all` e' `fsync`, non `fdatasync` e non solo `flush`. LETTO e confermato
alla riga. `store.rs::append_block` fa lo stesso.

Cio' che manca non e' la durabilita': e' **cosa** viene reso durevole. Il WAL
scrive `(height, round, phase) -> block_id`. Non scrive il lock. `Engine::start`
non lo puo' ricevere. ESEGUITO (PoC-3):

```text
PoC-3 DEMONSTRATED: after recording precommit(h=5,r=1,B), the WAL still returns
can_vote=true for precommit(h=5,r=2,C) and precommit(h=5,r=0,C).
```

E il cambio di round non e' ipotetico su questa devnet: nel log del Lead
(`data-val0.log`, LETTO) l'altezza 1 finalizza a **`round=1`**.

### La rete

- `network.rs` sottoscrive **un solo topic**, `consensus`, e ci pubblica tutto:
  proposte, voti, `finalized_block` e `block_request`. `wire.md` righe 73-78
  dichiara la separazione fra `consensus` e `blocks` **normativa e non
  organizzativa**, con la ragione scritta accanto.
- `message_id_fn` usa `std::collections::hash_map::DefaultHasher` sui byte del
  messaggio. `wire.md` riga 135: *«GossipSub's message-ID function MUST use this
  verified ID»*. `DefaultHasher` e' a 64 bit, non e' stabile fra versioni di
  `std`, e non e' l'ID verificato.
- `nonce` vale `[0u8; 16]` in **tutti e cinque** i siti di costruzione. La cache
  `(sender_node_id, nonce)` che `wire.md` impone e' strutturalmente impossibile.
- `ValidationMode::Permissive`. `MessageAuthenticity::Signed` copre il divieto di
  *anonymous author mode*, ma `Permissive` accetta messaggi privi di `signature`,
  `from` e `seqno` a livello gossipsub.
- `run()` trasmette un `block_request` **incondizionato ogni 500 ms**, e ogni
  ricevente risponde pubblicando `latest - from_height + 1` blocchi **sul topic**,
  non al richiedente. Con quattro nodi e una catena di lunghezza *n* questo e'
  8·*n* blocchi al secondo in stato stazionario, e un singolo `block_request`
  non autenticato con `from_height = 1` fa trasmettere a tutti l'intera catena.

### Chiavi

Non esistono chiavi su disco, e questa e' la scelta giusta per una devnet: sono
derivate da `--seed-index` (`[index + 1; 32]`) e il runbook lo dichiara con la
frase esatta che serve — *«the keys are reproducible by anyone who reads this
file. That is deliberate for a devnet and disqualifying for anything else»*.
Quella frase pero' vive in un file non versionato e la spec non la contiene.

Cio' che il threat model ha da dire e' altrove: `SigningKey` deriva `Debug`, e lo
scalare segreto esce in chiaro in qualunque riga di log o messaggio di panico che
formatti la chiave o la `NodeConfig` che la contiene. ESEGUITO (PoC-4):

```text
PoC-4 Debug(SigningKey) = SigningKey { scalar: Scalar{ bytes: [236, 153, 188, ...] },
prefix: [113, 67, 74, ...], public_key: [...] }
```

## Tests and verification

**ESEGUITO da me**

- `cargo test -p coblox-node --test greta_poc -- --nocapture`: quattro PoC, tutti
  dimostrati. File rimosso dopo l'esecuzione.
- Lettura riga per riga dei nove `if` collassati (`node.rs:116, 300, 316, 364,
  416`; `network.rs:96`; `wal.rs:67`; piu' i due rami `&&` di `handle_envelope`).
  **Nessuno cambia comportamento.** Il caso che meritava attenzione e' `wal.rs:67`,
  dove la condizione contiene l'effetto collaterale `recorded_votes.insert(...)`:
  l'inserimento avviene prima del confronto in entrambe le forme, e la mappa viene
  scartata sul ramo d'errore, quindi il collasso e' innocuo. **Limite dichiarato**:
  `clippy --fix` e la stesura sono nello stesso commit, quindi il testo
  precedente non e' in alcun diff; ho verificato le nove forme contro la logica
  circostante, non contro l'originale.
- `git ls-files` / `git status --porcelain`: `docs/devnet-runbook.md`, `data/`,
  `data-val*.log` non sono tracciati.
- `grep` su tutto il workspace per i chiamanti di `SignedEnvelope::verify` e di
  `FutureHeightBuffer::prune_before`: **zero** per entrambi.

**NON rifatto, e non contestato**: i 235 test verdi, `clippy -D warnings` pulito,
`fmt --all --check` pulito, i nove strumenti a exit 0, `libp2p` con il solo
sottoinsieme dichiarato nel `Cargo.toml`, [DEBT-047] chiuso. Ho verificato per
lettura che il `Cargo.toml` porti `tcp`, `noise`, `yamux`, `gossipsub` piu'
`tokio` e `macros`, che sono di runtime e non di protocollo: il **manifesto** e'
conforme. Cio' che non concorda con il testo e' il **codice** (RF-007).

**Cosa i test consegnati provano davvero**

`wal_safety.rs` non uccide nulla: «Phase 2: Simulate crash and restart by
reopening WAL» riapre il `Wal` nello stesso processo. E' un buon test di replay e
non e' un test di crash.

`devnet_multiprocess.rs::validator_crash_and_restart_recovers_without_equivocation`
uccide un processo vero, ed e' il test migliore della consegna. Ma uccide **dopo
l'altezza 2**, in un istante arbitrario, e non nella finestra fra `sync_all` e
`try_send`. E cio' che asserisce al ritorno e' che i blocchi finalizzati da
val-003 verificano — che sarebbe vero anche se val-003 avesse equivocato, perche'
un blocco finalizzato porta il certificato di **altri**.

## Production quality and documentation compliance

`QUALITY.md` §*Shortcuts*: il percorso di produzione porta segnaposto non
dichiarati come eccezione — `state_root: Digest32::repeated(0x33)`,
`consensus_parameters_hash: Digest32::repeated(0x44)`, `timestamp_ms` come
costante aritmetica `1_787_654_400_000 + h*5000 + r*1000 + idx`,
`key_binding_signature: [0u8; 64]`, `nonce: [0u8; 16]`. Alcuni sono inevitabili
finche' non esiste un esecutore, e allora vanno **dichiarati** nella spec con
scopo, rischio e condizione di scadenza, come la sezione impone. Nessuno lo e'.

`QUALITY.md` §*Verification standard*: l'evidenza contiene tre affermazioni che
non reggono alla lettura del codice che descrivono (RF-015).

## Review findings

RF-001 | category=security-boundary | severity=critical | criterion=**Il confine della busta non esiste.** `SignedEnvelope::verify` non ha un solo chiamante nel workspace (ESEGUITO: `grep` su `core/`, zero occorrenze fuori dalla propria definizione). `handle_envelope` confronta il solo `network_id` e instrada. `verify_proposal` non esegue alcuna verifica crittografica: valida appartenenza al set, turno di proposta, coerenza header/payload, e prende `sender_validator_id` sulla parola. Il nodo gli passa `envelope.sender_node_id`, un campo non verificato. `wire.md` riga 516 dichiara che l'autenticita' di una proposta **e'** quella della busta. **Scenario d'attacco**: un pari qualunque che completa l'handshake Noise — nessuna chiave di validatore, nessun certificato, e la devnet non fa autorizzazione a livello di connessione — pubblica sul topic una busta con `message_type = "block_proposal"`, `sender_node_id` uguale al proposer di `(h, r)` calcolato dalla regola pubblica, `previous_block_id` uguale all'ultimo blocco finalizzato osservato dal gossip, e un `state_root` a piacere. I nodi onesti prevotano e precommittano quel blocco; le loro firme sono genuine, il certificato che ne risulta e' valido, e il blocco scelto dall'attaccante e' finalizzato. **ESEGUITO** (PoC-1): un nodo onesto ha firmato e reso durevole un prevoto su una proposta forgiata da una chiave estranea al set, dentro una busta scaduta dal 1970. Ne discendono, nello stesso buco: nessun controllo di `chain_id`, nessun controllo di scadenza, nessuna cache di replay, e `block_request` accettato da chiunque. | remediation=Chiamare `SignedEnvelope::verify` come **prima** operazione di `handle_envelope`, su ogni `message_type` senza eccezioni, risolvendo `sender_node_id` alla `consensus_public_key` del membro del set e rifiutando un mittente non membro; confrontare `chain_id`; rifiutare `expires_at_ms - created_at_ms` oltre il tetto; usare un `nonce` casuale e tenere la cache `(sender_node_id, nonce)` che `wire.md` impone. **Condizione di chiusura**: il PoC-1 di questa review, portato nella suite come test di regressione, deve fallire l'ingresso — `handle_envelope` restituisce errore e `wal_vote_count()` resta invariato — e un test gemello con busta ben firmata dal proposer legittimo deve continuare a passare.

RF-002 | category=correctness | severity=critical | criterion=**Il WAL non ripristina il lock, quindi un nodo onesto che riparte equivoca fra round.** `Wal` persiste `(height, round, phase) -> block_id` e nient'altro. `lockedValue_p` e `lockedRound_p` — le variabili su cui poggia l'intersezione dei quorum di [ADR-018] — non sono scritte, e `EngineConfig` non ha un campo per riceverle: `Engine::start` pone `locked: None` incondizionatamente (`engine.rs:307`). `can_vote` guarda una chiave che include il round, quindi un precommit contraddittorio a un round **diverso** e' permesso. **Scenario, senza avversario**: val-003 precommitta B a `(h=5, r=1)` e si blocca su lock `(1, B)`; `kill -9`; riavvio. `start_height` torna 5, il motore riparte da `round 0` con `locked = None`. Al round 2, che questa devnet raggiunge davvero — nel log del Lead l'altezza 1 finalizza a `round=1` — arriva una proposta per C con `valid_round: None`; la riga 634 di `engine.rs` valuta `unlocked = true` perche' `locked` e' `None`, e il nodo precommitta C. Il WAL non lo vede: `(5, 2, precommit)` e' una chiave libera. Con `n = 4` e `f = 1`, un solo `kill -9` consuma l'intero budget di equivocazione **senza che nessuno sia malevolo**, ed e' esattamente l'ipotesi che ogni prova di sicurezza di [ADR-018] assume non violata. **ESEGUITO** (PoC-3). Il criterio di accettazione «il riavvio non produce equivocazione» e' marcato `[x]` su una proprieta' piu' debole di quella che il suo stesso testo enuncia. | remediation=Rendere durevole il lock, non solo il voto: o registrare `locked_round`/`locked_block_id` nel WAL a ogni cambio, o ricostruirlo al riavvio come il precommit di round massimo per l'altezza in corso presente nel log — che il WAL gia' contiene — e passarlo a `Engine::start` attraverso due campi nuovi di `EngineConfig`. La seconda strada e' additiva e non tocca la macchina a stati. **Condizione di chiusura**: un test che registra `precommit(h, r=1, B)`, ricostruisce il nodo dal disco, e osserva che una proposta per `C != B` a `r=2` con `valid_round` assente **non** produce un precommit; e il PoC-3 di questa review invertito, cioe' `can_vote(5, 2, Precommit, C)` che restituisce `false`.

RF-003 | category=verification-integrity | severity=high | criterion=**`GATE-DURABLE-BEFORE-SEND` e' marcata `[x]` e il test che la definisce non esiste.** La gate chiede di osservare che un voto non lascia il processo prima di essere durevole «uccidendo il processo nella finestra e verificando al riavvio», e il criterio di accettazione ripete la stessa frase. `wal_safety.rs` non uccide alcun processo: il commento dice «Simulate crash and restart by reopening WAL» e riapre il `Wal` nello stesso processo, che prova il replay e non la durabilita'. `devnet_multiprocess.rs` uccide un processo vero, ma dopo l'altezza 2, in un istante non correlato alla finestra. **Non e' nemmeno una finestra probabilistica costruita con una `sleep`**: non c'e' alcuna finestra. La regola sostanziale, va detto, **e' attuata correttamente** (LETTO: `record_vote(...)?` precede `try_send` su ogni percorso, e `sync_all` e' presente): cio' che manca e' l'osservazione che la gate chiede, e la gate e' marcata soddisfatta. | remediation=Un test che avvia un nodo con un punto di interruzione deterministico — una variabile d'ambiente che fa `std::process::abort()` subito dopo `sync_all` e prima del `try_send`, o un `Wal` istrumentato — uccida il processo li', riavvii dalla stessa `data_dir` e verifichi che `vote_of(h, r, phase)` restituisce il voto mentre nessun pari lo ha ricevuto. **Condizione di chiusura**: il test esiste, e il punto di uccisione e' un'istruzione, non un `sleep`.

RF-004 | category=test-quality | severity=high | criterion=**Il criterio sul buffering chiede un test per nome e il test non esiste.** Il testo e' *«esiste un test in cui un messaggio di un'altezza futura arriva presto, viene trattenuto, e viene consumato quando l'altezza comincia»*, marcato `[x]`. `buffer.rs` (60 righe) non ha alcun `#[cfg(test)]`, e nessun file di test nomina `FutureHeightBuffer`, `insert` o `drain_height`. ESEGUITO: `grep` su `core/coblox-node/tests/`, zero occorrenze. Il buffering e' esercitato solo di rimbalzo dalla devnet, che non asserisce nulla su di esso. | remediation=Il test che il criterio descrive, con l'asserzione sui tre momenti: trattenuto, non consegnato al motore, consegnato quando l'altezza avanza. Piu' i tre casi di bordo che `insert` gia' contiene e nessuno osserva: `message_height <= current_height` scartato, oltre `max_lookahead` scartato, oltre `max_messages_per_height` scartato.

RF-005 | category=provenance | severity=high | criterion=**Il runbook non e' nel commit.** `docs/devnet-runbook.md` e' **non versionato** (ESEGUITO: `git ls-files` vuoto, `git log --all --` vuoto, `git check-ignore` vuoto). Il criterio che lo richiede e' marcato `[x]`, e la gate di [ADR-012] non lo copre. Il file stesso dichiara di essere stato scritto dal Lead «because [SPEC-029] marked this criterion satisfied while no runbook existed», il che rende il rilievo doppio: l'evidenza originale era falsa, e la riparazione non e' entrata nell'albero versionato. Insieme al runbook restano non versionati e **non ignorati** `data/` — che contiene i `wal.jsonl` e i `blocks.jsonl` reali della sessione del Lead, cioe' voti firmati — e `data-val{0,1,2,3}.log`. `.gitignore` non ha alcuna voce per `data/`, e la directory dati predefinita del nodo e' `./data/val-000`, cioe' **dentro l'albero sorgente**. La disciplina di commit del Lead vieta `git add -A` proprio per questo. | remediation=Versionare `docs/devnet-runbook.md` e classificarlo nella chiusura documentale di `published_artifacts.toml`; aggiungere `data/` e `data-val*.log` a `.gitignore`; cambiare la `default_value` di `--data-dir` in un percorso fuori dall'albero sorgente, o rimuovere la default e renderla obbligatoria. **Condizione di chiusura**: `git status --porcelain` pulito su un albero dopo un giro di runbook.

RF-006 | category=security-boundary | severity=high | criterion=**`block_request` e' un amplificatore non autenticato, e il nodo se ne manda uno da solo ogni 500 ms.** Nessuna verifica di busta (RF-001), quindi chiunque puo' emetterlo. La risposta non va al richiedente: `handle_envelope` costruisce una busta `finalized_block` per **ogni** altezza da `from_height` a `latest` e la mette su `outbound_tx`, che `network.rs` **pubblica sul topic**. Un singolo messaggio con `from_height = 1` fa ritrasmettere a ognuno dei quattro validatori l'intera catena a tutti gli altri. In piu', `run()` emette un `block_request` incondizionato a `self.engine.height()` ogni 500 ms anche quando nessuno e' indietro: in stato stazionario, con quattro nodi, sono otto blocchi al secondo trasmessi per nulla, e la quantita' cresce con l'altezza. LETTO, non eseguito come attacco. | remediation=Rispondere al richiedente e non al topic — cioe' portare `block_request` su uno stream request/response, che e' cio' che `wire.md` prevede per `ledger-sync`; limitare il numero di blocchi per risposta e la frequenza per mittente; emettere il `block_request` periodico solo quando l'altezza locale e' rimasta indietro rispetto a un `finalized_block` osservato. **Condizione di chiusura**: un test che invia un `block_request` con `from_height = 1` su una catena di dieci blocchi e osserva un numero di buste in uscita limitato da una costante dichiarata.

RF-007 | category=schema-conformance | severity=high | criterion=**`GATE-SUBSET-DECLARED`: il sottoinsieme dichiarato non e' quello attuato, in tre punti, e nessuno dei tre e' nell'elenco delle esclusioni.** (a) `wire.md` §*Devnet transport subset* dichiara GossipSub «for broadcast of consensus messages (`/coblox/<network_id>/consensus/0.1`) **and blocks** (`/coblox/<network_id>/blocks/0.1`)»; `network.rs` sottoscrive **un solo topic**, `consensus`, e ci pubblica anche `finalized_block` e `block_request` — mentre `wire.md` righe 73-78 dichiara quella separazione **normativa e non organizzativa**, con la ragione scritta accanto. (b) `wire.md` riga 135: *«GossipSub's message-ID function MUST use this verified ID»*; `message_id_fn` usa `DefaultHasher` sui byte grezzi, a 64 bit e non stabile fra versioni di `std`. (c) `wire.md` impone la cache `(sender_node_id, nonce)`; il `nonce` e' `[0u8; 16]` in tutti e cinque i siti di costruzione, quindi la coppia e' costante e la cache e' inattuabile. Il **manifesto** e' invece conforme: `Cargo.toml` porta `tcp`, `noise`, `yamux`, `gossipsub` piu' `tokio` e `macros`, che sono di runtime; nessun QUIC, nessun Kademlia, nessun mDNS. La divergenza e' fra il testo e il codice, che e' precisamente cio' che la gate chiama «divergenza silenziosa». | remediation=Scegliere per ciascuno dei tre: attuare, o aggiungere all'elenco delle esclusioni di `wire.md` con la ragione e il rinvio, come le sei esclusioni gia' scritte. La probe di [ADR-012] fissa una frase; aggiungerne una che fissi il **topic dei blocchi** e la funzione di message-ID renderebbe il prossimo scostamento visibile.

RF-008 | category=robustness | severity=medium | criterion=**Una scrittura troncata rende il validatore inavviabile per sempre.** `Wal::open` propaga l'errore di `parse_canonical` su qualunque riga non interpretabile — che e' la scelta giusta, ed e' esattamente cio' che il Lead ha documentato. La conseguenza non e' documentata: un `kill -9` fra `write_all` e `sync_all`, cioe' **la finestra che questa spec esiste per proteggere**, puo' lasciare un record parziale in coda, e da quel momento `Wal::open` fallisce a ogni avvio. Il nodo non riparte, e i voti *integri* che lo precedono diventano illeggibili con lui. ESEGUITO (PoC-2): troncati 40 byte dell'ultimo record, `Wal::open` restituisce `Err(Core(Json(UnexpectedEnd)))`. Non e' un difetto di sicurezza — fallisce chiuso — ma trasforma una perdita di potenza in un validatore perso, e la spec non lo dice. | remediation=Distinguere una coda troncata da una riga corrotta: se **e solo se** l'ultima riga non termina con `\n`, scartarla e troncare il file alla fine dell'ultimo record completo, registrando l'evento; qualunque riga malformata **non** in coda resta un errore fatale. Documentare la distinzione nella sezione `# Errors` di `Wal::open`. **Condizione di chiusura**: il PoC-2, invertito — `Wal::open` riesce, `count()` vale 1, e il record troncato non e' nel file dopo la riapertura.

RF-009 | category=security-boundary | severity=medium | criterion=**Materiale segreto in chiaro in `Debug`, e nessun trattamento del ciclo di vita della chiave.** `SigningKey` deriva `Debug, Clone, Copy, PartialEq, Eq`. ESEGUITO (PoC-4): `format!("{k:?}")` stampa lo scalare segreto e il `prefix` in chiaro. `NodeConfig` deriva `Debug` e contiene la chiave, quindi qualunque `{:?}` su di essa, oggi o domani, la stampa; `main.rs` gia' formatta strutture di configurazione. In coda: `Copy` moltiplica le copie in memoria senza che nessuna sia azzerata, `PartialEq` confronta materiale segreto in tempo non costante, e `--seed-hex` prende il seme dalla riga di comando, dove ogni utente locale lo legge nella tabella dei processi. Che per una devnet le chiavi siano costanti pubbliche e' una scelta legittima e ben dichiarata — ma **nel runbook, che non e' versionato** (RF-005), e non nella spec. | remediation=`Debug` manuale che stampa la sola chiave pubblica; togliere `Copy` e `PartialEq` o sostituire il confronto con uno a tempo costante; azzerare lo scalare e il `prefix` al `Drop`; leggere il seme da file o variabile d'ambiente invece che da `argv`. Dichiarare nella spec, con scopo e scadenza, che le chiavi della devnet sono deterministiche e pubbliche. **Condizione di chiusura**: un test che asserisce che `format!("{:?}", key)` non contiene i byte dello scalare.

RF-010 | category=robustness | severity=medium | criterion=**Il buffer fra altezze e' l'asse illimitato di RF-004 di [REVIEW-047], un livello sopra, e ha un rilascio morto.** `FutureHeightBuffer` limita venti altezze per cinquecento **messaggi**, cioe' diecimila buste — ma il tetto e' sui messaggi, non sui byte, e `payload` e' un `JsonObject` di dimensione non limitata al confine (RF-001: non c'e' confine). Peggio: `prune_before` **non ha alcun chiamante** (ESEGUITO: `grep`, unica occorrenza la definizione). `drain_height` rimuove la sola altezza esatta, quindi ogni altezza saltata — e il percorso `finalized_block` fa saltare altezze per costruzione, sostituendo il motore a `blk_height + 1` — lascia la propria voce nella `BTreeMap` per sempre. Su una catena lunga il buffer accumula una voce per ogni altezza mai saltata. Infine, `finalized_block` con `blk_height > curr_height` viene **bufferizzato senza alcuna verifica**: nemmeno `finalized.verify`, che il ramo della stessa altezza esegue. | remediation=Chiamare `prune_before(self.engine.height())` a ogni avanzamento di altezza — la funzione esiste gia'; limitare il buffer in **byte** oltre che in messaggi; verificare il certificato di un `finalized_block` prima di trattenerlo, non dopo. **Condizione di chiusura**: un test che fa saltare dieci altezze e osserva il buffer vuoto, e uno che riempie il buffer con buste grandi e osserva il tetto in byte.

RF-011 | category=documentation | severity=medium | criterion=**Due delle quindici sezioni scritte a mano dal Lead affermano il falso.** (a) `NodeRunner::handle_envelope`: *«Una busta di un'altra rete o di **un'altra catena** non e' un errore: viene scartata.»* Il `chain_id` non e' mai confrontato in `handle_envelope`, ne' in alcun punto del percorso di ingresso; si confronta il solo `network_id`. (b) `NodeRunner::run`: *«Restituisce errore se un'azione del motore non e' eseguibile: ... **una trasmissione che non parte**, un timer che non si arma.»* Ogni trasmissione e' `let _ = self.outbound_tx.try_send(...)`: l'esito e' scartato in tutti e cinque i siti, e una trasmissione che non parte non produce alcun errore — ne' un log. E' la famiglia 2 del censimento, la stessa per cui `verifier.rs` e' stato corretto oggi. **Le due affermazioni che il Lead ha dichiarato portanti sono invece vere**, e la prima e' stata eseguita (RF-008). | remediation=Correggere le due frasi, oppure — meglio per (a) — attuare cio' che la frase dichiara, perche' il confronto di `chain_id` e' comunque richiesto da RF-001. Per (b), decidere se una trasmissione persa e' un errore: se lo e', propagarlo; se non lo e', dirlo.

RF-012 | category=documentation | severity=low | criterion=**La ragione accanto a `#[allow(clippy::too_many_arguments)]` descrive una firma diversa da quella che sormonta.** Dice: *«sono i campi che la busta firmata di `wire.md` impone, piu' il verificatore»*. `build_and_sign` non prende alcun verificatore: prende un **`signer`**. E i nove argomenti non sono i campi della busta: `validity_duration_ms` non e' un campo di `wire.md` (`expires_at_ms` lo e'), mentre `message_id`, `signature` e `schema_version` sono campi della busta e non argomenti. Gli altri tre `#[allow]` hanno ragioni che reggono e che condivido: non ristrutturare un ciclo di eventi di consenso dentro una passata dichiarata meccanica e' la scelta giusta, e averlo dichiarato prima di cominciare e' il modo giusto di farla. | remediation=Riscrivere la ragione su cio' che la firma e': i sette campi che il chiamante deve fornire, la durata da cui si deriva la scadenza, e la chiave che firma.

RF-013 | category=documentation | severity=medium | criterion=**La ragione scritta accanto a `now_ms`, e ripetuta nel messaggio di commit, e' falsa; e la saturazione sceglie il verso sbagliato.** Il commento dice *«il valore che arma ogni timeout di consenso»* e il commit ripete *«sul valore che arma ogni timeout di consenso»*. ESEGUITO (`grep -n now_ms`): i cinque siti d'uso sono tutti `created_at_ms` di una busta. I timeout non passano di li': `Action::ScheduleTimeout` porta un `delay_ms` che va a `tokio::time::sleep`, senza orologio a muro. Il cast reso esplicito e' reale e la scelta di renderlo esplicito e' giusta; la giustificazione descrive un'altra funzione. Sul merito: `unwrap_or(u64::MAX)` fallisce **aperto** nel verso che conta, perche' `expires_at_ms = created_at_ms.saturating_add(...)` diventa a sua volta `u64::MAX`, cioe' una busta che non scade mai — proprio il controllo che RF-001 chiede di attivare. Il ramo gemello `unwrap_or(Duration::ZERO)` per un orologio prima dell'epoca fallisce invece **chiuso**, con una busta scaduta all'istante. Due ripieghi silenziosi in due versi opposti sulla stessa funzione. | remediation=Far restituire a `now_ms` un `Result` e propagare: un orologio che non si legge e' un fallimento del nodo, non un valore. Se la propagazione costasse troppo, `0` e' onesto in entrambi i rami, perche' rende la busta immediatamente invalida invece che eterna. E correggere la frase.

RF-014 | category=maintainability | severity=medium | criterion=**Segnaposto sul percorso di produzione, non dichiarati come eccezione.** `Action::RequestValue` costruisce ogni header con `state_root: Digest32::repeated(0x33)`, `consensus_parameters_hash: Digest32::repeated(0x44)` e `timestamp_ms` come costante aritmetica `1_787_654_400_000 + height*5_000 + round*1_000 + proposer_idx`; `devnet_4_validator_set` pone `key_binding_signature: [0u8; 64]`; ogni busta porta `nonce: [0u8; 16]`. Alcuni sono inevitabili finche' non esiste un esecutore di stato, e la scelta puo' essere giusta. `QUALITY.md` §*Shortcuts* non la vieta: chiede che sia **dichiarata** con scopo, rischio, condizione di scadenza e lavoro seguente. La spec non li dichiara. Il `timestamp_ms` merita una riga in piu': e' un orologio finto e monotono nell'altezza, il che significa che nessun test di questa consegna esercita alcuna regola temporale sui blocchi. | remediation=Elencarli nella spec o in un debito, ciascuno con la condizione che lo chiude e la spec che la porta.

RF-015 | category=verification-integrity | severity=medium | criterion=**Tre affermazioni dell'evidenza non reggono alla lettura del codice che descrivono.** (a) *«`main.rs`: CLI binary supporting `start` **and `generate-keys`** subcommands»*: l'`enum Command` ha il solo `Start`. (b) *«`buffer.rs`: ... with bounded memory **and expiration**»*: non c'e' scadenza; la sola funzione che potrebbe fornirla, `prune_before`, non e' mai chiamata (RF-010). (c) *«`envelope.rs`: `SignedEnvelope` wire wrapper with JCS serialization **and cryptographic envelope verification**»*: la funzione esiste e non ha chiamanti, che e' RF-001 detto dal lato dell'evidenza. | remediation=Correggere le tre righe. Sono il genere di affermazione che una review successiva legge invece di verificare.

RF-016 | category=robustness | severity=low | criterion=**Ogni trasmissione e' `let _ = try_send(...)`, e la proposta non passa dal WAL.** Il canale `outbound_tx` ha capienza 1000; a canale pieno un voto **gia' reso durevole** viene scartato in silenzio, senza log e senza contatore, e il nodo prosegue credendo di aver votato. Separatamente, `Action::Broadcast(Outbound::Proposal)` non passa da alcun controllo del WAL: un proposer che riparte puo' proporre due valori diversi allo stesso `(h, r)`. `wire.md` riga 516 dichiara che l'equivocazione di proposta e' rilevabile ma non attribuibile e «costa un round e non piu'», il che tiene la severita' bassa — ma la simmetria con i voti va decisa, non lasciata. | remediation=Registrare l'esito del `try_send`, almeno come log ed errore per i messaggi di consenso; valutare se il WAL debba coprire anche `(h, r) -> proposta`.

RF-017 | category=robustness | severity=low | criterion=**`--seed-hex` corto fa panicare il binario.** `main.rs`: `seed.copy_from_slice(&bytes[..32])` dopo `hex::decode`, senza controllo di lunghezza. Un seme di 31 byte panica sull'indicizzazione. E' input dell'operatore, non della rete, quindi la severita' e' bassa — ma il runbook istruisce a passare quel flag. | remediation=Controllare la lunghezza e restituire un errore con il messaggio che dice quanti byte servono.

RF-018 | category=robustness | severity=low | criterion=**Un `finalized_block` che non verifica sparisce senza traccia.** Nel ramo `blk_height == curr_height`, la verifica e' `finalized.verify(...).is_ok()` dentro la condizione: se fallisce non succede nulla e nessuno lo sa. Nei rami `block_proposal`, `prevote` e `precommit` il rifiuto viene almeno stampato su `stderr`. Un pari che invia certificati non validi e' un segnale, e qui e' invisibile. | remediation=Registrare il rifiuto con la ragione, come fanno gli altri tre rami.

RF-019 | category=maintainability | severity=low | criterion=**Un cast `as` non guardato e' sopravvissuto alla passata che ha tolto l'altro.** `node.rs`, `Action::RequestValue`: `.position(...).unwrap_or(0) as u64`. Oltre al cast, `unwrap_or(0)` fa comportare come indice 0 un nodo che non si trova nel proprio set — condizione che `Engine::start` gia' rifiuta, quindi irraggiungibile oggi, ma il ripiego dice il contrario di cio' che il motore garantisce. | remediation=`u64::try_from(...)` e un errore invece di `unwrap_or(0)`, coerentemente con la scelta fatta per `now_ms`.

RF-020 | category=documentation | severity=low | criterion=**La derivazione dei timeout non produce i valori scritti sotto di essa.** `config.rs` documenta *«`prevote_ms`: 2 * `Delta_net` = 100ms»* e *«`precommit_ms`: 2 * `Delta_net` = 100ms»*; i valori restituiti sono `150` e `150`. Anche `round_increment_ms` e' documentato *«`Delta_net` = 50ms»* e vale `100`. Solo `propose_ms` corrisponde. Il criterio chiede valori «derivati da una grandezza dichiarata»: la grandezza e' dichiarata, la derivazione e' scritta, e tre valori su quattro non la seguono. | remediation=Correggere i valori o la derivazione, e dire quale delle due era quella giusta.

## Required follow-up

**Bloccanti — un giro di remediation, non un debito**

1. RF-001, il confine della busta. E' il difetto piu' grave di questa consegna e
   non ha attenuanti: la funzione che serve e' gia' scritta, manca la chiamata.
2. RF-002, il lock non ripristinato. E' il criterio che definisce la spec.
3. RF-003, RF-004, RF-005: tre criteri e una gate marcati `[x]` senza cio' che il
   loro testo richiede. Vanno riportati a `[ ]` o soddisfatti.
4. RF-006 e RF-007, che bloccano rispettivamente una postura di rete difendibile
   e `GATE-SUBSET-DECLARED`.

**Da portare in una review accettata, o promuovere a debito con un bersaglio**

RF-008 (WAL troncato), RF-009 (chiave in `Debug`), RF-010 (buffer e
`prune_before` morto), RF-011 (due `# Errors` false), RF-013 (`now_ms`),
RF-014 (segnaposto non dichiarati), RF-015 (tre affermazioni dell'evidenza).

**Note non bloccanti**: RF-012, RF-016, RF-017, RF-018, RF-019, RF-020.

**Non contestato, e va detto**: la pompa `Event`/`Action` e' scritta con
attenzione, la regola *durevole prima di trasmettere* e' attuata su ogni
percorso, il `fsync` c'e', `store.rs` verifica la continuita' della catena in
lettura e in scrittura, e il banco a quattro processi separati e' un test vero
che uccide un processo vero. La consegna non e' fragile dove ci si aspettava che
lo fosse: e' scoperta dove nessuno ha guardato.

## Nota di chiusura sull'albero

Mentre scrivevo questa review l'albero e' cambiato sotto di me, e lo dichiaro
invece di lasciare il rilievo disallineato. Al momento della verifica finale
(ESEGUITO, `git status --porcelain`): `docs/devnet-runbook.md` risulta **messo
in staging** e `data/` e `data-val*.log` sono stati rimossi dall'albero di
lavoro. Della parte di RF-005 che riguarda il runbook, quindi, resta da
verificare solo che il commit lo contenga davvero; **resta invece aperto** il
resto del rilievo: `.gitignore` non ha ancora alcuna voce per `data/`, e la
`default_value` di `--data-dir` continua a puntare dentro l'albero sorgente.
Tutto il resto di questa review descrive `fde4d6b`, che e' l'albero su cui la
gate e' stata eseguita.

## Final decision

**Raccomando `review_changes_requested`.**

Non `review_accept` con debiti, e la ragione non e' il numero dei rilievi. Per
RF-001 il progetto oggi finalizzerebbe un blocco scelto da chiunque sappia
aprire un socket, e per RF-002 un `kill -9` produce da solo l'equivocazione che
[ADR-018] assume impossibile. L'avvertenza rafforzata dell'incarico si applica
alla lettera: *un difetto che permetta a un nodo onesto di equivocare dopo un
riavvio e' bloccante*.

Non `review_block`, perche' nulla qui dipende da un fatto esterno al progetto: i
due bloccanti hanno entrambi un rimedio additivo e circoscritto — una chiamata
al confine, e due campi in `EngineConfig` alimentati da un log che gia' contiene
il dato. E' un giro di remediation, non una riprogettazione.

`GATE-CI-GREEN` e `GATE-LEAD-REPRO` restano del Lead. Segnalo, per la seconda,
che uccidere un nodo dal runbook **non** esercitera' RF-002: per vederlo serve
che l'uccisione cada a un round maggiore di zero di un'altezza non decisa, e la
finestra e' stretta. L'assenza di equivocazione osservata in quella prova non
sara' una smentita di questo rilievo.
