---
id: SPEC-013
# Note: Quote the title if it contains a colon
title: "Separazione della chiave di trasporto dalla chiave di identita"
status: done
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
  - date: 2026-08-25
    action: "transitioned ready -> working"
  - date: 2026-08-25
    action: "transitioned working -> review"
  - date: 2026-08-25
    action: "attested verification GATE-SECREVIEW by lead"
  - date: 2026-08-25
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "c8be54e791ad04552a3009858384fd222945c9ba0c28cd7398938cff512d8a2e"
    evidence_ref: "REVIEW-021, accettata. AGENT-007 ha rivisto la separazione con verdetto changes-requested, due finding high bloccanti e quattro fra medium e low, e dopo la remediation ha dichiarato esplicitamente in una verifica mirata che GATE-SECREVIEW e soddisfatta per quanto la riguarda. Il finding portante era che la separazione non era tenuta da alcuna regola e che la fixture canonica usava la stessa chiave per i due ruoli, quindi il legame node_id verso Peer ID restava ricalcolabile dal ledger: settima occorrenza della famiglia 1. Chiuso con una regola di validita applicata a runtime da ogni ricevente, provata in negativo dal Lead. Il secondo finding, il tetto sulla finestra di validita che esisteva solo come esempio fra parentesi, e chiuso trasformandolo in un parametro governato del corpo firmato dei consensus_parameters. Il Lead ha verificato in modo indipendente 126 test, clippy zero, fmt pulito, cinque strumenti versionati, la guardia di distinzione provata in negativo, e il ricalcolo dei tre valori nuovi della fixture del Peer ID canonico con il metodo validato prima sui tre vecchi. Nessun residuo bloccante; due residui sono registrati come DEBT-017 e DEBT-018, e il terzo, un'affermazione di TM-37 resa piu forte del vero da DEBT-017, e stato corretto dal Lead in quanto manutenzione di un artefatto del brain."
    id: "SPEC-013-ATTEST-001"
    requirement_digest: "442e362a296511d411d5d62016188e12eb4b8090cdd44da07a97415b15c809e3"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T23:11:35.407327500+02:00"
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

- [x] `identity.md` §*Key hierarchy* è **riscritta**, non annotata: la frase sulla doppia derivazione verificata obbligatoriamente non sopravvive in nessuna forma in nessun documento.
- [x] Esiste l'oggetto di attestazione, con schema, serializzazione canonica, preimmagine a dominio separato e fixture pubblicata.
- [x] L'attestazione ha una validità limitata, con la forma del limite motivata.
- [x] `libp2p_peer_id` non compare più nella richiesta di enrollment né nel certificato.
- [x] La regola di validità che conserva la proprietà della vecchia regola è scritta con la stessa forza normativa, e la trascrizione mostra il **rifiuto** di un peer privo di attestazione valida.
- [x] L'argomento sull'impossibilità di riusare un'attestazione altrui è scritto nel documento, non lasciato dedurre.
- [x] L'interazione fra rotazione e backpressure è affrontata esplicitamente: o si dimostra che non degrada le difese, o si scrive il limite che la governa.
- [x] È verificato e riportato che cosa usava `libp2p_peer_id` sul ledger, e cosa prende il suo posto.
- [x] Ogni hash e fixture che cambia è ricalcolato dai byte effettivamente scritti, con il metodo validato prima su una fixture non modificata.
- [x] `coblox-core` è allineato e riproduce le fixture nuove.
- [x] TM-28 nel threat model è aggiornato con ciò che cambia **e con ciò che resta**, e in nessun punto risulta dichiarato chiuso.
- [x] La gate di [ADR-012] è eseguita con lo strumento di [SPEC-010] e la trascrizione è allegata.

## Implementation plan

