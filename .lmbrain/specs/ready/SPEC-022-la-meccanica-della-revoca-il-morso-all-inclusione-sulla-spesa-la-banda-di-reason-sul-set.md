---
id: SPEC-022
# Note: Quote the title if it contains a colon
title: "La meccanica della revoca: il morso all'inclusione sulla spesa, la banda di reason sul set"
status: ready
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
thinking_level: extended
effort_observations: []
depends_on: [SPEC-019]
dependency_events: []
parking_events: []
skills: [SKILL-001, SKILL-002, SKILL-003, SKILL-004]
verification_gates: []
related_decisions: [ADR-017, ADR-012, ADR-010]
links: [DEBT-033, DEBT-034, DEBT-035]
created: 2026-08-26
updated: 2026-08-26
tags: [identity, ledger, conformance, governance]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "set tags"
  - date: 2026-08-26
    action: "transitioned backlog -> ready"
---
# La meccanica della revoca: il morso all'inclusione sulla spesa, la banda di reason sul set

## Objective

Attuare [ADR-017] nei documenti di protocollo e in `coblox-core`, chiudendo [DEBT-033].

La revoca fa **due lavori** e questa spec li separa: sul percorso di **spesa** la revoca morde all'**altezza del blocco che la include**, e `effective_height` smette di governarlo; sul percorso del **set** `effective_height` conserva il proprio significato e guadagna una **banda dipendente da `reason`**, con i parametri che la definiscono vincolati nell'ancora di genesi.

**Il lavoro non è la regola nuova: è ciò che la regola nuova rende falso.** Quattro artefatti pubblicati diventano inammissibili, e uno di essi — la fixture `AUTH-0` — ha due righe che si **ribaltano**. Trovarli e correggerli è la metà cara di questa consegna.

## Context

[SPEC-019] ha fissato una definizione unica di *enrolled, unrevoked* ancorata a `effective_height`, e così facendo ha spostato peso su quel campo: prima governava la transizione del set, ora governa anche se una chiave revocata possa svuotare un saldo. `ledger.md` dichiara l'esposizione in chiaro — *nulla in v0 limita `effective_height` dall'alto*, quindi **quanto una revoca protegga un saldo lo sceglie il quorum che revoca** — e nomina tre domande aperte, classificandole come «meccanica della revoca» e osservando che è **lavoro che nessuno ha fatto**.

[ADR-017] è quel lavoro, deciso dall'operatore il 2026-08-26 sulla **seconda** stesura. La prima è stata sottoposta a critica avversariale prima della decisione, [REVIEW-036] di AGENT-007: **dieci finding e cinque errori fattuali**. Leggere quella review non è facoltativo per chi implementa, perché tre cose che sembrano ovvie sono già state provate e sono sbagliate.

**Il fatto che governa la decisione.** Il pavimento `min_revocation_effective_delay_blocks` ha **due** lavori, contati: dare ai superstiti una finestra per impegnare un set successore, e fare da tetto a `max_weak_subjectivity_age_ms` per il MUST di `ledger.md:1079`. **Nessuno dei due riguarda il saldo.** Un saldo non ha bisogno di una finestra per essere protetto: ne ha bisogno il set, e ne ha bisogno il light client.

**Tre cose che [REVIEW-036] ha già stabilito e che non vanno riscoperte:**

1. **`effective_height` è nominato da tre MUST, non da due.** Il terzo è `ledger.md:785`, *«The effective height MUST be later than the block proposing the revocation»*, scritto **con lo spazio** invece che con l'underscore. Tre artefatti hanno ripetuto il conto sbagliato perché enumeravano sul token invece che sulla grandezza. **Ogni ricerca di questa spec va fatta su entrambe le grafie.**
2. **L'ordine di esecuzione dentro un blocco è già deciso** da `ledger.md:2819`: `revoke_identity` è **classe 0**, `burn` e `fund_app` sono **classe 1**. Sul percorso del saldo il morso a `h` è quindi già determinato e sicuro. Non è una scelta di questa spec.
3. **`min_revocation_effective_delay_blocks` non è nel blocco dei vincoli di magnitudine** né in `ElectionBounds`. Senza portarcelo, la banda della parte 2 non toglierebbe la discrezione: la sposterebbe su un parametro che lo stesso quorum firma.

## Scope
### Included

