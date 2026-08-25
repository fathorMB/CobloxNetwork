---
id: SPEC-007
# Note: Quote the title if it contains a colon
title: "Simulatore economico e taratura di alpha e dei parametri di elezione"
status: done
kind: feature
priority: high
area: token-economy
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-005, ADR-006, ADR-007, ADR-008]
links: [SPEC-004, SPEC-006, DEBT-007, DEBT-010]
created: 2026-08-25
updated: 2026-08-25
tags: [simulation, sybil, economy]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "set effort"
  - date: 2026-08-25
    action: "set tags"
  - date: 2026-08-25
    action: "transitioned backlog -> ready"
  - date: 2026-08-25
    action: "transitioned ready -> working"
  - date: 2026-08-25
    action: "transitioned working -> review"
  - date: 2026-08-25
    action: "attested verification GATE-SECREVIEW by lead"
  - date: 2026-08-25
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "f5662825f25c965ff075d158bdcec94ccd11c21577fa58a831655e6f6059df0d"
    evidence_ref: "REVIEW-011, accettata da AGENT-007 dopo un giro di remediation su sette finding, di cui uno critico. Le cinque voci in scope sono chiuse e verificate una per una, senza che alcun valore raccomandato sia cambiato: la remediation ha toccato solo cio che il progetto afferma dei valori, che era esattamente il verdetto. Le due voci fuori scope sono confluite in ADR-010, accettata dall'operatore. Il Lead ha verificato in modo indipendente le tre affermazioni decisive: il rapporto 5/4 collassa gli interi piccoli in un punto, quindi c, m e cooldown sono congelati per sempre; V puo crescere 27-33-36 in due documenti leciti mentre min_set resta 18, e la contrazione 36-25-18 consegna il set a una coalizione del 50 per cento; e l'importo assoluto dirottato da una flotta e identico a uso nullo e al regime di riferimento, 15725 crediti, perche non dipende dall'uso."
    id: "SPEC-007-ATTEST-001"
    requirement_digest: "95438463baaf16cf1253fe882f3d76986a9aee3521a381ff6e43462efe598ca8"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T16:31:52.269828600+02:00"
---
# Simulatore economico e taratura di alpha e dei parametri di elezione

## Objective

Costruire il simulatore economico della rete e usarlo per fissare i valori che oggi il progetto non ha: la forma del fondo del reddito di esistenza, la frazione `α` dell'emissione che vi transita, il valore `X` della metrica riformulata da [ADR-007], e i parametri di elezione che [SPEC-006] ha deliberatamente lasciato simbolici. Chiude [DEBT-007].

## Context

`α` è il parametro più importante dell'economia della rete e **oggi non esiste da nessuna parte**. [ADR-007] ha stabilito che la resistenza ai Sybil è una proprietà economica e non crittografica, e che la grandezza che la governa è esattamente la frazione di emissione che passa dal canale del reddito di esistenza. L'aritmetica è già verificata in modo indipendente dal Lead: con `α = 1` una flotta di 10.000 identità emulate contro 1.000 nodi onesti cattura il 90,9% dell'emissione; con `α = 0,1` ne cattura il 9,1%.

**La decisione difficile è ancora davanti, non alle spalle.** [ADR-007] non ha sciolto il nodo, lo ha convertito in un parametro e lo ha rimandato qui. Il nodo è questo: `α` bassa è ciò che rende la rete difendibile, ma `α` è anche quanta emissione arriva al reddito di esistenza, cioè **quanto quel reddito significa qualcosa per un utente reale**. Abbassarla per sicurezza la svuota come promessa di prodotto. Il simulatore deve **esporre questo compromesso**, non nasconderlo dietro un numero raccomandato: il suo prodotto più prezioso è la curva che lega difendibilità e significato, non il punto che sceglierai su quella curva.

[SPEC-006] ha chiuso l'unico debito `critical` del progetto scrivendo la regola di elezione, e ha lasciato **ogni sua grandezza come parametro simbolico**, deliberatamente, dichiarando che i valori vengono da qui. Sono diciassette, più i tetti di genesi di `ElectionBounds`.

## Scope
### Included

- Il simulatore economico: modello agent-based dell'emissione, della spesa e della partecipazione, con scenari di attacco.
- La decisione sulla forma del fondo del reddito di esistenza: tetto per epoca e criterio di ripartizione.
- Il valore iniziale di `α` e il suo **intervallo di sorveglianza**.
- Il valore `X` della metrica riformulata di [ADR-007].
- I valori dei parametri di elezione di [SPEC-006] e dei tetti di `ElectionBounds`.
- Il rapporto del simulatore, con le grandezze che `SEC-REQ-16` obbliga a misurare.
- La formulazione di prodotto del fatto che il reddito è una **quota variabile** e non un importo garantito.

