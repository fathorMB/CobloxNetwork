---
id: REVIEW-041
# Note: Quote the title if it contains a colon
title: "GATE-SECREVIEW di SPEC-023, terza passata: tre correzioni su dieci applicate alla sola tabella"
status: superseded
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-023
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-LEAD
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security, correctness, documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-041-EVENT-001"
    timestamp: "2026-08-27T01:20:00.000000+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-LEAD"
  - schema_version: "1"
    id: "REVIEW-041-EVENT-002"
    timestamp: "2026-08-27T00:35:36.811779400+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Terza passata: non soddisfatta. Sei rilievi su dieci chiusi, uno a metà, tre chiusi male. Due high nuovi, ENTRAMBI introdotti dalla remediation del Lead.\n\nIl Lead ha riverificato i tre fatti che ribaltano il proprio testo, e tutti e tre reggono.\n\nRF-001 high. Il Lead aveva scritto che il termine dominante della finestra di macinatura e' denominato in block_interval_ms, costante di genesi che nessun quorum scrive. La costante non e' scritta da nessuno, ma l'estremo basso della finestra NON e' la costante: e' la mediana di undici timestamp scritti dai validatori. E ledger.md, quattro righe sotto la regola citata, dice che l'intervallo obiettivo e' cinque secondi e che v0 NON lo impone, e che nessuna regola di validita' v0 vincola la distanza fra timestamp consecutivi. Un set collusivo che rallenti a 20 000 ms per blocco - dentro la banda di cadenza, quindi senza allarme - arretra la mediana a 120 000 ms e quadruplica la finestra, gratis. La premessa rassicurante e' falsa in tempo reale. Quarta occorrenza della stessa forma in questa sessione.\n\nRF-002 high. Il Lead aveva sostituito N_peers con N_min. N_min non e' definito nell'artefatto, non esiste in nessuno dei cinque documenti di protocollo, e il simbolo reale piu' vicino, validator_min_set_size, e' campo di ConsensusParametersBody, cioe' scritto dal quorum: la costante di genesi e' validator_min_set_size_min in ElectionBounds, che ne e' solo il pavimento. L'ancora nuova e' peggiore di quella che sostituisce. In piu' c'e' un errore di categoria: la cache anti-replay e' indicizzata per peer wire, non per validatore.\n\nRF-004. Una delle due riduzioni dichiarate non prese in v0 e' presa, e lo dice lo stesso documento in un altro sito: aggregare election_entropy_blocks blocchi consecutivi e' \"the reduction this document deferred to the dedicated randomness beacon and takes here\". DEBT-038 va quindi ristretto alla sola quantizzazione allo slot. E' la lezione di DEBT-038 applicata un livello sopra: il Lead ha letto la frase successiva alla citazione e non l'altro sito dello stesso documento che la supera.\n\nRF-005 e RF-006. Tre correzioni su dieci sono state applicate alla sola tabella, lasciando il corpo a contraddirla. Ne discende una regola meccanica per il prossimo giro: ogni rilievo va verificato in tabella E nel corpo.\n\nRF-007. La contabilita' di review e' entrata nell'artefatto, e il caso peggiore e' una frase del Lead - \"Verificato dal Lead sull'albero dei debiti\" - cioe' un appello all'autorita' dentro un artefatto scritto dalla stessa autorita'. La reviewer l'ha riverificata e regge, ma non deve sopravvivere all'ADR.\n\nLa reviewer ha attaccato e non ha rotto: l'aritmetica del 30 000, che sopravvive anche applicando l'intera banda di cadenza; il nome block_interval_ms, dove l'incoerenza e' del protocollo e preesiste; DEBT-038, che regge ma a portata dimezzata; e la separazione dentro la cella della riga 7, che sul pericolo tiene.\n\nCinque righe utilizzabili contro quattro della passata precedente, ma delle cinque non utilizzabili tre lo sono per difetti che questa remediation ha introdotto.\n\nNumerazione riportata alla forma canonica RF-*: REVIEW-040 usava NF-*, e il brain aveva rifiutato di legarvi un debito. Difetto del Lead, corretto qui."
    evidence_refs: ["SPEC-023", "REVIEW-040", "DEBT-036", "DEBT-038", "ADR-013"]
    implementation_agent: "AGENT-LEAD"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-041-EVENT-003"
    timestamp: "2026-08-27T10:26:48.221625300+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Quarta passata di remediation su .lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md, eseguita da AGENT-002. Un solo file modificato; nessun documento di protocollo, ADR, spec, debito o file di codice toccato.\n\nRegola meccanica applicata su richiesta esplicita di REVIEW-041: ogni rilievo chiuso in TRE luoghi - corpo della sezione, cella della tabella tassonomica, riquadro di correzione - e la chiusura verificata in tutti e tre. La tabella porta ora l'avvertenza che non e' autosufficiente e che dove diverge dalla sezione vale la sezione.\n\nRegola sulle ancore applicata prima di scrivere: per ogni grandezza a cui l'analisi ancora un vincolo e' stato verificato CHI la scrive. L'esito ha cambiato tre conclusioni.\n\nRF-001 chiuso. L'estremo basso della finestra e' dichiarato mediana di undici timestamp scritti dai validatori, non una costante. Portate le due frasi che l'artefatto non aveva: l'intervallo obiettivo e' cinque secondi e v0 non lo impone; nessuna regola di validita' v0 vincola la distanza fra timestamp consecutivi. Aggiunta la frase generale di README.md che dichiara un predicato su quella distanza rifiutato e non assente. Scenario aritmetico completo: rallentamento a max_ms_per_block 20 000 ms, dentro la banda quindi senza allarme, mediana a 120 000 ms, finestra quadruplicata; lato veloce min_ms_per_block 2 500 ms da' 15 000 ms. Conclusione dichiarata su tutta la banda. Aggiunta al punto 3 la distinzione di specie: la banda di cadenza e' una misura, non un predicato.\n\nRF-002 chiuso. Errore di categoria dichiarato con le frasi di wire.md: la cache indicizza (sender_node_id, nonce) e l'attribuzione non si lega mai al Peer ID di trasporto, quindi il peer e' un'identita' enrollata e non un seggio di validatore. Dichiarato che N_min non esiste - zero occorrenze in docs/protocol/ e in core/ - che validator_min_set_size e' campo firmato dal quorum e che la costante di genesi e' solo il suo pavimento validator_min_set_size_min in ElectionBounds, con lo scenario di crollo del denominatore. Dichiarato che README.md contiene UNA SOLA costante di genesi, block_interval_seconds, e che nessun documento firmato porta un numero di peer o di identita'. La relazione e' riscritta su due soli operandi, entrambi campi dello stesso documento, con il divisore k dichiarato NON DECISO e le due sole forme ammissibili con il loro costo. Corpo di 5 e 6 allineati; le celle 5 e 6 non portano piu' N_min.\n\nRF-003 chiuso. L'argomento e' riscritto come aritmetico e la regola generale sui vincoli a due lati e' RITIRATA esplicitamente, con la constatazione che pavimento e tetto di quella riga non sono in conflitto.\n\nRF-004 chiuso nell'analisi. L'aggregazione su K blocchi e' dichiarata PRESA, con K = election_entropy_blocks e la frase citata per intero; dichiarato che la Challenge evidence la elenca ancora come non presa, quindi che il documento di protocollo si contraddice - rilievo aperto e NON corretto, docs/protocol/ e' fuori perimetro. La quantizzazione allo slot e' dichiarata la sola via residua e la nota dichiara che DEBT-038 va ristretto a essa. La correzione del debito e' del Lead e non e' stata applicata.\n\nRF-005 chiuso. Corpo della sezione 2 riscritto ai punti 2 e 4: il danno massimo porta il termine di prodotto e le frasi di wire.md sulla ritenzione e sulla non-evizione; le due proprieta' sono separate; sparita dal corpo la dizione saturazione irreversibile e la classificazione sola magnitudine.\n\nRF-006 chiuso. I due danni della riga 7 sono separati come (a) magnitudine e (b) disaccordo ANCHE NEL CORPO, con la dichiarazione esplicita che (b) non e' conseguenza di (a) e che il nesso causale precedente era un non-sequitur. La colonna del vincolo porta ora entrambi i rimedi, con per (b) il modello dell'obbligo di ripubblicazione gia' presente nel protocollo.\n\nRF-007 chiuso. Tutti i riferimenti di review spostati nei riquadri; nessun rilievo citato in linea nella prosa dei punti 1-5. Tutte le attribuzioni di verifica personale rimosse: zero occorrenze di \"Verificato dal Lead\".\n\nIn piu', allineata di propria iniziativa la riga 9: la cella scioglieva l'oppure di NF-04 mentre il corpo lo portava ancora. Non era un rilievo di REVIEW-041 ma e' la stessa forma che RF-005 e RF-006 censurano.\n\nVerifica eseguita. Risolutore di citazioni su tutte le frasi virgolettate dell'analisi contro i cinque documenti sorgente, con normalizzazione degli spazi e spogliatura dell'enfasi: 132 su 132 risolvono, zero non risolte. Una prima esecuzione ne riportava una non risolta, difetto introdotto da questa stessa passata - virgolette interne scritte con barra rovesciata - corretto e rieseguito. Verifica mirata preventiva delle 22 frasi e valori portanti: 22 su 22. Enumerazione di N_min: zero. Enumerazione della tabella delle costanti di genesi: una riga. Gate rieseguite: consensus_parameters_closure.py PASS 22 su 22, prova in negativo PASS su C1 e C2, published_artifacts.py PASS. Suite Rust non rieseguita perche' nessun file di codice e' stato toccato: scelta dichiarata.\n\nDue difetti trovati e non corretti, entrambi riportati. Primo: ledger.md si contraddice sull'aggregazione su K blocchi, famiglia 2 in un documento di protocollo, fuori perimetro. Secondo: la sotto-affermazione di RF-002 secondo cui la cache sarebbe indicizzata per peer wire e' inesatta - wire.md dichiara che l'attribuzione si lega a sender_node_id e MAI al Peer ID di trasporto. Il rilievo regge e si rafforza, ma l'artefatto e' stato scritto sul fatto verificato e non sulla formulazione della review.\n\nResiduo trovato da questa passata e senza verifica indipendente: il vincolo relazionale della riga 7 e' ancorato a block_interval_ms, che e' dichiarato e non imposto; sul lato veloce della banda di cadenza il predicato ammette fino al DOPPIO della finestra reale di successione. Scritto come residuo di grado con il fattore limitato dalla banda, e segnalato per attacco nella prossima passata insieme al suo riflesso sulla riga 10.\n\nLimiti noti dichiarati nell'evidence: k non deciso (perimetro della spec), nessuno scenario eseguito, fedelta' di significato delle citazioni non verificata meccanicamente, threat-model.md non letto, ADR non riletti per intero, implementazione di election_entropy_blocks non verificata in codice."
    evidence_refs: ["SPEC-023", "REVIEW-041", "REVIEW-040", "REVIEW-038", "DEBT-036", "DEBT-038"]
    implementation_agent: "AGENT-LEAD"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-041-EVENT-004"
    timestamp: "2026-08-27T14:53:56.093039300+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "superseded"
    actor_role: "project-lead"
    reason: "Superseduta da REVIEW-043, accepted il 2026-08-27. Tutti e sette i suoi rilievi sono stati chiusi dalla quarta passata di AGENT-002, e REVIEW-043 ha verificato la chiusura nei tre luoghi. Porta gia' l'evento di remediation REVIEW-041-EVENT-003 con il rapporto completo di quella passata: e' quindi la review che documenta il giro, non una richiesta di modifica ancora pendente. SPEC-023 e' done.\n\nStessa causa delle due precedenti, censita in knowledge/review-lifecycle-discipline.md."
    evidence_refs: ["SPEC-023", "REVIEW-043"]
    implementation_agent: "AGENT-LEAD"
