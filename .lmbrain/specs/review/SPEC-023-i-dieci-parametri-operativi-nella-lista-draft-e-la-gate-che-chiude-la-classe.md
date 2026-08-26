---
id: SPEC-023
# Note: Quote the title if it contains a colon
title: "I dieci parametri operativi nella lista DRAFT, e la gate che chiude la classe"
status: review
kind: feature
priority: high
area: governance
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: terra
thinking_level: maximum
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: [SKILL-001, SKILL-002, SKILL-003]
verification_gates: []
related_decisions: [ADR-012, ADR-010, ADR-017]
links: [DEBT-036]
created: 2026-08-26
updated: 2026-08-26
tags: [conformance, ledger, light-client]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "set tags"
  - date: 2026-08-26
    action: "transitioned backlog -> ready"
  - date: 2026-08-26
    action: "transitioned ready -> working"
  - date: 2026-08-26
    action: "transitioned working -> review"
---
# I dieci parametri operativi nella lista DRAFT, e la gate che chiude la classe

## Objective

Chiudere la **prima metà** di [DEBT-036]: portare nella lista DRAFT dei parametri di lancio i dieci campi di `ConsensusParametersBody` che oggi non vi compaiono, e costruire la gate che impedisce all'undicesimo di nascere fuori da entrambe le liste.

**E produrre l'analisi che la seconda metà richiede**, senza prenderne la decisione: per ciascuno dei dieci, cosa governa, cosa guadagna un quorum sedente che lo porta al proprio estremo, e cosa già lo vincola per altra via. Quell'analisi va all'operatore e diventa un ADR.

**La divisione è deliberata.** Aggiungere un limite di magnitudine in genesi è una regola di protocollo nuova, e su questo progetto quelle si decidono in un ADR — è il precedente appena stabilito da [ADR-017]. Questa spec prepara la decisione e **non la prende**.

## Context

`ConsensusParametersBody` ha **venti campi**. I dieci di elezione hanno **due** protezioni: un limite di magnitudine preso dall'ancora di fiducia di genesi in `ledger.md#magnitudes-not-only-relations`, e una voce nella lista DRAFT di `README.md` che li dichiara aperti.

Gli altri dieci non hanno **né l'una né l'altra**, e valgono tutti `1`, che è il valore delle fixture:

`max_clock_drift_ms`, `max_envelope_validity_ms`, `max_transport_attestation_validity_ms`, `max_transport_attestation_future_skew_ms`, `replay_cache_entries_per_peer`, `replay_cache_entries_global`, `max_weak_subjectivity_age_ms`, `max_current_balance_age_ms`, `app_suspension_notice_epochs`, `min_revocation_effective_delay_blocks`.

Sono la metà **operativa e di sicurezza**: orologi, finestre di validità, cache anti-replay, freschezza dell'ancora di fiducia, ritardo della revoca. **Li firma il quorum sedente**, che è la ragione dichiarata per cui il blocco dei vincoli esiste per l'altra metà.

**Cinque dei dieci erano già emersi, uno alla volta e in incidenti separati** — `max_clock_drift_ms` in [SPEC-020], `D_max`/`S_max` da [SPEC-013], il pavimento della revoca da [ADR-017], `max_weak_subjectivity_age_ms` guardandolo adesso — e ogni volta sono stati registrati come una taratura pendente, mai come il sintomo di una lista incompleta. **La metà restante non l'ha guardata nessuno.**

`README.md` dice che finché i parametri firmati non selezionano valori, un deployment è una rete di sviluppo e **MUST NOT** identificarsi come mainnet. Dieci parametri fuori dalla lista sono dieci decisioni che nessun documento reclama.

## Scope
### Included

- **I dieci nella lista DRAFT**, raggruppati per cosa governano e non elencati alla rinfusa, con dichiarata accanto a ciascuno **la grandezza che lo vincola o lo vincolerebbe**.
- **`min_revocation_effective_delay_blocks` incluso**, anche se [SPEC-022] lo tocca per altro motivo. Se le due spec corrono insieme, coordinarsi sul punto di contatto e dichiararlo.
- **La gate di chiusura della classe**: uno strumento versionato che confronta i campi di `ConsensusParametersBody` con l'**unione** di lista DRAFT e blocco dei vincoli, e **fallisce su un campo che non sta in nessuna delle due**. Provata in negativo.
- **L'analisi per ciascuno dei dieci**, nel formato dato sotto, consegnata come documento e non come opinione sparsa nella trascrizione.

