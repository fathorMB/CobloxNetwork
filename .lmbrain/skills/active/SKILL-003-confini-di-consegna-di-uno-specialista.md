---
id: SKILL-003
# Note: Quote the title if it contains a colon
title: "Confini di consegna di uno specialista su Coblox"
status: active
scope: project
kind: process
risk: low
applies_to: [AGENT-001, AGENT-002, AGENT-006, AGENT-007, AGENT-008]
domains: [process]
commands: []
requires_operator_approval: false
links: [SPEC-014]
created: 2026-08-26
updated: 2026-08-26
tags: [process, handoff]
activity:
  - date: 2026-08-26
    action: "transitioned proposed -> active"
---
# Confini di consegna di uno specialista su Coblox

## Purpose

Le regole di consegna che il Lead ha finora riscritto a mano in ogni dispatch, cinque volte in una notte e ogni volta leggermente diverse. Una regola riscritta a mano è una regola che prima o poi manca.

## When to use

Sempre, da `spec_start` a `spec_submit`.

## Preconditions

Nessuna.

## Procedure

### Git

- **Non eseguire `git commit` né `git push`.** Il Lead è l'unico che committa e spinge. Il lavoro si lascia nell'albero e si riporta.
- **Non eseguire `git add -A`, `git add .`, né alcuno staging ampio.** Su questo progetto è normale che più agenti lavorino sullo stesso working tree nello stesso momento.
- La strategia dichiarata è `main-only` con push riservato al Lead. Non creare branch se non richiesto.

### Perimetro

- Implementare **solo** lo scopo dichiarato. Se la spec esclude un file, quel file non si tocca — nemmeno per una correzione ovvia trovata di passaggio.
- Un difetto trovato fuori scopo si **riporta**, non si corregge. Diventa un debito, e il debito riceve la gate che quella correzione meriterebbe. Correggerlo dentro una spec che non ha quella gate significa scavalcarla.
- **Fermarsi e riportare è un esito previsto**, non un fallimento. Vale in particolare quando la chiusura richiederebbe di muovere un valore pubblicato o di cambiare una decisione accettata.

### Lingua e convenzioni

- Testo rivolto al prodotto — `docs/`, `README`, `SECURITY.md`, l'interfaccia — in **inglese**.
- Artefatti del cervello, sotto `.lmbrain/`, in **italiano**.
- Le convenzioni sull'**unità del token** — nome, posizione, separatore, simboli ritirati — sono in [ADR-009] e si applicano al testo rivolto al prodotto. **Non sono ripetute qui**: questa skill si legge a ogni dispatch, e ogni riga che non riguarda chi la sta leggendo la fa scremare. Segnalato da AGENT-002 al primo impiego, su una spec di consenso a cui quelle convenzioni non si applicavano.

### Lifecycle

- `lmbrain__spec_start` come prima azione, **se e solo se** la spec è in `ready`. Se è già in `working`, non chiamarlo.
- `lmbrain__spec_submit` quando **tutte** le gate `before-submit` sono spuntate con la loro evidenza.
- In remediation dopo una review, la spec **resta in `review`**: non riportarla a `working`. La remediation è la continuazione del giro di review, non un azzeramento della lifecycle.

## Expected output

Un rapporto in prosa italiana che contenga, oltre a cosa è stato fatto:

- **Ciò che non è stato fatto, e perché.**
- **Ogni punto in cui la spec è sembrata sbagliata.** Su questo progetto è già successo cinque volte che un agente avesse ragione contro il Lead, e il Lead preferisce un disaccordo motivato a un'esecuzione fedele di un'istruzione difettosa.
- I conteggi dei test prima e dopo.

## Failure handling

Se un'istruzione della spec contraddice una decisione accettata o una di queste regole, **non risolvere il conflitto in silenzio**: nominarlo e riportarlo. Una contraddizione risolta senza dirlo diventa un precedente che nessuno ha deciso.

## Evidence to record

I file cambiati, i comandi eseguiti con il loro output, e i limiti noti. Una previsione di comando o un elenco di controlli intenzionali **non è evidenza di esecuzione** ([[QUALITY]]).