links: [DEBT-036, DEBT-038]
created: 2026-08-27
updated: 2026-08-27
tags: [security, review, governance]
related_decisions: [ADR-012, ADR-013]
activity:
  - date: 2026-08-27
    action: "created"
  - date: 2026-08-27
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-27
    action: "recorded review remediation"
  - date: 2026-08-27
    action: "transitioned changes-requested -> superseded"
---
# Review

> **Terza passata di `GATE-SECREVIEW` su [SPEC-023].** L'`implementation_agent` è **AGENT-LEAD**: la remediation di [REVIEW-040] l'ha scritta il Lead, su decisione dell'operatore, perché l'oggetto sta in `.lmbrain/knowledge/`. **Chi scrive non verifica**, e questa review è l'unica verifica che quel testo ha avuto.
>
> **La numerazione dei finding torna alla forma canonica `RF-*`.** [REVIEW-040] usava `NF-*`, e il brain ha rifiutato di legarvi un debito perché la forma stabile richiede `RF-*`. Difetto del Lead, corretto qui.

## Outcome

**`GATE-SECREVIEW` non soddisfatta.** Sei rilievi su dieci chiusi, uno a metà, **tre chiusi male** — e i tre hanno la stessa forma: **la correzione è stata scritta nella tabella e non nel corpo**, oppure ha sostituito un'ancora sbagliata con **un'ancora peggiore**.