### Excluded

- **Qualunque modifica alle regole di protocollo.** Questa spec produce numeri e un simulatore, non regole. Se la simulazione mostrasse che una regola è sbagliata, **fermati e segnalalo**: si supera con una ADR, non si modifica il documento.
- L'implementazione del ledger in Rust, che è spec parallela e indipendente.
- La ponderazione per contributo dimostrato della quota al creatore ([ADR-006]), che il threat model segnala come lavoro di consenso e non di taratura.

## Existing-project analysis

**Le grandezze da fissare, censite dal Lead sui documenti di protocollo.** Dal blocco dei parametri di consenso: `election_epoch_blocks`, `candidacy_close_blocks`, `election_entropy_blocks`, `validator_min_set_size`, `validator_target_set_size`, `validator_max_set_size`, `validator_churn_cap_seats`, `validator_max_consecutive_terms`, `validator_cooldown_epochs`, `validator_min_capture_epochs`. Dalla politica di ricompensa: `storage_units_per_contribution_unit`, `compute_units_per_contribution_unit`, `validator_eligibility_threshold_units`, `validator_eligibility_window_epochs`, `validator_eligibility_min_issuers`. Da `ElectionBounds` nella genesi: `election_epoch_blocks_max`, `validator_max_consecutive_terms_max`, `validator_max_set_size_max`, `validator_min_set_size_min`, `validator_min_capture_epochs_min`, il rapporto di variazione e `election_parameter_min_activation_gap_blocks`.

**Non sei libero: il blocco di vincoli va rispettato e contiene due accoppiamenti non ovvi.** `T >= max(4, 3m)` e `ceil(V/T) <= c < V/3`. Il secondo implica che **`T <= 3` è insoddisfacibile a ogni dimensione del set** — verificato per forza bruta dal Lead fino a `V = 399`. Qualunque combinazione tu proponga deve passare il blocco: verificalo tu, non presumerlo.

**Tre accoppiamenti che vanno simulati insieme e non uno per volta**, segnalati da chi ha scritto la regola:

1. `validator_cooldown_epochs`, la soglia di eleggibilità e la dimensione del pool. Il pavimento di contrazione converte un degrado in un **arresto**: una rete che perda più di un terzo dei validatori vivi fra due confini si ferma. Se il cooldown è lungo e la soglia alta, il pool si svuota e l'arresto diventa raggiungibile senza avversario.
2. `validator_max_consecutive_terms_max`, il tetto di genesi. [DEBT-010] stabilisce che il limite di mandato è un **cricchetto spingibile e non tirabile**: un quorum che tocchi i due terzi anche una sola volta lo porta al tetto in modo permanente, e da lì quel tetto è l'**unico presidio residuo** sulla velocità di rotazione. Ne segue una regola operativa: sceglierlo **stretto quanto la rete tollera**, e il simulatore deve dire quanto tollera.
3. `α` e la forma del fondo. Sono la stessa decisione vista da due lati.

**Due orizzonti di cattura, e non uno.** [SPEC-006] li distingue e la distinzione va rispettata nella taratura: la cattura **per ammissione** ha orizzonte `ceil((V/3)/c)` ed è **tarabile** con `m`; la cattura **per attrito** ha orizzonte `ceil(log(V/k)/log(3/2))` ed è **fissa**, circa tre confini, e nessuna scelta di parametri la allunga. La sicurezza della regola è quella del suo percorso più debole: tarare `m` alto sapendo che l'attrito resta a tre confini è autoinganno.

## Technical proposal

Il modello deve produrre la **curva** prima del punto. Per ogni `α` nell'intervallo plausibile, e per popolazioni di nodi onesti ed emulati che coprano gli scenari del threat model, riporta la quota di emissione catturata, la quota che resta al nodo onesto mediano, e la vita economica di una flotta.

`AT-07` e `AT-10` di [SPEC-004] sono i test che il valore scelto deve superare, e `AT-10` ha ora tre configurazioni: cattura per ammissione, censura totale rifiutata dal pavimento, censura selettiva. Il verdetto numerico di `AT-10` era stato rimandato qui: emettilo.

**Sulla forma del fondo**, [ADR-007] ha già deciso il tetto per epoca; restano tetto e criterio di ripartizione. Considera che il criterio interagisce con `α`: una ripartizione uniforme fra i presenti massimizza la cattura per numerosità, una ponderata la riduce ma sposta il reddito verso chi contribuisce di più, cioè verso ciò che il canale `storage`/`compute` già premia — con il rischio di rendere il reddito di esistenza un doppione e non un pavimento.