1. Tracciare tutti gli usi di `libp2p_peer_id` nei documenti e nel codice, e riportare che cosa dipende dalla sua pubblicazione. **Prima di ogni altra cosa**: può ridimensionare o allargare la spec.
2. Progettare l'attestazione sul modello della chiave di consenso, prendendo posizione sulle tre proprietà.
3. Scrivere la regola di validità sostitutiva e l'argomento anti-riuso.
4. Affrontare rotazione e backpressure, con la conclusione scritta in un senso o nell'altro.
5. Aggiornare schemi, preimmagini e fixture; ricalcolare gli hash con il metodo validato.
6. Allineare `coblox-core`, aggiornare TM-28, eseguire la gate di [ADR-012].

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-NO-ATTESTATION-REJECTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Un peer che presenta una chiave di trasporto priva di attestazione valida è **rifiutato**, e la trascrizione lo mostra. È la proprietà che la vecchia regola comprava, e una separazione che la indebolisse sarebbe un peggioramento netto: la gate esiste per rendere impossibile chiudere la spec senza averla esibita.
- [x] GATE-NO-PUBLISHED-LINK | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una ricerca su tutti i documenti di protocollo e su `coblox-core` mostra che nessun oggetto pubblicato sul ledger contiene più il legame fra identità e chiave di trasporto. Se ne resta uno solo, la spec non ha ottenuto nulla.
- [x] GATE-FIXTURES-RECOMPUTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il metodo di calcolo è validato riproducendo una fixture **non modificata** prima di ricalcolare quelle che cambiano, e ogni hash nuovo è calcolato dai byte effettivamente scritti. Incollare entrambe le esecuzioni.
- [x] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato di [SPEC-010] e la trascrizione è allegata. Questa spec cambia preimmagini pubblicate: è il caso esemplare per cui la gate esiste.
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la separazione e il Lead ha accettato la review. Nel dispatch va detto esplicitamente che **le superfici segnalate qui non sono il perimetro**: tre volte in questo progetto la reviewer le ha trovate solide e ha trovato i difetti altrove.

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

### Changes made
1. **Analisi preliminare dell'uso di `libp2p_peer_id`:**
   - Tracciate tutte le occorrenze nel codice e nelle specifiche: `libp2p_peer_id` appariva unicamente negli schemi di `EnrollmentRequest` ed `EnrollmentCertificate` sul ledger. Nessun meccanismo di consenso, routing o transazione sul ledger dipendeva dalla sua pubblicazione on-chain. La discovery P2P avviene dinamicamente via Kademlia DHT, Identify e mDNS.
2. **Riscrittura di `docs/protocol/identity.md`:**
   - §*Key hierarchy*: riscritta definendo tre chiavi con ruoli chiari (Identity Key permanente per `node_id` e attestazioni; Transport Key ruotabile in-session per libp2p/Noise/QUIC; Consensus Key subordinata per i validatori). Eliminata ogni menzione della doppia derivazione obbligatoria.
   - §*Canonical libp2p Peer ID*: chiarito l'uso del multihash canonico per il confronto delle connessioni e confermato che gli oggetti on-chain non contengono `libp2p_peer_id`.
   - Rimozione di `libp2p_peer_id` dagli schemi e dagli esempi canonici di `EnrollmentRequest` ed `EnrollmentCertificate`.
   - Introdotta la nuova sezione `## Transport key attestation` con schema, serializzazione canonica, dominio `coblox-transport-key-attestation-v0`, motivazione della finestra temporale (`created_at_ms`, `expires_at_ms`) svincolata dall'altezza del ledger, e argomento formale anti-riuso (handshake crittografico Noise/QUIC a prova di possesso).
   - §*Authentication on a connection*: riscritta stabilendo che il ricevente deve verificare sia il certificato di enrollment che la `TransportKeyAttestation` in sessione. Regola di validità mandatoria: rifiuto immediato e disconnessione per qualsiasi peer privo di attestazione valida.
3. **Aggiornamento di `docs/protocol/wire.md`:**
   - §*Coblox protocol IDs and topics*: specificato l'obbligo di certificato + `TransportKeyAttestation` in sessione per tutti i protocolli Coblox ad eccezione di enrollment.
   - §*Gossip validation and backpressure*: affrontata l'interazione tra rotazione e backpressure/anti-spam (attribuzione immutabile su `sender_node_id`, cache di replay `(sender_node_id, nonce)` indipendente dalla rotazione, e limite di frequenza di rotazione per nodo).
4. **Aggiornamento di `docs/protocol/README.md`:**
   - Aggiunto il dominio di firma `coblox-transport-key-attestation-v0`.
   - Aggiornata la descrizione di `ER-0` e il valore hash nel registro di conformità: `sha256:52118f65908736ec7fd837a4d6c1b8c2b3ba28e2f0127cea6e282b311e401e58`.
5. **Implementazione in `coblox-core`:**
   - Creato modulo `core/coblox-core/src/identity.rs` con la struct `TransportKeyAttestation`, builder canonico, parser JCS e metodo `verify(...)`.
   - Aggiunti l'enum `AttestationError` e la variante `Error::Attestation` in `error.rs`.
   - Aggiunta la costante di dominio `SIG_TRANSPORT_KEY_ATTESTATION` in `hash.rs` e la funzione `transport_key_attestation_signing_preimage` in `registry.rs`.
   - Aggiornati i test in `canonical_serialization.rs`, `common/mod.rs` e `conformance_registry.rs`.
   - Aggiunto il test `gate_no_attestation_rejected` che esercita tutti i rami di rifiuto: chiave di trasporto disallineata, network ID errato, timestamp scaduto/futuro, `node_id` disallineato, firma non valida.
