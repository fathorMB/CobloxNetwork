---
id: SPEC-017
# Note: Quote the title if it contains a colon
title: "Il legame di catena dove oggi e ambiguo o assente"
status: backlog
kind: feature
priority: high
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
capability_tier: sol
thinking_level: standard
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012]
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [conformance, ledger, rust, security]
---

# Il legame di catena dove oggi e ambiguo o assente

## Objective

Chiudere [DEBT-020] e [DEBT-021], che sono la stessa questione a due altezze: **`chain_id` lega quasi tutto il protocollo, e in due punti quel legame non tiene.**

Alla genesi `chain_id` è **ambiguo**, perché la sua derivazione è circolare e nessuna regola dice come si rompe. Nel verificatore è **assente**, perché `SigningPreimage` garantisce da dove vengono i byte e non quale contesto rappresentino.

## Context

**[DEBT-020].** `chain_id` ← `genesis_block_id` ← intestazione di genesi ← `validator_set_hash`. La fixture `HASH-0` usa 32 byte a zero, ma è una fixture e non una regola. Due implementazioni possono derivare **due `chain_id` diversi dalla stessa distribuzione di genesi**, e poiché `chain_id` entra in quasi tutte le preimmagini a dominio separato, **non concorderebbero su nulla**. Colpisce anche l'ancora del light client, il cui passo 1 impone `chain_id` uguale al configurato: se il valore corretto è ambiguo, non esiste un solo valore configurabile giusto.

È la stessa forma di [DEBT-012], chiuso da [SPEC-010]: un valore che entra in una preimmagine e che nessun documento fissa, **invisibile a ogni test di questa base di codice** perché una sola implementazione è internamente coerente.

**[DEBT-021].** `SigningPreimage`, introdotto da [SPEC-014], garantisce che i byte siano stati prodotti da `signing_preimage`, e **nulla su quali byte siano**: il tipo non trasporta il `Domain` né il `chain_id`, che `signing_preimage` impasta nel prefisso e il tipo poi dimentica. Un chiamante che costruisse la preimmagine con il dominio sbagliato, o con il `chain_id` di un'altra catena, otterrebbe **un valore ben tipato e semanticamente falso**, e il verificatore lo accetterebbe.

È un fallimento **più difficile da notare** di quello che [SPEC-014] ha chiuso: là il prefisso spariva del tutto, qui c'è ma può essere quello sbagliato. E la separazione di dominio esiste precisamente per impedire che una firma valida in un contesto lo sia in un altro.

**Perché insieme e perché ora.** Entrambi hanno la stessa scadenza — **prima del primo chiamante del verificatore e prima che una devnet accumuli storia** — ed entrambi toccano la stessa grandezza. Chiuderne uno solo lascerebbe il legame di catena difeso a metà.

## Scope

### Included

- La regola normativa che rompe la circolarità di `chain_id` alla genesi, con la fixture pubblicata corrispondente.
- La generalizzazione che [DEBT-020] pone: quali altri valori entrano in una preimmagine **senza essere derivabili in un solo modo**.
- Il legame fra `SigningPreimage` e il contesto per cui è stato costruito, o la dimostrazione motivata che non serve.

### Excluded

- Qualunque modifica alla logica di verifica delle firme. [SPEC-012] l'ha chiusa e [REVIEW-019] l'ha verificata con tre oracoli indipendenti: **non si tocca**.
- Il contenimento della via non-consensus, chiuso da [SPEC-014] e verificato in entrambi i sensi.
- [DEBT-022], che è di AGENT-007 e attende la sua valutazione.

## Existing-project analysis

**Verificato dal Lead il 2026-08-26.** Le quattro produttrici di preimmagini dell'albero — tre in `registry.rs`, una in `validator_set.rs` — restituiscono tutte `SigningPreimage`, e nessuna resta a `Vec<u8>`: la conversione è completa, ed è la proprietà su cui questa spec può appoggiarsi per imporre il contesto **in un punto solo** invece che su ogni chiamante.

`signing_preimage(domain, chain_id, payload)` compone `dominio || 0x00 || chain_id_32 || payload`: **entrambe le grandezze sono già nei byte**, e il problema non è produrle ma conservarle nel tipo.

L'inventario di [SPEC-010] conta le preimmagini prive di fixture pubblicata, ma **non verifica che ogni valore che vi entra sia derivabile in un solo modo**. È la lacuna che [DEBT-020] indica, ed è la classe generale a cui questa spec deve rispondere con un elenco.

## Technical proposal

### 1. La circolarità, rotta da una regola

Una regola normativa che dice **come** si rompe alla genesi, con la fixture pubblicata corrispondente, così che due implementazioni indipendenti derivino lo stesso `chain_id` dalla stessa distribuzione. Il valore a 32 byte zero di `HASH-0` può essere la risposta giusta: **ciò che manca non è un valore, è che sia una regola.**

Nella stessa passata, la generalizzazione: **quali altri valori entrano in una preimmagine senza essere derivabili in un solo modo.** È l'esercizio che [SPEC-010] ha fatto per le codifiche simboliche e che va rifatto per le derivazioni — e come là, **la risposta è un elenco e non una rassicurazione.**

### 2. Il contesto, portato dal tipo

