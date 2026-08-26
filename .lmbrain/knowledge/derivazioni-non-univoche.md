---
title: Censimento delle derivazioni non univoche
updated: 2026-08-26
---

# Censimento delle derivazioni non univoche

Pagina nata da [DEBT-020] e prodotta da [SPEC-017]. La domanda che la genera non
e la circolarita di `chain_id` ma la sua **generalizzazione**: *quali valori
entrano in una preimmagine di questo protocollo senza essere derivabili in un
solo modo?*

E l'esercizio che [SPEC-010] ha fatto per le codifiche simboliche, rifatto per
le derivazioni. Come la, **la risposta e un elenco e non una rassicurazione**, e
l'elenco vale piu della fixture che la spec doveva aggiungere.

## Perche la domanda non e la stessa di «esiste una fixture»

L'inventario di [ADR-012] conta le preimmagini prive di fixture pubblicata.
Contare le fixture risponde a *questo valore e stato verificato*; questa pagina
risponde a *due implementazioni indipendenti calcolerebbero lo stesso valore*.
Le due domande si separano nel caso peggiore: `chain_id` alla genesi era
**coperto indirettamente** dall'inventario — ogni riga del registro lega il
`chain_id` a zero di `HASH-0` — ed era comunque ambiguo, perche la fixture
fissava un valore e nessuna regola diceva come si ottenesse.

## Metodo

Per ciascuna delle **51 preimmagini** censite in
`sim/tools/published_artifacts.toml`, ogni ingresso della formula e stato
classificato in una di quattro classi:

- **C — costante**: fissata dai documenti (le stringhe di dominio, i byte di
  tag, `0x00`, i `32 byte zero`). Univoca per definizione.
- **T — trasmessa**: un dato che l'oggetto porta e che il verificatore legge
  invece di calcolarlo (nonce, timestamp, chiavi pubbliche, segreti rivelati,
  `voting_power`, `admission_solution`).

  **La condizione che rende T vera, e senza la quale la classe divora l'elenco.**
  Nella prima stesura questa voce diceva soltanto *«univoca dopo la trasmissione:
  la domanda non si pone»*, ed e un criterio che assorbe qualunque grandezza che
  un oggetto porti con se — cioe la classe piu numerosa, e quella in cui un
  elenco lungo diventa un elenco vuoto. La prova e dentro questa pagina:
  **entrambe le voci non chiuse sono valori trasmessi**, `election_epoch` e un
  campo dell'`ElectionRecord` e `reward_epoch` un campo del corpo del mint, e il
  criterio come era scritto le avrebbe chiuse come T. Sono finite in A e in
  T-dichiarata solo perche ho applicato una domanda che il criterio non
  enunciava ([REVIEW-029] RF-006a). La domanda e questa, ed e parte della classe:

  > **Un valore trasmesso e univoco solo se una regola di validita lo lega a una
  > grandezza derivabile, e quella regola nomina il documento che ne fissa il
  > denominatore. Altrimenti e A.**

  E il criterio che separa `reward_epoch`, la cui regola nomina *il
  `reward_policy` che il mint nomina attraverso il proprio `policy_hash`*, da
  `election_epoch`, la cui regola dice *«dai parametri di consenso attivi»* e
  non nomina nulla.
- **D — derivata**: calcolata da una formula pubblicata. Univoca se e solo se lo
  sono i suoi ingressi, quindi la classificazione e ricorsiva.
- **A — ambigua**: piu di una lettura del documento produce piu di un valore.
  **E l'elenco.**

Il censimento non e stato fatto a impressione: le formule sono quelle del
registro di `README.md` §*Hash preimage registry*, di `ledger.md` §*Hashing
primitives*, di `identity.md` §*enrollment proof of work* e di
`app-manifest.md`; gli ingressi sono stati letti uno per uno.

**Il perimetro, e perche la prima stesura ne aveva uno sbagliato.** Le 51 sono
le preimmagini di **hash**. Le preimmagini di **firma** — i dodici domini
`SIG_*` — non erano nella popolazione, e non era detto che non lo fossero: la
voce 4 dell'elenco qui sotto e una preimmagine di firma **trovata di lato**,
arrivandoci dall'angolo della genesi invece che dal metodo. [REVIEW-029] RF-002
e la prova che il buco costava: applicare la domanda del censimento al payload
del `key_binding_signature` da *«derivabile in un solo modo, e nello stesso modo
su ogni catena»* — la forma sorella dell'ambiguita — e nessuno l'aveva applicata
perche quella preimmagine non era nella lista. Il perimetro e ora **51 + 12**, e
la sezione *Le dodici preimmagini di firma* le censisce una per una.

