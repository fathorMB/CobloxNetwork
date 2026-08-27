---
id: REVIEW-047
# Note: Quote the title if it contains a colon
title: "GATE-SECREVIEW su SPEC-025: la regola di blocco regge, e il blocco non e' legato al suo carico"
status: changes-requested
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-025
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, correctness, robustness, documentation, test-quality, verification-integrity]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-047-EVENT-001"
    timestamp: "2026-08-27T17:43:38.881829900+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Tre rilievi bloccanti su codice di consenso, e il Lead ha verificato i due che decidono.\n\nRF-001, il piu' grave, verificato leggendo il codice: `FinalizedBlock::verify` in `consensus/certificate.rs` controlla tre cose — che il certificato punti al `block_id` calcolato dall'header, che le altezze coincidano, e che le firme verifichino — e **non ricalcola mai `transactions_root` da `transactions`**. La lista delle transazioni non e' vincolata da nulla. Due nodi onesti possono quindi tenere `Block` diversi allo stesso `block_id` finalizzato, entrambi accettati dal verificatore spedito. Il criterio di sicurezza della spec asserisce su `(height, block_id)` e resta vero: e' l'artefatto pubblicato a divergere, ed e' un fork nello stato anche senza avversario. Il rimedio e' gia' nel crate: `registry::tx_id` piu' `merkle::transactions_root`.\n\nRF-003 verificato sul codice vero e non sulla ricostruzione: `proposer_at` calcola `index = (height + round) % total_power` su una scala di potere, quindi round consecutivi avanzano di un'unita' e un membro con potere `k` occupa `k` posizioni consecutive. La frase di `wire.md` chiude con «which is what makes a height survive a proposer that says nothing», ma con poteri non uniformi un proponente muto costa tanti round quanto il suo potere, non uno, e il test copre il solo set uniforme dove ogni membro ha una posizione sola. E' una proprieta' pubblicata piu' forte del fatto.\n\nRF-002 riguarda `header.round` non controllato mentre il doc-comment dichiara di controllarlo: e' una divergenza fra implementazioni conformi, che su codice di consenso e' un fork.\n\nCio' che regge, e va detto perche' e' la parte grossa della consegna. La regola di blocco e' sana e la reviewer l'ha verificata invece di crederle, compresa la direzione non percorsa: con `lockedRound == vr` l'intersezione forza `lockedValue == v`, e cio' che rende valido l'argomento e' nel codice — la POL e' verificata contro il proprio log e non contro il numero della proposta. Lo sblocco stretto di ADR-018 e' sano e l'affermazione dell'implementatrice e' vera. La collocazione di `f+1` fuori da `quorum.rs` e' giusta e non e' la forma di DEBT-045: e' una regola diversa, non una seconda scrittura della stessa. Lo scheduler avverso e' avverso davvero, perche' il taglio per coppia ordinata lascia entrambi i lati capaci di quorum, che e' la configurazione pericolosa.\n\nDue correzioni alle motivazioni dell'implementatrice, che il Lead registra perche' la conclusione sopravvive e la ragione no. L'assenza del voto nil perde informazione che l'Algorithm 1 usa, ed e' la riga 55 e non le tre elencate; e l'affermazione che nessuna regola che blocca o precommette legga un timer e' falsa. La sicurezza resta intatta, la vivacita' no.\n\nRilievo con conseguenza fuori dalla spec: `verifier.rs:94` dice «no consensus caller to fence today» ed e' la giustificazione portante per lasciare aperta la scappatoia non-consensus. Quel chiamante ora esiste, quindi la finestra che DEBT-029 dichiarava si e' chiusa. La meta' buona e' che l'implementatrice ha preso la strada giusta, `verify_in_context` in entrambi i punti.</reason>\n<evidence_refs>[\"SPEC-025\", \"ADR-018\", \"DEBT-029\"]</evidence_refs>\n<remediation_agent>AGENT-002</remediation_agent>\n</invoke>"
    implementation_agent: "AGENT-002"
links: [ADR-018, ADR-012, ADR-001, DEBT-029, DEBT-045]
created: 2026-08-27
updated: 2026-08-27
tags: [review, security, consensus]
activity:
  - date: 2026-08-27
    action: "transitioned pending -> changes-requested"
---
# Review

## Outcome

**Changes requested**, su tre rilievi. Il resto della consegna regge, e va detto per primo
perche' e' il risultato piu' importante di questa passata.

**La regola di blocco e' corretta e l'ho verificata invece di crederle.** Le quattro
divergenze dall'Algorithm 1 sono dichiarate, e la piu' pericolosa — lo sblocco stretto —
e' **sana**, con l'argomento che vale in entrambe le direzioni e non solo in quella che
l'implementatrice ha scritto. La sicurezza sotto avversario non e' rotta da niente che
io sia riuscita a costruire: nessuno dei rilievi qui sotto produce due `block_id`
finalizzati alla stessa altezza.

**Cio' che ho rotto e' un'altra cosa, e non e' meno grave: il `Block` pubblicato non e'
determinato dal consenso.** Un solo proponente guasto, dentro il budget di un terzo, fa
pubblicare a due nodi onesti due artefatti `Block` **diversi byte per byte alla stessa
altezza**, con lo **stesso `block_id`** e **ognuno con un certificato che il verificatore
spedito accetta**. E' costruito, non argomentato: `E5` piu' avanti. Il motivo e' che
`transactions` non e' legato a `header.transactions_root` da nessuna parte, e la
verifica e' interamente calcolabile dentro questo crate oggi — `registry::tx_id` e
`merkle::transactions_root` esistono entrambe.

Il secondo bloccante e' della stessa famiglia e piu' piccolo: `header.round` non e'
controllato da nulla, mentre il doc-comment di `verify_proposal` **dichiara** di
controllarlo e `wire.md#block_proposal` porta un MUST che nessuno impone. E' la forma
che l'incarico nomina come bloccante anche senza avversario — due implementazioni
conformi, una che legge il commento e una che legge il codice, accettano proposte
diverse.