### Excluded

- **La scelta dei valori di lancio.** Nessun numero viene fissato da questa spec. I dieci restano DRAFT.
- **L'aggiunta di limiti di magnitudine al blocco dei vincoli.** È la seconda metà di [DEBT-036], è una regola di protocollo, e va in un ADR deciso dall'operatore sull'analisi che questa spec produce. **Non anticiparla.**
- Ogni modifica a `params.rs` o alla validazione: finché i limiti non sono decisi non c'è nulla da imporre.

## Existing-project analysis

- `docs/protocol/README.md:808-829` — `ConsensusParametersBody`, i venti campi.
- `docs/protocol/README.md:1608-1632` — la sezione DRAFT, che oggi copre i parametri di enrollment, quelli economici e i dieci di elezione.
- `docs/protocol/ledger.md:1985-2012` — il blocco dei vincoli di magnitudine, e la frase che ne dichiara lo scopo.
- `docs/protocol/README.md:1599-1606` — la risoluzione della circolarità su `max_weak_subjectivity_age_ms`, che è il secondo canale di cui l'analisi deve tenere conto.
- `sim/tools/published_artifacts.py` e `.toml` — la macchina delle gate, dove la gate nuova va cablata o accanto a cui va costruita.

## Technical proposal

**Per ciascuno dei dieci, l'analisi risponde a quattro domande, e la quarta è quella che conta:**

1. **Cosa governa**, in una frase, con la riga del documento che lo stabilisce.
2. **Cosa ottiene un quorum sedente che lo porta al massimo**, e cosa ottiene portandolo al minimo. I due estremi non sono simmetrici e vanno guardati separatamente — è la lezione di [ADR-013] sulla cadenza.
3. **Cosa già lo vincola per altra via**: un altro parametro legato da un MUST, un secondo canale come il checkpoint, o nulla.
4. **Da quale grandezza dipende la proprietà che vorremmo**, che **non è necessariamente il parametro stesso**. Dove le due coincidono un tetto assoluto è la forma giusta; dove divergono lo è un vincolo relazionale.

**La domanda 4 è il punto della spec.** Aggiungere dieci righe di tetto in blocco sarebbe la **famiglia 3** del censimento — vincolare la grandezza nominata invece di quella da cui la proprietà dipende — e [DEBT-036] la nomina esplicitamente come la cosa da non fare. `max_weak_subjectivity_age_ms` è già l'esempio: ha un MUST che lo lega a `min_revocation_effective_delay_blocks`, quindi il suo vincolo naturale è **relazionale**, e ha in più un secondo canale che lo vincola di fatto.

**La gate** è la contromisura alla forma e non alla singola occorrenza. Cinque volte su questo progetto una gate ha misurato l'insieme **dichiarato** invece di quello **osservato**, e ogni volta il membro mancante era l'ultimo arrivato. Questa gate va costruita per fallire nella direzione che conta: **un campo dello schema che non compare in nessuna delle due liste**.

## Files and areas involved

- `docs/protocol/README.md` — la sezione DRAFT
- `sim/tools/` — la gate nuova e la sua prova in negativo
- `sim/tools/published_artifacts.toml` — se la gate va cablata lì
- `.lmbrain/knowledge/` — il documento di analisi dei dieci

## Acceptance criteria

