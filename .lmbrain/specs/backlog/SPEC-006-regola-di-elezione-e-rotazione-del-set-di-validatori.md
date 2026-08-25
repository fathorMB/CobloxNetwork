---
id: SPEC-006
# Note: Quote the title if it contains a colon
title: "Regola di elezione e rotazione del set di validatori"
status: backlog
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
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-001, ADR-002, ADR-007, ADR-008]
links: [SPEC-001, SPEC-004, DEBT-005]
created: 2026-08-25
updated: 2026-08-25
tags: [governance, sybil, light-client]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "set effort"
  - date: 2026-08-25
    action: "set tags"
  - date: 2026-08-25
    action: "set tags"
---
# Regola di elezione e rotazione del set di validatori

## Objective

Scrivere nei documenti di protocollo la regola che determina **chi** entra nel set di validatori successivo, chiudendo [DEBT-005]. Oggi `ledger.md` autentica in modo sicuro la transizione da un set al successivo ma non vincola in alcun modo la sua composizione, e in quello spazio vuoto il set corrente è l'unico soggetto che scrive il proprio successore.

## Context

[DEBT-005] è l'unico debito `critical` del progetto e nasce dallo scenario `TM-18` di [SPEC-004]. `ledger.md` lo dichiara apertamente: *"This continuity rule specifies safe authentication but not how members are elected or rotated"*. La conseguenza è che un quorum raggiunto una sola volta può impegnare un successore composto interamente da sé stesso, all'infinito, e la catena resta formalmente valida a ogni passo.

**Perché ora e non in M-07.** La roadmap colloca la rotazione automatica in M-07, e per l'*automazione* è ragionevole. Non lo è per l'*invariante*: una rete che accumuli storia sotto questa regola mancante può chiudersi permanentemente senza che nessuno se ne accorga, e la chiusura **non è reversibile a posteriori**, perché il set insediato controlla ogni transizione futura. [DEBT-005] impone quindi che nessuna devnet accumuli storia conservabile prima che la regola esista — e la devnet è il prodotto centrale di M-02. L'ordine è forzato, non scelto.

**Non sei bloccato da [DEBT-007].** Si potrebbe temere che l'eleggibilità dipenda dalla taratura economica ancora aperta. Non è così: `α` governa la *distribuzione dell'emissione*, l'eleggibilità governa *chi può validare*. Sono meccanismi distinti. Scrivi la regola con soglie e tetto di rotazione come **parametri simbolici dichiarati e limitati**, non come numeri: il simulatore economico di M-02 li riempirà dopo. Se ti accorgessi di un accoppiamento reale che qui non abbiamo visto, fermati e segnalalo invece di inventare un numero.

## Scope
### Included

- La regola di elezione e rotazione nei documenti di protocollo, con la definizione dell'insieme degli eleggibili, la derivazione deterministica, il tetto di rotazione per epoca e l'impegno che ne consente il ricalcolo.
- L'aggiornamento di `ledger.md` nel punto che oggi dichiara la lacuna, e di ogni sezione che vi rimanda.
- La verifica che i test di attacco `AT-09` e `AT-10` di [SPEC-004] passino sulla regola scritta, e l'aggiornamento di `threat-model.md` dove `TM-18` e `SEC-REQ-13` ne risultano coperti.

### Excluded

- **Implementazione in Rust.** Questa spec produce specifica, non codice: come [SPEC-001], il deliverable sono documenti versionati. L'implementazione è spec successiva di M-02.
- Il valore numerico dei parametri, che dipende dal simulatore ([DEBT-007]).
- Lo slashing reputazionale e la governance dei parametri, che la roadmap colloca in M-07 e che qui non servono per l'invariante.
- Qualunque modifica alla regola di revoca, che è già scritta e corretta.

## Existing-project analysis

Il Lead ha letto `ledger.md` §*Validator-set continuity* e §*Revocation forces a validator set transition* prima di redigere. Tre fatti vincolano la soluzione.

**Il problema del light client è già documentato, ed è lo stesso problema.** La sezione sulla revoca contiene una confessione esplicita e la corregge: un light client *"checks set-hash continuity and never sees transactions"*, quindi *"observes a transition if one happens; it cannot establish that a transition was due"*. È esattamente la forma del difetto di [DEBT-005]. Ne segue l'inquadramento che questa spec deve adottare: **la regola di elezione non serve solo a impedire la cattura, serve a rendere il set verificabile come quello *giusto* e non soltanto come uno *continuo*.** Chi progetta la regola guardando solo l'auto-perpetuazione produrrà una regola che i full node possono verificare e i light client no, cioè ripeterà il difetto che quel paragrafo ha già ammesso una volta.

