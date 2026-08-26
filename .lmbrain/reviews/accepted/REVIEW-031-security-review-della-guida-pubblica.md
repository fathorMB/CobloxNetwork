---
id: REVIEW-031
# Note: Quote the title if it contains a colon
title: "Security review della guida pubblica (GATE-SECREVIEW di SPEC-015, chiude DEBT-023): la pagina è invecchiata su tutto ciò che il protocollo ha imparato a misurare"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-015
reviewer: AGENT-007
review_requested_by: AGENT-LEAD
implementation_agent: AGENT-006
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-031-EVENT-001"
    timestamp: "2026-08-26T12:14:20.888541700+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Security review della guida pubblica, che chiude DEBT-023. Verdetto: pubblicabile dopo correzioni, non prima. Tre high, quattro medium, due low. I bloccanti sono tre paragrafi e non una riscrittura.\n\nIl risultato che vale oltre i finding e' che il rinvio dell'operatore era corretto: nessuno dei tre high era falso quando AGENT-006 ha scritto la pagina. Una review il 25 agosto avrebbe trovato due low e avrebbe assolto la guida, firmandola.\n\nRiverificato dal Lead sui file: la parola \"epoch\" compare ZERO volte nella pagina, \"four ninths\" zero, \"two thirds\" una, e nessuno degli otto details porta open.\n\nRF-001 high: §05 promette che il set cede seggi \"whatever anybody intends\" (riga 419). In blocchi e' vero, in tempo reale e' falso, e la manovra costa un terzo bloccante - meno di ogni altra cosa in questo protocollo, ed e' l'attacco che SECURITY.md nomina come il piu' economico.\n\nRF-002 high: \"period\" porta quattro significati distinti tutti contati in blocchi, mai definiti, e la pagina non contiene ne' \"epoch\" ne' alcuna menzione dei blocchi come unita'. Il lettore esce convinto che l'emissione sia limitata nel tempo; e' limitata per epoca, e il lato veloce della banda ammette il doppio dell'emissione reale prima di obiettare. E' la frase di SECURITY.md senza il qualificatore per-epoca che SECURITY.md ha acquistato dopo SPEC-016.\n\nRF-003 high: la pagina dice due terzi e non dice mai quattro noni. E' il punto che DEBT-023 nominava per nome.\n\nLa classe che sta sotto tre high su tre e' la scoperta di questa review, ed e' nuova: entrambi gli strumenti passano, 126 probe e 65 claims presenti nei due versi, ma in RF-001, RF-002 e RF-004 la claims della probe si ferma PRIMA della clausola che porta il rischio e difende la mezza frase modesta accanto. Non e' un caso: la clausola che eccede non e' ancorabile proprio in quanto eccede. Serve una passata sulle 65 con una domanda sola.\n\nRF-008 e' l'unico finding che aggiunge invece di togliere: il §07 si ferma un passo prima del passo 4b di ledger.md, e omette l'unica difesa che il lettore possa esercitare da solo - misurare se la catena stia correndo troppo contro una banda che non puo' apprendere da un peer."
    evidence_refs: ["SPEC-015", "DEBT-023", "SECURITY.md"]
    implementation_agent: "AGENT-006"
    remediation_agent: "AGENT-006"
  - schema_version: "1"
    id: "REVIEW-031-EVENT-002"
    timestamp: "2026-08-26T13:01:13.977573400+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Remediation dei nove finding consegnata da AGENT-006. Tre high chiusi, RF-004 RF-005 RF-006 RF-008 RF-009 chiusi, RF-007 non fatto con argomento. Piu' la passata sulle 76 claims, che ha prodotto una regola misurabile. Da verificare dal Lead."
    evidence_refs: ["SPEC-015", "DEBT-023"]
    implementation_agent: "AGENT-006"
    remediation_agent: "AGENT-006"
  - schema_version: "1"
    id: "REVIEW-031-EVENT-003"
    timestamp: "2026-08-26T13:01:33.466141800+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificata dal Lead sui file: \"whatever anybody intends\" zero occorrenze, \"four ninths\" due, la definizione di periodo in blocchi presente. published_artifacts.py PASS con 137 candidati C10 da 126; prova in negativo PASS con 15 mutazioni su 11 classi piu' tutte e 137 le probe osservate fallire una per una; check-guide-pairs.mjs PASS su sei classi con 76 claims da 65; check-contrast.mjs 130 coppie su 130.\n\nLe tre mutazioni che reintroducono i bloccanti verbatim ora fanno rosso e prima non facevano fallire nulla: rimettere \"whatever anybody intends\" fa fallire due probe per nome, cancellare il paragrafo dei quattro noni ne fa fallire una, cancellare il paragrafo che definisce il periodo ne fa fallire due.\n\nLa passata sulle claims ha prodotto piu' dell'elenco chiesto, ed e' il contributo che vale oltre questa remediation. L'implementatrice ha reso misurabile la domanda della reviewer con uno script che, per ogni probe, riporta cio' che resta della frase dopo la fine della claims: 52 coprono la frase intera, 24 lasciano una coda, e cinque erano della classe oltre ai tre della review. Fra queste, una probe che si chiama needs-your-signature e la cui claims non conteneva la parola signature, e una che pinnava quattro parole di un'etichetta di diagramma.\n\nE ne ha ricavato una regola provata contro se stessa: estendere la claims fino al confine di clausola non basta. Aveva esteso una claims fino alla virgola, ha rimesso la frase falsa nella pagina, e il controllo e' rimasto verde perche' la stringa era ancora li' seguita da una virgola invece che da un punto. La regola e' che la claims finisce dove finisce la frase, punto fermo compreso, ed e' scritta nel README della guida.\n\nHa inoltre trovato una forma dell'omissione che la review non contemplava: in una probe la clausola che porta il rischio PRECEDE la claims invece di seguirla. E ha contestato una riga su tre della tabella della reviewer, correttamente: li' il campo conteneva gia' la frase intera, e il difetto non era l'ancoraggio ma il fatto che la probe pinnava una parola il cui significato la pagina non aveva mai dato al lettore. Terza forma distinta del guasto.\n\nRF-007 non fatto, con l'argomento giusto: la reviewer lo formula come una differenza da decidere dal Lead invece che assorbire in silenzio, e assorbirla scrivendo sei parole sarebbe stato il gesto contro cui e' scritto."
    evidence_refs: ["SPEC-015", "REVIEW-031"]
    implementation_agent: "AGENT-006"
    remediation_agent: "AGENT-006"
  - schema_version: "1"
    id: "REVIEW-031-EVENT-004"
    timestamp: "2026-08-26T13:01:46.882992100+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata dopo un giro di remediation. La guida e' pubblicabile.\n\nIl risultato che vale piu' dei nove finding e' che il rinvio dell'operatore era corretto e ora e' dimostrato: nessuno dei tre high era falso quando la pagina fu scritta, e una review fatta il 25 agosto avrebbe trovato due low e l'avrebbe assolta firmandola. La ragione per cui l'operatore aveva rinviato - le affermazioni di sicurezza non sono rivedibili contro un protocollo che sta ancora cambiando - si e' verificata in ventiquattro ore.\n\nRestano tre questioni che sono decisioni del Lead o dell'operatore e non residui nascosti. RF-007: la quarta cosa scomoda arriva alla quarta sezione mentre il debito ne chiedeva tre; costa sei parole e l'implementatrice si e' fermata invece di assorbirla. La probe guide-light-client-fails-closed pinna sei parole e lascia scoperte quattro condizioni: estenderla farebbe sembrare ancorate quattro condizioni su una regola sola, che e' il peccato che questa review e' venuta a censire, e servono tre probe nuove verso ledger.md. E il campo claims e' uno per probe mentre la pagina afferma alcune cose due volte, quindi la seconda occorrenza resta non difesa comunque la si scelga: le due uscite cambiano lo schema di un file che published_artifacts.py legge."
    evidence_refs: ["SPEC-015", "DEBT-023", "SECURITY.md"]
    implementation_agent: "AGENT-006"
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [review, security, documentation]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-26
    action: "recorded review remediation"
  - date: 2026-08-26
    action: "recorded review remediation-verification"
  - date: 2026-08-26
    action: "transitioned changes-requested -> accepted"
