---
id: SPEC-010
# Note: Quote the title if it contains a colon
title: "Inventario degli artefatti pubblicati, codifica del lifecycle e precisione normativa"
status: backlog
kind: feature
priority: high
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012, ADR-013]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [conformance, ledger, documentation]
---

# Inventario degli artefatti pubblicati, codifica del lifecycle e precisione normativa

## Objective

Rendere **eseguibile** la gate di [ADR-012], che oggi non lo è, e chiudere nella stessa passata i due difetti di specifica che quella gate avrebbe intercettato se fosse esistita: la codifica mancante di `lifecycle_u8` ([DEBT-012]) e le due affermazioni sovradimensionate del protocollo v0 ([DEBT-008]).

[ADR-012] impone a ogni spec che introduca o modifichi una regola di validità una passata su **tutti gli artefatti pubblicati**. La ADR lo scrive di sé stessa nelle proprie conseguenze: *«Chi scrive una spec deve sapere quali siano gli artefatti pubblicati, il che è un esercizio utile in sé: la quarta occorrenza esiste perché quell'inventario non era mai stato fatto.»* Finché l'elenco non esiste, la gate è un buon proposito con un nome da meccanismo.

## Context

Quattro volte, in tre spec diverse, un artefatto pubblicato ha insegnato una forma che le regole del protocollo rendono inammissibile. La tabella completa è in [ADR-012]. Ogni volta è emerso per caso.

[DEBT-012] ne indica la generalizzazione, e questa spec la raccoglie come oggetto primario invece che come nota a margine:

> Colpisce inoltre una superficie che il registro di conformità non copre, il che indica una lacuna più generale da valutare nella stessa occasione: **quante altre preimmagini non hanno una fixture pubblicata, e fra queste quante contengono campi la cui codifica non è fissata altrove.**

Il caso concreto che l'ha rivelata: la preimmagine di `app_leaf` in `docs/protocol/ledger.md` commette un campo `lifecycle_u8`, ma nessuno dei quattro documenti assegna un valore numerico agli stati `active`, `grace`, `suspended`. Due implementazioni conformi calcolano `app_leaf` diverse per lo stesso stato, quindi `state_root` diverse, quindi una divisione della catena al primo conto di app che non sia `active`. `coblox-core` usa una codifica provvisoria `0/1/2` documentata come tale e bloccata da un test che dichiara di **non** essere una prova di correttezza.

Questa spec porta anche due contenuti normativi decisi dall'operatore il 2026-08-25 e non ancora scritti nei documenti: l'intervallo di blocco di [ADR-013] e la conseguenza che quella ADR dichiara insieme al valore.

## Scope

### Included

- L'**inventario degli artefatti pubblicati**, in forma versionata e leggibile da uno strumento.
- Lo **strumento versionato** che [ADR-012] richiede, con la sua prova in negativo.
- La codifica numerica del ciclo di vita del conto di app, con la fixture che oggi manca ([DEBT-012]).
- Le due riformulazioni di [DEBT-008].
- L'intervallo di blocco di [ADR-013] nei documenti di protocollo, con la sua non-imposizione dichiarata.
- L'allineamento di `coblox-core` dove la codifica provvisoria diventa definitiva.

### Excluded

- **`RewardBounds` e le tre regole di validità di [SPEC-009] in `coblox-core`.** Sono [SPEC-011] e non vanno anticipate qui: questa spec deve produrre l'inventario *prima* che qualcuno lo usi, altrimenti si autocertifica.
- **Il verificatore Ed25519** ([SPEC-012]) e **la separazione della chiave di trasporto** ([SPEC-013]).
- Qualunque modifica alla **regola** di elezione o alla reward policy. Qui si scrivono codifiche mancanti e si correggono frasi, non si cambiano regole — con la sola eccezione discussa in *Risks and open decisions*.

## Existing-project analysis

**Verificato dal Lead il 2026-08-25.** Ciò che segue è stato controllato nei file, non ricordato; dove il Lead non ha verificato, è detto.

