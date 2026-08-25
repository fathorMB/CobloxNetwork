---
id: REVIEW-019
# Note: Quote the title if it contains a colon
title: "Security review of SPEC-012 (GATE-SECREVIEW): il verificatore Ed25519 consensus-critical"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-012
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, correctness, test-quality, provenance]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-019-EVENT-001"
    timestamp: "2026-08-25T21:36:23.018764800+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-019-EVENT-002"
    timestamp: "2026-08-25T21:43:25.009994600+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Un finding high, due medium, due low, nessuno a carico di verifier.rs che risulta corretto.\n\nRF-001 verificato dal Lead ricostruendo la divergenza con il proprio oracolo, non leggendola. Confermato che nessuna delle 24 codifiche di punto della fixture ha y mascherata maggiore o uguale a p, quindi la clausola normativa sulle codifiche non canoniche ridotte modulo 2^255-19 non e esercitata da alcun oracolo del progetto. Confermato che i vettori 8-11 hanno y uguale a p-1, che e canonica: sono il caso x uguale zero con bit di segno uno, caso diverso che la regola 1 non nomina. Confermata la divergenza costruita: con R_enc uguale alla codifica little-endian di p+1, k uguale a SHA-512 di R_enc concatenato A_enc concatenato M ridotto mod L, e S uguale a k per a mod L, la regola Coblox come scritta accetta e la decodifica che rifiuta y maggiore o uguale a p rifiuta.\n\nUna precisazione del Lead che rende il finding piu forte e non piu debole. La review afferma che entrambe le implementazioni passano tutti e dodici i vettori; il Lead ha misurato che questo e vero per l'implementazione intermedia e falso per quella pienamente stretta secondo RFC 8032, che fallisce il vettore 9 perche rifiuta x uguale zero con bit di segno uno. La distinzione identifica esattamente quale classe di seconda implementazione e pericolosa, ed e la classe plausibile: chi segue ZIP-215 sul bit di segno e la RFC sulla canonicita di y. Quell'implementazione e identica a Coblox su tutti e dodici i vettori e diverge sull'input costruito, verificato dal Lead. Costo per l'attaccante nullo e innescabile a comando da qualunque possessore di chiave su un voto di finalita.\n\nRF-002 e la stessa lacuna vista dal lato della prosa: la regola 1 nomina un comportamento che nessun vettore esercita e tace su quello che quattro vettori esercitano, e la prosa su 8 e 9 chiama non canonica una y che e canonica.\n\nRF-003, il contratto della cucitura: message accetta una fetta di byte e Digest32::as_bytes coercisce, quindi un chiamante che passasse un digest compilerebbe e passerebbe ogni test. Oggi nessun chiamante esiste in src, quindi il primo definira la convenzione, ed e l'unica cucitura consensus-critical in cui la separazione strutturale dichiarata in lib.rs non e stata applicata.\n\nRF-004 e RF-005 sono low e vanno chiusi nella stessa passata perche costano poco: la provenienza della fixture dice master branch senza SHA di commit ne digest, il che rende irripetibile la verifica byte per byte che il Lead ha eseguito contro upstream; e l'affermazione libreria vagliata va resa esatta, perche il lockfile porta curve25519-dalek 5.0.0 e sha2 0.11.0, major nuovi non coperti dagli audit citabili."
    evidence_refs: ["SPEC-012", "REVIEW-018", "SPEC-001"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-019-EVENT-003"
    timestamp: "2026-08-25T22:09:47.496220700+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Chiusi quattro finding su cinque. RF-003 non chiuso e riportato invece che forzato, perche la chiusura richiede di modificare verifier.rs che il Lead aveva escluso dal perimetro: promosso a DEBT-016 insieme a OSS-001.\n\nRF-001 chiuso con sette vettori di estensione Coblox in file separato, generati da uno strumento deterministico versionato con --check in CI, che firmano una preimmagine di voto di finalita reale. La fixture upstream resta intatta byte per byte. La prova in negativo e eseguita a ogni run invece che trascritta una volta.\n\nL'implementatore ha ricalcolato invece di concedere e ha ristretto il perimetro che la review chiedeva: due dei quattro casi non sono costruibili come accettazioni perche richiederebbero il logaritmo discreto del punto ridotto, e le codifiche non canoniche la cui riduzione ha ordine piccolo sono esattamente quattro. I vettori 0-3 non sono quindi un campione della classe divergente ma la classe al completo, a meno di chiave e messaggio.\n\nRF-002 chiuso separando la regola 1 in 1a e 1b, ciascuna con il vettore che la esercita, e riscrivendo il paragrafo su 8 e 9 che chiamava non canonica una y canonica. RF-004 chiuso appuntando la provenienza a uno SHA di commit e versionando cases.json upstream verbatim, con due test che leggono il digest atteso dal README della fixture invece che da una costante, cosi che la verifica byte per byte del Lead sia ripetibile da chiunque. RF-005 chiuso rendendo esatta l'affermazione sulla libreria vagliata, senza ritrattare la scelta.\n\nCensita una famiglia 4 in recurring-defects.md, la clausola normativa che nessun oracolo esercita, tenuta distinta dalla famiglia 1 con l'argomento corretto: nella famiglia 1 un artefatto dice una cosa falsa, qui ogni parola era vera e il difetto era che la gate portava evidenza per meta della regola e nessuna evidenza per l'altra."
    evidence_refs: ["SPEC-012", "DEBT-016"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-019-EVENT-004"
    timestamp: "2026-08-25T22:10:04.496000400+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificato dal Lead rieseguendo. 124 test passati contro i 119 di prima, clippy zero warning, fmt pulito, tutti e cinque gli strumenti versionati PASS. La fixture upstream e verifier.rs non hanno un solo byte di differenza, verificato per diff: la proprieta che il Lead aveva stabilito contro upstream non e stata persa e l'implementazione non e stata toccata.\n\nLa restrizione del perimetro proposta dall'implementatore e stata verificata dal Lead per enumerazione esaustiva e non accettata sulla parola. Le codifiche y non canoniche possibili sono 38, cioe y_raw da p a 2^255-1 per due bit di segno; di queste 24 decodificano a un punto valido; e quelle la cui riduzione ha ordine piccolo sono esattamente quattro, y_raw uguale a p e a p+1 per entrambi i segni. I quattro vettori discriminanti sono quindi la classe divergente al completo e non un campione, il che rende il finding piu forte e non piu debole: il costo per l'attaccante non e solo nullo, e enumerabile.\n\nLa precisazione del Lead e ora eseguibile invece che scritta: l'oracolo versionato con decodificatore stretto riporta che l'implementazione intermedia concorda sui dodici upstream e diverge esattamente sui quattro nuovi, e che quella pienamente stretta secondo RFC 8032 e esclusa dal solo vettore 9. Rifacibile con un comando.\n\nLa famiglia 4 censita in recurring-defects.md e giudicata corretta dal Lead nel merito e nella separazione dalla famiglia 1, e la coppia di domande che la intercetta e utile in entrambi i versi: per ogni clausola quale vettore la esercita, e per ogni vettore quale clausola lo nomina.\n\nSul giudizio che l'implementatore ha segnalato invece di prendersi in silenzio, il Lead conferma: la seconda tabella aggiunge un obbligo di conformita e non una regola di validita, nessun esito cambia, e la clausola 1b era gia normativa perche gli esiti pubblicati dei vettori 8-11 sono ottenibili solo accettandola. Segnalarlo era il comportamento corretto."
    evidence_refs: ["SPEC-012", "REVIEW-019", "DEBT-016"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-019-EVENT-005"
    timestamp: "2026-08-25T22:10:17.106405+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata sulla condizione che AGENT-007 aveva dichiarato in anticipo: chiusi RF-001, RF-002 e RF-003 con la prova in negativo trascritta, GATE-SECREVIEW e soddisfatta senza riserve per quanto la riguarda, e verifier.rs non va toccato.\n\nIl Lead dichiara la sola deviazione da quella condizione invece di lasciarla intendere. RF-003 non e chiuso in codice ma promosso a DEBT-016, perche l'unica chiusura possibile richiede di modificare verifier.rs, che il Lead stesso aveva escluso dal perimetro avendo stabilito con lei che l'implementazione e corretta. L'implementatore si e fermato e ha riportato invece di forzare, che e il comportamento che il mandato chiede. Il debito porta la scadenza reale, prima del primo chiamante e non prima della devnet, ed e raggruppato con DEBT-015 perche entrambi sono cambiamenti breaking della stessa API e farne due passate raddoppierebbe il costo per gli stessi consumatori. RF-004 e RF-005, che la condizione non nominava, sono chiusi comunque.\n\nIl Lead ha verificato ogni anello in modo indipendente e non letto dall'evidenza, compresa l'enumerazione esaustiva che conferma la restrizione del perimetro. Nessun finding resta aperto in codice oltre a quello registrato come debito."
    evidence_refs: ["SPEC-012", "DEBT-016", "REVIEW-018"]
    implementation_agent: "AGENT-001"
links: [SPEC-012, REVIEW-018, ADR-003, ADR-012]
created: 2026-08-25
updated: 2026-08-25
tags: [security, review, cryptography, consensus]
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

**Changes requested**, con un finding alto, due medi e due bassi. **Nessuno è a carico di
`core/coblox-core/src/verifier.rs`**, che ho verificato in modo indipendente e che giudico corretto
rispetto alla regola pubblicata, clausola per clausola.

Il finding alto sta dove il Lead mi ha detto di non cercare, e ha la forma che questo progetto
continua a ripetere: **una clausola normativa scritta, e mai eseguita da nessun oracolo.**

La regola 1 di `docs/protocol/README.md#consensus-critical-ed25519-verification` prescrive due
comportamenti distinti di decodifica. I dodici vettori `speccheck` ne esercitano **uno solo**.
L'altro — «non-canonical y-coordinate encodings are accepted and reduced modulo `2^255-19`» — non è
toccato da nessuno dei dodici: ho ispezionato tutte e ventiquattro le codifiche di punto presenti
nella fixture (dodici `pub_key` e dodici `R_enc`) e **nessuna** ha `y` mascherata `>= p`.
`GATE-SPECCHECK` non porta quindi alcuna evidenza per metà della regola 1, e porta evidenza per una
metà che la regola 1 non nomina.

La conseguenza non è teorica. Ho costruito una firma su cui **due implementazioni che passano
entrambe tutti e dodici i vettori danno verdetto opposto**, e che qualunque possessore di chiave può
fabbricare a comando in tempo costante. È esattamente lo scenario che la spec dichiara di voler
escludere, e cade nel punto cieco della gate costruita per escluderlo.

Il resto è solido, e lo dico avendolo provato e non letto. Le tre superfici che il Lead segnalava
come probabili — confine `curve25519-dalek`/Coblox, malleabilità e vincolo scalare, contratto con
`registry::signing_preimage` — reggono; sulla terza ho un finding medio, ma è di forma, non di
sostanza.

## Cosa ho verificato in modo indipendente

Non ho rieseguito i test dell'implementatore né l'oracolo del Lead. Ho scritto **un terzo oracolo da
zero**, aritmetica di Edwards in Python puro con `hashlib` per SHA-512 e nient'altro, con decodifica
ZIP-215 e decodifica RFC 8032 stretta come varianti selezionabili, per poter mettere le due regole a
confronto sullo stesso input. Non condivide una riga con `verifier.rs` né con
`sim/tools/ed25519_speccheck_oracle.py`.

**1. La tabella corretta è corretta.** Il mio oracolo produce, sui dodici vettori della fixture:

```
reject reject accept accept accept accept reject reject reject accept reject reject
```

identica alla riga pubblicata dopo la remediation di [REVIEW-018], **`reject` all'ottavo incluso**.
È la terza derivazione indipendente dello stesso risultato. RF-001 di [REVIEW-018] è confermato.

**2. L'attribuzione dei motivi di rifiuto è corretta.** Il documento afferma che «each rejection
above is produced by exactly one of the rules: `[8]A = identity` on vectors 0, 1, 10 and 11;
`S >= L` on vectors 6 and 7; a failed cofactored equation on vector 8». È un'affermazione compilata
a mano dentro un documento normativo, cioè la forma esatta della famiglia 1 di [ADR-012], quindi
l'ho eseguita invece di leggerla. Ho calcolato per ciascun vettore l'insieme **completo** delle
regole violate:

| Vettore | Esito | Regole violate (insieme completo) |
| --- | --- | --- |
| 0 | reject | `[8]A = identity` |
| 1 | reject | `[8]A = identity` |
| 2–5 | accept | — |
| 6 | reject | `S >= L` |
| 7 | reject | `S >= L` |
| 8 | reject | equazione con cofattore |
| 9 | accept | — |
| 10 | reject | `[8]A = identity` |
| 11 | reject | `[8]A = identity` |

Ogni rifiuto è prodotto da **esattamente una** regola, come il documento dichiara. L'affermazione è
vera.

**3. La semantica di `curve25519-dalek` 5.0.0 corrisponde alla regola, letta nel sorgente e non
nella documentazione del crate.** Ho aperto
`~/.cargo/registry/src/index.crates.io-*/curve25519-dalek-5.0.0/src/`:

| Regola | Chiamata | Semantica verificata nel sorgente |
| --- | --- | --- |
| 1 | `CompressedEdwardsY::decompress` (`edwards.rs:211`, `decompress::step_1`/`step_2`) | `FieldElement::from_bytes` maschera il bit 255 e lavora mod `p` in modo lazy, quindi `y >= p` è accettata e ridotta; `X.conditional_negate(sign)` è applicata **incondizionatamente**, senza il rifiuto RFC 8032 §5.1.3 passo 3 per `x = 0, x_0 = 1`. Conforme a ZIP-215 e alla regola Coblox. |
| 2 | `Scalar::from_canonical_bytes` (`scalar.rs:259`) | `high_bit_unset & candidate.is_canonical()`, dove `is_canonical` è `self.ct_eq(&self.reduce())`. Esattamente `0 <= S < L`; `S = 0` ammesso, come la regola richiede. |
| 3 | `EdwardsPoint::is_small_order` (`edwards.rs:1405`) | `self.mul_by_cofactor().is_identity()`, e `mul_by_cofactor` è `mul_by_pow_2(3)`. Esattamente `[8]A = identità`. |
| 4 | `vartime_double_scalar_mul_basepoint` + `mul_by_cofactor` | La trasformazione algebrica scritta nel commento è corretta: `R' = [k](-A) + [S]B = [S]B - [k]A`, e `[8](R - R') = O` se e solo se `[8][S]B = [8]R + [8][k]A`. |

Nessuna delle cinque condizioni è delegata a un default di libreria non ispezionato. La condizione
propria di Coblox — `[8]A != identità`, che **non è ZIP-215** e che una libreria ZIP-215 corretta non
applicherebbe — è aggiunta esplicitamente sopra lo strato, nel punto giusto e con la semantica giusta.

**4. La firma non è malleabile.** Ho costruito una firma valida ordinaria e provato le tre
trasformazioni:

| Trasformazione | Esito | Perché |
| --- | --- | --- |
| `S -> S + L` | **rifiutata** | `from_canonical_bytes` esige `S < L`. |
| `R -> R + T`, `T` di ordine 2 | **rifiutata** | `R_enc` entra in `k = SHA-512(R_enc \|\| A_enc \|\| M)`: cambiare `R` cambia `k`, e la stessa `S` non soddisfa più l'equazione. |
| `A -> A + T`, `T` di ordine 2 | **rifiutata** | idem, `A_enc` è nel preimage di `k`. |

Il rischio classico della verifica **con cofattore** è che `[8](R+T) = [8]R` renda la firma
malleabile per aggiunta di torsione. Qui non lo è, perché `R_enc` e `A_enc` sono nell'hash. È una
proprietà che regge per costruzione e che nessuno aveva messo per iscritto: la scrivo qui perché è il
primo argomento che un revisore esterno cercherà.

**5. Nessun identificatore dipende dai byte di firma.** Ho controllato il registry: `message_id`,
`challenge_id`/`request_hash` e `response_hash` sono tutti calcolati sull'oggetto **privo** della
firma (`envelope_without_id_or_signature`, `request_without_id_or_signature`,
`response_without_signature`). Anche se una malleabilità comparisse in futuro, non sposterebbe un
identificatore. La difesa è doppia e indipendente.

**6. La catena di fornitura è igienicamente in ordine.** `Cargo.lock` è versionato con i checksum,
CI usa `cargo build --locked` e `cargo test --locked`, `cargo-deny-action` è appuntata per SHA di
commit (`3c6349835b2b…`), `[advisories] ignore = []` è vuoto, e l'oracolo indipendente è nel job CI
(`sim/tools/ed25519_speccheck_oracle.py --explain`). Il parser di
`published_outcomes_from_document` è genuino: `include_str!` del documento, delimitazione della
sezione all'heading successivo, rifiuto esplicito di una seconda riga duplicata, `panic!` su ogni
forma inattesa invece di un default silenzioso. RF-002 di [REVIEW-018] è chiuso nel modo giusto.

## Il finding alto, per esteso

### La regola 1 ha due metà, e l'oracolo ne copre una

Le codifiche a 32 byte con `y` mascherata in `[p, 2^255-1]` sono un insieme piccolo e concreto: dei
19 valori possibili di `y_raw = p + t`, **24 codifiche** (contando il bit di segno) decodificano a un
punto valido sotto la regola Coblox. Un decodificatore RFC 8032 stretto le rifiuta **tutte**, perché
RFC 8032 §5.1.3 impone `y < p`.

Nessuna compare nella fixture. Le quattro codifiche che la fixture chiama «non-canonical»
(`ec ff … ff ff` su `R_enc` ai vettori 8 e 9, su `A_enc` ai vettori 10 e 11) hanno `y` mascherata
`= 2^255 - 20 = p - 1`, che è **canonica**. Non sono un caso di riduzione: sono il caso `x = 0` con
il bit di segno a 1, cioè il punto di ordine 2 `(0, -1)`, che RFC 8032 rifiuta e ZIP-215 accetta. È
un caso diverso, che la regola 1 non nomina affatto (vedi RF-002).

**La divergenza costruita.** Prendo una chiave qualsiasi `a`, `A = [a]B`, un messaggio `M` nella
forma di `signing_preimage(SIG_BLOCK_VOTE, …)`, e pongo:

- `R_enc = LE(p + 1)` con bit di segno 0 — codifica non canonica dell'**identità**;
- `k = SHA-512(R_enc || A_enc || M) mod L`;
- `S = k · a mod L`.

Allora `[S]B = [k]A` e `[8]R = O`, quindi `[8][S]B = [8]R + [8][k]A` **vale**. Esiti osservati:

```
  A_enc : 40147a199bce475942df78e84a62a5a5aa6abf46b1caddede017c3b1fc3ef9e5
  R_enc : eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f
  regola Coblox come scritta (riduzione y, ZIP-215) : ACCEPT
  stessa regola con decodifica stretta y < p        : REJECT
```

Entrambe le implementazioni passano tutti e dodici i vettori. Divergono su questa firma.

**Perché è alto.** Il costo per l'attaccante è nullo: un validatore emette un voto di finalità così
costruito e la rete si divide sulla sua validità, a comando, senza compromettere alcuna chiave. La
regola 1 è la **prima** che un secondo implementatore scriverà, e la lettura naturale — usare
`ed25519-dalek::verify_strict`, o un decodificatore RFC 8032, o quasi qualunque libreria Ed25519 non
ZIP-215 — produce proprio il rifiuto di `y >= p`. Il documento vieta quelle sostituzioni con un
`MUST NOT`, ma il `MUST NOT` è prosa: **la gate che dovrebbe renderlo esecutivo non lo rende
esecutivo**, e la spec dice per iscritto che un verificatore non validato sui casi limite è
indistinguibile da uno corretto fino alla divisione della catena.

**Perché non è a carico di `verifier.rs`.** L'implementazione fa la cosa giusta: `curve25519-dalek`
riduce, come la regola prescrive. Il difetto è nell'evidenza, cioè nel perimetro dell'oracolo, cioè
in ciò per cui `GATE-SECREVIEW` esiste.

## Acceptance-criteria compliance

| Criterio | Esito | Nota |
| --- | --- | --- |
| Implementazione di `SignatureVerifier` con i quattro punti più il rifiuto di ordine piccolo | soddisfatto | Verificato clausola per clausola nel sorgente di `curve25519-dalek` 5.0.0, non nella sua documentazione. |
| Equazione **con cofattore**, assenza della forma senza cofattore dimostrata | soddisfatto | `gate_cofactor_differential_verification` esibisce l'esito opposto sul vettore 4. Verificata anche la trasformazione algebrica del commento. |
| Hash per `k` sulle codifiche originali, con test differenziale | soddisfatto | La coppia 8/9 inverte gli esiti sotto la variante che ricodifica. Confermato dal mio oracolo. |
| Dodici vettori versionati con la provenienza | **soddisfatto con riserva** | La provenienza dice «Master branch», senza SHA di commit. Vedi RF-004. |
| Esito osservato accanto a quello pubblicato, riga per riga | soddisfatto | Tre colonne distinte, documento estratto con `include_str!`. |
| Scelta libreria/aritmetica motivata, equivalenza sui casi limite **mostrata** | **soddisfatto solo sui dodici vettori** | L'equivalenza è mostrata dove i vettori guardano. Metà della regola 1 non è mostrata da nessuna parte. Vedi RF-001. |
| Contratto di `registry::signing_preimage`: messaggio, non digest | **soddisfatto nella lettera, non strutturalmente** | Vedi RF-003. |
| Limite dichiarato in `coblox-core` aggiornato | soddisfatto | `lib.rs` dichiara il modulo e la tabella dei moduli lo elenca. |
| `cargo-deny` passa e la scelta è giustificata | **soddisfatto con riserva** | Passa. La giustificazione è a livello di crate, non di versione. Vedi RF-005. |

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

**RF-001 | category=test-quality | severity=high | criterion=«La scelta fra libreria vagliata e
aritmetica propria è motivata nel merito, e se si usa una libreria l'equivalenza sui casi limite è
mostrata, non affermata» + GATE-SPECCHECK**

I dodici vettori non esercitano la riduzione `y mod 2^255-19` che la regola 1 prescrive: nessuna delle
24 codifiche di punto della fixture ha `y` mascherata `>= p`. Metà della regola 1 è normativa e non
validata.

*Scenario d'attacco.* Un validatore possessore di chiave costruisce `R_enc = LE(p+1)`, calcola
`k = SHA-512(R_enc || A_enc || M) mod L` e pone `S = k·a mod L`. La firma soddisfa l'equazione con
cofattore perché `[8]R = O`. Un nodo che decodifica secondo la regola come scritta accetta; un nodo
la cui libreria rifiuta `y >= p` — cioè quasi ogni libreria Ed25519 non ZIP-215, incluse quelle che
il documento vieta ma che la gate non può escludere — rifiuta. Entrambi passano i dodici vettori.
Divisione della catena su un voto di finalità, a comando, costo nullo, nessuna chiave compromessa.

*Condizione di chiusura.* Esiste un file di vettori di **estensione Coblox**, distinto dalla fixture
upstream che resta identica byte per byte alla fonte, contenente almeno quattro casi con `y >= p`:
`R_enc` che riduce a un punto di ordine grande, `A_enc` che riduce a un punto di ordine grande,
`R_enc` che riduce all'identità, `A_enc` che riduce a un punto di ordine piccolo (rifiuto per la
regola 3). L'esito atteso di ciascuno è **derivato dalla regola pubblicata** e confermato da
`sim/tools/ed25519_speccheck_oracle.py`, mai trascritto dall'implementazione: è la stessa disciplina
di RF-002 di [REVIEW-018]. Il test Rust li esegue riga per riga come i dodici. Il documento di
protocollo dichiara che la conformità richiede questa seconda tabella oltre a quella upstream, e dice
perché. Prova in negativo: sostituendo la decodifica con una che rifiuta `y >= p`, i dodici vettori
restano verdi e i nuovi falliscono. La riga pubblicata dei dodici vettori non cambia.

**RF-002 | category=documentation | severity=medium | criterion=regola 1 di
`docs/protocol/README.md#consensus-critical-ed25519-verification`**

La regola 1 nomina un comportamento che nessun vettore esercita e tace su quello che quattro vettori
esercitano. Nel dettaglio: (a) «non-canonical y-coordinate encodings are accepted and reduced modulo
`2^255-19`» descrive il caso `y >= p`, assente dalla fixture; (b) il caso realmente presente ai
vettori 8–11 è `x = 0` con bit di segno 1, che RFC 8032 §5.1.3 passo 3 impone di rifiutare e ZIP-215
di accettare, e su cui la regola 1 non dice **nulla**; (c) la prosa afferma che i vettori 8 e 9
portano «the same non-canonical `R_enc` (`y = 2^255 - 20`)», ma `2^255 - 20 = p - 1 < p` è una `y`
perfettamente canonica: la descrizione del meccanismo è sbagliata anche se la conclusione è giusta;
(d) di conseguenza «reduced R» in quel paragrafo non significa «`y` ridotta mod `p`» ma «punto
ricodificato canonicamente, cioè con il bit di segno azzerato», e il documento non lo dice.

*Scenario d'attacco.* Indiretto ma reale: un secondo implementatore che segue il testo della regola
non ha istruzione sul caso `x = 0, x_0 = 1`, applica RFC 8032 e rifiuta il vettore 9. Questa
direzione **è** intercettata dalla tabella, quindi la severità è media e non alta; ma un documento
normativo che descrive il proprio oracolo in modo inesatto è la stessa forma di difetto che
[ADR-012] censisce, su un documento che dichiara `MUST` a implementatori esterni.

*Condizione di chiusura.* La regola 1 elenca **entrambe** le condizioni di decodifica, ciascuna con
il vettore che la esercita accanto: riduzione di `y >= p` (vettori di estensione di RF-001) e
accettazione di `x = 0` con bit di segno 1 (vettori 8–11), con il riferimento esplicito a RFC 8032
§5.1.3 passo 3 come alla cosa che Coblox **non** fa. Il paragrafo su 8/9 dice «stessa `R_enc`, punto
di ordine 2 codificato con il bit di segno a 1» invece di «non-canonical `y`», e chiarisce che
«reduced» significa ricodificato canonicamente.

**RF-003 | category=security-boundary | severity=medium | criterion=«Il verificatore rispetta il
contratto di `registry::signing_preimage`: riceve il messaggio, non un suo digest»**

La firma del metodo è `fn verify(&self, public_key: &[u8; 32], message: &[u8], signature: &[u8; 64])`.
`signing_preimage` restituisce `Vec<u8>`, ma `Digest32::as_bytes()` restituisce `&[u8; 32]` che
coercisce a `&[u8]`: **un chiamante che passa un digest compila, passa ogni test esistente, e ottiene
un verificatore che funziona e non verifica nulla di ciò che si crede.** Il contratto è documentato in
due doc-comment e provato da un solo test sull'uso corretto, che non può vincolare un chiamante futuro.

Rilevo inoltre che **oggi non esiste alcun chiamante**: `verify_consensus_ed25519` e
`ConsensusVerifier` non compaiono in nessun file di `src/` fuori dalla propria definizione. Il
contratto ha quindi zero applicazione pratica, e il primo chiamante lo definirà di fatto.

*Scenario d'attacco.* Un chiamante futuro verifica su `message_id` (32 byte) invece che sul preimage.
Il verificatore accetta. Il legame al `chain_id` e al dominio non è più nella cosa firmata ma solo nel
calcolo del digest, quindi il vincolo che [SPEC-001] chiama «every Coblox signature input is the ASCII
domain shown by the schema» decade in silenzio, e una firma emessa in un dominio diventa riutilizzabile
in ogni altro contesto che verifichi un digest a 32 byte. Nessun errore, nessun test rosso, effetto
visibile solo su una rete viva.

*Condizione di chiusura.* Il parametro `message` ha un tipo che solo `registry::signing_preimage` e i
suoi derivati (`block_vote_preimage` e simili) possono costruire — per esempio un newtype
`SigningPreimage` senza costruttori pubblici alternativi — così che passare un digest non compili. È
la convenzione che `lib.rs` dichiara per il resto del crate («la seconda serializzazione non è
costruibile», «la separazione di dominio è strutturale»): il verificatore è l'unica cucitura
consensus-critical dove non è stata applicata, ed è quella dove l'errore è silenzioso. Se il Lead
giudica il cambio di firma prematuro finché non esiste un chiamante, l'alternativa accettabile è un
debito registrato con condizione di scadenza esplicita «prima del primo chiamante», non un
doc-comment in più.

**RF-004 | category=provenance | severity=low | criterion=«I dodici vettori sono versionati nel
repository con la loro provenienza»**

`core/coblox-core/tests/fixtures/README.md` registra «Version / commit: Master branch `cases.json` /
`cases.txt`». `master` è una referenza mobile: la verifica byte per byte che il Lead ha eseguito in
`REVIEW-018-EVENT-003` è irripetibile, perché non c'è modo di sapere contro quale stato di `master` è
stata fatta. Non è un difetto di sicurezza immediato — una fixture manomessa cambierebbe l'esito e
farebbe divergere il documento, che è la guardia — ma la provenienza è la sola cosa che distingue
questi dodici vettori da dodici vettori inventati, e la verifica più costosa di questa spec non è
riproducibile.

*Condizione di chiusura.* Il README della fixture porta lo SHA di commit upstream, lo SHA-256 del
`cases.json` originale, e la data della verifica.

**RF-005 | category=provenance | severity=low | criterion=GATE-DEPENDENCY, «la scelta della libreria
è motivata con la sua provenienza»**

L'evidenza descrive `curve25519-dalek` come «libreria primitiva vagliata». Gli audit pubblici di
`curve25519-dalek` coprono le linee 2.x/3.x/4.x; `Cargo.lock` contiene **5.0.0**, un major nuovo con
superficie API rimaneggiata, e non è citato alcun audit per quella versione. Lo stesso vale per
`sha2 0.11.0`. La provenienza è data a granularità di crate, mentre l'artefatto che finisce nel
binario è la versione appuntata. In un componente dove il difetto è un'accettazione silenziosa, la
distinzione conta.

Registro come mitigazioni già presenti e adeguate: lockfile versionato con checksum, `--locked` in
CI, azione `cargo-deny` appuntata per SHA, `ignore = []` vuoto.

*Condizione di chiusura.* La motivazione nomina la versione, dice quale audit copre quale linea e
dichiara che 5.0.0 non è coperto da un audit citato; oppure la scelta si sposta su una versione
coperta; oppure il residuo è registrato come debito di catena di fornitura con la condizione di
riesame. Vale la pena valutare `cargo-vet`, che è la risposta strutturale a questa classe.

## Osservazioni senza finding

**OSS-001.** `vartime_double_scalar_mul_basepoint` è a tempo variabile su `k` e `S`. È la scelta
**giusta**: tutti gli input della verifica sono pubblici, e la controparte a tempo costante costerebbe
prestazioni senza guadagno. Ma `verifier.rs` non lo dice, e la stessa funzione invita al riuso in un
contesto dove un input potrebbe non essere pubblico. Una frase nei doc di modulo.

**OSS-002.** `curve25519-dalek` è tirato con `default-features = false`, il che disattiva
`precomputed-tables`. È una scelta di dimensione/prestazioni senza effetto sulla correttezza, ma non è
documentata accanto alla dipendenza. Segnalo anche che il crate espone una feature
`legacy_compatibility`, che abilita `Scalar::from_bits`, cioè esattamente la «legacy-compatibility
mode» che il documento vieta: non è abilitata e `from_bits` non è usata, ma l'unificazione delle
feature di cargo potrebbe abilitarla da un altro punto del grafo in futuro. Innocuo oggi perché la
funzione non è chiamata; degno di una riga di commento accanto alla dipendenza.

## Production quality and documentation compliance

Conforme a [[QUALITY]] su tutto ciò che ho potuto verificare. Nessuna scorciatoia, nessun segnaposto,
nessun comportamento noto-incompleto non dichiarato. L'evidenza di consegna porta l'affermazione falsa
**barrata e non riscritta, con l'errore chiamato errore**, che è la disciplina corretta e che voglio
registrare come positiva: è raro e va tenuto.

I due difetti che [REVIEW-018] ha chiuso sono chiusi nel modo giusto — il documento è un input a tempo
di compilazione del binario di test, non una trascrizione — e le guardie sono state provate in negativo
dal verso giusto, cioè reintroducendo il difetto **nel documento**.

## Required follow-up

1. RF-001: vettori di estensione Coblox per la riduzione `y >= p`, con prova in negativo trascritta.
2. RF-002: riscrittura della regola 1 e del paragrafo su 8/9 nel documento di protocollo.
3. RF-003: tipo del parametro `message`, oppure debito con scadenza «prima del primo chiamante».
4. RF-004: SHA di commit upstream e SHA-256 del `cases.json` nel README della fixture.
5. RF-005: provenienza a granularità di versione, oppure debito di catena di fornitura.

RF-001 e RF-002 sono la stessa lacuna vista da due lati: l'oracolo non copre ciò che la regola dice,
e la regola non dice ciò che l'oracolo copre. Vanno chiusi insieme o nessuno dei due è chiuso davvero.

## Final decision

**Changes requested.** RF-001 è la condizione: finché la metà non validata della regola 1 resta non
validata, `GATE-SECREVIEW` non può affermare ciò per cui esiste, cioè che il verificatore rifiuta
tutto ciò che deve rifiutare anche fuori dai dodici vettori. RF-002 e RF-003 sono condizioni di
chiusura; RF-004 e RF-005 sono chiudibili con un paragrafo o promuovibili a debito, a giudizio del
Lead.

Dichiaro in anticipo la base su cui accetterò: chiusi RF-001, RF-002 e RF-003, con la prova in
negativo richiesta da RF-001 eseguita e trascritta, `GATE-SECREVIEW` è soddisfatta senza riserve per
quanto mi riguarda. **`core/coblox-core/src/verifier.rs` non va toccato**: nessuno dei cinque finding
è a suo carico, e l'ho verificato in modo indipendente contro un terzo oracolo scritto da zero.
