---
id: SPEC-013
# Note: Quote the title if it contains a colon
title: "Separazione della chiave di trasporto dalla chiave di identita"
status: ready
kind: feature
priority: high
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: [SPEC-010]
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-015, ADR-012, ADR-014]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [identity, security, conformance]
activity:
  - date: 2026-08-25
    action: "transitioned backlog -> ready"
---
# Separazione della chiave di trasporto dalla chiave di identita

## Objective

Attuare [ADR-015]: la chiave di trasporto libp2p diventa distinta dalla chiave di identità Coblox, subordinata a essa, ruotabile, e il suo legame **non è più pubblicato sul ledger**.

Ha una scadenza dura e non negoziabile: **prima che la devnet emetta il primo certificato di enrollment**. Dopo, non è più questa spec — è una migrazione di identità su una rete con storia.

## Context

`identity.md` §*Key hierarchy* impone oggi che la stessa chiave Ed25519 sia importata in libp2p, così che Peer ID e `node_id` siano due derivazioni della stessa chiave pubblica, verificate entrambe obbligatoriamente. La regola compra una proprietà di sicurezza precisa — *un peer non può sostituire un'identità di trasporto dopo l'enrollment* — e la paga con una proprietà che nessun documento aveva dichiarato: il certificato di enrollment pubblica `libp2p_peer_id` accanto a `node_id`, quindi **il legame fra identità di ledger e indirizzo di rete è un fatto di dominio pubblico e permanente**, disponibile a chi legge il ledger senza connettersi a nulla.

[ADR-015] è precisa su ciò che questa spec ottiene e su ciò che non ottiene, e la precisione va conservata nell'attuazione: **chiude l'osservatore passivo e fuori sessione, non chiude il peer con cui parli.** Il guadagno è un cambio di costo dell'attacco, da lettura gratuita e retroattiva a partecipazione attiva e contemporanea. Chi scrive questa spec non deve dichiarare TM-28 chiuso, in nessun documento.

## Scope

### Included

- La riscrittura di `identity.md` §*Key hierarchy* e §*Authentication on a connection*.
- L'oggetto di attestazione che lega l'identità enrollata alla chiave di trasporto, con schema, serializzazione canonica e preimmagini.
- La rimozione di `libp2p_peer_id` dalla richiesta di enrollment e dal certificato, con le preimmagini e le fixture che ne discendono.
- Ciò che si presenta e quando, in `wire.md`.
- La regola di validità che conserva la proprietà comprata dalla vecchia regola.
- L'allineamento di `coblox-core` e del registro di conformità.

### Excluded

- **Il Circuit Relay v2 obbligatorio.** È l'altra metà del rimedio a TM-28 e [ADR-015] la esclude esplicitamente: è una scelta di topologia con conseguenze su latenza e carico che richiedono una devnet per essere misurate.
- **Rendere anonimo il gossip applicativo.** `wire.md` vieta la modalità autore anonimo di libp2p e la ADR conferma il rifiuto: distruggerebbe l'attribuzione su cui poggiano validazione, backpressure e ogni difesa anti-spam.
- Le chiavi di consenso dei validatori, che sono già subordinate e restano come sono.

## Existing-project analysis

**Verificato dal Lead il 2026-08-25:**

- `identity.md` §*Key hierarchy* dice che la stessa chiave è importata in libp2p e che le implementazioni **MUST** verificare entrambe le derivazioni.
- La chiave di consenso del validatore è già una chiave subordinata, legata da una prova di possesso all'identità enrollata, e non è una seconda identità. **È il modello da seguire**, e [ADR-015] lo dice: il costrutto esiste già nel protocollo e non se ne inventa uno nuovo.
- `libp2p_peer_id` compare nello schema della **richiesta** di enrollment e in quello del **certificato**, quindi in due preimmagini pubblicate.
- §*Canonical libp2p Peer ID* impone la forma base58btc legacy negli oggetti firmati e vieta di riserializzare una forma CID prima della verifica della firma; c'è una fixture di conformità con il protobuf canonico e il Peer ID atteso.
- §*Authentication on a connection* impone al ricevente di ottenere il certificato e confermare che *la chiave pubblica del certificato derivi il Peer ID libp2p autenticato*. **È la riga che questa spec deve sostituire, non cancellare.**
- `wire.md` §*Gossip validation and backpressure* ancora le code al peer, e rimanda a `identity.md` per i limiti dello stream di enrollment, che è l'unico ad accettare peer di trasporto non autenticati.

## Technical proposal

### 1. L'attestazione, sul modello della chiave di consenso

