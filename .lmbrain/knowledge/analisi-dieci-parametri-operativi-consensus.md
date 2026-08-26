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

`ConsensusParametersBody` definisce **venti parametri**. I dieci parametri di elezione e rotazione (da `election_epoch_blocks` a `validator_min_capture_epochs`) godono di due livelli di tutela:
1. Una presenza esplicita nella lista DRAFT dei parametri aperti prima del mainnet;
2. Un insieme di vincoli relazionali e di magnitudine in `docs/protocol/ledger.md#rotation-the-cap-and-the-floor` ancorati all'oggetto di genesi `ElectionBounds`.

Gli altri dieci parametri sono la metà **operativa e di sicurezza della rete**: orologi, finestre temporali delle buste, tolleranze di skew, cache anti-replay, freschezza dell'ancora di fiducia e ritardi di revoca.

Per ciascuno dei dieci parametri, questa analisi risponde a **quattro domande fondamentali**:
1. **Cosa governa**, in una frase, con le righe esatte del documento di protocollo che lo stabiliscono.
2. **Cosa ottiene un quorum sedente che lo porta al massimo**, e cosa ottiene portandolo al minimo (gli estremi non sono simmetrici, come dimostrato da [ADR-013] e [ADR-016]).
3. **Cosa già lo vincola per altra via**: un altro parametro legato da un MUST, un secondo canale (come il checkpoint di distribuzione), o nulla.
4. **Da quale grandezza dipende la proprietà che vorremmo**: se dalla grandezza stessa (vincolo di **magnitudine** / tetto assoluto) o dalla relazione tra grandezze diverse (vincolo **relazionale**). Questa distinzione è critica per non cadere nella *famiglia 3* dei difetti ricorrenti (vincolare la grandezza nominata invece di quella da cui dipende la proprietà).

---

## 2. Analisi dei dieci parametri

---

### 1. `max_clock_drift_ms`

- **1. Cosa governa:**
  La massima discrepanza temporale tollerabile tra l'orologio locale del nodo ricevente e i timestamp registrati su blocchi, messaggi o attestazioni (`docs/protocol/README.md:809`, `docs/protocol/identity.md:503`, `docs/protocol/README.md:1624`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Un quorum bizantino può produrre blocchi con timestamp proiettati arbitrariamente nel futuro, forzando i nodi onesti ad accettare altezze con orologi disallineati, falsando le misure di cadenza e allargando le finestre di validità temporale dipendenti dall'orologio di blocco.
  - *Portato al minimo (es. 0 o pochi ms):* Causa un arresto operativo diffuso (liveness failure): qualsiasi fisiologico jitter NTP o normale latenza di propagazione geografica induce i nodi a scartare i blocchi e i messaggi validi emessi da peer onesti.
- **3. Cosa già lo vincola per altra via:**
  Nessuna regola on-chain. All'esterno della catena, l'ancora di fiducia di genesi definisce `CadenceBand.max_external_clock_slack_ms` ([ADR-016]), che limita la tolleranza usata dai light client durante la verifica del ritmo di emissione, ma non vincola il parametro firmato dentro `ConsensusParametersBody`.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà desiderata è che la deriva tollerata sia sufficiente ad assorbire il jitter di rete e le tolleranze ordinarie dei demoni NTP (decine/centinaia di millisecondi), rimanendo strettamente inferiore a una frazione dell'intervallo di blocco (`block_interval_ms = 5 000 ms`).
  - **Tipo di vincolo naturale:** **Ibrido (Relazionale + Magnitudine)**. Deve essere vincolato relazionalmente rispetto a `block_interval_ms` (es. $\text{max\_clock\_drift\_ms} < \frac{1}{2} \text{block\_interval\_ms}$) e limitato da un tetto assoluto in genesi $\text{max\_clock\_drift\_ms\_max}$.

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

---

### 3. `max_transport_attestation_validity_ms` ($D_{\max}$)