Due `high` nuovi, **entrambi introdotti dalla remediation del Lead**.

**Cinque righe utilizzabili così** contro le quattro della seconda passata. Ma delle cinque non utilizzabili, **tre lo sono per difetti che questa remediation ha introdotto**.

## Tests and verification

La reviewer **non ha eseguito alcuno strumento** in questa passata, e lo dichiara. **Il Lead ha riverificato i tre fatti che ribaltano il proprio testo**, e tutti e tre reggono.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security | severity=high | criterion=riga 1, punto 2 | remediation=dichiarare che l'estremo basso è prodotto dai timestamp dei validatori e che v0 non impone la cadenza
**La correzione attribuisce il termine dominante a un'ancora che l'avversario muove.**

Il Lead ha scritto: *«Il termine dominante è denominato in `block_interval_ms`, che è costante di genesi e che nessun quorum scrive»*. **La costante non è scritta da nessuno, ma l'estremo basso della finestra non è la costante: è la mediana di undici timestamp scritti dai validatori.**

E `ledger.md`, quattro righe sotto la regola che la riga cita, dice: *«**The target block interval is 5 seconds, and v0 does not enforce it.**»* e *«**No v0 validity rule constrains the distance between consecutive `timestamp_ms` values.**»* **Verificato dal Lead.** L'artefatto non porta quella frase in alcun punto.

