---
id: ADR-017
# Note: Quote the title if it contains a colon
title: "La revoca ha due lavori, e il pavimento ne giustifica uno solo"
status: proposed
decision_date: 2026-08-26
decider: AGENT-LEAD
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: [ADR-010, ADR-015, ADR-012]
tags: [consensus, security, identity]
created: 2026-08-26
updated: 2026-08-26
activity:
  - date: 2026-08-26
    action: "created"
---
# La revoca ha due lavori, e il pavimento ne giustifica uno solo

> Proposta dal Lead il 2026-08-26 per chiudere [DEBT-033] e [DEBT-034]. **Non ancora decisa.**

## Context

[SPEC-019] ha fissato una definizione unica di *enrolled, unrevoked* per autorizzare una transazione, ancorandola a `effective_height`, e ha scartato la lettura *«revoca finalizzata sotto `h`»* con un argomento che regge: quella lettura dipende da quali certificati un nodo ha raccolto, quindi **non è una regola stretta con margine largo, è un fork con una specifica**.

Chiudendola, [SPEC-019] ha spostato peso su `effective_height`. Prima quel campo governava la transizione del set di validatori; ora governa **anche** se una chiave revocata possa svuotare un saldo. E il documento lo ha dichiarato invece di tacerlo: `ledger.md` dice in chiaro che *nulla in v0 limita `effective_height` dall'alto*, e che finché quel campo non è limitato **quanto una revoca protegga un saldo lo sceglie il quorum che revoca**.

Lo stesso documento nomina le tre domande rimaste aperte — quanto debba essere corta la finestra, se il pavimento debba dipendere da `reason`, cosa limiti `effective_height` dall'alto — e le classifica come *«meccanica della revoca»*, aggiungendo che quel lavoro **è lavoro che nessuno ha fatto, non una cosa che non si possa fare**. Questo ADR è quel lavoro.

### I fatti verificati, con il perimetro su cui valgono

Verificati dal Lead sull'albero a `3f1bef7`, non ereditati dagli artefatti che li affermano:

- **`effective_height` è nominato da esattamente due MUST.** Verificato **contando**: `effective_height` compare **trenta volte** in `ledger.md`, di cui tre su righe che portano `MUST`. Una delle tre — *«satisfies every MUST of this document»* — è prosa **su** i MUST e non è un MUST. Restano la clausola 4 della regola di transizione forzata, che è un **pavimento** (`min_revocation_effective_delay_blocks` sopra il blocco proponente), e la regola del light client. **Nessun altro MUST lo nomina**, per esaurimento delle trenta.
- **Un solo riferimento lo limita verso l'alto, ed è la clausola 8** della regola di contrazione del set. Verificato **enumerando** le trenta occorrenze e classificandole: le clausole 1, 2 e 6 confrontano `effective_height` con un'altra altezza, ma lo usano come **soglia da cui** un set o un blocco diventa invalido, cioè non lo limitano affatto; la clausola 8 è **la sola** che lo pone *sotto* un'altra grandezza (`is at most that activation_height`). E non è un tetto: dice che una revoca non può giustificare una contrazione più alta della propria efficacia. Una revoca con `effective_height` assurdo resta valida.
- **`reason` è inerte.** Quattro occorrenze in tutto l'albero: la dichiarazione dello schema, la fixture canonica, la copia della fixture in un test di serializzazione, e la probe dell'inventario di [ADR-012]. **Nessuna regola lo legge, nessun codice lo ramifica.** È però già impegnato nell'ID della transazione, quindi renderlo letto **non cambia il formato**.
- **`ChallengeEvidenceBody` porta `completed_at_ms` e nessun campo di altezza.** Questo è il fatto che nessuno dei due debiti riporta, e capovolge il costo del rimedio di [DEBT-034]: far portare all'evidenza l'altezza a cui l'auditor ha giudicato **non è rendere letto un campo esistente, è aggiungerne uno**. È l'opposto di [DEBT-033], dove il campo c'è già.
- **`min_revocation_effective_delay_blocks` è un parametro governato senza valore di lancio.** Verificato **contando**, sul perimetro `docs/` + `core/` + `sim/` e non sul brain: **diciassette occorrenze** — sei di prosa e regole in `ledger.md`, una nello schema di `README.md`, quattro in `params.rs` che lo dichiarano e lo leggono senza vincolarlo, quattro in `tests/common/mod.rs`, una in `protocol_hashes.py`, una nella probe di `published_artifacts.toml`. **Tre soltanto gli assegnano un valore** — `mod.rs:242`, `mod.rs:516`, `protocol_hashes.py:142` — e tutte e tre assegnano `1`, e tutte e tre sono fixture. Non compare nemmeno nella lista DRAFT dei parametri di lancio, che quindi non lo dichiara aperto.