- [x] Tutti e dieci i parametri compaiono nella lista DRAFT, raggruppati per cosa governano, ciascuno con la grandezza che lo vincola dichiarata accanto.
- [x] La lista DRAFT continua a coprire ciò che copriva prima: **nessuna voce esistente è stata persa** riorganizzandola, e la trascrizione lo dimostra confrontando prima e dopo.
- [x] Esiste uno strumento versionato che confronta i campi di `ConsensusParametersBody` con l'unione di lista DRAFT e blocco dei vincoli.
- [x] Lo strumento **fallisce** su un campo dello schema assente da entrambe le liste, e il fallimento è stato **osservato** aggiungendo un campo finto.
- [x] Lo strumento fallisce anche **nell'altra direzione**: una voce delle liste che non corrisponde ad alcun campo dello schema. Osservato.
- [x] Lo strumento è `PASS` sull'albero reale a fine consegna.
- [x] Il documento di analisi copre **dieci parametri su dieci**, con le quattro domande risposte per ciascuno, e dichiara per ciascuno **cosa è stato letto** per rispondere.
- [x] L'analisi distingue esplicitamente i parametri per cui il vincolo naturale è **relazionale** da quelli per cui è **di magnitudine**, e non propone un tetto uniforme.
- [x] **Nessun valore di lancio è stato fissato**, e nessun limite è stato aggiunto al blocco dei vincoli.
- [x] `cargo test --workspace --all-features`, `clippy -D warnings`, `fmt --check` puliti; `published_artifacts.py` `PASS`.

## Implementation plan

1. Enumerare i venti campi dallo schema, non dal ricordo, e classificarli contro le due liste. È l'inventario da cui discende il resto.
2. Scrivere la gate **prima** di correggere la lista, così che la si veda fallire sui dieci reali e non solo su un campo finto. È la prova più onesta che lo strumento funziona.
3. Riorganizzare la sezione DRAFT e aggiungere i dieci, verificando di non aver perso voci.
4. Provare la gate in negativo nelle due direzioni ([SKILL-001]).
5. Scrivere l'analisi, un parametro alla volta, dichiarando le fonti.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-CLASS-CLOSED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Lo strumento nuovo è `PASS` sull'albero e la trascrizione mostra i venti campi classificati uno per uno.
- [x] GATE-NEGATIVE-PROOF | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Lo strumento è stato **osservato fallire** in entrambe le direzioni: un campo dello schema fuori da entrambe le liste, e una voce di lista senza campo corrispondente ([SKILL-001]).
- [x] GATE-SEEN-IT-FAIL-FIRST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Lo strumento è stato eseguito **prima** della correzione della lista, e la trascrizione mostra che nominava i dieci parametri reali. Una gate che nasce verde non ha mai dimostrato di vedere.
- [x] GATE-DRAFT-NO-LOSS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Confronto fra la sezione DRAFT prima e dopo, che dimostra che nessuna voce preesistente è stata persa nella riorganizzazione.
- [ ] GATE-CI-GREEN | kind=manual | owner=agent | phase=before-done | evidence=transcript | Pipeline reale verde, con numero di run e commit.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | Review di AGENT-007 **sull'analisi**, non sullo strumento: è l'analisi che l'operatore userà per decidere, e un errore lì si propaga in un ADR.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

**Il carico di questa spec è nell'analisi, non nel diff**, ed è la ragione per cui il `thinking_level` è `maximum` mentre il `capability_tier` è `terra`. I file cambiati sono pochi; le domande sono dieci, e la quarta di ciascuna è quella su cui il progetto ha già sbagliato tre volte.

**La tentazione da cui guardarsi è il tetto uniforme.** È rapido, sembra completo, e sarebbe famiglia 3. Se l'analisi concludesse che per un parametro il vincolo giusto è relazionale e non di magnitudine, **è un risultato e va scritto**, non una lacuna da riempire con un numero.

**`min_revocation_effective_delay_blocks` è toccato anche da [SPEC-022].** Se le due corrono insieme, il punto di contatto va coordinato e dichiarato; se corrono in sequenza, la seconda verifica di non aver disfatto la prima.

**Se l'analisi trovasse che uno dei dieci non è governato affatto** — cioè che il quorum non può cambiarlo perché qualcosa lo fissa altrove — è un risultato più interessante della classificazione, e va riportato al Lead invece che archiviato come caso banale.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **Non decidere i limiti.** Questa spec prepara una decisione dell'operatore e non la prende. Proporre è il compito; scegliere no.
- **Consegna ogni dimostrazione insieme al perimetro su cui vale**, e conta ogni superlativo assoluto invece di scriverlo.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

