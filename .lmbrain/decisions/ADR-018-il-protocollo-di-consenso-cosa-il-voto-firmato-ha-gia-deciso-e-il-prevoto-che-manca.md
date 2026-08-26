---
id: ADR-018
# Note: Quote the title if it contains a colon
title: "Il protocollo di consenso: cosa il voto firmato ha gia' deciso, e il prevoto che manca"
status: proposed
decision_date: 2026-08-27
decider: OPERATOR
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: [ADR-001, ADR-013, ADR-012]
tags: [consensus, security]
created: 2026-08-27
updated: 2026-08-27
activity:
  - date: 2026-08-27
    action: "created"
---
# Il protocollo di consenso: cosa il voto firmato ha già deciso, e il prevoto che manca

> Proposta dal Lead il 2026-08-27. **Non ancora decisa.**
>
> È la decisione che sblocca l'esito di [M-02] — *«una devnet di validatori seed raggiunge consenso BFT»* — e non era mai stata presa.

## Context

Il progetto sa **verificare** un quorum e non sa **raggiungerlo**. `coblox-core` ha sedicimila righe di regole: firme, hash, Merkle, set di validatori, elezione, autorizzazione. Non c'è nulla che faccia accordare due processi su un blocco, perché **il protocollo che lo farebbe non è mai stato specificato**.

[ADR-001] dice che il ledger è mantenuto da *«una federazione BFT»* di venti-cento validatori. Nomina la famiglia e non l'algoritmo.

### Cosa il protocollo ha già fissato, e che vincola questa scelta

Verificato sull'albero, non ricordato. **Sono vincoli, non preferenze**: qualunque algoritmo si scelga deve entrarci, o va dichiarato cosa si rompe.

1. **Un blocco porta il proprio certificato di quorum.** `Block = { header, transactions, quorum_certificate }`, e il `QuorumCertificate` nomina `block_id`, che è l'hash dell'header. **Un blocco sul filo è quindi un artefatto già finalizzato**, non una proposta. Questo esclude lo stile in cui un blocco porta il certificato del *genitore*.
2. **Esiste esattamente un dominio di firma per i voti**, `coblox-block-vote-v0`, su **quaranta** domini in tutto. Enumerati e contati su `docs/protocol/` e su `hash.rs`. Il voto firma `chain_id || height || round || block_id`.
3. **Il predicato di quorum è stretto e unico**: `signed_power * 3 > total_power * 2`, in `u128` con controllo di overflow, per **ogni** quorum di v0. Non `>=`, non una frazione arrotondata, non un conteggio di validatori.
4. **I round esistono già**, sia nel `BlockHeader` sia nella preimmagine del voto. Il protocollo prevede quindi che un'altezza possa essere tentata più volte.
5. **Nessuna aggregazione di firme in v0**: solo Ed25519, e il documento lo dichiara. Un certificato a cento validatori pesa 6,4 KB di firme.
6. **`wire.md` non ha alcun messaggio di consenso.** Il catalogo ha enrollment, sfida, sync del ledger, prove di saldo, annuncio di blocco, annuncio di evidenza. **Zero occorrenze** di proposta, voto, prevoto, precommit. Nulla porta un voto da un validatore all'altro.
7. **Nessuna regola dice chi propone.** La parola *proposer* compare in tutto il documento come **soggetto di cui si ragiona** — cosa può macinare, cosa può ordinare — e mai come ruolo assegnato da una regola.
8. **Nessun timeout di consenso è specificato.**

### Il crux, e discende dai punti 2, 3 e 4

**Un solo tipo di voto non basta a un protocollo che sia insieme sicuro e vivo.**

Con un solo voto firmato, la sicurezza attraverso i round si ottiene in un modo solo: **votare al più una volta per altezza**. Funziona — due insiemi disgiunti che superino entrambi i due terzi non esistono, quindi due blocchi diversi non possono finalizzare alla stessa altezza. Ma **uccide la vivacità**: se un round fallisce dopo che alcuni hanno votato, quei validatori non possono più votare a quell'altezza, il quorum non è più raggiungibile, e **l'altezza resta bloccata per sempre**. Basta che un proponente taccia dopo aver raccolto qualche voto.

