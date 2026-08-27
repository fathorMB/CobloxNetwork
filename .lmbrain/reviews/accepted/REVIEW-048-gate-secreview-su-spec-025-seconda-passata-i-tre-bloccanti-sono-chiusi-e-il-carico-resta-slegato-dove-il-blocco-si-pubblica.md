---
id: REVIEW-048
# Note: Quote the title if it contains a colon
title: "GATE-SECREVIEW su SPEC-025, seconda passata: i tre bloccanti sono chiusi, e il carico resta slegato dove il blocco si pubblica"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-025
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, robustness, test-quality, verification-integrity, documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-048-EVENT-001"
    timestamp: "2026-08-27T18:33:50.236276500+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-048-EVENT-002"
    timestamp: "2026-08-27T18:42:47.510912400+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata su richiesta dell'operatore del 2026-08-27. Tutti e tre i bloccanti di REVIEW-047 sono chiusi, e la reviewer lo ha stabilito mutando l'albero invece di leggere l'evidenza: allargare di un solo campo la rimozione in `transactions_root_of` fa fallire tre test, quindi la rimozione e' esattamente quella che `ledger.md` definisce per `tx_id`. Il Lead ha verificato che il legame esista al confine, `messages.rs:376`.\n\nLa reviewer ha rieseguito la mutazione del Lead sull'intero crate con `--no-fail-fast` invece che sulle due sole suite, e conferma che falliscono i due test dichiarati e nient'altro in diciassette target: GATE-LEAD-REPRO regge.\n\nSette rilievi nuovi, nessuno bloccante, e i due che contano sono promossi a debito con un bersaglio invece di aprire un terzo giro.\n\nDEBT-047, `high`: `FinalizedBlock::verify` continua ad accettare un `Block` il cui carico l'header non impegna, perche' il rimedio e' al confine della proposta e non alla verifica. Verificato dal Lead — zero occorrenze di `transactions_root` in quell'impl. Oggi il danno e' nullo perche' nessun percorso porta un `Block` da fuori, ma SPEC-029 introduce **entrambi** i percorsi che lo portano, disco e rete, quindi il buco diventa raggiungibile esattamente li'. Instradato su SPEC-029, che porta ora un criterio di accettazione proprio: il debito non poggia sulla memoria di chi scrivera' quella spec.\n\nDEBT-048, `medium`: rendere la radice insensibile all'ordine lascia i 230 test verdi, e l'ordine e' la proprieta' per cui questo albero e' diverso dagli altri cinque. Non e' un difetto del codice ma della sua prova. Instradato su SPEC-029, che aggiungera' un secondo sito che ricalcola la radice — con due siti invece di uno la lacuna varrebbe il doppio.\n\nGli altri cinque restano note nella review: il tetto del Merkle addebitato dopo il lavoro, l'attribuzione della riga di GATE-SAFETY-UNDER-ADVERSARY che asserisce su `block_id` e non sul `Block`, e tre imprecisioni.\n\nUn limite dichiarato dall'implementatrice e' stato verificato e ritirato: `Engine::on_value` non ricalcola la radice, e la reviewer ha stabilito per costruzione che non esiste ordine di eventi in cui un valore entri senza passare dal confine, perche' `VerifiedMessage` ha campo privato e nel crate esistono tre sole costruzioni, tutte al confine.\n\nAccettata e non rimandata perche' la consegna e' al secondo giro, chiude il primo esito di M-02, e l'operatore ha gia' rilevato una volta che il progetto si stava arenando. Un terzo giro su rilievi che non bloccano alcun criterio costerebbe piu' di quanto vale, e cio' che resta e' tracciato su due debiti con un bersaglio e un criterio, non lasciato alla memoria.</reason>\n<evidence_refs>[\"SPEC-025\", \"REVIEW-047\", \"SPEC-029\", \"DEBT-047\", \"DEBT-048\"]</evidence_refs>\n</invoke>"
    implementation_agent: "AGENT-002"
links: []
created: 2026-08-27
updated: 2026-08-27
tags: [review, security, consensus]
related_decisions: [ADR-018, ADR-012, ADR-001]
activity:
  - date: 2026-08-27
    action: "created"
  - date: 2026-08-27
    action: "transitioned pending -> accepted"
---
# Review

> **Seconda passata di `GATE-SECREVIEW` su [SPEC-025]**, sull'albero al commit `437f4c4`
> (remediation `31669eb`, attestazione `GATE-LEAD-REPRO` `437f4c4`). La prima passata e'
> [REVIEW-047], dieci rilievi, tre bloccanti.
>
> La gate e' stata rieseguita e non ereditata: accettare sulla passata precedente
> significherebbe attestare su un albero che non esiste piu'. Ogni affermazione di questa
> review dichiara se e' stata **ESEGUITA** o **LETTA**.
>
> **I sette rilievi non bloccanti di [REVIEW-047] non sono ri-sollevati come ignorati.**
> Erano fuori dal giro per istruzione del Lead e la 5.1.0 vieta di aprire un giro per
> rilievi minori. Dove li ho ritrovati cambiati lo dico; dove li ho ritrovati dove li
> avevo lasciati non li riconto.

## Outcome

