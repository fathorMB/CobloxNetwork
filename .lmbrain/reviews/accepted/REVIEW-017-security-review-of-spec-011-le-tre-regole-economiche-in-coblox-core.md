---
id: REVIEW-017
# Note: Quote the title if it contains a colon
title: "Security review of SPEC-011: le tre regole economiche in coblox-core"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-011
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, test-quality, documentation, requirements-completeness]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-017-EVENT-001"
    timestamp: "2026-08-25T20:21:16.857884+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-017-EVENT-002"
    timestamp: "2026-08-25T20:26:23.517304100+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Tre finding medium, nessuno critical o high. Le tre regole nate dai finding critici di REVIEW-014 sono implementate e nella direzione giusta, e le due superfici che il Lead segnalava come probabili sono solide: per la terza volta consecutiva i difetti erano altrove.\n\nIl Lead ha riprodotto in modo indipendente i due finding che pesano. RF-001: RewardBounds::validate non e chiamata da alcuna riga di src/. Verificato che l'unico chiamante di bounds.validate e authenticate_consensus_parameters a light_client.rs:117 per ElectionBounds, e che RewardBounds compare in src/ solo in params.rs e in un commento di lib.rs. Non esiste alcun authenticate_reward_policy che componga la validazione, quindi con reward_parameter_change_denominator uguale a zero le due disuguaglianze del rapporto diventano vere per qualunque coppia e il limite di variazione e vacuo in silenzio.\n\nRF-002 e il piu importante e riprodotto dal Lead alla lettera. Disattivata la sola meta inferiore del rapporto di variazione sulla reward policy, lasciando new_bounded e togliendo old_bounded, l'intera suite Rust resta verde a 109 test e l'oracolo Python riporta 34 casi, 0 mismatch, GATE-RULES-REJECT PASS. La meta mancante e quella verso il basso, cioe la direzione che README dichiara pericolosa per le due tariffe di lavoro e per la soglia di eleggibilita. GATE-DIRECTION e quindi soddisfatta a meta su questa regola, e GATE-TWO-ORACLES non copre il buco perche i due oracoli sono indipendenti nell'implementazione ma non nella derivazione dei casi. params.rs e stato ripristinato e verificato identico, 109 test verdi e fmt pulito.\n\nRF-003 verificato: sim/coblox_sim/__main__.py righe 317 e 321 affermano ancora al presente che F e un valore senza tetto ne pavimento e che la reward policy non ha alcun limite di variazione. Quattro affermazioni false in un artefatto versionato ed eseguibile, che la passata di ADR-012 non ha raggiunto.\n\nQuesta richiesta di modifiche non contraddice REVIEW-016 e la completa: quella review verificava la conformita ai criteri della spec e li trovava soddisfatti, ma la sua evidenza sul vincolare della suite era una sola mutazione generalizzata a tutte le regole, mentre AGENT-007 le ha mutate tutte e diciannove. E la stessa forma di difetto che il progetto censisce, un'affermazione piu forte dell'evidenza che la sostiene, ed e del Lead."
    evidence_refs: ["SPEC-011", "REVIEW-016", "REVIEW-014", "ADR-010", "ADR-012"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-017-EVENT-003"
    timestamp: "2026-08-25T20:52:42.095833200+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Remediation dei tre finding medium, piu un'estensione richiesta dal Lead. RF-001 chiuso con light_client::authenticate_reward_policy, gemello di authenticate_consensus_parameters, piu un secondo presidio dentro check_against_active che rifiuta rapporto degenere e gap nullo su ogni percorso e non solo sull'entry point; publisher_reward_within_cap spostato su ValidatedRewardPolicy, che ora ha un consumatore, con il cambiamento breaking dichiarato. RF-002 chiuso senza prendere casi dalla tabella: i due oracoli derivano i casi in modi diversi e nessuno legge la tabella, il Rust calcola i due estremi in forma chiusa su tutti e 12 i parametri e il Python cerca il punto di rottura per ricerca binaria, con bound larghi su ogni magnitudine perche un rifiuto non possa venire da un pavimento. RF-003 chiuso, e l'implementatore ha trovato e corretto una quinta affermazione falsa non elencata nel finding.\n\nEstensione richiesta dal Lead dopo una propria verifica: la stessa lacuna di RF-002 esisteva sul lato elezione, dove disattivando old_bounded sul loop ELECTION_PARAMETERS la suite restava interamente verde. Chiusa con un test gemello, senza toccare codice di produzione perche check_against_active era gia corretta in entrambe le direzioni e mancava solo chi la vincolasse."
    evidence_refs: ["SPEC-011"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-017-EVENT-004"
    timestamp: "2026-08-25T20:52:58.181309300+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificato dal Lead rieseguendo, non letto dall'evidenza. Albero integro: 113 test workspace passati, clippy zero warning con -D warnings, fmt pulito, reward_rules.py 58 casi 0 mismatch, protocol_hashes tutti MATCH, published_artifacts e la prova in negativo PASS, nessun hash pubblicato mosso.\n\nLe due guardie nuove provate in negativo dal Lead in modo indipendente. Mutazione sul loop REWARD_PARAMETERS, lasciando new_bounded e togliendo old_bounded: the_rate_of_change_binds_in_both_directions_on_every_parameter FAILED. Mutazione sul loop ELECTION_PARAMETERS: the_election_rate_of_change_binds_downward_on_every_parameter FAILED, con il primo parametro accettato a 7999. Ripristinato entrambe le volte e verificato che params.rs contiene due occorrenze integre della condizione e zero residui di mutazione, con la suite di nuovo a 113 e fmt pulito.\n\nTre informazioni riportate dall'implementatore invece di essere forzate, e sono la parte piu utile della consegna. La trappola del rifiuto anticipato esiste anche sul lato elezione ed e peggiore che sul lato reward, perche arriva dal blocco relazionale che validate esegue per primo invece che da un pavimento di magnitudine, e il test la mette in evidenza mostrando la stessa discesa rifiutata da una regola diversa su una base sbagliata. validator_min_set_size e validator_target_set_size non sono spazzabili in entrambe le direzioni dalla stessa base perche 3 min_set maggiore uguale 2V li accoppia e i due intervalli non si intersecano, quindi servono due basi. validator_max_consecutive_terms non ha alcun documento discendente lecito, e quale regola lo rifiuti dipende da quanto si muove, con entrambi i casi asseriti.\n\nSegnalazione da portare oltre questa review: l'oracolo Python non copre affatto il rapporto di variazione sul lato elezione, quindi per quel gemello non esiste un secondo oracolo e GATE-TWO-ORACLES vi si applica a vuoto."
    evidence_refs: ["SPEC-011", "REVIEW-017"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-017-EVENT-005"
    timestamp: "2026-08-25T20:53:21.548776600+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata sulla condizione che AGENT-007 aveva dichiarato in anticipo nel proprio verdetto: chiusi RF-001, RF-002 e RF-003, GATE-SECREVIEW e soddisfatta senza riserve per quanto la riguarda. Tutti e tre sono chiusi e il Lead li ha verificati in modo indipendente riproducendo le mutazioni e osservando le guardie nuove fallire, poi ripristinando l'albero e verificandone l'integrita.\n\nIl Lead dichiara la base su cui accetta invece di lasciarla intendere: non e una seconda lettura di AGENT-007 sul lavoro di remediation, e la condizione che aveva posto. L'unica parte che lei non ha visto e il test gemello sul lato elezione, che il Lead ha richiesto dopo una propria verifica; e additivo, non tocca alcuna regola, e non cambia il codice di produzione perche check_against_active era gia corretta in entrambe le direzioni.\n\nNessun finding resta aperto. OSS-003, che l'implementatore ha segnalato invece di chiudere, e registrato come debito separato: check_internal e check_magnitudes sono pub mentre i gemelli del lato consenso sono privati, quindi un chiamante puo invocare un sotto-controllo credendo di aver validato. E un secondo cambiamento breaking e non e condizione di chiusura di questa review, come l'implementatore ha correttamente argomentato."
    evidence_refs: ["SPEC-011", "REVIEW-016", "ADR-010", "ADR-012"]
    implementation_agent: "AGENT-001"
links: [SPEC-011, REVIEW-014, REVIEW-016, ADR-010, ADR-011, ADR-012]
created: 2026-08-25
updated: 2026-08-25
tags: [security, review, sybil, ledger]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "recorded review remediation-verification"
  - date: 2026-08-25
    action: "transitioned changes-requested -> accepted"
---
# Review

## Outcome

**Changes requested**, con tre finding di severità media e nessun finding critico o alto.

Le tre regole che nascono dai finding critici di [REVIEW-014] sono implementate, e sono implementate
**nella direzione giusta**. Ho verificato la direzione di ciascuno degli undici limiti di magnitudine
contro il testo pubblicato, incluso i tre che vanno al contrario dell'intuizione
(`validator_eligibility_window_epochs` è un tetto, i due divisori di contribuzione sono tetti, le due
tariffe di lavoro sono pavimenti): tutti corretti. Ho poi verificato per mutazione — non per lettura —
che ciascuna regola è davvero vincolata dalla suite. Le due superfici che il Lead segnalava come
probabili sono solide, per la terza volta in questo progetto.

I difetti sono altrove, e hanno tutti e tre **la stessa forma della cucitura** che questa spec esiste
per chiudere: una regola implementata bene, e il punto in cui va composta con il resto lasciato scoperto.

- **RF-001** — la regola è giusta ma non è mai ricucita: `RewardBounds::validate` non è chiamata da
  nessuna riga di `src/`. Un oggetto bound degenere disattiva in silenzio il limite di variazione.
- **RF-002** — la regola è giusta ma la metà verso il basso non è vincolata da **nessuno dei due**
  oracoli. `GATE-DIRECTION` è soddisfatta a metà sulla regola dove la direzione conta di più.
- **RF-003** — un artefatto versionato ed eseguibile continua ad affermare al presente che queste
  difese non esistono.

Nessuno dei tre richiede una riprogettazione. RF-002 è un test, RF-003 un paragrafo, RF-001 una
composizione di poche righe o un punto d'ingresso.

## Acceptance-criteria compliance

Undici criteri su undici sono dichiarati soddisfatti. Ne contesto uno e ne qualifico due.

| Criterio | Esito | Nota |
| --- | --- | --- |
| `RewardBounds` esiste con la disciplina di `ElectionBounds` | **soddisfatto con riserva** | Il tipo, i campi e la separazione bound/documento sono corretti. Ciò che `ElectionBounds` ha e `RewardBounds` no è il **punto d'ingresso che lo valida** (RF-001). |
| `availability_microtokens_per_unit` rifiutata se positiva | soddisfatto | Verificato per mutazione: neutralizzata la regola, due test falliscono nominandola. |
| `3 * validator_min_set_size >= 2 * V` | soddisfatto | Verificato per mutazione due volte: rimossa (3 test falliscono) e invertita in `<=` (4 test falliscono). |
| Magnitudine, rapporto di variazione, gap di attivazione | soddisfatto | Undici limiti su undici catturati singolarmente per mutazione. |
| Ogni riga delle tabelle pubblicate ha un caso col suo verdetto | **soddisfatto con riserva** | Le righe ci sono tutte, ma una riga rifiuta per il motivo sbagliato in entrambe le implementazioni (RF-002). |
| Per ogni limite nuovo, il rifiuto nella direzione giusta | **NON soddisfatto** | La metà verso il basso del rapporto di variazione non ha alcun caso. Vedi RF-002. |
| L'aritmetica verificata e l'overflow che rifiuta | soddisfatto in sostanza | Vedi OSS-001: il progetto è corretto, la dimostrazione copre solo l'addizione. |
| I due oracoli concordano su ogni caso pubblicato | soddisfatto, ma vedi RF-002 | Concordano — anche dove sono ciechi entrambi. |
| `recommended.py` non porta più il fondo maturo | soddisfatto | `300_000_000` con la ragione accanto, coerente con la base di `reward_rules.py`. |
| Nessun hash pubblicato mosso | soddisfatto | Riverificato dal Lead in [REVIEW-016]; non l'ho ripetuto. |
| Gate di [ADR-012] eseguita | soddisfatto nella lettera, **incompleta nel perimetro** | La passata non ha guardato `sim/coblox_sim/__main__.py`. Vedi RF-003 e RF-004. |

## Code observations

**La disciplina di `ElectionBounds` è riprodotta fedelmente nella forma.** `RewardBounds` è
configurazione, i limiti sono presi dai bound e mai dal documento in valutazione, `check_magnitudes`
non legge nulla dal documento se non la grandezza confrontata, e `ValidatedRewardPolicy` non ha
costruttori pubblici oltre `validate`. Su questo non ho nulla.

**Le direzioni.** Ho confrontato riga per riga `check_magnitudes`
(`core/coblox-core/src/params.rs:721-806`) con `docs/protocol/README.md` §*Reward bounds*. Tutti e
undici i confronti puntano nel verso che il documento motiva, compresi i tre che il documento stesso
segnala come contro-intuitivi. `check_internal` applica `kn < kd` **strettamente**, che è la proprietà
di perdita strutturale, non `<=`.

**Il rapporto di variazione copre tutte e tredici le grandezze**, come il documento richiede
esplicitamente ("every quantity rather than the bounded ones"). `REWARD_PARAMETERS` enumera i tredici
campi di `RewardPolicy` senza omissioni: l'ho verificato per confronto diretto con la struttura.

**L'asimmetria fra il lato consenso e il lato reward è il filo che lega i finding.** Sul lato consenso
il crate spedisce `light_client::authenticate_consensus_parameters`, che compone in un solo punto il
legame all'header, il controllo di `chain_id`, `bounds.validate(chain_id)` e
`parameters.validate(...)`. Sul lato reward **non esiste nessun equivalente**. Le conseguenze
osservabili sono tre, e sono tutte visibili con un grep:

- `RewardBounds::validate` non compare in nessun file di `src/` fuori dalla propria definizione (RF-001);
- `ValidatedRewardPolicy` non ha **nessun consumatore** in `src/`;
- `publisher_reward_within_cap`, l'unico calcolo di ricompensa del crate, è un metodo su
  `RewardPolicy` **non validata**, mentre i suoi omologhi del lato consenso
  (`election_boundary_height`, `entropy_window`, …) sono metodi su `ValidatedConsensusParameters`.

Il commento di modulo in `core/coblox-core/src/lib.rs:54-57` afferma che «the election derivation and
reward validation accept only `ValidatedConsensusParameters` and `ValidatedRewardPolicy`». Per il lato
elezione è vero. Per il lato reward non è vero di nessuna funzione oggi esistente.

## Tests and verification

Non ho contato test verdi. Ho ricostruito il workspace in una copia isolata
(`core/` + `Cargo.toml`, fuori dall'albero di lavoro del Lead, senza toccare il repository) ed
eseguito una campagna di mutazione: per ogni regola economica, sostituzione della condizione di
rifiuto con `if false` — cioè la regola smette di rifiutare senza smettere di compilare — e
osservazione di quali test cadono. Una regola la cui neutralizzazione lascia la suite verde è una
regola che nessun caso `invalid` sta esercitando, che è esattamente ciò che `GATE-INVALID-REJECTED`
esiste per escludere.

**Diciannove regole di `check_internal` e `check_magnitudes`, esito:**

| Regola neutralizzata | Esito |
| --- | --- |
| `availability_microtokens_per_unit == 0` | catturata (2 test) |
| `publisher_reward_cap_denominator` non nullo | **sopravvissuta** — mutante equivalente, vedi OSS-002 |
| `kn` strettamente `< kd` | catturata |
| divisori storage/compute `> 0` | catturate |
| `validator_eligibility_window_epochs >= 1` | catturata |
| `validator_eligibility_min_issuers >= 2` | catturata |
| `existence_fund <= F_max` | catturata (2 test) |
| `reward_epoch_ms >= min` e `<= max` | catturate (2 test ciascuna) |
| `publisher_reward_cap_numerator <= max` | catturata |
| `publisher_reward_cap_denominator >= min` | catturata |
| `validator_eligibility_threshold_units >= min` | catturata (2 test) |
| `validator_eligibility_window_epochs <= max` | catturata (2 test) |
| `validator_eligibility_min_issuers >= min` | catturata |
| divisori storage/compute `<= max` | catturate (2 test ciascuna) |
| tariffe storage/compute `>= min` | catturate (2 test ciascuna) |

**Regole relazionali e di variazione, esito:**

| Mutante | Esito |
| --- | --- |
| ciclo del rapporto di variazione disattivato | catturato (2 test) |
| gap di attivazione disattivato | catturato (2 test) |
| monotonia di `sequence` disattivata | catturato |
| `3 * min_set >= 2 * V` rimossa | catturata (3 test) |
| `3 * min_set >= 2 * V` invertita in `<=` | catturata (4 test) |
| `validate()` non chiama più `check_internal` | catturato |
| **rapporto di variazione ridotto alla sola metà superiore** | ***SOPRAVVISSUTO*** |
| rapporto di variazione ridotto alla sola metà inferiore | catturato (2 test) |

L'ultimo blocco è RF-002. Ho ripetuto la stessa mutazione sull'oracolo Python
(`sim/tools/reward_rules.py:137`, riducendo la condizione a `new * den <= old * num`) su una copia:
**34 casi su 34, zero disallineamenti, `GATE-RULES-REJECT: PASS`**. I due oracoli sono ciechi nello
stesso punto, perché sono stati scritti dalla stessa tabella.

Ho inoltre scritto una prova mirata per RF-001 (`greta_probe.rs`, nella copia isolata, non versionata):
con un `RewardBounds` il cui `validate()` rifiuta, un documento che moltiplica per 100 000 il fondo di
esistenza in un solo passo **è accettato** da `RewardPolicy::validate`. Il test passa, cioè la
vulnerabilità è riproducibile.

Ciò che **non** ho ripetuto, perché il Lead lo ha già verificato in modo indipendente e non ho motivo
di dubitarne: i 110 test, clippy, fmt, i quattro strumenti Python, gli hash pubblicati.

## Production quality and documentation compliance

Il codice è di qualità di produzione: errori nominati e distinti per regola, nessun panico su input
di documento, nessuna costante di lancio compilata nel crate, commenti che spiegano il *perché* del
verso di ogni limite e non solo il *cosa*. La documentazione del crate è però in due punti più
ambiziosa del codice (`lib.rs:54-57`, vedi RF-001), e un artefatto versionato del simulatore è
apertamente falso (RF-003).

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security-boundary | severity=medium | criterion=`RewardBounds::validate` è chiamata dal percorso di accettazione della reward policy, oppure il crate espone un punto d'ingresso che la compone; un test dimostra che un `RewardBounds` degenere è rifiutato invece di rendere vacuo il limite di variazione | remediation=AGENT-001

**Il limite è sulla grandezza giusta e nel verso giusto, e non è ricucito.**
`RewardBounds::validate` — che rifiuta denominatore nullo, numeratore non superiore al denominatore,
gap nullo, `reward_epoch_ms_min` nullo e `min > max` — **non è invocata da nessuna riga di `src/`**.
Non la chiama `RewardPolicy::validate`, e non esiste un `authenticate_reward_policy` che la componga
come `authenticate_consensus_parameters` compone `ElectionBounds::validate`.

*Scenario d'attacco concreto.* Una distribuzione di rete il cui `RewardBounds` porta
`reward_parameter_change_denominator: 0` — errore di packaging, campo omesso e ricostruito a zero, o
packager ostile — supera ogni controllo che il crate esegue sul percorso reward. Con denominatore
zero le due disuguaglianze `new * 0 <= old * num` e `old * 0 <= new * num` sono entrambe vere per
qualunque coppia di valori: **il rapporto di variazione diventa vacuo in silenzio**. Un quorum
sedente può allora portare `F` direttamente al suo tetto di genesi e le due tariffe di lavoro
direttamente ai loro pavimenti in un unico documento — precisamente ciò che `README.md` dichiara che
questo limite esiste per impedire («prevents a sitting quorum from jumping a parameter to its genesis
ceiling, or its denominator to its floor, in a single step»). Con
`reward_parameter_min_activation_gap_blocks: 0` sparisce anche la spaziatura, e con
`reward_epoch_ms_min: 0` sparisce il pavimento che protegge dall'attacco ×1000 sull'epoca. Nessun
errore, nessuna traccia: `validate` restituisce `Ok`.

Riproduzione (copia isolata del workspace, non versionata):

```rust
let mut bounds = permissive_reward_bounds();
bounds.reward_parameter_change_denominator = 0;
bounds.reward_parameter_change_numerator = 0;
assert!(bounds.validate(&zero_chain_id()).is_err());  // l'oggetto è invalido di per sé

let jump = RewardPolicy { existence_fund_microtokens_per_epoch: base_fund * 100_000, ..base };
assert!(jump.validate(&bounds, 1_000_000, 2, Some(&active)).is_ok());  // e passa
```

```text
test a_degenerate_rewardbounds_silently_disables_the_rate_limit ... ok
```

Lo stesso vale in astratto per `ElectionBounds`, ma lì il crate spedisce il punto d'ingresso che
chiude il buco. Il difetto non è la regola: è che il lato reward non ha ancora il suo
`authenticate_*`, e la spec ha giustamente escluso il verificatore Ed25519 senza accorgersi che il
legame `policy_hash` → documento → `validate` è una cosa diversa dalla verifica di firma e non era
escluso da niente.

Nella stessa famiglia, e da chiudere insieme: `publisher_reward_within_cap` è un metodo su
`RewardPolicy` non validata, e `ValidatedRewardPolicy` non ha consumatori. Finché il tetto del creator
share si calcola da una policy che può non essere mai passata da `validate`, il compilatore non sta
proteggendo nulla su questo lato, mentre sul lato consenso lo fa. `lib.rs:54-57` afferma il contrario.

RF-002 | category=test-quality | severity=medium | criterion=Esiste in `coblox-core` **e** in `sim/tools/reward_rules.py` almeno un caso che viola il rapporto di variazione **verso il basso** su una grandezza il cui pericolo è verso il basso, e che non sia già rifiutato da un pavimento di magnitudine; rimuovere `old * den <= new * num` fa cadere un test in entrambe le implementazioni | remediation=AGENT-001

**`GATE-DIRECTION` è soddisfatta a metà sulla regola dove la direzione conta di più.**
Il rapporto di variazione è bidirezionale per costruzione:

```text
x_new * den <= x_old * num   e   x_old * den <= x_new * num
```

La prima metà limita la salita, la seconda la discesa. Ridurre il controllo alla sola prima metà
lascia **verde l'intera suite Rust e l'intero oracolo Python**: nessun caso pubblicato esercita una
discesa oltre il rapporto.

La direzione mancante è quella pericolosa. `README.md` motiva i pavimenti su
`storage_microtokens_per_byte_epoch` e `compute_microtokens_per_million_fuel` dicendo che «the
dangerous direction here is **downward**», e il pavimento su
`validator_eligibility_threshold_units` protegge dall'erosione verso il basso della barriera
d'ingresso. Il rapporto di variazione è, per il documento stesso, «the only residual defence» che
trasforma questi movimenti in un processo osservabile invece che in un salto. Oggi la sua metà
discendente non è vincolata da nulla.

*Perché è successo, e perché i due oracoli non l'hanno intercettato.* L'unica riga della tabella
pubblicata che nomina una discesa oltre il rapporto è
`reward_epoch_ms | 86 400 000 -> 86 400 in one document | invalid | rate of change exceeded by a
factor of 1000`. Quel caso **è rifiutato dal pavimento `reward_epoch_ms_min`, non dal rapporto**: la
trascrizione dell'implementatrice lo mostra letteralmente, riportando come motivo *«epoch below the
floor inflates real issuance»* invece del motivo che la tabella dichiara. Il caso c'è, il verdetto
coincide, e la regola che il caso doveva coprire non viene mai raggiunta. Entrambi gli oracoli sono
stati scritti dalla stessa tabella, quindi concordano — anche nella cecità. È il limite dell'evidenza
a due oracoli quando i due non sono indipendenti nella *derivazione dei casi*, e vale la pena
registrarlo perché `GATE-TWO-ORACLES` tornerà su altre spec.

*Scenario d'attacco concreto.* Un quorum sedente pubblica un `reward_policy` che porta
`storage_microtokens_per_byte_epoch` dal valore corrente a un decimo in un solo documento. `W` crolla,
il rapporto sorvegliato `F / (F + W)` sale verso uno senza che `F` si muova di un microtoken, e il
documento rispetta ogni pavimento di magnitudine perché il pavimento è molto più in basso del valore
corrente. Oggi il codice lo rifiuta — la metà discendente c'è. Domani, dopo un refactor che nessun
test fa cadere, potrebbe non rifiutarlo più, e nessuno lo saprebbe.

Verifica riproducibile: sostituire in `params.rs:825` la condizione con `if !new_bounded {` e in
`sim/tools/reward_rules.py:137` con `if not (new * den <= old * num):`. La suite Rust resta verde;
l'oracolo Python stampa `cases: 34, mismatches: 0` e `GATE-RULES-REJECT: PASS`.

RF-003 | category=documentation | severity=medium | criterion=`sim/coblox_sim/__main__.py` non afferma più che `F` è senza tetto, che la reward policy non ha limite di variazione, che la sua unica regola di validità è `kn < kd`, o che `RewardBounds` è una modifica ancora da decidere | remediation=AGENT-001

**Un artefatto versionato ed eseguibile afferma al presente che queste difese non esistono.**
Il rapporto economico stampato da `sim/coblox_sim/__main__.py` dice ancora, alle righe 317-330 e
388-395:

- «the per-epoch cap F — a value with **no ceiling and no floor**. One lawful document can take F from
  15 882 cr to 2^60 microtokens.» — falso: `existence_fund_microtokens_per_epoch_max` esiste ed è
  applicato.
- «the 5/4 discipline on F above — a **PRACTICE**. […] the reward policy has **none**. Its **only**
  validity rule is `kn < kd`.» — falso su tre affermazioni su tre.
- «Closing this **needs** a RewardBounds object in the genesis trust anchor […] a protocol change,
  **out of this spec's scope, and the Lead's ADR to open**.» — falso: [ADR-010] e [ADR-011] sono
  accettate, [SPEC-009] ha pubblicato l'oggetto e [SPEC-011] lo applica.
- «it is the second thing RewardBounds **would** fix» — condizionale su una cosa ormai fatta.

Questo è precisamente il caso previsto da [ADR-012]: la regola nuova non invalida gli artefatti che la
spec tocca, ma quelli che nessuno sta guardando. Il criterio di accettazione nominava
`.lmbrain/knowledge/economic-simulation-report.md`; il rapporto **eseguibile e versionato** che porta
le stesse affermazioni non è stato nominato da nessuno, e la passata di [ADR-012] non lo ha raggiunto.
Rilevo che la stessa forma condizionale sopravvive anche nel documento del brain (righe 12, 218, 230,
381, 749): quella parte è del Lead e non del riparatore.

*Perché conta e non è cosmesi.* Il rapporto economico è l'artefatto che un revisore esterno legge per
capire se il claim anti-Sybil regge. Un rapporto che dichiara assenti le difese esistenti produce
esattamente lo stesso giudizio di un rapporto onesto su un sistema indifeso, e su questo progetto è la
seconda volta che un artefatto non guardato contraddice quello guardato.

RF-004 | category=requirements-completeness | severity=low | criterion=Esiste un artefatto versionato che propone i valori di lancio di `RewardBounds`, oppure una spec o un debito che ne assegna la proprietà | remediation=AGENT-LEAD

**La domanda del Lead sulle superfici di cucitura, risposta sull'unica che ho trovato.**
`sim/coblox_sim/recommended.py` definisce `RECOMMENDED = ParameterSet(name="coblox-v0-genesis-candidate", …)`
con `consensus`, `reward` e `bounds` — dove `bounds` è un `ElectionBounds`. **Non c'è alcun
`RewardBounds`**, né in `recommended.py` né in `sim/coblox_sim/params.py`, che non definisce affatto
quella classe. L'unico `RewardBounds` di `sim/` è il fixture privato di
`sim/tools/reward_rules.py:25-41`, valori scelti per far girare le tabelle di frontiera e non
proposti come valori di lancio da nessuna parte.

Conseguenza concreta: **nessun artefatto del repository propone un valore per `F_max`**, per i due
pavimenti tariffari o per `validator_eligibility_threshold_units_min`. Il numero più portante
dell'intera difesa anti-Sybil — il tetto sul fondo di esistenza, da cui `D = F · N/(N+H)` dipende
interamente — non ha un candidato, mentre il suo gemello `ElectionBounds` ce l'ha da [SPEC-009].

La forma è identica a quella che [SPEC-011] chiude: [SPEC-009] possedeva i documenti, [SPEC-011] il
crate, e il *candidato di genesi* non è di nessuno dei due. Non è un difetto di questa
implementazione e non chiedo di ripararlo qui: chiedo che il Lead lo assegni, perché altrimenti la
prima distribuzione firmata dovrà inventare quei numeri sotto pressione di rilascio, che è il momento
peggiore per sceglierli.

## Observations (non-blocking)

OSS-001 — **la dimostrazione dell'overflow copre meno di quanto il criterio dichiari.** Ogni
`checked_mul_u128` dei percorsi economici moltiplica due valori allargati da `u64`, e il prodotto di
due `u64` non può eccedere `u128`: quel ramo di errore è **irraggiungibile per costruzione**. La
scelta è corretta — l'allargamento rende l'overflow impossibile invece che soltanto rilevato, che è
la difesa più forte — ma significa che l'unico percorso genuinamente sormontabile è il `checked_add`
del gap di attivazione, ed è l'unico che
`the_arithmetic_overflow_rejection_for_economic_rules` esercita. Il criterio «l'overflow rifiuta
invece di troncare, con un caso che lo dimostra» è quindi soddisfatto nella sostanza e sovradichiarato
nella forma. Suggerisco un commento sul perché il ramo dei prodotti non è raggiungibile, così che un
lettore futuro non lo scambi per copertura mancante e non lo "ripari" restringendo gli intermedi.

OSS-002 — **`publisher_reward_cap_denominator` non nullo è un controllo ridondante.** Neutralizzarlo
non fa cadere nulla, perché con denominatore zero il controllo successivo `kn >= kd` è
necessariamente vero e rifiuta comunque. Il fixture pubblicato `1/0` resta rifiutato. Vale la pena
tenerlo per l'errore nominato, ma non è una regola indipendente e non conta come copertura.

OSS-003 — **asimmetria di visibilità.** `RewardPolicy::check_internal` e `check_magnitudes` sono
`pub`, mentre i gemelli `ConsensusParameters::check_relations` e `check_magnitudes` sono privati. Un
chiamante può invocare un sotto-controllo e credere di aver validato. Osservo anche che
`the_reward_policy_acceptance_rules` verifica i casi invalidi chiamando `check_internal` invece di
`validate`; per fortuna `the_reward_policy_boundary_fixtures_mirroring_reward_rules_py` passa da
`validate` e la mia mutazione "`validate` non chiama più `check_internal`" viene catturata. Se quei
sotto-controlli tornassero privati, l'ambiguità sparirebbe insieme alla tentazione.

## Required follow-up

1. **RF-001**, **RF-002**, **RF-003** sono condizioni di chiusura di questa review e restano in
   `review` finché non sono verificate. Sono tre interventi piccoli e indipendenti.
2. **RF-004** non è remediation di [SPEC-011]: è una spec o un debito da aprire, di competenza del Lead.
3. Riporto a [SPEC-010], come la spec chiedeva: il perimetro della passata di [ADR-012] su `sim/`
   **non ha raggiunto `sim/coblox_sim/__main__.py`**, che è versionato, eseguibile e porta
   affermazioni normative sul protocollo. È un caso concreto a favore di includere `sim/` nel
   perimetro, e una prova che `published_artifacts.py` oggi controlla forme (domini, tag, valori
   rispecchiati) ma non **affermazioni in prosa rese false da una regola nuova** — che è la classe di
   difetto di RF-003 e la più difficile da meccanizzare.
4. Riporto a `GATE-TWO-ORACLES`, per le spec future: due oracoli scritti dalla stessa tabella
   pubblicata sono indipendenti nell'*implementazione* e non nella *derivazione dei casi*. Il loro
   accordo non è evidenza sui punti che la tabella non copre. RF-002 è il primo esempio misurato.

## Final decision

**Changes requested.** Le tre regole per cui questa gate esiste sono implementate correttamente e
vincolate dalla suite: l'ho verificato per mutazione e non per lettura, e nessun limite è nel verso
sbagliato. Ciò che manca è la cucitura in tre punti, che è la stessa forma di difetto che questa spec
è nata per chiudere. Chiuse RF-001, RF-002 e RF-003, `GATE-SECREVIEW` è soddisfatta senza riserve per
quanto mi riguarda.

Nota di metodo, perché il Lead me l'ha chiesta esplicitamente: le due superfici segnalate come
probabili — le direzioni e l'aritmetica `u128` — sono risultate solide. È la terza volta consecutiva
su questo progetto. Suggerisco che questa regolarità sia ormai un dato: le superfici che il Lead sa
nominare sono anche quelle che l'implementatore legge con più attenzione, e il valore marginale della
review sta nel guardare ciò che nessuna delle due parti ha nominato. RF-004 e RF-003 sono stati
trovati così: guardando file che nessuna spec stava toccando.
