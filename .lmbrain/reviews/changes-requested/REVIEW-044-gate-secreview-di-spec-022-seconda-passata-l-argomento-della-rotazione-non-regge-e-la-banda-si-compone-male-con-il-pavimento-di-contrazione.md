---
id: REVIEW-044
# Note: Quote the title if it contains a colon
title: "GATE-SECREVIEW di SPEC-022, seconda passata: l'argomento della rotazione non regge, e la banda si compone male con il pavimento di contrazione"
status: changes-requested
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-022
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, robustness, documentation, verification-integrity]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-044-EVENT-001"
    timestamp: "2026-08-27T11:58:48.038972800+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Due high confermati dal Lead verificando ogni gamba, e nessuno dei due sta nella remediation di AGENT-002: stanno entrambi nell'argomento del Lead che la remediation attua fedelmente. RF-002..RF-007 di REVIEW-042 sono chiusi bene e non sono stati rotti.\n\nRF-001 verificato: ADR-018 punto 7, redatta dal Lead il 2026-08-27, dice \"Nessuna regola dice chi propone\", e l'argomento della rotazione presuppone il contrario di un fatto che il Lead aveva firmato lo stesso giorno. voting_power = 1 e' imposto ai soli set eletti, quindi il set di genesi non e' vincolato. permissive_bounds() porta revocation_effective_grace_blocks_min e validator_min_set_size_min entrambi a 1 con set massimo 1000: soddisfa la relazione con G = 1, la finestra di due blocchi da cui REVIEW-042 era partita. La probe revocation-grace-floor-is-one-rotation-of-the-minimum-set pinna la frase falsa e published_artifacts.py e' verde su di essa.\n\nIl Lead ha inoltre accertato che la soglia sbagliata rende il pavimento una difesa contro la minaccia sbagliata: sotto il protocollo a due fasi di ADR-018 oltre un terzo del potere fa fallire ogni round trattenendo i precommit, e nessuna larghezza di finestra difende da quella soglia.\n\nDirezione decisa dall'operatore il 2026-08-27, dopo che il Lead ha corretto la propria raccomandazione della mattina: togliere il tetto della banda sul reason key_compromise. I due high hanno la stessa causa — il tetto e' cio' che rende la finestra una superficie da difendere e cio' che ha tolto il rimedio della diluizione su piu' transizioni. Sul reason urgente un ritardo di inclusione torna a poter solo rimandare, mai distruggere.\n\nNessuna remediation parte prima che la correzione di ADR-017 parte 2 sia scritta e approvata: mandare l'implementatrice a chiudere questi due high senza quella decisione produrrebbe numeri inventati, che e' esattamente cio' che questa consegna ha evitato bene."
    evidence_refs: ["SPEC-022", "ADR-017", "ADR-018", "REVIEW-042", "REVIEW-036", "DEBT-040"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
links: [DEBT-040, DEBT-045, DEBT-035]
created: 2026-08-27
updated: 2026-08-27
tags: [security, review, ledger, consensus]
related_decisions: [ADR-017, ADR-018, ADR-010, ADR-012]
activity:
  - date: 2026-08-27
    action: "transitioned pending -> changes-requested"
---
# Review

> **`GATE-SECREVIEW` di [SPEC-022]**, seconda passata, eseguita sull'albero a `852abe3` (`git status` pulito all'inizio e alla fine).
> Oggetto: la remediation di AGENT-002 su tutti e otto i rilievi di [REVIEW-042], la seconda derivazione di `AUTH-0`, e — su richiesta esplicita — **l'argomento con cui il Lead ha derivato la forma del pavimento di `G`**, che nessuno aveva attaccato.

## Outcome

**Non superata.** Due `high`, due `low` non bloccanti.

**Sei rilievi su otto di [REVIEW-042] sono chiusi bene**, e due dei tre che sospettavo vuoti — il selettore di RF-004 e il nuovo oracolo — sono reali e li ho rotti per provarlo. **Nessuno dei difetti nuovi sta nella remediation: stanno tutti e due nell'argomento che la remediation attua.**