Il terzo e' una frase falsa in un documento pubblicato: la regola del proponente e'
**pesata**, e la proprieta' di vivacita' con cui `wire.md` la giustifica vale solo a
potere uniforme.

Non ho inventato rilievi per giustificare la passata. Otto delle undici affermazioni che
ho controllato le ho trovate vere, e le tre gia' rilevate dall'implementatrice sono
giudicate qui invece che rifatte.

## Acceptance-criteria compliance

| criterio | esito | nota |
| --- | --- | --- |
| `coblox-block-prevote-v0` esiste con la preimmagine di [ADR-018] | soddisfatto | verificato: il payload dopo il separatore e' byte-identico a quello del precommit, e il test lo asserisce invece di dirlo |
| I tre messaggi in `wire.md` | soddisfatto con RF-002, RF-003 | le forme ci sono; due regole normative dentro `block_proposal` non sono imposte da nulla |
| Regola del proponente deterministica, stesso `validator_set_hash` -> stesso proponente | soddisfatto | `proposer_at` legge solo il set e la coppia |
| Motore senza I/O, dimostrato dalla forma dell'interfaccia | soddisfatto | l'argomento primario e' valido: `step_event(Event) -> Vec<Action>`, niente generici, niente `dyn`, niente closure, chiave mai dentro. Il lint secondario ha il limite di RF-009 |
| Quattro validatori, catena di dieci blocchi, certificati veri | soddisfatto | riletto: `Devnet::finalize` verifica **prima** di accettare, quindi la catena esiste solo se ogni certificato e' passato |
| Sicurezza sotto partizione, con il numero di esecuzioni dichiarato | soddisfatto sul `block_id`, **non** sul `Block` | 530 esecuzioni dichiarate e onestamente divise in due numeri. Ma l'asserzione e' su `(height, block_id)`, e RF-001 mostra due `Block` diversi con lo stesso `block_id` |
| Vivacita' dopo un proponente muto | soddisfatto a potere uniforme | RF-003: a potere pesato il proponente muto tiene `w` round consecutivi |
| Equivocazione rifiutata | soddisfatto per il precommit | `precommit_of` e' una difesa che non ha bisogno di rilevare, ed e' la scelta giusta. Il prevoto usa lo stesso percorso e non e' provato (RF-006) |
| Determinismo, byte per byte | soddisfatto **sul perimetro dell'harness** | vero perche' l'harness non varia mai `transactions`. RF-001 e' esattamente il caso in cui smette di esserlo |
| Regola di blocco confrontata con la fonte, e la fonte nominata | soddisfatto, ed e' il pezzo migliore | tabella riga per riga, `.tex` dell'e-print con i due sha256, numero di riga in commento su ogni regola |
| Nessuna modifica agli artefatti pubblicati | soddisfatto | riletto il diff sui cinque file: zero righe. La domanda su `vote_payload` condiviso e' posta dall'implementatrice e risposta con una fixture anteriore alla spec, per due strade indipendenti |
| Passata di [ADR-012], `PASS` e prova in negativo | soddisfatto sul meccanismo, **non** sul contenuto | 177 probe provate una per una e' un lavoro serio. Ma RF-003 e' una frase falsa **dentro** un documento pubblicato, e nessuna probe la copre |
| `cargo test`, `clippy -D warnings`, `fmt --check` | soddisfatto | verificato indipendentemente: vedi *Tests and verification* |

## Code observations

### La regola di blocco: cosa ho verificato, e perche' e' sana

**Il punto che rende valido tutto l'argomento della Divergenza 4 non e' nel testo che
lo espone, ed e' nel codice: il motore controlla il `valid_round` contro il
*proprio* log di prevoti**, non contro l'affermazione che la proposta porta.

```rust
Some(valid_round) => {
    if !self.prevote_quorum_for(valid_round, block_id)? {
        return Ok(false);
    }
```

Senza questo, `*locked_round < valid_round` confronterebbe il proprio blocco con un
numero scelto dal proponente, e l'intero ragionamento di intersezione dei quorum
cadrebbe. Con questo, il ragionamento e' esatto. Lo riscrivo perche' vada agli atti in
forma controllabile:

> `lockedRound_p = vr` con `v != lockedValue_p` richiede due insiemi di prevoti allo
> stesso round `vr`, ciascuno oltre due terzi del potere, per blocchi diversi. Due
> insiemi che superano entrambi i due terzi si intersecano in **piu' di un terzo** del
> potere, e ogni membro dell'intersezione avrebbe prevotato due volte in un round.
> Sotto meno di un terzo di potere guasto l'intersezione contiene potere onesto, e un
> motore onesto prevota al piu' una volta per round — `try_prevote_on_proposal` esige
> `step == Propose` e chiama `enter_prevote` in ogni ramo, quindi la seconda volta la
> guardia e' gia' chiusa. Il caso non esiste.

**E vale anche nella direzione che l'implementatrice non ha percorso.** Il timore
legittimo su uno sblocco piu' stretto non e' la sicurezza — sbloccare di meno non puo'
mai far finalizzare due blocchi — e' la **vivacita'**: se `<` rifiutasse un prevoto che
`<=` avrebbe concesso, un proponente onesto dopo GST potrebbe non raccogliere il quorum.
Non succede: nel caso `lockedRound == vr`, la stessa intersezione **forza**
`lockedValue == v`, e quel caso e' preso dal secondo ramo,
`|| locked.block_id == block_id`. Le due scritture coincidono in entrambe le direzioni
sotto l'ipotesi di guasto. **La Divergenza 4 non e' un difetto e non chiede rimedio in
codice** — chiede solo che [ADR-018] §2 e la riga 29 vengano riconciliati **nel testo**,
perche' un implementatore successivo non "corregga" lo stretto nella direzione insicura
credendo di allinearsi al paper (RF-008).

