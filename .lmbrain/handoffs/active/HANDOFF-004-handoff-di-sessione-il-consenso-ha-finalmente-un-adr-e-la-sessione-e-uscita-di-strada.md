---
id: HANDOFF-004
title: "Handoff di sessione — il consenso ha finalmente un ADR, e la sessione è uscita di strada"
status: ready
from_role: AGENT-LEAD
to_role: AGENT-LEAD
created: 2026-08-27
updated: 2026-08-27
related_specs: [SPEC-022, SPEC-023, SPEC-024, SPEC-025]
related_reviews: [REVIEW-036, REVIEW-037, REVIEW-038, REVIEW-039, REVIEW-040, REVIEW-041, REVIEW-042]
related_decisions: [ADR-017, ADR-018]
links: [DEBT-035, DEBT-036, DEBT-037, DEBT-038]
tags: [session-handoff]
activity:
  - date: 2026-08-27
    action: "created"
---
# Project Lead session handoff

## Leggi prima questo

**La sessione è uscita dalla roadmap, e il Lead uscente non se n'è accorto per ore.** L'operatore l'ha fermata con una domanda sola: *«ti sembra in linea con la roadmap?»*

Non lo era. **M-02 nomina quattro esiti** — devnet BFT, light client con prove Merkle, mint & burn, simulatore economico. Solo il simulatore è fatto, e risale a [SPEC-007]. **Ventuno spec chiuse, diciassette dentro M-02, e nessuna consegna uno dei tre esiti restanti.**

La sessione ha invece prodotto: una spec di attuazione difendibile ([SPEC-022], chiude un `high` sul predicato di autorizzazione), e poi **tre passate di review su un documento preparatorio che non blocca niente**, più due spec di igiene. Ogni singolo passo sembrava giusto: ogni giro trovava difetti veri, e seguirli sembrava doveroso.

> **Il difetto non stava in nessuno dei passi: stava nel fatto che nessuno guardava dove si stava andando.** Quello è il lavoro del Lead, ed è il lavoro che non è stato fatto.

**Regola pratica che ne discende, e che il Lead entrante dovrebbe applicarsi a ogni giro:** prima di dispacciare qualunque cosa, chiedersi **quale esito della milestone quella cosa avvicina**. Se la risposta è «nessuno, ma è un difetto vero», allora è un debito da registrare, non un lavoro da fare adesso.

## Cosa abbiamo davvero in mano

Il Lead uscente l'ha misurato solo alla fine, ed è la seconda cosa da sapere.

**C'è:** una libreria di regole, `coblox-core`, **16 000 righe di Rust e 191 test**. Sa verificare firme Ed25519, hash, Merkle, serializzazione canonica, set di validatori, elezione, autorizzazione, quorum. Più **6 300 righe di specifiche di protocollo** con gate automatiche che le tengono coerenti.

**Non c'è:** rete — `libp2p` non è nemmeno una dipendenza, compare solo nei commenti. Persistenza: nessun database, nessun file. Un nodo: `coblox-node` è **ventun righe**, e `start` stampa *«not configured yet»*. Consenso in esecuzione: sappiamo **validare** un quorum, non c'è nulla che lo **raggiunga**. Le app sono gusci vuoti.

> **Abbiamo un validatore di regole, non una rete.** Due macchine oggi non potrebbero parlarsi, né produrre un blocco, né salvare niente.

## Il risultato che vale della sessione

**[ADR-018], accettato.** È la decisione che sblocca l'esito di M-02 e **non era mai stata presa**: `wire.md` non ha alcun messaggio di consenso, nessuna regola dice chi propone, nessun timeout esiste, e [ADR-001] nomina la famiglia BFT senza l'algoritmo.

Ma la scelta era **molto più stretta di quanto sembrasse**, e il protocollo l'aveva quasi fatta da sé. Due vincoli decidono: **un blocco porta il *proprio* certificato di quorum**, non quello del genitore; e i domini di firma per i voti sono **esattamente uno su quaranta**.

Da lì il crux: con un solo voto firmato un protocollo è sicuro **oppure** vivo, mai entrambi. Votare una volta per altezza è sicuro e blocca l'altezza per sempre se un proponente tace. Per sopravvivere a un round fallito serve rivotare; per restare sicuri serve una regola di blocco; e un blocco dev'essere **dimostrabile agli altri**, cioè firmato.

> **`coblox-block-vote-v0` è, senza saperlo, un precommit. Manca la prima fase.**

Si aggiunge `coblox-block-prevote-v0` e **non cambia nulla di pubblicato**.