### Il fatto che governa la decisione

**Il pavimento ha una giustificazione, e quella giustificazione esiste su un percorso solo.**

`min_revocation_effective_delay_blocks` esiste per dare ai validatori superstiti una finestra dichiarata in cui impegnare un set successore conforme, così che la catena non si fermi. È una ragione di **liveness del set**.

Su un saldo quella ragione non esiste. **Un saldo non ha bisogno di una finestra per essere protetto: ne ha bisogno il set.** Il pavimento è quindi giusto per un percorso e arbitrario per l'altro, ed è proprio sul percorso in cui è arbitrario che [SPEC-019] ha appena messo il peso.

### La cosa che va guardata prima di scegliere, perché cambia il costo di un'opzione

[DEBT-033] scarta la via *«far mordere la revoca sul percorso di spesa a `min(effective_height, proponente + pavimento)`»* perché *«reintrodurrebbe due significati di "revocata" alla stessa altezza, cioè esattamente ciò contro cui [SPEC-019] ha argomentato»*.

**Quella lettura eredita la conclusione di [SPEC-019] senza il suo confine.** L'argomento di [SPEC-019] contro la seconda lettura aveva due parti, e solo una era fatale:

1. la lettura era **dipendente dal verificatore** — due nodi con certificati diversi danno verdetti diversi. **Questa è la parte fatale**, ed è un fork.
2. dava **due significati di *revocata* alla stessa altezza**. Questa è una parte di leggibilità, non di correttezza.

`min(effective_height, proponente + pavimento)` innesca la **(2)** e non la **(1)**: entrambe le grandezze sono nel corpo della transazione, lette dagli stessi byte da ogni verificatore, e la funzione è totale sul blocco e i suoi antenati e monotona in `h`. Non è un fork.

E la **(2)** è già lo stato del protocollo, in forma più forte: `ledger.md` dichiara che dentro la finestra un nodo è **contemporaneamente** autorizzato a spendere, contato a piena potenza di voto, e **irraggiungibile da ogni peer conforme**. Sono già **tre** risposte diverse alla stessa altezza. Il peccato non è averne una in più: è averne una **non dichiarata**.

## Decision

> **Da decidere dall'operatore.** Quanto segue è la proposta del Lead.

La revoca fa due lavori — togliere una chiave dal set, e togliere a una chiave il potere di spendere — e questo ADR li separa lungo la linea su cui corre la giustificazione del pavimento.

### 1. Il percorso di spesa non ha pavimento: la revoca morde all'inclusione

Sul percorso di autorizzazione delle transazioni, una revoca qualifica la chiave a partire dall'**altezza del blocco che la include**, non da `effective_height`.

È la **terza lettura** che `ledger.md` descrive e certifica: *«un fatto sul blocco e i suoi antenati, monotono in `h`, letto dagli stessi byte da ogni verificatore»*, e **chiude la finestra**. Il documento non l'ha adottata per un motivo solo — che adottarla significa ridefinire cos'è `effective_height`, cioè meccanica della revoca — e questo ADR è la sede in cui quella ridefinizione si fa.