### Il carico del blocco non e' legato al blocco

`verify_proposal` esegue quattro controlli e nessuno guarda `transactions`.
`try_prevote_on_proposal` e `try_lock_and_precommit` chiamano `links_to_the_chain`, che
guarda `header.height` e `header.previous_block_id`. `FinalizedBlock::verify` ricalcola
`block_id` **dall'header**. In nessun punto della catena qualcuno chiede che
`transactions` sia il preimmagine di `header.transactions_root`.

`messages.rs:143` dice il contrario: *«il valore su cui si accorda e' `block_id`, e
`block_id` li copre attraverso `transactions_root`»*. Copre l'**hash**. Non lega il
**carico**, perche' nessuno confronta i due.

La difesa disponibile e' che sia `valid(v)` del chiamante. Non regge fino in fondo, per
la ragione che il modulo stesso stabilisce: il motore risponde gia' alla parte di
`valid(v)` **decidibile qui** — altezza e legame col genitore — e delega solo
`state_root`, che ha bisogno di un esecutore. `transactions_root` non ha bisogno di un
esecutore: `registry::tx_id` e `merkle::transactions_root` sono in questo crate, e la
verifica e' una passata sull'array. Il principio e' applicato a due controlli su tre.

### La memoria che un pari puo' far crescere, e l'argomento fermato a meta'

`mod.rs` fa questo argomento, testualmente, e lo fa bene:

> *Holding an unbounded number of future **heights** inside the engine would be a memory
> a remote peer could grow.*

E lo ferma li'. **I round dentro un'altezza non hanno alcun limite**, e `record_vote`,
`record` e `participants` scrivono per qualunque `round: u64`. Vedi RF-004.

### Cio' che ho controllato e ho trovato giusto

- **`precommit_of` come difesa senza rilevamento.** E' la scelta corretta e il commento
  la motiva meglio di come l'avrei motivata io: il primo precommit di ogni
  `(round, validator)` sta, un secondo diverso cade, quindi il potere di un equivocante
  raggiunge al piu' un `block_id` per round nel conteggio di **qualunque** nodo onesto,
  che l'equivocazione sia notata o no. Non dipende da una regola di rilevamento che
  qualcuno deve ricordarsi di eseguire.
- **La separazione dei due domini.** `verify_vote` sceglie la preimmagine dalla fase e
  passa da `verify_in_context`, quindi un prevoto presentato come precommit fallisce sul
  dominio prima che sulla firma. E' la confusione che lascerebbe a un solo messaggio
  bloccare un validatore **e** contribuire a finalizzare, ed e' chiusa nel punto giusto.
- **`power_of_one` che ritorna zero per un nome ignoto.** Fallisce chiuso: puo' solo
  rendere un quorum piu' difficile. Il commento lo dice e ha ragione.
- **Il precommit non puo' avvenire senza il blocco.** `try_lock_and_precommit` scrive
  `self.locked` e spinge il precommit dentro lo **stesso** `if self.step == Step::Prevote`.
  E' l'invariante da cui dipende tutta la sicurezza attraverso i round, ed e' impossibile
  romperla per distrazione perche' non ci sono due rami.
- **`Engine::start` rifiuta un `validator_id` non membro.** Un non-membro prevoterebbe
  dentro un insieme che scartera' ogni sua firma; fallire alla costruzione lo dice una
  volta invece che ogni round.
- **`verify_proposal` impone `sender == proposer_at(height, round)`.** Ho cercato la
  strada per cui un guasto singolo forgia proposte a nome altrui per gonfiare
  `participants` e forzare il salto di round: **e' chiusa**, perche' il mittente e'
  autenticato dalla busta e il controllo e' fatto prima del log. Resta che il legame
  `sender_node_id -> validator_id` e' del chiamante e il chiamante non esiste; `wire.md`
  lo specifica ed e' materia della spec del trasporto, non di questa.
- **La Divergenza 3 e' collocata bene.** `one_correct_threshold` e'
  `signed_power * 3 > total_power`, cioe' oltre **un terzo**: un predicato **diverso** da
  quello di `quorum`, non una seconda scrittura dello stesso. La forma di [DEBT-045] e di
  [DEBT-012] e' due scritture della **stessa** regola in due posti; qui c'e' una scrittura
  di una regola diversa, privata a `engine.rs`, che autorizza una cosa sola e quella cosa
  non decide niente. **Tenerla fuori da `quorum.rs` e' la scelta giusta** e la contesto
  solo per il refuso di RF-010.

## Tests and verification

### Rieseguito, non ripreso dall'evidenza

Le passate del Lead non le ho rifatte; ho rieseguito quel che serviva a me e non ho
trovato niente da contestare in esse.

```text
$ cargo test -p coblox-core --test greta_probe -- --nocapture     (file mio, temporaneo, rimosso)
$ cargo check -p coblox-core                                       exit 0
$ python sim/tools/consensus_no_io.py                              exit 0, PASS
$ cargo doc -p coblox-core --no-deps                               exit 0, 16 warning (9 nuovi)
```

L'albero e' stato mutato tre volte e ripristinato da copie prese **prima**, in
`scratchpad/engine.rs.bak` e `scratchpad/registry.rs.bak`, mai con `git checkout`. Stato
finale: nessuna modifica mia in `git status`.

### E5 — due `Block` diversi alla stessa altezza, costruito

Il rilievo centrale non e' un ragionamento. Due motori onesti (`val-002` e `val-003`),
un proponente che manda a ciascuno lo **stesso header** con un carico diverso, tre
prevoti e tre precommit firmati con le chiavi vere:

```text
E5 node val-002 finalized block_id 8d865f71e86907ba...
E5 node val-003 finalized block_id 8d865f71e86907ba...
E5 same block_id: true
E5 tx count A = 0
E5 tx count B = 1
E5 Block bytes A len 1364 / B len 1408
E5 published Block artifacts identical: false
```