1. **Strumento versionato di chiusura della classe (`sim/tools/consensus_parameters_closure.py`)**:
   - Creato lo strumento di verifica che estrae i 20 campi di `ConsensusParametersBody` dallo schema in `docs/protocol/README.md` e verifica che l'unione dei parametri coperti dal blocco dei vincoli di rotazione/magnitudine in `docs/protocol/ledger.md` e dai parametri dichiarati aperti nella sezione DRAFT di `docs/protocol/README.md` copra tutti i 20 campi senza eccezioni (`C1-SCHEMA-NOT-COVERED`).
   - Verifica inoltre in direzione inversa l'assenza di parametri di consenso orfani o fantasma (`C2-ORPHAN-PARAM`).
   - Include un test runner di prova negativa automatica su directory isolate temporanee (`--negative`).

2. **Verifica "Fail First" prima della modifica di `README.md` (`GATE-SEEN-IT-FAIL-FIRST`)**:
   - Eseguito `sim/tools/consensus_parameters_closure.py` prima di toccare la sezione DRAFT: lo strumento ha fallito identificando esattamente i 10 parametri operativi mancanti da entrambe le liste.

3. **Riorganizzazione della sezione DRAFT in `docs/protocol/README.md` (`GATE-DRAFT-NO-LOSS`)**:
   - Inseriti tutti i 10 parametri operativi di `ConsensusParametersBody` raggruppati per ambito logico (orologi/buste/attestazioni, cache anti-replay, soggettività debole e freschezza saldi, sospensioni app e ritardi revoca), dichiarando per ciascuno la grandezza che lo vincola o lo vincolerebbe.
   - Verificato che nessuna voce preesistente (parametri di enrollment, economici/ricompense, elezione validatori e vincoli di governance) sia andata perduta.

4. **Prova in negativo nelle due direzioni (`GATE-NEGATIVE-PROOF`)**:
   - Eseguito `python sim/tools/consensus_parameters_closure.py --negative`, provando il fallimento sia su un campo dello schema fittizio assente da entrambe le liste (`C1-SCHEMA-NOT-COVERED`), sia su un parametro orfano presente nella lista DRAFT ma assente dallo schema (`C2-ORPHAN-PARAM`).

5. **Documento di analisi approfondita dei dieci parametri operativi (`.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md`)**:
   - Redatto il documento che esamina ciascuno dei 10 parametri rispondendo alle 4 domande obbligatorie (cosa governa con citazione esatta delle righe, asimmetria tra gli estremi massimo/minimo, vincoli preesistenti o canali out-of-band, e grandezza da cui dipende la proprietà).
   - Distinti esplicitamente i parametri con vincolo naturale di magnitudine assoluta da quelli con vincolo relazionale o ibrido.
   - Nessun valore di lancio o limite è stato introdotto arbitrariamente nel blocco dei vincoli.

### Files changed

- `docs/protocol/README.md`: riorganizzazione sezione DRAFT con inserimento dei dieci parametri operativi.
- `sim/tools/consensus_parameters_closure.py`: nuovo strumento versionato di chiusura della classe con prova in negativo.
- `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md`: documento di analisi dei dieci parametri operativi per la futura decisione ADR.
- `.lmbrain/specs/working/SPEC-023-i-dieci-parametri-operativi-nella-lista-draft-e-la-gate-che-chiude-la-classe.md`: aggiornamento criteri di accettazione ed evidenze.

### Verification performed

- `python sim/tools/consensus_parameters_closure.py` (esecuzione prima della modifica: FAIL sui 10 parametri operativi mancanti).
- `python sim/tools/consensus_parameters_closure.py` (esecuzione finale: PASS con 20/20 campi coperti).
- `python sim/tools/consensus_parameters_closure.py --negative` (PASS su mutazioni C1 e C2).
- `python sim/tools/published_artifacts.py` (PASS, 11 classi verificate).
- `cargo fmt --check` (PASS, formattazione pulita).
- `cargo clippy -- -D warnings` (PASS, zero warning).
- `cargo test --workspace --all-features` (PASS, 85 test passati).

### Verification transcript

#### 1. GATE-SEEN-IT-FAIL-FIRST (Esecuzione prima della modifica della lista DRAFT)

