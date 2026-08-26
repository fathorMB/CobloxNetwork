---
id: REVIEW-039
# Note: Quote the title if it contains a colon
title: "Review di SPEC-022: il morso all'inclusione e la banda di reason"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-022
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [correctness, security, documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-039-EVENT-001"
    timestamp: "2026-08-26T23:55:00.000000+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-039-EVENT-002"
    timestamp: "2026-08-26T23:29:07.536465800+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Due finding, una high e una medium. L'attuazione di ADR-017 e' corretta nella sostanza: parte 1 e parte 2 sono nei documenti e nel crate, i tre vincoli di genesi ci sono, 190 test contro i 181 di prima, tutte le gate escono 0. Otto criteri su quattordici rieseguiti dal Lead, sette reggono.\n\nRF-001 high, trovata mutando e non leggendo. ledger.md scrive che la riga 21 di AUTH-0 e' la frontiera della clausola 2, \"so clause 2 is <= and not <\". La riga 21 non distingue le due letture: con la revoca inclusa a 20, sia 20 <= 21 sia 20 < 21 sono veri. I due predicati differiscono esattamente dove 20 == h, cioe' sul singoletto {20}, ottenuto per esaurimento e non per campione. Quella riga non c'e' ne' nella tabella ne' nel test: le tre asserzioni di authorization_unrevoked.rs sono a 19, 21 e 49.\n\nDimostrato eseguendo: sostituito <= con < in authorization.rs, cioe' fatto mordere la revoca dal blocco successivo al proprio invece che dal proprio, e la suite resta a 190 passati e 0 falliti. File ripristinato e suite riverificata a 190. Un'implementazione conforme a ogni test di questo progetto puo' quindi divergere sull'altezza 20, che e' l'altezza che la regola nuova esiste per definire: due nodi conformi, verdetti opposti, su un burn.\n\nE' la stessa forma di REVIEW-033 RF-004, dove la frontiera della clausola 1 non era pinnata e la mutazione lasciava 176 test verdi. Quella remediation aggiunse la riga 5, che oggi c'e' ed e' corretta: la clausola nuova ha ereditato la lacuna che la vecchia aveva gia' chiuso. Il documento contiene inoltre, tre righe sotto, la frase che condanna il proprio difetto.\n\nRF-002 medium: e' RF-009 di REVIEW-038 che si materializza. La passata di ADR-012 non nomina ne' l'analisi dei dieci parametri ne' consensus_parameters_closure.py, e ConsensusParametersBody passa da venti a ventidue campi. Peggio del previsto: lo strumento conta 22 e stampa \"Classification of all 20 fields\", col venti cablato nella riga di stampa. La chiusura sostanziale regge e l'exit e' 0, quindi lo strumento passa mentendo - famiglia 1 nella forma peggiore, non un artefatto che fallisce ma uno che afferma il falso restando verde.\n\nLa chiusura di RF-001 va provata come il difetto: mutare <= in < deve far fallire la suite. Per RF-002 conviene togliere il numero cablato invece che aggiornarlo, cosi' che il difetto non torni al ventitreesimo campo.\n\nDichiarato come non guardato dal Lead: GATE-TWO-ORACLES resta dichiarata dall'implementatore e non riprodotta, e se quella seconda derivazione di AUTH-0 non fosse davvero indipendente RF-001 sarebbe il sintomo e non la causa. Vale la pena che AGENT-007 la guardi durante GATE-SECREVIEW, che resta da soddisfare insieme a GATE-CI-GREEN."
    evidence_refs: ["SPEC-022", "ADR-017", "REVIEW-033", "REVIEW-038", "DEBT-036", "ADR-012"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-039-EVENT-003"
    timestamp: "2026-08-26T23:38:24.305067800+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "RF-001 e RF-002 rimediate.\n\nRF-001: aggiunta la riga h=20 alla tabella AUTH-0 di ledger.md (verdetto invalid) e il caso di conformita' the_revocation_bites_exactly_at_its_inclusion_height in authorization_unrevoked.rs, che asserisce Revoked{height:20, included_height:20}. Corretta la frase che attribuiva la frontiera della clausola 2 alla riga 21: la frontiera e' 20, ed e' l'unica altezza che separa <= da <, perche' i due predicati differiscono esattamente dove 20 == h. Chiusura provata come il difetto: mutando record.included_height <= including_height in <, cargo test --workspace --all-features --no-fail-fast passa da 191 passati / 0 falliti a 190 passati / 1 fallito, e il test che fallisce e' esattamente quello nuovo. File ripristinato con git checkout e suite riverificata a 191/0. Conteggio guardato: 190 prima, 191 dopo.\n\nAggiornati anche i due probe C10 di published_artifacts.toml che pinnavano la vecchia frase, e aggiunto unrevoked-clause-two-boundary-row che pinna la riga 20 come unrevoked-clause-one-boundary-row pinna la riga 5. Probe: 158 prima, 159 dopo, lette dalla riga C10-PROBE dello strumento. La frase della divergenza porta ora il proprio perimetro: tre righe divergenti di questa tabella, contro l'intervallo 20 <= h <= 49 di altezze divergenti di cui le righe sono un campione - la distinzione che REVIEW-033 RF-007 imponeva.\n\nRF-002: tolto il numero cablato da consensus_parameters_closure.py invece di aggiornarlo. Le due righe di stampa che dicevano 20 (Classification of all 20 fields e PASS: all 20 ... fields; la seconda non era nel finding ma era lo stesso difetto) derivano ora il conteggio dai campi estratti dallo schema; la docstring non afferma piu' alcun conteggio. Lo strumento stampa 22 ovunque, exit=0, e la sua prova in negativo resta PASS. Aggiornata la lista di passata di ADR-012 nella spec nominando entrambi gli artefatti e registrando che la riga d'apertura di .lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md e' affidata alla remediation di SPEC-023, fuori dal confine di file di questa.\n\nGate rieseguite: cargo test 191/0, clippy exit=0, fmt exit=0, published_artifacts.py PASS, published_artifacts_negative.py PASS, protocol_hashes.py PASS, genesis_chain_id.py exit=0, lead_claims_check.py PASS, consensus_parameters_closure.py e --negative exit=0. Non toccato il file dell'altra remediation. GATE-SECREVIEW e GATE-CI-GREEN restano da soddisfare e non sono state affrontate qui."
    evidence_refs: ["SPEC-022", "REVIEW-039", "ADR-017", "ADR-012", "REVIEW-033"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-039-EVENT-004"
    timestamp: "2026-08-27T00:06:20.063539700+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Remediation verificata dal Lead rieseguendo e riproducendo, non leggendo. Entrambi i finding chiusi, e RF-002 chiuso oltre il richiesto.\n\nRF-001 chiuso e la prova riprodotta dal Lead. La riga h=20 e' nella tabella AUTH-0, marcata invalid subito dopo la 19 valida, e il caso di conformita' the_revocation_bites_exactly_at_its_inclusion_height esiste. Il Lead ha applicato la mutazione da <= a < su authorization.rs e ha osservato la suite passare da 191 passati e 0 falliti a 190 passati e 1 fallito, con il test che fallisce esattamente quello aggiunto e nessun altro. File ripristinato e suite riverificata a 191 e 0. La stessa mutazione, prima della remediation, lasciava 190 test verdi: e' la differenza fra una frontiera dichiarata e una tenuta.\n\nDue cose fatte meglio del richiesto. La prosa sulle righe divergenti porta ora il proprio perimetro, distinguendo le tre righe divergenti di quella tabella dalle altezze divergenti, che sono l'intero intervallo 20..49 di cui le righe sono un campione. E sullo strumento l'implementatore ha trovato da se' una seconda riga col numero cablato che il finding non nominava, togliendola allo stesso modo: i numeri ora derivano da len() in tutte e tre le righe che li portano, e la docstring non afferma piu' conteggi.\n\nRF-002 chiuso anche nella parte di processo: la lista di passata di ADR-012 nomina ora entrambi gli artefatti e registra che la riga d'apertura dell'analisi era affidata alla remediation di SPEC-023, invece di tacere dove fosse finito il pezzo non fatto.\n\nVerificato dal Lead eseguendo: 191 test 0 falliti, C10 da 158 a 159 probe, published_artifacts e la sua negativa, consensus_parameters_closure e la sua negativa, lead_claims_check, tutte exit 0.\n\nPerimetro dichiarato di questa accettazione. Il Lead non ha riprodotto GATE-TWO-ORACLES: la seconda derivazione di AUTH-0 resta dichiarata dall'implementatore e non verificata indipendentemente. Non ha verificato la passata di ADR-012 in modo esaustivo, ma solo i quattro artefatti che REVIEW-036 RF-008 aveva enumerato piu' i due di RF-002. Non ha valutato la parte 2 contro un avversario: e' materia di GATE-SECREVIEW.\n\nAccettazione della sola review. SPEC-022 NON e' chiudibile: GATE-CI-GREEN e GATE-SECREVIEW sono before-done ed entrambe insoddisfatte, e la seconda e' AGENT-007 su una consegna che cambia il predicato di autorizzazione delle transazioni - la superficie su cui REVIEW-036 aveva prodotto dieci finding sulla sola decisione."
    evidence_refs: ["SPEC-022", "ADR-017", "REVIEW-033", "REVIEW-036", "REVIEW-038", "ADR-012"]
    implementation_agent: "AGENT-002"
links: [DEBT-033, DEBT-034, DEBT-035, DEBT-036]
created: 2026-08-26
updated: 2026-08-27
tags: [review, security, ledger]
related_decisions: [ADR-017, ADR-012]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-26
    action: "recorded review remediation"
  - date: 2026-08-27
    action: "transitioned changes-requested -> accepted"
---
# Review

## Outcome

**Changes requested, su due voci.** L'attuazione di [ADR-017] è corretta nella sostanza: la parte 1 e la parte 2 sono nei documenti e nel crate, i tre vincoli di genesi ci sono, la ritrattazione è una vera ritrattazione, e la regola locale del ricevente **non è stata toccata** — che era lo scopo escluso più facile da violare.

**Il finding che conta è uno solo, ed è `high`: la frontiera della clausola nuova non è tenuta da nulla, e il documento afferma il contrario.** L'ho dimostrato mutando l'implementazione, non leggendo.

## Acceptance-criteria compliance

Quattordici criteri su quattordici spuntati. **Il Lead ne ha rieseguiti otto in modo indipendente**; sette reggono, e il dodicesimo è quello che porta RF-001.

| Criterio | Verifica del Lead |
| --- | --- |
| Clausola 2 legge l'altezza di inclusione | **Confermato** in `ledger.md` e in `authorization.rs` |
| Banda di `reason` come regola, `reason` letto, due righe esercitate | **Confermato**: `identity_revocation.rs` ha `key_compromise_effective_height_band` e il gemello per le altre due ragioni |
| Tre parametri nel blocco dei vincoli e in genesi | **Confermato**: `F >= 1`, `G >= 1`, `P >= F + G` |
| `F = 0` rifiutato | **Confermato** dalla riga `min_revocation_effective_delay_blocks >= 1` |
| Vincoli valutati all'altezza di inclusione | **Confermato**: `effective_height_evaluated_at_inclusion_height_against_active_parameters` |
| `AUTH-0` ricalcolata, righe `21` e `49` ribaltate | **Confermato leggendo la tabella**: entrambe ora `invalid`, e la colonna `effective_height <= h` è stata sostituita da `revocation included at <= h` |
| `ledger.md:785` riletta e legata a `F >= 1` | **Confermato** |
| Commento di `RevocationRecord` ritrattato | **Confermato, ed è fatto bene**: dice *«That rationale was invalidated by [ADR-017]»* e perché, invece di riscrivere in silenzio |
| Checkpoint annotato | **Confermato** |
| Non retroattività diventa due frasi | **Confermato**: una per la via dell'autorizzazione, una per quella del set |
| Passata di [ADR-012] eseguita, `published_artifacts.py` `PASS` | **Rieseguito**, `PASS` — **ma la passata ha un buco**, RF-002 |
| **Prova in negativo su ogni regola nuova** | **Falsificato dal Lead.** Vedi RF-001 |
| Regola locale del ricevente non toccata | **Confermato**: il diff di `identity.md` non contiene alcuna modifica alla riga della testa finalizzata del ricevente |
| Test, clippy, fmt puliti | **Rieseguiti**: **190 passati, 0 falliti** (erano 181), gate di progetto tutte `exit=0` |

## Code observations

`RevocationRecord` porta ora `included_height` al posto del campo assente, e il commento di ritrattazione è il modello di come si fa: **nomina la motivazione precedente, dice che è stata invalidata, e da cosa.**

La riga che decide tutto è `authorization.rs`:

```rust
// The comparison is `<=`: `included_height` is the first height at which
.find(|record| record.included_height <= including_height)
```

Il commento dichiara l'intento. **Nessun test lo tiene**, ed è RF-001.

## Tests and verification

Eseguiti dal Lead, non presi dall'evidenza: `cargo test --workspace --all-features` → **190 passati, 0 falliti**; `published_artifacts.py`, la sua prova in negativo, `consensus_parameters_closure.py`, la sua prova in negativo e `lead_claims_check.py` → **tutte `exit=0`**.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=correctness | severity=high | criterion=dodicesimo criterio, prova in negativo | remediation=aggiungere la riga `20` alla tabella `AUTH-0` e il caso di conformità corrispondente, e correggere la frase che attribuisce la frontiera alla riga `21`
**La frontiera della clausola 2 non è esercitata, e il documento afferma che lo sia.**

`ledger.md` scrive, della fixture `AUTH-0`:

> Row `21` (and the inclusion height `20`) is the boundary of clause 2: a revocation bites *at* its inclusion height, so clause 2 is `<=` and not `<`.

**La riga `21` non distingue `<=` da `<`.** La revoca è inclusa a `20`; con `<=` la riga `21` è invalida perché `20 <= 21`, e con `<` è invalida lo stesso perché `20 < 21`.

**Le due letture divergono su una sola altezza, e la si ottiene per esaurimento invece che per campionamento.** I predicati `20 <= h` e `20 < h` differiscono esattamente dove il primo è vero e il secondo falso, cioè dove `20 == h`: **l'insieme delle altezze divergenti è quindi il singoletto `{20}`**, contato e non stimato. Quella riga non c'è né nella tabella né nel test: la tabella salta da `19` (valido) a `21` (invalido), e le tre asserzioni di `authorization_unrevoked.rs` sono `qualification_at(REVOKED, 19).is_ok()`, `a_revocation_included_at_20_bites_at_21` e `..._at_49`. **Nessuna a `20`** — enumerazione dei casi del file, non campione.

**Dimostrato eseguendo, non dedotto.** Il Lead ha sostituito in `authorization.rs` il confronto `record.included_height <= including_height` con `record.included_height < including_height` — cioè ha fatto mordere la revoca dal blocco **successivo** al proprio invece che dal proprio — e ha rieseguito la suite: **190 passati, 0 falliti.** Un'implementazione conforme a ogni test di questo progetto può quindi divergere sull'altezza `20`, che è precisamente l'altezza che la regola nuova esiste per definire. Il file è stato ripristinato e la suite riverificata a 190.

**È la stessa forma di [REVIEW-033] RF-004**, dove AGENT-007 aveva trovato che la frontiera della clausola 1 non era pinnata e che mutare `valid_from_height` in minore-stretto lasciava 176 test verdi. Quella volta la remediation aveva aggiunto la riga `5`, che infatti oggi c'è ed è corretta. **La clausola nuova ha ereditato la lacuna che la clausola vecchia aveva già chiuso.**

**E il documento contiene la frase che condanna il proprio difetto**, tre righe sotto: *«a clause stated with an inclusive comparison and exercised only away from the boundary is a clause whose boundary is a guess»*. È scritta, ed è vera della clausola che la ospita.

**Scenario.** Non serve un avversario: serve una seconda implementazione. Un implementatore che legga la regola come *«dal blocco dopo l'inclusione»* — lettura non irragionevole, visto che `effective_height` MUST essere **strettamente** successivo al proponente — passa ogni test di conformità di questo progetto e diverge su un'autorizzazione di spesa all'altezza `20`. **Due nodi conformi, verdetti opposti, su un burn.** È un fork sulla validità di un blocco, e la classe di danno che [SPEC-019] esisteva per chiudere.

RF-002 | category=documentation | severity=medium | criterion=undicesimo criterio, passata di ADR-012 | remediation=aggiornare i tre numeri, e togliere il `20` cablato dalla riga di stampa
**La passata di [ADR-012] non ha nominato i due artefatti che questa spec rende falsi**, ed è il rilievo che [REVIEW-038] RF-009 aveva previsto quando questa spec era ancora in corso.

`ConsensusParametersBody` passa da venti a **ventidue** campi. Nessuno dei due file seguenti è nominato nella passata di questa spec, e nessuno dei due è nell'inventario, il cui perimetro dichiarato è `docs/protocol/`:

- `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md` apre con *«`ConsensusParametersBody` definisce **venti parametri**»*;
- `sim/tools/consensus_parameters_closure.py` ha una docstring che dice *«twenty»*, *«Ten election»*, *«The other ten»*.

**E c'è di peggio di quanto RF-009 prevedesse: il numero falso è nell'output a runtime.** Lo strumento oggi stampa:

```
ConsensusParametersBody fields: 22 total
Classification of all 20 fields:
```

Conta ventidue e ne dichiara venti, perché il `20` è **cablato nella riga di stampa**. Verificato dal Lead eseguendo.

**Nessuna gate lo coglie e nessuna lo coglierà**, ed è la ragione per cui questo conta: la chiusura sostanziale **regge** — i due campi nuovi sono in entrambe le liste, `union covered: 22`, `exit=0` — quindi lo strumento **passa mentendo**. È famiglia 1, nella forma peggiore: non un artefatto che fallisce, ma uno che afferma il falso restando verde.

## Cosa il Lead ha attaccato senza riuscire a romperlo

1. **La frontiera della clausola 1.** Prima di guardare la clausola 2 ho controllato che la riga `5` fosse ancora lì e ancora corretta: `h = valid_from_height` autorizza, quindi la clausola è `<=`. **Regge**, ed è la remediation di [REVIEW-033] che ha tenuto.
2. **Lo sconfinamento sulla regola locale del ricevente**, che era lo scopo escluso più facile da violare — è la superficie di [DEBT-034] e sarebbe stato comodo «sistemarla» passando. Il diff di `identity.md` **non la tocca**.
3. **La ritrattazione del commento.** Ho controllato che fosse una ritrattazione e non un aggiornamento silenzioso, perché la regola di forma lo impone e perché era facile aggirarla riscrivendo. **Nomina la motivazione precedente e dice da cosa è stata invalidata.**
4. **La non retroattività.** Ho cercato il caso in cui la parte 1 renda invalida retroattivamente una transazione storica. Non c'è, e ora il documento lo dice in due frasi invece che in una, una per percorso.
5. **I tre vincoli di genesi.** Ho verificato che ci siano davvero e nel blocco giusto, e non solo nella prosa: `F >= 1`, `G >= 1`, `P >= F + G`.
6. **Le due righe della banda di `reason`.** Entrambe esercitate, più il caso che valuta i vincoli contro i parametri in vigore all'altezza di inclusione, che era la clausola 3 di [ADR-017] — quella che [REVIEW-036] aveva dichiarato di **non** aver attaccato.

## Cosa il Lead non ha guardato

- **Non ho verificato la passata di [ADR-012] in modo esaustivo.** Ho eseguito lo strumento e ho controllato i quattro artefatti che [REVIEW-036] RF-008 aveva enumerato. **Se la passata avesse mancato un quinto artefatto in `docs/protocol/`, questa review non lo coglierebbe** — lo coglierebbe l'inventario solo se il difetto cadesse dentro una probe esistente.
- **Non ho riderivato `AUTH-0` per una seconda strada indipendente.** `GATE-TWO-ORACLES` chiede due derivazioni; io ho verificato la tabella **contro la regola**, che è una sola strada. La seconda è nell'evidenza dell'implementatore e **non l'ho riprodotta**.
- **Non ho valutato la parte 2 contro un avversario.** Che la banda di `reason` regga a un quorum ostile è materia di `GATE-SECREVIEW`, che resta da soddisfare — ed è la gate su cui [REVIEW-036] aveva prodotto dieci finding sulla sola decisione.
- **Non ho eseguito la pipeline CI.** `GATE-CI-GREEN` resta `before-done`.
- **Non ho riletto `app-manifest.md`**, che [REVIEW-036] aveva dichiarato di non aver guardato e che la definizione di [SPEC-019] raggiunge.

## Required follow-up

- **RF-001 prima dell'accettazione.** È una riga di tabella e un caso di test. La chiusura va provata come il difetto: mutare `<=` in `<` deve far **fallire** la suite.
- **RF-002 nello stesso giro**, e conviene togliere il numero cablato dalla riga di stampa invece che aggiornarlo, così che il difetto non torni al ventitreesimo campo.
- **`GATE-SECREVIEW` resta da soddisfare**, ed è AGENT-007 su una consegna che cambia il predicato di autorizzazione delle transazioni.
- **`GATE-CI-GREEN` resta da soddisfare.**
- **`GATE-TWO-ORACLES` è dichiarata dall'implementatore e non riprodotta dal Lead**: se la seconda derivazione di `AUTH-0` non fosse davvero indipendente, RF-001 sarebbe il sintomo e non la causa. Vale la pena che AGENT-007 la guardi.