Entrambi passano `FinalizedBlock::verify(&chain_id, &set, &ConsensusVerifier)`, cioe' la
stessa verifica che `Devnet::finalize` usa per ammettere un blocco in catena. Il carico B
e' `{"amount":1000000,"pay_to":"the-attacker"}`. **Un solo proponente guasto, dentro il
budget di un terzo.**

### E2 — il controllo dichiarato che non c'e'

```text
E2 verify_proposal(header.round=424242, proposal.round=0) -> Ok(...)
```

Una proposta di prima mano (`valid_round: None`) al round 0 il cui header dichiara il
round 424242 e' ammessa. Il doc-comment di `verify_proposal`, punto 3, dice *«the
header's `height` **and `round`** are the message's»*. Solo l'altezza e' confrontata
(`messages.rs:304`). Nota che `on_value` **impone** `header.round == round` sulle
proposte che il nodo costruisce da se': il motore e' severo con se stesso e permissivo
con gli altri, ed e' l'asimmetria che segnala il difetto.

### E1 — la regola del proponente a potere pesato

```text
E1 powers [1,1,1,7] height 1 rounds 0..12 ->
   [val-001, val-002, val-003, val-003, val-003, val-003, val-003, val-003, val-003, val-000, val-001, val-002]
E1 longest consecutive run by one proposer: 7
```

`wire.md` pubblica: *«Two consecutive rounds at the same height therefore step one unit
along the power ladder and **cannot name the same member while an unvisited one
remains**»*. Ai round 2..8 nomina `val-003` sette volte mentre `val-000` non e' ancora
stato visitato.

### E4 — il log dell'altezza non ha limite in round

```text
E4 2000 prevotes at distinct arbitrary rounds of height 1 were admitted and retained;
   engine still at height 1 round 0
```

Duemila prevoti firmati con la chiave vera di **un** membro, a round arbitrari fino a
`u64`, tutti ammessi al confine e tutti ritenuti in `prevotes`, `prevote_of` e
`participants`. Il motore resta al round 0 — il salto di round richiede oltre un terzo
del potere e un guasto singolo su quattro non ce l'ha, quindi **la sicurezza non e'
toccata** — ma la memoria cresce e non viene liberata finche' l'altezza non decide. Il
test `a_split_that_denies_both_sides_a_quorum_finalizes_nothing` della suite stessa
raggiunge il round 210 senza decidere niente: la finestra non e' ipotetica.

### E6 — che cosa aggira `consensus_no_io.py`

Eseguito, non argomentato. Ho aggiunto a `registry.rs` una funzione che chiama
`std::time::SystemTime::now()` e l'ho chiamata da `Engine::round()` in `engine.rs`:

```text
$ cargo check -p coblox-core                       Finished, exit 0
$ python sim/tools/consensus_no_io.py
  N1-IO-PATH       1888 candidate(s) checked
consensus engine no-I/O lint: PASS                 exit 0
```

Il codice compila, il motore legge l'orologio di sistema, il lint passa. Lo strumento
guarda `core/coblox-core/src/consensus/*.rs` e nient'altro: **un percorso di I/O a un
modulo di distanza e' invisibile**, e questo e' un residuo piu' largo dell'alias che la
docstring dichiara. Albero ripristinato.

### Lo scheduler avverso: e' avverso, e dove non lo e'

Letto `Adversary` e il ciclo di consegna. **E' avverso davvero** — ritardo estratto,
duplicazione, e soprattutto **taglio diretto per coppia ordinata**, che e' la forma che
conta: con `n=4` e quorum 3, tagliare `(0,1)` e `(1,0)` lascia **entrambi** i lati capaci
di quorum attraverso `2` e `3`, che e' esattamente la configurazione per cui la regola di
blocco esiste. Non e' un rimescolamento benigno.

Dove non lo e': **`blocked` e' fisso per tutta l'esecuzione e i messaggi tagliati sono
scartati, non ritardati**, quindi **una partizione non si richiude mai**. Le 530
esecuzioni non contengono una partizione che guarisce. Il compenso c'e' ed e' onesto
nominarlo: i ritardi fino a 400 ms contro timeout di 100 ms fanno arrivare messaggi molti
round dopo, che e' una riapertura morbida. Ma il caso canonico — una minoranza isolata
che si blocca, rientra, e trova la maggioranza andata avanti — non e' percorso. Vedi
RF-006.

### Le classi di condotta bizantina non rappresentate, e quale romperebbe la sicurezza

Censite: doppio **prevoto** (stesso percorso di codice del precommit, provato solo per il
precommit); **equivocazione di proposta** (ragionata in tre documenti, provata in
nessuno); una proposta che dichiara un `valid_round` la cui POL non esiste; un motore che
prevota contro il proprio blocco.

**Nessuna di queste rompe la sicurezza sotto meno di un terzo di potere guasto**, e lo
dico invece di lasciarlo intendere: ognuna richiederebbe due quorum di prevoto allo
stesso round, o due precommit dello stesso membro contati entrambi, e `prevote_of` e
`precommit_of` chiudono la seconda mentre l'intersezione dei quorum chiude la prima. La
classe che romperebbe la sicurezza e' **oltre un terzo di potere guasto**, che e' fuori
dal modello. La piu' preziosa da aggiungere e' l'equivocazione di proposta, perche' e'
quella su cui i documenti ragionano senza provarla — ed e' anche il veicolo di RF-001.

## Production quality and documentation compliance

[[QUALITY]] chiede di riportare limitazioni e deviazioni **onestamente**, e su questo la
consegna e' sopra la media del progetto: nove limitazioni dichiarate, tre affermazioni
riportate contro se stessa, due numeri invece di uno dove uno sarebbe bastato a fare
scena, e il perimetro consegnato insieme alla dimostrazione del determinismo. La
Divergenza 4 e' segnalata al Lead invece che risolta dall'implementatrice, che e' il
comportamento che l'incarico chiedeva.

