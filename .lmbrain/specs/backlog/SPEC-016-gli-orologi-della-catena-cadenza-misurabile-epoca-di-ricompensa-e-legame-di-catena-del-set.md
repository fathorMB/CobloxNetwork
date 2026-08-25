---
id: SPEC-016
# Note: Quote the title if it contains a colon
title: "Gli orologi della catena: cadenza misurabile, epoca di ricompensa, e il legame di catena del set"
status: backlog
kind: feature
priority: high
area: consensus
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012, ADR-013, ADR-010]
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [consensus, governance, light-client]
---

# Gli orologi della catena: cadenza misurabile, epoca di ricompensa, e il legame di catena del set

## Objective

Chiudere [DEBT-013], [DEBT-019] e [DEBT-014], che sono **tre facce della stessa domanda**: quali grandezze della catena sono scritte dai validatori, e quali proprietà il protocollo promette poggiandovi sopra.

Le prime due si chiudono nella stessa forma perché hanno la stessa causa, stabilita da AGENT-007 valutando [DEBT-013] e scritta in [ADR-013] parte 3: **nessuna regola di validità interna alla catena può vincolare il tempo reale, perché ogni orologio della catena è scritto dai validatori.** La terza si chiude in due paragrafi ed è qui perché **lavora sullo stesso oggetto** — il checkpoint di soggettività debole — e chi scrive la prima deve conoscerne la conclusione per non riaprirla.

## Context

**[DEBT-013].** `docs/protocol/` non specifica né la selezione del proposer né la meccanica dei round: manca il livello di produzione dei blocchi per intero. Un **terzo bloccante** — non un quorum — può allungare la durata reale delle epoche con la catena viva e ogni blocco valido. Tre effetti, con gravità diverse: l'incumbency diventa **illimitata**; il ritardo effettivo di revoca è denominato in blocchi e `ledger.md` promette che la catena *si ferma* a `effective_height`, altezza che rallentando **non arriva mai**; e l'emissione **si muove verso il basso**, quindi il rallentamento ha un costo **esternalizzato** — si perde l'emissione di tutta la rete e si conserva il seggio del solo cartello.

**[DEBT-019].** Nessun documento deriva `reward_epoch` da alcunché. Il pavimento su `reward_epoch_ms` introdotto da [SPEC-009] vincola la **durata dichiarata in un documento firmato**, non la velocità con cui gli indici avanzano nei mint. È [REVIEW-014] un livello più sotto.

**[DEBT-014] — rifiuto motivato, già valutato.** `validator_set_hash` **non ha bisogno** del legame con `chain_id`: i byte di un `ValidatorSet` lo legano già tre volte, e il Lead ne ha verificate due (`election_seed` ed `election_ticket` contengono `chain_id_32`). Regge su tutte e tre le superfici. **Resta da scriverlo**, perché un'eccezione non dichiarata si legge come una dimenticanza.

## Scope

### Included

- La misura della cadenza reale lato light client, dal checkpoint di soggettività debole.
- La procedura di rilascio dei checkpoint e la riga di genesi che la banda richiede.
- La derivazione di `reward_epoch`, o la dimostrazione motivata che non è ottenibile dentro la catena.
- I due paragrafi dichiarativi di [DEBT-014] e l'allineamento del commento in `registry.rs`.

### Excluded

- **Specificare la produzione dei blocchi** — selezione del proposer, meccanica dei round. È lavoro proprio e più grande, e [ADR-013] lo nomina come la premessa che, se cadesse, imporrebbe di riesaminare la sua parte 3.
- **Una regola di validità sulla distanza fra `timestamp_ms` consecutivi.** È **respinta** da [ADR-013]: obbliga i validatori a *scrivere* timestamp vicini, non a *produrre* blocchi vicini, e darebbe una chiusura falsa al prezzo pieno di una passata di [ADR-012]. Non va reintrodotta da nessuna porta.
- La taratura dei valori di banda, se la spec conclude che servono: è decisione dell'operatore, come `α` e la popolazione al lancio.

## Existing-project analysis

**Verificato dal Lead il 2026-08-26.** `election_seed = H("coblox-election-seed-v0\0" || chain_id_32 || …)` ed `election_ticket` idem: il legame di catena dentro `ValidatorSet` c'è. `reward_epoch` compare diciannove volte fra `ledger.md` e `README.md` e **nessuna occorrenza lo deriva**; l'unico `MUST` che lo nomina riguarda i limiti su `reward_epoch_ms`. Il termine *proposer* compare solo come sostantivo incidentale in discussioni di minaccia.

Il checkpoint di soggettività debole porta `height`, `timestamp_ms` e `issued_at_ms` firmati da una chiave che **non appartiene a nessun validatore**: è l'unico orologio esterno del protocollo, ed è già normativo.

## Technical proposal

L'ordine è quello di forza stabilito da AGENT-007, e la prima è la sola imprescindibile.

**1. La misura, lato light client.** Il light client ricava blocchi per millisecondo reale dal checkpoint che già detiene più l'intestazione fidata, e **fallisce chiuso o segnala** fuori da una banda dichiarata alla genesi. È contenuto normativo nuovo, quindi **la gate di [ADR-012] si applica**.

**2. La procedura di rilascio.** Il processo che emette i checkpoint non ne emette per una catena fuori banda. È procedura più una riga di genesi, non una regola di consenso.

