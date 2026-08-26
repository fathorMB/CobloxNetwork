---
id: ADR-016
# Note: Quote the title if it contains a colon
title: "La banda di cadenza di genesi: larga sul lato lento, stretta dove sta l'emissione"
status: accepted
decision_date: 2026-08-26
decider: OPERATOR
supersedes: []
superseded_by: []
links: [ADR-010, ADR-013]
tags: [consensus, governance]
updated: 2026-08-26
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned proposed -> accepted"
---
# La banda di cadenza di genesi: larga sul lato lento, stretta dove sta l'emissione

> Decisa dall'operatore il 2026-08-26, sui valori istruiti da AGENT-002 in [SPEC-016].

## Context

[SPEC-016] ha chiuso [DEBT-013] rendendo la cadenza reale **misurabile e dichiarata**, non impedita — l'unica forma che [ADR-013] lascia disponibile, perché nessuna regola interna alla catena può vincolare il tempo reale. La misura confronta due estremi che non sono orologi della catena: il checkpoint di soggettività debole e l'orologio di chi verifica.

**La tolleranza di quella misura era l'unica cosa rimasta aperta**, ed è una decisione dell'operatore per la stessa ragione per cui lo sono `α` e la popolazione al lancio: l'algoritmo è fissato, il valore no.

**I due lati non commerciano contro la stessa cosa, ed è il fatto che governa questa decisione.**

- Il **lato lento** è la questione dell'incumbency. Il client si limita a **segnalare**, perché una lettura lenta è indistinguibile dal ritardo di sync a qualunque grandezza. Ma il **processo di rilascio dei checkpoint fallisce chiuso in entrambi i versi**, quindi il bordo lento decide anche quando i checkpoint smettono di essere emessi.
- Il **lato veloce** è la questione dell'emissione, ed è diventato tale **solo quando `reward_epoch` è stato ritmato dall'altezza**. Prima di [SPEC-016] accelerare era considerato benigno. Su questo lato il client **fallisce chiuso**, perché nulla di onesto fa apparire blocchi.

## Decision

**Banda di genesi:**

| Campo | Valore | Significato |
| --- | --- | --- |
| `block_interval_ms` | `5000` | La costante dichiarata di [ADR-013] |
| `min_ms_per_block` | `2500` | `interval / 2` — obietta a un **raddoppio** dell'emissione reale |
| `max_ms_per_block` | `20000` | `4 × interval` |
| `min_measured_blocks` | `720` | Un'ora di catena: sotto, la misura **non è fatta** |
| `max_external_clock_slack_ms` | `600000` | Dieci minuti fra latenza di rilascio ed errore d'orologio |

I vincoli sono rispettati: `2500 ≤ 5000 ≤ 20000`, e `600000 < 720 × 5000 = 3 600 000`.

**Larga in generale, e la ragione è che stringere costa una release e allargare costerebbe fiducia.** La banda vive nella distribuzione firmata e **nessun documento on-chain può cambiarla** — una banda che un quorum seduto potesse allargare sarebbe una tolleranza sotto l'unica misura che il protocollo ha del comportamento di quel quorum. Ma **una release nuova può stringerla**, quindi partire larghi non è una rinuncia: è rimandare la precisione a quando la devnet darà misure reali. Scegliere adesso una soglia stretta significherebbe scegliere un numero che nessuna misura sostiene, che è ciò che [ADR-010] chiama un valore ben scelto invece di una proprietà.

**Stretta dove sta l'emissione.** `interval / 2` è **più stretto** dell'esempio `interval / 4` con cui `README.md` illustra il costo di una banda larga sul lato veloce, e obietta già a un raddoppio. È il lato su cui vale la pena essere severi, perché è l'unico dove il guadagno dell'attaccante è diretto.

**Larga sul lato lento, e questa parte corregge un errore del Lead.** La proposta portata all'operatore diceva `2 ×` su entrambi i lati. `README.md` avverte esplicitamente che una banda di `2 × block_interval_ms` *«calls a network out of band during an ordinary partition»*, e poiché il processo di rilascio fallisce chiuso in entrambi i versi, quel valore **fermerebbe l'emissione dei checkpoint durante una partizione ordinaria** — cioè toglierebbe ai client l'unico orologio esterno che possiedono, per un evento che non è un attacco. Il Lead ha proposto il valore **senza aver letto l'istruzione che lo governava**, che è la stessa forma di errore che questa sessione ha censito cinque volte. `4 ×` sta sopra il rumore di una partizione ordinaria e molto sotto i `20 ×` che lo stesso paragrafo dichiara inutili.

## Alternatives considered

- **Stretta su entrambi i lati** (`1,25 ×` veloce, `1,5 ×` lento, tre ore di finestra). Intercetterebbe già un `+25 %` di emissione. Rifiutata: su una rete che non esiste ancora sceglierebbe una soglia che nessuna misura sostiene, bloccherebbe client onesti su variazioni piccole, e non produrrebbe alcuna misura prima di tre ore di catena oltre il checkpoint.
- **Molto larga** (`3 ×` per lato, mezz'ora di finestra). Rifiutata perché coglierebbe solo una manovra grossolana: la guardia esisterebbe e direbbe poco, che è la condizione in cui una guardia smette di essere letta.
- **`2 ×` anche sul lato lento**, come nella proposta originale. Rifiutata per la ragione scritta sopra, e resta la prima da riconsiderare **al contrario**: se le misure della devnet mostrassero che le partizioni ordinarie non avvicinano mai `4 ×`, il bordo lento va stretto.

## Consequences

- **Un set attivo può stirare le proprie epoche fino a quattro volte** prima che qualcosa lo dica. Le garanzie anti-cattura di [SPEC-006] restano vere **in epoche**; la loro traduzione in giorni dipende da chi le epoche le produce, e questa banda dichiara di quanto.
- **Il lato veloce blocca il client**, quindi un errore su `max_external_clock_slack_ms` si paga in disponibilità. Dieci minuti sono una scelta sulla latenza di rilascio attesa, non una misura: vanno riverificati appena il processo di rilascio esiste davvero.
- **I valori vanno scritti** in `docs/protocol/README.md` al posto della voce DRAFT, e nell'ancora di fiducia di genesi. È contenuto normativo nuovo, quindi **la gate di [ADR-012] si applica alla spec che lo scrive** — non a questa decisione.
- La lista DRAFT dei parametri di lancio perde una voce e ne conserva le altre tre.

## Review conditions

Rivedere quando la devnet produce misure reali di cadenza e di latenza di rilascio: è l'unica evidenza che dovrebbe muovere questi numeri, e la banda si stringe con una release senza toccare la genesi.

Rivedere **il lato veloce prima del lato lento** se l'emissione reale divergesse dalla nominale: è il lato dove il guadagno è diretto e dove il client fallisce chiuso.

**Non rivedere** per allineare la banda alla variabilità osservata: una banda tarata sul comportamento osservato di un set è una banda che quel set ha scelto, ed è precisamente ciò che il divieto di modifica on-chain esiste per impedire.