**Cosa chiude:** l'intera superficie di [DEBT-033] sul percorso di spesa. Non la limita, la toglie: non c'è più alcuna grandezza che il quorum sceglie e da cui dipende quanto una revoca protegga un saldo.

### 2. Il percorso del set tiene il pavimento e guadagna un tetto, e il tetto dipende da `reason`

Sul percorso della transizione del set, `effective_height` conserva il significato che ha oggi e il pavimento resta, perché lì la sua ragione esiste. Guadagna un tetto, e **il tetto non è un parametro nuovo: è `reason`, reso letto**.

| `reason` | Vincolo su `effective_height` | Perché |
| --- | --- | --- |
| `key_compromise` | **derivato**: `= proponente + pavimento`, esattamente | La chiave è in mano al nemico. Il set ha bisogno della sua finestra, e di nient'altro. **Nessuna discrezione.** |
| `validator_misconduct` | `<= proponente + 2 × pavimento` | Cattiva condotta non implica chiave compromessa: un margine di programmazione è legittimo, il doppio non lo è. |
| `operator_request` | `<= proponente + max_planned_revocation_delay_blocks` | Uscita volontaria e programmata. La latitudine è il punto, e va dichiarata come parametro invece che come assenza di regola. |

Su `key_compromise` — **il caso per cui la revoca esiste** — il campo smette di essere scelto e diventa **calcolato**, quindi verificabile. È la forma che [DEBT-033] chiede: *togliere la discrezione invece di limitarla*.

Un tetto sul percorso del set **non è famiglia 3**, e la distinzione va dichiarata perché è proprio la famiglia che [DEBT-033] avverte di non commettere. Famiglia 3 è vincolare la grandezza *nominata* invece di quella da cui la proprietà dipende. Qui la proprietà è *«la chiave compromessa smette di votare abbastanza presto»*, e quella proprietà dipende **esattamente** da `effective_height`. È la grandezza giusta.

### 3. `challenge_evidence` porta l'altezza a cui l'auditor ha giudicato

`ChallengeEvidenceBody` guadagna un campo che registra l'altezza finalizzata contro cui l'auditor ha valutato la raggiungibilità del soggetto. Un verdetto smette di essere **asserito** e diventa **ricalcolabile**, che è la stessa forma con cui [SPEC-019] ha scelto fra le due letture di *unrevoked*.

**Cosa non fa, e va detto:** non toglie la divergenza. Due auditor a teste diverse continuano a raggiungere conclusioni opposte, e devono, perché la regola locale di `identity.md` è lì per proteggere il ricevente in tempo reale. Quello che cambia è che la divergenza diventa **visibile nell'oggetto**, quindi i validatori possono rifiutare un'evidenza il cui esito non è sostenuto dall'altezza dichiarata.

### 4. Quale versione del parametro governa

Ogni vincolo di cui sopra si valuta contro i **parametri di consenso in vigore all'altezza del blocco che include la `revoke_identity`**.

Questa clausola esiste per non aprire una terza porta sulla famiglia di [DEBT-012], [DEBT-020] e [DEBT-028] — una regola che dipende da un parametro governato senza dire quale versione valga. Sarebbe stata la quarta.

## Alternatives considered

