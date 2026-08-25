---
id: REVIEW-006
# Note: Quote the title if it contains a colon
title: "Ri-verifica di sicurezza di SPEC-001 — GATE-SECREVIEW dopo Lotto A e Lotto B"
status: superseded
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-001
reviewer: AGENT-007
review_requested_by: AGENT-LEAD
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, robustness, requirements-completeness, documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-006-EVENT-001"
    timestamp: "2026-08-25T02:16:56.594491600+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-006-EVENT-002"
    timestamp: "2026-08-25T02:22:19.622084900+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "GATE-SECREVIEW non superato. Dei 18 finding di REVIEW-002 ne verifico 15 chiusi bene, con chiusure in piu punti migliori della condizione richiesta: RF-001 scrive l'equazione ZIP-215 per esteso e la tabella degli esiti attesi sui 12 vettori di ed25519-speccheck e internamente coerente con la regola (0/1/10/11 reject per A di ordine piccolo, 6/7 reject per S>=L, 8/9 accept con R non canonica); RF-002 usa il predicato intero con le fixture 100/101/102 che riproducono il mio controesempio; RF-008 definisce tutte e 12 le preimmagini e l'auditabilita regge davvero perche ledger_range_response porta protocol_documents, quindi la reward policy firmata e recuperabile; RF-012 fallisce chiuso senza sfrattare voci vive; RF-015 e RF-018 superano la mia condizione di chiusura. Restano 4 finding high, tutti dentro le tre aree del gate. RF-101: i bound di RFC 9106 citati da README.md come vincoli di validita sono il dominio della funzione, non un pavimento di sicurezza — verificato sul testo primario della RFC, memory_kib >= 8*lanes con lanes=1 significa 8 KiB, quindi un parameter set firmato e pienamente conforme azzera il pavimento memory-hard di ADR-007 restituendo all'attaccante un vantaggio ~8.000x, superiore al 2.750x misurato per SHA-256 che ha motivato l'ADR. RF-102: RF-004(c) e chiuso per i full node e non per il light client, mentre ledger.md afferma esplicitamente \"A light client needs no new field to see this\" — il client verifica continuita di hash e key binding e non legge transazioni, quindi non sa che una revoca esiste; con oltre due terzi di chiavi trapelate i full node stallano correttamente e i light client seguono la catena dell'attaccante, esposizione limitata a max_weak_subjectivity_age_ms ma allungata da min_revocation_effective_delay_blocks. RF-103: il checkpoint di soggettivita debole, unica difesa residua contro RF-003, e l'unico oggetto firmato privo di schema, dominio e preimmagine, con i campi descritti in modo discordante fra README.md e ledger.md e la network-release trust key senza provenienza ne rotazione. RF-104: i limiti di concorrenza aggiunti da AGENT-001 chiudono correttamente il DoS di memoria e il limite per chiave non e a sua volta un vettore (verificato), ma convertono il problema in negazione dell'ammissione: asimmetria ~10^4:1 fra una firma da 20 microsecondi e uno slot da 64 MiB per 200 ms, e poiche il certificato richiede quorum basta saturare poco piu di un terzo del potere di voto — circa 34 core su un set da 100 — per fermare l'onboarding della rete. Medium: RF-105 grinding del proposer sottostimato, il commit-reveal blinda davvero il proposer non colluso ma timestamp_ms offre 10^3-10^6 varianti legali a un hash ciascuna e con emittente colluso la ricerca e congiunta, mitigato dalla copertura a due emittenti; RF-106 eligible_node_count senza radice committata, incoerente con il criterio di fissare-ora-il-formato che AGENT-001 ha applicato a RF-013. Low: RF-107 wire.md omette lo stream di enrollment dai limiti di concorrenza, RF-108 pubblicazione di HostAcceptancePolicy non definita. Sulle tre scelte di giudizio indipendente: concorrenza direzione giusta e mitigazione incompleta; governare il profilo di costo e intuizione giusta con vincolo sbagliato; stallo dichiarato accettabile e verificato (innescarlo richiede un quorum, che puo fermare la catena comunque) e la scelta di non toccare BlockHeader e corretta e confermata, ma la frase sul light client no. Nessuna raccomandazione di modifica ad ADR-007, che la verifica rafforza: il fondo a tetto limita anche il residuo di reputazione che il tetto k non chiude, quindi non e un secondo canale."
    evidence_refs: ["SPEC-001", "REVIEW-002", "ADR-007", "SPEC-004", "docs/protocol/README.md", "docs/protocol/identity.md", "docs/protocol/ledger.md", "docs/protocol/wire.md", "docs/protocol/app-manifest.md"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-006-EVENT-003"
    timestamp: "2026-08-25T02:40:17.978814200+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Rimediati tutti e otto i finding di REVIEW-006 su docs/protocol. RF-101: i bound di RFC 9106 non sono piu presentati come vincoli di sicurezza; il costo minimo e ora una regola di validita sull'accettazione del documento enrollment_parameters, con lo stesso schema del tetto kn<kd. Contestata e corretta la forma proposta: 'iterations >= 3' rifiuterebbe la PRIMA configurazione raccomandata da RFC 9106 (t=1,p=4,m=2^21, 2 GiB, che la RFC chiama 'a uniformly safe option'), cioe la piu forte delle due; ho quindi imposto memory_kib*iterations >= 196608 in u128 controllato come pavimento di area PIU memory_kib >= 65536 come vincolo separato, perche l'area da sola ammetterebbe 8 KiB x 24576 passate, che e compute-bound e GPU-friendly, cioe la proprieta per cui ADR-007 ha scartato SHA-256. Sotto questa forma entrambe le raccomandazioni RFC sono valide e tutto cio che e piu debole di entrambe e invalido; cinque fixture al confine incluse 65535/65536 e 65536/2 e 2097152/1; memory_kib e iterations dichiarati parametri di sicurezza e legati al dispositivo di riferimento. Il valore numerico resta quello raccomandato dal Lead: cambiata la forma del vincolo, non la taratura. RF-102: rimossa la frase 'A light client needs no new field to see this' con la spiegazione del perche era falsa e dell'attacco che consentiva; il checkpoint porta ora revoked_validators e revocation_root e il passo 4 dell'algoritmo light-client applica le regole 1 e 2 della transizione forzata con quei dati; dichiarata la tensione fra min_revocation_effective_delay_blocks e max_weak_subjectivity_age_ms con il vincolo che il secondo non superi la durata attesa del primo; nessun campo aggiunto a BlockHeader. RF-103: schema normativo WeakSubjectivityCheckpoint in un unico punto che risolve la discordanza README/ledger, dominio di firma dedicato coblox-weak-subjectivity-signature-v0, voce nel registro delle preimmagini con chain_id_32, fixture WSC-0 sha256:2bc543a3f8e4df60735e6431a6c1fb7293ed53047e98fe2e5bc1a879f200c71e nella tabella di conformita, primitive revocation_leaf/node/empty con REVL-0 e radice vuota pubblicate, network-release trust key definita con provenienza pluralita rotazione a due release sovrapposte recupero fuori banda e limite dichiarato, circolarita di max_weak_subjectivity_age_ms risolta leggendolo dal checkpoint firmato. RF-104: scudo di ammissione fra il passo 7 e la valutazione memory-hard, che diventa il passo 9; accettata la classe di contromisura di AGENT-007 (puzzle interattivo non precomputabile, SHA-256 di proposito per la verifica costante, con la ragione scritta nel documento) ma contestata su due punti. Primo, 'millisecondi una volta' non regge: per assorbire ~10^10 H/s contro una capacita di poche decine di valutazioni al secondo servono ~2^28 tentativi, cioe decine di secondi sul dispositivo di riferimento, piu della proof of work che lo scudo protegge, e pagati una volta per validatore; ho quindi reso admission_difficulty_bits adattivo alla saturazione osservata, zero sotto soglia e con un massimo normativo che non puo superare il tempo che lo stesso dispositivo spende nella proof of work. Secondo, il puzzle da solo non sposta l'asimmetria giusta perche il quorum obbliga l'onesto a soddisfarlo presso ~2/3 dei validatori mentre l'attaccante ne satura ~1/3; ho affiancato la validazione della sorgente con nonce legato al Peer ID autenticato e all'indirizzo osservato, monouso e a vita breve, che costa all'attaccante un indirizzo raggiungibile per slot concorrente. Formato fissato ora sul filo (enrollment_admission_request/_challenge, wrapper EnrollmentSubmission, codice invalid_admission, registro admission_tag con fixture ADM-0) perche dopo sarebbe incompatibile; dichiarato che la disponibilita dell'enrollment non e una garanzia di protocollo. RF-105: limite di grinding riscritto con il suo ordine di grandezza, 10^3-10^6 timestamp_ms legali a un SHA-256 ciascuno sotto collusione emittente-proposer, con il riconoscimento che il commit-reveal blinda davvero il proposer non colluso, le due riduzioni non prese in v0, e la copertura a due emittenti dichiarata in wire.md come LA mitigazione del grinding con rinvio incrociato. RF-106: eligible_set_root richiesto in MintBody per existence_income con primitive eligible_leaf/node/empty; i validatori full che gia ricalcolano E dalla stessa evidenza ricalcolano anche la radice, quindi il conteggio passa da asserzione a fatto falsificabile; limite residuo del light client dichiarato; esempio canonico del mint rigenerato. RF-107: wire.md nomina lo stream di enrollment per primo nei limiti di concorrenza con rinvio ai bound di identity.md. RF-108: HostAcceptancePolicy dichiarata oggetto locale e non di rete, assegnazione ottimistica, rifiuto come canale di scoperta previsto. Verifica reale eseguita e incollata nella spec: 49 controlli tutti PASS, fra cui ER-0 PD-0 object_id input_hash che riproducono i digest gia pubblicati (canonicalizzatore di controllo), ADM-0 WSC-0 REVL-0 e radice vuota ricalcolati dalle definizioni, 19 esempi canonici tutti JCS-identici, 40 link e ancore locali tutti risolti, node_id derivato dalla chiave pubblica, e gli invarianti di Lotto A e B intatti (predicato di quorum e confine V=101, equazione ZIP-215 cofactored e divieto cofactorless, tabella dei 12 vettori speccheck, tetto kn<kd, difficolta 2-6, fondo di esistenza a tetto, regole di validita della revoca). Contabilita del diff: un solo esempio canonico rimosso e uno aggiunto, il mint di esistenza. lmbrain_validate: unique_ids=true, zero errori di lifecycle o contratto, solo le 11 diagnostiche informative di vocabolario dei tag gia presenti. RFC 9106 riconsultata sul testo primario. Nessun commit, nessun push, nessuna modifica ad ADR roadmap PROJECT .lmbrain/design o workspace Rust, algoritmo di elezione dei validatori non toccato."
    evidence_refs: ["SPEC-001", "REVIEW-006", "ADR-007", "docs/protocol/README.md", "docs/protocol/identity.md", "docs/protocol/ledger.md", "docs/protocol/wire.md", "docs/protocol/app-manifest.md"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-006-EVENT-004"
    timestamp: "2026-08-25T15:00:51.067989600+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "superseded"
    actor_role: "project-lead"
    reason: "Superata da REVIEW-007, che ha eseguito la verifica finale dello stesso GATE-SECREVIEW su SPEC-001 dopo la terza remediation e lo ha attestato come superato. REVIEW-007 registra che tutti i finding di questa review sono stati chiusi, e che in un caso la forma alternativa scelta dall'implementatore e migliore di quella indicata qui: la reviewer dichiara che una propria condizione di chiusura era sbagliata e che applicarla alla lettera avrebbe peggiorato il risultato. Questa review non e quindi mai arrivata all'accettazione, e il suo contenuto resta valido come snapshot del secondo giro. Disposizione emessa il 2026-08-25 dal Lead entrante, per la stessa ragione registrata su REVIEW-002."
    evidence_refs: ["REVIEW-007", "SPEC-001"]
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
    action: "transitioned changes-requested -> superseded"
---
# Review

## Outcome

**Changes requested. `GATE-SECREVIEW` non è superato.**

Non lo dico per prudenza. Dei 18 finding di [REVIEW-002] ne considero **15 chiusi bene**,
e la qualità della remediation è alta: le chiusure di RF-001, RF-002, RF-008, RF-009,
RF-013, RF-016 e RF-018 sono migliori di quanto avessi chiesto. Ma tre chiusure hanno
lasciato un residuo che si trova solo verificandole, e le tre scelte di giudizio
indipendente di AGENT-001 — che nessuno aveva ancora rivisto — ne hanno introdotto un
quarto. Sono **quattro finding high**, e tre delle quattro sono la stessa classe di
errore: *una difesa è stata costruita e poi lasciata revocabile o non verificabile da
chi doveva proteggere.*

In sintesi, ciò che il gate deve ancora vedere:

1. **Il pavimento memory-hard di [ADR-007] è revocabile da un parameter set firmato**
   che resta conforme alla specifica (RF-101). I bound di RFC 9106 non sono un
   pavimento di sicurezza: sono il *dominio della funzione*. `memory_kib` può scendere
   legalmente a 8 KiB.
2. **RF-004(c) è chiuso per i full node e non per il light client**, mentre `ledger.md`
   afferma esplicitamente il contrario (RF-102).
3. **Il checkpoint di soggettività debole — unica difesa contro RF-003 — non ha schema,
   né dominio di firma, né preimmagine nel registro** (RF-103). È l'ancora di fiducia
   dell'intero light client e come scritta non è implementabile in modo interoperabile.
4. **I limiti di concorrenza sull'enrollment sono corretti e chiudono il DoS di memoria,
   ma rendono l'enrollment negabile** a costo di circa un core per validatore (RF-104).

Nessuno dei quattro chiede di cambiare architettura, e tre si chiudono con poche righe
normative. Ma sono tutti e quattro nelle **tre aree del gate** — identità, enrollment,
light client — e ognuna delle tre ne ha almeno uno.

## Acceptance-criteria compliance

Valuto il solo `GATE-SECREVIEW`. Confronto con l'esito di [REVIEW-002].

| Area di focus | [REVIEW-002] | Ora | Sintesi |
| --- | --- | --- | --- |
| **Identità** | Non superata | **Non superata** (per poco) | RF-001, RF-004(a,b), RF-010 chiusi e verificati sui vettori. Resta RF-102: la revoca è invisibile al light client, che è metà della premessa di prodotto. |
| **Enrollment** | Non superata | **Non superata** | Primitiva, scadenza, ordine di validazione, dichiarazione dei limiti: tutto chiuso e ben fatto. Ma il costo è governabile a zero (RF-101) e il canale è negabile (RF-104). |
| **Light client** | Non superata | **Non superata** | RF-002, RF-011, RF-016 chiusi con fixture. I 7 passi sono 9 e sono corretti. Ma il passo 1 poggia su un oggetto non specificato (RF-103) e il passo 4 non può vedere le revoche (RF-102). |

Le tre aree sono più vicine al superamento di quanto il conteggio dei finding suggerisca:
il lavoro strutturale è fatto. Ciò che manca sono **pavimenti e ancore**, non meccanismi.

## Code observations

Verifiche positive. Le riporto perché sono affermazioni che ho controllato, non concesso,
e perché una review che elenca solo ciò che non va dà al Lead una stima falsa del rischio.

**RF-001 è chiuso in modo esemplare, e la tabella dei vettori è internamente corretta.**
`README.md` §"Consensus-critical Ed25519 verification" scrive **l'equazione**, non il nome
di una libreria: `[8][S]B = [8]R + [8][k]A`, con `0 <= S < L`, riduzione ZIP-215 delle
codifiche non canoniche di `A`/`R`, e `[8]A != identity`. L'equazione cofactorless è
vietata per nome, e `ed25519-dalek::verify_strict` è vietato esplicitamente. Ho
**ricontrollato la tabella degli esiti attesi sui 12 vettori di `ed25519-speccheck`
contro la regola scritta**, non contro la memoria: i vettori 0, 1, 10 e 11 hanno `A` di
ordine piccolo (nei due ultimi in codifica non canonica) e sono correttamente `reject`
per la regola 3, che è la delta dichiarata rispetto a ZIP-215 puro; 6 e 7 hanno `S >= L`
e sono `reject` per la regola 2; 2, 3, 4, 5, 8 e 9 hanno `A` di ordine misto e sono
`accept` sotto l'equazione cofactored, inclusi 8 e 9 che portano `R` non canonica —
esattamente il punto in cui ZIP-215 e `verify_strict` divergono. La tabella non è
copiata da altrove: è coerente con la regola che il documento stesso enuncia.

**RF-002 è chiuso con la formula che avevo chiesto e con le fixture al confine giuste.**
`quorum(signed_power, total_power) := signed_power * 3 > total_power * 2`, in `u128`
controllato, dichiarato unico per finalità, autorizzazione di transazione, certificato di
enrollment, documento di validator set e documento di protocollo. Le fixture citate —
`V=100`: 66 rifiuta, 67 accetta; `V=101`: 67 **rifiuta**, 68 accetta; `V=102`: 68 rifiuta,
69 accetta — riproducono numero per numero il controesempio della mia Verifica 1. Che
`README.md` non ripeta l'aritmetica ma rimandi a `ledger.md` è corretto: una sola fonte.

**RF-008 è chiuso oltre la richiesta, e la parte di auditabilità regge davvero.**
Il registro delle preimmagini contiene tutte e otto le voci che avevo censito, più
`issuer_commitment`, `challenge_randomness` e `enrollment_pow_salt`, ciascuna con dominio
esplicito e `chain_id_32` dove serve. Avevo posto una condizione ulteriore: che per
`policy_hash` e il rate card fosse dichiarato **dove** il documento firmato è reperibile,
"altrimenti l'auditabilità resta teorica". Ho verificato che lo sia: `ledger_range_response`
in `wire.md` porta `protocol_documents: [SignedProtocolDocument]`. Un utente può quindi
ottenere la reward policy firmata e ricalcolare la funzione di ricompensa. **La promessa
di verificabilità di [[PROJECT]] è mantenuta sul lato policy** — il residuo è solo su
`eligible_node_count` (RF-106), che è un divisore, non la funzione.

**Il commit-reveal di RF-013 blinda davvero il proposer, e questo l'ho verificato contro
la mia stessa ipotesi iniziale.** Ero partita convinta che il grinding del proposer sul
blocco-beacon fosse più grave del dichiarato. Lo è, ma non per la ragione che pensavo:
`challenge_randomness` è funzione di `beacon_block_id` **e** di `issuer_secret_32`, che al
momento del beacon non è ancora rivelato. Un proposer non colluso **non può calcolare** la
randomness che otterrà e quindi non può macinarla, per quanti candidati provi. Il
commit-reveal fa esattamente il lavoro per cui è stato introdotto. Il residuo reale
richiede collusione proposer-emittente ed è RF-105, di severità inferiore.

**Il tetto `k` chiude il ciclo, e il residuo di reputazione è più piccolo di come è
dichiarato.** `amount * kd <= kn * counted_subscription_burn_microtokens` con `kn < kd`
imposto all'accettazione del documento rende il ciclo strutturalmente in perdita di
`S(1 - kn/kd)` per abbonato falso per periodo, qualunque curva scelga il simulatore.
`ledger.md` dichiara onestamente che il tetto non impedisce l'acquisto di *reputazione*
finanziato dal reddito di esistenza. Ho verificato l'ordine di grandezza di quel residuo e
**va a favore del documento**: con il fondo a tetto di [ADR-007], il reddito di esistenza
totale per epoca è limitato a `F`, quindi la spesa complessiva in reputazione falsa di
tutta la rete è a sua volta limitata da `F` — cioè dallo stesso parametro `α` che l'ADR
sorveglia. Non è un canale illimitato: è lo stesso canale, già prezzato. **Non lo elevo a
finding**, e raccomando al Lead di non trattarlo come tale.

**RF-015 e RF-018 sono chiusi meglio della mia condizione di chiusura.** La
`HostAcceptancePolicy` è totale, deterministica e chiusa ("absence denies"), i due percorsi
di consenso sono dichiarati disgiunti con un divieto esplicito di sostituirli, fuori
politica è **rifiuto** senza terza via, e una policy mancante o non parsabile equivale a
`accept_protocol_assignments: false`. Su `http_fetch`: risoluzione una sola volta,
validazione di **ogni** indirizzo restituito, rifiuto dell'intera risposta DNS se un solo
indirizzo è vietato, pinning con SNI preservato, divieto esplicito di ri-risolvere,
procedura ripetuta a ogni redirect, IPv4-mapped IPv6 normalizzato prima della
classificazione, e quattro fixture di conformità. Non ho rilievi.

**RF-012 e RF-016 sono chiusi con il comportamento in saturazione giusto.** La cache di
replay ha cap globale e per peer e in saturazione **rifiuta il nuovo envelope** senza mai
sfrattare una voce ancora viva: fail-closed, non LRU — che è la scelta corretta e quella
che di solito viene sbagliata. La prova Merkle usa "1 **se e solo se**" con fixture
negativa `SMT-1`. La frase "without ambiguity" non è più falsa.

## Tests and verification

Verifiche indipendenti eseguite per questa review, su documentazione primaria corrente
dove il comportamento era incerto.

**Verifica A — i bound di RFC 9106 non sono un pavimento di sicurezza (RF-101).**
`README.md` impone che il parameter set firmato "obey the RFC 9106 limits: `lanes` in
1–16, `memory_kib` at least `8 * lanes`, `iterations` at least 1, and `tag_length_bytes`
exactly 32." Ho consultato [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html) e
riporto le stringhe esatte della sezione dei parametri:

| Parametro | Intervallo RFC 9106 | Natura |
| --- | --- | --- |
| Parallelism `p` | "an integer value from 1 to 2^(24)-1" | dominio |
| Memory `m` | "an integer number of kibibytes from **8\*p** to 2^(32)-1" | dominio |
| Passes `t` | "an integer number from **1** to 2^(32)-1" | dominio |
| Tag length `T` | "an integer number of bytes from 4 to 2^(32)-1" | dominio |

Quei valori sono **l'intervallo di validità dell'algoritmo**, non una raccomandazione.
Le raccomandazioni della RFC sono altre due, e sono configurazioni intere: "Argon2id with
t=1 iteration, p=4 lanes, m=2^(21) (2 GiB of RAM)" oppure "t=3 iterations, p=4 lanes,
m=2^(16) (64 MiB of RAM)". Il documento ha adottato il vincolo sbagliato dei due:
`memory_kib >= 8 * lanes` con `lanes = 1` significa **`memory_kib >= 8`**, cioè 8 KiB.
Conseguenza aritmetica: una GPU da 24 GB ospita `24·2^30 / 8192 ≈ 3,1 milioni` di
valutazioni concorrenti, contro le ~375 del profilo a 64 MiB su cui [ADR-007] ha fondato
il proprio "due ordini di grandezza". Il fattore recuperato dall'attaccante è ~8.000×, che
è **più grande del vantaggio 2.750× che RF-005 aveva misurato per SHA-256** e che ha
motivato l'intera ADR.

**Verifica B — il light client non può vedere una revoca (RF-102).**
`ledger.md` §"Revocation forces a validator set transition" afferma: "A light client needs
no new field to see this. Because the set must change, it observes the transition through
the mechanism it already verifies." Ho ripercorso i 9 passi con quella affermazione in
mano. Il light client verifica (passo 4) la continuità `validator_set_hash[h] ==
next_validator_set_hash[h-1]` e ogni `key_binding_signature`. Non legge transazioni —
`ledger.md` lo dice esso stesso due paragrafi sopra: "a light client, which checks only
set-hash continuity and never sees transactions". Quindi il client osserva *una*
transizione se avviene, ma **non può stabilire che una transizione fosse dovuta**, né chi
fosse il revocato. Una catena in cui la transizione non è mai avvenuta soddisfa la
continuità e tutte le key binding: è **indistinguibile** per il light client. L'affermazione
del documento è falsa nella direzione che conta. Il contenimento è il checkpoint del passo
1: la finestra di esposizione è al più `max_weak_subjectivity_age_ms`. Ma è proprio la
finestra dell'emergenza, e `min_revocation_effective_delay_blocks`, scelto lungo per
evitare lo stallo, la **allunga**: il documento non nota che i due parametri tirano in
direzioni opposte.

**Verifica C — lo stallo dichiarato è accettabile, e questo va detto (RF-004(c), parte
positiva).** Ho verificato chi può innescarlo. `revoke_identity` richiede una
`TransactionQuorumCertificate`, cioè `signed_power * 3 > total_power * 2`. Un attaccante
capace di far finalizzare una revoca controlla già più di due terzi del potere di voto, e
con quel potere può fermare la catena comunque, senza bisogno della revoca. **Lo stallo non
concede quindi alcuna capacità nuova a nessun attaccante sotto la soglia di quorum**, ed è
una scelta di safety-over-liveness legittima. Sul secondo quesito del Lead — se il light
client possa distinguere lo stallo da una partizione — la risposta è **no, e non è un
problema di safety**: in entrambi i casi il passo 5 rifiuta le punte più vecchie di
`max_current_balance_age_ms` e il client mostra il saldo come non aggiornato invece che
come corrente. Fallisce chiuso in entrambi gli scenari. È un limite di *diagnosticabilità*,
non di sicurezza, e lo classifico come tale. **Confermo la scelta di AGENT-001 di non
aggiungere campi all'header**: il costo era reale e il beneficio nullo *per i full node*.
Il difetto non è il campo mancante nell'header, è l'ancora mancante nel checkpoint (RF-102).

**Verifica D — costo dell'attacco di starvation sull'enrollment (RF-104).**
I limiti di identity.md sono corretti e il documento è onesto nel dire che "Ordering alone
is necessary but **not sufficient**". Ho misurato che cosa comprano. Un attaccante genera
coppie di chiavi gratis, firma richieste sintatticamente perfette e le invia. Le richieste
superano i passi 1–7 (`recent_block_id` è uno solo, riusabile per tutte; il profilo di
costo è quello attivo; le chiavi sono fresche quindi il passo 5 non le blocca) e falliscono
solo al passo 8 — dopo aver occupato uno slot memory-hard.

| Lato | Costo per richiesta |
| --- | --- |
| Attaccante | una firma Ed25519 ≈ **20 µs** su un core |
| Validatore | verifica firma ≈ 50 µs **più uno slot da 64 MiB per ~200 ms** |

L'asimmetria sullo stadio memory-hard è dell'ordine di **10⁴:1**. Un validatore con un
budget di picco dichiarato di 4 GiB regge 64 valutazioni concorrenti; con 16 core reali la
sua portata è dell'ordine di poche decine di valutazioni al secondo. Un solo core
d'attacco, che firma ~50.000 richieste al secondo, la satura con tre ordini di grandezza di
margine. Il moltiplicatore che rende l'attacco interessante: **il certificato di enrollment
richiede un quorum**, quindi non serve saturare tutti i validatori — basta superare **un
terzo del potere di voto** perché nessuna richiesta raggiunga più la soglia. Su un set da
100 membri sono ~34 validatori, cioè **~34 core**. Il risultato è il blocco permanente
dell'onboarding di una rete il cui prodotto è l'onboarding di dispositivi ordinari.

Nota, contro la mia ipotesi iniziale: ho controllato se il limite "at most one in-flight
step-8 evaluation per public key" fosse a sua volta un vettore, cioè se un attaccante
potesse occupare lo slot della vittima replicandone la richiesta pubblica. **Non lo è**:
il passo 4 verifica la firma, quindi solo richieste genuinamente firmate dalla vittima
occupano il suo slot, e quelle superano anche il passo 8 e la enrollano. Il limite per
chiave è sano. Il problema è solo il limite globale.

**Verifica E — superficie di grinding del proposer (RF-105).** Come detto in Code
observations, il commit-reveal impedisce al proposer non colluso di macinare. Ho quindi
misurato il caso colluso. `BlockHeader` porta `timestamp_ms`, vincolato solo a essere
maggiore della mediana degli 11 blocchi precedenti e non oltre il drift massimo: a
granularità di millisecondo la finestra offre dell'ordine di **10³–10⁶ valori legali**,
ognuno dei quali produce un `block_id` diverso al costo di **un solo SHA-256** su un header
di poche centinaia di byte. Il dichiarato "it can discard a candidate block and try
another" descrive quindi un costo per tentativo molto più alto di quello reale: non serve
scartare blocchi, basta variare un campo. Con l'emittente colluso che fornisce il segreto,
la coppia può cercare congiuntamente un beacon che soddisfi *due* condizioni — che la
funzione di assegnazione produca proprio quella coppia (emittente, soggetto) e che la
randomness selezioni il chunk conservato. Il vincolo "every subject MUST be covered by at
least two distinct issuers per epoch" di `wire.md` **limita davvero il guadagno**, perché
il secondo emittente onesto interroga comunque il soggetto: l'attacco degrada da "superare
la challenge" a "superarne una su due". Per questo è medium e non high.

## Production quality and documentation compliance

Conforme a [[QUALITY]], e su un punto in modo notevole. §"Declared limits of this
mechanism" di `identity.md` è il miglior testo di sicurezza del progetto: dichiara che il
protocollo non distingue `N` nodi emulati da `N` dispositivi, che è una proprietà
**permanente e deliberata** di v0 e "MUST NOT be described as a temporary gap", che un
costo una tantum non può prezzare un flusso perpetuo, e che il contenimento è economico e
non crittografico — chiudendo con l'accoppiata corretta: robusta contro la falsificazione,
non resistente ai Sybil per via crittografica, "and those two claims must be stated
together". Il rilievo di onestà dichiarativa di [REVIEW-002] è chiuso senza riserve.

Rilievo di coerenza: l'onestà del §"Declared limits" e quella del §"Declared limit" di
`ledger.md` sul grinding non hanno lo stesso standard. Il primo quantifica; il secondo usa
"bounded" senza numero, e la Verifica E mostra che il bound reale è più largo di quanto la
parola suggerisca. Un limite dichiarato senza ordine di grandezza è a metà strada fra una
dichiarazione e una rassicurazione. Lo tratto in RF-105.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

- RF-101 | category=security-boundary | severity=high | criterion=enrollment — il pavimento memory-hard è governabile fino all'irrilevanza | remediation=**I bound di RFC 9106 non sono vincoli di sicurezza, e usarli come tali rende il pavimento di [ADR-007] revocabile da un parameter set che resta pienamente conforme.** `README.md` impone che il set firmato "obey the RFC 9106 limits: `lanes` in 1–16, `memory_kib` at least `8 * lanes`, `iterations` at least 1". La Verifica A mostra che quelli sono il *dominio della funzione*, non una configurazione sicura: la RFC raccomanda separatamente due profili interi (2 GiB/t=1/p=4 oppure 64 MiB/t=3/p=4), e nessuno dei due è stato reso vincolante. Il documento ha dato un pavimento normativo alla manopola meno importante — `difficulty_bits` è vincolato 2–6 — e nessun pavimento a quella che determina il costo reale. **Attacco:** (1) si fa accettare un parameter set con `memory_kib: "8"`, `lanes: "1"`, `iterations: "1"`, `tag_length_bytes: "32"`, `difficulty_bits: "2"`; (2) **nessuna regola di validità lo rifiuta**: rispetta ogni vincolo scritto, e il passo 6 dell'ordine di validazione confronta l'eco della richiesta con il set attivo, non con un minimo; (3) il costo di enrollment diventa ~4 valutazioni di un Argon2id da 8 KiB, cioè microsecondi; (4) una GPU da 24 GB ospita ~3,1 milioni di valutazioni concorrenti invece di ~375, un fattore ~8.000× — **superiore al 2.750× misurato per SHA-256 in RF-005**, che è la misura per cui [ADR-007] esiste; (5) il pavimento memory-hard è tornato a zero senza che alcun nodo abbia violato la specifica e senza alcun segnale on-chain che lo denunci. Aggravante di percezione: `identity.md` continua a dichiarare "Argon2id raises the floor by about two orders of magnitude over SHA-256", affermazione che diventa falsa sotto un set conforme. **Chiusura verificabile:** trasformare i minimi in regole di validità del documento, esattamente con il pattern che `ledger.md` usa già per il tetto al creatore ("`kd > 0` and `kn < kd` enforced when the document is accepted"): un parameter set con `memory_kib < 65536`, oppure `iterations < 3`, oppure `lanes` fuori 1–16, è **invalido all'accettazione** e non solo sconsigliato; il minimo di `memory_kib` è legato per iscritto al dispositivo di riferimento dichiarato, così che abbassarlo richieda di ridichiarare il dispositivo. Fixture al confine: set con `memory_kib: "65536"` valido, `"65535"` invalido. Dichiarare inoltre che `memory_kib` è un parametro **di sicurezza** e non di prestazione, così che la governance futura sappia che cosa sta toccando. **Costo:** nullo a runtime, due righe normative e una fixture. È la correzione con il rapporto beneficio/costo più alto di questa review.

- RF-102 | category=security-boundary | severity=high | criterion=light client — la revoca di un validatore è invisibile a chi deve esserne protetto | remediation=**RF-004(c) è chiuso per i full node e non per il light client, e `ledger.md` afferma il contrario.** La regola di validità sul set è corretta e la accetto: un set con `activation_height >= effective_height` che contiene un `node_id` revocato è invalido, così come ogni blocco e ogni QC contati contro di esso. Per i full node il buco è chiuso. Ma il documento aggiunge: "A light client needs no new field to see this. Because the set must change, it observes the transition through the mechanism it already verifies." La Verifica B mostra che è falso: il light client verifica continuità di hash e `key_binding_signature`, non legge transazioni — cosa che `ledger.md` stesso afferma due paragrafi sopra — e quindi **non conosce l'esistenza della revoca né l'identità del revocato**. Osserva una transizione se avviene; non può stabilire che fosse dovuta. **Attacco:** (1) le chiavi di consenso di un insieme di validatori che somma oltre due terzi del potere trapelano — lo scenario esatto per cui la regola è stata scritta; (2) la governance finalizza le revoche con `effective_height` H; (3) i full node onesti rifiutano da H ogni set che li contenga, e la catena **stalla** come dichiarato: safety preservata; (4) l'attaccante, che possiede quelle chiavi, firma con il **vecchio set** una catena parallela da H in poi — hash-continua da H-1, con ogni `key_binding_signature` valida, perché i binding non scadono con la revoca; (5) i light client eseguono i 9 passi su quella catena e **li superano tutti**; (6) risultato: i full node si fermano e i light client seguono l'attaccante. La scelta safety-over-liveness protegge la popolazione che non era a rischio e lascia scoperta quella che [[PROJECT]] mette al centro, cioè Android e desktop. **Contenimento esistente, da riconoscere:** il passo 1 impone un checkpoint non più vecchio di `max_weak_subjectivity_age_ms`, quindi l'esposizione è limitata a quella finestra e dopo il client fallisce chiuso. Non è quindi illimitata — ma è **la finestra dell'emergenza**, e `min_revocation_effective_delay_blocks`, che il documento raccomanda lungo per rendere lo stallo raro, la allunga: i due parametri tirano in direzioni opposte e il documento non lo dice. **Chiusura verificabile:** (a) il checkpoint di soggettività debole porta un impegno alle revoche attive — è sufficiente `min_validator_set_activation_height`, oppure un digest dell'insieme dei `node_id` revocati con la relativa `effective_height` — e il light client rifiuta ogni header il cui set sia anteriore a quella soglia; (b) cancellare o riscrivere la frase "A light client needs no new field to see this", che oggi è un'affermazione di sicurezza non vera e che ha motivato la scelta di progetto; (c) dichiarare esplicitamente la tensione fra `min_revocation_effective_delay_blocks` e `max_weak_subjectivity_age_ms`, con la regola che il secondo non deve superare il primo. **Costo:** un campo nel checkpoint, che è un oggetto fuori catena già distribuito con la release. Nessun byte sul filo, nessun campo nell'header — la scelta di AGENT-001 di non toccare `BlockHeader` **resta corretta** e questa chiusura la preserva.

- RF-103 | category=security-boundary | severity=high | criterion=light client — l'ancora di fiducia introdotta dalla remediation non è specificata | remediation=**Il checkpoint di soggettività debole è ora la sola difesa contro l'attacco di lungo raggio di RF-003, ed è l'unico oggetto firmato del protocollo privo di schema, dominio e preimmagine.** La chiusura di RF-003 è strutturalmente giusta: il passo 1 impone un checkpoint recente, il passo 3 la non regressione persistita, il rifiuto della sincronizzazione da sola genesi è esplicito, e `ledger.md` §"Validator-set continuity" ha aggiunto l'obbligo di distruggere o ruotare la chiave di consenso all'uscita dal set. Ma il perno di tutto è descritto solo così: `README.md` §"Trust anchors" dice che il checkpoint è `(height, block_id, timestamp_ms, validator_set_hash)` e "signed by the network-release trust key", e `ledger.md` passo 1 dice "Load a signed checkpoint containing chain ID, finalized height/block ID, validator-set hash, and issued time". **Le due descrizioni non concordano nemmeno sui campi** (`chain ID` compare in una e non nell'altra). Non esiste uno schema, non esiste un dominio `coblox-...-v0` per quella firma, il registro delle preimmagini — che ha 12 voci e le ha tutte — **non ne contiene nessuna**, e la "network-release trust key" non ha provenienza, formato, rotazione né procedura di revoca definiti in alcun documento. La regola globale di `README.md` dice che ogni input di firma è "the ASCII domain shown by the schema" — ma qui non c'è schema, quindi non c'è dominio. **Attacco (divergenza, la stessa classe di RF-008 ma sull'ancora di fiducia):** (1) due implementazioni serializzano il checkpoint diversamente, o lo firmano con dominio diverso, o una include `chain_id` e l'altra no; (2) il checkpoint distribuito con la release ufficiale è valido per l'una e invalido per l'altra; (3) i client della seconda implementazione **falliscono chiuso e non sincronizzano affatto**, oppure — l'esito peggiore e quello probabile sotto pressione di prodotto — l'implementazione rilassa il controllo del checkpoint per "compatibilità", e RF-003 si riapre integralmente su quella metà della rete. **Attacco (chiave):** senza procedura di rotazione dichiarata, la compromissione della trust key di release consente di emettere un checkpoint che ancora i client nuovi a una catena scelta dall'attaccante, e nessun documento dice come la rete se ne accorga o come si recuperi. **Chiusura verificabile:** (a) uno schema `WeakSubjectivityCheckpoint` normativo, con i campi fissati una volta sola e coerenti fra `README.md` e `ledger.md`; (b) un dominio dedicato, `coblox-weak-subjectivity-checkpoint-v0`, e la relativa voce nel registro delle preimmagini con `chain_id_32`, più una fixture nella tabella di conformità come per le altre 11; (c) la definizione della network-release trust key — dove risiede, come è distribuita, come si ruota, che cosa fa un client quando incontra un checkpoint firmato da una chiave che non conosce; (d) risolvere la dipendenza circolare per cui `max_weak_subjectivity_age_ms` è un parametro di consenso che va letto dalla catena prima di potersi fidare della catena, dichiarando che il valore usato al passo 1 è quello del bundle di release firmato. **Costo:** redazione, nessun costo a runtime. È l'ultimo pezzo mancante di RF-003 e senza di esso RF-003 non è chiuso, solo abbozzato.

- RF-104 | category=robustness | severity=high | criterion=enrollment — i limiti di concorrenza chiudono il DoS di memoria e aprono quello di ammissione | remediation=**La mitigazione di AGENT-001 è corretta, necessaria e insufficiente, e il documento lo dice a metà.** Do atto che la scelta era giusta: aver riconosciuto spontaneamente che mettere il proof of work per ultimo non basta, e aver aggiunto limite per chiave, limite globale, coda limitata che scarta con `rate_limited` e fail-closed, chiude il DoS di **esaurimento memoria**, che era il rischio che avevo segnalato in RF-005. Ho anche verificato che il limite per chiave non sia a sua volta un vettore contro un nodo onesto (Verifica D): non lo è, perché solo richieste genuinamente firmate occupano lo slot di una chiave. Ma la mitigazione **converte** il problema invece di risolverlo: bounded memory diventa bounded ammissione, e l'ammissione è la porta d'ingresso della rete. **Attacco:** (1) l'attaccante genera coppie di chiavi Ed25519 gratis e firma richieste sintatticamente perfette, riusando un solo `recent_block_id` per tutte e l'attuale profilo di costo; (2) ogni richiesta supera i passi 1–7 — chiavi fresche, quindi il passo 5 non la blocca — e fallisce solo al passo 8, **dopo** aver occupato uno slot da 64 MiB per ~200 ms; (3) l'asimmetria è ~20 µs di firma contro uno slot memory-hard, dell'ordine di **10⁴:1**; (4) un validatore con budget di picco 4 GiB e 16 core regge poche decine di valutazioni al secondo e viene saturato da **un solo core** d'attacco, con tre ordini di grandezza di margine; (5) e non serve colpirli tutti: poiché il certificato di enrollment richiede `signed_power * 3 > total_power * 2`, **saturare poco più di un terzo del potere di voto impedisce a qualunque richiesta di raggiungere il quorum**. Su un set da 100 membri sono ~34 core; (6) l'onboarding della rete si ferma a tempo indeterminato, a costo trascurabile e senza che l'attaccante debba possedere né token né identità enrollate. Aggravante di coerenza: `wire.md` elenca i limiti espliciti di concorrenza per "Challenge request and ledger sync streams" e **non nomina lo stream di enrollment**, che è l'unico ad accettare peer non autenticati (RF-107). **Chiusura verificabile:** inserire fra il passo 7 e il passo 8 un **puzzle interattivo a costo di verifica costante**, legato a un nonce effimero fornito dal validatore e quindi non precomputabile né riusabile fra validatori: SHA-256 con pochi bit di zeri va benissimo, e l'ironia è deliberata — SHA-256 è la primitiva sbagliata per un costo Sybil, per la stessa ragione per cui è quella giusta per uno scudo DoS, cioè verifica in un hash. Costo per il richiedente onesto: millisecondi, una volta. Costo per l'attaccante: moltiplicato per il numero di richieste, per validatore, senza possibilità di ammortizzarlo. Dichiarare inoltre in `identity.md` che la **disponibilità** dell'enrollment non è una garanzia di protocollo ma dipende da policy locali non firmate, così come è già dichiarato che l'anti-Sybil è economico: è lo stesso standard di onestà, applicato al secondo caso. Fixture: una raffica di richieste con firma valida e proof of work assente deve lasciare limitati **sia** la memoria **sia** la latenza di ammissione delle richieste oneste. **Costo:** un campo di challenge nello handshake di enrollment e un hash per richiesta. Va fissato ora perché aggiungere un round-trip allo stream di enrollment dopo è un cambio di formato incompatibile — lo stesso argomento che AGENT-001 ha usato, correttamente, per fissare il formato di RF-013.

- RF-105 | category=security-boundary | severity=medium | criterion=challenge — la superficie di grinding del proposer è dichiarata ma sottostimata | remediation=**Il residuo dichiarato è reale, ma la parola "bounded" descrive un costo per tentativo che non è quello vero.** `ledger.md` dichiara: "the proposer of the beacon block has a bounded grinding advantage — it can discard a candidate block and try another." Do atto che il commit-reveal funziona: la Verifica E conferma che un proposer **non colluso non può macinare affatto**, perché `challenge_randomness` dipende da `issuer_secret_32`, non ancora rivelato — il meccanismo fa il suo lavoro e questo va scritto. Il problema è la quantificazione nel caso colluso. Scartare e riprovare un blocco candidato suggerisce un costo di un round di consenso per tentativo; in realtà `BlockHeader` porta `timestamp_ms`, vincolato solo a superare la mediana degli 11 blocchi precedenti e a non eccedere il drift massimo, quindi a granularità di millisecondo offre **10³–10⁶ valori legali**, ognuno dei quali produce un `block_id` diverso al costo di **un solo SHA-256** su poche centinaia di byte. Il proposer non scarta nulla: enumera. **Attacco:** (1) il proposer del blocco-beacon e un emittente colludono, e l'emittente gli passa il proprio segreto già impegnato; (2) il proposer enumera i timestamp legali, ricalcolando per ognuno l'assegnazione dell'epoca e la `challenge_randomness` risultante; (3) cerca un beacon che soddisfi due condizioni insieme — che la funzione di assegnazione accoppi proprio quell'emittente al soggetto bersaglio, e che la randomness selezioni il chunk che il soggetto ha conservato; (4) con 10⁵–10⁶ tentativi gratuiti la ricerca congiunta è alla portata per spazi di selezione piccoli; (5) il soggetto supera una challenge di storage per dati che non conserva. **Mitigazione già presente, che è la ragione della severità medium:** `wire.md` impone che "every subject MUST be covered by at least two distinct issuers per epoch, none of which is the subject", quindi il secondo emittente onesto interroga comunque il soggetto e l'attacco degrada da "superare la challenge" a "superarne una su due" — il che lo rende una riduzione del tasso di rilevamento, non un bypass. **Chiusura verificabile:** (a) ridurre la superficie quantizzando `timestamp_ms` allo slot di consenso, o derivando il materiale del beacon dai `block_id` di `K` blocchi consecutivi così che macinare richieda `K` proposte consecutive dello stesso attaccante; (b) riscrivere il limite dichiarato con il suo **ordine di grandezza** e con la condizione che lo attiva (collusione emittente-proposer), applicando lo stesso standard del §"Declared limits of this mechanism" di `identity.md`, che quantifica; (c) dichiarare che la copertura a due emittenti non è una ridondanza ma **la mitigazione del grinding**, perché oggi le due regole vivono in documenti diversi e nessuno le collega. **Costo:** (b) e (c) sono redazione; (a) va coordinato con AGENT-002 su M-02 e può attendere il beacon dedicato, purché il residuo sia dichiarato correttamente adesso.

- RF-106 | category=requirements-completeness | severity=medium | criterion=ledger — `eligible_node_count` non è verificabile e il campo che lo renderebbe tale manca | remediation=**Il limite è dichiarato onestamente ma la decisione di formato è incoerente con il criterio che AGENT-001 ha applicato altrove.** `ledger.md` dichiara: "v0 does not commit a per-epoch eligible-set root, so a light client verifies the arithmetic and the quorum rather than independently recomputing eligibility." La dichiarazione è corretta e il residuo è più piccolo di quanto sembri, perché ho verificato (vedi Code observations) che la reward policy firmata **è** recuperabile via `ledger_range_response.protocol_documents`: un utente può quindi ricalcolare la funzione di ricompensa e conosce `F`. Ciò che non può verificare è il divisore `E`. **Attacco:** (1) un quorum di validatori pubblica mint di `existence_income` con un `eligible_node_count` gonfiato; (2) ogni mint è aritmeticamente perfetto — `amount = F / E` con divisione intera esatta — e supera ogni controllo che un light client sappia eseguire; (3) il reddito di ogni nodo onesto viene ridotto in modo arbitrario, e la differenza resta non coniata quindi non compare come inflazione da nessuna parte; (4) nessun utente può contraddire il numero, perché non esiste un impegno all'insieme degli eleggibili contro cui confrontarlo. Serve un quorum, quindi non è un bypass della soglia di fiducia — ma è precisamente la classe di garanzia che [[PROJECT]] promette come "verificabile in tempo reale", e RF-008 è stato accettato come high per lo stesso motivo applicato a `policy_hash`. **Il punto dirimente è di formato, non di consenso:** AGENT-001 ha correttamente fissato i campi di RF-013 pur rinviando l'algoritmo a M-03, motivandolo così — "the fields it needs are fixed here so that adding it later is not a breaking format change". Lo stesso criterio non è stato applicato qui, e aggiungere `eligible_set_root` a `MintBody` dopo il lancio **è** un cambio di formato incompatibile. **Chiusura verificabile:** riservare ora il campo `eligible_set_root` in `MintBody` per `existence_income`, con la sua preimmagine nel registro sulla falsariga di `subscription_leaf`/`subscription_node`, dichiarando che in v0 può essere l'impegno all'insieme vuoto o non ancora imposto, ma che il campo esiste e la sua serializzazione è fissata. In alternativa, se il Lead preferisce non fissarlo, dichiarare esplicitamente in `ledger.md` che l'aggiunta sarà un cambio incompatibile che richiede una migrazione — così la scelta è presa consapevolmente e non scoperta a M-02. **Costo:** un campo riservato e una voce di registro; molto meno di una migrazione.

- RF-107 | category=documentation | severity=low | criterion=coerenza fra documenti sui limiti di risorse | remediation=**`wire.md` elenca i limiti di concorrenza per ogni stream tranne l'unico non autenticato.** `wire.md` afferma: "Challenge request and ledger sync streams use timeouts and explicit concurrency limits; no untrusted peer can cause an unbounded task, allocation, or retained response." Lo stream di enrollment non è nominato, benché lo stesso documento dichiari che "The enrollment stream accepts unauthenticated transport peers; all other Coblox protocols require a valid enrollment certificate" — cioè è l'unico stream a cui l'affermazione "no untrusted peer" si applica davvero, ed è quello che consuma 64 MiB per richiesta. I limiti esistono, ma vivono solo in `identity.md`. **Rischio:** un implementatore che costruisca il livello di trasporto leggendo `wire.md` — la divisione di lavoro naturale — implementa i limiti per gli stream elencati e non per l'enrollment, e la mitigazione di RF-104 non viene applicata affatto. Non è un attacco: è il modo in cui una difesa specificata nel documento sbagliato non viene implementata. **Chiusura verificabile:** `wire.md` nomina lo stream di enrollment nella stessa frase e rimanda a [identity.md#validation-order-and-its-reason](../protocol/identity.md) per i bound specifici. **Costo:** una riga.

- RF-108 | category=requirements-completeness | severity=low | criterion=host acceptance policy — pubblicazione e scoperta non definite | remediation=**`HostAcceptancePolicy` è un oggetto con `schema_version` di cui non si dice dove viva.** La logica di valutazione è completa e corretta e non ho rilievi su di essa (vedi Code observations). Manca però ogni indicazione su come la policy sia pubblicata, se sia firmata, come il percorso di assegnazione del protocollo la scopra, e quale ne sia il limite di dimensione. **Conseguenza, che è di liveness e non di sicurezza:** se la policy è puramente locale, il protocollo assegna alla cieca e apprende i vincoli solo tramite rifiuti, il che rende la lista di rifiuto per nodo di [ADR-006] il canale di scoperta primario invece che l'eccezione. Se invece è destinata a essere pubblicata, allora è un oggetto firmato in più e ricade nella stessa lacuna di RF-103 — nessun dominio, nessuna preimmagine. **Chiusura verificabile:** dichiarare quale delle due sia il caso. Se locale: una frase che dica che la policy non è un oggetto di rete, che l'assegnazione è ottimistica e che il rifiuto è il meccanismo previsto. Se pubblicata: schema, dominio, preimmagine nel registro e limite di dimensione. **Costo:** una frase nel primo caso.

## Required follow-up

1. **AGENT-001 rimedia RF-101, RF-102, RF-103, RF-104.** Sono i quattro che bloccano il
   gate. RF-101 e RF-107 sono poche righe; RF-102 e RF-103 richiedono redazione
   coordinata fra `README.md` e `ledger.md`; RF-104 introduce un round-trip nello stream
   di enrollment e va fissato ora perché dopo è incompatibile.
2. **Priorità.** RF-101 per primo: costa due righe e restituisce ad [ADR-007] la
   proprietà che l'ADR crede di avere. Poi RF-103, perché senza di esso RF-003 non è
   chiuso. Poi RF-102 e RF-104.
3. **Non spetta ad AGENT-001** decidere il valore del minimo di `memory_kib` (RF-101) né
   se riservare `eligible_set_root` (RF-106): la prima è una scelta di taratura che
   dipende dal dispositivo di riferimento e la seconda è una decisione di formato con
   implicazioni economiche. Entrambe vanno sbloccate dal Lead. Io raccomando il profilo a
   64 MiB come minimo e la riserva del campo.
4. **RF-105, RF-106, RF-107, RF-108 possono essere rimediati nello stesso passaggio o
   subito dopo**, ma non impediscono il gate se il Lead decide di accettarli come rischio
   dichiarato — con l'eccezione di RF-105 punto (b), che è una correzione di onestà
   dichiarativa e costa una frase.
5. **Ri-verifica.** RF-101 e RF-103 vanno controllati sulle fixture, non sul testo: un
   parameter set al confine `65535`/`65536` e una fixture di preimmagine del checkpoint
   nella tabella di conformità.
6. **Nessuna delle mie osservazioni tocca [ADR-007], che considero corretta.** Vedi sotto.

## Final decision

**Changes requested. `GATE-SECREVIEW` non è superato**, con 4 finding high, 2 medium e
2 low.

Non è un verdetto con riserve travestito da bocciatura, né il contrario. La distanza dal
superamento è piccola in termini di lavoro — RF-101 sono due righe, RF-107 una — ma i
quattro high stanno tutti e quattro dentro le tre aree che il gate nomina, e uno di essi
(RF-102) è una chiusura che **afferma esplicitamente di coprire il light client mentre non
lo copre**. Un finding chiuso male è peggio di uno aperto, e questa è la ragione per cui
non passo il gate su un lavoro che per il resto è di qualità alta.

**Sulle tre scelte di giudizio indipendente di AGENT-001**, poiché il Lead ha chiesto un
parere esplicito e nessuno le aveva riviste:

- **Limiti di concorrenza sull'enrollment: scelta giusta, mitigazione insufficiente.**
  Aver riconosciuto senza che nessuno lo chiedesse che l'ordine dei controlli non basta è
  la parte migliore della remediation. Il DoS di memoria è chiuso e il limite per chiave
  non è a sua volta un vettore, il che ho verificato. Ma il vettore si è spostato
  sull'ammissione, e per la soglia di quorum basta saturare un terzo del potere di voto
  per fermare l'onboarding della rete (RF-104). **Approvo la direzione, chiedo il
  completamento.**
- **Governare il profilo di costo Argon2id: intuizione giusta, vincolo sbagliato.** Che
  i bit di difficoltà siano una manopola troppo grossolana è **corretto**, e renderli
  governabili insieme al profilo è la scelta giusta. L'errore è aver preso i bound di
  RFC 9106 per vincoli di sicurezza: sono il dominio della funzione, e sotto di essi
  `memory_kib` può scendere legalmente a 8 KiB, restituendo all'attaccante un vantaggio
  ~8.000× — più grande di quello che ha motivato [ADR-007]. **Sì, governare quei
  parametri apre una superficie**, ed è esattamente quella che il Lead sospettava. Si
  chiude con un pavimento normativo (RF-101).
- **Chiudere RF-004(c) senza campi nell'header, accettando lo stallo: metà giusta.** Lo
  stallo **è accettabile** e l'ho verificato invece di concederlo: innescarlo richiede un
  quorum, e chi ha un quorum può fermare la catena comunque, quindi non concede capacità
  nuove a nessun attaccante sotto la soglia. Il light client **non** può distinguerlo da
  una partizione, e non è un problema: in entrambi i casi fallisce chiuso e mostra il
  saldo come non aggiornato. Anche la decisione di non toccare `BlockHeader` è corretta e
  la confermo. Ciò che non regge è la frase che ne è stata tratta — "A light client needs
  no new field to see this" — perché il campo mancante non era nell'header, era nel
  checkpoint (RF-102).

**Su [ADR-007]: non ho raccomandazioni di modifica.** La verifica indipendente che ho
condotto la rafforza su due punti. Il primo: il tetto al fondo di esistenza non solo
impedisce a una flotta emulata di aumentare l'emissione, ma **limita anche il residuo che
il tetto `k` non chiude** — l'acquisto di reputazione è finanziato dal reddito di
esistenza, quindi la spesa totale della rete in abbonati falsi è a sua volta limitata da
`F`, cioè dallo stesso `α` che l'ADR sorveglia. È un canale già prezzato, non un secondo
canale, e raccomando al Lead di non trattarlo come finding aperto. Il secondo: l'esclusione
dell'attestazione hardware regge, e non ho trovato nulla nella remediation che la rimetta
in discussione. L'unico avvertimento è che **l'ADR presuppone un pavimento memory-hard che
la specifica non impone** (RF-101): finché quel pavimento è governabile a zero, il punto 3
della decisione è una raccomandazione e non un vincolo.

**Sul claim "super-sicura", con la franchezza che il Lead mi ha chiesto.**

Oggi il claim è **difendibile nella forma che `identity.md` ha adottato, e in nessun'altra**.
La forma corretta è quella che il documento già scrive: *robusta contro la falsificazione —
saldi, firme, doppia spesa — e non resistente ai Sybil per via crittografica, e le due
affermazioni vanno enunciate insieme.* Su quella prima metà il lavoro è genuinamente
buono e sopra la media del settore: una regola Ed25519 unica scritta come equazione e non
come nome di libreria, con la tabella dei vettori verificata; un predicato di quorum intero
corretto al confine; un divieto di trasferimento reso strutturale; preimmagini di hash
tutte definite con provenienza normativa; un commit-reveal che blinda davvero il proposer.
Poche specifiche a questo stadio hanno tutto questo.

Ciò che oggi **non** è ancora difendibile, e che i quattro high descrivono, è che quelle
difese siano **irrevocabili e verificabili da chi deve fidarsene**. Un pavimento anti-Sybil
che un parameter set conforme può azzerare non è un pavimento, è una consuetudine. Una
revoca che i full node applicano e i light client non possono vedere protegge chi gestisce
un server e non chi ha installato l'app. Un'ancora di fiducia senza schema è una promessa
di interoperabilità, non un meccanismo. Sono tutte e tre la stessa forma di difetto: la
sicurezza è stata *costruita* ma non *chiusa*, e in una specifica è la chiusura a fare
la differenza fra una proprietà e un'intenzione.

Chiuse queste quattro, il gate passa e il claim regge nella sua forma onesta. Non credo
serva altro giro dopo quello.