**La remediation regge, e i tre bloccanti sono chiusi.** Ognuno dei tre e' stato chiuso
per la strada che [REVIEW-047] aveva prescritto, e l'ho stabilito **mutando l'albero e
osservando**, non leggendo l'evidenza. Il difetto che questa passata cercava — il `high`
nuovo nato dalla correzione, che su [SPEC-023] si e' prodotto due volte — **non c'e' nel
codice di consenso**: il controllo aggiunto e' quello giusto, e' nel posto giusto,
rifiuta nelle due direzioni e non rifiuta nelle due direzioni in cui rifiutare
romperebbe il protocollo.

Restano sette rilievi nuovi, **nessuno bloccante**. Il piu' grave, RF-001, e' la meta'
della classe di [REVIEW-047] RF-001 che il rimedio non copre: `verify_proposal` lega il
carico al confine della **proposta**, ma `FinalizedBlock::verify` — l'unico verificatore
di `Block` che questo crate spedisce — continua ad accettare un `Block` il cui
`transactions` non riproduce `header.transactions_root`. **Misurato, non dedotto.** Non
blocca perche' oggi nessun percorso consegna a quel verificatore un `Block` di
provenienza non fidata, e perche' le condizioni di chiusura che ho scritto io in
[REVIEW-047] sono soddisfatte alla lettera; ma e' la stessa forma di argomento —
*«nessun chiamante oggi»* — che [REVIEW-047] RF-007 ha rifiutato altrove, e va messa a
debito adesso e non scoperta quando la sincronizzazione del ledger verra' scritta.

**Raccomando `accept`.**

## Che cosa ho ESEGUITO

Tutte le esecuzioni su `E:\Git\CobloxNetwork` a `437f4c4`. L'albero e' stato mutato
quattro volte e ripristinato ogni volta da una copia presa **prima** della mutazione,
tenuta fuori dal repository (`%TEMP%\claude\messages.rs.bak`), non con `git checkout`.
**A fine passata `git status --porcelain` e' vuoto.**

| # | esecuzione | esito |
| --- | --- | --- |
| G1 | `FinalizedBlock::verify` su un `Block` col carico sostituito, certificato genuino | **accettato**, tre carichi divergenti su tre |
| G2 | `verify_proposal` con 16.384 / 200.000 / 1.000.000 transazioni, cronometrato (build `--release`) | 86 ms / 956 ms / **5,07 s** |
| MA | mutazione: guardia sulla radice disattivata (`if false && ...`), `cargo test -p coblox-core --no-fail-fast` | **esattamente** `one_header_with_two_payloads_does_not_produce_two_blocks` e `a_proposal_whose_payload_does_not_reproduce_its_root_is_refused`, nient'altro |
| MB | mutazione: controllo `header.round` disattivato | **esattamente** `a_first_hand_proposal_must_carry_its_own_round_in_the_header` |
| MC | mutazione: regola di rimozione allargata di un campo (`created_at_ms` oltre ad `authorization`) | tre test rossi, fra cui la forma onesta |
| MD | mutazione: `transactions_root_of` reso **insensibile all'ordine** (`ids.sort_unstable_by_key`) | **suite interamente verde** |

Sono **LETTI** e non rieseguiti: i 230 test come conteggio complessivo, `clippy`, `fmt`,
i nove strumenti a exit 0, il conteggio delle probe da 172 a 180, e le due parti di
`GATE-LEAD-REPRO` che non sono la mutazione (proponente muto, esecuzione avversa). Ho
rieseguito la suite di consenso per intero cinque volte nel corso delle mutazioni, e
sull'albero non mutato non ho mai osservato un rosso.

## Stato dei tre bloccanti di [REVIEW-047]

### [REVIEW-047] RF-001 — **CHIUSO**

Il legame carico-blocco esiste, e' il controllo 5 di `verify_proposal`
(`messages.rs:376-386`), e' eseguito **prima** di `valid_round`/`block_id` e quindi
prima di qualunque regola che possa prevotare, e produce
`ConsensusError::ProposalTransactionsRootMismatch` con entrambe le radici.

**La rimozione e' quella giusta.** `ledger.md#unsigned-transaction-and-authorization`
recita, verbatim e con gli spazi normalizzati: *«The unsigned transaction used for its
ID is the object with `authorization` removed.»* `transactions_root_of`
(`messages.rs:281-295`) ricostruisce l'oggetto saltando la sola chiave di primo livello
`authorization`, poi chiama `registry::tx_id`, che e' `H("coblox-tx-id-v0\0" ||
chain_id_32 || JCS(unsigned))`, e poi `merkle::transactions_root`, che e' l'albero in
**ordine di blocco** con il tetto di 16.384 di `ledger.md`. Non differisce di un campo:
e' quel campo e nessun altro. `JsonObject` e' un `BTreeMap` e `insert` rifiuta le chiavi
duplicate, quindi la ricostruzione non puo' ne' riordinare ne' collassare nulla che JCS
non riordinasse comunque.

Le due direzioni pericolose sono entrambe pinnate, ed e' la parte che vale piu' del
rifiuto: `the_boundary_computes_the_root_over_the_unsigned_transaction` prova che una
`authorization` **diversa** non cambia il verdetto — cioe' che il confine non hasha
l'oggetto firmato e quindi non rifiuta ogni proposta onesta, che e' il modo peggiore in
cui una regola di rifiuto puo' sbagliare — e `a_re_proposal_keeps_the_round...` fa lo
stesso per RF-002.

**Come l'ho stabilito**: mutazione MA (la guardia disattivata riproduce il difetto e
mordono i due test attesi), mutazione MC (una regola di rimozione sbagliata di **un solo
campo** e' catturata da tre test), lettura verbatim di `ledger.md`, `registry.rs:290`,
`merkle.rs:177`.

**Cio' che resta aperto della classe** e' RF-001 di questa review: il confine della
proposta e' legato, il verificatore del `Block` pubblicato no.

### [REVIEW-047] RF-002 — **CHIUSO**, e il motivo scritto e' vero

Nel ramo `valid_round: None` il confine esige `header.round == proposal.round`
(`messages.rs:369-371`). Nel ramo `Some(vr)` non c'e' confronto, e le tre scritture del
motivo — doc-comment punto 4, commento in linea, `wire.md#block_proposal` — dicono la
stessa cosa. **Ho verificato che il motivo sia vero e non solo coerente**, che era la
domanda dell'incarico:

