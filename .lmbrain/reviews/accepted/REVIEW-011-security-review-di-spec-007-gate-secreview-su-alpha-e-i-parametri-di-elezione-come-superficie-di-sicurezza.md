---
id: REVIEW-011
# Note: Quote the title if it contains a colon
title: "Security review di SPEC-007 — GATE-SECREVIEW su alpha e i parametri di elezione come superficie di sicurezza"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-007
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, requirements-completeness, documentation, robustness]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-011-EVENT-001"
    timestamp: "2026-08-25T15:58:40.167043700+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-011-EVENT-002"
    timestamp: "2026-08-25T16:08:51.080071800+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Security review di SPEC-007 per GATE-SECREVIEW. I valori scelti sono difendibili e nessun numero del rapporto e contestato; le affermazioni che li accompagnano non lo sono, e due di esse sono false fuori dalla configurazione di riferimento pur essendo gia trasferite in forma incondizionata nella matrice dei requisiti. RF-001 critico: la difesa anti-Sybil di ADR-007 vive interamente nella reward policy, che non ha ne vincoli relazionali, ne magnitudini ancorate alla genesi, ne limite di variazione, mentre i parametri di elezione hanno tutti e tre; un set in carica firma un documento con availability_microtokens_per_unit positivo e existence_fund a 2^60, ogni regola di validita enunciata e soddisfatta (kn<kd intatto, divisori positivi), il criterio (a) di ADR-007 cade perche work_compensation di tipo availability e un importo per nodo senza tetto, e non resta traccia on-chain distinguibile da un normale atto di governance: e la stessa proprieta per cui identity.md ha reso il pavimento Argon2id una regola di validita e per cui ElectionBounds sta nella genesi. Fissare availability a zero e necessario ma non sufficiente: F resta senza tetto e senza limite di variazione, e la disciplina 5/4 su F proposta dal rapporto e prassi non imposta da nulla. Serve un RewardBounds di genesi, cioe un'ADR. RF-002 alto: AT-07 e dichiarato superato al regime d'uso di riferimento ed e schedulato in M-03 su devnet, cioe nel regime in cui non e superato; alpha e osservata e al lancio W circa 0 implica alpha circa 1, quindi la cattura e circa 99 per cento contro il X=20 per cento dichiarato, violato di circa cinque volte per tutto l'avviamento. Il rapporto vede il problema e propone la soglia d'uso, ma la qualificazione non ha raggiunto la metrica, il criterio (c), la riga AT-07 della matrice ne la nota di prodotto in inglese. RF-003 alto: la chiusura della cattura per attrito sotto 2V/3 attribuita a validator_min_set_size non e una regola e non sopravvive alla governance; nessun vincolo lega min_set a V, e con c e m congelati a 3 dal limite 5/4 si ottiene V<=36, raggiungibile in due documenti leciti in circa 14 giorni, dove check_constraint_block accetta riga per riga e la simulazione di censura selettiva del progetto stesso da cattura completa dell'intero set in 2 confini gia a k=18, cioe al 50,0 per cento di V, dove la safety BFT non e ancora persa. La chiusura e il vincolo 3*validator_min_set_size >= 2*V, soddisfatto con uguaglianza dalla combinazione raccomandata e quindi a costo zero. RF-004 medio: il sesto criterio di AT-10 non e soddisfacibile, la diagnosi e la correzione di AGENT-002 sono corrette, e il difetto e nel modo in cui il test e stato scritto e non nel singolo criterio: entrambe le occorrenze sono affermazioni assolute su una grandezza emergente che non nominano la regola che dovrebbe garantirla. RF-005 medio: ADR-007 va annotata e non superata, perche nessuno dei cinque punti della Decision e falsificato e l'identita alpha*N/(N+H) la conferma invece di contraddirla; l'annotazione non e pero editoriale perche tocca la promessa di prodotto che ADR-007 stessa protegge, e va confermata dall'operatore. La banda [0,10-0,20] e X=20 per cento sono dichiarate in due mondi diversi: il bordo inferiore protegge il significato del reddito in assenza di avversario, mentre l'attacco tollerato lo attraversa di due ordini di grandezza (il telefono onesto conserva lo 0,99 per cento). RF-006 medio: c=3, m=3 e cooldown=2 sono congelati per sempre dal limite 5/4 sugli interi piccoli, quindi V<=36 per sempre e validator_max_set_size=45 e max_set_max=81 sono margini di crescita irraggiungibili; se la rete si scoprisse il pool troppo sottile la mossa correttiva non esiste. RF-007 basso: la soglia di partecipazione (pool stabile >=30 per non arrestarsi, >=36 per tenere V seggi, circa il 3 per cento dei contribuenti) e la sola condizione nota in cui la rete si ferma senza avversario e viveva solo in un rapporto di taratura. Attaccati senza trovare nulla: il criterio (d) di ADR-007, che tiene per regola perche availability contribuisce zero al contribution_score; il criterio (a) via il fondo a tetto con amount=F/E e resto scartato; l'evasione del cooldown con la condizione 5 per qualunque ragione; min_set=18 alla configurazione raccomandata; la macinatura del seme; e la scelta di X=20 per cento, il cui argomento di dimostrabilita per costruzione e corretto e va conservato. Le sei voci in-scope sono qualificazioni di affermazioni e non cambiano alcun valore raccomandato; RF-004 parte 2 e RF-007 li ho gia applicati in knowledge/threat-model.md, che e documento mio. Le cinque voci fuori scope (RewardBounds, vincolo su min_set, annotazione di ADR-007, sostituzione del criterio di AT-10 e passata di verifica sugli AT esistenti) vanno aperte dal Lead. Alpha=0,15 con banda [0,10-0,20] e difendibile con dichiarazioni accanto; il cooldown=2 e scelto bene e va dichiarato irreversibile."
    evidence_refs: ["SPEC-007", "ADR-007", ".lmbrain/knowledge/economic-simulation-report.md", ".lmbrain/knowledge/threat-model.md", "docs/protocol/ledger.md", "sim/coblox_sim/params.py", "sim/coblox_sim/scenarios.py", "REVIEW-010"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-011-EVENT-003"
    timestamp: "2026-08-25T16:25:31.545616800+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Applicate le cinque voci in-scope di REVIEW-011, nessun valore raccomandato cambiato. RF-002: X dichiarata condizionata alla soglia d'uso in rapporto §1/§4/§7, righe SEC-REQ-16 e SEC-REQ-18, nota AT-07 del threat model e nota di prodotto inglese; AT-07 riformulato come parzialmente coperto, con nuovo scenario s11 che misura la rampa d'uso (99,01% a W=0 contro X=20%) e la correzione che D=F·N/(N+H) non contiene W, quindi ~15.725 cr per epoca dirottati al lancio con l'F di genesi. RF-003: percorso di erosione riprodotto col simulatore (V 27→33→36, T 9→11→12, due documenti leciti, min_set/V da 0,667 a 0,500; a V=36 censura selettiva 36→25→18, intero set in due confini a una coalizione del 50%); affermazione su validator_min_set_size qualificata come proprietà della combinazione raccomandata in rapporto §5 e nella nota AT-10; conclusione «appena sopra un terzo» di ledger.md dichiarata da non cambiare. RF-001 parte 1: dichiarato per iscritto in rapporto §2/§3 e riga SEC-REQ-18 che availability=0, il tetto di F e la disciplina 5/4 su F sono valori e prassi, non regole, e che i criteri (a) e (c) di ADR-007 sono condizionati alla reward policy attiva. RF-006: nuovo legal_next_intervals in params.py e nuova sezione del rapporto — c, m e cooldown congelati a [3,3], [3,3], [2,2], V ≤ 36 per sempre, validator_max_set_size 45 e max_set_max 81 dichiarati margini irraggiungibili con le loro motivazioni corrette, e registrato che l'argomento su T_max non vale per c e cooldown perché lì la mossa non esiste comunque. RF-005 parti 2 e 3: frase mancante nella nota onesta inglese (la fetta la prende al posto dell'utente), «under 20 %» condizionato, «protected» fra le parole da evitare, grandezza (d) aggiunta a SEC-REQ-16, e bordo inferiore della banda scritto esplicitamente come scelta di prodotto travestita da misura con l'incoerenza fra i due mondi. RF-004 adottato nella diagnosi generale sul modo di scrivere i criteri. Incorporata l'osservazione sul verso di avvicinamento (margine sul bordo superiore) e il cooldown 2 dichiarato irreversibile. Fuori scope e non toccati: RewardBounds di genesi e 3·min_set≥2V, che sono regole di validità nuove; docs/protocol/ invariato. Verifica: GATE-MODEL-VALIDATED e GATE-CONSTRAINTS rieseguite verdi, suite da 27 a 35 test con i due finding nuovi eseguiti invece che accettati."
    evidence_refs: ["sim/_gates_transcript.txt", "sim/coblox_sim/params.py", "sim/coblox_sim/scenarios.py", "sim/coblox_sim/__main__.py", "sim/tests/test_simulator.py", ".lmbrain/knowledge/economic-simulation-report.md", ".lmbrain/knowledge/threat-model.md", ".lmbrain/specs/review/SPEC-007-simulatore-economico-e-taratura-di-alpha-e-dei-parametri-di-elezione.md"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-011-EVENT-004"
    timestamp: "2026-08-25T16:30:20.161432900+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verifica indipendente della remediation da AGENT-007. Tutte e cinque le voci in-scope sono chiuse. RF-002: la condizione d'uso ha raggiunto tutti e quattro i punti indicati (rapporto §1 riquadro su X, §4 verdetto e rampa, §7 voci (a) e (c), nota di prodotto §9) piu le righe SEC-REQ-16, SEC-REQ-18 e la nota AT-07 del threat model; parzialmente coperto e la formula giusta perche AT-07 ha ora due regimi e uno dei due non e ancora valutato, e la matrice non deve poter essere letta come coperto senza riaprire il rapporto. RF-003: qualificazione presente in rapporto §5, riassunto §5 e nota AT-10, con la tabella del percorso di erosione e la clausola corretta che ledger.md non va cambiato. RF-001 parte 1: la distinzione valori-contro-regole e scritta in rapporto §2 e §3 e nella riga SEC-REQ-18. RF-006: tabella degli intervalli leciti con i tre parametri congelati, V<=36 per sempre, e i due margini nominali corretti nelle motivazioni. RF-005: la nota inglese ha ora la frase che le mancava e la formulazione e difendibile senza altre dichiarazioni accanto (dice che la fetta la prende al posto dell'utente, che la rete non puo impedirlo, e che e un costo pagato direttamente); grandezza (d) aggiunta a SEC-REQ-16 con H/(N+H) e lo 0,99 percento; protected aggiunto alle parole da evitare con la ragione corretta. RF-004: la diagnosi generale e applicata al rapporto §5 nella stessa forma in cui l'ho applicata al threat model, cioe la firma comune alle due occorrenze invece della sola correzione del sesto criterio. Test: suite eseguita, 35 test verdi, e i sei nuovi eseguono i finding invece di asserirli: s9_legal_intervals produce l'insieme dei congelati, s9b_max_reachable_v da 36, s10_min_set_ratio_erosion verifica che ogni passo passi il blocco di vincoli e che il rapporto vada da 0,667 a 0,500 in due documenti, s10b riproduce la cattura al 50 percento, s11 e s11b misurano la rampa d'uso e l'invarianza dell'importo assoluto. Riprodotta la sua autocorrezione: D = F*N/(N+H) non contiene W, importo dirottato identico lungo tutta la rampa. Residuo nuovo, non bloccante e non richiesto da REVIEW-011, emerso dalla sua stessa misura e registrato in SEC-REQ-18 di threat-model.md: la soglia d'uso del 25 percento non e il punto in cui la banda diventa tenibile, perche con l'F di genesi alpha<=0,20 solo dal 70,6 percento dell'uso di riferimento e a 25 percento vale 0,414, cioe il doppio di X; fra le due soglie la banda e dichiarata e violata a meno che la governance non porti F verso il bersaglio, che dal valore di genesi richiede sette documenti al 5/4. Radice unica con la sua autocorrezione: l'F di genesi e dimensionato sulla rete matura e non su quella che esiste al lancio."
    evidence_refs: ["sim/tests/test_simulator.py", "sim/coblox_sim/scenarios.py", "sim/coblox_sim/params.py", ".lmbrain/knowledge/economic-simulation-report.md", ".lmbrain/knowledge/threat-model.md", "ADR-010"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-011-EVENT-005"
    timestamp: "2026-08-25T16:30:46.700626300+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "GATE-SECREVIEW superato. Le cinque voci in-scope sono chiuse e verificate in modo indipendente, nessun valore raccomandato e cambiato, e le tre voci fuori scope sono correttamente instradate su ADR-010, che adotta l'argomento portante della review. La remediation e migliore di quanto la review chiedeva su due punti: AGENT-002 ha riprodotto i due finding nuovi con il simulatore invece di accettarli, e i sei test aggiunti eseguono i finding invece di asserirli, quindi da qui in avanti una deriva dei parametri che riaprisse RF-003 o RF-006 rompe una suite invece di passare inosservata; e ha corretto spontaneamente un errore proprio che nessuno le aveva contestato, che D = F*N/(N+H) non contiene W, quindi l'importo assoluto dirottato e identico a uso nullo e al regime di riferimento e la frase il 91 percento di un'emissione minuscola e un'emissione minuscola era vera solo se anche F e piccolo. Quella correzione e il contributo piu utile della remediation, perche sposta la questione dal rapporto al valore di genesi di F. Verdetto sulle formule: parzialmente coperto e la formula giusta per AT-07, perche il test ha ora due regimi e uno non e ancora valutato; la nota di prodotto inglese e difendibile senza altre dichiarazioni accanto. Due residui non bloccanti, entrambi consegnati al Lead e nessuno dei due richiesto da REVIEW-011. Primo, e principale: la soglia d'uso del 25 percento non e il punto in cui la banda diventa tenibile, perche con l'F di genesi alpha scende a 0,20 solo dal 70,6 percento dell'uso di riferimento mentre a 25 percento vale 0,414, il doppio di X; fra le due soglie la banda e dichiarata e violata, salvo portare F verso il bersaglio con sette documenti al 5/4. Ha la stessa radice della sua autocorrezione, cioe un F di genesi dimensionato sulla rete matura invece che su quella che esiste al lancio, ed e registrato in SEC-REQ-18 di threat-model.md. Non e coperto dal tetto su F di ADR-010: un tetto e statico e dimensionato sulla rete matura, e il limite di variazione governa i documenti e non il valore di genesi, che viaggia nella distribuzione firmata e non e prodotto da alcun atto di governance. E quindi una disposizione ulteriore e di tipo diverso dalle altre tre, perche vincola cio che la genesi deve contenere invece di cio che la governance puo fare, e va segnalata all'operatore; il rimedio apparente di un tetto proporzionale a E va nominato come sbagliato, perche sarebbe un tetto che una flotta alza gonfiando E e riaprirebbe il criterio (a). Secondo, cosmetico: il test test_usage_floor_is_where_the_band_becomes_holdable asserisce che a uso 0,25 alpha supera il bordo della banda, cioe dimostra il contrario di cio che il suo nome afferma; e la stessa firma di RF-004 in miniatura ed e un rinominare, non un difetto di cio che il test prova. Su validator_max_set_size = 45: non e un difetto di sicurezza. Il valore e inerte perche V<=36 per sempre e max_set non vincola nient'altro, e cio che era sbagliato erano le parole che lo motivavano, ora corrette. Va all'operatore come scelta di valore cosmetica, e la mia raccomandazione e lasciarlo dove sta."
    evidence_refs: ["SPEC-007", "ADR-010", "sim/tests/test_simulator.py", ".lmbrain/knowledge/economic-simulation-report.md", ".lmbrain/knowledge/threat-model.md"]
    implementation_agent: "AGENT-002"
links: [SPEC-007, ADR-007, ADR-005, REVIEW-010, DEBT-010]
created: 2026-08-25
updated: 2026-08-25
tags: [review]
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
    action: "transitioned changes-requested -> accepted"
---
# Review

## Outcome

**Changes requested.** I *valori* scelti sono difendibili. Le *affermazioni* che
oggi li accompagnano non lo sono nella forma in cui sono scritte, e due di esse
— «`AT-07` superato» e «`validator_min_set_size` chiude la cattura per attrito
sotto i due terzi» — sono **false fuori dalla configurazione di riferimento**,
non con un avversario più forte ma con un uso più basso e con una deriva di
parametri pienamente lecita. Entrambe sono già state trasferite nella matrice dei
requisiti di `threat-model.md` in forma incondizionata.

Il lavoro di AGENT-002 è onesto in modo non comune: le quattro voci che il Lead
mette davanti sono **auto-segnalate dall'implementatrice**, non scoperte in
review, e §8 del rapporto le elenca senza attenuarle. Nessun finding qui
contraddice un numero del rapporto. Tre findings nascono invece dalla domanda che
il rapporto non si pone: *questi numeri sono difesi da una regola, o soltanto
scritti bene?* La risposta, tre volte su tre, è la seconda.

L'aritmetica non è stata rifatta: il Lead l'ha verificata e l'ho assunta. Ho
attaccato ciò che l'aritmetica **non** copre: il regime d'uso, la governance dei
parametri, e la durata delle proprietà nel tempo.

## Acceptance-criteria compliance

Gli undici criteri della spec risultano soddisfatti come formulati, e nessun
finding di questa review ne rovescia uno. In particolare il criterio «nessuna
regola di protocollo è stata modificata» è rispettato, ed è **la ragione per cui
tre findings non possono essere chiusi dentro SPEC-007**: la loro chiusura
completa richiede regole di validità nuove, cioè ADR e lavoro di protocollo. Per
ognuno è indicata la parte in-scope (qualificare l'affermazione) e la parte da
aprire come seguito.

`GATE-MODEL-VALIDATED` e `GATE-CONSTRAINTS` sono superati e non sono in
discussione. `GATE-SECREVIEW` **non è superato** con i documenti nello stato
attuale.

## Code observations

`sim/` non è codice di prodotto e non entra nel percorso di build del nodo, quindi
non è rivisto come superficie d'attacco. È stato usato come strumento di verifica:
`check_constraint_block` e `s6c_at10_selective_censorship` sono stati rieseguiti
con parametri diversi da quelli raccomandati per produrre le misure di RF-003.
Entrambi si sono comportati correttamente sui parametri derivati, il che è di per
sé un buon segno sul simulatore.

Due osservazioni di merito sul modello, nessuna delle quali è un difetto:

- `contribution_score` che conta `availability` come zero è **la** ragione per cui
  il criterio (d) di [ADR-007] tiene per regola e non per taratura, ed è
  correttamente identificata come tale nel rapporto §4. È l'unica delle quattro
  garanzie della metrica che sia ancorata a una regola di validità.
- La correzione discreta di §5 — la formula continua predice meno confini della
  simulazione perché il pavimento è stretto — è metodologicamente giusta e va
  conservata: citare la cifra misurata e non quella della formula è la scelta
  conservativa nella direzione corretta.

## Tests and verification

Verifiche indipendenti eseguite in questa review, con il codice del progetto:

```text
$ cd sim && python -c "... check_constraint_block su parametri derivati ..."
recommended  V=27 T=9  min_set=18  -> ALL PASS
drifted      V=36 T=12 min_set=18  -> ALL PASS      <-- RF-003
step1        V=33 T=11 min_set=18  -> ALL PASS      <-- RF-003, passo intermedio

$ cd sim && python -c "... s6c_at10_selective_censorship con V patchato ..."
--- V=27  (2V/3 = 18)
  k=17 (63,0 %): [27, 19, 18] -> bloccata, 17 su 18, mai l'intero set
  k=18 (66,7 %): [27, 19, 18] -> intero set in 2 confini
--- V=36  (2V/3 = 24)
  k=17 (47,2 %): [36, 25, 18] -> bloccata, 17 su 18, mai l'intero set
  k=18 (50,0 %): [36, 25, 18] -> intero set in 2 confini
  k=20 (55,6 %): [36, 25, 20] -> intero set in 2 confini
  k=23 (63,9 %): [36, 25, 23] -> intero set in 2 confini
```

Verifica a mano del limite di variazione 5/4 sugli interi piccoli, con la regola
`x_new * 4 <= x_old * 5` e `x_old * 4 <= x_new * 5` di `ledger.md`:

```text
x_old = 3 (c, m)        ->  x_new <= 3,75 -> 3   e   x_new >= 2,4 -> 3    CONGELATO
x_old = 2 (cooldown)    ->  x_new <= 2,5  -> 2   e   x_new >= 1,6 -> 2    CONGELATO
x_old = 27 (V)          ->  x_new in [22, 33]                             mobile
x_old = 18 (min_set)    ->  x_new in [18, 22]  (18 dal pavimento di genesi) mobile
x_old = 9  (T)          ->  x_new in [8, 11], e T_new >= T_active          mobile
```

Ciò che ho attaccato **senza trovare nulla**:

- **Il criterio (d) di [ADR-007], seggi a una flotta.** Non c'è percorso.
  `contribution_score` legge solo `challenge_evidence` di tipo `storage` e
  `compute`, la condizione 4 esige almeno `validator_eligibility_min_issuers = 3`
  emittenti distinti nessuno dei quali il soggetto, e la condizione 3 è una soglia
  positiva. Una flotta che si limita a firmare ha punteggio 0 a qualunque
  numerosità. La difesa è una regola, non un valore.
- **Il criterio (a) via il fondo di esistenza.** `amount = F / E` a divisione
  intera con resto scartato, somma dei mint di esistenza dell'epoca `<= F`,
  ricomputo di `E` dal set impegnato in `eligible_set_root`. Una flotta non può
  gonfiare l'emissione da questo canale: può solo gonfiare il divisore. Solido.
  Il canale che rompe (a) è un altro ed è RF-001.
- **Evasione del cooldown.** La condizione 5 «per qualunque ragione» chiude
  l'uscita volontaria anticipata; ho cercato una terza via d'uscita che non sia
  «essere membro del set attivo a un confine e non essere ritenuto al
  successivo» e non l'ho trovata: la definizione è per differenza fra due
  documenti firmati, e non lascia una porta laterale.
- **`min_set_size = 18` alla configurazione raccomandata.** Alla combinazione
  raccomandata la proprietà è **vera** e le misure la reggono. RF-003 non la
  contesta: contesta la sua *durata*.
- **Macinatura del seme.** Il vincolo «solo un proposer può macinare» e il tetto
  di rotazione limitano il guadagno a meno di un confine su cinquanta. Non ho
  trovato un miglioramento, e la configurazione 1 è impostata correttamente
  (attaccante che paga storage e compute reali, non uptime da datacenter).
- **La scelta di `X` = 20 %.** L'argomento che il bordo superiore della banda sia
  l'unico `X` dimostrabile per costruzione invece che per il particolare `N/H`
  del banco di prova è **corretto e va conservato**. Il difetto non è nel valore
  di `X`: è nel fatto che `α` non è tenuta dentro la banda da nulla (RF-001) e non
  vi è dentro al lancio (RF-002).

## Production quality and documentation compliance

Il rapporto soddisfa lo standard di [[QUALITY]] sul punto che qui conta di più:
dichiara i limiti invece di seppellirli, e §8 nomina sei assunzioni contestate con
la loro sede di decisione. La formulazione di prodotto di §9, inclusa «the honest
note the network owes its users», è il tipo di onestà che la promessa di prodotto
richiede — e RF-005 chiede di spingerla **un passo più in là**, non di ritirarla.

Il difetto documentale è di **trasferimento**: le righe `SEC-REQ-16` e
`SEC-REQ-18` della matrice di `threat-model.md` e la riga di `AT-07` riportano gli
esiti in forma incondizionata, mentre nel rapporto sono condizionati a un regime
d'uso. Una matrice dei requisiti è il posto da cui si legge «coperto» senza
riaprire il rapporto: è lì che la condizione deve comparire.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security-boundary | severity=critical | criterion=La reward policy ha un ancoraggio di magnitudini alla genesi, oppure il progetto dichiara per iscritto che la propria difesa anti-Sybil è affidata a valori non difesi da alcuna regola | remediation=ADR proposta e accettata, poi regola di validità e test di rifiuto

**La difesa anti-Sybil di [ADR-007] vive interamente in un documento governato che
non ha `ElectionBounds`.** Questa è la voce 2 del Lead, generalizzata, e la
generalizzazione la peggiora.

I parametri di elezione hanno tre strati di difesa: vincoli relazionali, magnitudini
ancorate alla genesi in `ElectionBounds`, e limite di variazione per documento.
`ledger.md` §"Magnitudes, not only relations" spiega perché i primi da soli non
bastano, con un esempio — `election_epoch_blocks = 2^60` — che era esattamente
[RF-001] di [REVIEW-010]. La **reward policy non ha nessuno dei tre.** L'unica
regola di validità che la governa è `kd > 0 and kn < kd` sul tetto della quota al
creatore, introdotta per chiudere un ciclo di stampa del token: il precedente
esiste, è dello stesso tipo, ed è **la stessa riga di codice di accettazione** in
cui la regola mancante andrebbe scritta.

*Scenario d'attacco.* Il set di validatori in carica — che non deve essere
malevolo dall'inizio: §7(b) del rapporto misura un margine di **circa 3 finti
abbonati per nodo controllato per periodo di 30 giorni**, dichiarato non chiudibile
per taratura, e ciò che quel margine compra è reputazione — firma una reward policy
con `availability_microtokens_per_unit = 10^6` e
`existence_fund_microtokens_per_epoch = 2^60`. Ogni regola di validità enunciata è
soddisfatta: `kn < kd` è intatto, i due divisori di contributo sono positivi,
nessun'altra disuguaglianza esiste. Da quell'epoca:

- `work_compensation` con `work_kind = "availability"` è un importo **per nodo
  senza tetto**, quindi `N` identità emulate **aumentano l'emissione totale**: il
  criterio (a) di [ADR-007] cade, ed è il criterio su cui poggia l'intera
  affermazione strutturale «una flotta non può ingrandire la torta, può solo
  prenderne una fetta». Il controesempio misurato dal rapporto §4 —
  2,06·10¹¹ → 1,02·10¹³ µt — è questo, e non è un'ipotesi: è un'uscita del
  simulatore;
- `α` sale a piacere e con essa la quota catturata, quindi cade anche il criterio
  (c) e con esso `X`.

Non serve un cambio di regime, non serve un fork, e **nessuna traccia on-chain
distingue il documento da un normale atto di governance**: è l'identica proprietà
per cui `identity.md` ha reso il pavimento Argon2id una regola di validità e per
cui `ElectionBounds` sta nella genesi. Il Lead ha ragione a dire che è la terza
volta: le prime due volte il progetto ha risposto con una regola, e la coerenza
esige che risponda con una regola anche qui. Uno zero scritto in un commento
Python non è una difesa.

Nota di gravità, perché spiega il salto a critico rispetto al giudizio del Lead:
la parte `availability = 0` è **necessaria ma non sufficiente**. Anche fissandola,
`F` resta senza tetto, senza pavimento e senza limite di variazione, mentre `α` è
per [ADR-007] «il parametro più importante dell'economia». La regola di governance
di `F` al 25 % per documento proposta in §2 del rapporto è una **prassi non
imposta da nulla**: un solo documento può portare `F` da 15 882 a `2^60`. La
regola di elezione ha un limite di variazione perché il progetto ha già stabilito
che un limite di variazione serve; l'economia non ce l'ha.

*Condizione di chiusura verificabile.*
1. **In-scope SPEC-007:** il rapporto §3 e la riga `SEC-REQ-18` di
   `threat-model.md` dichiarano esplicitamente che `availability = 0`, il tetto di
   `F` e la disciplina 5/4 su `F` sono **valori e prassi, non regole**, e che i
   criteri (a) e (c) della metrica di [ADR-007] sono validi *a condizione* che la
   reward policy attiva li rispetti.
2. **Seguito, fuori scope:** ADR proposta e accettata che introduca un
   `RewardBounds` nel trust anchor di genesi, sullo stesso modello di
   `ElectionBounds`, con come minimo `availability_microtokens_per_unit == 0`
   (o `<= availability_microtokens_per_unit_max`),
   `existence_fund_microtokens_per_epoch <= existence_fund_max`, e un limite di
   variazione per documento su `F`. Chiusa quando esiste un test che **rifiuta**
   in accettazione un documento con tariffa di availability positiva, allo stesso
   modo in cui oggi viene rifiutato `kn >= kd`.

RF-002 | category=security-boundary | severity=high | criterion=`AT-07` ha un verdetto valido nel regime in cui verrà eseguito, oppure `X` è dichiarata condizionata a una soglia d'uso ovunque compaia | remediation=Qualificare l'esito nella matrice dei requisiti ed eseguire AT-07 anche nel regime di lancio

**`AT-07` è dichiarato superato al regime d'uso di riferimento, ed è schedulato in
M-03 su devnet, cioè nel regime in cui non è superato.** Questo finding non è
nell'elenco del Lead ed è il più immediatamente operativo.

Il rapporto §1 stabilisce che `α = F/(F+W)` **deriva** con l'uso e che al lancio
`W ≈ 0`, quindi `α ≈ 1` qualunque sia `F`. La sua stessa tabella:

| uso (frazione del riferimento) | `α` con `F` fisso | cattura di `N=10⁴` contro `H=100` |
| --- | --- | --- |
| 0,00 | 1,0000 | ~99,0 % |
| 0,05 | 0,7792 | ~77,1 % |
| 0,25 | 0,4138 | ~41,0 % |
| 1,00 | 0,1500 | 14,9 % |

Il criterio (c) della metrica di [ADR-007] — «non ottiene più di una quota `X`
dichiarata dell'emissione totale dell'epoca», con `X = 20 %` — è quindi
**violato di circa cinque volte per tutto il periodo di avviamento**, che è
precisamente il periodo in cui una flotta costa meno, rende di più in termini
relativi, e in cui la rete è più esposta perché sta costruendo la propria
reputazione. La matrice di `threat-model.md` colloca `AT-07` in M-03 e nota che
«in devnet il reddito di esistenza può essere disattivato o simbolico»: un test
eseguito con il fondo disattivato non prova nulla su `X`, e un test eseguito con
il fondo attivo a uso quasi nullo lo fallisce.

Il rapporto **vede** il problema (§1, «la banda non può valere al lancio») e
propone la risposta giusta: sotto la soglia d'uso, pubblicare l'importo assoluto
dirottato invece del rapporto, perché il 91 % di un'emissione minuscola è
un'emissione minuscola. L'argomento è corretto. Il difetto è che quella
qualificazione **non ha raggiunto né la metrica di [ADR-007], né il criterio (c),
né la riga `AT-07` della matrice**, dove l'esito compare come «superato» senza
condizione. Un `X` dichiarato pubblicamente e violato per i primi mesi di vita
della rete è peggio di un `X` più onesto: è la forma di danno reputazionale
contro cui [ADR-007] stessa ha riformulato la metrica di [[PROJECT]].

*Scenario d'attacco.* Non serve un attaccante sofisticato. Una flotta di 10 000
identità su un host, al costo Argon2id di circa quattro minuti totali, presente
dalla prima epoca in cui il fondo è attivo, cattura ~99 % dell'emissione di
esistenza e ~99 % dell'emissione totale finché il canale di lavoro è vuoto. Il
danno non è il valore catturato — che è minuscolo in assoluto, ed è esattamente il
punto del rapporto — ma la **smentita pubblica e verificabile da chiunque di una
tolleranza dichiarata**, in un progetto la cui promessa è «super-sicura» e che ha
già dovuto ritirare una metrica una volta.

*Condizione di chiusura verificabile.*
1. La soglia d'uso — 25 % dell'uso di riferimento è la proposta di §2, e va
   confermata dall'operatore — è scritta **dentro** la formulazione di `X` e
   dentro la riga `AT-07` della matrice, nella forma: «`X` = 20 % vincola al di
   sopra della soglia d'uso dichiarata; al di sotto la rete pubblica l'importo
   assoluto dirottato e `X` non è un'affermazione».
2. `AT-07` ha **due** verdetti, non uno: al regime di riferimento (già misurato,
   superato) e al regime di lancio, dove il criterio applicabile è quello
   assoluto. Fino ad allora la matrice riporta `AT-07` come **parzialmente
   coperto**, non come superato.
3. La nota di prodotto di §9 dice «under 20 %» senza condizione: va allineata o
   ritardata all'attivazione della soglia.

RF-003 | category=security-boundary | severity=high | criterion=La proprietà anti-cattura di `validator_min_set_size` è una regola, oppure è dichiarata come proprietà della sola configurazione corrente | remediation=Qualificare l'affermazione ora; proporre `3 * validator_min_set_size >= 2 * V` come vincolo di accettazione

**È la voce 3 del Lead, e la risposta alla domanda «proprietà della taratura, o
regola da scrivere?» è: oggi è proprietà della taratura, e non sopravvive alla
governance.** Va scritta come regola.

Il risultato di AGENT-002 è corretto e importante: a `V = 27` e
`min_set = 18 = 2V/3` la cattura per attrito è chiusa per ogni coalizione più
piccola di 18, e `ledger.md` attribuisce al solo pavimento di contrazione un
merito che il minimo del set condivide. L'ho verificato e non lo contesto.

Contesto la sua **durata**. Il blocco di vincoli impone `0 < min_set <= V` e
nient'altro: **nessuna regola lega `min_set` a `V`.** `validator_min_set_size_min`
= 18 nella genesi impedisce di abbassare il minimo, ma nulla impedisce di alzare
`V`, e la proprietà dipende dal **rapporto**.

Il percorso è lecito, breve e misurato con il codice del progetto. `c = 3` e
`m = 3` sono congelati dal limite 5/4 (vedi RF-006), quindi `3cm <= V` dà
`V >= 27` e `ceil(V/T) <= c` con `T <= 12` dà `V <= 36`. **`V = 36` è raggiungibile
in due documenti** — `V: 27 → 33 → 36`, `T: 9 → 11 → 12`, entrambi entro il 5/4,
entrambi con `T_new >= T_active` — distanziati da un'epoca di elezione, cioè
**circa 14 giorni**. `check_constraint_block` del simulatore accetta lo stato
finale: `V=36, T=12, c=3, m=3, min_set=18, max_set=45` → tutte le righe passano.

A quel punto `2V/3 = 24 > 18 = min_set`, e la simulazione di censura selettiva del
progetto stesso dà:

```text
V=36, min_set=18:  k=18 (50,0 % di V)  ->  [36, 25, 18]  intero set in 2 confini
                   k=23 (63,9 % di V)  ->  [36, 25, 23]  intero set in 2 confini
```

**La soglia di cattura per attrito scende da due terzi a esattamente una metà.** E
questo distrugge la rassicurazione che regge l'intera affermazione: a `V = 27` il
rapporto poteva dire «solo da `2V/3` in su, punto oltre il quale la safety BFT è
già caduta e nessuna regola di composizione stava più promettendo niente». A
`V = 36` la cattura si completa **al 50 %, dove la safety BFT non è affatto
caduta** e dove la rete crede ancora di avere una garanzia. La proprietà non si
degrada gradualmente: si sposta sotto il confine che la rendeva innocua.

*Scenario d'attacco.* Una coalizione che si trovi già intorno al 50 % dei seggi —
sotto la soglia di safety, quindi in una posizione che le regole non trattano come
compromissione — propone in due tornate di governance ordinarie un aumento del set
bersaglio da 27 a 36, motivato con la crescita della rete, e un aumento
corrispondente del limite di mandato. Entrambi i documenti sono validi, firmati,
rate-limited e distanziati dall'epoca prescritta; ogni light client li verifica e
li accetta. Al confine successivo la coalizione applica censura selettiva e ottiene
l'intero set in due confini. Nessuna regola è stata violata e nessun documento
segnala che una proprietà di sicurezza è stata rimossa — che è, parola per parola,
la ragione per cui `ledger.md` ha ancorato le magnitudini di elezione alla genesi.

*Condizione di chiusura verificabile.*
1. **In-scope SPEC-007:** l'affermazione nel rapporto §5 e nella nota di `AT-10` in
   `threat-model.md` è qualificata: «chiusa sotto `2V/3` **alla combinazione
   raccomandata**; in generale la cattura per attrito è chiusa per ogni coalizione
   più piccola di `validator_min_set_size`, e il rapporto `min_set/V` non è
   preservato da alcuna regola — a `V = 36`, raggiungibile in due documenti
   leciti, la soglia scende a `V/2`».
2. **Seguito, fuori scope:** proporre come vincolo di accettazione del documento
   `consensus_parameters` la disuguaglianza `3 * validator_min_set_size >= 2 * V`.
   Alla combinazione raccomandata è soddisfatta con uguaglianza (`54 >= 54`) e non
   costa nulla; non impedisce di far crescere il set, impone soltanto di far
   crescere il minimo insieme, cosa che il limite 5/4 permette (`18 → 22 → 24`).
   Chiusa quando esiste un test che rifiuta `V = 36` con `min_set = 18`.
3. La conclusione «soglia effettiva appena sopra un terzo» di `ledger.md` **non
   va cambiata** finché la regola non esiste: oggi è la cifra corretta nel caso
   peggiore governabile, e alzarla a due terzi sarebbe scrivere una garanzia più
   forte di quella che le regole impongono.

RF-004 | category=requirements-completeness | severity=medium | criterion=I criteri di superamento di `AT-10` sono tracciabili a una regola imposta o a un parametro pubblicato | remediation=Adottare la formulazione corretta e aggiungere una convenzione di scrittura dei test d'attacco

**Il sesto criterio di `AT-10` non è soddisfacibile, la diagnosi di AGENT-002 è
corretta, e la sua correzione va adottata.** La derivazione è verificata: il
criterio letterale equivale a `m >= 50`, che per `T >= 3m` forza `T >= 150` e
`c <= V/150`, cioè almeno 150 validatori con mandati di circa tre anni; e
comunque l'orizzonte per attrito è fisso a tre confini e non si allunga con alcun
parametro, quindi nemmeno un `m` enorme comprerebbe il tempo richiesto. Non è un
fallimento di taratura.

Alla domanda del Lead — il difetto è nei singoli criteri o nel modo in cui il test
è stato scritto — **il difetto è nel modo in cui il test è stato scritto**, e le
due occorrenze hanno la stessa firma. «La coalizione non arriva mai al 100 % sotto
i due terzi» e «l'attaccante non raggiunge 1/3 entro 50 epoche» sono entrambe
**affermazioni assolute su una grandezza emergente**, formulate prima che
esistesse la regola che quella grandezza produce, e nessuna delle due nomina la
regola che dovrebbe garantirla. Un criterio così non è verificabile in fase di
scrittura: è verificabile solo quando la simulazione lo smentisce, e a quel punto
lo smentisce **addosso all'implementatrice**. È esattamente il modo di fallire che
il documento stesso nomina — *un criterio di test errato viene attribuito
all'implementazione invece che alla specifica* — e lo ha nominato **prima** di
commetterlo una seconda volta, il che dice che la nota non è bastata e serve una
regola di scrittura.

La formulazione proposta da AGENT-002 non ha quella firma: ogni sua clausola è una
disuguaglianza contro un parametro dichiarato, e ognuna nomina la regola che la
impone. È quella da adottare.

*Scenario.* Non è un attacco esterno ma un difetto di processo con conseguenze di
sicurezza: un test d'attacco che nessuna rete può superare viene, alla terza
occorrenza, **derogato invece che corretto**, e la deroga si estende ai criteri che
il test dichiara e che sono validi. La rete perde la capacità di distinguere «non
superato» da «mal scritto», che è la sola cosa che rende utile una batteria di
test d'attacco.

*Condizione di chiusura verificabile.*
1. Il sesto criterio di `AT-10` è sostituito dalla formulazione proposta in
   `threat-model.md`. La sostituzione **riduce una affermazione pubblica** e va
   quindi portata all'operatore: è la ragione per cui AGENT-002 l'ha registrata
   senza applicarla, e la ragione è giusta.
2. Alle convenzioni dei test d'attacco di `threat-model.md` è aggiunta la regola:
   *un criterio di superamento deve essere una disuguaglianza contro un parametro
   pubblicato o una proprietà imposta da una regola di validità nominata; un
   criterio che esprime un esito desiderato senza nominare la regola che lo produce
   non è un criterio ed è marcato come aspirazione finché non lo diventa.* È lavoro
   in-scope sul mio documento e lo assumo io.
3. La stessa regola va passata **una volta** su tutti gli `AT-*` esistenti, per
   scoprire eventuali terze occorrenze prima che sia una simulazione a farlo. Lo
   registro come seguito.

RF-005 | category=documentation | severity=medium | criterion=La grandezza rivolta all'utente è scritta accanto a quella rivolta all'attaccante, ovunque la seconda sia dichiarata | remediation=Annotare [ADR-007], non superarla; rafforzare la nota onesta di §9; confermare con l'operatore

**Sulla domanda del Lead — [ADR-007] va annotata o superata? — la mia posizione è:
annotata, con conferma esplicita dell'operatore, e superata solo se l'operatore
non accetta la perdita che l'annotazione rende esplicita.**

Le ragioni per non superarla. Nessuno dei cinque punti della Decision è falsificato:
il fondo a tetto regge, l'ancoraggio dell'eleggibilità a lavoro difficile da
falsificare regge ed è anzi l'unica garanzia ancorata a una regola, Argon2id regge,
la dichiarazione di ciò che il protocollo non fa regge, l'esclusione
dell'attestazione regge. La riformulazione della metrica regge **su ciò che
misura**: è una metrica rivolta all'attaccante e su quel piano è corretta.
L'identità `α · N/(N+H)` non contraddice [ADR-007]: la **conferma**, ed è il
motivo per cui il modello è stato accettato al `GATE-MODEL-VALIDATED`. Superare
un'ADR i cui cinque punti tengono significa riaprire cinque decisioni per
aggiungerne una sesta.

Le ragioni per cui l'annotazione non è editoriale. [ADR-007] scrive di sé che tocca
una promessa di prodotto di [[PROJECT]] e che, se l'operatore non concorda, va
superata e non modificata in silenzio. Ciò che l'annotazione aggiunge tocca
esattamente quella promessa, ed è più duro di come è stato finora scritto:

> Abbassare `α` **non protegge il dispositivo onesto**. La sua perdita sotto
> attacco è `H/(N+H)` e non contiene `α`. Al banco di prova che il progetto
> dichiara di tollerare — `N = 10 000` contro `H = 100`, `AT-07` — un telefono
> onesto conserva lo **0,99 %** del proprio reddito di esistenza. La difesa
> economica limita ciò che l'attaccante ottiene; **non** limita ciò che l'utente
> onesto perde.

E qui sta l'incoerenza che merita di essere nominata, perché è il punto in cui la
voce 1 del Lead diventa operativa: **la banda `[0,10 – 0,20]` e la tolleranza
`X = 20 %` sono dichiarate in due mondi diversi.** Il bordo inferiore 0,10 è
motivato dal significato del reddito — sotto, «il reddito di esistenza smette di
essere un pavimento e diventa una riga di arrotondamento» — ed è calcolato **in
assenza di avversario**. `X = 20 %` dichiara invece un avversario presente e
tollerato. Nel mondo in cui `X` è dichiarata, il rapporto telefono/medio non è
0,15: il telefono onesto prende 0,0157 cr per epoca invece di 1,588. **Il bordo
inferiore della banda protegge una grandezza che l'attacco tollerato distrugge di
due ordini di grandezza.** Le due dichiarazioni non sono in contraddizione
aritmetica — misurano cose diverse — ma sono incoerenti come **promesse**, e il
progetto le pubblicherà entrambe.

Non ne segue che la banda sia sbagliata (vedi il giudizio su `α` nel seguito). Ne
segue che **non può essere pubblicata da sola**.

*Scenario.* Non è un attacco tecnico ma un fallimento della promessa, che per un
progetto la cui prima pagina promette un reddito di esistenza è la superficie che
conta. Un utente legge «la rete si impegna a tenere sotto il 20 % la quota che
passa da questo fondo», ne deduce che il proprio reddito è protetto entro un
ordine di grandezza, e osserva invece una caduta di due ordini di grandezza in
un'epoca in cui la rete sta rispettando ogni impegno pubblicato. Il progetto ha
già ritirato una metrica una volta ([ADR-007] stessa); ritirarne una seconda per
la stessa ragione — aver misurato la grandezza rivolta all'attaccante e taciuto
quella rivolta all'utente — sarebbe evitabile e non evitato.

*Condizione di chiusura verificabile.*
1. [ADR-007] riceve un'aggiunta in **Consequences** con il testo sopra, più una
   **Review condition** nuova: «rivedere se una misura di campo mostra che la
   diluizione da flotta è osservata in produzione», e la conferma esplicita
   dell'operatore che la perdita è accettata. Se l'operatore non la accetta, la
   risposta non è modificare `α` — che non la cambia — ma riaprire la posizione
   anti-Sybil, e **allora** serve una nuova ADR che superi la 007.
2. La nota onesta di §9 dice oggi che un nodo finto «can only take a slice, never
   bake a bigger cake», il che è vero e insufficiente: la fetta la prende **al
   posto dell'utente**. Va aggiunta una frase che dica, in inglese e senza
   attenuazioni, che la presenza di nodi finti riduce la quota dell'utente e che
   la rete non può impedirlo. Il testo attuale è già buono; questa è la frase che
   gli manca.
3. La riga `SEC-REQ-16` della matrice espone (a), (b), (c) — tutte grandezze
   rivolte all'attaccante o al sistema. Va aggiunta **(d): la frazione di reddito
   che un nodo onesto di sola availability conserva sotto il banco di `AT-07`**,
   che è la grandezza rivolta all'utente e oggi non è obbligatoria da nessuna
   parte.

RF-006 | category=robustness | severity=medium | criterion=I parametri che il limite 5/4 congela sono identificati e la loro irreversibilità è dichiarata | remediation=Aggiungere la tabella dei parametri congelati al rapporto §3

**Tre dei parametri raccomandati non potranno mai più essere cambiati da alcun
documento lecito, e il rapporto non lo dice.** Il limite di variazione 5/4, applicato
a interi piccoli, non è un limite: è un congelamento.

```text
c = validator_churn_cap_seats = 3      -> intervallo lecito [3, 3]   CONGELATO
m = validator_min_capture_epochs = 3   -> intervallo lecito [3, 3]   CONGELATO
validator_cooldown_epochs = 2          -> intervallo lecito [2, 2]   CONGELATO
```

Conseguenze non dichiarate, tutte verificate sopra:

- `ceil(V/T) <= c` con `c = 3` e `T <= 12` implica **`V <= 36` per sempre**.
  `validator_max_set_size = 45` e `validator_max_set_size_max = 81` sono quindi
  **margini di crescita irraggiungibili**: il rapporto motiva 45 come «margine di
  crescita» e 81 come `3V`, e nessuno dei due può essere usato. Un margine che non
  si può occupare non è un margine, ed è meglio saperlo ora che quando la rete
  vorrà crescere.
- Il rapporto §6 misura che a pool 33 un cooldown di **1** conserva tutti e 27 i
  seggi mentre **2** si assesta a 24. Se la rete scoprisse in produzione di avere
  quel pool, **la mossa correttiva non esiste**: il cooldown non è abbassabile.
- Il rapporto motiva `T_max = 12` con l'argomento che «una rete che si scoprisse il
  pool troppo sottile non avrebbe più alcuna mossa lecita» se il tetto fosse 9.
  L'argomento è giusto e vale per `T`; **per `c` e per il cooldown la mossa non
  esiste comunque**, e il rapporto non se ne accorge.

*Scenario.* Nessun avversario. La rete si scopre con un pool di candidati sottile
— la soglia di partecipazione di RF-007 è al 3 % e non è garantita da nulla — e
cerca la leva prevista dal proprio stesso studio: alzare `c` per riempire più
seggi per confine, o abbassare il cooldown. Entrambi i documenti sono **rifiutati
in accettazione** dal limite di variazione. Le sole mosse residue sono alzare `T`
(cricchetto irreversibile, [DEBT-010], e ne restano due passi) o fermarsi. Una
rete che perde la liveness per esaurimento del pool avendo la manopola giusta
sotto gli occhi e non potendola girare è un esito che si previene solo
scrivendolo prima.

*Condizione di chiusura verificabile.* Il rapporto §3 riporta, accanto ai valori,
l'**intervallo lecito** raggiungibile da ciascun parametro sotto il limite 5/4 e i
pavimenti di `ElectionBounds`, e marca esplicitamente i tre congelati. La
conseguenza `V <= 36` e l'irraggiungibilità di `validator_max_set_size = 45` sono
dichiarate. È lavoro documentale in-scope, e non richiede di cambiare alcun valore:
`c = 3`, `m = 3` e `cooldown = 2` restano, a mio giudizio, le scelte giuste
(vedi il giudizio sul cooldown nel seguito).

RF-007 | category=documentation | severity=low | criterion=La soglia di partecipazione sotto cui la rete si ferma è una grandezza pubblicata e sorvegliata | remediation=Promuoverla da cifra del rapporto a metrica dichiarata

**La soglia di partecipazione di 30 candidati è una grandezza di sicurezza e vive
solo in un rapporto di conoscenza.** Il rapporto §6 misura che sotto un pool stabile
di 30 la catena si arresta — in 3 confini con pool zero, in 11 con pool 24 — e che
al minimo aritmetico di 36 tiene tutti i seggi. Alla rete di riferimento significa
che circa il **3 % dei contribuenti** deve essere disposto a candidarsi.

La misura è ben fatta e la conclusione è giusta. Il difetto è la sede: è la sola
condizione nota in cui **la rete si ferma senza che nessuno l'abbia attaccata**, e
non compare in `SEC-REQ`, non ha un `AT-*`, non è nella matrice, e nessuno la
osserverà mai perché nessun documento dice di osservarla. Una perdita di liveness
è una perdita di disponibilità, ed è dentro il perimetro di questo threat model
tanto quanto una cattura.

*Scenario.* Nessun avversario, o un avversario che si limita a **non fare nulla di
illecito**: basta che l'entusiasmo cali. Il pool di candidati scivola da 36 a 24
nell'arco di alcune settimane; nessun allarme esiste perché nessuna soglia è
pubblicata; all'undicesimo confine il set non può più contrarsi sotto 18 e la
catena si ferma. La variante avversariale è a costo quasi nullo e va nominata: un
avversario che censuri una candidatura per una sola epoca rimuove quel nodo per
`1 + cooldown = 3` epoche, quindi **il pool efficace è più fragile del pool
nominale** proprio nella direzione in cui non c'è margine.

*Condizione di chiusura verificabile.* La soglia entra in `threat-model.md` come
grandezza sorvegliata con il suo numero (pool stabile `>= 30` per non arrestarsi,
`>= 36` per tenere `V` seggi, cioè circa il 3 % dei contribuenti della rete di
riferimento), con la nota che il cooldown moltiplica per `1 + cooldown` l'effetto
di una censura sul pool. Lo assumo io come lavoro sul mio documento. La decisione
se promuoverla a `SEC-REQ` con un test d'attacco dedicato è del Lead.

## Required follow-up

**In-scope SPEC-007, prerequisiti di `GATE-SECREVIEW`** — sono qualificazioni di
affermazioni, non cambi di valore, e nessuno tocca `docs/protocol/`:

1. RF-002: `X` dichiarata condizionata alla soglia d'uso ovunque compaia — rapporto
   §1 e §7, righe `SEC-REQ-16`, `SEC-REQ-18` e `AT-07` della matrice, nota di
   prodotto §9. `AT-07` riportato come **parzialmente coperto**.
2. RF-003: l'affermazione su `validator_min_set_size` qualificata come proprietà
   della combinazione raccomandata, con la misura a `V = 36`.
3. RF-001, parte 1: dichiarato per iscritto che `availability = 0`, il tetto di `F`
   e la disciplina 5/4 su `F` sono valori e prassi, non regole, e che i criteri (a)
   e (c) di [ADR-007] sono condizionati alla reward policy attiva.
4. RF-006: intervalli leciti e parametri congelati nel rapporto §3.
5. RF-005, parti 2 e 3: la frase mancante nella nota onesta inglese; la grandezza
   (d) in `SEC-REQ-16`.
6. RF-004 parte 2 e RF-007: convenzione di scrittura dei criteri di `AT-*` e soglia
   di partecipazione in `threat-model.md`. **Li assumo io**, sono sul mio documento.

**Da aprire dal Lead, fuori dallo scope di SPEC-007** — richiedono regole di
validità nuove:

7. RF-001, parte 2: **ADR per un `RewardBounds` di genesi**. È la voce più
   importante di questa review e la sola che chiuda una superficie invece di
   descriverla.
8. RF-003, parte 2: vincolo `3 * validator_min_set_size >= 2 * V` nel blocco di
   vincoli.
9. RF-005, parte 1: **annotazione di [ADR-007]** con conferma esplicita
   dell'operatore, ed eventuale nuova ADR che la superi solo se l'operatore non
   accetta la perdita dichiarata.
10. RF-004, parti 1 e 3: sostituzione del sesto criterio di `AT-10`, che è una
    riduzione di affermazione pubblica e va all'operatore; e una passata di
    verifica della convenzione su tutti gli `AT-*` esistenti.
11. RF-006: nessuna azione di protocollo richiesta, ma il Lead valuti se
    `validator_max_set_size = 45` vada portato a 36, dato che il valore attuale
    dichiara un margine che non esiste.

## Final decision

**Changes requested.** `GATE-SECREVIEW` non è superato con i documenti nello stato
attuale, e lo sarà con i sei punti in-scope sopra, nessuno dei quali cambia un
valore raccomandato.

### Il giudizio sui valori, richiesto esplicitamente

**`α = 0,15` con banda `[0,10 – 0,20]`: i due bordi sono difendibili, con
dichiarazioni accanto, e la banda vale più del punto.**

Il bordo superiore 0,20 è il più solido dei due, perché è **duale a `X`**: la
cattura è strettamente inferiore ad `α` per ogni `N` e ogni `H`, quindi tenere `α`
sotto 0,20 è ciò che rende `X = 20 %` dimostrabile per costruzione invece che per
il particolare banco di prova. L'argomento è corretto e va conservato così com'è.
La sua debolezza non è il valore: è che **nulla tiene `α` sotto 0,20** (RF-001) e
che `α` non ci sta al lancio (RF-002).

Il bordo inferiore 0,10 è difendibile ma **è una scelta di prodotto travestita da
misura**, e va scritto come tale: nessuna grandezza simulata seleziona 0,10, e la
motivazione — sotto, il reddito di esistenza smette di essere un pavimento — è un
giudizio sul significato, non un risultato. Il rapporto lo dice quasi
esplicitamente («non compare in nessun numero di cattura, ed è esattamente per
questo che va scritto come limite») e ha ragione a scriverlo: un limite che
protegge una grandezza che nessun numero di sicurezza vede è precisamente il
limite che sparirebbe per primo sotto pressione di taratura. Ma va accompagnato
dalla dichiarazione di RF-005: **è un pavimento sul significato in assenza di
avversario, e l'attacco che la rete dichiara di tollerare lo attraversa di due
ordini di grandezza.**

`0,15` come punto: concordo con AGENT-002 che nulla lo preferisce a 0,12 o 0,18, e
concordo che sia una scelta dell'operatore. Aggiungo una sola osservazione di
sicurezza: poiché `α` è **osservata** e non impostata, e poiché è massima quando la
rete è più nuova, il punto iniziale conta molto meno del **verso di avvicinamento**.
Partire dal centro significa che la rete attraverserà comunque tutta la banda
dall'alto durante l'avviamento. Se l'operatore volesse un margine, lo prenda sul
bordo **superiore** — è quello duale a `X`, ed è quello che verrà messo alla prova
per primo.

**Il cooldown `= 2`: scelto bene, e va dichiarato irreversibile.** I due argomenti
del rapporto puntano nella stessa direzione e sono corretti — il cooldown
moltiplica per `1 + cooldown` la leva del censore, ed è l'unica grandezza di
elezione il cui aumento aiuta un avversario; e prosciuga il pool senza che serva un
avversario. `1` sarebbe migliore sulla liveness (27 seggi contro 24 a pool 33) e
peggiore sul limite di mandato, che con un'assenza di una sola epoca smette
quasi di mordere. `2` è il compromesso giusto. Ma la scelta va fatta **sapendo che
è definitiva** (RF-006): non c'è un secondo tentativo.

**La soglia di partecipazione va dichiarata**, e non nel rapporto: RF-007.

### La forma esatta in cui ritengo difendibile il claim sulla resistenza Sybil

Non è difendibile in una frase, ed è questo il risultato. È difendibile in quattro,
e ognuna ha una sede diversa perché ognuna è vera per una ragione diversa:

> 1. **Ancorata a una regola, vera sempre.** Una flotta di identità emulate non
>    ottiene alcun seggio di validatore e alcun accredito di storage o compute, a
>    qualunque numerosità, perché l'evidenza di availability contribuisce **zero**
>    al `contribution_score` e la soglia di eleggibilità è positiva. Questa è
>    l'unica delle quattro che una regola di validità impone, ed è la sola che il
>    progetto può dichiarare senza qualificazioni.
> 2. **Ancorata a una regola, vera sempre, con una condizione oggi non imposta.**
>    Una flotta **non aumenta l'emissione totale**: il reddito di esistenza è un
>    fondo a tetto ripartito in parti uguali, `amount = F/E`, con il resto mai
>    coniato. La condizione è che nessun canale paghi un importo per nodo senza
>    tetto — cioè che `availability_microtokens_per_unit` resti zero — e oggi
>    **nessuna regola lo impone** (RF-001).
> 3. **Vera sopra una soglia d'uso, non al lancio.** La quota di emissione che una
>    flotta può catturare è `α · N/(N+H)`, strettamente inferiore ad `α`, e la rete
>    si impegna a tenere `α` sotto il 20 %. L'impegno vincola **al di sopra della
>    soglia d'uso dichiarata**; sotto, `α` tende a 1 per costruzione e la grandezza
>    onesta da pubblicare è l'importo assoluto dirottato (RF-002).
> 4. **Ciò che il progetto non fa, dichiarato per primo e non per ultimo.** Il
>    protocollo non distingue `N` nodi emulati su un host da `N` dispositivi reali
>    e non lo pretende. Ne segue, e va scritto accanto e non altrove, che
>    **abbassare `α` non protegge il dispositivo onesto**: sotto attacco esso
>    conserva `H/(N+H)` del proprio reddito — lo 0,99 % al banco che il progetto
>    dichiara di tollerare — e quel fattore non contiene `α` (RF-005).

Quello che il progetto **non** può dichiarare, e va detto qui perché è l'esito
netto di questa review: che la resistenza Sybil della rete sia una proprietà
stabile. Oggi è una **combinazione di valori ben scelti**, di cui uno soltanto è
difeso da una regola. [ADR-007] dice, correttamente, che «la resistenza Sybil è un
parametro economico, non una garanzia crittografica». Manca la seconda metà: un
parametro economico è governato, e ciò che è governato senza limiti di magnitudine
non è un parametro, è una preferenza. Il progetto lo ha già capito due volte — il
pavimento Argon2id, `ElectionBounds` — e le sue difese economiche sono la terza
volta, non ancora fatta.
