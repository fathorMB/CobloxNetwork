---
title: "Quando un vincolo può essere un predicato di accettazione, e cosa fare quando non può"
updated: 2026-08-27
tags: [consensus, governance, method]
---

# Quando un vincolo può essere un predicato di accettazione

## A cosa serve questa pagina

Prima di scrivere in un ADR *«questo parametro va limitato così»*, questa pagina dice come sapere se quel limite **può essere una regola** — cioè se un validatore, all'accettazione di un documento firmato, può valutarlo.

Nasce da un caso reale che è costato **tre passate di review e una trentina di rilievi**: l'analisi dei dieci parametri operativi di [DEBT-036]. Cinque righe su dieci hanno continuato a cedere, e ogni correzione produceva un'ancora falsa. Non era un difetto di scrittura.

## Il criterio

> **Un vincolo può essere un predicato di accettazione soltanto se la grandezza da cui il pericolo dipende è portata da un documento firmato o è una costante di genesi.**

E c'è un secondo requisito, che il caso della sospensione delle app ha isolato:

> **Non basta che sia esprimibile: deve essere valutabile in un solo punto di accettazione.** Se il predicato lega due specie di documento che attivano in modo indipendente, legarlo all'accettazione di una sola lo fa eludere pubblicando l'altra dopo.

## Il protocollo lo aveva già scritto

Il criterio non è un'invenzione di questa pagina. `docs/protocol/ledger.md`, §*"Block format"*, lo dichiara in forma generale spiegando perché una regola sulla cadenza **non esisterà mai**:

> **«No rule here will ever impose it, and the reason is general.»** Every clock this chain carries is written by the validators, so **a validity rule can only compare a validator-written number to a validator-written number**. In particular a rule on the distance between consecutive `timestamp_ms` values is **rejected** and not merely absent: it would oblige a set to *write* a cadence, not to *produce* one, and would buy **a false closure at the full price of a specification change**.

**«Rifiutata e non semplicemente assente»** è la parte che conta. Non è una lacuna da riempire: è una porta chiusa con la sua ragione.

## Le due specie di oggetto di genesi, che non vanno confuse

L'ancora di fiducia di genesi contiene oggetti di **due nature diverse**, e `docs/protocol/README.md` avverte esplicitamente che la differenza *«is not a detail»*:

| specie | esempio | cosa fa |
| --- | --- | --- |
| **vincola un campo** | `ElectionBounds`, `RewardBounds` | *«bound values that a signed document carries, so a document outside them is rejected on acceptance»* — **è un predicato** |
| **vincola una misura** | `CadenceBand` | *«bounds nothing any document carries … no validity rule of this protocol compares anything to it»* — **è un'osservazione** |

Una catena fuori dalla propria banda di cadenza **non è invalida**. Confondere le due specie porta a credere di aver scritto una regola quando si è scritto un termometro.

**È la trappola in cui il Lead è caduto** ragionando su questo caso: dall'osservazione corretta che *«in genesi si può mettere quel che si vuole, lo prova `ElectionBounds`»* aveva concluso che qualunque limite fosse ancorabile. Vero sul meccanismo, **falso come principio** — e applicato alla finestra di macinatura avrebbe autorizzato precisamente la falsa chiusura che `ledger.md` rifiuta per nome.

## L'asimmetria fra pavimento e tetto

**Questa è la parte utile in pratica, e per tre passate nessuno l'ha guardata.**

Sui cinque parametri che non si lasciavano vincolare, il pericolo **al tetto** dipende da una grandezza che il protocollo non nomina — e lì il predicato non si può scrivere. Ma **al pavimento** il pericolo è una proprietà della magnitudine **di quel parametro stesso**:

- deriva d'orologio troppo piccola → i propri blocchi vengono scartati;
- validità di busta troppo piccola → la propria busta scade prima del primo hop;
- cache a `1` voce → la propria seconda busta è `rate_limited`;
- età del checkpoint a zero → il proprio checkpoint è scaduto all'arrivo.

**Al pavimento la coincidenza c'è.** Il pavimento è quindi **sempre esprimibile** come predicato di accettazione — un `_min` di genesi su un campo di documento, meccanismo già dimostrato in `core/coblox-core/src/params.rs` — e va chiuso.