## Files and areas involved

- Il simulatore: crate o strumento nuovo, collocazione a tua scelta motivata, fuori dal percorso di build del nodo.
- `.lmbrain/knowledge/`: il rapporto del simulatore come pagina di conoscenza versionata.
- `.lmbrain/knowledge/threat-model.md`: `SEC-REQ-16`, `SEC-REQ-18`, e i verdetti di `AT-07` e `AT-10`. È documento di AGENT-007: segui le sue convenzioni.
- Nessun file in `docs/protocol/` va modificato da questa spec.

## Acceptance criteria
- [x] Il simulatore è eseguibile, deterministico a seme fissato, e il rapporto è riproducibile da chi lo riesegue.
- [x] È riportata la **curva** che lega `α` alla quota catturata e al reddito del nodo onesto mediano, non il solo punto scelto.
- [x] `α` iniziale è fissata **con il suo intervallo di sorveglianza**, e la scelta è motivata sul compromesso fra difendibilità e significato del reddito, non solo sulla difendibilità.
- [x] Forma del fondo fissata: tetto per epoca e criterio di ripartizione, con l'interazione fra criterio e cattura per numerosità argomentata.
- [x] Il valore `X` di [ADR-007] è fissato e i test `AT-07` e `AT-10` hanno un verdetto numerico, con `AT-10` valutato su tutte e tre le configurazioni.
- [x] Tutti i parametri censiti sopra hanno un valore, e la combinazione **passa il blocco di vincoli**, verificato e non presunto.
- [x] `validator_max_consecutive_terms_max` è scelto **stretto quanto la rete tollera**, con il numero che dice quanto tollera ([DEBT-010]).
- [x] I tre accoppiamenti sono simulati **insieme**, e il rapporto mostra dove la rete si ferma senza avversario.
- [x] Il rapporto espone le grandezze richieste da `SEC-REQ-16`.
- [x] La formulazione di prodotto del reddito come **quota variabile** esiste, in inglese, pronta per l'interfaccia.
- [x] Nessuna regola di protocollo è stata modificata.

## Implementation plan
1. Leggere [ADR-007], [ADR-005], `threat-model.md` §6.2.4 e §7, e la sezione *Rotation: the cap and the floor* di `ledger.md`.
2. Costruire il modello e validarlo riproducendo l'aritmetica già verificata: 10.000 emulati contro 1.000 onesti, 90,9% a `α=1` e 9,1% a `α=0,1`. Se il modello non riproduce quei due numeri, è il modello a essere sbagliato.
3. Produrre la curva su `α` e gli scenari di attacco.
4. Fissare forma del fondo, `α`, intervallo di sorveglianza e `X`.
5. Tarare i parametri di elezione simulando i tre accoppiamenti insieme; verificare il blocco di vincoli.
6. Emettere i verdetti di `AT-07` e `AT-10` e aggiornare il threat model.
7. Scrivere il rapporto e la formulazione di prodotto.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-MODEL-VALIDATED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il modello riproduce l'aritmetica di [ADR-007] già verificata in modo indipendente dal Lead — 90,9% a `α=1` e 9,1% a `α=0,1` sullo scenario 10.000 contro 1.000 — prima di essere usato per decidere qualunque cosa. Incollare l'esecuzione e l'output reale. Un simulatore che non riproduce il caso noto non è evidenza per i casi ignoti.
- [x] GATE-CONSTRAINTS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La combinazione di parametri proposta è verificata contro il blocco di vincoli del documento dei parametri di consenso, riga per riga, con l'esito di ciascuna. Incollare la verifica eseguita, non l'asserzione che passi.
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto i valori scelti come superficie di sicurezza e il Lead ha accettato la review. `α` è il parametro che governa la resistenza ai Sybil dichiarata da [ADR-007]: sceglierlo senza revisione di sicurezza sarebbe incoerente con il modo in cui la regola che lo consuma è stata accettata.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio principale è produrre un numero raccomandato invece di una curva.** Un `α` con accanto la sua motivazione qualitativa è indistinguibile da un `α` scelto bene, finché qualcuno non lo mette alla prova. La curva è ciò che consente all'operatore di scegliere, e la scelta è sua: tu la istruisci.
- **Rischio di ottimizzare la sola difendibilità.** È la direzione in cui il modello spinge naturalmente, perché la cattura è misurabile e il significato del reddito no. Un `α` che rende la rete inattaccabile e il reddito irrilevante ha risolto il problema misurato e distrutto la promessa di prodotto che [[PROJECT]] mette in prima pagina.
- **Se la simulazione mostra che una regola è sbagliata**, e non solo che un parametro è scomodo, **fermati e segnalalo**. [SPEC-006] ha attraversato quattro giri di review adversariale e non va modificata da una spec di taratura: si supera con una ADR.
- **Aperto, e non lo risolvi tu:** la scelta finale di `α` è una decisione di prodotto dell'operatore, perché fissa quanto la rete tollera di perdere. Tu produci la curva, gli scenari e la raccomandazione motivata.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence

### Changes made

**Il simulatore.** Nuovo strumento in `sim/`, Python 3.11, solo libreria standard,
fuori dal percorso di build del nodo e fuori dal workspace Cargo: `Cargo.toml` di
radice e `core/` **non sono stati toccati**, come richiesto dal lavoro parallelo di
AGENT-001 su [SPEC-008]. Si esegue con `python -m coblox_sim` da `sim/`.

Il modello è deterministico per costruzione e non solo per seme: ogni estrazione è
`SHA-256(seme | flusso | indice)` invece di `random`, quindi le cifre sono
riproducibili fra versioni di Python e fra piattaforme e non soltanto dentro un
processo. La derivazione dell'elezione usa i **preimage del protocollo** — la
`election_ticket` è la SHA-256 con tag che `ledger.md` specifica — quindi
l'ordinamento dei candidati è quello del protocollo e non un sostituto.

Moduli: `params.py` (contenitori dei parametri e il **blocco di vincoli** valutato
regola per regola, con il testo di ciascuna allegato), `emission.py` (contabilità
dei mint: fondo a tetto `F // E` con resto scartato e mai emesso, compensi di
lavoro, quota al creatore, margine di reputazione di `threat-model.md` §6.3),
`election.py` (retain, commit, pool di riempimento, seme, ranking, riempimento sotto
il tetto, pavimento di contrazione, minimo del set, cooldown), `population.py`
(popolazioni sintetiche deterministiche a coda pesante), `recommended.py` (la
combinazione raccomandata e le sue assunzioni), `scenarios.py` (gli esperimenti),
`__main__.py` (il rapporto). 27 test in `sim/tests/test_simulator.py`.

**Le decisioni.** `α` iniziale **0,15** con intervallo di sorveglianza
**[0,10 – 0,20]**; `X` di [ADR-007] fissato al **20 %**; forma del fondo:
ripartizione **uniforme** (già regola di validità in `ledger.md`) con tetto di
genesi `F = 15 882 352 941` µt per epoca di ricompensa di un giorno, più una regola
di governance di `F` che lo muove al più del 25 % per documento per tenere `α` in
banda. I ventidue parametri censiti hanno tutti un valore, elencati in
`.lmbrain/knowledge/economic-simulation-report.md` §3.

**Il rapporto** è `.lmbrain/knowledge/economic-simulation-report.md`: la curva, il
compromesso, la forma del fondo, i verdetti di `AT-07` e `AT-10`, i tre
accoppiamenti, le tre grandezze di `SEC-REQ-16`, le assunzioni contestate, e la
formulazione di prodotto in inglese.

**Il threat model** (`.lmbrain/knowledge/threat-model.md`, documento di AGENT-007,
convenzioni seguite) riceve: una nota di valutazione sotto `AT-07` con le quattro
misure e il verdetto; una nota di verdetto numerico sotto `AT-10` con le tre
configurazioni; le righe di copertura di `SEC-REQ-16` e `SEC-REQ-18` aggiornate.

### Files changed

- `sim/` (nuovo): `README.md`, `coblox_sim/{__init__,__main__,params,emission,election,population,recommended,scenarios}.py`, `tests/test_simulator.py`, `_gates_transcript.txt`.
- `.lmbrain/knowledge/economic-simulation-report.md` (nuovo): il rapporto.
- `.lmbrain/knowledge/threat-model.md` (modificato): note di valutazione `AT-07` e `AT-10`, righe `SEC-REQ-16` e `SEC-REQ-18`.
- **Nessun file in `docs/protocol/`**, nessun file in `core/`, nessuna modifica a `Cargo.toml`. Verificato con `git status`.

### Verification performed

- `GATE-MODEL-VALIDATED`: il modello riproduce il caso noto **prima** di essere usato
  per decidere. Trascrizione sotto. Le cifre non sono la formula riscritta: il
  modello conia un'epoca nodo per nodo sotto la regola del ledger
  (`amount = F // E`, resto scartato e mai emesso) e poi somma chi ha ricevuto cosa.