- **RF-001**, `high`: l'argomento della rotazione con cui [ADR-017] giustifica `revocation_effective_grace_blocks_min + 1 >= validator_min_set_size_min` **non regge su nessuna delle sue tre gambe**, ed è ora **testo normativo di `ledger.md`**, ripetuto in `README.md` e in `params.rs`, e **pinnato da una probe**. Lo strumento della passata di [ADR-012] è verde su un'affermazione di sicurezza che il protocollo non sostiene. È la stessa forma di [REVIEW-042] RF-005 un livello sopra: là una frase resa ambigua, qui una frase resa falsa.
- **RF-002**, `high`: la **composizione fra la banda e il pavimento di contrazione del set** — la superficie che [REVIEW-042] e AGENT-002 hanno entrambe dichiarato non attaccata. Un lotto di `key_compromise` che condivide un solo `effective_height` può **fermare la catena per sempre**, e la mossa che la ferma è la risposta d'emergenza corretta, non l'attacco. Il tetto della banda, nuovo di questa consegna, **toglie il rimedio** che il documento stesso dichiara (`ledger.md:1986`, *«over several transitions it can»*).

**RF-001 è di nuovo contro il Lead e contro un ADR approvato**, come [REVIEW-042] RF-001. Non è lo stesso difetto: quello diceva *dove* stava il pavimento, questo dice che **la grandezza scelta è derivata da un argomento sbagliato**. La correzione del 2026-08-27 chiude davvero lo scenario che [REVIEW-042] aveva costruito — l'ho verificato — e lascia aperto il perché del numero.

## Cosa ho ESEGUITO, e cosa ho solo LETTO

**Eseguito** (Windows 11, ambiente locale unico, `cargo 1.96.0`):

- `cargo test --workspace --all-features --no-fail-fast` → **195 passed, 0 failed**, contati sommando le quindici righe `test result:` non vuote. Conferma il conto del Lead.
- `python sim/tools/published_artifacts.py` → `PASS`, `C10-PROBE 172`. Conferma il conto delle probe.
- `python sim/tools/auth0_oracle.py` → `PASS`, frontiere `[5, 20]` per esaurimento, `effective_height 50` non è frontiera. `--negative` → `PASS`, due mutazioni.
- **Tre mutazioni del documento che l'oracolo legge**, che la sua prova in negativo **non** esegue — perché quella muta solo il predicato dello strumento, mai `ledger.md`. Ogni file ripristinato da una copia presa prima, `git status` pulito dopo ognuna:
  - riga `49` della tabella `AUTH-0` ribaltata a `valid` → `FAIL row at h=49 ... table says valid, the rule derives invalid`;
  - riga di frontiera `20` cancellata dalla tabella → `FAIL the boundary height 20 found by exhaustion has no row in the table`;
  - testo della clausola 2 spostato da *«at a height at most `h`»* a *«at a height below `h`»* → `FAIL: the clause this oracle implements is not in ledger.md`.
  **L'oracolo morde da tutti e tre i lati.** La prova in negativo che porta con sé non lo dimostra; queste tre sì.

**Letto e non eseguito:** `params.rs` (`ElectionBounds`, `check_revocation_grace_floor`, `check_magnitudes`, `ELECTION_PARAMETERS`), `identity.rs` (le due funzioni della banda), `light_client.rs` (`authenticate_consensus_parameters`), `constraint_block.rs` (le 24 righe di sweep e le loro asserzioni), `identity_revocation.rs` (i due test della clausola 3), `common/mod.rs` (PD-0 e `permissive_bounds`), `ledger.md` §*What `enrolled, unrevoked` means*, §*Revocation forces a validator set transition*, §*Rotation: the cap and the floor*, §*Magnitudes, not only relations*, `README.md` §`ElectionBounds`, `SECURITY.md`, [ADR-017] corretta, [ADR-018], [DEBT-040], [DEBT-045], `published_artifacts.toml`, `sim/coblox_sim/recommended.py`.

**Non guardato, dichiarato:** `GATE-CI-GREEN` — nessuna pipeline eseguita; `apps/`, `dist/`, i binding FFI; la superficie della raggiungibilità ([DEBT-034]); il costo reale della rifirma sotto censura, che resta il numero mancante che [ADR-017] nomina nella propria sezione *Revisit*.

**Perimetro di ogni affermazione sul proponente qui sotto:** riguardano la regola **decisa** in [ADR-018], non del codice. **Un motore di consenso non esiste**: `coblox-core` non ha alcun ciclo di proposta o di voto, e [SPEC-025] non è consegnata. È precisamente il punto di RF-001.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security-boundary | severity=high | criterion=ADR-017 parte 2, correzione del 2026-08-27; GATE-ADR012-PASS; QUALITY.md §*Technical judgement* | remediation=togliere da `ledger.md`, `README.md`, `params.rs` e [ADR-017] ogni affermazione di un turno di proposta per validatore che nessuna regola di `docs/protocol/` sostenga, e ripuntare la probe che la pinna

**L'argomento della rotazione ha tre gambe e nessuna regge. È ora testo normativo, ed è pinnato da una probe.**

