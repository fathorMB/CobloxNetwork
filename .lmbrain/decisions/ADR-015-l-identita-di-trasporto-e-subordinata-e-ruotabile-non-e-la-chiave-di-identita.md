---
id: ADR-015
# Note: Quote the title if it contains a colon
title: "L'identita di trasporto e subordinata e ruotabile, non e la chiave di identita"
status: proposed
decision_date: 2026-08-25
decider: AGENT-LEAD
supersedes: []
superseded_by: []
links: [ADR-007, ADR-012, ADR-014]
tags: [architecture, security, privacy]
---
# L'identita di trasporto e subordinata e ruotabile, non e la chiave di identita

> Decisa dall'operatore il 2026-08-25, scegliendo la piu costosa delle opzioni
> offerte. Affronta TM-28 di [SPEC-004], severita alta, aperta e finora senza
> alcun ADR ne debito alle spalle. **Supera una regola gia accettata**:
> `identity.md` §*Key hierarchy* impone oggi la chiave unica.

## Context

`identity.md` §*Key hierarchy* dice che la stessa chiave Ed25519 e importata in
libp2p, che il Peer ID e il `node_id` sono quindi due derivazioni indipendenti
della stessa chiave pubblica, e che chi accetta un certificato **deve**
verificare entrambe le derivazioni. La regola non e arbitraria: compra una
proprieta di sicurezza precisa, scritta nella stessa riga — *un peer non puo
sostituire un'identita di trasporto dopo l'enrollment*.

La proprieta e giusta. Il modo in cui e ottenuta ha un costo che nessun documento
del progetto aveva dichiarato: **il legame fra l'identita di ledger e l'indirizzo
di rete diventa un fatto di dominio pubblico e permanente.** Il certificato di
enrollment porta `libp2p_peer_id` accanto a `node_id`, entrambi su un registro
immutabile. Chi legge il ledger — senza connettersi a nulla, senza enrollarsi,
anche a distanza di anni — ha la coppia gia fatta.

**Cio che la separazione chiude, e cio che non chiude.** Questa distinzione e la
sostanza della decisione, e la prima formulazione che il Lead aveva proposto
all'operatore era piu forte del vero: la separazione **non** chiude TM-28 alla
radice.

- **Chiude l'osservatore passivo e fuori sessione.** Sparisce il legame
  pubblicato: il lettore del ledger, il partecipante alla DHT, il nodo di
  scoperta e chiunque osservi il traffico senza tenere una sessione Coblox
  smettono di ottenere `node_id` da un indirizzo. E il perimetro piu ampio, ed e
  l'unico che oggi non costa nulla all'avversario.
- **Non chiude il peer con cui parli.** `identity.md` §*Authentication on a
  connection* impone al ricevente di ottenere il certificato prima che un peer
  possa pubblicare gossip o aprire stream protetti, e ogni envelope firmato
  porta `sender_node_id` in chiaro perche `wire.md` vieta la modalita autore
  anonimo. L'avversario di TM-28 enrolla identita legittime e apre sessioni: il
  certificato glielo si presenta comunque.

Il guadagno reale e quindi un **cambio di costo dell'attacco**, da lettura
gratuita e retroattiva a partecipazione attiva e contemporanea. Non e la
chiusura del difetto, ed e comunque la meta con la finestra che si chiude: una
volta che una devnet ha emesso certificati che legano le due identita, toglierli
e una migrazione di identita su una rete con storia.

## Decision

**La chiave di trasporto libp2p e distinta dalla chiave di identita Coblox,
subordinata a essa, ruotabile, e il suo legame non e pubblicato sul ledger.**

**1. Il costrutto esiste gia nel protocollo e non se ne inventa uno nuovo.** E lo
stesso schema della chiave di consenso del validatore: una chiave Ed25519
distinta, subordinata all'identita enrollata e legata da una prova di possesso,
che non e una seconda identita enrollata. La terza chiave segue quel modello.

**2. La prova di possesso e presentata in sessione, non pubblicata.**
`libp2p_peer_id` esce dalla richiesta di enrollment e dal certificato. Chi
stabilisce una connessione riceve, insieme al certificato, l'attestazione firmata
dall'identita che autorizza quella chiave di trasporto.

**3. La proprieta che la vecchia regola comprava va conservata, e come regola di
validita.** *Un peer non puo sostituire un'identita di trasporto dopo
l'enrollment* diventa: nessun peer puo presentarsi con una chiave di trasporto
priva di un'attestazione valida dell'identita che dichiara. Verifica
obbligatoria, rifiuto in caso di assenza, e la stessa forma normativa che ha
oggi la doppia derivazione. Una separazione che indebolisse questa proprieta
sarebbe un peggioramento netto, non un compromesso.

