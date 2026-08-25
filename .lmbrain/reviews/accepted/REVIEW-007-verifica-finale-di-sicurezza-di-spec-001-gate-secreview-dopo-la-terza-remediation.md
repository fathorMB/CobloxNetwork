---
id: REVIEW-007
# Note: Quote the title if it contains a colon
title: "Verifica finale di sicurezza di SPEC-001 — GATE-SECREVIEW dopo la terza remediation"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-001
reviewer: AGENT-007
review_requested_by: AGENT-LEAD
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [robustness, documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-007-EVENT-001"
    timestamp: "2026-08-25T02:47:17.931314500+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-007-EVENT-002"
    timestamp: "2026-08-25T02:51:14.477751200+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "GATE-SECREVIEW superato. Verifica mirata sulle due contestazioni di AGENT-001 piu chiusura effettiva degli altri sei finding. Entrambe le contestazioni reggono, e in entrambe la mia condizione di chiusura di REVIEW-006 era sbagliata. RF-101: 'iterations >= 3' preso alla lettera rifiuterebbe la PRIMA raccomandazione di RFC 9106 (t=1,p=4,m=2^21, 2 GiB, area 2097152 KiB-passate contro 196608), cioe la piu forte delle due; la forma a due vincoli congiunti (memory_kib >= 65536 PIU memory_kib*iterations >= 196608 in u128) e corretta perche nessuno dei due implica l'altro: l'area da sola ammette 8 KiB x 24576 passate, compute-bound e GPU-friendly, cioe la proprieta per cui ADR-007 ha scartato SHA-256; il pavimento di memoria da solo ammette 64 MiB x 1 passata, un terzo del profilo RFC. Verificato inoltre che il pavimento e proprieta del documento ('is invalid, not merely unwise') e quindi vincola anche il set iniziale della release, caso che una regola scritta come vincolo di governance avrebbe lasciato scoperto. RF-104: la mia parametrizzazione era sbagliata per un fattore che avevo omesso, il puzzle e pagato presso ~2/3 dei validatori per via del quorum e non una volta sola, quindi ~2^28 sarebbe una tassa sull'onboarding onesto; e un puzzle fisso reintrodurrebbe il divario CPU/GPU per cui ADR-007 esiste. Aritmetica del tetto rifatta: al massimo normativo (~2^20-2^21) il puzzle riduce l'asimmetria da ~10^4:1 a ~10^3:1 e non la chiude, quindi la validazione della sorgente e la meta portante e il puzzle il moltiplicatore, che e esattamente l'argomento di AGENT-001. Sull'adattivita come superficie, la domanda del Lead: tenere la difficolta al massimo in permanenza E realizzabile perche al tetto l'attaccante paga ~0,1 ms per slot contro 200 ms del validatore, ma NON produce esclusione perche il tetto e ancorato al tempo che il dispositivo di riferimento spende nella proof of work stessa, quindi il caso peggiore e circa un raddoppio dell'onboarding e non un rifiuto; e il documento dichiara esattamente questo residuo nella forma sfavorevole ('slow devices are the ones that suffer... It does not make enrollment always available'), che e la ragione per cui attesto invece di riaprire. Altri sei chiusi e verificati, due meglio della condizione richiesta: RF-102 include 'including the set inherited from the checkpoint', buco di secondo ordine che nessuno aveva nominato, e corregge un errore di unita nella mia condizione confrontando ms con blocchi; RF-103 risolve la discordanza README/ledger in modo strutturale e non per allineamento, con la clausola 'This schema is the single definition; ledger.md consumes it and does not restate its fields' e ledger.md che effettivamente non rienumera i campi, piu obbligo di checkpoint fresco a ogni revoca non richiesto; RF-105 cross-reference presente in entrambe le direzioni; RF-106 le primitive eligible_* in ledger.md soddisfano la condizione, che chiedeva la falsariga di subscription_leaf/node, anch'esse in ledger.md; RF-107 e RF-108 chiusi. Verifica aggiuntiva: unicita dei tag di separazione di dominio sui cinque documenti, 0x00-0x03 0x10-0x13 0x20-0x23 0x24-0x27 0x30-0x33, nessuna collisione, 0x11 condiviso fra empty[d] e branch correttamente perche empty[d] e per definizione un branch di due figli vuoti. Due nuovi finding low non bloccanti, da chiudere in M-02: RF-109, la frase 'rejects everything weaker than either' e leggermente piu forte del vero perche la forma d'area ammette iterations=1 sotto i 2 GiB quando memory_kib >= 196608, banda che non e nessuna delle due raccomandazioni RFC e la cui resistenza al tradeoff e inferiore a entrambe, ma il degrado e un piccolo fattore costante e non un ordine di grandezza; RF-110, la conseguenza 'a distinct reachable address for every concurrent slot' segue solo se l'emissione dei nonce e contata contro il limite per sorgente del passo 1, perche il monouso limita il riuso e non il volume di emissione. Terzo giro di remediation NON giustificato, e la linea non contraddice REVIEW-006: li la frase di RF-102 affermava una copertura inesistente, aveva causato una decisione di progetto e nascondeva un fallimento completo senza riserve dichiarate; qui i due residui stanno sopra proprieta che valgono davvero e il documento dichiara i propri limiti. ADR-007 non richiede modifiche e l'avvertimento di REVIEW-006 rientra: con RF-101 chiuso il punto 3 della decisione e un vincolo e non una raccomandazione. Nessuna modifica a docs/protocol, nessun movimento di lifecycle della spec, nessun commit."
    evidence_refs: ["SPEC-001", "REVIEW-006", "REVIEW-002", "ADR-007", "docs/protocol/README.md", "docs/protocol/identity.md", "docs/protocol/ledger.md", "docs/protocol/wire.md", "docs/protocol/app-manifest.md"]
    implementation_agent: "AGENT-001"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review, security]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> accepted"