```text
$ python sim/tools/consensus_parameters_closure.py
ConsensusParametersBody fields: 20 total
  In constraint block:          10
  In DRAFT list:                0
  Union covered:                10

Classification of all 20 fields:
  app_suspension_notice_epochs                  [             ] [     ]
  candidacy_close_blocks                        [CONSTRAINED] [     ]
  election_entropy_blocks                       [CONSTRAINED] [     ]
  election_epoch_blocks                         [CONSTRAINED] [     ]
  max_clock_drift_ms                            [             ] [     ]
  max_current_balance_age_ms                    [             ] [     ]
  max_envelope_validity_ms                      [             ] [     ]
  max_transport_attestation_future_skew_ms      [             ] [     ]
  max_transport_attestation_validity_ms         [             ] [     ]
  max_weak_subjectivity_age_ms                  [             ] [     ]
  min_revocation_effective_delay_blocks         [             ] [     ]
  replay_cache_entries_global                   [             ] [     ]
  replay_cache_entries_per_peer                 [             ] [     ]
  validator_churn_cap_seats                     [CONSTRAINED] [     ]
  validator_cooldown_epochs                     [CONSTRAINED] [     ]
  validator_max_consecutive_terms               [CONSTRAINED] [     ]
  validator_max_set_size                        [CONSTRAINED] [     ]
  validator_min_capture_epochs                  [CONSTRAINED] [     ]
  validator_min_set_size                        [CONSTRAINED] [     ]
  validator_target_set_size                     [CONSTRAINED] [     ]

FAIL: 10 finding(s):
  C1-SCHEMA-NOT-COVERED: field 'app_suspension_notice_epochs' (docs/protocol/README.md:817) of ConsensusParametersBody is present in neither the DRAFT launch parameters list nor the ledger.md constraint block
  C1-SCHEMA-NOT-COVERED: field 'max_clock_drift_ms' (docs/protocol/README.md:809) of ConsensusParametersBody is present in neither the DRAFT launch parameters list nor the ledger.md constraint block
  C1-SCHEMA-NOT-COVERED: field 'max_current_balance_age_ms' (docs/protocol/README.md:816) of ConsensusParametersBody is present in neither the DRAFT launch parameters list nor the ledger.md constraint block
  C1-SCHEMA-NOT-COVERED: field 'max_envelope_validity_ms' (docs/protocol/README.md:810) of ConsensusParametersBody is present in neither the DRAFT launch parameters list nor the ledger.md constraint block
  C1-SCHEMA-NOT-COVERED: field 'max_transport_attestation_future_skew_ms' (docs/protocol/README.md:812) of ConsensusParametersBody is present in neither the DRAFT launch parameters list nor the ledger.md constraint block
  C1-SCHEMA-NOT-COVERED: field 'max_transport_attestation_validity_ms' (docs/protocol/README.md:811) of ConsensusParametersBody is present in neither the DRAFT launch parameters list nor the ledger.md constraint block
  C1-SCHEMA-NOT-COVERED: field 'max_weak_subjectivity_age_ms' (docs/protocol/README.md:815) of ConsensusParametersBody is present in neither the DRAFT launch parameters list nor the ledger.md constraint block
  C1-SCHEMA-NOT-COVERED: field 'min_revocation_effective_delay_blocks' (docs/protocol/README.md:818) of ConsensusParametersBody is present in neither the DRAFT launch parameters list nor the ledger.md constraint block
  C1-SCHEMA-NOT-COVERED: field 'replay_cache_entries_global' (docs/protocol/README.md:814) of ConsensusParametersBody is present in neither the DRAFT launch parameters list nor the ledger.md constraint block
  C1-SCHEMA-NOT-COVERED: field 'replay_cache_entries_per_peer' (docs/protocol/README.md:813) of ConsensusParametersBody is present in neither the DRAFT launch parameters list nor the ledger.md constraint block
```

#### 2. GATE-DRAFT-NO-LOSS (Confronto DRAFT section prima e dopo)