Il testo attaccato, con gli spazi normalizzati perché attraversa tre a capo — `ledger.md:2099-2103`, ripetuto in `README.md:1057-1060` e in `core/coblox-core/src/params.rs:59-61`:

> *«the window lasts at least one full rotation of the minimum set — so every validator of the smallest lawful set holds a proposal turn inside it. A coalition able to censor an entire rotation already holds the quorum, and against that coalition this parameter is not the defence in question.»*

**(a) Nessuna regola del protocollo assegna un proponente.** [ADR-018], punto 7 dei fatti verificati dal Lead sull'albero **lo stesso giorno**, lo scrive in queste parole: *«Nessuna regola dice chi propone. La parola proposer compare in tutto il documento come soggetto di cui si ragiona — cosa può macinare, cosa può ordinare — e mai come ruolo assegnato da una regola.»* Ho rieseguito l'enumerazione, e il perimetro è `docs/protocol/`: **undici righe** contengono `proposer` — dieci in `ledger.md`, una in `wire.md` — e tutte lo usano come soggetto di cui si ragiona, nessuna come ruolo assegnato da una regola. **«Ogni validatore ha un turno di proposta dentro la finestra» è un'affermazione su un meccanismo che il protocollo non ha.**

**(b) La sola regola di proposta che esista è in [ADR-018] §3, ed è a sorteggio pesato, non a rotazione per teste.** *«Round-robin deterministico sul set di validatori attivo, indicizzato da `(height, round)` e **pesato per potere di voto**.»* Due conseguenze, entrambe fatali per la gamba (a):

- **Il periodo di una rotazione pesata è la somma dei pesi, non il numero dei membri.** `ledger.md:1278` e `:2401` impongono `voting_power = 1` **soltanto ai set eletti** — *«Elected sets use `validator_id = node_id` and uniform voting power»* — mentre la regola generale di validità di un `ValidatorSet` (`ledger.md:1003`) chiede solo che il potere sia *positivo*. Il set di genesi non è un set eletto. **È esattamente il set della devnet di [M-02], cioè il set in cui un `key_compromise` conta di più**, e su di esso una finestra di `validator_min_set_size_min` blocchi non dà un turno a ciascuno.
- **L'indice è `(height, round)`, non `height`.** [ADR-018] decide un protocollo a due fasi in cui un'altezza può essere tentata più volte (punto 4 dei fatti: *«I round esistono già»*). Su `k` altezze si consumano `k + (round falliti)` slot di proponente, e **quali** validatori vedano il proprio blocco finalizzare non è una permutazione del set. Una finestra di `k` altezze non è una finestra di `k` turni.

**(c) «Censurare una rotazione intera implica il quorum» è falso, e le due soglie sono i due lati della stessa scissione BFT.** Il quorum di questo protocollo è unico e stretto: `signed_power * 3 > total_power * 2` ([ADR-018] punto 3), cioè **oltre due terzi**. Sotto il protocollo a due fasi che [ADR-018] ha deciso, una coalizione con **oltre un terzo** del potere trattiene i precommit e fa fallire qualunque round il cui proponente non le piaccia: il round scade, si passa a `r+1` con un proponente diverso, e si ripete. **Con poco più di un terzo la coalizione decide di chi sia il blocco che finalizza a ogni altezza, quindi censura ogni altezza, indefinitamente.** Non le serve un terzo di potere *e* la fortuna di stare sui turni giusti: le serve un terzo. **La soglia di censura è `> 1/3`, la soglia di quorum è `> 2/3`.** L'ADR le dichiara la stessa cosa e non lo sono.

**(d) La relazione è ancorata al pavimento del pavimento, mentre la proprietà dipende dal set attivo.** `check_revocation_grace_floor` confronta `revocation_effective_grace_blocks_min + 1` con `validator_min_set_size_min`. Chi scrive ciascuna grandezza:

| grandezza | chi la scrive | mobile? |
| --- | --- | --- |
| `revocation_effective_grace_blocks_min` | la distribuzione di genesi | no |
| `validator_min_set_size_min` | la distribuzione di genesi | no |
| `validator_min_set_size` | il set seduto, in `ELECTION_PARAMETERS` | sì, per rapporto |
| `validator_max_set_size` | il set seduto, in `ELECTION_PARAMETERS` | sì, per rapporto |
| **dimensione del set attivo** | l'elezione, entro `[validator_min_set_size, validator_max_set_size]` | **sì** |