---
# Review

## Outcome

**Raccomandazione: changes requested. Tre finding `high`, quattro `medium`, due `low`.**

**Verdetto sulla pubblicazione, che è la domanda a cui l'operatore aspetta risposta: la guida è pubblicabile dopo correzioni, e le correzioni sono paragrafi e non una riscrittura.** Non è pubblicabile così com'è. I tre `high` non sono difetti di stile né di prudenza: sono tre punti in cui la pagina afferma, nel filo principale e in una frase che il lettore ripeterà, qualcosa che il protocollo di oggi nega esplicitamente. Nessuno dei tre era falso quando AGENT-006 ha scritto la pagina.

Questo è il risultato che conta e che giustifica il rinvio deciso dall'operatore il 25 agosto. La ragione dichiarata del rinvio era che *le affermazioni di sicurezza della guida non sono rivedibili contro un protocollo che sta ancora cambiando*. Il protocollo è cambiato, e ha cambiato **esattamente** i due punti su cui questa pagina appoggia le sue promesse più forti: la cadenza reale della catena e il modo in cui la rotazione dei validatori è tenuta. Se questa review fosse stata fatta il 25 agosto avrebbe trovato due `low` e avrebbe assolto la pagina. Il rinvio non è stato un rinvio della disciplina: è stato il momento giusto scelto per la ragione giusta.

Va detto in positivo e prima dei finding, perché è vero e perché [[lead-claims-discipline]] impone di attaccare per prima la parte che sembra migliore: **il metodo di questa pagina è il migliore del progetto e non è il metodo a essere in questione.** La regola di forma — *se la versione semplice è più forte di quella esatta, la versione semplice è sbagliata* — è applicata davvero e non dichiarata; le 65 probe `guide-*` sono un'invenzione utile, e la seconda direzione (`claims`, la frase che deve restare nella pagina) è la metà che quasi nessuno scrive. Ho eseguito entrambi gli strumenti e passano entrambi. **Il difetto non è nella macchina, è in ciò che è stato scelto di ancorare:** in tutti e tre i `high` la clausola portante è quella che la `claims` della probe **non copre**, e la probe difende la mezza frase innocua che le sta accanto.

## I tre criteri di [DEBT-023]

Rispondo ai tre criteri per esteso più sotto; qui il verdetto secco di ciascuno.

**Criterio 1 — una qualunque affermazione insegna una forma che il protocollo non ammette?** **Sì, in quattro punti**, tre dei quali nel filo principale: RF-001, RF-002, RF-004, RF-005.

**Criterio 2 — le tre cose scomode sono leggibili a blocchi chiusi?** **Sì. Il criterio è soddisfatto**, e l'ho verificato in modo meccanico e non a impressione: nessuno degli otto `<details>` porta l'attributo `open`, e nessuno dei tre blocchi `.plainly` è annidato dentro un `<details>`. Un residuo esiste ed è RF-007 `low`, non un fallimento del criterio.

**Criterio 3 — la pagina lascia intendere garanzie più forti di `SECURITY.md`?** **Sì, sui due punti nominati dal debito, e su entrambi nel modo previsto.** La soglia di controllo del set è RF-003: la pagina dice *due terzi* e non dice mai *quattro noni*. La resistenza Sybil è RF-002: la pagina la dà per contenuta senza il qualificatore **per-epoca** che `SECURITY.md` ha aggiunto dopo [SPEC-017], e senza quel qualificatore la frase della guida è più forte della frase di `SECURITY.md` che dovrebbe sostenerla.

**La quarta domanda, che il debito non conteneva — la guida è invecchiata?** **Sì, in tutti e due i versi.** In avanti: RF-001, RF-002 e RF-003 dicono cose che [SPEC-016], [SPEC-017] e [SPEC-021] hanno reso false o gravemente incomplete. All'indietro: RF-008 — la pagina non dice la cosa migliore che il protocollo abbia da dire a un lettore ordinario, cioè che un telefono può **misurare** se la catena sta correndo troppo, contro una banda fissata alla genesi, e rifiutarsi di procedere. È l'unica difesa di questo protocollo che il lettore della guida possa esercitare da solo, ed è arrivata dopo che la pagina era stata scritta.