```diff
--- docs/protocol/README.md (prima)
+++ docs/protocol/README.md (dopo)
@@ -1610,23 +1610,30 @@
 The algorithms and parameter names are fixed in v0, but their launch values are
 not economic facts and remain open:
 
-- enrollment `difficulty_bits` and the Argon2id cost profile: benchmark-derived
-  fixed values vs epoch values bounded by governance. Both must be chosen
-  together, because with a memory-hard primitive the cost of one evaluation and
-  the expected number of evaluations are independent knobs;
-- the per-epoch existence fund, work reward curves, hosting prices, and
-  subscription minimums: simulator output vs conservative bootstrap values;
-- the validator election parameters — epoch length, candidacy close, entropy
-  window, set sizes, churn cap, term limit, cooldown, declared capture horizon,
-  the eligibility threshold with its window, and the minimum number of distinct
-  issuers behind a contribution score. The **algorithm** is no longer open: it is
-  specified in [ledger.md](ledger.md#validator-election-and-rotation). Nor are
-  the relations among these values open, nor their magnitudes — a
-  consensus-parameters document that violates the constraint block of
-  [ledger.md](ledger.md#rotation-the-cap-and-the-floor), or that leaves the
-  [election bounds](#election-bounds) of the genesis trust anchor, is rejected on
-  acceptance. The simulator therefore chooses inside a feasible region that the
-  chain's own governance cannot widen.
+- **Enrollment proof-of-work parameters**:
+  - `difficulty_bits` and the Argon2id cost profile (`memory_kib`, `lanes`, `passes`): benchmark-derived
+    fixed values vs epoch values bounded by governance. Both must be chosen
+    together, because with a memory-hard primitive the cost of one evaluation and
+    the expected number of evaluations are independent knobs;
+- **Economic and reward policy parameters**:
+  - the per-epoch existence fund (`existence_fund_microtokens_per_epoch`), work reward curves (`storage_microtokens_per_byte_epoch`, `compute_microtokens_per_million_fuel`), hosting prices (`microtokens_per_replica_epoch`, `microtokens_per_gib_epoch`, `microtokens_per_million_fuel`), and subscription minimums (`minimum_billable_epochs`, `billing_epoch_ms`): simulator output vs conservative bootstrap values;
+- **Validator election and rotation parameters**:
+  - epoch length (`election_epoch_blocks`), candidacy close (`candidacy_close_blocks`), entropy window (`election_entropy_blocks`), set sizes (`validator_min_set_size`, `validator_target_set_size`, `validator_max_set_size`), churn cap (`validator_churn_cap_seats`), term limit (`validator_max_consecutive_terms`), cooldown (`validator_cooldown_epochs`), declared capture horizon (`validator_min_capture_epochs`), the eligibility threshold (`validator_eligibility_threshold_units`) with its window (`validator_eligibility_window_epochs`), and the minimum number of distinct issuers (`validator_eligibility_min_issuers`) behind a contribution score. The **algorithm** is no longer open: it is specified in [ledger.md](ledger.md#validator-election-and-rotation). Nor are the relations among these values open, nor their magnitudes — a consensus-parameters document that violates the constraint block of [ledger.md](ledger.md#rotation-the-cap-and-the-floor), or that leaves the [election bounds](#election-bounds) of the genesis trust anchor, is rejected on acceptance. The simulator therefore chooses inside a feasible region that the chain's own governance cannot widen;
+- **Operational, transport, and network security consensus parameters** (the ten operational fields of `ConsensusParametersBody`):
+  - *Clocks, envelope validity, and transport attestations*:
+    - `max_clock_drift_ms`: allowable clock skew across nodes and block timestamps; constrained by physical clock synchronization tolerances and network round-trip latency;
+    - `max_envelope_validity_ms`: maximum lifetime of wire protocol message envelopes from `created_at_ms`; constrained by gossip message propagation latency and anti-replay horizon;
+    - `max_transport_attestation_validity_ms`: maximum validity duration ($D_{\max}$) of transport key attestations; constrained by transport key compromise exposure window and session rotation frequency;
+    - `max_transport_attestation_future_skew_ms`: forward clock skew tolerance ($S_{\max}$) on transport key attestation timestamps; constrained by `max_clock_drift_ms` and peer clock divergence;
+  - *Anti-replay cache capacity*:
+    - `replay_cache_entries_per_peer`: maximum tracked envelope identifiers per connected peer; constrained by peer transmission rate limit and node memory budget;
+    - `replay_cache_entries_global`: maximum total tracked envelope identifiers across all peers; constrained by global network gossip volume and node memory footprint;
+  - *Trust anchor freshness and state queries*:
+    - `max_weak_subjectivity_age_ms`: maximum age of a weak subjectivity checkpoint accepted by syncing nodes; constrained relationally by `min_revocation_effective_delay_blocks` (MUST) and bounded by the release distribution channel;
+    - `max_current_balance_age_ms`: maximum allowed staleness for served balance query state proofs; constrained by query turnaround latency and state-history retention window;
+  - *Lifecycle delays and revocations*:
+    - `app_suspension_notice_epochs`: notice period in epochs between proposing app suspension and enforcement; constrained by epoch duration and operator dispute remediation window;
+    - `min_revocation_effective_delay_blocks`: minimum delay between proposing an identity revocation and its effective height on validator set transitions; constrained relationally with `max_weak_subjectivity_age_ms` and validator succession coordination margin ($F$).
```