**Nulla lega `revocation_effective_grace_blocks_min` alla dimensione del set attivo, né al suo tetto di genesi `validator_max_set_size_max`.** Il controesempio non va costruito: **vive dentro questa consegna**. `core/coblox-core/tests/common/mod.rs:356,361,349` — `permissive_bounds()` con `revocation_effective_grace_blocks_min: 1`, `validator_min_set_size_min: 1`, `validator_max_set_size_max: 1_000` — passa `ElectionBounds::validate` (`1 + 1 >= 1`), e sotto quei bounds PD-0 (`:242-244`) porta `G = 1`: **finestra di due blocchi**, cioè esattamente lo scenario di [REVIEW-042] RF-001, con un set attivo di dodici e fino a mille ammessi. Sui soli bounds tarati che questo repository possieda — `sim/coblox_sim/recommended.py:54-56`, `validator_min_set_size_min=18`, `validator_max_set_size_max=81`, `validator_max_set_size=45` — la finestra sarebbe di **18 blocchi contro un set di 45**, cioè **meno di mezza rotazione**, e di **meno di un quarto** contro il tetto di genesi.

**Scenario d'attacco.** Una coalizione ha compromesso una chiave di un set attivo di 45 e vuole sopravvivere alla propria revoca. Il pavimento di genesi le garantisce che la finestra di inclusione sia di 18 blocchi. Le serve controllare i proponenti di 18 altezze consecutive, e sotto il protocollo deciso questo si ottiene con **oltre un terzo** del potere di voto — non con i due terzi che l'argomento presuppone. Con quel terzo non ha *«già il quorum»*, non può revocare nessuno, non può firmare un blocco da sola: può solo censurare, che è precisamente la capacità che l'argomento dichiara inesistente sotto il quorum. **La revoca è invalidata e il giro di firma va rifatto, contro un avversario che non ha né il quorum né bisogno di riprovare.**

**Cosa questo finding NON dice.** Lo scenario originale di [REVIEW-042] RF-001 **è chiuso, e l'ho verificato**: `revocation_effective_grace_blocks >= revocation_effective_grace_blocks_min` è ora un vincolo di magnitudine preso dai bounds di genesi, `ConsensusParameters::validate` lo raggiunge attraverso `check_magnitudes`, e un set seduto non può più pubblicare `G = 1` per governance ordinaria. **La doppia imposizione non è ridondanza**: ho letto `ConsensusParameters::validate` e non passa da `ElectionBounds::validate`, quindi la seconda chiamata è la sola che protegga un chiamante diretto. Ciò che questo finding attacca è **la magnitudine di quel pavimento, e l'argomento che la sceglie**, non la sua collocazione.

**La quarta domanda, chiesta e verificata: la relazione lega il caso peggiore.** `G >= G_min` è imposto, quindi `G + 1 >= G_min + 1 >= validator_min_set_size_min` per ogni documento ammissibile. **Su questo punto il Lead ha ragione** ed è scritto qui perché si sappia che è stato chiesto.

**Perché è bloccante e non una nota.** La probe `revocation-grace-floor-is-one-rotation-of-the-minimum-set` (`published_artifacts.toml:2019-2022`) **pinna l'affermazione**, e `published_artifacts.py` è `PASS`. Lo strumento della passata di [ADR-012] è quindi **verde su un'affermazione di sicurezza falsa**, ed è la terza volta in questa spec che un artefatto della passata certifica una frase che non regge ([REVIEW-042] RF-002 e RF-005 sono le prime due). `lead_claims_check.py` esiste in questo repository proprio perché *«per ciò che scrive il Lead autore e revisore sono la stessa persona»*: L2-SUPERLATIVE chiede l'enumerazione dietro un universale, e *«ogni validatore ha un turno»* è un universale senza enumerazione.

**Condizione di chiusura verificabile.** Nessuna fra `ledger.md`, `README.md`, `core/coblox-core/src/params.rs` e [ADR-017] afferma un turno di proposta per validatore che una regola di `docs/protocol/` non sostenga. La forma sostitutiva è una delle tre, ed è **decisione dell'operatore, non dell'implementatrice**: (i) la regola del proponente entra in `docs/protocol/` e l'argomento viene riderivato contro di essa, con la soglia di censura al posto della soglia di quorum e con il tetto del **set attivo** (`validator_max_set_size_max`) al posto di `validator_min_set_size_min`; (ii) il pavimento resta la relazione attuale ma la giustificazione viene sostituita da *«è la magnitudine che l'operatore ha scelto e questo è il perché»*, senza derivazione; (iii) il pavimento viene rifatto contro una grandezza che lo sostenga. In tutti e tre i casi la probe va ripuntata sulla frase nuova e `published_artifacts.py` deve restare `PASS`. **La correzione va fatta in [ADR-017] e non solo nella spec**: è un argomento del Lead, e correggere il documento attuativo lasciando in piedi la decisione ripete la forma di [REVIEW-042] RF-001.

