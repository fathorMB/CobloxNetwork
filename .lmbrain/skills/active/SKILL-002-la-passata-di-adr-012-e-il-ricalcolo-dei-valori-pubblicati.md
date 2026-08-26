---
id: SKILL-002
# Note: Quote the title if it contains a colon
title: "La passata di ADR-012, e il ricalcolo di un valore pubblicato"
status: active
scope: project
kind: verification
risk: low
applies_to: [AGENT-001, AGENT-002, AGENT-006, AGENT-008]
domains: [verification, conformance, ledger, core]
commands:
  - "python sim/tools/published_artifacts.py"
  - "python sim/tools/published_artifacts_negative.py"
  - "python sim/tools/protocol_hashes.py"
requires_operator_approval: false
links: [ADR-012, SPEC-010]
created: 2026-08-26
updated: 2026-08-26
tags: [verification, conformance]
activity:
  - date: 2026-08-26
    action: "transitioned proposed -> active"
---
# La passata di ADR-012, e il ricalcolo di un valore pubblicato

## Purpose

[ADR-012] impone che **ogni spec che introduce o modifica una regola di validità esegua una passata su tutti gli artefatti pubblicati**, con uno strumento versionato e provato in negativo. Questa skill dice come si esegue e cosa si allega.

Contiene anche la procedura per **ricalcolare un valore pubblicato**, che è la sede in cui questo progetto ha sbagliato più spesso: un hash ricalcolato con un metodo mai validato è un valore nuovo, non un valore corretto.

## When to use

- Sempre, quando la spec ha `GATE-ADR012`.
- Ogni volta che un valore pubblicato cambia — hash nei documenti di protocollo, `consensus_parameters_hash`, hash degli `ER-*`, fixture.
- **Anche quando si è convinti che nulla sia cambiato.** La gate va eseguita e la trascrizione allegata *anche se non trova nulla*: è quel caso che dimostra che la passata è stata fatta.

## Preconditions

- Le mutazioni vanno eseguite su una copia dell'albero (`COBLOX_REPO` puntato altrove), mai sull'albero condiviso.
- Conoscere lo stato di partenza: quante probe, quali classi. Un conteggio che scende senza che nessuno lo noti è una copertura persa in silenzio.

## Procedure

### La passata

1. `python sim/tools/published_artifacts.py` — deve dare `PASS`. L'output riporta i candidati per classe: **annotare i conteggi prima e dopo il proprio lavoro**.
2. `python sim/tools/published_artifacts_negative.py` — deve dare `PASS`. Verifica che ogni classe di difetto sia osservata fallire, **e ogni probe individualmente**.
3. Se si aggiungono probe: **le proprie devono entrare nel conteggio individuale**. Che il tool passi non basta.
4. Allegare entrambe le trascrizioni.

**Se la passata fallisce su qualcosa che non è opera propria**, non correggerlo: isolarlo, dimostrare che il proprio diff non introduce fallimenti nuovi, e riportarlo come preesistente. Correggere un artefatto fuori scopo dentro una spec significa scavalcare la gate che quell'artefatto meriterebbe.

### Il ricalcolo di un valore pubblicato

**La regola è una sola e non ha eccezioni: il metodo si valida prima su un valore che non è cambiato, poi si applica a quello che è cambiato.**

1. Prendere un valore pubblicato **non toccato** dal proprio lavoro.
2. Ricalcolarlo con il metodo che si intende usare, e verificare che riproduca il valore già pubblicato.
3. Solo allora applicare lo stesso metodo al valore che cambia.
4. `python sim/tools/protocol_hashes.py` deve dare `PASS`, e la trascrizione dice **quali valori sono cambiati e quali no**.

Il passo 2 è quello che si salta quando si ha fretta, ed è quello che distingue un valore corretto da un valore nuovo. Senza di esso, un metodo sbagliato produce un hash coerente con se stesso e con nient'altro — e nessun test lo rileva, perché l'unica cosa che poteva rilevarlo era il confronto con un valore noto.

## Expected output

- I conteggi delle classi prima e dopo, con la differenza spiegata.
- La trascrizione di `published_artifacts.py`, anche se `PASS` senza modifiche.
- La trascrizione di `published_artifacts_negative.py`.
- Per ogni valore pubblicato cambiato: la validazione del metodo sul valore invariato, **poi** il nuovo valore.
- L'elenco esplicito dei valori pubblicati **non** cambiati, quando la spec avrebbe potuto cambiarli.

**Se nessun valore e' cambiato, la passata si esegue lo stesso e l'evidenza si scrive lo stesso.** E' il caso che dimostra di averla fatta, ed e' quello in cui e' piu' facile saltarla: la riga «per ogni valore pubblicato cambiato...» qui sopra non ha soggetto, e un lettore frettoloso legge l'assenza di variati come assenza di obbligo. Cio' che va scritto in quel caso e': la trascrizione di `published_artifacts.py`, quella di `protocol_hashes.py`, e **la frase che dice che nessun valore e' cambiato e perche'** — di norma perche' la spec non introduce preimmagini ne' tocca fixture. Segnalato da AGENT-002 al primo impiego di questa skill.

## Failure handling

**Se la passata trova un difetto in un artefatto che la spec non doveva toccare**, riportare al Lead: è un debito, non un fix da infilare qui.

**Se il ricalcolo di un valore invariato non riproduce il valore pubblicato**, fermarsi. Il metodo è sbagliato, oppure il valore pubblicato lo è: in entrambi i casi è una decisione del Lead e apre la propria passata.

## Evidence to record

Le trascrizioni integrali in `### Verification transcript`, con i comandi esatti. Per i valori pubblicati, la coppia *validazione su invariato → applicazione al variato* deve essere leggibile come tale, non ricostruibile.