Le tre cose che [[QUALITY]] non tollera e che ho trovato:

1. un doc-comment che **descrive un controllo che non esiste** (RF-002) — e' la forma
   peggiore, perche' un lettore che si fida non ha modo di accorgersene;
2. una frase **falsa** in un documento **pubblicato** (RF-003), sotto il regime di
   [ADR-012];
3. quattro doc-comment che affermano il contrario di `lib.rs` (RF-007), uno dei quali
   e' la **giustificazione portante** per lasciare aperta la scappatoia di [DEBT-029].

`cargo doc` produce nove warning nuovi da questo modulo (sette link a item privati,
`engine::one_correct_threshold` irrisolto, `vote_payload` privato). Non e' in nessuna
gate; lo nomino come RF-010 e non come motivo di blocco.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security-boundary | severity=high | **BLOCCANTE** | criterion=Il carico di un blocco non e' legato al blocco: nulla, in nessun punto del percorso, confronta `transactions` con `header.transactions_root`, e nessun documento dice a chi tocchi. **Scenario d'attacco**: il proponente di `(h, r)` — un singolo membro guasto, dentro il budget di meno di un terzo — invia lo stesso `header` a due nodi onesti con due array `transactions` diversi. `verify_proposal` ammette entrambi; `links_to_the_chain` guarda altezza e genitore; il quorum si chiude sul `block_id`, che e' identico; entrambi i nodi finalizzano. **Esito misurato (E5)**: stesso `block_id` `8d865f71e86907ba...`, `Block` di 1364 e 1408 byte, **entrambi accettati da `FinalizedBlock::verify` con il verificatore spedito**. Il carico B e' `{"amount":1000000,"pay_to":"the-attacker"}`. Il criterio di sicurezza della spec e' asserito su `(height, block_id)` e resta vero; l'artefatto `Block` che `ledger.md` definisce come `{header, transactions, quorum_certificate}` **e' diverso**, e il determinismo dichiarato «byte per byte» e' vero solo perche' l'harness non varia mai il carico. E' inoltre il caso in cui due implementazioni conformi divergono senza avversario: una che controlla il root rifiuta la proposta, una che non lo controlla la prevota. | remediation=Imporre il legame al confine, in `verify_proposal`: ricalcolare `merkle::transactions_root` sui `registry::tx_id` dei `transactions` portati e rifiutare se non riproduce `header.transactions_root`. **Entrambe le funzioni sono gia' in questo crate e non serve alcun esecutore**, quindi il controllo appartiene alla stessa classe di `links_to_the_chain` e non a quella di `state_root`. Scrivere la regola come MUST del ricevente in `wire.md#block_proposal`, dove oggi non c'e'. **Condizione di chiusura**: (a) un test in cui `verify_proposal` rifiuta una proposta il cui `transactions` non riproduce `header.transactions_root`; (b) E5 invertito — due motori onesti che ricevono lo stesso header con carichi diversi non finalizzano due `Block` diversi; (c) la riga MUST in `wire.md` e una probe di [ADR-012] che la fissa.

RF-002 | category=security-boundary | severity=high | **BLOCCANTE** | criterion=`header.round` non e' imposto da nulla, e il doc-comment di `verify_proposal` **dichiara di imporlo**. Il punto 3 dell'elenco recita *«the header's `height` and `round` are the message's»*; `messages.rs:304` confronta solo l'altezza. `wire.md#block_proposal` porta il MUST *«`header.round` ... MUST NOT be rewritten when the value is re-proposed»* e nessuno dei due capi lo verifica. **Scenario d'attacco (E2)**: il proponente del round 0 invia una proposta di prima mano il cui header dichiara `round: 424242`; e' ammessa, prevotata, bloccata, precommittata e finalizzata, quindi **un campo di un `BlockHeader` pubblicato e' scelto dall'attaccante**. Questo *aggrava* l'affermazione gia' rilevata su [ADR-018] Consequences: `header.round` non e' «solo a volte» il tentativo riuscito, e' **arbitrario**. **Divergenza fra implementazioni conformi**: chi legge il doc-comment e implementa `header.round == proposal.round` rifiuta ogni **ri-proposta**, dove `header.round < proposal.round` e' la forma corretta e necessaria; quell'implementazione stalla ogni altezza che richieda un secondo round. Chi legge il codice non rifiuta niente. **Bloccante anche senza avversario.** | remediation=Nel ramo `valid_round: None` imporre `header.round == proposal.round`; nel ramo `valid_round: Some(vr)` non serve alcun controllo aggiuntivo, perche' `block_id` copre l'header e la POL a `vr` e' gia' verificata contro il log proprio — dirlo esplicitamente nel commento invece di lasciarlo dedurre. Correggere il punto 3 del doc-comment cosi' che descriva il controllo che esiste. Portare in `wire.md#block_proposal` la regola nella forma che un ricevente puo' applicare. **Condizione di chiusura**: E2 invertito (la proposta con `header.round` riscritto e `valid_round` assente e' rifiutata da `verify_proposal`), un test che dimostra che la **ri-proposta** con `header.round < round` resta accettata, e i due testi allineati.