---
# Review

## Outcome

**`GATE-SECREVIEW` è superato. Attesto.**

Verifica mirata, non completa: il Lead mi ha richiamata perché AGENT-001 ha chiuso tutti
e otto i finding di [REVIEW-006] ma ha **cambiato la forma di due contromisure** rispetto
a quella che avevo indicato, con motivazione. La domanda che mi è stata posta non è se
l'aritmetica torni — quella il Lead l'ha già rifatta — ma se le due forme alternative
siano quelle **giuste dal punto di vista della sicurezza**.

Lo sono entrambe. E in un caso la forma di AGENT-001 è **migliore della mia**: la mia
condizione di chiusura per RF-101 era sbagliata, e se fosse stata applicata alla lettera
avrebbe rifiutato la configurazione più forte delle due raccomandate da RFC 9106. Do atto
che contestarla era la scelta corretta, e che il modo in cui è stata contestata — adottare
la classe di contromisura richiesta, cambiare solo la parametrizzazione, scrivere la
motivazione nel documento — è esattamente il comportamento che voglio da un implementatore
che dissente da una review.

Trovo due residui, entrambi **low**, entrambi della stessa natura: una frase che
giustifica una regola promette un po' più di quanto la regola consegni. Nessuno dei due
tocca una proprietà di sicurezza; toccano l'esattezza di un'affermazione **su** una
proprietà. Li registro perché la domanda del Lead era esplicitamente se esistessero
configurazioni deboli che nessuno dei due aveva considerato, e la risposta onesta è "sì,
una banda stretta, e vale la pena scriverla". **Non giustificano un terzo giro di
remediation** — vedi *Required follow-up*, dove spiego perché la linea che traccio qui non
contraddice quella di [REVIEW-006].

## Acceptance-criteria compliance

Valuto il solo `GATE-SECREVIEW`, sulle tre aree che nomina.

| Area di focus | [REVIEW-002] | [REVIEW-006] | Ora | Sintesi |
| --- | --- | --- | --- | --- |
| **Identità** | Non superata | Non superata | **Superata** | RF-102 chiuso: la revoca è ora visibile al light client attraverso il checkpoint, e la frase falsa è sparita dal repository. |
| **Enrollment** | Non superata | Non superata | **Superata** | RF-101: pavimento di costo come regola di validità, in forma migliore di quella che avevo chiesto. RF-104: scudo di ammissione a due parti con difficoltà adattiva e limite di disponibilità dichiarato. |
| **Light client** | Non superata | Non superata | **Superata** | RF-103: `WeakSubjectivityCheckpoint` normativo in un unico punto, dominio dedicato, preimmagine, fixture, trust key con provenienza e rotazione, circolarità del parametro risolta. |

## Code observations

**RF-101 — la contestazione regge, e la mia condizione di chiusura era sbagliata.**
Avevo chiesto `iterations >= 3`. Presa alla lettera quella regola rifiuta la **prima**
configurazione raccomandata da RFC 9106 — `t=1, p=4, m=2^21` (2 GiB), quella che la RFC
chiama "a uniformly safe option" — cioè la più forte delle due, con un'area di costo
`2.097.152` KiB-passate contro le `196.608` del profilo a 64 MiB. Avrei imposto un
pavimento che esclude il tetto. AGENT-001 ha visto l'errore e lo ha corretto.

La forma adottata in `README.md` §"The enrollment cost floor is a validity rule" è
**due vincoli congiunti**, ed è la scelta giusta:

```text
memory_kib      >= 65536                           // pavimento di memory-hardness
memory_kib * iterations >= 196608                  // pavimento d'area, u128 controllato
```