**Il crux è la verificabilità con informazione parziale.** Un light client non vede le transazioni, quindi non può ricalcolare da sé l'insieme degli eleggibili a partire dalle evidenze. Perché la regola sia verificabile anche da lui, **sia la casualità sia l'impegno all'insieme degli eleggibili devono essere raggiungibili dagli header**, così che il controllo si riduca a: dato quell'impegno e quella casualità, il set dichiarato è la derivazione corretta. Il protocollo ha già un precedente utile da riusare invece che reinventare: `existence_income` impegna `eligible_set_root` con foglie `H(0x24 || u64be(reward_epoch) || account_key_32)` in un albero di Merkle su insieme ordinato bytewise, ed è già lo schema di *"insieme calcolabile da chiunque, impegnato in un punto"*.

**La casualità ha già un meccanismo nel protocollo.** Esistono `challenge_commitment` con `(issuer_node_id, commitment_epoch, issuer_commitment)` e una `randomness_source` verificata. Valuta se riusare quel meccanismo o se serva casualità finalizzata di natura diversa: in entrambi i casi la scelta va motivata, perché una casualità che il set uscente può influenzare a proprio favore riapre l'auto-perpetuazione da un'altra porta, in forma più difficile da vedere.

**Vincolo da [ADR-007], non negoziabile qui.** L'eleggibilità a validatore è ancorata a **lavoro difficile da falsificare** — storage e compute dimostrati — e **mai al solo uptime**, che una VPS con SLA batte per costruzione contro qualunque telefono reale. [ADR-001] parla di *"reputazione e uptime dimostrato"*: è formulazione anteriore ad [ADR-007] e va letta come superata su questo punto. Se ritieni che [ADR-007] vada rivisto, non modificarlo: segnalalo al Lead.

**Vincolo da [ADR-008].** L'eleggibilità ancorata a storage e compute deve poter dimostrare di avere un **tetto di fabbisogno**. Il punto 1 del test di [ADR-008] dice che se un nodo può guadagnare di più spendendo di più senza che la rete abbia bisogno di più, è mining. Una soglia di eleggibilità che premi senza limite chi spende di più reintrodurrebbe per la porta di servizio la corsa alla spesa che l'esclusione vieta. Dichiara esplicitamente l'esito dei tre punti del test, come [ADR-008] impone a ogni spec che introduca una forma di lavoro remunerato o premiato.

## Technical proposal

Progetta la regola attorno alle quattro proprietà che [DEBT-005] fissa nei propri criteri di risoluzione, e trattale come requisiti e non come suggerimenti: derivazione **deterministica** a partire da casualità **finalizzata**; insieme degli eleggibili **calcolabile** da chiunque disponga delle informazioni impegnate; **tetto di rotazione per epoca**, perché un ricambio totale improvviso è una superficie di attacco quanto un ricambio nullo; **impegno nell'header** che consenta il ricalcolo a posteriori.

Il tetto di rotazione merita attenzione in entrambe le direzioni. Troppo basso, si torna all'auto-perpetuazione lenta; troppo alto, un avversario che conquista l'eleggibilità in un'epoca ribalta il set in una sola transizione. Dichiara il ragionamento, non solo il parametro.

Definisci anche cosa accade nei casi degeneri, perché sono quelli in cui una regola elegante si rompe: eleggibili in numero inferiore alla dimensione del set; parità nella derivazione; un'epoca senza casualità valida; e l'interazione con la regola di revoca, che può svuotare un set fra un'elezione e la successiva.

## Files and areas involved

- `docs/protocol/ledger.md` — §*Validator-set continuity*, in particolare la riga che dichiara la lacuna, e le sezioni di verifica del light client che vi rimandano.
- `docs/protocol/identity.md` — i due rimandi a `ledger.md#validator-set-continuity`.
- `.lmbrain/knowledge/threat-model.md` — `TM-18`, `SEC-REQ-13`, e i test `AT-09` e `AT-10`.
- Eventuali altri documenti in `docs/protocol/` che assumano la composizione del set: verificali, non presumerli.

