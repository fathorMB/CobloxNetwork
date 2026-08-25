---
id: ADR-013
# Note: Quote the title if it contains a colon
title: "L'intervallo di blocco e una costante di genesi dichiarata, non un parametro governato"
status: accepted
decision_date: 2026-08-25
decider: AGENT-LEAD
supersedes: []
superseded_by: []
links: [ADR-001, ADR-010, ADR-011]
tags: [architecture, consensus]
updated: 2026-08-25
activity:
  - date: 2026-08-25
    action: "transitioned proposed -> accepted"
---
# L'intervallo di blocco e una costante di genesi dichiarata, non un parametro governato

> Decisa dall'operatore il 2026-08-25, nella passata di chiusura delle decisioni di prodotto aperte.

## Context

**Nessun documento di protocollo fissa un intervallo di blocco.** La ricerca su
`docs/protocol/` non restituisce alcuna occorrenza: l'unico vincolo temporale su
un blocco e che `timestamp_ms` superi la mediana degli undici precedenti
(`ledger.md` §*Block header*), che impone monotonia e non passo.

Il valore 5 s esiste in un solo punto del repository, ed e dichiarato per quello
che e: `sim/coblox_sim/recommended.py` porta
`BLOCK_INTERVAL_SECONDS = 5  # assumption`. Su quell'assunzione poggia la
taratura di [SPEC-007]: `election_epoch_blocks = 120 960` significa "7 giorni"
solo se un blocco dura 5 secondi.

**Le due meta del protocollo hanno denominatori diversi, e una sola era chiusa.**
L'emissione e denominata in millisecondi (`reward_epoch_ms`), e [SPEC-009] ne ha
chiuso il denominatore con `reward_epoch_ms_min` e `reward_epoch_ms_max` in
`RewardBounds`, perche accorciare l'epoca moltiplicherebbe l'emissione reale
senza violare alcun tetto. L'elezione e denominata in blocchi
(`election_epoch_blocks`, `candidacy_close_blocks`, `election_entropy_blocks`,
`min_revocation_effective_delay_blocks`,
`election_parameter_min_activation_gap_blocks`), ed e limitata in blocchi da
`ElectionBounds`. Un limite in blocchi non e un limite in tempo reale finche il
blocco non ha una durata.

E la stessa domanda che [REVIEW-014] ha posto sul fondo — *qual e il
denominatore?* — applicata alla meta che quella review non guardava.

## Decision

**L'intervallo di blocco obiettivo di Coblox v0 e di 5 secondi, dichiarato nei
documenti di protocollo come costante di genesi.**

Ne discendono tre cose, e la terza e la piu importante perche e una rinuncia.

**1. I ventidue parametri tarati da [SPEC-007] restano invariati** e acquistano
il significato in tempo reale che finora era un'assunzione: epoca di elezione
7 giorni, chiusura delle candidature 1 giorno prima del confine, finestra di
entropia 1 ora, mandato massimo 9 epoche pari a 63 giorni, tetto di genesi del
mandato 12 epoche pari a 84 giorni.

**2. Non e un parametro governato.** Non entra in `ConsensusParametersBody` e
non e modificabile da un documento firmato. La ragione e esattamente quella per
cui `reward_epoch_ms` ha un pavimento: un intervallo di blocco governabile
sarebbe un denominatore che la governance puo muovere sotto ogni limite
espresso in blocchi. Cambiarlo richiede una genesi nuova.

**3. v0 non specifica la produzione dei blocchi, e nessuna regola interna alla
catena potrebbe imporla.**

> **Questa parte e stata riscritta il 2026-08-25**, non annotata, come le
> *Review conditions* di questa ADR prevedono per il caso in cui il debito sul
> passo di produzione si risolva. La formulazione precedente diceva che «cio che
> v0 dichiara non e cio che v0 impone», il che descriveva una cadenza dichiarata
> e non applicata. **Era una descrizione sbagliata di un fatto piu grande**, e
> lasciava disponibile un rimedio che non rimedia. La versione precedente e
> conservata nella storia di questo file; l'errore e chiamato errore.

La dichiarazione fissa il significato dei parametri per chi implementa e per chi
tara. Cio che manca **non e una regola sulla cadenza: e il livello di produzione
dei blocchi per intero.** `docs/protocol/` non specifica ne la selezione del
proposer ne la meccanica dei round, e il solo vincolo su `timestamp_ms` e la
mediana degli undici precedenti, che impone monotonia e non passo.

**La proposizione generale, che e la ragione per cui questa parte e una rinuncia
e non una lacuna da colmare:** *nessuna regola di validita interna alla catena
puo vincolare il tempo reale, perche ogni orologio della catena e scritto dai
validatori.* Stabilita da AGENT-007 valutando [DEBT-013].

**Un rimedio apparente e nominato qui perche non venga adottato.** Una regola di
validita sulla distanza fra `timestamp_ms` consecutivi **non chiude nulla**:
`timestamp_ms` e scritto dagli stessi validatori, quindi una simile regola li
obbliga a **scrivere** timestamp vicini, non a **produrre** blocchi vicini.
Adottarla darebbe una chiusura falsa al prezzo pieno di una passata di
[ADR-012], e sarebbe la famiglia 3 di `.lmbrain/knowledge/recurring-defects.md`
commessa dentro il rimedio: vincolata la grandezza nominata, non quella da cui
la proprieta dipende. E registrata come primo esito ammissibile in [DEBT-013] ed
e **respinta**.

