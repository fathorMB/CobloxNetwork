---
id: SKILL-001
# Note: Quote the title if it contains a colon
title: "Provare una gate in negativo, e verificare che i suoi casi varino"
status: active
scope: project
kind: verification
risk: low
applies_to: [AGENT-001, AGENT-002, AGENT-006, AGENT-007, AGENT-008]
domains: [verification, conformance, consensus, core, security]
commands: []
requires_operator_approval: false
links: [ADR-012]
created: 2026-08-26
updated: 2026-08-26
tags: [verification, gates]
activity:
  - date: 2026-08-26
    action: "transitioned proposed -> active"
---
# Provare una gate in negativo, e verificare che i suoi casi varino

## Purpose

**Una misura che non si è mai vista scattare è un calcolo, non una guardia.** Questa skill contiene la procedura con cui su Coblox si dimostra che una gate *vincola*, e la domanda — imparata a caro prezzo — che stabilisce se la prova sia sufficiente.

Fino al 2026-08-26 questa procedura viveva solo nei prompt di dispatch del Lead, riscritti a mano ogni volta. La conseguenza è misurata: la regola della **varianza dei casi** non è mai arrivata a chi ha scritto `GATE-MEASURE-BINDS` in [SPEC-016], perché il Lead l'ha imparata dopo. Quella gate è passata verde su un difetto `high`.

## When to use

Ogni volta che una spec ha una gate `before-submit`, senza eccezioni. Anche quando la gate è ovvia, anche quando passa al primo colpo, e **soprattutto** quando passa al primo colpo.

## Preconditions

- L'albero condiviso **non** si muta. Copiare l'albero in una directory temporanea e puntarci `COBLOX_REPO`, oppure lavorare su una copia dei file. Altri agenti lavorano in parallelo sullo stesso working tree.
- La gate deve già passare in positivo. Provare in negativo una gate rossa non dimostra nulla.

## Procedure

**1. Reintrodurre il difetto che la gate esiste per impedire.** Non un difetto qualsiasi: *quello*. Se la gate dice «nessuna regola sulla distanza fra `timestamp_ms`», il difetto da reintrodurre è una regola sulla distanza fra `timestamp_ms`, non un test rotto.

**2. Osservare il fallimento, e verificare che nomini la classe.** Un fallimento generico non distingue la guardia che ha morso da un errore di sintassi. La trascrizione deve riportare l'identificatore della classe o della probe.

**3. Ripristinare, e riverificare il verde.** Un ripristino non verificato lascia il dubbio che il verde iniziale fosse un artefatto.

**4. Chiedersi cosa hanno in comune tutti i casi.** *Se ogni caso di prova condivide lo stesso valore su una grandezza che non è quella sotto test, quella grandezza non è testata, ed è la prossima da variare.*

   Questo passo non è opzionale ed è quello che gli altri tre non sostituiscono. `GATE-MEASURE-BINDS` di [SPEC-016] provava tre catene — dentro banda, troppo veloce, troppo lenta — e **tutte e tre a latenza di rilascio zero**. La grandezza sotto test era la cadenza; la latenza era la grandezza costante. Il difetto stava nella latenza, ed è passato attraverso la gate *e* attraverso la review del Lead.

**5. Se la gate copre un insieme, provarne ogni elemento.** Una prova che mostra che *una* probe su 91 può fallire dimostra che una può fallire, e nulla sulle altre novanta. Una probe scritta contro un testo poi riscritto continua a eseguire e a passare: ha solo smesso di pinnare qualcosa.

**6. Se la gate legge una lista, chiedersi da dove viene la lista.** Una lista *dichiarata* — costante nel codice, campo in un manifesto — non è l'insieme reale. `SECURITY.md` è rimasto fuori dall'inventario di [ADR-012] per tutta la vita di quello strumento senza che nulla lo dicesse. Il lato «disco» va enumerato e confrontato nei due versi.

## Expected output

Una trascrizione che mostra, in ordine: il verde iniziale; la mutazione applicata e **quale difetto reintroduce**; il fallimento con la classe nominata; il ripristino e il verde riverificato; e la risposta esplicita al passo 4 — quale grandezza è costante in tutti i casi, e perché va bene che lo sia oppure quale caso nuovo è stato aggiunto.

## Failure handling

**Se la mutazione non fa fallire la gate**, la gate non vincola: è un finding, non un intoppo. Va riportato al Lead con la mutazione esatta, e la gate va riscritta o dichiarata insufficiente. Non aggiustare la mutazione finché non fallisce — è il modo di trasformare una prova in un rituale.

**Se il passo 4 rivela una grandezza costante e aggiungere il caso è costoso**, riportare invece di tacere. Un residuo dichiarato vale più di una gate che sembra completa.

## Evidence to record

La trascrizione intera in `### Verification transcript`, non un riassunto. Il comando esatto, l'output esatto, e la mutazione in forma di diff o di descrizione riproducibile. **Una previsione di comando non è evidenza di esecuzione** ([[QUALITY]]).