## Acceptance criteria
- [ ] `ledger.md` non contiene più l'affermazione che la regola di continuità non specifica come i membri siano eletti o ruotati: al suo posto c'è la regola.
- [ ] La regola è **deterministica**: due verificatori indipendenti con gli stessi input impegnati derivano lo stesso set, e questo è affermato in forma verificabile.
- [ ] L'insieme degli eleggibili è definito in modo **calcolabile da chiunque** disponga delle informazioni impegnate, con criterio ancorato a storage e compute dimostrati e **mai al solo uptime** ([ADR-007]).
- [ ] È dichiarato un **tetto di rotazione per epoca**, come parametro simbolico, con il ragionamento su entrambi gli estremi.
- [ ] Esiste un **impegno** che consente il ricalcolo a posteriori della derivazione.
- [ ] È dichiarato esplicitamente **che cosa può verificare un light client e che cosa no**, senza sopravvalutare la garanzia. Se resta un residuo non verificabile da light client, va scritto come tale: una dichiarazione di sicurezza sbagliata è peggio di una mancante, ed è la posizione che `ledger.md` ha già preso su sé stesso.
- [ ] La casualità usata non è influenzabile a proprio favore dal set uscente, e la ragione è argomentata.
- [ ] I casi degeneri sono coperti: eleggibili insufficienti, parità, epoca senza casualità valida, interazione con la revoca.
- [ ] È dichiarato l'esito dei tre punti del test di [ADR-008], con particolare attenzione al punto 1, il tetto di fabbisogno.
- [ ] `AT-09` e `AT-10` di [SPEC-004] sono valutati contro la regola scritta, con l'esito argomentato; `TM-18` e `SEC-REQ-13` sono aggiornati dove risultano coperti.
- [ ] Nessun parametro numerico è inventato: ciò che dipende dal simulatore resta simbolico e dichiarato come tale.

## Implementation plan
1. Leggere [DEBT-005], `TM-18` e `SEC-REQ-13`, e le due sezioni di `ledger.md` indicate nell'analisi.
2. Definire l'insieme degli eleggibili e il suo impegno, valutando il riuso dello schema `eligible_set_root` già presente.
3. Scegliere e motivare la sorgente di casualità finalizzata.
4. Definire la derivazione deterministica, il tetto di rotazione e l'impegno nell'header.
5. Trattare i casi degeneri e l'interazione con la revoca.
6. Scrivere esplicitamente il perimetro di verifica del light client, residui compresi.
7. Valutare `AT-09` e `AT-10`, aggiornare il threat model.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-DETERMINISM | kind=manual | owner=agent | phase=before-submit | evidence=artifact | La derivazione è illustrata su un esempio numerico completo e riproducibile a mano, dagli input impegnati al set risultante, in modo che un revisore possa rifarlo senza codice. Un esempio che non si può rifare non è una specifica verificabile.
- [ ] GATE-LIGHTCLIENT | kind=manual | owner=agent | phase=before-submit | evidence=artifact | Il documento dichiara in modo esplicito e circoscritto che cosa un light client può stabilire sulla composizione del set e che cosa resta fuori dalla sua portata, nella stessa forma onesta già adottata da `ledger.md` per la revoca.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la regola come superficie di sicurezza e il Lead ha accettato la review. Il debito che questa spec chiude è `critical`: la review di sicurezza non è facoltativa.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio più probabile è progettare per i full node e non per i light client.** È l'errore che `ledger.md` ha già commesso una volta sulla revoca, se n'è accorto, e lo ha corretto scrivendo che una dichiarazione di sicurezza sbagliata è peggio di una mancante. Una regola verificabile solo da chi vede tutte le transazioni lascia in piedi metà del difetto e la fa sembrare risolta. `GATE-LIGHTCLIENT` esiste per questo.
- **Rischio di casualità catturabile.** Se il set uscente può influenzare a proprio favore la sorgente di casualità, l'auto-perpetuazione rientra da una porta più difficile da vedere di quella che stiamo chiudendo. Questa è la parte da attaccare per prima, non per ultima.
- **[ADR-001] è in tensione con [ADR-007] su questo punto** e la spec risolve la tensione a favore del secondo: eleggibilità su lavoro difficile da falsificare, mai sul solo uptime. Se ritieni che la scelta giusta sia l'opposta, **non modificare gli ADR**: argomenta e segnala al Lead, che valuterà se serve una nuova ADR.
- **Aperto, e non lo risolvi qui**: i valori numerici dei parametri. Dipendono dal simulatore economico ([DEBT-007]). Lasciarli simbolici è la risposta corretta, non un compromesso.

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