RF-002 | category=robustness | severity=high | criterion=GATE-SECREVIEW; QUALITY.md §*Required engineering standard* (percorsi di fallimento e condizioni al contorno) | remediation=dichiarare la composizione dove la banda e lo stallo sono dichiarati, nominare la regola 10 fra le cause dello stallo, aprire il debito, e portarla nelle *Review conditions* di ADR-017 — **non** inventare un vincolo nuovo

**Un lotto di `key_compromise` che condivide un solo `effective_height` ferma la catena per sempre, e a fermarla è la risposta d'emergenza corretta.**

È la composizione che [REVIEW-042] ha dichiarato *«superficie reale, dichiarata e non raggiunta»* e che AGENT-002 ripete fra i limiti noti. L'ho raggiunta.

**Le quattro regole che si compongono**, tutte lette in `ledger.md`:

1. **regola 2** (`:1071`) — un blocco ad altezza `>= effective_height` il cui set attivo contenga il `node_id` revocato è **invalido**, e così ogni certificato di quorum contato contro quel set;
2. **regola 8** (`:1149`) — una transizione fuori boundary può rimuovere un `node_id` **solo se** la sua revoca ha `effective_height` **al più** l'`activation_height` di quella transizione. Un `effective_height` è quindi una **scadenza**, non un calendario: prima di `e` quel nodo **non si può togliere**;
3. **regola 10** (`:1153`) e **passo 7 dell'elezione** (`:1582`) — **pavimento di contrazione** `3 * member_count(new) > 2 * member_count(old)`, imposto **sia** fuori boundary **sia** al boundary di elezione. In un solo passo si possono togliere al più `V − ⌊2V/3⌋ − 1` membri;
4. **la banda di [SPEC-022]** — per `key_compromise`, `effective_height ∈ [p + F, p + F + G]`.

**Scenario d'attacco.** L'avversario compromette **`k` chiavi di consenso**, con `k` scelto appena sopra il limite di contrazione in un passo. Il quorum onesto fa la cosa corretta: firma **un lotto** di `k` revoche `key_compromise` — è un incidente solo — con **un** `effective_height` `e`, e le fa includere. Da quel momento:

- nessuna transizione ad `activation_height < e` può togliere alcuno dei `k` (regola 8);
- ogni blocco ad altezza `>= e` che li contenga è invalido (regola 2);
- la transizione ad `activation_height = e` dovrebbe toglierli **tutti insieme**, e viola il pavimento di contrazione (regola 10 / passo 7).

**Non esiste alcun set valido a `e`, quindi non esiste alcun blocco valido a nessuna altezza `>= e`. La catena si ferma, e non riparte**, perché ogni via di rimedio passa da un blocco che non può essere prodotto. Le revoche sono già finalizzate e non si disfano.

**Numeri, presi da PD-0** (`common/mod.rs:248-250`, `V = 12`): in un passo si tolgono al più `12 − 8 − 1 = 3` membri. **Con `k = 4` la catena si ferma per sempre.** Quattro chiavi su dodici sono un terzo, cioè meno della soglia a cui il documento dichiara lo stallo accettabile. Sul set raccomandato (`V = 45`) il valore è `k = 15`.

**Il documento dichiara lo stallo e ne dà la causa sbagliata.** `ledger.md:1132` lo attribuisce a *«if the remaining validators fail to commit a compliant successor set within the delay window»* — un fallimento dei superstiti. **Qui i superstiti non falliscono: gli è vietato.** E `ledger.md:1986` dichiara il rimedio — *«over several transitions it can, on the same terms and with the same publicity as any other contraction»* — che richiede `⌈log(V/k) / log(3/2)⌉` transizioni e quindi **altrettanti `effective_height` distinti**. La regola 8 rende un `effective_height` una scadenza: se coincidono, le transizioni intermedie non esistono.

**Cosa ha cambiato questa consegna, ed è il motivo per cui il finding è di [SPEC-022] e non del passato.** L'ipotesi coincidente è raggiungibile anche prima. Ma la clausola 4 preesistente aveva **pavimento e nessun tetto**: un quorum consapevole poteva sempre scaglionare gli `effective_height` **quanto voleva**, e il rimedio dichiarato a `:1986` era sempre disponibile. **Il tetto è nuovo di [SPEC-022]**: per `key_compromise` lo scarto ottenibile da una data altezza di inclusione è al più `G`, e **il pavimento di `G` è ora legato a `validator_min_set_size_min`**, che con `⌈log(V/k)/log(3/2)⌉` non ha alcuna relazione. È la forma del **denominatore sbagliato** che [ADR-017] ha tolto come [REVIEW-036] RF-009 — *«alzare `F` per rendere lo stallo più raro raddoppierebbe la latitudine sulla cattiva condotta»* — **rientrata dall'altro lato**: ora la grandezza tarata sulla dimensione del set governa quante transizioni di contrazione una revoca di massa può permettersi.