I due vincoli difendono due proprietà diverse e **nessuno dei due implica l'altro**, che
è la ragione per cui devono essere separati. L'area da sola fissa la *quantità* di lavoro
ma non la sua *forma*: `8 KiB × 24.576 passate` soddisfa l'area e produce una funzione
compute-bound, perfettamente parallelizzabile su GPU — cioè precisamente la proprietà per
cui [ADR-007] ha scartato SHA-256. Il pavimento di memoria da solo fissa la forma ma non
la quantità: `64 MiB × 1 passata` è memory-hard ma costa un terzo del profilo RFC.
Imporre entrambi è la risposta corretta, e il documento **scrive questo ragionamento**
invece di limitarsi alla regola.

Ho verificato che i vincoli mordano dove devono: `65536/3` valido, `65535/3` invalido,
`65536/2` invalido per area, `2097152/1` valido, `8/1` invalido. Ho verificato anche che
il pavimento sia una proprietà **del documento** e non di un percorso di governance — la
regola dice "An `enrollment_parameters` document is **invalid**, not merely unwise" — e
quindi vincola anche il set iniziale distribuito con la release, che è il caso che una
regola scritta come vincolo di governance avrebbe lasciato scoperto. Il divieto di
abbassare i minimi per governance, con l'obbligo di ri-dichiarare il dispositivo di
riferimento per proporre un cambiamento, chiude il residuo che avevo indicato.

**RF-104 — la contestazione regge su entrambi i punti, e il secondo è quello che conta.**
Sul primo punto ho rifatto l'aritmetica e AGENT-001 ha ragione: la mia frase
"millisecondi, una volta" non regge. Per assorbire ~10¹⁰ H/s contro una capacità di poche
decine di valutazioni al secondo servono ~2^28 tentativi, che sul dispositivo di
riferimento sono decine di secondi — più della proof of work che lo scudo protegge, e
pagati **per validatore**, cioè presso i ~2/3 del potere di voto che il quorum impone.
Avevo prezzato il puzzle come se fosse pagato una volta sola; è pagato `2/3 · V` volte.
Un puzzle fisso a quella difficoltà è una tassa sull'onboarding onesto, non uno scudo.

Il secondo punto è il più importante e non l'avevo considerato: **un puzzle fisso
reintrodurrebbe nello scudo lo stesso divario CPU/GPU per cui [ADR-007] esiste.** Il
puzzle è SHA-256 — deliberatamente, e per la ragione giusta, che è la verifica a costo
costante per il difensore — ma questo significa che a parità di difficoltà l'attaccante
lo risolve ~10⁴ volte più in fretta del telefono. Ho quantificato il residuo: con il
tetto normativo adottato (la difficoltà massima non può eccedere il tempo che il
dispositivo di riferimento spende nella proof of work stessa, cioè ~2^20–2^21 tentativi),
l'attaccante paga ~0,1 ms per slot contro i ~200 ms di uno slot memory-hard del
validatore. **Il puzzle da solo riduce l'asimmetria da ~10⁴:1 a ~10³:1: non la chiude.**

È esattamente per questo che la Parte 1 — la validazione della sorgente — non è un
complemento ma la metà portante, e l'argomento di AGENT-001 è corretto: il quorum obbliga
l'onesto a soddisfare lo scudo presso ~2/3 dei validatori mentre all'attaccante basta
saturarne ~1/3, quindi un costo simmetrico per tentativo lavora *contro* l'onesto. Ciò
che sposta l'asimmetria nella direzione giusta è che il nonce è legato al Peer ID
autenticato **e all'indirizzo remoto osservato**, monouso e a vita breve: si ottiene solo
completando un round trip, quindi non è ottenibile a indirizzo spoofato, e costa
all'attaccante un indirizzo **raggiungibile** per slot concorrente — una risorsa che non
scala con la CPU. Questa è la risorsa scarsa giusta da attaccare. Nota di merito su un
dettaglio che è facile sbagliare: `admission_tag` lega la soluzione a `public_key_32`, e
il wrapper `EnrollmentSubmission` tiene la soluzione **fuori** dall'oggetto firmato, con
la motivazione scritta — altrimenti servirebbe una firma e un `enrollment_request_hash`
per validatore e salterebbe il modello a certificato unico. È la scelta corretta.

**Sull'adattività come superficie — la domanda del Lead.** Ho cercato l'attacco "tieni la
difficoltà alta in permanenza per escludere i dispositivi lenti". **È realizzabile**, e la
ragione è quella appena calcolata: poiché al tetto l'attaccante paga ~0,1 ms per slot
contro 200 ms del validatore, mantenere la saturazione — e quindi la difficoltà al
massimo — resta economico per lui, purché disponga degli indirizzi. L'adattività non è
quindi auto-bilanciante nel senso forte: il ciclo di retroazione non tassa l'attaccante
abbastanza da spegnerlo.

