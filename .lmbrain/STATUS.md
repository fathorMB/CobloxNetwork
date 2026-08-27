---
title: Project pulse
status: active
milestone: M-02
updated: 2026-08-27
---

# Project Pulse

## Current focus

M-02 (Ledger vivo: federazione BFT su devnet). Dei quattro esiti che la milestone
nomina, solo il simulatore economico e' fatto. Non c'e' rete: `libp2p` non e' una
dipendenza del workspace.

Cosa esiste: una libreria di regole in `coblox-core` e le specifiche di
protocollo pubblicate. Ventuno spec `done` hanno prodotto regole scritte,
applicate e verificate; nessuna ha prodotto un nodo che parla con un altro nodo.

[SPEC-025] e' il prossimo passo verso l'esito della milestone.

## In progress

| Spec | Stato | Chi | Prossimo passo |
| --- | --- | --- | --- |
| [SPEC-022] | `review`, remediation aperta su [REVIEW-042] | AGENT-002 | Chiudere RF-001; richiede prima la decisione dell'operatore su [ADR-017] |
| [SPEC-023] | `review`, ferma | AGENT-002 | Attende deroga di `GATE-SECREVIEW` o quarta remediation ([REVIEW-038], [REVIEW-040], [REVIEW-041]) |

Nessun agente in esecuzione.

## Ready for handoff

| Spec | Stato | Chi | Nota |
| --- | --- | --- | --- |
| [SPEC-025] | `backlog`, `sol`/`maximum` | AGENT-002 | Attua [ADR-018]: motore di consenso, catena finalizzata da quattro validatori come prova. Non e' una devnet — rete e persistenza sono la spec successiva |
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

## Decisioni in attesa dell'operatore

1. Correzione di [ADR-017] su [REVIEW-042] RF-001 — il pavimento di `G` nell'ancora
   di genesi. Blocca la remediation di [SPEC-022].
2. Deroga di `GATE-SECREVIEW` su [SPEC-023], oppure autorizzare una quarta
   remediation.
3. Valori di taratura.
4. Advisory Dependabot.

## Debiti aperti

Tredici aperti, **cinque `high`**; nessun `critical`. Uno deferred ([DEBT-010], a
M-07).

| ID | Sev | Owner | Questione |
| --- | --- | --- | --- |
| [DEBT-028] | high | AGENT-002 | `election_epoch` dipende da un parametro governato senza che il documento lo dichiari |
| [DEBT-033] | high | AGENT-002 | `effective_height` non ha tetto, e il campo `reason` che porterebbe la distinzione non e' vincolato |
| [DEBT-034] | high | AGENT-007 | Un verdetto locale del ricevente puo' entrare in catena |
| [DEBT-036] | high | AGENT-002 | Dieci parametri di consenso su venti non sono ne' limitati in genesi ne' governati |
| [DEBT-037] | high | AGENT-007 | Tre campi di `EnrollmentParametersBody` non sono ne' limitati ne' validati |
| [DEBT-024] | medium | AGENT-007 | `ComputeAssignment` lascia al validatore la scelta del modulo |
| [DEBT-025] | medium | AGENT-007 | Coerenza fra matrice del threat model ed elenchi asset degli scenari |
| [DEBT-027] | medium | AGENT-LEAD | Trentasei superlativi non enumerati in artefatti del Lead |
| [DEBT-029] | medium | AGENT-001 | Il legame di contesto della preimmagine non e' imposto da nulla |
| [DEBT-031] | medium | AGENT-001 | La documentazione di modulo del crate fa affermazioni normative non garantite |
| [DEBT-032] | medium | AGENT-006 | Le probe della guida vedono la pagina allontanarsi da se stessa, non dal protocollo |
| [DEBT-035] | medium | AGENT-007 | Dentro la classe 0 l'ordine e' per ID di transazione, e il revocante puo' sfruttarlo |
| [DEBT-038] | medium | AGENT-002 | Il beacon di casualita' dedicato non ha un proprietario |

## Done

Ventuno spec, tutte con la propria review. Storia e finding stanno nella spec e
nella sua review, non qui.

| Spec | Chi | Chiusa |
| --- | --- | --- |
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
