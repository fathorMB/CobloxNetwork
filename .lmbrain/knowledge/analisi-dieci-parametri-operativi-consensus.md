---
title: "Analisi dei dieci parametri operativi di ConsensusParametersBody"
spec: SPEC-023
debt: DEBT-036
author: AGENT-002
updated: 2026-08-26
---

# Analisi dei dieci parametri operativi di ConsensusParametersBody

Prodotta da [SPEC-023] per chiudere la prima metà di [DEBT-036] e preparare l'ADR con cui l'operatore deciderà la seconda metà (le regole di genesi e i limiti dei parametri operativi).

Questa analisi **prepara la decisione e non la prende**: nessun valore di lancio viene fissato in questa sede, e nessun limite viene iniettato nel blocco dei vincoli senza approvazione dell'operatore.

---

## 1. Premessa e metodo di analisi

`ConsensusParametersBody` raggruppa tre famiglie di campi, e il totale non è il modo giusto di riferirsi al suo perimetro: il conteggio cambia a ogni spec che vi tocca ed è già cambiato una volta dentro questa stessa catena di lavoro (venti in [SPEC-023], ventidue dopo [SPEC-022]). I gruppi sono: i parametri di elezione e rotazione del validator set (da `election_epoch_blocks` a `validator_min_capture_epochs`), i parametri della banda di revoca (`min_revocation_effective_delay_blocks`, `revocation_effective_grace_blocks`, `max_planned_revocation_delay_blocks`), e i dieci parametri operativi e di sicurezza che sono l'oggetto di questa analisi. Il primo gruppo gode di due livelli di tutela:
1. Una presenza esplicita nella lista DRAFT dei parametri aperti prima del mainnet;
2. Un insieme di vincoli relazionali e di magnitudine in `docs/protocol/ledger.md#rotation-the-cap-and-the-floor` ancorati all'oggetto di genesi `ElectionBounds`.

Il terzo gruppo — i dieci parametri **operativi e di sicurezza della rete** — è l'oggetto di questa analisi: orologi, finestre temporali delle buste, tolleranze di skew, cache anti-replay, freschezza dell'ancora di fiducia e ritardi di revoca.

Per ciascuno dei dieci parametri, questa analisi risponde a **cinque domande fondamentali**:
1. **Cosa governa**, in una frase, con le righe esatte del documento di protocollo che lo stabiliscono.
2. **Cosa ottiene un quorum sedente che lo porta al massimo**, e cosa ottiene portandolo al minimo (gli estremi non sono simmetrici, come dimostrato da [ADR-013] e [ADR-016]).
3. **Cosa già lo vincola per altra via**: un altro parametro legato da un MUST, un secondo canale (come il checkpoint di distribuzione), o nulla.
4. **Da quale grandezza dipende la proprietà che vorremmo**: se dalla grandezza stessa (vincolo di **magnitudine** / tetto assoluto) o dalla relazione tra grandezze diverse (vincolo **relazionale**). Questa distinzione è critica per non cadere nella *famiglia 3* dei difetti ricorrenti (vincolare la grandezza nominata invece di quella da cui dipende la proprietà).
5. **Il pavimento è necessario alla liveness, e da quale grandezza dipende?** Aggiunta dopo [REVIEW-038] (RF-007): i valori oggi in albero sono fixture di test, tutti a `1`, e nessuna regola v0 impedisce a una distribuzione di adottarli in genesi così come sono. Con quei valori, **otto dei dieci parametri falliscono al pavimento** (deriva zero blocca la finalizzazione, validità zero scarta ogni busta al primo hop, una sola voce di cache rate-limita la seconda busta di chiunque, età zero del checkpoint fa fallire chiuso ogni light client, e così via) e **nessuno fallisce al tetto**. Questa analisi propone un pavimento esplicito solo per tre dei dieci; per gli altri sette la domanda è aperta e va risposta prima che l'ADR fissi i limiti.

---

## 2. Analisi dei dieci parametri

---

### 1. `max_clock_drift_ms`

> **Corretto dopo [REVIEW-038] (RF-002).** La versione precedente citava `identity.md:503`, che parla di un altro parametro (`max_transport_attestation_future_skew_ms`), e un bullet della lista DRAFT come se fossero normativi. La grandezza è normativa sotto una **seconda grafia** — *"the maximum clock drift"* — in `docs/protocol/ledger.md:703` e `:810`, che l'analisi precedente non citava e quindi non leggeva.