**Scenario.** Il set collusivo rallenta la produzione a 20 000 ms per blocco — **dentro `max_ms_per_block` della banda di cadenza, quindi nessun allarme di light client**. La mediana degli undici arretra a ~120 000 ms e la finestra di macinatura **quadruplica** a ~1,2·10⁵ valori legali, gratis, senza violare nulla. Il tetto su `max_clock_drift_ms` resta irrilevante come prima, **ma la premessa rassicurante è falsa in tempo reale**.

**È la quarta occorrenza della stessa forma in questa sessione**: una grandezza ancorata a un denominatore che lo stesso avversario muove per un canale diverso.

RF-002 | category=security | severity=high | criterion=righe 5 e 6 | remediation=ancorare a una costante di genesi reale, definire il simbolo al primo uso, allineare il corpo
**`N_min` non esiste, e il candidato più vicino è scritto dal quorum.** La correzione ha sostituito `N_peers` con `N_min`. Tre difetti sovrapposti, tutti verificati:

- **`N_min` non è definito nell'artefatto** — tre occorrenze, zero definizioni;
- **`N_min` non esiste nel protocollo** — **zero occorrenze in tutti e cinque i documenti**, verificato dal Lead;
- il simbolo reale più vicino, `validator_min_set_size`, è **campo di `ConsensusParametersBody`**, cioè **scritto dal quorum sedente**. La costante di genesi è `validator_min_set_size_min` in `ElectionBounds`, che ne è solo il pavimento. **Verificato dal Lead: `README.md:826` contro `:1043`.**

**Scenario.** L'ADR scrive `per_peer <= global / N_min`. Il quorum porta `validator_min_set_size` al proprio pavimento di genesi: il denominatore crolla, il tetto su `per_peer` si alza, e il DoS incrociato torna disponibile **senza violare la relazione**.

**E c'è un errore di categoria indipendente dai tre:** la cache anti-replay è indicizzata per **peer connesso al livello wire**, non per validatore. Ancorare la relazione alla dimensione del set di validatori è sbagliato comunque si scelga la costante.

**Il corpo non è stato toccato:** i punti 4 delle sezioni 5 e 6 portano ancora `N_peers`, e il riquadro di correzione pubblicizza ancora la correzione precedente.

RF-003 | category=correctness | severity=medium | criterion=riga 1, punto 4 | remediation=riscrivere l'argomento come aritmetico, e ritirare la regola generale o dimostrarla altrove
**L'incompatibilità fra pavimento e tetto è vera ma attribuita alla cosa sbagliata.** Il Lead ha scritto che *«pavimento e tetto sono incompatibili se il tetto viene tarato sulla macinatura»* e ne ha tratto una regola generale: *«un vincolo i cui due lati non possono essere soddisfatti insieme non è un vincolo»*.

