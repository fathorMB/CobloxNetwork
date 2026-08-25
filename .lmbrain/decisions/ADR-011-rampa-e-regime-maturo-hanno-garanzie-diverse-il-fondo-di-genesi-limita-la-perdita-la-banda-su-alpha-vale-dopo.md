---
id: ADR-011
# Note: Quote the title if it contains a colon
title: "Rampa e regime maturo hanno garanzie diverse: il fondo di genesi limita la perdita, la banda su alpha vale dopo"
status: accepted
decision_date: 2026-08-25
decider: AGENT-LEAD
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: [ADR-005, ADR-007, ADR-010]
tags: [architecture, security]
created: 2026-08-25
updated: 2026-08-25
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned proposed -> accepted"
---
# Rampa e regime maturo hanno garanzie diverse: il fondo di genesi limita la perdita, la banda su `α` vale dopo

> Decisa dall'operatore il 2026-08-25. Nasce da un'autocorrezione di AGENT-002 in remediation di [REVIEW-011] e dall'analisi che AGENT-007 ne ha tratto verificandola.

## Context

[SPEC-007] ha fissato `α = 0,15` con banda `[0,10 – 0,20]` e `X = 20%`, e ha dichiarato la metrica valida sopra una **soglia d'uso** del 25% del regime di riferimento. La revisione di sicurezza ha misurato che quel numero è **scelto e non misurato**: con il fondo di genesi, `α` scende sotto `0,20` solo dal **70,6%** dell'uso di riferimento, e al 25% vale `0,414`, cioè il doppio di `X`. Fra le due soglie la banda è *dichiarata e violata*, non sospesa. Verificato dal Lead.

**L'osservazione che riorienta la questione** è un'autocorrezione dell'implementatrice. Aveva scritto che «il 91% di un'emissione minuscola è un'emissione minuscola»; riesaminando, ha trovato che **l'importo assoluto dirottato da una flotta vale `F · N/(N+H)` e non contiene l'uso della rete**. Il Lead l'ha eseguito: a uso nullo e al regime di riferimento l'importo è **identico**, 15.725 crediti per epoca. Quell'affermazione era vera solo se anche il fondo è piccolo, e il fondo è una scelta di governance, non una conseguenza del poco uso.

Ne discende la struttura che questa ADR mette per iscritto, e che nessuno dei documenti precedenti aveva:

**`α ≈ 1` durante l'avviamento è strutturale, non un difetto di taratura, e nessun valore del fondo lo cambia.** `α` è il rapporto fra il fondo e l'emissione totale; al lancio non c'è lavoro nel denominatore, quindi il rapporto vale circa 1 **qualunque sia il fondo**. Rimpicciolire il fondo non abbassa `α` durante la rampa: abbassa il fondo.

Ciò che il fondo governa è un'altra grandezza — l'importo assoluto a rischio — e sono quindi **due strumenti separati per due problemi separati**. Trattarli come uno solo, che è ciò che la soglia d'uso faceva, cura il sintomo: rende onesta la dichiarazione e lascia la rampa senza alcuna garanzia, proprio nella fase in cui la rete è più debole e una flotta costa meno.

## Decision

**Ogni fase ha la garanzia che può avere, e le due sono dichiarate separatamente.**

**1. `existence_fund_microtokens_per_epoch` di genesi è dimensionato sulla popolazione attesa al lancio**, non sulla rete matura, e cresce verso la scala di riferimento con la successione di documenti che il limite di variazione consente. È ciò che limita l'importo assoluto a rischio nella fase più esposta, ed è la sola garanzia che quella fase può avere.

Questa disposizione è di **natura diversa** dalle tre di [ADR-010] e per questo non vi appartiene: quelle vincolano ciò che la **governance può fare**, questa vincola ciò che la **genesi deve contenere**. Il fallimento che previene non richiede alcun atto di governance — basta accendere la rete con un fondo dimensionato sulla rete che non c'è ancora.

**2. La banda su `α` e il valore `X` sono dichiarati come proprietà del regime maturo**, con la soglia d'uso reale e non con un numero scelto. La formulazione non è «la metrica vale sopra una soglia» ma **«la metrica è una proprietà della rete a regime; durante l'avviamento la garanzia è un'altra e la si dichiara»**.

**3. La garanzia della rampa è enunciata in termini assoluti**, perché è la forma in cui è vera: l'importo che una flotta può dirottare per epoca è limitato dal fondo di genesi, indipendentemente dall'uso e dalla dimensione della flotta.

**Un rimedio apparente è nominato qui perché non venga implementato.** Un tetto del fondo **proporzionale al numero di eleggibili** sarebbe un tetto che una flotta alza gonfiando il denominatore, e riaprirebbe il criterio (a) di [ADR-007]. È la stessa trappola della ripartizione pesata raggiunta da un'altra strada, e va rifiutata con la stessa motivazione.

## Alternatives considered

- **Correggere solo la soglia, portandola dal 25% al 70,6%.** Necessaria e non sufficiente: rende onesta la dichiarazione e lascia l'importo assoluto a rischio al massimo esattamente quando la rete è più debole. Adottata come *parte* della decisione, rifiutata come decisione intera.
- **Fondo dimensionato al lancio, dichiarazione invariata al 25%.** Limita il danno assoluto ma continua a dichiarare un numero scelto e non misurato. Rifiutata: il progetto ha appena speso quattro giri di revisione per togliere le affermazioni non sostenute da [SPEC-006], e reintrodurne una qui sarebbe incoerente.
- **Tetto del fondo proporzionale agli eleggibili.** Rifiutata sopra, ed è il rimedio che sembra ovvio.
- **Accettare `α ≈ 1` in avviamento senza alcuna garanzia sostitutiva**, dichiarandolo e basta. È la posizione più semplice e sarebbe onesta, ma lascia senza risposta la domanda che un utente della prima ora porrà — *quanto posso perdere* — quando la risposta esiste ed è limitata.

## Consequences

- Il dimensionamento del fondo di genesi diventa una **decisione da prendere con un numero**, cioè la popolazione attesa al lancio. Non è fissata qui: appartiene alla spec che attua questa ADR insieme ad [ADR-010].
- La crescita del fondo verso la scala di riferimento richiede **governance attiva**: una successione di documenti al limite di variazione, con la spaziatura minima in altezze di catena. È un costo operativo ricorrente e va dichiarato come tale, non scoperto.
- La comunicazione di prodotto acquista una seconda frase e ne perde una falsa. Il reddito della prima ora è **più piccolo in valore assoluto** di quello a regime, per costruzione e non per accidente, e questo va detto a chi entra presto invece di essere lasciato scoprire.
- `RewardBounds` di [ADR-010] deve poter esprimere un tetto che il fondo di genesi rispetta e che la crescita successiva non viola: le due ADR si attuano insieme, in una sola spec.
- Il claim sulla resistenza ai Sybil passa da una frase condizionata a due frasi vere, ciascuna nel proprio regime. È un miglioramento di sostanza e non di forma: oggi la frase sulla quota è falsa per tutta la rampa.

## Review conditions

Rivedere se: la popolazione reale al lancio differisce dall'attesa per un ordine di grandezza, il che renderebbe il fondo di genesi sbagliato in una delle due direzioni e va corretto con la governance prevista invece che scoperto tardi; oppure se il costo operativo della crescita del fondo risultasse insostenibile per l'operatore, nel qual caso la leva è il limite di variazione di [ADR-010] e non il dimensionamento deciso qui. **Non rivedere** per rendere la dichiarazione più semplice: la semplicità di una frase sola è precisamente ciò che questa ADR ha stabilito essere falso.