Perché un'altezza sopravviva a un round fallito, un validatore deve poter votare **di nuovo** in un round successivo. E perché ciò resti sicuro serve una regola di **blocco**: *«sono vincolato a questo blocco finché non vedo che una maggioranza non lo era»*. Una regola di blocco richiede che il vincolo sia **dimostrabile agli altri**, cioè firmato.

> **Ne segue che `coblox-block-vote-v0` è, senza saperlo, un *precommit*. Ciò che manca è la prima fase.**

Questa è la lettura che il Lead propone, ed è il cuore della decisione: non *quale famiglia di algoritmi*, ma **riconoscere che la forma già scritta ne ammette una sola con un'aggiunta, e nessuna senza**.

## Decision

> **Da decidere dall'operatore.** Quanto segue è la proposta del Lead.

**Consenso a due fasi in stile Tendermint, con un dominio di firma aggiunto e nulla di esistente cambiato.**

### 1. La seconda fase esiste già: si aggiunge la prima

Nasce `coblox-block-prevote-v0`, con la stessa forma di preimmagine del voto esistente:

```text
"coblox-block-prevote-v0\0" || chain_id_32 || u64be(height)
|| u64be(round) || raw_32_bytes(block_id)
```

`coblox-block-vote-v0` **resta invariato** e diventa esplicitamente il **precommit**. Il `QuorumCertificate` allegato al blocco **resta l'insieme dei precommit** e non cambia forma. **Nessun artefatto pubblicato cambia**: si aggiunge un dominio e un messaggio, non si modifica nulla.

### 2. La regola di blocco, che è dove questi protocolli si sbagliano

- Un validatore che vede **oltre due terzi di prevoti** per un blocco al round `r` si **blocca** su quel blocco e invia il precommit.
- Un validatore bloccato su un blocco **prevota quel blocco** nei round successivi, **a meno che** non veda oltre due terzi di prevoti per un blocco diverso a un round **maggiore** di quello del proprio blocco — nel qual caso si sblocca e si rialllinea.
- Un validatore **non invia mai due precommit diversi allo stesso round**.

**È qui che va speso il grosso della verifica.** La sicurezza dell'intero ledger poggia su queste tre righe, e la loro forma corretta è nota da anni: vanno prese dalla letteratura e provate contro di essa, **non derivate da capo**.

### 3. Chi propone

Round-robin deterministico sul set di validatori attivo, indicizzato da `(height, round)` e pesato per potere di voto. Ogni nodo lo calcola dallo stesso `validator_set_hash` e ottiene lo stesso proponente senza scambiare messaggi.

**Va deciso guardando [DEBT-035]**, che ha già stabilito che il revocante può macinare il proprio ID di transazione: se il proponente dipendesse da qualcosa che un partecipante sceglie, sarebbe la stessa superficie a un livello più caro. **L'indice `(height, round)` non è scelto da nessuno**, ed è la ragione per cui è quello proposto.

### 4. I messaggi che nascono in `wire.md`

Tre, e nessuno esiste oggi: **proposta di blocco**, **prevoto**, **precommit**. Viaggiano sulla busta firmata già specificata e nel catalogo già esistente.

### 5. I timeout

Nascono `propose_timeout_ms`, `prevote_timeout_ms`, `precommit_timeout_ms`, con crescita per round. Sono i parametri della **vivacità**, e questa decisione **non fissa i valori**.

**Vanno trattati secondo il criterio di [predicato-di-accettazione]**, ed è la ragione per cui questo punto è nominato qui invece che lasciato all'implementazione: sono grandezze **locali a ciascun nodo**, non portate da alcun documento firmato, quindi **nessuna regola di validità potrà mai confrontarle**. Vanno dichiarati tali dall'inizio, non scoperti tali fra tre passate di review.

## Alternatives considered

