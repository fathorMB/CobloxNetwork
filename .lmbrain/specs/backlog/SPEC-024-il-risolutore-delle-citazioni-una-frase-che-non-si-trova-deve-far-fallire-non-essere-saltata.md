---
id: SPEC-024
# Note: Quote the title if it contains a colon
title: "Il risolutore delle citazioni: una frase che non si trova deve far fallire, non essere saltata"
status: backlog
kind: feature
priority: high
area: tooling
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-008
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: terra
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: [SKILL-001, SKILL-003]
verification_gates: []
related_decisions: [ADR-012]
links: [DEBT-027, DEBT-033, DEBT-034]
created: 2026-08-26
updated: 2026-08-27
tags: [conformance, ci]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-27
    action: "set tags"
---
# Il risolutore delle citazioni: una frase che non si trova deve far fallire, non essere saltata

## Objective

Costruire uno strumento versionato che **risolve** le citazioni dagli artefatti del brain ai documenti di protocollo, e che **fallisce** su una citazione che non si trova e su una che non riesce a leggere.

Il progetto ha appena convertito le citazioni di un documento da numeri di riga a **frasi citate**, perché i numeri scadono. La conversione è un miglioramento reale, e ha già stanato due difetti che i numeri nascondevano. **Ma senza uno strumento che le risolva, citare per frase resta lo stesso onore dei numeri** — solo più difficile da falsificare per caso.

## Context

**Quattro volte in una sola sessione un puntatore di riga è morto.** [DEBT-033] citava `1037` per una clausola che sta a `1107`; [DEBT-034] citava `identity.md:614` per una regola che sta a `814`, scivolata di duecento righe quando due spec hanno riscritto il documento; e l'analisi dei dieci parametri ha visto le proprie citazioni scadere **mentre venivano scritte**, perché una remediation parallela stava modificando `ledger.md`.

L'ultimo caso è il più istruttivo: le citazioni erano corrette al momento in cui l'autore le ha aperte e lette. **Scritture disgiunte non sono riferimenti disgiunti**, e nessuna disciplina di coordinamento chiude una forma che dipende da un numero.

**Il progetto ha già la contromisura, applicata a un artefatto solo.** Le probe di [ADR-012] in `sim/tools/published_artifacts.toml` pinnano **una frase del documento** e falliscono se quella frase non è più lì. È il motivo per cui quell'inventario non è mai scaduto mentre i debiti scadevano attorno.

**E il progetto conosce già il modo in cui questa forma di strumento fallisce in silenzio.** La prova in negativo dell'inventario contiene questa mutazione, con la sua ragione scritta:

> una probe porta un campo `claims` in una forma che nessun consumatore può leggere, il che è peggio di un claim sbagliato: un consumatore che non riesce a leggerlo **salta** l'ancora, e un'ancora saltata è indistinguibile da una che tiene.

**Questa spec esiste perché quella frase vale anche qui**, ed è il criterio da cui discende tutto il resto.

### Lo stato di partenza, misurato

Il Lead ha scritto un risolutore usa-e-getta per verificare la conversione. Sul solo documento convertito:

- **37 frasi citate, 35 risolvono**; le due che non risolvono sono la stessa frase citata due volte, e non risolve perché la fonte porta `**10³–10⁶ legal values**` in **grassetto** e la citazione omette gli asterischi;
- **61 riferimenti di sezione, 12 distinti, tutti e 12 esistono**;
- **zero numeri di riga nudi** rimasti in quel documento.

Il risolutore ha inoltre avuto bisogno di **normalizzare gli spazi**, perché le frasi citate attraversano gli a capo del documento sorgente. Un risolutore che confronta riga per riga non trova quasi nulla e **non se ne accorge**.

## Scope
### Included

- **Lo strumento**, in `sim/tools/`, che per ogni citazione di un artefatto del brain verso `docs/protocol/` verifica che la frase citata **si trovi** nel documento nominato, e che la sezione nominata **esista**.
- **Il fallimento su una citazione illeggibile**, e non il salto. È il criterio portante.
- **Il fallimento su un numero di riga nudo** verso `docs/protocol/`: la pratica è la frase citata, e lo strumento è ciò che la rende una regola invece che un'abitudine.
- **La prova in negativo**, con una mutazione osservata fallire per ciascuna classe.
- **Il cablaggio in CI**, nello stesso job delle altre gate documentali.
- **La misura dell'arretrato**, e la decisione su come trattarlo, secondo il precedente di [DEBT-027] descritto sotto.
- **La chiusura delle due frasi che oggi non risolvono**, che lo strumento troverà da sé.
- **I rimandi a debiti chiusi dentro i documenti pubblicati** — estensione decisa il 2026-08-27 su proposta di [DEBT-038]. È la stessa forma di difetto che questa spec esiste per cogliere: un riferimento che **scade** senza che nessuno se ne accorga. Un documento di protocollo che cita `[DEBT-nnn]` risolto, superseduto o inesistente deve far fallire, perché chi segue il rimando trova un debito chiuso e conclude che la questione sia chiusa con esso. Il caso reale che motiva l'estensione: `ledger.md` rimandava a [DEBT-005] per il beacon di casualità dedicato, e [DEBT-005] è risolto da [SPEC-006] e non aveva quell'oggetto.
- **Le frasi che attraversano un a capo** devono risolvere. Il confronto normalizza gli spazi bianchi, e la trascrizione deve mostrare almeno una frase risolta che nel sorgente sta su due righe. Il 2026-08-27 il Lead ha sbagliato tre verifiche di citazione cercando a riga singola, e in un caso ha quasi dichiarato falsa un'affermazione vera di un agente.