- **1. Cosa governa:**
  La massima discrepanza temporale tollerabile tra l'orologio locale del nodo ricevente e i timestamp registrati su blocchi, messaggi o attestazioni. Riga di schema: `docs/protocol/README.md:811`. Siti normativi sotto la grafia estesa *"the maximum clock drift"*: `docs/protocol/ledger.md:703` (finestra di macinatura sul beacon di elezione) e `docs/protocol/ledger.md:810` (vincolo su `timestamp_ms` del `BlockHeader`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* `ledger.md:701-709` quantifica la conseguenza. `timestamp_ms` è vincolato solo a superare la mediana degli undici blocchi finalizzati precedenti e a non eccedere `max_clock_drift_ms` dopo la ricezione; a granularità di millisecondo quella finestra ammette **10³–10⁶ valori legali**, ciascuno un `block_id` diverso al costo di **una SHA-256**. Un proposer colluso con un issuer che gli consegna il segreto committato può enumerare quel campo — non "scartare blocchi e riprovare", **macinare un hash per candidato** — cercando un beacon che accoppi quell'issuer al soggetto bersaglio. Il costo dell'attacco è **lineare in `max_clock_drift_ms`**.
  - *Portato al minimo (es. 0 o pochi ms):* Causa un arresto operativo diffuso (liveness failure): qualsiasi fisiologico jitter NTP o normale latenza di propagazione geografica induce i nodi a scartare i blocchi e i messaggi validi emessi da peer onesti.
- **3. Cosa già lo vincola per altra via:**
  Nessuna regola on-chain. All'esterno della catena, l'ancora di fiducia di genesi definisce `CadenceBand.max_external_clock_slack_ms` ([ADR-016]), che limita la tolleranza usata dai light client durante la verifica del ritmo di emissione, ma non vincola il parametro firmato dentro `ConsensusParametersBody`.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà non è "la deriva sia una frazione ragionevole di `block_interval_ms`": è che la finestra di macinatura sul beacon di elezione resti troppo costosa da percorrere per intero. Il vincolo relazionale proposto dalla versione precedente — metà di `block_interval_ms`, cioè 2 500 ms — lascerebbe **2 500 candidati a un hash l'uno**, un ordine di grandezza dentro la banda quantificata da `ledger.md:704` (10³–10⁶): **non morde la proprietà**.
  - **Tipo di vincolo naturale:** **Magnitudine assoluta in millisecondi, tarata sul costo di macinatura** (il numero di SHA-256 che un proposer colluso può permettersi per candidato di beacon, non una frazione di `block_interval_ms`). Un pavimento resta necessario ed è di natura diversa dal tetto: deve assorbire il jitter di rete e le tolleranze ordinarie dei demoni NTP.
  - **Asimmetria dichiarata:** la tolleranza è **a una sola direzione**. `ledger.md:808-810` vieta solo che il timestamp ecceda la deriva massima **dopo** la ricezione della proposta; all'indietro il vincolo è tenuto dalla mediana degli undici blocchi precedenti, non da `max_clock_drift_ms`. Trattarla come simmetrica è la forma dell'errore che [ADR-013] nomina.
- **5. Pavimento e liveness:** **Necessario.** A `max_clock_drift_ms = 0` (il valore oggi in albero) nessun blocco con jitter di rete fisiologico finalizza: è la lettura letterale del punto 2 al minimo. Il pavimento dipende dal jitter NTP e dalla latenza di propagazione, non dal costo di macinatura — le due grandezze che fissano tetto e pavimento sono diverse.

---

### 2. `max_envelope_validity_ms`

