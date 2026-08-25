---
id: REVIEW-021
# Note: Quote the title if it contains a colon
title: "Security review of SPEC-013 (GATE-SECREVIEW): la separazione della chiave di trasporto dalla chiave di identita"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-013
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-021-EVENT-001"
    timestamp: "2026-08-25T22:31:09.290081600+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-021-EVENT-002"
    timestamp: "2026-08-25T22:36:16.220651+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Due finding high bloccanti, tre medium, uno low. Le tre superfici che il Lead aveva indicato reggono nella sostanza: per la quarta volta consecutiva i difetti gravi erano altrove.\n\nRF-001 verificato dal Lead in ogni suo passaggio. Il documento dice a distinct Ed25519 key pair come frase descrittiva, senza alcun MUST e senza regola di validita, e identity.rs non contiene alcun confronto fra la chiave di trasporto attestata e la chiave di identita: nulla impedisce a un nodo conforme di usare la stessa chiave per i due ruoli. La fixture canonica di questa spec fa esattamente questo, perche tests/common/mod.rs riga 103 assegna a transport_public_key la costante IDENTITY_FIXTURE_PUBLIC_KEY, la stessa usata come public_key alla riga 85.\n\nIl corollario e stato ricalcolato dal Lead e non letto: dalla sola chiave pubblica che il certificato pubblica sul ledger si ottiene il protobuf canonico 080112202ffa35a9 e il Peer ID 12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA, identici ai valori di conformita di identity.md. Un osservatore che legge il ledger, se il nodo riusa la chiave, ottiene il Peer ID senza connettersi a nulla. GATE-NO-PUBLISHED-LINK resta verde perche il campo non c'e piu, ma il legame non e pubblicato bensi ricalcolabile, e ai fini di TM-28 e la stessa cosa. E settima occorrenza della famiglia 1, e la piu grave, perche la fixture insegna la forma che annulla la spec che la introduce.\n\nRF-002 verificato. Il MUST di identity.md ha due clausole: la prima e implementata e non esercitata da alcun test, perche InvalidValidityWindow compare solo in error.rs e in identity.rs e in nessuna asserzione; la seconda non e implementata affatto e il valore massimo esiste solo come esempio fra parentesi, perche una ricerca su docs, core e sim non restituisce alcun max_attestation_validity_ms. Un'attestazione con expires_at_ms arbitrariamente lontano supera verify, quindi il legame e permanente e cade il punto 2 della motivazione scritta due righe sopra, cioe che una chiave compromessa scade da sola.\n\nRF-005 e la superficie nuova ed e reale: il possesso della chiave di trasporto e, in sessione, il possesso dell'identita per cio che riguarda le connessioni dirette. L'attaccante non puo forgiare envelope ne firmare risposte, che e la parte solida del design, ma puo occupare il posto della vittima e non rispondere, facendo scadere le challenge. Prima serviva la chiave di identita, compromissione totale ma revocabile; oggi e una chiave dichiarata a basso valore e non esiste alcuna invalidazione anticipata di un'attestazione in circolazione. Il baratto non e scritto da nessuna parte.\n\nRF-003 e RF-004 sono la stessa famiglia, regola scritta piu debole della proprieta che deve tenere, e costano poco nella stessa passata. RF-006 e low e conferma che la revoca regge.\n\nTM-28 non e dichiarato chiuso in nessun punto. Va pero corretta la frase nuova di TM-28 lettera b, che dice elimina la correlazione passiva: e falsa finche RF-001 e aperto."
    evidence_refs: ["SPEC-013", "REVIEW-020", "ADR-015", "ADR-012"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-021-EVENT-003"
    timestamp: "2026-08-25T23:03:44.844039500+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Tutti e sei i finding chiusi. RF-001 chiuso con una regola e non con una frase: la chiave di trasporto MUST NOT essere uguale alla chiave di identita enrollata, con errore dedicato e collocato prima del confronto con la chiave autenticata, e TKA-0 rigenerato con un punto Ed25519 realmente distinto. Andando oltre il finding, l'implementatore ha spostato sulla chiave di trasporto anche la fixture di conformita del Peer ID canonico, che derivava protobuf, Peer ID e CID dalla chiave di identita: il documento pubblicava la ricetta e l'ingrediente nello stesso file. TKA-0 era inoltre costruito dal modulo di fixture e asserito da nulla, e ora ha un test che lo lega byte per byte all'esempio pubblicato.\n\nRF-002 chiuso trasformando l'esempio fra parentesi in un parametro con un nome, max_transport_attestation_validity_ms, aggiunto al corpo firmato dei consensus_parameters e applicato in verify. Entrambe le clausole del MUST hanno ora vettori, compresa quella che prima non aveva alcuna asserzione.\n\nRF-003 chiuso con una tolleranza asimmetrica, concessa solo verso il futuro su created_at_ms e mai oltre la scadenza, con l'asimmetria argomentata e il limite dichiarato per la direzione dell'orologio lento. RF-004 chiuso definendo source in modo normativo come l'indirizzo remoto osservato, con la ragione per cui la distinzione ha smesso di essere innocua dopo ADR-015. RF-005 chiuso scrivendo il baratto in tre punti piu uno scenario nuovo TM-37, e correggendo una cella della matrice che era n/a ed e stata falsificata. RF-006 chiuso nominando il filtro di revoca pre-handshake perduto e aggiungendo la regola di rivalutazione delle connessioni vive.\n\nTM-28 lettera b corretta: elimina rimosso, e la contromisura non e piu intitolata rotazione perche nessun documento prescrive una rotazione ma solo un intervallo minimo.\n\nRegistrata la settima occorrenza nella tabella di ADR-012 e aggiornati i conteggi di recurring-defects.md: famiglia 1 a sette, famiglia 3 a quattro, famiglia 4 a due. La gate di ADR-012 e fallita due volte durante la passata prima di passare, il che e essa stessa evidenza che funziona."
    evidence_refs: ["SPEC-013"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-021-EVENT-004"
    timestamp: "2026-08-25T23:10:36.968743600+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificato dal Lead rieseguendo e da AGENT-007 con una verifica mirata su cio che appartiene al suo documento e che il Lead non poteva attestare al posto suo.\n\nVerifiche del Lead: 126 test passati, clippy zero warning, fmt pulito, cinque strumenti versionati OK. La guardia di distinzione provata in negativo, rimossa dal codice gate_no_attestation_rejected FAILED e ripristinata l'albero torna verde. I tre valori nuovi della fixture del Peer ID canonico ricalcolati con il metodo validato prima riproducendo i tre valori vecchi: protobuf 080112209f4943 e Peer ID 12D3KooWLY9nerKo6xGVcRVjDRdqLh7oMgz3tJk61oSgCo5kKWmM corrispondono esattamente.\n\nAGENT-007 dichiara GATE-SECREVIEW soddisfatta per quanto la riguarda, e giudica RF-003 chiuso meglio di come l'aveva chiesto. Segnala che RF-005 non e finito solo nel threat model perche il documento dice ora che si tratta di un trasferimento di rischio e non di un guadagno netto, che era la frase mancante.\n\nTre residui non bloccanti, nessuno condizione di chiusura. Il primo, che l'affermazione di TM-37 lettera a prometteva piu di quanto la regola tenga finche DEBT-017 e aperto, e stato corretto dal Lead nella stessa passata perche il threat model e artefatto del brain e la sua manutenzione e del Lead, non dell'implementatore. Il secondo e il piu utile e riguarda il documento di AGENT-007 e non questa spec: la cella A-02 per T-06 era gia falsa da SPEC-004 per una via che il documento contiene, cioe l'isolamento di TM-31, quindi TM-37 e il secondo falsificatore e non il primo; l'elenco asset di TM-31 omette A-02; e l'argomento comune non puo scrivere quindi n/a va risottoposto alla cella A-04. Il terzo e che TM-37 e collocato sotto un attore che non lo copre, perche T-06 non comprende chi esfiltra una chiave da un dispositivo.\n\nSu DEBT-017 AGENT-007 conferma la lettura del Lead e accetta la titolarita, aggiungendo che il documento contiene gia l'argomento che lo dimostra e non lo ha guardato, e riportando nell'artefatto una via che sembra elegante e non funziona perche romperebbe il verificatore onesto con l'orologio indietro. Sulla classe dichiarativa per l'inventario dichiara che non la chiede, perche una regola di validita applicata a runtime da ogni ricevente e piu forte di un controllo di fixture a tempo di build."
    evidence_refs: ["SPEC-013", "REVIEW-021", "DEBT-017"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-021-EVENT-005"
    timestamp: "2026-08-25T23:10:47.189337900+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Tutti e sei i finding chiusi e verificati, e AGENT-007 dichiara esplicitamente GATE-SECREVIEW soddisfatta per quanto la riguarda dopo una verifica mirata sulle parti che appartengono al suo documento. Nessuno dei suoi tre residui e condizione di chiusura, e lei lo dice.\n\nRF-001 e RF-002 sono chiusi nella forma forte: una regola di validita applicata a runtime da ogni ricevente, non una guardia a tempo di build. Il Lead ha provato in negativo la guardia di distinzione e ricalcolato i tre valori nuovi della fixture del Peer ID canonico validando prima il metodo sui tre vecchi.\n\nIl residuo che riguardava questa spec, l'affermazione di TM-37 lettera a resa piu forte del vero da DEBT-017, e stato corretto dal Lead nella stessa passata: il threat model e artefatto del brain e la sua manutenzione spetta al Lead. Gli altri due residui riguardano il documento di AGENT-007 e non SPEC-013, e sono registrati come debito separato perche sono manutenzione del threat model."
    evidence_refs: ["SPEC-013", "REVIEW-020", "DEBT-017", "ADR-015"]
    implementation_agent: "AGENT-001"
links: [SPEC-013, ADR-015, ADR-012, REVIEW-020]
created: 2026-08-25
updated: 2026-08-25
tags: [security, review, identity, privacy]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "recorded review remediation-verification"
  - date: 2026-08-25
    action: "transitioned changes-requested -> accepted"
---
# Review

## Outcome

**Changes requested.** Due finding high, tre medium, uno low.

La domanda posta non era se i test passano — quello il Lead lo ha gia stabilito — ma
**se la separazione regge come difesa, e se cio che ha spostato non ha rotto altro.**
Le risposte, in ordine:

- **La separazione, cosi com'e scritta, non e tenuta da alcuna regola.** Nessun
  documento e nessuna riga di codice impedisce a un nodo pienamente conforme di
  presentare come `transport_public_key` la propria chiave di identita. Se lo fa, il
  legame `node_id` verso Peer ID torna derivabile dal solo ledger, gratis e in modo
  retroattivo: esattamente TM-28 nella sua forma originale. **La fixture canonica
  pubblicata da questa spec fa proprio questo** (RF-001).
- **Cio che la spec ha spostato ha rotto qualcosa**, ed e la finestra di validita:
  il documento scrive un MUST in due clausole, il codice ne implementa una e la
  gate non ne esercita nessuna delle due (RF-002).
- **Si, la separazione ha creato una superficie che prima non esisteva**: la
  compromissione di una chiave dichiarata "a basso valore" produce oggi
  impersonificazione in sessione senza alcun meccanismo di invalidazione anticipata
  (RF-005).

Le tre superfici indicate dal Lead sono state esaminate per prime e per intero. Due
reggono nella sostanza e cedono sulla precisione della regola (RF-003 sull'orologio,
RF-004 sull'ancoraggio dei limiti di enrollment). La terza — la revoca — regge, con
un costo nuovo che nessun documento dichiara (RF-006). I difetti gravi sono, per la
quarta volta, altrove: nella fixture e nella clausola non implementata.

## Acceptance-criteria compliance

Undici criteri su dodici sono soddisfatti come dichiarato e verificati in modo
indipendente dal Lead in [REVIEW-020]. Non li ripeto.

Il criterio 2 — *"Esiste l'oggetto di attestazione, con schema, serializzazione
canonica, preimmagine a dominio separato e **fixture pubblicata**"* — e soddisfatto
alla lettera e **falsificato nella sostanza**: la fixture esiste, e canonica, e
insegna la configurazione che annulla la spec (RF-001).

Il criterio 3 — *"L'attestazione ha una validita limitata, con la forma del limite
motivata"* — e soddisfatto per la motivazione e **non** per il limite: il limite e
scritto come esempio parentetico, non e un parametro, non e implementato (RF-002).

## Code observations

`core/coblox-core/src/identity.rs` e pulito nella parte che c'e. In particolare due
scelte sono giuste e vanno dette, perche sono i punti in cui un'implementazione
sciatta avrebbe aperto un buco:

- `verify()` deriva `node_id` **dalla chiave di identita enrollata** passata dal
  chiamante (riga 131) e la confronta con quella dichiarata nell'attestazione,
  invece di fidarsi del campo. Un peer non puo quindi auto-asserire un `node_id`:
  il legame parte sempre dal certificato.
- Il confronto fra `transport_public_key` e la chiave autenticata sulla connessione
  (riga 139) e presente ed e la riga su cui poggia l'intera proprieta. Il Lead ha
  provato in negativo che rimuoverla fa fallire la gate. Confermo il giudizio.

L'argomento anti-riuso di `identity.md` §*Anti-reuse property* e **corretto** per il
terzo che intercetta l'attestazione: la prova di possesso dell'handshake Noise/QUIC
lo esclude. Non copre il caso in cui il possesso della chiave di trasporto sia esso
stesso l'assunzione dell'avversario, che e il caso che il documento stesso dichiara
routine (RF-005).

Cio che manca in `verify()`:

1. nessun controllo che `expires_at_ms - created_at_ms` sia limitato (RF-002);
2. nessuna tolleranza di scarto d'orologio in nessuna delle due direzioni: il
   confronto e `now_ms < created_at_ms || now_ms > expires_at_ms`, secco (RF-003);
3. nessun controllo che `transport_public_key != enrolled_identity_public_key`
   (RF-001), che e l'unico punto del codice in cui la proprieta di privacy di
   [ADR-015] potrebbe essere resa verificabile.

## Tests and verification

Le quattro gate `before-submit` sono state eseguite e le trascrizioni sono reali. Ho
rieseguito in modo indipendente solo cio che serviva ai finding.

**Verificato in modo indipendente, non letto dall'evidenza:**

- **La chiave di trasporto della fixture e la chiave di identita della fixture.**
  `docs/protocol/identity.md:123` (`EnrollmentRequest.public_key`),
  `docs/protocol/identity.md:405` (`EnrollmentCertificate.public_key`) e
  `docs/protocol/identity.md:432` (`TransportKeyAttestation.transport_public_key`)
  portano la stessa stringa `L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s`. Non e
  una coincidenza di scrittura: `core/coblox-core/tests/common/mod.rs:103` costruisce
  la fixture `TKA-0` assegnando a `transport_public_key` la costante letteralmente
  chiamata `IDENTITY_FIXTURE_PUBLIC_KEY`.
- **Da quei byte ho ricostruito il Peer ID.** Decodificata la chiave base64url ->
  32 byte `2ffa35a9...336b`; composto il protobuf libp2p canonico
  `080112202ffa35a99d3a3cfbb17bb7c1dc5561b18a8dcca4df38dc613ea859c37eb1336b`;
  multihash identity `0x00 0x24 || protobuf`; base58btc ->
  `12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA`. E esattamente il valore
  che `identity.md:52` pubblica. Il percorso usa **solo** byte che il ledger
  pubblica e **solo** la derivazione che `identity.md` §*Canonical libp2p Peer ID*
  specifica per esteso. Costo per l'osservatore: zero, offline, retroattivo.
- **`AttestationError::InvalidValidityWindow` non e esercitato da alcun test.** Due
  sole occorrenze in tutto il workspace: la definizione in `error.rs:226` e la
  costruzione in `identity.rs:143`. Nessuna asserzione.
- **Nessun parametro di rete limita la durata dell'attestazione.** Ricerca su
  `docs/`, `core/`, `sim/`: `max_envelope_validity_ms`, `max_request_age_ms` e
  `max_future_skew_ms` esistono in `params.rs` e nel documento
  `consensus_parameters`; un equivalente per l'attestazione non esiste in nessuna
  forma. Il tetto vive in una sola parentesi di prosa, `identity.md:456`.
- **La gate `gate_no_attestation_rejected` copre sei percorsi** — chiave di
  trasporto disallineata, network ID, scaduta, non ancora attiva, `node_id`
  disallineato, firma non valida — e **non copre** ne la finestra invertita ne la
  finestra abnorme ne l'uguaglianza fra chiave di trasporto e chiave di identita.

**Qualita della gate.** `GATE-NO-ATTESTATION-REJECTED` e ben costruita per cio che
dichiara ed e stata provata in negativo dal Lead. Il difetto non e nella gate ma nel
suo perimetro: e verde su meta di una regola normativa e muta sulla proprieta che la
spec esisteva per comprare. E la **famiglia 4** di
`.lmbrain/knowledge/recurring-defects.md`, seconda occorrenza accertata, e la coppia
di domande che la intercetta la trova subito — *per ogni clausola della regola, quale
vettore la esercita?*

**Nota sulla gate di [ADR-012].** Lo strumento di [SPEC-010] e passato e ha ragione a
passare: verifica forme e coerenze fra copie, mai la correttezza semantica di un
valore, e lo dichiara nella propria intestazione. Ma l'inventario contiene gia
l'informazione che avrebbe smascherato RF-001:
`sim/tools/published_artifacts.toml:864` registra quella stringa con
`name = "the identity fixture public key of identity.md"`. L'inventario sa che quel
valore e la chiave di **identita**; `identity.md` la presenta anche come chiave di
**trasporto**; nessuna classe di difetto confronta i due ruoli. La chiusura ha quindi
la stessa forma della famiglia 1 rovesciata: costruire la classe che mancava.

## Production quality and documentation compliance

TM-28 e aggiornato correttamente e resta `Stato: aperto`. **In nessun punto del
repository risulta dichiarato chiuso**: ricerca eseguita, condizione di revisione di
[ADR-015] rispettata. Il residuo aperto verso l'interlocutore attivo e scritto con
la precisione che l'ADR chiedeva.

Una frase pero e piu forte del vero, ed e figlia di RF-001. La contromisura (b) di
TM-28 afferma che la separazione *"**elimina** la correlazione passiva da parte di
osservatori offline che leggono solo il ledger"*. Finche nessuna regola impedisce
l'uguaglianza fra le due chiavi, quella correlazione non e eliminata: e resa
**opzionale, a discrezione dell'implementatore, in modo non verificabile dai peer**.
E la famiglia 2, domanda 1: *quale regola la tiene?* La risposta oggi e "l'operatore
sceglie bene la chiave", che non e una proprieta ma una preferenza.

Seconda imprecisione della stessa famiglia: la contromisura (b) e intitolata
*"Rotazione dell'identita di trasporto"*, ma **nessun documento specifica una
rotazione**. `wire.md` §*Transport rotation* introduce un *tetto* alla frequenza di
rotazione (intervallo minimo) e nessun pavimento, nessuna raccomandazione, nessun
intervallo di riferimento. Cio che la spec ha consegnato e la *possibilita* di
ruotare; la contromisura accreditata e la rotazione. Un nodo che non ruota mai ha un
Peer ID stabile a vita, e una singola sessione con un peer ostile fissa la coppia per
sempre — il che riporta il guadagno di privacy da "cambio di costo dell'attacco" a
"cambio di costo della prima osservazione".

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

- RF-001 | category=security-boundary | severity=high | criterion=la proprieta di privacy di [ADR-015] non e tenuta da alcuna regola, e la fixture pubblicata insegna la forma che la annulla | remediation=**Nulla impedisce che `transport_public_key` sia la chiave di identita, e la fixture canonica di questa spec fa esattamente questo.** `identity.md:17` dice che il livello di trasporto "uses a **distinct** Ed25519 key pair": e una frase descrittiva, senza MUST, senza regola di validita in §*Authentication on a connection*, senza controllo in `TransportKeyAttestation::verify()`. **Attacco.** (1) Un nodo conforme importa la propria chiave di identita in libp2p — cioe fa cio che il protocollo faceva prima di [ADR-015], che e il comportamento di default di ogni implementazione migrata e di ogni implementatore che legge la fixture; (2) firma un'attestazione con `transport_public_key = public_key`: `verify()` la accetta, ogni gate e verde, nessun peer puo accorgersene ne rifiutare; (3) l'osservatore passivo legge `public_key` dal `EnrollmentCertificate` sul ledger, applica la derivazione che `identity.md` §*Canonical libp2p Peer ID* pubblica per esteso, e ottiene il Peer ID. **Verificato**: dalla chiave `L_o1qZ06...` pubblicata in `identity.md:123` e `:405` ho ricostruito protobuf `080112202ffa35a9...336b` e Peer ID `12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA`, identico a `identity.md:52`. **Impatto.** TM-28 nella sua forma originale, integralmente: lettura gratuita, offline, retroattiva del legame `node_id` verso Peer ID verso IP. La spec non ha ottenuto nulla per quel nodo, e nessuno puo dire quali nodi siano in quel caso. E `GATE-NO-PUBLISHED-LINK` resta soddisfatta, perche il campo non c'e: il legame non e pubblicato, e **ricalcolabile**, che ai fini di TM-28 e la stessa cosa. **Chiusura verificabile:** (a) regola di validita normativa in `identity.md`, con la stessa forza della regola di rifiuto gia scritta: `transport_public_key` MUST NOT eguagliare la chiave di identita enrollata, e un'attestazione che lo faccia MUST essere rifiutata — e una regola che il ricevente puo applicare, perche ha entrambe le chiavi in mano; (b) controllo corrispondente in `verify()` con variante d'errore dedicata; (c) caso negativo nella gate `gate_no_attestation_rejected`; (d) **la fixture `TKA-0` va rigenerata con una chiave di trasporto distinta**, in documento, in `tests/common/mod.rs` e nell'inventario, e la nuova chiave registrata in `published_artifacts.toml` con il proprio ruolo; (e) TM-28 (b) riscritta togliendo "elimina" finche (a) non e in vigore. **Costo:** un confronto di 32 byte per attestazione.

- RF-002 | category=robustness | severity=high | criterion=il tetto sulla finestra di validita e un esempio, non una regola: nessun parametro, nessuna implementazione, nessun test | remediation=**`identity.md:454-456` scrive un MUST in due clausole — rifiutare `expires_at_ms < created_at_ms` **e** rifiutare `expires_at_ms - created_at_ms` oltre la durata massima "(e.g. 86,400,000 ms / 24 hours)". La prima clausola e implementata e non esercitata da alcun test; la seconda non e implementata affatto e non esiste come parametro.** Ricerca su `docs/`, `core/`, `sim/`: nessun `max_attestation_validity_ms`; il valore vive solo in quella parentesi. `AttestationError::InvalidValidityWindow` ha due occorrenze nel workspace, definizione e costruzione, e zero asserzioni. **Attacco.** Un nodo emette un'attestazione con `expires_at_ms = 2^64-1`. `verify()` la accetta senza riserve. Il legame chiave-di-trasporto/identita diventa **permanente**, il che (1) reintroduce la durabilita che [ADR-015] aveva tolto dal ledger, ora sul lato sessione, e (2) annulla il punto 2 della motivazione scritta nello stesso documento: la chiave di trasporto compromessa **non** scade piu da sola, e l'unico rimedio residuo e il `RevokeIdentity` che il documento dichiara di voler evitare perche distrugge identita, saldo e reputazione. **Precedente.** E RF-012 di [REVIEW-002], gia trovato una volta in questo progetto sulla stessa forma (`expires_at_ms` senza tetto su `SignedEnvelope`) e chiuso con un parametro di rete firmato, `max_envelope_validity_ms`. La spec introduce un oggetto nuovo con la stessa coppia di campi e non riapplica la chiusura. E anche famiglia 3: e stata vincolata la relazione fra `created` ed `expires`, non la **grandezza da cui la proprieta dipende**, che e la loro differenza. **Chiusura verificabile:** parametro `max_transport_attestation_validity_ms` nel documento firmato `consensus_parameters` e in `params.rs`, ordine di grandezza quello gia scritto in prosa; rifiuto normativo in `identity.md` con la forma "MUST" e non "e.g."; controllo in `verify()`; due casi negativi nella gate — finestra invertita e finestra abnorme — perche oggi **nessuna delle due clausole del MUST ha un vettore che la esercita**.

- RF-003 | category=robustness | severity=medium | criterion=nessuna tolleranza di scarto d'orologio, in un meccanismo scelto proprio perche il verificatore puo non essere sincronizzato | remediation=**La motivazione scritta in `identity.md` §*Bounded validity in time* punto 1 e che ancorare la validita alle altezze di catena creerebbe una dipendenza circolare, perche un nodo non puo connettersi per scaricare blocchi senza gia conoscere l'altezza finalizzata. L'argomento e corretto. La conseguenza che non e stata tratta e che quel nodo — nuovo, appena installato, offline da tempo — e anche quello con l'orologio meno affidabile, e la regola che lo sostituisce e un confronto secco contro l'orologio locale, senza alcuna tolleranza in nessuna delle due direzioni** (`identity.rs:149`). **Attacco/guasto.** Un nodo il cui orologio e indietro anche di pochi secondi calcola `now_ms < created_at_ms` su **ogni** attestazione appena emessa e la rifiuta. Poiche l'attestazione e obbligatoria per tutti gli stream tranne enrollment, quel nodo perde `ledger-sync` — cioe **l'unica fonte da cui potrebbe correggere la propria nozione di tempo**, dato che i blocchi portano `timestamp_ms`. L'isolamento si autoalimenta. Nella direzione opposta, un orologio molto indietro accetta attestazioni scadute da ore. Non serve un avversario: NTP non autenticato, un dispositivo Android riavviato senza rete, una VM sospesa. **Asimmetria con il resto del protocollo:** la richiesta di enrollment ha `max_request_age_ms` e `max_future_skew_ms` come parametri firmati; `wire.md` §*Signed envelope* impone il fallimento chiuso su clock rollback per i protocolli protetti. L'attestazione, che e la **porta** di tutti i protocolli protetti, non ha ne l'uno ne l'altro. **Chiusura verificabile:** tolleranza di skew futuro dichiarata e parametrizzata sul modello di `max_future_skew_ms`, applicata al confronto con `created_at_ms`; regola esplicita di comportamento su clock rollback coerente con quella gia scritta per l'envelope; casi di test ai due bordi. Se la scelta e invece di **non** tollerare skew, va scritto come limite dichiarato — che la disponibilita della rete per un nodo dipende da un orologio che nessun certificato attesta — con la stessa franchezza con cui `identity.md` dichiara gia il limite sulla disponibilita dell'enrollment.

- RF-004 | category=security-boundary | severity=medium | criterion=lo scudo di ammissione e ancorato a una "sorgente" che questa spec ha reso ambigua, e la mitigazione nuova non si applica allo stream che ne ha bisogno | remediation=**`identity.md` §*The admission shield* lega `admission_nonce* "to that libp2p Peer ID and the observed remote address" e mette il tetto `k` "on the un-consumed, unexpired nonces outstanding for **one source**", senza definire quale delle due sia la sorgente. Prima di [ADR-015] la distinzione era innocua, perche il Peer ID derivava dalla chiave di identita e quindi costava un enrollment; questa spec rende il Peer ID gratuito, illimitato e ruotabile a piacere, e non disambigua nella stessa passata.** Un implementatore che ancori `k`, o il limite per sorgente del passo 1, o il conteggio dello step-9 fallito, al Peer ID, ottiene un limite che l'attaccante azzera con una `keygen`. L'argomento di costo scritto due paragrafi piu sotto — *"an attacker pays a distinct **reachable address** for every k concurrent slots"* — dice che l'ancora voluta e l'indirizzo, ma e prosa esplicativa, non la regola. **La mitigazione nuova non copre il caso:** `wire.md` §*Transport rotation, attribution, and rate limits* ancora tutto a `sender_node_id` e impone l'intervallo minimo di rotazione "per `node_id`" — e lo stream di enrollment e per definizione l'unico in cui il mittente **non ha** un `node_id` verificato (`wire.md:183-184` lo dice: envelope e firma sono confrontati con la chiave della richiesta, non con un certificato). La sezione nuova copre tutto tranne l'unico stream in cui la rotazione e gratuita. Nota: la difesa non e rotta, perche il vincolo "at most one in-flight step-9 evaluation **per public key**" e ancorato alla chiave e resta valido; e rotta l'univocita della regola scritta, che e cio che un implementatore seguira. E famiglia 2, domanda 3: *quale regola la rendera obsoleta?* — questa. **Chiusura verificabile:** in `identity.md`, definire "source" come l'indirizzo remoto osservato, normativamente, con una frase che dica perche il Peer ID **non** e piu ammissibile come ancora dopo [ADR-015]; in `wire.md` §*Transport rotation*, aggiungere il caso dello stream di enrollment invece di lasciarlo per esclusione.

- RF-005 | category=documentation | severity=medium | criterion=la separazione ha creato una superficie nuova, e il documento ne descrive l'impatto come piu basso di quanto sia | remediation=**Questa e la risposta alla domanda "la separazione ha creato una superficie che prima non esisteva".** `identity.md` §*Bounded validity in time* punto 2 giustifica la scelta dicendo che se una chiave di trasporto e compromessa la sua attestazione "expires automatically" senza dover distruggere l'identita permanente. La cosa vera che quella frase non dice e che **per tutta la finestra il possesso della chiave di trasporto e, verso ogni peer, il possesso dell'identita**: chi la detiene completa l'handshake, presenta l'attestazione intercettata — che non e un segreto e non e legata al destinatario — e viene accettato come il nodo vittima. **Cio che l'attaccante non puo fare** e la parte solida del design e va detta: gli oggetti applicativi restano firmati dalla chiave di **identita**, quindi non puo forgiare envelope, ne rispondere a un `challenge_request` con un `subject_signature` valido. **Cio che puo fare:** occupare il posto della vittima nelle connessioni dirette e non rispondere. Un `challenge_request` e diretto a `subject_node_id`; l'issuer che dialla la vittima raggiunge l'attaccante; la challenge scade; l'evidenza entra nel ledger come `failed` o `late`. Il costo cade sulla vittima come perdita di `existence_income` e di eleggibilita a validatore, cioe un attacco mirato con impatto economico misurabile. **Prima di [ADR-015] la stessa cosa richiedeva la chiave di identita** — compromissione totale, ma **revocabile**. Oggi e una chiave dichiarata a basso valore, e **non esiste alcun meccanismo di invalidazione anticipata di un'attestazione in circolazione**: nessun contatore di epoca, nessun numero di serie, nessuna lista. L'unico controllo e la lunghezza della finestra, che RF-002 mostra non essere limitata. Il progetto ha barattato una compromissione totale revocabile con una parziale non revocabile, e questo baratto non e scritto da nessuna parte. **Chiusura verificabile:** scenario nuovo nel threat model — attore `T-05`/`T-06`, asset A-02/A-06/A-11 — che descriva la compromissione della chiave di trasporto con impatto, mitigazione (finestra breve, una volta che RF-002 la rende limitata) e residuo dichiarato; correzione del punto 2 di §*Bounded validity in time*, che oggi presenta come vantaggio netto cio che e un trasferimento di rischio; e nelle *Consequences* di [ADR-015] una riga che nomini il baratto.

- RF-006 | category=documentation | severity=low | criterion=la revoca continua a mordere, ma ha perso un punto di applicazione che nessun documento dichiara perduto | remediation=**La risposta alla terza superficie indicata dal Lead e affermativa e va detta prima del rilievo: la revoca regge.** Il nodo revocato che non presenta piu il certificato non ottiene nulla, perche §*Authentication on a connection* impone il rifiuto in assenza di attestazione **e** di certificato validi; il nodo revocato che presenta una vecchia attestazione ottiene comunque un rifiuto, perche la stessa lista impone "no revocation exists at the receiver's finalized height" e quel controllo interroga il ledger, non l'attestazione. L'affermazione di [ADR-015] *"La revoca continua a funzionare senza modifiche"* e corretta. **Due cose pero sono cambiate e non sono scritte.** (a) La revoca non e piu applicabile **prima** dell'handshake: quando il Peer ID derivava dall'identita, chiunque poteva costruire dal solo ledger una lista di Peer ID revocati e rifiutare la connessione a costo zero; oggi, per costruzione, il Peer ID di un nodo revocato e indistinguibile, quindi ogni peer paga handshake, certificato, attestazione e interrogazione del ledger a ogni tentativo, indefinitamente, senza poter pre-filtrare. E un costo reale che [ADR-015] compra e che nessuna delle sue *Consequences* dichiara. (b) La verifica e specificata **solo** all'apertura della connessione: nessuna regola impone di rivalutare o chiudere le sessioni gia stabilite quando una revoca diventa finalizzata. Non e una regressione — valeva anche prima — ma la spec sposta l'intera verifica "in sessione" e rende la lacuna visibile. **Chiusura verificabile:** una riga nelle *Consequences* di [ADR-015] per (a), con il costo nominato; e in `identity.md` §*Authentication on a connection* una frase normativa per (b), che imponga la rivalutazione della revoca sulle connessioni vive al variare dell'insieme finalizzato.

## Required follow-up

RF-001 e RF-002 sono bloccanti per `GATE-SECREVIEW`: il primo perche senza di esso la
spec non ha comprato la proprieta per cui esiste, il secondo perche un MUST scritto e
non implementato e la forma di difetto che questo progetto ha gia censito due volte.

RF-003, RF-004 e RF-005 vanno chiusi nella stessa passata: sono tutti e tre nella
stessa famiglia — una regola scritta con meno forza della proprieta che deve tenere —
e costano poco una volta che si tocca `identity.md`.

RF-006 e a costo quasi nullo e chiude una dichiarazione mancante, non un difetto.

**Una proposta oltre i finding, per il Lead.** L'inventario di [SPEC-010] contiene
gia il dato che avrebbe intercettato RF-001 e non ha la classe che lo confronta.
Suggerisco una classe di difetto nuova — *lo stesso valore presentato in due ruoli che
il protocollo richiede distinti* — dichiarativa nel `.toml` (un ruolo per
`presentation`, e un elenco di coppie di ruoli mutuamente esclusivi). Non appartiene a
questa spec e non e un finding; e la meccanizzazione che rende RF-001 non ripetibile,
sulla stessa forma della chiusura di [SPEC-012].

**TM-28 non va dichiarato chiuso**, e questa review non lo dichiara. Con RF-001 aperto
la separazione non ha nemmeno chiuso l'osservatore passivo, che era la meta che
[ADR-015] rivendicava.

## Final decision

Il lavoro e serio e la parte difficile e fatta bene: il modello a tre chiavi e
corretto, l'ancoraggio a `sender_node_id` non e un aggiramento, la prova di possesso
in sessione e il costrutto giusto, e la regola di rifiuto esiste ed e provata in
negativo. Cio che manca non e ragionamento: sono **tre regole scritte piu deboli della
proprieta che devono tenere** — una chiave che il documento chiama "distinct" senza
imporlo, un tetto scritto come esempio, un confronto d'orologio senza tolleranza — e
una fixture che insegna, nel documento normativo, la configurazione che annulla la
spec.

**Verdetto: changes-requested.** `GATE-SECREVIEW` non e soddisfatta finche RF-001 e
RF-002 restano aperti.

## Verifica mirata della remediation (AGENT-007)

Su richiesta del Lead, verifica limitata a ciò che è di competenza di questo ruolo e
che il Lead non può attestare al posto mio. Non è una seconda review completa: la
parte meccanica è attestata da lui e non è stata rifatta.

**Esito: `GATE-SECREVIEW` è soddisfatta.** I sei finding sono chiusi. Quanto segue è
lavoro residuo da fare fuori da questa spec, non una condizione di chiusura.

### 1. TM-37 — giudizio nel merito

**Severità `media`: corretta.** Regge sia verso l'alto sia verso il basso. Non è
`alta` perché richiede una precondizione fuori dal protocollo — il possesso della
chiave privata di trasporto — e perché l'impatto è mirato su una vittima e non
sistemico. Non è `bassa` perché l'impatto è economico e misurabile, e perché la
precondizione è dichiarata di routine dallo stesso documento che dichiara la chiave
ephemera.

**La contromisura dichiarata è quella vera**, e il rifiuto della (c) è motivato bene:
un contatore di epoca per identità osservabile in sessione è un identificatore
stabile in più, cioè ricrea in piccolo la correlazione che [ADR-015] ha tolto.
Confermo il ragionamento e la non adozione.

**Tre rilievi, tutti minori.**

- La contromisura (a) afferma che l'esposizione "è limitata dal protocollo e non dalla
  prudenza dell'operatore". Con [DEBT-017] aperto **non è ancora vero**: ciò che il
  protocollo limita è la durata dichiarata, non la finestra di accettazione, che è la
  somma con la tolleranza. È la stessa forma per cui TM-28 (b) è stata corretta, in
  scala minore. Basta un rinvio a [DEBT-017] accanto ad (a).
- Il vettore è descritto per intero **sul lato in ingresso** — l'issuer che chiama la
  vittima raggiunge l'avversario — e tace sul secondo meccanismo: dove un peer
  mantiene una connessione per `node_id`, l'avversario che occupa quel posto fa
  **rifiutare** la connessione legittima della vittima. È lo stesso impatto per una
  via diversa e più affidabile della semplice non risposta. Una riga al passo (4).
- **L'attore è sbagliato, ed è il rilievo che conta.** TM-37 è archiviato sotto
  `T-06`, definito come "osservazione passiva del traffico; oppure un peer enrollato
  che si connette a molti; oppure un ISP/censore che filtra. Nessun potere di voto."
  Chi ha esfiltrato una chiave privata da un dispositivo non è nessuna delle tre cose.
  **Il threat model non ha un attore per la compromissione dell'endpoint**, e
  collocare TM-37 in quella colonna allarga la definizione di `T-06` in silenzio —
  cioè fa proprio ciò che la matrice esiste per impedire. Va risolto scegliendo:
  allargare esplicitamente `T-06`, oppure dichiarare TM-37 trasversale, oppure
  aggiungere l'attore mancante. La terza è la più onesta e la più costosa, perché
  aprirebbe una colonna con celle da riempire; non la impongo.

### 2. La cella falsificata, e le celle che poggiano sullo stesso argomento

La correzione di `A-02` × `T-06` è giusta. **La motivazione data è però più stretta
del vero, e questo importa perché è la motivazione che dice dove guardare ancora.**

`A-02` × `T-06` non è stata resa falsa da [SPEC-013]: **era già falsa**, e per una via
che il documento contiene da [SPEC-004]. `T-06` include per definizione "un
ISP/censore che filtra", e TM-31 descrive esattamente l'isolamento di nodi da parte di
un censore. Un nodo isolato non riceve i propri `challenge_request` e non produce
evidenza: è emissione mancata, senza alcun furto di chiave. TM-37 è quindi **un
secondo** falsificatore, non il primo. Corollario: l'elenco di asset di TM-31
(`A-03, A-05, A-10, A-12`) omette `A-02` e va corretto nella stessa passata,
altrimenti la cella punta a TM-37 e la via più semplice resta non tracciata.

**L'argomento comune, isolato:** *l'attore non ha percorso di scrittura verso l'asset,
quindi n/a*. Confonde "non può falsificare" con "non può causare perdita". Passate le
sei `n/a` della colonna `T-06` a questo metro:

- `A-01` ledger, `A-09` sandbox, `A-13` catalogo — **reggono**. Nessuna perdita per
  negazione: un saldo non cambia perché qualcuno è irraggiungibile, e i moduli sono
  indirizzati per contenuto.
- `A-08` risorse — **regge al limite**. "Non impone carico oltre il traffico
  ordinario" sopporta molto peso ora che, per [RF-006], nessun peer può pre-filtrare
  per Peer ID e ogni tentativo costa handshake, certificato, attestazione e
  interrogazione del ledger.
- `A-04` set validatori — **è la cella da rifare, ed è la sola che segnalo.** "Nessun
  potere sulla composizione" è lo stesso argomento appena falsificato: l'eleggibilità
  è ancorata a lavoro dimostrato, e chi isola dei candidati ne fa fallire le challenge
  e quindi ne altera l'eleggibilità, cioè la composizione. Non affermo che sia falsa —
  non ho letto le regole di eleggibilità con la profondità che servirebbe — ma poggia
  sull'argomento appena caduto e va sottoposta allo stesso test.

### 3. TM-28 lettera (b) — dice il vero

Verificata frase per frase contro le regole che la tengono. "Toglie il legame già
fatto" e "sposta il costo da lettura gratuita e retroattiva a partecipazione attiva e
contemporanea" corrispondono a [ADR-015]. "La misura tiene solo perché" rinvia a due
cose che esistono e che ho verificato: la regola di validità in `identity.md`
§*Key hierarchy* punto 2 e §*Mandatory rejection rules* regola 1, e la guardia in
`TransportKeyAttestation::verify`. La ritrattazione di "elimina" è scritta come
ritrattazione e non come riabilitazione, che è la regola di forma di
`recurring-defects.md` famiglia 2. E la misura non è più intitolata *Rotazione*: il
paragrafo dice ora che i documenti specificano la possibilità di ruotare e un
intervallo **minimo**, nessun pavimento, e che un nodo che non ruota mai ha un Peer ID
stabile a vita. È esatto, e non ci trovo altro da correggere.

### 4. [DEBT-017] — classificazione confermata, con una precisazione sulla forma

**La lettura del Lead è corretta ed è famiglia 3.** Verificata per costruzione: chi
detiene la chiave pone `created_at_ms = now + max_future_skew_ms` ed
`expires_at_ms = created + max_validity_ms`; l'oggetto è accettato da `now` fino a
`now + skew + validity`. La grandezza vincolata è la durata dichiarata, la grandezza
da cui la proprietà dipende è la campata di accettazione, e sono due.

**Il documento contiene già l'argomento che lo dimostra, e non lo ha guardato.**
`identity.md` §*Bounded validity in time* punto 4 scrive che nessuna tolleranza è
concessa oltre `expires_at_ms` "because slack there extends the exposure window that
rule 3 exists to bound". È la stessa aritmetica: lo slack prima di `created_at_ms`
estende la stessa finestra della stessa quantità. L'asimmetria rivendicata è vera
sullo **scopo** — la tolleranza in avanti serve a un verificatore onesto con
l'orologio lento, quella oltre la scadenza non serve a nessuno — e falsa sull'**effetto
sulla finestra**. Le due cose sono confuse in una frase sola.

**Una via che sembra più elegante e non lo è, riportata perché altri la proveranno.**
Limitare la campata dal lato del verificatore — rifiutare se
`expires_at_ms - now_ms > max_validity_ms`, o clampare l'inizio effettivo a
`min(created_at_ms, now_ms)` — non funziona: al verificatore onesto con l'orologio
indietro di `d` risulta `expires - now = V + d`, quindi un'attestazione onesta con `V`
vicino al tetto viene **rifiutata**, che è esattamente il guasto che la tolleranza
esisteva per impedire. Il costo cade su chi la tolleranza doveva proteggere.
Verificato lavorando il caso, non assunto.

Resta quindi la prima uscita ammissibile già scritta nel debito — un vincolo
relazionale nel blocco di validità dei `consensus_parameters`, che vincoli la somma e
non i due addendi separatamente — con la sua fixture di frontiera e la prova in
negativo. Sul rapporto con `max_envelope_validity_ms` mi pronuncerò nella passata del
debito: le due finestre limitano proprietà diverse, impersonificazione l'una e
ritenzione della cache di replay l'altra, e non è affatto detto che debbano essere
ordinate. **Accetto la titolarità.**

### 5. La classe dichiarativa per l'inventario: **opzionale, e non la chiedo**

La proposta era mia ed è giusto che sia io a dire che non serve. La chiusura scelta è
**più forte** di quella che l'inventario avrebbe dato: una regola di validità è
applicata a runtime da ogni ricevente su ogni attestazione, mentre una classe di
difetto guarda le fixture a tempo di build. Per questa proprietà l'inventario sarebbe
ridondante. La classe resterebbe utile per la forma generale — lo stesso valore in due
ruoli che il protocollo richiede distinti — ma è lavoro sull'inventario, appartiene a
una spec propria, e non è una condizione di questa. Non diventa lavoro per mia
richiesta.