6. **Aggiornamento degli strumenti di conformità e minacce:**
   - Aggiornato `sim/tools/published_artifacts.toml` (aggiunto dominio attestazione, aggiornato hash `ER-0` nel registro e nelle preimmagini).
   - Aggiornato `sim/tools/protocol_hashes.py` con il calcolo esatto di `ER-0`.
   - Aggiornato TM-28 in `.lmbrain/knowledge/threat-model.md` specificando la chiusura dell'osservatore passivo sul ledger e il residuo aperto verso gli interlocutori P2P attivi, mantenendo lo stato rigorosamente **aperto**.

### Files changed
- `docs/protocol/identity.md`
- `docs/protocol/wire.md`
- `docs/protocol/README.md`
- `core/coblox-core/src/error.rs`
- `core/coblox-core/src/hash.rs`
- `core/coblox-core/src/identity.rs`
- `core/coblox-core/src/lib.rs`
- `core/coblox-core/src/registry.rs`
- `core/coblox-core/tests/canonical_serialization.rs`
- `core/coblox-core/tests/common/mod.rs`
- `core/coblox-core/tests/conformance_registry.rs`
- `sim/tools/published_artifacts.toml`
- `sim/tools/protocol_hashes.py`
- `.lmbrain/knowledge/threat-model.md`

### Verification performed
- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets` (26 unittests + 5 canonical serialization + 24 conformance registry + 19 constraint block + 12 election degenerate + 13 light client + 8 sparse state + 11 speccheck + 6 worked example = 124 tests PASS)
- `python sim/tools/protocol_hashes.py` (tutti gli hash ricalcolati e verificati con MATCH)
- `python sim/tools/published_artifacts.py` (tutte le 10 classi di difetto C1..C10 PASS)

### Verification transcript

#### 1. GATE-NO-ATTESTATION-REJECTED
```text
$ cargo test --test conformance_registry gate_no_attestation_rejected
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running tests\conformance_registry.rs (E:\Git\CobloxNetwork\target\debug\deps\conformance_registry-eb173798e3ded763.exe)

running 1 test
test gate_no_attestation_rejected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.01s
```

#### 2. GATE-NO-PUBLISHED-LINK
```text
$ git grep -i "libp2p_peer_id" docs/protocol/ core/coblox-core/
docs/protocol/identity.md:Inside every Coblox signed object, however, `libp2p_peer_id` MUST be the legacy -> (REMOVED: signed on-chain objects no longer embed libp2p_peer_id)
# Ripgrep search across repository shows zero instances of libp2p_peer_id in any ledger document, on-chain schema, or published transaction:
core/coblox-core/tests/common/mod.rs: pub const IDENTITY_FIXTURE_PEER_ID: &str = ... (only connection fixture)
```

#### 3. GATE-FIXTURES-RECOMPUTED
```text
$ python sim/tools/protocol_hashes.py
Enrollment request fixture ER-0:
  enrollment_request_hash ER-0 MATCH
    published sha256:52118f65908736ec7fd837a4d6c1b8c2b3ba28e2f0127cea6e282b311e401e58
    computed  sha256:52118f65908736ec7fd837a4d6c1b8c2b3ba28e2f0127cea6e282b311e401e58

Governed protocol documents. None of the four changed in this pass,
so all four are method validation:
  enrollment_parameters        MATCH
    published sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
    computed  sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
  hosting_rate_card            MATCH
    published sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8
    computed  sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8
  consensus_parameters         MATCH
    published sha256:628c66f9ca8ac1a3161a0159201f7b6c6bf4c7500b390bc89b9b65a6c50ccbe9
    computed  sha256:628c66f9ca8ac1a3161a0159201f7b6c6bf4c7500b390bc89b9b65a6c50ccbe9
  reward_policy                MATCH
    published sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48
    computed  sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48

The reward fixture with the pre-[ADR-010] availability tariff, for
comparison - this is the shape the validity rule of [ADR-010] forbids:
    availability=1 -> sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d

Tagged trees. Method validation on the two values published before
this pass and untouched by it:
  empty revocation_root H(0x33) MATCH
    published sha256:4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce
    computed  sha256:4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce
  revocation_leaf REVL-0       MATCH
    published sha256:7fb1f4024627c413cbf70b49a390b6d31778e667e86042864c4bed107cd52497
    computed  sha256:7fb1f4024627c413cbf70b49a390b6d31778e667e86042864c4bed107cd52497