## Domanda 1 — La pagina ha una nozione di tempo? (criterio 1, e la radice dei due `high` economici)

**No, e la sua assenza è la radice di RF-001 e RF-002.**

La parola **`period`** compare dieci volte e non è mai definita. Porta quattro significati distinti:

| Occorrenza | Significato reale | Contato in |
| --- | --- | --- |
| §02 «two independent askers per period» | epoca di sfida | blocchi |
| §03 «a fixed pot for each period» | `reward_epoch` | blocchi |
| §05 «terms of `T` periods … every period» | `election_epoch` | blocchi |
| §04 «goes into a grace period» | grazia dell'app | blocchi |
| §05, §07 «cooling-off period» | raffreddamento | blocchi |

Tutte e cinque sono contate in **blocchi**. La pagina non lo dice mai, in nessun punto, aperto o chiuso. Un lettore che non ha letto una specifica di protocollo — cioè il lettore per cui questa pagina esiste, dichiaratamente — legge «period» come un'unità di tempo, perché in italiano come in inglese è ciò che la parola significa fuori da questo documento.

La forma che questo insegna è che **esiste un orologio**, e che le quantità della rete siano ancorate a esso. `docs/protocol/README.md:100` dice il contrario nella riga più netta del documento: *«no v0 validity rule imposes the cadence»*. Ogni grandezza che questa pagina promette al lettore — la sua quota di emissione, la rotazione dei validatori, la sua attesa di raffreddamento, la scadenza dell'app che ha finanziato — è denominata in blocchi, e **il numero di blocchi al secondo è scritto dai validatori stessi**.

Non chiedo che la pagina spieghi `reward_epoch_blocks`. Chiedo una frase, nel filo, che dica che i periodi sono contati in blocchi e non in minuti, e che il ritmo dei blocchi è misurato e non imposto. È la stessa cosa che `SECURITY.md` dice in un titolo di paragrafo — *How fast the chain runs is measured, not enforced* — e la pagina che spiega la rete a chi non leggerà `SECURITY.md` non la dice.

## Domanda 2 — Le probe coprono le clausole che portano il peso? (criterio 3, e la ragione per cui i `high` sono sopravvissuti)

**No, e la forma dell'omissione è la stessa tre volte su tre.**

Ho eseguito entrambi gli strumenti e passano:

```text
$ python sim/tools/published_artifacts.py
  C10-PROBE        126 candidate(s) checked
published-artifact inventory: PASS

$ node .lmbrain/design/coblox-public-guide/tools/check-guide-pairs.mjs
  G6-CLAIM-STILL-MADE      65 candidate(s) checked
public-guide form check: PASS
```

Il campo `claims` di una probe è una **sottostringa** della frase della guida, e `G6` verifica che quella sottostringa sia ancora presente. La conseguenza, che nessuno strumento può vedere, è che la clausola **oltre** la sottostringa non è difesa da nulla. Tre casi, tutti e tre finding di questa review:

| Probe | `claims` (protetto) | La clausola che segue nella pagina (non protetta) |
| --- | --- | --- |
| `guide-turnover-is-arithmetic` | «gives up at least» | «**whatever anybody intends**» → RF-001 |
| `guide-quorum-cannot-name-its-successors` | «…they can narrow the field of candidates» | «**but the composition comes out of a derivation from randomness they cannot choose**» → RF-004 |
| `guide-existence-fund-capped-by-construction` | «capped by construction rather than by anyone's restraint» | il **soggetto** della frase, «the period's», che è ciò che la rende vera → RF-002 |

In tutti e tre i casi la parte ancorata è la parte modesta e la parte che eccede è la parte scoperta. Non è un caso: un'affermazione modesta è facile da ritrovare in un documento di regole, e un'affermazione eccessiva non lo è **perché nessuna regola la sostiene** — che è precisamente ciò che il `README.md` della guida dichiara di voler impedire (*«se un'affermazione non è ancorabile perché nessuna regola la tiene, non è una semplificazione: è un'invenzione»*). La regola è giusta ed è stata scritta; la scelta della sottostringa l'ha aggirata senza che nessuno se ne accorgesse.

Questo produce anche RF-009, perché il colophon della pagina promette al lettore che il meccanismo copre **ogni** frase di garanzia.

## Domanda 3 — Le tre cose scomode arrivano in tempo? (criterio 2)

**Sì.** Verificato sui file e non a impressione.

- **Nessuno degli otto `<details>` porta `open`**: `grep -c open` sulle otto righe `<details` dà `0`. La pagina si apre chiusa.
- **I tre blocchi `.plainly` sono nel filo**, non dentro un apribile: righe 64 (§01), 177 (§03), 353 (§04), tutte prima del `<details>` della rispettiva sezione (75, 300, 380). `guide.css` non li nasconde: sono prosa con un bordo.

Leggendo il filo con tutti i blocchi chiusi:

| Cosa scomoda | Dove è **detta** | Dentro un apribile? |
| --- | --- | --- |
| non c'è resistenza Sybil crittografica | §01 `.plainly` | no |
| lo pseudonimo è stabile, quindi debole | §03 `.plainly` | no |
| il saldo è pubblico e permanente | §03 `.plainly` | no |
| gli **abbonamenti** sono pubblici e permanenti | §04 `.plainly` | no |

Il §06 le raccoglie e non le introduce, che è ciò che il `README.md` della guida chiede. Il criterio è soddisfatto.

Il residuo, ed è RF-007: [DEBT-023] pone la prova nella forma «un lettore che si convince nelle prime tre sezioni e non apre nulla». Al termine del §03 quel lettore ha incontrato tre delle quattro, e **non** ha incontrato quella sugli abbonamenti — che è la sola delle quattro su cui deve compiere un atto irreversibile, ed è la sola che la pagina stessa (§06, apribile) dichiara dover arrivare *prima* dell'atto. Il §04 la dice bene e la dice prima che l'abbonamento sia possibile, quindi non è un fallimento. È un margine di una sezione su un criterio che la pagina ha scelto di darsi.