E c'è un secondo lato, che è la faccia in tensione con la parte 1: **scaglionare gli `effective_height` obbliga a scaglionare le inclusioni**, perché `e >= p + F` e il quorum non conosce `p` quando firma. **Scaglionare le inclusioni è precisamente ciò che, sotto la parte 1, lascia le chiavi compromesse a spendere più a lungo.** Le due metà di [ADR-017] tirano l'una contro l'altra sul caso di massa, ed è l'amplificazione della tensione che [ADR-017] dichiara aperta come *«la tenuta della parte 1 dipende dalla prontezza dell'autorizzazione»*.

**Perché è bloccante.** È `GATE-SECREVIEW` a esistere per questo, e [SPEC-022] §*Risks* istruisce esplicitamente che una via emersa su questa composizione va **riportata al Lead invece che risolta dentro la spec**. Non è bloccante perché serva una regola nuova: è bloccante perché un arresto permanente della catena, raggiungibile dalla risposta d'emergenza corretta a un attacco che l'avversario dimensiona, **non può restare non scritto in un documento che dichiara il proprio stallo e ne nomina la causa sbagliata**.

**Condizione di chiusura verificabile.** (1) `ledger.md` §*Revocation forces a validator set transition* nomina la **regola 10** fra le cause dello stallo e dichiara che un lotto di `key_compromise` che condivide un `effective_height` è limitato dal pavimento di contrazione in un passo; (2) la sezione della banda dichiara che il tetto limita lo scaglionamento e quindi il numero di transizioni di contrazione disponibili, e che `G` non è tarato su quel numero; (3) una probe pinna entrambe le frasi e `published_artifacts.py` resta `PASS`; (4) esiste un debito con criterio di risoluzione, e le *Review conditions* di [ADR-017] lo nominano. **Nessun vincolo numerico nuovo va inventato dall'implementatrice**: se ne serve uno è dell'operatore, e il suo denominatore è `⌈log(V/k)/log(3/2)⌉`, non la dimensione del set.

RF-003 | category=documentation | severity=low | criterion=nessuno; nota non bloccante | remediation=allargare i criteri di risoluzione di DEBT-045 a `sim/coblox_sim/recommended.py`, o dichiarare che quel file non copre la revoca

**L'unico valore tarato di `validator_min_set_size_min` che il repository possieda impone `G >= 17`, e la fixture pubblicata porta `G = 1`. Nessuno strumento li confronta.**

`sim/coblox_sim/recommended.py:54` porta `validator_min_set_size_min=18`. Sotto la relazione nuova ciò impone `revocation_effective_grace_blocks_min >= 17` e quindi `G >= 17`. Lo stesso file **non porta alcun parametro di revoca** e `sim/coblox_sim/params.py` non conosce la parola `revocation` — è [DEBT-045], già aperto e correttamente riportato invece che corretto. La nota che [DEBT-045] non copre è che il difetto non è solo di trascrizione: **la relazione nuova rende il set raccomandato e la fixture pubblicata reciprocamente incoerenti**, e i criteri di risoluzione di [DEBT-045] nominano `params.py` e non `recommended.py`.

RF-004 | category=verification-integrity | severity=low | criterion=GATE-TWO-ORACLES; nota non bloccante | remediation=aggiungere alla prova in negativo di `auth0_oracle.py` almeno una mutazione del documento, non solo del proprio predicato

**La prova in negativo dell'oracolo prova che l'oracolo sbaglia quando lo si rompe, non che accorga un documento sbagliato.**

Le due mutazioni di `auth0_oracle.py --negative` cambiano entrambe il **predicato dello strumento** (`strict_clause_2`, `read_effective_height`) e mai `ledger.md`. Il caso che la gate esiste per rifiutare — una **tabella** costruita male — non è fra quelle. Ho eseguito io tre mutazioni del documento e **tutte e tre sono state colte**, quindi lo strumento è sano: la nota è che **la sua evidenza non lo dimostra**, e chi la leggesse in futuro concluderebbe più di quanto sia scritto.

