---
id: SPEC-020
# Note: Quote the title if it contains a colon
title: "L'orologio su cui si misura la scadenza di un'attestazione"
status: done
kind: feature
priority: medium
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: [SPEC-016, SPEC-018]
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012, ADR-013, ADR-015]
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [security, identity, light-client]
activity:
  - date: 2026-08-26
    action: "transitioned backlog -> ready"
  - date: 2026-08-26
    action: "transitioned ready -> working"
  - date: 2026-08-26
    action: "transitioned working -> review"
  - date: 2026-08-26
    action: "attested verification GATE-SECREVIEW by lead"
  - date: 2026-08-26
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "64ca916d7cb31431f601b35b7764d4421c7ff990b28bda250c0ae087c864469f"
    evidence_ref: "REVIEW-035"
    id: "SPEC-020-ATTEST-001"
    requirement_digest: "0620483dc649ca8cbff2cb6212d72dee106b6a9496eec7b48ff67f4b2c780289"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-26T15:01:47.548573200+02:00"
---
# L'orologio su cui si misura la scadenza di un'attestazione

## Objective

Chiudere [DEBT-017], **che non è il debito che il suo titolo diceva**. Il titolo originale — *«la finestra è skew più durata, e solo la durata è limitata»* — è vero e nomina il termine **minore**. L'esposizione reale è `D_max + S_max + (ritardo dell'orologio del ricevente)`, e **solo il terzo addendo è illimitato**.

Il termine illimitato è già dichiarato in `identity.md` §*Declared limit*: *«A receiver whose clock is far behind accepts attestations that expired hours ago, and no certificate attests a clock.»* Questa spec chiude quel termine, o dichiara motivatamente perché non lo chiude.

## Context

**La premessa da cui il Lead era partito era sbagliata, e la correzione è la sostanza di questa spec.** Il Lead aveva chiesto ad AGENT-007 se il rimedio richiedesse un orologio che la catena non ha, citando [ADR-013]. La risposta: [ADR-013] stabilisce che nessuna regola interna può **vincolare il tempo reale**, ed è vero — ma qui non serve vincolarlo. Serve un **minorante** di `now_ms` per chi verifica. E per un minorante l'abuso dei validatori è **fail-closed**:

- il `timestamp_ms` dei blocchi è monotono crescente sulla mediana degli undici precedenti, quindi l'ultimo blocco finalizzato è un minorante non decrescente;
- gonfiarlo fa **rifiutare** più attestazioni, non accettarne di più longeve;
- sgonfiarlo è impedito dalla mediana.

Ne discende un rimedio scrivibile con l'orologio che la catena **già ha**: chi possiede una testa finalizzata valuta con `now_ms = max(orologio locale, timestamp_ms dell'ultimo blocco finalizzato)`; chi è in bootstrap ricade sull'orologio locale come oggi.

**Il costo che quella valutazione non nomina, e che decide la spec.** *Fail-closed* qualifica la sicurezza, non la disponibilità, e qui le due divergono. `identity.md` non dice che un peer senza attestazione valida viene ignorato: dice **MUST be rejected and disconnected**. Un set di validatori che gonfia `timestamp_ms` non degrada una verifica — **disconnette la rete**, su ogni nodo onesto che possieda una testa finalizzata, simultaneamente, senza firmare nulla di invalido.

Ed è **esattamente l'attore di [DEBT-013], con esattamente la capacità che [DEBT-013] gli ha accertato.** Quel debito ha stabilito che il set attivo scrive gli orologi della catena e che un terzo bloccante può muoverli con la catena viva e ogni blocco valido. Il rimedio prenderebbe quella capacità — oggi limitata a **rallentare** — e la convertirebbe in una leva di **partizione del trasporto**. Un rallentamento è invisibile e reversibile; una disconnessione di massa è immediata.

**Una sola domanda decide quale rimedio sia il più forte.** L'ampiezza della leva dipende interamente dall'unica interazione che AGENT-007 ha dichiarato **non istruita**: se il controllo di drift verso l'alto su `timestamp_ms` sia applicato dai validatori onesti in accettazione del blocco. Se sì, l'inflazione è limitata dal loro orologio e la leva è piccola. Se no, la leva è illimitata e **l'ordine di forza si inverte**.

La stessa domanda regge una seconda proprietà, che AGENT-007 aveva nominato: se quel controllo si applicasse ai blocchi storici durante il sync, un nodo con orologio molto indietro rifiuterebbe i blocchi recenti e **non acquisirebbe mai la testa che gli servirebbe** — la circolarità che l'attestazione esisteva per evitare, rientrata da un'altra porta.

**Perché dipende da [SPEC-016].** Entrambe cercano un orologio esterno alla volontà di chi verifica. [SPEC-016] ne costruisce uno attorno al checkpoint di soggettività debole. Se questa spec ne costruisse un altro, il progetto avrebbe **due orologi diversi** per la stessa proprietà, e nessuno saprebbe quale sia quello vero.

## Scope

### Included

- La risposta alla domanda sul controllo di drift, **prima** di scrivere qualunque regola.
- Il minorante su `now_ms` per la valutazione delle regole di scadenza dell'attestazione, se la risposta lo consente.
- La misura e la dichiarazione dell'ampiezza della leva di partizione, se il minorante viene adottato.
- La dichiarazione della somma `D_max + S_max` accanto ai due parametri, **in ogni caso**, e la dichiarazione esplicita del residuo.
- La riscrittura della contromisura (a) di TM-37, coordinata con [SPEC-018].

### Excluded

- **Un vincolo relazionale fra `S_max` e `D_max` come chiusura.** Chiude solo l'evasione della regola 4 da parte di un emittente che postdata, e lascia intatto il termine illimitato: è la famiglia 3 commessa dentro il rimedio, ed è il rimedio che il debito originale suggeriva. Ammesso solo **in aggiunta** e mai come chiusura, e solo se accompagnato dalla dichiarazione del residuo.
- **Ritarare la tolleranza in genesi.** [ADR-010]: un valore scelto bene non è una proprietà. E `S_max` è un parametro **a due pericoli** — verso l'alto l'evasione del tetto, verso il basso l'isolamento del nodo con orologio indietro.
- **Concedere tolleranza oltre `expires_at_ms` per simmetria.** Aggiungerebbe un quarto addendo limitato a un termine già illimitato.
- **Invalidazione anticipata con epoca o numero di serie.** Già respinta in TM-37(c): un contatore per identità osservabile in sessione è un identificatore stabile in più, e ricrea la correlazione che [ADR-015] ha tolto.
- Qualunque modifica alla meccanica dell'envelope. Le due finestre governano proprietà indipendenti e **non esiste una relazione da scrivere fra loro**.

## Existing-project analysis

La finestra `D_max + S_max` è accertata riga per riga in `identity.md` §*rejection rules* e implementata in `core/coblox-core/src/identity.rs`, già asserita in negativo dalla suite di conformità che verifica il rifiuto un millisecondo prima del bordo.

`S_max` **non è disponibile all'avversario di TM-37**: postdatare richiede la chiave di identità, che quell'avversario non ha. L'unico abuso reale del termine è un nodo che collude con sé stesso per consegnare a un confederato una credenziale più longeva del tetto.

`D_max` e `S_max` non sono fissati da alcun documento di genesi. La loro taratura **non cambia** questa spec, perché il termine dominante non dipende da nessuno dei due.

## Technical proposal

**La verifica dell'interazione non è una precondizione fra le altre: è il primo passo, e il suo esito sceglie il rimedio.** Va istruita e riportata prima che una riga di regola sia scritta, e riportata **anche se l'esito è scomodo**.

Se l'inflazione è limitata dagli onesti in accettazione: il minorante è l'esito più forte, e la leva va misurata e dichiarata, non solo nominata. La misura è la distanza massima che il set può spingere il minorante oltre il tempo reale prima che un blocco venga rifiutato.

Se l'inflazione non è limitata: il minorante trasferisce un rischio da rallentamento a partizione, e l'ordine di forza si inverte. La chiusura diventa la dichiarazione della somma e del residuo, più eventualmente il vincolo relazionale — **e va detto esattamente che chiude l'evasione e nient'altro**, perché un lettore che vede il vincolo conclude che l'esposizione sia limitata mentre è limitata solo la parte che i due parametri governano.

**Il checkpoint di soggettività debole è la seconda fonte di minorante**, più debole in freschezza ma disponibile **anche al nodo in bootstrap**. Va considerato in alternativa o in aggiunta, e **deve essere lo stesso oggetto che [SPEC-016] costruisce**.

## Files and areas involved

- `docs/protocol/identity.md` — la regola, la somma dichiarata, il residuo.
- `core/coblox-core/src/identity.rs`, `light_client.rs` — il minorante e la ricaduta di bootstrap.
- `core/coblox-core/tests/` — le prove in negativo e il caso di bootstrap.
- `.lmbrain/knowledge/threat-model.md` — la contromisura (a) di TM-37, coordinata con [SPEC-018].
- `sim/tools/` — la gate di [ADR-012], se una regola viene scritta.

## Acceptance criteria

- [x] La domanda sul controllo di drift è **risolta e riportata**, prima di ogni regola, e il suo esito è quello che ha scelto il rimedio.
- [x] Se il minorante è adottato: l'ampiezza della leva di partizione è **misurata e dichiarata**, non solo nominata.
- [x] Se il minorante è adottato: un nodo **senza** testa finalizzata si comporta esattamente come oggi, e una prova lo mostra.
- [x] L'orologio esterno usato è **lo stesso** che [SPEC-016] ha costruito, e non un secondo.
- [x] La somma `D_max + S_max` è dichiarata accanto ai due parametri, e il **residuo illimitato** è dichiarato con essa — in ogni caso, anche se il minorante non viene adottato.
- [x] Nessuno dei quattro rimedi esclusi è stato adottato come chiusura.
- [x] Se una regola di validità è stata scritta: la gate di [ADR-012] è eseguita e la trascrizione allegata.

## Implementation plan

1. Istruire e riportare la domanda sul controllo di drift. Nessuna regola prima.
2. Scegliere il rimedio in base all'esito, e dichiarare la scelta con la sua ragione.
3. Se minorante: implementarlo, misurare la leva, provare il caso di bootstrap.
4. La somma e il residuo dichiarati, in ogni caso.
5. TM-37(a), coordinata con [SPEC-018].
6. Gate di [ADR-012], se applicabile.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-DRIFT-ANSWERED-FIRST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La domanda sul controllo di drift verso l'alto è risolta e riportata **prima** che una regola sia scritta, e la trascrizione mostra l'ordine. Non è una precondizione fra le altre: il suo esito decide se il minorante sia il rimedio più forte o il più caro, e una regola scritta prima della risposta sarebbe stata scelta senza sapere.
- [x] GATE-LEVER-MEASURED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Se il minorante è adottato, l'ampiezza della leva di partizione è **misurata**: la distanza massima che un set può spingere il minorante oltre il tempo reale prima che un blocco sia rifiutato. Una leva nominata e non misurata è una minaccia dichiarata a spanne, e questa nasce dalla capacità che [DEBT-013] ha già accertato a quello stesso attore.
- [x] GATE-BOOTSTRAP-UNCHANGED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Un nodo **senza** testa finalizzata si comporta esattamente come oggi, e una prova lo mostra. È il caso che la ragione 1 di `identity.md` protegge deliberatamente, ed è dove la circolarità rientrerebbe se la ricaduta fosse imperfetta.
- [x] GATE-ONE-CLOCK | kind=manual | owner=agent | phase=before-submit | evidence=transcript | L'orologio esterno usato è **lo stesso oggetto** che [SPEC-016] ha costruito, e la trascrizione lo mostra citando il codice condiviso. Due orologi per la stessa proprietà sono peggio di nessuno: nessuno saprebbe quale sia quello vero.
- [x] GATE-NO-EXCLUDED-REMEDY | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una ricerca sul diff mostra che nessuno dei quattro rimedi esclusi è stato adottato **come chiusura**: né il vincolo relazionale da solo, né una ritaratura di `S_max` o `D_max`, né tolleranza oltre `expires_at_ms`, né un contatore di invalidazione. Sono i quattro che sembrano ovvi, ed è la ragione per cui la gate li cerca.
- [x] GATE-RESIDUAL-DECLARED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La somma `D_max + S_max` **e il residuo illimitato** sono dichiarati accanto ai parametri, anche se il minorante è stato adottato e anche se lo chiude. Un lettore che vede solo la somma conclude che l'esposizione sia limitata: è la famiglia 2, la pretesa avanti rispetto alla regola, su un documento che parla di scadenze.
- [x] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Se una regola di validità è stata scritta, la passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato e la trascrizione allegata. Se **nessuna** regola è stata scritta, la gate è dichiarata non applicabile con la ragione.
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la chiusura e il Lead ha accettato la review. È lei ad aver stabilito che il minorante è fail-closed, ed è il Lead ad aver aggiunto che *fail-closed* qualifica la sicurezza e non la disponibilità: la review deve pronunciarsi su entrambe le affermazioni, comprese quelle del Lead.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è chiudere il termine sbagliato.** Il debito originale suggeriva il vincolo relazionale, il titolo originale nominava il termine minore, e il dispatch del Lead partiva da una premessa falsa. Tre indicazioni convergenti verso la grandezza sbagliata: se questa spec le segue, la famiglia 3 diventa la settima occorrenza.
- **Il rischio secondario è il rimedio che sposta il rischio invece di ridurlo.** Il minorante converte una leva di rallentamento in una leva di partizione, e la seconda è peggiore. È ammissibile solo se misurata.
- **Un rischio terzo, ed è di coordinamento.** [SPEC-016] è in corso sullo stesso oggetto. Se questa spec parte prima che quella sia atterrata, costruirà un secondo orologio senza accorgersene, perché il primo non sarà ancora nel codice da leggere.
- **`D_max` e `S_max` restano non fissati** da alcun documento di genesi. Non è compito di questa spec fissarli, e la loro taratura non ne cambia le conclusioni; ma il fatto va lasciato scritto e non chiuso in silenzio.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Fermarsi e riportare è l'esito previsto** se la domanda sul controllo di drift avesse una risposta che rende il minorante impraticabile: in quel caso la chiusura è la dichiarazione, ed è un esito ammissibile e non un fallimento.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.


### Passo 1 — La domanda sul controllo di drift, risolta prima di ogni regola

*Registrato prima che una riga di regola o di codice fosse scritta. Baseline
dell'albero al momento di questa registrazione: 177 test verdi.*

**La domanda.** Il controllo di drift verso l'alto su `timestamp_ms` è applicato
dai validatori onesti in accettazione del blocco, oppure no? E si applica ai
blocchi storici durante il sync?

**Come l'ho istruita.** Tre fonti indipendenti, due normative e una di codice.

1. `docs/protocol/ledger.md` §*Block format*: *«Timestamps MUST be greater than
   the median of the previous 11 finalized blocks and no more than the active
   maximum clock drift after the proposal is received.»* È un **MUST**, ed è
   scritto accanto alla regola della mediana degli undici, cioè fra i vincoli di
   validità del blocco.
2. `docs/protocol/README.md` §*Genesis constants*: *«The only temporal
   constraints on a block are that its `timestamp_ms` exceed the median of the
   previous eleven finalized blocks and not run ahead of **the receiver's clock**
   by more than the active maximum drift — monotonicity and a ceiling, not a
   step.»* La seconda fonte nomina esplicitamente **l'orologio del ricevente**
   come termine di paragone.
3. `core/coblox-core/src/params.rs:340-341`:
   `/// Maximum accepted clock drift for a received proposal.` —
   `pub max_clock_drift_ms: u64`, campo del documento firmato
   `consensus_parameters`. La documentazione del campo dice *«for a received
   proposal»*, non *«for a block»*.

**Risposta (a): sì, il controllo è applicato, e il termine di paragone è
l'orologio del validatore onesto che riceve la proposta.** Non è un controllo
locale opzionale: è un MUST di validità nel medesimo periodo che porta la regola
della mediana, ed è parametrato da un valore del documento firmato attivo. Ne
segue che **l'inflazione di `timestamp_ms` è limitata dall'orologio degli
onesti**, e la leva è piccola e misurabile.

**Risposta (b): no, non si applica ai blocchi storici durante il sync, e la
circolarità non rientra.** Tutte e tre le fonti legano il controllo al momento
della **ricezione della proposta** — *«after the proposal is received»*, *«for a
received proposal»* — e non allo stato finalizzato. La lettura opposta si
confuta da sé: un controllo riapplicato in sync contro l'orologio corrente
rifiuterebbe **ogni** blocco più vecchio di `max_clock_drift_ms`, cioè l'intera
storia a partire da genesi, su ogni nodo, sempre. Una regola con quella
conseguenza non è quella che il documento scrive. Il nodo con orologio molto
indietro **acquisisce la testa** senza ostacoli.

**Cosa sceglie questo esito.** Entrambe le risposte puntano nella stessa
direzione: la leva di partizione paventata dal Lead è **limitata**, e l'esito (A)
di AGENT-007 — un minorante su `now_ms` — resta il rimedio più forte. La
chiusura per sola dichiarazione (B+C) **non** è quindi l'esito, e la ragione è
questa e non un'altra.

**E qui interviene `GATE-ONE-CLOCK`, che cambia *quale* minorante.** Vedi il
passo 2: il minorante non si costruisce su `timestamp_ms`.

### Passo 2 — Il rimedio scelto, e perché non è quello che il debito descriveva

**Adottato: il minorante su `now_ms`, costruito sull'`issued_at_ms` del
checkpoint di soggettività debole — non sul `timestamp_ms` dell'ultimo blocco
finalizzato.**

`GATE-ONE-CLOCK` non è una formalità di coordinamento: **è ciò che sceglie la
sorgente del minorante**, e cambia l'esito (A) in meglio. L'orologio esterno che
[SPEC-016] ha costruito in `core/coblox-core/src/cadence.rs` è
`checkpoint_issued_at_ms`, e la documentazione di quel modulo lo dice come
divieto oltre che come definizione:

> *«What v0 does have is one clock the validators do not write: the **weak
> subjectivity checkpoint**, signed by a release key that belongs to no
> validator»* … *«**It is not reintroduced here by any door: `timestamp_ms` is
> not an input to any function in this module.**»*

Un minorante costruito su `timestamp_ms` sarebbe stato **un secondo orologio**, e
per giunta precisamente quello che `cadence.rs` dichiara di non usare. Sarebbe
passato per `GATE-ONE-CLOCK` solo a parole.

**Le tre conseguenze, e la seconda è quella che il Lead chiedeva.**

1. **`GATE-ONE-CLOCK` è soddisfatta strutturalmente, non per dichiarazione.** Il
   minorante vive in `cadence.rs`, accanto alle due misure che usano lo stesso
   campo, e `identity.rs` non può costruirne un altro: il valore passa per un
   tipo con campo privato e due soli costruttori.
2. **La leva di partizione del set di validatori è esattamente zero.**
   `timestamp_ms` non entra nel minorante. L'attore di [DEBT-013] — il terzo
   bloccante che scrive gli orologi della catena — non ha alcuna presa su
   `issued_at_ms`, che è firmato da una chiave di rilascio che *«belongs to no
   validator, and not to a node»* (`README.md` §*The network-release trust key*).
   La conversione «rallentamento → partizione» che il Lead teme **non avviene**.
3. **La ragione 1 di `identity.md` non è indebolita.** Il minorante da
   `timestamp_ms` avrebbe richiesto una testa finalizzata, cioè `ledger-sync`,
   cioè un'attestazione: la dipendenza che la ragione 1 evita, riammessa come
   opzionale. Il checkpoint è un **artefatto di rilascio fuori banda** che il
   nodo appena installato già possiede o ottiene fuori dal trasporto, quindi il
   minorante è disponibile **prima** della sincronizzazione e non dopo.

### GATE-LEVER-MEASURED — la leva, misurata due volte

Il minorante **è** stato adottato, quindi la gate si applica. La misura è doppia,
perché la domanda del Lead era posta sulla formulazione che non ho adottato e la
risposta su quella adottata è diversa.

**(1) La leva del set di validatori sul minorante adottato: esattamente zero, e
strutturalmente.** `timestamp_ms` non è un ingresso di `AttestationClock`, e non
può diventarlo per errore: il tipo ha un campo privato e due soli costruttori,
`local_only(local_clock_ms)` e
`with_checkpoint_floor(local_clock_ms, checkpoint_issued_at_ms)`. Nessuna
scrittura di un validatore raggiunge quel valore. Non è un limite su un
attaccante: è l'assenza dell'ingresso.

**(2) La leva che il Lead chiedeva di misurare, sulla formulazione da
`timestamp_ms` che ho rifiutato.** La misura si dà comunque, perché è la ragione
quantitativa per cui quella formulazione sarebbe stata ammissibile e resta
comunque peggiore.

> *La distanza massima che un set può spingere il minorante oltre il tempo reale
> prima che un blocco sia rifiutato è* `max_clock_drift_ms + ε`, *con* `ε` *la
> dispersione degli orologi onesti attorno al tempo reale.*

Derivazione, in tre passi ciascuno appoggiato a una fonte:

1. un blocco è finalizzato solo con un certificato di quorum, quindi il terzo
   bloccante di [DEBT-013] **non può finalizzare da solo**: almeno un validatore
   onesto deve accettare la proposta;
2. un validatore onesto rifiuta una proposta con
   `timestamp_ms > proprio orologio + max_clock_drift_ms` — è la risposta (a) del
   passo 1, `ledger.md` §*Block format* e `params.rs:340-341`;
3. quindi il massimo `timestamp_ms` finalizzabile è
   `max(orologio degli onesti firmatari) + max_clock_drift_ms`, cioè al più
   `tempo reale + ε + max_clock_drift_ms`.

**E la leva non è un cricchetto.** La mediana degli undici impone monotonia, non
accumulo: ogni blocco è ricontrollato contro un orologio onesto **fresco**,
quindi lo scarto è un soffitto e non si somma di blocco in blocco. Poiché anche
l'orologio del ricevente onesto dista al più `ε` dal tempo reale, il minorante
potrebbe eccedere la lettura del ricevente di al più `max_clock_drift_ms + 2ε`:
sarebbero disconnesse solo le attestazioni a cui resta **meno di quel tanto** di
validità, cioè quelle già in scadenza e da riemettere.

**Un fatto che va lasciato scritto e non chiuso in silenzio:
`max_clock_drift_ms` non è fissato da alcun documento di genesi.** L'unico valore
nell'albero è `1` ed è un input di test (`tests/common/mod.rs`,
`sim/tools/protocol_hashes.py`). La leva sopra è quindi misurata in funzione di
un parametro che nessuno ha ancora scelto, ed è un secondo motivo per non
appoggiarci la disponibilità del trasporto.