## Review findings

### RF-001 — `high` — «whatever anybody intends» è falso in tempo reale, e la manovra che lo rende falso costa un terzo bloccante

§05, filo principale:

> *They are not a permanent committee. Every seat carries an expiry stamped on it when it is filled, so a set with `V` members and terms of `T` periods gives up at least `V` divided by `T` seats every period, **whatever anybody intends**.*

È la promessa anti-cattura più enfatica della pagina, e la clausola finale è quella su cui il lettore appoggia la fiducia. In blocchi è vera. In tempo reale è falsa, e la sua falsità è **il caso più economico** dell'intera tabella delle minacce di `SECURITY.md`:

> *«**Stretching** lengthens, in real time, everything the protocol denominates in blocks: validator incumbency, and the effective delay of a revocation. It requires only a **blocking third**, which simply withholds the quorum. […] The cheaper attack is the one on incumbency.»*

Un terzo bloccante — cioè **meno** di quanto serva a fare qualunque altra cosa in questo protocollo, e meno della soglia di RF-003 — allunga il mandato reale dei validatori in carica di quanto vuole, senza violare alcuna regola e senza che nessun blocco diventi invalido. `docs/protocol/README.md:107` lo dice per esteso: *«the active validator set determines the real-time duration of its own epochs, and therefore of its own incumbency, without breaking any rule»*.

La pagina insegna quindi la forma inammissibile per eccellenza: che la rotazione sia garantita **contro un avversario**, quando è garantita solo contro l'inerzia. E lo fa nella sezione che il lettore legge per decidere se fidarsi di chi tiene i registri.

**Perché non è stato colto.** La probe `guide-turnover-is-arithmetic` ancora la sottostringa `"gives up at least"` alla riga `Turnover is consequently not a target but an arithmetic certainty` di `ledger.md`. La regola citata è vera e resta vera: la certezza aritmetica è **in blocchi**. La clausola «whatever anybody intends» non è nella `claims`, non è in nessuna probe, e non è sostenuta da nessuna regola.

**Riproduzione.**
```text
$ grep -n "whatever anybody intends" .lmbrain/design/coblox-public-guide/index.html
419:            period, whatever anybody intends. A departing member sits out a cooling-off period and

$ grep -n 'claims = ' sim/tools/published_artifacts.toml | sed -n '/turnover/p'
# la claims della probe è "gives up at least" — la clausola non vi compare

$ grep -n "no v0 validity rule imposes the cadence" docs/protocol/README.md
104:seconds. But **no v0 validity rule imposes the cadence.**
```
Prova in negativo del fatto che la guardia non vedrebbe il difetto: cancellando la clausola dalla pagina, `check-guide-pairs.mjs` continua a passare, perché la `claims` che difende è intatta. La guardia non protegge la frase che porta il rischio.

**Rimedio suggerito.** Sostituire la clausola con la cosa esatta, che è più corta e non più difficile: i periodi sono contati in blocchi, quindi la rotazione è certa **in blocchi** e i validatori in carica possono rallentare la produzione dei blocchi, il che allunga il loro mandato in tempo reale. E aggiungere la probe sulla clausola nuova, verso il paragrafo di `README.md:104-112`.

### RF-002 — `high` — «capped by construction» è vero per epoca e falso in tempo reale, e la pagina non ha la parola per dire «epoca»

§03, filo principale e apribile:

> *(filo)* *…that part is a **fixed pot for each period**, divided among everyone who qualified.*
> *(apribile)* *The remainder is not minted at all, so the **period's** presence emission is **capped by construction rather than by anyone's restraint**.*
> *(§07, filo)* *That is why the pot for presence income is **capped and shared**.*

Sotto il significato reale di «period» — `reward_epoch`, contata in blocchi — le tre frasi sono vere e sono ancorate correttamente. Sotto il significato che il lettore darà a «period», sono false, e sono false nel verso che `SECURITY.md` ha imparato a dichiarare **dopo** che questa pagina è stata scritta:

> *«**This bound is per reward epoch, and not per unit of real time.** The epoch index is paced by block height, so a validator quorum that compresses the real cadence multiplies real issuance whatever the fleet does»*

E `docs/protocol/README.md:1427-1431` misura di quanto: il lato veloce della banda di cadenza *«admits a real issuance rate up to **twice** the intended one and refuses beyond it»*. Cioè un quorum può raddoppiare l'emissione reale **restando dentro la banda**, senza che nulla se ne accorga.

Questo è il criterio 3 nella sua forma prevista dal debito. `SECURITY.md` oggi porta il qualificatore per-epoca in due punti e lo porta perché il progetto ha deciso deliberatamente di portarlo. **La guida dice la stessa cosa senza il qualificatore**, ed è quindi più forte del documento che dovrebbe sostenerla — che è la definizione operativa del criterio 3.

Osservazione che aggrava: il §01 apribile dice *«Ten fake devices do not create more credits»*. È vero per una flotta, e resta vero. Ma composto con «capped by construction» e con «capped and shared» del §07, il lettore esce dalla pagina con la convinzione che **la quantità totale di credits creata nel tempo sia limitata da una regola**. Non lo è. È limitata da una regola **per epoca**, e chi controlla la cadenza controlla quante epoche entrano in un giorno.

**Riproduzione.**
```text
$ grep -n "period" .lmbrain/design/coblox-public-guide/index.html | wc -l
10
$ grep -in "epoch\|block height\|blocks per\|in blocks" .lmbrain/design/coblox-public-guide/index.html
# nessun risultato: la pagina non contiene la parola "epoch" né alcuna menzione dei blocchi come unità

$ grep -n "This bound is per reward epoch" SECURITY.md
$ grep -n "reward_epoch. is derived from height" docs/protocol/ledger.md
669:because [`reward_epoch` is derived from height](#reward_epoch-is-derived-from-height).
```