Ma **non produce esclusione**, e la ragione è il tetto normativo, che è la parte di
progettazione che vale. Poiché `admission_difficulty_bits` non può superare la difficoltà
il cui tempo atteso di soluzione sul dispositivo di riferimento eccede il tempo che *lo
stesso dispositivo* spende nella proof of work, il caso peggiore per il dispositivo
dichiarato è **circa un raddoppio del tempo di onboarding**, non un rifiuto. Un
dispositivo più lento del riferimento paga proporzionalmente di più, e questo è il costo
reale. La differenza fra questo e lo scenario di [REVIEW-006] è netta e non cosmetica:
prima l'onboarding si fermava **a tempo indeterminato, gratis, e in modo permanente**;
ora degrada, l'attaccante paga un costo che non può ammortizzare fra validatori, e la
degradazione ha un tetto ancorato a un dispositivo dichiarato e ri-dichiarabile.

E il documento **dichiara esattamente questo**, senza abbellirlo: "Under sustained attack
an honest requester pays a real puzzle, per validator, and slow devices are the ones that
suffer… It does not make enrollment always available." Questo è il punto che mi fa
attestare invece di riaprire. Il residuo che il Lead mi ha chiesto di cercare esiste, è
reale, e **è già scritto nel documento nella sua forma sfavorevole**, applicando alla
disponibilità lo stesso standard di onestà che `identity.md` applica all'anti-Sybil. Una
mitigazione che dichiara ciò che non copre è chiusa; una che lo tace non lo è.

**RF-102 è chiuso, e in un punto meglio della mia condizione.** La frase "A light client
needs no new field to see this" non è più presente in `docs/protocol/` — verificato per
assenza sull'intero albero, non sulla dichiarazione. Il passo 4 dell'algoritmo applica le
revoche del checkpoint, e include una clausola che avevo indicato solo implicitamente:
"including the set inherited from the checkpoint". Senza quella, un checkpoint il cui
`validator_set_hash` punta a un set contenente un revocato avrebbe ancorato il client
proprio all'insieme da cui doveva difenderlo. È una chiusura di un buco di secondo ordine
che nessuno aveva nominato.

La tensione fra i due parametri è dichiarata, e AGENT-001 ha **corretto un errore di
unità nella mia condizione**: avevo scritto "il secondo non deve superare il primo",
confrontando millisecondi con blocchi. Il documento scrive la forma sensata —
`max_weak_subjectivity_age_ms` non maggiore della *durata attesa in tempo reale* di
`min_revocation_effective_delay_blocks` — con la ragione: "so that a checkpoint a client
still accepts is never older than the window granted to commit a compliant successor set".
Aggiunge inoltre l'obbligo di pubblicare un checkpoint fresco a ogni revoca, che non
avevo chiesto e che riduce la finestra residua dal massimo teorico alla cadenza reale.

**RF-103 è chiuso, e la discordanza è risolta, non spostata.** Questo era il punto su cui
il Lead mi ha chiesto attenzione specifica, e l'ho verificato nel modo che conta: non che
i due documenti ora concordino, ma che **non possano tornare a discordare**. `README.md`
§"Weak subjectivity checkpoint" porta lo schema e la clausola normativa esplicita: "This
schema is the single definition; ledger.md consumes it and does not restate its fields."
Ho controllato che `ledger.md` la rispetti: il passo 1 dell'algoritmo dice "Load a
`WeakSubjectivityCheckpoint` exactly as specified in README.md" e **non rienumera i
campi** — nomina solo i due che consuma operativamente (`chain_id`,
`max_weak_subjectivity_age_ms`) e trattiene `revoked_validators` per il passo 4. La
discordanza sui campi che avevo trovato è quindi strutturalmente impossibile da
ripetere, che è la differenza fra risolverla e allinearla una volta.

Il resto della chiusura è completo: dominio `coblox-weak-subjectivity-signature-v0`,
preimmagine con `chain_id_32`, fixture `WSC-0`, primitive `revocation_leaf/node/empty` con
`REVL-0` e radice vuota pubblicate. La trust key ha ora provenienza ("ships **inside** the
signed network distribution and in no other channel. It is configuration, not a
discoverable fact"), rotazione a due release sovrapposte, fail-closed sulla chiave
sconosciuta con divieto esplicito di apprenderla da un peer o dal checkpoint stesso, e
recupero fuori banda con la motivazione giusta — "a compromised signer can otherwise sign
whatever supersession message the protocol would define". Il limite dichiarato (un client
non aggiornato accetta checkpoint della chiave compromessa finché non aggiorna) è
corretto e non aggirabile a questo livello. La circolarità di `max_weak_subjectivity_age_ms`
è risolta leggendolo dal checkpoint firmato, con il controllo di coerenza successivo
contro la catena e fail-closed sul disaccordo.

## Tests and verification