**Il pavimento non c'entra.** Il pavimento vale decine di millisecondi; «sotto la banda» significa finestra sotto i 1 000 ms, e un tetto di 100 ms sarebbe compatibile con quel pavimento. Ciò che rende l'obiettivo impossibile è che **a tetto zero la finestra è già 3·10⁴**: nessun tetto **non negativo** basta, pavimento o no. **L'argomento corretto è aritmetico, non un conflitto fra i due lati.**

**Perché conta:** la riga consegna a un ADR una **regola di ragionamento** dimostrata su un caso in cui i due lati non sono incompatibili. Chi la riuserà su un'altra riga concluderà male.

RF-004 | category=correctness | severity=medium | criterion=riga 1 e nota su DEBT-038 | remediation=nominare `election_entropy_blocks` come riduzione presa, e restringere DEBT-038
**Una delle due «riduzioni non prese in v0» è presa, e lo dice lo stesso documento altrove.** `ledger.md` scrive: *«Aggregating `election_entropy_blocks` consecutive blocks raises the cost of controlling the whole window to holding consecutive proposal slots — **the reduction this document deferred to "the dedicated randomness beacon" and takes here**»*. **Verificato dal Lead.**

**Conseguenza su [DEBT-038]:** la via di chiusura senza proprietario **non è la coppia, è una sola** — la quantizzazione allo slot. **Metà del lavoro esiste già, sotto altro nome, nella sezione della regola di elezione.**

**È la lezione di [DEBT-038] applicata un livello sopra**: il Lead ha letto la frase *successiva* alla citazione, e non l'altro sito dello stesso documento che la supera.

RF-005 | category=documentation | severity=medium | criterion=riga 2 | remediation=riscrivere i punti 2 e 4 della sezione 2, con marcatore
**Corretta in tabella, non nel corpo.** La cella dichiara ora «magnitudine + relazionale con le due cache»; la sezione dichiara ancora **«Magnitudine (tetto massimo di genesi)»** e parla ancora di «saturazione irreversibile» senza il termine di prodotto. **Riga e sezione si contraddicono**, e l'ADR può ereditare l'una o l'altra a seconda di quale metà viene copiata.

RF-006 | category=correctness | severity=medium | criterion=riga 7 | remediation=separare i due danni anche nel corpo, aggiornare il riquadro, e portare la chiusura sulla variazione nella colonna del vincolo
**Il corpo contraddice la cella corretta, e mantiene il nesso causale falso.** Due residui: **(a)** la colonna del vincolo non porta alcun rimedio per la seconda voce, quindi chi compila l'ADR riga per riga **non ha nulla da scrivere** — e [REVIEW-040] aveva nominato il modello, il governo sulla variazione; **(b)** il corpo afferma ancora che *«un valore alto ... in combinazione col fail-closed obbligatorio ... fa sì che un disaccordo faccia fallire chiuso ogni light client»*. **Il disaccordo fa fallire chiuso a qualunque valore**: è un non-sequitur, ed è precisamente la confusione fra magnitudine e disaccordo che il rilievo precedente nominava.

RF-007 | category=documentation | severity=low | criterion=trasversale | remediation=spostare i riferimenti nei riquadri di correzione, e togliere le attribuzioni di verifica personale
**La contabilità di review è entrata nell'artefatto.** Sei righe citano un rilievo **in linea nel testo**, non in un riquadro. E la correzione che rispondeva all'obiezione *«lo stato di review non è una risposta di liveness»* **rimette la storia del rilievo dentro la propria risposta**.

**Il caso peggiore è una frase del Lead:** *«Verificato dal Lead sull'albero dei debiti»* — **un appello all'autorità dentro un artefatto scritto dalla stessa autorità**. La reviewer l'ha riverificata e regge, **ma non deve sopravvivere all'ADR**.

## La tabella riga per riga