**Rimedio suggerito.** Una frase nel filo del §03 che dica che un «period» è un numero fisso di blocchi e non un intervallo di tempo, e che la cadenza reale dei blocchi è scritta dai validatori — quindi il tetto vale **per periodo** e non al mese. È il qualificatore che `SECURITY.md` porta già, tradotto. Poi correggere la `claims` di `guide-existence-fund-capped-by-construction` perché includa il soggetto della frase.

### RF-003 — `high` — La pagina dice «due terzi» e non dice mai «quattro noni», che è il punto su cui il debito avvertiva per nome

§05, prima frase, filo principale:

> *A block of entries counts as settled when more than two thirds of their voting power has signed it — strictly more, not two thirds exactly.*

La frase è vera, è precisa fin nel `>` contro `>=`, e ha due probe. **Non è però la sola soglia che esiste, e il lettore non ha modo di saperlo.** Ciò che questa frase insegna, letta da chi non conosce il protocollo, è: *per fare qualunque cosa contro questa rete servono più di due terzi dei validatori*. `docs/protocol/ledger.md:2194-2202` nega esattamente questo:

> *«Control of the set […] about **four ninths** […] the argument that "above two thirds BFT safety has already failed" **does not apply to the quorum threshold**, because at `4V/9` BFT safety has not failed at all. That argument was what made the previous claim look harmless, and it is exactly the step that was wrong in each of the three refutations.»*

La tabella dell'attacco è pubblicata nello stesso documento: con `V = 27`, una coalizione di **13 seggi, il 48,1 %**, forza una contrazione lecita a 19 seggi e da lì detiene il quorum, *«no rule is violated and honest nodes sign the block, because the block is valid»*. La frazione tende a `4/9` dal di sopra.

Il debito nomina questo punto per nome — *«la soglia di controllo del set, che è circa quattro noni e non i due terzi che l'intuizione suggerisce»* — e `ledger.md` registra che il progetto ci ha già sbagliato **tre volte**. La guida pubblica ripete oggi l'intuizione che è stata rifiutata tre volte, e la ripete al pubblico invece che in una specifica interna.

Non chiedo che la guida spieghi la contrazione per attrito. Chiedo che non lasci il lettore con l'unico numero che è stato dichiarato ingannevole. Una frase basta: *«due terzi servono per firmare un blocco. Prendere il controllo del gruppo costa meno — circa quattro noni — con una manovra descritta per intero in `SECURITY.md`.»*

**Riproduzione.**
```text
$ grep -in "four ninth\|4/9\|nine\|contraction\|attrition" .lmbrain/design/coblox-public-guide/index.html
# nessun risultato

$ grep -n "about four ninths" docs/protocol/ledger.md
1660:The fraction decreases with set size and approaches **`4/9`, 44.4 %**, from
2197:> `S_new = max(floor(2V/3) + 1, validator_min_set_size)` — about **four ninths**

$ grep -n "four" SECURITY.md
97:selectively, drive a lawful contraction, and reach a quorum from roughly **four
```
`SECURITY.md` porta il numero; la guida non lo porta in alcuna forma.

### RF-004 — `medium` — «randomness they cannot choose» afferma la proprietà che `ledger.md` dichiara esplicitamente di non avere, e la pagina si smentisce da sola

§05, filo principale:

> *the sitting validators cannot name their successors: they can narrow the field of candidates, but the composition comes out of **a derivation from randomness they cannot choose**.*

`docs/protocol/ledger.md:1223`:

> *«So the seed is **not** trusted to be unbiasable, and the security of this section does not rest on it»*

Il documento rifiuta la proprietà e costruisce la sicurezza altrove — sul tetto `validator_churn_cap_seats` applicato **dopo** l'estrazione, e sul fatto che la scadenza del mandato non sia funzione del seed. Il residuo è quantificato: un attaccante che grinda ottiene `c*p + O(sqrt(c*p*(1-p)*2*ln G))` seggi invece di `c*p`. La guida attribuisce la sicurezza alla proprietà sbagliata: quella che il protocollo ha deliberatamente **non** assunto.

Aggrava che la pagina lo sappia già, altrove. Il §02 apribile spiega correttamente il grinding del beacon, con la sua misura, e conclude *«It is a reduction, not a fix, and it is written down as one.»* La stessa pagina è precisa sul grinding in §02 e ne dimentica l'esistenza in §05. Questo è il caso più chiaro in cui la disciplina della pagina ha funzionato in un punto e non nell'altro.

**Riproduzione.**
```text
$ grep -n "randomness they cannot choose" .lmbrain/design/coblox-public-guide/index.html
423:            narrow the field of candidates, but the composition comes out of a derivation from
$ grep -n "not\*\* trusted to be unbiasable" docs/protocol/ledger.md
1222:So the seed is **not** trusted to be unbiasable, and the security of this section
```
La `claims` di `guide-quorum-cannot-name-its-successors` è `"the sitting validators cannot name their successors: they can narrow the field of candidates"` e si ferma **prima** della clausola incriminata.

**Rimedio suggerito.** Riscrivere la clausola sul meccanismo che davvero regge, che è più semplice da dire: *quanti seggi cambiano a un confine è limitato da un tetto applicato dopo l'estrazione, quindi chi può influenzare l'estrazione non può comunque prenderne più di quel tetto*. Ed estendere la `claims` fino a coprire la nuova clausola.

### RF-005 — `medium` — «neither does the node asking them» è falso per le sfide di calcolo, ed è nel filo principale

§02, filo principale:

> *You do not choose which questions you get, and **neither does the node asking them**.*

Vero per la coppia richiedente-soggetto, che `wire.md` deriva dal beacon. Vero per la `randomness`, che non è né scelta dal soggetto né dall'emittente. **Falso per il contenuto della sfida di calcolo.** `docs/protocol/wire.md:234-240`:

```text
ComputeAssignment = {
  "app_id": sha256-string,
  "module_hash": sha256-string,
  "input_hash": sha256-string,
  "input": base64url(bytes),
  "fuel_limit": u64-string
}
```