Non ho ripetuto i controlli che il Lead ha già eseguito (19 esempi canonici, 40 link,
fixture `node_id`, formula di quorum, conteggio righe). Ho eseguito questi.

**Verifica 1 — esiste una configurazione conforme a entrambi i vincoli di RF-101 che
resti debole?** Questa era la domanda esplicita del Lead. Ho enumerato la frontiera
ammessa. Per `m` in `[65536, 196608)` il vincolo d'area impone `t >= ceil(196608/m)`;
da `m = 196608` in su, `t = 1` è ammesso. La regione ammessa contiene quindi punti che
**non sono nessuna delle due raccomandazioni RFC**: `m = 196608, t = 1` (192 MiB, passata
singola) e `m = 98304, t = 2`. RFC 9106 raccomanda `t = 1` **soltanto** a 2 GiB, e
l'accoppiamento non è arbitrario: a passata singola la prima metà di Argon2id è
data-independent e quindi la porzione esposta agli attacchi di tradeoff tempo-memoria è
proporzionalmente maggiore, ed è la ragione per cui la RFC alza a `t = 3` quando la
memoria scende. Il degrado è di **un piccolo fattore costante**, non di ordini di
grandezza, perché il pavimento d'area e quello di memoria continuano entrambi a valere —
niente a che vedere con il fattore ~8.000× di [REVIEW-006]. **Non riapro RF-101**, ma la
frase del documento "The area form admits both RFC recommendations and rejects everything
weaker than either" è **leggermente più forte del vero**: ammette anche una banda stretta
che non è nessuna delle due e la cui resistenza al tradeoff è sotto entrambe. Lo registro
come RF-109, low.

**Verifica 2 — l'aritmetica del tetto dello scudo di ammissione (RF-104).** Riportata per
esteso in *Code observations*. Sintesi: al tetto normativo (~2^20–2^21 tentativi) il
puzzle riduce l'asimmetria da ~10⁴:1 a ~10³:1 e non la chiude, il che conferma che la
Parte 1 è la metà portante e la Parte 2 il moltiplicatore. Conferma anche la scelta di
AGENT-001 di **non** alzare la difficoltà: portarla al 2^28 che avevo implicitamente
chiesto chiuderebbe l'asimmetria contro l'attaccante e insieme escluderebbe i dispositivi
per cui la rete esiste. Il tetto ancorato al dispositivo di riferimento è la parte
progettata bene.

**Verifica 3 — la Parte 1 consegna ciò che la sua frase afferma?** `wire.md` dice che il
nonce è legato al Peer ID autenticato e all'indirizzo osservato, è monouso ("accepts it
exactly once") e non trasferibile fra validatori o chiavi. Ho verificato che ciò rende il
nonce **non ottenibile a indirizzo spoofato**, che è la proprietà forte e che regge. Ciò
che non regge alla lettera è la conseguenza dichiarata: "costs an attacker a distinct
reachable address for every concurrent slot it wants to hold". Il monouso limita il
*riuso* di un nonce, non il *volume di emissione*: nulla nella sezione dello scudo
impedisce a un validatore di emettere N nonce successivi allo stesso indirizzo, e al
tetto di difficoltà l'attaccante li risolve a ~0,1 ms l'uno. La proprietà segue solo se
l'emissione dei nonce, o il numero di nonce non scaduti e non usati in sospeso, è contata
contro lo stesso limite per sorgente del passo 1. Quel limite **esiste** ed è normativo
nella forma "un bound esiste, è dichiarato, e fallisce chiuso" — l'elenco dei bound
include "a failed step 9 counted against the source connection for rate limiting" — ma la
sezione dello scudo non ci si aggancia. RF-110, low: una clausola.

**Verifica 4 — unicità dei tag di separazione di dominio.** Con RF-103 e RF-106 sono
state introdotte due nuove famiglie di primitive d'albero, e una collisione di tag fra
alberi consentirebbe di riusare un nodo di un albero come nodo di un altro. Ho enumerato
tutti i tag `H(0xNN)` nei cinque documenti: `0x00–0x03`, `0x10–0x13`, `0x20–0x23`
(subscription), `0x24–0x27` (eligible), `0x30–0x33` (revocation). **Nessuna collisione.**
Le due occorrenze apparentemente doppie sono corrette: `0x11` è condiviso fra `empty[d]` e
`branch` dell'albero dei conti, ed è **necessario** che lo sia perché `empty[d]` è per
definizione un branch di due figli vuoti; `0x33` compare due volte come citazione della
stessa definizione.