**(3) La leva di chi detiene la chiave di rilascio, sul minorante adottato.**
> **Corretto dalla remediation di REVIEW, RF-002: l'affermazione di sussunzione
> qui sotto era falsa come scritta, e vale solo dopo la separazione delle due
> metà della regola 5. Vedi la sezione RF-002 in fondo.** Va
nominata perché il minorante non è senza padrone. Il detentore della chiave di
rilascio **è già** l'ancora di fiducia totale del client: firma `height`,
`block_id`, `validator_set_hash` e le revoche, e un client *«MUST NOT learn a
trust key from a checkpoint, from a peer, or from any network source»*. Può già
far fallire in chiuso l'intera verifica di catena. La capacità aggiunta qui è
**strettamente sussunta** da quella che ha già, quindi l'ampiezza *marginale* è
zero. La misura non marginale, per completezza: gonfiare `issued_at_ms` di `Δ`
riduce di `Δ` il denominatore di `measure_cadence_from_checkpoint`, quindi lo
stesso client fallisce in chiuso con `FasterThanBand` non appena
`Δ > B·(r - min_ms_per_block) + max_external_clock_slack_ms`, con `B` i blocchi
misurati e `r` la cadenza reale. È un numero reale ma **dipendente dalla
finestra**, quindi cresce con `B`: non lo dichiaro come un limite, lo dichiaro
come ciò che è.

