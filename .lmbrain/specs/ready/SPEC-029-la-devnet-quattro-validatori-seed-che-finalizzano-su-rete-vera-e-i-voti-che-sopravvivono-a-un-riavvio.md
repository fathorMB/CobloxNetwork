---
id: SPEC-029
# Note: Quote the title if it contains a colon
title: "La devnet: quattro validatori seed che finalizzano su rete vera, e i voti che sopravvivono a un riavvio"
status: ready
kind: feature
priority: high
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
capability_tier: sol
thinking_level: maximum
effort_observations: []
depends_on: [SPEC-025]
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-001, ADR-003, ADR-012, ADR-018]
links: []
created: 2026-08-27
updated: 2026-08-27
tags: [rust, p2p, devnet]
activity:
  - date: 2026-08-27
    action: "created"
  - date: 2026-08-27
    action: "transitioned backlog -> ready"
---
# La devnet

## Objective

Chiudere il **primo esito nominato da M-02**: *«una devnet di validatori seed
raggiunge consenso BFT»*. Quattro processi separati, su rete vera, finalizzano
una catena con i certificati che il verificatore gia' spedito accetta — e la
finalizzano ancora dopo che uno di loro e' stato ucciso e riavviato.

Il motore esiste ed e' verificato ([SPEC-025]). Questa spec costruisce il
**chiamante** che il motore dichiara di non avere.

## Context

Oggi `core/coblox-node/src/main.rs` sono **ventuno righe** che stampano *«start
is not configured yet»*. Nel workspace non c'e' `libp2p`, non c'e' `tokio`, non
c'e' alcuna dipendenza di rete. Il consenso funziona su uno scheduler in memoria
con un orologio virtuale.

**Il motore e' senza I/O per costruzione**, e il suo contratto e' esplicito:
`Event` dentro, `Action` fuori. Il chiamante deve firmare e trasmettere
(`Action::Broadcast`), armare timer (`Action::ScheduleTimeout`), costruire un
blocco quando gli viene chiesto (`Action::RequestValue`), e consegnare al motore
i messaggi **gia' verificati** e i timeout scaduti. Nulla di tutto questo esiste.

[REVIEW-047] ha inoltre dichiarato del chiamante due cose che nessuno ha ancora
scritto: il **buffering fra altezze** e il predicato `valid(v)` oltre altezza e
genitore.

## Scope

### Included

- **Il ciclo di vita del nodo**: `coblox-node start` che carica una chiave, una
  configurazione e un elenco di pari seed, si connette, e partecipa.
- **Il trasporto**, nel sottoinsieme che una devnet usa davvero — vedi *Perimetro
  ristretto* sotto.
- **Il chiamante del motore**: la pompa `Event`/`Action`, la firma dei voti, i
  timer su orologio reale, la costruzione del blocco su `RequestValue`, il
  buffering dei messaggi di altezze future, e `valid(v)`.
- **La persistenza dei voti**, che e' un requisito di **sicurezza** e non una
  comodita' — vedi sotto.
- **La persistenza della catena**: i blocchi finalizzati sopravvivono al riavvio.
- **La taratura dei tre timeout**, oggi dichiarata non fatta.
- **Il recinto di [DEBT-029]**: `verify_consensus_ed25519` e' riesportata alla
  radice senza feature gate ne' guardia, e il primo chiamante di consenso ora
  esiste. E' **la prima cosa da fare qui**, ed e' l'ultima consegna in cui costa
  poco: dopo diventa una migrazione con chiamanti al seguito.
- Un **runbook** che avvii la devnet a quattro nodi su una macchina sola.

### Excluded, e la ragione va letta

**Il perimetro ristretto del trasporto.** `docs/protocol/wire.md` §*Network
stack* specifica lo stack WAN completo: QUIC-v1 preferito, TCP con Noise e Yamux
come fallback obbligatorio, Identify, Ping, Kademlia DHT, mDNS, AutoNAT v1,
Circuit Relay v2, DCUtR, GossipSub 1.1. **Una devnet di validatori seed non ne
ha bisogno**: i nodi si conoscono per configurazione, girano su una rete
raggiungibile, e non devono trovarsi.

