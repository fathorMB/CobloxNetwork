---
title: Project pulse
status: active
milestone: M-02
updated: 2026-08-27
---

# Project Pulse

## Current focus

**M-02, e il primo esito ha ora un motore.** La milestone nomina devnet BFT,
light client con prove Merkle, mint & burn, e simulatore economico. Solo il
simulatore e' fatto, chiuso il 2026-08-25. Non c'e' rete: `libp2p` non e' una
dipendenza del workspace.

**[SPEC-025] e' consegnata** e in `review`. Quattro validatori producono una
catena di dieci blocchi finalizzati con certificati veri che il verificatore
esistente accetta: il primo consenso reale del progetto, e **la regola di blocco
regge** — [REVIEW-047] l'ha verificata invece di crederle.

[REVIEW-047] chiede pero' modifiche su tre punti, e il piu' grave non e' nel
consenso ma in cio' che il consenso lascia libero: `transactions` non e' legato a
`header.transactions_root` da nessuna parte, quindi due nodi onesti possono
tenere `Block` diversi allo stesso `block_id` finalizzato, entrambi accettati dal
verificatore. Il rimedio e' gia' nel crate.

Non e' ancora una devnet — rete, persistenza e ciclo di vita del nodo sono la
spec successiva, e `coblox-node/src/main.rs` e' intatto.

**Rilievo dell'operatore del 2026-08-27, e va tenuto in vista.** La sessione si
era arenata su questioni che non servivano la milestone: tre spec nuove di cui
una di M-06, quattro review, otto debiti, tre versioni di una correzione a un ADR
sulla revoca. [SPEC-025] aveva `depends_on: []` e stava in `backlog` da ore.
Nulla la bloccava. E' la seconda sessione consecutiva a uscire di strada, ed
entrambe le volte se n'e' accorto l'operatore per primo.

La 5.1.0 impone di ancorarsi alla roadmap all'apertura di ogni sessione e di dire
quando il lavoro richiesto non serve la milestone corrente.

## In progress

| Spec | Stato | Chi | Prossimo passo |
| --- | --- | --- | --- |
| [SPEC-025] | `review`, [REVIEW-047] non superata | AGENT-002 | Tre bloccanti, **nessuno nella regola di blocco**, che regge. Il piu' grave: `transactions` non e' legato a `header.transactions_root`, quindi due nodi onesti possono tenere `Block` diversi allo stesso `block_id`. `GATE-CI-GREEN` attestata |
| [SPEC-022] | `review`, **congelata** | AGENT-002 | Attende la forma iniettiva del punto 3 di [ADR-017], riaperto da [REVIEW-046]. Nessuno tocca `ledger.md` ne' `core/` per essa: non e' in contesa con [SPEC-025] |

Nessun agente in esecuzione.

## Ready for handoff

| Spec | Stato | Chi | Nota |
| --- | --- | --- | --- |
| [SPEC-026] | `backlog`, `terra` | AGENT-008 | Rende eseguibili due discipline che oggi sono prosa: review non terminali su spec chiuse, e probe che portano un argomento di sicurezza senza nominare la review che l'ha attaccato |
| [SPEC-024] | `backlog` | AGENT-008 | Igiene sulle citazioni: una frase che non si trova deve far fallire, non essere saltata |

Sequenziare, non parallelizzare: la remediation di [SPEC-022] tocca `ledger.md` e
`core/`, gli stessi file di [SPEC-025], e dal 2026-08-27 sono entrambe di AGENT-002.
Due remediation parallele hanno gia' fatto scadere le citazioni di una mentre
venivano scritte.

## Blockers and risks

- **Il claim di sicurezza.** La rete e' robusta contro la falsificazione ma **non**
  resistente ai Sybil per via crittografica. Rinuncia esplicita, decisa su delega
  con [ADR-007] e dichiarata in `SECURITY.md`. Dettaglio in
  `knowledge/threat-model.md`.