E il pavimento è il lato che conta: la passata su [DEBT-036] ha stabilito che **otto dei dieci parametri falliscono al pavimento e nessuno al tetto**. Tre passate di review hanno litigato **solo sui tetti**.

> **Una riga che sembra «non limitabile» è di solito limitabile per metà, e la metà limitabile è quella letale.**

## Cosa scrivere quando il predicato non si può fare

*«Non si limita per magnitudine»* **è una resa, e non è necessaria.** Il protocollo ha già un modello lavorato per questo caso, e ha **tre parti**:

1. **Nominare la grandezza da cui il pericolo dipende davvero**, separatamente dal parametro.
2. **Dire se è portata da un documento firmato.** Se non lo è, dichiarare che un predicato su di essa è **rifiutato e non assente**, con la ragione generale.
3. **Consegnare la contromisura della seconda specie**: una **misura** contro un estremo fuori catena, con tolleranza di genesi, e disposizione **asimmetrica** — fallire chiuso dove il verdetto è attribuibile, **segnalare** dove non lo è.

Il terzo punto è la forma di [ADR-016], che è la sede in cui questo progetto ha già scelto quella disposizione: il lato lento **segnala** perché una lettura lenta è indistinguibile da un ritardo di sync; il lato veloce **fallisce chiuso** perché nulla di onesto fa apparire blocchi.

E il tetto sul campo **va scritto comunque**, dichiarato come **mitigazione di grado** con il termine residuo nominato accanto — non come chiusura.

## Il caso che non rientra: quando manca l'ordine, non la grandezza

Esiste una quarta forma, e va tenuta distinta perché il rimedio è diverso.

`max_weak_subjectivity_age_ms` **non** manca di un'ancora: entrambi gli operandi sono portati — il campo on-chain e la copia dentro il checkpoint firmato. Il pericolo è che i due **non concordino**, e il disaccordo fa fallire chiuso ogni light client conforme **a qualunque valore**.

**Qui il predicato è esprimibile, ma non è un ordinamento.** Una banda è la forma sbagliata, non il luogo sbagliato: serve una **regola di uguaglianza o di variazione**. Il co-rimedio esiste già nel protocollo, che impone di ripubblicare un checkpoint fresco a ogni revoca di validatore.

## Le due tesi sbagliate, e perché sono registrate

Sono qui perché sono state formulate, attaccate e rotte, e chi ripercorrerà questa strada le incontrerà di nuovo.

**Tesi A — *«non esiste ancora ancorabile in genesi»*.** Cade: il meccanismo esiste. La sua seconda clausola — *«tutto ciò a cui si ancorerebbe è governato dallo stesso quorum»* — è falsa due volte: per quattro righe la grandezza mancante **non è governata, non è nominata affatto**; per la quinta l'ancora è firmata dalla chiave di release, che nessun quorum controlla.

**Tesi B — *«il pericolo non è una proprietà della magnitudine di quel parametro»*.** Descrive correttamente il tetto di quattro righe su cinque, **ma non discrimina**: la non-coincidenza vale per sette righe su dieci, e tre di quelle hanno convertito. E ignora che **al pavimento la coincidenza c'è**.

**Perché entrambe hanno mancato il bersaglio:** guardavano se il parametro *coincide* col pericolo, invece di guardare se la grandezza del pericolo **è nominata dal protocollo**. La prima domanda è sul significato, la seconda su cosa un validatore può leggere — e solo la seconda decide se una regola si può scrivere.

## Come usarla

Davanti a un vincolo che si vorrebbe mettere in un ADR:

1. **Qual è la grandezza da cui il pericolo dipende?** Non il parametro nominato: quella.
2. **È portata da un documento firmato, o è costante di genesi?** Se no, il predicato **non si scrive**, e va dichiarato rifiutato con la ragione.
3. **Se sì, in quanti punti di accettazione va valutata?** Se più di uno, nominarli tutti.
4. **Il pavimento è esprimibile?** Quasi sempre sì. **Chiuderlo.**
5. **Il tetto che resta è mitigazione di grado?** Scriverlo come tale, col residuo nominato.

Correlate: [[recurring-defects]] famiglia 3, di cui questa pagina è il raffinamento — non basta chiedersi *quale* grandezza vincolare, bisogna chiedersi se quella grandezza **sia leggibile da chi deve applicare la regola**.