Questa spec attua il **fallback obbligatorio** — TCP con Noise e Yamux — piu'
GossipSub 1.1 per la diffusione, e **dichiara escluso** il resto: QUIC,
Kademlia, mDNS, AutoNAT, Relay, DCUtR. L'esclusione e' una scelta di
proporzione, non una semplificazione silenziosa: `wire.md` resta la baseline di
interoperabilita' v0 e questa spec ne implementa un sottoinsieme dichiarato. Il
resto e' lavoro di M-04, quando i nodi devono stare dietro NAT reali.

Altre esclusioni:

- **Mint & burn e light client con prove Merkle**: sono gli altri due esiti di
  M-02 e hanno spec proprie.
- **Il livello compute** e qualunque cosa di M-06.
- **Modifiche al motore di consenso**, salvo cio' che [REVIEW-047] ha gia'
  chiesto in remediation. Se il chiamante rivelasse un difetto del motore,
  **fermarsi e riportarlo**: e' un rilievo su [SPEC-025], non lavoro di questa.

## Technical proposal

**La persistenza dei voti prima di tutto, perche' e' sicurezza.** Un validatore
che riparte e dimentica i propri precommit puo' **equivocare in buona fede**:
firma due volte lo stesso round senza essere malevolo, e l'intersezione dei
quorum su cui poggia l'intera sicurezza di [ADR-018] non regge piu'. Il motore e'
senza I/O per costruzione, quindi il write-ahead log e' del chiamante — cioe' di
questa spec, e di nessun'altra.

La regola: **un voto e' trasmesso solo dopo che e' stato reso durevole**, mai
prima. Al riavvio il nodo rilegge il proprio log e non firma nulla che
contraddica cio' che vi trova.

Il resto e' la pompa: un ciclo che consuma `Action` e produce `Event`, con la
rete e l'orologio ai due capi e il motore in mezzo, invariato.

## Acceptance criteria

- [ ] `coblox-node start` avvia un nodo che carica chiave, configurazione e pari
      seed, e si connette agli altri.
- [ ] **Quattro processi separati** su rete vera finalizzano una catena di almeno
      dieci blocchi, e i certificati sono accettati da `FinalizedBlock::verify`
      con il verificatore spedito. Non un test in memoria: quattro processi.
- [ ] **Il riavvio non produce equivocazione.** Un nodo e' ucciso mentre l'altezza
      e' in corso e riavviato; la catena prosegue e il nodo riavviato non firma
      nulla che contraddica il proprio log. **Il caso e' esercitato, non
      argomentato.**
- [ ] Un voto non e' mai trasmesso prima di essere durevole. Dimostrato da un
      test che uccide il processo fra la scrittura e la trasmissione e verifica
      che al riavvio il voto sia noto.
- [ ] I blocchi finalizzati sopravvivono al riavvio: il nodo riparte dall'altezza
      che aveva, non da genesi.
- [ ] **`FinalizedBlock::verify` ricalcola `transactions_root` dal carico
      portato** e rifiuta se non riproduce quello dell'header, con la stessa
      definizione di `tx_id` che il confine della proposta gia' usa. Chiude
      [DEBT-047]. Un test osserva il rifiuto su un blocco con **certificato
      genuino** e carico divergente.

      Non e' un'aggiunta di comodo: [SPEC-025] ha messo il legame al confine
      della **proposta**, e finche' non esistono rete e persistenza nessun
      `Block` arriva da altrove. Questa spec introduce **entrambi** i percorsi che
      lo portano — un blocco letto da disco al riavvio, uno ricevuto da un pari
      in sincronizzazione — quindi il buco diventa raggiungibile qui e in nessun
      posto prima.
- [ ] **Il buffering fra altezze** e `valid(v)` sono implementati e nominati:
      esiste un test in cui un messaggio di un'altezza futura arriva presto,
      viene trattenuto, e viene consumato quando l'altezza comincia.
- [ ] I tre timeout hanno valori **derivati da una grandezza dichiarata** e non
      scelti, oppure sono dichiarati parametri locali con la ragione. La
      trascrizione nomina la derivazione.
- [ ] Il recinto di [DEBT-029] e' posato: `verify_consensus_ed25519` non e' piu'
      raggiungibile senza contesto dalla radice del crate, e la trascrizione
      mostra un chiamante che prima compilava e ora no.
