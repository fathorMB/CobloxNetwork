---
id: SPEC-007
# Note: Quote the title if it contains a colon
title: "Simulatore economico e taratura di alpha e dei parametri di elezione"
status: ready
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
- [ ] Il simulatore è eseguibile, deterministico a seme fissato, e il rapporto è riproducibile da chi lo riesegue.
- [ ] È riportata la **curva** che lega `α` alla quota catturata e al reddito del nodo onesto mediano, non il solo punto scelto.
- [ ] `α` iniziale è fissata **con il suo intervallo di sorveglianza**, e la scelta è motivata sul compromesso fra difendibilità e significato del reddito, non solo sulla difendibilità.
- [ ] Forma del fondo fissata: tetto per epoca e criterio di ripartizione, con l'interazione fra criterio e cattura per numerosità argomentata.
- [ ] Il valore `X` di [ADR-007] è fissato e i test `AT-07` e `AT-10` hanno un verdetto numerico, con `AT-10` valutato su tutte e tre le configurazioni.
- [ ] Tutti i parametri censiti sopra hanno un valore, e la combinazione **passa il blocco di vincoli**, verificato e non presunto.
- [ ] `validator_max_consecutive_terms_max` è scelto **stretto quanto la rete tollera**, con il numero che dice quanto tollera ([DEBT-010]).
- [ ] I tre accoppiamenti sono simulati **insieme**, e il rapporto mostra dove la rete si ferma senza avversario.
- [ ] Il rapporto espone le grandezze richieste da `SEC-REQ-16`.
- [ ] La formulazione di prodotto del reddito come **quota variabile** esiste, in inglese, pronta per l'interfaccia.
- [ ] Nessuna regola di protocollo è stata modificata.

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
- [ ] GATE-MODEL-VALIDATED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il modello riproduce l'aritmetica di [ADR-007] già verificata in modo indipendente dal Lead — 90,9% a `α=1` e 9,1% a `α=0,1` sullo scenario 10.000 contro 1.000 — prima di essere usato per decidere qualunque cosa. Incollare l'esecuzione e l'output reale. Un simulatore che non riproduce il caso noto non è evidenza per i casi ignoti.
- [ ] GATE-CONSTRAINTS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La combinazione di parametri proposta è verificata contro il blocco di vincoli del documento dei parametri di consenso, riga per riga, con l'esito di ciascuna. Incollare la verifica eseguita, non l'asserzione che passi.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto i valori scelti come superficie di sicurezza e il Lead ha accettato la review. `α` è il parametro che governa la resistenza ai Sybil dichiarata da [ADR-007]: sceglierlo senza revisione di sicurezza sarebbe incoerente con il modo in cui la regola che lo consuma è stata accettata.

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