- **Una fase sola, votando al più una volta per altezza.** È ciò che il protocollo scritto oggi letteralmente permette, e va nominata perché è la strada che si imbocca senza accorgersene. **Sicura e non viva**: un proponente che tace dopo aver raccolto qualche voto blocca quell'altezza per sempre. Rifiutata, e vale la pena dire che **non è un compromesso accettabile nemmeno per una devnet**, perché il modo in cui fallisce — uno stallo permanente da un guasto singolo — è indistinguibile da un difetto di implementazione, e si passerebbero settimane a cercarlo nel posto sbagliato.
- **Stile HotStuff con blocchi concatenati.** Comunicazione lineare invece che quadratica, cambio di vista più semplice. Rifiutata per due ragioni: **richiede che un blocco porti il certificato del genitore**, e il nostro porta il proprio, quindi cambierebbe `Block` e con esso ogni fixture e ogni digest pubblicato; e il suo vantaggio è la linearità, che paga con **l'aggregazione delle firme**, che v0 dichiara di non avere. A cento validatori il quadratico è diecimila messaggi per round, che è molto e sostenibile.
- **Adottare una libreria BFT esistente e adattarla.** Non valutata in questa sede e **non esclusa**: è una scelta di implementazione che la spec attuativa può portare, e a quel punto la parte 2 di questa decisione diventa un criterio di conformità invece che codice da scrivere. Va detto perché tacerlo lascerebbe credere che l'unica strada sia scriverlo a mano.

## Consequences

- **Nulla di pubblicato cambia.** Si aggiunge un dominio di firma, tre messaggi, una regola di proposta e tre parametri di timeout. Il `BlockHeader`, il `QuorumCertificate`, il predicato di quorum e la preimmagine del voto esistente **restano identici**. La gate di [ADR-012] si applica comunque alla spec che scrive tutto questo, perché è contenuto normativo nuovo.
- **La superficie di sicurezza più grande del progetto nasce qui.** Finora il rischio stava nelle regole; da qui in avanti sta in un protocollo distribuito con stati, timeout e avversari che tacciono. La revisione di sicurezza su questa attuazione **non è una gate fra le altre**.
- **I tre timeout entrano nella classe di [DEBT-036]** con una differenza: non sono governati da alcun documento firmato, quindi non appartengono né alla lista DRAFT né al blocco dei vincoli. **Vanno dichiarati come parametri locali**, ed è una terza specie che quel debito non contempla ancora.
- **`round` smette di essere un campo inerte.** Oggi sta nel `BlockHeader` e nella preimmagine del voto senza che alcuna regola lo produca. Da qui in poi lo produce il protocollo, e il suo valore nel blocco finalizzato dice a quale tentativo quell'altezza è riuscita.
- **La devnet diventa scrivibile**, e con essa gli altri due esiti di [M-02]: il light client verifica contro una catena che esiste, mint & burn sono transazioni che qualcuno finalizza.
- **Resta fuori, e va detto:** persistenza, rete, scoperta dei peer, e il ciclo di vita di un nodo che sopravvive a un riavvio. Sono lavoro reale e indipendente da questa decisione — `wire.md` li specifica già e **non ne è implementato niente**.

## Review conditions

Rivedere **la regola di blocco** se e solo se una verifica formale o un test di partizione ne mostrasse un caso non coperto. Non rivederla per semplificarla: è la parte del protocollo dove la semplificazione ha storicamente prodotto perdite di sicurezza.

Rivedere **la scelta del proponente** se emergesse che il round-robin pesato dà a un validatore una frequenza di proposta che si compone con la macinatura del beacon di [DEBT-038].

Rivedere **la forma quadratica** quando il set superasse le dimensioni che [ADR-001] dichiara, o se v0 acquisisse l'aggregazione delle firme. Nessuna delle due è prevista.

**Non rivedere** la scelta delle due fasi per ridurre i messaggi. La prima fase non è un costo: è ciò che permette a un'altezza di sopravvivere a un round fallito, e toglierla riporta esattamente allo stallo permanente descritto fra le alternative.
