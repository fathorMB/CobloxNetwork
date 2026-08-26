---
id: REVIEW-027
# Note: Quote the title if it contains a colon
title: "Security review of SPEC-016 (GATE-SECREVIEW): gli orologi della catena, la banda a due lati e il legame di catena del set"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-016
reviewer: AGENT-007
review_requested_by: AGENT-LEAD
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-027-EVENT-001"
    timestamp: "2026-08-26T02:02:59.223623+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Security review di AGENT-007 su SPEC-016, GATE-SECREVIEW. Un finding high, quattro medium, tre low. Il Lead registra il verdetto raccomandato dalla reviewer e lo condivide.\n\nRF-001 high, riverificato dal Lead sui file: l'argomento dell'asimmetria fuori banda guarda un estremo solo. Vale per il numeratore; il denominatore ha una distorsione propria e opposta, perche' README.md:1122-1125 dice che issued_at_ms e' quando il checkpoint e' stato prodotto e non quando l'altezza che nomina e' stata raggiunta - \"The two are distinct and both are required\" - mentre measure_cadence_from_checkpoint prende i blocchi da checkpoint_height e il tempo da issued_at_ms. I blocchi prodotti durante la latenza di rilascio sono contati senza il loro tempo, e la lettura e' spinta verso FasterThanBand, cioe' verso il lato su cui il client fallisce chiuso, da qualcosa che non e' la catena. Il documento nomina la distinzione e il codice la perde: famiglia 3. E' high perche' la distorsione decresce con l'eta' del checkpoint, quindi servire il checkpoint piu' recente e genuino nega la verifica a un client onesto senza possedere alcuna chiave; e perche' min_measured_blocks e' un pavimento sul numeratore e sul denominatore non ne esiste alcuno.\n\nVa registrato che RF-001 colpisce esattamente la parte che REVIEW-025, del Lead, aveva lodato di piu' e che nessuno aveva attaccato. E' la quinta volta su questo progetto che una misura risulta puntata sulla grandezza sbagliata, e la seconda volta che accade dentro un lavoro gia' accettato dal Lead.\n\nRF-005 medium: C11-CLAIMDOC non ha un lato disco, confronta il manifesto con una costante Python. Un SECURITY-OVERVIEW.md aggiunto alla copia, che dice \"Sybil-resistant\" e \"prevents\", lascia la gate verde. E' RF-001 di REVIEW-025 ancora possibile nella stessa forma: la guardia costruita contro \"la gate misura l'insieme piu' piccolo\" ha essa stessa un insieme dichiarato invece che osservato.\n\nRF-004 medium: le sette probe su SECURITY.md proteggono una direzione sola del paragrafo. RF-002 medium: SECURITY.md attribuisce a un terzo bloccante la cadenza \"in either direction\", ma accelerare richiede un quorum perche' ogni blocco porta un certificato di quorum; README.md e ledger.md non commettono l'errore. RF-003 medium: la classe di DEBT-014 e' enunciata senza \"a dominio separato\" ed e' in quella forma di nuovo falsa, terzo giro su quella frase. RF-006/007/008 low.\n\nSulla domanda 3 la reviewer conferma senza riserve la correzione del Lead a DEBT-013 e contesta la gerarchia dei moventi: il lato veloce costa un quorum contro un terzo, il guadagno e' pro quota e non esclusivo, non ha negabilita' ed e' osservabile senza banda. Il movente dominante resta il rallentamento, e il progetto fallisce chiuso sul lato debole.\n\nSulla domanda 5 conferma che le tre disposizioni di SPEC-009 sono in albero e che il paragrafo anti-Sybil va rafforzato, obbligatoriamente col qualificatore per-epoca, altrimenti la pretesa sarebbe smentita tre righe piu' sotto dallo stesso file. Il rafforzamento e' lavoro del Lead.\n\nCosa ha attaccato senza romperlo, e va registrato perche' e' informazione: il divieto su timestamp_ms, chiuso meglio del richiesto; il legame di catena del ValidatorSet su tutte e tre le superfici, genesi compresa, quindi il rifiuto di DEBT-014 e' corretto nel merito e RF-003 riguarda solo come e' motivato; la derivazione di reward_epoch dal lato dell'evasione; check_cadence_release, che non e' toccato da RF-001 perche' la latenza si cancella fra due issued_at_ms dello stesso processo; e il meccanismo claim_count, che morde davvero."
    evidence_refs: ["SPEC-016", "REVIEW-025", "DEBT-013", "DEBT-014"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-027-EVENT-002"
    timestamp: "2026-08-26T02:28:58.671893600+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Remediation degli otto finding consegnata da AGENT-002. RF-001 chiuso senza invertire l'asimmetria: la formulazione nuova dice che entrambi gli estremi sono distorti verso il basso e che le due distorsioni spingono il rapporto in versi opposti, e sposta il criterio su cosa c'e' oltre la tolleranza. Campo di genesi nuovo max_external_clock_slack_ms, piu' il vincolo simmetrico sulla procedura di rilascio. RF-005 chiuso togliendo il lato dichiarato a entrambe le liste che ne erano prive. RF-002, RF-003, RF-004, RF-006, RF-007, RF-008 chiusi. Da verificare dal Lead."
    evidence_refs: ["SPEC-016"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-027-EVENT-003"
    timestamp: "2026-08-26T02:29:15.427937800+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificata dal Lead rieseguendo: 151 test da 147, published_artifacts.py PASS con 103 candidati C10 e 8 C11, prova in negativo PASS con 15 mutazioni su 11 classi piu' tutte e 103 le probe singolarmente, clippy zero warning, fmt pulito.\n\nRF-001 chiuso senza invertire l'asimmetria e senza la premessa falsa. La formulazione nuova sposta il criterio dove regge davvero: cio' che separa le due direzioni non e' l'attribuibilita' in se' ma cosa c'e' oltre la tolleranza, perche' nulla di onesto fa apparire blocchi mentre una lettura lenta e' indistinguibile dal ritardo del client a qualunque grandezza. Sei siti corretti e non quattro: la review ne nominava quattro, l'implementatrice ha trovato la stessa affermazione anche nell'intestazione di modulo e nella doc di check_cadence_light_client.\n\nHa rifiutato il nome max_release_latency_ms che la review proponeva, e il rifiuto e' corretto: lo stesso ammanco lo producono tre cause additive e indistinguibili dentro la misura, quindi un campo che ne nomina una vincolerebbe un termine di una somma, cioe' la famiglia 3 dentro il rimedio a un finding di famiglia 3. Il campo si chiama max_external_clock_slack_ms, che e' il nome della somma.\n\nHa inoltre sbagliato la regola relazionale e l'ha corretta prima di consegnare, riportandolo invece di nasconderlo: aveva scritto una soglia che vietava esattamente la latenza che il campo esiste per tollerare, e i suoi stessi test l'hanno respinta.\n\nGATE-MEASURE-BINDS ha ora il caso che non aveva: catena onesta a cadenza nominale con latenza di rilascio non nulla, che prima del rimedio dava Err(FasterThanBand) e ora procede. Il residuo e' dichiarato e non nascosto.\n\nRF-005 chiuso un livello sopra il finding, come il giro precedente. Il difetto non era che una lista fosse corta ma che una lista possa essere corta senza che nulla lo dica: le liste prive di lato disco erano due e non una, e ora nessuna. I documenti parcheggiati come non spazzati vengono cercati per vocabolario di pretesa di sicurezza, altrimenti lo scenario SECURITY-OVERVIEW.md si riprodurrebbe classificando invece che nascondendo. La chiusura simmetrica sulle trascrizioni ha trovato alla prima esecuzione quindici valori pubblicati copiati in suite di conformita' e mai confrontati con la fonte; C5 passa da 43 a 57.\n\nIl rafforzamento del paragrafo anti-Sybil e' stato applicato dal Lead con la formulazione esatta della reviewer, qualificatore per-epoca compreso, e la gate e' stata riverificata PASS dopo l'applicazione."
    evidence_refs: ["SPEC-016", "REVIEW-027"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-027-EVENT-004"
    timestamp: "2026-08-26T02:29:31.964193600+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "GATE-SECREVIEW soddisfatta. Otto finding, tutti chiusi e riverificati dal Lead rieseguendo: 151 test, 103 probe C10 e 8 C11, prova in negativo su 15 mutazioni e su tutte e 103 le probe individualmente, protocol_hashes.py senza valori mossi, clippy e fmt puliti.\n\nIl valore di questa review non e' nei numeri. RF-001 ha colpito esattamente la parte che REVIEW-025, del Lead, aveva lodato come la migliore del lavoro, e che nessuno aveva attaccato. L'argomento dell'asimmetria guardava un estremo solo, e la frase falsa era deducibile dalla stessa sezione di README.md che l'implementatrice stava citando mentre la scriveva - il difetto gia' scritto e non guardato, applicato al proprio lavoro, che e' il tratto comune delle quattro famiglie del censimento.\n\nE' la quinta volta su questo progetto che una misura risulta puntata sulla grandezza sbagliata, e la seconda dentro un lavoro gia' accettato dal Lead. La lezione operativa e' che GATE-MEASURE-BINDS provava tre catene tutte a latenza zero: una gate che esercita solo il regime nominale non ha mai visto lo scenario che la rompe.\n\nRegistrata la contestazione della reviewer sulla gerarchia dei moventi, che il Lead accetta: il lato veloce costa un quorum contro un terzo, il guadagno e' pro quota e non esclusivo, non ha negabilita' ed e' osservabile senza banda. Il movente dominante resta il rallentamento, e il progetto fallisce chiuso sul lato debole - il che e' la scelta giusta ma va saputa.\n\nRegistrato anche cio' che la reviewer ha attaccato senza romperlo, perche' e' informazione: il divieto su timestamp_ms, chiuso meglio del richiesto; il legame di catena del ValidatorSet su tutte e tre le superfici, genesi compresa, quindi il rifiuto di DEBT-014 e' corretto nel merito; la derivazione di reward_epoch dal lato dell'evasione; check_cadence_release, esplicitamente non toccato da RF-001 perche' la latenza si cancella fra due issued_at_ms dello stesso processo; e il meccanismo claim_count."
    evidence_refs: ["SPEC-016", "DEBT-013", "DEBT-014", "DEBT-019"]
    implementation_agent: "AGENT-002"
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [review, security]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-26
    action: "recorded review remediation"
  - date: 2026-08-26
    action: "recorded review remediation-verification"
  - date: 2026-08-26
    action: "transitioned changes-requested -> accepted"
---
# Review

## Outcome

**Raccomandazione: changes requested.** Il lavoro è corretto in quasi tutto ciò che dichiara, e in due punti è migliore di quanto la spec chiedesse. Ma la parte che [REVIEW-025] indica come la migliore del lavoro — **l'asimmetria fuori banda** — poggia su una premessa che **non regge**, e la premessa è scritta in quattro artefatti compreso `SECURITY.md`. È il finding che porta il peso di questa passata.

Otto finding: uno `high`, quattro `medium`, tre `low`. Nessuno è a carico della qualità dell'esecuzione: l'implementazione fa esattamente ciò che l'argomento le diceva di fare, e l'argomento è dell'analisi, non del codice.

Rispondo alle sei domande nell'ordine in cui il Lead le ha poste, e le due su cui si aspettava di più — la 2 e la 5 — sono quelle da cui escono RF-001 e la formulazione che propongo per il paragrafo anti-Sybil.

## Acceptance-criteria compliance

Non ho rifatto le prove che il Lead ha già rifatto rieseguendo (147 test, 98 candidati C10 e 4 C11, `published_artifacts_negative.py` con «every one of the 98 probes was observed failing», `protocol_hashes.py` con nessun valore mosso). Le do per acquisite.

Ho eseguito in proprio, **su una copia dell'albero fuori dal repository** (`COBLOX_REPO` puntato allo scratchpad, mai sull'albero condiviso), la passata di `published_artifacts.py` come controllo e **quattro mutazioni mie**, che nessuno aveva provato. Tre di esse hanno prodotto RF-004 e RF-005. Le trascrizioni sono sotto ai finding.

I sette criteri di accettazione sono soddisfatti nella lettera. Il quinto — *«i due paragrafi di [DEBT-014] sono scritti e l'argomento falso non compare»* — è soddisfatto quanto all'argomento falso, e RF-003 riguarda l'argomento **sostitutivo**, non quello rimosso.

---

## Domanda 1 — La chiusura di [DEBT-013] promette più di quanto mantenga?

**Nella forma dichiarata, no, e l'ho verificato per esaurimento sui tre artefatti che possono dirlo.**

- `README.md#genesis-constants`: *«No rule of this protocol prevents that, and none can»*, e *«the slowdown is not prevented, it is made visible»*.
- `ledger.md#block-format`: *«The slowdown is not prevented. It is made visible, and given a threshold that was declared before anyone had a reason to argue about it.»*
- `README.md#cadence-band`: *«A chain running outside its band is not invalid; it is observably outside its band, which is the strongest true statement available and is the one made here.»*
- `SECURITY.md`: *«The manoeuvre is not prevented.»*, sotto un titolo che dice *«measured, not enforced»*.
- `cadence.rs`, documentazione di modulo: *«"closed" would say more than the code says.»*

Le parole «risolto», «impedito», «enforced», «guaranteed» non compaiono in nessuno dei cinque siti. La procedura di rilascio è dichiarata **procedura e non regola di validità**, e la frase che lo dice — *«calling it enforcement would overstate it»* — è quella giusta. `CadenceBand` è documentata come l'unico dei tre oggetti di ancora che **non è confrontato da alcuna regola di validità**, il che è la precisazione che impedisce a un lettore di scambiarla per `ElectionBounds`. Questa parte del lavoro è esemplare, ed è il motivo per cui la famiglia 2 **non** si ripete qui in forma diretta.

**Ma tre affermazioni dicono più del vero, e due sono nel perimetro di questa domanda.**

La prima è la premessa dell'asimmetria — *«la sua misura è distorta verso il basso e solo verso il basso»*, che `SECURITY.md` riassume in *«each fails closed where its own vantage point makes the reading sound»*. È **RF-001**, ed è l'affermazione più forte del lavoro perché è quella che giustifica un fallimento chiuso. Un fallimento chiuso è una pretesa: dice *questa lettura è attribuibile alla catena*. Non lo è.

La seconda è **RF-002**: `SECURITY.md` attribuisce a un **terzo bloccante** la capacità di muovere la cadenza *«in either direction»*. Il lato veloce non costa un terzo, costa un quorum.

La terza è **RF-007**, minore: il limite cumulativo di `ledger.md` è enunciato con `F`, che è una grandezza governata.

**Verdetto della domanda 1: la forma «misurabile e dichiarato, non impedito» è mantenuta. Ciò che promette più di quanto mantenga non è la chiusura, è l'argomento con cui una delle due misure decide da che parte fallire.** È famiglia 2 spostata di un livello: non la frase rimasta indietro rispetto alla regola, ma la frase che **motiva** la regola e non è stata messa alla prova.

---

## Domanda 2 — L'asimmetria fuori banda regge? (attacco)

**No. Esiste più di una posizione da cui una lettura *veloce* è prodotta da qualcosa che non è la catena, e una di esse è la posizione ordinaria.**

L'argomento in albero, nelle sue parole (`cadence.rs:170-175`, `ledger.md` passo 4b, `README.md#cadence-band`):

> Un client non allineato conta meno blocchi di quanti la catena ne ha prodotti, quindi la sua misura è distorta verso il basso e solo verso il basso: una lettura lenta non è attribuibile alla catena da quella posizione, una veloce sì, **perché il ritardo di sync non fabbrica blocchi**.

La proposizione *«il ritardo di sync non fabbrica blocchi»* è vera. La conclusione *«e solo verso il basso»* non ne segue, perché **la misura ha due estremi e l'argomento ne guarda uno solo**. Il numeratore è `tip_height - checkpoint_height`; il denominatore è `now_ms - checkpoint_issued_at_ms`. L'argomento dimostra che il **numeratore** è distorto verso il basso. Non dice nulla sul **denominatore**, e il denominatore ha una distorsione propria, sistematica, **nella direzione opposta**.

### L'attacco che riesce: la latenza di rilascio del checkpoint

`README.md#weak-subjectivity-checkpoint` è esplicito, ed è il documento stesso a fornire l'arma:

> `height`/`block_id`/`validator_set_hash` describe a **finalized** block and its active set; `timestamp_ms` is that block header's timestamp and **`issued_at_ms` is when the checkpoint itself was produced**. The two are distinct and both are required.

I due sono distinti e `issued_at_ms` viene **dopo**. Sia `L` la latenza di rilascio, cioè l'intervallo reale fra il momento in cui l'altezza `H` è finalizzata e il momento in cui il checkpoint su `H` è firmato. Nella misura del light client:

- i blocchi prodotti durante `L` **sono contati** (stanno fra `checkpoint_height` e `tip_height`);
- il tempo trascorso durante `L` **non è contato** (l'orologio parte da `issued_at_ms`).

Sono blocchi gratuiti. La misura è quindi distorta **verso l'alto**, cioè verso `FasterThanBand`, che è **il lato su cui il light client fallisce chiuso**. E la distorsione non viene dalla catena: viene dal processo di rilascio, cioè dalla stessa parte che il progetto tratta come il proprio orologio esterno fidato.

**Quantificazione esatta**, con `I = block_interval_ms`, `m = min_ms_per_block`, `d` il tempo trascorso dall'emissione del checkpoint, su una catena **onesta che produce esattamente a `I`**:

```text
blocchi  = (d + L) / I          tempo misurato = d
veloce  <=>  d < blocchi * m  <=>  d * (I - m) < L * m  <=>  d < L * m/(I - m)
```

La finestra di falso positivo dura `L · m/(I−m)`. Con la banda della trascrizione di `GATE-MEASURE-BINDS` (`I = 5000`, `m = 2500`) vale esattamente `L`.

E la misura è fatta appena `blocchi >= min_measured_blocks`, cioè a `d = min_measured_blocks·I − L`. Sostituendo: **il primo verdetto che il client emette è `FasterThanBand` ogni volta che**

```text
L > min_measured_blocks * min_ms_per_block
```

Con i numeri della trascrizione: `100 × 2500 = 250 000 ms`. **Una latenza di rilascio superiore a quattro minuti e dieci secondi manda in fallimento chiuso, su una catena perfettamente onesta e con un checkpoint perfettamente onesto, ogni light client al primo verdetto che riesce a formulare.** Il processo di rilascio tiene una chiave che «non appartiene a nessun validatore», fuori banda per costruzione, plausibilmente con un intervento umano o un HSM: quattro minuti sono una latenza ordinaria, non un caso patologico.

**Riproduzione.** L'aritmetica è quella di `measure()` (`cadence.rs:116-147`) e si riproduce senza costruire nulla; l'ho eseguita trascrivendo la funzione:

```text
band: I=5000, m=2500, M=10000, min_measured_blocks=100
catena onesta a esattamente 5000 ms/blocco; checkpoint su height 0 firmato L ms dopo
d = primo istante in cui la misura è fatta

L=  60 000  d= 440 000  ->  WithinBand
L= 250 000  d= 250 000  ->  WithinBand      <- la soglia, esatta
L= 300 000  d= 200 000  ->  FasterThanBand  <- light client: fail closed
L= 600 000  d=       0  ->  FasterThanBand
L=3 600 000 d=       0  ->  FasterThanBand
```

In forma di test sull'API consegnata, da aggiungere a `core/coblox-core/tests/cadence_and_reward_epoch.rs`:

```rust
// catena onesta a 5 000 ms/blocco; il checkpoint su height 0 è firmato a t = 600 000
// (dieci minuti di latenza di rilascio); il client misura a t = 900 000, tip = 180.
let v = measure_cadence_from_checkpoint(&chain(), 0, 600_000, 180, 900_000, &band()).unwrap();
assert!(matches!(v, CadenceVerdict::FasterThanBand { .. }));   // passa oggi
assert!(check_cadence_light_client(v).is_err());               // e il client fallisce chiuso
```

180 blocchi × 2500 = 450 000 > 300 000 ms misurati: `FasterThanBand`. La catena non ha fatto nulla.

### Il secondo attacco: l'orologio del client, e quello del rilascio

`now_ms` è l'orologio del client. Il commento lo giustifica con *«lo stesso che il passo 1 già usa per la freschezza»*, e la giustificazione non trasferisce: al passo 1 un orologio **indietro** fa sembrare un checkpoint più fresco di quanto sia — un errore che il passo 1 tratta come un rischio accettato — mentre al passo 4b un orologio indietro **accorcia il denominatore**, cioè spinge verso `FasterThanBand`, cioè verso il fallimento chiuso. Un client con l'orologio indietro di `S` ha una finestra di falso positivo di `S·m/(I−m)` esattamente come sopra. Un avversario che possa influenzare l'ora del dispositivo (NTP in chiaro, captive portal) sceglie il verso che chiude il client. Simmetricamente, un orologio del **processo di rilascio** avanti di `S` post-data `issued_at_ms` e produce lo stesso effetto **su tutti i client contemporaneamente**: l'unico orologio esterno del protocollo diventa un punto singolo di guasto con semantica fail-closed.

### Il terzo: la scelta di quale checkpoint servire

I checkpoint sono artefatti di rilascio distribuiti fuori banda; nulla obbliga il client a possedere il **più vecchio**. Poiché la distorsione decresce con `d`, **servire il checkpoint più recente e autentico è la mossa che massimizza la distorsione verso il veloce**. Un peer, un mirror, un aggiornamento d'app che consegna materiale interamente genuino e firmato — checkpoint valido, intestazioni valide, quorum valido — mette un client onesto in fallimento chiuso e nega la verifica del saldo. Non serve alcuna chiave.

### Conclusione, e la parte in cui **non** rompo il lavoro

**Non concludo che il lato che fallisce chiuso sia sbagliato, e va detto con precisione, perché è la conclusione che l'attacco sembra suggerire e non è quella giusta.** Invertire l'asimmetria sarebbe peggio: sul lato lento la distorsione del numeratore è reale, non ha soglia, e produrrebbe falsi positivi ancora più comuni. E il fallimento chiuso sul lato veloce protegge la cosa giusta, che dopo [DEBT-019] è l'emissione.

Ciò che l'attacco rompe è **l'argomento**, e con esso la forma della guardia:

1. la frase *«distorta verso il basso e solo verso il basso»* è **falsa** e va corretta in `cadence.rs:170-175`, in `README.md#cadence-band`, in `ledger.md` passo 4b e in `SECURITY.md`. La frase vera è: *il conteggio dei blocchi è distorto verso il basso; il tempo misurato è distorto verso il basso a sua volta, per la latenza di rilascio e per l'errore d'orologio, e le due distorsioni spingono il rapporto in versi opposti*;
2. il fallimento chiuso sul lato veloce ha bisogno di una **tolleranza dichiarata**, altrimenti è una guardia che grida al lupo sul percorso ordinario — che è precisamente il modo in cui [ADR-012] precisazione 3 registra la fine di una guardia, citato dal codice stesso a due righe di distanza dal difetto;
3. `min_measured_blocks` è un pavimento sul **numeratore** e non ne esiste alcuno sul **denominatore**. Il fatto che manchi metà del pavimento è la famiglia 3 in miniatura: è vincolata la grandezza nominata (i blocchi) e non quella da cui la proprietà dipende (l'intervallo su cui il rapporto è significativo).

Il rimedio è in RF-001.

---

## Domanda 3 — La correzione del Lead in [DEBT-013], e quale movente è più forte

### Confermo la correzione, senza riserve

La frase barrata — *«la direzione del pericolo è verso il rallentamento, non verso l'accelerazione: blocchi più veloci accorciano tutto e favoriscono il ricambio»* — è **falsa** e non incompleta, ed è falsa nel senso preciso in cui il Lead lo scrive: non era falsa quando l'ho scritta; è il rimedio ad aver cambiato il fatto. Verificato in albero e non letto dall'evidenza:

- `cadence.rs:332` `check_mint_reward_epoch` impone `(e+1) * reward_epoch_blocks <= h`;
- `reward_epoch_blocks = reward_epoch_ms.div_ceil(block_interval_ms)`, con `block_interval_ms` costante di genesi e `reward_epoch_ms` sotto il pavimento `reward_epoch_ms_min` di `RewardBounds` (`params.rs:189`);
- quindi il numero di epoche liquidabili è funzione di `height`, e comprimere la produzione moltiplica l'emissione **per unità di tempo reale**.

Accetto anche la **condizione** che il Lead ha allegato alla correzione: è scritta contro un'implementazione in review, e se la derivazione cambiasse va rivista e non conservata. È la disciplina giusta, ed è la ragione per cui l'annotazione su [ADR-013] è rinviata.

Registro inoltre che questa è la forma di errore che nessuna gate cerca — **una chiusura che falsifica la descrizione del problema che chiudeva** — e che nel giro di questa stessa spec si è presentata **due volte**: qui, e in RF-001, dove il rimedio ha introdotto un argomento che il rimedio stesso rende falso. Vale la pena registrarlo come tratto della classe e non come coincidenza.

### Contesto invece la gerarchia dei moventi

Il Lead scrive in [DEBT-013] che *«sul lato veloce [il movente] è diretto»*, e ne fa un argomento a sostegno della scelta di fallire chiusi lì. **La prima metà è vera con una condizione, la seconda no. Il movente sul lato veloce è più debole, non più forte, e per quattro ragioni indipendenti.**

**1. La soglia è diversa, ed è la differenza che conta.** Rallentare costa un **terzo bloccante**: basta negare il quorum. Accelerare costa un **quorum**, perché `ledger.md` regola 7 impone che ogni blocco porti un certificato di quorum del set attivo, e il predicato è `signed_power * 3 > total_power * 2`. Nessun terzo bloccante può far esistere più blocchi. **Il lato su cui il light client fallisce chiuso costa più del doppio, in potere di voto, del lato su cui si limita a segnalare.** Questa differenza non è scritta in nessun documento, e `SECURITY.md` la cancella attivamente: è RF-002.

**2. Il guadagno non è esclusivo.** L'emissione di esistenza è `F / E` per nodo idoneo (`ledger.md#existence-income-is-a-share-of-a-capped-fund`). Accelerare raddoppia le epoche al giorno **per tutti**: il cartello che accelera guadagna in proporzione alla propria quota di nodi idonei, non alla propria quota di potere di voto. Il guadagno *relativo* di chi accelera è **zero**, salvo che tenga già una quota sproporzionata di identità idonee — cioè **salvo che sia già una flotta**. Il movente diretto sul lato veloce è quindi un movente **condizionato al Sybil**, e questo lega [DEBT-013] alla superficie che [ADR-007] dichiara aperta. È l'osservazione più utile che esce da questa domanda, e vincola la risposta alla domanda 5.

**3. Non c'è negabilità.** Una catena lenta e una rete partizionata producono la stessa lettura, e i documenti lo dicono bene. Una catena **veloce** non ha alcuna spiegazione onesta: nessun guasto accelera la produzione di blocchi. Il lato veloce è quindi il lato **su cui l'osservazione è già conclusiva senza banda**, ed è osservabile da qualunque nodo completo senza checkpoint, senza `CadenceBand` e senza il passo 4b.

**4. Il costo è esternalizzato nei due versi, non solo nel lento.** Sul lato lento si perde l'emissione di tutta la rete e si conserva il seggio del solo cartello — la mia valutazione originale, che resta valida. Sul lato veloce si diluisce la scarsità interna di tutta la rete e si conserva un vantaggio pro quota. In entrambi i casi il beneficio esclusivo è il seggio, e il seggio si compra **rallentando**, al terzo del prezzo.

**Conclusione della domanda 3.** La banda a due lati è corretta e necessaria: entrambe le direzioni sono ora dannose, e le due conseguenze non si scambiano fra loro. Ma il movente **dominante resta il rallentamento**, e il progetto ha costruito una guardia che fallisce chiusa sul lato debole e si limita a segnalare sul lato forte. È difendibile — l'argomento di attribuzione è l'unico che lo giustifichi, ed è il motivo per cui, nonostante RF-001, non chiedo di invertirla. Ma ne discende una conseguenza operativa: **il segnale del lato lento è la metà che porta il rischio maggiore, ed è la metà che nulla trattiene.** È RF-006.

---

## Domanda 4 — Il rifiuto di [DEBT-014] è motivato correttamente?

**L'argomento falso non compare, e la conclusione è giusta. L'argomento portante scelto è però il più debole dei due disponibili, ed è enunciato in una forma che è di nuovo troppo larga.**

### Ciò che regge

Verificato che *«è una lista di chiavi, legarla impedirebbe di riusarla in una genesi nuova»* non compare in `README.md`, `ledger.md`, `registry.rs`, `hash.rs`. Correttamente: sarebbe un precedente, e la ragione per cui è falsa — ogni `key_binding_signature` andrebbe riemessa comunque — è quella giusta.

La correzione del superlativo è esatta e l'ho riverificata per esaurimento. Le sei preimmagini che omettono `chain_id` per ragioni proprie sono sei e non sette: `account_key` è **una** preimmagine con due derivazioni sotto lo stesso dominio (`ledger.md:2329-2330`), e il documento scrive «the `account_key` derivations» al plurale contandola una volta. Il conto torna.

Il legame per byte esiste dove il documento dice che esiste: `key_binding_signature` è presa sulla procedura globale legata alla catena, e i **byte della firma** stanno dentro `JCS(ValidatorSet)`, quindi dentro `validator_set_hash`. Due catene con gli stessi validatori producono firme diverse e quindi hash diversi. È vero.

### La pronuncia separata sulle tre superfici, che i criteri di risoluzione del debito chiedono e che nessun documento fa

[DEBT-014] chiede una valutazione che si pronunci **separatamente** su certificati di quorum, checkpoint di soggettività debole e transizioni di set. La consegna asserisce che «regge su tutte e tre le superfici» ma non le distingue, e il paragrafo pubblicato non le nomina. La pronuncia è mia e la faccio qui:

- **Certificati di quorum.** Reggono, e **non** grazie al legame per byte del set: reggono perché le firme del certificato sono prese su `"coblox-block-vote-v0\0" || chain_id_32 || …` (`ledger.md:673-674`). Un certificato replicato su un'altra catena fallisce sulla verifica della firma prima ancora che il `validator_set_hash` sia rilevante.
- **Checkpoint di soggettività debole.** Reggono, e di nuovo per una ragione propria: la preimmagine del checkpoint porta `chain_id_32`, e il client rifiuta un checkpoint il cui `chain_id` non è quello configurato. Il `validator_set_hash` che il checkpoint trasporta è un campo dentro un oggetto già legato alla catena.
- **Transizioni di set.** Reggono perché `next_validator_set_hash` è un campo dell'intestazione e `block_id` porta `chain_id_32`; una transizione non è mai osservata fuori da un'intestazione autenticata.

**In tutti e tre i casi la protezione è dell'oggetto che nomina il set, non del set.** Questo è l'argomento completo, e il documento lo enuncia in una subordinata — *«every object that names a set by hash is one whose contents differ between chains»* — mentre mette in evidenza quello per byte.

### Perché l'ordine dei due argomenti va invertito: il set di genesi

L'argomento per byte ha **un caso in cui si riduce a un solo legame**, e il documento lo nomina senza trarne la conseguenza: *«il set di genesi, l'unico senza `election`, porta comunque i key binding»*. Per il set di genesi cadono due dei tre legami, e resta il `key_binding_signature`, che lega la catena attraverso `chain_id`.

Ma `chain_id` alla genesi è oggetto di **[DEBT-020], aperto**: `chain_id = H(dominio || len(network_id) || network_id || genesis_block_id)` e `genesis_block_id = H(dominio || chain_id || JCS(header))`, con l'intestazione di genesi che contiene il `validator_set_hash` del set di genesi. La circolarità non è risolta da alcuna regola, e finché non lo è **non è dimostrato quale `chain_id` i key binding di genesi leghino**. L'argomento per byte, sul set di genesi, poggia quindi su una grandezza che un debito aperto dichiara indeterminata.

Non è un difetto sfruttabile oggi — le tre superfici reggono comunque, per la ragione dell'oggetto che nomina — ma è la ragione per cui **l'ordine degli argomenti nel documento è sbagliato**: il documento mette avanti quello che ha un caso scoperto e in coda quello che è completo. È RF-003, seconda parte.

### La classe enunciata è di nuovo più larga del vero

E questa è la parte che va corretta comunque. Il documento scrive:

> What `validator_set_hash` is the exception to is the narrower class it belongs to: **a preimage over a chain-specific consensus object that other consensus objects reference by hash.**

`ledger.md` ripete la stessa formula. **In quella forma la classe è falsa, e i controesempi sono numerosi**: `node_leaf` (`0x10`), `app_leaf` (`0x13`), `subscription_leaf` (`0x20`), `eligible_leaf` (`0x24`), `revocation_leaf` (`0x30`), `candidate_leaf` (`0x40`) e i rispettivi nodi interni sono preimmagini su oggetti di consenso specifici della catena, nominate per hash da altri oggetti di consenso (`state_root` nell'intestazione, `eligible_set_root` in un mint, `revocation_root` nel checkpoint), e **nessuna porta `chain_id`**. La parola che manca è quella del titolo del debito: **a dominio separato**. Le preimmagini ad albero sono separate per tag byte, non per dominio, e la loro classe ha un'esenzione propria — ereditano il legame dall'oggetto che nomina la radice, che è esattamente l'argomento che qui va promosso a portante.

È **famiglia 2 dentro il paragrafo scritto per correggere un superlativo falso**: un superlativo sostituito da un altro superlativo, più difendibile ma ancora non vero. Il costo della correzione è due parole. È RF-003.

---

## Domanda 5 — `SECURITY.md`

### (a) Il paragrafo anti-Sybil: sì, va rafforzato, e la condizione è soddisfatta

**Ho verificato in albero che le tre disposizioni di [SPEC-009] ci sono tutte e tre**, non l'ho letto dall'evidenza:

1. `RewardBounds` è nell'ancora di genesi con `existence_fund_microtokens_per_epoch_max` (`params.rs:181-216`), e `RewardBounds::validate` è chiamata (il difetto di [REVIEW-017] RF-001 è chiuso);
2. `availability_microtokens_per_unit != 0` è **rifiutato in accettazione** come regola di validità (`params.rs:807-810`, regola nominata `"availability_microtokens_per_unit == 0"`);
3. `3 * validator_min_set_size >= 2 * V` è nel blocco dei vincoli (`params.rs:463-469`) e verificato da `reward_rules.py`.

La condizione che [ADR-010] pone è quindi soddisfatta, e l'affermazione «una flotta non aumenta l'emissione totale» è passata da **vera-a-condizione** a **vera-per-regola**. L'implementatrice ha avuto ragione a non toccarla da sola e ha avuto ragione a segnalarla: rafforzare una pretesa di sicurezza è lavoro di chi ha mandato di attaccarla.

**Ma la formulazione ovvia sarebbe un errore, e la ragione viene dalla domanda 3.** La proprietà è per-regola **per epoca di ricompensa**, non per unità di tempo reale: `SPEC-016` ha appena stabilito che l'indice di epoca è ritmato da `height`, quindi un quorum che comprime la cadenza **aumenta l'emissione totale per unità di tempo reale** senza violare nulla. Scrivere «una flotta non aumenta l'emissione totale» senza il qualificatore per-epoca creerebbe, nello stesso file, una pretesa che il paragrafo tre righe più sotto smentisce. Sarebbe la famiglia 2 commessa **rafforzando** invece che dimenticando, che è la variante che nessuno cerca.

**Formulazione esatta che propongo**, in sostituzione delle due frasi che oggi dicono *«Sybil resistance is treated as an economic property governed by the fraction of emission that flows through the existence income, not as a cryptographic guarantee»*:

> Sybil resistance is treated as an economic property, not as a cryptographic
> guarantee, and since [SPEC-009] one half of it is held by a rule rather than by
> a well-chosen value. **A fleet of `N` emulated nodes cannot enlarge what the
> network pays out in a reward epoch.** Existence income is a fund divided among
> eligible nodes, not an amount per node; the fund has a ceiling fixed in the
> genesis trust anchor and outside on-chain governance (`RewardBounds`); the one
> channel that would have paid per node without an aggregate ceiling is required
> by a validity rule to be zero, so a policy document that sets it positive is
> rejected on acceptance rather than discouraged. What a fleet buys is a larger
> **share** of a fixed fund — dilution of honest nodes, not inflation.
>
> **This bound is per reward epoch, and not per unit of real time.** The epoch
> index is paced by block height, so a validator quorum that compresses the real
> cadence multiplies real issuance whatever the fleet does; see *How fast the
> chain runs is measured, not enforced* below. The two limitations are adjacent
> and only one of them is held by a rule.

Il secondo paragrafo è la parte che rende il rafforzamento **onesto**, ed è la ragione per cui la risposta a questa domanda non poteva essere data senza la domanda 3.

Va conservata invariata l'enumerazione delle tre cose *specifically not guaranteed*: sono ancora tutte e tre vere, compresa la terza — `ledger.md#what-a-light-client-can-establish-about-set-composition` mantiene le otto voci di `CANNOT_ESTABLISH`, e il passo 4b vi si accosta senza entrarci, come l'implementatrice ha correttamente osservato. La sua deviazione 5 è **corretta** e la confermo.

### (b) Il resto del file, guardato con l'occhio che nessuna probe sostituisce

**Ho trovato una cosa detta più forte del vero, ed è la più importante del file dopo l'anti-Sybil.**

> A blocking third can therefore move the real production rate **in either direction** while the chain stays live and every block stays valid.

**Falso sul lato veloce.** Un terzo bloccante nega il quorum e rallenta; non può far esistere un blocco, perché ogni blocco richiede un certificato di quorum con `signed_power * 3 > total_power * 2`. Accelerare richiede il quorum. È RF-002, e non è un cavillo: la frase cancella l'unica asimmetria di **costo** fra le due direzioni proprio nel documento in cui un ricercatore esterno va a cercare quanto costa un attacco, e proprio nella direzione su cui il light client fallisce chiuso. Né `README.md` né `ledger.md` commettono l'errore: entrambi parlano di «a set of validators», senza soglia. È localizzato al file riscritto nella remediation.

**Le altre due imprecisioni che avevo cercato non ci sono più**, e la loro chiusura è migliore di quanto chiesto: i tre conteggi del threat model sono ora **ricalcolati dalla fonte** e non pinnati, il che è la forma di [SPEC-012] applicata correttamente. Ho verificato che il meccanismo mordeva: il file oggi dichiara **43** scenari, cioè un valore che nessuno ha trascritto ma che la gate impone. Questa parte è ineccepibile.

**L'osservazione (b) dell'implementatrice — «Milestone M-01 covers the protocol on paper» mentre il lavoro è in M-02 — non è un finding di sicurezza e va lasciata a lei.** È una frase di **perimetro**, non di stato: dice cosa la milestone copriva, e ciò che segue («no production network, no public devnet, no released binary», «nothing here is deployed anywhere that a vulnerability could currently harm a user») è ancora vero. Sottostima l'avanzamento, non sovrastima la sicurezza, e la direzione dell'errore è quella innocua. Correggerla è igiene redazionale, non materia di questa gate: **la giudico non-finding e la lascio dichiarata come tale**, perché una segnalazione lasciata senza risposta si ripresenta.

---

## Domanda 6 — Le sette probe pinnano le cose giuste?

**Cinque su sette sì. Le due che mancano sono le due che il finding originale avrebbe dovuto insegnare a proteggere, e l'ho verificato eseguendo, non leggendo.**

La prova in negativo `prove_every_probe` è un lavoro eccellente e chiude una lacuna reale — dimostra che ciascuna delle 98 probe è *raggiungibile e può fallire*. Ma per costruzione **non può dire se una probe pinna la frase giusta**: cancella il passaggio che la probe stessa dichiara di pinnare, quindi una probe che pinna una frase vera ma non portante supera la prova esattamente come una che pinna quella portante. La domanda del Lead è esattamente questa, e la risposta si ottiene solo cancellando **le frasi che nessuna probe nomina**.

Le ho cancellate. Trascrizione, su copia dell'albero fuori dal repository:

```text
--- controllo, copia non mutata ---
$ COBLOX_REPO=<copia> python sim/tools/published_artifacts.py
  C10-PROBE  98 candidate(s) checked
  C11-CLAIMDOC  4 candidate(s) checked
published-artifact inventory: PASS

--- mutazione 1: cancellata da SECURITY.md la frase della direzione LENTA ---
"**Stretching** lengthens, in real time, everything the protocol denominates in
 blocks: validator incumbency, and the effective delay of a revocation."
$ COBLOX_REPO=<copia> python sim/tools/published_artifacts.py
published-artifact inventory: PASS        <-- VERDE

--- mutazione 2: cancellate le tre cose "specifically not guaranteed" ---
$ COBLOX_REPO=<copia> python sim/tools/published_artifacts.py
published-artifact inventory: PASS        <-- VERDE
```

La mutazione 1 è **RF-001 di [REVIEW-025] riprodotto nell'altro verso**: il paragrafo torna a nominare una direzione sola, la gate resta verde, e il documento che un ricercatore esterno legge per primo torna a essere incompleto. Il rimedio ha pinnato la direzione che ha **aggiunto** e non quella che c'era: la simmetria che il paragrafo ha guadagnato non è protetta.

È RF-004, e vale la pena dire perché è successo: le sette probe sono state scritte guardando la *modifica*, non la *proprietà*. È la stessa asimmetria di attenzione che RF-001 di [REVIEW-025] censura un livello sopra.

**Le altre cinque probe sono ben scelte.** In particolare `security-cadence-not-prevented` pinna la frase giusta — quella che impedisce al paragrafo di promettere più del codice — e `security-quorum-four-ninths` più `security-owning-is-not-controlling` pinnano insieme il numero **e** la distinzione su cui il numero poggia, che è la forma corretta: pinnare un numero senza la sua definizione lo lascia vero e incontrollabile.

**Una nota di forma, non un finding a carico di questa spec.** Tre delle sette probe codificano un a-capo (`\*\*four\nninths\*\*`), quindi falliscono su un semplice riflusso del paragrafo, a testo immutato:

```text
--- mutazione 3: paragrafo ri-avvolto, nessuna parola cambiata ---
FAIL C10-PROBE: probe 'security-quorum-four-ninths' expected 1 match(es) ... found 0
```

È un falso positivo, cioè la forma che [ADR-012] precisazione 3 registra come inizio della fine di una guardia. **Non lo imputo a [SPEC-016]**: 99 pattern su 101 nel manifesto hanno la stessa forma, ed è una convenzione preesistente del progetto. La segnalo perché `SECURITY.md` è prosa destinata a lettori esterni, cioè il documento con la probabilità più alta di essere ri-avvolto da un editor. Un `\s+` al posto di `\n` costa nulla ed è la correzione giusta ovunque, quando qualcuno vi passerà.

---

## Code observations

**L'aritmetica esatta in `u128` è corretta ed è la scelta giusta**, e il test `the_band_comparison_does_not_divide_first` è costruito bene: prova il caso in cui l'implementazione che divide **concorderebbe per caso** (`249_999 / 100 == 2_499`) accanto a quello in cui divergerebbe (`1_000_099 / 100 == 10_000`). Provare il vicino che concorda è ciò che rende il test differenziale, ed è raro vederlo fatto.

**`CadenceVerdict::Inconclusive` come esito proprio e mai un pass** è corretto e chiude una via silenziosa: `min_measured_blocks = 0` sarebbe stato un `WithinBand` su un blocco.

**La validazione dell'ancora come primo atto** chiude il difetto di [REVIEW-017] RF-001 prima che si ripresenti, e la regola relazionale `min_ms_per_block <= block_interval_ms <= max_ms_per_block` è la regola che dà all'oggetto il significato del proprio nome. Entrambe non erano chieste.

**`the_cadence_module_never_reads_a_chain_written_clock`** trasforma un divieto in prosa in un oracolo che si esercita, ed è il rimedio alla famiglia 4 applicato preventivamente. La sua forma — leggere il sorgente, togliere i commenti, cercare il nome — è riusabile.

**Il rifiuto del punto 3 è corretto** e concordo con [REVIEW-025]: un limite di mandato in millisecondi di catena si evade scrivendo incrementi di timestamp piccoli a piacere, ed è la famiglia 3 commessa dentro il rimedio che esiste per non commetterla. Non ho nulla da aggiungere se non che la spec era mia nell'ordine dei punti, e il punto 3 era mio: **è il quarto caso in cui un agente ha ragione contro chi ha scritto la spec, e questa volta contro di me.**

**`check_mint_reward_epoch` non ha chiamanti di produzione.** L'ho verificato (`grep` sull'albero: solo il test d'integrazione). **Non è un finding**: in `coblox-core` non esiste ancora alcun percorso di validazione delle transazioni — non c'è un `tx.rs` — e `existence_income_share` in `params.rs` ha esattamente la stessa forma. Lo registro perché la regola è dichiarata **di validità** in `ledger.md` (*«A block containing a mint that violates this is invalid»*) e la sua sede di applicazione non esiste ancora: è il tipo di riga che si dimentica quando quella sede verrà scritta, e appartiene alla spec che la scriverà.

---

## Tests and verification

Le quattro gate `before-submit` sono soddisfatte e tutte e quattro provate in negativo; non le rifaccio. `GATE-NO-TIMESTAMP-RULE` è la più forte delle quattro e la sua chiusura è migliore della richiesta.

Ciò che nessuna delle quattro copre, e che questa review aggiunge:

- **nessuna gate prova la banda contro una catena onesta con una latenza di rilascio realistica.** `GATE-MEASURE-BINDS` prova tre catene — dentro, veloce, lenta — tutte con `checkpoint_issued_at_ms = 0`, cioè con latenza di rilascio **zero**. Il caso che rompe la guardia è quello che la trascrizione non contiene: la catena **onesta** con un checkpoint **onesto ma non istantaneo**. È RF-001, e la condizione di chiusura è che quel caso entri nella trascrizione;
- **la prova in negativo non può pronunciarsi sull'adeguatezza di una probe**, solo sulla sua raggiungibilità. È RF-004;
- **`C11-CLAIMDOC` non ha un lato «disco».** È RF-005.

---

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

- **RF-001 | category=security-boundary | severity=high | criterion=«Un light client che detiene un checkpoint e un'intestazione fidata misura la cadenza reale e si comporta come la regola dichiara fuori banda»**

  **L'argomento che giustifica il fallimento chiuso sul lato veloce è falso, e il fallimento chiuso scatta su una catena onesta.** La distorsione «verso il basso e solo verso il basso» vale per il **numeratore** della misura. Il **denominatore** ha una distorsione propria e opposta: `issued_at_ms` è, per definizione del documento, il momento in cui il checkpoint è stato *prodotto*, non quello in cui l'altezza che nomina è stata raggiunta, quindi i blocchi prodotti durante la latenza di rilascio `L` sono **contati senza il loro tempo**. La lettura è spinta verso `FasterThanBand`, che è il lato su cui il client fallisce chiuso, e la spinta non viene dalla catena.

  Soglia esatta, su catena che produce a esattamente `block_interval_ms`: il primo verdetto che il client riesce a formulare è `FasterThanBand` ogni volta che `L > min_measured_blocks * min_ms_per_block`, e la finestra di falso positivo dura `L * m/(I-m)`. Con la banda della trascrizione: **quattro minuti e dieci secondi di latenza di rilascio**. Il processo di rilascio tiene una chiave fuori banda, che non appartiene a nessun validatore: quattro minuti sono la sua condizione normale, non un guasto.

  Due varianti della stessa causa, entrambe reali: un orologio del client **indietro** di `S`, o un orologio del **processo di rilascio** avanti di `S`, producono lo stesso fallimento chiuso — il secondo su tutti i client contemporaneamente, il che rende l'unico orologio esterno del protocollo un punto singolo di guasto con semantica fail-closed. E poiché la distorsione decresce con l'età del checkpoint, **servire il checkpoint più recente e genuino è la mossa che massimizza la distorsione**: un peer che consegna materiale interamente autentico e firmato nega la verifica del saldo a un client onesto, senza possedere alcuna chiave.

  `min_measured_blocks` è un pavimento sul numeratore; **non esiste alcun pavimento sul denominatore**, ed è la metà mancante.

  **Riproduzione.** Aritmetica di `measure()`, catena onesta a 5 000 ms/blocco, banda `2500..=10000`, `min_measured_blocks = 100`:

  ```text
  L= 250 000 ms -> WithinBand      (la soglia)
  L= 300 000 ms -> FasterThanBand  -> check_cadence_light_client = Err
  L= 600 000 ms -> FasterThanBand
  ```

  In forma di test sull'API consegnata, da aggiungere a `tests/cadence_and_reward_epoch.rs`:
  `measure_cadence_from_checkpoint(&chain(), 0, 600_000, 180, 900_000, &band())` → `FasterThanBand`, e `check_cadence_light_client(...)` → `Err`. La catena non ha fatto nulla.

  **Rimedio, in tre parti e tutte e tre necessarie:**

  1. **la frase va corretta** in `cadence.rs:170-175`, `README.md#cadence-band`, `ledger.md` passo 4b e `SECURITY.md`. La forma vera è che *entrambi* gli estremi sono distorti verso il basso, e che le due distorsioni spingono il **rapporto** in versi opposti;
  2. **il lato veloce ha bisogno di una tolleranza dichiarata.** La forma che raccomando è una grandezza di genesi in `CadenceBand` — `max_release_latency_ms`, o equivalentemente un pavimento `min_measured_ms` sul denominatore — con il confronto veloce che diventa `elapsed_ms + max_release_latency_ms < blocks * min_ms_per_block`. È un'ancora di genesi come il resto della banda, quindi fuori dalla governance on-chain, e il suo valore è una decisione dell'operatore come gli altri tre: va **istruito**, non scelto. La regola relazionale che ne deriva va aggiunta a `CadenceBand::validate`;
  3. **il caso onesto-con-latenza entra nella trascrizione di `GATE-MEASURE-BINDS`.** Oggi la gate prova tre catene tutte con `issued_at_ms = 0`. Una guardia provata solo a latenza zero è provata sul caso che non esiste.

  **Condizione di chiusura verificabile:** la stessa costruzione — catena onesta a `block_interval_ms`, checkpoint firmato `L` ms dopo l'altezza che nomina, con `L` superiore alla soglia — produce `WithinBand` e non `FasterThanBand`, e la trascrizione mostra il caso. In alternativa, se l'operatore decidesse che la latenza di rilascio va vincolata invece che tollerata, la condizione è che il vincolo sia scritto come parte della procedura di rilascio in `README.md#weak-subjectivity-checkpoint` **e** che la misura del client resti corretta quando la procedura lo viola, perché una procedura non è una regola — che è la distinzione su cui tutta questa spec poggia.

- **RF-002 | category=documentation | severity=medium | criterion=«Nessun artefatto pubblicato dice o lascia intendere più del vero»**

  `SECURITY.md`, §*Known limitations*: *«A blocking third can therefore move the real production rate in either direction while the chain stays live and every block stays valid.»* **Il lato veloce non è alla portata di un terzo bloccante.** Ogni blocco richiede un certificato di quorum del set attivo (`ledger.md`, regola 7) sotto il predicato `signed_power * 3 > total_power * 2`: un terzo bloccante può negare il quorum e quindi rallentare, non può far esistere un blocco. Accelerare richiede più di due terzi del potere di voto.

  Non è un cavillo. La differenza di soglia — **un terzo per rallentare, un quorum per accelerare** — è il fatto che decide dove valga la pena fallire chiusi, ed è cancellato proprio nel documento in cui un ricercatore esterno cerca quanto costa un attacco. Interagisce direttamente con RF-001: la guardia fallisce chiusa sul lato che costa il doppio.

  **Riproduzione:** `grep -n "blocking third" SECURITY.md`; confrontare con `ledger.md` regola 7 e con il predicato di quorum. `README.md` e `ledger.md` non commettono l'errore — entrambi dicono «a set of validators» senza soglia — quindi la correzione è localizzata a un file.

  **Rimedio:** riscrivere la frase distinguendo le due soglie, per esempio: *«A blocking third can stretch the real production rate; compressing it requires a quorum, since every block carries a quorum certificate. The two directions therefore do not cost the same, and the cheaper one is the one this protocol only reports on.»* L'ultima subordinata è l'informazione che oggi manca e che RF-006 rende operativa.

  **Condizione di chiusura:** la frase distingue le due soglie e una probe la pinna.

- **RF-003 | category=documentation | severity=medium | criterion=«I due paragrafi di [DEBT-014] sono scritti, e l'argomento falso non compare»**

  Due difetti nello stesso paragrafo, e nessuno dei due è l'argomento falso rimosso.

  **La classe è ancora enunciata più larga del vero.** *«a preimage over a chain-specific consensus object that other consensus objects reference by hash»* — in `README.md#hash-preimage-registry` e ripetuta in `ledger.md#validator-set-continuity` — **ha controesempi**: `node_leaf` (`0x10`), `app_leaf` (`0x13`), `subscription_leaf` (`0x20`), `eligible_leaf` (`0x24`), `revocation_leaf` (`0x30`), `candidate_leaf` (`0x40`) e i nodi interni corrispondenti sono preimmagini su oggetti di consenso specifici della catena, nominate per hash da `state_root`, `eligible_set_root` e `revocation_root`, e nessuna porta `chain_id`. La parola mancante è quella del titolo del debito: **a dominio separato**. È famiglia 2 dentro il paragrafo scritto per correggere un superlativo falso.

  **L'ordine dei due argomenti è invertito rispetto alla loro forza.** Il documento mette avanti il legame per byte e in subordinata il legame dell'oggetto che nomina. Il primo ha un caso scoperto: sul **set di genesi** cadono `election_seed` ed `election_ticket`, resta il solo `key_binding_signature`, che lega attraverso `chain_id` — grandezza che **[DEBT-020], aperto**, dichiara circolare e non risolta da alcuna regola alla genesi. Il secondo argomento è completo su tutte e tre le superfici e non ha eccezioni.

  **Riproduzione:** `grep -n "H(0x24\|H(0x30\|H(0x40" docs/protocol/ledger.md` e verificare che nessuna di quelle preimmagini porti `chain_id`, contro la classe come è scritta; per la seconda parte, leggere `README.md` righe 73-75 accanto alla formula di `block_id` e confrontarle con [DEBT-020].

  **Rimedio:** (a) inserire «domain-separated» nella definizione della classe, nei due documenti, e una riga che dica perché le preimmagini ad albero sono fuori classe (ereditano il legame dall'oggetto che nomina la radice); (b) invertire l'ordine, portando avanti l'argomento dell'oggetto che nomina e dichiarando esplicitamente che è **quello** a coprire il set di genesi finché [DEBT-020] è aperto; (c) aggiungere la pronuncia separata sulle tre superfici che i criteri di risoluzione di [DEBT-014] chiedono — certificati di quorum, checkpoint, transizioni — che questa review fornisce nella sezione della domanda 4 e che il documento può riassumere in tre righe.

  **Condizione di chiusura:** la classe con «domain-separated» regge a una ricerca per esaurimento sulle preimmagini a tag byte, e il paragrafo nomina le tre superfici.

- **RF-004 | category=verification-gap | severity=medium | criterion=«Le probe pinnano i passaggi normativi portanti»**

  **Le sette probe su `SECURITY.md` proteggono una direzione sola del paragrafo che il finding aveva censurato per averne nominata una sola.** Cancellando da `SECURITY.md` la frase *«**Stretching** lengthens, in real time, everything the protocol denominates in blocks: validator incumbency, and the effective delay of a revocation»*, la passata resta **verde**: il documento torna a nominare una direzione e nulla fallisce. Stesso esito cancellando l'enumerazione *«Three things are specifically not guaranteed: enrollment availability under sustained attack, cryptographic Sybil resistance, and independent verification of validator eligibility»*, che è il periodo che impedisce al paragrafo anti-Sybil di leggersi come un'ammissione generica.

  `prove_every_probe` non può trovarlo per costruzione: cancella il passaggio che ciascuna probe **dichiara** di pinnare, quindi non distingue una probe che pinna la frase portante da una che pinna una frase vera e sostituibile.

  **Riproduzione (eseguita, su copia dell'albero fuori dal repository):**

  ```text
  $ COBLOX_REPO=<copia> python sim/tools/published_artifacts.py     # controllo
  published-artifact inventory: PASS   (98 C10, 4 C11)
  # cancellata la frase "**Stretching** lengthens, ..." da SECURITY.md
  $ COBLOX_REPO=<copia> python sim/tools/published_artifacts.py
  published-artifact inventory: PASS   <-- verde, il difetto di RF-001 nell'altro verso
  # ripristinato; cancellata "Three things are specifically not guaranteed: ..."
  $ COBLOX_REPO=<copia> python sim/tools/published_artifacts.py
  published-artifact inventory: PASS   <-- verde
  ```

  **Rimedio:** due probe in più — `security-cadence-stretching` su `\*\*Stretching\*\* lengthens` e `security-sybil-three-not-guaranteed` sull'enumerazione — più, dopo la chiusura di RF-001, una probe sulla frase che dichiara il limite della misura, qualunque forma prenda. Il criterio generale che propongo di scrivere nel commento del blocco `SECURITY.md` del manifesto: **una probe per ciascuna metà di ogni affermazione a due lati**, perché una limitazione enunciata a metà si legge come completa.

  **Condizione di chiusura:** le due cancellazioni sopra fanno fallire il tool nominando la probe per id, dentro `prove_every_probe`.

- **RF-005 | category=verification-gap | severity=medium | criterion=«C11-CLAIMDOC verifica che l'insieme dei documenti di pretese su disco coincida con quello dichiarato, nei due versi»**

  **Non esiste un lato «disco».** `check_claim_documents` confronta `meta.claim_documents` del manifesto con `set(claims)`, e `claims` è costruito da `claim_documents()`, che itera la costante Python `CLAIM_DOCS = ("SECURITY.md",)` (`published_artifacts.py:90, 130-131`). I due versi confrontano **due dichiarazioni fra loro**, non una dichiarazione con la realtà. `C6-ORPHAN` funziona perché `documents()` fa `DOCS.glob("*.md")`, cioè ha un lato reale; la classe nuova no.

  La conseguenza è che il difetto di RF-001 di [REVIEW-025] — *la gate misura l'insieme sbagliato* — **resta possibile esattamente nella forma in cui si è già verificato**: un documento pubblicato nuovo non entra in nessuna delle due liste e nessuno se ne accorge. Oggi c'è già un candidato: `README.md` alla radice è pubblicato da GitHub, non è in `meta.documents` né in `meta.claim_documents`, e non porta pretese di sicurezza **oggi**.

  **Riproduzione (eseguita):**

  ```text
  # aggiunto alla copia un documento pubblicato nuovo alla radice:
  #   SECURITY-OVERVIEW.md, contenente
  #   "Coblox is Sybil-resistant and prevents a validator cartel from stretching the chain."
  $ COBLOX_REPO=<copia> python sim/tools/published_artifacts.py
  published-artifact inventory: PASS        exit=0
  ```

  Due affermazioni false, entrambe della famiglia che questo progetto ha già sbagliato cinque volte, in un file che GitHub pubblica, e la gate verde.

  **Rimedio:** dare a `C11-CLAIMDOC` un lato reale. La forma che raccomando è quella che il registro delle preimmagini usa già per la copertura: **enumerare dal disco e pretendere una classificazione esplicita per ciascuno**. Concretamente, i markdown della radice più `docs/**/*.md` vanno ripartiti fra `meta.documents`, `meta.claim_documents` e una terza lista dichiarata `meta.unpublished` con la ragione; un file che non compare in nessuna delle tre fa fallire `C11-CLAIMDOC`. Il default deve essere il fallimento, non il silenzio, perché è il silenzio ad aver prodotto RF-001.

  **Condizione di chiusura:** la creazione di un markdown nuovo alla radice fa fallire il tool nominando `C11-CLAIMDOC`, e la mutazione entra in `published_artifacts_negative.py`.

- **RF-006 | category=api-design | severity=low | criterion=«Il light client fallisce chiuso sul lato veloce e *segnala* sul lato lento»**

  **La metà che fallisce chiusa è tenuta dal tipo; la metà che segnala è tenuta dalla prosa.** `check_cadence_light_client` restituisce `Result<CadenceVerdict>`, e `CadenceVerdict` non è `#[must_use]` (`cadence.rs:52-53`, le derive sono `Debug, Clone, Copy, PartialEq, Eq`). Un chiamante che scrive

  ```rust
  check_cadence_light_client(measure_cadence_from_checkpoint(...)?)?;
  ```

  compila senza warning e **scarta silenziosamente `SlowerThanBand`**. Il `?` consuma il `#[must_use]` del `Result`, e nulla trattiene il valore interno.

  Non è una svista di stile: per la domanda 3, il lato lento è quello con la **soglia più bassa** (un terzo contro un quorum), il movente **esclusivo** (il seggio) e la **negabilità**. È la direzione dominante del pericolo, e la sua unica manifestazione nel codice è un valore che il compilatore permette di buttare via. Un divieto che nessuno esercita è famiglia 4; qui è un obbligo che nessuno esercita, che è la stessa cosa con il segno cambiato.

  **Riproduzione:** aggiungere l'espressione sopra a un test e osservare che `cargo clippy --workspace --all-targets --all-features -- -D warnings` resta pulito.

  **Rimedio:** `#[must_use]` sull'enum `CadenceVerdict`. Con `-D warnings` in CI il costrutto sopra diventa un errore di compilazione, e il chiamante è obbligato a **guardare** il verdetto — che è esattamente ciò che «segnala» significa. Costa un attributo. Va accompagnato, in `ledger.md` passo 4b, dalla precisazione di **a chi** il client riporta: oggi la norma dice «MUST report», e un `report` senza destinatario è una parola.

  **Condizione di chiusura:** l'espressione sopra non compila sotto `-D warnings`.

- **RF-007 | category=documentation | severity=low | criterion=«Ciò che il limite dà, detto stretto quanto è vero»**

  `ledger.md#reward_epoch-is-derived-from-height`: *«Cumulative existence emission through height `h` is at most `floor(h / reward_epoch_blocks) * F`»*. `F` è `existence_fund_microtokens_per_epoch`, **una grandezza della reward policy che la governance può muovere** fra un'epoca e l'altra, entro il rapporto di variazione e il tetto di `RewardBounds`. Il limite come è scritto assume `F` costante e vale quindi *a policy ferma*, non per regola.

  È il criterio di [ADR-010] applicato alla frase che [SPEC-016] ha appena scritto: *quale regola tiene questa proprietà?* La risposta per-regola è `existence_fund_microtokens_per_epoch_max` di `RewardBounds` (`params.rs:187`), che è nell'ancora di genesi e fuori dalla governance on-chain.

  **Riproduzione:** confrontare la frase con `RewardBounds`, che esiste proprio perché `F` è mobile.

  **Rimedio:** enunciare il limite con il tetto di genesi — *«at most `floor(h / reward_epoch_blocks) * existence_fund_microtokens_per_epoch_max`, and at most `floor(h / reward_epoch_blocks) * F` while the policy that carries `F` is in force»* — che è più debole e vera, invece di più forte e condizionata.

  **Condizione di chiusura:** la frase nomina la grandezza che una regola tiene.

- **RF-008 | category=maintainability | severity=low | criterion=nessuno (nota di lettura)**

  `cadence.rs:505-506`, nel test `the_lag_names_an_index_that_is_not_advancing`: il commento dice *«Height 3 456 000 is 200 epochs of chain, so epochs 0..=198 are settleable»*, mentre l'asserzione della riga successiva è `Some(199)` ed è quella corretta — con `(e+1)*17 280 <= 3 456 000` l'epoca 199 è liquidabile. Il commento sbaglia di uno nel verso restrittivo. Un commento fuori di uno accanto a un limite di liquidazione è il tipo di riga che un lettore futuro usa come definizione. Riproduzione: leggere le due righe adiacenti. Rimedio: `0..=199`.

---

## Ciò che ho attaccato senza riuscire a romperlo

Lo registro separatamente perché è informazione, e perché in tre casi l'attacco ha richiesto più lavoro dei finding.

**1. Il divieto su `timestamp_ms`.** Ho cercato una via per farlo rientrare: non è parametro di nessuna funzione del modulo, `measure()` è **privata** con la ragione scritta — un punto d'ingresso generico `(blocks, ms)` permetterebbe a un chiamante futuro di passarglielo — e il test che legge il sorgente lo esercita. Nessuna via aperta. È chiuso meglio di quanto la spec chiedesse.

**2. Il legame di catena del `ValidatorSet` sulle tre superfici.** Ho cercato di costruire un riuso fra catene: certificato di quorum replicato (fallisce sulla firma del voto, che porta `chain_id_32`), checkpoint replicato (fallisce sul `chain_id` del checkpoint), transizione di set replicata (fallisce sul `block_id` dell'intestazione). **Regge su tutte e tre, e regge anche per il set di genesi**, ma per l'argomento che il documento tiene in subordinata e non per quello che mette avanti. Il rifiuto di [DEBT-014] è **corretto nel merito**: RF-003 riguarda come è motivato, non se sia giusto.

**3. La derivazione di `reward_epoch` dal lato dell'evasione.** Ho cercato di allargare il permesso: `reward_epoch_ms` è scelto dalla policy che il mint nomina, quindi un quorum potrebbe accorciare l'epoca per accelerare l'indice — ma `reward_epoch_ms_min` di `RewardBounds` è nell'ancora di genesi e mette un pavimento su `reward_epoch_blocks` attraverso il `ceil`. Il `ceil` e non il `floor` è la scelta giusta e la ragione scritta è quella vera. Ho cercato una via per mintare due volte la stessa epoca: `ledger.md` impone *«at most one `existence_income` mint per `(beneficiary_node_id, reward_epoch)`»* e *«the sum of existence mints for an epoch MUST NOT exceed `F`»*, quindi la premessa del limite cumulativo tiene. **La chiusura di [DEBT-019] è solida**; RF-007 è sulla frase, non sulla regola.

**4. La procedura di rilascio.** Fallisce chiusa nei due versi e su `Inconclusive`, e l'esenzione del primo checkpoint è dichiarata invece che lasciata dedurre. Ho cercato di trovare in essa la stessa asimmetria di RF-001: **non c'è**, perché i due estremi sono due `issued_at_ms` firmati dallo stesso processo e la latenza si cancella fra i due. `check_cadence_release` è corretto e non è toccato da RF-001. Vale la pena dirlo esplicitamente, perché è la misura di cui RF-001 potrebbe far dubitare per contagio.

**5. `min_ms_per_block = 0` e le altre ancore degeneri.** `CadenceBand::validate` le rifiuta tutte, ed è chiamata come primo atto da entrambe le misure. Non ho trovato una via che raggiunga `measure()` senza passare per la validazione: è privata.

**6. La non convertibilità dichiarata in `SECURITY.md`.** *«The project's token is permanently non-convertible to money»* è una pretesa forte, e l'ho attaccata perché una moneta convertibile renderebbe monetario il movente del lato veloce e cambierebbe la risposta alla domanda 3. **Tiene, per una regola e non per una convenzione**: `ledger.md` rende il trasferimento diretto fra utenti **irrappresentabile**, non sconsigliato. L'unica riserva è sulla parola «permanently», che è un'affermazione sul futuro della governance e non su una regola presente; non la promuovo a finding perché la frase vive in una sezione il cui scopo è dire che non ci sono premi in denaro, e in quel contesto non è una pretesa di sicurezza.

**7. I tre conteggi del threat model.** Ho verificato che il meccanismo `[[claim_count]]` morde davvero e non è decorativo: il file oggi dichiara 43 scenari, un numero che nessuno ha trascritto. La forma scelta — ricalcolare dalla fonte invece di pinnare — è quella giusta e non l'ho scalfita.

---

## Required follow-up

**RF-001 è quella che vale il giro di remediation**, ed è l'unica che richiede una decisione di progetto e non una riscrittura: la tolleranza sul lato veloce è una grandezza nuova nell'ancora di genesi, e il suo **valore** è una decisione dell'operatore come `α`, la popolazione al lancio e gli altri tre valori della banda — va istruito, non scelto da un agente. La correzione della frase, nei quattro artefatti, va fatta **in ogni caso e anche se il rimedio 2 fosse rimandato**: una frase falsa che motiva un fallimento chiuso è il difetto anche senza il falso positivo.

RF-002, RF-003, RF-004, RF-006, RF-007 e RF-008 costano ciascuna fra una parola e un paragrafo e stanno nello stesso giro.

**RF-005 va valutata dal Lead per la sede.** È un difetto della gate di [ADR-012] e non di questa spec: la spec ha aggiunto `C11-CLAIMDOC`, che è un miglioramento netto, e il difetto è che la classe nuova ha ereditato la forma dichiarativa invece di quella reale. Se il Lead preferisce, è un debito proprio con owner sulla gate; la mia raccomandazione è di chiuderla qui, perché il costo è basso e perché è la **quarta** volta che una gate di questo progetto misura l'insieme che le è stato detto invece di quello che c'è.

**Il rafforzamento del paragrafo anti-Sybil** (domanda 5a) non è un finding: è lavoro che il Lead mi ha chiesto e che consegno con la formulazione esatta sopra. Va scritto **insieme** alla sua seconda metà — il qualificatore per-epoca — o non va scritto: la prima metà da sola sarebbe una pretesa che il paragrafo successivo dello stesso file smentisce.

**Due residui che confermo fuori scopo**, come [REVIEW-025]: `BLOCK_INTERVAL_SECONDS = 5 # assumption` in `sim/coblox_sim/recommended.py`, e le due liste chiuse di `what a light client can establish`, dove la deviazione 5 dell'implementatrice ha ragione — la cadenza è una grandezza calcolata e non un fatto di composizione.

**Un residuo nuovo da registrare:** `check_mint_reward_epoch` è una regola di **validità** dichiarata in `ledger.md` la cui sede di applicazione non esiste ancora in `coblox-core`. Non è un difetto oggi; va ripreso dalla spec che scriverà la validazione delle transazioni, e vale la pena che il Lead lo tenga in vista, perché è la forma di [REVIEW-017] RF-001 spostata avanti nel tempo.

## Final decision

**Changes requested.** Il verdetto non è sulla qualità dell'esecuzione, che è alta: la spec chiude tre debiti, l'esito su [DEBT-019] è quello forte, il rifiuto del punto 3 è corretto contro la spec che l'aveva chiesto, e il divieto su `timestamp_ms` è chiuso meglio di come era stato scritto.

Il verdetto è su una cosa sola: **la parte del lavoro che [REVIEW-025] ha lodato di più è quella che non è stata attaccata.** L'asimmetria fuori banda è un ragionamento sulla posizione dell'osservatore, ed è un buon ragionamento — ma guarda un estremo della misura e non l'altro, e l'estremo che non guarda ha una distorsione sistematica **verso il lato su cui il client fallisce chiuso**. La conseguenza è che una catena onesta, con un checkpoint onesto firmato da un processo che impiega quattro minuti a firmare, manda in fallimento chiuso ogni light client al primo verdetto che riesce a formulare.

Non era prevedibile da una gate. `GATE-MEASURE-BINDS` prova tre catene e le prova tutte a latenza di rilascio zero, che è la condizione che non esiste. Il difetto era, ancora una volta, **già scritto**: sta in `README.md#weak-subjectivity-checkpoint`, nella frase che dice che `timestamp_ms` e `issued_at_ms` sono distinti e che il secondo è *«when the checkpoint itself was produced»*. Quella frase è il difetto, ed è stata scritta perché l'orologio esterno funzionasse.

Aggiungo una cosa che riguarda me. La formulazione *«la direzione del pericolo è verso il rallentamento»* era mia, il Lead l'ha corretta e la correzione è giusta. Ma la correzione ha spostato l'attenzione sul lato veloce, e sul lato veloce il progetto ha messo il fallimento chiuso — mentre il lato lento resta quello con la soglia più bassa, il movente esclusivo e la negabilità, ed è tenuto da un valore che il compilatore permette di buttare via. **La correzione di una mia frase falsa ha prodotto una gerarchia dei moventi che è a sua volta sbagliata**, e questo è il pezzo che chiedo al Lead di registrare accanto alla correzione in [DEBT-013]: non che accelerare sia benigno — non lo è — ma che rallentare **costa un terzo e accelerare costa un quorum**, e che nessuno dei due documenti lo dice.