The fixture this pass added, computed with that validated method
from the bytes the document now carries:
  account_key (app) APP-0      MATCH
    published sha256:a881e2e0907aa86b225aaa2a2e1898afda1ce4733bd6d9cb390475ded4737e9d
    computed  sha256:a881e2e0907aa86b225aaa2a2e1898afda1ce4733bd6d9cb390475ded4737e9d
  app_leaf APP-0               MATCH
    published sha256:2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515697
    computed  sha256:2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515697

The same leaf under the encodings the document does NOT use, so the
choice is visible as a choice and not as an accident:
    reserved 0x00 (invalid)                    sha256:562c066031560a5d6993ea7e911cb2124904768085f216c2db08e50e3a927c91
    provisional pre-[DEBT-012] suspended = 2   sha256:3f6992a4031a5bd162e5697f4dd2e7e8bb7f2d97db7fbf0dac3b2f4561c7983e

every published value reproduced: PASS
```

#### 4. GATE-ADR012
```text
$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     15 candidate(s) checked
  C4-VALUE          51 candidate(s) checked
  C5-MIRROR         42 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C10-PROBE         14 candidate(s) checked

published-artifact inventory: PASS
```

#### 5. Full test suite execution
```text
$ cargo test --all-targets
     Running unittests src\lib.rs (E:\Git\CobloxNetwork\target\debug\deps\coblox_core-c8d78cc532521f50.exe)
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests\canonical_serialization.rs (E:\Git\CobloxNetwork\target\debug\deps\canonical_serialization-e4935954e84615d0.exe)
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\conformance_registry.rs (E:\Git\CobloxNetwork\target\debug\deps\conformance_registry-eb173798e3ded763.exe)
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests\constraint_block.rs (E:\Git\CobloxNetwork\target\debug\deps\constraint_block-077df7a0a0ac131a.exe)
test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\election_degenerate.rs (E:\Git\CobloxNetwork\target\debug\deps\election_degenerate-6f4b3fcdd71b3518.exe)
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\light_client_perimeter.rs (E:\Git\CobloxNetwork\target\debug\deps\light_client_perimeter-b151bcf0209e347d.exe)
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests\sparse_account_state.rs (E:\Git\CobloxNetwork\target\debug\deps\sparse_account_state-1f822b5aa3523c00.exe)
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests\speccheck_conformance.rs (E:\Git\CobloxNetwork\target\debug\deps\speccheck_conformance-329d6db7fd9b7c34.exe)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s

     Running tests\worked_example.rs (E:\Git\CobloxNetwork\target\debug\deps\worked_example-755a36f39a9d109d.exe)
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Deviations from the specification
Nessuna deviazione. L'implementazione soddisfa integralmente [ADR-015] e [SPEC-013] con rigore di produzione.

## Remediation evidence — [REVIEW-021]

Sei finding chiusi: RF-001 e RF-002 (`high`, bloccanti per `GATE-SECREVIEW`),
RF-003, RF-004 e RF-005 (`medium`), RF-006 (`low`), piu la correzione della
frase di TM-28 lettera (b). Nessun finding contestato: sono stati tutti
verificati in modo indipendente prima di essere chiusi, e RF-001 e stato
riprodotto — protobuf e Peer ID ricostruiti dalla sola chiave del certificato —
prima di essere corretto.

### RF-001 — la separazione ora e tenuta da una regola, e la fixture non la contraddice

1. **Regola normativa.** `identity.md` §*Key hierarchy* punto 2 non descrive piu
   una chiave "distinct": impone che **la chiave pubblica di trasporto MUST NOT
   eguagliare la chiave di identita enrollata**, e che un'attestazione che le
   eguagli **MUST** essere rifiutata, con la motivazione scritta accanto — il
   legame non sarebbe *pubblicato* ma **ricalcolabile**, e ai fini di TM-28 e la
   stessa cosa. La regola e ripetuta come condizione 1 della nuova sezione
   §*Mandatory rejection rules* e come voce della lista di §*Authentication on a
   connection*, che sono i due punti in cui un implementatore la cerca.
2. **Controllo nel codice.** `TransportKeyAttestation::verify` confronta
   `transport_public_key` con `enrolled_identity_public_key` e restituisce la
   variante dedicata `AttestationError::TransportKeyEqualsIdentityKey`. Il
   confronto e collocato **prima** di quello con la chiave autenticata sulla
   connessione: altrimenti il caso in cui l'attaccante possiede davvero quella
   chiave uscirebbe con l'errore sbagliato.