**[SPEC-025] è redatta** e attua tutto questo, in `backlog`. Il taglio è dichiarato: consegna il **motore** e lo fa girare in un processo su trasporto in memoria, con **una catena finalizzata da quattro validatori** come prova. **Non è una devnet**, e la spec lo dice nella prima riga. Rete e persistenza sono la spec successiva.

## Lo stato, in numeri

**Venticinque commit**, da `3f1bef7` a `2f40deb`, tutti spinti su `main`. Albero pulito, **191 test**, `clippy` e `fmt` puliti, tutte le gate di progetto `exit=0`.

| | |
| --- | --- |
| Spec | 21 `done`, 2 in `review`, 2 in `backlog` |
| Review | 36 `accepted`, **4 `changes-requested`** |
| Debiti | **13 aperti**, di cui **5 `high`**, 1 deferred |

## Cosa è aperto, e cosa serve per chiuderlo

**[SPEC-022]** — attua [ADR-017]. Review di merito accettata; **`GATE-SECREVIEW` non superata**, [REVIEW-042], due `high`.

- **[REVIEW-042] RF-001 non è un difetto della spec: è un difetto di [ADR-017]**, cioè del Lead uscente. La banda di `key_compromise` ammette una finestra di inclusione larga `G+1`; il tetto di `G` è ancorato in genesi, **il pavimento no**. Con `G = 1` — documento lecito — una revoca per chiave compromessa va firmata a quorum e **inclusa entro due blocchi**, e due blocchi di censura la invalidano. **Il tetto è nuovo di questa spec**: prima un ritardo poteva solo rimandare. [ADR-017] ha corretto un veto di un proponente per un turno sostituendolo con **lo stesso veto a chi censura `G+1` blocchi**. Va corretto **nell'ADR**, non nella spec.
- **RF-002**: il testo della clausola 2 contraddice la riga `20` che la remediation precedente ha appena imposto, e **la probe la pinna nella forma ambigua**, quindi la gate è verde su di essa.
- **`GATE-TWO-ORACLES` non era soddisfatta su `AUTH-0`**: la seconda derivazione non è mai stata fatta. La tabella nuova è stata ottenuta **ribaltando le righe della vecchia**, che era costruita attorno a `effective_height = 50` e non aveva ragione di contenere `20`.
- **`GATE-CI-GREEN`** resta da soddisfare: i commit sono spinti, la run va guardata.

**[SPEC-023]** — la parte utile **è chiusa e accettata**: lo strumento che chiude la classe dei parametri gira in CI, e i dieci parametri operativi sono nella lista DRAFT. **L'analisi dei dieci parametri è ferma** dopo tre passate, [REVIEW-041] e [REVIEW-040] `changes-requested`. **Chiuderla richiede derogare `GATE-SECREVIEW`**, ed è una decisione dell'operatore, come fu per la guida pubblica. **Non lavorarci senza quella decisione.**

**[SPEC-024]** e **[SPEC-025]** in `backlog`. [SPEC-025] è la prossima cosa da fare.

## Le tre lezioni che hanno un artefatto, e vanno lette

**1. Il criterio del predicato di accettazione**, `.lmbrain/knowledge/predicato-di-accettazione.md`. Nato da tre passate su cinque righe che continuavano a cedere, e dalla scoperta che **il protocollo aveva già scritto la risposta**: una regola di validità può solo confrontare un numero scritto dai validatori con un altro, e una regola sulla distanza fra timestamp è *«rejected and not merely absent»*. Contiene anche l'asimmetria che nessuno aveva guardato: **al pavimento il pericolo è una proprietà del parametro, al tetto no**, e otto dei dieci parametri muoiono al pavimento mentre tre passate hanno litigato solo sui tetti.

**2. I riferimenti per numero di riga sono un generatore di difetti.** Quattro puntatori morti in una sessione. Le citazioni dell'analisi sono state convertite a **frase citata** — la stessa forma delle probe di [ADR-012], che infatti non sono mai scadute — e la conversione ha **stanato due difetti che i numeri nascondevano**, fra cui una citazione al documento sbagliato. **[SPEC-024]** versiona il risolutore che rende la pratica una regola.

**3. Scritture disgiunte non sono riferimenti disgiunti.** Due remediation lanciate in parallelo su file disgiunti: una citava per riga un documento che l'altra stava riscrivendo, e **le citazioni sono scadute mentre venivano scritte**. Sequenziare quando un agente cita ciò che un altro modifica.

## Cosa il Lead uscente ha sbagliato, per nome

Sono qui perché il Lead entrante non li ripeta, e perché tacerli renderebbe questo documento inaffidabile come gli altri che questa sessione ha corretto.

