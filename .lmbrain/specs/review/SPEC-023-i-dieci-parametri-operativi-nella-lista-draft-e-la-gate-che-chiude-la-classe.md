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
updated: 2026-08-27
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

- `docs/protocol/README.md`, blocco `ConsensusParametersBody` (§*"Signed protocol documents"*) — i venti campi.
- `docs/protocol/README.md`, §*"DRAFT: governance-selected launch parameters"* — la sezione DRAFT, che oggi copre i parametri di enrollment, quelli economici e i dieci di elezione.
- `docs/protocol/ledger.md`, §*"Rotation: the cap and the floor"* — il blocco dei vincoli di magnitudine, e la frase che ne dichiara lo scopo.
- `docs/protocol/README.md`, §*"The network-release trust key"*, paragrafo *"Resolving the parameter circularity"* — la risoluzione della circolarità su `max_weak_subjectivity_age_ms`, che è il secondo canale di cui l'analisi deve tenere conto.
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
- [ ] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=transcript | Pipeline reale verde, con numero di run e commit.
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
   - Formattazione conforme: rimossa ogni notazione matematica LaTeX (`$D_{\max}$`, `$S_{\max}$`, `$F$`) e imposto a capo rigoroso a <= 80 colonne su tutte le righe (remediation RF-002 e RF-003).

4. **Prova in negativo nelle due direzioni (`GATE-NEGATIVE-PROOF`)**:
   - Eseguito `python sim/tools/consensus_parameters_closure.py --negative`, provando il fallimento sia su un campo dello schema fittizio assente da entrambe le liste (`C1-SCHEMA-NOT-COVERED`), sia su un parametro orfano presente nella lista DRAFT ma assente dallo schema (`C2-ORPHAN-PARAM`).

5. **Documento di analisi approfondita dei dieci parametri operativi (`.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md`)**:
   - Redatto il documento che esamina ciascuno dei 10 parametri rispondendo alle 4 domande obbligatorie (cosa governa con citazione esatta delle righe, asimmetria tra gli estremi massimo/minimo, vincoli preesistenti o canali out-of-band, e grandezza da cui dipende la proprietà).
   - Distinti esplicitamente i parametri con vincolo naturale di magnitudine assoluta da quelli con vincolo relazionale o ibrido.
   - Nessun valore di lancio o limite è stato introdotto arbitrariamente nel blocco dei vincoli.

6. **Cablaggio in CI (`.github/workflows/ci.yml`)**:
   - Cablata l'esecuzione di `consensus_parameters_closure.py` e della sua prova in negativo `--negative` nel job `protocol-docs` della pipeline CI (remediation RF-004).

### Files changed

- `docs/protocol/README.md`: riorganizzazione sezione DRAFT con inserimento dei dieci parametri operativi, rimozione notazione LaTeX e wrap a 80 colonne.
- `sim/tools/consensus_parameters_closure.py`: nuovo strumento versionato di chiusura della classe con prova in negativo.
- `.github/workflows/ci.yml`: cablaggio della gate di chiusura e della prova in negativo nel job `protocol-docs`.
- `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md`: documento di analisi dei dieci parametri operativi per la futura decisione ADR.
- `.lmbrain/specs/review/SPEC-023-i-dieci-parametri-operativi-nella-lista-draft-e-la-gate-che-chiude-la-classe.md`: aggiornamento criteri di accettazione ed evidenze con remediation REVIEW-037.

### Verification performed