Un oggetto firmato dalla chiave di identità che autorizza una chiave di trasporto. Deve stabilire, per chi lo riceve, che **quella** identità ha autorizzato **quella** chiave di trasporto, e non un'altra.

Tre proprietà che il Lead ritiene necessarie e su cui chiede una posizione esplicita, non un'assunzione silenziosa:

- **Validità limitata nel tempo o in altezze di catena.** Un'attestazione perpetua rende una chiave di trasporto trapelata valida per sempre, e la revoca oggi opera sull'identità: revocare l'identità per una chiave di trasporto compromessa sarebbe un rimedio sproporzionato. La forma del limite è una scelta di progetto e va motivata.
- **Legame con la rete**, coerente con il resto del protocollo, dove ogni oggetto firmato porta il `network_id`.
- **Nessuna pubblicazione.** L'attestazione vive in sessione. Se una qualunque parte del progetto la scrivesse sul ledger, l'intera spec sarebbe stata inutile.

### 2. Perché un terzo non può riusare un'attestazione altrui

Va **scritto come argomento**, non lasciato dedurre: chi presenta un'attestazione deve anche completare l'handshake Noise o QUIC con la chiave di trasporto che l'attestazione nomina, quindi dimostrare il possesso della corrispondente privata. Un terzo che intercetti l'attestazione non può usarla.

L'implementatore verifichi che l'argomento regga nella forma esatta in cui scrive il protocollo, e **lo contesti se non regge**: è la proprietà su cui poggia tutto il resto.

### 3. La regola di validità che sostituisce la doppia derivazione

*Un peer non può sostituire un'identità di trasporto dopo l'enrollment* diventa: **nessun peer può presentarsi con una chiave di trasporto priva di un'attestazione valida dell'identità che dichiara.** Verifica obbligatoria, rifiuto in assenza, con la stessa forza normativa che ha oggi la doppia derivazione. Una separazione che indebolisse questa proprietà sarebbe un peggioramento netto, non un compromesso.

### 4. La rotazione, e ciò che tocca

[ADR-015] segnala questo come **il punto più probabile di un difetto**, e va trattato come tale: una chiave di trasporto ruotabile è una chiave che **azzera lo stato per peer**. Code, backpressure e difese anti-spam di `wire.md` sono ancorate al peer di trasporto, e vanno confrontate con lo scudo di ammissione di [ADR-007].

Lo stream di enrollment è meno esposto, perché accetta già peer non autenticati e i suoi limiti sono ancorati alla chiave e alla sorgente. **Il resto del gossip no**, ed è lì che va guardato.

Se ne discende un limite alla frequenza di rotazione, va scritto come regola e non come raccomandazione — e va detto quanta privacy quel limite costa, perché una rotazione troppo rara è una pseudonimia stabile con passi in più.

### 5. Ciò che smette di funzionare come prima

`libp2p_peer_id` esce da due schemi pubblicati. Va verificato **che cosa lo stava usando**: in particolare come un peer trova e riconosce un nodo, se qualcosa risolveva `node_id` verso un indirizzo passando dal ledger, e cosa prende il suo posto. Il Lead non ha tracciato tutti i chiamanti e lo dichiara: è la prima cosa da fare, e potrebbe ridimensionare o allargare questa spec.

## Files and areas involved

- `docs/protocol/identity.md` — gerarchia delle chiavi, Peer ID canonico, autenticazione sulla connessione, schemi di richiesta e certificato.
- `docs/protocol/wire.md` — cosa si presenta e quando, backpressure, rotazione.
- `docs/protocol/README.md` — schemi, preimmagini, registro di conformità.
- `core/coblox-core/src/registry.rs`, `hash.rs` — separazione di dominio e preimmagine dell'oggetto nuovo.
- `core/coblox-core/tests/` — le fixture che cambiano e quelle nuove.
- `.lmbrain/knowledge/threat-model.md` — TM-28 va aggiornato con ciò che questa spec cambia e con ciò che **non** cambia.

## Acceptance criteria