**4. Il residuo e dichiarato qui e non altrove.** Contro un avversario che tiene
una sessione Coblox, il legame `node_id` verso IP resta ottenibile. L'altra meta
del rimedio e il Circuit Relay v2, gia previsto da `wire.md`, che nasconde l'IP
di origine al peer remoto; **non e adottato ora** perche introduce un insieme di
nodi privilegiati che vedono tutto, cioe sposta la fiducia invece di eliminarla,
e perche non ha la finestra che questa decisione ha. Va valutato come decisione
propria, non incorporato qui per completezza apparente.

## Alternatives considered

- **Mantenere la chiave unica e limitarsi a dichiarare.** E la raccomandazione
  minima di AGENT-007 e sarebbe stata coerente con la proporzionalita che
  l'operatore ha invocato su [ADR-014]. Rifiutata dall'operatore per la ragione
  che il threat model stesso indica: la riprogettazione *va valutata prima che
  la rete abbia utenti*, e il costo cresce in modo monotono fino a diventare una
  migrazione.
- **Solo il relay obbligatorio per i nodi domestici.** Affronta la meta che
  questa decisione non affronta — l'IP verso il peer diretto — e lascia intatto
  il legame pubblicato sul ledger, che e la meta gratuita per l'avversario. Le
  due misure sono complementari e non alternative; questa ha la scadenza.
- **Entrambe subito.** Rifiutata sul dimensionamento e non sul merito: il relay
  obbligatorio e una scelta di topologia con conseguenze su latenza e carico che
  vanno misurate, e misurarle richiede una devnet che ancora non esiste.
- **Rendere anonimo il gossip applicativo**, togliendo `sender_node_id`
  dall'envelope. Rifiutata: e la sola misura che chiuderebbe TM-28 verso il peer
  diretto, e distruggerebbe l'attribuzione su cui poggiano validazione,
  backpressure e ogni difesa anti-spam. `wire.md` vieta gia la modalita autore
  anonimo di libp2p per questa ragione.

## Consequences

- **Tocca quattro superfici**: `identity.md` (gerarchia delle chiavi,
  autenticazione sulla connessione), `wire.md` (cio che si presenta e quando),
  lo schema della richiesta di enrollment e del certificato, e le preimmagini di
  hash che ne discendono — quindi `enrollment_request_hash` e le fixture
  pubblicate. **La gate di [ADR-012] si applica**, e con essa l'inventario degli
  artefatti pubblicati che oggi non esiste.
- **Deve atterrare prima che la devnet emetta il primo certificato.** Dopo, non
  e piu questa decisione: e una migrazione.
- **Un'interazione da attaccare in review, non da assumere risolta.** Le code per
  peer e la backpressure di `wire.md` sono ancorate al peer di trasporto. Una
  chiave di trasporto ruotabile e una chiave che azzera lo stato per peer, il che
  tocca le difese anti-spam e va confrontato con lo scudo di ammissione di
  [ADR-007]. Lo stream di enrollment e meno esposto, perche accetta gia peer di
  trasporto non autenticati e i suoi limiti sono ancorati alla chiave e alla
  sorgente; il resto del gossip no. **Questa e la conseguenza che il Lead giudica
  piu probabile fonte di un difetto**, ed e segnalata come tale a chi fara la
  review.
- La revoca continua a funzionare senza modifiche, perche opera sull'identita e
  l'identita e presentata in sessione.
- `identity.md` §*Key hierarchy* va riscritta, non annotata. La frase sulla
  doppia derivazione verificata obbligatoriamente diventa falsa nel momento in
  cui questa ADR e attuata, ed e esattamente la forma della famiglia 2 di
  `.lmbrain/knowledge/recurring-defects.md`: un'affermazione rimasta indietro
  rispetto alla regola che la rendeva vera.

## Review conditions

Rivedere se: la review adversariale mostra che la rotazione della chiave di
trasporto indebolisce la backpressure oltre il guadagno di privacy, nel qual
caso la leva e vincolare la frequenza di rotazione e non rinunciare alla
separazione; oppure se il relay obbligatorio venisse adottato, nel qual caso il
residuo dichiarato al punto 4 va riscritto e non solo cancellato. **Non
rivedere** dichiarando TM-28 chiuso: questa decisione ne sposta il costo e non
lo elimina, e registrarla come chiusura sarebbe la prima riga di una quinta
occorrenza gia vista.