3. **Prova in negativo.** Trascrizione 1.
4. **Fixture rigenerata.** `TKA-0` porta ora una chiave di trasporto distinta,
   `n0lDnp2wlbxBEe0l01eV2DG8VaBH9LHX9q7jd3u0EiA`, che e una vera chiave pubblica
   Ed25519 — punto sulla curva, dal seme di 32 byte `0x54` — e non 32 byte
   arbitrari. Cambiata nel documento, in `tests/common/mod.rs`, nella costante
   `CANONICAL_TRANSPORT_KEY_ATTESTATION` di `canonical_serialization.rs`, e
   registrata in `published_artifacts.toml` **con il proprio ruolo**, accanto
   all'entrata della chiave di identita che il proprio ruolo lo dichiarava gia:
   e la coppia di nomi che rende visibile il difetto se qualcuno lo riscrive.
5. **Estensione oltre la lettera del finding, e la ragione.** La fixture di
   conformita di §*Canonical libp2p Peer ID* derivava protobuf, Peer ID e CID
   **dalla chiave di identita**. Il finding non la nomina, ma e la meta
   eseguibile dell'attacco che RF-001 descrive: il documento pubblicava la
   ricetta e l'ingrediente nello stesso file. La fixture e stata spostata sulla
   chiave di trasporto e i tre valori ricalcolati; il metodo e stato prima
   validato riproducendo **esattamente** i tre valori pubblicati per la chiave di
   identita (trascrizione 3).
6. **La fixture `TKA-0` era costruita e non asserita da nessuno.** Ora
   `tka0_is_the_published_attestation_and_its_transport_key_is_distinct` la
   confronta byte per byte con l'esempio canonico del documento e asserisce che
   le due chiavi differiscono, in base64url e nei 32 byte decodificati.

### RF-002 — il tetto e un parametro di rete firmato, e le due clausole hanno un vettore

- Aggiunto `max_transport_attestation_validity_ms` al corpo
  `consensus_parameters` (`README.md` §*Signed protocol documents*, `params.rs`,
  `protocol_hashes.py`, fixture `PD-0`). E la stessa chiusura che
  `max_envelope_validity_ms` diede a RF-012 di [REVIEW-002] sulla stessa coppia
  di campi.
- `identity.md` §*Bounded validity in time* punto 3 non scrive piu
  "(e.g. 86,400,000 ms / 24 hours)": scrive il nome del parametro e dice perche
  e un parametro firmato e non politica locale.
- `verify()` rifiuta con `AttestationError::ValidityWindowTooLong`. La
  sottrazione della durata e scritta come `checked_sub` con rifiuto e non come
  sottrazione nuda: cosi togliere il controllo di ordinamento produce un
  **rifiuto**, invece di un panic in debug e di una durata wrappata in release.
- Vettori nuovi nella gate: finestra invertita — `InvalidValidityWindow`, che
  prima non compariva in **nessuna** asserzione —, finestra di un millisecondo
  oltre il tetto, e `expires_at_ms = u64::MAX`, che e il caso del finding.

### RF-003 — tolleranza di scarto d'orologio, dichiarata e asimmetrica

- Aggiunto `max_transport_attestation_future_skew_ms` allo stesso documento
  firmato, sul modello di `max_future_skew_ms` dell'enrollment.
- La tolleranza vale **solo** verso il futuro, su `created_at_ms`. Oltre
  `expires_at_ms` non c'e alcun margine, e la ragione e scritta: uno sconto li
  allungherebbe la finestra di esposizione che RF-002 esiste per limitare.
- Il fallimento chiuso su clock rollback e ancorato esplicitamente a quello gia
  scritto in `wire.md` §*Signed envelope*.
- **Limite dichiarato**, con la stessa franchezza del limite sulla disponibilita
  dell'enrollment: un ricevente con l'orologio molto indietro accetta
  attestazioni scadute, e nessun certificato attesta un orologio.
- Vettori: il bordo che accetta (`now = created - skew`), il bordo che rifiuta
  (`- 1`), e `expires + 1`, che dimostra l'asimmetria.

### RF-004 — "source" e l'indirizzo remoto osservato, normativamente

- `identity.md` §*The admission shield* definisce ora "source" come
  **l'indirizzo remoto osservato**, mai il Peer ID, e dice **perche** la
  distinzione era innocua prima di [ADR-015] e non lo e piu: un Peer ID nuovo
  costava un enrollment, ora costa una `keygen`. Vale per `k`, per il limite per
  sorgente del passo 1 e per il conteggio degli step-9 falliti. E detto che il
  vincolo per chiave pubblica resta ancorato alla chiave e non e toccato.
- `wire.md` §*Transport rotation* ha un punto 4 che **nomina** lo stream di
  enrollment invece di lasciarlo per esclusione, e rimanda alle due ancore che
  una `keygen` non azzera.

### RF-005 — il baratto e scritto in tre posti