- `python sim/tools/consensus_parameters_closure.py` (esecuzione prima della modifica: FAIL sui 10 parametri operativi mancanti).
- `python sim/tools/consensus_parameters_closure.py` (esecuzione finale: PASS con 20/20 campi coperti).
- `python sim/tools/consensus_parameters_closure.py --negative` (PASS su mutazioni C1 e C2).
- `python sim/tools/published_artifacts.py` (PASS, 11 classi verificate).
- `cargo fmt --check` (PASS, formattazione pulita).
- `cargo clippy -- -D warnings` (PASS, zero warning).
- `cargo test --workspace --all-features` (PASS, 181 test passati: 35 in coblox_core lib + 145 nei 9 binari di integrazione + 1 in coblox_ffi).

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
@@ -1610,23 +1610,77 @@
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
+  - `difficulty_bits` and the Argon2id cost profile (`memory_kib`, `lanes`,
+    `passes`): benchmark-derived fixed values vs epoch values bounded by
+    governance. Both must be chosen together, because with a memory-hard
+    primitive the cost of one evaluation and the expected number of
+    evaluations are independent knobs;
+- **Economic and reward policy parameters**:
+  - the per-epoch existence fund (`existence_fund_microtokens_per_epoch`), work
+    reward curves (`storage_microtokens_per_byte_epoch`,
+    `compute_microtokens_per_million_fuel`), hosting prices
+    (`microtokens_per_replica_epoch`, `microtokens_per_gib_epoch`,
+    `microtokens_per_million_fuel`), and subscription minimums
+    (`minimum_billable_epochs`, `billing_epoch_ms`): simulator output vs
+    conservative bootstrap values;
+- **Validator election and rotation parameters**:
+  - epoch length (`election_epoch_blocks`), candidacy close
+    (`candidacy_close_blocks`), entropy window (`election_entropy_blocks`), set
+    sizes (`validator_min_set_size`, `validator_target_set_size`,
+    `validator_max_set_size`), churn cap (`validator_churn_cap_seats`), term
+    limit (`validator_max_consecutive_terms`), cooldown
+    (`validator_cooldown_epochs`), declared capture horizon
+    (`validator_min_capture_epochs`), the eligibility threshold
+    (`validator_eligibility_threshold_units`) with its window
+    (`validator_eligibility_window_epochs`), and the minimum number of
+    distinct issuers (`validator_eligibility_min_issuers`) behind a
+    contribution score. The **algorithm** is no longer open: it is specified in
+    [ledger.md](ledger.md#validator-election-and-rotation). Nor are the
+    relations among these values open, nor their magnitudes — a
+    consensus-parameters document that violates the constraint block of
+    [ledger.md](ledger.md#rotation-the-cap-and-the-floor), or that leaves the
+    [election bounds](#election-bounds) of the genesis trust anchor, is
+    rejected on acceptance. The simulator therefore chooses inside a feasible
+    region that the chain's own governance cannot widen;
+- **Operational, transport, and network security consensus parameters** (the ten
+  operational fields of `ConsensusParametersBody`):
+  - *Clocks, envelope validity, and transport attestations*:
+    - `max_clock_drift_ms`: allowable clock skew across nodes and block
+      timestamps; constrained by physical clock synchronization tolerances and
+      network round-trip latency;
+    - `max_envelope_validity_ms`: maximum lifetime of wire protocol message
+      envelopes from `created_at_ms`; constrained by gossip message propagation
+      latency and anti-replay horizon;
+    - `max_transport_attestation_validity_ms`: maximum validity duration of
+      transport key attestations; constrained by transport key compromise
+      exposure window and session rotation frequency;
+    - `max_transport_attestation_future_skew_ms`: forward clock skew tolerance
+      on transport key attestation timestamps; constrained by
+      `max_clock_drift_ms` and peer clock divergence;
+  - *Anti-replay cache capacity*:
+    - `replay_cache_entries_per_peer`: maximum tracked envelope identifiers per
+      connected peer; constrained by peer transmission rate limit and node
+      memory budget;
+    - `replay_cache_entries_global`: maximum total tracked envelope identifiers
+      across all peers; constrained by global network gossip volume and node
+      memory footprint;
+  - *Trust anchor freshness and state queries*:
+    - `max_weak_subjectivity_age_ms`: maximum age of a weak subjectivity
+      checkpoint accepted by syncing nodes; constrained relationally by
+      `min_revocation_effective_delay_blocks` (MUST) and bounded by the release
+      distribution channel;
+    - `max_current_balance_age_ms`: maximum allowed staleness for served
+      balance query state proofs; constrained by query turnaround latency and
+      state-history retention window;
+  - *Lifecycle delays and revocations*:
+    - `app_suspension_notice_epochs`: notice period in epochs between proposing
+      app suspension and enforcement; constrained by epoch duration and
+      operator dispute remediation window;
+    - `min_revocation_effective_delay_blocks`: minimum delay between proposing
+      an identity revocation and its effective height on validator set
+      transitions; constrained relationally with `max_weak_subjectivity_age_ms`
+      and validator succession coordination margin.
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
$ cargo test --workspace --all-features
   Compiling coblox-core v0.1.0 (E:\Git\CobloxNetwork\core\coblox-core)
     Running unittests src\lib.rs (target\debug\deps\coblox_core-048c534e667c80c4.exe)
test result: ok. 35 passed; 0 failed; 0 ignored (coblox-core unit tests)
     Running tests\canonical_serialization.rs
test result: ok. 61 passed; 0 failed; 0 ignored
     Running tests\constraint_block.rs
test result: ok. 19 passed; 0 failed; 0 ignored
     Running tests\election_degenerate.rs
test result: ok. 12 passed; 0 failed; 0 ignored
     Running tests\genesis_derivation.rs
test result: ok. 9 passed; 0 failed; 0 ignored
     Running tests\light_client_perimeter.rs
test result: ok. 14 passed; 0 failed; 0 ignored
     Running tests\preimage_context.rs
test result: ok. 5 passed; 0 failed; 0 ignored
     Running tests\sparse_account_state.rs
test result: ok. 8 passed; 0 failed; 0 ignored
     Running tests\speccheck_conformance.rs
test result: ok. 11 passed; 0 failed; 0 ignored
     Running tests\worked_example.rs
test result: ok. 6 passed; 0 failed; 0 ignored
     Running unittests src\lib.rs (coblox_ffi)
test result: ok. 1 passed; 0 failed; 0 ignored
Total: 181 passed; 0 failed; 0 ignored
```

### Remediation evidence (REVIEW-037)

- **RF-001 (process, medium)**: Corretto il conteggio in trascrizione da 85 a 181 test passati. La precedente trascrizione riportava il subtotale dei soli test di integrazione visibili dopo troncamento shell (85 test), omettendo i 35 unit test di `coblox_core` e i 61 test di `canonical_serialization.rs`. La tabella completa è ora documentata riga per riga (35 unit + 145 integration + 1 ffi = 181 test passati, 0 falliti).
- **RF-002 (documentation, medium)**: Rimossa tutta la notazione matematica LaTeX (`$D_{\max}$`, `$S_{\max}$`, `$F$`) da `docs/protocol/README.md`. Il simbolo non definito `$F$` è stato sostituito con la dizione in lingua naturale `validator succession coordination margin`.
- **RF-003 (documentation, low)**: Applicato il vincolo di larghezza a 80 colonne all'intera sezione DRAFT in `docs/protocol/README.md`. Nessuna riga supera gli 80 caratteri, eliminando tutte le 14 righe sovradimensionate.
- **RF-004 (process, low)**: Cablata l'esecuzione della nuova gate `sim/tools/consensus_parameters_closure.py` e della sua prova in negativo `--negative` nel workflow GitHub Actions `.github/workflows/ci.yml` (job `protocol-docs`). In questo modo la chiusura dello schema `ConsensusParametersBody` viene verificata automaticamente a ogni commit e pull request (inclusi i futuri ampliamenti di parametri in SPEC-022).

### Remediation evidence (REVIEW-041, quarta passata — AGENT-002)

**Perimetro.** Un solo file modificato: `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md`. Nessun documento di protocollo, nessun ADR, nessuna altra spec, nessun debito, nessun file di codice, nessun `STATUS.md`. `git status --porcelain` a fine passata riporta esattamente una riga.

**Regola meccanica applicata, ed è la richiesta esplicita di [REVIEW-041] punto 2 dei follow-up.** Ogni rilievo è stato chiuso in **tre luoghi** e la chiusura è verificata in tutti e tre: **(1)** il corpo della sezione del §2, **(2)** la cella della tabella tassonomica del §3, **(3)** il riquadro di correzione della sezione. Le tre righe che nella terza passata erano state corrette nella sola tabella (2, 5–6, 7) sono state riportate in accordo con il proprio corpo, e la tabella porta ora un'avvertenza esplicita: non è autosufficiente, e dove tabella e sezione divergessero **vale la sezione**.

**Regola sulle ancore applicata prima di scrivere.** Per ogni grandezza a cui l'analisi ancora un vincolo è stato verificato **chi la scrive**. L'esito ha cambiato tre conclusioni: la mediana degli undici timestamp è scritta dai validatori e non è un'ancora (riga 1); nessuna costante di genesi conta le identità enrollate, quindi la relazione delle cache non può ancorarsi ad alcuna grandezza esistente (righe 5–6); `block_interval_ms` è dichiarato e non imposto, e sul lato veloce della banda di cadenza il predicato relazionale della riga 7 ammette fino al doppio della finestra reale.

#### Rilievi di REVIEW-041, uno per uno

- **RF-001 (security, high) — chiuso.** *Corpo §1 punto 2:* tre paragrafi nuovi. L'estremo basso della finestra è dichiarato **mediana di undici `timestamp_ms` scritti dai validatori**, con la regola di `ledger.md` §*"Block format"* citata a frase; è dichiarato che v0 **non impone** l'intervallo obiettivo né la distanza fra timestamp consecutivi, con le due frasi che [REVIEW-041] chiedeva e che l'artefatto non portava; è dichiarato che un predicato su quella distanza è **rifiutato e non assente**, con la frase generale di `README.md` §*"Genesis constants"*. Scenario aritmetico completo: rallentamento a `max_ms_per_block` = 20 000 ms, **dentro la banda quindi senza allarme**, mediana a ~120 000 ms, finestra quadruplicata; lato veloce `min_ms_per_block` = 2 500 ms → 15 000 ms. Conclusione dichiarata su tutta la banda. *Corpo §1 punto 3:* aggiunta la distinzione di specie — la banda di cadenza è una **misura**, non un predicato. *Corpo §1 punto 4:* il tetto è dichiarato mitigazione di grado con il **residuo nominato**. *Tabella riga 1:* la cella non attribuisce più il termine dominante a `block_interval_ms`; nomina la mediana, i validatori che la scrivono, il non-enforcement e i due estremi numerici. *Riquadro §1:* la storia del difetto.
- **RF-002 (security, high) — chiuso.** *Corpo §5 punto 1:* la definizione dice ora **per singolo `sender_node_id`**. *Corpo §5 punto 4:* riscritto in quattro punti. È dichiarato l'errore di categoria con le due frasi di `wire.md` §*"Transport rotation, attribution, and rate limits"* punti 1 e 2 — la cache indossa `(sender_node_id, nonce)` e l'attribuzione non si lega mai al Peer ID di trasporto, quindi il «peer» è un'**identità enrollata** e non un seggio di validatore. È dichiarato che `N_min` **non esiste** in alcun documento né in `core/`, che `validator_min_set_size` è **campo firmato dal quorum** (`README.md` §*"Signed protocol documents"*) e che la costante di genesi è solo il suo pavimento `validator_min_set_size_min` in `ElectionBounds`, con lo scenario di crollo del denominatore. È dichiarato che `README.md` §*"Genesis constants"* contiene **una sola** costante e che nessun documento firmato porta un numero di peer. La relazione è riscritta su **due soli operandi, entrambi campi dello stesso documento**, con il divisore $k$ **dichiarato non deciso** e le due sole forme ammissibili (costante di genesi nuova, o letterale d'ADR) con il loro costo — e la nomina esplicita di ciò che **non** è ammissibile. *Corpo §6 punti 4 e 5:* allineati, con rinvio al §5 e senza duplicare la relazione. *Tabella righe 5 e 6:* `N_min` sostituito, la colonna «ambito» porta ora `sender_node_id` con le due esclusioni, la colonna del vincolo porta $k$ e il fatto che nessuna grandezza esistente serve. *Riquadro §5:* copre entrambe le righe; §6 rinvia a esso.
- **RF-003 (correctness, medium) — chiuso.** *Corpo §1 punto 4:* l'argomento è riscritto come **aritmetico** — a tetto zero la finestra è già dentro la banda, e il termine di questo parametro è additivo su uno che lo domina — e la **regola generale è ritirata esplicitamente**, con la constatazione che pavimento e tetto di questa riga **non** sono in conflitto fra loro. Nessun'altra riga dell'artefatto invoca quella regola: verificato per assenza della stringa.
- **RF-004 (correctness, medium) — chiuso nell'analisi; la parte su DEBT-038 è riportata e non applicata.** *Corpo §1 punto 4:* le tre vie di chiusura sono ora enumerate con il proprio stato. L'aggregazione su `K` blocchi è dichiarata **presa**, con la frase della sezione della regola di elezione citata per intero e `K = election_entropy_blocks`; è dichiarato che la §*"Challenge evidence"* la elenca **ancora** come non presa, quindi che **il documento di protocollo si contraddice** — rilievo sul protocollo, **non corretto qui** perché `docs/protocol/` è fuori perimetro, e riportato al Lead. La quantizzazione allo slot è dichiarata **la sola via residua**. La nota su [DEBT-038] dichiara che la sua portata **va dimezzata**; la correzione del debito è del Lead e non è stata applicata.
- **RF-005 (documentation, medium) — chiuso.** *Corpo §2 punto 2:* il danno massimo porta ora il **termine di prodotto** (ritenzione × tasso d'inserimento contro le due cache) e le due frasi di `wire.md` sulla ritenzione fino a scadenza e sulla non-evizione; «saturazione irreversibile» è scomparso dal corpo. *Corpo §2 punto 4:* le due proprietà sono separate, e la classificazione è **magnitudine per la prima, relazionale con le due cache per la seconda**, con la nota che i tre operandi stanno nello stesso documento quindi il predicato è valutabile in un solo punto. *Tabella riga 2:* già corretta, ora in accordo col corpo. *Riquadro §2:* aggiunto.
- **RF-006 (correctness, medium) — chiuso.** *Corpo §7 punto 2:* i due danni sono separati come **(a)** e **(b)**, con le frasi di fonte per ciascuno; è dichiarato esplicitamente che **(b) non è conseguenza di (a)** e che il nesso causale precedente era un non-sequitur; la frase incriminata è stata rimossa. *Corpo §7 punto 4:* due rimedi distinti, e per **(b)** è nominato il modello del protocollo (obbligo di ripubblicazione, sul modello di quello già imposto a ogni revoca). *Tabella riga 7:* la colonna del vincolo porta ora **entrambi** i rimedi, quindi chi compila l'ADR riga per riga ha qualcosa da scrivere anche per (b). *Riquadro §7:* aggiunto, con l'avvertenza che i due danni vanno letti separatamente in tutta la sezione.
- **RF-007 (documentation, low) — chiuso.** Tutti i riferimenti di review sono stati spostati nei **riquadri di correzione**; nessun rilievo è più citato in linea nella prosa dei punti 1–5 (righe 1, 2, 4, 5, 8, 9, 10 riscritte a questo scopo). **Tutte le attribuzioni di verifica personale sono state rimosse**: `grep -c "Verificato dal Lead"` restituisce `0`, e le formule «verificato in tre punti dell'albero» e «come nota RF-007» sono scomparse. La didascalia della tabella non nomina più le review.

#### Residui delle passate precedenti

- **[REVIEW-038] RF-003 — non chiuso, e correttamente:** è [DEBT-037], dichiarato non rimediabile dentro questa spec dalla reviewer stessa.
- **[REVIEW-038] RF-009 — non mio:** appartiene alla passata di [ADR-012] su [SPEC-022]. La prosa dell'analisi non conta più i campi (correzione già in albero dalla seconda passata) e lo strumento stampa ora `22`, verificato eseguendolo.
- **[REVIEW-038] RF-001, RF-002, RF-004, RF-005, RF-006, RF-007, RF-008, RF-010, RF-011** e **[REVIEW-040] NF-01…NF-10:** disposti dalle review successive; i punti riaperti da [REVIEW-041] sono quelli trattati sopra. In più, **la riga 9 è stata allineata al proprio corpo di propria iniziativa**: la cella scioglieva l'«oppure» di NF-04 mentre il §9 punto 4 lo portava ancora. Non era un rilievo di [REVIEW-041] — che giudica la riga 9 utilizzabile — ma è la stessa forma di difetto che RF-005 e RF-006 censurano, quindi è stata chiusa nello stesso giro.

#### Difetti trovati da questa passata e non corretti qui

1. **`docs/protocol/ledger.md` si contraddice sull'aggregazione su `K` blocchi.** La §*"Challenge evidence"* la elenca fra le riduzioni *"available and are not taken in v0"*; la sezione della regola di elezione dichiara *"the reduction this document deferred to 'the dedicated randomness beacon' and takes here"*. È **famiglia 2** — un'affermazione rimasta indietro rispetto alla regola — in un documento di protocollo. Fuori dal perimetro di scrittura di questa spec. **Aperto come rilievo al Lead.**
2. **Sotto-affermazione inesatta in [REVIEW-041] RF-002.** La review scrive che la cache è indicizzata *"per peer wire, non per validatore"*. La seconda metà è esatta e portante; **la prima no**: `wire.md` §*"Transport rotation, attribution, and rate limits"* punto 1 dichiara che l'attribuzione si lega a `sender_node_id` e **mai** al Peer ID di trasporto. L'indice è **l'identità enrollata**. Il rilievo **regge e si rafforza** — il denominatore corretto non è né il set di validatori né il numero di connessioni, ed è una grandezza che il protocollo non nomina affatto — ma l'artefatto è stato scritto sul fatto verificato, non sulla formulazione della review.
3. **L'incoerenza `block_interval_ms` / `block_interval_seconds`** fra la tabella delle costanti di genesi di `README.md` e l'uso in `ledger.md` e nella §*"Cadence band"*. Preesistente e già registrata da [REVIEW-041]; non toccata.

### Verification transcript (quarta passata)

#### 1. Risolutore di citazioni: ogni frase citata dall'analisi contro le proprie fonti

Verifica della **presenza letterale** di ogni frase fra virgolette dell'analisi in uno dei cinque documenti sorgente, con normalizzazione degli spazi bianchi e spogliatura dell'enfasi Markdown. Perimetro dichiarato: **presenza**, non fedeltà di significato.

```text
$ python /tmp/chk2.py
quoted phrases found: 132
unresolved: 0
```

Una prima esecuzione ne riportava **una** non risolta, ed era un difetto reale introdotto da questa passata: le virgolette interne della citazione sull'aggregazione erano state scritte con `\"`, e le barre rovesciate rompevano la frase. Corretto e rieseguito.

Verifica mirata, prima di scrivere, delle **22 frasi e valori portanti** su cui poggiano le correzioni di RF-001, RF-002, RF-004 e RF-006:

```text
$ python /tmp/chk.py
OK   docs/protocol/ledger.md :: The target block interval is 5 seconds, and v0 does not enforce it.
OK   docs/protocol/ledger.md :: No v0 validity rule constrains the distance between consecutive `timestamp_ms` values.
OK   docs/protocol/README.md :: a rule on the distance between consecutive `timestamp_ms` values is **rejected** rather than merely absent
OK   docs/protocol/README.md :: `block_interval_seconds = 5` is declared, not enforced.
OK   docs/protocol/ledger.md :: MUST be greater than the median of the previous 11 finalized blocks and no more than the active maximum clock drift after the proposal is received
OK   docs/protocol/ledger.md :: Aggregating `election_entropy_blocks` consecutive blocks raises the cost of controlling the *whole* window to holding consecutive proposal slots
OK   docs/protocol/ledger.md :: the reduction this document deferred to "the dedicated randomness beacon" and takes here
OK   docs/protocol/ledger.md :: Two reductions are available and are not taken in v0: quantizing `timestamp_ms` to the consensus slot
OK   docs/protocol/ledger.md :: with the dedicated randomness beacon, which is M-02 work under
OK   docs/protocol/ledger.md :: the mitigation of this grinding
OK   docs/protocol/wire.md :: They cache message IDs and `(sender_node_id, nonce)` until expiry
OK   docs/protocol/wire.md :: The replay cache indexes `(sender_node_id, nonce)` pairs up to `replay_cache_entries_per_peer`
OK   docs/protocol/wire.md :: `sender_node_id`, never to ephemeral transport Peer IDs
OK   docs/protocol/wire.md :: an insertion that would exceed either cap rejects the new envelope as `rate_limited` and MUST NOT evict a still-live entry
OK   docs/protocol/README.md :: the value a client uses at step 1 of the light-client algorithm is **the one in the signed checkpoint**, never one learned from a peer
OK   docs/protocol/README.md :: Once the client has an authenticated header it MUST check that the two agree and fail closed if they do not
OK   docs/protocol/ledger.md :: A network MUST publish a fresh checkpoint on any validator revocation rather than waiting for its ordinary release cadence
OK   docs/protocol/ledger.md :: its exposure window is at most `max_weak_subjectivity_age_ms` and it then fails closed
OK   docs/protocol/README.md :: "validator_min_set_size_min":u64-string
OK   docs/protocol/README.md :: "validator_min_set_size":u64-string
OK   docs/protocol/README.md :: | `min_ms_per_block` | `2500` |
OK   docs/protocol/README.md :: | `max_ms_per_block` | `20000` |

failures: 0
```

#### 2. Enumerazione di `N_min`: il simbolo non esiste

```text
$ grep -rn "N_min" docs/protocol/ core/
(nessun risultato)
```

Zero occorrenze in tutti i documenti di protocollo e in tutto `core/`. Il candidato più vicino ha invece **due** occorrenze con **due ruoli distinti**:

```text
$ grep -n "validator_min_set_size" docs/protocol/README.md
826:  "validator_min_set_size":u64-string          <- ConsensusParametersBody: scritto dal quorum
1043:  "validator_min_set_size_min":u64-string     <- ElectionBounds: costante di genesi, solo pavimento
```

#### 3. Enumerazione completa delle costanti di genesi

`docs/protocol/README.md`, §*"Genesis constants"*, tabella letta per intero: **una sola riga**, `block_interval_seconds` = `5`, non governata. Nessuna costante conta peer, connessioni o identità enrollate. È il fatto su cui poggia la conclusione di RF-002 che la relazione delle cache **non ha oggi alcuna ancora disponibile**.

#### 4. Assenza dei termini ritirati

```text
$ grep -c "Verificato dal Lead"                    -> 0
$ grep -c "in combinazione col fail-closed"        -> 0
$ grep -c "Un vincolo i cui due lati"              -> 0
$ grep -c "saturazione irreversibile"              -> 1  (solo dentro il riquadro §2, come storia del difetto)
$ grep -c "N_min"                                  -> 3  (solo riquadro §5, corpo §5 come negazione, cella 5 come negazione)
$ grep -c "N_peers"                                -> 2  (solo riquadro §5 e cella 5, come negazioni)
```

Le occorrenze residue sono state ispezionate una per una: nessuna afferma il termine ritirato, tutte lo negano o ne registrano la storia.

#### 5. Gate di progetto rieseguite dopo la modifica

```text
$ python sim/tools/consensus_parameters_closure.py
PASS: all 22 ConsensusParametersBody fields are covered by constraint block or DRAFT list.

$ python sim/tools/consensus_parameters_closure.py --negative
ok   unmutated copy passes
ok   C1-SCHEMA-NOT-COVERED caught schema field missing from both lists
ok   C2-ORPHAN-PARAM caught orphan parameter in DRAFT list
Negative proof: PASS - all defect classes observed failing.

$ python sim/tools/published_artifacts.py
published-artifact inventory: PASS
```

#### 6. Perimetro del diff

```text
$ git status --porcelain
 M .lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md
```

Nessuna suite Rust rieseguita: **nessun file di codice è stato toccato**, e la trascrizione della terza passata resta la prova valida per `cargo`. È una scelta dichiarata, non un'omissione.

### Limiti noti di questa passata

1. **Il divisore $k$ della relazione delle cache non è deciso.** L'analisi enumera le due sole forme ammissibili con il loro costo e dichiara che la scelta è dell'operatore. **Non è una lacuna: è il perimetro della spec** — «prepara la decisione e non la prende». Ma significa che le righe 5 e 6 non sono utilizzabili verbatim in un ADR senza quella scelta.
2. **Nessuno scenario è stato eseguito.** Il rallentamento a 20 000 ms per blocco, l'arretramento della mediana e il DoS incrociato sulla cache sono **derivati da regole lette e da aritmetica su costanti dichiarate**, mai osservati su una catena in esecuzione. Non esiste in questo repository un simulatore di consenso in grado di produrli.
3. **La fedeltà di significato delle 132 citazioni non è verificata meccanicamente.** Il risolutore prova che ogni frase **esiste** dove l'analisi dice; non prova che dica ciò che l'analisi le fa dire. L'intorno è stato letto per le circa venticinque citazioni su cui poggia una correzione di questa passata; sulle altre la verifica è di presenza.
4. **`threat-model.md` non è stato letto**, e TM-37 (riga 3) non è stato riverificato in questa passata.
5. **[ADR-010], [ADR-013], [ADR-015], [ADR-016], [ADR-017] non sono stati riletti per intero.** Le conseguenze citate sono state verificate sui documenti di protocollo e su `params.rs`.
6. **Se `election_entropy_blocks` sia implementato in codice non è stato verificato.** La conclusione di RF-004 poggia sul solo documento, come già dichiarato da [REVIEW-041].
7. **Le righe 3 e 8 non sono state riesaminate nel merito.** [REVIEW-041] le giudica utilizzabili così; su di esse questa passata ha toccato soltanto la forma (riquadro della riga 8, etichetta di deduzione nella cella).
8. **Il residuo di ancoraggio della riga 7 è stato trovato da questa passata e non da una review**, quindi non ha avuto verifica indipendente: `block_interval_ms` è dichiarato e non imposto, e sul lato veloce della banda di cadenza il predicato relazionale ammette fino al doppio della finestra reale di successione. È scritto come residuo di grado con il fattore limitato dalla banda. **Va attaccato nella prossima passata**, insieme al suo riflesso sulla riga 10.