`input` è scelto **verbatim dall'emittente**, e con esso `module_hash`. È [DEBT-024], scenario TM-42, che descrive la capacità nei termini esatti che questa frase della guida nega: *«un validatore sceglie quale modulo un host determinato esegue e con quale input, senza alcun tetto dichiarato sul numero di assegnazioni né sulla taglia dell'input»*.

Il lettore della guida esce con la convinzione di non poter essere **puntato**. È la proprietà che [ADR-006] ha deliberatamente tolto al publisher, e che il canale delle sfide di calcolo restituisce a un validatore. La frase seguente della guida rafforza l'errore invece di limitarlo: *«so it cannot go looking for a question you happen to be able to answer»* — che per il calcolo è vero al contrario, perché l'emittente sceglie il codice.

Severità `medium` e non `high` per la stessa ragione dichiarata in [DEBT-024]: richiede il ruolo di validatore, e il livello compute è M-06, quindi nessuna riga lo implementa oggi. Resta `medium` e non `low` perché è nel **filo principale**, in una sezione intitolata *What does your device actually do?*, ed è una frase che un lettore ripete.

**Rimedio suggerito.** Qualificare per genere: la coppia e la casualità sono derivate per tutte e tre le sfide; *quale* calcolo è chiesto è scelto da chi lo chiede, ed è un debito aperto e nominato. La guida ha già il registro giusto per dirlo — il §05 apribile dice *«qualifying is expensive to fake, not impossible to fake»*, che è la stessa onestà.

### RF-006 — `medium` — «revocation removes a key from participation» è più largo di quanto la regola dica, ed è più largo esattamente dove [DEBT-022] è aperto

§06, apribile:

> *revocation **removes a key from participation**; it does not remove what that key did.*

La seconda metà è vera e ben detta. La prima è più larga della regola. [DEBT-022] è aperto proprio su questo: la regola di autorizzazione del burn di abbonamento a `docs/protocol/ledger.md:347` richiede soltanto che la chiave derivi `payer_node_id`, mentre le tre regole sorelle dello stesso documento — righe 312, 398, 871 — dicono tutte *enrolled, unrevoked*. Una chiave revocata perché compromessa può quindi apparentemente ancora autorizzare addebiti sul saldo del nodo.

Va detto in credito ad AGENT-006, e [DEBT-022] lo registra: **è AGENT-006 che ha trovato questo debito scrivendo la guida**, e di conseguenza la guida *«non afferma che la revoca fermi la spesa»*. La disciplina ha funzionato dove il debito è stato trovato. Ciò che è rimasto indietro è la frase generale del §06, che pretende di dire cosa la revoca **fa** invece di cosa non fa, e nel farlo copre anche il caso che il suo stesso autore aveva scoperto non essere coperto.

Severità `medium` e non `high` perché è dentro un apribile, quindi non è la forma che il lettore incontra per forza, e perché il §03 `.plainly` — che è nel filo — è formulato correttamente.

**Rimedio suggerito.** *«revocation removes a key from taking part in consensus and from enrolling again»*, che è ciò che le regole dicono, invece di *participation* senza qualificazione. Oppure nominare l'eccezione, che sarebbe la forma migliore e che [DEBT-022] renderebbe facile.

### RF-007 — `low` — La quarta cosa scomoda arriva una sezione dopo la prova che [DEBT-023] pone

Vedi la Domanda 3 sopra per la verifica meccanica. Il criterio 2 è **soddisfatto**: tutte e quattro le affermazioni scomode sono nel filo, nessuna richiede di aprire nulla.

Il residuo è di collocazione e non di presenza. [DEBT-023] formula la prova come *«un lettore che si convince nelle prime tre sezioni e non apre nulla»*. Al termine del §03 quel lettore ha incontrato Sybil, pseudonimo debole e saldo pubblico, ma **non** la permanenza pubblica degli abbonamenti, che è la sola delle quattro su cui compirà un atto irreversibile e la sola che la pagina stessa dichiara dover arrivare prima dell'atto. Sta nel §04, cioè prima che il lettore sappia che gli abbonamenti esistono, quindi non è tardi in senso funzionale.

`low` e non `medium` perché nessuna lettura ragionevole produce un danno, e perché il `README.md` della guida dichiara §04 come collocazione **scelta**. Lo registro perché il debito pone la prova a tre sezioni e la pagina la supera a quattro, e la differenza va decisa dal Lead invece che assorbita in silenzio.

**Rimedio suggerito, se il Lead lo vuole:** una subordinata nel §03 `.plainly`, dove il saldo pubblico è già detto — *«e così sarà per ogni abbonamento che pagherai»* — che costa sei parole e sposta la quarta dentro le tre sezioni.

### RF-008 — `low` — La pagina non dice la cosa migliore che il protocollo abbia da dire al suo lettore (il verso opposto dell'invecchiamento)

Questo finding è l'unico che non toglie: aggiunge. È la ragione per cui il rinvio di [DEBT-023] valeva la pena.

§07, apribile, elenca ciò su cui un cliente leggero **fallisce chiuso**: checkpoint mancante, troppo vecchio, di catena sbagliata, firmato da chiave sconosciuta, e il rifiuto di tornare indietro. L'elenco è corretto e si ferma **un passo prima** di quello nuovo. `docs/protocol/ledger.md:2495` porta il passo **4b**, arrivato con [SPEC-017]:

> *«**Measure the real cadence.** […] The client MUST fail closed when the chain is faster than the band, and MUST report — not reject — when it is slower.»*

Cioè: un telefono ordinario, con una banda a due lati fissata alla genesi e che non può essere appresa da un peer, **misura se la catena sta correndo troppo veloce e si ferma**. È l'unica difesa di questo protocollo che il lettore della guida possa esercitare da solo, sul proprio dispositivo, senza fidarsi di nessuno. La pagina non la nomina.

Va notato che l'asimmetria è essa stessa un'informazione onesta e degna di questa pagina — chiuso sul veloce, riportato sul lento, perché *«nothing honest makes blocks appear»* mentre un lettura lenta è indistinguibile dal proprio ritardo di sincronizzazione. È esattamente il registro in cui la guida è già scritta.

