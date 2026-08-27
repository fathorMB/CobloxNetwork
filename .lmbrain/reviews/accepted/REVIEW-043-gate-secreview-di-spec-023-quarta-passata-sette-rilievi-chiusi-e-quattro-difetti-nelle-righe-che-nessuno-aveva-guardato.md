---
id: REVIEW-043
# Note: Quote the title if it contains a colon
title: "GATE-SECREVIEW di SPEC-023, quarta passata: sette rilievi chiusi, e quattro difetti nelle righe che nessuno aveva guardato"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-023
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, correctness, documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-043-EVENT-001"
    timestamp: "2026-08-27T10:44:02.713063500+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata su richiesta esplicita dell'operatore del 2026-08-27. I sette rilievi di REVIEW-041 sono chiusi e verificati nei tre luoghi — corpo, cella, riquadro — e i tre criteri di accettazione sull'analisi sono soddisfatti. I sei rilievi nuovi restano come note NON bloccanti: nessuno blocca un criterio di accettazione, una gate dichiarata o QUALITY.md, e la 5.1.0 vieta di aprire un giro di remediation per rilievi minori. Questa spec ne ha gia' bruciati tre.\n\nVerifica indipendente del Lead sui due rilievi con conseguenze fuori dalla review: RF-004 regge — min_revocation_effective_delay_blocks_max e' in ElectionBounds ed e' imposto in check_magnitudes, core/coblox-core/src/params.rs:589 — quindi \"mantiene all'infinito\" sulla riga 10 e' falso e la classe di DEBT-036 e' di nove parametri e non di dieci; il precedente di RewardBounds citato in RF-003 regge — docs/protocol/README.md:1246 dichiara invalido esattamente il collasso 86 400 000 -> 86 400. Entrambe le verifiche hanno richiesto due tentativi: la prima ricerca del Lead ha fallito per proprio difetto di strumento, non per un'affermazione falsa dell'agente.\n\nResiduo dichiarato e accettato consapevolmente: le righe 2, 4, 5 e 6 non hanno mai avuto un attacco nel merito in quattro passate. L'operatore ha scelto di accettare e di registrare il buco su un debito proprio invece di lasciarlo dentro questa review, cosi' che sia attivo alla stesura dell'ADR."
    evidence_refs: ["SPEC-023", "REVIEW-041", "REVIEW-040", "REVIEW-038", "DEBT-036", "DEBT-041"]
    implementation_agent: "AGENT-002"
links: [DEBT-036, DEBT-038]
created: 2026-08-27
updated: 2026-08-27
tags: [review, security, governance]
related_decisions: [ADR-013, ADR-016, ADR-017]
activity:
  - date: 2026-08-27
    action: "transitioned pending -> accepted"
---
# Review

> **Quarta passata di `GATE-SECREVIEW` su [SPEC-023]**, sull'oggetto consegnato da AGENT-002 al commit `78da559`. L'oggetto della gate è **l'analisi**, non lo strumento: `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md`.
>
> **Chi scrive non verifica.** Le passate 2 e 3 le aveva rimediate il Lead e le aveva riviste questa reviewer; questa passata l'ha scritta AGENT-002 e la rivede questa reviewer. La separazione regge per la prima volta dall'inizio della catena.

## Outcome

**`GATE-SECREVIEW` soddisfatta, con sei note non bloccanti.** Raccomando **`review_accept`**.

**I sette rilievi di [REVIEW-041] sono chiusi, e chiusi bene.** Ho verificato in proprio i due `high`. La regola meccanica dei tre luoghi — corpo, cella, riquadro — è stata applicata e tiene su tutti e sette. Le due conclusioni che l'applicazione della regola sulle ancore ha ribaltato (mediana degli undici alla riga 1; `sender_node_id` e il divisore `k` alle righe 5–6) sono **sostanzialmente corrette** e ricostruite sulle frasi giuste dei documenti giusti.

**Ma i tre bersagli dichiarati hanno ceduto tutti e tre**, e sono le righe che nessuna delle quattro passate aveva guardato nel merito. Nessuno dei quattro difetti che ho trovato blocca un criterio di accettazione, una gate dichiarata o [[QUALITY]]: sono **correzioni da fare prima della conversione in ADR**, non un quinto giro di remediation. La spec ha già bruciato tre giri, e [[CONTRACT]] dice che dopo due giri sullo stesso rilievo si escala all'operatore invece di ripetere. **Questi non sono lo stesso rilievo**: sono righe nuove, e il modo giusto di chiuderle è dentro l'ADR, dove l'operatore le legge una volta sola.

**Il difetto peggiore che ho trovato è il più economico da correggere**: un numero. Il residuo della riga 7 non è «al più 2»: è **3** sulla finestra minima misurabile, e **non è limitato affatto** sotto di essa.

## Cosa ho ESEGUITO e cosa ho solo LETTO