- **1. Cosa governa:**
  La durata massima di validità di una busta firmata nel protocollo wire dal momento della sua emissione, ossia `expires_at_ms - created_at_ms` (`docs/protocol/README.md:810`, `docs/protocol/wire.md:106`, `docs/protocol/identity.md:563`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Buste con durata di vita di giorni o settimane. Poiché i nodi mantengono in cache i `message_id` fino alla scadenza, ciò comporta la saturazione irreversibile della cache anti-replay o l'esposizione a replay tardivi una volta raggiunti i limiti di capienza.
  - *Portato al minimo:* Buste che scadono prima di poter completare la propagazione gossip multi-hop nella topologia P2P o prima di essere elaborate dal ricevente, causando lo scarto sistematico di messaggi e transazioni lecite.
- **3. Cosa già lo vincola per altra via:**
  `wire.md:106` impone la regola di validità sintattica `expires_at_ms - created_at_ms <= max_envelope_validity_ms` e il rifiuto di `expires_at_ms < created_at_ms`. Non vi è alcun limite di genesi né accoppiamento vincolante con la dimensione della cache.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà dipende dal diametro della rete P2P e dalla latenza massima di propagazione GossipSub (ordine di pochi secondi), relazionata al tempo di ritenzione sostenibile nella cache anti-replay.
  - **Tipo di vincolo naturale:** **Magnitudine (Tetto massimo di genesi)**. Un tetto superiore fisso (es. nell'ordine di 30–60 secondi) impedisce al quorum di trasformare messaggi effimeri in credenziali persistenti.
- **5. Pavimento e liveness:** **Necessario.** A validità zero o quasi zero, come nota il punto 2 al minimo, una busta scade prima di completare la propagazione gossip multi-hop: scarto sistematico di messaggi e transazioni lecite. Il pavimento dipende dal diametro della rete P2P e dalla latenza massima di propagazione GossipSub.

---

### 3. `max_transport_attestation_validity_ms` ($D_{\max}$)

> **Corretto dopo [REVIEW-038] (RF-004).** La finestra di esposizione ha **tre** termini, non due: la somma $D_{\max}+S_{\max}$ non è la finestra intera, ed `identity.md` lo dice esplicitamente immediatamente prima della riga già citata.

- **1. Cosa governa:**
  La durata massima di validità dell'attestazione della chiave di trasporto effimera (`TransportKeyAttestation`) firmata dalla chiave di identità permanente (`docs/protocol/README.md:811`, `docs/protocol/identity.md:500`, `556`, `576`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Dilata la finestra temporale in cui una chiave di trasporto sottratta o compromessa può essere usata per impersonare il nodo nelle connessioni P2P dirette (scenario TM-37 nel threat model) senza alcuna possibilità di revoca on-chain anticipata.
  - *Portato al minimo:* Costringe tutti i nodi della rete a invocare continuamente la chiave di identità principale per firmare nuove attestazioni effimere a intervalli brevissimi, aumentando drasticamente il rischio operativo e il carico crittografico sulle chiavi fredde.
- **3. Cosa già lo vincola per altra via:**
  `identity.md:576` formalizza che la finestra totale di accettazione da parte dei peer è la somma:
  $$\text{accepted\_window} = \text{max\_transport\_attestation\_validity\_ms} + \text{max\_transport\_attestation\_future\_skew\_ms}$$
  Ma quella somma **non è la finestra di esposizione reale**. `identity.md:547-551`, immediatamente prima della riga citata sopra, lo dice per esteso: la finestra *"non è `expires_at_ms - created_at_ms`: è la somma [...] spostata di quanto l'orologio del ricevente sia indietro"*; quello scarto, per un ricevente che ha un checkpoint, si riduce all'età del checkpoint stesso — *"che nessuna regola di questo protocollo limita"*. Il codice della sezione 5 ripete lo stesso punto: solo i primi due termini sono limitati da un parametro firmato. Il terzo termine vale `max_weak_subjectivity_age_ms` per un ricevente che possiede un checkpoint, e **nulla** per uno che non ne possiede.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà cercata è il contenimento della finestra di esposizione per compromissione di chiave effimera, e quella finestra è a tre termini: $D_{\max} + S_{\max} + \text{età del checkpoint del ricevente}$.
  - **Tipo di vincolo naturale:** **Magnitudine sulla somma $D_{\max}+S_{\max}$, più relazionale con `max_weak_subjectivity_age_ms` per il terzo termine.** Un tetto assoluto sulla somma dei primi due limita la parte del protocollo che ha un canale per farlo; il terzo termine non ha oggi alcun vincolo, e finché resta così un tetto sui primi due non è un tetto sulla finestra di esposizione reale — solo su una sua parte.
- **5. Pavimento e liveness:** **Necessario.** A validità quasi zero nessuna attestazione sopravvive al tempo di firma e propagazione: nessun nodo può stabilire una sessione di trasporto, liveness failure di rete. Il pavimento dipende dal tempo di firma della chiave di identità e dalla latenza di consegna dell'attestazione, non dalla finestra di esposizione che il tetto governa.

---

### 4. `max_transport_attestation_future_skew_ms` ($S_{\max}$)

- **1. Cosa governa:**
  La tolleranza temporale in avanti concessa al timestamp `created_at_ms` di una `TransportKeyAttestation` rispetto all'orologio locale del ricevente (`docs/protocol/README.md:812`, `docs/protocol/identity.md:503`, `568`, `576`).
- **2. Cosa ottiene un quorum sedente ai two estremi:**
  - *Portato al massimo:* Aumenta artificialmente la finestra di validità complessiva dell'attestazione ($D_{\max} + S_{\max}$), consentendo l'accettazione di attestazioni postdatate e vanificando il principio di rotazione tempestiva.
  - *Portato al minimo (es. 0):* Qualsiasi nodo il cui orologio sia anche di pochi millisecondi avanti rispetto al peer vedrà le proprie attestazioni respinte al passo 5 del controllo (`identity.md:503`), rendendo impossibile stabilire sessioni di trasporto QUIC/Noise.
- **3. Cosa già lo vincola per altra via:**
  È sommato direttamente a `max_transport_attestation_validity_ms` nella definizione della finestra di esposizione (`identity.md:576`).
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà è assorbire esclusivamente le asimmetrie di orologio locali tra nodi senza diventare un canale per estendere la durata dell'attestazione.
  - **Tipo di vincolo naturale:** **Relazionale + Magnitudine.** $S_{\max}$ deve essere proporzionato e limitato superiormente da `max_clock_drift_ms` (o coincidere con esso), **più** un tetto massimo di magnitudine in genesi indipendente — il corpo del paragrafo porta già entrambi i termini; è la riga di tabella che, prima di questa correzione, ne perdeva uno (RF-008). Nota anche: l'ancora relazionale `max_clock_drift_ms` è essa stessa senza tetto oggi (§1, RF-002), quindi finché quel tetto manca la relazione da sola non basta a chiudere $S_{\max}$.
- **5. Pavimento e liveness:** **Necessario.** A skew zero, come nota il punto 2 al minimo, qualunque nodo con l'orologio anche di pochi millisecondi avanti vede le proprie attestazioni respinte al passo 5: impossibile stabilire sessioni di trasporto. Il pavimento dipende dalla stessa tolleranza di jitter/NTP di `max_clock_drift_ms`.

---

### 5. `replay_cache_entries_per_peer`

> **Corretto dopo [REVIEW-038] (RF-006).** I §5 e §6 proponevano due relazioni diverse per la stessa coppia (`per_peer <= global` qui, `global >= k × per_peer` sotto). La prima è vacua: ammette `per_peer == global`. Resta **una relazione sola**, con il numero di peer nominato esplicitamente, riportata sotto in entrambi i paragrafi. Il danno all'estremo massimo era anche sbagliato: non è consumo di memoria, è un DoS incrociato.

- **1. Cosa governa:**
  Il numero massimo di identificatori di messaggio (`message_id`) o coppie `(sender_node_id, nonce)` tracciate nella cache anti-replay per singolo peer connesso (`docs/protocol/README.md:813`, `docs/protocol/wire.md:109`, `443`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* **DoS incrociato, non consumo di memoria.** `wire.md:110` impone che, al superamento della soglia, i nuovi messaggi vengano rifiutati come `rate_limited` **senza evincere voci ancora vive**. Un attaccante con una sola identità enrollata emette `replay_cache_entries_per_peer` buste con validità massima verso un nodo bersaglio; quelle voci non sono evincibili per regola per l'intera finestra di validità. Se `per_peer` è vicino a `global`, il monopolio di un singolo peer sulla cache globale fa sì che **ogni altra busta, di qualunque peer onesto**, sia rifiutata come `rate_limited`. Non serve banda sostenuta né esaurimento di memoria: basta la regola di non-evizione.
  - *Portato al minimo (es. 0 o valori trascurabili):* Provoca il rate-limiting immediato (`rate_limited`) di tutto il traffico legittimo proveniente da qualsiasi peer non appena vengono inviati più di pochissimi messaggi all'interno della finestra di validità, bloccando di fatto la comunicazione P2P.
- **3. Cosa già lo vincola per altra via:**
  `wire.md:110` impone che al superamento del limite i nuovi messaggi vengano rifiutati come `rate_limited` e **non** provochino l'evizione anticipata di voci ancora vive — è la stessa regola che rende possibile lo scenario del punto 2.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà dipende dal throughput massimo ammissibile di messaggi per peer durante la finestra `max_envelope_validity_ms`, ponderato sulla disponibilità di memoria allocabile per canale.
  - **Tipo di vincolo naturale:** **Pavimento di magnitudine + relazione unica con `replay_cache_entries_global` e il numero di peer target ($N_{\text{peers}}$):** $N_{\text{peers}} \times \text{replay\_cache\_entries\_per\_peer} \le \text{replay\_cache\_entries\_global}$. Questa è la stessa relazione del §6, scritta una sola volta; sostituisce sia `per_peer <= global` (vacua) sia `global >= k × per_peer` (stesso vincolo ma senza nominare $k = N_{\text{peers}}$).
- **5. Pavimento e liveness:** **Necessario.** A `1` voce (il valore oggi in albero), la seconda busta di **chiunque** — peer onesto incluso — è `rate_limited`: è lo scenario RF-007 letto sulla riga 5. Il pavimento dipende dal throughput minimo di traffico legittimo per peer nella finestra `max_envelope_validity_ms`.

---

### 6. `replay_cache_entries_global`

- **1. Cosa governa:**
  Il limite massimo globale di identificatori tracciati simultaneamente nella cache anti-replay dell'intero nodo attraverso tutte le connessioni (`docs/protocol/README.md:814`, `docs/protocol/wire.md:109`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Consumo incontrollato di memoria globale dell'applicazione nodo.
  - *Portato al minimo:* Saturazione globale precoce della cache che porta al rifiuto generalizzato di qualunque nuovo messaggio sulla rete (`rate_limited`), creando un DoS sistemico dell'intero livello wire. È anche l'estremo in cui il DoS incrociato del §5 diventa più facile: basta una cache globale piccola perché un solo peer la saturi.
- **3. Cosa già lo vincola per altra via:**
  Nessun vincolo on-chain.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - Dipende dal numero target di connessioni peer simultanee ($N_{\text{peers}}$) moltiplicato per la capacità per-peer richiesta durante la finestra temporale delle buste.
  - **Tipo di vincolo naturale:** **Pavimento di magnitudine + relazione unica con `replay_cache_entries_per_peer`**, la stessa del §5: $\text{replay\_cache\_entries\_global} \ge N_{\text{peers}} \times \text{replay\_cache\_entries\_per\_peer}$.
- **5. Pavimento e liveness:** Riga utilizzabile così com'è (nessun rilievo di [REVIEW-038] la tocca). Un pavimento resta comunque necessario per la stessa ragione del §5, ponderata su $N_{\text{peers}}$ connessioni simultanee anziché su una sola.

---

### 7. `max_weak_subjectivity_age_ms`

> **Corretto dopo [REVIEW-038] (RF-005).** Il danno all'estremo massimo era quello sbagliato — non è un attacco long-range, che richiede la chiave di distribuzione che nessun quorum controlla — ed è un fail-closed di flotta. Il MUST relazionale citato al punto 3 non è oggi imposto all'accettazione: renderlo tale è la modifica che l'ADR proporrà, non lo stato attuale.

- **1. Cosa governa:**
  L'età massima ammissibile per un checkpoint di soggettività debole affinché un light client o un nodo in bootstrap lo consideri una valida ancora di fiducia (`docs/protocol/README.md:815`, `1599-1606`, `docs/protocol/ledger.md:1073-1081`, `2673`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* **Non un attacco long-range: un'arma di liveness di flotta.** Il valore operativo non è mai uno appreso da un peer — è quello **dentro il checkpoint firmato**, e `README.md:2661` impone il fail-closed obbligatorio sul disaccordo con `now - issued_at_ms`. Un long-range attack richiederebbe la chiave di distribuzione della release, che nessun quorum di consenso controlla. Ciò che il quorum compra alzando il parametro è più semplice e più pericoloso: `ledger.md:1054` dice che l'esposizione a una revoca finalizzata ma non ancora vista dal checkpoint è "al più `max_weak_subjectivity_age_ms`" — un valore alto allarga quella finestra e, in combinazione col fail-closed obbligatorio del light client, fa sì che un disaccordo (anche innocuo, un checkpoint non ancora aggiornato) faccia fallire chiuso **ogni light client conforme** in possesso di un header autenticato. DoS di flotta, un solo documento, costo zero per il quorum.
  - *Portato al minimo:* Light client che non si connettono quotidianamente trovano il checkpoint scaduto e falliscono chiusi (`fail closed`), costringendo a un onere insostenibile di rilascio continuo di aggiornamenti client out-of-band.
- **3. Cosa già lo vincola per altra via:**
  - **MUST relazionale citato, ma non imposto all'accettazione.** `ledger.md:1058-1061` dice che *"Governance MUST [...] choose `max_weak_subjectivity_age_ms` no greater than the expected wall-clock duration of `min_revocation_effective_delay_blocks`"*. Il predicato non è calcolabile su un documento firmato — "durata wall-clock attesa" non è un campo — e infatti **non compare nel blocco dei vincoli di `docs/protocol/ledger.md#rotation-the-cap-and-the-floor` né in `core/coblox-core/src/params.rs`**: verificato che `params.rs` controlla `min_revocation_effective_delay_blocks` (righe 526-590) ma nessuna relazione lì tocca `max_weak_subjectivity_age_ms`. Presentarlo come vincolo già attivo sopravvaluta la copertura odierna; imporlo all'accettazione è la proposta che l'ADR porterà, non lo stato presente.
  - **Canale out-of-band:** `README.md:1599-1606` risolve la circolarità includendo il parametro nel checkpoint firmato dalla release; il client verifica l'accordo esatto tra checkpoint e catena e fallisce chiuso in caso di divergenza. È questo canale, non il MUST relazionale, a essere oggi effettivo.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà fondamentale è che nessun client accetti un'ancora di fiducia più vecchia del tempo concesso per rendere effettiva una revoca di validatori.
  - **Tipo di vincolo naturale:** **STRETTAMENTE RELAZIONALE**. Dipende direttamente da $\text{min\_revocation\_effective\_delay\_blocks} \times \text{block\_interval\_ms}$. Un tetto isolato e disaccoppiato violerebbe [DEBT-036] (famiglia 3). Perché la relazione operi davvero serve però che l'ADR la renda un predicato accettato all'ingresso, non solo una prosa MUST.
- **5. Pavimento e liveness:** **Necessario.** A età zero, come nota il punto 2 al minimo, ogni checkpoint è istantaneamente scaduto e ogni light client fallisce chiuso al bootstrap: liveness failure totale per i client leggeri. Il pavimento dipende dalla cadenza con cui la release ripubblica checkpoint firmati, non da una proprietà di rete.

---

### 8. `max_current_balance_age_ms`

> **Integrato dopo [REVIEW-038] (RF-011), non bloccante — riga di tabella utilizzabile così com'era.** Mancava un danno e un credito: alzare il parametro non acceca solo la freschezza del saldo ma anche l'allarme di fork, e la non-regressione limita già in parte il danno.

- **1. Cosa governa:**
  La massima anzianità temporale ammissibile del blocco di stato (tip) fornito come prova Merkle autenticata in risposta a una richiesta di saldo corrente (`docs/protocol/README.md:816`, `docs/protocol/ledger.md:2798`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Consente a server RPC o validatori disonesti di fornire prove di saldo obsolete (stale balance proofs), nascondendo all'utente spese, trasferimenti o burn avvenuti di recente. **Danno ulteriore, non solo sul saldo:** il passo 6 dell'algoritmo di verifica (`ledger.md:2785-2788`) lega la stessa soglia all'accordo tra peer indipendenti, che è descritto letteralmente come *"an availability/fork alarm"*. Alzare `max_current_balance_age_ms` non acceca solo la freschezza del saldo mostrato: acceca anche quell'allarme, perché tip più vecchi restano nella banda accettata più a lungo prima che un disaccordo fra peer venga segnalato.
  - *Portato al minimo:* In presenza di ordinarie fluttuazioni di latenza di rete o brevi ritardi di finalizzazione, le interrogazioni di saldo falliscono sistematicamente per timeout di freschezza.
- **3. Cosa già lo vincola per altra via:**
  Nessun vincolo in genesi. È un controllo prescritto dal passo 6 dell'algoritmo di verifica light client (`ledger.md:2798`). **Credito parziale:** il passo 7 (`ledger.md:2789`) richiede che l'altezza della risposta uguagli esattamente l'altezza richiesta e non scenda mai sotto la fiducia già persistita — una forma di non-regressione che limita in parte il danno anche a soglia larga, perché un server malevolo non può comunque far *retrocedere* silenziosamente il client sotto uno stato già osservato.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - Dipende dal ritmo di finalizzazione dei blocchi e dalla latenza accettabile per l'interazione utente (es. multiplo di `block_interval_ms`).
  - **Tipo di vincolo naturale:** **Magnitudine (Tetto massimo in genesi)** parametrato sull'ordine di pochi secondi o minuti.
- **5. Pavimento e liveness:** **Necessario.** A `1 ms` (il valore oggi in albero) nessuna query di saldo riesce mai, come nota RF-007. Il pavimento dipende dal ritmo di finalizzazione (`block_interval_ms`) e dalla latenza di rete ordinaria verso i peer interrogati.

---

### 9. `app_suspension_notice_epochs`

> **Riclassificato dopo [REVIEW-038] (RF-001), la riga peggiore della tabella prima di questa correzione.** L'unità non è un'epoca autonoma: è l'epoca di **fatturazione**, la cui durata è `billing_epoch_ms`, campo di un documento diverso (`HostingRateCardBody`) firmato dallo stesso quorum sedente. Una banda a due lati in "epoche" senza dire di quale ancora non vincola nulla se il quorum può cambiare quanto dura l'epoca. Corretto anche il punto 3 (RF-010): `reward_epoch_ms` non governa la fatturazione delle app e nominarlo accanto a `billing_epoch_ms` sfocava esattamente la dipendenza che questa riclassificazione rende esplicita.

- **1. Cosa governa:**
  Il periodo di preavviso in epoche di fatturazione tra la transizione di un'applicazione allo stato di `grace` (per mancato pagamento dei costi di hosting/abbonamento) e la sua effettiva sospensione operativa (`docs/protocol/README.md:817`, `docs/protocol/ledger.md:590-593`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Un'applicazione inadempiente continua a occupare risorse di hosting ed esecuzione senza corrispondere il dovuto per un numero arbitrariamente alto di epoche di fatturazione (abuso di risorse / free-riding) — **ma la durata di quel numero di epoche non è fissa** (vedi punto 3).
  - *Portato al minimo (es. 0):* Sospensione immediata alla prima scadenza senza alcun margine di rimedio (grace period) per lo sviluppatore che dovesse subire ritardi nella transazione di ricarica `fund_app`.
- **3. Cosa già lo vincola per altra via:**
  Nessun vincolo di genesi. Il parametro opera sul ciclo di fatturazione definito da `billing_epoch_ms`, campo di `HostingRateCardBody` — un documento di protocollo diverso, ma firmato dallo **stesso quorum sedente** che firma `consensus_parameters`. `billing_epoch_ms` ha **due sole occorrenze in tutto `docs/protocol/`** (`README.md:805`, la riga di schema, e `README.md:1637`, la voce nella lista DRAFT): nessun limite di genesi, nessuna regola di validità, nessun MUST relazionale lo vincola oggi.

  **Perché questo rompe la banda in epoche.** Uno scenario a due passi, entrambi accettati da ogni validatore conforme: (1) il quorum pubblica `consensus_parameters` con `app_suspension_notice_epochs` dentro qualunque banda l'ADR fisserà; (2) lo stesso quorum pubblica `hosting_rate_card` che porta `billing_epoch_ms` da un giorno a pochi secondi. La finestra di rimedio reale — il tempo wall-clock che uno sviluppatore ha per un `fund_app` di recupero — collassa da giorni a secondi **senza violare la banda**, perché la banda è scritta in un'unità la cui durata il quorum controlla per un canale diverso.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà è consentire a sviluppatori onesti una finestra di rimedio congrua **in tempo wall-clock**, non in un conteggio di epoche la cui durata può cambiare sotto lo stesso quorum.
  - **Tipo di vincolo naturale:** **Banda a due lati relazionale con `billing_epoch_ms`** — il pavimento e il tetto vanno espressi sul prodotto $\text{app\_suspension\_notice\_epochs} \times \text{billing\_epoch\_ms}$ (una finestra in millisecondi), oppure la banda in epoche va accompagnata da un tetto di genesi su `billing_epoch_ms` che oggi non esiste. Finché `billing_epoch_ms` resta senza limite, qualunque banda scritta solo in epoche è famiglia 3: vincola la grandezza nominata invece di quella da cui dipende la proprietà.
- **5. Pavimento e liveness:** Non necessario alla liveness del protocollo di consenso in senso stretto — a `0` un'app perde la finestra di rimedio, ma la catena continua a finalizzare. È necessario alla liveness **applicativa** (la disponibilità continuata delle app oneste), che è la proprietà che questo parametro esiste per proteggere.

---

### 10. `min_revocation_effective_delay_blocks` ($F$)

- **1. Cosa governa:**
  Il ritardo minimo in blocchi tra l'inclusione di una transazione di revoca identità (`revoke_identity`) e la sua altezza di efficacia (`effective_height`) sul percorso di transizione del set di validatori (`docs/protocol/README.md:818`, `docs/protocol/ledger.md:169`, `837`, `1033`, `1075`, `1090`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Mantiene all'infinito un validatore compromesso o sanzionato all'interno del set attivo; costringe ad aumentare in parallelo `max_weak_subjectivity_age_ms`, allargando la finestra di vulnerabilità per i light client.
  - *Portato al minimo (es. 0 o 1 blocco):* Se il ritardo è inferiore al tempo necessario ai validatori superstiti per coordinarsi ed emettere un set successore conforme, la catena incorre in uno **stallo irreversibile** (`ledger.md:1088`).
- **3. Cosa già lo vincola per altra via:**
  - [ADR-017] separa i due percorsi della revoca: sul percorso di spesa la revoca morde all'inclusione ($h$), mentre sul percorso del set $F$ costituisce il pavimento della banda vincolata da `revocation_effective_grace_blocks` ($G$) e `max_planned_revocation_delay_blocks` ($P$).
  - `ledger.md:1079` lo vincola per MUST a `max_weak_subjectivity_age_ms`.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà cercata è duplice: garantire la liveness del set di validatori (tempo di coordinamento per set successore) senza concedere all'avversario una finestra di latenza arbitraria.
  - **Tipo di vincolo naturale:** **Banda a due lati + Vincolo relazionale bidirezionale** (pavimento di liveness $F \ge 1$, tetto di sicurezza di genesi $F \le F_{\max}$, e legame relazionale con $G$, $P$ e `max_weak_subjectivity_age_ms`).
- **5. Pavimento e liveness:** **Necessario, e già imposto oggi** — a differenza degli altri nove, questa è la sola riga dove il pavimento è già vincolo di genesi verificato: `core/coblox-core/src/params.rs:526-527` impone `min_revocation_effective_delay_blocks >= 1`. Il pavimento dipende dal tempo minimo che i validatori superstiti richiedono per coordinarsi e committare un set successore conforme; a `F = 0` la catena rischia lo stallo irreversibile che `ledger.md:1088` nomina.

---

## 3. Quadro di sintesi tassonomica

La seguente tabella riassume per ciascuno dei dieci parametri la natura del vincolo ottimale emersa dall'analisi, corretta contro [REVIEW-038]. La colonna **Pavimento necessario** risponde a RF-007: con i valori oggi in albero (fixture di test, tutti a `1`, senza alcuna regola v0 che impedisca a una distribuzione di adottarli in genesi) otto dei dieci parametri falliscono al pavimento e nessuno al tetto.

| # | Parametro | Ambito di governo | Rischio estremo massimo | Rischio estremo minimo | Vincolo naturale | Pavimento necessario |
|---|---|---|---|---|---|---|
| 1 | `max_clock_drift_ms` | Orologi di rete / beacon di elezione | **Finestra di macinatura sul beacon** (10³–10⁶ candidati a una SHA-256 l'uno) | Partizionamento / liveness failure | **Magnitudine assoluta** tarata sul costo di macinatura, asimmetrica (avanti: deriva; indietro: mediana degli undici) | Sì — dipende dal jitter NTP e dalla latenza di rete |
| 2 | `max_envelope_validity_ms` | Buste wire | Saturazione cache anti-replay | Scarto messaggi in transito gossip | **Magnitudine** (tetto massimo di genesi) | Sì — dipende dal diametro P2P e dalla latenza GossipSub |
| 3 | `max_transport_attestation_validity_ms` | Attestazioni trasporto | Esposizione chiave di sessione (TM-37), finestra a **tre** termini ($D_{\max}+S_{\max}$+età checkpoint) | Sovraccarico firma su chiave di identità | **Magnitudine su $D_{\max}+S_{\max}$ + relazionale con l'età del checkpoint** (terzo termine oggi non limitato) | Sì — dipende dal tempo di firma e consegna dell'attestazione |
| 4 | `max_transport_attestation_future_skew_ms` | Tolleranza skew trasporto | Allungamento finestra accettazione | Rifiuto connessioni per drift minimo | **Relazionale (`max_clock_drift_ms`) + Magnitudine** (tetto indipendente; l'ancora relazionale è essa stessa senza tetto oggi) | Sì — stessa tolleranza jitter/NTP della riga 1 |
| 5 | `replay_cache_entries_per_peer` | Cache anti-replay per-peer | **DoS incrociato**: un peer monopolizza la cache globale, ogni altro riceve `rate_limited` (la regola di rifiuto non evince voci vive) | DoS da `rate_limited` su peer leciti | **Pavimento + relazione unica** $N_{\text{peers}} \times \text{per\_peer} \le \text{global}$ | Sì — a `1` voce, la seconda busta di chiunque è `rate_limited` |
| 6 | `replay_cache_entries_global` | Cache anti-replay globale | Consumo memoria RAM totale | DoS sistemico da blocco messaggi | **Pavimento + relazione unica** (stessa di riga 5) | Sì — ponderato su $N_{\text{peers}}$ |
| 7 | `max_weak_subjectivity_age_ms` | Checkpoint light client | **Fail-closed di flotta** (non long-range attack: quello richiede la chiave di distribuzione) | Fallimento chiuso sistematico client | **Strettamente relazionale** con $F \times \text{block\_interval}$ — il MUST è citato ma non imposto all'accettazione oggi | Sì — a età zero ogni light client fallisce chiuso al bootstrap |
| 8 | `max_current_balance_age_ms` | Prova Merkle saldo | Prova di stato obsoleta **+ allarme di fork accecato** | Rifiuto query saldo per asincronia | **Magnitudine** (tetto massimo di genesi); non-regressione già limita in parte il danno | Sì — a `1 ms` nessuna query di saldo riesce |
| 9 | `app_suspension_notice_epochs` | Preavviso sospensione app | Free-riding risorse hosting, **ma la finestra reale collassa se il quorum accorcia `billing_epoch_ms`** (senza limite di genesi) | Sospensione immediata senza rimedio | **Banda a due lati relazionale con `billing_epoch_ms`** (o banda sul prodotto epoche×durata) | No alla liveness di consenso; sì alla liveness applicativa |
| 10 | `min_revocation_effective_delay_blocks` | Ritardo revoca set | Esposizione e stallo checkpoint | Stallo della catena per mancato set | **Banda a due lati + Relazionale** (con $G$, $P$ e WS) | Sì, e **già imposto**: `params.rs:526-527` verifica $F \ge 1$ |

---

## 4. Fonti consultate

Per la redazione di questa analisi sono stati esaminati integralmente:
1. `docs/protocol/README.md` (in particolare le righe 781–833 per gli schemi di tutti e quattro i documenti firmati e il conteggio corrente dei campi di `ConsensusParametersBody`, 1599–1606 per la risoluzione della circolarità del checkpoint, e 1658–1680 per la sezione DRAFT dei dieci parametri operativi);
2. `docs/protocol/identity.md` (in particolare le righe 490–589 per le regole di validazione delle attestazioni di trasporto, la finestra a tre termini del §"Bounded validity in time", e lo scenario TM-37);
3. `docs/protocol/wire.md` (in particolare le righe 95–130 per la struttura delle buste wire, `max_envelope_validity_ms`, la regola di non-evizione della cache anti-replay, e 440–445 per la sezione sul replay a livello di sicurezza del trasporto);
4. `docs/protocol/ledger.md` (in particolare le righe 690–725 per la finestra di macinatura sul beacon di elezione quantificata sotto la grafia estesa "the maximum clock drift", 800–811 per il secondo sito normativo della stessa grafia e l'asimmetria della tolleranza, 589–595 per il preavviso di sospensione app e la sua ancora `billing_epoch_ms`, 1020–1110 per la meccanica del ritardo di revoca e weak subjectivity, e 2780–2806 per il light client, l'allarme di fork e la freschezza del saldo);
5. `.lmbrain/decisions/ADR-010`, `ADR-013`, `ADR-015`, `ADR-016`, `ADR-017`;
6. `.lmbrain/debts/open/DEBT-036-dieci-parametri-di-consenso-su-venti-non-sono-ne-limitati-in-genesi-ne-dichiarati-aperti.md`;
7. `core/coblox-core/src/params.rs` (per la validazione a livello di codice delle strutture `ElectionBounds`, `RewardBounds`, `CadenceBand` e `ConsensusParameters`).
