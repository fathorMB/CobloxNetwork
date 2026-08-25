---
title: Disciplina del ciclo di review
updated: 2026-08-25
---

# Disciplina del ciclo di review

Pagina nata da un difetto reale trovato nel brain, non da una buona intenzione. Vale per chiunque ricopra il ruolo di Lead e per ogni profilo con `can_review: true`.

## Il fatto

Il 2026-08-25 l'operatore ha notato dalla board **tre review ferme in `changes-requested` mentre la spec relativa era `done` da un giorno**: `REVIEW-001`, `REVIEW-002` e `REVIEW-006`, tutte su [SPEC-001].

Nessuna era un errore di merito. I loro finding erano stati chiusi, e in due casi la chiusura era stata verificata e registrata. Mancava solo il verdetto finale.

## La causa

Durante le tre remediation di [SPEC-001] è stata creata **una review nuova a ogni giro** — `REVIEW-002` → `REVIEW-006` → `REVIEW-007` — invece di ri-esprimere il verdetto su quella esistente. Ogni review a monte è rimasta con l'ultimo verdetto emesso, che era `changes-requested`, e nessuno è tornato a chiuderla.

`spec_done` verifica i gate, non lo stato delle review. Anche `lmbrain_validate` non segnala nulla. **Il difetto è quindi invisibile agli strumenti e visibile solo sulla board**, che è il posto peggiore per un difetto: chi riprende il progetto vede richieste di modifica aperte su lavoro chiuso e non sa se siano reali.

## La regola

**Una review porta il verdetto del giro in cui vive, e ogni review deve arrivare a uno stato terminale.**

Preferire il **ri-verdetto sullo stesso artefatto**: `review_changes_requested` → remediation → `review_remediation_verified` → `review_accept`. La storia degli eventi è append-only, quindi il giro precedente non si perde. È il modo usato su [SPEC-006], dove `REVIEW-009` e `REVIEW-010` hanno attraversato quattro giri restando ciascuna un solo artefatto — `REVIEW-010` ne conserva quattro sezioni in coda, una per giro.

Creare una review nuova è legittimo quando il **perimetro** cambia — una revisione di sicurezza distinta da quella di Lead, o una ri-verifica mirata con un mandato diverso. In quel caso **la review precedente va chiusa esplicitamente**, e la disposizione non è automatica:

- `review_accept` se i suoi finding sono stati chiusi e verificati. È il caso di `REVIEW-001`.
- `review_supersede` se è stata **rimpiazzata** da una review successiva sullo stesso gate senza essere mai arrivata all'accettazione. È il caso di `REVIEW-002` e `REVIEW-006`.

**Non accettare una review superata.** Sembra più ordinato sulla board e riscrive la storia in meglio: dichiara superato un giro che si era chiuso con dei residui. `REVIEW-006` aprì quattro finding `high` su tre chiusure che `REVIEW-002` aveva ritenuto sufficienti — registrarla come accettata cancellerebbe proprio l'informazione che rende leggibile la catena.

## Controllo prima di `spec_done`

Prima di chiudere una spec, verificare che **nessuna review che la riguarda sia in `pending`, `changes-requested` o `blocked`**. Nessuno strumento lo impone, quindi è una verifica del Lead:

```bash
ls .lmbrain/reviews/pending .lmbrain/reviews/changes-requested .lmbrain/reviews/blocked
```

Se qualcosa resta lì mentre la spec sta per chiudersi, o quella review va chiusa con il verdetto giusto, o la spec non è pronta.

## Cosa c'era di buono sotto il difetto

Le tre review appese contengono il materiale migliore prodotto sul protocollo v0, ed è la ragione per cui la disposizione è stata scelta caso per caso invece che in blocco. In `REVIEW-007` la reviewer scrive che una **propria** condizione di chiusura era sbagliata e che la forma alternativa scelta dall'implementatore era migliore della sua. Quel tipo di annotazione è il valore vero di una review, e va conservato con lo stato che dice il vero su come quel giro si è concluso.