- **Set di validatori auto-perpetuante in v0** ([DEBT-005], chiuso da [SPEC-006]
  come regola; il rischio residuo resta dichiarato).
- **Advisory Dependabot moderato** sul default branch, non ancora esaminato.

## Decisioni prese il 2026-08-27

Le quattro che erano in attesa sono decise. Nessuna decisione dell'operatore e'
oggi pendente.

1. **[ADR-017] corretta e approvata.** Pavimento di `G` ancorato in genesi come
   relazione: `revocation_effective_grace_blocks_min + 1 >= validator_min_set_size_min`.
   Nessun valore provvisorio da scegliere. Approvata dall'operatore dopo lettura
   il 2026-08-27, quindi vincolante per l'implementazione.
2. **[SPEC-023]: quarta passata ad AGENT-002**, dispacciata. Le passate 2 e 3
   erano state rimediate dal Lead e avevano introdotto due `high`.
3. **Taratura:** solo il pavimento di `G` ora, e la relazione lo risolve senza
   numeri. Il resto — `F`, `P`, `max_clock_drift_ms`, `D_max`/`S_max`, i dieci
   parametri — dopo la chiusura di [SPEC-023].
4. **Advisory Dependabot su `glib`:** chiuso come rischio accettato,
   [DEBT-039]. Nessun percorso di aggiornamento e nessuna esposizione del
   codice di progetto, entrambi accertati eseguendo.

## Debiti

Passata di triage il 2026-08-27, la prima del progetto: fino a quel giorno la
cartella `planned` era vuota e nessun debito era mai stato instradato. I diciotto
aperti si chiudevano solo come effetto collaterale di lavoro fatto per altro.

| Stato | N | Nota |
| --- | --- | --- |
| `planned` | 16 | Ognuno ha una spec bersaglio |
| `open` | 0 | La cartella e' vuota per la prima volta |
| `deferred` | 2 | [DEBT-010] a M-07, [DEBT-027] a innesco |
| `accepted-risk` | 1 | [DEBT-039], `glib` |
| `resolved` | 18 | 9 il 25 agosto, 9 il 26, zero il 27 |
| `superseded` | 2 | [DEBT-036] e [DEBT-041], entrambi per un conteggio o una diagnosi del Lead sbagliati |

**Instradati:** [DEBT-033], [DEBT-035], [DEBT-040] su [SPEC-022] — [DEBT-029],
[DEBT-034] su [SPEC-025] — [DEBT-028], [DEBT-037], [DEBT-043], [DEBT-044] su
[SPEC-027] — [DEBT-025] su [SPEC-026] — [DEBT-032] su [SPEC-024].

**Le tre decisioni dell'operatore sono prese il 2026-08-27.** [DEBT-024]: la
scelta congiunta dell'host non e' deliberata, l'host si deriva — [SPEC-028].
[DEBT-031]: adottato il criterio dell'enumerazione, controllo su [SPEC-026], che
passa a tre guardie e da `terra` a `sol`. [DEBT-038]: il beacon di casualita'
vive in M-03.

**Nessun debito e' `open`.** [DEBT-045] e' andato su [SPEC-027] perche' quella
spec **aggiunge** limiti al blocco dei vincoli: lasciare fuori il simulatore lo
farebbe divergere di piu' di quanto gia' diverge.

**[DEBT-041] superseduto da [DEBT-046] il 2026-08-27.** Non descriveva una
contraddizione: le due frasi stanno in sezioni diverse e parlano di due
consumatori di casualita' diversi. Il Lead aveva verificato che esistessero,
non che si contraddicessero. Il difetto residuo e' che una delle due non
qualifica il proprio ambito.

**Avvertenza su [DEBT-040]:** la sua *Statement* poggia su una premessa che
[REVIEW-045] ha accertato falsa. Va riscritta quando il debito viene lavorato,
non chiusa sulla motivazione attuale.

## Done

Ventidue spec, tutte con la propria review. Storia e finding stanno nella spec e
nella sua review, non qui.