- [ ] Il trasporto e' TCP con Noise e Yamux piu' GossipSub 1.1, e `wire.md`
      dichiara **quale sottoinsieme** della propria baseline la devnet attua.
- [ ] Il runbook avvia quattro nodi su una macchina sola e la trascrizione mostra
      la catena crescere.
- [ ] Passata di [ADR-012] eseguita e trascritta.

## Verification gates

- [ ] GATE-FOUR-PROCESSES | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Quattro **processi separati**, non quattro motori in un test, finalizzano almeno dieci blocchi su rete vera. La trascrizione mostra i PID, gli indirizzi e le altezze.
- [ ] GATE-RESTART-NO-EQUIVOCATION | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Un nodo ucciso a meta' altezza e riavviato non firma nulla che contraddica il proprio log, e il caso e' **eseguito**. E' il criterio di sicurezza di questa spec: senza di esso la devnet e' una dimostrazione e non un nodo.
- [ ] GATE-DURABLE-BEFORE-SEND | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Osservato che un voto non lascia il processo prima di essere durevole, uccidendo il processo nella finestra e verificando al riavvio.
- [ ] GATE-NEGATIVE-PROOF | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Ogni regola nuova osservata fallire su un albero mutato, una mutazione per regola.
- [ ] GATE-ENGINE-UNCHANGED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il diff non modifica `core/coblox-core/src/consensus/`, salvo cio' che [REVIEW-047] ha chiesto. Verificabile guardando il diff.
- [ ] GATE-SUBSET-DECLARED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | `wire.md` dichiara quale sottoinsieme della propria baseline la devnet attua, e una probe di [ADR-012] la fissa. Un sottoinsieme non dichiarato e' una divergenza silenziosa.
- [ ] GATE-ADR012-PASS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Passata eseguita con lo strumento versionato, trascrizione allegata.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | Review di AGENT-007. **Non e' facoltativa**: questa spec porta le chiavi di firma, la persistenza e la rete nello stesso processo, ed e' la prima volta che un nodo Coblox parla con un altro.
- [ ] GATE-CI-GREEN | kind=manual | owner=lead | phase=before-done | evidence=transcript | Pipeline reale verde, con numero di run e commit.
- [ ] GATE-LEAD-REPRO | kind=manual | owner=lead | phase=before-done | evidence=transcript | Il Lead avvia la devnet dal runbook e uccide un nodo lui stesso, invece di prendere le trascrizioni dall'evidenza.

## Instructions for the assigned specialist

- La spec e' in `backlog` e **dipende da [SPEC-025]**, che e' in `review`. Non
  cominciare prima che sia `done`: il motore potrebbe ancora cambiare.
- Se questa spec passa a `ready`, esegui `spec_start` come prima azione e
  `spec_submit` a implementazione completa.
- **Il motore non si tocca.** Se il chiamante rivela un difetto del motore,
  fermati e riportalo come rilievo su [SPEC-025].
- **I timeout non si scelgono a occhio.** Derivali o dichiarali aperti con la
  ragione, come [SPEC-027] impone alla classe dei parametri.
- Se trovi un difetto fuori perimetro, **aprilo come rilievo e non correggerlo**.
  Precedenti: [DEBT-041], [DEBT-045], [DEBT-046].
- Niente commit ne' push: il push su `main` e' del Lead.
- Codice di produzione, niente segnaposto. Se un vincolo ti sembra fragile,
  contestalo e proponi l'alternativa pulita invece di aggirarlo.

## Risks

- **Prima esperienza del progetto con `libp2p`.** Il rischio di toolchain
  multipiattaforma e' gia' stato sbancato da [SPEC-002], ma la rete non e' la
  toolchain. Se lo stack si rivelasse un ostacolo, **riportalo presto**: e' il
  genere di scoperta che vale piu' di una settimana di aggiramenti.
- **La persistenza e' il punto in cui questa spec puo' fallire in silenzio.** Un
  write-ahead log che sembra funzionare e non regge a un `kill -9` nel momento
  sbagliato e' peggio di nessun log, perche' produce fiducia.

## Implementation evidence

> Compilata dallo specialista a lavoro concluso.

### Changes made

### Files changed

### Verification performed

### Verification transcript