- `docs/protocol/` conta 4 566 righe su quattro documenti: `README.md` 941, `ledger.md` 2 382, `identity.md` 447, `wire.md` 442, `app-manifest.md` 354.
- Il registro di conformità in `README.md` elenca dieci valori di hash attesi con la loro fixture. `app_leaf` **non è fra questi**, e nessuna fixture pubblicata l'avrebbe intercettata.
- `sim/tools/protocol_hashes.py` e `sim/tools/reward_rules.py` esistono e sono versionati. Sono i due strumenti che [ADR-012] cita come precedente, e **hanno già trovato un difetto ciascuno**. Sono il modello da seguire, non da riscrivere.
- `core/coblox-core/tests/conformance_registry.rs` e `light_client_perimeter.rs` incorporano gli hash attesi come costanti. Sono una terza copia degli stessi valori, dopo il documento e lo strumento Python.
- La ricerca di `lifecycle_u8` su tutti i documenti restituisce **una sola riga**, quella che lo usa. Nessuna che lo definisca.

**Un'osservazione del Lead che l'implementatore deve contestare se la giudica sbagliata.** Gli stessi valori di hash vivono oggi in almeno tre sedi indipendenti — documento, strumento Python, test Rust — allineate a mano. È la forma in cui un artefatto pubblicato invecchia in silenzio, ed è plausibile che l'inventario debba affrontare anche questo. Il Lead **non** prescrive la soluzione: prescrive che la questione sia affrontata esplicitamente e che la scelta sia motivata.

## Technical proposal

### 1. L'inventario non è un elenco scritto a mano

Un elenco compilato a mano ha lo stesso difetto degli artefatti che deve sorvegliare: invecchia in silenzio, e si legge come se fosse aggiornato. L'inventario deve quindi avere **due metà**, e la seconda è quella che conta.

**a. Un manifesto versionato** che enumera gli artefatti pubblicati con, per ciascuno, la sede, il tipo e ciò che il documento asserisce su di esso.

**b. Una verifica di completezza del manifesto stesso**, che enumera meccanicamente i candidati presenti nei documenti — i blocchi di esempio canonici, le righe delle tabelle di conformità, i valori dichiarati attesi — e **fallisce se un candidato non compare nel manifesto**. Senza questa metà, il manifesto è una dichiarazione di intenti con un formato.

Il perimetro è quello di [ADR-012]: registro di conformità, esempi normativi e ogni valore che i documenti espongono come atteso; non gli artefatti interni al brain.

### 2. Lo strumento, e la sua prova in negativo

Lo strumento versionato esegue l'inventario e verifica ciò che l'inventario dichiara verificabile. [ADR-012] pone tre condizioni, e la terza è quella che il progetto ha pagato: **lo strumento deve saper fallire, e va verificato in negativo**, reintroducendo il difetto e osservando la guardia fallire. Una guardia che non sa fallire non è una guardia, e un falso positivo insegna a non fidarsi — [SPEC-009] ha creato il precedente con una costante di confronto invecchiata che gridava al lupo su un allineamento corretto.

### 3. La codifica del ciclo di vita

Assegnare valori numerici espliciti agli stati del conto di app, con due vincoli di forma:

- La codifica va **dichiarata nel documento di protocollo**, non solo nel codice, perché il difetto è di interoperabilità e vive fra due implementazioni.
- **Un valore sconosciuto è invalido, non predefinito.** Un default è il modo in cui la divergenza rientra: due implementazioni che sbagliano allo stesso modo su un valore noto si accorgono del problema, due che applicano default diversi a un valore ignoto no.

La scelta dei numeri è dell'implementatore ed è libera, perché nessuna fixture pubblicata la vincola. **Va però motivata**, e la tensione va nominata: se `0` sia lo stato attivo — l'ordine di elencazione, che è ciò che tre implementatori sceglierebbero — oppure un valore riservato che intercetta lo stato non inizializzato. Il Lead non ha una preferenza verificata e non ne impone una.

Serve poi la **fixture pubblicata di `app_leaf`**, che oggi manca, e l'allineamento di `coblox-core` dove la codifica provvisoria diventa definitiva — compresa la rimozione del test che dichiara di non essere una prova.