1. *«`block_id` copre ogni byte dell'header»*: **vero**. `BlockProposal::block_id`
   delega a `BlockHeader::block_id`, che e' `H(dominio || chain_id || JCS(header))` su
   tutti i campi, `round` incluso. Un `header.round` riscritto e' un `block_id` diverso,
   quindi non e' un campo che l'attaccante possa muovere lasciando fermo il valore su
   cui il quorum si chiude.
2. *«la POL a `vr` e' verificata contro il log proprio in ogni percorso che porta ad
   accettare una ri-proposta»*: **vero sotto l'ipotesi di guasto, e il testo lo
   semplifica di un passo.** Il percorso del prevoto e' letterale:
   `try_prevote_on_proposal`, ramo `Some(valid_round)`, esce con `Ok(false)` se
   `prevote_quorum_for(valid_round, block_id)` e' falso — quorum nel **proprio** log,
   non nel numero che la proposta porta. Gli altri due percorsi che agiscono su una
   proposta — `try_lock_and_precommit` e `try_decide` — **non** guardano `valid_round`:
   il primo esige un quorum di prevoti al round **corrente**, il secondo un quorum di
   precommit. Nessuno dei due e' raggiungibile senza che oltre due terzi del potere
   abbia gia' prevotato, e sotto meno di un terzo di potere guasto quel quorum contiene
   almeno un onesto, che la POL l'ha verificata. La proprieta' regge; la frase
   pubblicata la enuncia come se il controllo fosse nel percorso invece che
   nell'intersezione dei quorum. **Non la sollevo come rilievo**: e' una semplificazione
   vera, non una falsa, ed e' quella che un ricevente puo' applicare.

**Come l'ho stabilito**: mutazione MB (esattamente un test morde), lettura di
`engine.rs:620-705` e `block.rs`/`registry.rs` per la copertura dell'header.

Il test `a_re_proposal_keeps_the_round_the_value_was_first_proposed_at` gira ai round
1..4 ed e' la meta' che conta: senza di esso un'implementatrice successiva
«correggerebbe» il controllo verso il largo e stallerebbe ogni altezza a due round.
Questa era una condizione di chiusura esplicita di [REVIEW-047] ed e' soddisfatta.

### [REVIEW-047] RF-003 — **CHIUSO**, e la restrizione e' vera come scritta

Il testo pubblicato di `wire.md:428-444` ora dice: *«**At uniform voting power** — one
unit each, which is the shape an elected set is required to have — two consecutive
rounds cannot name the same member while an unvisited one remains»*, seguito dal caso
pesato in grassetto e dalla conseguenza sulla vivacita'.

**La restrizione e' vera come scritta.** A potere uniforme `total_power = n` e l'indice
e' `(h + r) mod n`: round consecutivi danno indici consecutivi, distinti per ogni
`n >= 2`, e per `n = 1` non esiste un membro non visitato, quindi l'enunciato e' vero
anche li'. **La clausola d'ancoraggio e' vera**: `ValidatorSet::check_elected_shape`
(`validator_set.rs:383-389`) rifiuta ogni entry con `voting_power != 1`, e
`ledger.md:1278-1282` pubblica la stessa regola per un set eletto. **Il caso pesato e'
vero e misurato**: `[1,1,1,7]` da' sette round consecutivi a `val-003`, e il test lo
misura invece di asserirlo, e asserisce anche che il banco pesato **non** si comporti
come uno uniforme — che e' la clausola che impedisce al test di restare verde se
qualcuno rendesse uniforme il banco.

**Il residuo che l'implementatrice ha riportato non cambia la gravita' di cio' che
resta pubblicato.** Nessun percorso di consenso chiama `check_elected_shape`, quindi il
set di genesi della devnet di M-02 non e' vincolato a potere uniforme. Ma: (a) la
conseguenza e' **solo** sulla vivacita', e la ragione e' nel codice e non nel testo — la
regola del proponente autorizza a **proporre**, e ogni regola che finalizza conta voti
firmati; (b) il residuo e' scritto in `proposer.rs` nella forma piu' forte possibile
(*«No path in this module calls `check_elected_shape` ... A deployment whose set is not
uniform therefore gets the weighted behaviour above and not the property in bold»*), il
che e' piu' di quanto la strada (a) chiedesse; (c) `check_structure`, che il percorso
chiama davvero, rifiuta il potere zero via `quorum::total_voting_power`, quindi non
esiste il caso peggiore — un membro che la scala non raggiunge **mai** — e il
doc-comment di `proposer_at` che lo afferma dice il vero. **Sotto [ADR-012] non resta
una frase falsa in un documento pubblicato**, che era il capo d'accusa.