## L'elenco — chiuso da [SPEC-017]

**1. `chain_id` alla genesi.** `chain_id` ← `genesis_block_id` ←
`JCS(intestazione di genesi)` + `chain_id_32`. Circolare, e nessuna regola
diceva come si rompesse. Chiuso da `README.md` §*Genesis derivation and the
placeholder chain ID*: il **segnaposto di genesi** e 32 byte zero, e lo usa ogni
valore che sia ingresso di `genesis_block_id` e ogni firma su un tale valore.
Fixture `GEN-0`, derivata da due strade che non condividono codice.

**2. `previous_block_id` dell'intestazione di genesi.** `ledger.md` diceva *«the
configured all-zero previous ID»*, che ammette entrambe le letture: *configurato*
(quindi scelto) e *tutto a zero* (quindi fissato). E un ingresso diretto di
`genesis_block_id`, quindi di `chain_id`: una distribuzione libera di sceglierlo
lascerebbe `chain_id` indeterminato **da una seconda porta**. Ora e una regola.

**3. Quale `network_id` entra in `chain_id`.** La formula nomina
`network_id_utf8`, e i campi chiamati `network_id` sono molti: l'intestazione,
le transazioni, i documenti firmati, i tre oggetti di ancoraggio, le buste di
rete. Nessuna regola li obbligava a coincidere, ne diceva quale fosse quello
della derivazione. E la stessa forma dell'1 e del 2 — `chain_id` indeterminato —
per una terza porta. Ora: e il campo `network_id` dell'intestazione di altezza 0,
byte per byte, e ogni altro oggetto della catena DEVE portare la stessa stringa
di byte. La normalizzazione Unicode e gia vietata dalla regola 5 di *Common
representation*, quindi due grafie che si vedono uguali sono due reti.

**4. Con quale `chain_id` firma il `key_binding_signature` del set di genesi.**
I byte del set sono ingresso di `validator_set_hash`, che e un campo
dell'intestazione di genesi: la firma non poteva usare il `chain_id` derivato.
Chiuso dal segnaposto. **Nessun valore pubblicato esercita questa clausola**, ed
e detto nel documento: `GEN-0` copre l'intestazione e il documento, non le
firme, e pubblicare un `ValidatorSet` significherebbe pubblicare una coorte di
genesi che il blocco di vincoli governa.

**5. Con quale `chain_id` si calcola il documento `consensus_parameters` di
genesi.** Stessa forma del 4, attraverso `consensus_parameters_hash`. Chiuso dal
segnaposto e **esercitato** da `GEN-0`.

## L'elenco — aperto, e non chiuso qui

**6. `election_epoch`.** `ledger.md` §*Election epochs and the boundary*:

> With `election_epoch_blocks = L` from the active consensus parameters, epoch
> `e` begins at `election_boundary_height(e) = e * L`.

`election_epoch` entra in tre preimmagini — `election_entropy`, `election_seed`,
`election_ticket` — e in `ElectionRecord`. Il suo denominatore `L` e un
**parametro governato**. Il documento dice *«dai parametri di consenso attivi»*
e non dice **attivi a quale altezza**, ne quale documento un verificatore debba
usare per un'epoca passata. Se una governance porta `L` da 100 a 200, l'altezza
5000 e l'epoca 50 sotto il documento vecchio e l'epoca 25 sotto quello nuovo: il
passo 2 del light client — *«ogni set eletto attiva esattamente a
`election_epoch * election_epoch_blocks`»* — da verdetti opposti sullo stesso
set a seconda del documento che ha in mano, e i tre semi cambiano con esso.

**Il contrasto che rende la diagnosi certa e nella stessa base di codice.**
[SPEC-016] ha chiuso esattamente questa forma per `reward_epoch`, e l'ha chiusa
nominando il documento: *«sia `reward_epoch_ms` il valore portato dal documento
`reward_policy` che il mint nomina attraverso il proprio `policy_hash`»*.
L'oggetto porta con se il documento che fissa il proprio denominatore. Nessun
oggetto dell'elezione porta il proprio `consensus_parameters_hash`, e
`election_epoch` non ha quella cucitura.