Il tipo deve rendere impossibile — o almeno rilevabile — usare una preimmagine costruita per un contesto in un altro. Tre forme sono plausibili e la scelta è dell'implementatore, con l'argomento:

- un tipo **parametrizzato sul dominio**, che sposta il controllo alla compilazione;
- **campi conservati** e confrontati in verifica;
- una funzione di verifica che **prende dominio e `chain_id` attesi** e li confronta.

**Il criterio che le distingue:** l'ergonomia dei chiamanti che ancora non esistono. Una forma che rende scomodo il caso corretto verrà aggirata dal primo che ha fretta — ed è la ragione per cui questa decisione si prende **ora** che i chiamanti si possono immaginare, e non dopo che sono scritti.

**Se la conclusione è che il legame non serve**, va scritta accanto al tipo con la sua ragione, non lasciata implicita.

## Files and areas involved

- `docs/protocol/README.md`, `ledger.md` — la regola di genesi e la fixture.
- `core/coblox-core/src/registry.rs` — il tipo e le produttrici.
- `core/coblox-core/src/lib.rs`, `verifier.rs` — solo la firma, **mai la logica**.
- `core/coblox-core/tests/`, `sim/tools/` — fixture, gate di [ADR-012], eventuale elenco delle derivazioni.

## Acceptance criteria

- [ ] Una regola normativa dice come si rompe la circolarità di `chain_id` alla genesi, e una fixture pubblicata la esercita.
- [ ] Due derivazioni indipendenti dalla stessa distribuzione di genesi producono lo stesso `chain_id`, e la seconda è fatta **senza riusare il codice della prima**.
- [ ] Esiste l'elenco dei valori che entrano in una preimmagine **senza essere derivabili in un solo modo**, anche se vuoto.
- [ ] Una preimmagine costruita per un dominio o una catena non è utilizzabile in un altro senza che qualcosa lo dica, **oppure** la ragione per cui non serve è scritta accanto al tipo.
- [ ] Se la forma scelta è di compilazione, il caso sbagliato **non compila**, e la trascrizione riporta l'errore.
- [ ] La logica di verifica delle firme è **invariata**: i dodici vettori upstream e i sette di estensione danno gli stessi esiti di prima.
- [ ] La gate di [ADR-012] è eseguita e la trascrizione allegata.

## Implementation plan

1. Stabilire come si rompe la circolarità e con quale fixture, **prima** di toccare il tipo: è l'unica delle due che cambia artefatti pubblicati.
2. Produrre l'elenco delle derivazioni non univoche.
3. Scegliere la forma del legame di contesto, motivandola sull'ergonomia dei chiamanti futuri.
4. Verificare che gli esiti dei vettori Ed25519 non si siano mossi.
5. Eseguire la gate di [ADR-012] e le prove in negativo.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-TWO-DERIVATIONS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il `chain_id` di genesi è derivato **due volte per due strade che non condividono codice**, e i due valori coincidono. Una regola che rompe una circolarità è verificabile solo così: un'implementazione sola è internamente coerente per costruzione, ed è precisamente il motivo per cui [DEBT-012] è rimasto invisibile fino a [SPEC-010].
- [ ] GATE-WRONG-CONTEXT-REJECTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una preimmagine costruita per un dominio o un `chain_id` e usata per un altro **è rifiutata o non compila**, e la trascrizione lo mostra. Se la conclusione è che il legame non serve, questa gate è sostituita dalla ragione scritta accanto al tipo e la sostituzione è dichiarata.
- [ ] GATE-VERIFIER-UNCHANGED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | I dodici vettori upstream e i sette di estensione producono gli stessi esiti di prima, e l'oracolo indipendente concorda. Questa spec tocca la **forma** e non il **comportamento**: se un esito si muove, l'ha capita male.
- [ ] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato e la trascrizione allegata. Questa spec aggiunge una regola di genesi e una fixture: è della classe che quella ADR governa.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto entrambe le chiusure e il Lead ha accettato la review. La separazione di dominio è la difesa che impedisce a una firma di valere in due contesti, ed è materia sua.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è una seconda derivazione che non è indipendente.** Rileggere il codice della prima e riscriverlo in un altro linguaggio non è una seconda strada: è la stessa strada con un'altra sintassi. La derivazione va rifatta **dal documento**, come il Lead ha fatto per `ER-0` e per i vettori Ed25519.
- **Il rischio secondario è la forma scomoda.** Un legame di contesto che rende scomodo il caso corretto verrà aggirato dal primo chiamante che ha fretta, e il tipo diventerà un ostacolo invece che una difesa. È il criterio esplicito con cui va scelta la forma.
- **La circolarità potrebbe non essere l'unica.** L'elenco delle derivazioni non univoche è la parte da cui il Lead si aspetta di più, esattamente come l'elenco delle preimmagini scoperte in [SPEC-010] valeva più della fixture che quella spec doveva aggiungere.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Fermarsi e riportare è un esito previsto**, e in questa spec vale in particolare se la rottura della circolarità richiedesse di cambiare una preimmagine già pubblicata: è una decisione del Lead e apre la propria passata.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

### Files changed

### Verification performed

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

```text

```

### Deviations from the specification

### Handoff status
- [ ] Ready for Project Lead review