Seconda faccia, minore: in esercizio ordinario l'asserzione `flips == [valid_from_height, included_height]` è una **tautologia**, perché `flip_heights` usa lo stesso `verdict()` le cui uniche frontiere sono quei due fatti. Morde solo sotto le mutazioni dello strumento. Non è un difetto, è una riga che dimostra meno di quanto sembri.

## Cosa la reviewer ha attaccato senza riuscire a romperlo

1. **RF-002 di [REVIEW-042], la frase della clausola 2: ha una lettura sola.** Verificata con gli spazi normalizzati. Il preambolo dice *«of the chain formed by that block and its ancestors»*, la clausola dice *«no `revoke_identity` in that chain names `node_id` at a height at most `h` — the block at `h` included»*, e **la parola `finalized` è tolta da entrambe le clausole**, con un paragrafo che dichiara perché — *«a revocation in block `h` and a spend in block `h` share the fate of block `h`»*. Ho cercato il singoletto su cui le due letture divergevano: a `h = 20` la nuova frase dà **invalid** senza ambiguità. La giustificazione è rovesciata come richiesto: *«The predicate never consults intra-block execution order, and that is the reason it is safe»* è ora la frase portante e l'ordine di esecuzione è etichettato come coerenza. **Cinque probe nuove** pinnano la forma corretta e le due frasi che la spiegano, e le due che pinnavano la forma ambigua sono ripuntate. **Non rotto.**
2. **RF-004, il selettore: non è un criterio soddisfatto alla lettera.** `validate_effective_height_in_block` ricava **entrambi** gli argomenti dallo stesso `BlockHeader` e delega il legame a `light_client::authenticate_consensus_parameters`, che **ricalcola** l'hash del documento e lo confronta con `header.consensus_parameters_hash`, rifiutando prima di computare qualunque banda. Il test vacuo è rinominato in ciò che dimostra davvero, e i due nuovi coprono la clausola 3 nei due sensi: stesso corpo e stessa altezza con due epoche di parametri danno verdetti opposti, e un documento **autentico ma di un'altra epoca** è rifiutato con `consensus_parameters_hash MUST equal the hash of the trusted header`. **Non rotto.** Resta vero che non ci sono chiamanti fuori dai test, ma **in questo crate non esiste alcuna pipeline di validazione dei blocchi**: è lo stato del progetto, non di questo criterio.
3. **RF-003, lo sweep: copre il confine esatto in entrambe le direzioni.** 24 righe, per ognuna il confine `old*4/5` o `old*5/4` **accettato** e un passo oltre **rifiutato** con `ChangeRatio { parameter: name }` che nomina il parametro. Ho cercato il conflitto che il rilievo chiedeva: la seconda base è costruita perché `P >= F + G` regga **un passo oltre ogni confine in entrambe le direzioni**, così che un rifiuto possa venire solo dal rapporto. Ho verificato a mano che il rapporto non confligga né con `P >= F + G` — entrambi i lati scalano dello stesso fattore — né con `G >= G_min`, che è un tetto sul cammino e non una contraddizione. **Non rotto.**
4. **L'indipendenza del secondo oracolo.** Non legge `core/coblox-core/`: l'unico percorso del file è `docs/protocol/ledger.md`. Reimplementa le clausole dal **testo letterale** e **rifiuta di girare** se quel testo non c'è — l'ho provato. Legge la tabella *soltanto* per confrontarla. Trova le frontiere **per esaurimento** su `0..60`. Fallisce se `effective_height` è una frontiera, che è la firma del ribaltamento che [REVIEW-042] sospettava, e fallisce se una frontiera trovata per esaurimento non ha una riga — l'ho provato togliendo la riga `20`. **La prima spunta era falsa; la seconda non lo è.**
5. **La collocazione del pavimento di `G`, e la doppia imposizione.** Lo scenario di [REVIEW-042] RF-001 è chiuso: un set seduto non può più pubblicare `G = 1`. Ho verificato la giustificazione della doppia chiamata leggendo `ConsensusParameters::validate`, che **non** passa da `ElectionBounds::validate`: la seconda linea in `check_magnitudes` è la sola che protegga un chiamante diretto, e non è ridondanza.
6. **`G_min + 1` contro `G + 1`.** La relazione lega il **caso peggiore**, perché `G >= G_min` è imposto. La domanda era giusta da porre e la risposta è a favore del Lead.
7. **RF-005, RF-006, RF-007.** `SECURITY.md:113-118` distingue ora i due ritardi e dice in chiaro che sul percorso di spesa il ritardo è zero; `threat-model.md` e la voce `AUTH-0` di `published_artifacts.toml` sono riscritte; `morde` ha **zero** occorrenze residue su `docs/` + `core/` + `sim/` + `SECURITY.md`, con la sola stringa dentro il campo `why` della probe che documenta la correzione — l'ho contato; `min_by_key` sostituisce `find` con il test delle due permutazioni. **Chiusi.**