### GATE-BOOTSTRAP-UNCHANGED — trascrizione

La proprietà è resa **strutturale prima che asserita**: la forma di bootstrap è
`AttestationClock::local_only`, il cui `now_ms` è il solo orologio locale e il
cui `floor_ms()` è zero per costruzione. Non esiste un percorso in cui un nodo
senza checkpoint riceva un pavimento, perché l'unico altro costruttore richiede
un `checkpoint_issued_at_ms` che quel nodo non ha.

Le prove, e sono due indipendenti:

1. **L'intera gate preesistente `gate_no_attestation_rejected` è ora eseguita in
   forma `local_only`** — nove percorsi di rifiuto più i due bordi della
   tolleranza di skew, tutti e undici — e passa **invariata**. Undici asserzioni
   scritte prima di [SPEC-020] che continuano a valere è la definizione operativa
   di «si comporta esattamente come oggi».
2. **Un caso che riproduce il limite dichiarato invece di assumerlo**, in
   `the_external_clock_floor_reduces_the_term_no_parameter_bounds`: un ricevente
   con orologio a `1 500 000` mentre il tempo reale è `5 000 000` **accetta**
   un'attestazione scaduta a `2 000 000` quando non ha checkpoint
   (`floor_ms() == 0`), e la **rifiuta** con lo stesso orologio non appena ne ha
   uno. Il ricevente non è cambiato: è cambiato solo ciò che possiede.