- **1. Cosa governa:**
  La durata massima di validità dell'attestazione della chiave di trasporto effimera (`TransportKeyAttestation`) firmata dalla chiave di identità permanente (`docs/protocol/README.md:811`, `docs/protocol/identity.md:500`, `556`, `576`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Dilata la finestra temporale in cui una chiave di trasporto sottratta o compromessa può essere usata per impersonare il nodo nelle connessioni P2P dirette (scenario TM-37 nel threat model) senza alcuna possibilità di revoca on-chain anticipata.
  - *Portato al minimo:* Costringe tutti i nodi della rete a invocare continuamente la chiave di identità principale per firmare nuove attestazioni effimere a intervalli brevissimi, aumentando drasticamente il rischio operativo e il carico crittografico sulle chiavi fredde.
- **3. Cosa già lo vincola per altra via:**
  `identity.md:576` formalizza che la finestra totale di accettazione da parte dei peer è la somma:
  $$\text{accepted\_window} = \text{max\_transport\_attestation\_validity\_ms} + \text{max\_transport\_attestation\_future\_skew\_ms}$$
  Nessun vincolo di genesi fissa un limite massimo a questa somma.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà cercata è il contenimento della finestra di esposizione per compromissione di chiave effimera.
  - **Tipo di vincolo naturale:** **Magnitudine (Tetto massimo in genesi sulla somma $D_{\max} + S_{\max}$)**. Un tetto assoluto sulla durata massima garantisce che la rotazione avvenga entro una finestra temporale massima prefissata (es. 24–72 ore).

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
  - **Tipo di vincolo naturale:** **Relazionale (legato a `max_clock_drift_ms`)**. $S_{\max}$ deve essere proporzionato e limitato superiormente da `max_clock_drift_ms` (o coincidere con esso), con un tetto massimo di magnitudine in genesi.

---

### 5. `replay_cache_entries_per_peer`