### Excluded

- **Riscrivere le citazioni degli artefatti arretrati.** Se l'arretrato è grande, va **contato e tracciato**, non sanato dentro questa spec — vedi *Risks*.
- **Le citazioni fra artefatti del brain** ([ADR-017], [DEBT-036], le review): sono identificate per ID, che è già stabile. Fuori scopo.
- **Le citazioni al codice** (`core/`, `sim/`). Fuori scopo per ora, e va **detto** invece che sottinteso: il perimetro dichiarato di questo strumento è `docs/protocol/`.
- Qualunque modifica ai documenti di protocollo.

## Existing-project analysis

- `sim/tools/published_artifacts.py` e `.toml` — la macchina delle gate e il modello di come si pinna una frase. **Da leggere prima di scrivere una riga.**
- `sim/tools/published_artifacts_negative.py` — il modello della prova in negativo, e la mutazione sull'ancora saltata citata nel *Context*.
- `sim/tools/consensus_parameters_closure.py` — lo strumento più recente, e il modello di come si stampa un conteggio **senza cablarlo**.
- `sim/tools/lead_claims_check.py` — il modello per l'arretrato: vincola in avanti da una data, conta ciò che resta indietro, e rimanda a un debito.
- `.github/workflows/ci.yml`, job `protocol-docs` — dove le gate documentali sono cablate.
- `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md` — l'unico artefatto già convertito, e quindi il banco di prova.

## Technical proposal

**Il criterio portante: ciò che lo strumento non riesce a leggere deve farlo fallire.**

Un risolutore che incontra una citazione in una forma che non riconosce ha due condotte possibili, e una sola è ammissibile qui. Saltarla la rende **indistinguibile da una che risolve**, ed è il difetto che la prova in negativo dell'inventario già nomina. Lo strumento deve quindi enumerare le citazioni **e dichiarare quante ne ha lette**, fallendo su ciò che non ha saputo interpretare.

**Tre classi di difetto, come minimo:**

1. **frase che non risolve** nel documento nominato;
2. **sezione che non esiste** nel documento nominato;
3. **numero di riga nudo** verso `docs/protocol/`.

E una quarta che è la ragione di questa spec:

4. **citazione non interpretabile** — una forma che lo strumento non sa leggere.

**Due decisioni di progetto che la spec nomina e non prende**, perché vanno decise guardando i casi reali:

- **L'enfasi Markdown.** La fonte scrive `**10³–10⁶ legal values**`; la citazione riporta il testo senza asterischi. Normalizzare via l'enfasi prima di confrontare rende lo strumento più permissivo e chiude il caso reale che oggi fallisce; confrontare alla lettera è più stretto e obbliga chi cita a copiare la sintassi invece del testo. **Scegliere, e scrivere perché.**
- **La normalizzazione degli spazi è obbligatoria e non è una scelta**: le frasi attraversano gli a capo, e un confronto riga per riga fallisce quasi ovunque **senza dirlo**.

**Come si trovano gli artefatti da controllare.** Per **scoperta**, non per elenco: qualunque file di `.lmbrain/` che nomini `docs/protocol/`. Un elenco dichiarato non si accorge di un artefatto nuovo, ed è la famiglia che questo progetto ha censito **sette volte**, l'ultima delle quali proprio su un insieme di insiemi. Se lo strumento avesse un elenco, sarebbe l'ottava.

## Files and areas involved

- `sim/tools/` — lo strumento nuovo e la sua prova in negativo
- `.github/workflows/ci.yml` — il cablaggio nel job `protocol-docs`
- `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md` — le due frasi da chiudere
- Un debito nuovo, se l'arretrato lo richiede

## Acceptance criteria

