---
id: ADR-014
# Note: Quote the title if it contains a colon
title: "Gli abbonamenti sono pubblici e correlabili, e il progetto lo dichiara prima che lo siano"
status: accepted
decision_date: 2026-08-25
decider: AGENT-LEAD
supersedes: []
superseded_by: []
links: [ADR-005, ADR-006, ADR-015]
tags: [architecture, security, product]
updated: 2026-08-25
activity:
  - date: 2026-08-25
    action: "transitioned proposed -> accepted"
---
# Gli abbonamenti sono pubblici e correlabili, e il progetto lo dichiara prima che lo siano

> Decisa dall'operatore il 2026-08-25. Chiude [DEBT-006], l'unica superficie del
> threat model che non aveva un ADR alle spalle. Coincide con la raccomandazione
> di AGENT-007 per v0 in [SPEC-004] (TM-26, TM-29).

## Context

La quota al creatore di [ADR-006] richiede, per costruzione, che i validatori
raggruppino i burn di abbonamento per `payer_node_id` e ne ricavino
`active_subscriber_count` e `active_subscription_root`. Contare per identita su
un ledger pubblico significa pubblicare la lista. **Non e un abuso del sistema:
e il funzionamento del sistema**, ed e un conflitto strutturale fra la
ricompensa al creatore e la privacy dell'abbonato.

**Cosa e esposto, alla lettera.** Un burn `app_subscription` porta
`payer_node_id`, `app_id`, l'importo, il periodo di servizio e gli orari; il
saldo sta in un albero pubblico indicizzato per `account_key = H(node_id)`. Il
`node_id` e uno pseudonimo della forma `cblx1...`, derivato dalla chiave
pubblica di identita. **Nel ledger non compaiono nomi, indirizzi email o
indirizzi IP.** Su questo l'operatore ha ragione e la lettura e stata verificata
prima di decidere.

**Dove il conteggio non e il cardine.** Il rimedio che sembra ovvio — togliere
la quota al creatore, quindi la ragione di contare — non funziona, e capire
perche e cio che rende questa decisione difendibile invece che comoda. La fuga
non sta nel conteggio: sta nel **burn**, che nomina `payer_node_id` perche e la
firma del pagatore ad autorizzare l'addebito. Togliere la ricompensa toglie la
ragione di contare e lascia la lista intatta. La grandezza da cui la proprieta
dipende e l'invariante *un pagatore, un voto*, e romperlo richiede una prova a
divulgazione nulla di unicita.

**Due precisazioni al ragionamento di proporzionalita dell'operatore**, che non
lo ribaltano e ne delimitano la portata.

1. *«Se non ci sono dati personali in gioco»* vale per il ledger da solo. Il
   legame fra lo pseudonimo e un indirizzo IP e disponibile a chiunque
   partecipi, ed e materia di [ADR-015]; questa ADR presuppone quella e non la
   sostituisce.
2. *«I crediti non hanno reale valore economico»* toglie il ladro, non il
   profilatore. Il bene esposto non sono i crediti ma il profilo di consumo, e
   chi si abbona a un servizio che dice qualcosa di se lo scrive accanto a un
   identificatore stabile per sempre. Su un ledger immutabile non esiste rimedio
   retroattivo.

## Decision

**Il conteggio per identita resta, e il progetto dichiara pubblicamente la
proprieta che ne discende prima che esista il primo partecipante esterno.**

**1. La dichiarazione e un obbligo con una scadenza, non una nota.** Dice che
identificatore di nodo, saldo, abbonamenti e orari di attivita sono pubblici,
permanenti e correlabili fra loro da chiunque, e che l'assenza di valore
monetario del credito non riduce questa proprieta. E `SEC-REQ-22`, e la scadenza
e il primo partecipante esterno alla rete — non il lancio pubblico, non la beta.

**2. Va dove l'utente la legge prima di abbonarsi**, non solo in `SECURITY.md`.
Su un registro immutabile la dichiarazione e **l'unico controllo che agisce in
tempo**: tutto cio che viene dopo descrive dati gia pubblicati.

**3. La prova aggregata e ricerca, e va nominata come ricerca.** Resta il
candidato per la beta pubblica e **non e una promessa**: un progetto che
annuncia una privacy futura ottiene oggi la fiducia che quella privacy
meriterebbe domani, e la incassa su dati che si stanno pubblicando adesso.

## Alternatives considered

- **Prova aggregata subito**, con un accumulatore che non espone i membri.
  Rifiutata per v0 perche i validatori devono comunque conoscere i membri per
  verificarli: sposta il problema dal pubblico ai validatori invece di
  eliminarlo, a fronte di un costo criptografico sproporzionato. Chiude meta di
  TM-29 e nulla di TM-26 verso i validatori.
- **Chiave di spesa per app**, cosi che il ledger non correli gli abbonamenti di
  una stessa persona. E il rimedio che affronta la causa vera, ed e per questo
  che va registrato qui e non liquidato: rompe *un pagatore, un voto*, che e cio
  che impedisce il doppio conteggio, e ripararlo richiede una prova a
  divulgazione nulla di unicita. Riprogettazione, non regolazione.
- **Rimandare la quota al creatore a M-06.** Rifiutata sul merito e non sul
  costo: toglie la ragione di contare e lascia la lista, per la ragione spiegata
  sopra. E l'alternativa che sembra risolvere e non risolve, ed e registrata
  qui perche non venga riproposta.

## Consequences

- [DEBT-006] e chiuso da questa decisione; la sua esecuzione — il testo pubblico
  — e lavoro di una spec con `GATE-SECREVIEW`, perche e un documento di sicurezza
  rivolto all'utente e non una nota di rilascio.
- Il progetto assume una posizione dichiarata sulla privacy che finora non
  aveva, e che e coerente con quella gia presa su Sybil da [ADR-007]: dire i
  limiti invece di lasciarli scoprire. E la stessa moneta con cui `SECURITY.md`
  paga gia i limiti noti.
- La ricerca sulla prova aggregata entra nel backlog di M-08 come lavoro
  possibile, senza data e senza promessa.
- La dichiarazione va scritta **una volta** e citata, non riscritta in ogni
  superficie: due copie divergono, ed e la famiglia 2 di
  `.lmbrain/knowledge/recurring-defects.md`.

## Review conditions

Rivedere se: una prova di unicita a divulgazione nulla diventa praticabile a un
costo proporzionato per un nodo domestico, che e la sola cosa che cambierebbe il
merito; oppure se un obbligo normativo applicabile all'operatore rendesse
insufficiente la sola dichiarazione, nel qual caso la leva e la chiave di spesa
per app e non l'aggregazione. **Non rivedere** perche la dichiarazione risulta
scomoda in comunicazione: la scomodita e il contenuto informativo della
dichiarazione, ed e la ragione per cui va fatta prima e non dopo.