- `GATE-CONSTRAINTS`: la combinazione è verificata contro il blocco di vincoli riga
  per riga, con l'esito di ciascuna, **due volte**: come documento di genesi, e con
  `T` spinto al tetto di genesi che [DEBT-010] rende raggiungibile in modo
  irreversibile. In più, i due accoppiamenti non ovvi sono **eseguiti per forza
  bruta** e non citati: `T <= 3` risulta insoddisfacibile a ogni `V` fino a 399, e
  il `T` minimo che ammette qualche `V` risulta `max(4, 3m)` per ogni `m` provato.
- Suite di test: 27 test, tutti verdi. Includono l'asserzione delle due cifre di
  [ADR-007], l'identità «reddito del telefono / reddito medio = `α`» su tutta la
  griglia, il rifiuto di una combinazione che viola `3c < V`, il rifiuto di una
  riduzione di `T` contro un documento attivo, il rifiuto di uno spazio di
  attivazione insufficiente, lo stagger di genesi, e la riproducibilità delle
  estrazioni.
- Nessuna regola di protocollo modificata: `git status -- docs/` è pulito.

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

```text
$ cd sim && python -m coblox_sim gates
Coblox economic simulator — SPEC-007
seed: SPEC-007/coblox-economic-simulator/v1
deterministic: every draw is SHA-256 of (seed, stream, index); no RNG state,
so the figures are reproducible across Python versions and platforms.

==============================================================================
GATE-MODEL-VALIDATED — the model reproduces the arithmetic of [ADR-007]
==============================================================================
Scenario: N = 10 000 emulated identities against H = 1 000 honest nodes.
Expected (verified independently by the Lead): 90,9 % at alpha=1, 9,1 % at alpha=0,1.

  alpha  observed alpha   captured   expected   verdict
   1.00        1.000000   90.9091%   90.9091%      PASS
   0.10        0.100000    9.0909%    9.0909%      PASS

The figures are not the formula restated: the model mints one epoch node by
node under the ledger's own rule (amount = F // E, remainder discarded and
never minted) and then sums who received what.

GATE-MODEL-VALIDATED: PASS

==============================================================================
GATE-CONSTRAINTS — the recommended combination against the constraint block
==============================================================================
parameter set: coblox-v0-genesis-candidate
V = 27   T = 9   c = 3   m = 3   cooldown = 2   min_set = 18

  [PASS] 0 < validator_min_set_size <= V <= validator_max_set_size 0 < 18 <= 27 <= 45
  [PASS] election_entropy_blocks >= 2                         720 >= 2
  [PASS] candidacy_close_blocks > election_entropy_blocks     17280 > 720
  [PASS] election_epoch_blocks > candidacy_close_blocks       120960 > 17280
  [PASS] T >= 1 and validator_cooldown_epochs >= 1            9 >= 1 and 2 >= 1
  [PASS] validator_cooldown_epochs <= T                       2 <= 9
  [PASS] validator_eligibility_window_epochs >= 1             28 >= 1
  [PASS] ceil(V / T) <= c                                     ceil(27/9) = 3 <= 3
  [PASS] 3 * c < V                                            3*3 = 9 < 27
  [PASS] 3 * c * m <= V                                       3*3*3 = 27 <= 27
  [PASS] storage_units_per_contribution_unit > 0              1073741824 > 0
  [PASS] compute_units_per_contribution_unit > 0              1000000 > 0
  [PASS] validator_eligibility_min_issuers >= 2               3 >= 2
  [PASS] election_epoch_blocks <= election_epoch_blocks_max   120960 <= 241920
  [PASS] T <= validator_max_consecutive_terms_max             9 <= 12
  [PASS] validator_max_set_size <= validator_max_set_size_max 45 <= 81
  [PASS] validator_min_set_size >= validator_min_set_size_min 18 >= 18
  [PASS] m >= validator_min_capture_epochs_min                3 >= 3
  [PASS] change_numerator > change_denominator > 0            5 > 4 > 0
  [PASS] election_parameter_min_activation_gap_blocks > 0     120960 > 0
  [PASS] rate of change vs active document                    genesis document: no active document to compare against
  [PASS] activation_height spacing                            genesis document: no previous activation height
  [PASS] T_new >= T_active                                    genesis document: no active T

  constraint block (genesis document): PASS

After the [DEBT-010] ratchet pushes T to its genesis ceiling (12):
  [PASS] 0 < validator_min_set_size <= V <= validator_max_set_size 0 < 18 <= 27 <= 45
  [PASS] election_entropy_blocks >= 2                         720 >= 2
  [PASS] candidacy_close_blocks > election_entropy_blocks     17280 > 720
  [PASS] election_epoch_blocks > candidacy_close_blocks       120960 > 17280
  [PASS] T >= 1 and validator_cooldown_epochs >= 1            12 >= 1 and 2 >= 1
  [PASS] validator_cooldown_epochs <= T                       2 <= 12
  [PASS] validator_eligibility_window_epochs >= 1             28 >= 1
  [PASS] ceil(V / T) <= c                                     ceil(27/12) = 3 <= 3
  [PASS] 3 * c < V                                            3*3 = 9 < 27
  [PASS] 3 * c * m <= V                                       3*3*3 = 27 <= 27
  [PASS] storage_units_per_contribution_unit > 0              1073741824 > 0
  [PASS] compute_units_per_contribution_unit > 0              1000000 > 0
  [PASS] validator_eligibility_min_issuers >= 2               3 >= 2
  [PASS] election_epoch_blocks <= election_epoch_blocks_max   120960 <= 241920
  [PASS] T <= validator_max_consecutive_terms_max             12 <= 12
  [PASS] validator_max_set_size <= validator_max_set_size_max 45 <= 81
  [PASS] validator_min_set_size >= validator_min_set_size_min 18 >= 18
  [PASS] m >= validator_min_capture_epochs_min                3 >= 3
  [PASS] change_numerator > change_denominator > 0            5 > 4 > 0
  [PASS] election_parameter_min_activation_gap_blocks > 0     120960 > 0
  [PASS] rate of change vs active document                    genesis document: no active document to compare against
  [PASS] activation_height spacing                            genesis document: no previous activation height
  [PASS] T_new >= T_active                                    genesis document: no active T

  constraint block (T at ceiling): PASS

Coupling 1, executed rather than quoted: is any (V, c) satisfiable for a
given term limit T at m = 1?  ledger.md claims T <= 3 is unsatisfiable at
every set size; brute force over V in [1, 399]:

    T  satisfiable  smallest V
    1        False        None
    2        False        None
    3        False        None
    4         True           4
    5         True           4
    6         True           4
    7         True           4
    8         True           4
    9         True           4
   10         True           4
   11         True           4
   12         True           4
   13         True           4
   14         True           4
   15         True           4
   16         True           4

  T >= max(4, 3m) coupling reproduced: PASS

Coupling 2: the declared capture horizon m bounds the term limit from below.
    m   smallest T that admits any V
    1                              4
    2                              6
    3                              9
    4                             12
    5                             15
    6                             18

GATE-CONSTRAINTS: PASS

==============================================================================
Gate summary
==============================================================================
GATE-MODEL-VALIDATED : PASS
GATE-CONSTRAINTS     : PASS

$ cd sim && python -m unittest discover -s tests
...................................
----------------------------------------------------------------------
Ran 35 tests in 0.078s

OK

$ cd sim && python -m coblox_sim > /dev/null; echo $?
0

$ git status --short -- docs/
(nessun output: nessun file di protocollo modificato)
```