E famiglia 3 di `recurring-defects.md` — *qual e il denominatore?* — su una
grandezza che entra in una preimmagine. **Non e chiuso da [SPEC-017] e non
doveva esserlo**: chiuderlo significa scegliere fra rinumerare le epoche,
vietare il cambio di `L` fuori da un confine, o far portare all'`ElectionRecord`
il `consensus_parameters_hash` sotto cui e stato derivato. Sono tre regole di
validita nuove sulle regole di elezione, cioe una decisione del Lead con la sua
passata di [ADR-012]. Aperto dal Lead come [DEBT-028].

**7. `reward_epoch`, che sembra del 6 e non lo e.** Non e derivabile in un solo
modo **per scelta dichiarata**: la regola e un **pavimento** di insediamento
(`(e + 1) * reward_epoch_blocks <= h`) e non un'uguaglianza, perche un mint per
un'epoca si finalizza dopo che l'epoca e finita e nessuna regola puo dire quanto
dopo. L'indice e quindi **trasmesso** e limitato, non derivato, e la ragione e
scritta accanto alla regola. E in questa pagina perche la sua forma e quella del
6 e la sua natura no: la differenza e che qui l'ambiguita e stata **decisa**.

## Le dodici preimmagini di firma

Aggiunte al perimetro dopo [REVIEW-029] RF-006b. La domanda del censimento
applicata a una preimmagine di firma ha una seconda faccia che quelle di hash
non hanno: **il payload e derivabile in un solo modo anche su una catena
diversa?** Se si, la firma non attribuisce nulla, e la forma e sorella
dell'ambiguita invece di esserne l'opposto.

La domanda morde **solo dentro la finestra di genesi**, perche fuori di essa
`chain_id_32` e derivato e distingue le catene per costruzione. Dodici domini,
enumerati:

| Dominio | Payload firmato | Byte che distinguono la rete | Materiale di genesi? |
| --- | --- | --- | --- |
| `coblox-block-vote-v0` | `height`, `round`, `block_id` | `block_id`, su un'intestazione con `network_id` | si |
| `coblox-protocol-document-v0` | l'hash del documento | il documento porta `network_id` | si, il solo `consensus_parameters` |
| `coblox-consensus-key-binding-v0` | JCS della quaterna | **nessuno, era la voce rotta**; ora `network_id` | si |
| `coblox-ledger-transaction-v0` | `raw_32_bytes(tx_id)` | la transazione porta `network_id` | no: altezza 0 non porta transazioni |
| `coblox-challenge-request-v0` | `raw_32_bytes(challenge_id)` | idem, per transazione | no, stessa ragione |
| `coblox-challenge-response-v0` | `response_hash` | idem | no, stessa ragione |
| `coblox-challenge-evidence-v0` | l'evidenza in transazione | idem | no, stessa ragione |
| `coblox-enrollment-request-v0` | JCS della richiesta | la richiesta porta `network_id` | no: nessun campo dell'intestazione la nomina |
| `coblox-enrollment-certificate-v0` | JCS del certificato | il certificato porta `network_id` | no, idem |
| `coblox-transport-key-attestation-v0` | JCS dell'attestazione | l'attestazione porta `network_id` | no, idem |
| `coblox-wire-envelope-v0` | la busta senza firma | la busta porta `network_id` | no: non e mai un oggetto di catena |
| `coblox-weak-subjectivity-signature-v0` | `weak_subjectivity_checkpoint_hash` | il checkpoint porta `network_id` | no, per clausola dichiarata |

**Una sola voce era rotta, e il conteggio e per enumerazione e non a
impressione**: ho letto lo schema di ciascun oggetto firmato e cercato in
ciascuno un campo `network_id`. `coblox-consensus-key-binding-v0` era l'unico il
cui payload non portava ne `network_id` ne alcunche di derivato da esso, perche
il suo oggetto era la quaterna `{activation_height, consensus_public_key,
node_id, validator_id}` e il `ValidatorSet` non ha campo di rete. Chiuso da
questa passata aggiungendo `network_id` all'oggetto.