### 4. Le due frasi di [DEBT-008]

- **RF-109.** La frase secondo cui il pavimento Argon2id rifiuta tutto ciò che è più debole di entrambe le raccomandazioni RFC è più ampia del vero: la forma ad area ammette una banda con `iterations = 1` sotto i 2 GiB quando `memory_kib >= 196608`, che non corrisponde ad alcuna delle due raccomandazioni.
- **RF-110.** La conseguenza secondo cui lo scudo di ammissione costa all'attaccante un indirizzo raggiungibile per ogni slot concorrente vale **solo se** l'emissione dei nonce è conteggiata contro il limite per sorgente del primo passo; il carattere monouso del nonce limita il riuso, non il volume.

Per entrambe la scelta è la stessa e va presa nel merito: **riformulare la frase** perché descriva esattamente ciò che le regole impongono, **oppure cambiare la regola** perché imponga ciò che la frase promette. Vedi *Risks and open decisions* per la conseguenza della seconda strada.

### 5. L'intervallo di blocco

Scrivere nei documenti di protocollo la costante di [ADR-013] — **5 secondi** — come costante di genesi, non come parametro governato, e con essa la parte che una scrittura compiacente ometterebbe: **v0 dichiara la cadenza e non la impone**, perché il solo vincolo su `timestamp_ms` è la mediana degli undici precedenti. La conseguenza è già registrata come [DEBT-013] e **non va risolta qui**: va nominata nel documento e collegata al debito.

## Files and areas involved

- `docs/protocol/README.md` — registro di conformità, fixture di `app_leaf`, intervallo di blocco.
- `docs/protocol/ledger.md` — codifica del ciclo di vita, preimmagine di `app_leaf`, cadenza dichiarata.
- `docs/protocol/identity.md` — le due frasi di [DEBT-008].
- `sim/tools/` — lo strumento versionato dell'inventario, accanto ai due esistenti.
- Sede dell'inventario, da proporre: `docs/protocol/` se è contenuto normativo, `sim/tools/` se è materiale di verifica. La scelta va motivata.
- `core/coblox-core/src/` e `tests/` — allineamento della codifica del ciclo di vita e della fixture nuova.

## Acceptance criteria

- [ ] L'inventario degli artefatti pubblicati esiste, è versionato, ed enumera il perimetro di [ADR-012].
- [ ] La verifica di **completezza** del manifesto esiste e fallisce quando un artefatto presente nei documenti non vi compare. Dimostrato reintroducendo l'omissione.
- [ ] Lo strumento è versionato nel repository, eseguibile in un comando, e la sua esecuzione è nella trascrizione.
- [ ] Lo strumento è **provato in negativo** su ogni classe di difetto che dichiara di intercettare.
- [ ] Gli stati del ciclo di vita del conto di app hanno valori numerici dichiarati nel documento di protocollo, con la motivazione della scelta.
- [ ] Un valore di ciclo di vita sconosciuto è **rifiutato**, e la trascrizione mostra il rifiuto.
- [ ] Esiste una fixture pubblicata di `app_leaf` nel registro di conformità, riprodotta da `coblox-core`.
- [ ] `coblox-core` non contiene più codifiche dichiarate provvisorie per il ciclo di vita, né il test che dichiara di non essere una prova.
- [ ] Le due frasi di [DEBT-008] corrispondono esattamente a ciò che le regole impongono, per riformulazione o per modifica della regola, con la scelta motivata nel merito.
- [ ] L'intervallo di blocco di [ADR-013] è nei documenti di protocollo come costante di genesi, insieme alla dichiarazione esplicita che v0 non ne impone il rispetto, con rimando a [DEBT-013].
- [ ] La domanda generale di [DEBT-012] ha una risposta scritta: **quali preimmagini non hanno una fixture pubblicata, e quali fra queste contengono campi la cui codifica non è fissata altrove.** Un elenco, non una rassicurazione.
- [ ] Nessun hash pubblicato esistente cambia senza che il cambiamento sia dichiarato e ricalcolato dai byte effettivamente scritti.

## Implementation plan