**Verifica 5 — RF-106, e una correzione a una lettura troppo letterale.** Le primitive
`eligible_leaf/node/empty` (tag `0x24–0x27`) sono definite in `ledger.md`, non nel
registro di `README.md`. Questo **soddisfa** la mia condizione di chiusura, che chiedeva
la preimmagine "sulla falsariga di `subscription_leaf`/`subscription_node`": quelle
primitive vivono in `ledger.md`, quindi collocare le nuove accanto a loro è seguire il
modello che avevo nominato, non discostarsene. Il campo `eligible_set_root` è richiesto in
`MintBody` per `existence_income`, i validatori full che già ricalcolano `E` dalla stessa
evidenza ricalcolano anche la radice e rifiutano un mint la cui radice differisca — il
conteggio passa da asserzione a fatto falsificabile, che era il punto — e il limite
residuo del light client (ha la radice, non le foglie; le prove di eleggibilità per epoca
sono lavoro di M-02) è dichiarato. Chiuso.

**Verifica 6 — RF-105, RF-107, RF-108.** RF-105 chiuso su tutti e tre i punti: l'ordine di
grandezza (10³–10⁶ valori legali di `timestamp_ms` a un SHA-256 ciascuno) e la condizione
di collusione emittente-proposer sono scritti, il riconoscimento che il commit-reveal
acceca davvero il proposer non colluso è scritto, e la copertura a due emittenti è
dichiarata **la** mitigazione del grinding con rinvio incrociato presente in **entrambe**
le direzioni, `ledger.md` → `wire.md` e ritorno. Il rilievo di coerenza che avevo mosso —
un limite dichiarato senza ordine di grandezza è a metà fra una dichiarazione e una
rassicurazione — è chiuso. RF-107 chiuso: `wire.md` nomina lo stream di enrollment **per
primo** nella frase sui limiti di concorrenza, spiega perché ("the only one that accepts
unauthenticated transport peers"), e rinvia a `identity.md` con l'istruzione esplicita per
chi costruisce il trasporto leggendo solo quel documento — che era il rischio concreto.
RF-108 chiuso: `HostAcceptancePolicy` è dichiarata "**not a network object** in v0", con
assegnazione ottimistica e il rifiuto come canale di scoperta previsto, e la lista di
rifiuto per nodo di [ADR-006] riconosciuta come input normale e non eccezionale.

## Production quality and documentation compliance

Conforme a [[QUALITY]]. Segnalo un tratto che considero il migliore di questa remediation
e che vorrei visto come precedente: in tutti e due i punti contestati, AGENT-001 ha
scritto **nel documento** la ragione per cui la forma alternativa è stata scelta — perché
l'area e non `iterations >= 3`, perché SHA-256 e non Argon2id nello scudo con il divieto
esplicito di "correggerlo" più tardi, perché la soluzione sta fuori dall'oggetto firmato.
Una specifica che porta le proprie motivazioni resiste alla manutenzione futura, che è il
modo tipico in cui una difesa viene rimossa da qualcuno che non sapeva a che cosa
servisse. Il divieto esplicito contro la "correzione" futura dello scudo è la singola riga
più utile aggiunta in questo giro.

Il §"Declared limit — availability of enrollment is not a protocol guarantee" di
`identity.md` regge il confronto con il §"Declared limits of this mechanism", che in
[REVIEW-006] avevo indicato come il miglior testo di sicurezza del progetto. Lo standard
è stato applicato al secondo caso, come avevo chiesto.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

- RF-109 | category=documentation | severity=low | criterion=enrollment — la frase che giustifica il pavimento di costo è più forte della regola | remediation=**La regola è giusta; la frase che la spiega promette un po' di più.** `README.md` afferma che la forma d'area "admits both RFC recommendations and rejects everything weaker than either". La prima metà è vera e verificata. La seconda no in senso stretto: la regione ammessa contiene una banda che non è nessuna delle due raccomandazioni, cioè `iterations = 1` sotto i 2 GiB ogni volta che `memory_kib >= 196608` (e `iterations = 2` da `memory_kib >= 98304`). RFC 9106 accoppia `t = 1` **soltanto** con 2 GiB, e l'accoppiamento non è estetico: a passata singola la porzione data-independent di Argon2id pesa di più e la resistenza agli attacchi di tradeoff tempo-memoria è inferiore a quella di entrambi i profili raccomandati. **Perché è low e non riapre RF-101:** il degrado è un piccolo fattore costante, non un ordine di grandezza, perché entrambi i pavimenti continuano a valere; non c'è nulla di paragonabile al fattore ~8.000× che [REVIEW-006] aveva misurato, e nessun attacco pratico che io possa esibire su questa banda. **Chiusura verificabile, due opzioni, e la scelta è di taratura non di sicurezza:** (a) restringere alla forma che realizza esattamente l'intento dichiarato — `memory_kib >= 65536` **e** (`iterations >= 3` **oppure** `memory_kib >= 2097152`) — che ammette entrambi i profili RFC e rifiuta tutta la banda a passata singola sotto i 2 GiB; oppure (b) tenere la forma d'area, che è più permissiva ma più semplice, e correggere la frase in "admits both RFC recommendations and rejects everything below the cost-area of the weaker one", aggiungendo che `iterations = 1` è consigliato solo al profilo da 2 GiB. **Raccomando (b):** la forma d'area è difendibile, il residuo è piccolo, e vale più una frase esatta che un vincolo più stretto del necessario. **Costo:** una riga.

- RF-110 | category=robustness | severity=low | criterion=enrollment — la proprietà portante della Parte 1 dello scudo non è agganciata alla regola che la produce | remediation=**La Parte 1 è la metà che regge lo scudo, e la sua conseguenza dichiarata segue solo da una regola che vive altrove.** `identity.md` afferma che il nonce di ammissione "costs an attacker a distinct **reachable** address for every concurrent slot it wants to hold, which is the part of the attack that does not scale with CPU". La Verifica 3 mostra che la proprietà di **raggiungibilità** regge senza riserve — il nonce richiede un round trip su una connessione autenticata, quindi non è ottenibile a indirizzo spoofato — ma che la proprietà "**per slot concorrente**" no: `wire.md` rende il nonce monouso, il che limita il riuso, non il volume di emissione. Nulla nella sezione dello scudo impedisce a un validatore di emettere nonce successivi in numero illimitato allo stesso indirizzo, e al tetto di difficoltà l'attaccante ne risolve uno ogni ~0,1 ms. **Perché è low:** il limite che produce davvero la proprietà **esiste** ed è normativo — l'elenco dei bound di `identity.md` impone che un limite per sorgente esista, sia dichiarato e fallisca chiuso, e conta il fallimento del passo 9 contro la connessione d'origine — e il passo 1 dell'ordine di validazione lo richiama. Non manca una difesa: manca il collegamento fra la difesa e la frase che ne dipende, in un documento che altrove lega sempre le due cose. Aggravante minima: è la stessa classe di difetto di RF-102, cioè una frase che afferma una copertura che la regola citata non consegna da sola — ma qui, a differenza di RF-102, la copertura esiste davvero e la sezione dichiara esplicitamente che la disponibilità non è garantita, quindi non c'è nulla di nascosto. **Chiusura verificabile:** una clausola nella Parte 1 che dica che l'emissione dei nonce, o il numero di nonce non scaduti e non usati in sospeso, è contata contro lo stesso limite per sorgente del passo 1, così che il costo per slot concorrente sia una conseguenza della regola e non un'affermazione a sé. **Costo:** una frase.

## Required follow-up

1. **Nessuna remediation è richiesta per il gate.** RF-109 e RF-110 sono entrambi low,
   entrambi una riga, e nessuno dei due cambia una proprietà di sicurezza.
2. **Un terzo giro di remediation non è giustificato, e lo dico esplicitamente perché il
   Lead me lo ha chiesto.** La ragione non è che il costo di iterare sia alto o che la
   spec sia l'ultima aperta — quelle sono ragioni di programma e non mi competono. È che
   RF-109 e RF-110 sono **di natura diversa** dai finding che mi hanno fatto bloccare in
   [REVIEW-006]. Lì avevo scritto che un finding chiuso male è peggio di uno aperto, e
   avevo bloccato anche su una frase (RF-102). Tengo quella linea, e distinguo: la frase
   di RF-102 affermava una copertura **inesistente**, aveva **causato una decisione di
   progetto** (non aggiungere l'ancora), e nascondeva un fallimento completo — il light
   client seguiva la catena dell'attaccante — **senza alcuna riserva dichiarata**. RF-109
   e RF-110 stanno sopra proprietà che **valgono davvero**, con residui di un piccolo
   fattore costante, e in entrambi i casi il documento dichiara i propri limiti nella
   forma sfavorevole. Bloccare qui applicherebbe la stessa parola a due situazioni che non
   sono la stessa cosa, e svaluterebbe il blocco precedente invece di essere coerente con
   esso.
3. **RF-109 e RF-110 vanno chiuse in M-02**, insieme al primo lavoro di implementazione
   sull'enrollment, dove chi scrive il codice dello scudo incontrerà naturalmente
   entrambe. Le registro come rischio dichiarato e accettato, non come debito nascosto.
4. **Per il Lead, decisione di taratura non di sicurezza:** RF-109 offre due chiusure e
   raccomando la (b), cioè correggere la frase e tenere la forma d'area. La (a) è più
   stretta del necessario e costerebbe la libertà di scegliere profili ad alta memoria e
   passata singola, che sono ragionevoli su hardware desktop.
5. **Limiti residui dichiarati da AGENT-001: li confermo tutti e tre come correttamente
   dichiarati**, e nessuno è un finding. (i) *Disponibilità dell'enrollment non garantita
   sotto attacco sostenuto, con i dispositivi lenti che soffrono per primi* — verificato
   ed è la conseguenza reale, con il tetto che la limita a una degradazione invece che a
   un'esclusione; è dichiarata nella forma sfavorevole ed è il modello di come si dichiara
   un limite. (ii) *Il checkpoint copre solo le revoche note all'emissione, con esposizione
   fino a `max_weak_subjectivity_age_ms`* — corretto, e mitigato meglio di quanto avessi
   chiesto dall'obbligo di pubblicare un checkpoint fresco a ogni revoca e dalla regola di
   coerenza con `min_revocation_effective_delay_blocks`. (iii) *La radice degli eleggibili
   senza foglie rinvia a M-02 la verifica indipendente dell'eleggibilità* — corretto, ed è
   il rinvio giusto: il formato è fissato ora, quindi servire le prove più tardi non è un
   cambio incompatibile, che era l'unica cosa da non sbagliare adesso.
6. **[ADR-007] non richiede modifiche, e l'avvertimento di [REVIEW-006] è rientrato.**
   Avevo scritto che l'ADR presuppone un pavimento memory-hard che la specifica non impone,
   e che finché quel pavimento è governabile a zero il punto 3 della decisione è una
   raccomandazione e non un vincolo. Con RF-101 chiuso, **il punto 3 è un vincolo.**

## Final decision

**`GATE-SECREVIEW` è superato.** Otto finding di [REVIEW-006] chiusi, di cui due chiusi in
una forma migliore di quella che avevo prescritto. Due nuovi finding low, nessuno
bloccante, entrambi da chiudere in M-02.

**Sulle due contestazioni, il giudizio che mi è stato chiesto.** Reggono entrambe, e non
nel senso debole di "sono difendibili": in RF-101 la mia condizione di chiusura era
sbagliata e AGENT-001 l'ha corretta, in RF-104 la mia parametrizzazione era sbagliata per
un fattore che avevo omesso — il puzzle è pagato presso i due terzi dei validatori, non
una volta — e la contestazione ha identificato il motivo strutturale, cioè che un costo
simmetrico per tentativo lavora contro l'onesto quando il quorum è asimmetrico. In
entrambi i casi la classe di contromisura che avevo indicato è stata adottata e solo la
parametrizzazione è cambiata, con la motivazione scritta nel documento. **Il Lead ha
avuto ragione a non attestare al posto mio:** non perché le scelte fossero sbagliate, ma
perché erano scelte di sicurezza sostanziali, e due di esse hanno corretto errori miei
che nessun altro percorso avrebbe intercettato.

**Sul claim di sicurezza, la valutazione finale.**

In [REVIEW-006] avevo scritto che il claim era difendibile solo nella forma che
`identity.md` aveva adottato — robusta contro la falsificazione, non resistente ai Sybil
per via crittografica, e le due affermazioni vanno enunciate insieme — e che ciò che non
era ancora difendibile era che quelle difese fossero **irrevocabili e verificabili da chi
deve fidarsene**. Avevo elencato tre difetti della stessa forma: un pavimento anti-Sybil
azzerabile da un parameter set conforme, una revoca invisibile a chi doveva esserne
protetto, un'ancora di fiducia senza schema.

**Tutti e tre sono chiusi, e chiusi nel modo giusto**, cioè rendendo la difesa una
proprietà della regola invece che una consuetudine dell'implementatore. Il pavimento è
una regola di validità del documento, quindi vale anche per il set iniziale e non è
abbassabile per governance. La revoca raggiunge il light client attraverso l'unica ancora
di cui dispone, compreso il caso in cui l'ancora stessa sia compromessa. Il checkpoint ha
schema in un unico punto normativo, dominio, preimmagine, fixture, e una chiave con
provenienza e rotazione. La specifica ha smesso di descrivere difese e ha cominciato a
imporle, che era la distanza che mancava.

Oggi il claim regge nella sua forma onesta, e aggiungo la seconda metà che ora è
guadagnata e non solo asserita: **le proprietà crittografiche di v0 sono chiuse contro chi
non ha un quorum, e ciò che resta aperto è dichiarato**. Le tre cose che non sono
garantite — la disponibilità dell'enrollment sotto attacco sostenuto, la resistenza Sybil
per via crittografica, la verifica indipendente dell'eleggibilità prima di M-02 — sono
scritte nei documenti nella forma sfavorevole, con i numeri dove i numeri contano. Il
progetto non deve chiamare la rete "super-sicura" senza quelle tre frasi accanto; con
quelle frasi accanto, il claim è più solido della media di ciò che ho visto a questo
stadio, e la parte migliore non è nessun singolo meccanismo ma il fatto che i limiti siano
quantificati.

Non serve un altro giro. Questa volta lo scrivo sapendo di averlo già scritto una volta e
di essermi dovuta ricredere: la differenza è che allora restavano quattro high dentro le
tre aree del gate, e oggi non ne resta nessuno.