**Cio che il rimedio non compra, ed e un soffitto e non un'omissione.** Prima
che `genesis_block_id` esista, l'unica grandezza distintiva disponibile e un
**nome scelto dall'operatore**: qualunque altro candidato sarebbe o il `chain_id`
che si sta derivando, o un secondo nome. Quindi l'attribuzione dentro la
finestra di genesi e al livello del nome di rete, e `README.md` dichiara che
l'unicita di `network_id` e una convenzione operativa e non un controllo di
replay. Due catene che condividono un `network_id` condividono ogni payload di
genesi. E il residuo, ed e scritto nel documento invece che lasciato implicito.

## Cio che e stato guardato e trovato univoco

Elencato perche un censimento che riporta solo i propri ritrovamenti non dice
dove ha guardato, e la classe successiva nascera dove nessuno ha guardato.

- **Il ricorso di ogni identificatore che e anche un campo del proprio oggetto.**
  Sono la forma di [DEBT-020] a un livello piu basso, e nessuno di essi e
  circolare: `tx_id` e sulla transazione **senza** `authorization`; `block_id`
  sull'intestazione, che non contiene se stessa; `challenge_id` deve essere
  uguale a `request_hash`, che e sulla richiesta **senza** `challenge_id` e
  senza firma; `message_id` sulla busta **senza** `message_id` e senza firma;
  `app_id` sul manifesto **senza** `publisher_signature`, e lo schema del
  manifesto non ha alcun campo `app_id`; i quattro `*_hash` di documento sono
  sull'`UnsignedProtocolDocument`, che non porta il proprio hash. Verificato uno
  per uno leggendo gli schemi, non assunto per analogia.
- **Argon2id.** `identity.md` fissa `version = 0x13`, `secret = empty`,
  `associated_data = empty`, `m`, `t`, `p`, `tag_length` dal documento attivo, e
  costruisce sia `pow_password` sia `pow_salt` per formula. Non resta alcun
  parametro alla scelta dell'implementazione — che e il modo in cui questa
  famiglia colpisce di solito una KDF.
- **I sei alberi con tag.** Ordine, riempimento e radice vuota sono dichiarati
  per ciascuno, con tag distinti per foglia, nodo, riempimento e vuoto:
  `revoked_validators` e ordinato bytewise per `node_id`, l'albero delle
  transazioni preserva l'ordine di blocco e riempie con `H(0x02)`, la radice
  vuota e `H(0x03)`, e il pareggio nella derivazione dell'elezione e risolto per
  `account_key` crescente.
- **La rappresentazione.** JCS RFC 8785 su I-JSON, chiavi ASCII, interi come
  stringhe nella forma piu corta, base64url senza padding, `sha256:` con 64
  cifre minuscole, niente float, niente `null`, nessuna normalizzazione Unicode.
  Ogni grandezza che potrebbe avere due grafie ne ha una sola dichiarata.
- **`state_root` e `transactions_root`.** Derivati dall'esecuzione, il cui
  ordine canonico e definito. Lo **stato di genesi** non e derivato affatto: e
  dichiarato dalla distribuzione, quindi trasmesso, ed e per questo che `GEN-0`
  lo porta come letterale.
- **`validator_set_hash`.** Senza `chain_id` per eccezione dichiarata, non per
  dimenticanza, e la ragione e ora scritta in tre punti. L'assenza di
  `election` nel set di genesi e dichiarata come assenza del campo, non come
  `null`, che il tipo non sa nemmeno rappresentare.

## Cio che questa pagina non copre

- **Le preimmagini che v0 non definisce.** Se una regola futura ne aggiunge una,
  questo censimento non la conosce. La domanda va rifatta, ed e per questo che
  la pagina porta il metodo e non solo l'esito.
- **La correttezza semantica di un valore.** Come `published_artifacts.py`,
  questa pagina chiede se un valore e determinato, mai se e giusto.
- **Le grandezze che non entrano in una preimmagine.** Sono soggetto di
  `recurring-defects.md` famiglia 3, non di questa pagina, e il confine e
  sottile: `election_epoch` sta qui perche entra in tre preimmagini, e ci
  starebbe comunque nell'altra pagina.
- **Il perimetro e 51 preimmagini di hash piu 12 di firma**, ed e una
  dichiarazione verificabile invece che una rassicurazione. Se un dominio nuovo
  compare in `hash.rs`, non e censito finche qualcuno non rifa la passata. La
  prima stesura dichiarava 51 e **taceva** sulle 12: taceva, non sbagliava, ed e
  la forma in cui un perimetro sbagliato passa inosservato.

Vedi anche [[recurring-defects]], [ADR-012], [DEBT-020] e [DEBT-028].