RF-003 | category=correctness | severity=medium | **BLOCCANTE** | criterion=La regola del proponente e' **pesata per potere**, e la proprieta' di vivacita' con cui `wire.md` e `proposer.rs` la giustificano vale solo a potere **uniforme**. `wire.md` pubblica *«Two consecutive rounds at the same height ... **cannot name the same member while an unvisited one remains**»*; `proposer.rs` la dichiara *«the one liveness obligation»* per cui l'indice `(height + round)` e' stato scelto. **E' falsa (E1)**: con poteri `[1,1,1,7]` l'indice cammina la scala del potere di un'unita' per round, e il membro pesante occupa **sette posizioni consecutive** — i round 2..8 nominano tutti `val-003` mentre `val-000` non e' stato visitato. **Scenario**: un membro guasto con potere `w` tiene `w` round consecutivi della stessa altezza; con `delay_ms = base + round * increment` il costo dello stallo cresce quadraticamente in `w`. L'unica regola che forza potere 1 e' `ValidatorSet::check_elected_shape`, che **nessun percorso di consenso chiama**: `Engine::start` e `proposer_at` chiamano `check_structure`, che ammette poteri arbitrari, e [ADR-001] prevede un set pesato. Il test `consecutive_rounds_visit_every_member_before_repeating` gira sul set uniforme dell'harness, quindi **verifica un caso particolare e viene presentato come se verificasse l'obbligo**. Sotto [ADR-012] questa e' una frase falsa dentro un documento pubblicato, e nessuna delle cinque probe nuove la copre. | remediation=Una delle due. **(a)** Restringere l'affermazione pubblicata: dire che la proprieta' vale a potere uniforme, che e' cio' che `check_elected_shape` impone a un set eletto, e dichiarare che a potere pesato un membro con potere `w` propone in `w` round consecutivi — con la conseguenza sulla vivacita' scritta accanto. **(b)** Cambiare l'indice in modo che round consecutivi non possano ripetere un membro finche' ne resta uno non visitato, conservando la proporzionalita' su un ciclo intero (e' il problema che la selezione a priorita' incrementale risolve; e' una modifica di [ADR-018] §3 e va dal Lead, non dall'implementatrice). **Condizione di chiusura**: il test del proponente esteso a un set a potere non uniforme, e il testo di `wire.md` e di `proposer.rs` che dice il vero su quel set.

RF-004 | category=robustness | severity=medium | criterion=`HeightLog` non ha alcun limite sui **round**. `record` e `record_vote` accettano qualunque `round: u64` dell'altezza in corso e scrivono in `proposals`, `prevotes`, `prevote_of`, `precommits`, `precommit_of` e `participants`; nulla viene liberato finche' l'altezza non decide. **Scenario d'attacco (E4)**: un singolo membro guasto — dentro il budget — firma prevoti a round arbitrari e ne ho fatti ammettere e ritenere **2000**, ognuno con la propria voce in tre mappe. Il costo per l'attaccante e' una firma Ed25519; il costo per il ricevente e' memoria che resta. La finestra e' quella di un'altezza che non decide, e il test `a_split_that_denies_both_sides_a_quorum_finalizes_nothing` della suite stessa la tiene aperta per 210 round. In coda, `try_skip_round` scandisce ogni round di `participants` a ogni giro di `drive`, quindi il costo in CPU cresce con la stessa quantita'. **La sicurezza non e' toccata**: il salto di round esige oltre un terzo del potere. **Non bloccante** perche' non e' un difetto di consenso e il rimedio e' additivo, ma va nominato perche' `mod.rs` fa **esattamente questo argomento per le altezze** — *«a memory a remote peer could grow»* — e si ferma prima di applicarlo ai round, che e' l'asse illimitato dentro l'altezza che il motore esegue. | remediation=Rifiutare, al confine o in `record`, un messaggio con `round > round_p + W` per una finestra `W` dichiarata; pubblicarla nella sezione di validazione gossip di `wire.md` insieme alle altre regole di contropressione, e dichiararla parametro **locale** come i tre timeout. **Condizione di chiusura**: un test in cui un membro che invia oltre `W` round avanti non fa crescere il log, e la riga in `wire.md`.

RF-005 | category=documentation | severity=medium | criterion=La compensazione della Divergenza 1 non e' il soprainsieme che il testo afferma, in tre punti verificabili. **(a)** `mod.rs` scrive *«no rule that locks, precommits or decides reads a timer»*. Falso come scritto: `try_lock_and_precommit` e' guardato da `self.step`, e `self.step` e' scritto da `on_timeout`. La **conclusione** sopravvive — un timer puo' solo *sopprimere* un blocco e un precommit, e sopprimerli non puo' produrre finalita' in conflitto — ma la ragione data non e' la ragione. **(b)** Alla domanda se l'assenza del nil perda informazione che l'Algorithm 1 usa, la risposta e' **si', alla riga 55**: i voti nil sono messaggi `<*, h_p, round, *, *>` contati dalla regola di salto di round `f+1`, e `mod.rs` elenca soltanto le righe 34, 44 e 47. Senza nil, un round il cui proponente tace **non produce alcun messaggio onesto**, quindi un nodo in ritardo non riceve da quel round alcun segnale di salto e deve camminare i round uno alla volta con timeout crescenti. Composto con RF-003 — un membro pesante muto che tiene `w` round — il recupero e' quadratico. **(c)** Armare `OnTimeoutPrevote` al cambio di passo invece che al quorum di due terzi di prevoti significa che il timer puo' **scadere prima che arrivi un solo prevoto**: il nodo lascia `Prevote`, e da li' non puo' piu' bloccarsi ne' precommittare quel round anche se il quorum arriva un evento dopo. L'Algorithm 1 garantisce un `timeoutPrevote(r)` **intero dopo** il quorum; questo motore no. E' soprainsieme nell'**armare** e non nel **comportamento**. **La sicurezza non e' toccata in nessuno dei tre punti.** | remediation=Correggere le tre frasi in `mod.rs` §*Divergence 1*: sostituire l'argomento (a) con quello vero (un timer puo' solo sopprimere un blocco o un precommit, e la soppressione e' sicura per costruzione); aggiungere la riga 55 all'elenco di cio' per cui i nil servono, con la conseguenza sul recupero; dichiarare (c) come il costo di vivacita' che e'. **Condizione di chiusura**: le tre frasi corrette, e nessuna modifica di codice richiesta.