**Eseguito** (tutto in sola lettura sull'albero al commit `78da559`):

1. **Risolutore di citazioni con normalizzazione degli spazi e spogliatura dell'enfasi**, su **18 frasi portanti** dei quattro documenti di protocollo — quelle su cui poggia ciascuno dei miei rilievi, non quelle dell'analisi. **18 su 18 risolvono, zero non risolte.** Tre delle diciotto attraversano un a capo nel sorgente (`Suspension becomes effective only after ...`, `Governance MUST therefore choose ...`, `the chain is faster than the band when ...`) e un confronto a riga singola le avrebbe mancate: è la ragione per cui il metodo lo chiede.
2. **Enumerazioni mirate** con `grep` su `docs/`, `core/` e `sim/`: `billing_epoch_ms` (**due** occorrenze in `docs/protocol/`, come l'analisi dichiara), `app_suspension_notice_epochs`, `HostingRateCard`, `min_revocation_effective_delay_blocks`, `min_activation_gap`, `activation_height`.
3. **Lettura integrale, non campionaria, di cinque sezioni sorgente**: `README.md` §*"Cadence band"* e §*"The genesis band"*; `README.md` §*"Signed protocol documents"* (i quattro corpi per intero); `README.md` `RewardBounds` §*"Rate of change ratio"* e §*"Minimum activation gap"* con la tabella di conformità; `identity.md` §*"Mandatory rejection rules"* e §*"Bounded validity in time"* punti 1–5; `ledger.md` §*"Revocation forces a validator set transition"* e §*"Light-client balance verification"* passi 1–10 compreso il **4b** per intero.
4. **Aritmetica sul predicato del passo 4b**, sui valori della banda di genesi, riprodotta sotto in RF-001.
5. **Enumerazione di `check_relations` e `check_magnitudes`** in `core/coblox-core/src/params.rs` per i campi citati.

**Solo letto, e non eseguito:**

- **Non ho rieseguito alcuna gate di progetto.** Non ho eseguito `published_artifacts.py`, `consensus_parameters_closure.py`, la sua prova in negativo, `lead_claims_check.py`, né alcuna suite `cargo`. Il Lead le dichiara eseguite e `PASS`; **non le contesto e non le ho verificate.** La mia raccomandazione poggia su quelle esecuzioni per tutto ciò che riguarda lo strumento e la lista DRAFT.
- **Non ho rieseguito il risolutore sulle 132 citazioni dell'analisi.** Ho verificato le mie diciotto.
- **`threat-model.md` non l'ho letto**, e **TM-37 non è riverificato** — quarta passata consecutiva in cui resta non guardato, ed è la fonte che la riga 3 cita per il proprio danno massimo.
- **[ADR-010], [ADR-013], [ADR-015], [ADR-016], [ADR-017] non riletti per intero.** Le conseguenze che cito le ho verificate sui documenti di protocollo e su `params.rs`.
- **Nessuno scenario eseguito.** Come nella passata precedente, non esiste in questo repository un simulatore di consenso che li produca. Tutto ciò che chiamo scenario è derivato da regole lette e da aritmetica su costanti dichiarate.
- **Se `election_entropy_blocks` sia implementato in codice: non verificato.** Terza passata consecutiva.

## Acceptance-criteria compliance

I criteri che questa gate tocca sono i tre sull'analisi. Tutti e tre **soddisfatti**:

- **dieci parametri su dieci, quattro domande ciascuno, fonti dichiarate** — verificato riga per riga. La quinta domanda aggiunta dall'analisi (pavimento e liveness) è oltre il richiesto e non è un difetto.
- **distinzione esplicita fra relazionale e di magnitudine, nessun tetto uniforme** — verificato. Sei righe portano una forma mista o relazionale, e la riga 5–6 dichiara che **nessuna grandezza esistente serve** invece di inventarne una. È il risultato che la spec chiedeva di scrivere invece di riempire con un numero.
- **nessun valore di lancio fissato, nessun limite aggiunto al blocco dei vincoli** — verificato: `git status --porcelain` a un solo file, e nessun documento di protocollo toccato.

I quattro difetti sotto stanno **dentro** risposte esistenti, non al posto di risposte mancanti. È la ragione per cui non bloccano.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security-boundary | severity=medium | criterion=riga 7, punto 4, e riflesso sulla riga 10 | remediation=sostituire il fattore 2 con il predicato reale del passo 4b, e dichiarare che sotto `min_measured_blocks` la misura NON è fatta

**Il residuo della riga 7 è dichiarato, ed è dichiarato più piccolo di quanto sia. Ed è dichiarato «visibile» dove invece è imposto, e «limitato» dove invece non lo è.**

L'analisi scrive: *«il fattore di errore è limitato dalla banda di cadenza (al più 2 sul lato veloce), e la banda è un'osservazione e non un predicato, quindi il fattore non è imposto ma è visibile»*. **Due metà, e sono sbagliate in due direzioni opposte.**

*Sulla prima metà.* La soglia oltre cui il lato veloce diventa visibile **non è `min_ms_per_block`**: è `min_ms_per_block` **meno la tolleranza**, ammortizzata sul numero di blocchi misurati. `ledger.md`, §*"Light-client balance verification"*, passo 4b, dichiara il predicato esatto: la catena è più veloce della banda quando `elapsed_ms + max_external_clock_slack_ms < blocks * min_ms_per_block`. Con i valori della banda di genesi (`README.md`, §*"The genesis band"*: `min_ms_per_block` `2500`, `max_external_clock_slack_ms` `600000`, `min_measured_blocks` `720`), sulla **finestra minima misurabile** l'allarme scatta solo sotto `720 × 2500 − 600 000 = 1 200 000` ms, cioè sotto **1 667 ms per blocco**. Contro i `5000` ms della costante su cui il predicato di accettazione confronta, il fattore d'errore silenzioso è **3, non 2**. Tende a 2 solo asintoticamente, al crescere di `blocks`, quando la tolleranza si ammortizza.

*Sulla seconda metà, ed è la parte che nessuna aritmetica ripara.* Lo stesso passo dichiara: *«When `blocks < min_measured_blocks` the measurement is **not made**, and that is reported as its own outcome and never as a pass»*. **Sotto i 720 blocchi non c'è alcun fattore limite, perché non c'è alcuna misura.** Un client appena avviato su un checkpoint fresco sta esattamente in quel regime. Non è il regime peggiore per il danno (a) — un checkpoint fresco ha una finestra di esposizione piccola — ed è la ragione per cui questo rilievo è `medium` e non `high`; ma la frase «il fattore è limitato dalla banda» è **falsa senza qualificazione** e va qualificata.

*E in senso opposto, l'analisi si sottovaluta.* Il lato veloce **non** è soltanto «visibile»: `ledger.md`, passo 4b, dichiara *«The client MUST fail closed when the chain is faster than the band, and MUST report — not reject — when it is slower»*. Sul lato veloce — l'unico che questa riga usa — il light client **fallisce chiuso**, e il light client è **la parte che questa riga esiste per proteggere**. La differenza di specie fra i due lati è portante e l'analisi l'ha già scritta bene alla riga 1 per il lato **lento**; alla riga 7 l'ha trasportata sul lato **veloce**, dove non vale. È lo stesso trasporto indebito che [REVIEW-041] RF-001 censurava, applicato all'altra estremità.

**Scenario d'attacco.** Un quorum che detiene le chiavi di consenso accelera la produzione a `1 700` ms per blocco — sopra la soglia di fallimento chiuso del 4b, quindi **nessun light client rifiuta**. La finestra reale di successione, `min_revocation_effective_delay_blocks × 1 700` ms, si riduce a poco meno di **un terzo** di quella che il predicato di accettazione assume (`F × 5 000`). Un client accetta allora un checkpoint vecchio fino a tre volte la finestra reale in cui i superstiti devono committare un set successore conforme — che è esattamente ciò che il MUST di `ledger.md` §*"Revocation forces a validator set transition"* esiste per vietare: *«so that a checkpoint a client still accepts is never older than the window granted to commit a compliant successor set»*. Su una finestra sotto i 720 blocchi il fattore non ha limite dichiarato. **Il costo dell'attaccante è però reale e va scritto accanto**: `README.md` §*"Cadence band"* dichiara che *«Speeding it up requires a **quorum**»* — non un terzo bloccante — quindi il lato veloce costa più del lato lento della riga 1.

**Riflesso sulla riga 10, e qui il residuo non c'è affatto in due dei tre luoghi.** La §10 punto 3 rinvia correttamente alla §7 punto 4 (*«vale per questa riga negli stessi termini»*). Ma la §10 **punto 4** — la domanda che conta — e la **cella 10** della tabella portano soltanto *«Banda a due lati + Relazionale (con $G$, $P$ e WS)»*, senza alcun residuo. La regola dei tre luoghi, che questa passata ha applicato ai sette rilievi, **non è stata applicata al difetto che la passata ha trovato da sé**. E la riga 10 è quella che [REVIEW-041] dichiara *«utilizzabile senza riserve»*: chi compila l'ADR riga per riga dalla tabella non trova alcuna riserva da scrivere.

**Condizione di chiusura verificabile.** La §7 punto 4 e la §10 punto 4 e la cella 10 riportano tutte e tre: (i) il predicato del 4b citato per esteso invece del fattore 2; (ii) la constatazione che sotto `min_measured_blocks` la misura **non è fatta**, quindi il fattore non è limitato; (iii) il fatto che sul lato veloce il client **fallisce chiuso**, non riporta; (iv) il costo dichiarato dell'attaccante (quorum, non terzo bloccante). Chiuso quando le tre sedi concordano e le quattro frasi risolvono contro `ledger.md` passo 4b e `README.md` §*"Cadence band"*.

RF-002 | category=correctness | severity=medium | criterion=riga 3, punto 3 | remediation=allineare il punto 3 al punto 4 e alla propria fonte, e invertire l'ordine di rischio fra i due riceventi

**La riga 3 si contraddice al proprio interno, e la metà sbagliata è quella rassicurante.** È una delle due righe che l'evidence dichiara *«non riesaminate nel merito»* da nessuna delle quattro passate.

Il punto 3 chiude così: *«Il terzo termine vale `max_weak_subjectivity_age_ms` per un ricevente che possiede un checkpoint, e **nulla** per uno che non ne possiede.»*

**Tre difetti sovrapposti, tutti verificati contro la fonte che la riga stessa cita.**

- **La fonte nega esplicitamente il bound che la frase afferma.** `identity.md`, §*"Bounded validity in time"*, punto 2: *«the residue is then the checkpoint's own age, which no rule of this protocol bounds either»*. L'analisi scrive un valore dove il documento scrive che **non ce n'è uno**. La derivazione implicita — un checkpoint verificato ha età al più `max_weak_subjectivity_age_ms` per il passo 1 del light client — è **plausibile**, ma è una **deduzione**, e vive su un percorso diverso da quello del trasporto. La riga 8 di questa stessa analisi ha imparato a etichettare le proprie deduzioni ([REVIEW-040] NF-09) e lo fa bene; **la riga 3 non l'ha imparato**, ed è la stessa forma di difetto un documento più in là.
- **La seconda metà inverte l'ordine del rischio.** «Vale nulla» per un ricevente senza checkpoint è il contrario di ciò che la fonte dichiara: `identity.md` punto 2 dice che il punto 5 *«reduces that offset for a receiver holding a checkpoint and leaves it untouched for one that is not»*, e il punto 3 lo ripete: il pavimento sotto `now_ms` *«reduces it without bounding it, and leaves it exactly as it was for those that do not»*. Il ricevente **senza** checkpoint è quello per cui il terzo termine è **massimo e non ridotto** — ed è, per la ragione 4 dello stesso documento, il nodo *«freshly installed, long offline»*, cioè quello **il cui orologio è meno affidabile**. La frase dell'analisi lo presenta come il caso senza esposizione.
- **Il punto 4 e la cella dicono la cosa giusta.** Il punto 4 scrive *«il terzo termine non ha oggi alcun vincolo»*; la cella 3 scrive *«(terzo termine oggi non limitato)»*. **La divergenza è dentro la sezione**, quindi la regola che la tabella dichiara — *«dove le due divergessero, vale la sezione»* — non risolve nulla qui: la sezione litiga con sé stessa.

**Scenario d'attacco.** Un compilatore d'ADR che legge il punto 3 conclude che la finestra di esposizione TM-37 è chiusa da un tetto su $D_{\max}+S_{\max}$ per i riceventi con checkpoint, e che i riceventi senza checkpoint non hanno terzo termine. Scrive quindi un limite di magnitudine sulla somma e nessun rimedio per il terzo termine. Un detentore di chiave di trasporto sottratta presenta l'attestazione a un nodo appena installato, il cui orologio è indietro di un tempo che nessuna regola limita: l'attestazione è accettata **oltre** la somma dei due parametri, per un tempo pari all'errore d'orologio del ricevente. Il nodo occupa il posto della vittima nelle connessioni dirette e lascia scadere le challenge in evidenza `failed` o `late` a spese economiche della vittima — il danno che `identity.md` punto 2 descrive per esteso.

**Condizione di chiusura verificabile.** Il punto 3 dice ciò che dicono il punto 4, la cella 3 e `identity.md`: il terzo termine **non è limitato da alcuna regola**; il pavimento del punto 5 lo **riduce senza limitarlo** per chi ha un checkpoint e **lo lascia intatto** per chi non ne ha; e se la riduzione all'età del checkpoint viene mantenuta, è marcata **deduzione di questa analisi** come la riga 8 marca la propria. Chiuso quando le tre sedi concordano.

RF-003 | category=security-boundary | severity=medium | criterion=riga 9, punto 4 | remediation=contare il superlativo «comunque si muovano i due fattori», e nominare il contro-operando su cui il predicato va valutato

**La riga 9 porta un assoluto che non regge, ed è la riga che AGENT-002 ha allineato di propria iniziativa: nessuno l'ha chiesta e nessuno l'ha rivista.**

Il punto 4 chiude così: *«La banda sul prodotto limita la finestra wall-clock **comunque si muovano i due fattori**.»* La spec istruisce esplicitamente la specialista a *«contare ogni superlativo assoluto invece di scriverlo»*. Questo non è contato, e non regge.

**La riga costruisce correttamente il proprio scenario e poi lo chiude a metà.** Il punto 4 vede il difetto giusto — legare il predicato alla sola accettazione di `consensus_parameters` lo farebbe eludere pubblicando `hosting_rate_card` **dopo** — e prescrive giustamente la valutazione **all'accettazione di entrambe le specie**. Ma non dice **contro quale copia** dell'altro operando il predicato si valuta, e **è lì che l'assoluto cade**.

**Scenario d'attacco.** Sia la banda sul prodotto $[L, H]$, con $r = H/L$. Stato iniziale: `app_suspension_notice_epochs` $= N_0$, `billing_epoch_ms` $= B_0$, prodotto in banda. Il quorum pubblica **due documenti di specie diversa prima che l'uno o l'altro attivi**: `README.md` §*"Signed protocol documents"* dichiara *«Sequence is strictly increasing per kind; activation cannot be retroactive»* — la sequenza è **per specie**, e i due `activation_height` sono indipendenti. Se ciascun documento è valutato contro il contro-operando **attivo** al momento della propria accettazione, allora:

- il nuovo `consensus_parameters` porta $N_1$ con $N_1 \times B_0 \in [L,H]$ — **accettato**;
- il nuovo `hosting_rate_card` porta $B_1$ con $N_0 \times B_1 \in [L,H]$ — **accettato**.

Quando entrambi attivano, il prodotto reale è $N_1 \times B_1$, che può scendere fino a $L^2 / (N_0 B_0)$, cioè **fino a un fattore $r$ sotto il pavimento della banda**. La finestra di rimedio wall-clock dello sviluppatore onesto collassa di quel fattore **senza che alcuna accettazione abbia violato la banda**. È la stessa forma del difetto che la riga già nomina — misurare contro l'insieme dichiarato invece di quello osservato — spostata di un passo.

**È un residuo di grado, non una confutazione della banda sul prodotto**, ed è limitato dalla larghezza $r$ della banda: va scritto accanto ad essa, non al posto suo.

**E la riga manca un credito che rafforza il proprio rimedio.** Alla domanda 3 l'analisi risponde *«Nessun vincolo di genesi»*, il che è esatto per `HostingRateCardBody` — ho verificato che nessuno dei tre oggetti di ancoraggio (`ElectionBounds`, `RewardBounds`, `CadenceBand`) lo governa. Ma **il protocollo porta già il rimedio, per un'altra specie di documento**: `RewardBounds` impone a `reward_policy` un **rapporto di variazione** fra sequenze consecutive e un **`reward_parameter_min_activation_gap_blocks`** (`README.md`, §*"Minimum activation gap"*: *«Requires a minimum spacing in chain height between activations of consecutive `reward_policy` documents»*). E la tabella di conformità della stessa sezione nomina **esattamente questo attacco**, su `reward_epoch_ms`: `86 400 000 -> 86 400` in un documento è **invalido**, *«rate of change exceeded by a factor of 1000»*. **L'analisi propone per `billing_epoch_ms` uno scenario che il protocollo ha già dichiarato invalido per il campo gemello di un'altra specie, e non lo dice.** Il rapporto di variazione ha inoltre il pregio che la banda sul prodotto non ha: rende la modifica un **processo osservabile** invece di un salto.

**Condizione di chiusura verificabile.** Il punto 4 e la cella 9 riportano: (i) il superlativo contato — il predicato sul prodotto va valutato contro l'operando **della sequenza più recente per specie, attiva o già accettata e non ancora attiva**, non contro quello attivo, e il residuo se non lo si fa è il fattore $r$; (ii) il precedente di `RewardBounds` — rapporto di variazione **più** gap di attivazione — nominato alla domanda 3 come la forma che l'ADR può riusare. Chiuso quando corpo e cella concordano e le due frasi di `README.md` risolvono.

RF-004 | category=correctness | severity=low | criterion=riga 10, punti 2 e 3 | remediation=contare l'«infinito», e portare alla domanda 3 il tetto di genesi che esiste già ed è già imposto

**La sola riga dichiarata intatta per tre passate porta un assoluto falso, e la sua smentita sta nella stessa analisi, una riga più in su.**

La §10 punto 2 scrive: *«Portato al massimo: Mantiene **all'infinito** un validatore compromesso o sanzionato all'interno del set attivo»*. **Non all'infinito.** `min_revocation_effective_delay_blocks` ha già oggi un tetto di magnitudine di genesi imposto all'accettazione: `min_revocation_effective_delay_blocks_max` è campo di `ElectionBounds` (`README.md` riga 1037), e `core/coblox-core/src/params.rs`, funzione `check_magnitudes`, porta la regola `"min_revocation_effective_delay_blocks <= min_revocation_effective_delay_blocks_max"` — **verificato enumerando la funzione**.

**E l'analisi lo sa già**: lo scrive alla §7 punto 3, dove enumera `params.rs` per l'altro capo della stessa relazione. **Il fatto è nel documento, nella riga sbagliata.**

Ne discendono due difetti minori e uno di forma:

- la **domanda 3 della riga 10** — *«cosa già lo vincola per altra via»*, che è la domanda con cui la spec vuole evitare di proporre tetti ridondanti — non nomina il tetto che c'è già;
- la **domanda 4** presenta *«tetto di sicurezza di genesi $F \le F_{\max}$»* fra i vincoli **naturali da adottare**, cioè fra le proposte, quando è **stato presente e imposto**. Un ADR che copiasse la riga proporrebbe di introdurre un limite esistente;
- la **cella 10** ripete la stessa omissione.

**Scenario.** Il compilatore d'ADR legge «all'infinito» alla riga 10, non trova alla domanda 3 il tetto esistente, e scrive nell'ADR un `min_revocation_effective_delay_blocks_max` come regola di protocollo **nuova**. Nel migliore dei casi è lavoro duplicato e un'ADR che descrive male lo stato presente; nel peggiore è un secondo tetto con un nome diverso accanto a uno esistente, che è la forma con cui su questo progetto nascono le liste che divergono.

**Condizione di chiusura verificabile.** La §10 punto 2 conta l'estremo (limitato da `min_revocation_effective_delay_blocks_max`, non infinito); la §10 punto 3 nomina il tetto esistente con la sua sede in `check_magnitudes`; la §10 punto 4 e la cella 10 distinguono ciò che **esiste** da ciò che si **propone**. Chiuso quando le tre sedi concordano con `params.rs`.

RF-005 | category=documentation | severity=low | criterion=riga 8, punti 3 e 4 | remediation=portare alla domanda 3 la terza clausola del passo 6, e applicare alla riga 8 la regola sulle ancore che questa passata ha applicato alle righe 1, 5–6 e 7

**La seconda delle due righe mai riesaminate nel merito. Regge, ma con due lacune.**

*Prima lacuna, di enumerazione.* La domanda 3 risponde *«Nessun vincolo in genesi»* e concede il credito parziale del passo 7 (non-regressione), ma **omette la terza clausola del passo 6 stesso**, che l'analisi cita a metà. `ledger.md`, §*"Light-client balance verification"*, passo 6, per intero: *«Query independently operated enrolled peers, reject tips older than `max_current_balance_age_ms`, and **require the selected finalized height to be consistent with the recent checkpoint**»*. La terza clausola è un secondo vincolo alla scelta del tip, nella stessa frase da cui l'analisi estrae la prima. La domanda 3 chiede *cosa già lo vincola per altra via*, e questa è una via.

*Seconda lacuna, ed è la regola di questa passata non applicata a questa riga.* La riga 8 ancora **sia il tetto sia il pavimento** alla cadenza: il punto 4 dice *«Dipende dal ritmo di finalizzazione dei blocchi ... (es. multiplo di `block_interval_ms`)»*, e il punto 5 dice *«Il pavimento dipende dal ritmo di finalizzazione (`block_interval_ms`)»*. **Chi scrive il ritmo di finalizzazione? Lo stesso quorum.** `README.md`, §*"Cadence band"*: *«The chain's real production rate is chosen by whoever produces the blocks»* e *«no validity rule of this protocol compares anything to it»*. È **letteralmente la stessa ancora** su cui RF-001 di [REVIEW-041] ha ribaltato la riga 1 e su cui la riga 7 ha nominato il proprio residuo. Alla riga 8 la regola non è stata applicata, e la conseguenza è simmetrica alle altre due.

**Scenario d'attacco.** Un pavimento di genesi su `max_current_balance_age_ms` tarato su un multiplo di `block_interval_ms` = 5 000 ms. Il quorum rallenta la produzione a 20 000 ms per blocco — **dentro `max_ms_per_block`, quindi nessun allarme**, ed è il lato che costa solo **un terzo bloccante** e non un quorum. Il tip più fresco che qualunque peer onesto può servire è ora sistematicamente più vecchio del pavimento tarato sui 5 000: **ogni interrogazione di saldo fallisce per freschezza**, su tutta la rete, senza che alcuna regola sia violata. È l'estremo minimo della riga 8 — *«le interrogazioni di saldo falliscono sistematicamente per timeout di freschezza»* — reso raggiungibile **senza toccare il parametro**, da un canale diverso.

**Condizione di chiusura verificabile.** La §8 punto 3 nomina la clausola di coerenza col checkpoint del passo 6; la §8 punti 4 e 5 e la cella 8 dichiarano che `block_interval_ms` è la costante **dichiarata e non imposta** e che il ritmo reale è scritto dai validatori, con il rinvio alla §1 punto 2 dove il fatto è già stabilito. Chiuso quando le tre sedi concordano.

RF-006 | category=documentation | severity=low | criterion=trasversale, §3 | remediation=estendere l'avvertenza della tabella alle righe 8, 9 e 10

**L'avvertenza della tabella nomina tre righe e ne servono sei.**

La didascalia del §3 dichiara: *«per tre righe — 1, 5–6, 7 — la sezione porta una distinzione che la cella può solo nominare»*. È la correzione giusta, e ha chiuso RF-005 e RF-006 di [REVIEW-041]. Ma i quattro rilievi sopra mostrano che **le celle 3, 8, 9 e 10 sono nella stessa condizione**: portano un riassunto che perde una distinzione che la sezione porta (riga 3: il terzo termine; riga 8: l'ancora della cadenza; riga 9: il contro-operando; riga 10: il tetto esistente e il residuo). L'enumerazione «tre righe» è un conteggio, ed è il settimo conteggio non guardato di questa catena — questa volta **non** del Lead.

**Condizione di chiusura verificabile.** La didascalia enumera le righe per cui la cella non è autosufficiente, oppure dichiara la regola in forma universale — *nessuna cella è autosufficiente* — che è la forma che non richiede di ricontare a ogni passata. Il secondo è preferibile e costa una riga.

## Cosa ho attaccato senza riuscire a romperlo

Ho attaccato per primi i tre bersagli dichiarati, e sono caduti. Ho attaccato anche le sette chiusure, e **reggono**:

1. **RF-001 di [REVIEW-041], la riga 1.** L'aritmetica della mediana regge su tutta la banda: sesto blocco più recente, `6 × 5 000 = 30 000`, `6 × 20 000 = 120 000`, `6 × 2 500 = 15 000`. Le tre frasi portanti risolvono. E la classificazione della banda di cadenza come **misura e non predicato** è confermata alla lettera da `README.md`: *«no validity rule of this protocol compares anything to it»*. **Sul lato lento la riga 1 è corretta e completa.** Ho verificato in più un fatto che la riga non usa e che la rafforza: rallentare costa **un terzo bloccante**, non un quorum, quindi lo scenario della riga 1 è più economico di quello della riga 7.
2. **RF-002, le righe 5 e 6.** L'errore di categoria è reale e la correzione è esatta: `wire.md` lega l'attribuzione a `sender_node_id` e mai al Peer ID di trasporto. `validator_min_set_size` è campo di `ConsensusParametersBody` (README riga 826) e `validator_min_set_size_min` è di `ElectionBounds` (riga 1037): **ho riletto entrambe le righe**, e lo scenario di crollo del denominatore regge. La scelta di **non decidere `k`** e di enumerare le due forme ammissibili con il loro costo è la risposta giusta al perimetro della spec, non una lacuna.
3. **RF-004 e la contraddizione di `ledger.md` sull'aggregazione su `K`.** Riportata correttamente e **non corretta**, che è la disposizione giusta: `docs/protocol/` è fuori perimetro. La restrizione di [DEBT-038] alla sola quantizzazione allo slot è coerente con ciò che ho letto. Non la riapro, come istruito.
4. **La separazione (a)/(b) della riga 7.** Il predicato di uguaglianza contro l'ordinamento è esatto, e il modello del rimedio — l'obbligo di ripubblicazione già imposto a ogni revoca — è la forma giusta. **Il difetto della riga 7 è nel residuo di (a), non nella separazione.**
5. **La riga 9 sul proprio scenario a due passi.** Ho verificato in proprio ogni operando: `billing_epoch_ms` è in `HostingRateCardBody`, ha **due** occorrenze in `docs/protocol/`, e **nessun oggetto di ancoraggio lo governa** — `RewardBounds` governa `RewardPolicyBody`, non il rate card. Lo scenario di collasso della finestra regge. **Il difetto è nel rimedio, non nella diagnosi.**
6. **Il perimetro della consegna.** Un solo file di deliverable modificato, nessun documento di protocollo, nessun ADR, nessun debito, nessun codice, `STATUS.md` intatto. Verificato sul commit.

## Cosa non ho guardato

Oltre a quanto già dichiarato sopra: **non ho riesaminato nel merito le righe 2, 4, 5 e 6** — ho verificato che le loro chiusure di [REVIEW-041] siano applicate nei tre luoghi e che le loro ancore siano scritte da chi l'analisi dice, non ho ricostruito i loro scenari da zero. Le righe **1, 3, 7, 8, 9 e 10** le ho riesaminate nel merito in questa passata.

**Restano quindi quattro righe su dieci che nessuna passata ha attaccato nel merito dall'inizio della catena**, e sono 2, 4, 5 e 6. Le prime due [REVIEW-041] le dichiara utilizzabili; le seconde due sono state riscritte da zero in questa passata e non hanno mai avuto una verifica indipendente **del proprio merito**, solo della propria correzione. **È il residuo dichiarato di questa review, e la mia raccomandazione di accettare va letta con esso accanto.**

## La tabella riga per riga

| # | parametro | quarta passata |
| --- | --- | --- |
| 1 | `max_clock_drift_ms` | **Utilizzabile** — il difetto `high` di [REVIEW-041] è chiuso in tutti e tre i luoghi, e l'aritmetica regge su tutta la banda |
| 2 | `max_envelope_validity_ms` | **Utilizzabile** — corpo e cella ora concordano; merito non riesaminato in questa passata |
| 3 | `max_transport_attestation_validity_ms` | **Da correggere prima dell'ADR** — il punto 3 contraddice il punto 4, la cella e la propria fonte (RF-002) |
| 4 | `max_transport_attestation_future_skew_ms` | **Utilizzabile** — merito non riesaminato in questa passata |
| 5 | `replay_cache_entries_per_peer` | **Utilizzabile con la decisione di `k` pendente** — il difetto `high` è chiuso, e la non-decisione è il perimetro della spec |
| 6 | `replay_cache_entries_global` | Stessa disposizione della 5 |
| 7 | `max_weak_subjectivity_age_ms` | **Da correggere prima dell'ADR** — la separazione (a)/(b) regge, il residuo di (a) è sottostimato (RF-001) |
| 8 | `max_current_balance_age_ms` | **Da correggere prima dell'ADR** — due lacune, nessuna fatale (RF-005) |
| 9 | `app_suspension_notice_epochs` | **Da correggere prima dell'ADR** — diagnosi giusta, rimedio con un residuo non contato (RF-003) |
| 10 | `min_revocation_effective_delay_blocks` | **Da correggere prima dell'ADR** — non più intatta: un assoluto falso e una domanda 3 incompleta (RF-004) |

**La partizione che ne discende non è stabile/instabile.** Cinque righe sono utilizzabili così; cinque portano una correzione da fare, e **nessuna delle cinque richiede di rifare l'analisi**: quattro sono frasi da riscrivere e una è un numero da sostituire. È la ragione della raccomandazione.

## Production quality and documentation compliance

**Conforme.** [[QUALITY]] chiede di riportare onestamente limiti e deviazioni, e questa passata lo fa meglio delle tre precedenti: gli otto limiti noti dell'evidence sono **veri**, verificati uno per uno, e il punto 8 — il residuo della riga 7 senza verifica indipendente — è **la dichiarazione che ha reso possibile questo rilievo**. Una passata che dichiara il proprio punto debole e lo consegna al reviewer sta facendo esattamente ciò che il processo chiede.

**Due osservazioni non bloccanti sul processo:**

- la suite `cargo` non è stata rieseguita perché nessun file di codice è stato toccato. **È una scelta dichiarata e la condivido**: il diff è un solo file Markdown sotto `.lmbrain/`.
- l'evidence dichiara di aver eseguito gli strumenti di verifica da `/tmp`. Non li ho trovati in albero e non li ho eseguiti. **Non è un difetto** — sono strumenti di verifica di una passata, non deliverable — ma significa che **quel risolutore non è riproducibile da un terzo**, e il mio l'ho riscritto da zero per questo. Se il progetto vuole un risolutore di citazioni durevole è materia di una spec sua, non di questa.

## Required follow-up

**Nessuno di questi apre un giro di remediation.** Vanno eseguiti **dal Lead in sede di conversione in ADR**, o assegnati come spec propria se l'operatore preferisce che li chiuda la specialista.

1. **RF-001 e RF-002 prima di scrivere l'ADR.** Sono i due punti in cui il documento consegna all'operatore **una rassicurazione più forte del vero** su una finestra di esposizione. RF-001 è un numero e due frasi; RF-002 è una frase.
2. **RF-003 con RF-001**, perché sono lo stesso difetto di forma in due righe: un rimedio corretto valutato contro l'operando sbagliato.
3. **RF-004 tocca [DEBT-036].** Il tetto di magnitudine su `min_revocation_effective_delay_blocks` **esiste già**: la seconda metà del debito è di nove parametri, non di dieci. Il Lead verifichi che il testo di [DEBT-036] non conti dieci.
4. **RF-005 e RF-006** in coda, prima della conversione.
5. **Fuori da questa review, e già riportato dalla passata:** la contraddizione di `ledger.md` sull'aggregazione su `K` ([DEBT-041]) e la restrizione di [DEBT-038] alla sola quantizzazione allo slot. Non li riapro.
6. **Il residuo strutturale, e va deciso dall'operatore e non da me:** **quattro righe su dieci non hanno mai avuto un attacco nel merito**. Accettare questa consegna significa portare in ADR anche quelle. È accettabile — sono le quattro righe con la superficie d'attacco più piccola — ma è una scelta, non un fatto, e va presa consapevolmente.

## Final decision

**Verdetto raccomandato: `review_accept`, con i sei rilievi registrati come note NON bloccanti.**

**La ragione, in una frase:** i sette rilievi di [REVIEW-041] sono chiusi e verificati nei tre luoghi, i tre criteri di accettazione sull'analisi sono soddisfatti, e **nessuno dei quattro difetti che ho trovato blocca un criterio, una gate dichiarata o [[QUALITY]]** — sono quattro frasi e un numero, in righe che nessuna delle quattro passate aveva attaccato, e il luogo giusto per correggerli è l'ADR che l'operatore sta per scrivere, non un quinto giro su un documento che ne ha già bruciati tre.

**La ragione per cui non ho chiesto modifiche pur avendo trovato un difetto in ogni bersaglio dichiarato:** un quinto giro chiuderebbe questi quattro e ne produrrebbe altri, perché è ciò che i giri due e tre hanno fatto. La catena ha smesso di produrre difetti nuovi quando è cambiato l'attore, non quando è aumentato il numero di giri. **Ciò che questo documento non ha e che nessun giro gli darà è un attacco nel merito sulle righe 2, 4, 5 e 6.**

**L'avvertenza che accompagna l'accettazione, e va letta come parte del verdetto:** questa analisi è **utilizzabile come base d'ADR**, non **copiabile come testo d'ADR**. Cinque delle dieci righe portano una correzione, e la tabella del §3 **non è la fonte**: la sezione lo è, ed è vero per sei celle e non per le tre che la didascalia nomina.

**Un rilievo lasciato aperto e dichiarato vale più di uno dichiarato chiuso e non chiuso.** I sei sopra sono aperti, dichiarati, e ciascuno porta la propria condizione di chiusura verificabile.