**3. Il secondo limite in millisecondi di catena**, accanto a quello in blocchi, per le quantità che portano una promessa in tempo reale — il limite di mandato per primo. **Da sola è la stessa illusione** della regola respinta, perché i millisecondi di catena li scrivono i validatori: vale solo insieme al punto 1. Tocca `ElectionBounds` e la taratura di [SPEC-007].

**4. `reward_epoch`.** Derivarlo da una grandezza che i validatori non scrivono liberamente, oppure dimostrare che non è ottenibile dentro la catena — nel qual caso la chiusura ha **la stessa forma dei punti 1 e 2**: renderne l'avanzamento misurabile da fuori invece che vincolabile da dentro. Va valutato anche il verso opposto, un indice che non avanza affatto e congela l'emissione.

**5. [DEBT-014], due paragrafi.** In `README.md` accanto al registro delle preimmagini e in `ledger.md` accanto alla formula, più il commento in `registry.rs` cui manca la ragione. **Attenzione a un argomento falso:** la motivazione registrata nel debito — *«è una lista di chiavi, legarla impedirebbe di riusarla in una genesi nuova»* — **è sbagliata**, perché ogni `key_binding_signature` andrebbe riemessa comunque. Non va scritta: un'eccezione con la ragione sbagliata diventa un precedente.

## Files and areas involved

- `docs/protocol/ledger.md`, `docs/protocol/README.md` — la misura, la banda, `reward_epoch`, i due paragrafi.
- `core/coblox-core/src/light_client.rs`, `params.rs`, `registry.rs` — l'implementazione e il commento.
- `sim/tools/` — la gate di [ADR-012] e le eventuali probe.
- `sim/coblox_sim/` — solo se il punto 3 muove la taratura.

## Acceptance criteria

- [ ] Un light client che detiene un checkpoint e un'intestazione fidata **misura la cadenza reale** e si comporta come la regola dichiara fuori banda.
- [ ] La banda è dichiarata alla genesi e non scelta a runtime.
- [ ] La procedura di rilascio dei checkpoint rifiuta una catena fuori banda.
- [ ] `reward_epoch` è derivato da una grandezza che i validatori non scrivono liberamente, **oppure** la dimostrazione del contrario è scritta e la chiusura ha la forma dei punti 1 e 2. Entrambi i versi — indice troppo veloce e indice fermo — sono trattati.
- [ ] I due paragrafi di [DEBT-014] sono scritti, e **l'argomento falso non compare**.
- [ ] **Nessuna regola sulla distanza fra `timestamp_ms` è stata introdotta**, per nessuna via.
- [ ] Ogni valore pubblicato che cambia è ricalcolato con il metodo validato prima su un valore non modificato.
- [ ] La gate di [ADR-012] è eseguita e la trascrizione allegata.

## Implementation plan

1. Leggere la valutazione di AGENT-007 su [DEBT-013] e [DEBT-014] nel threat model: contiene la derivazione, non solo la conclusione.
2. Progettare la misura lato light client e la banda; stabilire il comportamento fuori banda **prima** di scrivere codice.
3. Affrontare `reward_epoch`, dichiarando quale dei due esiti si è raggiunto.
4. Scrivere i due paragrafi di [DEBT-014].
5. Eseguire la gate di [ADR-012] e le prove in negativo.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-MEASURE-BINDS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una catena simulata **fuori banda** produce l'esito che la regola dichiara, e una dentro banda no. La trascrizione mostra entrambi. Una misura che non si è mai vista scattare è un calcolo, non una guardia.
- [ ] GATE-NO-TIMESTAMP-RULE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una ricerca su tutto il diff mostra che **nessuna regola sulla distanza fra `timestamp_ms` consecutivi è stata introdotta**. È respinta da [ADR-013] e la sua reintroduzione sarebbe la famiglia 3 commessa dentro il rimedio: la gate esiste perché è il rimedio che sembra ovvio.
- [ ] GATE-BOTH-DIRECTIONS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Per `reward_epoch` entrambi i versi sono trattati con un caso ciascuno: indice che avanza troppo in fretta e indice che non avanza. Il secondo congela l'emissione senza violare alcuna regola ed è il gemello del caso che `README.md` già dichiara invalido per `reward_epoch_ms` sopra il tetto.
- [ ] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato e la trascrizione allegata, **anche se non trova nulla**.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la chiusura e il Lead ha accettato la review. Due dei tre debiti nascono da una sua valutazione, e il terzo da una sua osservazione adiacente: chiuderli senza la sua verifica sarebbe incoerente con il modo in cui sono stati aperti.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è dichiarare «chiuso» più di quanto sarà scritto.** Nessuno dei tre punti **impedisce** il rallentamento: lo rendono misurabile e dichiarato. Per un difetto la cui gravità è tutta nell'invisibilità è la parte che conta, ma la parola giusta va usata. AGENT-007 lo dice nella propria valutazione ed è il criterio con cui il Lead leggerà la consegna.
- **Il rischio secondario è il punto 3 da solo.** Un limite in millisecondi di catena senza la misura del punto 1 è la stessa illusione della regola respinta, con un'aria più tecnica.
- **La banda potrebbe richiedere un numero che non spetta all'implementatore.** Se emerge, va istruito come `α` e la popolazione al lancio — mostrando cosa comporta ciascun ordine di grandezza — e **portato all'operatore**, non scelto.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Contestare le formulazioni del Lead fa parte del mandato.** In questa spec l'ordine dei tre punti è di AGENT-007 e il Lead lo ha adottato: se l'implementazione mostra che è sbagliato, dillo.
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