- [ ] `identity.md` §*Key hierarchy* è **riscritta**, non annotata: la frase sulla doppia derivazione verificata obbligatoriamente non sopravvive in nessuna forma in nessun documento.
- [ ] Esiste l'oggetto di attestazione, con schema, serializzazione canonica, preimmagine a dominio separato e fixture pubblicata.
- [ ] L'attestazione ha una validità limitata, con la forma del limite motivata.
- [ ] `libp2p_peer_id` non compare più nella richiesta di enrollment né nel certificato.
- [ ] La regola di validità che conserva la proprietà della vecchia regola è scritta con la stessa forza normativa, e la trascrizione mostra il **rifiuto** di un peer privo di attestazione valida.
- [ ] L'argomento sull'impossibilità di riusare un'attestazione altrui è scritto nel documento, non lasciato dedurre.
- [ ] L'interazione fra rotazione e backpressure è affrontata esplicitamente: o si dimostra che non degrada le difese, o si scrive il limite che la governa.
- [ ] È verificato e riportato che cosa usava `libp2p_peer_id` sul ledger, e cosa prende il suo posto.
- [ ] Ogni hash e fixture che cambia è ricalcolato dai byte effettivamente scritti, con il metodo validato prima su una fixture non modificata.
- [ ] `coblox-core` è allineato e riproduce le fixture nuove.
- [ ] TM-28 nel threat model è aggiornato con ciò che cambia **e con ciò che resta**, e in nessun punto risulta dichiarato chiuso.
- [ ] La gate di [ADR-012] è eseguita con lo strumento di [SPEC-010] e la trascrizione è allegata.

## Implementation plan

1. Tracciare tutti gli usi di `libp2p_peer_id` nei documenti e nel codice, e riportare che cosa dipende dalla sua pubblicazione. **Prima di ogni altra cosa**: può ridimensionare o allargare la spec.
2. Progettare l'attestazione sul modello della chiave di consenso, prendendo posizione sulle tre proprietà.
3. Scrivere la regola di validità sostitutiva e l'argomento anti-riuso.
4. Affrontare rotazione e backpressure, con la conclusione scritta in un senso o nell'altro.
5. Aggiornare schemi, preimmagini e fixture; ricalcolare gli hash con il metodo validato.
6. Allineare `coblox-core`, aggiornare TM-28, eseguire la gate di [ADR-012].

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-NO-ATTESTATION-REJECTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Un peer che presenta una chiave di trasporto priva di attestazione valida è **rifiutato**, e la trascrizione lo mostra. È la proprietà che la vecchia regola comprava, e una separazione che la indebolisse sarebbe un peggioramento netto: la gate esiste per rendere impossibile chiudere la spec senza averla esibita.
- [ ] GATE-NO-PUBLISHED-LINK | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una ricerca su tutti i documenti di protocollo e su `coblox-core` mostra che nessun oggetto pubblicato sul ledger contiene più il legame fra identità e chiave di trasporto. Se ne resta uno solo, la spec non ha ottenuto nulla.
- [ ] GATE-FIXTURES-RECOMPUTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il metodo di calcolo è validato riproducendo una fixture **non modificata** prima di ricalcolare quelle che cambiano, e ogni hash nuovo è calcolato dai byte effettivamente scritti. Incollare entrambe le esecuzioni.
- [ ] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato di [SPEC-010] e la trascrizione è allegata. Questa spec cambia preimmagini pubblicate: è il caso esemplare per cui la gate esiste.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la separazione e il Lead ha accettato la review. Nel dispatch va detto esplicitamente che **le superfici segnalate qui non sono il perimetro**: tre volte in questo progetto la reviewer le ha trovate solide e ha trovato i difetti altrove.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è ottenere la privacy indebolendo l'anti-spam.** È la conseguenza che [ADR-015] segnala come più probabile fonte di difetto: una chiave ruotabile azzera lo stato per peer. Va risolta, non aggirata, e la soluzione va scritta come regola.
- **La tentazione di dichiarare TM-28 chiuso.** [ADR-015] lo vieta nelle proprie condizioni di revisione: la decisione sposta il costo dell'attacco e non lo elimina, e registrarla come chiusura sarebbe la prima riga di una quinta occorrenza già vista. Vale per ogni documento che questa spec tocca, threat model compreso.
- **La rimozione di `libp2p_peer_id` potrebbe rompere qualcosa che il Lead non ha tracciato.** È dichiarato in *Existing-project analysis*, ed è il primo passo del piano proprio per questo.
- **La finestra si chiude alla prima devnet.** Se questa spec dovesse crescere oltre ciò che è ragionevole in una passata, la risposta corretta non è rimandarla: è riportarlo al Lead perché sia scomposta, con la parte che tocca le preimmagini fatta **prima** della devnet e il resto dopo.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- **Contestare le formulazioni del Lead fa parte del mandato.** In questa spec sono marcate come non verificate: gli usi di `libp2p_peer_id` che il Lead non ha tracciato, e l'argomento anti-riuso, che va confermato nella forma esatta in cui il protocollo viene scritto.
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
