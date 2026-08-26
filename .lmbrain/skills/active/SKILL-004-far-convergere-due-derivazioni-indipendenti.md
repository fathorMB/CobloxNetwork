---
id: SKILL-004
# Note: Quote the title if it contains a colon
title: "Far convergere due derivazioni indipendenti senza farle copiare l'una dall'altra"
status: active
scope: project
kind: verification
risk: low
applies_to: [AGENT-001, AGENT-002, AGENT-008]
domains: [verification, conformance, core, consensus]
commands: []
requires_operator_approval: false
links: [SPEC-010, SPEC-017, ADR-012]
created: 2026-08-26
updated: 2026-08-26
tags: [verification, conformance]
activity:
  - date: 2026-08-26
    action: "transitioned proposed -> active"
---
# Far convergere due derivazioni indipendenti senza farle copiare l'una dall'altra

## Purpose

Quando una regola rompe una **circolarità**, un'implementazione sola è internamente coerente *per costruzione* e non prova nulla. Serve una seconda strada — ed è precisamente il motivo per cui [DEBT-012] è rimasto invisibile fino a [SPEC-010].

Ma far *concordare* due strade è facile nel modo sbagliato. Questa skill contiene la procedura, trovata da AGENT-001 durante la remediation di [SPEC-017], che dimostra l'accordo invece di costruirlo.

## When to use

Ogni volta che una gate chiede due derivazioni indipendenti, o che un valore vada verificato contro un oracolo scritto apposta.

## Preconditions

- Le due strade non devono condividere codice. **Rileggere il codice della prima e riscriverlo in un altro linguaggio non è una seconda strada: è la stessa strada con un'altra sintassi.** La seconda si costruisce **dal documento**.
- Nessuna delle due deve importare l'altra, né invocarla, né leggerne l'output.

## Procedure

**1. Scrivere prima il documento, poi la seconda strada dal testo del documento.** Non dal codice, non dai propri appunti: dal testo appena pubblicato. Annotare **cosa si è letto** — è l'unica evidenza che la strada sia davvero seconda.

**2. Dare a entrambe lo stesso terzo, e a nessuna delle due l'altra.** I valori attesi vanno **pubblicati** — nel registro di `README.md` — e ciascuna strada si confronta con il documento. Nessuna strada asserisce contro una costante presa dall'altra: sarebbe **confermare quella strada invece di incontrarla**, cioè far concordare le due per costruzione.

**3. Non spuntare un accordo osservato a occhio.** Confrontare due output stampati è un'osservazione, non una guardia: se una strada derivasse domani, nessun test fallirebbe.

**4. La procedura delle righe a zero.** Prima di riempire la tabella, **pubblicare le righe a zero** e far fallire ciascuna strada separatamente. Ogni fallimento *nomina il valore che quella strada ha calcolato*. Se i due valori nominati coincidono, sono stati prodotti **prima che il documento li contenesse** e senza che nessuna delle due potesse copiarli. Solo allora riempire le righe e verificare il verde.

   Su [SPEC-017] la trascrizione mostra `sha256:6ba582b4…` nominato da entrambe le strade contro un documento che portava ancora zeri.

**5. Conservare un'asserzione propria sulla varianza.** Il confronto col documento non basta: due tabelle di digest che per caso coincidessero lo soddisferebbero senza dire nulla sulla grandezza che si voleva far variare. Va asserito **separatamente** che i valori si muovono quando quella grandezza si muove.

**6. Variare la grandezza che sarebbe rimasta costante.** Una seconda fixture che differisce per **un solo campo** — e per un campo di lunghezza in byte diversa, se la lunghezza entra nella preimmagine. È [SKILL-001] passo 4 applicato alle fixture invece che ai casi di prova.

**7. Difendere la fixture che sembra un doppione.** Una fixture che differisce per un campo verrà cancellata dal primo lettore che riordina, e con essa l'unico caso che esercita quella grandezza. Scrivere accanto **perché non è un doppione**, e tenerla con una probe.

## Expected output

- Cosa si è letto per costruire la seconda strada.
- La trascrizione delle **righe a zero**, con i valori che ciascuna strada nomina, leggibile come tale.
- La trascrizione del verde dopo il riempimento.
- L'asserzione di varianza, separata dal confronto col documento.

## Failure handling

**Se le due strade non concordano**, non aggiustare la seconda finché non concorda: è il modo di trasformare un oracolo in un'eco. Stabilire **quale** delle due sbaglia leggendo il documento, e se sbaglia il documento, fermarsi e riportare.

**Se pubblicare la fixture avesse un costo** — insegna una configurazione inammissibile, cioè famiglia 1 — riportare invece di eseguire. **Ma un oracolo fuori dal registro è peggio**: è un oracolo che nessuna implementazione indipendente riceve, cioè l'opposto di ciò che una fixture serve a fare.

## Evidence to record

Le trascrizioni integrali, e in particolare quella delle righe a zero: è la sola che distingue un accordo dimostrato da un accordo costruito, e da un riassunto non è ricostruibile.