**Come l'ho stabilito**: lettura verbatim di `wire.md`, `proposer.rs`,
`validator_set.rs:332-398`, `quorum.rs:45-56`, `ledger.md:1278`; verifica aritmetica
dei casi limite `n = 1` e potere zero.

### [REVIEW-047] RF-005 — chiuso anche lui, e non era bloccante

Le tre frasi di `mod.rs` §*Divergence 1* **ora dicono il vero**, tutte e tre, e l'ho
verificato contro il codice e non contro il testo.

- (a) L'argomento falso e' sparito e il testo **dichiara** che era falso
  (*«the reason is **not** that no rule reads a timer»*). L'argomento sostitutivo — la
  **direzione** — e' vero: `on_timeout` puo' solo portare `self.step` da `Propose` a
  `Prevote`, da `Prevote` a `Precommit`, o iniziare il round successivo
  (`engine.rs:533-560`); `try_prevote_on_proposal` esige `step == Propose` e
  `try_lock_and_precommit` blocca e precommette solo con `step == Prevote`. Nessun ramo
  di `on_timeout` emette un voto. Un timer puo' quindi solo **sopprimere** un blocco o
  un precommit. La chiusura *«un blocco gia' preso sopravvive al cambio di round, perche'
  `locked` si azzera solo su una decisione»* e' vera: l'unica scrittura `self.locked =
  None` e' in `try_decide`.
- (b) La riga 55 e' nell'elenco, la conseguenza sul recupero e' scritta, ed e' composta
  con RF-003 nel modo giusto (quadratica in `w`). Il conteggio delle regole e' passato
  da «two rules (34, 44, 47)» — che ne elencava tre — a «four rules (34, 44, 47, 55)»,
  che ne elenca quattro. Corretto.