RF-006 | category=test-quality | severity=medium | criterion=Due lacune di copertura, entrambe nominabili con precisione. **(a) Nessuna partizione guarisce.** `Adversary.blocked` e' fissato alla partenza dell'esecuzione e i messaggi tagliati sono **scartati**, non ritardati: nelle 530 esecuzioni un taglio non si riapre mai, quindi il caso canonico della regola di blocco — una minoranza isolata che si blocca su un valore, rientra, e trova la maggioranza andata avanti — non e' percorso. Il compenso esiste e va riconosciuto: il taglio e' **per coppia ordinata**, quindi con `n=4` e quorum 3 entrambi i lati restano capaci di quorum attraverso i due membri comuni, che e' la configurazione pericolosa; e i ritardi fino a 400 ms contro timeout di 100 ms sono una riapertura morbida. **(b) Tre condotte bizantine non rappresentate**: doppio **prevoto** (stesso `first_of` del precommit, quindi stesso percorso di codice, provato solo per il precommit); **equivocazione di proposta**, su cui `mod.rs`, `messages.rs` e `wire.md` ragionano tutti e tre senza che un test la produca — ed e' il veicolo di RF-001; una proposta che dichiara un `valid_round` la cui POL non esiste. **Nessuna delle tre rompe la sicurezza sotto meno di un terzo di potere guasto**, e questa review lo afferma invece di lasciarlo intendere: ognuna esigerebbe due quorum di prevoto a un round o due precommit dello stesso membro contati entrambi, e l'intersezione dei quorum chiude la prima mentre `prevote_of`/`precommit_of` chiudono la seconda. | remediation=Aggiungere allo sweep una partizione che **si richiude**: un insieme di coppie tagliato per una finestra di tempo e poi riaperto, con i messaggi trattenuti consegnati alla riapertura invece che scartati — e' la forma in cui un nodo bloccato rientra. Aggiungere un test di doppio prevoto e uno di equivocazione di proposta con consegna mirata. **Condizione di chiusura**: lo sweep dichiara quante delle sue esecuzioni contengono un taglio riaperto, con lo stesso rigore con cui oggi dichiara quante ne contengono uno.

RF-007 | category=documentation | severity=medium | criterion=Quattro doc-comment affermano che questo crate non spedisce un verificatore di firma — `identity.rs:421`, `light_client.rs:119`, `light_client.rs:172`, `verifier.rs:94` — mentre `lib.rs` riesporta `ConsensusVerifier`, `verify_consensus_ed25519` e `verify_in_context`. **Il quarto e' quello che conta**, perche' non e' una descrizione ma la **giustificazione** per lasciare aperta la scappatoia: *«there is no consensus caller to fence today»*. **[SPEC-025] e' quel chiamante**, e [DEBT-029] poggia sulla stessa frase: *«Il danno oggi e' nullo ... non esiste alcun chiamante di consenso da proteggere. Il difetto e' interamente sul futuro»*, con `severity: medium` motivata da *«la finestra si chiude quando il primo chiamante viene scritto»*. **Quella finestra si e' chiusa con questo commit.** Va detta anche la meta' buona, perche' e' la piu' importante: il primo chiamante di consenso ha preso la strada **giusta** — `messages::verify_vote` e `certificate::verify` passano entrambi da `verify_in_context` — quindi la convenzione ha ora l'esempio che il proprio file non aveva, ed e' esattamente cio' che i criteri di risoluzione di [DEBT-029] chiedevano di poter giudicare *«avendo davanti il chiamante reale»*. Cio' che non e' successo e' il recinto: `verify_consensus_ed25519` resta riesportata alla radice senza feature gate ne' guardia. [DEBT-029] e' `planned` con `target_specs: [SPEC-025]` e questa spec non lo ha toccato. | remediation=Del Lead, non dell'implementatrice, e in due mosse. Correggere le quattro frasi. Rivalutare [DEBT-029]: la premessa dichiarata e' cambiata, quindi o la severita' sale — la finestra e' chiusa e il costo e' ora una migrazione, non un recinto — o il debito viene ri-instradato su una spec successiva con la ragione scritta. **Condizione di chiusura**: le quattro frasi vere, e un evento su [DEBT-029] che registri che il chiamante di consenso esiste e quale strada ha preso.

RF-008 | category=correctness | severity=info | criterion=**Non e' un difetto, ed e' registrato perche' e' il rilievo che questa gate doveva giudicare.** La Divergenza 4 — `*locked_round < valid_round` contro `lockedRound_p <= vr` della riga 29 — e' **sana**, e l'ho verificata invece di accettarla. Il caso di differenza, `lockedRound = vr` con `v != lockedValue`, esige due insiemi di prevoti allo stesso round, ciascuno oltre due terzi del potere, per blocchi diversi: si intersecano in oltre un terzo del potere, e ogni membro dell'intersezione avrebbe prevotato due volte in un round, che un motore onesto non fa (`try_prevote_on_proposal` esige `step == Propose` e chiama `enter_prevote` in ogni ramo). L'affermazione dell'implementatrice e' vera. **In piu', vale nella direzione che lei non ha percorso**: nel caso `lockedRound == vr` la stessa intersezione forza `lockedValue == v`, e quel caso e' preso dal ramo `|| locked.block_id == block_id`, quindi le due scritture coincidono anche per la **vivacita'** e non solo per la sicurezza. Cio' che rende valido l'argomento sta nel codice e non nel testo che lo espone: il motore verifica la POL contro il **proprio** log (`prevote_quorum_for(valid_round, block_id)`) invece di fidarsi del numero che la proposta porta — senza questo l'intera catena di ragionamento cadrebbe. | remediation=Nessuna modifica di codice. Riconciliare il **testo**: [ADR-018] §2 dice «maggiore» e la riga 29 dice `<=`; scrivere accanto alla decisione che le due coincidono sotto l'ipotesi di guasto, in entrambe le direzioni, e che la stretta e' la piu' restrittiva — cosi' che un implementatore successivo non «corregga» lo stretto verso il largo credendo di allinearsi al paper. E' l'unica direzione in cui questa regola puo' costare sicurezza.

