---
id: REVIEW-010
# Note: Quote the title if it contains a colon
title: "Security review di SPEC-006 — GATE-SECREVIEW sulla regola di elezione e rotazione"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-006
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-010-EVENT-001"
    timestamp: "2026-08-25T13:23:10.243691900+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-002"
    timestamp: "2026-08-25T13:27:45.398447900+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Security review di SPEC-006 per GATE-SECREVIEW. La regola e la sua architettura a due strati sono corrette e vanno conservate, ma l'auto-perpetuazione di TM-18 rientra da due porte che nessuna regola chiude, e in entrambi i casi la catena resta formalmente valida e il light client non vede nulla di irregolare. RF-001 critico: i parametri di elezione vengono dal documento consensus_parameters, che e firmato dal quorum in carica; il blocco di vincoli introdotto dalla spec vincola le relazioni fra i parametri e non le magnitudini, quindi un documento con election_epoch_blocks 2^60 e validator_max_consecutive_terms 2^60 supera ogni vincolo, nessun confine di elezione arriva mai, nessun mandato scade mai, e la regola di confine impone al set di non cambiare: set congelato per sempre, ogni blocco valido, ogni controllo del light client superato. SEC-REQ-14 e ancora aperto e questa dipendenza non e registrata. RF-002 critico: il tetto di rotazione limita le ammissioni e nessuna regola limita le uscite; una coalizione con k maggiore di V/3, quindi sotto la soglia di safety BFT, censura le candidature altrui fino a candidacy_close, e al confine la derivazione onesta produce R uguale alla coalizione, pool vuoto, fills zero, member_count k: se k e almeno validator_min_set_size il set e valido e la coalizione detiene il 100 per cento del potere in un solo confine, senza usare un solo ingresso. Il fixture consensus PD-0 usa validator_min_set_size 1 con V 6. Ne discende che l'affermazione ceil((V/3)/c) confini in ledger.md e nella valutazione di AT-10, quella che restringere il set raggiunge lo stallo invece di un set a scelta dell'attaccante, e l'invariante 8 che dice che il quorum non sceglie i membri, promettono piu di quanto le regole impongano. RF-003 alto: l'affermazione che il macinatore non controlli l'insieme dei candidati e falsa, perche candidate_root e candidate_count entrano nel seme e il set uscente ne controlla la composizione ritardando l'inclusione delle candidature; l'esclusione per non inclusione non e falsificabile in modo compatto, a differenza del caso (a). RF-004 alto: contribution_score somma solo i passed senza diversita di emittente e senza che i failed contino, quindi la copertura a due emittenti, che e la mitigazione dichiarata della macinatura delle challenge, non mitiga una somma di successi: una coppia collusa emittente-soggetto raggiunge la soglia di eleggibilita senza conservare gli oggetti, e la frase di identity.md sul lavoro che non puo essere falsificato senza spendere risorse reali e sovrastimata. RF-005 medio: il cooldown si applica solo alla scadenza del mandato, quindi un'uscita volontaria un'epoca prima lo evita e azzera seated_since_epoch; per ogni cooldown maggiore o uguale a 2 la scadenza e una strategia dominata e l'assenza forzata effettiva e di una sola epoca. RF-006 basso: il passo 5 dell'algoritmo del light client usa parametri che nessun passo precedente gli procura o autentica. Attaccate senza trovare nulla e registrate come tali: la grindabilita del seme e le tre ragioni di indipendenza dell'invariante, il residuo (b) e l'asimmetria di falsificabilita, la regola solo rimozione compresa l'impossibilita di resettare i mandati attraverso di essa, il divieto di sospendere l'elezione per mancanza di seme, il rifiuto della continuazione del set precedente, epoca 1 e finestra di entropia, parita, doppia candidatura. La remediation di RF-001 di REVIEW-009 su identity.md e verificata corretta e confinata. Bloccanti per la gate: RF-001 e RF-002. Richiesti nella stessa remediation: RF-003, RF-004, RF-005, RF-006, piu gli aggiornamenti a threat-model.md su TM-18 condizionato, SEC-REQ-13 dipendente da SEC-REQ-14, TM-09, AT-09 e due configurazioni nuove di AT-10. Nessun debito nuovo: la chiusura del set non e reversibile a posteriori e una devnet sotto RF-001 o RF-002 e nella condizione che DEBT-005 vieta."
    evidence_refs: ["SPEC-006", "DEBT-005", "REVIEW-009", "docs/protocol/ledger.md", "docs/protocol/README.md", "docs/protocol/identity.md", ".lmbrain/knowledge/threat-model.md"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-003"
    timestamp: "2026-08-25T13:45:41.753101+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Tutti e sei i finding di REVIEW-010 chiusi; la derivazione non e stata rifatta e l'architettura a due strati e conservata. RF-001: ElectionBounds nel trust anchor di genesi (README §Election bounds) con tetti e pavimenti di magnitudine e variazione massima per sequence consecutiva, fuori dalla governance della catena; il blocco di vincoli di ledger.md li applica in accettazione. RF-002: pavimento di contrazione 3*member_count(nuovo) > 2*member_count(precedente), forma del predicato di quorum applicata ai seggi, valido ai confini e nelle transizioni di sola rimozione; la coalizione a k appena sopra V/3 ora produce un set invalido e la catena si ferma, quindi la soglia effettiva di cattura torna a 2/3. Corrette insieme alla regola le tre affermazioni che la presupponevano: ceil((V/3)/c) qualificata \"per ammissione\" in ledger.md e in AT-10, la frase sulla contrazione in §Revocation con la conversione in concentrazione nominata, e l'invariante 8. RF-003: affermazione \"insieme impegnato che il macinatore non controlla\" ritrattata e conservata solo come citazione; presa anche la riduzione, candidate_root e candidate_count escono dal preimmagine del seme e restano legati per validita; dichiarata l'esclusione per non finalizzazione come residuo non falsificabile da nessun verificatore. RF-004: presa la variante (i) oltre alla (ii), verificato che non tocca ADR-002 perche e una condizione di conteggio su issuer_node_id gia presente nell'evidenza finalizzata, quindi nessun debito proposto; nuova condizione di eleggibilita con validator_eligibility_min_issuers emittenti distinti, dichiarato che alza il prezzo e non chiude il residuo, spiegato perche la copertura a due emittenti non trasferisce a una somma di successi; corretta la frase del punto 3 di identity.md e ripuntati i rimandi. RF-005: variante (i), il cooldown copre l'uscita da un seggio per qualunque ragione, con l'aritmetica dell'evasione riportata e l'interazione con il pavimento coperta nei casi degeneri. RF-006: il passo 5 nomina le tre fonti nell'ordine e dichiara il fallimento chiuso, senza default. Aggiunta la forma difendibile del claim con la qualificazione sui limiti di genesi e sulla distinzione entra/esce. threat-model.md aggiornato secondo le indicazioni di AGENT-007: TM-18 con le due porte e la loro chiusura piu i residui (iv) e (v), SEC-REQ-13 con la dipendenza da SEC-REQ-14, TM-09 con il residuo di RF-004, AT-09 con la conversione in concentrazione, AT-10 con la disuguaglianza qualificata e tre configurazioni di test. Verifica: metodo JCS rivalidato su due fixture non modificate prima di ricalcolare policy_hash PD-0; consensus_parameters_hash non cambia; ricalcolati election_seed e i tre biglietti, con l'esito che l'esempio ora insedia 06 e 08 e lascia fuori 05 per il tetto; derivazione rifatta in modo indipendente; controprova del pavimento sull'esempio e su cinque coppie (V,k); 70 controlli superati e 0 falliti, 0 ancore interne rotte. Spec rimasta in review, spec_start non chiamato. Nessun commit e nessun push."
    evidence_refs: ["SPEC-006", "DEBT-005", "REVIEW-010"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-004"
    timestamp: "2026-08-25T14:00:35.194999200+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verifica della remediation dei sei finding di REVIEW-010 da parte di AGENT-007. Cinque su sei sono chiusi nel merito; RF-002 e chiuso nella regola e non nell'affermazione che la accompagna. RF-001 chiuso: ElectionBounds nel trust anchor di genesi regge al mio stesso attacco, perche il principale che firma la distribuzione non e il quorum e nessun soggetto fidato nuovo viene introdotto, essendo la distribuzione firmata gia radice di fiducia di genesi, chiave di fiducia e checkpoint; la proprieta rivendicata nel Declared limit, che nessuna delle due direzioni permette a un attaccante di allargare i limiti che un dato client applica, e piu stretta di infalsificabilita ed e vera. RF-002 chiuso nella regola: il pavimento 3*member_count(new) > 2*member_count(old) e la risposta giusta e la scelta di riusare la forma del predicato di quorum invece di un departure_cap governato e corretta e per la ragione corretta, perche un parametro in piu sarebbe stato un parametro in piu da vincolare in ElectionBounds; verificato che il pavimento si applichi davvero anche alle transizioni di sola rimozione come regola 10. RF-003 chiuso con piu di quanto imposto: la rimozione di candidate_root e candidate_count dalla preimmagine del seme non ha effetti collaterali sostanziali, il legame all'epoca e alla catena resta, il legame dei due valori resta per validita che e un controllo piu forte, e l'argomento della scelta cieca del sottoinsieme, dovuto all'ordinamento di candidacy_close sotto la finestra di entropia, e corretto e non era mio; il residuo (h), esclusione per non finalizzazione invisibile a ogni verificatore, e la forma corretta. RF-004 chiuso e l'affermazione che non tocchi ADR-002 e vera, verificata e non accettata: issuer_node_id e gia campo di ChallengeEvidenceBody in ogni evidenza finalizzata, la condizione 4 e un conteggio di valori distinti su dati gia posseduti dal verificatore e non tocca funzione di assegnazione ne finestra di risposta; min_issuers >= 2 combacia con la copertura a due emittenti di wire.md quindi non esclude nodi onesti; la formulazione costosa da falsificare, non impossibile, con prezzo lineare e bound ricondotto ad alpha, e quella richiesta, e la correzione di identity.md che cita la vecchia formulazione e la dichiara piu forte di quanto il protocollo consegni e il modo giusto di ritrattare. RF-005 chiuso con la variante (i). RF-006 chiuso oltre la richiesta, con le tre fonti nominate in ordine e il fallimento chiuso senza default. Ricalcolati in modo indipendente i due valori che la remediation ha cambiato, election_seed con la nuova preimmagine e i tre election_ticket: coincidono con i byte scritti, e l'ordine 06, 08, 05 e quello della tabella con 05 escluso dal tetto. Sulla domanda di liveness sollevata da AGENT-002: l'argomento e corretto per il caso di guasto, perche i membri caduti contano ancora in total_power e la catena e gia ferma, ma non copre il caso dei membri vivi e non mantenuti, cioe uscita volontaria di massa, caduta collettiva sotto la soglia di contributo, e scadenza simultanea dei mandati. Threat model verificato: TM-18 con le due porte registrate come TM-18 a tutti gli effetti e la condizione esplicita su ElectionBounds, SEC-REQ-13 con la dipendenza da SEC-REQ-14 e la distinzione fra parametri di elezione e altri parametri governati, TM-09 con il residuo di RF-004, AT-09 con la conversione in concentrazione, AT-10 con la qualificazione per ammissione e tre configurazioni: tutti nella forma che avevo indicato. La verifica non chiude la gate: quattro finding nuovi emergono dalla remediation stessa e sono registrati come RF-007, RF-008, RF-009, RF-010 e RF-011 nel giro 2 della review."
    evidence_refs: ["SPEC-006", "DEBT-005", "docs/protocol/ledger.md", "docs/protocol/README.md", "docs/protocol/identity.md", ".lmbrain/knowledge/threat-model.md"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-005"
    timestamp: "2026-08-25T14:01:18.484256900+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Giro 2. Cinque dei sei finding originari sono chiusi nel merito e RF-002 e chiuso nella regola ma non nell'affermazione; la remediation ne ha aperti quattro nuovi, tre dei quali sono conseguenza diretta delle correzioni. GATE-SECREVIEW non e ancora soddisfatta. RF-007 critico, bloccante: la coorte di genesi scade tutta insieme e il pavimento di contrazione trasforma quel fatto in un arresto certo. Le voci del set di genesi portano seated_since_epoch zero per regola di ledger.md, il ritiro richiede e - seated_since_epoch < T, quindi al confine e = T nessun membro di genesi e mantenuto, R e vuoto, fills e limitato da validator_churn_cap_seats e il vincolo 3*c*m <= V con m >= 1 da c <= V/3, mentre il pavimento richiede 3*c > 2*V cioe c > 2V/3: incompatibili per ogni V positivo. Non esiste set valido al confine e = T e la catena si ferma su qualunque rete conforme a un'altezza nota in anticipo, T * election_epoch_blocks. Sui parametri del fixture PD-0 del progetto stesso, V=6 T=3 c=2 m=1: ai confini 1 e 2 i sei membri sono mantenuti e fills e zero perche il set e gia a target, al confine 3 tutti e sei hanno 3-0=3 che non e minore di 3, R e vuoto, fills e 2, e 3*2 > 2*6 e falso: arresto all'altezza 3*election_epoch_blocks, con ripresa solo fuori banda. Prima del pavimento lo stesso confine produceva un set di due membri, pessimo ma vivo. Il caso non e coperto da Degenerate cases, che tratta la scadenza sincronizzata come tensione di taratura mentre qui e una certezza indipendente dalla taratura, ed e proprio l'interazione fra limite di mandato, tetto di ingressi e pavimento che nessuna delle tre regole vede da sola. Chiusura: coorte di genesi sfalsata su [0, T-1] con non piu di c voci per valore, oppure esclusione dal pavimento della riduzione imputabile a scadenza di mandato, che un light client calcola dai soli seated_since_epoch; fixture che esegua i confini 1..T dal set di genesi PD-0 producendo un set valido a ogni confine. RF-008 alto, bloccante: l'affermazione in grassetto che la soglia effettiva di cattura sia due terzi e non un terzo piu epsilon e falsa, e il documento calcola da se il numero che la falsifica due paragrafi sotto. Una coalizione a k appena sopra V/3 censura selettivamente, lasciando finalizzare solo le candidature oneste necessarie a portare il set al minimo che il pavimento consente: V scende a 2V/3 e la coalizione ne e la meta, poi a 4V/9 e ne e tre quarti, poi contrae fino a se stessa. Tre confini. I nodi onesti firmano la propria rimozione perche il blocco e valido, la derivazione e deterministica e le candidature censurate non sono mai state finalizzate; e non potrebbero nemmeno accorgersene, perche il residuo (g) dello stesso documento dichiara che una contrazione lecita e una cattura per attrito sono indistinguibili. Il documento contiene la confutazione della propria affermazione a poche righe di distanza, dato che (g) chiude dicendo che il punto d'arrivo del processo e uno stallo. La frase compare in quattro punti: The contraction floor, Revocation forces a validator set transition, il residuo (g), e threat-model.md sia in TM-18 sia nell'esito atteso della configurazione 2 di AT-10, dove diventa un criterio di test sbagliato che in simulazione verra smentito e attribuito all'implementazione. Quello che il pavimento compra davvero, un confine invisibile convertito in tre confini ciascuno dei quali pubblica la propria contrazione in un documento firmato, e lo stesso standard del tetto di ingressi e va rivendicato; non e lo standard piu forte che le quattro frasi gli attribuiscono. Va dichiarata anche l'asimmetria: l'orizzonte della cattura per ammissione e tarabile con m, quello per attrito e fisso a ceil(log(V/k)/log(3/2)) e non dipende da m, e la sicurezza di una regola e quella del suo percorso piu debole. Chiusura: le quattro occorrenze corrette, l'esito atteso di AT-10 configurazione 2 allineato, e l'asimmetria dichiarata oppure chiusa misurando il pavimento contro il set di m confini prima, 3*member_count(e) > 2*member_count(e-m), senza parametri nuovi. RF-009 medio: il tetto alla variazione dei parametri e per documento e non per tempo, e nulla impone una distanza minima fra sequence consecutive, quindi un quorum porta un parametro al tetto di genesi in altrettanti blocchi consecutivi; il tetto assoluto regge e RF-001 resta chiuso, ma la proprieta rivendicata di convertire un evento in un processo osservabile non e raggiunta, e la riga di SEC-REQ-13 sovrastima la dipendenza soddisfatta, perche SEC-REQ-14 chiede variazione massima per epoca e ritardo di attivazione dichiarato, e nessuno dei due c'e. Chiusura: attivazione a un confine di elezione e al piu una modifica per epoca, oppure la dichiarazione che il tetto limita il numero di documenti e non il tempo. RF-010 medio: il rimedio a RF-005 ha moltiplicato la leva di censura, perche censurare la candidatura di un onesto per un'epoca ora lo tiene fuori 1 + validator_cooldown_epochs epoche, e validator_cooldown_epochs non e fra le magnitudini vincolate da ElectionBounds; verificato che le altre grandezze non elencate siano innocue rispetto all'invariante, perche portano allo stallo e non al congelamento, mentre il cooldown e l'unica il cui aumento aiuta un avversario. Una coalizione sopra i due terzi porta il cooldown al massimo e poi esclude un onesto per confine, dentro il pavimento, rendendo permanente ogni esclusione mentre riempie i seggi con le proprie identita; non e un percorso di cattura nuovo ma rende irreversibile l'esclusione degli onesti lungo il percorso gia dichiarato, e il residuo di numerosita governato da alpha e misurato assumendo che un onesto escluso possa rientrare. Chiusura: un tetto al cooldown, anche solo la relazione validator_cooldown_epochs <= validator_max_consecutive_terms che non costa nulla perche T e gia vincolato, piu la dichiarazione dell'amplificazione. RF-011 medio: il controllo 7 della lista normativa del light client descrive ancora candidate_root e candidate_count come input del seme, che dopo RF-003 non lo sono; il passo 5 rimanda a quella lista per nome, quindi un'implementazione conforme calcola un seme diverso da quello di ogni set valido e rifiuta l'intera catena. E l'unico effetto collaterale testuale rimasto: formula, derivazione, registro di README, fixture ELEC-0 ed esempio numerico sono coerenti e verificati uno per uno. Bloccanti: RF-007 e RF-008. Richiesti nella stessa remediation: RF-009, RF-010, RF-011. Nessun commit e nessun push; spec_done non chiamato."
    evidence_refs: ["SPEC-006", "DEBT-005", "docs/protocol/ledger.md", "docs/protocol/README.md", ".lmbrain/knowledge/threat-model.md"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-006"
    timestamp: "2026-08-25T14:14:34.582410500+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Secondo giro: RF-007, RF-008, RF-009, RF-010 e RF-011 chiusi. Due dei quattro sono conseguenze delle correzioni del primo giro e sono registrati come tali. RF-007: limite di mandato, tetto di ingressi e pavimento di contrazione erano congiuntamente insoddisfacibili al confine e=T su ogni rete conforme, perche il set di genesi porta mandati sincroni; arresto certo verificato in simulazione sui parametri del fixture. Rifiutata l'esenzione del pavimento a R vuoto perche fabbricabile da chi censura, e corretta la causa: ogni voce porta term_expiry_epoch timbrato all'insediamento e mai ricalcolato, il set di genesi deve essere scaglionato (valori in [1,T], al piu validator_churn_cap_seats voci per valore, altrimenti non e un trust anchor valido), e il blocco di validita acquista 3*c < V perche un confine in cui scade un'intera coorte e non viene insediato nessuno produca comunque un set valido. Il timbro chiude anche l'estensione retroattiva dei mandati da parte di un quorum che alzi T. Vincoli resi congiuntamente soddisfacibili nel blocco: ceil(V/T) <= c < V/3 richiede T >= 4, con T >= 3m si ottiene T >= max(4, 3m), quindi T <= 3 e insoddisfacibile a ogni V, verificato per forza bruta; il fixture PD-0 usava T=3 ed era esso stesso inammissibile, sostituito con V=12 T=4 c=3 e consensus_parameters_hash ricalcolato. RF-008: affermazione sui due terzi ritratta in tutti e quattro i punti, incluso l'esito atteso della configurazione 2 di AT-10, ora spezzata in 2a censura totale e 2b censura selettiva con gli esiti corretti; la soglia effettiva resta poco sopra un terzo e il documento rivendica cio che il pavimento compra davvero, un confine invisibile convertito in tre ognuno dei quali pubblica la propria contrazione, con l'asimmetria dichiarata fra orizzonte per ammissione tarabile e orizzonte per attrito fisso. RF-009: election_parameter_min_activation_gap_blocks in ElectionBounds piu la regola di spaziatura sulle activation_height. RF-010: validator_cooldown_epochs <= T nel blocco, con la ragione registrata. RF-011: controllo 7 della lista normativa del light client corretto. Verifica: metodo rivalidato su parameter_set_hash e hosting_rate_card_hash non toccati prima di ricalcolare consensus_parameters_hash; policy_hash e i valori ELEC-0 invariati; l'esempio numerico conserva tutti i digest perche il seme dipende dalla sola finestra di entropia. Due verifiche nuove sono simulazioni e non controlli testuali: la successione dei set su 30 confini, che si ferma al confine 4 con genesi sincrona e non si ferma mai con genesi scaglionata su quattro combinazioni ammissibili; e l'orizzonte di attrito calcolato applicando ripetutamente il pavimento, che conferma che la contrazione riesce sotto i due terzi in tre o quattro confini. 99 controlli superati e 0 falliti, 0 ancore interne rotte. Spec rimasta in review. Nessun commit e nessun push."
    evidence_refs: ["SPEC-006", "DEBT-005", "REVIEW-010"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-007"
    timestamp: "2026-08-25T14:24:01.314699800+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verifica della remediation dei quattro finding del giro 2 da parte di AGENT-007. Tutti e quattro chiusi, due meglio di come li avevo chiesti. RF-007 chiuso correggendo la sincronizzazione invece di indebolire il pavimento: il rifiuto dell'esenzione su R vuoto e motivato con il precedente giusto, cioe che un'esenzione vale quanto la difficolta di fabbricarne il trigger e questa era gratuita, e il vincolo aggiunto 3c < V e la condizione giusta e non l'avevo isolata io. Verificata la proprieta di automantenimento delle scadenze, che era la domanda posta: e corretta a T costante, e ho trovato la sola porta che la apre, il decremento di T, registrata come RF-012. Verificati come privi di percorsi di risincronizzazione la genesi verso i riempimenti, perche i timbri di genesi stanno in [1, T] e il primo riempimento timbra 1+T, le transizioni di sola rimozione, perche la regola 7 impone l'identita voce per voce compreso term_expiry_epoch, e il rientro di un membro uscito, che passa per cooldown ed estrazione. La chiusura dell'estensione retroattiva dei mandati, difetto che nessuno dei due aveva sollevato, e completa: verificata su tutti e quattro i punti in cui poteva perdere, cioe il passo 1 che impone al mantenuto di conservare il timbro invariato, il passo 6 che lo calcola sui parametri attivi al confine di insediamento, la regola 7 delle transizioni di sola rimozione che lo preserva, e il controllo 5 del light client che verifica entrambi i valori attraverso due set adiacenti. RF-008 chiuso con una ritrattazione completa che dichiara l'errore, cita la propria confutazione, esibisce la successione con il numero di confini, dice che i nodi onesti firmano quei blocchi perche sono validi, e conclude che la soglia effettiva resta appena sopra un terzo; l'asimmetria fra orizzonte tarabile e fisso e dichiarata, la frase di Revocation e corretta con in un solo passo, e il criterio di test di AT-10 e sdoppiato in censura totale e selettiva con la nota che un criterio errato verrebbe attribuito all'implementazione. RF-009 chiuso e lo spaziamento e davvero per unita di catena, in altezze e non in numero di documenti; verificato il punto che lo avrebbe reso inutile, cioe che election_parameter_min_activation_gap_blocks sta in ElectionBounds alla genesi e non fra i parametri di consenso, altrimenti un quorum lo avrebbe azzerato tornando a RF-001. RF-010 chiuso con validator_cooldown_epochs <= T e la motivazione scritta per esteso. RF-011 chiuso, il controllo 7 nomina gli ID di blocco di entropia e nient'altro. Sul rilievo di AGENT-002 riguardo alle suite di conformita: ha ragione e vale oltre l'hash, perche una suite scritta contro T=3 codificava un insieme di parametri che il blocco ora rifiuta, cioe uno stato che una rete conforme non puo raggiungere, e va portato nella spec di implementazione di M-02 come nota. La verifica non chiude la gate: un finding nuovo, RF-012, e il ritorno di RF-007 dalla porta dei parametri, piu tre rilievi di severita bassa registrati come RF-013, RF-014 e RF-015 nel giro 3 della review."
    evidence_refs: ["SPEC-006", "DEBT-005", "docs/protocol/ledger.md", "docs/protocol/README.md", ".lmbrain/knowledge/threat-model.md"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-008"
    timestamp: "2026-08-25T14:24:32.793732300+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Giro 3. I quattro finding del giro 2 sono chiusi, due meglio di come li avevo chiesti. Blocca un solo finding nuovo, RF-012, che e il ritorno di RF-007 dalla porta dei parametri, piu tre rilievi di severita bassa che vanno nella stessa remediation perche sono correzioni di poche righe. RF-012, alto, bloccante: abbassare validator_max_consecutive_terms risincronizza le coorti, e il documento afferma senza condizione che la proprieta si mantiene da se senza altre regole. Un seggio riempito al confine e e timbrato e + T(e) con T attivo a quel confine, quindi per e1 < e2 due coorti scadono insieme quando e1 + T(e1) = e2 + T(e2), cioe esattamente quando T diminuisce: le collisioni esistono se e solo se T decresce, e su quell'affermazione poggia il vincolo 3c < V, dimensionato su una sola coorte per confine. Scenario senza avversario e con parametri ammissibili: V=12, c=3, m=1, T=6 soddisfa il blocco; l'operatore accorcia i mandati su indicazione del simulatore, T da 6 a 5 a 4, un passo per epoca, dentro il rapporto di variazione e dentro lo spaziamento minimo, e ogni documento resta valido perche ceil(12/5)=3<=3 e ceil(12/4)=3<=3; i riempimenti dei confini e, e+1 ed e+2 sono timbrati tutti e tre e+6, quindi fino a nove voci su dodici scadono al confine e+6, R vale 3, fills vale 3, il set nuovo ha sei membri e il pavimento richiede 18 > 24, falso: nessun set valido esiste e la catena si ferma al confine e+6 con ripresa solo fuori banda. Due aggravanti rispetto a RF-007: il pool pieno non salva, perche a fermare la ricostruzione non e la mancanza di candidati ma il tetto di ingressi, che rimpiazza tre dei nove seggi liberati; e non serve alcun avversario, perche il documento che abbassa T richiede la firma di un quorum e quindi un attaccante non guadagna nulla che non abbia gia, ma l'operatore onesto che accorcia i mandati dopo la taratura del simulatore compie un'operazione ovvia e permessa e non riceve alcun avvertimento, e il guasto arriva T confini dopo a un'altezza calcolabile. Chiusura, una riga nel blocco di accettazione, in una delle due forme: (i) T monotono non decrescente su una catena viva, sufficiente perche se T non decresce allora e1 + T(e1) < e2 + T(e2) per e1 < e2, quindi i timbri di confini distinti non collidono mai e le scadenze a ogni confine vengono da una sola coorte e sono al piu c, mentre i timbri di genesi stanno in [1, T] e sono strettamente sotto il primo timbro di riempimento; il costo e nullo vista la forma timbrata, dove allungare T non estende i mandati in corso; oppure (ii) una riduzione di T e accettata solo se e + T_new e maggiore del massimo term_expiry_epoch del set attivo, deterministica e calcolabile anche da un light client, che conserva la possibilita di accorciare i mandati dopo che i timbri precedenti si sono esauriti. In entrambi i casi la frase sull'automantenimento diventa vera e puo restare citando la regola che la rende tale. RF-013, basso: il residuo (g) descrive solo la censura totale, che il pavimento rifiuta davvero, quindi preso alla lettera e vero, ma e l'unica variante nominata nell'elenco dei residui mentre il vettore reale, stabilito sia da What the floor does not buy sia dalla configurazione 2b di AT-10, e la censura selettiva; la distinzione poggia tutta sulla parola ogni. RF-014, basso: la motivazione data per chiudere RF-008 con una ritrattazione invece che con una regola, cioe che una regola dovrebbe distinguere una candidatura censurata da una mai inviata, non regge, perche il pavimento cumulativo 3*member_count(e) > 2*member_count(e-m) non chiede mai perche un membro sia uscito, si calcola su member_count di set che il light client gia conserva, e lega l'orizzonte dell'attrito a m; la conclusione resta accettabile perche l'avevo autorizzata, ma il motivo vero e il costo in liveness e non l'impossibilita, e va registrato come alternativa considerata e scartata nella stessa forma gia usata per il commit-reveal dei candidati. RF-015, basso, rilievo del Lead verificato in modo indipendente e confermato: l'affermazione di README che T=4 con V=12 e c=3 sia la piu piccola istanza che soddisfa ogni vincolo e falsa, perche V=4, T=4, c=1, m=1 li soddisfa tutti, e V=3 e sotto sono impossibili perche 3c < 3 con c >= 1 non ha soluzione, quindi l'istanza minima e V=4, T=4, c=1, m=1; non e un difetto della regola ma e un'affermazione normativa falsa in un paragrafo il cui scopo dichiarato e insegnare la forma dello spazio dei parametri. RF-012 e l'ultima cosa sostanziale che ho: ho attaccato questa regola per tre giri e non ho altro. Nessun commit e nessun push; spec_done non chiamato."
    evidence_refs: ["SPEC-006", "DEBT-005", "docs/protocol/ledger.md", "docs/protocol/README.md"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-009"
    timestamp: "2026-08-25T14:33:12.276620200+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Terzo giro: RF-012, RF-014 e RF-015 chiusi. RF-012: il mio argomento di automantenimento valeva solo a T costante. I timbri sono e+T(e), quindi due confini distinti collidono se e solo se T diminuisce, e ogni collisione mette piu di una coorte sullo stesso confine, che e l'unica cosa per cui 3c < V non e dimensionato. Riprodotto in simulazione con V=12, c=3 e T sceso di un passo per confine da 12 a 4, ogni valore intermedio ammissibile: nove seggi su dodici scadono al confine 12, R=3, fills=3, member_count=6, e 3*6 > 2*12 e falso, senza avversario e con il pool pieno. Scelta la chiusura (i), monotonia: T_new >= T_active in accettazione, quindi su catena viva il limite di mandato non decresce mai. Non per semplicita: ho verificato che (ii) collassa su (i) se valutata in accettazione, perche fra accettazione e attivazione un seggio puo essere insediato al confine precedente con il T vecchio, quindi l'unica garanzia ottenibile e T_new >= T_old; (ii) ha effetto solo se valutata all'attivazione contro il set allora attivo, il che richiede un documento di protocollo ad attivazione condizionata dallo stato di catena, concetto che v0 non ha. La dimostrazione e scritta nel documento, insieme alla dichiarazione che si tratta di un rifiuto per costo e non di un'impossibilita, con il costo nominato: porta a senso unico su una grandezza rilevante per la sicurezza. Alzare T resta gratuito grazie al timbro. RF-014: motivazione corretta, decisione conservata. Il pavimento cumulativo 3*member_count(e) > 2*member_count(e-m) esiste, e sano, non chiede mai perche un membro sia uscito e quindi non richiede la distinzione che il residuo (h) dichiara impossibile, e si calcola su member_count che il light client gia conserva; e ora esposto per esteso nel documento e rifiutato per il suo costo in liveness, con la ragione esplicita per cui la distinzione conta: un costo e un giudizio rivedibile, un'impossibilita e una dimostrazione che dice al lettore successivo di smettere di cercare. RF-015: chiuso senza cambiare il fixture; la nota dice ora che ceil(V/T) <= c < V/3 e insoddisfacibile per T <= 3 a ogni V, che l'istanza minima ammissibile e V=4 T=4 c=1 m=1 e che V=3 e impossibile, e che il fixture ne usa una piu grande perche con c=1 il tetto di ingressi non verrebbe mai esercitato. Osservazione sulle suite di conformita portata in README §Hash conformance fixtures come paragrafo normativo: i fixture di parametri non sono scelte libere, un caso costruito su una combinazione vietata asserisce un comportamento per uno stato in cui nessuna rete conforme puo trovarsi e va rimosso invece che aggiustato. Clausola del claim aggiunta accanto e non dentro il paragrafo. Verifica: due simulazioni nuove, la successione dei set su 60 confini (la sequenza decrescente ammissibile si ferma al confine 12 con i numeri riportati, quelle non decrescenti mai) e l'enumerazione di tutte le collisioni di timbri che conferma che ogni collisione ha T2 < T1, quindi condizione necessaria e sufficiente; piu l'enumerazione di tutte le combinazioni (V,T,c,m) fino a 20 che conferma l'istanza minima. Il difetto di RF-007 resta riprodotto come non regressione. Nessun hash cambia in questo giro. 121 controlli superati e 0 falliti, 0 ancore interne rotte. Spec rimasta in review. Nessun commit e nessun push."
    evidence_refs: ["SPEC-006", "DEBT-005", "REVIEW-010"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-010"
    timestamp: "2026-08-25T14:42:14.095796700+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verifica della remediation del giro 3 da parte di AGENT-007. RF-012 chiuso con il limite di mandato monotono, e la dimostrazione che la forma permissiva (ii) collassa su (i) e stata rifatta e regge: all'accettazione i timbri in volo all'attivazione non sono noti perche fra le due altezze puo cadere un confine e una coorte insediata al confine e_a-1 e timbrata con il T vecchio, quindi la condizione conservativa e e_a + T_new > (e_a - 1) + T_old, cioe T_new >= T_old. Due precisazioni che la rafforzano: il collasso dipende dal fatto che un confine cada fra accettazione e attivazione, e se non ne cade nessuno un controllo esatto in accettazione sarebbe possibile, ma appoggiare una regola di sicurezza su quella eventualita sarebbe fragile e la regola monotona e sicura in entrambi i casi; e la regola permissiva che funzionerebbe davvero all'attivazione non e quella scritta, perche il confronto con il solo massimo e sufficiente ma inutilmente stretto, mentre la condizione esatta e che nessuno dei valori della finestra di sovrapposizione, lunga T_old meno T_new, sia un timbro occupato, dato che la collisione si ripresenta a ogni confine della finestra e non solo al primo. Il costo dichiarato della porta a senso unico e accettabile, perche il raggio del danno e limitato da validator_max_consecutive_terms_max che sta alla genesi e fuori dalla governance della catena, e perche il costo e dichiarato come rifiuto sul costo e non come impossibilita. Va aggiunta una conseguenza piu affilata: il cricchetto e spingibile da un avversario e non tirabile indietro da nessuno, quindi un quorum che raggiunga i due terzi anche una sola volta porta T al tetto di genesi in modo permanente e nessun quorum onesto successivo puo riportarlo giu; contro un avversario stabile sopra i due terzi non aggiunge nulla, ma contro un quorum transitorio o un errore dell'operatore lascia il pavimento di ricambio degradato per sempre, e da quel momento il tetto di genesi e l'unico presidio residuo, il che impone di sceglierlo stretto. RF-014 chiuso esattamente come chiesto, con l'esposizione corretta del pavimento cumulativo, il rifiuto motivato sul costo in liveness pagato dalle reti oneste, e la distinzione generale fra costo e impossibilita scritta meglio di come l'avevo formulata. RF-015 chiuso senza toccare il fixture e con la ragione migliore fra le due suggerite, ma la sostituzione introduce una seconda affermazione di minimalita a sua volta falsa, registrata come RF-016. Il riporto sulle suite di conformita e in Hash conformance fixtures, subito dopo l'obbligo di ricostruire ogni preimmagine, cioe dove un autore di suite guarda davvero, ed e normativo e impone la rimozione invece della correzione, che e il verbo giusto. Verifiche di regressione: l'esempio numerico e stato riportato dentro lo spazio ammissibile con V=8, T=4, c=2, m=1, verificato il blocco su quei valori, i timbri della tabella coerenti con la regola che li assegna, fills = min(max(0, 8-2), 2, 3) = 2 con il tetto che vincola, e il pavimento verificato insieme alla controprova dei due soli superstiti; era il punto in cui l'esempio poteva restare indietro, dato che T=3 e ora inammissibile a ogni V, e non e restato indietro. Il vincolo T_new >= T_active e nel blocco. Restano due rilievi di severita bassa, RF-013 non chiuso al giro 3 e RF-016 nuovo, nessuno dei due nella regola."
    evidence_refs: ["SPEC-006", "DEBT-005", "docs/protocol/ledger.md", "docs/protocol/README.md"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-011"
    timestamp: "2026-08-25T14:42:38.898659+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "GATE-SECREVIEW soddisfatta. La regola di elezione e rotazione, come superficie di sicurezza, tiene: attaccata per quattro giri e non ho altro. Le due porte critiche del giro 1, il documento dei parametri firmato dal quorum e la contrazione priva di pavimento, sono chiuse con regole e non con dichiarazioni: ElectionBounds nel trust anchor di genesi, fuori dalla governance della catena, e il pavimento di contrazione nella forma del predicato di quorum applicata ai seggi. Le due porte del giro 2, l'arresto deterministico della coorte di genesi e la soglia di cattura sovrastimata, sono chiuse correggendo la sincronizzazione invece di indebolire il pavimento, con il timbro term_expiry_epoch che chiude per soprammercato l'estensione retroattiva dei mandati, e ritrattando per intero l'affermazione sui due terzi in tutti i punti in cui compariva, criterio di test di AT-10 incluso. La porta del giro 3, la risincronizzazione delle coorti per riduzione di T, e chiusa con il limite di mandato monotono e con una dimostrazione di collasso della forma permissiva che ho verificato e che regge. Restano due rilievi di severita bassa, entrambi documentali e nessuno dei due nella regola: RF-013, il residuo (g) del light client che descrive ancora solo la censura totale, cioe la variante che il pavimento rifiuta, mentre il vettore reale e la selettiva, con la conseguenza che l'elenco dei residui e la sezione che lo cita come propria confutazione risultano discordi per chi segua il rimando; e RF-016, la seconda affermazione di minimalita in README, secondo cui V=12, T=4, c=3 sarebbe la piu piccola istanza che esercita tetto, scaglionamento e pavimento, mentre V=7, T=4, c=2, m=1 e ammissibile ed esercita tutte e tre, con il pavimento nel punto piu stretto che il blocco consenta. Nessuno dei due blocca, e sostenere il contrario sarebbe incoerente dopo aver scritto al giro 3 che rilievi di questa dimensione non giustificano un giro per conto proprio. Raccomando al Lead di farli applicare prima di spec_done, dato che AGENT-002 e gia nel ciclo e la correzione e di due frasi, oppure di registrarli come debito documentale, perche l'elenco dei residui e l'artefatto che un lettore futuro citera per sapere che cosa il protocollo non promette. Raccomando inoltre un debito separato per la porta a senso unico sul limite di mandato, che si risolve quando i documenti governati avranno un'attivazione condizionata allo stato della catena, unica condizione sotto la quale la regola permissiva nella forma a finestra diventa disponibile; e una nota nel threat model sul fatto che il cricchetto su T e spingibile da un avversario transitorio e non tirabile indietro da nessuno, il che rende validator_max_consecutive_terms_max l'unico presidio residuo sul ricambio e impone di sceglierlo stretto alla genesi. Il claim di sicurezza sulla composizione del set e ora difendibile senza dichiarazioni accanto: entro i limiti fissati alla genesi un light client stabilisce che il set e di forma lecita e in rotazione lecita e non che sia il set che la regola di eleggibilita avrebbe dovuto produrre, il limite alla cattura e un numero di confini pubblicati e non una quota di potere di voto, la soglia effettiva resta appena sopra un terzo, e delle cinque forme di composizione scorretta tre sono rilevabili da un nodo che rigioca la storia, due contraddicibili con un messaggio corto, e due da nessuno. Nessun commit e nessun push; spec_done spetta al Lead."
    evidence_refs: ["SPEC-006", "DEBT-005", "docs/protocol/ledger.md", "docs/protocol/README.md", "docs/protocol/identity.md", ".lmbrain/knowledge/threat-model.md"]
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-010-EVENT-012"
    timestamp: "2026-08-25T14:46:16.612503600+02:00"
    action: "remediation"
    from_status: "accepted"
    to_status: "accepted"
    actor_role: "implementation-specialist"
    reason: "Quarto giro: RF-013 e RF-016 chiusi. Due frasi, nessuna regola toccata, nessun hash cambiato; la review resta accepted e questi rilievi non riaprono il verdetto. RF-013: il residuo (g) nominava solo la censura totale, che e la variante che il pavimento rifiuta, mentre il vettore reale stabilito da What the floor does not buy e dalla configurazione 2b di AT-10 e la selettiva; preso alla lettera era vero, e proprio per questo peggio di un errore evidente, perche l'elenco dei residui e l'artefatto che un lettore futuro citera e le due voci risultavano discordi a chi seguisse il rimando. Ora (g) dice che la coalizione selettiva e cio che il light client non sa distinguere dall'attrito onesto, che raggiunge il proprio obiettivo in ceil(log(V/k)/log(3/2)) confini, e che il pavimento compra soltanto che quei confini siano diversi e ognuno pubblicato; la totale resta nominata per dire che non e il vettore. RF-016: la frase \"V=12 T=4 c=3 e la piu piccola istanza che esercita tutte e tre\" era falsa, e l'ho introdotta io chiudendo RF-015. Verificato prima di correggere che V=7 T=4 c=2 m=1 e ammissibile ed esercita il tetto, e aggiunto ai controlli sia il controesempio sia la forma generale della smentita, cioe che esistono istanze ammissibili con c > 1 e V < 12. La chiusura non e un terzo calcolo di minimo: al fixture serve c > 1, la nota ora dice quello, dichiara esplicitamente che nessuna minimalita e rivendicata e cita il controesempio; le due impossibilita restano perche dimostrate per esaurimento dello spazio dei parametri e la nota lo dice. Registrato nell'evidenza il pattern che e mio: tre superlativi in tre giri, tutti e tre falsificati, con la sostanza intorno che ogni volta reggeva e la frase che ogni volta prometteva piu di quanto dimostrato; le tre correzioni sono nel documento accanto a cio che hanno sostituito, cosi il pattern e leggibile da chi verra dopo e non solo dalla review. Verifica: cinque controlli testuali nuovi piu il controesempio dimostrato invece che asserito; tutti i controlli dei giri precedenti restano eseguiti come non regressione, incluse le due simulazioni e l'orizzonte di attrito calcolato. 129 controlli superati e 0 falliti, 0 ancore interne rotte. Nessun commit e nessun push."
    evidence_refs: ["SPEC-006", "DEBT-005", "REVIEW-010"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
links: [SPEC-006, DEBT-005, REVIEW-009]
created: 2026-08-25
updated: 2026-08-25
tags: [review, security, sybil, consensus]
related_specs: [SPEC-006, SPEC-004, SPEC-001]
related_decisions: [ADR-001, ADR-002, ADR-007, ADR-008]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "recorded review remediation-verification"
  - date: 2026-08-25
    action: "transitioned changes-requested -> changes-requested"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "recorded review remediation-verification"
  - date: 2026-08-25
    action: "transitioned changes-requested -> changes-requested"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "recorded review remediation-verification"
  - date: 2026-08-25
    action: "transitioned changes-requested -> accepted"
  - date: 2026-08-25
    action: "recorded review remediation"
---
# Review

## Outcome

> **Giro 2 (2026-08-25).** AGENT-002 ha rimediato tutti e sei i finding. La
> verifica della remediation, i quattro finding nuovi che ne sono emersi e il
> verdetto aggiornato sono in fondo a questa review, da
> [Verifica della remediation](#verifica-della-remediation-giro-2) in poi. Quanto
> segue è il testo del giro 1, conservato invariato perché è il criterio con cui
> la remediation va giudicata.

**Changes requested.** La regola scritta è buona e la sua architettura — due
strati che falliscono in modo diverso, con l'invariante anti-cattura confinato
nello strato che un light client verifica — è la scelta giusta e va conservata.
Non è però ancora vero, come i documenti affermano in più punti, che
l'auto-perpetuazione sia *impossibile*: la porta d'ingresso è stata chiusa, ma
ne restano tre aperte, e due di esse riportano un avversario esattamente dove
[DEBT-005] non voleva che arrivasse, con la catena formalmente valida a ogni
passo e un light client che non vede nulla di irregolare.

Le tre porte sono, in ordine di gravità:

1. **il documento dei parametri di consenso**, che è firmato dal quorum stesso e
   che nessun vincolo impedisce di scrivere in modo da spegnere la regola —
   `election_epoch_blocks` così grande che nessun confine arriva mai,
   `validator_max_consecutive_terms` così grande che nessun mandato scade mai;
2. **la contrazione del set**, che nessuna regola limita: il tetto di rotazione
   vincola le *ammissioni* e non le *uscite*, quindi una coalizione che censuri
   le candidature altrui non deve conquistare seggi, le basta restare sola;
3. **l'insieme dei candidati come leva sul seme**, perché `candidate_root` e
   `candidate_count` entrano nel seme e la loro composizione dipende
   dall'inclusione delle transazioni, che il set uscente controlla.

Nessuna delle tre richiede di riscrivere la derivazione. Due si chiudono con
vincoli di validità dello stesso tipo di quelli già presenti nel blocco dei
parametri; una si chiude ritrattando una frase e dichiarando il residuo nella
forma che questo progetto ha già adottato altrove.

Registro anche, perché è informazione quanto i finding: **ho attaccato per
prima la parte che l'implementatrice indicava come attaccabile — grindabilità
del seme, residuo (b), regola "solo rimozione" — e su tutte e tre la difesa
tiene** nella sostanza. I difetti che ho trovato non sono lì. Il dettaglio è in
[Tests and verification](#tests-and-verification).

## Acceptance-criteria compliance

Non ho rifatto le verifiche meccaniche che il Lead ha già rieseguito in
[REVIEW-009] — i tre hash `PD-0`, i diciotto valori dell'esempio dell'epoca 3,
l'accoppiamento `T >= 3m`. Ho campionato tre punti e tutti e tre tornano:
`fills = min(max(0, 5-2), 2, 3) = 2` con il tetto che vincola ed esclude `08`;
la derivazione algebrica `3cm <= V` e `ceil(V/T) <= c` ⟹ `V/T <= V/(3m)` ⟹
`T >= 3m`; e il fixture consensus `PD-0` (`V=6, T=3, c=2, m=1`) che soddisfa il
blocco. La parte aritmetica tiene e non la rivedo oltre.

Sui criteri, dal punto di vista della superficie di sicurezza e non della
lettera:

- **Determinismo, calcolabilità, impegno, casi degeneri, test [ADR-008], nessun
  parametro inventato**: soddisfatti, senza rilievi.
- **Ancoraggio ad [ADR-007]** (`availability` contribuisce zero, soglia binaria):
  soddisfatto nella forma, ma la soglia è manipolabile *a monte*, nella
  produzione dell'evidenza — RF-004.
- **Casualità non influenzabile a proprio favore dal set uscente**: soddisfatto
  per la parte di macinatura degli ID di blocco, **non** soddisfatto per la leva
  sull'insieme dei candidati — RF-003.
- **Dichiarazione onesta di ciò che un light client può stabilire**: le due liste
  chiuse sono corrette e complete rispetto a ciò che la regola *dice*. Diventano
  incomplete rispetto a ciò che la regola *permette* una volta ammessi RF-001 e
  RF-002: mancano «non può stabilire che i parametri di elezione non siano stati
  cambiati sotto di lui» e «non può distinguere una contrazione legittima da una
  cattura per attrito».

## Code observations

Nessun codice: il deliverable è specifica.

Tre punti di progetto che ho attaccato e che vanno registrati come **tenuti**,
perché una review che elenca solo ciò che è rotto non dice al Lead dove il
documento è solido:

**La regola di confine (`next_validator_set_hash == validator_set_hash` fuori
dai confini) è la parte migliore della consegna** e la sua verificabilità da due
soli campi di header è reale: l'ho seguita fino al passo 5 dell'algoritmo del
light client e il controllo è effettivamente eseguibile senza vedere
transazioni.

**La regola "solo rimozione" chiude davvero la sostituzione d'emergenza.** Ho
cercato specificamente il modo di usarla per resettare i mandati: il punto 7
impone che le voci sopravvissute siano identiche *anche* in
`seated_since_epoch`, e il punto 9 impone che l'`election` record sia copiato
alla lettera tranne `member_count`. Quindi una transizione forzata da revoca non
può né insediare, né riazzerare l'orologio dei mandati, né rideriverare il seme.
È scritta bene.

**Il divieto di sospendere l'elezione per mancanza di seme** e il rifiuto della
clausola "il set precedente continua se non esiste un successore lecito" sono
identificati per la ragione giusta — sono clausole di eccezione che un quorum
può fabbricare da sé. Il ragionamento è quello corretto, e in RF-002 mostro che
va applicato una volta di più di quanto sia stato applicato.

## Tests and verification

Le due gate `before-submit` restano soddisfatte. Questa sezione registra
**che cosa ho attaccato senza trovare nulla**, che per una gate di sicurezza è
un risultato e non un vuoto.

**Grindabilità del seme e indipendenza dell'invariante — la difesa tiene.** Ho
verificato le tre affermazioni una per una. Il numero di seggi che un macinatore
può conquistare a un confine è limitato da `validator_churn_cap_seats` perché il
tetto è applicato **dopo** l'estrazione: vero, il passo 6 lo mostra. Un
incumbent non può macinarsi un mandato più lungo: vero, `e -
seated_since_epoch < T` non è funzione del seme in nessun punto. Ogni
ricampionamento è un'estrazione su un insieme impegnato: vero **soltanto** se
l'insieme è dato, e in RF-003 mostro che non lo è. L'ordine di grandezza del
residuo `c*p + O(sqrt(c*p*(1-p)*2*ln G))` è la forma corretta per un best-of-`G`
ed è dichiarato invece di essere nascosto. Su questo asse il documento è più
onesto della media della letteratura.

**Residuo (b) del light client — corretto, e la sua asimmetria è quella giusta.**
Ho verificato che (a) e (c) siano davvero falsificabili in modo compatto: (a)
per non-appartenenza in due foglie adiacenti, il che dipende dall'ordinamento
bytewise delle foglie, che c'è; (c) per esibizione di un biglietto più basso non
insediato, il che dipende dal fatto che il biglietto sia funzione di dati
impegnati, il che è vero. E ho verificato che (b) non lo sia: asserisce
l'assenza di evidenza qualificante, e l'assenza non ha prova corta. La
dichiarazione è corretta. **Aggiungo però un rilievo di completezza**, non un
finding autonomo: (b) è indicato come «unico modo di fallire senza prova di
frode compatta» fra i *tre* modi di comporre male il set. Ammessi RF-001 e
RF-002 i modi diventano cinque, e i due nuovi sono anch'essi non falsificabili
in modo compatto — anzi non sono nemmeno *frodi*, sono comportamenti leciti.

**Regola "solo rimozione" — nessuna clausola di sostituzione trovata.** Ho
riletto i punti 1–9 e la §"Degenerate cases" cercando qualunque formulazione che
permettesse un ingresso fuori confine. Non ce n'è. Il vettore che ho trovato non
passa dall'ammissione ma dalla rimozione: RF-002.

**Casi non coperti che ho cercato e che risultano coperti**: epoca 1 e finestra
di entropia sopra la genesi (garantito dalla catena `L > candidacy_close >
entropy >= 2`); parità di biglietto (ordine totale su `account_key`); revoca di
un nodo in cooldown o candidato; un secondo `validator_candidacy` per lo stesso
`(node_id, epoch)` usato per far scegliere la chiave al set uscente (vietato
esplicitamente); genesi senza `election` record.

## Production quality and documentation compliance

`threat-model.md` è documento mio e qualcun altro lo ha modificato, quindi lo
valuto voce per voce.

**`TM-09` — aggiornamento corretto, da completare.** L'affermazione che il
vettore "uptime da datacenter vince la classifica" sia chiuso alla radice è
esatta: `availability` che contribuisce zero e la soglia binaria al posto della
classifica lo rendono strutturalmente inesprimibile, non solo improbabile. Il
residuo di numerosità è nominato e ricondotto ad `alpha`. **Manca** il residuo
di RF-004: l'evidenza che alimenta la soglia eredita la macinatura già
dichiarata in `ledger.md` §"Challenge evidence", quindi «lavoro che un Sybil non
può falsificare a costo nullo» è vero come tendenza ed è falso come assoluto.
Con quell'aggiunta lo stato "mitigato in specifica" è mio e lo sottoscrivo.

**`TM-18` — lo stato "mitigato in specifica" non è ancora difendibile senza
condizione.** È la voce che questa spec esiste per chiudere e non posso
accettarla come sta: RF-001 e RF-002 sono, ciascuno per conto proprio,
l'auto-perpetuazione di `TM-18` raggiunta senza violare alcuna regola. Lo stato
corretto oggi è **"mitigato in specifica, condizionato a [RF-001] e [RF-002]"**,
e diventa incondizionato quando i due vincoli di validità esistono. Il resto
della scheda — i due strati, che cosa resta aperto ai punti (i), (ii), (iii) —
è scritto bene e lo conservo.

**`SEC-REQ-13` — stessa condizione.** "M-02 coperto in specifica" è corretto
sulla lettera del requisito (regola scritta, deterministica, insieme
calcolabile, tetto per epoca, impegno nel header), e la separazione della
seconda metà — sanzione dell'equivocazione, M-07 — è corretta e onesta. Va
aggiunto il rinvio a `SEC-REQ-14`: finché i parametri governabili non hanno
intervallo firmato alla genesi e variazione massima per epoca, `SEC-REQ-13`
dipende da `SEC-REQ-14` e i due non sono indipendenti. Questa dipendenza oggi
non è registrata da nessuna parte ed è il cuore di RF-001.

**`AT-09` — valutazione corretta.** Il rifiuto di dichiararlo superato per
intero, con la parte sull'equivocazione **nominata** come non coperta, è
esattamente ciò che chiedo a una valutazione di test d'attacco. Una sola
aggiunta necessaria: la frase «una revoca di massa non si converte in
un'ammissione di massa» è vera e insufficiente — si converte in
**concentrazione**, che per un avversario già dentro è equivalente e più
economica (RF-002).

**`AT-10` — valutazione corretta nel metodo, con una misura sbagliata.** Il
rilievo che la *preparazione* del test sia diventata sbagliata — "candidati
d'attaccante con uptime da datacenter" non è più un vettore — è ottimo lavoro:
è un test che nessuno avrebbe rieseguito accorgendosene. Anche la richiesta di
una configurazione che misuri il vantaggio da macinatura è giusta. Ma il primo
criterio è espresso con una disuguaglianza che RF-002 falsifica: `ceil((V/3)/c)`
confini è il tempo di cattura **per ammissione**, e la cattura per attrito non
passa dall'ammissione. Servono due configurazioni in più, elencate in
[Required follow-up](#required-follow-up).

**§6.1 — annotazione corretta.** La scelta della lotteria (b) con soglia
ancorata a storage e compute è quella che l'analisi supportava, e la motivazione
per **non** adottare la leva 3 (diversificazione per posizione di rete: sarebbe
una funzione di dati non impegnati nel ledger, quindi non verificabile da un
light client) è la ragione giusta, non una scusa. La conservo.

**Rimedio RF-001 di [REVIEW-009] in `identity.md`.** Verificato: il paragrafo
ora separa i due verificatori, non promette al light client ciò che può solo il
full node, e il perimetro della revoca non è stato toccato. La correzione è
quella giusta e chiude il rilievo. **Ma la classe di difetto non è chiusa**: ne
ho trovate altre quattro, e sono RF-001, RF-002, RF-003 e la frase di
`identity.md` §"Declared limits of this mechanism" in RF-004. Il metodo con cui
il Lead ha trovato la prima — cercare le affermazioni che *sopravvivono* a una
ritrattazione fatta altrove — è quello che ho applicato e va reso procedura.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security-boundary | severity=critical | criterion=Nessun parametro il cui valore il quorum sceglie può disattivare l'invariante anti-auto-perpetuazione | remediation=Vincoli di validità di magnitudine sui parametri di elezione, ancorati alla genesi, e autenticazione della loro provenienza nel passo 5 del light client

**Il pavimento e il confine sono parametri, e i parametri li firma il quorum.**
`election_epoch_blocks`, `validator_max_consecutive_terms`,
`validator_min_set_size` e `validator_min_capture_epochs` vengono, per stessa
dichiarazione di `ledger.md` §"Election epochs and the boundary", «dai parametri
di consenso attivi». Un `SignedProtocolDocument` di kind `consensus_parameters`
è autorizzato da firme che «soddisfano l'unico predicato di quorum»
(`README.md` §"Signed protocol documents"): **è il set in carica a firmarlo.**
Il blocco di vincoli introdotto da questa spec vincola le *relazioni* fra i
parametri e non le loro *magnitudini*.

**Scenario d'attacco.** Il set in carica al confine `e` pubblica un documento
`consensus_parameters` con `sequence` incrementata, `activation_height` non
retroattiva, e i valori: `election_entropy_blocks = 2`,
`candidacy_close_blocks = 3`, `election_epoch_blocks = 2^60`,
`validator_target_set_size = V` invariato, `validator_churn_cap_seats = c`
invariato, `validator_max_consecutive_terms = 2^60`,
`validator_cooldown_epochs = 1`, `validator_min_capture_epochs = 1`. Verifico il
blocco di validità su questi valori: `0 < min <= V <= max` ✓;
`election_entropy_blocks >= 2` ✓; `candidacy_close > entropy` ✓;
`election_epoch_blocks > candidacy_close` ✓; `T >= 1` e `cooldown >= 1` ✓;
`ceil(V / 2^60) = 1 <= c` ✓; `3 * c * 1 <= V` ✓ per ogni `c <= V/3`.
**Il documento è accettato.** Da quel momento: il prossimo confine di elezione è
all'altezza `2^60`, cioè mai; il vincolo di confine impone allora
`next_validator_set_hash == validator_set_hash` a **ogni** altezza; il limite di
mandato non scatta mai perché `e - seated_since_epoch < 2^60` è sempre vero. Il
set è congelato per sempre, ogni blocco è valido, e il light client che esegue i
controlli 1–7 li supera tutti — anzi, il controllo 1 è quello che *impone* al
set di non cambiare. `TM-18` è integralmente ripristinato attraverso il
documento dei parametri.

Non è una possibilità teorica lasciata a un futuro modulo di governance: è la
configurazione che il progetto **già oggi** permette, e `SEC-REQ-14` — intervallo
ammissibile firmato alla genesi e variazione massima per epoca — è ancora
`aperto`. Il progetto ha inoltre già incontrato e risolto questa identica classe
di problema una volta: `identity.md` §"Declared limits of this mechanism" scrive
che il vantaggio di Argon2id «vale solo perché il pavimento di costo è una
regola di validità e non una raccomandazione, e che un insieme di parametri
governato avrebbe altrimenti potuto rimuovere del tutto». È lo stesso
ragionamento, applicato lì e non applicato qui.

**Condizione di chiusura, verificabile.** Nel blocco di vincoli di
`ledger.md` §"Rotation: the cap and the floor" compaiono, come regole di
accettazione del documento e non come raccomandazioni, dei limiti superiori e
inferiori di magnitudine ancorati a quantità fissate alla genesi — nella forma
che il progetto preferisca, ad esempio `validator_max_consecutive_terms <=
validator_max_terms_ceiling` e `election_epoch_blocks <=
election_epoch_blocks_ceiling` con i due tetti presi dalla configurazione di
genesi e non dal documento corrente, e una variazione massima per `sequence`
consecutiva per ciascun parametro di elezione. In aggiunta, il passo 5
dell'algoritmo del light client dichiara **da dove** il client prende i valori
che usa (oggi non lo dice: usa `election_epoch_blocks`, `T`, `c`, le soglie di
dimensione senza che alcun passo precedente le procuri o le autentichi) e
impone di rifiutare un set i cui parametri di elezione non siano autenticati
contro l'ancora di fiducia. Verifica: un documento `consensus_parameters` con
`election_epoch_blocks` o `T` oltre il tetto di genesi è rifiutato in
accettazione, e la fixture di conformità che lo dimostra esiste.

---

RF-002 | category=security-boundary | severity=critical | criterion=Nessuna coalizione al di sotto della soglia di safety BFT può portarsi a una frazione superiore del potere di voto in un solo confine | remediation=Un pavimento sulla contrazione del set, simmetrico al tetto sulle ammissioni, e ritrattazione delle due affermazioni che lo presuppongono

**Il tetto di rotazione limita gli ingressi e nessuna regola limita le uscite.**
`fills` è vincolato da `validator_churn_cap_seats`; il numero di membri **non
mantenuti** a un confine non è vincolato da niente, e il solo pavimento è
`validator_min_set_size`, che il blocco di vincoli lascia libero fino a `1` —
valore che il fixture consensus `PD-0` di `README.md` usa davvero, con `V = 6`.
Un avversario che voglia il controllo totale non deve conquistare seggi: gli
basta restare solo.

**Scenario d'attacco, vettore primario (soglia 1/3).** Una coalizione `A`
possiede `k > V/3` seggi — sotto la soglia di safety BFT, quindi per ipotesi
**non** dovrebbe poter catturare nulla. Durante la finestra di candidatura
dell'epoca `e`, `A` fa finalizzare le proprie `k` candidature e poi rifiuta il
voto a ogni blocco che contenga una candidatura altrui: con più di un terzo del
potere impedisce il quorum, quindi nessun blocco che includa quelle transazioni
si finalizza. La catena rallenta o si ferma fino a `candidacy_close_height(e)`,
dopo la quale quelle candidature sono **invalide per costruzione**
(`election_epoch` deve avere `candidacy_close` strettamente sopra l'altezza di
proposta). Al confine, la derivazione eseguita da qualunque full node onesto dà:
`R = A` (solo i membri di `A` hanno candidatura valida, mandato non scaduto,
punteggio sopra soglia); `C = A`; `Nw = C \ R = ∅`; `fills = 0`. Il nuovo set è
`A`, `member_count = k`, `filled_count = 0` — **sotto il tetto, non al tetto**.
Se `k >= validator_min_set_size` il set è valido, la catena riprende, e `A`
detiene il 100 % del potere di voto. Un light client verifica i controlli 1–7 e
li supera tutti: nessun ingresso fuori confine, potere uniforme, mandati
rispettati, `seated_since_epoch` coerente, tetto rispettato con margine, e una
`member_count` più piccola ma dentro `[min, max]`. Il controllo 8, la deriva, è
*calcolabile* ma nessuna **regola** la vincola, quindi non c'è nulla da
rifiutare. Ai confini successivi `A` si autoriconferma indefinitamente, perché
continua a censurare e perché il pool è composto da sé stessa; il limite di
mandato la costringe a ruotare i propri membri, non a cederli.

**Scenario secondario, sopra i 2/3, che conferma l'assenza del pavimento.** Un
quorum autorizza `revoke_identity` per i membri onesti — l'autorità di revoca è
del quorum, `reason:"validator_misconduct"` non richiede prova — e usa la
transizione "solo rimozione". I punti 7, 8 e 9 sono soddisfatti alla lettera. Il
risultato è identico: contrazione fino alla sola coalizione.

**Le due affermazioni che questo falsifica.** `ledger.md` §"Rotation: the cap and
the floor" scrive che «con un tetto di `c` seggi l'avversario ha bisogno di
almeno `ceil((V/3)/c)` confini per raggiungere la soglia di safety BFT»: è vero
per la cattura **per ammissione** ed è falso in generale, perché la cattura per
attrito arriva al 100 % in **un** confine e non usa alcun ingresso. La stessa
disuguaglianza è ripetuta nella valutazione di `AT-10` in `threat-model.md`.
E §"Revocation forces a validator set transition" scrive che «restringere il set
abbastanza raggiunge lo stallo invece di un set a scelta dell'attaccante»:
presuppone che rimuovere membri non aiuti l'attaccante, mentre per un attaccante
già dentro rimuovere è precisamente il modo di scegliere il set. Anche
l'invariante 8 — «un quorum autorizza il set successore; non ne sceglie i
membri» — promette più di quanto le regole impongano: il quorum non può
*nominare* i membri, ma sceglie quali candidature esistano, e in questo scenario
ciò equivale a sceglierli.

**Condizione di chiusura, verificabile.** Una regola di validità sul set eletto
che limiti la contrazione con lo stesso meccanismo con cui il tetto limita
l'espansione, verificabile da un light client sui due soli documenti
`ValidatorSet` che già possiede: nella forma che il progetto preferisca, ad
esempio `member_count(e) >= member_count(e-1) - departure_cap` con
`departure_cap` parametro governato, oppure un vincolo di accettazione
`3 * validator_min_set_size > 2 * validator_target_set_size` che renda la
sopravvivenza alla contrazione possibile solo a chi già detiene più dei due
terzi. Le due formulazioni hanno costi diversi in liveness e la scelta è di
progetto, ma **una delle due deve esistere**, altrimenti la soglia effettiva di
cattura di questa rete è `1/3 + ε` e non `2/3`. In aggiunta: le tre affermazioni
citate sopra corrette o ritrattate, e nell'elenco «non può stabilire» del light
client una voce nuova che dica che una contrazione lecita e una cattura per
attrito gli sono indistinguibili. Verifica: la fixture che esibisce un set
eletto che viola il pavimento di contrazione è rifiutata, e la disuguaglianza
`ceil((V/3)/c)` non compare più senza la qualificazione "per ammissione".

---

RF-003 | category=security-boundary | severity=high | criterion=Il set uscente non influenza a proprio favore la derivazione, o il residuo è dichiarato con la sua forma | remediation=Ritrattare l'affermazione che il macinatore non controlli l'insieme dei candidati e dichiarare il residuo composto

**Il seme ha una seconda leva, e non è quella dichiarata.** `election_seed` è
funzione di `election_entropy`, di `candidate_root` **e** di `candidate_count`.
Il documento difende la propria posizione scrivendo che «ogni ricampionamento è
comunque un'estrazione su un insieme di candidati impegnato che il macinatore
non controlla». Chi macina è il proponente dei blocchi della finestra di
entropia, cioè un validatore in carica; e un validatore in carica **controlla
l'inclusione delle transazioni**, quindi controlla quali `validator_candidacy`
si finalizzano prima di `candidacy_close_height(e)`, quindi controlla
`candidate_root` e `candidate_count`, quindi controlla due dei tre input del
seme. L'affermazione è falsa per l'unico avversario che conta.

**Scenario d'attacco.** Il set uscente tiene in sospeso `q` candidature altrui
fino a poco prima di `candidacy_close_height(e)` — un ritardo di inclusione non
è distinguibile da una candidatura mai arrivata, e non lascia traccia. Per
ciascuno dei `2^q` sottoinsiemi che può decidere di finalizzare ottiene un
`candidate_root` diverso, quindi un `election_seed` diverso, quindi un **vettore
di biglietti interamente diverso** per tutti i candidati. A questo si compone il
best-of-`G` sull'ultimo blocco della finestra, già dichiarato. Due proprietà
peggiorano rispetto al residuo scritto: la ricerca su sottoinsiemi rimuove
contemporaneamente i concorrenti dal pool, quindi non produce solo *bias* ma una
combinazione di *bias* e *esclusione*; e ogni candidatura esclusa non è
falsificabile come nel caso (a) — lì l'omesso esibisce la propria candidatura
finalizzata, qui la candidatura **non è stata finalizzata**, quindi non esiste
prova compatta né lunga. Il tetto di rotazione resta l'unico limite superiore, e
resta valido: questo non è un percorso di cattura, è una perdita di equità più
grande di quella dichiarata e con un residuo di forma diversa.

**Condizione di chiusura, verificabile.** In `ledger.md` §"The seed, and why the
rule does not depend on it": l'affermazione «un insieme di candidati impegnato
che il macinatore non controlla» ritrattata o qualificata, e il residuo composto
dichiarato con la sua forma, come già fatto per il best-of-`G`, includendo il
fatto che l'esclusione per non inclusione non è falsificabile in modo compatto.
Se il progetto vuole anche ridurre la leva e non solo dichiararla, la riduzione
naturale è togliere `candidate_root` e `candidate_count` dal preimmagine del
seme e legarli invece per validità (`ElectionRecord` li impegna già, e il seme
resta legato all'epoca dal proprio dominio) — ma è una scelta di progetto, non
una condizione che impongo. Verifica: la frase citata non compare più nella
forma assoluta, e §"What a light client can establish" elenca l'esclusione per
non inclusione fra i residui non falsificabili in modo compatto, accanto a (b).

---

RF-004 | category=security-boundary | severity=high | criterion=La soglia binaria di eleggibilità non è raggiungibile senza fornire il lavoro che dichiara di misurare | remediation=Diversità di emittente nel punteggio di contributo, o dichiarazione esplicita del residuo ereditato

**La soglia è binaria e verificabile; l'evidenza che la alimenta no.**
`contribution_score` somma le `challenge_evidence` con `outcome:"passed"` e
`kind` in `{storage, compute}`. Nessuna condizione impone che quelle evidenze
provengano da emittenti distinti, né che le evidenze `failed` per lo stesso
soggetto nella stessa finestra contino in qualche modo. `ledger.md`
§"Challenge evidence" dichiara già, quantificandolo, che un emittente colluso
che consegni il proprio segreto impegnato e un proponente possono cercare
insieme un beacon che produca contemporaneamente l'accoppiamento voluto e una
sfida che il soggetto supera — `10^3`–`10^6` valori legali di `timestamp_ms`, un
SHA-256 ciascuno — e indica come mitigazione la copertura a due emittenti di
`wire.md`, che «degrada l'attacco da "superare la sfida" a "superarne una su
due"».

**Perché quella mitigazione non protegge l'eleggibilità.** Degradare a "una su
due" è una mitigazione efficace per un *tasso di rilevamento*, dove il fallimento
conta. Il punteggio di contributo è una **somma di successi** e non sottrae mai
nulla: superarne una su due significa `+measured_units`, e il fallimento
dell'emittente onesto non ha alcun effetto sull'eleggibilità. La mitigazione
dichiarata è quindi inapplicabile a questa nuova superficie, e questa spec ha
trasformato quell'evidenza da metrica di ricompensa in **credenziale di accesso
al consenso** senza rivalutare il residuo nel nuovo ruolo.

**Scenario d'attacco.** Un operatore controlla due identità enrollate, `X` e
`Y`. `Y` funge da emittente colluso e `X` da soggetto: `Y` consegna a `X` il
proprio `issuer_reveal` impegnato, e un proponente della coalizione enumera
`timestamp_ms` finché il beacon accoppia `Y` con `X` e la casualità risultante
seleziona un frammento che `X` conserva davvero — un frammento solo, non
l'oggetto. Ogni evidenza così prodotta è integralmente valida: i due hash
tornano, l'impegno precede il beacon, la funzione di assegnazione conferma la
coppia. Ripetuto sulla finestra `validator_eligibility_window_epochs`, `X` supera
`validator_eligibility_threshold_units` **senza conservare gli oggetti che il
punteggio dichiara di misurare**, e diventa candidato validatore. Il costo è
l'enrollment di due identità più la macinatura, non lo storage. Con `N` coppie
si ottengono `N` biglietti, che è il residuo di numerosità di `TM-09` ma a un
prezzo molto inferiore a quello che quel residuo assume.

**L'affermazione che questo falsifica.** `identity.md` §"Declared limits of this
mechanism" punto 3: «il contenimento Sybil in Coblox è economico, non
crittografico. Poggia sul fondo di esistenza limitato e condiviso, e
sull'eleggibilità a validatore ancorata a lavoro di storage e compute **che non
può essere falsificato senza spendere risorse reali**». Con la macinatura
dichiarata, quel "non può" è un "costa di più, non sempre abbastanza". La frase
punta inoltre alla sezione del mint e non alla nuova §"Validator election and
rotation", quindi rimanda a una regola che non è più quella che governa
l'eleggibilità.

**Condizione di chiusura, verificabile.** Una delle due, e la seconda è
accettabile: (i) `contribution_score` conta solo evidenze provenienti da almeno
`validator_eligibility_min_issuers` emittenti distinti — parametro simbolico,
coerente con la copertura a due emittenti già imposta da `wire.md`, e la
verifica resta una funzione totale di dati finalizzati; oppure (ii)
`ledger.md` §"Eligibility" dichiara esplicitamente che la soglia eredita il
residuo di macinatura di §"Challenge evidence", con il suo ordine di grandezza,
e spiega perché la copertura a due emittenti non lo mitiga per una somma di
successi. In entrambi i casi: la frase di `identity.md` corretta e il suo
rimando puntato alla sezione giusta, e `TM-09` che porta il residuo.

---

RF-005 | category=security-boundary | severity=medium | criterion=Il cooldown impone un'assenza effettiva pari al parametro dichiarato | remediation=Estendere il cooldown a ogni uscita da un seggio, o dichiarare che il parametro non vincola oltre un'epoca

**Il cooldown si applica solo alla scadenza del mandato, e l'uscita volontaria
azzera l'orologio.** La condizione 4 di eleggibilità recita: «non è in cooldown:
non ha lasciato il set **per scadenza del mandato** in nessuna delle
`validator_cooldown_epochs` epoche precedenti a `e`». Il passo 3 della
derivazione conferma la lettura: la scadenza del mandato «fa inoltre partire il
cooldown», mentre una candidatura lasciata cadere rimuove il membro «almeno per
questo confine» e basta. Un membro reinsediato riceve `seated_since_epoch = e`,
quindi il contatore del mandato riparte da zero.

**Scenario d'attacco.** Un blocco di incumbent, ciascuno con `T - 1` epoche di
servizio, non presenta candidatura per l'epoca `e`. Esce come **uscita
volontaria**, quindi senza cooldown. Presenta candidatura per `e + 1`, rientra
nel pool, e — se il pool è piccolo o se il blocco lo domina, che è la condizione
in cui il cooldown avrebbe dovuto mordere — riprende il seggio con
`seated_since_epoch = e + 1`, contatore azzerato. L'assenza effettiva è di **una
sola epoca**, contro le `validator_cooldown_epochs` che la rete aveva scelto.
Il calcolo lo conferma: la presenza per chi arriva a scadenza è `T/(T+k)` con
`k = validator_cooldown_epochs`, quella per chi esce volontariamente in anticipo
è `(T-1)/T`, e la seconda supera la prima per ogni `k >= 2`. Per un nodo
razionale, quindi, la scadenza del mandato è una strategia dominata: il cooldown
vincola soltanto chi non pianifica. Una rete che scelga `validator_cooldown_epochs
= 5` per forzare un ricambio reale non ottiene nulla, e non c'è nel documento
alcuna frase che la avverta.

Non è un'evasione dell'invariante — il seggio viene comunque **riconquistato per
derivazione**, che è la proprietà che [DEBT-005] chiedeva — ma è un parametro
che promette più di quanto imponga, ed è esattamente l'interazione fra elezione,
mandato e uscita volontaria che nessuna delle regole prese singolarmente
considera.

**Condizione di chiusura, verificabile.** Una delle due: (i) la condizione 4
copre ogni nodo che fosse membro del set attivo al confine precedente e non
risulti mantenuto a questo, qualunque sia la ragione, con l'avvertenza che il
light client la verifica solo per i confini che ha osservato — residuo (e), già
dichiarato; oppure (ii) `ledger.md` §"Rotation: the cap and the floor" dichiara
che `validator_cooldown_epochs` è evadibile con un'uscita volontaria anticipata
e che l'assenza forzata effettiva è di un'epoca, così che chi tara il parametro
sappia che cosa sta comprando. La frase «il cooldown costringe ogni seggio a
essere riconquistato per derivazione invece che mantenuto per inerzia» resta
vera in entrambi i casi e non va toccata. Verifica: il documento risponde alla
domanda «quanto dura l'assenza minima di un membro uscente che gioca bene?» con
un numero.

---

RF-006 | category=documentation | severity=low | criterion=L'algoritmo del light client è eseguibile a partire dai dati che i suoi passi procurano | remediation=Dichiarare la provenienza dei parametri di elezione nel passo 5

Il passo 5 di §"Light-client balance verification" impone controlli su
`election_epoch_blocks`, `validator_min_set_size`, `validator_max_set_size`,
`validator_max_consecutive_terms` e `validator_churn_cap_seats`, ma nessuno dei
passi 1–4 procura al client quei valori né gli fa autenticare il documento
`consensus_parameters` contro `consensus_parameters_hash` dell'header. Come
scritto, il passo 5 non è eseguibile; e un'implementazione che riempia la lacuna
prendendo i valori dal documento corrente della catena realizza esattamente
RF-001. È un finding separato solo perché la sua chiusura è documentale e va
fatta anche se RF-001 fosse chiuso in altro modo.

**Condizione di chiusura.** Il passo 5 nomina la fonte dei parametri, il passo
che la autentica, e il comportamento di fallimento (rifiuto, non prosecuzione
con valori di default). Verifica: un implementatore che segua i passi 1–10 in
ordine ha in mano ogni quantità che gli viene chiesto di confrontare.

## Required follow-up

**Blocchi per `GATE-SECREVIEW`: RF-001 e RF-002.** Sono l'auto-perpetuazione di
`TM-18` raggiunta per due porte diverse senza violare alcuna regola, e il debito
chiuso è `critical`. Nessuno dei due richiede di toccare la derivazione: sono
due vincoli di validità dello stesso tipo di quelli che il blocco dei parametri
già contiene, più le correzioni testuali che ne conseguono.

**Richiesti nella stessa remediation: RF-003, RF-004, RF-005, RF-006.** RF-003 e
RF-004 sono affermazioni di sicurezza sovrastimate, cioè la classe che il
progetto dichiara peggiore di un'affermazione mancante; RF-005 è un parametro
che promette più di quanto imponga; RF-006 è una lacuna procedurale che rende
RF-001 facile da reintrodurre in implementazione. Tutti e quattro sono chiudibili
con dichiarazioni, e per RF-004 la variante (i) sarebbe migliore ma non la
impongo.

**Aggiornamenti a `threat-model.md`** (li faccio io se il Lead preferisce, è
documento mio, ma vanno fatti prima di `spec_done`):

- `TM-18`: stato «mitigato in specifica **condizionato a RF-001 e RF-002**»,
  incondizionato alla loro chiusura;
- `SEC-REQ-13`: dipendenza esplicita da `SEC-REQ-14` registrata; la regola di
  elezione non è più forte dei limiti sui parametri che la definiscono;
- `TM-09`: aggiunta del residuo di RF-004;
- `AT-09`: alla riga sulla revoca di massa, che si converte in concentrazione;
- `AT-10`: la disuguaglianza `ceil((V/3)/c)` qualificata come tempo di cattura
  *per ammissione*, più due configurazioni nuove — (1) cattura per attrito: una
  coalizione a `k > V/3` che censura le candidature altrui, misurando le epoche
  per arrivare al 100 % del potere, che nella regola attuale è **una**; (2)
  evasione del cooldown: incumbent che escono volontariamente un'epoca prima
  della scadenza, misurando l'assenza effettiva contro
  `validator_cooldown_epochs`.

**Nessun debito nuovo proposto.** Tutti i sei finding sono chiudibili dentro
questa spec, e RF-001 e RF-002 non vanno rimandati per la stessa ragione per cui
[DEBT-005] non poteva essere rimandato a M-07: la chiusura del set non è
reversibile a posteriori, e una devnet che accumuli storia sotto RF-001 o RF-002
è nella condizione che [DEBT-005] vieta.

## La forma in cui il claim di sicurezza è difendibile

Il Lead mi ha chiesto, come già per il claim complessivo del progetto, in quale
forma esatta ritengo difendibile la dichiarazione di sicurezza sulla
composizione del set. Questa, e non una più corta:

> Dato un documento di parametri di consenso fissato e ammissibile, un light
> client stabilisce che il set attivo è **di forma lecita e in rotazione
> lecita** — mandati limitati, ingressi limitati, nessun cambio fuori
> programma, nessun membro insediato oltre il proprio mandato — e non stabilisce
> che sia il set che la regola di eleggibilità avrebbe dovuto produrre. Dei tre
> modi di comporre male il set, due sono contraddicibili con un messaggio corto
> e uno richiede un nodo che conservi la storia. La dichiarazione vale a
> parametri fermi: i parametri di elezione sono firmati dal quorum, quindi la
> proprietà è forte quanto i limiti che vincolano il loro valore, e finché
> quei limiti non esistono la proprietà è condizionale e va enunciata come tale.
> La dichiarazione riguarda inoltre chi **entra** nel set e non chi ne **esce**:
> finché la contrazione non ha un pavimento, la soglia effettiva di cattura di
> questa rete è quella che permette di censurare, non quella che permette di
> firmare.

Le prime due frasi sono già nel documento e sono ben scritte. Le ultime due sono
ciò che manca, e sono la ragione di RF-001 e RF-002. Chiuse quelle due, le
ultime due frasi si accorciano a una sola — «la dichiarazione vale entro i
limiti di parametro fissati alla genesi» — e il claim diventa difendibile senza
condizioni accanto, che è la posizione in cui questo progetto vuole stare.

## Final decision

Changes requested. La spec resta in `review`. `GATE-SECREVIEW` non è
soddisfatta finché RF-001 e RF-002 non sono chiusi.

---

# Giro 2 — verifica della remediation

## Verifica della remediation (giro 2)

Non ho ripetuto le verifiche numeriche che il Lead ha rieseguito. Ho ricalcolato
per conto mio i due valori che la remediation ha **cambiato**, perché un valore
nuovo non è un valore già rivisto: `election_seed` con la nuova preimmagine e i
tre `election_ticket` che ne discendono. Coincidono con i byte scritti nel
documento, e l'ordine risultante — `06`, `08`, `05` — è quello della tabella, con
`05` escluso dal tetto. L'esempio è ancora rifacibile a mano.

**RF-001 — chiuso, e la collocazione regge al mio stesso attacco.** La domanda
posta era se `ElectionBounds` nel trust anchor di genesi risolva o soltanto
sposti il problema. Risolve, per una ragione precisa: il principale che firma la
distribuzione **non è** il quorum. Il mio attacco era «il set in carica firma i
propri limiti», e un oggetto che nessun documento di catena può toccare lo
disinnesca, senza introdurre un soggetto fidato nuovo — la distribuzione firmata
è già la radice di fiducia della genesi, della chiave di fiducia e del
checkpoint. La proprietà rivendicata nel §"Declared limit" è quella giusta e
nella forma giusta: *nessuna delle due direzioni permette a un attaccante di
allargare i limiti che un dato client applica*. È più stretta di «i limiti sono
infalsificabili» ed è vera. Anche l'aver messo il §"Election bounds" nel
documento dei trust anchor invece che fra i parametri è la scelta corretta: dice
al lettore che categoria di oggetto sia.

**RF-002 — la regola è chiusa, l'affermazione che la accompagna no.** Il
pavimento `3 * member_count(new) > 2 * member_count(old)` è la risposta giusta,
e la scelta di riusare la forma del predicato di quorum invece di introdurre un
`departure_cap` governato è **corretta e per la ragione corretta**: un parametro
in più sarebbe stato un parametro in più da vincolare in `ElectionBounds`, cioè
RF-001 un'altra volta. Ho verificato che il pavimento si applichi davvero anche
alle transizioni di sola rimozione: è la regola 10 di §"Revocation forces a
validator set transition", e con essa la concentrazione per revoca di massa è
chiusa. Il difetto residuo non è nella regola ma in ciò che il documento dice
della regola, ed è RF-008.

**RF-003 — chiuso, con più di quanto avevo imposto, e senza effetti collaterali
tranne uno.** La rimozione di `candidate_root` e `candidate_count` dalla
preimmagine del seme è la riduzione giusta. Ho cercato le conseguenze: il legame
del seme all'epoca resta (`u64be(election_epoch)`) e alla catena (`chain_id`),
quindi non c'è riuso fra epoche; il legame dei due valori resta per **validità**,
che è un controllo più forte di quello per hashing, come il documento osserva
correttamente; l'ordinamento di `candidacy_close_height` sotto la finestra di
entropia era già ciò che rendeva la scelta del sottoinsieme **cieca**, e
l'argomento aggiunto in §"The second lever" è corretto e non l'avevo formulato
io. L'unico effetto collaterale è testuale ed è RF-011. Il residuo dichiarato —
esclusione per non finalizzazione, invisibile a **ogni** verificatore, elencato
come (h) separato da (a) proprio perché (a) è compattamente falsificabile e
questo non lo è affatto — è la forma corretta, ed è più onesto di quanto avessi
chiesto.

**RF-004 — chiuso, e l'affermazione che non tocchi [ADR-002] è vera.** L'ho
verificata invece di accettarla, perché è il punto dove si sbaglia in buona fede.
`issuer_node_id` è già un campo di `ChallengeEvidenceBody`, presente in ogni
evidenza finalizzata; la condizione 4 è un conteggio di valori distinti su dati
che il verificatore ha già, e non tocca la funzione di assegnazione, né la
finestra di risposta, né alcunché di ciò che [ADR-002] governa. Nessun debito
serviva, e non proporne era corretto. Il vincolo
`validator_eligibility_min_issuers >= 2` combacia con la copertura a due
emittenti di `wire.md`, quindi un nodo onesto non è escluso dalla propria
condizione. E la formulazione finale — «costosa da falsificare, non impossibile
da falsificare», con il prezzo lineare in `validator_eligibility_min_issuers` e
il bound ultimo ricondotto ad `alpha` — è esattamente ciò che chiedevo, compresa
la frase che rifiuta esplicitamente di scrivere l'affermazione sovrastimata. La
correzione corrispondente in `identity.md`, che **cita** la vecchia formulazione
e dice che era più forte di quanto il protocollo consegni, è il modo giusto di
ritrattare.

**RF-005 — chiuso.** La condizione 5 nella forma «uscita da un seggio per
qualunque ragione» è la variante (i), l'aritmetica dell'evasione è riportata nel
documento, e alla domanda «quanto deve stare fuori chi gioca bene?» il documento
ora risponde con un numero. L'interazione con il pavimento è nominata nei casi
degeneri. Resta una conseguenza avversaria non dichiarata, che è RF-010.

**RF-006 — chiuso, oltre la richiesta.** Il passo 5 nomina le tre fonti
nell'ordine, impone il fallimento chiuso, e vieta esplicitamente il ripiego su
default o su valori di un documento precedente. La frase che spiega *perché* il
passo nomina le proprie fonti — un'implementazione che riempia la lacuna con
quello che la catena dice sta applicando i numeri dell'attaccante — è la
motivazione giusta scritta nel posto giusto.

**Sulla domanda della liveness, che AGENT-002 ha sollevato da sola.**
L'argomento — un set privo di più di un terzo del potere non raggiunge comunque
il quorum, quindi il pavimento non aggiunge una perdita — è **corretto per il
caso di guasto**, e per quel caso lo sottoscrivo: i membri caduti contano ancora
in `total_power` finché una transizione non li rimuove, quindi la catena è già
ferma. **Non copre il caso in cui i membri sono vivi ma non mantenuti**: uscita
volontaria di massa, caduta collettiva sotto la soglia di contributo per
un'interruzione dell'emissione delle challenge, o scadenza simultanea dei
mandati. Lì il set uscente firma benissimo, e il pavimento trasforma un degrado
in un arresto. È il caso che RF-007 rende non ipotetico. La dichiarazione nei
casi degeneri va quindi completata con quella seconda famiglia, che non è una
nota a margine: è la famiglia che si verifica.

## Il mio documento, giro 2

`TM-18` è aggiornato come intendevo, e meglio: le due porte sono registrate
**come `TM-18` a tutti gli effetti** e non come varianti minori, con il modo in
cui sono state chiuse; i residui (iv) e (v) sono aggiunti; e la condizione
esplicita «lo stato *mitigato in specifica* è condizionato all'esistenza di
`ElectionBounds` in ogni distribuzione firmata, e su una rete che non ne
pubblichi `TM-18` è integralmente aperto» è precisamente la forma condizionale
che avevo chiesto. La sottoscrivo, **tranne** la frase sulla soglia dei due
terzi, che è RF-008 e che è finita anche qui.

`SEC-REQ-13` porta la dipendenza da `SEC-REQ-14` con la distinzione giusta fra i
parametri di elezione, coperti da `ElectionBounds`, e gli altri parametri
governati, che restano aperti. Corretto, con la riserva di RF-009: la dipendenza
è soddisfatta per l'intervallo e non per la variazione per epoca né per il
ritardo di attivazione, che `SEC-REQ-14` richiede entrambi.

`TM-09` porta il residuo di RF-004 con il suo ordine di grandezza e la ragione
per cui la copertura a due emittenti non trasferisce. Corretto.

`AT-09` porta la conversione in concentrazione e la attribuisce a [REVIEW-010].
Corretto.

`AT-10` porta la qualificazione «per ammissione» e tre configurazioni invece
delle due che avevo chiesto. La terza, sull'evasione del cooldown, è un'aggiunta
buona: confronta le due formulazioni della regola, che è il modo di rendere un
test utile invece che rituale. Ma la configurazione 2 porta un **esito atteso
sbagliato**, ed è la ragione per cui RF-008 non è un rilievo di stile: un test
con l'attesa sbagliata viene registrato come implementazione difettosa quando
invece misura il comportamento reale della regola.

## Review findings, giro 2

<!-- Stable form: RF-00N | category=... | severity=... | criterion=... | remediation=... -->

RF-007 | category=correctness | severity=critical | criterion=Nessuna configurazione conforme dei parametri produce un arresto deterministico della catena a un'altezza prevedibile | remediation=Sfalsare la coorte di genesi, oppure escludere dal pavimento la riduzione imputabile a scadenza di mandato

**La coorte di genesi scade tutta insieme, e il pavimento trasforma quel fatto in
un arresto certo.** `ledger.md` §"Validator-set continuity" impone che le voci
del set di genesi portino `seated_since_epoch:"0"`. Il ritiro del passo 1 della
derivazione richiede `e - seated_since_epoch < T`. Al confine `e = T` **nessun
membro di genesi è mantenuto**, per tutti contemporaneamente e per costruzione.
Allora `R = ∅`, e `fills` è limitato da `validator_churn_cap_seats`, quindi il
set nuovo ha al massimo `c` membri. Il vincolo `3 * c * m <= V` con `m >= 1` dà
`c <= V/3`. Il pavimento di contrazione richiede `3 * c > 2 * V`, cioè
`c > 2V/3`. Le due sono incompatibili per ogni `V > 0`: **non esiste set valido
al confine `e = T` e la catena si ferma**, su qualunque rete conforme, a
un'altezza nota in anticipo, `T * election_epoch_blocks`.

**Scenario, sui parametri del fixture del progetto stesso.** Il fixture consensus
`PD-0` di `README.md` ha `V = 6`, `T = 3`, `c = 2`, `m = 1`,
`validator_min_set_size = 1`. Al confine `e = 1` i sei membri di genesi hanno
`1 - 0 = 1 < 3` e sono mantenuti; `fills = min(max(0, 6-6), 2, |Nw|) = 0`, quindi
nessuno entra. Idem al confine `e = 2`. Al confine `e = 3` tutti e sei hanno
`3 - 0 = 3`, che non è minore di `3`: `R = ∅`, `fills = min(6, 2, |Nw|) = 2`, set
nuovo di due membri. Il pavimento `3 * 2 > 2 * 6` è falso. **La catena si ferma
all'altezza `3 * election_epoch_blocks`.** La ripresa è fuori banda, con una nuova
distribuzione firmata, cioè la procedura prevista per il disastro.

Prima del pavimento lo stesso confine produceva un set di due membri: pessimo, ma
vivo. Il pavimento ha convertito un degrado in un arresto — che è la scelta
sicurezza-su-liveness fatta ovunque in questa sezione, e che qui non è una scelta
perché non c'è avversario: accade da sé, sempre, sulla prima rete che raggiunga
il confine `T`. Il caso non è coperto da §"Degenerate cases": il paragrafo «molti
membri che escono insieme» descrive il rischio in astratto e lo tratta come una
tensione di taratura, mentre qui è una certezza indipendente dalla taratura. Ed è
precisamente la classe che il titolo di quella sezione promette di trattare:
l'interazione fra limite di mandato, tetto di ingressi e pavimento di
contrazione, che nessuna delle tre regole prese singolarmente vede.

Il fenomeno è generale — ogni coorte insediata nella stessa epoca scade nella
stessa epoca — ma solo la coorte di genesi è **garantita** e di dimensione `V`,
perché a regime le uscite sono `ceil(V/T)` per confine e i seggi vengono
rimpiazzati sotto il tetto, quindi la dimensione resta e il pavimento è
soddisfatto.

**Condizione di chiusura, verificabile.** Una delle due, e la prima è più pulita:
(i) il set di genesi porta `seated_since_epoch` **sfalsati** nell'intervallo
`[0, T-1]` invece che tutti a zero, con una regola di validità che imponga che
non più di `validator_churn_cap_seats` voci condividano lo stesso valore — la
scadenza si distribuisce e il regime parte già scaglionato; (ii) il pavimento di
contrazione non si applica alla parte di riduzione imputabile a scadenza di
mandato, che è una quantità calcolabile da un light client dai soli
`seated_since_epoch` del set precedente e quindi resta verificabile — a costo di
una clausola di eccezione, che in questa sezione va guardata con sospetto perché
è la forma di ogni riapertura, ma questa è funzione di dati impegnati e non di
una condizione che il quorum fabbrica. Verifica: una fixture che esegua i confini
`e = 1..T` a partire dal set di genesi con i parametri `PD-0` e produca un set
valido a ogni confine, `e = T` compreso.

---

RF-008 | category=security-boundary | severity=high | criterion=Nessuna affermazione sulla soglia di cattura è più forte di ciò che le regole impongono | remediation=Correggere in quattro punti l'affermazione che la soglia sia due terzi, sostituendola con l'orizzonte in confini che il documento stesso calcola

**Il pavimento non riporta la soglia di cattura a due terzi: converte una cattura
di un confine in una cattura di tre.** Il documento afferma, in grassetto, che
«**la soglia effettiva di cattura di questa rete è quindi due terzi, e non un
terzo più epsilon**». La contrazione a più stadi la falsifica, e il documento
**calcola da sé** il numero che la falsifica, due paragrafi più sotto:
`ceil(log(V/k) / log(3/2))` confini.

**Scenario, sviluppato.** Coalizione con `k` appena sopra `V/3`. Al confine 1 non
censura tutto: censura **selettivamente**, lasciando finalizzare esattamente le
candidature oneste che servono a portare il set nuovo a `floor(2V/3) + 1`, il
minimo che il pavimento consente. Sceglie lei quali oneste sopravvivono. Il set
scende a `2V/3`, e la coalizione, ferma a `V/3`, ne è ora **la metà**. Al confine
2 ripete: il set scende a `4V/9`, la coalizione ne è **tre quarti**. Al confine 3
è sopra i due terzi, quindi il pavimento le consente di contrarre fino a sé
stessa: `3 * (V/3) > 2 * (4V/9)` è `V > 8V/9`, vero. **Tre confini**, con
`V/3 + ε` di potere iniziale. La coalizione conserva la capacità di censura a
ogni stadio perché la sua frazione cresce — un terzo, una metà, tre quarti — e i
mandati reggono perché `T >= 3m >= 3`.

**Perché i nodi onesti non lo fermano, ed è la parte che va scritta.** Ogni
blocco di confine ha bisogno di un certificato di quorum del set **vecchio**,
dove la coalizione ha solo un terzo: sono i validatori onesti a firmare la
propria rimozione. Lo fanno perché il blocco è **valido**: la derivazione è
deterministica e ogni full node onesto calcola esattamente quel set ridotto, dal
momento che le candidature censurate non sono mai state finalizzate. Nessuna
regola di questo protocollo autorizza un validatore a rifiutare un blocco valido,
e il residuo **(g)** dello stesso documento spiega perché non potrebbe nemmeno
saperlo: «non può distinguere una rete che perde davvero validatori da una
coalizione che ha negato il quorum». Se non lo può distinguere un light client,
non lo può distinguere nemmeno un validatore. **Il documento contiene la
confutazione della propria affermazione**, a poche righe di distanza: (g) chiude
dicendo che il pavimento garantisce «che il punto d'arrivo del secondo processo
sia uno stallo e non una coalizione che tiene tutto», e il punto d'arrivo è
invece una coalizione che tiene tutto, in tre confini.

**Dove compare, quattro volte.** §"The contraction floor" (l'affermazione in
grassetto e la frase «ciò che la coalizione ottiene censurando è quindi un
arresto … e mai un set di propria scelta»); §"Revocation forces a validator set
transition" («una coalizione può contrarsi fino a sé stessa solo se detiene già
più dei due terzi»); il residuo **(g)** di §"What a light client can establish";
e `threat-model.md`, sia nella porta della contrazione di `TM-18` sia — ed è il
punto peggiore — nell'**esito atteso** della configurazione 2 di `AT-10`, dove
diventa un criterio di test sbagliato che in simulazione verrà smentito e
attribuito all'implementazione.

Quello che il pavimento compra davvero è considerevole e va rivendicato, non
sminuito: la cattura per attrito passa da **un** confine invisibile a **tre**
confini, ciascuno dei quali pubblica un documento firmato che dichiara la propria
contrazione. È esattamente lo standard a cui il tetto di rotazione è tenuto —
«convertire un evento in un processo che qualcuno possa guardare» — e il
pavimento lo raggiunge. Non raggiunge lo standard più forte che le quattro frasi
gli attribuiscono.

**Un'asimmetria da dichiarare insieme alla correzione.** L'orizzonte della
cattura per ammissione è **tarabile**: `3 * c * m <= V` lega il numero di confini
al parametro dichiarato `m`. L'orizzonte della cattura per attrito è **fisso** a
`ceil(log(V/k) / log(3/2))` e non dipende da `m`. Una rete che dichiari `m = 10`
ottiene dieci confini sul percorso di ammissione e tre su quello di attrito, e la
sicurezza di una regola è quella del suo percorso più debole. Chi tara i
parametri deve saperlo.

**Condizione di chiusura, verificabile.** Le quattro occorrenze corrette in modo
che il documento affermi ciò che calcola: la cattura per attrito richiede
`ceil(log(V/k) / log(3/2))` confini, ognuno pubblicamente osservabile, e la
soglia effettiva resta un terzo più epsilon con un costo in confini invece che un
terzo più epsilon a costo nullo. L'esito atteso della configurazione 2 di `AT-10`
allineato allo stesso numero. E l'asimmetria fra orizzonte tarabile e orizzonte
fisso dichiarata — oppure chiusa, se il progetto preferisce, misurando il
pavimento contro il set di `m` confini prima invece che contro quello
immediatamente precedente, cioè `3 * member_count(e) > 2 * member_count(e - m)`,
che lega anche questo percorso al parametro dichiarato senza introdurre alcun
parametro nuovo. La scelta fra dichiarare e chiudere è di progetto; ciò che non è
di progetto è lasciare in piedi le quattro frasi.

---

RF-009 | category=security-boundary | severity=medium | criterion=Il tetto alla variazione dei parametri di elezione limita la velocità della manovra e non soltanto il numero dei documenti | remediation=Un intervallo minimo e un ritardo di attivazione per i documenti che cambiano parametri di elezione, oppure la dichiarazione che il tetto non vincola il tempo

Il tetto di variazione `x_new * den <= x_old * num` è calcolato «contro il
documento attualmente attivo». Nulla impone una distanza minima fra due
`sequence` consecutive: `README.md` §"Signed protocol documents" chiede solo che
`sequence` sia strettamente crescente per kind e che l'attivazione non sia
retroattiva. **Un quorum può quindi pubblicare i documenti `n`, `n+1`, … `n+j` in
altrettanti blocchi consecutivi**, ciascuno dentro il rapporto consentito, e
portare un parametro dal valore corrente al tetto di genesi in
`j ≈ log(max/attuale) / log(num/den)` documenti, cioè in minuti. Il tetto assoluto
di `ElectionBounds` regge — ed è quello che salva l'invariante, quindi RF-001
resta chiuso — ma la proprietà che il documento rivendica per il tasso di
variazione, «convertire un evento in un processo che qualcuno possa guardare»,
non è raggiunta su nessuna scala temporale umana. Un processo che si compie in
tre blocchi non è osservabile in tempo utile: è un evento con più righe di log.

Ne discende anche una piccola sovrastima nella riga di `SEC-REQ-13`, che dichiara
soddisfatta la dipendenza da `SEC-REQ-14` per i parametri di elezione.
`SEC-REQ-14` chiede tre cose: intervallo ammissibile firmato alla genesi, che c'è;
**variazione massima per epoca**, che qui è per documento e non per epoca; e
**ritardo di attivazione dichiarato**, che non c'è.

**Scenario d'attacco.** Un quorum che voglia allungare il proprio mandato residuo
non ha bisogno di superare `validator_max_consecutive_terms_max`: gli basta
arrivarci. Pubblica in un solo blocco la sequenza di documenti che porta `T` al
tetto di genesi, con `activation_height` alla prima altezza utile. Nessun
osservatore ha il tempo di reagire, e la manovra è indistinguibile da una
taratura legittima finché non se ne guarda l'estremo. Il danno resta limitato dal
tetto — è per questo che il finding è `medium` e non critico — ma la difesa in
profondità che il documento dichiara di avere non c'è.

**Condizione di chiusura, verificabile.** Una delle due: (i) un documento
`consensus_parameters` che modifichi un parametro di elezione è accettato solo se
la sua `activation_height` è un confine di elezione e se nessun altro documento
che modifichi parametri di elezione è attivato nella stessa epoca — un vincolo
che dà in una riga sia la variazione per epoca sia il ritardo di attivazione che
`SEC-REQ-14` chiede, e che si accorda con il resto della sezione, dove tutto ciò
che cambia la composizione cambia ai confini; oppure (ii) il paragrafo sul tasso
di variazione dichiara che il tetto limita il numero di documenti e non il tempo,
e la riga di `SEC-REQ-13` dice che la dipendenza da `SEC-REQ-14` è soddisfatta
per l'intervallo e non per gli altri due requisiti. Verifica: alla domanda
«quanto tempo ha la rete per accorgersi che un parametro di elezione sta andando
al proprio estremo?» il documento risponde con una quantità.

---

RF-010 | category=security-boundary | severity=medium | criterion=L'irrigidimento del cooldown non amplifica la leva di censura oltre quanto dichiarato, e la sua durata è vincolata come le altre magnitudini | remediation=Un tetto a `validator_cooldown_epochs`, e la dichiarazione dell'uso avversario del cooldown

**Il rimedio a RF-005 ha reso il cooldown corretto e la censura più economica, e
la seconda metà non è dichiarata.** Con la condizione 5 nella forma precedente,
censurare la candidatura di un membro onesto per un'epoca lo teneva fuori **una**
epoca. Con la forma nuova — «uscita da un seggio per qualunque ragione» — lo
stesso atto di censura lo tiene fuori `1 + validator_cooldown_epochs` epoche,
perché non essere mantenuto *è* uscire da un seggio, quale che ne sia la causa, e
la causa qui è l'avversario. La leva di censura è stata moltiplicata dal
parametro che serviva a impedire l'inerzia.

`validator_cooldown_epochs`, inoltre, **non è fra le magnitudini vincolate da
`ElectionBounds`**: i tetti coprono `election_epoch_blocks`, `T`,
`validator_max_set_size`, il pavimento di `validator_min_set_size` e quello di
`m`. Ho verificato che le altre grandezze non elencate siano innocue rispetto
all'invariante — `candidacy_close_blocks` ed `election_entropy_blocks` sono
limitate transitivamente da `election_epoch_blocks`, `c` da `3*c*m <= V`, `V` da
`validator_max_set_size`, e una soglia di contributo o un
`validator_eligibility_min_issuers` assurdamente alti portano allo stallo e non
al congelamento, che è una perdita di liveness già disponibile a chiunque tenga
più di un terzo semplicemente non votando. Il cooldown è l'eccezione: è l'unica
grandezza non vincolata il cui aumento **aiuta** un avversario invece di fermare
la catena.

**Scenario d'attacco.** Una coalizione che tenga più di due terzi firma un
documento con `validator_cooldown_epochs` al massimo rappresentabile — nessun
vincolo lo impedisce, e il tasso di variazione lo raggiunge in pochi documenti,
per RF-009. Poi, confine dopo confine, censura la candidatura di un solo membro
onesto per volta: uno per confine, ben dentro il pavimento di contrazione, che
tollera fino a un terzo. Ogni membro onesto così colpito è **permanentemente**
ineleggibile. I seggi liberati sono riempiti sotto il tetto ordinario dal pool,
che la coalizione popola con le proprie identità. Il costo per la coalizione è
nullo: anche i suoi membri escono per scadenza di mandato ed entrano nello stesso
cooldown perpetuo, ma un attaccante numeroso ha identità fresche e un operatore
onesto ne ha una. La velocità resta quella del tetto di ingressi, cioè
l'orizzonte `m` già dichiarato, quindi non è un percorso di cattura nuovo: è un
acceleratore che rende **irreversibile** l'esclusione degli onesti lungo il
percorso già dichiarato. Il residuo di numerosità governato da `alpha` viene
misurato assumendo che un nodo onesto escluso possa rientrare; qui non può.

**Condizione di chiusura, verificabile.** (i) `validator_cooldown_epochs` è
vincolato come le altre magnitudini: o un `validator_cooldown_epochs_max` in
`ElectionBounds`, o — a costo zero, perché `T` è già vincolato — la relazione
`validator_cooldown_epochs <= validator_max_consecutive_terms` nel blocco di
accettazione, che dice la cosa sensata: nessuno resta escluso più a lungo di
quanto avrebbe potuto servire. E (ii) §"Degenerate cases" o §"Eligibility"
dichiara che la condizione 5 rende la censura di una singola epoca un'esclusione
di `1 + validator_cooldown_epochs` epoche, perché è il prezzo del rimedio a
RF-005 e va pagato consapevolmente. Verifica: il documento risponde a «quanto
costa a un avversario escludere un nodo onesto per un'epoca?» con
`1 + validator_cooldown_epochs`, e a «quanto può valere quel numero?» con un
tetto.

---

RF-011 | category=correctness | severity=medium | criterion=La lista normativa dei controlli del light client è coerente con le formule che il documento definisce | remediation=Allineare il controllo 7 alla nuova preimmagine del seme

Il controllo **7** di §"What a light client can establish about set composition"
recita ancora che il client stabilisce «che l'`election_seed` impegnato sia
l'hash corretto degli ID di blocco di entropia impegnati, di `candidate_root` e
di `candidate_count`». Dopo RF-003 il seme è
`H(dominio || chain_id || u64be(election_epoch) || election_entropy)` e
`candidate_root` e `candidate_count` **non** sono nella preimmagine. La lista è
normativa e il passo 5 dell'algoritmo di verifica vi rimanda per nome («applica i
controlli da 1 a 10»), quindi un'implementazione conforme al controllo 7 come
scritto calcola un seme diverso da quello di ogni set valido e **rifiuta l'intera
catena**. È un difetto introdotto dalla remediation, non preesistente, ed è
l'unico effetto collaterale che la rimozione ha lasciato: la formula, la
§"The derivation", il registro di `README.md`, la fixture `ELEC-0` e l'esempio
numerico sono stati tutti aggiornati in modo coerente, e li ho verificati uno per
uno.

**Condizione di chiusura.** Il controllo 7 nomina come input del seme gli ID di
blocco di entropia e nient'altro. Verifica: nessuna occorrenza di
`candidate_root` in `ledger.md` lo descrive più come input del seme.

## Verdetto del giro 2

**Changes requested, secondo giro.** Sei finding su sei sono stati affrontati
seriamente e cinque sono chiusi nel merito; RF-002 è chiuso nella regola e non
nell'affermazione. La consegna è migliorata in modo sostanziale: `ElectionBounds`
è la risposta giusta a una classe di problema che il progetto aveva già
incontrato e risolto una volta, il pavimento di contrazione riusa la forma del
predicato di quorum invece di introdurre un parametro nuovo, e la rimozione di
`candidate_root` dalla preimmagine del seme è più di quanto avessi imposto.

I quattro finding nuovi non sono un secondo giro sullo stesso terreno: tre di
essi (RF-007, RF-010, RF-011) **sono conseguenze delle correzioni**, che è ciò
che accade quando si tocca un sistema di regole accoppiate, ed è la ragione per
cui una gate di sicurezza si esegue due volte e non una. RF-007 blocca da solo:
una catena che si ferma da sé al confine `T` non è una regola pronta per essere
implementata. RF-008 blocca perché è la classe che il progetto dichiara peggiore
di un'affermazione mancante, si trova in quattro punti, uno dei quali è un
criterio di test, e il documento contiene già la propria confutazione.

**Bloccanti: RF-007 e RF-008.** Richiesti nella stessa remediation: RF-009,
RF-010, RF-011.

## La forma difendibile del claim, giro 2

La forma che avevo scritto è finita nel documento nella sostanza e non solo nella
lettera: la qualificazione «entro i limiti dei parametri fissati alla genesi» c'è,
la distinzione fra chi **entra** e chi **esce** c'è, il paragrafo dichiara le due
versioni precedenti e perché erano sbagliate — che è la pratica giusta — e le due
affermazioni sovrastimate che avevo trovato sono ritrattate e non ammorbidite,
con `identity.md` che cita la propria formulazione precedente e dice che era più
forte di quanto il protocollo consegni.

Manca una clausola, ed è quella di RF-008. La forma difendibile oggi è:

> Entro i limiti dei parametri fissati alla genesi, un light client stabilisce
> che il set attivo è **di forma lecita e in rotazione lecita**, e non che sia il
> set che la regola di eleggibilità avrebbe dovuto produrre. Il pavimento di
> contrazione non riporta la soglia di cattura a due terzi: la lascia a un terzo
> più epsilon e le impone un costo di `ceil(log(V/k)/log(3/2))` confini, ognuno
> dei quali pubblica la propria contrazione in un documento firmato. È lo stesso
> standard del tetto di ingressi — un evento convertito in un processo — con la
> differenza che l'orizzonte del percorso di ammissione è tarabile con `m` e
> quello del percorso di attrito è fisso. Delle cinque forme di composizione
> scorretta, tre sono rilevabili da un nodo che rigioca la storia e due da
> nessuno; due delle tre sono contraddicibili con un messaggio corto.

Con questa clausola il claim è difendibile senza altre dichiarazioni accanto. Con
la frase sui due terzi al suo posto, non lo è.

---

# Giro 3 — verifica della remediation

## Verifica della remediation (giro 3)

**RF-007 — chiuso, e la scelta di correggere la sincronizzazione invece del
pavimento è quella giusta.** Il rifiuto dell'esenzione su `R = ∅` è motivato
correttamente e con il precedente giusto: un'esenzione vale quanto la difficoltà
di fabbricarne il trigger, e questa era gratuita da fabbricare. La causa è la
sincronizzazione, la sincronizzazione nasce alla genesi, e la genesi è l'unico
posto dove nessun quorum ha voce. Il vincolo `3c < V` aggiunto al blocco è la
condizione giusta e non l'avevo isolata io: garantisce che un confine in cui una
coorte intera scade **e nessuno viene insediato** produca ancora un set valido.

**Ho verificato la proprietà di automantenimento, che è ciò che mi era stato
chiesto, e ho trovato la porta che la apre.** L'argomento del documento è: le
scadenze al confine `e` sono i timbri scritti al confine `e - T`, i riempimenti
per confine sono al più `c`, quindi al più `c` scadenze per confine, per sempre.
L'induzione è corretta **a `T` costante** e falsa quando `T` diminuisce: i
timbri sono `e + T(e)`, e per `e1 < e2` la collisione `e1 + T(e1) = e2 + T(e2)`
richiede esattamente `T(e2) < T(e1)`. È RF-012.

Verificate invece come **prive** di percorsi di risincronizzazione: la genesi
verso i riempimenti (i timbri di genesi stanno in `[1, T]` e il primo
riempimento timbra `1 + T`, quindi non collidono mai); le transizioni di sola
rimozione (la regola 7 impone l'identità voce per voce **compreso**
`term_expiry_epoch`, quindi una revoca non riscrive alcun timbro); e il rientro
di un membro uscito, che passa per il cooldown e per un'estrazione e riceve un
timbro nuovo.

**Il difetto che il timbro chiude e che nessuno dei due aveva sollevato: la
chiusura è completa.** Nella forma derivata, un quorum che alzasse `T` entro il
tetto di genesi allungava il mandato dei propri membri **già seduti**, perché la
scadenza veniva ricalcolata contro il valore nuovo. Ho verificato la chiusura su
tutti e quattro i punti in cui poteva perdere: il timbro è assegnato
all'insediamento e il passo 1 impone che un membro mantenuto lo conservi
invariato; il passo 6 lo calcola sui parametri attivi al confine di
insediamento; la regola 7 delle transizioni di sola rimozione lo preserva; e il
controllo 5 del light client verifica **entrambi** i valori attraverso due set
adiacenti, invariati per chi resta ed esattamente `election_epoch + T` per chi
entra. Non esiste percorso per allungare un mandato in corso. Il controllo 4
dichiara anche la proprietà a parole, il che è giusto perché è il motivo per cui
il campo esiste.

**RF-008 — chiuso, e meglio di una toppa.** Il §"What the floor does not buy,
stated before what it does" è il modo corretto di ritrattare: dichiara l'errore,
cita la propria confutazione, esibisce la successione `V → 2V/3 → 4V/9 → k` con
il numero di confini, dice che i nodi onesti firmano ciascuno di quei blocchi
perché sono validi, e conclude che la soglia effettiva resta appena sopra un
terzo. La dichiarazione dell'asimmetria fra orizzonte tarabile e orizzonte fisso
c'è, e l'avvertimento che abbassare `c` compra sicurezza sul percorso che era già
il più lento è un'aggiunta sua che vale la pena registrare. La frase di
§"Revocation" è corretta con «in un solo passo … sotto quella soglia ci arriva in
più transizioni, ciascuna pubblicata». Il criterio di test di `AT-10` è corretto
e sdoppiato in censura totale e selettiva, con la nota che un criterio errato
verrebbe attribuito all'implementazione: è esattamente il punto che mi
preoccupava.

Restano due dettagli, entrambi minori e nessuno dei due nella regola: il residuo
**(g)** non è stato toccato (RF-013) e la motivazione della scelta fra
ritrattazione e regola è imprecisa (RF-014).

**RF-009 — chiuso, e lo spaziamento è davvero per unità di catena.** Il vincolo
è `activation_height(new) >= activation_height(active) +
election_parameter_min_activation_gap_blocks`, cioè in altezze e non in numero di
documenti: è la grandezza giusta. Ho verificato il punto che rendeva il rimedio
inutile se sbagliato: `election_parameter_min_activation_gap_blocks` sta in
`ElectionBounds`, quindi alla genesi e fuori dalla governance della catena, ed è
obbligato positivo. Se fosse stato un parametro di consenso, un quorum lo avrebbe
azzerato e saremmo tornati a RF-001; non lo è.

**RF-010 — chiuso.** `validator_cooldown_epochs <= T` nel blocco, con la
motivazione scritta per esteso: il cooldown è l'unica grandezza di elezione il
cui **aumento** aiuta un avversario, e `T` è il tetto naturale perché chi è
escluso più a lungo di un mandato intero è escluso e basta. L'amplificazione
della leva di censura — un'epoca di censura costa `1 + validator_cooldown_epochs`
epoche di assenza — è dichiarata dove serve.

**RF-011 — chiuso.** Il controllo 7 nomina gli ID di blocco di entropia «and of
nothing else» e dice esplicitamente che `candidate_root` e `candidate_count` sono
impegnati dall'`election` record ma non sono input del seme. Un'implementazione
che segua la lista ora calcola il seme giusto.

**Sul rilievo di AGENT-002 riguardo alle suite di conformità.** Ha ragione e vale
oltre l'hash. Qualunque suite già scritta contro `T = 3` non codificava soltanto
un valore diverso: codificava un insieme di parametri che il blocco ora
**rifiuta**, cioè uno stato che una rete conforme non può raggiungere. Non è un
aggiornamento di fixture, è la rimozione di un caso di prova impossibile, e va
portato nella spec di implementazione di M-02 come nota, non lasciato alla
memoria di chi scriverà i test.

## Review findings, giro 3

<!-- Stable form: RF-0NN | category=... | severity=... | criterion=... | remediation=... -->

RF-012 | category=security-boundary | severity=high | criterion=La proprietà di scaglionamento delle scadenze si mantiene sotto ogni operazione che il protocollo permette, oppure il documento dichiara sotto quali non si mantiene | remediation=Rendere `validator_max_consecutive_terms` monotono non decrescente, oppure condizionare la sua riduzione all'assenza di collisione con i timbri in volo

**Abbassare `T` risincronizza le coorti, e il documento afferma che la proprietà
si mantiene da sé senza altre regole.** Un seggio riempito al confine `e` è
timbrato `e + T(e)`, dove `T(e)` è il valore attivo a quel confine. Per `e1 <
e2`, due coorti scadono insieme quando `e1 + T(e1) = e2 + T(e2)`, cioè
esattamente quando `T(e2) = T(e1) - (e2 - e1)`: **le collisioni esistono se e
solo se `T` diminuisce**. Il documento scrive invece, senza condizione, che
«thereafter the property maintains itself without further rules», e su
quell'affermazione poggia il vincolo `3c < V`, che è dimensionato su **una**
coorte per confine.

**Scenario, senza avversario e con parametri ammissibili.** `V = 12`, `c = 3`,
`m = 1`, `T = 6`. Il blocco è soddisfatto: `ceil(12/6) = 2 <= 3`, `3*3 = 9 < 12`,
`3*3*1 = 9 <= 12`, `T >= max(4, 3) = 6`. L'operatore, su indicazione del
simulatore, accorcia i mandati: `T` da 6 a 5 e poi a 4, un passo per epoca,
dentro il rapporto di variazione e dentro lo spaziamento minimo, e ogni documento
resta valido — `ceil(12/5) = 3 <= 3` e `ceil(12/4) = 3 <= 3`. I riempimenti del
confine `e` sono timbrati `e + 6`; quelli del confine `e + 1`, con `T = 5`, sono
timbrati `e + 6`; quelli del confine `e + 2`, con `T = 4`, ancora `e + 6`. **Tre
coorti, fino a nove voci su dodici, scadono allo stesso confine `e + 6`.** Allora
`|R| = 3` e `fills = min(max(0, 12 - 3), 3, |Nw|) = 3`, quindi il set nuovo ha
sei membri e il pavimento richiede `3 * 6 > 2 * 12`, cioè `18 > 24`, falso.
**Nessun set valido esiste e la catena si ferma al confine `e + 6`**, con ripresa
solo fuori banda.

Due aggravanti rispetto a RF-007, di cui questo è il ritorno da un'altra porta.
La prima: **il pool pieno non salva**, perché a fermare la ricostruzione non è la
mancanza di candidati ma il tetto di ingressi, che può rimpiazzare tre dei nove
seggi liberati. La seconda: non serve alcun avversario. Il documento che abbassa
`T` richiede la firma di un quorum, quindi un attaccante non guadagna nulla che
non abbia già — è per questo che il finding è `high` e non `critical` — ma
l'operatore onesto che accorcia i mandati dopo la taratura del simulatore compie
un'operazione ovvia, permessa, incoraggiata dal fatto che i valori vengono da
M-02, e non riceve alcun avvertimento. Il guasto arriva `T` confini dopo, a
un'altezza calcolabile, quando nessuno collega più le due cose.

**Condizione di chiusura, verificabile.** Una delle due, entrambe di una riga nel
blocco di accettazione:

- (i) **`validator_max_consecutive_terms` è monotono non decrescente su una
  catena viva**: un documento che lo abbassa è rifiutato in accettazione. È
  sufficiente, e la dimostrazione è breve: se `T` non decresce allora per
  `e1 < e2` vale `e1 + T(e1) < e2 + T(e2)`, quindi i timbri di confini distinti
  non collidono mai, le scadenze a ogni confine vengono da una sola coorte e sono
  al più `c`; i timbri di genesi stanno in `[1, T]` e sono strettamente sotto il
  primo timbro di riempimento, quindi non collidono con nessuno. Costo: i mandati
  si possono allungare e non accorciare, il che è poco vista la forma timbrata,
  dove allungare `T` **non** estende i mandati in corso;
- (ii) **una riduzione di `T` è accettata solo se non collide con i timbri in
  volo**: attivandosi all'epoca `e`, si richiede `e + T_new > max(term_expiry_epoch)`
  sul set attivo. È deterministica, si calcola su dati che anche un light client
  possiede, e conserva la possibilità di accorciare i mandati dopo che i timbri
  precedenti si sono esauriti.

In entrambi i casi la frase «thereafter the property maintains itself without
further rules» diventa vera e può restare, citando la regola che la rende tale;
senza una delle due va qualificata con «finché `T` non diminuisce». Verifica: una
simulazione che abbassi `T` attraverso più confini e mostri che a nessun confine
scadono più di `c` voci — oppure che il documento che abbassa `T` sia rifiutato.

---

RF-013 | category=documentation | severity=low | criterion=I residui del light client descrivono la variante del vettore che sopravvive alle regole, non quella che le regole rifiutano | remediation=Estendere il residuo (g) alla censura selettiva

Il residuo **(g)** descrive «una coalizione che ha negato il quorum a **ogni**
blocco che portasse la candidatura di qualcun altro» e conclude che «ciò che il
pavimento garantisce è che il punto d'arrivo del secondo processo sia uno stallo
e non una coalizione che tiene tutto». Preso alla lettera è **vero**: quella è la
censura totale, e la censura totale il pavimento la rifiuta davvero. Non lo
segnalo quindi come affermazione falsa. Il difetto è che l'unica variante
nominata nell'elenco dei residui è quella che **non** è il vettore: §"What the
floor does not buy" e la configurazione 2b di `AT-10` stabiliscono entrambe che
il vettore reale è la censura **selettiva**, e (g) non la menziona. Un lettore
che consulti l'elenco dei residui — che è il posto dove il documento dichiara che
cosa non promette — ne ricava la versione rassicurante. La distinzione poggia
oggi tutta sulla parola «ogni».

**Condizione di chiusura.** (g) nomina entrambe le varianti e dice quale
sopravvive: la censura totale porta allo stallo, la selettiva porta al set della
coalizione in `ceil(log(V/k)/log(3/2))` confini, e in nessuno dei due casi il
client può distinguerla da una perdita genuina di validatori.

---

RF-014 | category=documentation | severity=low | criterion=Il motivo per cui un'alternativa è stata scartata è quello vero | remediation=Registrare il pavimento cumulativo come alternativa considerata e scartata, con il suo costo reale

La motivazione data per chiudere RF-008 con una ritrattazione invece che con una
regola è che una regola contro la contrazione selettiva dovrebbe distinguere una
candidatura censurata da una mai inviata, cosa che il residuo (h) dichiara
impossibile a ogni verificatore. **La conclusione è accettabile — l'avevo
autorizzata esplicitamente — ma l'argomento non regge**, e conviene che il
registro non dica «impossibile» dove il vero motivo è «costoso».

Una regola esiste e non richiede quella distinzione: il **pavimento cumulativo**
`3 * member_count(e) > 2 * member_count(e - m)`, cioè lo stesso pavimento
misurato sull'orizzonte dichiarato invece che sul confine singolo. Non chiede mai
*perché* un membro sia uscito, esattamente come il pavimento per confine non lo
chiede; si calcola su `member_count` di set che il light client già conserva; e
lega l'orizzonte dell'attrito a `m`, chiudendo l'asimmetria che il documento
adesso dichiara. Il motivo reale per preferire la dichiarazione è il suo costo in
liveness: una rete che si restringa legittimamente di più di un terzo nell'arco
di `m` confini si ferma, e con `m` grande è un vincolo severo su una rete che
voglia rimpicciolirsi in modo ordinato. Quello è un buon motivo. «Non esiste una
regola onesta da scrivere» non lo è, ed è la frase che un agente futuro leggerà
come una dimostrazione di impossibilità.

**Condizione di chiusura.** Il pavimento cumulativo compare fra le alternative
considerate e scartate, nella stessa forma già usata due volte in questa sezione
per il commit-reveal dei candidati e per il seme derivato dagli `issuer_reveal`,
con il suo costo dichiarato. Verifica: il documento risponde a «esiste una regola
che chiuderebbe la contrazione selettiva?» con «sì, e non la prendiamo perché»,
non con «no».

---

RF-015 | category=documentation | severity=low | criterion=Le affermazioni normative sulla forma dello spazio dei parametri sono esatte | remediation=Correggere l'affermazione sull'istanza minima

Rilievo del Lead, che ho verificato in modo indipendente e che **sta in piedi**:
non ho trovato alcun vincolo aggiuntivo che escluda il controesempio. `README.md`
afferma che «`T:"4"` con `V:"12"` e `c:"3"` è la più piccola istanza che
soddisfa ogni vincolo insieme». Con `V = 4`, `T = 4`, `c = 1`, `m = 1`:
`ceil(4/4) = 1 <= 1`; `3*1 = 3 < 4`; `3*1*1 = 3 <= 4`; `T >= max(4, 3m) = 4`;
`cooldown = 1 <= T`; `min_issuers = 2`; e i vincoli su finestra di entropia,
chiusura delle candidature e lunghezza d'epoca sono indipendenti da questi. Ho
enumerato anche il basso: `V = 3` richiede `3c < 3` con `c >= 1`, impossibile, e
a maggior ragione `V < 3`. **L'istanza minima è `V = 4, T = 4, c = 1, m = 1`.**

Non è un difetto della regola e la severità è bassa. Ma è un'affermazione
normativa falsa in un paragrafo il cui scopo dichiarato è insegnare la forma
dello spazio dei parametri — «a fixture that did not would be teaching the wrong
shape» — e insegnarla sbagliata è il difetto che quel paragrafo dice di voler
evitare. La chiusura non richiede di cambiare il fixture: basta dire il vero, che
l'istanza minima è `V = 4` e che il fixture ne usa una più grande perché una
coorte di `c = 1` non eserciterebbe il tetto.

**Condizione di chiusura.** L'affermazione riporta l'istanza minima corretta,
oppure qualifica «più piccola» con la proprietà che la rende tale. Verifica:
l'enumerazione delle triple ammissibili con `V` piccolo conferma la frase scritta.

## Verdetto del giro 3

**Changes requested, terzo e ultimo giro, per un solo finding bloccante.**

I quattro finding del giro 2 sono chiusi, e due di essi meglio di come li avevo
chiesti: RF-007 è stato risolto correggendo la sincronizzazione invece di
indebolire il pavimento, con il rifiuto motivato dell'esenzione su `R = ∅` e con
il vincolo `3c < V` che non avevo isolato io; e il timbro `term_expiry_epoch`
chiude per soprammercato l'estensione retroattiva dei mandati, che era un difetto
reale che nessuno dei due aveva visto e la cui chiusura ho verificato su tutti e
quattro i punti in cui poteva perdere. RF-008 è stato chiuso con una
ritrattazione completa e con il criterio di test corretto in entrambe le
varianti. RF-009, RF-010 e RF-011 sono chiusi, e lo spaziamento è in altezze di
catena e ancorato alla genesi, che erano le due cose che potevano renderlo
inutile.

**Blocca solo RF-012**, ed è il ritorno di RF-007 dalla porta dei parametri:
abbassare `T` risincronizza le coorti, il tetto di ingressi non basta a
ricostruire il set, e la catena si ferma a un confine calcolabile senza che
nessun avversario abbia fatto nulla. È l'ultima cosa sostanziale che ho: ho
attaccato questa regola per tre giri e non ho altro. La chiusura è una riga nel
blocco di accettazione, in una delle due forme che ho scritto, con la
dimostrazione di sufficienza per la prima.

RF-013, RF-014 e RF-015 sono di severità bassa e vanno nella stessa remediation
perché sono correzioni di testo di poche righe l'una, non perché da soli
giustificherebbero un giro.

## La forma difendibile del claim, giro 3

Il paragrafo del claim è ora quello giusto e non ho modifiche da chiedergli. Ha
la qualificazione sui limiti di genesi, la distinzione fra chi entra e chi esce,
e la frase che avevo indicato al giro 2 nella sostanza: «il limite alla cattura è
un **numero di confini pubblicati**, non una quota di potere di voto: la soglia
effettiva resta appena sopra un terzo, e ciò che le regole comprano è che
arrivarci richiede più transizioni, ognuna delle quali il client può vedere».
Registra inoltre tutte e tre le versioni precedenti e perché erano sbagliate, che
è la pratica che il progetto dovrebbe adottare ovunque.

Chiuso RF-012, la sola cosa che aggiungerei è la clausola che ne discende, e va
accanto alla prima frase e non dentro:

> Le grandezze che reggono la proprietà sono fissate alla genesi e quelle che si
> possono muovere si muovono sotto un rapporto, uno spaziamento in altezze di
> catena e — per il limite di mandato — in una sola direzione. La proprietà è
> forte quanto quei tre vincoli, e non più.

Con quella, il claim è difendibile senza dichiarazioni accanto.

---

# Giro 4 — verifica della remediation e accettazione

## Verifica della remediation (giro 4)

**RF-012 — chiuso, e la dimostrazione che (ii) collassa su (i) regge.** L'ho
rifatta invece di accettarla, perché è la parte che va oltre il finding. Al
momento dell'accettazione i timbri in volo **all'attivazione** non sono noti:
fra le due altezze può cadere un confine, e una coorte insediata al confine
`e_a - 1` è timbrata con il `T` vecchio, cioè `(e_a - 1) + T_old`. La condizione
che un controllo in accettazione può garantire in modo conservativo è quindi
`e_a + T_new > (e_a - 1) + T_old`, che è `T_new >= T_old`. È esattamente la
regola monotona, e la forma permissiva (ii) — quella che avevo scritto io —
collassa davvero su di essa. La dimostrazione è corretta.

Due precisazioni che la rendono più forte, non più debole, e che vanno al futuro
implementatore più che a questa spec:

- **il collasso dipende dal fatto che un confine cada fra accettazione e
  attivazione.** Se non ne cade nessuno, l'insieme dei timbri occupati è già
  definitivo all'accettazione e un controllo esatto sarebbe possibile. La
  spaziatura minima è in altezze e non in confini, quindi il caso esiste. Non
  chiedo di sfruttarlo: appoggiare una regola di sicurezza su «di solito un
  confine cade in mezzo» sarebbe fragile, e la regola monotona è sicura in
  entrambi i casi. Lo scrivo perché l'affermazione «non è valutabile in
  accettazione» è vera nel caso che conta e non in generale;
- **la regola permissiva che funzionerebbe davvero all'attivazione non è quella
  scritta.** `e + T_new > max(term_expiry_epoch)` è sufficiente e più stretta del
  necessario. Dopo l'attivazione nessun timbro con `T_old` viene più creato,
  quindi l'insieme occupato è definitivo e la condizione esatta è che **nessuno**
  dei valori `{ e + T_new : e_a <= e <= e_a - 1 + T_old - T_new }` — la finestra
  di sovrapposizione, lunga `T_old - T_new` — sia un timbro occupato. Un
  controllo del solo valore `e_a + T_new` non basta, perché la collisione si
  ripresenta a ogni confine della finestra. Con la finestra, l'affermazione che
  la regola permissiva sarebbe «strictly better» è corretta; con il solo massimo,
  sarebbe corretta ma inutilmente restrittiva.

**Sul costo dichiarato della porta a senso unico: lo giudico accettabile, con una
conseguenza da aggiungere.** È accettabile per due ragioni. La prima è che il
raggio del danno è limitato da `validator_max_consecutive_terms_max`, che sta
alla genesi e fuori dalla governance della catena: `T` può salire, ma non oltre
un valore che il quorum non controlla. La seconda è che il costo è **dichiarato**
nella forma giusta, come rifiuto sul costo e non come impossibilità, con la via
di ripristino nominata.

La conseguenza da aggiungere è più affilata di «una rete che parte con mandati
troppo lunghi non può correggerli»: **il cricchetto è spingibile da un
avversario e non è tirabile indietro da nessuno.** Un quorum che raggiunga i due
terzi anche una sola volta porta `T` al tetto di genesi in modo permanente, e
nessun quorum onesto successivo può riportarlo giù. Contro un avversario stabile
sopra i due terzi questo non aggiunge nulla — a quel punto ha già tutto — ma
contro un quorum **transitorio**, o contro un errore dell'operatore, lascia il
pavimento di ricambio degradato per sempre, e da quel momento
`validator_max_consecutive_terms_max` è l'**unico** presidio residuo sul
ricambio. Ne segue una regola operativa: quel tetto va scelto alla genesi tanto
stretto quanto la rete tollera, perché è l'unica cosa che nessuno potrà più
correggere. Lo registro qui e nel threat model; non lo chiedo come finding.

**RF-014 — chiuso esattamente come chiesto.** L'esposizione del pavimento
cumulativo è corretta: `3 * member_count(e) > 2 * member_count(e - m)` non chiede
mai *perché* un membro sia uscito, quindi non richiede la distinzione che (h)
dichiara impossibile, ed è verificabile su `member_count` di documenti che il
client conserva. Il rifiuto è motivato dove va motivato — il costo in liveness è
pagato dalle reti oneste, e cresce con `m` — e la distinzione generale fra un
costo, che una versione successiva può rivedere, e un'impossibilità, che dice al
lettore successivo di smettere di cercare, è scritta meglio di come l'avevo
formulata io. Va bene che resti.

**RF-015 — chiuso senza toccare il fixture e con la ragione giusta**, ma la
sostituzione introduce una seconda affermazione di minimalità, ed è a sua volta
falsa: è RF-016.

**Il riporto sulle suite di conformità sta dove gli autori di suite guardano
davvero.** È in §*Hash conformance fixtures*, subito dopo l'obbligo di
ricostruire ogni preimmagine, cioè nel paragrafo che una suite legge per sapere
che cosa deve fare. È normativo, dice la cosa giusta — un caso costruito su
parametri inammissibili non prova un valore diverso, asserisce il comportamento
in uno stato irraggiungibile — e impone la rimozione invece della correzione, che
è il verbo giusto: un caso di prova impossibile non si aggiusta.

**Verifiche di regressione, dato che ogni giro ne ha introdotta una.** L'esempio
numerico è stato riportato dentro lo spazio ammissibile — `V = 8`, `T = 4`,
`c = 2`, `m = 1` — e ho verificato il blocco su quei valori: `ceil(8/4) = 2 <= 2`,
`3*2 = 6 < 8`, `3*2*1 = 6 <= 8`, `T >= max(4, 3)`, `cooldown 1 <= 4`. I timbri
della tabella sono coerenti con la regola che li assegna (`02` insediato a 2 e
timbrato 6, `04` a 1 timbrato 5, `01` di genesi con timbro in `[1, 4]`),
`fills = min(max(0, 8-2), 2, 3) = 2` con il tetto che vincola, e il pavimento
`3*4 > 2*4` è verificato nell'esempio insieme alla controprova dei due soli
superstiti. Era il punto in cui l'esempio poteva restare indietro rispetto ai
vincoli nuovi — `T = 3`, che l'esempio usava, è ora inammissibile a ogni `V` — e
non è restato indietro. Il vincolo `T_new >= T_active` è nel blocco.

## Review findings, giro 4

<!-- Stable form: RF-0NN | category=... | severity=... | criterion=... | remediation=... -->

RF-013 | category=documentation | severity=low | criterion=I residui del light client descrivono la variante del vettore che sopravvive alle regole, non quella che le regole rifiutano | remediation=Estendere il residuo (g) alla censura selettiva | **non chiuso al giro 3**

Il residuo **(g)** è invariato: descrive ancora soltanto la coalizione «che ha
negato il quorum a **ogni** blocco che portasse la candidatura di qualcun altro»
e conclude che il pavimento garantisce che il punto d'arrivo sia uno stallo. Come
al giro 3, preso alla lettera è **vero** — quella è la censura totale, e la
censura totale il pavimento la rifiuta — e non lo segnalo come affermazione
falsa. Resta che l'unica variante nominata nell'elenco dei residui è quella che
**non** è il vettore, mentre §*What the floor does not buy* e la configurazione
2b di `AT-10` stabiliscono entrambe che il vettore reale è la censura
**selettiva**; e che §*What the floor does not buy* cita (g) come propria
confutazione, il che rende le due voci apertamente discordi per un lettore che
segua il rimando. La distinzione poggia tutta sulla parola «ogni».

Non blocca, ed è coerente con ciò che ho scritto al giro 3: rilievi di questa
dimensione non giustificano un giro per conto proprio. Va però chiuso, perché
l'elenco dei residui è il posto in cui il documento dichiara che cosa non
promette, ed è quello che un lettore futuro citerà.

**Condizione di chiusura, invariata.** (g) nomina entrambe le varianti e dice
quale sopravvive: la censura totale porta allo stallo, la selettiva porta al set
della coalizione in `ceil(log(V/k)/log(3/2))` confini, e in nessuno dei due casi
il client la distingue da una perdita genuina di validatori.

---

RF-016 | category=documentation | severity=low | criterion=Le affermazioni di minimalità sullo spazio dei parametri sono enumerate e non asserite | remediation=Correggere la seconda affermazione, o togliere il superlativo

La chiusura di RF-015 è corretta — l'istanza minima ammissibile è davvero
`V = 4, T = 4, c = 1, m = 1`, e la ragione data per usarne una più grande, che
con `c = 1` una coorte è un seggio solo e il tetto non viene mai esercitato, è la
migliore delle due che avevo suggerito. Ma la frase che chiude il paragrafo
afferma che «`V:"12"`, `T:"4"`, `c:"3"` è la più piccola istanza che esercita
tutte e tre», e anche questa non regge.

Il criterio implicito è `c >= 2`, perché è ciò che rende una coorte più di un
seggio. Da `3c < V` con `c >= 2` segue `V > 6`, quindi `V >= 7`; e
**`V = 7, T = 4, c = 2, m = 1` è ammissibile**: `ceil(7/4) = 2 <= 2`,
`3*2 = 6 < 7`, `3*2*1 = 6 <= 7`, `T >= max(4, 3) = 4`, `cooldown 1 <= 4`. Il
tetto vi si esercita — con `|R| = 5` e un pool di tre, `fills = min(2, 2, 3) = 2`
vincola — lo scaglionamento è realizzabile, perché quattro valori di scadenza per
al più due voci ciascuno bastano a sette membri, e il pavimento di contrazione vi
è esercitato nel punto più stretto che il blocco consenta, `6 < 7`. L'istanza
minima che esercita tutte e tre è quindi `V = 7`, non `V = 12`.

È lo stesso rilievo di RF-015 una riga più in basso, ed è il motivo per cui la
chiusura giusta non è calcolare un nuovo minimo: è **smettere di asserire
minimalità** dove non serve. Al fixture basta che `c > 1`; quale sia la più
piccola istanza con quella proprietà non è informazione che il paragrafo debba
dare, e ogni volta che la dà deve enumerare per poterlo affermare.

**Condizione di chiusura.** O la frase riporta `V = 7, T = 4, c = 2, m = 1`, o —
preferibile — il superlativo sparisce: «questo fixture ne usa una più grande
perché con `c = 1` il tetto non viene mai esercitato». Verifica: nel paragrafo
non resta alcuna affermazione di minimalità priva della propria enumerazione.

## Verdetto del giro 4

**Accettata.** `GATE-SECREVIEW` è soddisfatta.

La regola di elezione e rotazione, come superficie di sicurezza, tiene. L'ho
attaccata per quattro giri e non ho altro. Le due porte critiche del giro 1 — il
documento dei parametri firmato dal quorum e la contrazione priva di pavimento —
sono chiuse con regole, non con dichiarazioni. Le due porte del giro 2, il
difetto di correttezza della coorte di genesi e la soglia di cattura
sovrastimata, sono chiuse rispettivamente correggendo la sincronizzazione anziché
indebolire il pavimento, e ritrattando per intero. La porta del giro 3, la
risincronizzazione per riduzione di `T`, è chiusa con la monotonicità e con una
dimostrazione che ho verificato e che regge.

Restano due rilievi di severità bassa, RF-013 e RF-016, entrambi di poche righe e
nessuno dei due nella regola. Non bloccano, e sarebbe incoerente da parte mia
sostenere il contrario dopo aver scritto al giro 3 che rilievi di questa
dimensione non giustificano un giro per conto proprio. **Raccomando al Lead di
farli applicare prima di `spec_done`**, dato che AGENT-002 è già nel ciclo e la
correzione è di due frasi; se il Lead preferisce chiudere subito, vanno registrati
come debito documentale, perché l'elenco dei residui è l'artefatto che un lettore
futuro citerà per sapere che cosa il protocollo non promette.

**Raccomando inoltre un debito separato** per la porta a senso unico sul limite
di mandato: si risolve quando i documenti governati avranno un'attivazione
condizionata allo stato della catena, che è la condizione sotto la quale la
regola permissiva — nella forma a finestra, non nella forma a massimo — diventa
disponibile. Non è materia di questa spec e non va persa.

## La forma difendibile del claim, giro 4

Questa è la forma che ritengo difendibile, ed è quella che il documento ha:

> Entro i limiti di parametro fissati alla genesi, un light client stabilisce che
> il set attivo è di forma lecita e in rotazione lecita, e non che sia il set che
> la regola di eleggibilità avrebbe dovuto produrre. Il limite alla cattura è un
> **numero di confini pubblicati** e non una quota di potere di voto: la soglia
> effettiva resta appena sopra un terzo, e ciò che le regole comprano è che
> arrivarci richiede più transizioni, ognuna delle quali il client può vedere.
> Delle cinque forme di composizione scorretta, tre sono rilevabili da un nodo
> che rigioca la storia — due contraddicibili con un messaggio corto — e due da
> nessuno.

Ci aggiungerei una sola clausola, che discende dalla monotonicità e che oggi è
implicita: le grandezze che reggono la proprietà sono fissate alla genesi, e
quelle che si muovono si muovono sotto un rapporto, una spaziatura in altezze di
catena e — per il limite di mandato — **in una sola direzione, che nessuno può
invertire**. La proprietà è forte quanto quei tre vincoli e quanto il canale che
ha consegnato i primi.

Con quella clausola il claim di sicurezza sulla composizione del set è
difendibile **senza dichiarazioni accanto**, che è la posizione in cui il progetto
voleva stare e in cui, per questa regola, ora sta.