## Giudizio sulle gate

- **`GATE-TWO-ORACLES`: ora soddisfatta anche su `AUTH-0`.** La seconda derivazione esiste, è indipendente nell'origine dei verdetti e nel modo in cui l'insieme dei casi è determinato, e i suoi guardiani mordono — provato con tre mutazioni del documento. La casella dichiara che la prima spunta era a torto invece di limitarsi a essere corretta, ed è la forma giusta.
- **`GATE-NEGATIVE-PROOF`: soddisfatta.** Cinque mutazioni sull'albero, ognuna con **un solo** test fallito e in tutti e cinque i casi quello che la regola esiste per tenere. Non le ho rieseguite: ho eseguito la suite intera e il conto quadra.
- **`GATE-ADR012-PASS`: soddisfatta come esecuzione, e questo è il problema di RF-001.** `published_artifacts.py` è `PASS` con 172 probe, e **una di quelle probe pinna l'affermazione falsa di RF-001**. La passata è verde su una frase che non regge, che è la terza occorrenza in questa spec della stessa forma.
- **`GATE-LEAD-REPRO`: attestata, e non la contesto.** La riproduzione della derivazione di `AUTH-0` è stata fatta senza l'oracolo, dai tre fatti della prosa, con le frontiere cercate per esaurimento su `0..60`, e coincide con la mia. La mutazione del pavimento è coerente con l'M1 dell'evidenza. **Nessuna delle verifiche del Lead che ho toccato è risultata falsa**, e il conto dei 195 test è esatto.
- **`GATE-CI-GREEN`: non toccata**, è del Lead e nessuna pipeline è stata eseguita.

## Verdetto raccomandato

**`review_changes_requested`.**

La ragione, in una riga: **due `high` restano aperti, e nessuno dei due sta nella remediation — stanno entrambi nell'argomento che la remediation attua fedelmente.** La consegna di AGENT-002 è la parte solida di questa passata; l'argomento del Lead è la parte che cede.

- **RF-001** blocca perché un'affermazione di sicurezza falsa è **testo normativo** di `ledger.md`, replicata in due altri artefatti, e **certificata verde** dallo strumento della passata di [ADR-012]. Va corretta dove è stata decisa, cioè in [ADR-017], e non solo dove è scritta.
- **RF-002** blocca perché `GATE-SECREVIEW` esiste per trovarlo e perché [SPEC-022] §*Risks* impone di riportare al Lead ciò che emerge su questa composizione. **La sua remediation è documentale**, non un vincolo nuovo: il vincolo, se lo si vuole, è dell'operatore.

**RF-003 e RF-004 sono note non bloccanti** e non aprono da sole un giro di remediation.

**Nota di processo per il Lead.** Entrambi i `high` richiedono una decisione dell'operatore prima che ci sia qualcosa da implementare: RF-001 sceglie fra tre forme sostitutive, RF-002 sceglie se dichiarare o vincolare. **Non sono rilievi che AGENT-002 possa chiudere da sola**, e mandarla a chiuderli senza la decisione produrrebbe numeri inventati — che è esattamente la cosa che questa consegna ha evitato bene.

## Required follow-up

- **RF-001 prima di tutto, e in [ADR-017]**: la correzione del 2026-08-27 ha chiuso lo scenario e ha lasciato in piedi la derivazione. Serve la scelta dell'operatore fra le tre forme, poi la riscrittura di `ledger.md:2099-2103`, `README.md:1057-1060`, `params.rs:55-62`, e il ripuntamento della probe `revocation-grace-floor-is-one-rotation-of-the-minimum-set`.
- **RF-002 nello stesso giro**, con un debito proprio e una riga nelle *Review conditions* di [ADR-017]. Il debito è vicino a [DEBT-040] ma non è lo stesso: [DEBT-040] dice che le larghezze sono ordinate al contrario fra i `reason`, questo dice che **la larghezza non è tarata sul lavoro che deve permettere**.
- **[DEBT-045]**: allargare i criteri di risoluzione a `sim/coblox_sim/recommended.py`, o dichiarare per iscritto che quel file non copre la revoca.
- **La regola del proponente va in `docs/protocol/`** prima che qualunque altra regola di sicurezza vi si appoggi. [ADR-018] l'ha decisa il 2026-08-27 e il documento non la porta: questo finding è il primo caso di un argomento di sicurezza che vi si appoggia senza che esista, e non sarà l'ultimo.