- [ ] Lo strumento risolve le citazioni di tutti gli artefatti di `.lmbrain/` che nominano `docs/protocol/`, **trovati per scoperta e non da un elenco**, e dichiara quanti artefatti e quante citazioni ha letto.
- [ ] **Fallisce su una frase che non risolve**, osservato.
- [ ] **Fallisce su una sezione che non esiste**, osservato.
- [ ] **Fallisce su un numero di riga nudo** verso `docs/protocol/`, osservato.
- [ ] **Fallisce su una citazione che non riesce a interpretare**, osservato — e non la salta. È il criterio portante: la trascrizione deve mostrare il caso costruito apposta.
- [ ] La normalizzazione degli spazi è applicata, e **un test lo dimostra** con una frase che attraversa un a capo nel documento sorgente.
- [ ] La decisione sull'enfasi Markdown è **scritta con la propria ragione**, non implicita nel codice.
- [ ] **Le due frasi che oggi non risolvono sono chiuse**, e la chiusura è provata: prima dello strumento fallivano, dopo no.
- [ ] Lo strumento è cablato in CI insieme alla propria prova in negativo, e **il numero di run è nella trascrizione**.
- [ ] **L'arretrato è contato**, e la scelta su come trattarlo è dichiarata secondo il precedente descritto nei rischi.
- [ ] Lo strumento è `PASS` sull'albero a fine consegna, oppure **rosso con l'arretrato dichiarato e tracciato in un debito** — e in quel caso il cablaggio in CI va coordinato col Lead, perché una gate che nasce rossa viene disattivata.
- [ ] Nessun documento di `docs/protocol/` è stato modificato.
- [ ] `cargo test --workspace --all-features`, `clippy -D warnings`, `fmt --check` puliti; le gate esistenti tutte `exit=0`.

## Implementation plan

1. Leggere `published_artifacts_negative.py`, e in particolare la mutazione sull'ancora saltata: è il difetto che questo strumento deve non avere.
2. Scrivere lo strumento **prima** di chiudere le due frasi, così da vederlo fallire su un caso reale e non solo su uno costruito. Una gate che nasce verde non ha mai dimostrato di vedere.
3. Misurare l'arretrato **prima** di decidere come trattarlo.
4. Provare in negativo le quattro classi ([SKILL-001]).
5. Cablare, e riportare il numero di run.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-UNREADABLE-FAILS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una citazione in forma non interpretabile è stata costruita apposta e lo strumento **fallisce** nominandola, invece di saltarla. È il criterio portante di questa spec e la trascrizione deve mostrarlo.
- [ ] GATE-SEEN-IT-FAIL-FIRST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Lo strumento è stato eseguito **prima** di chiudere le due frasi note, e la trascrizione mostra che le nominava.
- [ ] GATE-NEGATIVE-PROOF | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Le quattro classi di difetto sono state **osservate fallire**, una mutazione per classe ([SKILL-001]).
- [ ] GATE-WRAPPED-PHRASE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una frase che attraversa un a capo nel documento sorgente è risolta correttamente, dimostrato con il caso.
- [ ] GATE-DISCOVERY-NOT-LIST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Lo strumento trova gli artefatti per scoperta: aggiungendo un file nuovo che cita `docs/protocol/` con una frase falsa, lo strumento **fallisce senza essere stato modificato**. Osservato.
- [ ] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=transcript | Pipeline reale verde, con numero di run e commit.
- [ ] GATE-LEAD-REPRO | kind=manual | owner=lead | phase=before-done | evidence=transcript | Il Lead riesegue in modo indipendente almeno la classe *citazione illeggibile* e la scoperta, invece di prenderle dall'evidenza.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

**L'arretrato è il rischio principale, e c'è un precedente da seguire invece che inventare.** `lead_claims_check.py` vincola gli artefatti **dalla propria data in avanti**, conta l'arretrato a ogni esecuzione e lo traccia in [DEBT-027], che ne registra trentasei. È il modo in cui uno strumento nasce verde senza mentire. Se l'arretrato di citazioni fosse grande, **la stessa forma va usata qui**; se fosse piccolo, conviene sanarlo e non aprire nulla. **La decisione dipende dal numero, quindi va contato per primo.**

**[DEBT-025] è l'avvertimento opposto**, e vale la pena tenerlo accanto: `threat_model_matrix_coherence.py` esiste, non è cablato, e i suoi disallineamenti vanno sanati **prima** del cablaggio, altrimenti la gate nasce rossa e viene disattivata. Una gate rossa alla nascita è peggio di una gate assente, perché insegna a ignorarla.

**Il perimetro va dichiarato dentro lo strumento**, non solo qui. Questo strumento guarda `docs/protocol/`. Non guarda le citazioni al codice, né quelle fra artefatti del brain. Una docstring che non lo dica produrrà, fra qualche mese, la stessa domanda che ha aperto [DEBT-037]: *l'insieme era più grande di quello che l'insieme dichiarava.*

**Se durante l'implementazione emergesse che una citazione non sostiene ciò che le è attribuito**, non è un difetto di risoluzione ed è fuori scopo: **va riportato al Lead** e non corretto. È già successo una volta in questa catena di lavoro, ed è stato gestito così.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **Ogni numero che scrivi va guardato.** In questa sessione quattro numeri sono finiti in un artefatto senza essere contati, e la correzione del primo ne ha introdotti altri due.
- **Ogni superlativo assoluto va contato**: o porti l'enumerazione, o non lo scrivi.
- **Consegna ogni dimostrazione insieme al perimetro su cui vale.**
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

### Files changed

### Verification performed

### Verification transcript