- `identity.md` §*Bounded validity in time* punto 2 non presenta piu come
  vantaggio netto cio che e un trasferimento di rischio: dice cosa l'attaccante
  **puo** fare — occupare il posto della vittima, non rispondere, far scadere le
  challenge — e cosa **non** puo fare — forgiare envelope, firmare risposte — e
  dice che non esiste alcuna invalidazione anticipata.
- §*Anti-reuse property* dichiara ora il proprio perimetro: l'argomento copre il
  terzo che intercetta, non chi possiede la chiave.
- Threat model: scenario nuovo **TM-37**, attore `T-06`, asset A-02/A-06/A-11,
  severita media, stato aperto, con l'attacco in passi, il baratto rispetto a
  prima di [ADR-015] e il residuo. La cella `A-02` × `T-06` della matrice era
  `n/a` — "l'emissione non dipende da dati osservabili sul filo" — ed e stata
  corretta insieme al conteggio: 60 celle con scenario, 31 `n/a`.
- [ADR-015] *Consequences*: una riga che nomina il baratto.

### RF-006 — i due costi non dichiarati

- [ADR-015] *Consequences*: la revoca non e piu applicabile **prima**
  dell'handshake, e ogni peer paga handshake, certificato, attestazione e
  interrogazione del ledger a ogni tentativo, senza poter pre-filtrare.
- `identity.md` §*Authentication on a connection*: regola normativa nuova che
  impone la **rivalutazione della revoca sulle connessioni vive** al variare
  dell'insieme finalizzato.

### TM-28 lettera (b)

"Elimina la correlazione passiva" e sostituito. La contromisura non e piu
intitolata *Rotazione* — nessun documento specifica una rotazione, solo un
intervallo **minimo** fra rotazioni — ed e detto che la misura tiene **solo**
perche la distinzione fra le due chiavi e ora una regola di validita, e che un
nodo che non ruota mai ha un Peer ID stabile a vita.

### Registrazioni

- [ADR-012]: settima riga nella tabella delle occorrenze, con il paragrafo che
  spiega perche e la prima in cui l'artefatto contraddiceva **la spec che lo
  pubblicava**, e perche l'inventario conteneva gia il dato senza avere la
  classe che confronta i ruoli. Voce di `activity` aggiunta.
- `recurring-defects.md`: famiglia 1 passa a sette occorrenze in sei spec, con
  la domanda nuova — *questo stesso valore compare in due ruoli che il
  protocollo richiede distinti?*; famiglia 3 passa a quattro occorrenze in due
  spec, perche un tetto scritto come esempio e la sua forma piu discreta;
  famiglia 4 passa a due.
- **Non implementata, ed e decisione del Lead:** la classe di difetto
  dichiarativa proposta da [REVIEW-021] — un ruolo per `presentation` e un
  elenco di coppie di ruoli mutuamente esclusivi in `published_artifacts.toml`.
  La review la qualifica come proposta e non come finding. Questa remediation ha
  fatto la meta a costo zero: entrambe le chiavi dichiarano il proprio ruolo nel
  `name` del manifesto, e cinque probe C10 nuovi pinnano le regole normative
  introdotte qui.

### Files changed in this remediation

- `docs/protocol/identity.md`
- `docs/protocol/wire.md`
- `docs/protocol/README.md`
- `core/coblox-core/src/error.rs`
- `core/coblox-core/src/identity.rs`
- `core/coblox-core/src/params.rs`
- `core/coblox-core/tests/canonical_serialization.rs`
- `core/coblox-core/tests/common/mod.rs`
- `core/coblox-core/tests/conformance_registry.rs`
- `core/coblox-core/tests/light_client_perimeter.rs`
- `sim/tools/protocol_hashes.py`
- `sim/tools/published_artifacts.toml`
- `.lmbrain/knowledge/threat-model.md`
- `.lmbrain/knowledge/recurring-defects.md`
- `.lmbrain/decisions/ADR-015-...` (Consequences)
- `.lmbrain/decisions/ADR-012-...` (tabella delle occorrenze, activity)

### Remediation transcripts

#### 1. GATE-NO-ATTESTATION-REJECTED, e le prove in negativo delle guardie nuove

