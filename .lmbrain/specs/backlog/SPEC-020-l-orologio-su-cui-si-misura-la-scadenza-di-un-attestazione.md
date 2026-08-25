---
id: SPEC-020
# Note: Quote the title if it contains a colon
title: "L'orologio su cui si misura la scadenza di un'attestazione"
status: backlog
kind: feature
priority: medium
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: [SPEC-016, SPEC-018]
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012, ADR-013, ADR-015]
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [security, identity, light-client]
---

# L'orologio su cui si misura la scadenza di un'attestazione

## Objective

Chiudere [DEBT-017], **che non è il debito che il suo titolo diceva**. Il titolo originale — *«la finestra è skew più durata, e solo la durata è limitata»* — è vero e nomina il termine **minore**. L'esposizione reale è `D_max + S_max + (ritardo dell'orologio del ricevente)`, e **solo il terzo addendo è illimitato**.

Il termine illimitato è già dichiarato in `identity.md` §*Declared limit*: *«A receiver whose clock is far behind accepts attestations that expired hours ago, and no certificate attests a clock.»* Questa spec chiude quel termine, o dichiara motivatamente perché non lo chiude.

## Context

**La premessa da cui il Lead era partito era sbagliata, e la correzione è la sostanza di questa spec.** Il Lead aveva chiesto ad AGENT-007 se il rimedio richiedesse un orologio che la catena non ha, citando [ADR-013]. La risposta: [ADR-013] stabilisce che nessuna regola interna può **vincolare il tempo reale**, ed è vero — ma qui non serve vincolarlo. Serve un **minorante** di `now_ms` per chi verifica. E per un minorante l'abuso dei validatori è **fail-closed**:

- il `timestamp_ms` dei blocchi è monotono crescente sulla mediana degli undici precedenti, quindi l'ultimo blocco finalizzato è un minorante non decrescente;
- gonfiarlo fa **rifiutare** più attestazioni, non accettarne di più longeve;
- sgonfiarlo è impedito dalla mediana.

Ne discende un rimedio scrivibile con l'orologio che la catena **già ha**: chi possiede una testa finalizzata valuta con `now_ms = max(orologio locale, timestamp_ms dell'ultimo blocco finalizzato)`; chi è in bootstrap ricade sull'orologio locale come oggi.

**Il costo che quella valutazione non nomina, e che decide la spec.** *Fail-closed* qualifica la sicurezza, non la disponibilità, e qui le due divergono. `identity.md` non dice che un peer senza attestazione valida viene ignorato: dice **MUST be rejected and disconnected**. Un set di validatori che gonfia `timestamp_ms` non degrada una verifica — **disconnette la rete**, su ogni nodo onesto che possieda una testa finalizzata, simultaneamente, senza firmare nulla di invalido.

Ed è **esattamente l'attore di [DEBT-013], con esattamente la capacità che [DEBT-013] gli ha accertato.** Quel debito ha stabilito che il set attivo scrive gli orologi della catena e che un terzo bloccante può muoverli con la catena viva e ogni blocco valido. Il rimedio prenderebbe quella capacità — oggi limitata a **rallentare** — e la convertirebbe in una leva di **partizione del trasporto**. Un rallentamento è invisibile e reversibile; una disconnessione di massa è immediata.

**Una sola domanda decide quale rimedio sia il più forte.** L'ampiezza della leva dipende interamente dall'unica interazione che AGENT-007 ha dichiarato **non istruita**: se il controllo di drift verso l'alto su `timestamp_ms` sia applicato dai validatori onesti in accettazione del blocco. Se sì, l'inflazione è limitata dal loro orologio e la leva è piccola. Se no, la leva è illimitata e **l'ordine di forza si inverte**.

La stessa domanda regge una seconda proprietà, che AGENT-007 aveva nominato: se quel controllo si applicasse ai blocchi storici durante il sync, un nodo con orologio molto indietro rifiuterebbe i blocchi recenti e **non acquisirebbe mai la testa che gli servirebbe** — la circolarità che l'attestazione esisteva per evitare, rientrata da un'altra porta.

**Perché dipende da [SPEC-016].** Entrambe cercano un orologio esterno alla volontà di chi verifica. [SPEC-016] ne costruisce uno attorno al checkpoint di soggettività debole. Se questa spec ne costruisse un altro, il progetto avrebbe **due orologi diversi** per la stessa proprietà, e nessuno saprebbe quale sia quello vero.

## Scope

### Included

- La risposta alla domanda sul controllo di drift, **prima** di scrivere qualunque regola.
- Il minorante su `now_ms` per la valutazione delle regole di scadenza dell'attestazione, se la risposta lo consente.
- La misura e la dichiarazione dell'ampiezza della leva di partizione, se il minorante viene adottato.
- La dichiarazione della somma `D_max + S_max` accanto ai due parametri, **in ogni caso**, e la dichiarazione esplicita del residuo.
- La riscrittura della contromisura (a) di TM-37, coordinata con [SPEC-018].

### Excluded

- **Un vincolo relazionale fra `S_max` e `D_max` come chiusura.** Chiude solo l'evasione della regola 4 da parte di un emittente che postdata, e lascia intatto il termine illimitato: è la famiglia 3 commessa dentro il rimedio, ed è il rimedio che il debito originale suggeriva. Ammesso solo **in aggiunta** e mai come chiusura, e solo se accompagnato dalla dichiarazione del residuo.
- **Ritarare la tolleranza in genesi.** [ADR-010]: un valore scelto bene non è una proprietà. E `S_max` è un parametro **a due pericoli** — verso l'alto l'evasione del tetto, verso il basso l'isolamento del nodo con orologio indietro.
- **Concedere tolleranza oltre `expires_at_ms` per simmetria.** Aggiungerebbe un quarto addendo limitato a un termine già illimitato.
- **Invalidazione anticipata con epoca o numero di serie.** Già respinta in TM-37(c): un contatore per identità osservabile in sessione è un identificatore stabile in più, e ricrea la correlazione che [ADR-015] ha tolto.
- Qualunque modifica alla meccanica dell'envelope. Le due finestre governano proprietà indipendenti e **non esiste una relazione da scrivere fra loro**.

## Existing-project analysis

La finestra `D_max + S_max` è accertata riga per riga in `identity.md` §*rejection rules* e implementata in `core/coblox-core/src/identity.rs`, già asserita in negativo dalla suite di conformità che verifica il rifiuto un millisecondo prima del bordo.

`S_max` **non è disponibile all'avversario di TM-37**: postdatare richiede la chiave di identità, che quell'avversario non ha. L'unico abuso reale del termine è un nodo che collude con sé stesso per consegnare a un confederato una credenziale più longeva del tetto.

`D_max` e `S_max` non sono fissati da alcun documento di genesi. La loro taratura **non cambia** questa spec, perché il termine dominante non dipende da nessuno dei due.

## Technical proposal

**La verifica dell'interazione non è una precondizione fra le altre: è il primo passo, e il suo esito sceglie il rimedio.** Va istruita e riportata prima che una riga di regola sia scritta, e riportata **anche se l'esito è scomodo**.

Se l'inflazione è limitata dagli onesti in accettazione: il minorante è l'esito più forte, e la leva va misurata e dichiarata, non solo nominata. La misura è la distanza massima che il set può spingere il minorante oltre il tempo reale prima che un blocco venga rifiutato.

Se l'inflazione non è limitata: il minorante trasferisce un rischio da rallentamento a partizione, e l'ordine di forza si inverte. La chiusura diventa la dichiarazione della somma e del residuo, più eventualmente il vincolo relazionale — **e va detto esattamente che chiude l'evasione e nient'altro**, perché un lettore che vede il vincolo conclude che l'esposizione sia limitata mentre è limitata solo la parte che i due parametri governano.

**Il checkpoint di soggettività debole è la seconda fonte di minorante**, più debole in freschezza ma disponibile **anche al nodo in bootstrap**. Va considerato in alternativa o in aggiunta, e **deve essere lo stesso oggetto che [SPEC-016] costruisce**.

## Files and areas involved

- `docs/protocol/identity.md` — la regola, la somma dichiarata, il residuo.
- `core/coblox-core/src/identity.rs`, `light_client.rs` — il minorante e la ricaduta di bootstrap.
- `core/coblox-core/tests/` — le prove in negativo e il caso di bootstrap.
- `.lmbrain/knowledge/threat-model.md` — la contromisura (a) di TM-37, coordinata con [SPEC-018].
- `sim/tools/` — la gate di [ADR-012], se una regola viene scritta.

## Acceptance criteria

- [ ] La domanda sul controllo di drift è **risolta e riportata**, prima di ogni regola, e il suo esito è quello che ha scelto il rimedio.
- [ ] Se il minorante è adottato: l'ampiezza della leva di partizione è **misurata e dichiarata**, non solo nominata.
- [ ] Se il minorante è adottato: un nodo **senza** testa finalizzata si comporta esattamente come oggi, e una prova lo mostra.
- [ ] L'orologio esterno usato è **lo stesso** che [SPEC-016] ha costruito, e non un secondo.
- [ ] La somma `D_max + S_max` è dichiarata accanto ai due parametri, e il **residuo illimitato** è dichiarato con essa — in ogni caso, anche se il minorante non viene adottato.
- [ ] Nessuno dei quattro rimedi esclusi è stato adottato come chiusura.
- [ ] Se una regola di validità è stata scritta: la gate di [ADR-012] è eseguita e la trascrizione allegata.

## Implementation plan

1. Istruire e riportare la domanda sul controllo di drift. Nessuna regola prima.
2. Scegliere il rimedio in base all'esito, e dichiarare la scelta con la sua ragione.
3. Se minorante: implementarlo, misurare la leva, provare il caso di bootstrap.
4. La somma e il residuo dichiarati, in ogni caso.
5. TM-37(a), coordinata con [SPEC-018].
6. Gate di [ADR-012], se applicabile.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-DRIFT-ANSWERED-FIRST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La domanda sul controllo di drift verso l'alto è risolta e riportata **prima** che una regola sia scritta, e la trascrizione mostra l'ordine. Non è una precondizione fra le altre: il suo esito decide se il minorante sia il rimedio più forte o il più caro, e una regola scritta prima della risposta sarebbe stata scelta senza sapere.
- [ ] GATE-LEVER-MEASURED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Se il minorante è adottato, l'ampiezza della leva di partizione è **misurata**: la distanza massima che un set può spingere il minorante oltre il tempo reale prima che un blocco sia rifiutato. Una leva nominata e non misurata è una minaccia dichiarata a spanne, e questa nasce dalla capacità che [DEBT-013] ha già accertato a quello stesso attore.
- [ ] GATE-BOOTSTRAP-UNCHANGED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Un nodo **senza** testa finalizzata si comporta esattamente come oggi, e una prova lo mostra. È il caso che la ragione 1 di `identity.md` protegge deliberatamente, ed è dove la circolarità rientrerebbe se la ricaduta fosse imperfetta.
- [ ] GATE-ONE-CLOCK | kind=manual | owner=agent | phase=before-submit | evidence=transcript | L'orologio esterno usato è **lo stesso oggetto** che [SPEC-016] ha costruito, e la trascrizione lo mostra citando il codice condiviso. Due orologi per la stessa proprietà sono peggio di nessuno: nessuno saprebbe quale sia quello vero.
- [ ] GATE-NO-EXCLUDED-REMEDY | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una ricerca sul diff mostra che nessuno dei quattro rimedi esclusi è stato adottato **come chiusura**: né il vincolo relazionale da solo, né una ritaratura di `S_max` o `D_max`, né tolleranza oltre `expires_at_ms`, né un contatore di invalidazione. Sono i quattro che sembrano ovvi, ed è la ragione per cui la gate li cerca.
- [ ] GATE-RESIDUAL-DECLARED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La somma `D_max + S_max` **e il residuo illimitato** sono dichiarati accanto ai parametri, anche se il minorante è stato adottato e anche se lo chiude. Un lettore che vede solo la somma conclude che l'esposizione sia limitata: è la famiglia 2, la pretesa avanti rispetto alla regola, su un documento che parla di scadenze.
- [ ] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Se una regola di validità è stata scritta, la passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato e la trascrizione allegata. Se **nessuna** regola è stata scritta, la gate è dichiarata non applicabile con la ragione.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la chiusura e il Lead ha accettato la review. È lei ad aver stabilito che il minorante è fail-closed, ed è il Lead ad aver aggiunto che *fail-closed* qualifica la sicurezza e non la disponibilità: la review deve pronunciarsi su entrambe le affermazioni, comprese quelle del Lead.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è chiudere il termine sbagliato.** Il debito originale suggeriva il vincolo relazionale, il titolo originale nominava il termine minore, e il dispatch del Lead partiva da una premessa falsa. Tre indicazioni convergenti verso la grandezza sbagliata: se questa spec le segue, la famiglia 3 diventa la settima occorrenza.
- **Il rischio secondario è il rimedio che sposta il rischio invece di ridurlo.** Il minorante converte una leva di rallentamento in una leva di partizione, e la seconda è peggiore. È ammissibile solo se misurata.
- **Un rischio terzo, ed è di coordinamento.** [SPEC-016] è in corso sullo stesso oggetto. Se questa spec parte prima che quella sia atterrata, costruirà un secondo orologio senza accorgersene, perché il primo non sarà ancora nel codice da leggere.
- **`D_max` e `S_max` restano non fissati** da alcun documento di genesi. Non è compito di questa spec fissarli, e la loro taratura non ne cambia le conclusioni; ma il fatto va lasciato scritto e non chiuso in silenzio.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Fermarsi e riportare è l'esito previsto** se la domanda sul controllo di drift avesse una risposta che rende il minorante impraticabile: in quel caso la chiusura è la dichiarazione, ed è un esito ammissibile e non un fallimento.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

### Files changed