- **Sei conteggi scritti senza guardarli**, di cui l'ultimo in una review che ne condannava un altro: la cella della riga 3 diceva *«intatta per tre passate»* e sette righe sotto la riga 10 era dichiarata l'unica intatta. **Un superlativo e il suo controesempio nello stesso oggetto.** Enumerando le tre tabelle: la riga 9 ha ceduto **due** volte, una `high`; le righe 3 e 4 **una** ciascuna; la riga 10 **nessuna**. Quindi il superlativo era vero della riga 10 e falso della 3 — e **la partizione costruita su quella tabella poggiava sulla cella sbagliata**.
- **Una remediation scritta dal Lead ha introdotto due `high`**, fra cui un'ancora — `N_min` — che **non esiste in nessun documento di protocollo**, peggiore di quella che sostituiva.
- **Tre correzioni su dieci applicate alla sola tabella**, lasciando il corpo a contraddirla.
- **Un appello all'autorità dentro un artefatto scritto dalla stessa autorità**: *«Verificato dal Lead»*.
- **Chiesto all'operatore il permesso di pushare**, quando `.lmbrain/BRANCHING.json` dichiara `main-only` e `lead_only_push_branches: ["main"]`. La regola era scritta e versionata: non leggerla e chiedere è scaricare sull'operatore un lavoro del Lead.

**La forma comune: il difetto era quasi sempre già scritto da qualche parte, e non guardato.**

## Decisioni che aspettano l'operatore

1. **La correzione di [ADR-017]** su [REVIEW-042] RF-001 — il pavimento di `G` nell'ancora di genesi. Il Lead uscente la considera necessaria e non l'ha scritta.
2. **La deroga di `GATE-SECREVIEW` su [SPEC-023]**, o la quarta remediation dell'analisi.
3. **I valori di taratura**: `F`, `G`, `P`, `max_clock_drift_ms`, `D_max`/`S_max`, e i dieci di [DEBT-036]. [REVIEW-042] stabilisce che **il pavimento di `G` e lo scarto `P−(F+G)` sono grandezze di sicurezza, non di comodità**.
4. **L'advisory Dependabot**: GitHub segnala **una vulnerabilità moderata** sul default branch. `cargo-deny` è in CI su tutto il grafo, quindi o è un advisory che non copre o è arrivato dopo l'ultima run. **Nessuno l'ha ancora guardato.**

## Cosa fare per primo

**[SPEC-025].** **Enumerando** la coda per esaurimento — quattro voci, nessuna esclusa: [SPEC-022] in `review` (remediation, chiude un debito), [SPEC-023] in `review` (ferma, aspetta una deroga), [SPEC-024] in `backlog` (igiene sulle citazioni), [SPEC-025] in `backlog` (il motore di consenso). **Delle quattro, la sola che avvicini un esito nominato da M-02 è [SPEC-025]** — le altre tre chiudono debiti o igiene. Portarla a `ready` e dispacciarla ad AGENT-001 con modello `opus`, come impone `capability_tier: sol`.

Prima di lanciarla: **la remediation di [SPEC-022] tocca `ledger.md` e `core/`, cioè gli stessi file**. Sequenziare, non parallelizzare — è l'errore già pagato in questa sessione.

E le due gate del Lead su [SPEC-025] vanno prese sul serio: `GATE-LOCKING-FROM-SOURCE` chiede che la regola di blocco sia **presa dalla letteratura e confrontata riga per riga con la fonte nominata**, e dice che una regola derivata da capo **va respinta anche se i test passano** — perché un BFT sbagliato passa il caso felice sempre. `GATE-SAFETY-UNDER-ADVERSARY` chiede **quante esecuzioni avverse** sono state percorse: un numero che non c'è è una prova che non c'è.

## Receiving Project Lead checklist

- [ ] `cargo test --workspace --all-features` → attendersi **191**.
- [ ] `python sim/tools/published_artifacts.py` e la sua prova in negativo → `PASS`.
- [ ] `python sim/tools/consensus_parameters_closure.py` e `--negative` → `exit=0`.
- [ ] `python sim/tools/lead_claims_check.py` → `PASS`.
- [ ] Leggere `.lmbrain/knowledge/predicato-di-accettazione.md` **prima** di scrivere qualunque vincolo.
- [ ] Leggere [ADR-018] prima di toccare [SPEC-025].
- [ ] Guardare l'advisory Dependabot.
- [ ] **Prima di dispacciare qualunque cosa: quale esito di M-02 avvicina?**

## Handoff outcome

> Compilato dal Lead entrante.