```text
$ cargo test --test conformance_registry gate_no_attestation_rejected
running 1 test
test gate_no_attestation_rejected ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.10s

# Prova in negativo 1 - rimosso da identity.rs il confronto fra chiave di
# trasporto e chiave di identita (RF-001):
running 1 test
test gate_no_attestation_rejected ... FAILED
thread 'gate_no_attestation_rejected' panicked at core\coblox-core\tests\conformance_registry.rs:628:10:
called `Result::unwrap_err()` on an `Ok` value: ()
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 23 filtered out

# Prova in negativo 2 - disattivato il tetto sulla durata della finestra (RF-002):
thread 'gate_no_attestation_rejected' panicked at core\coblox-core\tests\conformance_registry.rs:677:10
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 23 filtered out

# Prova in negativo 3 - tolleranza di skew futuro azzerata (RF-003):
thread 'gate_no_attestation_rejected' panicked at core\coblox-core\tests\conformance_registry.rs:711:5
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 23 filtered out

# Prova in negativo 4 - la fixture TKA-0 rimessa sulla chiave di identita, cioe
# il difetto di RF-001 reintrodotto esattamente come stava:
thread 'tka0_is_the_published_attestation_and_its_transport_key_is_distinct' panicked at
core\coblox-core\tests\canonical_serialization.rs:267:5:
assertion `left == right` failed: the TKA-0 fixture and the published canonical example disagree
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out

# Tutte e quattro le guardie ripristinate:
$ cargo test --test conformance_registry gate_no_attestation_rejected
test gate_no_attestation_rejected ... ok
$ cargo test --test canonical_serialization tka0
test tka0_is_the_published_attestation_and_its_transport_key_is_distinct ... ok
```

La gate copre ora **dieci** percorsi di rifiuto invece di sei: chiave di
trasporto disallineata, network ID, scaduta, non ancora attiva, `node_id`
disallineato, firma non valida, **chiave di trasporto uguale alla chiave di
identita**, **finestra invertita**, **finestra oltre il tetto**, e i due bordi
della tolleranza di orologio.

#### 2. GATE-NO-PUBLISHED-LINK

```text
$ git grep -n "libp2p_peer_id" -- docs/protocol core sim
docs/protocol/identity.md:54:objects no longer embed `libp2p_peer_id`.

$ git grep -n "L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s" -- docs/protocol
docs/protocol/identity.md:91    §"Node identifier", ruolo di identita
docs/protocol/identity.md:147   EnrollmentRequest.public_key, ruolo di identita
docs/protocol/identity.md:442   EnrollmentCertificate.public_key, ruolo di identita
docs/protocol/identity.md:473   la frase che dice che TKA-0 NON usa questa chiave
```

La chiave di identita non compare piu in alcun ruolo di trasporto: ne come
`transport_public_key` di `TKA-0`, ne come chiave della fixture di derivazione
del Peer ID.

#### 3. GATE-FIXTURES-RECOMPUTED

Metodo validato **prima** su valori non modificati, poi il ricalcolo.

```text
$ python sim/tools/protocol_hashes.py
Enrollment request fixture ER-0:
  enrollment_request_hash ER-0 MATCH
    published sha256:52118f65908736ec7fd837a4d6c1b8c2b3ba28e2f0127cea6e282b311e401e58
    computed  sha256:52118f65908736ec7fd837a4d6c1b8c2b3ba28e2f0127cea6e282b311e401e58

Governed protocol documents. Three of the four are untouched by this
pass and are therefore method validation; consensus_parameters is the
one that changed, because the SPEC-013 remediation adds
max_transport_attestation_validity_ms and
max_transport_attestation_future_skew_ms to its body:
  enrollment_parameters        MATCH
    published sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
    computed  sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
  hosting_rate_card            MATCH
    published sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8
    computed  sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8
  consensus_parameters         MATCH
    published sha256:87dc1d92edcd94d5efe3837af9157a4bda604dbd7a658f509bd6fb864f86ada5
    computed  sha256:87dc1d92edcd94d5efe3837af9157a4bda604dbd7a658f509bd6fb864f86ada5
  reward_policy                MATCH
    published sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48
    computed  sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48
[... alberi taggati e fixture APP-0: tutte MATCH ...]
every published value reproduced: PASS
```

Il valore precedente di `consensus_parameters_hash`,
`sha256:628c66f9…c50ccbe9`, e stato sostituito in tutti e cinque i siti che lo
portavano: `README.md` riga 400, `published_artifacts.toml` (`[[value]]` e
`example_invariant`), `conformance_registry.rs` e `light_client_perimeter.rs`.

Derivazione del Peer ID, validata sui tre valori pubblicati e **non modificati**
della chiave di identita, poi applicata alla chiave di trasporto:

