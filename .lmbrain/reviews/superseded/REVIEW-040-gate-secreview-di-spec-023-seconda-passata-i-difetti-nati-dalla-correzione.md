---
id: REVIEW-040
# Note: Quote the title if it contains a colon
title: "GATE-SECREVIEW di SPEC-023, seconda passata: i difetti nati dalla correzione"
status: superseded
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-023
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security, correctness, documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-040-EVENT-001"
    timestamp: "2026-08-27T00:40:00.000000+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-040-EVENT-002"
    timestamp: "2026-08-27T00:16:48.377554+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Seconda passata di GATE-SECREVIEW: non soddisfatta. Due high, quattro medium, cinque low. Nessuno dei due high ripete la prima passata: entrambi nascono dalla correzione.\n\nIl Lead ha riverificato in modo indipendente i tre fatti portanti, e tutti e tre reggono.\n\nNF-01 high. La finestra di macinatura sul beacon non e' governata da max_clock_drift_ms. ledger.md dice che timestamp_ms deve superare la mediana degli undici blocchi finalizzati precedenti e non eccedere la deriva massima: la finestra ha due estremi, e quello basso e' la mediana degli undici, cioe' il sesto blocco piu' recente, che all'intervallo dichiarato di cinque secondi sta circa 30 000 ms indietro. A max_clock_drift_ms = 0 la finestra ammette gia' 3*10^4 valori legali, dentro la banda 10^3-10^6 che l'analisi usa come proprio metro. Il termine dominante e' denominato in block_interval_ms, non nel parametro che l'ADR limiterebbe: la riclassificazione ha cambiato la forma del vincolo e non la grandezza, quindi la riga 1 e' ancora famiglia 3. Ne segue anche che \"il costo dell'attacco e' lineare in max_clock_drift_ms\" e' falso - il contributo e' additivo su un termine che lo domina - e che pavimento e tetto sono incompatibili senza che il documento se ne accorga.\n\nNF-02 high. Il fail-closed di flotta non e' un rischio estremo: README.md impone al client di verificare che i due valori concordino e di fallire chiuso altrimenti, quindi il predicato e' disaccordo e non magnitudine, e nessuna banda lo tocca. Peggio, la correzione ha SOSTITUITO il danno estremo invece di affiancarlo: l'allargamento della finestra di esposizione, che una banda chiuderebbe davvero, e' uscito dalla cella di tabella. L'operatore che usasse la riga verbatim scriverebbe una banda credendo di aver chiuso cio' che la banda non puo' chiudere.\n\nNF-06 low ma significativo: l'analisi scrive \"max_clock_drift_ms = 0 (il valore oggi in albero)\" e il valore in albero e' 1, verificato dal Lead in tre punti. Contraddice la premessa dello stesso documento. E' il quinto numero non guardato di questa sessione, ed e' nella sezione aggiunta per rispondere alla domanda sul pavimento.\n\nLa tesi che il Lead riteneva piu' affilata - che la correzione di RF-001 avesse spostato il difetto di un anello, essendo billing_epoch_ms senza limite - e' stata attaccata e NON regge: una banda sul prodotto limita la finestra wall-clock comunque si muovano i due fattori. Il residuo e' di luogo e non di grado, ed e' NF-04.\n\nIl bilancio migliora: quattro righe su dieci utilizzabili cosi' contro tre della prima passata, e la riga 10 attraversa due passate intatta. Ma due righe vanno riclassificate, e la 7 non lo era prima.\n\nLa reviewer ha inoltre verificato lo strato di citazione con un risolutore proprio che controlla la tripla documento-sezione-frase e non la sola frase: 37 su 37 risolvono e tutte stanno nella sezione dichiarata, comprese le due che il risolutore del Lead rifiutava perche' non spogliava l'enfasi Markdown. E' evidenza a favore della normalizzazione, e va portata in SPEC-024 dove quella decisione e' aperta."
    evidence_refs: ["SPEC-023", "REVIEW-038", "DEBT-036", "DEBT-037", "SPEC-024", "ADR-013"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-040-EVENT-003"
    timestamp: "2026-08-27T00:23:43.451891800+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Remediation eseguita dal Lead e non da uno specialista, su decisione dell'operatore. E' ammissibile perche' l'oggetto e' `.lmbrain/knowledge/`, dentro il confine di scrittura del Lead; codice, test e documenti di protocollo restano fuori e non sono stati toccati.\n\nConseguenza dichiarata: il Lead non puo' verificare cio' che ha scritto. La terza passata di GATE-SECREVIEW resta ad AGENT-007, e questa remediation non porta alcun verdetto.\n\nNF-01. La riga 1 non prova piu' a vincolare la macinatura del beacon con questo parametro. Il punto 2 dichiara che il contributo e' additivo e non dominante, con l'aritmetica: la finestra ha due estremi, quello basso e' la mediana degli undici cioe' il sesto blocco piu' recente, che a cinque secondi sta circa 30 000 ms indietro, quindi a deriva zero la finestra ammette gia' 3*10^4 valori legali. Il punto 4 dichiara che due stesure hanno provato a vincolare qui la macinatura e nessuna morde; che pavimento e tetto sarebbero incompatibili se il tetto fosse tarato sulla macinatura, il che e' la prova che la grandezza e' sbagliata; e nomina le tre vie di chiusura che ledger.md indica, nessuna delle quali e' un tetto su questo parametro. La classificazione diventa magnitudine tarata sulla tolleranza d'orologio, con la macinatura dichiarata mitigazione di grado e non chiusura.\n\nIl Lead ha inoltre trovato, verificando NF-01, che la via di chiusura NON HA UN PROPRIETARIO: ledger.md colloca le due riduzioni non prese \"with the dedicated randomness beacon, which is M-02 work under DEBT-005\", e DEBT-005 e' risolto, per di piu' con un oggetto - la regola di elezione - che non e' un beacon di casualita'. La nota e' scritta nella riga 1 perche' l'ADR se la porti dietro.\n\nNF-02. La cella del rischio estremo massimo della riga 7 torna a portare l'allargamento della finestra di esposizione a una revoca finalizzata, che e' il danno che una banda chiude. L'arma di fail-closed e' ora una voce separata dentro la cella, dichiarata NON chiudibile da alcuna banda, con la ragione: il predicato e' disaccordo e non magnitudine, e si governa sulla variazione.\n\nNF-03. La relazione delle righe 5 e 6 e' ancorata a una costante di genesi, per_peer <= global / N_min, e la riga dichiara che N_peers non e' una grandezza di protocollo e che una relazione che lo invoca non e' valutabile su un documento firmato.\n\nNF-04. La riga 9 scioglie l'oppure: banda sul prodotto, valutata all'accettazione di ENTRAMBE le specie di documento, con la ragione - i quattro documenti firmati attivano indipendentemente, e legare il predicato alla sola accettazione di consensus_parameters lo farebbe eludere pubblicando hosting_rate_card dopo.\n\nNF-05. La riga 2 porta il termine relazionale con le due cache, e la cella del rischio massimo dice che la saturazione dipende dal prodotto ritenzione per tasso d'inserimento contro le cache e non dalla sola durata.\n\nNF-06. Il valore in albero e' 1 e non 0, corretto, con la nota che diceva il falso e contraddiceva la premessa dello stesso documento.\n\nNF-07. L'apertura dichiara che i tre gruppi NON sono una partizione e perche': min_revocation_effective_delay_blocks appartiene alla banda di revoca e ai dieci operativi, perche' ADR-017 gli ha dato il secondo ruolo senza togliergli il primo.\n\nNF-08. Il quinto punto della riga 6 risponde alla domanda di liveness invece di citare lo stato di review.\n\nNF-09. L'accecamento dell'allarme di fork e' marcato come deduzione dell'analisi, con la distinzione esplicita fra cio' che il documento dice e cio' che l'analisi ne inferisce, e con la ragione per cui va etichettato: un ADR eredita le frasi che cita.\n\nNF-10. La riga 10 dice check_relations invece di \"vincolo di genesi\", con la distinzione fra relazione e magnitudine e perche' conta.\n\nVerificato dal Lead con un risolutore che controlla la tripla documento-sezione-frase, non la sola frase: 36 frasi citate, 16 triple che risolvono su documento E sezione, le altre 16 risolvono a livello di albero, zero numeri di riga nudi. Nessun file fuori da .lmbrain/knowledge/ e' stato modificato."
    evidence_refs: ["SPEC-023", "REVIEW-040", "DEBT-036", "ADR-013", "ADR-017"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-LEAD"
  - schema_version: "1"
    id: "REVIEW-040-EVENT-004"
    timestamp: "2026-08-27T14:53:48.929514600+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "superseded"
    actor_role: "project-lead"
    reason: "Superseduta da REVIEW-043, accepted il 2026-08-27, verdetto dell'ultimo giro di GATE-SECREVIEW su SPEC-023. I suoi rilievi — i difetti nati dalla correzione della prima passata — sono stati ripresi e chiusi dalla terza e dalla quarta passata, e REVIEW-043 ha verificato la chiusura nei tre luoghi: corpo, cella della tabella, riquadro. SPEC-023 e' done.\n\nStessa causa di REVIEW-038: una review nuova a ogni giro invece del verdetto ri-espresso su quella esistente. Vedi knowledge/review-lifecycle-discipline.md."
    evidence_refs: ["SPEC-023", "REVIEW-043", "REVIEW-041"]
    implementation_agent: "AGENT-002"
links: [DEBT-036, DEBT-037]
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

> **Seconda passata di `GATE-SECREVIEW` su [SPEC-023]**, sul documento corretto dopo la remediation di [REVIEW-038]. L'oggetto resta `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md`, che diventerà un ADR.

## Outcome

**`GATE-SECREVIEW` non soddisfatta.** Due `high` residui, quattro `medium`, cinque `low`.

**Nessuno dei due `high` è una ripetizione della prima passata: entrambi nascono dalla correzione.** È la forma che questa sessione ha visto ripetutamente — una correzione che introduce ciò che correggeva — e qui si presenta al livello più caro, dentro la tabella destinata all'ADR.

**Il bilancio migliora comunque**: quattro righe su dieci sono utilizzabili così contro le tre della prima passata, e la riga 10 attraversa due passate intatta.

## Acceptance-criteria compliance

Non applicabile: la gate è `before-done`, i criteri di accettazione sono chiusi in [REVIEW-037].

## Tests and verification

La reviewer **non ha rieseguito** la suite né le gate di progetto, e lo dichiara. Ha eseguito una cosa sola, `consensus_parameters_closure.py`, per confermare i 22 campi e la stampa corretta — non come gate.

**Il Lead ha riverificato in modo indipendente i tre fatti portanti** dei due `high` e del `low` più significativo. Tutti e tre reggono.

## Disposizione dei rilievi della prima passata

| RF di [REVIEW-038] | Esito |
| --- | --- |
| RF-001 | **Chiuso sulla grandezza**, aperto sul luogo → NF-04 |
| RF-002 | **Chiuso sulle citazioni**, la grandezza è ancora sbagliata → **NF-01** |
| RF-003 | Fuori scopo, è [DEBT-037] |
| RF-004 | **Chiuso** |
| RF-005 | **Chiuso a metà**, e la metà chiusa è nella colonna sbagliata → **NF-02** |
| RF-006 | **Chiuso male** → NF-03 |
| RF-007 | **Risposto**, due risposte su dieci difettose → NF-06, NF-08 |
| RF-008 | **Chiuso, ed è la correzione meglio riuscita** |
| RF-010 | **Chiuso** |
| RF-011 | **Chiuso**, con una deduzione presentata come lettura → NF-09 |

**RF-001 non è famiglia 3 di secondo grado**, che era la tesi che il Lead considerava più affilata e aveva chiesto di attaccare. Una banda sul **prodotto** `epoche × billing_epoch_ms` limita la finestra wall-clock comunque si muovano i due fattori, e differisce dal caso RF-008 dove limitare una grandezza *rispetto a* un'ancora libera la lascia libera. **Il residuo è di luogo, non di grado.**

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

NF-01 | category=security | severity=high | criterion=tabella tassonomica riga 1 | remediation=dichiarare i due estremi della finestra, che il termine dominante è denominato in `block_interval_ms`, e che il tetto è mitigazione di grado e non chiusura
**La finestra di macinatura non è governata da `max_clock_drift_ms`, e la riga 1 è ancora famiglia 3 dopo la riclassificazione.**

`ledger.md` dice che `timestamp_ms` è *«constrained only to exceed the median of the previous 11 finalized blocks and not to exceed the maximum clock drift»* — e lo ripete nella regola: *«MUST be greater than the median of the previous 11 finalized blocks and no more than the active maximum clock drift after the proposal is received»*. **Verificato dal Lead, entrambe le occorrenze.**

La finestra ha quindi **due estremi**, e quello basso è la mediana degli undici. La mediana di undici valori è il **sesto** più recente: all'intervallo obiettivo dichiarato di cinque secondi sta circa **30 000 ms** nel passato. **A `max_clock_drift_ms = 0` la finestra ammette già dell'ordine di 3·10⁴ valori legali** — dentro la banda `10³–10⁶` che l'analisi usa come proprio metro.

**Il termine dominante è denominato in `block_interval_ms`, non nel parametro che l'ADR limiterebbe.** Ne seguono tre cose:

- *«il costo dell'attacco è **lineare in `max_clock_drift_ms`**»* è **falso**: il contributo di quel parametro è **additivo** su un termine che lo domina;
- **pavimento e tetto sono incompatibili** e il documento non se ne accorge: nessun tetto in ms compatibile col pavimento che la stessa riga dichiara — jitter NTP più propagazione — porta la finestra sotto la banda;
- l'analisi cita la sezione giusta e **omette le tre cose che quella sezione dice sulla mitigazione**: la regola di copertura a due issuer, dichiarata *«the mitigation of this grinding»*, e le due riduzioni non prese in v0 — quantizzazione di `timestamp_ms` allo slot, e aggregazione su `K` blocchi consecutivi. **Nessuna delle tre è un tetto su questo parametro.**

**Il documento contiene la premessa che confuta la propria conclusione**: il punto sull'asimmetria dichiarata dice che all'indietro il vincolo è tenuto dalla mediana degli undici.

**Scenario.** L'ADR fissa `max_clock_drift_ms` a 2 000 ms credendo di aver prezzato la macinatura. La coppia proposer/issuer colluso enumera comunque ~3·10⁴ timestamp a una SHA-256 l'uno — microsecondi di lavoro — e pilota l'assegnazione dei challenge. **Il tetto non ha cambiato l'ordine di grandezza.**

*Perimetro della reviewer: letto nelle due sezioni citate; l'aritmetica della mediana è derivata dall'intervallo obiettivo dichiarato, **non misurata su una catena in esecuzione**. Nota che v0 non impone l'intervallo, il che rende il termine dominante **meno** governabile, non più.*

NF-02 | category=security | severity=high | criterion=tabella tassonomica riga 7 | remediation=la cella del rischio massimo torna all'allargamento della finestra di esposizione; l'arma di fail-closed diventa una voce a sé, dichiarata indipendente dal valore
**Il fail-closed di flotta non è un rischio estremo, e la correzione l'ha messo dove non è azionabile.**

`README.md` dice che, ottenuto un header autenticato, il client *«MUST check that the two agree and fail closed if they do not»*. **Verificato dal Lead.** Il predicato è **disaccordo**, non **magnitudine**: un valore qualunque dentro qualunque banda, purché diverso da quello nel checkpoint, produce l'arma. **Una banda a due lati non la tocca.**

**E la correzione ha sostituito il danno estremo invece di affiancarlo.** Il danno genuinamente estremo al massimo — l'allargamento della finestra di esposizione, *«its exposure window is at most `max_weak_subjectivity_age_ms` and it then fails closed»* — resta nella prosa ma **è uscito dalla cella di tabella**, che ora legge solo «fail-closed di flotta». **L'operatore che usa la riga verbatim scriverà una banda credendo di aver chiuso ciò che la banda non può chiudere, e avrà perso di vista ciò che la banda chiuderebbe davvero.**

**Scenario.** L'ADR fissa la banda. Il quorum pubblica un `consensus_parameters` con un valore **in banda** e diverso da quello nel checkpoint firmato. Ogni light client conforme in possesso di un header autenticato fallisce chiuso. **Un documento, costo zero, banda rispettata.**

**La chiusura ha già un modello nel protocollo**: l'arma si governa sulla **variazione** e non sull'estremo, e `ledger.md` impone già *«A network MUST publish a fresh checkpoint on any validator revocation»*.

NF-03 | category=correctness | severity=medium | criterion=righe 5 e 6 | remediation=relazione su una costante di genesi, oppure dichiarare che il rapporto è linea guida e nominare a parte il predicato accettabile
**La relazione unica di RF-006 non è un predicato calcolabile su un documento, ed è il difetto che la correzione di RF-005 diagnostica due sezioni più su.** `N_peers` **non è una grandezza di protocollo**: zero occorrenze di `max_peers`, `peer_count`, `target_peers`, `connection_limit` in `docs/protocol/` e in `core/coblox-core/src/`, enumerazione eseguita dalla reviewer. Il paragrafo su `max_weak_subjectivity_age_ms` scrive, giustamente, che *«durata wall-clock attesa»* non è un predicato calcolabile e per questo il MUST non morde. **La relazione delle cache ha la stessa forma.** Terza occorrenza in questa sessione della correzione che introduce ciò che correggeva.

NF-04 | category=security | severity=medium | criterion=riga 9 | remediation=il predicato lega entrambe le specie all'accettazione, oppure un tetto di genesi su `billing_epoch_ms`; sciogliere l'«oppure»
**Il prodotto di RF-001 non ha un luogo di controllo, e senza quello l'attacco a due passi sopravvive parola per parola.** I quattro documenti firmati attivano in modo **indipendente**, ciascuno col proprio `activation_height` e la propria sequenza. In `params.rs` **non esiste alcun tipo per `HostingRateCardBody`**. Se l'ADR appendesse il prodotto alla sola accettazione di `consensus_parameters`, il secondo passo dello scenario — pubblicare `hosting_rate_card` **dopo** — lo eluderebbe senza toccare la banda. **Un «oppure» non è una decisione, e questa riga va nell'ADR.**

NF-05 | category=correctness | severity=medium | criterion=riga 2 | remediation=aggiungere il termine relazionale con le due cache, o riscrivere la cella del rischio massimo su ciò che il solo tetto governa
**La riga 2 resta «magnitudine» mentre il suo stesso danno massimo è una proprietà della cache.** La reviewer l'aveva giudicata incompleta nella prima passata **senza attaccarle un RF**, e infatti non è stata toccata. La saturazione dipende dal prodotto fra durata di ritenzione e tasso di inserimento contro `replay_cache_entries_*`, non dalla sola durata: un tetto su `max_envelope_validity_ms` non chiude una saturazione che il quorum ottiene **abbassando le due cache**.

NF-06 | category=correctness | severity=low | criterion=riga 1, quinto punto | remediation=correggere il valore
**«A `max_clock_drift_ms = 0` (il valore oggi in albero)»: il valore in albero è `1`.** Verificato dal Lead: `protocol_hashes.py` e `tests/common/mod.rs` in due punti. **Contraddice la premessa dello stesso documento**, che dice «tutti a `1`». È il **quinto numero non guardato di questa sessione**, ed è nella sezione aggiunta per rispondere alla domanda sul pavimento.

NF-07 | category=documentation | severity=low | criterion=riga d'apertura | remediation=dichiarare che i gruppi si sovrappongono, o partizionare davvero
L'apertura riscritta presenta tre famiglie come una **partizione**, ma si sovrappongono: `min_revocation_effective_delay_blocks` sta nella banda di revoca **e** è il decimo dei dieci. `10 + 3 + 10 = 23` contro uno schema di **22**. **La riga era stata riscritta apposta per smettere di contare, e ora invita a contare male.**

NF-08 | category=documentation | severity=low | criterion=riga 6, quinto punto | remediation=rispondere alla domanda
Il quinto punto della riga 6 risponde alla domanda sul pavimento **con lo stato di review della riga** — *«nessun rilievo di [REVIEW-038] la tocca»* — prima di rispondere. **Lo stato di review non è una risposta a una domanda di liveness.**

NF-09 | category=correctness | severity=low | criterion=riga 8 | remediation=marcare l'accecamento come deduzione
Il passo del light client **contiene** entrambe le cose e **non dichiara alcun legame**: *«Peer agreement is an availability/fork alarm»* qualifica l'accordo fra peer, non la soglia. **L'accecamento è una deduzione dell'analisi presentata come lettura della fonte**, e l'ADR eredita la frase.

NF-10 | category=documentation | severity=low | criterion=riga 10 | remediation=allineare la prosa alla cella
`min_revocation_effective_delay_blocks >= 1` è chiamato «vincolo di genesi» nella prosa. Sta in `check_relations`; il commento del codice riserva la provenienza di genesi a `check_magnitudes`. **La cella di tabella è precisa, la prosa no.**

## La tabella tassonomica, giudizio aggiornato

| # | parametro | giudizio della seconda passata |
| --- | --- | --- |
| 1 | `max_clock_drift_ms` | **Da riclassificare ancora** (NF-01) |
| 2 | `max_envelope_validity_ms` | Verso giusto, ancora incompleta (NF-05) |
| 3 | `max_transport_attestation_validity_ms` | **Utilizzabile così** |
| 4 | `max_transport_attestation_future_skew_ms` | **Utilizzabile così — la correzione meglio riuscita** |
| 5 | `replay_cache_entries_per_peer` | Utilizzabile con una correzione (NF-03) |
| 6 | `replay_cache_entries_global` | Utilizzabile con la stessa correzione (NF-03, NF-08) |
| 7 | `max_weak_subjectivity_age_ms` | **Da non lasciar passare — la riga peggiore di questa passata** (NF-02) |
| 8 | `max_current_balance_age_ms` | **Utilizzabile così**, con una deduzione da marcare (NF-09) |
| 9 | `app_suspension_notice_epochs` | Grandezza giusta, forma no (NF-04) |
| 10 | `min_revocation_effective_delay_blocks` | **Utilizzabile senza riserve — la sola che attraversa due passate intatta** |

**Quattro utilizzabili così** (3, 4, 8, 10) contro tre della prima passata; quattro con un termine da correggere (2, 5, 6, 9); **due da riclassificare** (1, 7), di cui la 7 **non lo era prima**.

## Cosa la reviewer ha attaccato senza riuscire a romperlo

1. **Lo strato di citazione, interamente e con strumento proprio.** Ha scritto un risolutore che normalizza gli spazi, spoglia l'enfasi Markdown e verifica la **tripla** *(documento, sezione, frase)* — non la sola frase. **37 su 37 risolvono, e tutte e 37 stanno nella sezione dichiarata.** Zero disallineamenti di sezione, che era l'attacco preparato: una frase che risolve nel documento giusto ma nella sezione sbagliata. *Perimetro: presenza e sezione, non significato.*
2. **Che `billing_epoch_ms` sia davvero senza vincolo**: due sole occorrenze in `docs/protocol/`, nessuna regola, nessun `validate()`, e **nessun tipo `HostingRateCard` in `params.rs`**.
3. **Il superlativo sulla riga 10**, enumerando per intero `check_relations` e `check_magnitudes`: dei dieci parametri operativi **solo** `min_revocation_effective_delay_blocks` vi compare. Regge.
4. **L'aritmetica del 2 500** contro l'intervallo di genesi dichiarato. Regge.
5. **Il conteggio «otto dei dieci al pavimento»**: righe 1–8 falliscono a valore `1`, la 9 è liveness applicativa, la 10 ha il pavimento già soddisfatto. Coerente.
6. **La tesi che RF-001 fosse stato spostato di un anello** — la domanda che il Lead riteneva più affilata. **Non regge**, e la ragione è scritta nella disposizione sopra.

## Cosa la reviewer non ha guardato

- **Non ha rieseguito la suite né alcuna gate di progetto.**
- **RF-003 / [DEBT-037]**: fuori scopo dichiarato.
- **Lo strumento, la lista DRAFT, la sezione *Existing-project analysis* di [SPEC-023]**, anch'essa toccata dalla seconda remediation.
- **`.lmbrain/knowledge/threat-model.md`**: non letto; TM-37 verificato solo attraverso la fonte di protocollo.
- **[ADR-010], [ADR-013], [ADR-015], [ADR-016]**: non letti per intero.
- **La fedeltà di significato di venticinque citazioni su trentasette.** Ha verificato **presenza e sezione per tutte e 37** e letto il contesto per circa dodici, quelle su cui poggia un finding. Sulle altre venticinque **la verifica dice che la frase esiste dove l'analisi dice, non che dica ciò che l'analisi le fa dire.**
- **Nessun attacco reale eseguito.** Ogni scenario è derivato da regole lette e da aritmetica su costanti dichiarate.

## Required follow-up

- **NF-01 e NF-02 prima di qualunque ADR.** Sono le due righe che l'operatore userebbe verbatim, e entrambe scriverebbero una banda che non chiude ciò che dicono di chiudere.
- **NF-03, NF-04, NF-05** nello stesso giro: sono i tre termini mancanti delle righe altrimenti utilizzabili.
- **NF-06 va corretto e contato**: è il quinto numero non guardato della sessione.
- **Il risolutore della reviewer conferma la decisione aperta in [SPEC-024]**: spogliando l'enfasi Markdown le 37 frasi risolvono tutte, comprese le due che il risolutore del Lead rifiutava. È evidenza a favore della normalizzazione, e va portata in quella spec.