Il rapporto completo — curva, deriva di `α`, forma del fondo, `AT-07`, `AT-10`, i
tre accoppiamenti, la portata della governance sui valori, `SEC-REQ-16`, i valori e
la formulazione di prodotto — si ottiene con `python -m coblox_sim` (895 righe dopo
la remediation di [REVIEW-011]) ed è trascritto in forma discorsiva in
`.lmbrain/knowledge/economic-simulation-report.md`.


### Remediation di [REVIEW-011] (2026-08-25)

`GATE-SECREVIEW` non superato: sette finding, **nessuno dei quali contesta un numero
o un valore raccomandato**. Il verdetto di AGENT-007 sui valori è che sono
difendibili; ciò che è stato contestato sono le **affermazioni** che li
accompagnavano. Quattro dei sette erano voci che questa spec aveva auto-segnalato in
*Deviations*. Nessun valore raccomandato è cambiato in questa passata.

**Cinque voci in-scope, tutte applicate. Nessuna tocca `docs/protocol/`.**

1. **RF-002 — `X` condizionata alla soglia d'uso ovunque compaia, e `AT-07`
   parzialmente coperto.** `AT-07` è schedulato su devnet, cioè nel regime in cui
   `W ≈ 0` e quindi `α ≈ 1`: il criterio (c) alla lettera è violato di circa cinque
   volte per tutto l'avviamento. Nuovo scenario `s11` che misura la rampa d'uso
   (99,01 % → 14,85 %), verdetto `AT-07` riformulato come **parzialmente coperto**,
   condizione scritta in rapporto §1, §4 e §7, nelle righe `SEC-REQ-16` e
   `SEC-REQ-18`, nella nota `AT-07` del threat model e nella nota di prodotto
   inglese. **Correzione a un'affermazione mia:** l'importo assoluto dirottato
   `D = F·N/(N+H)` **non contiene `W`** e quindi non cala col poco uso — «il 91 % di
   un'emissione minuscola è un'emissione minuscola» valeva solo se anche `F` è
   piccolo, e `F` è una scelta di governance. Con l'`F` di genesi una flotta al
   lancio dirotta ~15 725 cr per epoca. Il criterio assoluto è onesto solo se `F` al
   lancio è dimensionato sui nodi onesti presenti.