1. Enumerare gli artefatti pubblicati leggendo i quattro documenti, e produrre l'elenco delle preimmagini prive di fixture — cioè rispondere prima alla domanda generale di [DEBT-012], perché è ciò che dimensiona tutto il resto.
2. Decidere la sede e la forma del manifesto, motivando la scelta.
3. Scrivere lo strumento e la verifica di completezza; provarli in negativo su un difetto reintrodotto per ciascuna classe.
4. Fissare la codifica del ciclo di vita nel documento, aggiungere la fixture di `app_leaf`, allineare `coblox-core`.
5. Riformulare le due frasi di [DEBT-008], oppure cambiare la regola, dichiarando quale delle due strade e perché.
6. Scrivere l'intervallo di blocco e la sua non-imposizione.
7. Rieseguire l'intera verifica e incollare le trascrizioni.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-TOOL-NEGATIVE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Per **ogni** classe di difetto che lo strumento dichiara di intercettare, il difetto è reintrodotto e la trascrizione mostra la guardia fallire, poi passare a difetto rimosso. Include l'omissione di un artefatto dal manifesto. È la precisazione n.3 di [ADR-012] e non è negoziabile: uno strumento che non si è mai visto fallire non è evidenza.
- [ ] GATE-FIXTURES-RECOMPUTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il metodo di calcolo è validato riproducendo almeno una fixture **non modificata** prima di calcolare quelle nuove, e ogni hash è calcolato dai byte effettivamente scritti. Incollare entrambe le esecuzioni. È il metodo che il progetto ha già usato tre volte e che il Lead rifarà in modo indipendente.
- [ ] GATE-LIFECYCLE-REJECT | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Un valore di ciclo di vita non assegnato viene **rifiutato**, non interpretato come predefinito, e la trascrizione lo mostra. Un default è il modo in cui la divergenza che questa spec chiude rientrerebbe dalla finestra.
- [ ] GATE-INVENTORY-ANSWER | kind=manual | owner=lead | phase=before-done | evidence=artifact | Il Lead verifica che la domanda generale di [DEBT-012] abbia una risposta enumerata e non una rassicurazione, e ricalcola in modo indipendente almeno un hash nuovo. Le altre due volte in cui il Lead ha verificato così ha trovato qualcosa.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Questa spec potrebbe cambiare una regola di validità, e in tal caso ricade sotto [ADR-012] che essa stessa rende eseguibile.** Accade se per RF-109 si sceglie di restringere il pavimento Argon2id invece di descriverlo, o se per RF-110 si conteggia l'emissione dei nonce contro il limite per sorgente. Non è un paradosso da evitare ma una sequenza da rispettare: l'inventario si costruisce **prima**, poi lo si esegue sulla modifica. Se si sceglie quella strada, la passata va fatta e la trascrizione allegata.
- **Il rischio principale di questa spec è la falsa completezza.** Un inventario che dichiara di coprire tutto e ne copre il novanta per cento è peggio dell'assenza di inventario, perché la gate successiva restituirà un verde più stretto del vero — che è precisamente il difetto per cui [ADR-012] esiste. Se una classe di artefatti non è enumerabile meccanicamente, **va dichiarata come non coperta** e non inclusa in silenzio. Vale la regola *no silent caps*: ciò che resta fuori si scrive.
- **La triplice sede degli hash attesi** — documento, strumento Python, test Rust — è segnalata in *Existing-project analysis* come osservazione del Lead non verificata a fondo. Va affrontata esplicitamente, in un senso o nell'altro, non ignorata.
- **La codifica del ciclo di vita è irreversibile una volta pubblicata una fixture.** È il motivo per cui la scelta va motivata e non presa per ordine di elencazione.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- **Contestare le formulazioni del Lead fa parte del mandato, e vale anche quando l'assunzione arriva dal Lead.** In questa spec sono marcate come tali l'osservazione sulla triplice sede degli hash e l'assenza di preferenza sulla codifica. Tre errori del Lead in questo progetto sono stati trovati dagli specialisti e nessuno da lui; vedi `.lmbrain/knowledge/recurring-defects.md`.
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