Osservazione senza severità, nello stesso verso: il §07 apribile dice già *«from the wrong chain»*, che **oggi** è vero in modo più pieno di quando è stato scritto, perché [SPEC-021] ha chiuso il legame di catena alla genesi. La frase è invecchiata bene senza che nessuno la toccasse. Vale la pena registrarlo perché è il contro-esempio: non tutto ciò che è cambiato ha reso la pagina peggiore.

**Rimedio suggerito.** Due frasi nel §07 apribile, dopo il fallimento chiuso: il dispositivo confronta quanti blocchi sono passati con quanto tempo è passato secondo un orologio che nessun validatore scrive, e si ferma se la catena sta correndo troppo. Con la sua probe verso `ledger.md:2495`. Chiude anche metà di RF-002, perché è il posto in cui la pagina può dire cosa succede se qualcuno comprime la cadenza.

### RF-009 — `medium` — Il colophon promette al lettore un meccanismo più largo di quello che esiste, ed è la promessa che gli fa abbassare la guardia

Colophon, filo principale:

> ***Every sentence** on this page that says the system guarantees something is tied to the rule in the protocol specification that holds it, by an automated check that runs on every change.*

È un superlativo universale, nel senso della regola 2 di [[lead-claims-discipline]], rivolto al pubblico. Ciò che esiste sono **65 probe**, ciascuna ancorata a una **sottostringa scelta a mano** della frase che difende. RF-001, RF-002 e RF-004 sono tre frasi di garanzia in cui la clausola portante non è dentro la sottostringa e non è ancorata a nulla. La promessa è quindi più forte del meccanismo in modo dimostrabile, e la dimostrazione sono i tre `high` di questa review.

Aggrava che sia la frase che chiede al lettore di fidarsi delle altre. Un lettore che legge il colophon smette di chiedersi se una frase sia sostenuta, perché gli è stato detto che il controllo è automatico ed esaustivo. È famiglia 2 nella sua forma più costosa: la pretesa non solo è avanti alla regola, ma **disattiva la verifica del lettore**.

Va detto che il meccanismo è reale, funziona, ed è più di quanto quasi nessun progetto scriva. Il difetto è nella parola *every*.

**Riproduzione.**
```text
$ grep -c 'id = "guide-' sim/tools/published_artifacts.toml
65
$ grep -n "Every sentence on this page" .lmbrain/design/coblox-public-guide/index.html
630:          Every sentence on this page that says the system <em>guarantees</em> something is tied to
```
E i tre casi della tabella della Domanda 2 sono il controesempio, tutti e tre in albero.

**Rimedio suggerito.** *«Sixty-five statements of property on this page are tied to the rule that holds them…»*, con il numero, che è più impressionante del superlativo e ha il pregio di essere vero. Oppure `most`. In nessun caso `every`, che è un'affermazione universale non enumerata.

## Ciò che ho attaccato senza riuscire a romperlo

La regola è in [[lead-claims-discipline]] e in [SKILL-001], e il corollario dice che ciò che sembra migliore va attaccato per primo. Riporto gli attacchi che **non** hanno prodotto un finding, perché sapere dove ho guardato è informazione quanto sapere cosa ho trovato.

**Il criterio 2 in senso meccanico, cioè l'ipotesi che una delle tre cose scomode fosse dentro un apribile.** È l'ipotesi che il debito considera più probabile e l'ho attaccata per prima, sui file e non leggendo: nessuno degli otto `<details>` porta `open`; nessun `.plainly` è annidato in un `<details>`; `guide.css` non nasconde `.plainly` in alcuna regola. Le quattro affermazioni sono nel filo, in §01, §03 e §04. **Non si è rotto**, e la pagina fa qui la cosa che dichiara di fare.

**«Nobody can take credits away from you», cercandovi il canale di confisca.** È la frase che più invita a essere rotta. Ho attaccato per tre vie: la revoca (non c'è transazione che confischi, e l'elenco dei generi è chiuso e probato); la commissione (non esiste, ed è probato); il conto di finanziamento dell'app, che è il canale reale per cui i tuoi credits escono senza che tu firmi di nuovo. **La pagina lo dichiara**, nel §03 secondo apribile, in prima persona e senza attenuazione: *«credits you have moved into an app's funding account are no longer yours to hold»*. Restava [DEBT-022], cioè la chiave revocata che autorizza ancora un burn: **la guida non afferma che la revoca fermi la spesa**, e [DEBT-022] registra che è AGENT-006 a essere inciampata nel debito scrivendo la pagina. La frase regge in tutte e quattro le letture. **Non si è rotto**, e ha retto meglio di quanto mi aspettassi. Ciò che si è rotto è la frase *generale* sulla revoca in §06, che è RF-006 e sta altrove.

**«There is no transaction that takes credits from one person and gives them to another», cercandovi un trasferimento travestito.** Il conto di finanziamento è la via ovvia: ci metti credits e qualcun altro viene pagato. Non è un trasferimento — i validatori spendono l'escrow sull'hosting a un prezzo che il publisher non fissa, e chi ospita è pagato dal protocollo contro prova, non dal tuo saldo. L'abbonamento è la seconda via: brucia, non trasferisce, e il diagramma disegna la terza freccia **barrata**, che è la disposizione più forte e non la più decorativa. Il cap anti-auto-abbonamento chiude il ciclo. **Non si è rotto.**

**La separazione fra chiave di trasporto e chiave d'identità in §06, cercandovi la sovra-promessa.** È il punto in cui una pagina di trasparenza tipicamente esagera, e questa è la sezione che sembra migliore, quindi l'ho attaccata per prima fra le sezioni. La pagina dice che il registro non pubblica il legame **e poi lo smentisce da sola** nella frase successiva: *«every node you actually talk to sees the address you are connecting from, and your messages carry your node identifier in the clear. The register does not publish the link. Participating exposes it.»* Il secondo riquadro del diagramma 2 — *«Not written down, but seen»* — porta la stessa distinzione in forma visiva, e la didascalia dice che è quella che le persone sbagliano. È il passaggio migliore della pagina e non ho trovato da che parte prenderlo. **Non si è rotto.**