- **Solo `max_revocation_effective_delay_blocks`, un tetto secco.** È la prima cosa che verrà proposta, e [DEBT-033] la dichiara *inefficace* perché un quorum ostile sceglierebbe il massimo ammesso. **Quella liquidazione è troppo netta e va corretta**: un tetto converte un danno **illimitato** in un danno **limitato**, che non è nulla. Rifiutata non perché inutile, ma perché **insufficiente da sola** — lascia intatta la discrezione sul caso che conta, `key_compromise`, e non distingue fra ragioni che meritano latitudine diversa. La proposta sopra la contiene come caso particolare e la rende dipendente da `reason`.
- **`min(effective_height, proponente + pavimento)` sul percorso di spesa.** Non è un fork, per la ragione scritta nel Context. Rifiutata perché conserva un pavimento sul percorso dove la sua giustificazione non esiste: sarebbe scegliere di proteggere un saldo *meno* di quanto si potrebbe, in cambio di nessuna proprietà. La parte 1 è questa opzione con il pavimento a zero, che è la stessa meccanica senza il residuo arbitrario.
- **Allineare la regola locale di `identity.md` alla definizione di [SPEC-019].** Rifiutata, e [DEBT-034] lo dice già: renderebbe la raggiungibilità dipendente dagli antenati di un blocco che il ricevente potrebbe non avere ancora, cioè toglierebbe al ricevente la capacità di proteggersi in tempo reale. Sposterebbe il danno sul livello di trasporto.
- **Non fare nulla e dichiarare la finestra.** È lo stato attuale, ed è onesto: `ledger.md` dichiara già l'esposizione. Rifiutata perché la dichiarazione descrive un danno di cui **l'ampiezza la sceglie l'avversario**, e una dichiarazione del genere non è un limite.

## Consequences

- **`effective_height` cambia significato e resta un campo solo.** Governa la transizione del set; non governa più la spesa. È contenuto normativo nuovo su un campo pubblicato, quindi la gate di [ADR-012] si applica alla spec che lo scrive — non a questa decisione.
- **`ChallengeEvidenceBody` cambia forma**, quindi cambiano l'ID della transazione, la fixture canonica e i digest pubblicati che la trascrivono. È il pezzo più caro della proposta, ed è caro perché il campo non c'era.
- **La frase di `identity.md` sulla non retroattività va riscritta.** Dice che le firme storiche restano valide *«sotto l'altezza efficace»*; diventano due frasi, una per percorso. La non retroattività non viene tolta: viene detta rispetto all'altezza che governa ciascun percorso.
- **Nasce `max_planned_revocation_delay_blocks`**, un parametro governato nuovo, e con esso l'obbligo di fissarlo. **Si porta dietro un debito che questa decisione non chiude**: `min_revocation_effective_delay_blocks` non ha oggi alcun valore di lancio, e non è nemmeno nella lista DRAFT. Fissare un tetto come multiplo di un pavimento non fissato lascia il prodotto indeterminato. **Il Lead lo segnala come questione da portare all'operatore insieme a `max_clock_drift_ms` e a `D_max`/`S_max`, non come parte di questo ADR.**
- **Va deciso nella spec, non qui**, se una revoca inclusa nel blocco `h` morda per le transazioni **dello stesso blocco** `h` o dal successivo. Mordere a `h` è coerente con la forma *«una revoca morde *alla* propria altezza»* già scritta; mordere a `h+1` evita del tutto una regola di ordinamento intra-blocco. **La raccomandazione del Lead è `h`**, per non introdurre una seconda forma, ma la scelta ha conseguenze sull'ordinamento che l'implementatore deve guardare con il codice sotto mano.
- **[DEBT-034] non viene chiuso interamente da questa decisione**, e dirlo è parte della decisione. La parte 3 rende il verdetto ricalcolabile; **non** impedisce a un soggetto irraggiungibile-ma-iscritto di accumulare `no_response` per tutta la durata della finestra. Quella metà si accorcia per composizione con la parte 2, non per un rimedio proprio.

## Review conditions

Rivedere **la tabella di `reason`** se la devnet mostrasse che `validator_misconduct` viene usato per casi con l'urgenza di un `key_compromise`: sarebbe il segno che le tre ragioni sono due, e che il campo va ristretto invece che tarato.

Rivedere **la parte 3** quando esisterà una seconda implementazione del percorso di sfida: è lì che si vedrà se l'altezza dichiarata basta a rendere un verdetto contestabile, o se serve che l'evidenza porti anche cosa l'auditor ha osservato.

**Non rivedere** la parte 1 per riavvicinare i due percorsi. La loro asimmetria non è un difetto da sanare: è la conseguenza del fatto che il pavimento ha una ragione sola, e riallinearli significherebbe rimettere sul saldo un ritardo che nessuna proprietà chiede.