- **Parte 1 di [ADR-017]** — sul percorso di autorizzazione delle transazioni, una revoca qualifica la chiave a partire dall'**altezza del blocco che la include**. Nei documenti e in `coblox-core`.
- **Parte 2 di [ADR-017]** — la banda a due lati su `effective_height` dipendente da `reason`, con `reason` che smette di essere inerte, e i tre parametri che la definiscono.
- **Il blocco dei vincoli di genesi** — i tre parametri entrano in `ledger.md#magnitudes-not-only-relations` e nell'ancora di fiducia di genesi, con i rispettivi limiti di magnitudine.
- **La clausola 3 di [ADR-017]** — ogni vincolo si valuta contro i parametri di consenso in vigore all'altezza del blocco che include la `revoke_identity`.
- **La passata di [ADR-012]** su tutto ciò che le due regole nuove rendono falso, con l'elenco minimo dato sotto e **l'obbligo di cercarne altri**.
- **Due voci nuove nella lista DRAFT** dei parametri di lancio, più **una che oggi manca** (vedi *Risks*).

### Excluded

- **La parte 3 della prima stesura di [ADR-017]**, cioè l'altezza dell'auditor in `challenge_evidence`. È stata **tolta dalla decisione** e la sua sostanza è su [DEBT-034]. Non implementarla: [REVIEW-036] RF-003 e RF-004 hanno stabilito che nella forma ovvia è teatro contro l'auditor ostile e cancella la divergenza invece di renderla visibile.
- **[DEBT-035]**, l'ordinamento per ID dentro la classe 0. È un debito proprio. **Se però l'implementazione dovesse valutare la qualificazione in esecuzione anziché contro lo stato pre-blocco, va segnalato al Lead prima di procedere**, perché è la condizione che rende [DEBT-035] sfruttabile.
- **La scelta dei valori di lancio.** Questa spec scrive **regole e vincoli, non numeri**. `min_revocation_effective_delay_blocks` non ha oggi un valore di lancio e i due parametri nuovi nemmeno. I valori restano DRAFT e sono decisione dell'operatore.
- **[DEBT-028]** e ogni altro debito non nominato qui.

## Existing-project analysis

- `docs/protocol/ledger.md` — la definizione di *unrevoked* (clausole 1 e 2), la tabella delle quattro autorizzazioni qualificate, la fixture `AUTH-0`, lo schema di `RevokeIdentityBody` con `reason` ed `effective_height`, la riga 785, le dieci clausole della revoca, il blocco dei vincoli di magnitudine, l'ordine di transizione.
- `docs/protocol/identity.md` — §"Revocation and key replacement", la frase sulla non retroattività, e §"Authentication on a connection" con la regola locale del ricevente, che **non va toccata** (è la superficie di [DEBT-034]).
- `docs/protocol/README.md` — `ConsensusParametersBody`, il checkpoint di soggettività debole con `revoked_validators`, la lista DRAFT.
- `core/coblox-core/src/authorization.rs` — `RevocationRecord`, il cui commento dichiara che l'altezza di inclusione è *«deliberately absent: the predicate does not read it»*.
- `core/coblox-core/src/params.rs` — la lettura di `min_revocation_effective_delay_blocks`.
- `core/coblox-core/tests/authorization_unrevoked.rs` — il test di conformità di `AUTH-0`.
- `sim/tools/published_artifacts.toml` e `published_artifacts.py` — l'inventario e la gate.

## Technical proposal

Sia `p` l'altezza del blocco che include la `revoke_identity`, `F` = `min_revocation_effective_delay_blocks`, `G` = `revocation_effective_grace_blocks` (nuovo), `P` = `max_planned_revocation_delay_blocks` (nuovo).

**Parte 1 — percorso di spesa.** La clausola 2 della definizione di *unrevoked* smette di leggere `effective_height` e legge l'altezza di inclusione: *nessuna `revoke_identity` finalizzata che nomina `node_id` è inclusa a un'altezza al più `h`*. Resta un fatto sul blocco e i suoi antenati, monotono in `h`, letto dagli stessi byte da ogni verificatore.

**Parte 2 — percorso del set.** `effective_height` conserva significato e clausole, e guadagna la banda:

| `reason` | Vincolo |
| --- | --- |
| `key_compromise` | `p + F <= effective_height <= p + F + G` |
| `validator_misconduct`, `operator_request` | `p + F <= effective_height <= p + P` |