| Spec | Chi | Chiusa |
| --- | --- | --- |
| [SPEC-023] I dieci parametri operativi nella lista DRAFT, e la gate che chiude la classe | AGENT-002 | 2026-08-27 |
| [SPEC-021] I valori della banda di cadenza nei documenti e nell'ancora di genesi | AGENT-002 | 2026-08-26 |
| [SPEC-020] L'orologio su cui si misura la scadenza di un'attestazione | AGENT-001 | 2026-08-26 |
| [SPEC-019] Cosa significa "non revocata" per autorizzare una spesa | AGENT-002 | 2026-08-26 |
| [SPEC-018] Quando `n/a` e' un esito ammissibile | AGENT-007 | 2026-08-26 |
| [SPEC-017] Il legame di catena dove oggi e' ambiguo o assente | AGENT-001 | 2026-08-26 |
| [SPEC-016] Gli orologi della catena | AGENT-002 | 2026-08-26 |
| [SPEC-015] Guida pubblica al funzionamento di Coblox | AGENT-006 | 2026-08-26 |
| [SPEC-014] I due cambiamenti breaking dell'API di `coblox-core` | AGENT-001 | 2026-08-25 |
| [SPEC-013] Separazione della chiave di trasporto dalla chiave di identita' | AGENT-001 | 2026-08-25 |
| [SPEC-012] Verificatore Ed25519 con i vettori speccheck come oracolo | AGENT-001 | 2026-08-25 |
| [SPEC-011] `RewardBounds` e le regole di validita' economiche | AGENT-001 | 2026-08-25 |
| [SPEC-010] Inventario degli artefatti pubblicati e codifica del `lifecycle` | AGENT-001 | 2026-08-25 |
| [SPEC-009] Attuazione di [ADR-010] e [ADR-011] | AGENT-002 | 2026-08-25 |
| [SPEC-008] Core del ledger in Rust | AGENT-001 | 2026-08-25 |
| [SPEC-007] Simulatore economico e taratura di `alpha` | AGENT-002 | 2026-08-25 |
| [SPEC-006] Regola di elezione e rotazione del set di validatori | AGENT-002 | 2026-08-25 |
| [SPEC-005] Applicazione di [ADR-009] al design system | AGENT-006 | 2026-08-25 |
| [SPEC-004] Threat model iniziale | AGENT-007 | 2026-08-25 |
| [SPEC-003] Fondamenta del design system | AGENT-006 | 2026-08-25 |
| [SPEC-002] Workspace Rust `coblox-core` con CI multipiattaforma | AGENT-008 | 2026-08-25 |
| [SPEC-001] Specifica del protocollo Coblox v0 | AGENT-001 | 2026-08-25 |

M-01 e' chiusa. Le sue quattro spec sono [SPEC-001] .. [SPEC-004].

## Decisioni

Diciotto ADR, tutte `accepted`. Il testo e il ragionamento stanno in
`decisions/`.

[ADR-018] e' l'ultima e la piu' rilevante per il lavoro corrente: fissa il
protocollo di consenso — cosa il voto firmato aveva gia' deciso, e il prevoto che
mancava. [SPEC-025] la attua.

## Riferimenti

- **Strategia di branching:** `main-only`, dichiarata in `BRANCHING.json`. Push su
  `main` riservato al Lead, nessun branch di feature, `commit_on_doc_change: false`.
- **Kit LMBrain:** 5.1.0 dal 2026-08-27.
- **Lingua:** inglese per tutto cio' che vede l'utente finale; italiano per gli
  artefatti interni.
- **Prima di scrivere un vincolo:** `knowledge/predicato-di-accettazione.md`.
- **Prima di chiudere una spec:** `knowledge/review-lifecycle-discipline.md`.
- **Difetti che si ripetono:** `knowledge/recurring-defects.md`.
- **Postura di sicurezza del repo pubblico:** `knowledge/postura-sicurezza-repo-pubblico.md`.
- **Disciplina di commit:** `knowledge/commit-discipline.md`.
