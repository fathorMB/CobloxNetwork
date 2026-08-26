---
id: REVIEW-038
# Note: Quote the title if it contains a colon
title: "GATE-SECREVIEW di SPEC-023: l'analisi dei dieci parametri operativi"
status: changes-requested
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
    id: "REVIEW-038-EVENT-001"
    timestamp: "2026-08-26T23:30:00.000000+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-038-EVENT-002"
    timestamp: "2026-08-26T23:22:15.689592300+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "GATE-SECREVIEW di SPEC-023 non soddisfatta. Tre finding high, quattro medium, quattro low, di AGENT-007 sull'analisi dei dieci parametri operativi.\n\nDue delle tre high cadono dentro la tabella tassonomica, che e' la parte destinata all'ADR quasi verbatim. La terza stabilisce che la classe dei parametri governati e' piu' grande di venti, quindi che il perimetro di DEBT-036 e' piu' stretto del vero.\n\nIl Lead ha riverificato le tre high in modo indipendente contro lo stato committato 065760f, e non contro l'albero di lavoro, perche' SPEC-022 lo stava modificando durante la review. Tutte e tre reggono.\n\nRF-001: app_suspension_notice_epochs e' famiglia 3. L'unita' e' l'epoca di fatturazione, la cui durata e' billing_epoch_ms in un documento diverso firmato dallo stesso quorum, con due sole occorrenze in docs/protocol/ e nessun limite. Una banda in epoche non vincola nulla se il quorum sceglie quanto dura l'epoca.\n\nRF-002: max_clock_drift_ms compare zero volte in ledger.md come token, ed e' normativo sotto la grafia estesa. E' la stessa forma dell'errore su effective_height. La conseguenza e' sostanziale: quel parametro e' la larghezza della finestra di macinatura sul beacon di elezione, che ledger.md quantifica in 10^3-10^6 valori legali a una SHA-256 l'uno, e il vincolo proposto dall'analisi non morde quella proprieta'.\n\nRF-003: EnrollmentParametersBody ha tre campi - max_request_age_ms, max_future_skew_ms, recent_block_window - con una sola occorrenza in README.md, zero in ledger.md, zero nella lista DRAFT, e mai controllati da validate(). Non e' rimediabile dentro SPEC-023 e va aperto come debito proprio.\n\nLa reviewer ha attaccato e non ha rotto: tredici citazioni a ledger.md su tredici, le sei a identity.md, le quattro a wire.md, la tesi che l'analisi ignori ADR-017, la caccia all'undicesimo parametro in RewardPolicyBody e HostingRateCardBody dove ha enumerato tutti e diciotto i campi, e la distinzione registrata dal Lead in DEBT-036 su max_weak_subjectivity_age_ms, che regge ed e' piu' forte di come l'analisi la scrive. DEBT-036 non va corretto su quel punto.\n\nTre righe della tabella su dieci sono utilizzabili cosi', tre hanno la classificazione giusta e un termine da correggere, quattro vanno riclassificate.\n\nNessuna remediation e' dispacciata con questo verdetto, per decisione dell'operatore: SPEC-022 era in working sullo stesso albero e sugli stessi documenti. Ora e' in review, quindi la remediation dell'analisi e' dispacciabile.\n\nRF-009 va colto sulla passata di ADR-012 di SPEC-022 prima che quella spec chiuda, ed e' gia' materializzato: consensus_parameters_closure.py conta 22 campi e stampa \"Classification of all 20 fields\", con il venti cablato nella riga di stampa."
    evidence_refs: ["SPEC-023", "DEBT-036", "SPEC-022", "ADR-017", "ADR-012"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-038-EVENT-003"
    timestamp: "2026-08-26T23:38:35.119657700+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Rimediati RF-001, RF-002, RF-004, RF-005, RF-006, RF-007, RF-008, RF-010, RF-011 in .lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md. RF-003 lasciato fuori scope (debito proprio, non rimediabile qui). RF-009 non di mia competenza (passata ADR-012 di SPEC-022). Nessun file toccato fuori dal perimetro assegnato (solo il documento di analisi; letture di verifica su docs/protocol/ledger.md, identity.md, wire.md, README.md e core/coblox-core/src/params.rs)."
    evidence_refs: ["SPEC-023", "REVIEW-038", "DEBT-036"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-038-EVENT-004"
    timestamp: "2026-08-26T23:54:12.978569700+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Convertite tutte le citazioni di riga dell'analisi dei dieci parametri operativi (.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md) in citazioni a frase o a campo/blocco di schema, verificate cercandole nei documenti sorgente (docs/protocol/ledger.md, identity.md, wire.md, README.md, core/coblox-core/src/params.rs). Aggiornata anche la sezione \"Existing-project analysis\" di .lmbrain/specs/review/SPEC-023-i-dieci-parametri-operativi-nella-lista-draft-e-la-gate-che-chiude-la-classe.md con lo stesso schema. Nessun numero di riga nudo resta nell'analisi. Trovata e segnalata (non corretta) una citazione con documento sbagliato (README.md invece di ledger.md per il fail-closed su now-issued_at_ms) e una qualifica (\"stallo irreversibile\") non letteralmente supportata dal testo sorgente, che dice solo \"stalls\"."
    evidence_refs: [".lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md", ".lmbrain/specs/review/SPEC-023-i-dieci-parametri-operativi-nella-lista-draft-e-la-gate-che-chiude-la-classe.md"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
links: [DEBT-036]
created: 2026-08-26
updated: 2026-08-26
tags: [security, review, governance]
related_decisions: [ADR-012, ADR-010, ADR-017]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-26
    action: "recorded review remediation"
  - date: 2026-08-26
    action: "recorded review remediation"
---
# Review

> **`GATE-SECREVIEW` di [SPEC-023].** L'oggetto è `.lmbrain/knowledge/analisi-dieci-parametri-operativi-consensus.md`, **non** lo strumento: la gate è stata scritta così di proposito, perché lo strumento si verifica eseguendolo — e il Lead l'ha eseguito in [REVIEW-037] — mentre l'analisi **diventerà un ADR**, la seconda metà di [DEBT-036]. Un errore lì si propaga dentro una decisione di protocollo.

## Outcome

**`GATE-SECREVIEW` non soddisfatta.** Tre finding `high`, quattro `medium`, quattro `low`.

Due delle tre `high` cadono **dentro la tabella tassonomica**, che è la parte del documento destinata a entrare nell'ADR quasi verbatim. La terza stabilisce che **la classe dei parametri governati è più grande di venti**, e quindi che il perimetro di [DEBT-036] è più stretto del vero.

**La consegna non va rifatta.** Tre righe della tabella su dieci sono utilizzabili così, tre hanno la classificazione giusta e un termine da correggere, quattro vanno riclassificate. Lo strumento, la lista DRAFT e il cablaggio in CI non sono in discussione: erano [REVIEW-037] e sono chiusi.

## Acceptance-criteria compliance

Non applicabile: questa review copre una gate `before-done` e non i criteri di accettazione, chiusi in [REVIEW-037].

## Code observations

Lo strumento **non è in difetto** per F-03: la sua docstring dichiara il perimetro `ConsensusParametersBody`, e dentro quel perimetro è corretto. **È il perimetro della chiusura a essere più stretto della classe.**

`EnrollmentParameters::validate()` in `core/coblox-core/src/params.rs` **legge tre campi in `from_body` e non li controlla mai**. Verificato dal Lead in modo indipendente contro `HEAD`.

## Tests and verification

La reviewer **non ha rieseguito** lo strumento né alcuna gate di progetto, e lo dichiara: erano state eseguite dal Lead in [REVIEW-037], con `GATE-SEEN-IT-FAIL-FIRST` riprodotta e non letta. Ha letto la docstring e `negative_proof()` per stabilire il perimetro dichiarato, che è il presupposto di F-03.

**Il Lead ha riverificato in modo indipendente le tre `high` contro lo stato committato `065760f`**, e non contro l'albero di lavoro, per la ragione scritta nella nota di perimetro qui sotto. Tutte e tre reggono.

## Nota di perimetro: la review è stata scritta su un albero in movimento

**Va detto prima dei finding, perché condiziona come leggerli.** Mentre AGENT-007 rivedeva, [SPEC-022] era in `working` e AGENT-002 stava modificando `ledger.md`, `identity.md`, `README.md`, `params.rs` e sei altri file. **I numeri di riga di questa review sono già scaduti**: cita `ledger.md:695` e `802`, che a `HEAD` sono `739` e `829`, e `ledger.md:628` che nell'albero di lavoro è già `584`.

**Le conclusioni sopravvivono**, perché riguardano l'**assenza** di un vincolo — un campo che nessuna lista copre, una regola che nessun `validate()` impone — e un'assenza non si sposta con le righe. Il Lead le ha riverificate contro `HEAD` una per una.

**Ma la forma va registrata**, perché è nuova per questo progetto: *far rivedere un documento mentre un altro agente riscrive le fonti che quel documento cita*. Le citazioni restano vere e i puntatori muoiono, che è il difetto già visto su [DEBT-033] e [DEBT-034] — qui prodotto dal processo invece che dal tempo.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security | severity=high | criterion=tabella tassonomica riga 9 | remediation=riclassificare come banda a due lati relazionale con `billing_epoch_ms`, o banda sul prodotto
**`app_suspension_notice_epochs` è famiglia 3, ed è la riga peggiore della tabella.** L'analisi la classifica «banda a due lati in epoche». **L'unità è l'epoca di fatturazione** — `ledger.md` dice *«At each billing epoch consensus first records a `grace` transition... Suspension becomes effective only after `app_suspension_notice_epochs`»* — e la sua durata è `billing_epoch_ms`, campo di `HostingRateCardBody`, **documento diverso firmato dallo stesso quorum sedente**. Quel campo ha **due occorrenze in tutto `docs/protocol/`**: la riga di schema e la voce nella lista DRAFT. Nessun limite di genesi, nessuna regola di validità, nessun MUST relazionale. *Verificato dal Lead contro `HEAD`.*

**Scenario.** Un quorum che vuole sfrattare un'app senza dichiararlo. Capacità: la sola soglia di quorum sui documenti firmati. (1) Pubblica `consensus_parameters` con `app_suspension_notice_epochs` **dentro** la banda che l'ADR fisserà. (2) Pubblica `hosting_rate_card` che porta `billing_epoch_ms` da un giorno a pochi secondi. Entrambi accettati da ogni validatore conforme. **Guadagno:** la finestra di rimedio reale collassa da giorni a secondi e ogni `fund_app` in volo arriva tardi — la sospensione immediata che la banda esisteva per vietare, **senza violare la banda**.

RF-002 | category=security | severity=high | criterion=tabella tassonomica riga 1 | remediation=riclassificare come magnitudine assoluta in ms tarata sul costo di macinatura, correggere le due citazioni false, aggiungere i siti normativi sotto la grafia estesa
**`max_clock_drift_ms`: due citazioni false, i siti normativi mancati, e la grandezza sbagliata.** Tre difetti che si sommano.

**(a)** Il token `max_clock_drift_ms` compare **zero volte in `ledger.md`**. La grandezza è normativa sotto una **seconda grafia**, *«the maximum clock drift»*, in due righe di `ledger.md` che l'analisi non cita. **È la stessa forma dell'errore su `effective_height` in [ADR-017]**: enumerazione fatta sul token mentre l'oggetto è una grandezza con due grafie. *Verificato dal Lead contro `HEAD`.*

**(b)** Delle tre citazioni del §1, una è la riga di schema ed è corretta; `identity.md:503` parla di **un altro parametro** — `max_transport_attestation_future_skew_ms` — e la terza punta al bullet economico della lista DRAFT.

**(c) La conseguenza è sostanziale.** Il testo non trovato dice che `timestamp_ms` è vincolato *«solo a superare la mediana degli undici blocchi finalizzati precedenti e a non eccedere la deriva massima dell'orologio»*, e che quella finestra ammette **10³–10⁶ valori legali, ciascuno un `block_id` diverso al costo di una SHA-256**. `max_clock_drift_ms` è quindi **la larghezza della finestra di macinatura sul beacon di elezione**. Il vincolo proposto dall'analisi — metà di `block_interval_ms` — lascia 2 500 candidati a un hash l'uno: **non morde la proprietà**. *Verificato dal Lead contro `HEAD`.*

**Scenario.** Un proposer colluso con un issuer che gli consegna il segreto committato — lo scenario che `ledger.md` nomina come *«ciò che resta aperto, con il suo costo»*. Enumera `timestamp_ms` dentro la finestra ricalcolando un SHA-256 per valore finché il beacon accoppia quell'issuer al soggetto bersaglio. **Guadagno:** assegnazione dei challenge pilotata. **Il costo dell'attacco è lineare in `max_clock_drift_ms`, e quel parametro oggi non ha tetto.**

**Aggravante minore:** la tolleranza è **a una sola direzione** — «non più della deriva massima **dopo** che la proposta è ricevuta» — mentre l'indietro è tenuto dalla mediana degli undici. L'analisi la tratta come simmetrica, che è la forma di [ADR-013].

RF-003 | category=security | severity=high | criterion=perimetro di DEBT-036 | remediation=aprire un debito proprio; non è rimediabile dentro SPEC-023
**L'undicesimo parametro esiste, ed è un terzetto: la classe governata è più grande di venti.** `EnrollmentParametersBody` ha nove campi, sei coperti da una regola di validità o da un blocco di limiti. **Tre non lo sono:** `max_request_age_ms`, `max_future_skew_ms`, `recent_block_window`.

Verificato dal Lead contro `HEAD` in tre modi: ciascuno compare **una volta sola** in `README.md`, la riga di schema; **zero** volte in `ledger.md`; **zero** nella lista DRAFT. E `EnrollmentParameters::validate()` li legge in `from_body` e **non li controlla mai**.

Sono della **stessa famiglia dei dieci** — età di una richiesta, tolleranza di skew in avanti, finestra di freschezza — e non della famiglia economica. `identity.md` lo dice apertamente: `max_transport_attestation_future_skew_ms` è stato modellato *«sul modello del `max_future_skew_ms` che la finestra di enrollment già usa»*. **L'analisi analizza il figlio e non vede il padre.**

**Scenario, su `recent_block_window`, che è il più concreto.** La prova Argon2id è ancorata a `recent_block_id`, e `recent_block_height` non può stare più di `recent_block_window` dietro l'ultima altezza finalizzata. Un quorum che voglia diluire la difesa anti-Sybil pubblica un `enrollment_parameters` con quella finestra enorme: nessuna regola lo rifiuta. **Guadagno:** un attaccante **precalcola** tag Argon2id contro un blocco vecchio per l'intera durata della finestra e poi riversa N enrollment in un colpo. Il costo per identità non cambia; **il costo di picco per una flotta crolla**, che è la proprietà su cui [ADR-007] poggia. Simmetricamente, a zero nessun enrollment è più costruibile e la rete si chiude alle identità nuove.

**Nota per [ADR-012], dalla reviewer:** la lista DRAFT scrive la terna Argon2id come `memory_kib`, `lanes`, `passes`. Il campo si chiama **`iterations`**; `passes` è il termine della RFC 9106, non il nome del campo. È di nuovo una grandezza con due grafie, e il `grep` di uno strumento futuro non la troverà.

RF-004 | category=correctness | severity=medium | criterion=tabella tassonomica riga 3 | remediation=magnitudine su `D+S` più relazionale con `max_weak_subjectivity_age_ms`, oppure dichiarare il terzo termine non limitato
**La finestra di esposizione di `max_transport_attestation_validity_ms` ha tre termini, non due, e l'analisi cita la fonte che lo dice.** L'analisi scrive `D_max + S_max` e classifica «magnitudine sulla somma». Ma le righe immediatamente precedenti a quella citata dicono che la finestra *«non è `expires_at_ms - created_at_ms`: è la somma, **spostata di quanto l'orologio del ricevente sia indietro**... il residuo è l'età del checkpoint, **che nessuna regola di questo protocollo limita**»*. Il codice lo ripete: solo i primi due termini sono limitati da un parametro. Il terzo vale `max_weak_subjectivity_age_ms` per chi ha un checkpoint e **nulla** per chi non ne ha.

RF-005 | category=correctness | severity=medium | criterion=tabella tassonomica riga 7 | remediation=la colonna del rischio massimo deve leggere fail-closed di flotta, e il §7 deve dire che il MUST non è imposto all'accettazione
**`max_weak_subjectivity_age_ms`: il danno all'estremo massimo è quello sbagliato.** L'analisi scrive *«attacco long-range»*. Ma il valore operativo è quello **nel checkpoint firmato**, mai uno appreso da un peer, con fail-closed obbligatorio sul disaccordo: il campo on-chain è un filo d'inciampo di coerenza. Un quorum che lo alza **non compra un long-range attack** — quello richiede la chiave di distribuzione, che nessun quorum controlla. Compra un'**arma di liveness**: il disaccordo fa fallire chiuso ogni light client conforme in possesso di un header autenticato. DoS di flotta, un documento, costo zero.

**Seconda parte:** il MUST che lega il parametro a `min_revocation_effective_delay_blocks` parla di *«durata wall-clock attesa»*, che non è un predicato calcolabile su un documento — e infatti **non è nel blocco dei vincoli e non è in `params.rs`**. Presentarlo sotto «cosa già lo vincola» sopravvaluta la copertura attuale. *Il Lead conferma: il blocco dei vincoli, letto per intero, non lo contiene.*

RF-006 | category=correctness | severity=medium | criterion=tabella tassonomica righe 5 e 6 | remediation=una relazione sola, `N_peers × per_peer <= global`, e il danno massimo riscritto come DoS incrociato
**La relazione proposta è vacua, e i §5 e §6 ne propongono due diverse per la stessa coppia.** Il §5 propone `per_peer <= global`, il §6 propone `global >= k × per_peer`. La tabella li appiattisce entrambi. Il primo è vacuo: ammette `per_peer == global`. E la regola di rifiuto **non evince voci ancora vive**, quindi le voci restano fino alla scadenza della busta, che lo stesso quorum fissa.

**Scenario.** Un attaccante con **una sola identità enrollata**. Emette `per_peer` buste con validità massima; le voci non sono evincibili per regola. **Guadagno:** per l'intera finestra, ogni busta di **qualunque peer onesto** è rifiutata come `rate_limited` sul nodo bersaglio. Non serve né banda sostenuta né OOM.

RF-007 | category=security | severity=medium | criterion=analisi, domanda mancante | remediation=aggiungere per ciascuno dei dieci se il pavimento sia necessario alla liveness e da quale grandezza dipenda
**Il rischio che nessuno ha nominato: otto dei dieci falliscono al pavimento, e nessuno al tetto.** I valori in albero sono tutti `1`, e sono fixture di test — la reviewer verifica che **non esista alcun documento di genesi con valori reali**. Se una distribuzione fosse costruita da lì: a `1 ms` di deriva nessun blocco finalizza; a `1 ms` di validità nessuna busta passa un hop; a `1` voce di cache la seconda busta da chiunque è `rate_limited`; a `1 ms` di età del checkpoint ogni light client fallisce chiuso; a `1 ms` nessuna query di saldo riesce.

**Il punto per l'ADR non è la lista.** È che l'analisi propone un pavimento per **tre** parametri su dieci, e sui sette dove manca **il modo in cui la rete muore davvero è quello che l'ADR non limiterà**. Non serve un avversario: serve un operatore che costruisca una devnet dai valori in albero, che è la via di minor resistenza.

RF-008 | category=documentation | severity=low | criterion=tabella tassonomica riga 4 | remediation=la riga porta entrambi i termini
Il §4 di `max_transport_attestation_future_skew_ms` scrive «relazionale **più un tetto di magnitudine in genesi**». La riga di tabella **perde il tetto**. Poiché l'ancora a cui la relazione lo lega è a sua volta senza tetto (RF-002), la riga da sola è famiglia 3. **La tabella è la parte che l'operatore userà.**

RF-009 | category=documentation | severity=low | criterion=ADR-012 | remediation=i due file entrano nella lista di passata di SPEC-022
**[SPEC-022] renderà falsa la prosa di due artefatti, e nessuna gate lo coglierà.** L'analisi apre con «`ConsensusParametersBody` definisce **venti** parametri» e la docstring dello strumento dice «twenty», «Ten election», «The other ten». [SPEC-022] ne aggiunge **due**. La lista di passata di [ADR-012] in [SPEC-022] **non nomina né l'analisi né lo strumento**, e nessuno dei due è nell'inventario, il cui perimetro dichiarato è `docs/protocol/`. **Lo strumento non fallirà** — i due campi nuovi entrano nel blocco dei vincoli e la sua condizione è soddisfatta: falsi saranno i **numeri nella prosa**, che è il caso peggiore. Famiglia 1.

RF-010 | category=correctness | severity=low | criterion=analisi §9 | remediation=togliere `reward_epoch_ms`
Il §9 scrive che il parametro «opera sul ciclo di fatturazione definito da `billing_epoch_ms` **e `reward_epoch_ms`**». `reward_epoch_ms` non governa la fatturazione delle app. Nominare due ancore **sfoca precisamente la dipendenza che RF-001 chiede di rendere relazionale**.

RF-011 | category=correctness | severity=low | criterion=analisi §8 | remediation=nominare entrambi
`max_current_balance_age_ms`: manca un danno e manca un credito. Il danno — quel parametro sta dentro il passo *«Corroborate freshness... l'accordo fra peer è un allarme di disponibilità/**fork**»*, quindi alzarlo **acceca l'allarme di fork**, non solo la freschezza del saldo. Il credito — la non-regressione e il vincolo sull'altezza esatta limitano già in parte il danno.

## Giudizio sulla tabella tassonomica, riga per riga

È la parte che l'operatore userà per l'ADR.

| # | parametro | classificazione dell'analisi | giudizio della reviewer |
| --- | --- | --- | --- |
| 1 | `max_clock_drift_ms` | Relazionale + Magnitudine | **Da riclassificare.** → magnitudine assoluta in ms tarata sul costo di macinatura, pavimento sul jitter, asimmetria dichiarata (RF-002) |
| 2 | `max_envelope_validity_ms` | Magnitudine | **Verso giusto, incompleta.** → aggiungere il relazionale con le due cache |
| 3 | `max_transport_attestation_validity_ms` | Magnitudine su `D+S` | **Da correggere.** → più relazionale con l'età del checkpoint (RF-004) |
| 4 | `max_transport_attestation_future_skew_ms` | Relazionale | **Riga in disaccordo col proprio corpo.** → relazionale **+ magnitudine** (RF-008) |
| 5 | `replay_cache_entries_per_peer` | Pavimento + Relazionale | **Da riclassificare.** → relazione con `N_peers`, danno riscritto (RF-006) |
| 6 | `replay_cache_entries_global` | Pavimento + Relazionale | **Utilizzabile così.** La riga più solida delle dieci |
| 7 | `max_weak_subjectivity_age_ms` | Strettamente Relazionale | **Classificazione giusta, danno sbagliato** (RF-005) |
| 8 | `max_current_balance_age_ms` | Magnitudine | **Utilizzabile così**, due integrazioni non bloccanti (RF-011) |
| 9 | `app_suspension_notice_epochs` | Banda a due lati in epoche | **Da riclassificare — famiglia 3.** La riga da non lasciar passare (RF-001) |
| 10 | `min_revocation_effective_delay_blocks` | Banda + Relazionale | **Utilizzabile così, senza riserve.** Coincide con [ADR-017] e con il blocco di [SPEC-022] |

## Cosa la reviewer ha attaccato senza riuscire a romperlo

1. **Tutte le citazioni a `ledger.md` dell'analisi**, aperte una per una: **tredici su tredici reggono**, incluse le due che il Lead aveva già campionato.
2. **Le sei citazioni a `identity.md`** per i parametri 2, 3 e 4: reggono.
3. **Le quattro citazioni a `wire.md`**: reggono.
4. **La tesi che l'analisi ignori [ADR-017]** (domanda 5 dell'incarico). **Non regge: l'analisi lo riporta correttamente**, e la classificazione della riga 10 coincide con il blocco di vincoli che [ADR-017] decide e che [SPEC-022] sta portando in albero.
5. **La caccia all'undicesimo parametro in `RewardPolicyBody` e `HostingRateCardBody`**: enumerati tutti e diciotto i campi contro tre coperture. **Lì non c'è.** I due che sembravano scoperti non lo sono. L'ha trovato altrove, ed è RF-003.
6. **La distinzione registrata dal Lead in [DEBT-036] su `max_weak_subjectivity_age_ms`.** L'ha attaccata e **non l'ha rotta — regge, ed è più forte di come l'analisi la scrive**: il checkpoint firmato non solo vincola quel parametro, lo **sostituisce** per la proprietà in questione. **[DEBT-036] non va corretto su questo punto.**
7. **La tesi che `block_interval_ms` sia un'ancora relazionale sicura**: regge, è costante di genesi e nessun quorum la scrive.

## Cosa la reviewer non ha guardato

- **Non ha rieseguito lo strumento né alcuna gate di progetto.** Eseguite dal Lead in [REVIEW-037], `GATE-SEEN-IT-FAIL-FIRST` riprodotta. Ha letto docstring e prova in negativo solo per stabilirne il perimetro, che è il presupposto di RF-003.
- **Non ha verificato la lista DRAFT** per perdita di voci, colonne o LaTeX: proprietà di forma, verificate dal Lead.
- **Non ha letto `.lmbrain/knowledge/threat-model.md`**: ha verificato TM-37 attraverso la fonte di protocollo che l'analisi cita.
- **Non ha letto per intero [ADR-010], [ADR-013], [ADR-015], [ADR-016]** — solo le righe citate dalle fonti di protocollo. [ADR-017] letto per intero perché l'incarico lo chiedeva.
- **Non ha valutato i dieci parametri di elezione**: fuori perimetro e già coperti.
- **Non ha eseguito alcun attacco reale.** Nessun nodo, nessuna rete. **Ogni scenario è derivato da regole lette, mai osservato in esecuzione.** Il perimetro di ogni finding è la lettura, non l'esperimento.

## Required follow-up

**Nessuna remediation è stata dispacciata**, per decisione dell'operatore: [SPEC-022] è in `working` sullo stesso albero e sugli stessi documenti, e due remediation concorrenti sono la condizione che ha già reso inaffidabili i puntatori di questa review.

Quando [SPEC-022] sarà consegnata:

- **RF-001, RF-002, RF-004, RF-005, RF-006, RF-008, RF-010, RF-011** vanno ad AGENT-002 come remediation dell'analisi. Nessuna tocca lo strumento.
- **RF-007** aggiunge una domanda all'analisi — il pavimento, per ciascuno dei dieci.
- **RF-003 non è rimediabile dentro [SPEC-023]** e va aperto come debito proprio, con l'estensione del perimetro dello strumento a `EnrollmentParametersBody`.
- **RF-009 va colto sulla passata di [ADR-012] di [SPEC-022]**, e va segnalato **prima** che quella spec chiuda, perché è lì che i due campi nuovi rendono falsa la prosa dei due file.