```
$ cargo test --workspace
TOTAL PASSED: 179     (baseline prima di questa spec: 177)
0 righe FAILED / panicked
```

### GATE-ONE-CLOCK — trascrizione

L'orologio è lo stesso oggetto di [SPEC-016], e la condivisione è di **codice**,
non di intenzione: `AttestationClock` vive in
`core/coblox-core/src/cadence.rs`, lo stesso modulo di
`measure_cadence_from_checkpoint`, e prende lo stesso campo del medesimo oggetto
firmato.

```rust
// cadence.rs — la misura di [SPEC-016]
pub fn measure_cadence_from_checkpoint(
    chain_id: &ChainId,
    checkpoint_height: u64,
    checkpoint_issued_at_ms: u64,   // <-- l'orologio esterno
    ...

// cadence.rs — il pavimento di [SPEC-020], stesso modulo, stesso campo
pub const fn with_checkpoint_floor(local_clock_ms: u64, checkpoint_issued_at_ms: u64) -> Self
```

La documentazione di modulo che [SPEC-016] ha scritto è il vincolo che ha scelto
la sorgente, e resta vera parola per parola dopo questa spec:

> *«It is not reintroduced here by any door: `timestamp_ms` is not an input to
> any function in this module.»*

`identity.rs` **non può** costruire un secondo orologio: `verify` non prende più
un `u64` ma un `AttestationClock`, il cui campo è privato e i cui costruttori
sono i due sopra. Un secondo orologio non è vietato per convenzione: è
inesprimibile.

### GATE-NO-EXCLUDED-REMEDY — la ricerca sul diff

Nessuno dei quattro è nel diff, né come chiusura né altrimenti.

1. **Vincolo relazionale `S_max`-`D_max`.** Assente. Nessun blocco di validità
   dei `consensus_parameters` è toccato; `params.rs` non è modificato. La somma è
   **dichiarata** in `identity.md` punto 3, che è prosa e non una regola di
   rifiuto: non vincola nulla e non pretende di farlo.
2. **Ritaratura della tolleranza in genesi.** Assente. Nessun valore di
   `max_transport_attestation_validity_ms`,
   `max_transport_attestation_future_skew_ms` o `max_clock_drift_ms` è cambiato
   in alcun file; l'unica occorrenza numerica toccata è il `count` di una probe.
3. **Tolleranza oltre `expires_at_ms`.** Assente, e la direzione è asserita in
   negativo: il pavimento può solo **alzare** `now_ms`, e
   `verify(with_checkpoint_floor(behind_ms, 2_000_001))` è un rifiuto mentre
   `... expires_at_ms)` è un'accettazione. Il caso `stale_checkpoint` asserisce
   che un checkpoint più vecchio dell'orologio locale non resuscita nulla.
4. **Invalidazione anticipata con epoca o numero di serie.** Assente. Nessun
   campo è aggiunto a `TransportKeyAttestation`; la struct è invariata.

### GATE-RESIDUAL-DECLARED — e la correzione che ha imposto

La somma e il residuo sono in `identity.md` §*Bounded validity in time*, punto 3
per la somma e punto 5 per il residuo, entrambi **accanto ai parametri** e non in
fondo alla sezione. Entrambi sono pinnati da una probe C10.

