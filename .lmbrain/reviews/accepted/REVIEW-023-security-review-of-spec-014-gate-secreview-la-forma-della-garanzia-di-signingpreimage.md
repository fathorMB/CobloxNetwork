---
id: REVIEW-023
# Note: Quote the title if it contains a colon
title: "Security review of SPEC-014 (GATE-SECREVIEW): la forma della garanzia di SigningPreimage"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-014
reviewer: AGENT-007
review_requested_by: AGENT-LEAD
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-023-EVENT-001"
    timestamp: "2026-08-25T23:41:59.228737+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-023-EVENT-002"
    timestamp: "2026-08-25T23:47:04.464452+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Un finding medium e due low. Il medium e reale e verificato dal Lead in ogni passaggio; il low principale e il finding del Lead in REVIEW-022 con il ragionamento invertito, ed e AGENT-007 ad averlo invertito correttamente.\n\nRF-001, medium: la via non-consensus e nominata ma non contenuta. Verificato dal Lead: registry e un pub mod, from_raw_bytes_non_consensus e pub, e core/coblox-core/Cargo.toml non ha alcuna sezione features. La prova che e raggiungibile da fuori era gia in albero e nessuno l'aveva letta come tale: speccheck_conformance.rs e un test di integrazione, quindi un crate esterno, e importa via use coblox_core::registry, e la chiama otto volte. Le otto occorrenze non dimostrano solo che nessun percorso di consenso la tocca, dimostrano che chiunque puo raggiungerla da fuori dal crate senza feature, cioe coblox-node, coblox-ffi e la shell Tauri in una build di produzione. Lo scenario e quello che la sezione Risks della spec descrive alla lettera: il primo chiamante di consenso ha i byte dalla rete e la conversione piu breve che compila e la via d'uscita, che perde il prefisso dominio e chain_id, quindi replay cross-chain di voti fra devnet e mainnet, e non un errore ma un'accettazione.\n\nRF-003, low: e il RF-001 del Lead in REVIEW-022, confermato low ma con il ragionamento invertito, e l'inversione e corretta. Il Lead aveva scritto che pub(crate) e nominale proprio dentro il confine in cui verranno scritti i chiamanti di consenso. E il contrario: il workspace ha coblox-node e coblox-ffi come membri distinti che dipendono da core per path, quindi per il codice che verifichera i voti pub(crate) e un confine esterno e la garanzia e reale. E il delta di capacita e comunque zero, perche un modulo interno che volesse byte arbitrari chiama la funzione pubblica. Restringere il campo va fatto perche e gratuito, non perche chiuda un'ampiezza, e chiude anche la mutazione di una preimmagine gia costruita e non solo la costruzione.\n\nRF-002, low: il tipo non trasporta dominio ne chain_id. Non e un regresso ed e fuori dallo scope dei due debiti; va promosso a debito proprio dal Lead.\n\nUn risultato che AGENT-007 riporta come il piu importante della passata e che nessuno aveva dichiarato, verificato dal Lead: la conversione e completa e non parziale. Tutte e quattro le produttrici di preimmagini dell'albero restituiscono SigningPreimage, tre in registry.rs e una in validator_set.rs, e nessuna resta a Vec<u8>. Una sola lasciata indietro avrebbe costretto il primo chiamante di consenso a usare la via d'uscita per fare il ponte.\n\nVerificata anche la sua correzione sulla copertura: check_magnitudes e stato privatizzato insieme a check_internal ma non aveva alcun chiamante nei test su HEAD, quindi privatizzarlo non poteva costare nulla. I tre chiamanti erano tutti su check_internal."
    evidence_refs: ["SPEC-014", "REVIEW-022", "DEBT-016"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-023-EVENT-003"
    timestamp: "2026-08-25T23:56:40.726984300+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "RF-001 chiusa con entrambe le strade che AGENT-007 aveva nominato, con ruoli distinti e con la misura invece dell'ipotesi. Confine di compilazione portante: una feature non-default conformance-testing, abilitata dalla sola dev-dependency che il crate dichiara su se stesso, con il costruttore sotto cfg. Guardia d'albero come lint: sim/tools/non_consensus_containment.py, nuovo, con prova in negativo integrata ed eseguito dalla CI, prima gate testuale sull'albero sorgente della pipeline.\n\nL'implementatore ha misurato il limite invece di intuirlo, ed e la parte che rende la chiusura usabile: la stessa sonda che non compila in produzione compila sotto cargo test --workspace per feature unification, ed e precisamente il residuo che la guardia testuale copre. Ha inoltre coperto una classe che nessuna delle due opzioni della review nominava, N3-ENABLED: il contenimento disfatto da una riga in un altro manifesto, cioe un dipendente che abilita la feature per se, che nessuna build noterebbe.\n\nLa guardia ha catturato una violazione reale durante la lavorazione, il commento dell'implementatore stesso in ci.yml che nominava il costruttore.\n\nRF-003 chiusa portando il campo da pub(crate) a privato, con la motivazione del Lead ritirata esplicitamente nell'evidenza invece che silenziosamente sostituita, e con la nota di AGENT-007 sulla mutazione di una preimmagine gia costruita raccolta nel commento del tipo.\n\nRF-002 non toccata, come da mandato: e promossa a debito dal Lead."
    evidence_refs: ["SPEC-014"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-023-EVENT-004"
    timestamp: "2026-08-25T23:56:55.488444200+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificato dal Lead scrivendo la sonda dentro coblox-node, cioe un crate dipendente, nella forma esatta dello scenario della review: una funzione che costruisce una preimmagine dai byte presi dalla rete.\n\nBuild di produzione del crate dipendente: error E0599, no associated function or constant named from_raw_bytes_non_consensus found for struct SigningPreimage in the current scope. Il contenimento e reale.\n\nE il limite dichiarato e stato verificato nel verso opposto, che e la prova che conta di piu: la stessa sonda compila sotto cargo test --workspace --no-run, senza errori. Il residuo di feature unification e reale ed e stato dichiarato onestamente invece che taciuto, ed e esattamente cio che la guardia testuale copre. Sonda rimossa e albero verificato pulito.\n\nLa guardia d'albero passa e la sua prova in negativo copre quattro classi, ciascuna osservata fallire e nominata: il chiamante di consenso che prende la conversione piu breve, l'attributo cfg tolto dal costruttore, la feature resa default, e un dipendente che la abilita per se con una riga in un manifesto. L'ultima e la classe che nessuna delle due opzioni della review nominava.\n\nNon regressione verificata dal Lead: 126 test passati, identici a prima della passata e compresi quelli della suite di conformita che dipendono dalla via non-consensus; cargo clippy --workspace --all-targets --all-features zero warning, il che verifica anche che il percorso sotto feature compili; fmt pulito; Cargo.lock cresciuto di una riga. Nessun valore pubblicato mosso e nessun comportamento del verificatore cambiato."
    evidence_refs: ["SPEC-014", "REVIEW-023"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-023-EVENT-005"
    timestamp: "2026-08-25T23:57:03.949055600+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Tutti e tre i finding chiusi e verificati dal Lead in modo indipendente, compreso il limite dichiarato verificato nel verso che lo espone. Nessun finding resta aperto in codice; RF-002 e promosso a debito proprio dal Lead, come il mandato prevedeva.\n\nLa chiusura di RF-001 e migliore delle due opzioni che la review proponeva, perche le usa entrambe con ruoli distinti e perche copre una classe che nessuna delle due nominava."
    evidence_refs: ["SPEC-014", "REVIEW-022", "DEBT-016", "DEBT-015"]
    implementation_agent: "AGENT-001"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review, security]
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

**Raccomandazione: accettare.** Il lavoro regge, e il criterio che la spec temeva di più — che il tipo *vincoli* invece di limitarsi a esistere — è soddisfatto sui due punti d'ingresso.

La domanda che il Lead mi ha posto ha però una risposta che **inverte** il ragionamento di RF-001 di [REVIEW-022]: `low` non è sottostimato, è **corretto o addirittura generoso**, e per una ragione che rende visibile dove la garanzia si romperà davvero. Il confine `pub(crate)` non è dove verrà scritto il codice dei chiamanti di consenso, e non concede alcuna capacità che la via nominata non conceda già a chiunque.

Tre finding, nessuno bloccante: uno `medium` che è la risposta concreta alla domanda «esiste una forma che la renda inaccessibile invece che sconsigliata», e due `low`.

## Acceptance-criteria compliance

Non ho rifatto le prove che il Lead ha già rifatto compilando (E0308 su `lib.rs:143` e `verifier.rs:78`, otto occorrenze della via non-consensus in `speccheck_conformance.rs`, 126 test, cinque strumenti versionati). Le do per acquisite e mi sono concentrata sulla **forma** della garanzia.

Ho verificato in modo indipendente tre cose che nessuno aveva ancora guardato.

**1. La conversione è completa, non parziale.** Ho enumerato tutte le funzioni dell'albero che producono una preimmagine di firma. Sono quattro, e **tutte e quattro** restituiscono ora `SigningPreimage`:

- `registry.rs:361` `signing_preimage`
- `registry.rs:373` `block_vote_preimage`
- `registry.rs:388` `transport_key_attestation_signing_preimage`
- `validator_set.rs:114` `consensus_key_binding_preimage`

Questo era il modo silenzioso di fallire: **una** funzione lasciata a `Vec<u8>` avrebbe costretto il primo chiamante di consenso a usare `from_raw_bytes_non_consensus` per fare il ponte, legittimando la via d'uscita proprio sul percorso da cui deve stare fuori. Non è successo. È il risultato più importante di questa passata e non era stato dichiarato.

**2. Il tipo non ha altre superfici.** `registry.rs` non contiene alcun `impl Deref`, `impl From<Vec<u8>>`, `AsMut`, `AsRef`, né alcuna derive di serde su `SigningPreimage`. Le derive sono `Debug, Clone, PartialEq, Eq`, tutte innocue su un valore non segreto. Un solo sito costruisce il campo direttamente (`registry.rs:368`, dentro il modulo di definizione). Non esiste un secondo punto d'ingresso di verifica: `signature: &[u8; 64]` compare in tre righe, tutte e tre appartenenti alle due funzioni già tipizzate.

**3. La copertura di [DEBT-015] è conservata, e ho verificato il caso che il Lead non ha nominato.** `RewardPolicy::check_magnitudes` è stato privatizzato *insieme* a `check_internal`, ma `git grep` su `HEAD` mostra che `check_magnitudes` **non aveva alcun chiamante di test** prima della passata: gli unici tre erano su `check_internal`, tutti riscritti. Privatizzarlo non ha quindi potuto costare nulla, e non è una copertura persa senza che nulla diventi rosso. La riga 526 asserisce `validate_reward(&base).expect("the reward PD-0 values are admissible")`, che è ciò che rende differenziale l'argomento. Le asserzioni originali non nominavano la variante d'errore (`check_internal().is_err()`), quindi il passaggio a `validate_reward(...).is_err()` non perde precisione: non ce n'era da perdere.

## Code observations

### Dove il confine è davvero, e perché RF-001 di [REVIEW-022] è meno di quanto sembra

RF-001 di [REVIEW-022] osserva che il campo è `pub(crate)` e conclude che il confine interno al crate è «proprio dentro il confine in cui verrà scritto il codice dei chiamanti di consenso». **Ho verificato che non è così, su due fatti indipendenti.**

**Primo: i chiamanti di consenso non nasceranno dentro `coblox-core`.** Il workspace ha tre membri — `coblox-core`, `coblox-node`, `coblox-ffi` — più `apps/desktop/src-tauri`, e tutti e tre gli altri dipendono da `coblox-core` per *path*. `coblox-core` è la libreria di protocollo; il nodo è `coblox-node`. Per il codice che verificherà i voti, `pub(crate)` è un confine **esterno**, e la garanzia è quindi reale, non nominale, esattamente dove il Lead temeva fosse nominale.

**Secondo, e decisivo: `pub(crate)` sul campo non concede nulla che la via nominata non conceda già a tutti.** `from_raw_bytes_non_consensus` è dichiarata `pub`, dentro `pub mod registry`, con il tipo re-esportato alla radice. Non è dietro alcuna feature: `core/coblox-core/Cargo.toml` non ha affatto una sezione `[features]`. Un modulo interno al crate che volesse byte arbitrari **non ha bisogno del campo**: chiama la funzione pubblica, come farebbe chiunque.

La prova non richiede una sonda, perché è già in albero: `speccheck_conformance.rs` è un test d'integrazione, cioè un **crate esterno** (`use coblox_core::registry::{SigningPreimage, signing_preimage};`, riga 29), e chiama `SigningPreimage::from_raw_bytes_non_consensus` otto volte. Ciò che le otto occorrenze dimostrano non è solo che nessun percorso di consenso la tocca — è che **la via è raggiungibile da fuori dal crate, in una build ordinaria, senza alcuna feature**.

Ne segue che restringere il campo a privato è una pulizia corretta e a costo zero — la correzione è una parola chiave, come dice RF-001 di [REVIEW-022] — ma **non chiude nulla**. Il delta di capacità è nullo. `low` è la severità giusta; se qualcosa, è il *rimedio* a essere sopravvalutato, non la severità.

### Ciò che il tipo garantisce davvero

Vale la pena scriverlo, perché è la frase che i chiamanti futuri leggeranno. `SigningPreimage` garantisce, in modo forte e verificato: **il parametro non è un `Digest32` e non è una fetta di byte nuda.** È esattamente il bersaglio di [DEBT-016], è la confusione che nel v0 costa un'accettazione silenziosa, e tiene.

Non garantisce, e non è scritto da nessuna parte che non garantisca: che i byte siano legati alla catena, che siano separati per dominio, o che il dominio sia quello giusto per il messaggio in esame. Su questo, i due finding che seguono.

## Tests and verification

Nessuna copertura persa (verificato sopra in modo indipendente). Nessun test, però, **cattura una violazione futura**: non esiste alcun controllo — né `cargo`, né CI, né uno strumento versionato — che si accorga se un domani `from_raw_bytes_non_consensus` compare in `coblox-node`. `.github/workflows/ci.yml` non contiene alcuna gate testuale sull'albero, mentre `sim/tools/published_artifacts.py` mostra che l'inventario a livello d'albero è già un modo di casa per tenere un invariante. È la condizione di chiusura che propongo in RF-001.

## Production quality and documentation compliance

La documentazione del tipo è buona e onesta: il commento di `registry.rs` nomina la natura non-consensus della via, il `# Warning` sul costruttore è esplicito, e la locuzione di `verifier.rs` ora rimanda a `Cargo.toml` invece di dire «audited», che è dove RF-005 di [REVIEW-019] l'aveva resa esatta. Nessun valore pubblicato mosso. Nessuna deviazione non dichiarata trovata.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

- **RF-001 | category=security-boundary | severity=medium | criterion=«Il tipo non è costruibile da byte arbitrari se non attraverso una via nominata e documentata come non-consensus»**

  **La via non-consensus è nominata, non contenuta.** `SigningPreimage::from_raw_bytes_non_consensus` è `pub`, in un `pub mod`, senza alcuna feature che la circoscriva (`coblox-core/Cargo.toml` non ha una sezione `[features]`). È raggiungibile da `coblox-node`, da `coblox-ffi` e da `apps/desktop/src-tauri` in una build di **produzione**, non solo di test. Ciò che ferma un chiamante di consenso oggi è il nome, il commento e la review: nulla che il compilatore o la CI possano far fallire.

  Scenario d'attacco concreto, e il motivo per cui non è `low`. Il primo chiamante di consenso deve verificare un voto di finalità. Ha in mano i byte del messaggio che arrivano dalla rete e una funzione la cui firma pretende un `SigningPreimage`. La conversione più breve che compila è `from_raw_bytes_non_consensus(&bytes_dalla_rete)`, ed è quella che un chiamante frettoloso scriverà, perché il tipo che dovrebbe fermarlo gli offre lui stesso l'uscita, pubblica e a un metodo di distanza. Il risultato ha il tipo giusto ed è semanticamente falso: la preimmagine perde il prefisso `dominio || 0x00 || chain_id`, quindi la firma verificata non è più legata **né al dominio né alla catena**. Un voto raccolto su una devnet verifica su mainnet e viceversa, e un messaggio firmato in un dominio vale in un altro. Non si produce un errore: si produce un'accettazione. È letteralmente il rischio che la sezione *Risks* della spec dichiara come sola ragione per cui questa gate esiste.

  Nulla di tutto ciò è vero **oggi**: nessun percorso di consenso la tocca, e le otto occorrenze sono tutte nella suite di conformità. Il finding è sulla forma della garanzia nel momento in cui i chiamanti verranno scritti, che è ciò che questa spec esiste per preparare.

  **Rimedio (una delle due, non entrambe necessarie):**

  1. *Contenimento di build.* Mettere `from_raw_bytes_non_consensus` dietro una feature non-default (per esempio `conformance-testing`), abilitata solo dalla dev-dependency che `coblox-core` dichiara su sé stesso per i propri test d'integrazione. Onestà sul limite: la feature unification può riaccenderla per l'intero grafo durante un `cargo test --workspace`, quindi è una garanzia sulle build di produzione (`cargo build -p coblox-node --release`), non un divieto assoluto. È comunque il salto da «sconsigliata» a «non compilabile dove conta», ed è la risposta alla domanda posta nel dispatch.
  2. *Contenimento d'albero.* Uno strumento versionato nello stile di `published_artifacts.py`, oppure un test, che asserisca che le occorrenze di `from_raw_bytes_non_consensus` fuori da `core/coblox-core/tests/` sono zero, e che fallisca la CI altrimenti.

  **Condizione di chiusura verificabile:** un tentativo di chiamare `from_raw_bytes_non_consensus` da `coblox-node` fallisce — la build nel caso 1, la CI nel caso 2 — e il fallimento è riportato in trascrizione. Se resta la sola opzione 2, il rimedio va accompagnato da una riga nel commento del tipo che dica dove il controllo vive, perché un controllo che nessuno sa di avere non è un controllo.

- **RF-002 | category=security-boundary | severity=low | criterion=nessuno (residuo dichiarato fuori scopo dalla spec)**

  **Il tipo non trasporta né dominio né `chain_id`.** `signing_preimage(domain, chain_id, payload)` è `pub` e accetta un `Domain` qualsiasi e un payload qualsiasi; il prefisso viene impastato nei byte e il tipo lo dimentica. Un chiamante che costruisse la preimmagine di un voto con `Domain::SIG_TRANSPORT_KEY_ATTESTATION` otterrebbe un valore ben tipato e semanticamente falso, e nessun controllo lo intercetterebbe: la separazione di dominio esiste nei byte, ma la scelta del dominio giusto resta una responsabilità del chiamante, non una proprietà del valore.

  Questo **non è un regresso** — è identico a prima della spec, ed è esplicitamente fuori dai due debiti che la spec chiude. Lo registro perché è, un livello sopra, la stessa domanda che [DEBT-016] pone: una garanzia tenuta dalla convenzione invece che dal tipo. Ed è il residuo che morderà per secondo, dopo RF-001.

  **Rimedio:** non in questa spec. Candidato a un debito proprio, con due forme possibili: un parametro fantasma di dominio su `SigningPreimage`, oppure — più economico e probabilmente sufficiente — la regola che per ogni tipo di messaggio la costruzione della preimmagine e la chiamata al verificatore stiano nella **stessa** funzione, così che il dominio non possa divergere fra i due siti. **Condizione di chiusura:** l'una o l'altra è scritta e verificata, oppure il residuo è accettato per iscritto con la ragione.

- **RF-003 | category=maintainability | severity=low | criterion=conferma di RF-001 di [REVIEW-022]**

  Il campo `SigningPreimage(pub(crate) Vec<u8>)` (`registry.rs:332`) è più ampio del necessario e va ristretto a privato: il verificatore legge via `as_bytes()` (`verifier.rs:115`) e l'unico sito di costruzione diretta è `registry.rs:368`, nello stesso modulo. La correzione è una parola chiave e non ha costo.

  **Confermo la severità `low` del Lead, e ne correggo la motivazione.** Non è più di `low`, per due fatti verificati: i chiamanti di consenso nasceranno in `coblox-node`, un crate diverso, per cui `pub(crate)` è per loro un confine esterno e reale; e comunque il campo non concede **alcuna** capacità aggiuntiva rispetto a `from_raw_bytes_non_consensus`, che è `pub` e raggiungibile da chiunque. Il delta è nullo. La pulizia va fatta perché è giusta e gratuita, non perché chiuda un'ampiezza: l'ampiezza che conta è quella di RF-001.

  Va aggiunto che `pub(crate)` concede anche la **mutazione** di una preimmagine già costruita, da parte di un modulo del crate che ne tenga un `&mut`, non solo la costruzione. Vale la stessa conclusione: nessuno lo fa, e la parola chiave chiude entrambe.

  **Condizione di chiusura:** il campo è privato e l'albero compila senza altre modifiche.

## Required follow-up

Nessuna delle tre è bloccante. **RF-001 è quella che vale il giro di remediation**, e RF-003 costa una parola chiave nella stessa passata. RF-002 va promosso a debito e non risolto qui.

Una nota sul metodo, per la quinta volta: le superfici indicate nel dispatch reggevano tutte e tre nella sostanza — la copertura è conservata, il tipo vincola, la via è nominata bene — e il finding che porta il peso è nato dal contare i crate del workspace e dal leggere la parola `pub` sul costruttore. Il difetto era di nuovo già scritto, e la ragione per cui non era stato guardato è che sia la spec sia [REVIEW-022] avevano ragionato sul confine `pub(crate)`, che è il confine sbagliato.

## Final decision

**Accettare.** La spec fa ciò che dichiara: chiude [DEBT-016] e [DEBT-015] senza muovere nulla, e la garanzia contro la confusione `Digest32`/byte nudi è reale e verificata sui due punti d'ingresso. Su una spec di questa dimensione «regge» è l'esito corretto, e lo dico senza riserve.

Ciò che questa review aggiunge non è un difetto della passata: è che **la garanzia contro i byte arbitrari è nominale ovunque, non solo dentro il crate**, perché la via nominata è pubblica e senza gate. Il verdetto non cambia; cambia dove va messo il rinforzo prima che il primo chiamante di consenso esista, che era l'unica ragione per cui questa spec è stata fatta adesso.