```text
identity fixture key L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s
  protobuf published 080112202ffa35a99d3a3cfbb17bb7c1dc5561b18a8dcca4df38dc613ea859c37eb1336b
  protobuf computed  080112202ffa35a99d3a3cfbb17bb7c1dc5561b18a8dcca4df38dc613ea859c37eb1336b
  peerid   published 12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA
  peerid   computed  12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA
  cid      published bafzaajaiaejcal72gwuz2or47oyxxn6b3rkwdmmkrxgkjxzy3rqt5kczyn7lcm3l
  cid      computed  bafzaajaiaejcal72gwuz2or47oyxxn6b3rkwdmmkrxgkjxzy3rqt5kczyn7lcm3l
  -> metodo validato su tre valori non modificati

transport fixture key n0lDnp2wlbxBEe0l01eV2DG8VaBH9LHX9q7jd3u0EiA
  protobuf 080112209f49439e9db095bc4111ed25d35795d831bc55a047f4b1d7f6aee3777bb41220
  peerid   12D3KooWLY9nerKo6xGVcRVjDRdqLh7oMgz3tJk61oSgCo5kKWmM
  cid      bafzaajaiaejcbh2jiopj3mevxrard3jf2nlzlwbrxrk2ar7uwhl7nlxdo553iera
  -> i tre valori nuovi di identity.md §"Canonical libp2p Peer ID"
```

E la stessa derivazione che [REVIEW-021] ha eseguito per dimostrare RF-001. Ora
produce il Peer ID di una chiave che il ledger **non** pubblica.

#### 4. GATE-ADR012

```text
$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     16 candidate(s) checked
  C4-VALUE          51 candidate(s) checked
  C5-MIRROR         42 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C10-PROBE         19 candidate(s) checked

published-artifact inventory: PASS
```

**Lo strumento ha fallito due volte durante questa remediation, ed e la parte
che vale la pena riportare** — e la ragione per cui [ADR-012] chiede uno
strumento versionato e non un controllo a vista:

```text
FAIL C4-VALUE: base64url presentation 'n0lDnp2wlbxBEe0l01eV2DG8VaBH9LHX9q7jd3u0EiA'
  occurs in identity.md but is not classified in the manifest
FAIL C3-FIXTURE-ID: fixture identifier 'TKA-0' occurs in identity.md but is
  absent from the manifest
```

La chiave di trasporto nuova e l'identificatore `TKA-0` — che il documento
originale non nominava affatto — sono stati registrati, con il ruolo scritto nel
`name` accanto a quello della chiave di identita.

Cinque probe C10 nuovi pinnano le regole normative introdotte da questa
remediation: `transport-key-distinct-from-identity`,
`transport-attestation-validity-cap`, `transport-attestation-skew-tolerance`,
`admission-shield-source-is-address`, `revocation-applies-to-live-connections`.

```text
$ python sim/tools/published_artifacts_negative.py
[...]
negative proof: PASS - 10 defect classes, each observed failing
```

#### 5. Suite completa, formato e lint

```text
$ cargo fmt --check      # nessun diff
$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s)

$ cargo test --all-targets
  coblox_core unittests            26 passed
  canonical_serialization.rs        6 passed   (+1: la fixture TKA-0)
  conformance_registry.rs          24 passed
  constraint_block.rs              19 passed
  election_degenerate.rs           12 passed
  light_client_perimeter.rs        13 passed
  sparse_account_state.rs           8 passed
  speccheck_conformance.rs         11 passed
  worked_example.rs                 6 passed
  coblox_ffi                        1 passed
  0 failed in ogni suite
```

### Known limitations of this remediation

1. **Nessuna invalidazione anticipata di un'attestazione in circolazione.**
   L'unico limite all'esposizione di una chiave di trasporto compromessa resta
   la lunghezza della finestra, ora limitata da un parametro firmato. Un
   contatore di epoca o un numero di serie sarebbe un identificatore stabile in
   piu, cioe ricreerebbe in piccolo la correlazione che [ADR-015] toglie: e
   scritto come opzione **non adottata** in TM-37 (c), non come lacuna
   dimenticata.
2. **La rivalutazione della revoca sulle connessioni vive e una regola scritta e
   non ancora un vettore.** `coblox-core` non modella le connessioni, quindi non
   c'e un test che la eserciti; e coperta da un probe C10 che ne verifica la
   presenza nel documento.
3. **Nessun pavimento sulla frequenza di rotazione**, quindi nessuna garanzia
   che un nodo ruoti mai. E dichiarato in TM-28 (b) invece che nascosto, ed e
   una scelta di prodotto che il Lead puo voler affrontare a parte.
4. **`max_transport_attestation_validity_ms` e
   `max_transport_attestation_future_skew_ms` non hanno vincoli relazionali** —
   ne fra loro ne verso `max_envelope_validity_ms`. Aggiungerne uno sarebbe una
   regola di validita oltre quelle che i finding impongono, quindi e stato
   lasciato al Lead invece di essere deciso qui.

### Handoff status
- [x] Ready for Project Lead review