**E qui la spec — e la valutazione di AGENT-007 — dicevano una cosa più forte del
vero, e l'ho corretta invece di trascriverla.** Sia il debito sia la spec parlano
del minorante come di ciò che «chiude il termine illimitato». Non lo chiude. Con
`b` il ritardo dell'orologio e `A` l'età vera del checkpoint, il terzo addendo
passa da `b` a `min(b, A)`; e `A` **non** è limitato da
`max_weak_subjectivity_age_ms`, perché il passo 1 dell'algoritmo light-client
calcola l'età come `orologio locale - issued_at_ms`, cioè **sullo stesso orologio
rotto**. Un ricevente indietro di `b` accetta un checkpoint di età vera fino a
`max_weak_subjectivity_age_ms + b`, e nel caso peggiore `A = b`, il pavimento
coincide con l'orologio locale e non si è guadagnato nulla.

Il termine resta quindi **illimitato da ogni regola del protocollo**, ed è
scritto così. Ciò che il pavimento cambia — e che vale la spec — è *da quale
grandezza il residuo dipende*: prima era l'errore dell'orologio, che il ricevente
non osserva e l'operatore non corregge senza un riferimento esterno; ora è al più
l'età di un artefatto che l'operatore ottiene fuori banda e rinfresca a piacere,
**senza possedere un orologio giusto**. Un residuo piccolo è ora *ottenibile*,
non *garantito*. Scrivere «chiude» sarebbe stata la famiglia 2 commessa dentro la
gate che esiste per impedirla.

### GATE-ADR012 — la passata, applicabile perché una regola è stata scritta

Una regola di validità **è** stata scritta (`identity.md` punto 5, il pavimento
su `now_ms`), quindi la gate si applica e non è dichiarata n/a.

```
$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     20 candidate(s) checked
  C4-VALUE          60 candidate(s) checked
  C5-MIRROR         53 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C5-DISCOVERED     67 candidate(s) checked
  C10-PROBE        148 candidate(s) checked      (prima: 146)
  C11-CLAIMDOC       8 candidate(s) checked
published-artifact inventory: PASS

$ python sim/tools/published_artifacts_negative.py
=== C10-PROBE, every probe individually ===
deleting each probe's own pinned passage from its own document, 148 case(s)
  every one of the 148 probes was observed failing
negative proof: PASS - 15 mutations across 11 defect classes, plus every probe
individually, each observed failing

$ python sim/tools/protocol_hashes.py
every published value reproduced: PASS

$ python sim/tools/threat_model_matrix_coherence.py
celle: 104  coperte: 97  n/a: 7  scenari: 43
OK: matrice e scenari coerenti

$ cd sim && python -m pytest tests -q
44 passed
```

**Conteggi delle probe, con la differenza spiegata.** 146 → 148. Due probe nuove,
entrambe su `identity.md`: `transport-attestation-clock-floor` pinna la riga del
pavimento **sul nome del campo** (`issued_at_ms`, non `timestamp_ms`), e
`transport-attestation-residual-is-declared` pinna la riga del residuo. Una probe
preesistente ha cambiato `count`, da 3 a 7:
`transport-attestation-skew-tolerance` conta le menzioni di
`max_transport_attestation_future_skew_ms` in `identity.md`, e ne ho aggiunte
quattro — la disuguaglianza della finestra accettata, la formula dell'ampiezza,
la somma nominata, e il blocco dei tre addendi. Il `why` della probe registra
quali sono e perché sono la chiusura e non prosa incidentale.

**Nessun valore pubblicato è cambiato, e la frase va scritta perché è il caso in
cui è più facile saltare la passata** ([SKILL-002]). Questa spec non introduce
preimmagini, non tocca alcuna fixture, non modifica `params.rs` né alcun
documento firmato: `protocol_hashes.py` riproduce ogni valore invariato, ed è
quella trascrizione a dimostrare che la passata è stata fatta.

### La prova in negativo, e la risposta al passo 4 di [SKILL-001]

Eseguita su **due copie** dell'albero nello scratchpad, mai sull'albero
condiviso. Verde iniziale verificato su ciascuna copia prima di ogni mutazione, e
verde riverificato dopo il ripristino.

