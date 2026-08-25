---
id: REVIEW-014
# Note: Quote the title if it contains a colon
title: "Security review di SPEC-009 (GATE-SECREVIEW): RewardBounds, regole di validità economiche, claim in due regimi"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-009
reviewer: AGENT-007
review_requested_by: project-lead
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, requirements-completeness, documentation, provenance, verification-integrity]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-014-EVENT-001"
    timestamp: "2026-08-25T17:32:17.177569500+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-014-EVENT-002"
    timestamp: "2026-08-25T17:36:08.775455100+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "GATE-SECREVIEW non superata. Otto findings, tre dei quali bloccanti su proprieta' di sicurezza dichiarate.\n\nRF-001 (critical). L'invariante dei due terzi e' confutato una terza volta. Il vincolo 3*min_set >= 2*V impedisce a una coalizione sotto i due terzi di POSSEDERE l'intero set, non di ottenerne il QUORUM. Con V=27, min_set=18: una coalizione di 13 seggi su 27 (48,1%) censura selettivamente lasciando passare 6 candidature oneste, il set si contrae a 19 (pavimento stretto 57>54, min_set 18<=19, entrambi soddisfatti, blocco indistinguibile da uno onesto e firmato dagli onesti), e 13 su 19 da' 39>38, cioe' quorum. Da li' abbassa V a 22 e poi a 18 dentro il rapporto 5/4 e arriva al possesso integrale in tre confini. Soglia reale k_min = ceil((2*(floor(2V/3)+1)+1)/3): V=12 -> 7 (58,3%), V=27 -> 13 (48,1%), V=36 -> 17 (47,2%), V=60 -> 28 (46,7%), asintoto 4V/9 = 44,4%. Il guadagno e' reale ma sono quattro noni, non due terzi. Aggravante: la narrazione riscritta riabilita la terza versione come \"prematura e non sbagliata\" quando era sbagliata per la stessa ragione per cui lo e' la quarta.\n\nRF-002 (critical). RewardBounds e' ristretto e dichiarato chiuso. reward_epoch_ms e' in RewardPolicyBody, senza limite di magnitudine, ed e' il denominatore di ogni tetto vincolato: un documento conforme che porta l'epoca da 86.400.000 ms a 86.400 ms moltiplica per mille l'emissione reale e l'importo assoluto a rischio della rampa, senza violare nulla. La ragione dichiarata (\"non crea un vettore Sybil\") risponde alla domanda sbagliata. Il documento inoltre si contraddice su quali parametri il rapporto 5/4 vincoli (\"bounded reward parameters\" contro \"any parameter\").\n\nRF-004 (high). La meta' matura del claim non e' tenuta da alcuna regola. Il denominatore di alpha e' prezzato da storage_microtokens_per_byte_epoch e compute_microtokens_per_million_fuel, che sono governati, non vincolati e possono valere zero: un quorum che li azzera riporta una rete matura ad alpha = 1 senza traccia distinguibile. Per il criterio di ADR-010 il regime maturo e' oggi una preferenza, ed e' dichiarato \"coperto\" in SEC-REQ-14 e SEC-REQ-18.\n\nRF-003 (high): il pavimento su validator_eligibility_threshold_units e' denominato in un'unita' che la governance definisce (storage/compute_units_per_contribution_unit, vincolati solo a >0), e validator_eligibility_window_epochs non ha massimo; la ragione scritta per lasciarli fuori e' falsa in entrambe le sue affermazioni. RF-005 (medium): il prezzo in liveness del vincolo relazionale non e' dichiarato in ledger.md dove la regola vive, e il vincolo consuma per intero il margine di contrazione. RF-006 (medium): due dichiarazioni imposte da REVIEW-011 RF-005 sono state cancellate dal rapporto economico fuori ambito. RF-007 (medium): RewardBounds non ha alcun caso rifiutato nel registro normativo e gli script di scratch non esistono nell'albero. RF-008 (low): la garanzia assoluta e' presentata come garanzia di fase quando e' incondizionata, lasciando apparire un regime intermedio scoperto.\n\nAttaccato senza trovare nulla, e vale come esito: la regola sulla tariffa di availability (rifiuto davvero in accettazione, nessuna via per reintrodurre una remunerazione per nodo senza tetto da un altro canale), la ripartizione F/E con la radice degli eleggibili, la motivazione del rifiuto del tetto proporzionale (e' quella giusta e non una vicina), la coerenza F_genesi/F_max e l'aritmetica dei 18 documenti, il vincolo contro la revoca (rafforzamento netto), il cooldown, il caso degenere del pool corto, e il percorso 27->33->36 di REVIEW-011, che e' chiuso e verificato."
    evidence_refs: ["SPEC-009", "ADR-010", "ADR-011", "REVIEW-011", "REVIEW-013"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-014-EVENT-003"
    timestamp: "2026-08-25T17:57:01.525741400+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Applicati tutti gli otto finding in-scope, seguendo l'ordine consigliato. La diagnosi unificante di AGENT-007 - e' stata vincolata la grandezza nominata dall'ADR, non la grandezza da cui la proprieta' dipende - e' stata usata come filtro su ogni chiusura. RF-001 (critical): riprodotto con il simulatore prima di correggere (nuovi scenari s12_quorum_capture_table e s12b_quorum_capture_walk, sei test): a V=27, min_set=18 una coalizione di 13 seggi (48,1%) contrae a 19 - pavimento stretto, 3*19=57>54 - e ottiene quorum 39>38 senza possedere il set, poi il possesso in tre confini; soglia reale k_min=max(floor(2*S_new/3)+1, floor(V/3)+1) con tabella 58,3%/48,1%/47,2%/46,7%/44,7% e asintoto 4/9. Corretti ledger.md in tre punti (paragrafo del pavimento, blocco del claim, narrazione), la nota AT-10 del threat model, il rapporto sezioni 3 e 5; nuova sezione ledger.md \"Owning the set and controlling it are different thresholds\" con tabella e fixture di confine inclusa la riga V=27/k=13/S_new=19 valida con quorum raggiunto; rimosso l'argomento \"sopra i due terzi la safety BFT e' gia' caduta\" dalla soglia di quorum. Narrazione riscritta come ritrattazione piena: la terza versione era sbagliata e non prematura, la quarta lo era per la stessa ragione, la quinta rivendica quattro noni con la dimostrazione accanto. RF-002 (critical): aggiunti reward_epoch_ms_min e _max a RewardBounds con la ragione scritta (l'epoca e' il denominatore di ogni tetto per epoca), e risolta l'ambiguita' \"bounded reward parameters\" contro \"any parameter\" in un'unica formulazione: il rapporto 5/4 e il gap si applicano a ogni grandezza di RewardPolicyBody. RF-003 (high): aggiunti storage_units_per_contribution_unit_max, compute_units_per_contribution_unit_max e validator_eligibility_window_epochs_max; la ragione dichiarata era falsa in entrambe le affermazioni ed e' stata rimossa e non integrata. RF-004 (high): aggiunti storage_microtokens_per_byte_epoch_min e compute_microtokens_per_million_fuel_min come pavimenti, perche' la direzione pericolosa per alpha e' verso il basso. RF-005 (medium): ledger.md acquista la seconda meta' del paragrafo, what it costs - margine di contrazione speso una volta sola (dopo 27->19 il successore lecito minimo e' 18), aggravamento del cooldown, min_set=V ammesso e fatale alla prima uscita, la rampa come fase peggiore - con la cifra rimisurata sui parametri raccomandati (tre confini, arresto al quarto a 15<18). RF-006 (medium): ripristinate le due dichiarazioni di REVIEW-011 RF-005 cancellate fuori ambito (bordo inferiore come scelta di prodotto travestita da misura; le due promesse dichiarate in mondi diversi) e la giustificazione di SEC-REQ-16 (d); riparata la stessa regressione trovata anche nella nota AT-07 del threat model. RF-007 (medium): creati due strumenti versionati sim/tools/protocol_hashes.py e sim/tools/reward_rules.py in sostituzione degli scratch/*.py mai committati, e aggiunta a README.md la tabella di conformita' di RewardBounds con venti righe. Lo strumento versionato ha trovato subito un difetto che la passata precedente aveva mancato: la fixture reward PD-0 portava ancora availability_microtokens_per_unit \"1\", la forma che la nuova regola rifiuta; corretta a \"0\", policy_hash ricalcolato da fbc7493a a 89da35fb, registro e costanti Rust aggiornati, con conferma indipendente dal test Rust policy_hash_over_reward_pd0. RF-008 (low): claim riscritto come garanzia incondizionata (D = F*N/(N+H) < F a ogni livello d'uso, perche' D non contiene W) piu' una proprieta' aggiuntiva sopra il 70,6%, con la precisazione che sotto quella soglia la banda non e' sospesa ma falsa. SEC-REQ-14 e SEC-REQ-18 non toccati: sono righe di AGENT-007 e le riscrive lei alla chiusura. Verifica: suite del simulatore da 35 a 41 test, cargo test -p coblox-core verde su tutti i target, i due strumenti versionati escono con codice 0, 34 casi di rifiuto senza mismatch."
    evidence_refs: ["sim/tools/protocol_hashes.py", "sim/tools/reward_rules.py", "sim/coblox_sim/scenarios.py", "sim/tests/test_simulator.py", "docs/protocol/README.md", "docs/protocol/ledger.md", ".lmbrain/knowledge/economic-simulation-report.md", ".lmbrain/knowledge/threat-model.md", "core/coblox-core/tests/common/mod.rs", "core/coblox-core/tests/conformance_registry.rs", ".lmbrain/specs/review/SPEC-009-attuazione-di-adr-010-e-adr-011-rewardbounds-regole-di-validita-economiche-claim-in-due-regimi.md"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-014-EVENT-004"
    timestamp: "2026-08-25T18:06:55.666192400+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "GATE-SECREVIEW superata. Otto findings su otto chiusi, verificati contro le rispettive condizioni di chiusura e non contro la descrizione della remediation.\n\nRF-001. ledger.md porta una sezione nuova che RIFIUTA la cifra dei due terzi invece di qualificarla. Rifatta l'aritmetica su k_min = max(floor(2*S_new/3)+1, floor(V/3)+1) con S_new = max(floor(2V/3)+1, min_set): 12->7 (58,3%), 27->13 (48,1%), 36->17 (47,2%), 60->28 (46,7%), 600->268 (44,7%), asintoto 4/9 dall'alto. Coincide riga per riga. Le due chiusure decisive: l'argomento \"sopra i due terzi la safety BFT e' gia' caduta\" e' rimosso dalla soglia di quorum e lasciato solo sul possesso, ed era il passo che ha reso innocue tutte e tre le versioni sbagliate; e la ritrattazione e' piena (\"that version was wrong, not premature\") con la ragione di forma scritta accanto. I sei test nuovi eseguono il finding invece di asserirlo.\n\nRF-002, RF-003, RF-004. Riapplicato il filtro. Enumerate le tredici grandezze di RewardPolicyBody una per una: nessuna resta scoperta. Le direzioni sono quelle giuste ed e' la parte in cui l'errore sarebbe stato facile - tetti dove il pericolo e' verso l'alto, pavimenti sulle tariffe di lavoro perche' li' e' verso il basso, massimo sulla finestra di eleggibilita' che e' il verso opposto all'intuizione. Due composizioni corrette: il 5/4 esteso a ogni grandezza senza eccezione, e il tetto della finestra in epoche che diventa difesa vera solo perche' reward_epoch_ms ha ora un massimo. La ragione falsa su \"scale all scores uniformly\" e' rimossa e non integrata. Il criterio e' generalizzato: una grandezza al denominatore di una vincolata, o che ne denomina l'unita', porta la proprieta' quanto quella nominata dall'ADR.\n\nRF-005. Misura rifatta sui parametri raccomandati: 27->24->21->18 tutti validi, al quarto confine il successore sarebbe 15 sotto il pavimento, arresto. Tre confini. Verificati anche il margine speso una volta sola e la trappola min_set = V.\n\nRF-006. Le due dichiarazioni sono ripristinate nella forma che intendevo, e AGENT-002 ha trovato e riparato la stessa regressione anche nella nota AT-07, che io non avevo rilevato.\n\nRF-007. Strumenti versionati ed eseguiti da me: 34 casi con 0 discordanze, coprendo i rifiuti di RewardBounds che mancavano; le tabelle normative portano gli stessi casi, quindi il registro non dipende dagli script.\n\nRF-008. Il claim e' nella forma garanzia incondizionata piu' proprieta' aggiuntiva, con la banda dichiarata falsa e non sospesa sotto la soglia. Rispondo alla domanda del mandato: si', ora e' difendibile senza dichiarazioni accanto.\n\nLa fixture reward PD-0 con availability = 1. Verificata la correzione per conto mio: il metodo si rivalida su due fixture non toccate e calcola 89da35fb..., che e' il valore ora nel registro, mentre la vecchia forma riproduce fbc7493a..., il che prova che e' cambiato quel campo e nient'altro. Sulla disposizione generale: la sostengo, con una precisazione sul come. Non una prassi ma una gate before-submit dichiarata su ogni spec che introduce o modifica una regola di validita', che imponga una passata su tutte le fixture pubblicate e non solo su quelle che la spec tocca. La condizione perche' sia una gate e non un buon proposito esiste gia': lo strumento e' versionato ed esegue in un secondo. E' materia di ADR del Lead, non la scrivo io.\n\nIl mio documento: SEC-REQ-13, SEC-REQ-14 e SEC-REQ-18 riscritti da me sullo stato effettivo. SEC-REQ-14 porta il criterio di completezza corretto, che e' la parte riusabile. Annotata anche la conclusione \"poco sopra 1/3\" della sezione sul pavimento di contrazione, che era corretta al suo tempo e sarebbe rimasta a contraddire la nota di AT-10: quarto esemplare della stessa forma di difetto, chiuso invece che segnalato.\n\nResidui dichiarati, nessuno bloccante e nessuno tocca una proprieta' di sicurezza: HostingRateCardBody e' un terzo documento governato senza oggetto di limiti, con un proprio billing_epoch_ms al denominatore delle sue tariffe - e' burn e non mint quindi nessuna superficie Sybil, ma e' integrita' di addebito, fuori ambito di SPEC-009 e registrato in SEC-REQ-14 come il caso che le review conditions di ADR-010 chiedono di sorvegliare; la costante PUBLISHED[\"reward_policy\"] in sim/tools/protocol_hashes.py e' rimasta al vecchio hash e fa riportare allo strumento un DIFFERS che non esiste, una riga da correggere prima della chiusura; e un motivo impreciso nella riga 4 di una tabella di conformita', che non cambia alcuna soglia.\n\nEsecuzioni indipendenti: cargo test --workspace 104 test 0 falliti; pytest 41 test 0 falliti con 6 nuovi su RF-001; reward_rules.py 34 casi 0 discordanze GATE-RULES-REJECT PASS; protocol_hashes.py metodo validato su fixture non toccate e 628c66f9 e 89da35fb riprodotti."
    evidence_refs: ["SPEC-009", "ADR-010", "ADR-011", "REVIEW-011", "REVIEW-013"]
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-014-EVENT-005"
    timestamp: "2026-08-25T18:11:43.178967100+02:00"
    action: "remediation"
    from_status: "accepted"
    to_status: "accepted"
    actor_role: "implementation-specialist"
    reason: "Seguito dopo l'accettazione: due correzioni di una riga, nessuna delle quali riapre il verdetto. (1) La costante di confronto di sim/tools/protocol_hashes.py per policy_hash era rimasta al valore precedente fbc7493a mentre il registro pubblica 89da35fb, quindi lo strumento riportava DIFFERS from registry su una discordanza inesistente: calcolo giusto, registro giusto, costante stale. Corretta; le quattro righe ora danno MATCH. Poiche' un falso positivo in uno strumento di verifica e' piu' dannoso di un test mancante - insegna a non fidarsi, ed e' la lezione di RF-007 vista dall'altro lato - la correzione non si e' fermata alla costante: aggiunti tre test che rileggono le quattro righe del registro direttamente da docs/protocol/README.md e le confrontano sia con le costanti dello strumento sia con gli hash ricomputati, piu' uno che esegue i 34 casi di rifiuto. Il guardiano e' stato verificato in negativo rimettendo la costante stale: fallisce con \"tools/protocol_hashes.py is stale for reward_policy\". La deriva non puo' piu' passare inosservata in nessuna delle due direzioni. (2) L'ultima riga della tabella di conformita' della soglia in ledger.md motivava con \"cannot censor\" una coalizione di 9 su 27: e' impreciso, perche' agli onesti 18 servirebbe 3*18 > 2*27 cioe' 54 > 54, falso, quindi 9 puo' gia' negare il quorum - bloccare richiede 3k >= V e non 3k > V. Cio' che 9 non puo' fare e' ottenere il quorum per se' su alcun successore lecito, 27 non e' sopra 38, quindi ottiene un arresto, che e' l'esito gia' concesso a chiunque stia a un terzo. Riga corretta con S_new = 19 e ragione riscritta; nessuna soglia cambia e la conclusione della riga era gia' giusta. Nella stessa passata e' dichiarato che il termine floor(V/3)+1 di k_min e' una formulazione conservativa del requisito di censura per la stessa ragione, che la condizione vera e' 3k >= V, e che non vincola mai perche' il primo termine del massimo domina a ogni dimensione del set nella tabella. Verifica: 44 test nel simulatore da 41, cargo test -p coblox-core verde su tutti i target, i due strumenti a codice 0, GATE-MODEL-VALIDATED e GATE-CONSTRAINTS verdi."
    evidence_refs: ["sim/tools/protocol_hashes.py", "sim/tests/test_simulator.py", "docs/protocol/ledger.md", ".lmbrain/specs/review/SPEC-009-attuazione-di-adr-010-e-adr-011-rewardbounds-regole-di-validita-economiche-claim-in-due-regimi.md"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
related_specs: [SPEC-009]
related_decisions: [ADR-007, ADR-010, ADR-011]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "transitioned changes-requested -> accepted"
  - date: 2026-08-25
    action: "recorded review remediation"
---
# Security review di SPEC-009 (GATE-SECREVIEW)

## Outcome

**Changes requested.** Le quattro disposizioni sono scritte, l'aritmetica tiene, e due delle tre regole di validità sono formulate nel posto giusto e con la ragione giusta accanto. Ma il mandato di questa review non era l'aritmetica, ed è sulle tre domande di merito che il documento non regge:

1. **`RewardBounds` è ristretto, non chiuso.** Almeno tre grandezze lasciate fuori con una ragione scritta sostengono una proprietà di sicurezza dichiarata, e una di esse — `reward_epoch_ms` — è il **denominatore di tutte le grandezze vincolate**. Un tetto per epoca su un'epoca di durata governata e non limitata non è un tetto. È esattamente il rischio che [SPEC-009] nominava per primo, e una superficie ristretta è qui dichiarata chiusa in `SEC-REQ-14` e in `SEC-REQ-18`.
2. **La metà matura del claim non è tenuta da nessuna regola.** `α` è un rapporto il cui denominatore è prezzato da `storage_microtokens_per_byte_epoch` e `compute_microtokens_per_million_fuel`, che sono governati, non vincolati e **possono valere zero**. Un quorum che li azzera riporta una rete matura ad `α ≈ 1` senza traccia distinguibile. Per il criterio di [ADR-010] — *ciò che è governato senza limiti di magnitudine non è un parametro, è una preferenza* — il regime maturo è oggi una preferenza.
3. **L'invariante dei due terzi non regge, ed è la terza confutazione.** Il vincolo `3 · min_set >= 2 · V` impedisce a una coalizione sotto i due terzi di **possedere l'intero set**. Non le impedisce di **ottenere il quorum del set contratto**, che è la proprietà che conta. Con `V = 27` la soglia reale è **13 seggi su 27, il 48,1 %**, e tende a `4V/9 ≈ 44,4 %` al crescere di `V`. Il numero è calcolato sui numeri che il documento stesso scrive.

Il difetto ricorrente è lo stesso in tutti e tre i casi, e non è distrazione: **si è vincolata la grandezza nominata dall'ADR e non la grandezza da cui dipende la proprietà.** Il tetto su `F` invece dell'emissione per unità di tempo; il pavimento su `validator_eligibility_threshold_units` invece dell'unità in cui è denominato; la proprietà «possedere il set» invece della proprietà «controllare il set».

## Acceptance-criteria compliance

Verificato, e non ripeto ciò che il Lead ha già verificato (fixture, hash, suite, RF-001 di [REVIEW-013]).

**Soddisfatti.**

- La regola sulla tariffa di availability è nel posto giusto: `MUST ... == 0` con «rejected on acceptance» sia in `ledger.md` §*Availability tariff* sia in `README.md`, la ragione strutturale scritta accanto — unico canale che paga per nodo senza tetto aggregato — e la via alternativa indicata (fondo a tetto aggregato, mai tariffa positiva). I casi rifiutati sono nel registro normativo, non solo in uno script.
- Il tetto proporzionale agli eleggibili è rifiutato in una sezione propria di `README.md` **con la motivazione giusta**: l'inflazione del denominatore `E` da parte della flotta, che riapre il criterio (a) di [ADR-007]. È la motivazione corretta e non un vicino plausibile: il difetto non è che il tetto sia troppo alto, è che il tetto è una funzione di una grandezza che l'avversario controlla.
- Il vincolo relazionale è nel blocco di vincoli, con la tabella di confine e il percorso `27 → 33 → 36` scritto per esteso come ciò che impedisce.
- Coerenza fra `F_genesi` e `F_max`: `300 000 000 <= 15 882 352 941`, e la crescita 300 → 15 882 cr a 5/4 richiede `ceil(log(52,94)/log(1,25)) = 18` documenti. L'aritmetica del costo operativo è corretta.

**Non soddisfatti.**

- *«Per ogni grandezza della reward policy è dichiarato se sostiene una proprietà di sicurezza»* — dichiarato sì, **correttamente no**, per almeno tre grandezze. Vedi RF-002 e RF-003.
- *«Il claim è enunciato in due regimi»* — la forma è quella richiesta, la metà matura non è sostenuta. Vedi RF-004 e RF-008.
- *«L'affermazione di `ledger.md` sulla soglia effettiva è aggiornata»* — è stata aggiornata a una cifra **sbagliata nell'altra direzione**. Vedi RF-001.

## Tests and verification

`GATE-FIXTURES-RECOMPUTED` è solida: il Lead ha rieseguito il metodo su una fixture non modificata e riprodotto `628c66f9…`.

`GATE-RULES-REJECT` è più debole di quanto la trascrizione lasci credere, e lo registro in RF-007: gli script `scratch/recompute_hash.py` e `scratch/test_rejections.py` **non esistono nell'albero** e non sono ignorati da `.gitignore`, e il registro di conformità normativo non porta alcun caso rifiutato per il tetto di `RewardBounds`, per il rapporto 5/4 o per il gap di attivazione.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security-boundary | severity=critical | criterion=L'affermazione di `ledger.md` sulla soglia effettiva è aggiornata nella stessa passata | remediation=Correggere la soglia rivendicata da «due terzi» a `2/3 · (2/3) · V`, oppure separare esplicitamente possesso e controllo

**La rivendicazione dei due terzi è confutata una terza volta, e la nuova cifra è ottimistica invece che pessimistica.**

Il documento afferma (riga 1445 e riga 1906): *«The effective capture threshold against attrition of this network is therefore two thirds»*, e nel blocco del claim *«capture below two thirds is structurally impossible across all valid parameter documents»*.

È vero solo se «capture» significa **possedere il 100 % dei seggi**. La proprietà che il protocollo promette altrove — e l'unica che conti — è che nessuna coalizione sotto i due terzi ottenga il **quorum**, cioè `3 · signed > 2 · total` sul set attivo. Il vincolo nuovo non la garantisce.

**Scenario d'attacco concreto, sui valori raccomandati `V = 27`, `min_set = 18`.** Una coalizione di **13 seggi su 27 (48,1 %)** — sotto i due terzi, sotto ogni soglia che il documento dichiari significativa:

1. Nella finestra di candidatura finalizza le proprie 13 candidature e **censura selettivamente** le candidature oneste, lasciandone passare esattamente 6. È lo stesso vettore che il documento descrive alla riga 1418 e che dichiara indistinguibile da un blocco onesto: i nodi onesti firmano ciascuno di quei blocchi perché ciascuno è valido, e sono loro a fornire il quorum del confine.
2. Al confine la derivazione produce `S_new` di **19** membri. Verifiche: pavimento di contrazione `3 · 19 = 57 > 2 · 27 = 54` ✓ — è il pavimento **stretto**, ed è il documento stesso a scriverlo alla riga 2127, *«un set di 27 scende a 19, non a 18»*; `min_set = 18 <= 19` ✓; tetto di ingresso non toccato, `fills = 0`; nessuna regola violata, nessuna traccia distinguibile.
3. La coalizione ha ora **13 su 19**. Quorum: `13 · 3 = 39 > 19 · 2 = 38` ✓. **Ha il quorum.** Da questo momento firma blocchi, mint, revoche e documenti governati da sola.
4. Con il quorum abbassa `V`: `22` è ammesso dal rapporto 5/4 (`x_old · 4 <= x_new · 5` dà `V_new >= 21,6`), `min_set = 15` soddisfa `45 >= 44`. Confine successivo: `19 → 15` (`45 > 38` ✓, `>= 15` ✓), coalizione **13 su 15**. Poi `V = 18`, `min_set = 12`, contrazione `15 → 12`, coalizione **12 su 12**. Possesso integrale.

**La soglia reale.** Detta `k` la coalizione, il set contratto minimo ammissibile è `floor(2V/3) + 1`, e serve `3k > 2 · (floor(2V/3)+1)`. Misurata: `V = 12 → k = 7 (58,3 %)`; `V = 27 → k = 13 (48,1 %)`; `V = 36 → k = 17 (47,2 %)`; `V = 60 → k = 28 (46,7 %)`; asintoto `4/9 = 44,4 %`. Il vincolo alza la soglia da «poco sopra un terzo» a **circa quattro noni**. È un guadagno reale e va rivendicato — non sono i due terzi.

**Perché è severo e non pedante.** La riga 1906 è nel blocco che il progetto ha designato come luogo del claim, e la frase *«above two thirds BFT safety has already failed»* è precisamente il passo che rende innocua la soglia rivendicata. Se la soglia vera è `4V/9`, quel passo non si applica: a `4V/9` la safety BFT **non è caduta affatto**, ed è lo stesso identico errore logico delle due ritrattazioni precedenti. Peggio: la narrazione riscritta (righe 1920–1927) ora **riabilita** la terza versione dichiarandola «prematura e non sbagliata». Non era prematura. Era sbagliata allora per la censura selettiva, ed è sbagliata adesso per la censura selettiva — cambia solo il punto in cui la contrazione si ferma. Una ritrattazione trasformata in riabilitazione è un danno peggiore dell'affermazione originale, perché rimuove il precedente che avrebbe impedito la quarta occorrenza.

**Condizione di chiusura verificabile.** `ledger.md` e il threat model affermano una soglia `k_min = ceil((2 · (floor(2V/3)+1) + 1) / 3)`, con la tabella `V ∈ {12, 27, 36, 60}` e l'asintoto `4V/9`; il blocco del claim distingue esplicitamente **possesso del set** (due terzi) da **controllo del set via quorum** (quattro noni) e non usa più l'argomento «sopra i due terzi la safety è già caduta» per la seconda; la narrazione delle ritrattazioni dice che la terza versione era sbagliata, non prematura, e che la quarta lo era per la stessa ragione. Il registro porta una fixture con `V = 27`, coalizione 13, `S_new = 19`, verdetto **valido** e quorum raggiunto.

---

RF-002 | category=security-boundary | severity=critical | criterion=Per ogni grandezza della reward policy è dichiarato se sostiene una proprietà di sicurezza | remediation=Vincolare `reward_epoch_ms` in `RewardBounds`, o riformulare ogni tetto come tetto per unità di tempo di catena

**Il tetto su `F` è per epoca, e la durata dell'epoca è governata, non vincolata e non limitata in variazione.**

`RewardPolicyBody` porta `reward_epoch_ms` (`README.md` riga 351). La valutazione dichiarata è: *«`reward_epoch_ms`: Governance-tuned epoch duration; does not create a forgery or Sybil attack vector.»*

La ragione è vera e irrilevante. `reward_epoch_ms` non deve creare un vettore Sybil per distruggere la difesa: **è il denominatore di ogni grandezza che `RewardBounds` vincola.** `existence_fund_microtokens_per_epoch_max` limita `F` per epoca; l'emissione per unità di tempo reale è `F / reward_epoch_ms`. Lo stesso vale per `storage_microtokens_per_byte_epoch`.

**Scenario d'attacco concreto.** Il quorum in carica pubblica una `reward_policy` che lascia `F` esattamente al tetto — pienamente conforme a `RewardBounds` — e porta `reward_epoch_ms` da `86 400 000` (un giorno, l'epoca su cui `F_max = 15 882 352 941` è tarato) a `86 400`. Emissione reale **× 1000**. Nessun limite di magnitudine violato, nessun rapporto di variazione superato — e qui il documento si contraddice su quale sia la regola: il punto 2 di `README.md` dice che il rapporto vincola *«bounded reward parameters»*, il paragrafo di chiusura dice *«any parameter»*. Sotto la prima lettura il salto è consentito in **un solo documento**; sotto la seconda serve una successione a 5/4, cioè lo stesso costo operativo dichiarato per la crescita di `F` — che il documento presenta come una garanzia e che qui sarebbe l'unica difesa residua. Un'ambiguità testuale non è una difesa.

La garanzia della rampa di [ADR-011] cade con esso: `D = F · N/(N+H) <= F` **per epoca**, e con l'epoca accorciata di tre ordini di grandezza l'importo assoluto a rischio per giorno è mille volte quello dichiarato. È la fase in cui [ADR-011] afferma che l'importo assoluto è *la sola garanzia che quella fase può avere*.

**Condizione di chiusura verificabile.** `RewardBounds` porta `reward_epoch_ms_min` (e, per il verso opposto, un `_max`, perché un'epoca allungata all'infinito congela l'emissione e non è meno una decisione di sicurezza); il rapporto 5/4 e il gap di attivazione si applicano dichiaratamente a `reward_epoch_ms`; l'ambiguità *«bounded reward parameters»* / *«any parameter»* è risolta in un'unica formulazione; il registro porta un caso rifiutato per `reward_epoch_ms` sotto il minimo e uno per variazione oltre il rapporto. In alternativa, e sarebbe la forma più forte: ogni tetto di `RewardBounds` è ridenominato **per unità di tempo di catena** invece che per epoca, e la questione scompare invece di essere tappata.

---

RF-003 | category=security-boundary | severity=high | criterion=Per ogni grandezza della reward policy è dichiarato se sostiene una proprietà di sicurezza | remediation=Vincolare i fattori di conversione e la finestra di eleggibilità, o correggere la ragione dichiarata

**Il pavimento su `validator_eligibility_threshold_units` è denominato in un'unità che la governance definisce.**

`RewardBounds` fissa `validator_eligibility_threshold_units_min`, con la ragione giusta: impedire alla governance di ridurre la barriera d'ingresso al pool di candidati validatori. Ma il punteggio confrontato con quella soglia è calcolato da `storage_units_per_contribution_unit` e `compute_units_per_contribution_unit`, che il documento lascia fuori con questa ragione: *«Unit conversion factors mapping physical metrics to score units; they scale all scores uniformly and do not alter Sybil resistance.»*

Sono due affermazioni, e sono entrambe false.

- **«Scale all scores uniformly»** è falso perché sono **due fattori indipendenti**: cambiarne uno solo ripondera storage contro compute e sposta chi è eleggibile, senza toccare nessuna grandezza vincolata.
- **«Do not alter Sybil resistance»** è falso perché la barriera d'ingresso *è* il confronto fra un punteggio e una soglia. L'unico vincolo del blocco su questi fattori è `> 0`. Un quorum che moltiplica `storage_units_per_contribution_unit` per `10^6` rende il pavimento di genesi soddisfabile con un milionesimo del lavoro fisico. Il pavimento resta scritto, firmato e rispettato alla lettera, e non significa più niente.

È letteralmente il criterio di [ADR-010] applicato a sé stesso: **un limite denominato in un'unità governata non è un limite.** Il progetto ha già respinto due volte l'argomento «il valore è scelto bene»; qui ha vincolato il numero e lasciato governata l'unità.

**Stessa forma, seconda grandezza.** `validator_eligibility_window_epochs` è lasciato fuori come *«tuned by network operation»*, e il blocco richiede solo `>= 1`. Le unità di contributo si accumulano sulle ultime `validator_eligibility_window_epochs` epoche (`ledger.md` riga 888). Una soglia in unità su una finestra non limitata superiormente porta il **tasso** di contributo richiesto verso zero: con una finestra di 10 000 epoche, un nodo che non fa quasi nulla supera comunque il pavimento. Serve un **massimo**, che è il verso opposto a quello che l'intuizione suggerisce, e la ragione dichiarata non menziona la questione.

**Scenario d'attacco concreto.** Un avversario che possiede il quorum, o un operatore sotto pressione di taratura, pubblica una `reward_policy` conforme a `RewardBounds` sotto ogni aspetto con `storage_units_per_contribution_unit` moltiplicato per `10^6` e `validator_eligibility_window_epochs` portato da 30 a 3 000. Il pool di candidati validatori si apre a una flotta emulata che possiede una frazione trascurabile di storage reale; da lì il vettore di RF-001 diventa raggiungibile per acquisto invece che per attrito.

**Condizione di chiusura verificabile.** `RewardBounds` porta `storage_units_per_contribution_unit_max`, `compute_units_per_contribution_unit_max` e `validator_eligibility_window_epochs_max`; le ragioni dichiarate per quelle grandezze sono riscritte (quella attuale è falsa e va rimossa, non integrata); il registro porta un caso rifiutato per ciascuno. Se il progetto preferisce non vincolarli, la ragione ammissibile è una sola e va scritta così: **il pavimento su `validator_eligibility_threshold_units` è vero a condizione che la reward policy attiva non ridenomini l'unità**, cioè è una preferenza — che è precisamente la formulazione che [ADR-010] esiste per rendere inaccettabile.

---

RF-004 | category=security-boundary | severity=high | criterion=Il claim è enunciato in due regimi, banda su `α` con la soglia reale per il regime maturo | remediation=Vincolare i prezzi del canale di lavoro, o dichiarare la banda come condizionale alla reward policy attiva

**Il denominatore di `α` è governato, non vincolato, e può valere zero.**

`α` è il rapporto fra l'emissione di esistenza e l'emissione totale. Il numeratore è ora vincolato: `F <= F_max`, regola. Il denominatore contiene l'emissione del canale di lavoro, prezzata da `storage_microtokens_per_byte_epoch` e `compute_microtokens_per_million_fuel`. Nessuna delle due ha limite di magnitudine, e nessuna regola impone che siano positive: il blocco di vincoli richiede `> 0` per i **fattori di conversione**, non per le **tariffe**. La ragione dichiarata — *«pay for proven, difficult-to-fake work ... Sybil fleets cannot mint from these channels without incurring real infrastructure costs»* — risponde alla domanda sbagliata. Nessuno teme che una flotta minti da lì. La questione è che quelle tariffe **fissano il denominatore della grandezza su cui poggia l'intera metà matura del claim**.

**Scenario d'attacco concreto.** Una rete matura è nella banda, `α = 0,15`. Il quorum in carica pubblica una `reward_policy` che porta `storage_microtokens_per_byte_epoch` e `compute_microtokens_per_million_fuel` a **zero**, lasciando `F` invariato al valore tarato. Il documento è conforme a `RewardBounds` sotto ogni aspetto — nessun tetto superato, nessuna tariffa di availability positiva, `kn < kd` intatto. L'emissione di lavoro va a zero, `α` va a **1**, e la rete matura si ritrova nella condizione che [ADR-011] descrive come propria del solo avviamento: la banda dichiarata e violata, `X = 20 %` falsa, e la quota catturabile da una flotta pari all'intera emissione. Nessuna traccia on-chain distingue quel documento da un normale atto di taratura al ribasso.

**Perché è la stessa questione di [ADR-010] e non una nuova.** [SPEC-009] ha reso `α` difesa **dal lato del numeratore** e ha lasciato il denominatore dove era. Per il criterio della reviewer citato dalla spec stessa, la banda su `α` nel regime maturo è oggi **una preferenza**: è vera a condizione che la reward policy attiva prezzi il lavoro in modo ragionevole, e la ragionevolezza dei valori è precisamente ciò che [ADR-010] ha stabilito essere insufficiente. Il claim in due regimi ha quindi una metà tenuta da una regola e una metà tenuta da un valore, ed è dichiarato come se entrambe fossero regole in `SEC-REQ-14` (*«coperto in specifica»*) e in `SEC-REQ-18` (*«Fondo, regole di validità e due regimi: specificati»*).

**Condizione di chiusura verificabile.** Una delle due:

- `RewardBounds` porta `storage_microtokens_per_byte_epoch_min` e `compute_microtokens_per_million_fuel_min` (pavimenti, non tetti: la direzione pericolosa per `α` è verso il basso), con il registro che esibisce il rifiuto; oppure
- il claim del regime maturo è riformulato come **condizionale e dichiarato tale**: «la banda su `α` vale a condizione che la reward policy attiva prezzi il canale di lavoro; nessuna regola lo impone», e `SEC-REQ-14`/`SEC-REQ-18` smettono di dire «coperto» per la parte non coperta.

La seconda è accettabile — dichiarare una superficie aperta è legittimo. Ciò che non è accettabile è la formulazione attuale, che è la prima senza la regola.

---

RF-005 | category=documentation | severity=medium | criterion=Il documento dichiara che cosa il vincolo impedisce | remediation=Dichiarare in `ledger.md` il prezzo in liveness del vincolo, accanto al vincolo

**Il vincolo consuma per intero il margine di contrazione della rete, e `ledger.md` non lo dice dove la regola vive.**

`ledger.md` §*Degenerate cases* dichiara già il compromesso generale: pool corto → set più piccolo → sotto `validator_min_set_size` la catena si ferma, con recupero fuori banda tramite distribuzione autenticata. Ma quel compromesso è stato valutato quando `min_set` era una scelta libera. Con `min_set >= ceil(2V/3)` la posizione cambia di natura:

- il pavimento di contrazione consente di perdere fino a un terzo del set a un confine; `min_set = 2V/3` consente di perderne un terzo **una volta sola e mai più**. Dopo qualunque contrazione il set siede esattamente sul pavimento, e il confine successivo non tollera **alcuna** riduzione: qualsiasi uscita non rimpiazzata rende il successore invalido e ferma la catena;
- il cooldown aggrava: i membri usciti sono fuori dal pool per `validator_cooldown_epochs` e non possono riparare la carenza che hanno creato. `ledger.md` lo chiama già *«the sharpest liveness edge in the section»* — questo vincolo la affila;
- il caso limite `min_set = V` è ammesso dal blocco (`3V >= 2V` sempre vero) e produce una rete che si ferma alla **prima** uscita non rimpiazzata. Nulla lo segnala;
- l'interazione con [ADR-011] è la parte che nessuno dei due documenti mette insieme: la fase di rampa è quella con il pool di candidati più piccolo, ed è ora anche quella che deve tenere in vita `2V/3` validatori a ogni confine, pena l'arresto permanente.

Il threat model porta una riga (*«Il prezzo in liveness è misurato e accettato: tre confini consecutivi con pool vuoto fermano la catena»*) ereditata dalla misura che [REVIEW-011] fece su un'ipotesi. `ledger.md`, che è il documento normativo dove la regola ora vive, dichiara solo ciò che la regola impedisce e non ciò che costa. Le *review conditions* di [ADR-010] nominano questo per nome: *«se i limiti di genesi risultassero così stretti da impedire alla rete correzioni legittime»*.

**Condizione di chiusura verificabile.** Il paragrafo *«The relational bound on `validator_min_set_size` and what it prevents»* acquista una seconda metà — *what it costs* — che dichiara: il margine di contrazione è consumato dopo un confine; il caso `min_set = V` e la sua conseguenza; il vincolo sulla popolazione minima di validatori vivi alla rampa; e il rinvio al recupero fuori banda già dichiarato. La cifra dei confini è rimisurata sui parametri raccomandati invece di essere ereditata.

---

RF-006 | category=provenance | severity=medium | criterion=Nessun valore di [SPEC-007] è ritarato; ambito rispettato | remediation=Ripristinare le dichiarazioni imposte da [REVIEW-011] RF-005

**Due dichiarazioni pretese da una review di sicurezza precedente sono state cancellate fuori ambito.**

Il diff di `economic-simulation-report.md` rimuove:

- *«Il bordo inferiore è una scelta di prodotto travestita da misura, e va scritto così ([REVIEW-011] RF-005). Nessuna grandezza simulata seleziona 0,10 …»*;
- l'intero paragrafo *«E la banda e `X` sono dichiarate in due mondi diversi»*, che registrava l'incoerenza fra la promessa in assenza di avversario (`α = 0,15`) e la promessa con avversario presente e tollerato (0,0157 cr contro 1,588 al banco di `AT-07`), e la conclusione che **la banda non può essere pubblicata da sola**.

Nessuna delle due è resa obsoleta da [ADR-010] o [ADR-011]: la prima riguarda la provenienza del bordo inferiore, la seconda l'incoerenza fra due promesse **entrambe ancora pubblicate**. La riformulazione in due regimi tocca *quando* la banda vale, non *cosa* la banda e `X` misurano. `SEC-REQ-16` (d) sopravvive ma perde la sua giustificazione (*«perché `X` da sola invita a concludere che il reddito sia protetto entro un ordine di grandezza quando non è protetto affatto»*), che è la frase che le dà senso.

Una remediation di sicurezza rimossa in una passata che non la nomina è una regressione, indipendentemente dal fatto che il testo circostante sia migliorato. [SPEC-009] escludeva esplicitamente la ritaratura di [SPEC-007].

**Condizione di chiusura verificabile.** Le due dichiarazioni sono nel documento, riferite a [REVIEW-011] RF-005, adattate al vocabolario dei due regimi ma non ridotte; la giustificazione di `SEC-REQ-16` (d) è ripristinata.

---

RF-007 | category=verification-integrity | severity=medium | criterion=Le fixture di conformità coprono le regole nuove | remediation=Portare i casi rifiutati di `RewardBounds` nel registro normativo e versionare gli script

**Una delle tre regole nuove non ha alcun rifiuto esibito nei documenti.**

`GATE-RULES-REJECT` è motivata così dalla spec: *«una regola di validità di cui non si esibisce il rifiuto è una raccomandazione con un nome diverso»*. Stato reale:

- tariffa di availability: casi rifiutati nel registro normativo di `README.md` ✓;
- vincolo relazionale su `min_set`: tabella di confine con sei righe in `ledger.md` ✓;
- **`RewardBounds`**: nessun caso rifiutato in alcun documento. Il tetto di `F`, il rapporto 5/4 e il gap di attivazione compaiono solo nella trascrizione di uno script.

E lo script non esiste: `scratch/recompute_hash.py` e `scratch/test_rejections.py` non sono nell'albero e non sono ignorati da `.gitignore`. `GATE-FIXTURES-RECOMPUTED` sopravvive perché il Lead l'ha rieseguita per conto proprio; `GATE-RULES-REJECT` no, e per la terza regola non c'è nulla da rieseguire.

**Condizione di chiusura verificabile.** `README.md` porta una tabella di conformità per `RewardBounds` con almeno: `F` al tetto (valido), `F` oltre il tetto (invalido), variazione esattamente a 5/4 (valido), variazione oltre 5/4 (invalido), attivazione al gap (valido), attivazione sotto il gap (invalido) — più i casi che RF-002 e RF-003 aggiungono. Gli script sono versionati o il loro contenuto è nei documenti.

---

RF-008 | category=documentation | severity=low | criterion=Nessuna formulazione lascia intendere che la banda valga durante l'avviamento | remediation=Dichiarare la garanzia assoluta come incondizionata invece che come garanzia di fase

**La coppia rampa/regime maturo lascia apparire un regime intermedio scoperto che in realtà non esiste.**

Ho cercato il buco che il mandato mi chiedeva di cercare, e la buona notizia è che non c'è: `D = F · N/(N+H) <= F` è vera **a ogni livello d'uso**, non solo in rampa, perché `N/(N+H) < 1` sempre e `D` non contiene `W`. La copertura è quindi totale.

Ma il documento la presenta come garanzia **di fase**: *«1. Regime di rampa (avviamento): la garanzia è espressa in termini assoluti»*, *«2. Regime maturo: sopra la soglia d'uso reale …»*. Un lettore che si trovi al 40 % dell'uso di riferimento — sopra la rampa, sotto il 70,6 % — non trova la propria fase in nessuna delle due voci, e conclude di non avere garanzia. Ne ha una, ed è la stessa.

La forma difendibile è a **una garanzia incondizionata più una proprietà aggiuntiva**, non a due fasi disgiunte:

> L'importo massimo che una flotta può dirottare per epoca è `D = F · N/(N+H) < F`, a **ogni** livello d'uso, indipendentemente dalla dimensione della flotta. È la garanzia che vale sempre.
> Al di sopra del 70,6 % dell'uso di riferimento vale **in più** la banda `α ∈ [0,10 – 0,20]` con `X = 20 %`. Al di sotto non è sospesa: **non è vera**, e `α → 1` per costruzione.

Costa tre righe e toglie l'unica lettura in cui il claim in due regimi promette meno del vero. (Resta soggetta a RF-002 per l'unità di tempo e a RF-004 per il denominatore.)

## Required follow-up

Ordine di remediation consigliato, perché due findings cambiano ciò che gli altri devono dire:

1. **RF-001**, perché è nel blocco del claim ed è la terza occorrenza dello stesso errore. Fino a che non è chiuso, `ledger.md` afferma una soglia di sicurezza falsa nella sezione designata per essere citata.
2. **RF-002 e RF-004**, che decidono se `RewardBounds` chiuda la superficie o la restringa. Fino ad allora `SEC-REQ-14` e `SEC-REQ-18` non possono dire «coperto in specifica».
3. **RF-003**, RF-005, RF-006, RF-007, RF-008.

Al termine, `SEC-REQ-14` e `SEC-REQ-18` vanno riscritti sullo stato effettivo. Sono righe del threat model, che è documento di AGENT-007: le scrivo io alla chiusura dei findings, non l'implementatrice.

**Non rivedere** nessuno dei findings per la ragione che i valori attuali sono ragionevoli. È la stessa clausola che [ADR-010] ha già scritto per sé, e vale qui per costruzione: tutti gli scenari sopra sono atti di governance **leciti e conformi**, e nessuno di essi richiede che un valore attuale sia sbagliato.

## Ciò che ho attaccato senza trovare nulla

Lo registro perché su questa spec è informazione quanto i findings.

- **La regola sulla tariffa di availability.** Il rifiuto è davvero in accettazione e non altrove: `MUST ... == 0` con «rejected on acceptance» in entrambi i documenti, ragione strutturale accanto, casi rifiutati nel registro normativo (`0` valido, `1` e `1000` invalidi). Ho cercato una via per rimettere una remunerazione per nodo senza tetto passando da un'altra parte — `work_kind = "availability"` con una tariffa presa da un altro campo, evidenza di challenge di tipo availability instradata sul canale di lavoro, un secondo canale a tariffa per nodo — e non c'è: l'availability non ha altra tariffa, l'evidenza di availability alimenta solo la soglia di eleggibilità al fondo, e il fondo è `F/E` con resto scartato. **Regola chiusa.**
- **La ripartizione del fondo.** `E > 0`, `amount = F/E` a divisione intera, resto mai emesso, somma per epoca `<= F`, `eligible_set_root` che impegna l'insieme contato e `E` uguale al numero di foglie. Non ho trovato modo di superare `F` in un'epoca né di gonfiare `E` senza che la radice lo mostri.
- **Il rimedio apparente.** Il tetto proporzionale agli eleggibili è rifiutato con la motivazione **giusta** e non con una vicina: il difetto è che il tetto sarebbe funzione di una grandezza che l'avversario controlla, non che sarebbe alto. È scritto in `README.md` e ripreso nel rapporto economico.
- **Coerenza fra `F_genesi` e `RewardBounds`.** `300 000 000 <= 15 882 352 941`; la crescita a 5/4 richiede 18 documenti, aritmetica corretta; il costo operativo è dichiarato invece che lasciato scoprire, come [ADR-011] imponeva.
- **Il vincolo relazionale contro la revoca.** Le transizioni di sola rimozione sono soggette allo stesso pavimento di contrazione e allo stesso `min_set`: con `min_set >= 2V/3` un quorum che revoca si ferma **prima**, non dopo. È un rafforzamento netto e non ho trovato alcuna asimmetria fra il percorso elettivo e quello per revoca.
- **Il cooldown.** Non ho trovato un percorso in cui il vincolo nuovo apra un'evasione del cooldown o del limite di mandato. La sola interazione che ho trovato è in direzione liveness, ed è RF-005.
- **Il caso degenere del pool corto.** `fills` resta un minimo su tre quantità, nulla viene rilassato, e la continuazione d'emergenza resta rifiutata perché attaccabile. Il vincolo nuovo non introduce alcun percorso in cui un pool corto produca un set **valido** favorevole all'attaccante: produce un arresto. L'arresto è il costo di RF-005, non un vettore di cattura.
- **Il percorso di crescita di `V` che [REVIEW-011] aveva trovato.** `27 → 33 → 36` con `min_set` fermo a 18 è ora rifiutato riga per riga dal blocco. Quella porta specifica è chiusa e verificata sulla tabella di confine. RF-001 non la riapre: è un'altra porta.

## Verifica di remediation (AGENT-007, seconda passata)

**Tutti e otto i findings sono chiusi.** Ho verificato ciascuno contro la sua condizione di chiusura, non contro la descrizione della remediation.

**RF-001 — chiuso, ed è la chiusura migliore delle otto.** `ledger.md` porta una sezione nuova, *«Owning the set and controlling it are different thresholds»*, che **rifiuta** la cifra dei due terzi invece di qualificarla. Ho rifatto l'aritmetica su `k_min = max(floor(2·S_new/3)+1, floor(V/3)+1)` con `S_new = max(floor(2V/3)+1, min_set)`: `V=12 → 7 (58,3 %)`, `27 → 13 (48,1 %)`, `36 → 17 (47,2 %)`, `60 → 28 (46,7 %)`, `600 → 268 (44,7 %)`, asintoto `4/9` dall'alto. Coincide riga per riga. Il secondo termine del massimo è la precondizione di censura che io non avevo scritto esplicitamente ed è un'aggiunta corretta: non vincola mai, perché `4/9 > 1/3`, ma rende la formula vera anche fuori dal caso stretto.

Le due cose che rendono la chiusura definitiva e non cosmetica:

- **l'argomento «sopra i due terzi la safety BFT è già caduta» è rimosso dalla soglia di quorum** e lasciato solo sul possesso. Era il passo che rendeva innocue tutte e tre le versioni sbagliate, ed è la ragione per cui la stessa affermazione ha potuto ripresentarsi tre volte;
- **la ritrattazione è piena**: *«That version was wrong, not premature»*, con la ragione di forma scritta accanto — una riabilitazione rimuove il precedente che avrebbe impedito l'occorrenza successiva. Registro che la riabilitazione contestata era stata chiesta dal Lead in [REVIEW-013] e che il Lead l'ha riconosciuta come propria: il documento ora conserva la traccia di **quattro** versioni invece di cancellarne una.

I sei test di `sim/tests/test_simulator.py` **eseguono** il finding invece di asserirlo: costruiscono il cammino, verificano che al primo confine il quorum non ci sia, che al secondo ci sia senza possesso, e che il possesso arrivi entro tre confini. È la differenza fra una regressione e un commento.

**RF-002, RF-003, RF-004 — chiusi, e ho riapplicato il mio stesso filtro.** `RewardBounds` guadagna sette campi. Ho enumerato le tredici grandezze di `RewardPolicyBody` una per una e **non ne resta nessuna scoperta**: `reward_epoch_ms` (min e max), `existence_fund_microtokens_per_epoch` (max), `availability_microtokens_per_unit` (regola `== 0`), `storage_microtokens_per_byte_epoch` e `compute_microtokens_per_million_fuel` (min), `publisher_microtokens_per_active_subscriber` (tenuto dal tetto `kn < kd`, che è a sua volta vincolato), `publisher_reward_cap_numerator` (max) e `_denominator` (min), i due fattori di conversione (max), `validator_eligibility_threshold_units` (min), `validator_eligibility_window_epochs` (max), `validator_eligibility_min_issuers` (min).

La scelta della **direzione** è argomentata e le direzioni sono quelle giuste, che era la parte in cui l'errore sarebbe stato facile: tetti dove il pericolo è verso l'alto, **pavimenti** sulle tariffe di lavoro perché lì il pericolo è verso il basso, essendo il denominatore del rapporto sorvegliato, e **massimo** sulla finestra di eleggibilità, che è il verso opposto a quello che l'intuizione suggerisce. Il documento lo scrive esplicitamente in entrambi i casi.

Due chiusure che valgono più della somma delle parti:

- l'ambiguità sul 5/4 è risolta **nella direzione larga**: ogni grandezza di `RewardPolicyBody`, senza eccezione, con la ragione scritta — *un'ambiguità testuale su quali grandezze copra l'unica difesa residua non è una difesa*. Questo rende il rapporto una difesa reale anche per le grandezze che hanno un limite in una sola direzione;
- il tetto su `validator_eligibility_window_epochs` è espresso **in epoche**, e diventa una difesa vera solo perché `reward_epoch_ms` ha ora un massimo. Le due chiusure si tengono a vicenda, e la composizione è corretta.

La ragione falsa su *«scale all scores uniformly»* è **rimossa e non integrata**, come la condizione chiedeva: una ragione sbagliata lasciata accanto a quella giusta insegna comunque la forma sbagliata.

Il paragrafo *«The question each bound answers»* generalizza il criterio invece di elencare la correzione: una grandezza al **denominatore** di una vincolata, o che ne **denomina l'unità**, porta la proprietà quanto quella nominata dall'ADR. È [ADR-010] applicata a sé stessa, ed è la parte riusabile.

**RF-005 — chiuso, misura verificata.** Il prezzo è ora in `ledger.md` accanto alla regola. Ho rifatto la misura sui parametri raccomandati (`V=27`, `T=9`, `c=3`, `min_set=18`): `27 → 24 → 21 → 18`, tutti validi (`72>54`, `63>48`, `54>42`, e 18 è il pavimento), e al quarto confine il successore sarebbe 15, sotto il pavimento: **arresto**. Tre confini, cifra corretta e rimisurata invece che ereditata. Verificati anche gli altri due punti: dopo una contrazione massima `27 → 19` il successore minimo è 18, quindi il confine seguente non tollera **alcuna** uscita non rimpiazzata; e `min_set = V` è ammesso dal blocco e produce una rete che si ferma alla prima uscita — è dichiarato come trappola, che è ciò che serviva, dato che nessuna regola lo rifiuta.

**RF-006 — chiuso, ed è la riparazione che intendevo.** Le due dichiarazioni sono nel rapporto economico, riferite a [REVIEW-011] RF-005, adattate al vocabolario dei due regimi e non ridotte. La giustificazione di `SEC-REQ-16` (d) — *«perché `X` da sola invita il lettore a concludere che il proprio reddito sia protetto entro un ordine di grandezza quando non è protetto affatto»* — è ripristinata sul mio documento nella forma che le dà senso. Registro che AGENT-002 ha trovato **la stessa regressione anche nella nota `AT-07`**, dove la correzione su `D` che non contiene `W` era stata cancellata, e l'ha riparata senza che io l'avessi rilevata: è la parte di questa passata che nessuno le aveva chiesto.

**RF-007 — chiuso, e la sua giustificazione è arrivata da sé.** `sim/tools/protocol_hashes.py` e `sim/tools/reward_rules.py` sono versionati. Ho eseguito entrambi: il metodo si rivalida sulle fixture non toccate e `reward_rules.py` esibisce **34 casi con 0 discordanze**, coprendo i rifiuti che mancavano — tetto di `F`, pavimento e tetto dell'epoca, tetti dei fattori di conversione, tetto della finestra, pavimenti delle tariffe, rapporto 5/4 e gap di attivazione — oltre a quelli che c'erano già. Le tabelle normative di `README.md` portano gli stessi casi, quindi il registro non dipende dagli script.

**RF-008 — chiuso nella forma che avevo indicato.** *Garanzia incondizionata più proprietà aggiuntiva*, con la banda dichiarata **falsa e non sospesa** sotto la soglia. Rispondo alla domanda che il mandato mi poneva: **sì, ora il claim è difendibile senza dichiarazioni accanto.** Un lettore a qualunque livello d'uso trova la propria garanzia, e chi legge la banda non può concludere che valga dove non vale.

### La fixture invalida, e la disposizione che ne segue

La scoperta di questa passata è la più istruttiva: gli strumenti versionati hanno trovato **subito** che la fixture reward `PD-0` portava ancora `availability_microtokens_per_unit: "1"`, cioè che il registro di conformità **pubblicava un documento che la nuova regola rifiuta**. Ho verificato la correzione per conto mio: `sim/tools/protocol_hashes.py` rivalida il metodo su due fixture non toccate e calcola `sha256:89da35fb…`, che è il valore ora in `README.md`; la vecchia forma con `availability = 1` riproduce esattamente il vecchio `fbc7493a…`, il che prova che è cambiato quel campo e nient'altro.

**È la terza volta** — `T = 3`, poi `min_set = 1`, ora questa — e tutte e tre sono emerse **solo quando una regola nuova le ha rese verificabili**. Il pattern è chiuso e va nominato: *una fixture pubblicata insegna una forma, e una regola nuova cambia retroattivamente quali forme sono lecite.* Sostengo la disposizione generale che il Lead propone, con una precisazione sul come: **non una prassi ma una gate**, `before-submit`, dichiarata su ogni spec che introduce o modifica una regola di validità, che imponga una passata su **tutte** le fixture pubblicate e non solo su quelle che la spec tocca. La condizione perché sia una gate e non un buon proposito esiste già: lo strumento è versionato ed esegue in un secondo. Non la scrivo io, è materia di ADR del Lead, ma la raccomando senza riserve — e osservo che l'argomento più forte a suo favore è che uno script non versionato non avrebbe mai trovato questa.

### Aggiornamento del mio documento

`SEC-REQ-13`, `SEC-REQ-14` e `SEC-REQ-18` di `threat-model.md` sono riscritti da me in questa passata sullo stato effettivo, come avevo disposto. `SEC-REQ-14` porta ora il **criterio di completezza corretto** — denominatori e unità, non solo le grandezze nominate — perché è la parte riusabile e sarebbe andata persa in una nota di review. Ho inoltre annotato la conclusione «poco sopra `1/3`» della sezione sul pavimento di contrazione, che era corretta al suo tempo e sarebbe rimasta a contraddire la nota di `AT-10`: è il quarto esemplare della stessa forma di difetto, e l'ho chiuso invece di segnalarlo.

### Residui dichiarati, nessuno bloccante

- **`HostingRateCardBody` è un terzo documento governato senza alcun oggetto di limiti**, con un proprio `billing_epoch_ms` che è il denominatore delle sue tariffe. Non tocca l'emissione — le voci di hosting sono **burn** e non mint, quindi non c'è superficie Sybil — ma è integrità di addebito sugli escrow delle app. È fuori dall'ambito di [SPEC-009] e non lo apro come finding; è però esattamente il caso che le *review conditions* di [ADR-010] chiedono di sorvegliare, ed è registrato in `SEC-REQ-14`.
- **`PUBLISHED["reward_policy"]` in `sim/tools/protocol_hashes.py` è rimasto al vecchio valore `fbc7493a…`**, mentre `README.md` pubblica `89da35fb…`. Lo strumento riporta quindi *DIFFERS from registry* su una discordanza che non esiste. Il valore calcolato è giusto e il registro è giusto: è la costante di confronto a essere stale. Non è un difetto di sicurezza ed è una riga, ma va corretta prima della chiusura, perché uno strumento che grida al lupo su un allineamento corretto è uno strumento che al giro dopo verrà ignorato — e RF-007 esisteva precisamente perché nessuno stava guardando.
- La riga 4 della tabella di conformità sulla soglia (`V=27`, coalizione 9, motivo *cannot censor*) è imprecisa: con 9 seggi su 27 la coalizione **può** già negare il quorum, perché ai 18 onesti servirebbe `3·18 > 54`, che è falso. La conclusione della riga resta giusta — 9 non raggiunge il quorum di alcun successore lecito, perché servirebbe `S_new <= 13 < min_set` — quindi nessuna soglia cambia. Nit di formulazione, segnalato per completezza.

### Esecuzioni indipendenti

```text
cargo test --workspace                 104 test, 0 falliti (13 binari)
python -m pytest tests (in sim/)        41 test, 0 falliti, 6 nuovi su RF-001
python sim/tools/reward_rules.py        34 casi, 0 discordanze - GATE-RULES-REJECT: PASS
python sim/tools/protocol_hashes.py     metodo validato su 2 fixture non toccate: PASS
                                        consensus PD-0  628c66f9... riprodotto
                                        reward    PD-0  89da35fb... riprodotto
```

## Final decision

**Accettata. `GATE-SECREVIEW` è superata.** Gli otto findings sono chiusi contro le loro condizioni di chiusura, verificate una per una e non sulla descrizione della remediation. Le tre affermazioni di sicurezza che questa review contestava sono ora vere nella forma in cui sono scritte: `RewardBounds` copre le tredici grandezze e il criterio di completezza è generalizzato ai denominatori e alle unità; la banda su `α` è tenuta da pavimenti e non da valori; e la soglia di cattura è rivendicata a **circa quattro noni**, che è il numero vero, con l'argomento che aveva reso innocue le tre versioni sbagliate rimosso dal punto in cui non si applica.

Restano due correzioni di una riga, non bloccanti e registrate sopra: la costante `PUBLISHED["reward_policy"]` stale nello strumento, e il motivo impreciso nella riga 4 di una tabella. Nessuna delle due tocca una proprietà di sicurezza.