**È una banda e non un'uguaglianza**, e la ragione va conservata nel documento: l'autore di una revoca **non conosce l'altezza di inclusione**, e un'uguaglianza avrebbe regalato a chi controlla un turno di proposta un **veto** sulla revoca d'emergenza ([REVIEW-036] RF-002).

**Il blocco dei vincoli di genesi** guadagna:

```text
min_revocation_effective_delay_blocks >= 1
revocation_effective_grace_blocks     >= 1
max_planned_revocation_delay_blocks   >= min_revocation_effective_delay_blocks
                                         + revocation_effective_grace_blocks

// limiti di magnitudine, presi dall'ancora di genesi e mai dal documento sotto
// valutazione:
min_revocation_effective_delay_blocks <= min_revocation_effective_delay_blocks_max
revocation_effective_grace_blocks     <= revocation_effective_grace_blocks_max
max_planned_revocation_delay_blocks   <= max_planned_revocation_delay_blocks_max
```

**`F >= 1` non è cosmetico**: è la riga che tiene insieme la parte 2 e `ledger.md:785`. Con `F = 0`, oggi permesso, la banda ammetterebbe `effective_height = p`, che quella riga vieta.

## Files and areas involved

- `docs/protocol/ledger.md`, `docs/protocol/identity.md`, `docs/protocol/README.md`
- `core/coblox-core/src/authorization.rs`, `core/coblox-core/src/params.rs`, e i moduli di validità che ne dipendono
- `core/coblox-core/tests/authorization_unrevoked.rs`, `tests/common/mod.rs`, e le fixture toccate
- `sim/tools/published_artifacts.toml`, `sim/tools/protocol_hashes.py`
- `.lmbrain/knowledge/threat-model.md` se la matrice cambia; `.lmbrain/knowledge/recurring-defects.md` se emerge una forma nuova

## Acceptance criteria

- [ ] La clausola 2 di *unrevoked* legge l'altezza di inclusione, e la tabella delle quattro autorizzazioni qualificate è coerente con la nuova lettura.
- [ ] La banda di `reason` è scritta come regola di validità, `reason` è **letto** da almeno una regola, e le due righe della tabella sono entrambe esercitate da un test.
- [ ] I tre parametri sono nel blocco dei vincoli di magnitudine **e** nell'ancora di fiducia di genesi, e un documento di consenso che li viola è **rifiutato in accettazione**, con il rifiuto provato da un test.
- [ ] `min_revocation_effective_delay_blocks >= 1` è imposto, e un documento con `F = 0` è rifiutato.
- [ ] Ogni vincolo è valutato contro i parametri in vigore **all'altezza che include la revoca**, e un test lo dimostra con due versioni di parametri diverse.
- [ ] **La fixture `AUTH-0` è ricalcolata**: le righe `21` e `49` passano da `valid` a `invalid`, la tabella è coerente riga per riga con la regola nuova, e il test di conformità la riproduce.
- [ ] **La riga `ledger.md:785` è riletta** e resa coerente con la banda: la sua relazione con `F >= 1` è dichiarata nel documento e non lasciata implicita.
- [ ] **Il commento di `RevocationRecord` è ritrattato**, non aggiornato in silenzio: dice che la motivazione precedente non vale più e perché.
- [ ] **Il checkpoint è annotato**: `revoked_validators` porta la grandezza del percorso del **set** e non quella del saldo, e il documento lo dice, perché un consumatore futuro leggerebbe quel campo come *«da quando la chiave non spende»* e sbaglierebbe.
- [ ] La frase sulla non retroattività in `identity.md` diventa **due**, una per percorso.
- [ ] **La passata di [ADR-012] è eseguita e trascritta**, `python sim/tools/published_artifacts.py` è `PASS`, e la passata ha cercato entrambe le grafie di `effective_height`.
- [ ] **La prova in negativo esiste**: ogni regola nuova è stata **osservata fallire** su un albero mutato, e in particolare il ribaltamento di `AUTH-0` fallisce se la regola viene rimessa com'era.
- [ ] La regola locale del ricevente in `identity.md` **non è stata toccata**, e la trascrizione lo dichiara.
- [ ] `cargo test --workspace --all-features`, `clippy -D warnings`, `fmt --check` puliti.

## Implementation plan