**Mutazione 1 — si rimuove il pavimento** (`with_checkpoint_floor` restituisce
l'orologio locale: il comportamento di prima di [SPEC-020]).

```
test cadence::tests::the_bootstrap_form_applies_no_floor_and_the_floored_form_never_lowers ... FAILED
  panicked at core\coblox-core\src\cadence.rs:637: assertion `left == right` failed
    left: 1000
   right: 9000
test the_external_clock_floor_closes_the_term_no_parameter_bounds ... FAILED
  panicked at core\coblox-core\tests\conformance_registry.rs:523
```

**Mutazione 2 — il pavimento diventa una sostituzione** (`now_ms` è sempre
`checkpoint_issued_at_ms`), cioè il difetto in cui un checkpoint vecchio
resuscita un'attestazione scaduta.

```
test the_external_clock_floor_closes_the_term_no_parameter_bounds ... FAILED
  panicked at core\coblox-core\tests\conformance_registry.rs:534   <- il caso `stale_checkpoint`
test cadence::tests::the_bootstrap_form_applies_no_floor_and_the_floored_form_never_lowers ... FAILED
```

**Mutazione 3 — la sorgente del pavimento diventa `timestamp_ms`**, che è il
difetto che la probe esiste per impedire e **non** è la cancellazione della riga.

```
FAIL C10-PROBE: probe 'transport-attestation-clock-floor' expected 1 match(es) of
'now_ms = max\(local clock, checkpoint\.issued_at_ms\)' in identity.md, found 0.
```

**Mutazione 4 — si cancella la riga del residuo**, lasciando la somma sola.

```
FAIL C10-PROBE: probe 'transport-attestation-residual-is-declared' expected
1 match(es) of 'with a checkpoint; UNBOUNDED without one' in identity.md, found 0.
```

Ripristino e verde riverificato dopo ciascuna: `cargo test -p coblox-core` verde
su tutte le suite, `published_artifacts.py` `PASS` sulla copia dei documenti.

**Passo 4 — quale grandezza è costante in tutti i casi.**
> **Incompleto: la remediation di REVIEW (RF-006) ha trovato una seconda
> grandezza costante che questa risposta non nomina — quale metà della regola 5
> è esercitata. Vedi la sezione RF-006 in fondo.** Nella gate preesistente
`gate_no_attestation_rejected` la grandezza costante è **la forma dell'orologio**:
tutti e undici i casi — nove percorsi di rifiuto e i due bordi dello skew — usano
un orologio locale nudo. È corretto per ciò che quei casi testano, ed è
esattamente la ragione per cui il caso nuovo è **separato** e non aggiunto lì
dentro: `the_external_clock_floor_closes_the_term_no_parameter_bounds` varia la
forma dell'orologio tenendo fissa l'attestazione, cioè lo specchio della gate
esistente. La grandezza costante nel caso nuovo è a sua volta l'attestazione, e
resta costante deliberatamente: è la stessa attestazione valida a essere
accettata e rifiutata secondo cosa il ricevente possiede, ed è ciò che rende la
prova una prova sul ricevente.

## Implementation evidence

### Changes made

- **`core/coblox-core/src/cadence.rs`** — aggiunto `AttestationClock`, il
  pavimento su `now_ms`, nello stesso modulo dell'orologio esterno di
  [SPEC-016]. Campo privato, due costruttori (`local_only`,
  `with_checkpoint_floor`), due accessori (`now_ms`, `floor_ms`). Aggiornata la
  documentazione di modulo. Un test unitario nuovo.
- **`core/coblox-core/src/identity.rs`** — `TransportKeyAttestation::verify`
  prende `clock: AttestationClock` invece di `now_ms: u64`. Cambio breaking
  dell'API deliberato: è ciò che rende inesprimibile un secondo orologio.
- **`core/coblox-core/tests/conformance_registry.rs`** — i quattordici siti di
  chiamata passano alla forma `local_only`, il che rende l'intera gate
  preesistente anche la prova di bootstrap; un test nuovo per il pavimento, la
  sua direzione e i suoi bordi.
- **`docs/protocol/identity.md`** — la lista degli ingressi delle regole di
  rifiuto (rule 5 ne ha uno in più); la somma della finestra accettata e ciò che
  non limita, nel punto 3; il punto 5 nuovo con il pavimento, la sorgente, la
  direzione, il caso di bootstrap e il residuo; tre affermazioni preesistenti
  corrette perché erano rimaste indietro rispetto alla regola (punto 2,
  §*Anti-reuse property*, §*Authentication on a connection*).
- **`.lmbrain/knowledge/threat-model.md`** — TM-37 contromisura (a) riscritta.
- **`sim/tools/published_artifacts.toml`** — due probe nuove, un `count` alzato
  da 3 a 7 con la ragione.

### Files changed

- `core/coblox-core/src/cadence.rs`
- `core/coblox-core/src/identity.rs`
- `core/coblox-core/tests/conformance_registry.rs`
- `docs/protocol/identity.md`
- `.lmbrain/knowledge/threat-model.md`
- `sim/tools/published_artifacts.toml`
- `.lmbrain/specs/working/SPEC-020-l-orologio-su-cui-si-misura-la-scadenza-di-un-attestazione.md`
  (questa evidenza)

### Verification transcript

Le trascrizioni delle gate stanno nelle sezioni omonime qui sopra
(`GATE-BOOTSTRAP-UNCHANGED`, `GATE-ONE-CLOCK`, `GATE-ADR012`, e la prova in
negativo con le quattro mutazioni). Questa sezione raccoglie i comandi esatti e
l'output della passata finale sull'albero condiviso, dopo l'ultima modifica.

```
$ cargo fmt --all --check
FMT CLEAN

$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s

$ cargo test --workspace
TOTAL PASSED: 179
(baseline prima di questa spec: 177; nessuna riga FAILED, nessun panicked)

$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     20 candidate(s) checked
  C4-VALUE          60 candidate(s) checked
  C5-MIRROR         53 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C5-DISCOVERED     67 candidate(s) checked
  C10-PROBE        148 candidate(s) checked
  C11-CLAIMDOC       8 candidate(s) checked

published-artifact inventory: PASS

$ python sim/tools/published_artifacts_negative.py
=== C10-PROBE, every probe individually ===
deleting each probe's own pinned passage from its own document, 148 case(s)
  every one of the 148 probes was observed failing
negative proof: PASS - 15 mutations across 11 defect classes, plus every probe individually, each observed failing

$ python sim/tools/protocol_hashes.py
every published value reproduced: PASS

$ python sim/tools/threat_model_matrix_coherence.py
celle: 104  coperte: 97  n/a: 7  scenari: 43
OK: matrice e scenari coerenti

$ cd sim && python -m pytest tests -q
............................................                             [100%]
44 passed in 0.18s

$ git status --porcelain
 M .lmbrain/knowledge/threat-model.md
 D .lmbrain/specs/backlog/SPEC-020-l-orologio-su-cui-si-misura-la-scadenza-di-un-attestazione.md
 M core/coblox-core/src/cadence.rs
 M core/coblox-core/src/identity.rs
 M core/coblox-core/tests/conformance_registry.rs
 M docs/protocol/identity.md
 M sim/tools/published_artifacts.toml
?? .lmbrain/specs/working/SPEC-020-l-orologio-su-cui-si-misura-la-scadenza-di-un-attestazione.md
```

Nessun `git add`, `git commit` o `git push` è stato eseguito ([SKILL-003]). Le
mutazioni della prova in negativo sono state eseguite su due copie dell'albero
nello scratchpad e mai sull'albero condiviso.

## Remediation di REVIEW — 2026-08-26

### RF-001, passo zero — su quale orologio il passo 1 valuta la freschezza

*Registrato prima di ogni riga di codice della remediation, come RF-001 impone.*

**Il finding è confermato, ed è meccanico.** `light_client.rs:68-70` fallisce con
*«checkpoint issued in the future»* quando `issued_at_ms > now_ms`;
`cadence.rs` alza `now_ms` **solo** quando `checkpoint_issued_at_ms >
local_clock_ms`. Le due condizioni sono l'una la negazione dell'altra: se il
passo 1 fosse valutato sull'orologio locale nudo e il suo verdetto governasse
l'uso del checkpoint, **ogni checkpoint capace di produrre un pavimento non nullo
sarebbe già stato rifiutato**, e il rimedio sarebbe inerte. Non l'avevo visto, e
il caso di prova non poteva vederlo perché costruisce l'`AttestationClock`
direttamente senza passare dal passo 1.

**La risposta alla domanda, in due parti.**

**(1) Il passo 1 valuta la freschezza sull'orologio locale nudo, e non può fare
altrimenti.** Non per scelta: se lo valutasse sull'orologio pavimentato l'età
diventerebbe `max(0, locale - issued_at)`, quindi **ogni** checkpoint datato
avanti leggerebbe età zero, cioè freschezza perfetta, sempre. Il pavimento
renderebbe vacua la guardia che avrebbe dovuto proteggerlo. Il passo 1 resta
dunque sull'orologio locale, ed è ora scritto come regola invece che dedotto.

**(2) E proprio per questo il pavimento non è, e non deve essere, subordinato al
verdetto del passo 1.** La deroga che avevo commesso tacitamente non era «il
passo 1 accetta i checkpoint dal futuro» — sarebbe stata indifendibile. Era una
citazione sbagliata: il punto 5 nominava il passo 1 come precondizione del
pavimento, importandone la freschezza, che il pavimento non richiede.

**La ragione, ed è la sostanza di questa remediation: la freschezza è una
precondizione per *ancorare lo stato di catena*, non per *minorare il tempo
reale*.** Un minorante ha bisogno di una sola proprietà — che `issued_at_ms` sia
un istante realmente trascorso — e quella discende dalla **firma**, non dall'età.
Un checkpoint vecchio è un pavimento **più debole**, mai un pavimento **non
sicuro**: minora di meno. Un checkpoint stantio è invece inutilizzabile come
ancora di catena, perché lì la domanda è *cosa è successo dopo*, e a quella l'età
risponde. Sono due domande diverse sullo stesso artefatto, e solo la seconda ha
bisogno che sia recente.

**L'aggravante di AGENT-007 è vera e si spiega, e la spiegazione è verificabile e
non una scusa.** Delle tre funzioni che consumano `issued_at_ms` contro un
orologio locale, due lo trattano come errore — `ClockRegression` in
`measure_cadence_from_checkpoint` e *«checkpoint issued in the future»* nel passo
1. Entrambe calcolano una **differenza**, `now - issued_at`, e una differenza
negativa non ha significato: sono misure di durata. La terza calcola un
**massimo**, e per un massimo «il checkpoint è avanti al mio orologio» non è un
errore: è **l'unico caso in cui la funzione fa qualcosa**. La discriminante non è
la mia preferenza, è l'operazione — sottrazione contro massimo — ed è
ispezionabile in tre righe.

**La conseguenza, che va scritta e non nascosta.** Un nodo può ora trovarsi con
un checkpoint **autentico ma rifiutato dal passo 1**, che usa come pavimento sul
trasporto mentre fallisce in chiuso sulla verifica dei saldi. Non è
un'incoerenza: è il nodo con l'orologio molto indietro, che non deve fidarsi
dello stato di catena — non sa distinguere «stantio» da «io sono indietro» — e
che deve comunque ottenere un giudizio onesto sulla scadenza di un'attestazione.
Le due fiducie hanno costi diversi e ora hanno regole diverse.

**Non ho alzato alcuna soglia, e non ho toccato il passo 1.** `light_client.rs`
mantiene lo stesso comportamento: la modifica è che il pavimento non lo cita più
come propria precondizione, e che l'ordinamento fra i due è scritto in entrambi i
documenti invece di essere lasciato dedurre.

### RF-001 — high — chiuso

Il rimedio è: **il pavimento non è subordinato al verdetto del passo 1, ed è ora
scritto come regola in entrambe le direzioni.** Il passo 1 non è toccato:
`light_client.rs` ha lo stesso comportamento riga per riga.

- `identity.md` punto 5 non cita più il passo 1 come precondizione. La
  precondizione è esattamente due requisiti — firma verificata sotto una chiave
  già posseduta, `chain_id` uguale a quello configurato — e il documento dice
  ora **esplicitamente** che la freschezza non è fra essi, con la ragione: *«la
  freschezza è precondizione per ancorare lo stato di catena, non per minorare il
  tempo reale»*.
- La conseguenza è scritta e non nascosta: un nodo può tenere un checkpoint
  **autentico ma rifiutato dal passo 1**, fallire in chiuso sui saldi, e usare lo
  stesso artefatto come pavimento sul trasporto.
- La stessa regola è nella documentazione di `with_checkpoint_floor`, con la
  distinzione fra le tre funzioni che consumano `issued_at_ms`: due calcolano una
  **differenza** (durata negativa priva di senso → errore), la terza un
  **massimo** (essere avanti all'orologio locale è il solo caso in cui la
  funzione fa qualcosa).
- Pinnato dalla probe `transport-attestation-floor-not-gated-on-step-1`, e la
  ragione della probe è che **l'assenza di questa riga è invisibile**: nulla
  fallisce, il pavimento semplicemente non scatta mai.

L'analisi del residuo che invocava il tetto del passo 1 è stata riscritta: **nulla
limita `A`**, e la ragione non è più «il passo 1 misura sull'orologio rotto» — che
era la mia derivazione sbagliata — ma «quel tetto appartiene al passo 1, e il
pavimento non vi è subordinato».

**Le due richieste testuali di RF-001, entrambe soddisfatte alla lettera.** La
derivazione `A <= W + b` che poggiava sul passo 1 è stata **tolta**; e il punto 5
dichiara ora esplicitamente che **il valore di `issued_at_ms` accettato come
pavimento non è limitato verso l'alto da alcuna regola** — nessun controllo lo
confronta con l'orologio locale, che è il punto, e nessuno lo confronta con
altro. Ciò che impedisce a quel fatto di essere una superficie non è un limite ma
la separazione di RF-002: un'ancora arbitrariamente avanti può solo far scadere,
mai far accettare.

**E la composizione è ora asserita, non ragionata.** Il test che RF-001 abbozza è
in albero come `step_one_and_the_attestation_floor_do_not_compose`
(`tests/light_client_perimeter.rs`), nella stessa forma a due cicli della review:
ogni ancora avanti all'orologio locale ha pavimento vivo e viene rifiutata dal
passo 1 **a ogni finestra**, `u64::MAX` compresa; e ogni ancora che il passo 1
può accettare lascia il pavimento a **zero**. È il test che fallisce il giorno in
cui qualcuno «aggiusta» il pavimento subordinandolo al passo 1 — mossa che
altrimenti non romperebbe nulla e lo renderebbe semplicemente inerte per sempre.

### RF-002 — medium — chiuso, e la sussunzione era falsa

Accolto senza riserve. **La regola 5 è ora divisa nelle sue due metà, che leggono
due orologi diversi:**

```
anchored_now_ms > expires_at_ms                                  // pavimentato
created_at_ms   > local_clock_ms + max_..._future_skew_ms         // orologio nudo
```

`AttestationClock` espone ora `local_clock_ms()` accanto a `now_ms()`, e
`identity.rs` legge l'uno e l'altro nella metà giusta. Per un ricevente senza
checkpoint i due numeri coincidono, quindi il comportamento di bootstrap è
invariato una seconda volta.

**La mia affermazione di sussunzione era falsa e la ritiro.** Le capacità
preesistenti della chiave di rilascio sono di **diniego** e di **ancoraggio**;
quella che il pavimento non separato le avrebbe aggiunto è di **ammissione sul
trasporto**, e il diniego non sussume l'ammissione. La sussunzione diventa vera
**solo dopo la separazione**, e va letta come conseguenza del rimedio e non come
premessa: con il pavimento speso solo dove rifiuta, la capacità marginale torna a
essere di diniego, che quella chiave già possiede potendo semplicemente non
emettere un checkpoint.

Pinnato dalla probe `transport-attestation-admission-half-is-unfloored`, con
`count = 2` perché entrambe le occorrenze portano carico: la regola di rifiuto, e
la riscrittura dentro il punto 5 che dice che un'implementazione che valuta questa
metà contro il valore pavimentato **non implementa la regola**.

### RF-006 / [SKILL-001] passo 4 — la cella vuota, riempita

La grandezza costante che non avevo visto è **quale metà della regola 5 è
esercitata**: in tutti e dodici i casi `created_at_ms` era nel passato, quindi
tutti esercitavano la metà della scadenza. La cella «pavimento non nullo × metà
dell'ammissione» era vuota, ed è dove viveva RF-002.

Il caso nuovo `the_floor_is_spent_only_where_it_rejects` la riempie, e la
riempie **dimostrando l'ammissione invece di descriverla**: asserisce che un
ricevente il cui orologio legga davvero il valore dell'ancora **accetta**
l'attestazione postdatata — quindi l'accettazione esiste e non è ipotetica — e
che il ricevente con orologio esatto e ancora avanti la **rifiuta**, perché la
metà dell'ammissione non legge mai il valore pavimentato. Nello stesso test la
metà della scadenza continua a spendere il pavimento, sulla stessa chiamata.

La nota di [SKILL-001] passo 4 è ora scritta **in entrambi i casi di prova**, e
ciascuno nomina la grandezza che l'altro varia.

### RF-003 — low — chiuso

«Chiude» / «closes» è stato tolto dai tre luoghi che nessuna gate legge:

- `identity.rs`, documentazione di `verify`: ora *«the third is **reduced** — not
  closed — by a floor»*, e la riscrittura porta anche la ragione delle due metà.
- le due `why` di probe: *«the floor that **reduces** it — to the age of the
  checkpoint held, which no rule bounds»*, e nella seconda l'aggiunta esplicita
  che il pavimento riduce e non chiude.
- il test si chiamava `..._closes_the_term_...` e ora si chiama
  `..._reduces_the_term_no_parameter_bounds`; il suo titolo di documentazione
  diceva *«the receiver that closes it»* e ora dice *«what an anchor does to
  it»*.

Ho anche corretto un quarto luogo che la review non nomina e che era **sbagliato
e non solo generoso**: la documentazione di modulo di `cadence.rs` motivava il
residuo con «il controllo di freschezza è calcolato sull'orologio in ritardo»,
che è la derivazione che RF-001 ha demolito. Ora dice la ragione vera.

### RF-004 — low — chiuso

Delle tre correzioni, quella esatta (§*Anti-reuse property*) è stata resa più
precisa e le due generose sono state riscritte:

- punto 2: *«Point 5 **reduces** that offset … e **reduces è tutta la
  pretesa**»*, con il residuo nominato lì.
- punto 3: *«puts a floor under that third term …, which **reduces it without
  bounding it**»*.
- §*Anti-reuse property*: *«is **limited** only by the window … not the declared
  duration alone, **and not a bounded quantity**»*.

E l'elenco di §*Authentication on a connection*, che dopo RF-002 era diventato
falso in un modo nuovo — diceva che `now_ms` è il valore pavimentato per
**entrambe** le metà — ora nomina le due metà contro i due orologi.

### RF-005 — low — parzialmente curato dal rimedio di RF-002, residuo riportato

La review lo classifica debito e non blocco, e sono d'accordo; ma il rimedio di
RF-002 lo cura per metà e va detto quale metà.

**Prima:** `max` è simmetrico, quindi
`with_checkpoint_floor(a, b)` e `with_checkpoint_floor(b, a)` producevano lo
stesso `now_ms` e lo scambio era **invisibile nel comportamento** — il tipo era
convenzione, come la review dice e come il mio rapporto precedente non diceva.

**Ora:** `local_clock_ms` è letto separatamente dalla metà dell'ammissione,
quindi lo scambio **cambia il verdetto**. Il test lo asserisce su tutti e tre i
livelli — `now_ms()` identico, `local_clock_ms()` diverso, e il `verify` che
distingue — e la mutazione 6 qui sotto lo mostra fallire.

**Il residuo che resta, e lo riporto invece di correggerlo** ([SKILL-003]):
entrambi gli argomenti sono `u64` nudi, quindi lo scambio resta esprimibile e
solo un test lo cattura. La correzione vera sarebbe due tipi distinti
(`LocalClockMs`, `IssuedAtMs`), che è un cambiamento di API oltre lo scopo di
questa spec e merita la propria gate. Materiale per un debito, a giudizio del
Lead.

### Le mutazioni della remediation

Eseguite su due copie fresche nello scratchpad, verde iniziale verificato su
ciascuna, verde riverificato dopo ogni ripristino.

**Mutazione 5 — la metà dell'ammissione è ricondotta al valore pavimentato**
(esattamente il difetto di RF-002).

```
test the_floor_is_spent_only_where_it_rejects ... FAILED
  panicked at core\coblox-core\tests\conformance_registry.rs:673
test result: FAILED. 25 passed; 1 failed
```

**Mutazione 6 — i due argomenti di `with_checkpoint_floor` sono scambiati**, cioè
il difetto che prima di questa remediation era invisibile.

```
test the_floor_is_spent_only_where_it_rejects ... FAILED
  assertion `left == right` failed
    left: 1100000
   right: 1000000
```

**Mutazione 7 — la regola di RF-001 è rovesciata nel documento** («The floor
requires a checkpoint accepted by step 1»).

```
FAIL C10-PROBE: probe 'transport-attestation-floor-not-gated-on-step-1'
expected 1 match(es) of 'The floor does not require a fresh checkpoint' in
identity.md, found 0.
```

**Mutazione 8 — la metà dell'ammissione è ripuntata sull'orologio pavimentato nel
documento.**

```
FAIL C10-PROBE: probe 'transport-attestation-admission-half-is-unfloored'
expected 2 match(es) of
'created_at_ms > local_clock_ms \+ max_transport_attestation_future_skew_ms'
in identity.md, found 0.
```

Ripristino e verde riverificato dopo ciascuna.

### Le misure e i conteggi, aggiornati

**La correzione alla misura (3) di `GATE-LEVER-MEASURED`.** La riga che diceva
«la capacità aggiunta è **strettamente sussunta**, quindi l'ampiezza marginale è
zero» era **falsa come scritta** e resta valida solo dopo la separazione delle due
metà. La forma corretta: prima della separazione la capacità aggiunta era di
**ammissione** (`D_max + Δ`, con `Δ` scelto da chi firma) e non era sussunta da
nulla; dopo la separazione è di **diniego** ed è sussunta. Le misure (1) — leva
del set pari a zero per assenza dell'ingresso — e (2) —
`max_clock_drift_ms + ε` sulla formulazione rifiutata — non sono toccate da alcun
finding e restano come scritte.

**Conteggi.** Test `cargo test --workspace`: **179 → 181** (baseline della spec:
177). Due casi nuovi: `the_floor_is_spent_only_where_it_rejects` (RF-002/RF-006)
e `step_one_and_the_attestation_floor_do_not_compose` (RF-001). Probe C10: **148 → 150**, due nuove
(`transport-attestation-admission-half-is-unfloored` con `count = 2`,
`transport-attestation-floor-not-gated-on-step-1`), più
`transport-attestation-skew-tolerance` da `count = 7` a `8` per la menzione del
parametro nella metà dell'ammissione, con la ragione registrata nel `why`.

```
$ cargo fmt --all --check
FMT CLEAN
$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
$ cargo test --workspace
TOTAL: 181
$ python sim/tools/published_artifacts.py
  C10-PROBE        150 candidate(s) checked
published-artifact inventory: PASS
$ python sim/tools/published_artifacts_negative.py
negative proof: PASS - 15 mutations across 11 defect classes, plus every probe individually, each observed failing
$ python sim/tools/protocol_hashes.py
every published value reproduced: PASS
$ python sim/tools/threat_model_matrix_coherence.py
OK: matrice e scenari coerenti
$ cd sim && python -m pytest tests -q
44 passed in 0.17s
```

Nessun valore pubblicato è cambiato nella remediation, per la stessa ragione di
prima: nessuna preimmagine, nessuna fixture, nessun parametro.

### La correzione di AGENT-007 su sé stessa, scritta nel documento

L'accertamento *«il minorante è fail-closed»* era vero della sola metà della
scadenza. Il documento porta ora la forma corretta in tre luoghi, e in ciascuno
la porta come **ragione della regola** e non come nota:

- `identity.md` punto 5: *«A floor is not fail-closed; it is fail-closed on one
  half of rule 5 and fail-open on the other»*, seguito dalla misura `D_max + Δ`;
- `cadence.rs`, `local_clock_ms`: la stessa frase, con il motivo per cui
  l'accessore esiste;
- `identity.rs`, il commento sulla comparazione: *«both halves are fail-closed
  only once they are split»*;
- TM-37 contromisura (a): *«Un pavimento non è fail-closed, ed è la correzione che
  questa passata porta a un accertamento precedente.»*

Il debito è del Lead e non l'ho toccato.

### Cosa non ho fatto, in questa remediation

- **Non ho toccato il comportamento di `light_client.rs`.** RF-001 avvertiva di non aggirare il
  finding alzando una soglia; la soglia del passo 1 è esattamente dove era: l'unica aggiunta a quel file è un test, e `checkpoint_is_fresh` è invariata riga per riga.
- **Non ho corretto RF-005 alla radice** (due tipi distinti al posto di due
  `u64`): è un cambiamento di API oltre lo scopo, riportato sopra.
- **Nessun `git add`, `commit` o `push`.**