- (c) L'armamento anticipato di `OnTimeoutPrevote` e' dichiarato come costo di vivacita'
  e non spacciato per soprainsieme di comportamento, con la conseguenza esatta (dal
  passo `Precommit` il nodo non puo' piu' bloccarsi ne' precommittare quel round).

Resta un refuso nella frase **adiacente** a quella corretta: vedi RF-006 di questa
review.

## I due limiti che l'implementatrice ha dichiarato

### `Engine::on_value` non ricalcola la radice — **l'argomento regge, e l'ho verificato per costruzione**

La domanda dell'incarico era se esista un ordine di eventi in cui il proprio valore
entra senza passare dal confine. **Non esiste, e la ragione e' piu' forte di quella che
l'implementatrice ha dato.** Non e' che il trasporto lo riconsegna: e' che **non c'e'
altro modo**.

`Engine::step_event` accetta `Event::Message(VerifiedMessage)`. `VerifiedMessage` e' una
tupla a campo **privato** (`messages.rs:253`) e nel crate intero esistono **tre** sole
costruzioni — `messages.rs:394` in `verify_proposal` e `messages.rs:442` in
`verify_vote`. Non c'e' costruttore pubblico, non c'e' `From`, `into_inner` va nella
direzione opposta. Quindi nessun messaggio, incluso il proprio, entra nel log senza
passare dal confine. `on_value` non registra nulla: emette
`Action::Broadcast(Outbound::Proposal(..))` e basta (`engine.rs:602`). La ri-proposta di
`start_round` deriva da `self.valid`, che `try_lock_and_precommit` prende da una
proposta gia' passata dal confine. **Il costo del limite e' interamente sulla
vivacita'**: un chiamante il cui esecutore producesse un valore incoerente vedrebbe la
propria proposta rifiutata da tutti, il round fallirebbe, e il rifiuto sarebbe
attribuibile a lui. Non apro rilievo.

**Come l'ho stabilito**: `grep` esaustivo di `VerifiedMessage(` su `core/`, lettura di
`engine.rs:376-398, 400-428, 492-530, 584-614`, e la conferma nel banco
(`devnet.rs:494-510`: ogni consegna, compresa quella a se' stessi, passa da
`verify_proposal` o `verify_vote`).

### Il costo per messaggio — **misurato, e il tetto del Merkle e' il problema e non la difesa**

E' RF-002 di questa review. In breve: la passata SHA-256 costa **86 ms** su un blocco
pieno legittimo (16.384 transazioni, build `--release`), e il tetto di `ledger.md` e'
addebitato **dopo** aver hashato tutto, non prima: un milione di transazioni costa
**5,07 s** prima che `TooManyLeaves` esca.

## Verifiche del Lead: contestate dove serve, confermate dove reggono

**`GATE-LEAD-REPRO`, la mutazione: confermata, e ho provato a romperla.** Il Lead
attesta di aver disattivato la guardia sulla radice e di aver osservato fallire
esattamente due test. **Rieseguito (MA) sull'intero crate con `--no-fail-fast`, non
sulle due sole suite di consenso**: falliscono `one_header_with_two_payloads_does_not_
produce_two_blocks` e `a_proposal_whose_payload_does_not_reproduce_its_root_is_refused`,
e **nient'altro in nessuno dei diciassette target**. L'attestazione dice il vero. Non ho
trovato nulla che quella mutazione lasciasse passare e che avrebbe dovuto rompersi:
**e' l'insieme giusto**, perche' i due test sono esattamente i due lati della regola —
il confine e il banco a quattro nodi — e nessun altro test dipende da quella guardia.

**Ho pero' trovato una mutazione che la suite non vede**, ed e' RF-003 di questa review:
rendere la radice **insensibile all'ordine** lascia i 230 test verdi. Non e' un difetto
contro il Lead — la sua mutazione mordeva dove doveva — ma dice che la gate di
copertura si ferma un passo prima della proprieta' che `ledger.md` dichiara distintiva
di questo albero fra i sei.

**Una riga della tabella delle gate afferma piu' di cio' che e' stato eseguito**, ed e'
RF-004 di questa review.

Le altre verifiche del Lead sono **LETTE** e non contestate: i quattro test chiave
eseguiti singolarmente, `clippy`, `fmt`, i nove strumenti, le 180 probe. Le tre probe
nuove le ho lette una per una: pinnano le tre frasi giuste, con il `why` che dice
perche' quella frase e non un'altra, e la prova in negativo le esercita individualmente.

## Acceptance-criteria compliance

| criterio | verdetto dopo la remediation | nota |
| --- | --- | --- |
| Dominio `coblox-block-prevote-v0` | soddisfatto | invariato dalla prima passata |
| I tre messaggi in `wire.md` | **soddisfatto**, e i due difetti di [REVIEW-047] sono chiusi | le due regole normative dentro `block_proposal` sono ora imposte, e la forma del ricevente e' pubblicata |
| Regola del proponente deterministica | soddisfatto | invariato |
| Motore senza I/O | soddisfatto | il controllo nuovo e' in `messages.rs`, che e' il confine e non il motore; il lint resta `PASS` e il suo limite resta quello di [REVIEW-047] RF-009 |
| Catena di dieci blocchi | soddisfatto | invariato |
| Sicurezza sotto partizione | **soddisfatto sul `block_id`**; sul `Block` la proprieta' e' vera ma la prova non e' nello sweep | RF-004 |
| Vivacita' dopo un proponente muto | **soddisfatto**, e ora la proprieta' pubblicata e' quella vera | RF-003 di [REVIEW-047] chiuso |
| Equivocazione rifiutata | soddisfatto per il precommit; l'equivocazione di **proposta** e' ora esercitata per la prima volta | resta [REVIEW-047] RF-006, non bloccante |
| Determinismo byte per byte | **soddisfatto, e non piu' solo sul perimetro dell'harness** | e' cio' che RF-001 di [REVIEW-047] toglieva: ora il carico e' determinato dal valore |
| Regola di blocco confrontata con la fonte | soddisfatto, e le **ragioni** sono ora vere | [REVIEW-047] RF-005 chiuso |
| Nessuna modifica a cio' che era pubblicato | soddisfatto | verificato sul diff: `ledger.md`, `README.md`, `identity.md`, `block.rs`, `quorum.rs`, `registry.rs`, `hash.rs`, `merkle.rs` non toccati |
| Passata di [ADR-012] | soddisfatto sul meccanismo **e ora anche sul contenuto** | la frase falsa che nessuna probe copriva e' corretta e pinnata |
| `test`/`clippy`/`fmt` | soddisfatto (LETTO) | rieseguita da me la sola suite di consenso, sempre verde sull'albero non mutato |

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security-boundary | severity=high | criterion=**Il verificatore di `Block` spedito non lega il carico, e la remediation ha reso quel fatto piu' difficile da vedere.** `FinalizedBlock::verify` (`certificate.rs:305-330`) fa tre cose — che `quorum_certificate.block_id` sia l'ID dell'header portato, che le altezze coincidano, e `QuorumCertificate::verify` — e **non ricalcola mai `transactions_root` da `transactions`**. E' la meta' della classe di [REVIEW-047] RF-001 che il rimedio non copre. **Scenario d'attacco**: non serve nemmeno un validatore. Chiunque possieda un `Block` finalizzato genuino — e' un artefatto pubblico — ne sostituisce l'array `transactions` e lo serve a un nodo che sincronizza; `wire.md:594` dice che l'annuncio e' un suggerimento e che *«receivers ... fetch the full block via ledger sync»*, e ne' `wire.md` ne' `ledger.md` pongono un MUST sul ricevente del blocco intero, mentre `wire.md:483-487` lo pone ora sul ricevente della **proposta**. **Esito misurato (G1)**: header e certificato genuini a tre firme, `FinalizedBlock::verify` con `ConsensusVerifier` restituisce `Ok(())` su tre carichi divergenti su tre — array vuoto, carico dell'attaccante, carico onesto con una transazione dell'attaccante in coda. **L'asimmetria e' nuova e la ha creata questa passata**: il doc-comment del campo `BlockProposal::transactions` porta ora un paragrafo che spiega che l'identita' del carico **e'** legata, mentre il campo gemello `FinalizedBlock::transactions` resta *«The transactions, in canonical execution order»* e il doc di `verify` enumera *«Two checks the certificate alone cannot make»* come se l'elenco fosse completo. Un lettore che confronti i due conclude che il `Block` erediti il legame. **La suite stessa sa che il controllo serve**: `assert_every_published_payload_reproduces_its_root` (`consensus_devnet.rs:442-461`) applica a ogni blocco pubblicato esattamente il confronto che il verificatore spedito non fa. **Non bloccante**, e la ragione va detta per intero: le tre condizioni di chiusura che ho scritto io in [REVIEW-047] sono soddisfatte alla lettera, nessun percorso di questa consegna consegna a quel verificatore un `Block` di provenienza non fidata, e la sincronizzazione del ledger non e' di questa spec. Ma e' la stessa forma di argomento — *«non esiste ancora un chiamante»* — che [REVIEW-047] RF-007 ha rifiutato su `verifier.rs:94`, e questa volta la finestra si chiude quando qualcuno scrive `/coblox/ledger-sync/0.1.0`. | remediation=In due mosse, e la prima e' di questa passata. **(1) Subito, ed e' documentazione**: dire in `certificate.rs`, dove un lettore incontra `FinalizedBlock::verify` e il suo campo `transactions`, che il carico **non** e' verificato qui e di chi e' il compito — cosi' che l'asimmetria col gemello sia dichiarata invece che dedotta. **(2) Del Lead**: o il controllo in `FinalizedBlock::verify` (sono le stesse sei righe di `transactions_root_of`, gia' scritte, e nessun esecutore serve), o un debito con `target_specs` sulla spec di ledger sync, con la premessa scritta — *il chiamante non esiste oggi* — cosi' che il chiudersi della finestra sia un evento e non una scoperta. **Condizione di chiusura**: (a) un test in cui `FinalizedBlock::verify` rifiuta un blocco il cui `transactions` non riproduce `header.transactions_root`, **oppure** (b) la frase in `certificate.rs` piu' un debito aperto che nomini la spec dove il controllo diventa obbligatorio, piu' il MUST del ricevente di ledger sync in `wire.md` accanto a quello del ricevente di proposta.

RF-002 | category=robustness | severity=medium | criterion=**Il tetto del Merkle e' addebitato dopo il lavoro, quindi non e' la difesa che l'evidenza dichiara.** Il punto 3 di *«Cio' che resta non verificato»* della spec dichiara onestamente che il costo non e' profilato, e poi lo mitiga cosi': *«il tetto del Merkle e' imposto dalla stessa chiamata, quindi una proposta oltre il limite e' ora respinta al confine invece che accettata»*. La frase e' vera e la rassicurazione che porta e' falsa: `transactions_root_of` (`messages.rs:281-295`) calcola `tx_id` — JCS piu' SHA-256 — per **ogni** elemento dell'array, e solo dopo passa gli ID a `merkle::transactions_root`, che a `merkle.rs:178` rifiuta se sono piu' di 16.384. Il tetto limita l'**albero**, non il **lavoro**. Nulla a monte limita la lunghezza dell'array: `BlockProposal::from_json` (`messages.rs:187-207`) non ha cap. **Scenario**: un membro guasto, dentro il budget, invia una proposta al round di cui e' proponente — e i round non hanno finestra ([REVIEW-047] RF-004, non chiuso), quindi ogni membro e' proponente di infiniti round — con un milione di elementi. I controlli 1-4 lo lasciano passare perche' e' davvero il proponente. **Esito misurato (G2, `--release`)**: 16.384 elementi → **86 ms**; 200.000 → 956 ms; 1.000.000 → **5,07 s** prima di `Merkle(TooManyLeaves)`. Il riferimento per l'amplificazione: lo stesso array respinto al controllo 1 (mittente non membro) costa 1,57 s, che e' la sola distruzione delle strutture — quindi il lavoro **aggiunto** da questa passata e' circa 3,5 s su 5, un fattore ~3 sopra un costo di parsing che c'era gia'. **Non bloccante** per due ragioni che vanno dette entrambe: il fattore e' costante e non un'amplificazione asintotica, e la sicurezza non e' toccata. Ma gli 86 ms sul blocco pieno **legittimo** sono un costo per messaggio reale su un percorso di consenso, ed e' il numero che l'evidenza dichiara di non avere. | remediation=Spostare il tetto **prima** della passata: rifiutare `proposal.transactions.len() > TaggedTree::MAX_TRANSACTIONS` all'inizio di `transactions_root_of` (o in `BlockProposal::from_json`), con una variante d'errore di consenso invece di un `MerkleError` che attraversa il confine. Correggere il punto 3 dell'evidenza, che oggi offre il tetto come mitigazione di un costo che il tetto non limita, e registrarvi i tre numeri misurati sopra. **Condizione di chiusura**: una proposta che dichiara piu' di 16.384 transazioni respinta in tempo indipendente dalla sua lunghezza, misurato; e il punto 3 riscritto.

RF-003 | category=test-quality | severity=medium | criterion=**Nessun test lega la radice all'ordine, che e' la proprieta' per cui questo albero e' diverso dagli altri cinque.** `merkle.rs:173-176` la dichiara: *«The transaction Merkle tree preserves block order — so unlike the other five trees, this one is not sorted»*, ed e' la stessa frase di `ledger.md:50`. Ma la copertura nuova non la esercita. **Esito misurato (MD)**: ho mutato `transactions_root_of` aggiungendo `ids.sort_unstable_by_key(|d| *d.as_bytes());` prima della chiamata all'albero — cioe' ho reso il confine **insensibile all'ordine** — e **l'intera suite resta verde**, `consensus_rules` 25/25 e `consensus_devnet` 10/10. **Perche' conta**: `state_root` e' il risultato dell'esecuzione delle transazioni **in quell'ordine**, quindi una permutazione del carico e' un blocco diverso. Un confine che ordinasse accetterebbe una permutazione del carico che l'header impegna, e il difetto che [REVIEW-047] RF-001 esiste per chiudere — un header, due carichi, due `Block` pubblicati — tornerebbe in forma permutata senza che nulla lo veda. La forma onesta dei test nuovi usa **una** transazione, dove l'ordine non esiste; il caso a due transazioni compare solo nella lista dei carichi divergenti, dove qualunque radice diversa fa passare l'asserzione. **Non bloccante**: il codice spedito e' corretto, e questa e' una lacuna della prova e non del prodotto. | remediation=Aggiungere al test del confine il caso che manca: due transazioni distinte, header costruito sull'ordine `[A, B]`, proposta che porta `[B, A]`, rifiutata con `ProposalTransactionsRootMismatch`. E' una decina di righe e usa gli helper che esistono gia'. **Condizione di chiusura**: la mutazione MD osservata fallire.

RF-004 | category=verification-integrity | severity=medium | criterion=**Una riga della tabella delle gate afferma una prova che l'esecuzione non produce.** La riga `GATE-SAFETY-UNDER-ADVERSARY` della spec recita: *«30 esecuzioni sempre-attive rieseguite; nessuna altezza con due `block_id`, **e ora nessuna altezza con due `Block`**»*. La seconda meta' non e' asserita dallo sweep. `adversarial_sweep` (`consensus_devnet.rs:125-156`) chiama `assert_no_conflicting_finality` e `assert_chains_agree`, e **entrambe asseriscono solo su `block_id`**: la prima conta gli ID distinti per altezza (`devnet.rs:759-768`), la seconda confronta `node.chain[i].quorum_certificate.block_id` (`devnet.rs:771-788`). Nessuna delle due guarda i byte del `Block`, e lo sweep non fa variare mai `transactions`, che restano vuote in ogni sua esecuzione. Il confronto sull'artefatto e sulla radice esiste in **un solo** test — `one_header_with_two_payloads_does_not_produce_two_blocks`, tramite `chain_bytes` e `assert_every_published_payload_reproduces_its_root` — che non fa parte dello sweep e non e' fra le sue 530 esecuzioni. **La proprieta' e' vera** — l'ho verificata sul meccanismo e con la mutazione MA — ma l'attribuzione e' sbagliata, e sotto `QUALITY.md` §*Verification standard* un'evidenza deve dire cosa e' stato davvero verificato. **Non bloccante**: il difetto e' in una frase della spec, che e' del Lead, e si corregge senza toccare l'implementazione. | remediation=Una delle due, e la seconda vale di piu'. **(a)** Riscrivere la riga: lo sweep prova l'assenza di due `block_id`; l'assenza di due `Block` e' provata dal test dedicato e dalla regola al confine, e il Lead l'ha mutata indipendentemente. **(b)** Estendere `assert_chains_agree` a confrontare i byte canonici del `Block` invece del solo `quorum_certificate.block_id` — e' la stessa `chain_bytes` che il test dedicato usa gia', costa poco, e renderebbe la riga vera come e' scritta per tutte e 530 le esecuzioni. **Condizione di chiusura**: la riga dice cio' che l'esecuzione produce, oppure l'esecuzione produce cio' che la riga dice.

RF-005 | category=test-quality | severity=low | criterion=**La «seconda implementazione» dell'harness e' una copia letterale della prima.** Il doc-comment di `harness_transactions_root` (`consensus_support/devnet.rs:325-329`) afferma: *«This is deliberately a **second** implementation of the rule the boundary applies ... so a test that uses it is not asking `verify_proposal` whether it agrees with itself»*. La prima meta' e' vera nel senso che conta di piu' — l'harness non passa da `verify_proposal`, quindi un confine che **omettesse** il controllo verrebbe visto, ed e' la mutazione MA — ma il corpo e' lo stesso ciclo carattere per carattere del confine: stessa condizione `key != "authorization"`, stesso `tx_id`, stesso `transactions_root`. Una regola di rimozione **sbagliata allo stesso modo in entrambi i posti** resterebbe invisibile, e l'unico ancoraggio a `ledger.md` che sopravviverebbe e' la lettura umana piu' `the_boundary_computes_the_root_over_the_unsigned_transaction`, che prova che `authorization` e' rimossa ma non che sia l'**unica** cosa rimossa. Il rischio residuo e' basso — la mutazione MC mostra che una divergenza fra i due posti e' catturata da tre test — ed e' la parola *«implementazione»* a promettere piu' del fatto. | remediation=O correggere la frase (e' un **secondo percorso**, non una seconda implementazione, e cio' che difende e' l'omissione del controllo e non la sua deriva), o ancorare la radice a un vettore pubblicato invece che a una seconda copia del ciclo: un `tx_id` di fixture in `docs/protocol` e una probe di [ADR-012] che lo pinni. **Condizione di chiusura**: la frase dice cio' che il codice fa, oppure esiste un vettore esterno al crate che il confine deve riprodurre.

RF-006 | category=documentation | severity=low | criterion=**Il conteggio nella frase adiacente a quella corretta e' rimasto sbagliato.** `mod.rs:60` recita: *«Algorithm 1 has four broadcasts of a **nil** vote (lines 26, 32, 45, 59, 63)»* — quattro, e ne elenca **cinque**, che e' anche il numero giusto (26 e 32 nel blocco dei prevoti, 45 nel blocco dei precommit, 59 in `OnTimeoutPropose`, 63 in `OnTimeoutPrevote`). La remediation ha corretto la clausola successiva della **stessa riga** — «two rules (34, 44, 47)» → «four rules (34, 44, 47, 55)», che ora conta bene — e ha lasciato il numero precedente com'era. E' la forma di difetto che questo progetto ha gia' censito due volte su [SPEC-023]: correggere la meta' guardata e lasciare la meta' accanto. Nessuna conseguenza operativa; e' un doc-comment su codice di consenso, che qui e' materiale di prima classe. | remediation=«five broadcasts». **Condizione di chiusura**: il numero e la lista concordano.

RF-007 | category=documentation | severity=low | criterion=**Un doc-comment del motore nomina la riga sbagliata dell'Algorithm 1, e la tabella dello stesso modulo lo contraddice.** `engine.rs:583` introduce `on_value` con *«Algorithm 1 line 65-67 reached from a `getValue()` reply»*. Le righe 65-67 sono `OnTimeoutPrecommit`, come la tabella di `mod.rs:56` dichiara correttamente (*«65-67 `OnTimeoutPrecommit` → `StartRound(round_p + 1)` | `Precommit` arm»*); `getValue()` e la proposta che ne discende sono le righe 14-19, che la stessa tabella mappa correttamente alla riga precedente. Il difetto e' anteriore alla remediation — viene da `76b5bd3`, la consegna — e nessuna delle due passate lo aveva guardato. E' dentro il perimetro di `GATE-LOCKING-FROM-SOURCE`, che esiste perche' *«una regola di blocco che diverge in silenzio dal paper che dichiara di implementare»* e' il difetto che quella gate deve catturare: qui non diverge la regola, diverge il riferimento, e un lettore che seguisse il rimando finirebbe sulla funzione di timeout. | remediation=`14-19`. **Condizione di chiusura**: il riferimento del doc-comment e quello della tabella dello stesso modulo concordano.

## Required follow-up

**Nessun rilievo bloccante.** Nessuno dei sette blocca un criterio di accettazione, una
gate dichiarata o `QUALITY.md` in un modo che giustifichi un terzo giro su codice di
consenso che ho appena mutato quattro volte senza trovarlo rotto.

Da portare dentro la review accettata, nell'ordine in cui costano poco:

1. **RF-001**, ed e' l'unico che va **instradato** e non solo annotato: la frase in
   `certificate.rs` adesso, e la decisione fra il controllo e il debito al Lead. Se il
   Lead sceglie il controllo, sono sei righe gia' scritte altrove e non richiedono un
   giro di review — sono nella stessa classe della guardia che questa passata ha appena
   validato con la mutazione MA.
2. **RF-004**, che e' del Lead perche' e' una riga della spec, e la strada (b) —
   estendere `assert_chains_agree` ai byte del `Block` — chiuderebbe anche meta' di
   RF-003.
3. **RF-002** e **RF-003**, additivi e piccoli.
4. **RF-005**, **RF-006**, **RF-007**: tre frasi.

**Ancora aperti da [REVIEW-047]** e non riaperti qui: RF-004 (finestra sui round —
composta con RF-002 di questa review, perche' e' l'assenza di finestra a rendere ogni
membro proponente di infiniti round), RF-006 (partizione che guarisce, doppio prevoto),
RF-008 (riconciliare [ADR-018] §2 con la riga 29 **nel testo**), RF-009, RF-010.
**RF-007 di [REVIEW-047] risulta lavorato** fuori da questa remediation, al commit
`2d92244`: `verifier.rs:93-95` non porta piu' la frase falsa e dichiara che questo crate
un verificatore lo spedisce. Non l'ho riverificato oltre la lettura.

## Final decision

**Raccomando `accept`.**

La ragione, in una riga: **i tre bloccanti sono chiusi per la strada che [REVIEW-047]
aveva prescritto, l'ho stabilito mutando l'albero e non leggendo l'evidenza, e la
remediation non ha introdotto un difetto nuovo nel codice di consenso** — che era la
domanda che questa passata doveva rispondere.

Le tre correzioni sono nel **corpo** e non nella tabella: il controllo e' nel confine e
non in un commento, il motivo del ramo non confrontato e' vero e non solo coerente, e la
proprieta' pubblicata e' ristretta al fatto invece che al fatto comodo. Le due direzioni
in cui una regola **troppo forte** avrebbe rotto il protocollo sono pinnate da test che
restano verdi sotto le mutazioni che rendono rossi gli altri, ed e' il controllo che
mancava alle remediation di [SPEC-023]. La mutazione del Lead morde l'insieme giusto e
non uno piu' piccolo: l'ho rieseguita sull'intero crate e non sulle due suite.

Cio' che resta e' un rilievo `high` che **non e' un difetto nuovo**: e' la meta' della
classe di [REVIEW-047] RF-001 che le condizioni di chiusura, come le ho scritte io, non
richiedevano. Il verificatore di `Block` spedito accetta ancora un carico che il suo
header non impegna, e questa passata ha reso quel fatto piu' difficile da vedere
scrivendo il legame accanto al campo gemello. Va nominato ad alta severita' e messo a
debito **ora**, mentre nessun chiamante lo raggiunge, invece di essere riscoperto quando
la sincronizzazione del ledger arrivera'. Ma farlo costare un terzo giro a una spec che
chiude il primo esito di M-02, quando la strada per chiuderlo e' o una frase o sei righe
gia' scritte, costerebbe piu' di quanto valga.

**La consegna regge, e questa e' la seconda passata di una gate che serviva. Il verdetto
formale e' del Lead.**