2. **RF-003 — la proprietà di `validator_min_set_size` qualificata.** Riprodotto con
   il simulatore il percorso che la review indica: `V: 27 → 33 → 36` con
   `T: 9 → 11 → 12`, **due documenti leciti** a un'epoca di elezione di distanza,
   ognuno accettato dal blocco di vincoli, portano `min_set/V` da 0,667 a **0,500**;
   e a `V = 36` la censura selettiva dà `36 → 25 → 18`, cioè l'intero set in due
   confini a una coalizione del **50 %**. L'affermazione è ora enunciata come
   proprietà della combinazione raccomandata, e la conclusione «appena sopra un
   terzo» di `ledger.md` è dichiarata da **non** cambiare finché la regola non
   esiste. Nuovi scenari `s10` / `s10b`.
3. **RF-001 parte 1 — valori e prassi, non regole.** Dichiarato per iscritto in
   rapporto §2 e §3 e nella riga `SEC-REQ-18` che
   `availability_microtokens_per_unit = 0`, il tetto di `F` e la disciplina 5/4 su
   `F` **non sono imposti da alcuna regola**, e che i criteri (a) e (c) di [ADR-007]
   sono veri *a condizione che la reward policy attiva li rispetti*.
4. **RF-006 — intervalli leciti e parametri congelati.** Nuovo `legal_next_intervals`
   in `params.py` e nuova sezione del rapporto. Il limite 5/4 su interi piccoli è un
   **congelamento**: `c = 3`, `m = 3` e `cooldown = 2` hanno intervallo lecito
   `[3,3]`, `[3,3]` e `[2,2]`. Ne discende **`V ≤ 36` per sempre**, quindi
   `validator_max_set_size = 45` e `max_set_max = 81` sono margini irraggiungibili e
   le loro motivazioni («margine di crescita», «3V») erano sbagliate e sono corrette.
   Registrato anche che l'argomento con cui questa spec motivava `T_max = 12` — «una
   rete col pool sottile non avrebbe più mosse» — vale per `T` ma **per `c` e per il
   cooldown la mossa non esiste comunque**.
5. **RF-005 parti 2 e 3.** Aggiunta alla nota onesta inglese la frase mancante: la
   fetta il nodo finto la prende **al posto dell'utente**, e la rete non può
   impedirlo; più la condizione d'uso sul «under 20 %» e la voce «protected» fra le
   parole da evitare. Aggiunta la grandezza **(d)** a `SEC-REQ-16` — la frazione di
   reddito che un nodo onesto di sola availability conserva sotto il banco di
   `AT-07`, lo 0,99 %, che non contiene `α`. Aggiunto in rapporto §1 che i due bordi
   della banda sono dichiarati in mondi diversi, e che il bordo inferiore è una
   **scelta di prodotto travestita da misura**, detto esplicitamente e non quasi.

**RF-004 adottato** nella sua diagnosi più generale: il difetto non è nei singoli
criteri ma nel **modo in cui il test è scritto** — entrambe le occorrenze sono
affermazioni assolute su una grandezza emergente, scritte prima della regola che la
produce, e nessuna nomina una regola. Rapporto §5 allineato alla convenzione che
AGENT-007 ha aggiunto al proprio documento. **RF-007** è lavoro suo ed è già
applicato.

**Incorporata l'osservazione di sicurezza di AGENT-007 sul verso di avvicinamento:**
poiché `α` è osservata ed è massima quando la rete è più nuova, il punto conta meno
del verso, e un eventuale margine va preso sul **bordo superiore**, duale a `X` e
messo alla prova per primo. Il cooldown `= 2` è ora dichiarato **irreversibile**
accanto alla sua motivazione.

**Fuori scope, non toccato:** `RewardBounds` di genesi (RF-001 parte 2) e
`3 · min_set ≥ 2V` (RF-003 parte 2) sono regole di validità nuove, cioè modifiche di
protocollo che [SPEC-007] esclude dal proprio scope. `docs/protocol/` non è stato
toccato; l'ADR è del Lead. Segnalo che `ADR-010` risulta già in preparazione
nell'albero.