#### 3. GATE-CLASS-CLOSED (Esecuzione dopo l'aggiornamento della lista DRAFT)

```text
$ python sim/tools/consensus_parameters_closure.py
ConsensusParametersBody fields: 20 total
  In constraint block:          10
  In DRAFT list:                20
  Union covered:                20

Classification of all 20 fields:
  app_suspension_notice_epochs                  [             ] [DRAFT]
  candidacy_close_blocks                        [CONSTRAINED] [DRAFT]
  election_entropy_blocks                       [CONSTRAINED] [DRAFT]
  election_epoch_blocks                         [CONSTRAINED] [DRAFT]
  max_clock_drift_ms                            [             ] [DRAFT]
  max_current_balance_age_ms                    [             ] [DRAFT]
  max_envelope_validity_ms                      [             ] [DRAFT]
  max_transport_attestation_future_skew_ms      [             ] [DRAFT]
  max_transport_attestation_validity_ms         [             ] [DRAFT]
  max_weak_subjectivity_age_ms                  [             ] [DRAFT]
  min_revocation_effective_delay_blocks         [             ] [DRAFT]
  replay_cache_entries_global                   [             ] [DRAFT]
  replay_cache_entries_per_peer                 [             ] [DRAFT]
  validator_churn_cap_seats                     [CONSTRAINED] [DRAFT]
  validator_cooldown_epochs                     [CONSTRAINED] [DRAFT]
  validator_max_consecutive_terms               [CONSTRAINED] [DRAFT]
  validator_max_set_size                        [CONSTRAINED] [DRAFT]
  validator_min_capture_epochs                  [CONSTRAINED] [DRAFT]
  validator_min_set_size                        [CONSTRAINED] [DRAFT]
  validator_target_set_size                     [CONSTRAINED] [DRAFT]

PASS: all 20 ConsensusParametersBody fields are covered by constraint block or DRAFT list.
```

#### 4. GATE-NEGATIVE-PROOF

```text
$ python sim/tools/consensus_parameters_closure.py --negative
ok   unmutated copy passes
ok   C1-SCHEMA-NOT-COVERED caught schema field missing from both lists
ok   C2-ORPHAN-PARAM caught orphan parameter in DRAFT list

Negative proof: PASS - all defect classes observed failing.
```

#### 5. published_artifacts.py

```text
$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     20 candidate(s) checked
  C4-VALUE          60 candidate(s) checked
  C5-MIRROR         53 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C5-DISCOVERED     67 candidate(s) checked
  C10-PROBE        158 candidate(s) checked
  C11-CLAIMDOC       8 candidate(s) checked

published-artifact inventory: PASS
```

#### 6. Cargo checks & tests

```text
$ cargo fmt --check
$ cargo clippy -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
$ cargo test --workspace --all-features
test result: ok. 19 passed (tests/constraint_block.rs)
test result: ok. 12 passed (tests/election_degenerate.rs)
test result: ok. 9 passed (tests/genesis_derivation.rs)
test result: ok. 14 passed (tests/light_client_perimeter.rs)
test result: ok. 5 passed (tests/preimage_context.rs)
test result: ok. 8 passed (tests/sparse_account_state.rs)
test result: ok. 11 passed (tests/speccheck_conformance.rs)
test result: ok. 6 passed (tests/worked_example.rs)
test result: ok. 1 passed (coblox-ffi)
Total: 85 passed; 0 failed; 0 ignored
```