**Cio che v0 ha, ed e uno solo.** L'unico orologio esterno del protocollo e il
**checkpoint di soggettivita debole**, che porta `height`, `timestamp_ms` e
`issued_at_ms` firmati da una chiave che non appartiene a nessun validatore. Due
checkpoint misurano la cadenza reale. La chiusura praticabile passa di li, e non
**impedisce** il rallentamento: lo rende **misurabile e dichiarato**. Per un
difetto la cui gravita e tutta nell'invisibilita e la parte che conta, e dire
«chiuso» direbbe piu di quanto sara scritto.

**Le conseguenze, nella forma accertata e non in quella supposta.** Il set
attivo determina la durata in tempo reale delle proprie epoche, quindi la
propria incumbency, e la soglia non e il quorum ma un **terzo bloccante**, con
la catena viva e ogni blocco valido. Le garanzie anti-cattura di [SPEC-006] sono
denominate in epoche e restano vere in epoche; la loro traduzione in giorni
dipende da chi le epoche le produce. Il dettaglio, con i tre effetti valutati
separatamente, e in [DEBT-013] e negli scenari del threat model che lo chiudono.

## Alternatives considered

- **Lasciarlo assunto, decidendo a ridosso della devnet con misure reali.**
  Rifiutata dall'operatore: la devnet e cio che accumula storia conservabile, e
  ogni parametro denominato in blocchi entrerebbe nell'ancora di genesi con un
  significato in tempo reale indeterminato. E la stessa ragione per cui
  [DEBT-005] non poteva essere rimandato.
- **10 secondi.** Dimezzerebbe volume di messaggi, crescita della catena e
  consumo di batteria e dati, che e il rischio dichiarato n.2 di M-04. Costo:
  ogni parametro denominato in blocchi va dimezzato e l'intero blocco di
  vincoli ri-verificato, e la conferma percepita raddoppia. Resta la prima
  alternativa da riconsiderare se il vincolo mobile si rivelasse dominante.
- **2 secondi.** Rifiutata perche peggiora proprio il vincolo mobile, a fronte
  di un guadagno di latenza percepita che in un consenso BFT su set piccolo e
  dominato dai giri di firma e non dall'attesa del blocco.
- **Renderlo un parametro governato con pavimento e tetto**, per simmetria con
  `reward_epoch_ms`. Scartata perche la simmetria e apparente: `reward_epoch_ms`
  e un dato che i validatori leggono, l'intervallo di blocco e un comportamento
  che i validatori hanno. Un limite su un numero che nessuna regola confronta
  con la realta e una dichiarazione, non un vincolo — e questa ADR preferisce
  chiamarla dichiarazione.

## Consequences

- I documenti di protocollo acquistano la costante e la sua conseguenza
  dichiarata. E contenuto normativo nuovo, quindi la gate di [ADR-012] si
  applica alla spec che lo scrive.
- La taratura di [SPEC-007] smette di poggiare su un'assunzione non versionata
  in `docs/`, e `BLOCK_INTERVAL_SECONDS` nel simulatore diventa una copia di un
  valore normativo invece che la sua unica sede.
- ~~Il debito sul passo di produzione non imposto va aperto nella stessa
  passata, e va sottoposto a review adversariale invece che valutato dal Lead.~~
  **Fatto**: aperto come [DEBT-013] e valutato da AGENT-007 il 2026-08-25. La
  scelta di non farlo valutare al Lead si e rivelata quella giusta: la
  valutazione ha corretto **due** affermazioni che il Lead aveva scritto nel
  debito. Che la soglia fosse il quorum — e un **terzo bloccante**. E che
  l'emissione non si muovesse perche denominata in millisecondi — si muove
  **verso il basso**, perche la catena non ha altro orologio dei propri
  `timestamp_ms`. La seconda correzione **aggrava**: il rallentamento ha un
  costo, ma **esternalizzato**, perche si perde l'emissione di tutta la rete e
  si conserva il seggio del solo cartello. Il movente e piu forte, non piu
  debole.
- **La chiusura di [DEBT-013] e lavoro di una spec e passa dal checkpoint di
  soggettivita debole**, unico orologio esterno del protocollo. Quella spec
  tocca lo stesso oggetto di [DEBT-014] e deve conoscerne la conclusione per non
  riaprirla.
- La disposizione di [DEBT-010] usa questi numeri: 63 e 84 giorni sono
  l'incumbency prima e dopo una spinta irreversibile del cricchetto.

## Review conditions

Rivedere se: le misure reali di una devnet mostrano che 5 secondi sono
insostenibili per i nodi mobili, che e il rischio dichiarato di M-04 e l'unica
evidenza che dovrebbe muovere questo valore; ~~oppure se il debito sul passo di
produzione non imposto si risolvesse con una regola di validita, nel qual caso
la parte 3 di questa decisione va riscritta e non solo annotata~~ — **la parte 3
e stata riscritta il 2026-08-25**, e non perche quel debito si sia risolto con
una regola di validita ma perche la valutazione ha stabilito che **nessuna
regola interna alla catena potrebbe esserlo**. La condizione era scritta
prevedendo l'esito piu comodo; l'esito reale e stato l'opposto e ha comunque
imposto la riscrittura.

Rivedere inoltre se una revisione futura del protocollo specificasse la
produzione dei blocchi — selezione del proposer e meccanica dei round — che oggi
`docs/protocol/` non contiene affatto: quella e la premessa che rende vera la
parte 3, e se cade va riesaminata per intero. **Non
rivedere** per allineare Coblox alla cadenza di un'altra rete: il numero qui
serve a dare significato ai parametri tarati, non a somigliare a qualcosa.