RF-009 | category=verification-integrity | severity=low | criterion=`consensus_no_io.py` dichiara di essere «un lint e non un confine» e nomina come residuo l'alias. **Il residuo e' piu' largo, ed e' eseguito e non argomentato (E6)**: lo strumento legge `core/coblox-core/src/consensus/*.rs` e nient'altro, quindi un percorso di I/O raggiunto attraverso una funzione di un **modulo fratello dello stesso crate** e' invisibile. Ho aggiunto `registry::wall_clock_ms_probe()` con `std::time::SystemTime::now()` e l'ho chiamata da `Engine::round()`: `cargo check` exit 0, lint `PASS` exit 0 con 1888 candidati esaminati. Il motore leggeva l'orologio di sistema. In coda, la lista dei nomi non contiene `chrono`, `web_time`, `libc`, ne' `HashMap`/`RandomState` — quest'ultimo romperebbe il criterio di **determinismo** dichiarato dalla spec piu' che quello di no-I/O, e nessuna delle due guardie lo vedrebbe. **Non bloccante**: la dimostrazione primaria e' la forma dell'interfaccia e quella regge, e lo strumento dichiara di non essere un confine. | remediation=Aggiungere alla docstring che l'ambito sono i file del modulo e non i suoi chiamati, oppure estendere la scansione alle funzioni del crate che il modulo nomina. Aggiungere `HashMap` e `RandomState` alla lista, con la ragione (determinismo, non I/O). **Condizione di chiusura**: la mutazione di E6 osservata fallire nella prova in negativo, oppure il limite scritto dove un lettore lo incontra.

RF-010 | category=documentation | severity=low | criterion=`cargo doc -p coblox-core --no-deps` produce nove warning nuovi da questo modulo: sette link da documentazione pubblica a item privati di `Engine` (`start_round`, `try_prevote_on_proposal`, `try_lock_and_precommit`, `try_decide`, `try_skip_round`, `on_timeout`), `engine::one_correct_threshold` **irrisolto** perche' l'item e' privato, e `vote_payload` privato linkato da `block_prevote_preimage`. La tabella riga-per-riga di `mod.rs` — che e' il pezzo migliore della consegna e l'artefatto che `GATE-LOCKING-FROM-SOURCE` produce — **non e' navigabile nella documentazione generata**: meta' delle sue celle di destra sono link morti. `cargo doc` non e' in nessuna gate del progetto, quindi nessuna passata lo avrebbe visto. | remediation=Rendere i link navigabili o testuali. Valutare `cargo doc -D warnings` fra le passate di progetto, che e' una decisione del Lead e non di questa spec.

## Required follow-up

**Bloccanti, in ordine di costo crescente per chi rimedia:**

1. **RF-002** — `header.round == proposal.round` quando `valid_round` e' assente, il
   doc-comment corretto, la regola in `wire.md` nella forma che un ricevente applica.
2. **RF-001** — `transactions_root` ricalcolato e confrontato in `verify_proposal`, il
   MUST in `wire.md`, e E5 invertito. E' il rilievo centrale.
3. **RF-003** — o l'affermazione pubblicata si restringe al potere uniforme dicendo cosa
   succede altrove, o l'indice cambia. La seconda strada tocca [ADR-018] §3 ed e' del
   Lead.

**Non bloccanti, da portare in una review accettata**: RF-004 (finestra sui round),
RF-005 (le tre frasi della Divergenza 1), RF-006 (partizione che guarisce, doppio
prevoto, equivocazione di proposta), RF-007 (le quattro frasi e la rivalutazione di
[DEBT-029]), RF-008 (riconciliare [ADR-018] §2 con la riga 29 **nel testo**), RF-009,
RF-010.

**Del Lead e non di questa review**: [DEBT-029] e' `planned` con
`target_specs: [SPEC-025]` e questa spec non lo ha chiuso. La condizione che il debito
stesso dichiarava — *«SPEC-025 e' quel chiamante ... e' la consegna in cui la condizione
diventa vera»* — si e' avverata, e il debito non e' stato toccato. Va registrato in un
modo o nell'altro.

## Final decision

**Raccomando `changes-requested` su RF-001, RF-002 e RF-003.**

La ragione, in una riga: **RF-001 e RF-002 sono superfici di fork raggiungibili da un
solo partecipante dentro il budget di guasto, e RF-002 porta in piu' un doc-comment che
descrive un controllo inesistente** — la forma che l'incarico nomina come bloccante anche
senza avversario, perche' fa divergere due implementazioni conformi e su codice di
consenso una divergenza e' un fork. RF-003 e' una frase falsa in un documento pubblicato
sotto il regime di [ADR-012], con un test che verifica un caso particolare presentandolo
come l'obbligo generale.

**Nessuno dei tre e' un difetto della regola di blocco**, e questo va scritto accanto al
verdetto perche' e' l'informazione che serve al Lead. Il rischio dominante che
[SPEC-025] dichiarava — tre righe su cui poggia la sicurezza dell'intero ledger — e'
stato **preso dalla letteratura, confrontato riga per riga con la fonte, e attuato
correttamente**, e le quattro divergenze sono dichiarate e nessuna rompe la sicurezza.
Il primo consenso funzionante del progetto funziona. Cio' che non e' finito e' il
**legame fra il blocco su cui il consenso si accorda e i byte che quel blocco pubblica**:
il consenso decide un `block_id`, e il `Block` che ne esce non e' determinato da quel
`block_id`. E' un rimedio piccolo — due confronti al confine — e va fatto adesso, perche'
il chiamante che avrebbe potuto farli al posto suo non esiste, e quando esistera' avra'
gia' letto cosa trova.

Rimane fuori dal mio giudizio: `GATE-CI-GREEN` e `GATE-LEAD-REPRO` sono del Lead. Il
verdetto formale lo registra il Lead; questa e' una raccomandazione.