- **1. Cosa governa:**
  Il numero massimo di identificatori di messaggio (`message_id`) o coppie `(sender_node_id, nonce)` tracciate nella cache anti-replay per singolo peer connesso (`docs/protocol/README.md:813`, `docs/protocol/wire.md:109`, `443`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Incrementa il consumo di memoria RAM dedicato a ciascuna connessione peer, esponendo i nodi con risorse contenute (es. dispositivi mobili Android o nodi edge) a crash per esaurimento memoria (OOM DoS).
  - *Portato al minimo (es. 0 o valori trascurabili):* Provoca il rate-limiting immediato (`rate_limited`) di tutto il traffico legittimo proveniente da qualsiasi peer non appena vengono inviati più di pochissimi messaggi all'interno della finestra di validità, bloccando di fatto la comunicazione P2P.
- **3. Cosa già lo vincola per altra via:**
  `wire.md:110` impone che al superamento del limite i nuovi messaggi vengano rifiutati come `rate_limited` e **non** provochino l'evizione anticipata di voci ancora vive.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà dipende dal throughput massimo ammissibile di messaggi per peer durante la finestra `max_envelope_validity_ms`, ponderato sulla disponibilità di memoria allocabile per canale.
  - **Tipo di vincolo naturale:** **Pavimento di magnitudine + Relazionale con `replay_cache_entries_global`**. Deve esistere un pavimento minimo per impedire il blocco del traffico lecito e una relazione d'ordine $\text{replay\_cache\_entries\_per\_peer} \le \text{replay\_cache\_entries\_global}$.

---

### 6. `replay_cache_entries_global`

- **1. Cosa governa:**
  Il limite massimo globale di identificatori tracciati simultaneamente nella cache anti-replay dell'intero nodo attraverso tutte le connessioni (`docs/protocol/README.md:814`, `docs/protocol/wire.md:109`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Consumo incontrollato di memoria globale dell'applicazione nodo.
  - *Portato al minimo:* Saturazione globale precoce della cache che porta al rifiuto generalizzato di qualunque nuovo messaggio sulla rete (`rate_limited`), creando un DoS sistemico dell'intero livello wire.
- **3. Cosa già lo vincola per altra via:**
  Nessun vincolo on-chain.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - Dipende dal numero target di connessioni peer simultanee ($N_{\text{peers}}$) moltiplicato per la capacità per-peer richiesta durante la finestra temporale delle buste.
  - **Tipo di vincolo naturale:** **Pavimento di magnitudine + Relazionale** ($\text{replay\_cache\_entries\_global} \ge k \times \text{replay\_cache\_entries\_per\_peer}$).

---

### 7. `max_weak_subjectivity_age_ms`

- **1. Cosa governa:**
  L'età massima ammissibile per un checkpoint di soggettività debole affinché un light client o un nodo in bootstrap lo consideri una valida ancora di fiducia (`docs/protocol/README.md:815`, `1599-1606`, `docs/protocol/ledger.md:1073-1081`, `2673`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Consente l'accettazione di checkpoint arbitrariamente vecchi. Se i validatori dell'epoca passata sono stati nel frattempo revocati o hanno dismesso le chiavi, essi possono firmare una catena alternativa conflittuale (long-range attack) ingannando i client che si sincronizzano dopo lungo tempo.
  - *Portato al minimo:* Light client che non si connettono quotidianamente trovano il checkpoint scaduto e falliscono chiusi (`fail closed`), costringendo a un onere insostenibile di rilascio continuo di aggiornamenti client out-of-band.
- **3. Cosa già lo vincola per altra via:**
  - **MUST relazionale esplicito:** `ledger.md:1077-1081` stabilisce che `max_weak_subjectivity_age_ms` **MUST** essere non superiore alla durata stimata wall-clock di `min_revocation_effective_delay_blocks`.
  - **Canale out-of-band:** `README.md:1599-1606` risolve la circolarità includendo il parametro nel checkpoint firmato dalla release; il client verifica l'accordo esatto tra checkpoint e catena e fallisce chiuso in caso di divergenza.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà fondamentale è che nessun client accetti un'ancora di fiducia più vecchia del tempo concesso per rendere effettiva una revoca di validatori.
  - **Tipo di vincolo naturale:** **STRETTAMENTE RELAZIONALE**. Dipende direttamente da $\text{min\_revocation\_effective\_delay\_blocks} \times \text{block\_interval\_ms}$. Un tetto isolato e disaccoppiato violerebbe [DEBT-036] (famiglia 3).

---

### 8. `max_current_balance_age_ms`

- **1. Cosa governa:**
  La massima anzianità temporale ammissibile del blocco di stato (tip) fornito come prova Merkle autenticata in risposta a una richiesta di saldo corrente (`docs/protocol/README.md:816`, `docs/protocol/ledger.md:2798`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Consente a server RPC o validatori disonesti di fornire prove di saldo obsolete (stale balance proofs), nascondendo all'utente spese, trasferimenti o burn avvenuti di recente.
  - *Portato al minimo:* In presenza di ordinarie fluttuazioni di latenza di rete o brevi ritardi di finalizzazione, le interrogazioni di saldo falliscono sistematicamente per timeout di freschezza.
- **3. Cosa già lo vincola per altra via:**
  Nessun vincolo in genesi. È un controllo prescritto dal passo 6 dell'algoritmo di verifica light client (`ledger.md:2798`).
- **4. Da quale grandezza dipende la proprietà voluta:**
  - Dipende dal ritmo di finalizzazione dei blocchi e dalla latenza accettabile per l'interazione utente (es. multiplo di `block_interval_ms`).
  - **Tipo di vincolo naturale:** **Magnitudine (Tetto massimo in genesi)** parametrato sull'ordine di pochi secondi o minuti.

---

### 9. `app_suspension_notice_epochs`

- **1. Cosa governa:**
  Il periodo di preavviso in epoche tra la transizione di un'applicazione allo stato di `grace` (per mancato pagamento dei costi di hosting/abbonamento) e la sua effettiva sospensione operativa (`docs/protocol/README.md:817`, `docs/protocol/ledger.md:628`).
- **2. Cosa ottiene un quorum sedente ai due estremi:**
  - *Portato al massimo:* Un'applicazione inadempiente continua a occupare risorse di hosting ed esecuzione senza corrispondere il dovuto per un numero arbitrariamente alto di epoche (abuso di risorse / free-riding).
  - *Portato al minimo (es. 0):* Sospensione immediata alla prima scadenza senza alcun margine di rimedio (grace period) per lo sviluppatore che dovesse subire ritardi nella transazione di ricarica `fund_app`.
- **3. Cosa già lo vincola per altra via:**
  Nessun vincolo di genesi; opera sul ciclo di fatturazione definito da `billing_epoch_ms` e `reward_epoch_ms`.
- **4. Da quale grandezza dipende la proprietà voluta:**
  - La proprietà è consentire a sviluppatori onesti una finestra di rimedio congrua senza gravare indefinitamente sui costi dei provider di hosting.
  - **Tipo di vincolo naturale:** **Banda a due lati (Pavimento e Tetto di magnitudine in epoche)**: ad es. $1 \le \text{app\_suspension\_notice\_epochs} \le N$.

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

---

## 3. Quadro di sintesi tassonomica

La seguente tabella riassume per ciascuno dei dieci parametri la natura del vincolo ottimale emersa dall'analisi:

| Parametro | Ambito di governo | Rischio estremo massimo | Rischio estremo minimo | Vincolo naturale |
|---|---|---|---|---|
| `max_clock_drift_ms` | Orologi di rete | Deriva temporale / allargamento finestre | Partizionamento / liveness failure | **Relazionale + Magnitudine** (frazione di `block_interval_ms` con tetto max) |
| `max_envelope_validity_ms` | Buste wire | Saturazione cache anti-replay | Scarto messaggi in transito gossip | **Magnitudine** (tetto massimo di genesi) |
| `max_transport_attestation_validity_ms` | Attestazioni trasporto | Esposizione chiave di sessione (TM-37) | Sovraccarico firma su chiave di identità | **Magnitudine** (tetto su $D_{\max} + S_{\max}$) |
| `max_transport_attestation_future_skew_ms` | Tolleranza skew trasporto | Allungamento finestra accettazione | Rifiuto connessioni per drift minimo | **Relazionale** (limitato da `max_clock_drift_ms`) |
| `replay_cache_entries_per_peer` | Cache anti-replay per-peer | Consumo memoria RAM per canale | DoS da `rate_limited` su peer leciti | **Pavimento di magnitudine + Relazionale** |
| `replay_cache_entries_global` | Cache anti-replay globale | Consumo memoria RAM totale | DoS sistemico da blocco messaggi | **Pavimento di magnitudine + Relazionale** |
| `max_weak_subjectivity_age_ms` | Checkpoint light client | Attacco long-range fork | Fallimento chiuso sistematico client | **Strettamente Relazionale** (vincolato a $F \times \text{block\_interval}$) |
| `max_current_balance_age_ms` | Prova Merkle saldo | Prova di stato obsoleta | Rifiuto query saldo per asincronia | **Magnitudine** (tetto massimo di genesi) |
| `app_suspension_notice_epochs` | Preavviso sospensione app | Free-riding risorse hosting | Sospensione immediata senza rimedio | **Banda a due lati** (pavimento e tetto in epoche) |
| `min_revocation_effective_delay_blocks` | Ritardo revoca set | Esposizione e stallo checkpoint | Stallo della catena per mancato set | **Banda a due lati + Relazionale** (con $G$, $P$ e WS) |

---

## 4. Fonti consultate

Per la redazione di questa analisi sono stati esaminati integralmente:
1. `docs/protocol/README.md` (in particolare le righe 808–829 per lo schema `ConsensusParametersBody`, 1599–1606 per la risoluzione della circolarità del checkpoint, e 1608–1640 per la sezione DRAFT);
2. `docs/protocol/identity.md` (in particolare le righe 495–585 per le regole di validazione delle attestazioni di trasporto, la formula della finestra $D_{\max} + S_{\max}$ e lo scenario TM-37);
3. `docs/protocol/wire.md` (in particolare le righe 95–130 per la struttura delle buste wire, `max_envelope_validity_ms` e il funzionamento delle cache anti-replay globale e per-peer);
4. `docs/protocol/ledger.md` (in particolare le righe 620–640 per il preavviso di sospensione app, 1020–1110 per la meccanica del ritardo di revoca e weak subjectivity, 1980–2025 per il blocco dei vincoli di magnitudine e rotazione, e 2780–2820 per il light client e la freschezza del saldo);
5. `.lmbrain/decisions/ADR-010`, `ADR-013`, `ADR-015`, `ADR-016`, `ADR-017`;
6. `.lmbrain/debts/open/DEBT-036-dieci-parametri-di-consenso-su-venti-non-sono-ne-limitati-in-genesi-ne-dichiarati-aperti.md`;
7. `core/coblox-core/src/params.rs` (per la validazione a livello di codice delle strutture `ElectionBounds`, `RewardBounds`, `CadenceBand` e `ConsensusParameters`).