**Verifica dopo la remediation:** entrambe le gate `before-submit` rieseguite e
verdi; suite passata da 27 a **35 test**, con i nuovi che eseguono i due finding
della review invece di accettarli — `test_three_parameters_are_frozen_by_the_rate_limit`,
`test_target_set_size_is_permanently_capped`,
`test_min_set_over_v_is_not_preserved_by_any_rule`,
`test_attrition_capture_completes_at_half_the_set_once_v_has_grown`,
`test_x_as_written_is_violated_below_the_usage_floor`,
`test_the_absolute_diverted_amount_does_not_depend_on_usage`.

### Deviations from the specification

Nessuna deviazione dallo scope. Sette cose che il Lead deve vedere, tutte
registrate anche nel rapporto §8 e nessuna delle quali modifica una regola.
[REVIEW-011] ne ha promosse quattro a finding — le voci 1, 2, 3 e 4 — e ne ha
aggiunte due nuove, entrambe riprodotte con il simulatore e recepite sopra:

1. **Il criterio di superamento di `AT-10` non è soddisfacibile da alcuna rete
   operabile.** «Non raggiunge 1/3 entro 50 epoche» equivale alla richiesta
   `m >= 50`, che per `T >= 3m` forza `T >= 150` e `c <= V/150`: 150 validatori che
   ruotano un seggio per confine con mandati di tre anni. Il verdetto numerico è
   emesso (fallito a `N/H = 1` e `N/H = 10`) e la correzione proposta è **registrata
   e non applicata**: il criterio è di AGENT-007 e la perdita dichiarata è una scelta
   di prodotto. È la seconda volta che un criterio di `AT-10` risulta sbagliato
   invece che non soddisfatto; il documento ha già il precedente e la sua motivazione.
2. **`α` non protegge il telefono onesto dalla diluizione Sybil.** Il rapporto di
   perdita dell'onesto sotto attacco è `H/(N+H)` e non contiene `α`. [ADR-007]
   misura la grandezza rivolta all'attaccante e su quella ha ragione; la conseguenza
   rivolta all'utente non era scritta da nessuna parte. Da rivedere a
   `GATE-SECREVIEW`.
3. **`validator_min_set_size` fa lavoro anti-cattura che `ledger.md` non gli
   attribuisce.** Con il minimo a `2V/3` la censura selettiva si blocca e non
   completa mai la cattura sotto i due terzi. La conclusione «soglia effettiva
   appena sopra un terzo» resta esatta del pavimento preso da solo. **Non ho toccato
   `ledger.md`**: se la revisione conferma, è una passata futura su quel documento.
4. **`availability_microtokens_per_unit` deve valere 0.** Un valore positivo rompe
   il criterio (a) della metrica di [ADR-007] — la flotta stampa — ed è misurato nel
   controesempio. È una scelta di valore, non una regola nuova; renderla non
   violabile per governance richiederebbe un'ADR.
5. **`α` non è una manopola.** `ledger.md` lo dice già; la conseguenza operativa è
   che la banda **non può valere alla genesi**, dove `W ≈ 0` e `α ≈ 1` qualunque sia
   `F`. Proposta: la banda vincola da una soglia d'uso dichiarata (25 % dell'uso di
   riferimento) e sotto quella si pubblica l'importo assoluto dirottato. Da
   confermare dall'operatore.
6. **Il margine di reputazione di §6.3 resta aperto.** Ai valori tarati è di circa 3
   finti abbonati per nodo controllato per periodo — molto meno dei 50× stimati da
   cifre illustrative, ma non zero — e non è chiudibile per taratura. Le risposte
   sono le opzioni 2 e 3 di §6.3, lavoro di consenso sotto [ADR-006], escluse da
   questa spec. Riportato come `SEC-REQ-16` (b) obbliga.
7. **L'intervallo di blocco (5 s) è un'assunzione dichiarata**, perché nessun
   documento di protocollo ne fissa uno. Se il progetto ne sceglie un altro, i
   parametri espressi in blocchi vanno riscalati e il blocco di vincoli rieseguito —
   che costa un comando.

Scelta implementativa da motivare, come la spec chiedeva: **Python invece di un
crate Rust**. Il crate avrebbe richiesto di toccare il manifest del workspace, che
il mandato vieta mentre AGENT-001 lavora lì; e uno strumento il cui unico scopo è
essere rieseguito e contestato da qualcun altro non dovrebbe richiedere una
toolchain per girare. `sim/` è anche uno dei `primary_files` del profilo AGENT-002.

### Handoff status
- [x] Ready for Project Lead review