| # | parametro | terza passata |
| --- | --- | --- |
| 1 | `max_clock_drift_ms` | **Classificazione finalmente giusta, motivazione no** (RF-001, RF-003, RF-004) |
| 2 | `max_envelope_validity_ms` | Cella corretta, **sezione no**: si contraddicono (RF-005) |
| 3 | `max_transport_attestation_validity_ms` | **Utilizzabile così** — ha ceduto una volta ([REVIEW-038] RF-004, `medium`) e ha convertito |
| 4 | `max_transport_attestation_future_skew_ms` | **Utilizzabile così** |
| 5 | `replay_cache_entries_per_peer` | **Da non lasciar passare — l'ancora nuova è peggiore di quella che sostituisce** (RF-002) |
| 6 | `replay_cache_entries_global` | Stesso difetto (RF-002) |
| 7 | `max_weak_subjectivity_age_ms` | Cella molto migliorata, rimedio incompleto, corpo contraddittorio (RF-006) |
| 8 | `max_current_balance_age_ms` | **Utilizzabile così** |
| 9 | `app_suspension_notice_epochs` | **Utilizzabile così** — l'«oppure» è sciolto |
| 10 | `min_revocation_effective_delay_blocks` | **Utilizzabile senza riserve — la sola intatta per tre passate** |

## Cosa la reviewer ha attaccato senza riuscire a romperlo

1. **L'aritmetica del 30 000**, e **oltre la propria ipotesi**: applicando la banda di cadenza del trust anchor, il lato veloce dà `6 × 2 500 = 15 000` ms — sempre almeno 10³ valori legali. **La conclusione sopravvive a tutta la banda.**
2. **Il nome `block_interval_ms`.** L'ha attaccato come simbolo sbagliato — la tabella delle costanti di genesi nomina `block_interval_seconds` — e **non l'ha rotto**: `ledger.md` chiama esso stesso `block_interval_ms` la costante di genesi. **L'incoerenza è del protocollo e preesiste alla remediation.**
3. **[DEBT-038]**, riverificato in proprio. Il rilievo del Lead regge, **ma la sua portata è dimezzata** (RF-004).
4. **La separazione dentro la cella della riga 7**, che era la domanda esplicita del Lead. **Sul pericolo non l'ha rotta**: la formulazione è esplicita e un operatore attento non scriverà una banda per quella voce. La rottura è nella colonna del rimedio.
5. **La riga 9**, enumerando `params.rs`.

## Cosa la reviewer non ha guardato

- **Non ha eseguito alcuno strumento**, né la suite, né le gate.
- **Lo strato di citazione**, su istruzione. Ha però letto **l'intorno di sei citazioni portanti**, e **su una l'intorno smentisce il testo** (RF-004): la lezione di [DEBT-038] ha prodotto un secondo caso alla prima applicazione.
- **[ADR-010], [ADR-013], [ADR-016], [ADR-017]**: non letti; conseguenze verificate sui documenti e su `params.rs`.
- **`threat-model.md`**: non letto, TM-37 non riverificato.
- **Se `election_entropy_blocks` sia implementato in codice**: verificato solo nel documento.
- **Nessun attacco eseguito.**

## Required follow-up

1. **RF-001 e RF-002 prima di qualunque ADR**: sono i due punti in cui il documento consegnerebbe all'operatore **una rassicurazione falsa**.
2. **RF-005 e RF-006 nello stesso giro, e con essi una regola meccanica**: **ogni rilievo va verificato in tabella *e* nel corpo**. Tre su dieci sono stati applicati alla sola tabella, ed è un controllo, non un giudizio.
3. **RF-004 restringe [DEBT-038]** alla sola quantizzazione allo slot.
4. **RF-003 e RF-007** prima della conversione in ADR, non prima della prossima passata.

## Errata di questa review, e la partizione che ne dipendeva

**La cella della riga 3 diceva «intatta per tre passate».** Era falsa: la riga 3 ha ceduto in [REVIEW-038] con `RF-004`, `medium`, ed è stata corretta. **La stessa tabella, sette righe sotto, dice della riga 10 «la sola intatta per tre passate»** — un superlativo e il suo controesempio nello stesso oggetto. Corretto sopra.

**È il sesto conteggio non guardato di questa sessione, ed è del Lead.** Non è un dettaglio di contabilità: **la partizione stabile/instabile che il Lead ha costruito su questa tabella poggiava in parte su quella cella.** La partizione è stata attaccata separatamente e non regge, e la ragione è registrata in `.lmbrain/knowledge/predicato-di-accettazione.md`: enumerando le tre passate, la riga 9 ha ceduto **due volte**, di cui una `high`, e ha convertito; le righe 3 e 4 una volta ciascuna. **La sola riga intatta per tre passate è la 10.**

Quella tabella separa quindi **convertito da ancora aperto**, non stabile da instabile, e le due cose sono confuse dalla recenza.