**Gli strumenti, cercandovi la guardia che ha smesso di guardare.** Ho eseguito entrambi. `published_artifacts.py` passa con 126 candidati C10, `check-guide-pairs.mjs` passa con 65 `claims` ancora presenti nella pagina e le altre cinque classi verdi. Nessuna probe è appesa a una regola sparita, nessuna `claims` è appesa a una frase sparita, in nessuno dei due versi. La cerniera funziona. **Non si è rotto**, ed è per questo che RF-001, RF-002 e RF-004 sono `high`: se la macchina fosse rotta il rimedio sarebbe ripararla, e invece il rimedio è scegliere meglio cosa ancorare, che nessuno strumento può fare al posto di una persona.

**Il §05 sulla soglia di ammissibilità, cercandovi il ritorno del mining sotto altro nome.** *«Above the threshold, more storage buys nothing: no seat, no better odds, no extra weight. Every validator counts as exactly one.»* Ho cercato un canale in cui spendere di più ricompri qualcosa: il potere di voto è uniforme e probato; l'uptime non conta e ha la sua probe; l'ammissibilità è un test sì-o-no. E l'apribile dichiara da sé il residuo — *«qualifying is expensive to fake, not impossible to fake»* — con la sua misura, e dice esplicitamente che scrivere *«cannot be faked without spending real resources»* rivendicherebbe più di quanto le regole diano. C'è persino una probe che si chiama `guide-overstated-safety-claim-refused`. **Non si è rotto**, ed è il paragrafo che meglio esemplifica la regola di forma di questa pagina.

**Il §07 sul rifiuto parziale, cercandovi il blocco a metà applicato.** *«A block that contains a single entry which does not execute is rejected whole: there are no half-applied blocks.»* È la classe di affermazione che di solito ha un'eccezione da qualche parte. Ancorata da `guide-no-partially-applied-blocks`, e non ho trovato una lettura in cui l'eccezione esista. **Non si è rotto.**

**Il modello delle capacità in §04, cercandovi il default permissivo.** *«Anything not asked for is denied, and asking is not the same as getting.»* Due probe distinte, una per l'assenza-come-diniego e una per l'assenza di capacità di default, che è la coppia giusta perché sono due affermazioni e non una. **Non si è rotto.**

**Il §04 apribile sul documento di trasparenza che non esiste ancora.** Sospettavo la reticenza: un rimando a un documento futuro è il modo classico di rinviare una dichiarazione scomoda. La pagina fa l'opposto — dichiara che il documento **non esiste**, dice che fino ad allora la pagina e `SECURITY.md` sono tutto ciò che è stato detto, e scrive che il paragrafo è lì *«so that the gap is visible rather than merely unmentioned»*. È la disposizione onesta e la meno comoda. **Non si è rotto.**

## Required follow-up

**Bloccanti per la pubblicazione, e sono tre paragrafi:** RF-001, RF-002, RF-003. Tutti e tre hanno la stessa radice — la pagina non ha una nozione di tempo e non ha una seconda soglia — e tutti e tre si chiudono aggiungendo ciò che `SECURITY.md` già dice, invece di inventare. Ciascuno va accompagnato da una probe nuova o da una `claims` estesa: chiuderli senza estendere l'ancoraggio lascerebbe in piedi la ragione per cui non erano stati colti.

**Dovuti nello stesso giro, non bloccanti:** RF-004, RF-005, RF-006, RF-009. RF-009 costa una parola.

**Raccomandati e non dovuti:** RF-007 e RF-008. RF-008 è il solo finding di questa review che rende la pagina **migliore** invece che meno sbagliata, e se il Lead ne prende uno solo fra i non dovuti raccomando quello.

**Una cosa che non è dell'implementatrice ed è del Lead.** La forma di RF-001/RF-002/RF-004 — la `claims` che si ferma prima della clausola che porta il rischio — è una **classe**, non tre casi. Le altre 62 probe non le ho verificate una per una in questo verso, e non affermo che siano a posto: affermo di aver trovato tre casi guardando le tre affermazioni più forti della pagina, che è un campione scelto male apposta. Vale la pena una passata su tutte e 65 con una domanda sola: *la `claims` copre la clausola che porta il peso, o quella accanto?* È lavoro meccanico e non ha bisogno di me.

## Final decision

**Changes requested. Tre `high`, quattro `medium`, due `low`.**

**Sulla pubblicazione: pubblicabile dopo le correzioni di RF-001, RF-002 e RF-003, non prima.** Non sono correzioni di prudenza: sono tre frasi che il lettore ripeterà e che il protocollo di oggi nega per iscritto. Nessuna delle tre richiede di rifare la pagina, il diagramma, o la struttura; tutte e tre si chiudono aggiungendo al filo principale ciò che `SECURITY.md` ha già deciso di dire.

Il contributo che vale oltre questa review è **la conferma che il rinvio era corretto e che la sua condizione era ben scelta.** [DEBT-023] non è stato un modo di rimandare un obbligo: è stato l'obbligo attaccato all'evento giusto. Se questa review fosse stata scritta il 25 agosto, `whatever anybody intends` sarebbe passata — perché la cadenza non era ancora misurata e la manovra non era ancora nominata — e sarebbe passata **firmata**. La deroga di una gate con la condizione di sblocco al posto della data è il meccanismo che ha prodotto questo risultato, ed è la seconda volta dopo [DEBT-001] che funziona.

Il secondo contributo è meno gradevole e va registrato lo stesso: **l'ancoraggio meccanico ha creato un punto cieco esattamente dove si trovavano i finding.** Le 65 probe passano tutte, in tutti e due i versi, e i tre `high` sono tutti nella parte di frase che la probe non copre. La guardia ha protetto la metà modesta di ogni affermazione e ha lasciato scoperta la metà che eccede, e non poteva fare altrimenti, perché la metà che eccede non è ancorabile **proprio in quanto eccede**. È la quinta volta su questo progetto che il difetto era già scritto e non guardato, e stavolta era scritto accanto a uno strumento che passava.
