---
id: SPEC-023
# Note: Quote the title if it contains a colon
title: "I dieci parametri operativi nella lista DRAFT, e la gate che chiude la classe"
status: ready
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

- [ ] Tutti e dieci i parametri compaiono nella lista DRAFT, raggruppati per cosa governano, ciascuno con la grandezza che lo vincola dichiarata accanto.
- [ ] La lista DRAFT continua a coprire ciò che copriva prima: **nessuna voce esistente è stata persa** riorganizzandola, e la trascrizione lo dimostra confrontando prima e dopo.
- [ ] Esiste uno strumento versionato che confronta i campi di `ConsensusParametersBody` con l'unione di lista DRAFT e blocco dei vincoli.
- [ ] Lo strumento **fallisce** su un campo dello schema assente da entrambe le liste, e il fallimento è stato **osservato** aggiungendo un campo finto.
- [ ] Lo strumento fallisce anche **nell'altra direzione**: una voce delle liste che non corrisponde ad alcun campo dello schema. Osservato.
- [ ] Lo strumento è `PASS` sull'albero reale a fine consegna.
- [ ] Il documento di analisi copre **dieci parametri su dieci**, con le quattro domande risposte per ciascuno, e dichiara per ciascuno **cosa è stato letto** per rispondere.
- [ ] L'analisi distingue esplicitamente i parametri per cui il vincolo naturale è **relazionale** da quelli per cui è **di magnitudine**, e non propone un tetto uniforme.
- [ ] **Nessun valore di lancio è stato fissato**, e nessun limite è stato aggiunto al blocco dei vincoli.
- [ ] `cargo test --workspace --all-features`, `clippy -D warnings`, `fmt --check` puliti; `published_artifacts.py` `PASS`.

## Implementation plan

1. Enumerare i venti campi dallo schema, non dal ricordo, e classificarli contro le due liste. È l'inventario da cui discende il resto.
2. Scrivere la gate **prima** di correggere la lista, così che la si veda fallire sui dieci reali e non solo su un campo finto. È la prova più onesta che lo strumento funziona.
3. Riorganizzare la sezione DRAFT e aggiungere i dieci, verificando di non aver perso voci.
4. Provare la gate in negativo nelle due direzioni ([SKILL-001]).
5. Scrivere l'analisi, un parametro alla volta, dichiarando le fonti.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-CLASS-CLOSED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Lo strumento nuovo è `PASS` sull'albero e la trascrizione mostra i venti campi classificati uno per uno.
- [ ] GATE-NEGATIVE-PROOF | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Lo strumento è stato **osservato fallire** in entrambe le direzioni: un campo dello schema fuori da entrambe le liste, e una voce di lista senza campo corrispondente ([SKILL-001]).
- [ ] GATE-SEEN-IT-FAIL-FIRST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Lo strumento è stato eseguito **prima** della correzione della lista, e la trascrizione mostra che nominava i dieci parametri reali. Una gate che nasce verde non ha mai dimostrato di vedere.
- [ ] GATE-DRAFT-NO-LOSS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Confronto fra la sezione DRAFT prima e dopo, che dimostra che nessuna voce preesistente è stata persa nella riorganizzazione.
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

### Files changed

### Verification performed

### Verification transcript