1. Leggere [ADR-017] e [REVIEW-036] per intero **prima** di toccare qualunque file. La review contiene tre soluzioni ovvie già provate e scartate.
2. Enumerare le occorrenze di `effective_height` **su entrambe le grafie**, su `docs/` + `core/` + `sim/`, e classificarle una per una: quali leggono il percorso di spesa, quali il percorso del set, quali sono prosa. È l'inventario da cui discende tutto il resto.
3. Scrivere prima i **documenti**, poi il codice dal testo dei documenti — non dal proprio ricordo della decisione ([SKILL-004]).
4. Ricalcolare `AUTH-0` **derivandola due volte in modo indipendente**, e far concordare le due derivazioni invece di copiare la seconda dalla prima.
5. Attuare in `coblox-core`, con i test che esercitano entrambe le righe della banda e le due frontiere della clausola 2.
6. Eseguire la passata di [ADR-012] ([SKILL-002]) e provare in negativo ([SKILL-001]).

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-ADR012-PASS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | `python sim/tools/published_artifacts.py` è `PASS`, e la trascrizione mostra che la passata ha cercato **entrambe le grafie** di `effective_height` e ha classificato ogni occorrenza.
- [ ] GATE-NEGATIVE-PROOF | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Ogni regola nuova è stata **osservata fallire** su un albero mutato, una mutazione per regola, con la trascrizione di ciascun fallimento. Include il ribaltamento di `AUTH-0`: rimettere la regola vecchia deve far fallire il test di conformità.
- [ ] GATE-TWO-ORACLES | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La tabella `AUTH-0` è derivata **due volte per strade indipendenti**, nessuna delle quali legge l'output dell'altra, e la trascrizione dichiara cosa è stato letto per costruire la seconda ([SKILL-004]).
- [ ] GATE-CI-GREEN | kind=manual | owner=agent | phase=before-done | evidence=transcript | La pipeline reale è verde su tutti i job, con numero di run e commit.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | Review di sicurezza di AGENT-007 sulla consegna. **Non è facoltativa**: questa spec cambia il predicato di autorizzazione delle transazioni, ed è la superficie su cui [REVIEW-036] ha già trovato dieci voci sulla sola decisione.
- [ ] GATE-LEAD-REPRO | kind=manual | owner=lead | phase=before-done | evidence=transcript | Il Lead riesegue in modo indipendente la derivazione di `AUTH-0` e almeno una delle mutazioni negative, invece di prenderle dall'evidenza.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

**Il rischio dominante è la composizione, e questa spec ne porta una nota.** [REVIEW-036] RF-005 ha stabilito che la parte 2 crea, per un quorum onesto, un **incentivo a ritardare l'autorizzazione** della revoca se `F + G` non bastasse a coordinare un set successore — e durante quel ritardo la chiave compromessa spende ancora, perché la parte 1 morde all'inclusione. **La tenuta della parte 1 dipende dalla prontezza dell'autorizzazione.** La banda `G` riduce l'incentivo senza chiuderlo. Se implementando emergesse una via per chiuderlo, va **riportata al Lead** invece che risolta dentro la spec.

**`reason` non è verificabile da nessuno.** Renderlo letto significa che un quorum che vuole latitudine su una chiave compromessa può dichiarare `operator_request`. È dichiarato in [ADR-017] come rischio residuo e **non va risolto qui**: è taratura, non meccanica.

**Tre parametri senza valore di lancio.** `F` non ne ha uno, e non è nemmeno nella lista DRAFT — **questa spec deve aggiungercelo**, insieme ai due nuovi. I valori delle fixture (`F = 1`) restano valori di fixture e non vanno promossi a valori di lancio.

**La superficie che questa spec non chiude e che va lasciata aperta consapevolmente.** La **raggiungibilità** resta saldata al pavimento: un nodo non validatore con `key_compromise` resta irraggiungibile-ma-iscritto per `F` blocchi, con `F` tarato su tutt'altro. È [REVIEW-036] RF-007 e vive su [DEBT-034]. Non allargare lo scopo per prenderla.

**Se la passata di [ADR-012] trovasse un quinto artefatto** oltre ai quattro enumerati, è un risultato della consegna e va riportato come tale, non assorbito in silenzio.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **Consegna ogni dimostrazione insieme al perimetro su cui vale.** È la regola che questa sessione ha imparato quattro volte, l'ultima delle quali sull'ADR che questa spec attua: un'enumerazione fatta su un token invece che su una grandezza è stata ripetuta da tre artefatti.
- **Ogni superlativo assoluto va contato**: *l'unico*, *il solo*, *nessun altro* sono affermazioni universali, e o portano l'enumerazione o non si scrivono.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

### Files changed

### Verification performed

### Verification transcript
