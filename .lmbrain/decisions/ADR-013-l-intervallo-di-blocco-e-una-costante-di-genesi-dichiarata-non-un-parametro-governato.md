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

**3. Cio che v0 dichiara non e cio che v0 impone, e va detto.** La dichiarazione
fissa il significato dei parametri per chi implementa e per chi tara; **non
esiste in v0 una regola di validita che imponga il passo di produzione dei
blocchi**, perche il solo vincolo su `timestamp_ms` e la mediana degli undici
precedenti. La conseguenza e nominata qui invece di essere lasciata scoprire, ed
e registrata come debito separato: il set di validatori attivo determina di
fatto la durata in tempo reale delle proprie epoche, quindi la propria
incumbency. Le garanzie anti-cattura di [SPEC-006] sono denominate in epoche e
restano vere in epoche; la loro traduzione in giorni dipende da chi le epoche le
produce.

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
- Il debito sul passo di produzione non imposto va aperto nella stessa passata,
  e va sottoposto a review adversariale invece che valutato dal Lead: e
  un'osservazione sul potere del set attivo, cioe la superficie su cui questo
  progetto ha gia sbagliato tre volte di seguito.
- La disposizione di [DEBT-010] usa questi numeri: 63 e 84 giorni sono
  l'incumbency prima e dopo una spinta irreversibile del cricchetto.

## Review conditions

Rivedere se: le misure reali di una devnet mostrano che 5 secondi sono
insostenibili per i nodi mobili, che e il rischio dichiarato di M-04 e l'unica
evidenza che dovrebbe muovere questo valore; oppure se il debito sul passo di
produzione non imposto si risolvesse con una regola di validita, nel qual caso
la parte 3 di questa decisione va riscritta e non solo annotata. **Non
rivedere** per allineare Coblox alla cadenza di un'altra rete: il numero qui
serve a dare significato ai parametri tarati, non a somigliare a qualcosa.
