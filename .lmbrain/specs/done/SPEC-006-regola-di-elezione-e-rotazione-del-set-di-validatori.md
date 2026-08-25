---
id: SPEC-006
# Note: Quote the title if it contains a colon
title: "Regola di elezione e rotazione del set di validatori"
status: done
kind: feature
priority: high
area: consensus
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
# Implementation estimate. Required before this spec can become `ready`.
# capability_tier: luna | terra | sol   (expected change footprint)
# thinking_level: minimal | standard | extended | maximum (defaults from the tier)
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-001, ADR-002, ADR-007, ADR-008]
links: [SPEC-001, SPEC-004, DEBT-005]
created: 2026-08-25
updated: 2026-08-25
tags: [governance, sybil, light-client]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "set effort"
  - date: 2026-08-25
    action: "set tags"
  - date: 2026-08-25
    action: "set tags"
  - date: 2026-08-25
    action: "transitioned backlog -> ready"
  - date: 2026-08-25
    action: "transitioned ready -> working"
  - date: 2026-08-25
    action: "transitioned working -> review"
  - date: 2026-08-25
    action: "attested verification GATE-SECREVIEW by lead"
  - date: 2026-08-25
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "a516d113cbb3798ec47e2021cff0ce92dba42075b6dd2061627161d001e626dc"
    evidence_ref: "REVIEW-010, accettata da AGENT-007 dopo quattro giri di review adversariale e tredici finding, di cui tre critical e tre high. La reviewer chiude dichiarando che il claim di sicurezza sulla composizione del set e ora difendibile senza dichiarazioni accanto. Il Lead ha verificato in modo indipendente a ogni giro: i tre hash delle fixture PD-0 ricalcolati con il metodo validato su una fixture non modificata, l'intero esempio numerico riprodotto a ogni cambio di formula, l'aritmetica delle coppie di vincoli, la simulazione della successione dei set su 30 e 60 confini, e l'enumerazione di 36300 coppie di timbri che conferma che le collisioni esistono se e solo se il limite di mandato decresce."
    id: "SPEC-006-ATTEST-001"
    requirement_digest: "f8e99c2d6147e0faba4163f978a8cdb53836f68026069b906c77d1eea300d5f1"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T14:47:27.956639300+02:00"
---
# Regola di elezione e rotazione del set di validatori

## Objective

Scrivere nei documenti di protocollo la regola che determina **chi** entra nel set di validatori successivo, chiudendo [DEBT-005]. Oggi `ledger.md` autentica in modo sicuro la transizione da un set al successivo ma non vincola in alcun modo la sua composizione, e in quello spazio vuoto il set corrente è l'unico soggetto che scrive il proprio successore.

## Context

[DEBT-005] è l'unico debito `critical` del progetto e nasce dallo scenario `TM-18` di [SPEC-004]. `ledger.md` lo dichiara apertamente: *"This continuity rule specifies safe authentication but not how members are elected or rotated"*. La conseguenza è che un quorum raggiunto una sola volta può impegnare un successore composto interamente da sé stesso, all'infinito, e la catena resta formalmente valida a ogni passo.

**Perché ora e non in M-07.** La roadmap colloca la rotazione automatica in M-07, e per l'*automazione* è ragionevole. Non lo è per l'*invariante*: una rete che accumuli storia sotto questa regola mancante può chiudersi permanentemente senza che nessuno se ne accorga, e la chiusura **non è reversibile a posteriori**, perché il set insediato controlla ogni transizione futura. [DEBT-005] impone quindi che nessuna devnet accumuli storia conservabile prima che la regola esista — e la devnet è il prodotto centrale di M-02. L'ordine è forzato, non scelto.

**Non sei bloccato da [DEBT-007].** Si potrebbe temere che l'eleggibilità dipenda dalla taratura economica ancora aperta. Non è così: `α` governa la *distribuzione dell'emissione*, l'eleggibilità governa *chi può validare*. Sono meccanismi distinti. Scrivi la regola con soglie e tetto di rotazione come **parametri simbolici dichiarati e limitati**, non come numeri: il simulatore economico di M-02 li riempirà dopo. Se ti accorgessi di un accoppiamento reale che qui non abbiamo visto, fermati e segnalalo invece di inventare un numero.

## Scope
### Included

- La regola di elezione e rotazione nei documenti di protocollo, con la definizione dell'insieme degli eleggibili, la derivazione deterministica, il tetto di rotazione per epoca e l'impegno che ne consente il ricalcolo.
- L'aggiornamento di `ledger.md` nel punto che oggi dichiara la lacuna, e di ogni sezione che vi rimanda.
- La verifica che i test di attacco `AT-09` e `AT-10` di [SPEC-004] passino sulla regola scritta, e l'aggiornamento di `threat-model.md` dove `TM-18` e `SEC-REQ-13` ne risultano coperti.

### Excluded

- **Implementazione in Rust.** Questa spec produce specifica, non codice: come [SPEC-001], il deliverable sono documenti versionati. L'implementazione è spec successiva di M-02.
- Il valore numerico dei parametri, che dipende dal simulatore ([DEBT-007]).
- Lo slashing reputazionale e la governance dei parametri, che la roadmap colloca in M-07 e che qui non servono per l'invariante.
- Qualunque modifica alla regola di revoca, che è già scritta e corretta.

## Existing-project analysis

Il Lead ha letto `ledger.md` §*Validator-set continuity* e §*Revocation forces a validator set transition* prima di redigere. Tre fatti vincolano la soluzione.

**Il problema del light client è già documentato, ed è lo stesso problema.** La sezione sulla revoca contiene una confessione esplicita e la corregge: un light client *"checks set-hash continuity and never sees transactions"*, quindi *"observes a transition if one happens; it cannot establish that a transition was due"*. È esattamente la forma del difetto di [DEBT-005]. Ne segue l'inquadramento che questa spec deve adottare: **la regola di elezione non serve solo a impedire la cattura, serve a rendere il set verificabile come quello *giusto* e non soltanto come uno *continuo*.** Chi progetta la regola guardando solo l'auto-perpetuazione produrrà una regola che i full node possono verificare e i light client no, cioè ripeterà il difetto che quel paragrafo ha già ammesso una volta.

**Il crux è la verificabilità con informazione parziale.** Un light client non vede le transazioni, quindi non può ricalcolare da sé l'insieme degli eleggibili a partire dalle evidenze. Perché la regola sia verificabile anche da lui, **sia la casualità sia l'impegno all'insieme degli eleggibili devono essere raggiungibili dagli header**, così che il controllo si riduca a: dato quell'impegno e quella casualità, il set dichiarato è la derivazione corretta. Il protocollo ha già un precedente utile da riusare invece che reinventare: `existence_income` impegna `eligible_set_root` con foglie `H(0x24 || u64be(reward_epoch) || account_key_32)` in un albero di Merkle su insieme ordinato bytewise, ed è già lo schema di *"insieme calcolabile da chiunque, impegnato in un punto"*.

**La casualità ha già un meccanismo nel protocollo.** Esistono `challenge_commitment` con `(issuer_node_id, commitment_epoch, issuer_commitment)` e una `randomness_source` verificata. Valuta se riusare quel meccanismo o se serva casualità finalizzata di natura diversa: in entrambi i casi la scelta va motivata, perché una casualità che il set uscente può influenzare a proprio favore riapre l'auto-perpetuazione da un'altra porta, in forma più difficile da vedere.

**Vincolo da [ADR-007], non negoziabile qui.** L'eleggibilità a validatore è ancorata a **lavoro difficile da falsificare** — storage e compute dimostrati — e **mai al solo uptime**, che una VPS con SLA batte per costruzione contro qualunque telefono reale. [ADR-001] parla di *"reputazione e uptime dimostrato"*: è formulazione anteriore ad [ADR-007] e va letta come superata su questo punto. Se ritieni che [ADR-007] vada rivisto, non modificarlo: segnalalo al Lead.

**Vincolo da [ADR-008].** L'eleggibilità ancorata a storage e compute deve poter dimostrare di avere un **tetto di fabbisogno**. Il punto 1 del test di [ADR-008] dice che se un nodo può guadagnare di più spendendo di più senza che la rete abbia bisogno di più, è mining. Una soglia di eleggibilità che premi senza limite chi spende di più reintrodurrebbe per la porta di servizio la corsa alla spesa che l'esclusione vieta. Dichiara esplicitamente l'esito dei tre punti del test, come [ADR-008] impone a ogni spec che introduca una forma di lavoro remunerato o premiato.

## Technical proposal

Progetta la regola attorno alle quattro proprietà che [DEBT-005] fissa nei propri criteri di risoluzione, e trattale come requisiti e non come suggerimenti: derivazione **deterministica** a partire da casualità **finalizzata**; insieme degli eleggibili **calcolabile** da chiunque disponga delle informazioni impegnate; **tetto di rotazione per epoca**, perché un ricambio totale improvviso è una superficie di attacco quanto un ricambio nullo; **impegno nell'header** che consenta il ricalcolo a posteriori.

Il tetto di rotazione merita attenzione in entrambe le direzioni. Troppo basso, si torna all'auto-perpetuazione lenta; troppo alto, un avversario che conquista l'eleggibilità in un'epoca ribalta il set in una sola transizione. Dichiara il ragionamento, non solo il parametro.

Definisci anche cosa accade nei casi degeneri, perché sono quelli in cui una regola elegante si rompe: eleggibili in numero inferiore alla dimensione del set; parità nella derivazione; un'epoca senza casualità valida; e l'interazione con la regola di revoca, che può svuotare un set fra un'elezione e la successiva.

## Files and areas involved

- `docs/protocol/ledger.md` — §*Validator-set continuity*, in particolare la riga che dichiara la lacuna, e le sezioni di verifica del light client che vi rimandano.
- `docs/protocol/identity.md` — i due rimandi a `ledger.md#validator-set-continuity`.
- `.lmbrain/knowledge/threat-model.md` — `TM-18`, `SEC-REQ-13`, e i test `AT-09` e `AT-10`.
- Eventuali altri documenti in `docs/protocol/` che assumano la composizione del set: verificali, non presumerli.

## Acceptance criteria
- [x] `ledger.md` non contiene più l'affermazione che la regola di continuità non specifica come i membri siano eletti o ruotati: al suo posto c'è la regola.
- [x] La regola è **deterministica**: due verificatori indipendenti con gli stessi input impegnati derivano lo stesso set, e questo è affermato in forma verificabile.
- [x] L'insieme degli eleggibili è definito in modo **calcolabile da chiunque** disponga delle informazioni impegnate, con criterio ancorato a storage e compute dimostrati e **mai al solo uptime** ([ADR-007]).
- [x] È dichiarato un **tetto di rotazione per epoca**, come parametro simbolico, con il ragionamento su entrambi gli estremi.
- [x] Esiste un **impegno** che consente il ricalcolo a posteriori della derivazione.
- [x] È dichiarato esplicitamente **che cosa può verificare un light client e che cosa no**, senza sopravvalutare la garanzia. Se resta un residuo non verificabile da light client, va scritto come tale: una dichiarazione di sicurezza sbagliata è peggio di una mancante, ed è la posizione che `ledger.md` ha già preso su sé stesso.
- [x] La casualità usata non è influenzabile a proprio favore dal set uscente, e la ragione è argomentata.
- [x] I casi degeneri sono coperti: eleggibili insufficienti, parità, epoca senza casualità valida, interazione con la revoca.
- [x] È dichiarato l'esito dei tre punti del test di [ADR-008], con particolare attenzione al punto 1, il tetto di fabbisogno.
- [x] `AT-09` e `AT-10` di [SPEC-004] sono valutati contro la regola scritta, con l'esito argomentato; `TM-18` e `SEC-REQ-13` sono aggiornati dove risultano coperti.
- [x] Nessun parametro numerico è inventato: ciò che dipende dal simulatore resta simbolico e dichiarato come tale.

## Implementation plan
1. Leggere [DEBT-005], `TM-18` e `SEC-REQ-13`, e le due sezioni di `ledger.md` indicate nell'analisi.
2. Definire l'insieme degli eleggibili e il suo impegno, valutando il riuso dello schema `eligible_set_root` già presente.
3. Scegliere e motivare la sorgente di casualità finalizzata.
4. Definire la derivazione deterministica, il tetto di rotazione e l'impegno nell'header.
5. Trattare i casi degeneri e l'interazione con la revoca.
6. Scrivere esplicitamente il perimetro di verifica del light client, residui compresi.
7. Valutare `AT-09` e `AT-10`, aggiornare il threat model.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-DETERMINISM | kind=manual | owner=agent | phase=before-submit | evidence=artifact | La derivazione è illustrata su un esempio numerico completo e riproducibile a mano, dagli input impegnati al set risultante, in modo che un revisore possa rifarlo senza codice. Un esempio che non si può rifare non è una specifica verificabile.
- [x] GATE-LIGHTCLIENT | kind=manual | owner=agent | phase=before-submit | evidence=artifact | Il documento dichiara in modo esplicito e circoscritto che cosa un light client può stabilire sulla composizione del set e che cosa resta fuori dalla sua portata, nella stessa forma onesta già adottata da `ledger.md` per la revoca.
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la regola come superficie di sicurezza e il Lead ha accettato la review. Il debito che questa spec chiude è `critical`: la review di sicurezza non è facoltativa.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio più probabile è progettare per i full node e non per i light client.** È l'errore che `ledger.md` ha già commesso una volta sulla revoca, se n'è accorto, e lo ha corretto scrivendo che una dichiarazione di sicurezza sbagliata è peggio di una mancante. Una regola verificabile solo da chi vede tutte le transazioni lascia in piedi metà del difetto e la fa sembrare risolta. `GATE-LIGHTCLIENT` esiste per questo.
- **Rischio di casualità catturabile.** Se il set uscente può influenzare a proprio favore la sorgente di casualità, l'auto-perpetuazione rientra da una porta più difficile da vedere di quella che stiamo chiudendo. Questa è la parte da attaccare per prima, non per ultima.
- **[ADR-001] è in tensione con [ADR-007] su questo punto** e la spec risolve la tensione a favore del secondo: eleggibilità su lavoro difficile da falsificare, mai sul solo uptime. Se ritieni che la scelta giusta sia l'opposta, **non modificare gli ADR**: argomenta e segnala al Lead, che valuterà se serve una nuova ADR.
- **Aperto, e non lo risolvi qui**: i valori numerici dei parametri. Dipendono dal simulatore economico ([DEBT-007]). Lasciarli simbolici è la risposta corretta, non un compromesso.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence

### Changes made

**La regola, in una frase.** Il set successivo non è più scritto dal quorum
uscente: è l'**uscita esatta di una derivazione** su un insieme di eleggibili
impegnato e su un seme finalizzato, ed è vincolato da una forma che un light
client verifica senza vedere nemmeno una transazione. La regola è divisa in due
strati **perché falliscono in modo diverso**, e la divisione è la scelta di
progetto principale:

- **Strato 1 — forma e ricambio.** Limite di mandato (`seated_since_epoch` per
  voce contro `validator_max_consecutive_terms`), potere di voto uniforme a 1,
  `validator_id = node_id`, tetto di ingressi per epoca, limiti di dimensione, e
  la regola nuova che chiude il buco più largo trovato durante il lavoro: **a
  ogni altezza che non sia un confine di epoca o una transizione forzata da
  revoca, `next_validator_set_hash` DEVE essere uguale a `validator_set_hash`**.
  Tutto lo strato 1 è funzione dei soli documenti `ValidatorSet` e dei due campi
  di header che il light client già legge. È lo strato che rende
  l'auto-perpetuazione **impossibile**, e **non dipende dalla qualità della
  casualità**.
- **Strato 2 — composizione.** Chi riempie i seggi che lo strato 1 libera:
  biglietti `election_ticket` ordinati crescenti su `candidate_root`, con il
  taglio al tetto. Verificabile per intero solo replaying le transazioni
  finalizzate; falsificabile in modo compatto in due dei tre modi di
  fallimento.

**Ancoraggio ad [ADR-007].** L'eleggibilità è una **soglia binaria** su un
`contribution_score` calcolato dalle sole evidenze `storage` e `compute` con
`outcome:"passed"`. L'evidenza `availability` contribuisce **zero**: il punto 2
di [ADR-007] è scritto come aritmetica, non come intenzione. Sopra la soglia
nulla migliora — un biglietto per nodo, un'unità di potere per seggio.

**Impegno senza campi di header nuovi.** `ElectionRecord` sta dentro
`ValidatorSet`, quindi è impegnato da `validator_set_hash`, che l'altezza
precedente impegna già come `next_validator_set_hash`. Il criterio di [DEBT-005]
"impegno nel header che consenta il ricalcolo a posteriori" è soddisfatto
transitivamente ed esattamente, al costo di zero byte per blocco.

**Nuova transazione `validator_candidacy`.** È risultata necessaria e non
opzionale: `key_binding_signature` è firmata su un `activation_height`
specifico, quindi senza una dichiarazione anticipata il set uscente dovrebbe
inventare chiavi di consenso altrui. La candidatura è per singola epoca, vale
per incumbent e nuovi allo stesso modo, e ne consegue che smettere di
dichiararsi è l'uscita volontaria, senza bisogno di un meccanismo di rimozione.

**Casualità: attaccata per prima, e la regola è stata progettata per non
dipenderne.** Un beacon derivato dagli ID di blocco è *macinabile* da chi
propone quei blocchi — il campo `timestamp_ms` da solo offre 10^3–10^6 valori
legali, come `ledger.md` già quantifica per le challenge. L'aggregazione su
`election_entropy_blocks` blocchi consecutivi è una delle due riduzioni che
`ledger.md` aveva rinviato "al beacon dedicato, lavoro di M-02 sotto DEBT-005",
ed è presa qui; non elimina però il best-of-`G` di chi propone l'ultimo blocco
della finestra, e in v0 (solo Ed25519, niente VDF né firme a soglia) **non è
eliminabile**. La risposta di progetto non è fingere che lo sia: è rendere
l'invariante indipendente dal seme. La macinatura produce *bias* e mai *scelta*,
il guadagno è comunque limitato superiormente da `validator_churn_cap_seats`, e
un incumbent non può macinarsi un mandato più lungo a nessun prezzo perché la
scadenza del mandato non è funzione del seme. Il residuo è dichiarato con la sua
forma: `c*p + O(sqrt(c*p*(1-p)*2*ln G))` seggi invece di `c*p`.

**Tetto e pavimento.** Il tetto da solo non chiude il difetto: un set a cui è
solo vietato cambiare *in fretta* può comunque non cambiare *mai*. Il pavimento
è il limite di mandato, che rende il ricambio un'aritmetica (`ceil(V/T)` seggi
per epoca) e non un'intenzione. La relazione fra i due è un **vincolo di
validità sul documento dei parametri di consenso**, con lo stesso meccanismo del
pavimento di costo dell'enrollment e del tetto della quota al creatore.

### Files changed

- `docs/protocol/ledger.md` — invariante 8 in §"Model and invariants";
  `validator_candidacy` nell'enum `kind` e nella classe 0 dell'ordine di
  esecuzione; `ValidatorSet` esteso con `election` e `seated_since_epoch`; la
  frase che dichiarava la lacuna sostituita dalla regola di confine; regola
  "solo rimozione" per le transizioni fuori confine in §"Revocation forces a
  validator set transition"; nuova §"Validator election and rotation" con nove
  sottosezioni (epoche, candidatura, eleggibilità, insieme impegnato, seme,
  derivazione, tetto e pavimento, casi degeneri, test di [ADR-008], perimetro
  del light client, esempio numerico); nuovo passo 5 dell'algoritmo di verifica
  light-client con rinumerazione dei successivi; §"DRAFT" ridotta ai soli valori
  economici.
- `docs/protocol/README.md` — `election_entropy`, `election_seed`,
  `election_ticket` nel registro dei preimmagine; fixture `ELEC-0` e tre righe
  nella tabella di conformità; `RewardPolicyBody` +4 campi;
  `ConsensusParametersBody` +10 campi; **ricalcolo di `policy_hash` e
  `consensus_parameters_hash` per `PD-0`**, perché estendere i corpi cambia i
  byte JCS; nota sulle eccezioni di valore del fixture consensus; §"DRAFT"
  aggiornata.
- `docs/protocol/identity.md` — la chiave di consenso è pubblicata dal nodo
  stesso via `validator_candidacy`; nessuna identità è coscritta.
- `.lmbrain/knowledge/threat-model.md` — `TM-18` e `TM-09` passano a "mitigato
  in specifica" con l'aggiornamento e la contromisura estesa; `SEC-REQ-13`
  aggiornato a "M-02 coperto, M-07 aperto"; valutazione argomentata di `AT-09` e
  `AT-10`; §6.1 annotata con la scelta compiuta e la leva **non** adottata;
  riferimento obsoleto alla §"DRAFT: committee selection" corretto.

### Verification performed

1. **Metodo JCS validato prima di usarlo.** I due `PD-0` non toccati da questa
   modifica (`parameter_set_hash`, `hosting_rate_card_hash`) sono stati
   ricalcolati e coincidono con i valori pubblicati: la procedura che ha prodotto
   i due hash *nuovi* è quindi la stessa che ha prodotto quelli esistenti.
2. **Ogni digest pubblicato dalla modifica è stato ricalcolato dalle sue
   preimmagini** e confrontato con i byte effettivamente scritti nei documenti
   (non con una copia locale): 5 foglie candidato, la foglia vuota, 6 nodi
   interni, `candidate_root`, `election_entropy`, `election_seed`, 3 biglietti,
   più le corrispondenze incrociate `ledger.md` ↔ `README.md`.
3. **La derivazione dell'esempio è stata rifatta in modo indipendente** dai
   valori di partenza (ritenzione, cooldown, soglia, pool, ordinamento per
   biglietto, minimo a tre argomenti, assemblaggio) e confrontata con la tabella
   scritta nel documento.
4. **Il blocco di vincoli sui parametri è stato valutato** sui valori del fixture
   consensus `PD-0`, incluso il vincolo derivato `T >= 3*m`.
5. **Assenze e presenze testuali**: la frase che dichiarava la lacuna non esiste
   più in `ledger.md`; la sezione, la regola di confine e la voce di enum ci
   sono; l'intestazione "DRAFT: committee selection" è sparita.
6. **Ancore interne**: tutti i collegamenti `#anchor` fra i quattro documenti di
   `docs/protocol/` risolvono a un'intestazione esistente (0 rotti), verificato
   dopo ogni gruppo di modifiche.

### Verification transcript

Lo script di verifica è stato eseguito dalla radice del repository. Non è un
deliverable della spec (che produce specifica, non codice) ed è interamente
ricostruibile dalle formule scritte nei documenti: ogni digest sotto è
riproducibile con un qualsiasi strumento SHA-256 a partire dalle preimmagini
pubblicate in `ledger.md` §"Worked example of the derivation" e nel registro di
`README.md`.

```text
== 1. JCS method validated against the two UNCHANGED published fixtures ==
  parameter_set_hash       PASS sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
  hosting_rate_card_hash   PASS sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8
== 2. PD-0 fixtures whose bodies THIS CHANGE extends ==
  PASS policy_hash PD-0                   sha256:bff1dc6635d74bb39b4a1ea6a50340fb95f36da9725ab31416fede0aa2cdfe5c
  PASS consensus_parameters_hash PD-0     sha256:ab9886270886063d9f331fad63a8371fe6f00ff80ec609bdfbe26e4ee21f4f06
  RewardPolicyBody         PASS  missing=[] extra=[]
  ConsensusParametersBody  PASS  missing=[] extra=[]
== 3. Worked example, recomputed from the preimages stated in ledger.md ==
  PASS candidate_leaf 02                  cd3950bac9c60b73523a0f8157e5da8384515d2e9c4ef4127be2d910b878158c
  PASS candidate_leaf 04                  004e3b02570032774d181a20dbb4d5e23f6fe83092374635189f42c513db97d9
  PASS candidate_leaf 05                  154a73c742c2f580aafaf97401ef5633898862d761fc13ed230db7817b0111fd
  PASS candidate_leaf 06                  9ecce0184a6b8d6d3a1257add0af3a16cc64809fe2cf723cfe91b570abe07f44
  PASS candidate_leaf 08                  c9e67f59e0d4a6c08cc588bb906b17c8bddba657def06d6c2f111c64420796ca
  PASS candidate_empty H(0x42)            df7e70e5021544f4834bbee64a9e3789febc4be81470df629cad6ddb03320a5c
  PASS internal node 0                    00d36c1eb2fc336cb635735bbae46cf3e2a32322f4761db3d29a297644e45bb2
  PASS internal node 1                    db1bd3372cea52438df608862c3dcb3c0a2b1c16b152c296c2a8effe9005bc20
  PASS internal node 2                    b65c2a34ededd92d98721c11a620cc6f16e87a03f512e54fd35b35cd70c3bf39
  PASS internal node 3                    a7ee32ed571f99897d698653a14d292cfea8e8633f95a9db217ea993c7773e91
  PASS internal node 4                    a2bb9ac490112abad3c2af7197fdf6efe62af01b7c251f8d3443b20fa145a060
  PASS internal node 5                    5d426fa91db2b6acb77c0980837dea9e54380afc25eede062e05e2cab2dc6c2b
  PASS candidate_root                     42e4f6b1f01af3b69ba154aa464738829635a3ed7facf65e652d9712b461924a
  PASS candidate_root (README ELEC-0)     42e4f6b1f01af3b69ba154aa464738829635a3ed7facf65e652d9712b461924a
  PASS election_entropy                   29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
  PASS election_entropy (README)          29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
  PASS election_seed                      7c230b487507bda2fcc4786aa4233045cee026909b172daf67276a138440de30
  PASS election_seed (README)             7c230b487507bda2fcc4786aa4233045cee026909b172daf67276a138440de30
  PASS election_ticket 05                 621ddcec56959a7c2287c78599556fadbfa35ad67eb412c84c0c6ac2aaa23c92
  PASS election_ticket 06                 17e2da63fdeca8f09f38f7ab00a357feeacef4f615d9d45578670f9624b0efb1
  PASS election_ticket 08                 848e2604554bdec1c276688dc46e55d536290a0941ace3f357ff8e6ebc6ded60
  PASS election_ticket (README ELEC-0)    621ddcec56959a7c2287c78599556fadbfa35ad67eb412c84c0c6ac2aaa23c92
== 4. Worked example, derivation steps redone independently ==
  computed: {'retained': [2, 4], 'C': [2, 4, 5, 6, 8], 'candidate_count': 5, 'pool': [5, 6, 8], 'ticket_order': [6, 5, 8], 'fills': 2, 'elected': [2, 4, 5, 6], 'seated_since': {2: 2, 4: 1, 5: 3, 6: 3}}
  document: {'retained': [2, 4], 'C': [2, 4, 5, 6, 8], 'candidate_count': 5, 'pool': [5, 6, 8], 'ticket_order': [6, 5, 8], 'fills': 2, 'elected': [2, 4, 5, 6], 'seated_since': {2: 2, 4: 1, 5: 3, 6: 3}}
  PASS derivation matches the table in ledger.md
== 5. Parameter constraint block, evaluated on the consensus PD-0 fixture ==
  0 < min <= V <= max                        PASS
  election_entropy_blocks >= 2               PASS
  candidacy_close > entropy                  PASS
  election_epoch_blocks > candidacy_close    PASS
  ceil(V/T) <= c                             PASS
  3*c*m <= V                                 PASS
  derived: T >= 3*m                          PASS
== 6. The declared gap sentence is gone; the rule is present ==
  old gap sentence absent                              PASS
  "## Validator election and rotation" present         PASS
  boundary rule present                                PASS
  DRAFT committee-selection heading gone               PASS
  validator_candidacy in tx kind enum                  PASS
  threat model quote of the gap only inside TM-18 history PASS

checks passed: 42   failed: 0
```

Controllo delle ancore interne, eseguito separatamente sulla stessa copia:

```text
broken anchors: 0
```

### Deviations from the specification

Nessuna deviazione dallo scope. Quattro scelte vanno però dichiarate perché
eccedono la lettera del piano, pur restando dentro il perimetro:

1. **Una nuova classe di transazione, `validator_candidacy`.** La spec non la
   prevedeva esplicitamente. È risultata necessaria: senza dichiarazione
   anticipata della chiave di consenso non esiste modo di produrre un
   `key_binding_signature` valido per il set eletto, e l'unica alternativa
   sarebbe stata far scegliere al set uscente le chiavi altrui, cioè il difetto
   da chiudere.
2. **Il vincolo `next_validator_set_hash == validator_set_hash` fuori dai
   confini**, e la regola "solo rimozione" per le transizioni forzate da revoca.
   Non erano nel piano. Senza il primo, la regola di elezione sarebbe stata
   aggirabile cambiando set a un'altezza qualsiasi; il secondo chiude la
   "sostituzione d'emergenza", che sarebbe stata l'auto-perpetuazione con un
   pretesto. Nessuno dei due modifica la regola di revoca: la vincolano dal lato
   della composizione, che la spec assegna a questo lavoro.
3. **Ricalcolo di due hash di fixture già pubblicati** (`policy_hash` e
   `consensus_parameters_hash` per `PD-0`). Estendere i corpi dei documenti
   governati cambia i byte JCS; lasciare i vecchi valori avrebbe introdotto due
   fixture di conformità sbagliate.
4. **Modifica di `identity.md` oltre i due rimandi indicati.** Il paragrafo sulla
   chiave di consenso affermava solo dove sta la regola di legame; ora dice anche
   chi pubblica la chiave, che è la parte nuova.

Il rilievo che avevo segnalato senza correggerlo — `identity.md` §"Revocation"
affermava che «a light client can see that transition rather than having to trust
a claim about it» — è stato **corretto in remediation** su valutazione del Lead:
vedi [Remediation RF-001](#remediation-rf-001-review-009) sotto.

### Remediation RF-001 ([REVIEW-009])

**Finding.** `docs/protocol/identity.md` §"Revocation and key replacement"
affermava, per il caso di revoca di un validatore in carica, che *"a light client
can see that transition rather than having to trust a claim about it"*.
`ledger.md` aveva già ritrattato esattamente questa affermazione su sé stesso —
*"it observes **a** transition if one happens; it cannot establish that a
transition was **due**"* — quindi il documento sorella conservava la versione
riconosciuta come falsa. Rilievo individuato e segnalato da me in evidenza, non
corretto di iniziativa perché la spec escludeva la regola di revoca; il Lead ha
valutato che la correzione non tocchi quella regola, trattandosi di una
descrizione di ciò che un light client può stabilire, che è materia di questa
spec.

**Modifica, una sola.** Il paragrafo di `identity.md` ora separa esplicitamente i
due verificatori e non promette al secondo ciò che può solo il primo:

- un full node vede la transazione `revoke_identity` e **applica la regola per
  intero**;
- un light client non vede transazioni: **osserva *una* transizione se avviene**,
  e può verificare che sia **di sola rimozione** — la condizione aggiunta da
  questa stessa spec — ma **non può stabilire che la transizione fosse dovuta**;
- per la parte coperta si affida alla lista `revoked_validators` del proprio
  weak subjectivity checkpoint, con il limite già dichiarato in `ledger.md`: chiude
  solo per le revoche note al momento dell'emissione del checkpoint.

**Perimetro rispettato.** La **regola di revoca non è stata toccata**: nessuna
modifica a `ledger.md` §"Revocation forces a validator set transition", ai suoi
punti 1–9, né ad alcun altro documento. Il diff della remediation è confinato a
quel solo paragrafo di `docs/protocol/identity.md` (16 righe inserite, 4
rimosse). La spec è rimasta in `review` per tutta la remediation: `spec_start`
non è stato chiamato.

**Verifica della remediation.** Le ancore interne dei quattro documenti di
`docs/protocol/` risolvono ancora tutte (0 rotte), rieseguito dopo la modifica.
Nessun digest, fixture o formula è coinvolto dalla modifica, quindi il transcript
numerico sopra resta valido senza rigenerazione; è stato comunque rieseguito e
continua a dare 42 controlli superati e 0 falliti.

```text
broken anchors: 0
checks passed: 42   failed: 0
```

### Remediation REVIEW-010 (`GATE-SECREVIEW`, sei finding)

AGENT-007 ha attaccato senza successo le tre superfici che avevo indicato come
attaccabili e ha trovato i difetti altrove. Tutti e sei i finding sono chiusi in
questa remediation; nessuno ha richiesto di rifare la derivazione.

**RF-001 (critical) — le grandezze dei parametri, non solo le loro relazioni.**
Il blocco di vincoli legava i parametri fra loro e taceva sulle magnitudini, e i
parametri li firma il quorum: `election_epoch_blocks = 2^60` e
`validator_max_consecutive_terms = 2^60` soddisfano ogni relazione e spengono
l'invariante — nessun confine, nessuna scadenza, e la regola di confine che
*impone* al set di non cambiare. Chiuso con l'oggetto **`ElectionBounds` del
trust anchor di genesi** (`README.md` §"Election bounds"): tetti e pavimenti di
magnitudine più una variazione massima per `sequence` consecutiva, fuori dalla
governance della catena, spediti nella distribuzione firmata e non apprendibili
da un peer. È deliberatamente lo stesso ragionamento con cui `identity.md`
giustifica il pavimento di costo dell'enrollment, applicato un livello più in
fuori. Aggiunto anche il perché della variazione massima: senza, un set
camminerebbe un parametro fino al tetto in un passo, cioè la stessa manovra più
lentamente.

**RF-002 (critical) — il pavimento di contrazione.** Il tetto governava le
ammissioni e nulla governava le uscite. Una coalizione a `k > V/3`, sotto la
soglia di safety, faceva finalizzare le proprie candidature e poi negava il
quorum ai blocchi che ne contenessero altre: al confine `R = C =` coalizione,
`fills = 0` — **sotto il tetto, non al tetto** — e il 100 % del potere senza
ammettere nessuno. Chiuso con
**`3 * member_count(nuovo) > 2 * member_count(precedente)`**, la forma del
predicato di quorum applicata ai seggi anziché al potere, valida ai confini
**e** nelle transizioni di sola rimozione. Effetto verificato: a `k` appena sopra
`V/3` il set è invalido e la catena si ferma; contrarre fino a sé stessi resta
possibile solo sopra i due terzi, dove la safety BFT è già persa. **La soglia
effettiva di cattura torna a 2/3.** Corrette insieme alla regola, come chiesto,
le tre affermazioni che la presupponevano: la disuguaglianza `ceil((V/3)/c)`
qualificata «per ammissione» in `ledger.md` e in `AT-10`; la frase sulla
contrazione in §"Revocation" ora vera *perché* esiste il pavimento, con la
conversione in **concentrazione** nominata; l'invariante 8 che non promette più
che il quorum sia impotente sulla composizione, ma che non può nominare i membri
e che ciò che lo limita è il pavimento.

**RF-003 (high) — la seconda leva sul seme.** L'affermazione «un insieme
impegnato che il macinatore non controlla» è **ritrattata** e conservata solo
come citazione di ciò che si ritratta. Oltre alla dichiarazione ho preso la
riduzione che AGENT-007 indicava come naturale senza imporla: `candidate_root` e
`candidate_count` **escono dal preimmagine del seme** e restano legati per
validità, cosa che i full node già facevano. Aggiunta anche l'analisi di ordine
che è l'unica difesa realmente presente: `candidacy_close_height(e)` sta sotto la
finestra di entropia, quindi la scelta dei sottoinsiemi è **cieca** rispetto ai
biglietti. Dichiarato il residuo che resta: l'**esclusione** per non
finalizzazione, limitata nei suoi effetti dal tetto e dal pavimento e **non
falsificabile da nessuno** — nemmeno da un full node — perché una candidatura
censurata e una mai inviata sono la stessa assenza di transazione.

**RF-004 (high) — soglia binaria alimentata da evidenza macinabile.** Ho preso la
variante (i) *oltre* alla (ii), avendo verificato che **non tocca il disegno
delle challenge di [ADR-002]**: è una condizione di conteggio su dati già
finalizzati, perché `challenge_evidence` porta già `issuer_node_id`. Nuova
condizione di eleggibilità 4: almeno `validator_eligibility_min_issuers`
emittenti **distinti**, mai il soggetto. Nessun debito proposto, perché il
perimetro di [ADR-002] non è toccato. Dichiarato che alza il prezzo e **non**
chiude il residuo — il costo diventa `min_issuers` identità colluse per candidato
fabbricato, che è di nuovo un costo una tantum contro un flusso perpetuo, cioè
l'affermazione strutturale di [ADR-007]. Spiegato perché la copertura a due
emittenti di `wire.md` non trasferisce: funziona per un *tasso di rilevamento*,
non per una **somma di successi** che non sottrae nulla. Corretta la frase del
punto 3 dei *Declared limits* di `identity.md` e ripuntati i suoi due rimandi
alle sezioni giuste.

**RF-005 (medium) — cooldown evadibile.** Variante (i): la condizione di
eleggibilità 5 copre ora l'uscita da un seggio **per qualunque ragione**, non la
sola scadenza del mandato. Riportata l'aritmetica che rendeva l'evasione
dominante e non marginale (`(T-1)/T > T/(T+k)` per ogni `k >= 2`), perché chi
tara il parametro sappia da che cosa è protetto. Coperta l'interazione nei casi
degeneri: molti che escono insieme entrano insieme in cooldown, e insieme al
pavimento di contrazione è il bordo di liveness più stretto della sezione;
rifiutata la deroga al cooldown quando il pool è corto, per la stessa ragione per
cui è rifiutata la continuazione d'emergenza — è fabbricabile da chi censura.

**RF-006 (low) — provenienza dei parametri nel passo 5.** Il passo ora nomina le
tre fonti nell'ordine in cui vanno usate: `ElectionBounds` dalla distribuzione
firmata (trust anchor, mai da un peer), il documento `consensus_parameters`
autenticato ricalcolandone l'hash contro il `consensus_parameters_hash`
dell'header già fidato e verificandone il quorum, e il controllo dei parametri
contro i bound. **Fallimento chiuso** dichiarato esplicitamente: niente default,
niente valori di un documento precedente, niente valori da un peer.

**Forma difendibile del claim.** Le due frasi mancanti sono state aggiunte come
citazione a blocco in `ledger.md` §"What a light client can establish", con la
qualificazione «entro i limiti di parametro fissati alla genesi» e la distinzione
fra chi **entra** e chi **esce**, più il resoconto esplicito dei due
sovrannunci precedenti e del perché ora il claim regge senza condizione a fianco.

### Files changed in this remediation

- `docs/protocol/ledger.md` — invariante 8 qualificato; punto 10 (pavimento di
  contrazione) nelle transizioni di sola rimozione e frase sulla contrazione
  corretta; condizioni di eleggibilità 4 (diversità di emittente) e 5 (cooldown
  su qualunque uscita) con la loro motivazione; §"Declared limit, inherited and
  not created here" sul residuo di macinatura ereditato; formula del seme senza
  `candidate_root`/`candidate_count` e nuova §"The second lever"; passi 3, 4 e 7
  della derivazione; nota sul legame per validità in `ElectionRecord`; due nuove
  sottosezioni in §"Rotation" (pavimento di contrazione, magnitudini di genesi) e
  blocco di vincoli esteso; nuovo caso degenere sulle uscite di massa; elenchi
  del light client estesi (due nuove voci "può", due nuove "non può", (g) e (h));
  claim riscritto; esempio numerico aggiornato con la controprova del pavimento;
  passo 5 dell'algoritmo light-client riscritto.
- `docs/protocol/README.md` — preimmagine di `election_seed`; §"Election bounds"
  nuova, con `ElectionBounds`, il suo stato di configurazione e il limite
  dichiarato; `RewardPolicyBody` +1 campo; **ricalcolo di `policy_hash` `PD-0`**
  e nuova eccezione di valore del fixture; righe `ELEC-0` di `election_seed` e
  `election_ticket` ricalcolate; §"DRAFT" aggiornata.
- `docs/protocol/identity.md` — punto 3 dei *Declared limits* corretto e
  ripuntato.
- `.lmbrain/knowledge/threat-model.md` — `TM-18` con le due porte e la loro
  chiusura, più i residui (iv) e (v) e la condizione sull'esistenza di
  `ElectionBounds`; `SEC-REQ-13` con la dipendenza da `SEC-REQ-14` registrata;
  `TM-09` con il residuo di RF-004; `AT-09` con la conversione in concentrazione;
  `AT-10` con la disuguaglianza qualificata «per ammissione» e le tre
  configurazioni di test, fra cui quella che distingue le due versioni della
  regola.

### Verification performed in this remediation

Stesso metodo della consegna precedente, esteso: **il metodo JCS è stato
rivalidato su due fixture non modificate** (`parameter_set_hash`,
`hosting_rate_card_hash`) prima di ricalcolare `policy_hash` `PD-0`, che questa
remediation cambia perché aggiunge un campo al corpo. `consensus_parameters_hash`
**non** cambia: i vincoli nuovi non introducono parametri di consenso, perché il
pavimento di contrazione non ha parametro — è `2/3` — e i bound stanno nella
genesi e non nel documento.

Sono stati inoltre ricalcolati `election_seed` e i tre biglietti, con l'esito che
l'ordine dei biglietti cambia e la composizione dell'esempio con esso: i seggi
riempiti sono ora `06` e `08`, ed è `05` a restare fuori per il tetto. La
derivazione è stata rifatta in modo indipendente e confrontata con la tabella
scritta. È stata verificata la controprova del pavimento sull'esempio stesso
(`3*2 > 2*4` falso) e su cinque coppie `(V, k)`, più la proprietà che il
pavimento equivale esattamente a `k > 2V/3`.

Ventisei controlli testuali verificano che ogni regola nuova sia presente e che
ogni affermazione ritrattata sia sparita — inclusi due controlli di non
regressione sulle correzioni dei round precedenti.

```text
== 1. JCS method validated against UNCHANGED published fixtures ==
  parameter_set_hash reproduces published value            PASS
  hosting_rate_card_hash reproduces published value        PASS
== 2. PD-0 fixtures whose bodies THIS CHANGE extends ==
  PASS policy_hash PD-0                   sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d
  PASS consensus_parameters_hash PD-0     sha256:ab9886270886063d9f331fad63a8371fe6f00ff80ec609bdfbe26e4ee21f4f06
  RewardPolicyBody keys match hashed body (miss=[] extra=[])             PASS
  ConsensusParametersBody keys match hashed body (miss=[] extra=[])      PASS
== 3. Worked example, recomputed from the preimages stated in ledger.md ==
  PASS candidate_leaf 02                  cd3950bac9c60b73523a0f8157e5da8384515d2e9c4ef4127be2d910b878158c
  PASS candidate_leaf 04                  004e3b02570032774d181a20dbb4d5e23f6fe83092374635189f42c513db97d9
  PASS candidate_leaf 05                  154a73c742c2f580aafaf97401ef5633898862d761fc13ed230db7817b0111fd
  PASS candidate_leaf 06                  9ecce0184a6b8d6d3a1257add0af3a16cc64809fe2cf723cfe91b570abe07f44
  PASS candidate_leaf 08                  c9e67f59e0d4a6c08cc588bb906b17c8bddba657def06d6c2f111c64420796ca
  PASS candidate_empty H(0x42)            df7e70e5021544f4834bbee64a9e3789febc4be81470df629cad6ddb03320a5c
  PASS internal node 0                    00d36c1eb2fc336cb635735bbae46cf3e2a32322f4761db3d29a297644e45bb2
  PASS internal node 1                    db1bd3372cea52438df608862c3dcb3c0a2b1c16b152c296c2a8effe9005bc20
  PASS internal node 2                    b65c2a34ededd92d98721c11a620cc6f16e87a03f512e54fd35b35cd70c3bf39
  PASS internal node 3                    a7ee32ed571f99897d698653a14d292cfea8e8633f95a9db217ea993c7773e91
  PASS internal node 4                    a2bb9ac490112abad3c2af7197fdf6efe62af01b7c251f8d3443b20fa145a060
  PASS internal node 5                    5d426fa91db2b6acb77c0980837dea9e54380afc25eede062e05e2cab2dc6c2b
  PASS candidate_root                     42e4f6b1f01af3b69ba154aa464738829635a3ed7facf65e652d9712b461924a
  PASS election_entropy                   29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
  PASS election_entropy (README)          29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
  PASS election_seed                      9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85
  PASS election_seed (README)             9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85
  PASS election_ticket 05                 a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21
  PASS election_ticket 06                 547132161f56f2361faf3caae90224e1b5c04ae1986ab871703b7003693c5fdd
  PASS election_ticket 08                 9d04ef2f66f5403d275fa1f80819ce40a17af81931d263d3c390e0291e0b19c9
  PASS election_ticket (README ELEC-0)    a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21
  old candidate-bound seed no longer published             PASS
== 4. Worked example, derivation steps redone independently ==
  computed: {'retained': [2, 4], 'C': [2, 4, 5, 6, 8], 'candidate_count': 5, 'pool': [5, 6, 8], 'ticket_order': [6, 8, 5], 'fills': 2, 'elected': [2, 4, 6, 8], 'contraction_ok': True}
  document: {'retained': [2, 4], 'C': [2, 4, 5, 6, 8], 'candidate_count': 5, 'pool': [5, 6, 8], 'ticket_order': [6, 8, 5], 'fills': 2, 'elected': [2, 4, 6, 8], 'contraction_ok': True}
  derivation matches the table in ledger.md                PASS
  censored-pool counterfactual (R alone) fails the floor   PASS
== 5. Parameter constraint block on the consensus PD-0 fixture ==
  0 < min <= V <= max                                      PASS
  election_entropy_blocks >= 2                             PASS
  candidacy_close > entropy                                PASS
  election_epoch_blocks > candidacy_close                  PASS
  ceil(V/T) <= c                                           PASS
  3*c*m <= V                                               PASS
  derived: T >= 3*m                                        PASS
  validator_eligibility_min_issuers >= 2                   PASS
== 6. Contraction floor: the RF-002 attack is now rejected ==
  V=  6 k=  3 -> set INVALID (stall)                       PASS
  V=  6 k=  4 -> set INVALID (stall)                       PASS
  V=  6 k=  5 -> set valid                                 PASS
  V=100 k= 34 -> set INVALID (stall)                       PASS
  V=100 k= 67 -> set valid                                 PASS
  floor implies effective capture threshold is 2/3         PASS
== 7. Remediation rules and retractions present in the documents ==
  RF-001 genesis magnitude bounds in constraint block        PASS
  RF-001 rate-of-change rule in constraint block             PASS
  RF-001 ElectionBounds defined in README                    PASS
  RF-001 ElectionBounds is not chain state                   PASS
  RF-002 contraction floor stated in ledger                  PASS
  RF-002 floor applied at derivation step 7                  PASS
  RF-002 floor applied to removal-only transitions           PASS
  RF-002 capture inequality qualified as by-admission        PASS
  RF-002 revocation sentence corrected (concentration named) PASS
  RF-003 absolute grinder claim retracted                    PASS
  RF-003 absolute claim survives only as a quoted retraction PASS
  RF-003 seed no longer binds candidate_root                 PASS
  RF-004 issuer diversity is an eligibility condition        PASS
  RF-004 inherited grinding residual declared                PASS
  RF-004 identity.md overstated claim removed                PASS
  RF-004 identity.md repointed at the election section       PASS
  RF-005 cooldown covers any departure                       PASS
  RF-006 step 5 names the parameter source                   PASS
  RF-006 step 5 fails closed                                 PASS
  claim form carries the genesis-limits qualification        PASS
  claim form carries the entry/exit qualification            PASS
  light client can check the contraction floor               PASS
  light client cannot distinguish attrition (g)              PASS
  light client cannot see non-finalized exclusion (h)        PASS
  old gap sentence still absent                              PASS
  REVIEW-009 fix still in place                              PASS

checks passed: 70   failed: 0
```

Ancore interne dei quattro documenti di `docs/protocol/`, rieseguito dopo ogni
gruppo di modifiche:

```text
broken anchors: 0
```

### Deviations and judgement calls in this remediation

1. **RF-004: presa anche la variante (i), senza proporre debito.** Il Lead mi
   lasciava proporre un debito se la mitigazione strutturale fosse uscita dal
   perimetro. Ho verificato che non ne esce: la diversità di emittente è una
   condizione di conteggio su `issuer_node_id` già presente nell'evidenza
   finalizzata, e non cambia come le challenge vengono emesse, assegnate o
   verificate. [ADR-002] non è toccato. Un debito qui sarebbe stato un rinvio
   senza motivo.
2. **RF-003: presa la riduzione oltre alla dichiarazione.** AGENT-007 non la
   imponeva. L'ho presa perché rende la posizione del documento enunciabile senza
   dipendere da un argomento di ordine sottile, e perché il legame per validità è
   più forte del legame per hash. Costo: ricalcolo di `election_seed`, dei tre
   biglietti e dell'esito dell'esempio.
3. **Pavimento di contrazione senza nuovo parametro.** Fra le due forme proposte
   ho scelto la regola per transizione `3*nuovo > 2*vecchio` invece di un
   `departure_cap` governato, per tre ragioni: non aggiunge un parametro che il
   quorum firma (che sarebbe stato RF-001 di nuovo); riusa una forma aritmetica
   che il documento già definisce e di cui un revisore conosce i casi limite; e
   la sua conseguenza — soglia effettiva di cattura a 2/3 — è dimostrabile in una
   riga invece di dipendere da una taratura.
4. **Nessun campo nuovo di `BlockHeader` e nessuna modifica al checkpoint.** La
   provenienza dei parametri per il light client passa dal
   `consensus_parameters_hash` che l'header porta già, quindi il fixture `WSC-0`
   e lo schema del checkpoint non sono toccati.

### Remediation REVIEW-010, secondo giro (RF-007..RF-011)

Due dei quattro finding sono conseguenze dirette delle correzioni del primo giro:
RF-007 nasce dal pavimento di contrazione, RF-010 dal rimedio a RF-005. Sono
registrati come tali e non come difetti indipendenti.

**RF-007 (critical) — la coorte di genesi fermava ogni rete conforme.** Limite di
mandato, tetto di ingressi e pavimento erano **congiuntamente insoddisfacibili**
al confine `e = T`: il set di genesi porta mandati sincroni, quindi scadono tutti
insieme, `R` è vuoto, il nuovo set vale al più `c`, e il pavimento pretende
`3c > 2V` mentre il vincolo di cattura pretende `3cm <= V` — intervallo vuoto per
ogni `V`. Verificato in simulazione sui parametri del fixture: arresto certo
all'altezza `T * election_epoch_blocks`.

Ho **rifiutato** l'esenzione del pavimento a `R` vuoto, per la ragione che il Lead
segnalava e che è già la mia in due punti del documento: è fabbricabile da chi
controlla l'inclusione. Corretta invece la causa, la **sincronizzazione**:

- ogni voce porta ora `term_expiry_epoch`, **timbrato all'insediamento**
  (`e + T`) e mai ricalcolato;
- il set di genesi deve essere **scaglionato**: valori in `[1, T]`, al più
  `validator_churn_cap_seats` voci per valore. Un set di genesi che violi la
  regola non è un trust anchor valido;
- da lì la proprietà si mantiene da sola, perché le scadenze al confine `e` sono
  i timbri scritti al confine `e - T`, e per confine se ne scrivono al più `c`;
- aggiunto `3 * c < V` al blocco di validità: garantisce che un confine in cui
  scade un'intera coorte e **non viene insediato nessuno** produca comunque un
  set valido.

Il timbro chiude anche un difetto che nessuno aveva sollevato: con la forma
derivata `e - seated_since_epoch < T`, un quorum che alzasse `T` entro il tetto di
genesi **estendeva retroattivamente i mandati dei propri membri in carica**. Con
il timbro, un cambio di `T` governa solo i seggi insediati dopo.

Come chiesto, i vincoli sono resi **congiuntamente soddisfacibili nel blocco**:
`ceil(V/T) <= c < V/3` richiede `T >= 4`, e con `T >= 3m` si ottiene
`T >= max(4, 3m)`. Ne segue che **`T <= 3` è insoddisfacibile a ogni `V`** —
verificato per forza bruta su `V` da 1 a 59 — e il fixture `PD-0` del progetto,
che usava `T = 3`, era esso stesso inammissibile. Nuovi valori `V=12, T=4, c=3`,
la più piccola istanza che soddisfa tutto, con `consensus_parameters_hash`
ricalcolato.

**RF-008 (high) — l'affermazione sui due terzi era falsa, e il documento
conteneva la propria confutazione.** Ritratta in tutti e quattro i punti. La
censura **selettiva** porta `V` a `2V/3`, poi a `4V/9`, fino a `k`, in
`ceil(log(V/k)/log(3/2))` confini, tre per `k` vicino a `V/3`, e i nodi onesti
firmano ogni blocco perché ogni blocco è valido. **La soglia effettiva resta poco
sopra un terzo.** Il documento ora rivendica ciò che il pavimento compra davvero —
un confine invisibile convertito in tre, ognuno dei quali pubblica la propria
contrazione — sullo stesso standard del tetto di ingressi, e dichiara
l'asimmetria: orizzonte per ammissione **tarabile** con `m`, orizzonte per attrito
**fisso**, e la sicurezza di una regola è quella del suo percorso più debole. Il
punto peggiore, l'esito atteso della configurazione 2 di `AT-10`, è ora spezzato
in 2a (censura totale, che il pavimento rifiuta) e 2b (censura selettiva, tre
confini), con nota esplicita che il criterio precedente sarebbe stato smentito
dalla simulazione e attribuito all'implementazione invece che alla specifica.

**RF-009 (medium).** Aggiunto `election_parameter_min_activation_gap_blocks` a
`ElectionBounds` e la regola di spaziatura sulle `activation_height`. Dichiarato
che il tetto assoluto reggeva già, e che ciò che mancava era la proprietà
rivendicata: un processo che nessuno ha il tempo di osservare è un evento con più
scartoffie.

**RF-010 (medium).** Aggiunto `validator_cooldown_epochs <= T`, a costo zero
perché `T` è già vincolato. Registrata la ragione per cui questa magnitudine
appartiene all'elenco: è l'unica il cui **aumento aiuta un avversario**, perché
censurare un onesto per un'epoca lo tiene fuori `1 + cooldown` epoche.

**RF-011 (medium).** Controllo 7 della lista normativa corretto: il seme è hash
dei soli ID di blocco della finestra, e `candidate_root` con `candidate_count`
sono impegnati dal record ma **non** sono input del seme.

### Files changed in this second remediation

- `docs/protocol/ledger.md` — `term_expiry_epoch` nello schema delle voci, nel
  punto 7 delle transizioni di sola rimozione, nei passi 1 e 6 della derivazione e
  nei controlli 4, 5 e 7 del light client; nuova §"The genesis cohort, and why its
  terms must be staggered"; limite di mandato riscritto come timbro con la
  motivazione anti-estensione retroattiva; blocco di vincoli con `3c < V`,
  `cooldown <= T`, spaziatura delle attivazioni e il riquadro di soddisfacibilità
  congiunta; §"The contraction floor" con la ritrattazione dei due terzi,
  l'aritmetica dell'attrito e l'asimmetria dichiarata; §"Revocation" e il claim
  allineati; esempio numerico con parametri ammissibili e colonne di scadenza.
- `docs/protocol/README.md` — `ElectionBounds` con la spaziatura minima e la sua
  motivazione; valori e **hash ricalcolato** del fixture consensus `PD-0`, con la
  nota che spiega perché il set è più grande di quanto un fixture richiederebbe.
- `.lmbrain/knowledge/threat-model.md` — `TM-18` con la terza interazione
  (RF-007) e la ritrattazione dei due terzi con l'asimmetria; `AT-10`
  configurazione 2 spezzata in 2a e 2b con gli esiti attesi corretti.

### Verification performed in this second remediation

Metodo invariato e rivalidato: `parameter_set_hash` e `hosting_rate_card_hash`,
**non toccati**, riprodotti esattamente prima di ricalcolare
`consensus_parameters_hash`. `policy_hash` e i valori `ELEC-0` non cambiano in
questo giro. L'esempio numerico conserva tutti i digest, perché il seme dipende
dalla sola finestra di entropia: cambiano i parametri e le colonne di scadenza,
non le preimmagini.

Due verifiche nuove sono **simulazioni** e non controlli testuali, perché i due
finding bloccanti sono affermazioni su comportamento e non su testo:

- **RF-007**: simulazione della successione dei set su 30 confini. Genesi
  sincrona con `V=12, T=4, c=3` si ferma al confine 4, che riproduce il difetto;
  genesi scaglionata non si ferma mai, su quattro combinazioni ammissibili di
  `(V, T, c)`. Verificato separatamente che `3c < V` è ciò che regge il pavimento
  a riempimenti nulli, e per forza bruta che `T <= 3` non ammette alcun `c` valido.
- **RF-008**: orizzonte di attrito **calcolato** invece che asserito, applicando
  ripetutamente il pavimento. Tre coppie `(V, k)` danno da tre a quattro confini,
  e il controllo decisivo è che la contrazione **riesca** sotto i due terzi — cioè
  che la vecchia affermazione fosse falsa — invece di limitarsi a controllare che
  il testo sia cambiato.

```text
== 1. JCS method validated against UNCHANGED published fixtures ==
  parameter_set_hash reproduces published value            PASS
  hosting_rate_card_hash reproduces published value        PASS
== 2. PD-0 fixtures whose bodies THIS CHANGE extends ==
  PASS policy_hash PD-0                   sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d
  PASS consensus_parameters_hash PD-0     sha256:840dd6a980a6350b4879c60f8581466165125408a62839d67468c32ca3f0c33f
  RewardPolicyBody keys match hashed body (miss=[] extra=[])             PASS
  ConsensusParametersBody keys match hashed body (miss=[] extra=[])      PASS
== 3. Worked example, recomputed from the preimages stated in ledger.md ==
  PASS candidate_leaf 02                  cd3950bac9c60b73523a0f8157e5da8384515d2e9c4ef4127be2d910b878158c
  PASS candidate_leaf 04                  004e3b02570032774d181a20dbb4d5e23f6fe83092374635189f42c513db97d9
  PASS candidate_leaf 05                  154a73c742c2f580aafaf97401ef5633898862d761fc13ed230db7817b0111fd
  PASS candidate_leaf 06                  9ecce0184a6b8d6d3a1257add0af3a16cc64809fe2cf723cfe91b570abe07f44
  PASS candidate_leaf 08                  c9e67f59e0d4a6c08cc588bb906b17c8bddba657def06d6c2f111c64420796ca
  PASS candidate_empty H(0x42)            df7e70e5021544f4834bbee64a9e3789febc4be81470df629cad6ddb03320a5c
  PASS internal node 0                    00d36c1eb2fc336cb635735bbae46cf3e2a32322f4761db3d29a297644e45bb2
  PASS internal node 1                    db1bd3372cea52438df608862c3dcb3c0a2b1c16b152c296c2a8effe9005bc20
  PASS internal node 2                    b65c2a34ededd92d98721c11a620cc6f16e87a03f512e54fd35b35cd70c3bf39
  PASS internal node 3                    a7ee32ed571f99897d698653a14d292cfea8e8633f95a9db217ea993c7773e91
  PASS internal node 4                    a2bb9ac490112abad3c2af7197fdf6efe62af01b7c251f8d3443b20fa145a060
  PASS internal node 5                    5d426fa91db2b6acb77c0980837dea9e54380afc25eede062e05e2cab2dc6c2b
  PASS candidate_root                     42e4f6b1f01af3b69ba154aa464738829635a3ed7facf65e652d9712b461924a
  PASS election_entropy                   29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
  PASS election_entropy (README)          29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
  PASS election_seed                      9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85
  PASS election_seed (README)             9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85
  PASS election_ticket 05                 a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21
  PASS election_ticket 06                 547132161f56f2361faf3caae90224e1b5c04ae1986ab871703b7003693c5fdd
  PASS election_ticket 08                 9d04ef2f66f5403d275fa1f80819ce40a17af81931d263d3c390e0291e0b19c9
  PASS election_ticket (README ELEC-0)    a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21
  old candidate-bound seed no longer published             PASS
== 4. Worked example, derivation steps redone independently ==
  computed: {'retained': [2, 4], 'C': [2, 4, 5, 6, 8], 'candidate_count': 5, 'pool': [5, 6, 8], 'ticket_order': [6, 8, 5], 'fills': 2, 'elected': [2, 4, 6, 8], 'contraction_ok': True}
  document: {'retained': [2, 4], 'C': [2, 4, 5, 6, 8], 'candidate_count': 5, 'pool': [5, 6, 8], 'ticket_order': [6, 8, 5], 'fills': 2, 'elected': [2, 4, 6, 8], 'contraction_ok': True}
  derivation matches the table in ledger.md                PASS
  censored-pool counterfactual (R alone) fails the floor   PASS
== 5. Parameter constraint block on the consensus PD-0 fixture ==
  0 < min <= V <= max                                      PASS
  election_entropy_blocks >= 2                             PASS
  candidacy_close > entropy                                PASS
  election_epoch_blocks > candidacy_close                  PASS
  ceil(V/T) <= c                                           PASS
  3*c*m <= V                                               PASS
  3*c < V (floor survives a full cohort)                   PASS
  validator_cooldown_epochs <= T                           PASS
  derived: T >= max(4, 3*m)                                PASS
  T <= 3 is unsatisfiable at every V                       PASS
  validator_eligibility_min_issuers >= 2                   PASS
== 6. Contraction floor: the RF-002 attack is now rejected ==
  V=  6 k=  3 -> set INVALID (stall)                       PASS
  V=  6 k=  4 -> set INVALID (stall)                       PASS
  V=  6 k=  5 -> set valid                                 PASS
  V=100 k= 34 -> set INVALID (stall)                       PASS
  V=100 k= 67 -> set valid                                 PASS
  floor implies effective capture threshold is 2/3         PASS
== 7. Remediation rules and retractions present in the documents ==
  RF-001 genesis magnitude bounds in constraint block        PASS
  RF-001 rate-of-change rule in constraint block             PASS
  RF-001 ElectionBounds defined in README                    PASS
  RF-001 ElectionBounds is not chain state                   PASS
  RF-002 contraction floor stated in ledger                  PASS
  RF-002 floor applied at derivation step 7                  PASS
  RF-002 floor applied to removal-only transitions           PASS
  RF-002 capture inequality qualified as by-admission        PASS
  RF-002 revocation sentence corrected (concentration named) PASS
  RF-003 absolute grinder claim retracted                    PASS
  RF-003 absolute claim survives only as a quoted retraction PASS
  RF-003 seed no longer binds candidate_root                 PASS
  RF-004 issuer diversity is an eligibility condition        PASS
  RF-004 inherited grinding residual declared                PASS
  RF-004 identity.md overstated claim removed                PASS
  RF-004 identity.md repointed at the election section       PASS
  RF-005 cooldown covers any departure                       PASS
  RF-006 step 5 names the parameter source                   PASS
  RF-006 step 5 fails closed                                 PASS
  claim form carries the genesis-limits qualification        PASS
  claim form carries the entry/exit qualification            PASS
  light client can check the contraction floor               PASS
  light client cannot distinguish attrition (g)              PASS
  light client cannot see non-finalized exclusion (h)        PASS
  RF-007 term_expiry_epoch in the entry schema               PASS
  RF-007 genesis stagger is a normative rule                 PASS
  RF-007 the R-empty exemption is explicitly refused         PASS
  RF-007 3*c < V in the constraint block                     PASS
  RF-007 joint satisfiability T >= max(4, 3m) stated         PASS
  RF-007 stamped term blocks retroactive extension           PASS
  RF-008 two-thirds claim retracted                          PASS
  RF-008 old two-thirds claim text gone                      PASS
  RF-008 asymmetry tunable vs fixed declared                 PASS
  RF-008 AT-10 expected outcome corrected                    PASS
  RF-009 minimum activation gap rule                         PASS
  RF-010 cooldown bounded by T                               PASS
  RF-011 light-client check 7 corrected                      PASS
  old gap sentence still absent                              PASS
  REVIEW-009 fix still in place                              PASS
== 8. RF-007: the genesis cohort no longer stalls the chain ==
  synchronized genesis stalls at e=T (the RF-007 defect)   PASS
  staggered genesis never stalls over 30 boundaries        PASS
  no stall for V=12 T=4 c=3 over 30 boundaries             PASS
  no stall for V=24 T=6 c=5 over 30 boundaries             PASS
  no stall for V=100 T=10 c=20 over 30 boundaries          PASS
  no stall for V=8 T=4 c=2 over 30 boundaries              PASS
  3c < V holds the floor with zero fills (V=12, c=3)       PASS
  3c = V does not (V=12, c=4)                              PASS
  T <= 3 admits no valid c at any V                        PASS
== 9. RF-008: the attrition horizon, computed rather than asserted ==
  V=12 contracts to k=4 in 4 boundaries                    PASS
  V=100 contracts to k=34 in 3 boundaries                  PASS
  V=60 contracts to k=20 in 3 boundaries                   PASS
  capture by attrition is NOT blocked below two thirds     PASS

checks passed: 99   failed: 0
```

```text
broken anchors: 0
```

### Judgement calls in this second remediation

1. **Timbro invece di scaglionamento del solo `seated_since_epoch`.** La
   direzione indicata dal Lead era lo scaglionamento; scaglionare
   `seated_since_epoch` avrebbe richiesto valori anteriori alla genesi, non
   rappresentabili in `u64`, oppure un caso speciale nel confronto. Il campo di
   scadenza esplicito ottiene lo stesso effetto senza casi speciali, rende il
   controllo del light client una singola disuguaglianza su un solo documento, e
   in più chiude l'estensione retroattiva dei mandati.
2. **Il fixture `PD-0` è stato cambiato perché era inammissibile.** Non è una
   scelta estetica: `T = 3` non ammette alcun `c` valido, quindi il fixture
   pubblicato insegnava una forma impossibile. La nota in `README.md` lo dice.
3. **RF-008 chiuso con una ritrattazione e non con una regola nuova.** Una regola
   che impedisse la contrazione selettiva dovrebbe distinguere una candidatura
   censurata da una mai inviata, che il residuo (h) dichiara indistinguibile per
   ogni verificatore. Non esiste una regola onesta da scrivere qui; esiste una
   rivendicazione onesta, ed è quella che ho scritto.

### Remediation REVIEW-010, terzo giro (RF-012, RF-014, RF-015)

**RF-012 (high, bloccante) — il mio argomento di automantenimento valeva solo a
`T` costante.** L'aritmetica è quella indicata e l'ho verificata prima di
correggere: i timbri sono `e + T(e)`, quindi due confini distinti collidono **se e
solo se `T` diminuisce**, e ogni collisione mette più di una coorte sullo stesso
confine, che è l'unica cosa per cui `3c < V` non è dimensionato. Riprodotto in
simulazione con `V = 12`, `c = 3`, `T` sceso di un passo per confine da 12 a 4 —
ogni valore intermedio ammissibile, ogni documento dentro il rapporto e lo
spaziamento: `1 + 11`, `2 + 10`, … `8 + 4` valgono tutti 12, quindi nove seggi su
dodici scadono al confine 12, `R = 3`, `fills = 3`, `member_count = 6`, e
`3 * 6 > 2 * 12` è falso. **Senza avversario**, e con il pool pieno.

Ho scelto la **chiusura (i), monotonia**: `T_new >= T_active` in accettazione,
quindi su catena viva il limite di mandato non decresce mai. Non l'ho scelta per
semplicità ma perché ho verificato che **(ii) collassa su (i) se valutata in
accettazione**: fra accettazione e attivazione un seggio può essere insediato al
confine precedente con il `T` vecchio, quindi l'unica cosa che un controllo in
accettazione può garantire è `e_a + T_new > (e_a - 1) + T_old`, cioè
`T_new >= T_old`. La versione permissiva ha effetto solo se valutata
**all'attivazione** contro il set allora attivo, il che richiede un documento di
protocollo ad attivazione condizionata dallo stato di catena: un concetto che v0
non ha e che questa sezione non è il posto per introdurre.

Ho scritto quell'argomento nel documento **e** ho scritto che si tratta di un
rifiuto per costo e non di un'impossibilità, con il costo dichiarato: una porta a
senso unico su una grandezza rilevante per la sicurezza, per cui una rete che
parta con mandati troppo lunghi non può correggerli se non per la via fuori banda
riservata agli stalli. Se una versione futura introdurrà l'attivazione
condizionata, (ii) è disponibile ed è migliore.

Alzare `T` resta libero e **gratuito** grazie al timbro: un `T` più lungo governa
solo i seggi insediati dopo. È la seconda volta che il timbro si ripaga.

**RF-014 — motivazione corretta, decisione conservata.** Il rilievo è giusto e mi
riguarda come autore. Avevo scritto che non esiste una regola onesta da scrivere;
il pavimento **cumulativo** `3 * member_count(e) > 2 * member_count(e - m)`
esiste, è sano, non chiede mai *perché* un membro sia uscito — quindi non richiede
la distinzione che il residuo (h) dichiara impossibile — e si calcola su
`member_count` che il light client già conserva. Il documento ora lo espone per
esteso e lo rifiuta per il suo **costo in liveness**: una rete che si restringa
legittimamente di più di un terzo in `m` confini si fermerebbe, e più `m` è ampio
più attrito ordinario vieta. Ho aggiunto esplicitamente perché la distinzione
conta: un costo è un giudizio che una versione futura può rivedere quando il
compromesso cambia, un'impossibilità è una dimostrazione che dice al lettore
successivo di smettere di cercare. Era la seconda, ed era falsa.

**RF-015 (low) — chiuso senza cambiare il fixture.** La nota di `README.md` ora
dice il vero: `ceil(V/T) <= c < V/3` è insoddisfacibile per `T <= 3` a ogni `V`;
l'**istanza minima ammissibile è `V=4, T=4, c=1, m=1`**, e `V=3` è impossibile
perché `3c < 3` non vale per alcun `c >= 1`. Il fixture ne usa deliberatamente una
più grande, e la ragione scritta ora è quella giusta: con `c = 1` una coorte è un
seggio solo, quindi il tetto di ingressi non viene mai esercitato e con esso non
lo è l'interazione fra tetto, scaglionamento e pavimento che i vincoli esistono
per tenere coerente. `V=12, T=4, c=3` è la più piccola istanza che le esercita
tutte e tre.

**Riporto dell'osservazione sulle suite di conformità.** L'ho portata nel
documento invece che lasciarla alla memoria di chi scriverà i test, e nel punto in
cui chi scrive una suite guarda: `README.md` §"Hash conformance fixtures" ha ora
un paragrafo normativo che dice che i fixture di parametri non sono scelte libere,
che una combinazione vietata può sembrare del tutto ordinaria — `T = 3` è
l'esempio più chiaro ed è inammissibile a **ogni** dimensione di set — e che un
caso di prova costruito su una di queste non testa un valore diverso: **asserisce
un comportamento per uno stato in cui nessuna rete conforme può trovarsi**, e va
rimosso invece che aggiustato. Le suite devono validare i propri fixture di
parametri contro il blocco di vincoli prima di usarli.

**Clausola del claim.** Aggiunta **accanto** al paragrafo e non dentro, come
chiesto: le magnitudini che reggono la proprietà sono fissate alla genesi, e
quelle che si muovono si muovono sotto un rapporto, sotto uno spaziamento
misurato in altezze di catena e — per il limite di mandato — in una sola
direzione.

### Files changed in this third remediation

- `docs/protocol/ledger.md` — l'argomento di automantenimento nella §"The genesis
  cohort" ora nomina la condizione da cui dipende; nuova §"A term limit may not
  shrink" con l'aritmetica della collisione, l'istanza verificata, la regola di
  monotonia, la dimostrazione che (ii) collassa in accettazione e il rifiuto
  dichiarato come costo; `T_new >= T_active` nel blocco di vincoli; §"The
  contraction floor" con il pavimento cumulativo esposto e rifiutato per il suo
  costo in liveness, al posto dell'affermazione di impossibilità; clausola del
  claim accanto al paragrafo.
- `docs/protocol/README.md` — nota di conformità sui fixture di parametri;
  istanza minima ammissibile e ragione corretta della scelta del fixture.
- `.lmbrain/knowledge/threat-model.md` — `TM-18` con il corollario RF-012, la sua
  chiusura per monotonia e il costo dichiarato.

Nessun hash cambia in questo giro: nessuna formula, nessun corpo di documento
governato e nessun valore del fixture sono stati toccati.

### Verification performed in this third remediation

Le due affermazioni nuove sono su **comportamento**, quindi sono verificate per
simulazione e non per confronto testuale:

- **RF-012**: successione dei set su 60 confini con scaglionamento di genesi
  uniforme. La sequenza decrescente ammissibile si ferma al confine 12 con
  esattamente i numeri riportati sopra; le sequenze **non decrescenti**, costanti
  e crescenti, non si fermano mai. Verificato separatamente, per enumerazione su
  tutte le coppie `(e1, T1), (e2, T2)` in un intervallo, che **ogni** collisione
  di timbri ha `T2 < T1`, cioè che la condizione è necessaria e sufficiente e non
  soltanto sufficiente.
- **RF-015**: enumerazione di tutte le combinazioni `(V, T, c, m)` fino a 20 che
  soddisfano il blocco. `V = 3` non ne ammette nessuna, la minima è
  `(4, 4, 1, 1)`, e il fixture pubblicato è ammissibile.

Il difetto di RF-007 resta riprodotto come controllo di non regressione: la genesi
sincrona si ferma ancora al confine `e = T`, quella scaglionata no, su quattro
combinazioni ammissibili inclusa l'istanza minima `V=4, T=4, c=1`.

```text
== 1. JCS method validated against UNCHANGED published fixtures ==
  parameter_set_hash reproduces published value            PASS
  hosting_rate_card_hash reproduces published value        PASS
== 2. PD-0 fixtures whose bodies THIS CHANGE extends ==
  PASS policy_hash PD-0                   sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d
  PASS consensus_parameters_hash PD-0     sha256:840dd6a980a6350b4879c60f8581466165125408a62839d67468c32ca3f0c33f
  RewardPolicyBody keys match hashed body (miss=[] extra=[])             PASS
  ConsensusParametersBody keys match hashed body (miss=[] extra=[])      PASS
== 3. Worked example, recomputed from the preimages stated in ledger.md ==
  PASS candidate_leaf 02                  cd3950bac9c60b73523a0f8157e5da8384515d2e9c4ef4127be2d910b878158c
  PASS candidate_leaf 04                  004e3b02570032774d181a20dbb4d5e23f6fe83092374635189f42c513db97d9
  PASS candidate_leaf 05                  154a73c742c2f580aafaf97401ef5633898862d761fc13ed230db7817b0111fd
  PASS candidate_leaf 06                  9ecce0184a6b8d6d3a1257add0af3a16cc64809fe2cf723cfe91b570abe07f44
  PASS candidate_leaf 08                  c9e67f59e0d4a6c08cc588bb906b17c8bddba657def06d6c2f111c64420796ca
  PASS candidate_empty H(0x42)            df7e70e5021544f4834bbee64a9e3789febc4be81470df629cad6ddb03320a5c
  PASS internal node 0                    00d36c1eb2fc336cb635735bbae46cf3e2a32322f4761db3d29a297644e45bb2
  PASS internal node 1                    db1bd3372cea52438df608862c3dcb3c0a2b1c16b152c296c2a8effe9005bc20
  PASS internal node 2                    b65c2a34ededd92d98721c11a620cc6f16e87a03f512e54fd35b35cd70c3bf39
  PASS internal node 3                    a7ee32ed571f99897d698653a14d292cfea8e8633f95a9db217ea993c7773e91
  PASS internal node 4                    a2bb9ac490112abad3c2af7197fdf6efe62af01b7c251f8d3443b20fa145a060
  PASS internal node 5                    5d426fa91db2b6acb77c0980837dea9e54380afc25eede062e05e2cab2dc6c2b
  PASS candidate_root                     42e4f6b1f01af3b69ba154aa464738829635a3ed7facf65e652d9712b461924a
  PASS election_entropy                   29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
  PASS election_entropy (README)          29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
  PASS election_seed                      9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85
  PASS election_seed (README)             9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85
  PASS election_ticket 05                 a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21
  PASS election_ticket 06                 547132161f56f2361faf3caae90224e1b5c04ae1986ab871703b7003693c5fdd
  PASS election_ticket 08                 9d04ef2f66f5403d275fa1f80819ce40a17af81931d263d3c390e0291e0b19c9
  PASS election_ticket (README ELEC-0)    a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21
  old candidate-bound seed no longer published             PASS
== 4. Worked example, derivation steps redone independently ==
  computed: {'retained': [2, 4], 'C': [2, 4, 5, 6, 8], 'candidate_count': 5, 'pool': [5, 6, 8], 'ticket_order': [6, 8, 5], 'fills': 2, 'elected': [2, 4, 6, 8], 'contraction_ok': True}
  document: {'retained': [2, 4], 'C': [2, 4, 5, 6, 8], 'candidate_count': 5, 'pool': [5, 6, 8], 'ticket_order': [6, 8, 5], 'fills': 2, 'elected': [2, 4, 6, 8], 'contraction_ok': True}
  derivation matches the table in ledger.md                PASS
  censored-pool counterfactual (R alone) fails the floor   PASS
== 5. Parameter constraint block on the consensus PD-0 fixture ==
  0 < min <= V <= max                                      PASS
  election_entropy_blocks >= 2                             PASS
  candidacy_close > entropy                                PASS
  election_epoch_blocks > candidacy_close                  PASS
  ceil(V/T) <= c                                           PASS
  3*c*m <= V                                               PASS
  3*c < V (floor survives a full cohort)                   PASS
  validator_cooldown_epochs <= T                           PASS
  derived: T >= max(4, 3*m)                                PASS
  T <= 3 is unsatisfiable at every V                       PASS
  validator_eligibility_min_issuers >= 2                   PASS
== 6. Contraction floor: the RF-002 attack is now rejected ==
  V=  6 k=  3 -> set INVALID (stall)                       PASS
  V=  6 k=  4 -> set INVALID (stall)                       PASS
  V=  6 k=  5 -> set valid                                 PASS
  V=100 k= 34 -> set INVALID (stall)                       PASS
  V=100 k= 67 -> set valid                                 PASS
  floor implies effective capture threshold is 2/3         PASS
== 7. Remediation rules and retractions present in the documents ==
  RF-001 genesis magnitude bounds in constraint block        PASS
  RF-001 rate-of-change rule in constraint block             PASS
  RF-001 ElectionBounds defined in README                    PASS
  RF-001 ElectionBounds is not chain state                   PASS
  RF-002 contraction floor stated in ledger                  PASS
  RF-002 floor applied at derivation step 7                  PASS
  RF-002 floor applied to removal-only transitions           PASS
  RF-002 capture inequality qualified as by-admission        PASS
  RF-002 revocation sentence corrected (concentration named) PASS
  RF-003 absolute grinder claim retracted                    PASS
  RF-003 absolute claim survives only as a quoted retraction PASS
  RF-003 seed no longer binds candidate_root                 PASS
  RF-004 issuer diversity is an eligibility condition        PASS
  RF-004 inherited grinding residual declared                PASS
  RF-004 identity.md overstated claim removed                PASS
  RF-004 identity.md repointed at the election section       PASS
  RF-005 cooldown covers any departure                       PASS
  RF-006 step 5 names the parameter source                   PASS
  RF-006 step 5 fails closed                                 PASS
  claim form carries the genesis-limits qualification        PASS
  claim form carries the entry/exit qualification            PASS
  light client can check the contraction floor               PASS
  light client cannot distinguish attrition (g)              PASS
  light client cannot see non-finalized exclusion (h)        PASS
  RF-007 term_expiry_epoch in the entry schema               PASS
  RF-007 genesis stagger is a normative rule                 PASS
  RF-007 the R-empty exemption is explicitly refused         PASS
  RF-007 3*c < V in the constraint block                     PASS
  RF-007 joint satisfiability T >= max(4, 3m) stated         PASS
  RF-007 stamped term blocks retroactive extension           PASS
  RF-008 two-thirds claim retracted                          PASS
  RF-008 old two-thirds claim text gone                      PASS
  RF-008 asymmetry tunable vs fixed declared                 PASS
  RF-008 AT-10 expected outcome corrected                    PASS
  RF-009 minimum activation gap rule                         PASS
  RF-010 cooldown bounded by T                               PASS
  RF-011 light-client check 7 corrected                      PASS
  RF-012 monotonic term limit is a rule                      PASS
  RF-012 direction constraint in the block                   PASS
  RF-012 self-maintenance now names its condition            PASS
  RF-012 permissive variant refused with an argument         PASS
  RF-012 refusal marked as cost, not impossibility           PASS
  RF-014 cumulative floor named as existing and sound        PASS
  RF-014 old no-honest-rule motivation gone                  PASS
  RF-014 liveness cost is the stated reason                  PASS
  RF-015 true minimal instance stated                        PASS
  RF-015 real reason for the larger fixture                  PASS
  carry-over: conformance suites must validate parameter fixtures PASS
  claim clause added beside the paragraph                    PASS
  old gap sentence still absent                              PASS
  REVIEW-009 fix still in place                              PASS
== 8. RF-007 / RF-012: term stagger, cohort size and the term limit ==
  synchronized genesis stalls at e=T (the RF-007 defect)   PASS
  staggered genesis, constant T: no stall in 60 boundaries PASS
  no stall for V=12 T=4 c=3 over 60 boundaries             PASS
  no stall for V=24 T=6 c=5 over 60 boundaries             PASS
  no stall for V=100 T=10 c=20 over 60 boundaries          PASS
  no stall for V=4 T=4 c=1 over 60 boundaries              PASS
  3c < V holds the floor with zero fills (V=12, c=3)       PASS
  3c = V does not (V=12, c=4)                              PASS
  T <= 3 admits no valid c at any V                        PASS
== 9. RF-008: the attrition horizon, computed rather than asserted ==
  V=12 contracts to k=4 in 4 boundaries                    PASS
  V=100 contracts to k=34 in 3 boundaries                  PASS
  V=60 contracts to k=20 in 3 boundaries                   PASS
  capture by attrition is NOT blocked below two thirds     PASS
== 10. RF-012: a shrinking term limit desynchronizes the stamps ==
  every T in the shrinking schedule is itself admissible   PASS
  shrinking T stalls with no adversary (boundary 12)       PASS
  the stall is not a shortage of candidates: fills were capped, not starved PASS
  monotonic T (constant) never stalls                      PASS
  monotonic T (growing) never stalls                       PASS
  every stamp collision has T2 < T1 (collisions iff T decreases) PASS
== 11. RF-015: the minimal admissible parameter instance ==
  V=3 admits nothing                                       PASS
  smallest admissible instance is V=4 T=4 c=1 m=1          PASS
  the published fixture V=12 T=4 c=3 m=1 is admissible     PASS
  c=1 would not exercise the entry cap                     PASS

checks passed: 121   failed: 0
```

```text
broken anchors: 0
```

### Judgement calls in this third remediation

1. **Monotonia invece della regola permissiva, con dimostrazione.** Non è una
   preferenza: (ii) non è valutabile in accettazione e collassa su (i). Ho scritto
   la dimostrazione nel documento perché il prossimo lettore non rifaccia il
   ragionamento, e ho scritto accanto che diventa disponibile se v0 acquisterà
   l'attivazione condizionata — per non ripetere l'errore che RF-014 mi contesta.
2. **I numeri dell'esempio nel documento sono quelli della simulazione, non
   quelli della review.** La forma è identica (`R = 3`, `fills = 3`,
   `member_count = 6`, `18` contro `24`), ma la sequenza che la produce con uno
   scaglionamento di genesi uniforme è `T` da 12 a 4 di un passo per confine
   anziché `6 → 5 → 4`. Ho scritto la sequenza che ho eseguito, non quella che mi
   è stata riferita.
3. **Nessun fixture cambiato per RF-015.** Il fixture attuale è ammissibile ed
   esercita ciò che deve; ciò che era sbagliato era la ragione scritta accanto.

### Remediation REVIEW-010, quarto giro (RF-013, RF-016)

Due frasi, nessuna regola toccata, nessun hash cambiato. La review resta
`accepted`: questi rilievi non riaprono il verdetto.

**RF-013 — il residuo (g) nominava la variante sbagliata.** Preso alla lettera
era vero, e proprio per questo era peggio di un errore evidente: era l'unica
variante nominata nell'elenco che un lettore futuro citerà per sapere che cosa il
protocollo non promette, e §*What the floor does not buy* e la configurazione 2b
di `AT-10` avevano già stabilito che il vettore reale è un altro. Chi seguisse il
rimando avrebbe trovato le due voci in disaccordo, con la differenza appesa alla
parola «ogni». Ora (g) dice che la coalizione **selettiva** è ciò che il light
client non sa distinguere dall'attrito onesto, che raggiunge il proprio obiettivo
in `ceil(log(V/k)/log(3/2))` confini, e che ciò che il pavimento compra contro di
lei è che quei confini siano diversi e ognuno pubblicato — non che uno di essi sia
distinguibile. La censura **totale** resta nominata, ma per dire ciò che è: la
variante che il pavimento rifiuta, e non il vettore.

**RF-016 — il terzo superlativo, e l'ho introdotto io chiudendo il secondo.**
La frase «`V=12, T=4, c=3` è la più piccola istanza che esercita tutte e tre» era
falsa: con il criterio implicito `c >= 2`, da `3c < V` segue `V >= 7`, e
`V=7, T=4, c=2, m=1` è ammissibile ed esercita il tetto. L'ho verificato prima di
correggere e l'ho aggiunto ai controlli, così che il controesempio sia dimostrato
e non asserito — insieme al controllo che fra le istanze ammissibili con `c > 1`
ne esistono con `V < 12`, che è la forma generale della smentita.

La chiusura non è un terzo calcolo di minimo. Al fixture serve `c > 1`, e questo
è ciò che la nota dice adesso: `V=12, T=4, c=3` soddisfa il criterio ed è comodo,
**non è dichiarato minimo**, e un'istanza più piccola con `c > 1` è citata per
esteso. Le due affermazioni di impossibilità restano, perché quelle sono
dimostrate per esaurimento dello spazio dei parametri e la nota ora lo dice: `T <= 3`
non ammette alcun `c` a nessun `V`, e `V = 3` non ammette nulla.

Il pattern vale più delle due frasi, e lo registro perché è il mio: **tre
superlativi in tre giri, tutti e tre falsificati** — «la soglia effettiva torna a
due terzi», «non esiste una regola onesta da scrivere», «la più piccola istanza
che le esercita tutte e tre». Ogni volta la sostanza intorno reggeva; ogni volta
la frase prometteva una proprietà più forte di quella dimostrata. Un superlativo
in un documento normativo va dimostrato o non scritto, e le tre correzioni sono
ora tutte nel documento accanto a ciò che hanno sostituito, così che il pattern
sia leggibile da chi verrà dopo invece che solo dalla review.

### Files changed in this fourth remediation

- `docs/protocol/ledger.md` — residuo (g) riscritto sulla censura selettiva, con
  la totale riclassificata come la variante che il pavimento rifiuta.
- `docs/protocol/README.md` — nota del fixture: eliminata l'affermazione di
  minimalità non dimostrata, sostituita dal criterio reale `c > 1` con il
  controesempio `V=7, T=4, c=2, m=1`; le due impossibilità conservate e marcate
  come dimostrate per esaurimento.

Nessuna regola, nessuna formula, nessun corpo di documento governato e nessun
valore di fixture sono stati toccati: **nessun hash cambia in questo giro.**

### Verification performed in this fourth remediation

Cinque controlli testuali nuovi sulle due correzioni, e — per RF-016 —
**il controesempio dimostrato invece che asserito**: `V=7, T=4, c=2, m=1` è
verificato ammissibile contro il blocco di vincoli, e l'enumerazione conferma che
esistono istanze ammissibili con `c > 1` e `V < 12`, cioè che la minimalità
asserita era falsa e non soltanto non dimostrata. Tutti i controlli dei giri
precedenti restano eseguiti come non regressione, incluse le due simulazioni
(coorte di genesi e limite di mandato decrescente) e l'orizzonte di attrito
calcolato.

```text
== 1. JCS method validated against UNCHANGED published fixtures ==
  parameter_set_hash reproduces published value            PASS
  hosting_rate_card_hash reproduces published value        PASS
== 2. PD-0 fixtures whose bodies THIS CHANGE extends ==
  PASS policy_hash PD-0                   sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d
  PASS consensus_parameters_hash PD-0     sha256:840dd6a980a6350b4879c60f8581466165125408a62839d67468c32ca3f0c33f
  RewardPolicyBody keys match hashed body (miss=[] extra=[])             PASS
  ConsensusParametersBody keys match hashed body (miss=[] extra=[])      PASS
== 3. Worked example, recomputed from the preimages stated in ledger.md ==
  PASS candidate_leaf 02                  cd3950bac9c60b73523a0f8157e5da8384515d2e9c4ef4127be2d910b878158c
  PASS candidate_leaf 04                  004e3b02570032774d181a20dbb4d5e23f6fe83092374635189f42c513db97d9
  PASS candidate_leaf 05                  154a73c742c2f580aafaf97401ef5633898862d761fc13ed230db7817b0111fd
  PASS candidate_leaf 06                  9ecce0184a6b8d6d3a1257add0af3a16cc64809fe2cf723cfe91b570abe07f44
  PASS candidate_leaf 08                  c9e67f59e0d4a6c08cc588bb906b17c8bddba657def06d6c2f111c64420796ca
  PASS candidate_empty H(0x42)            df7e70e5021544f4834bbee64a9e3789febc4be81470df629cad6ddb03320a5c
  PASS internal node 0                    00d36c1eb2fc336cb635735bbae46cf3e2a32322f4761db3d29a297644e45bb2
  PASS internal node 1                    db1bd3372cea52438df608862c3dcb3c0a2b1c16b152c296c2a8effe9005bc20
  PASS internal node 2                    b65c2a34ededd92d98721c11a620cc6f16e87a03f512e54fd35b35cd70c3bf39
  PASS internal node 3                    a7ee32ed571f99897d698653a14d292cfea8e8633f95a9db217ea993c7773e91
  PASS internal node 4                    a2bb9ac490112abad3c2af7197fdf6efe62af01b7c251f8d3443b20fa145a060
  PASS internal node 5                    5d426fa91db2b6acb77c0980837dea9e54380afc25eede062e05e2cab2dc6c2b
  PASS candidate_root                     42e4f6b1f01af3b69ba154aa464738829635a3ed7facf65e652d9712b461924a
  PASS election_entropy                   29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
  PASS election_entropy (README)          29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
  PASS election_seed                      9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85
  PASS election_seed (README)             9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85
  PASS election_ticket 05                 a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21
  PASS election_ticket 06                 547132161f56f2361faf3caae90224e1b5c04ae1986ab871703b7003693c5fdd
  PASS election_ticket 08                 9d04ef2f66f5403d275fa1f80819ce40a17af81931d263d3c390e0291e0b19c9
  PASS election_ticket (README ELEC-0)    a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21
  old candidate-bound seed no longer published             PASS
== 4. Worked example, derivation steps redone independently ==
  computed: {'retained': [2, 4], 'C': [2, 4, 5, 6, 8], 'candidate_count': 5, 'pool': [5, 6, 8], 'ticket_order': [6, 8, 5], 'fills': 2, 'elected': [2, 4, 6, 8], 'contraction_ok': True}
  document: {'retained': [2, 4], 'C': [2, 4, 5, 6, 8], 'candidate_count': 5, 'pool': [5, 6, 8], 'ticket_order': [6, 8, 5], 'fills': 2, 'elected': [2, 4, 6, 8], 'contraction_ok': True}
  derivation matches the table in ledger.md                PASS
  censored-pool counterfactual (R alone) fails the floor   PASS
== 5. Parameter constraint block on the consensus PD-0 fixture ==
  0 < min <= V <= max                                      PASS
  election_entropy_blocks >= 2                             PASS
  candidacy_close > entropy                                PASS
  election_epoch_blocks > candidacy_close                  PASS
  ceil(V/T) <= c                                           PASS
  3*c*m <= V                                               PASS
  3*c < V (floor survives a full cohort)                   PASS
  validator_cooldown_epochs <= T                           PASS
  derived: T >= max(4, 3*m)                                PASS
  T <= 3 is unsatisfiable at every V                       PASS
  validator_eligibility_min_issuers >= 2                   PASS
== 6. Contraction floor: the RF-002 attack is now rejected ==
  V=  6 k=  3 -> set INVALID (stall)                       PASS
  V=  6 k=  4 -> set INVALID (stall)                       PASS
  V=  6 k=  5 -> set valid                                 PASS
  V=100 k= 34 -> set INVALID (stall)                       PASS
  V=100 k= 67 -> set valid                                 PASS
  floor implies effective capture threshold is 2/3         PASS
== 7. Remediation rules and retractions present in the documents ==
  RF-001 genesis magnitude bounds in constraint block        PASS
  RF-001 rate-of-change rule in constraint block             PASS
  RF-001 ElectionBounds defined in README                    PASS
  RF-001 ElectionBounds is not chain state                   PASS
  RF-002 contraction floor stated in ledger                  PASS
  RF-002 floor applied at derivation step 7                  PASS
  RF-002 floor applied to removal-only transitions           PASS
  RF-002 capture inequality qualified as by-admission        PASS
  RF-002 revocation sentence corrected (concentration named) PASS
  RF-003 absolute grinder claim retracted                    PASS
  RF-003 absolute claim survives only as a quoted retraction PASS
  RF-003 seed no longer binds candidate_root                 PASS
  RF-004 issuer diversity is an eligibility condition        PASS
  RF-004 inherited grinding residual declared                PASS
  RF-004 identity.md overstated claim removed                PASS
  RF-004 identity.md repointed at the election section       PASS
  RF-005 cooldown covers any departure                       PASS
  RF-006 step 5 names the parameter source                   PASS
  RF-006 step 5 fails closed                                 PASS
  claim form carries the genesis-limits qualification        PASS
  claim form carries the entry/exit qualification            PASS
  light client can check the contraction floor               PASS
  light client cannot distinguish attrition (g)              PASS
  light client cannot see non-finalized exclusion (h)        PASS
  RF-007 term_expiry_epoch in the entry schema               PASS
  RF-007 genesis stagger is a normative rule                 PASS
  RF-007 the R-empty exemption is explicitly refused         PASS
  RF-007 3*c < V in the constraint block                     PASS
  RF-007 joint satisfiability T >= max(4, 3m) stated         PASS
  RF-007 stamped term blocks retroactive extension           PASS
  RF-008 two-thirds claim retracted                          PASS
  RF-008 old two-thirds claim text gone                      PASS
  RF-008 asymmetry tunable vs fixed declared                 PASS
  RF-008 AT-10 expected outcome corrected                    PASS
  RF-009 minimum activation gap rule                         PASS
  RF-010 cooldown bounded by T                               PASS
  RF-011 light-client check 7 corrected                      PASS
  RF-012 monotonic term limit is a rule                      PASS
  RF-012 direction constraint in the block                   PASS
  RF-012 self-maintenance now names its condition            PASS
  RF-012 permissive variant refused with an argument         PASS
  RF-012 refusal marked as cost, not impossibility           PASS
  RF-014 cumulative floor named as existing and sound        PASS
  RF-014 old no-honest-rule motivation gone                  PASS
  RF-014 liveness cost is the stated reason                  PASS
  RF-015 impossibility statements kept and marked as proved  PASS
  RF-015 real reason for the larger fixture                  PASS
  carry-over: conformance suites must validate parameter fixtures PASS
  claim clause added beside the paragraph                    PASS
  RF-013 residual (g) names selective censorship             PASS
  RF-013 residual (g) marks total censorship as not the vector PASS
  RF-016 unproven minimality claim removed                   PASS
  RF-016 fixture states c > 1 as the requirement             PASS
  RF-016 counterexample recorded                             PASS
  old gap sentence still absent                              PASS
  REVIEW-009 fix still in place                              PASS
== 8. RF-007 / RF-012: term stagger, cohort size and the term limit ==
  synchronized genesis stalls at e=T (the RF-007 defect)   PASS
  staggered genesis, constant T: no stall in 60 boundaries PASS
  no stall for V=12 T=4 c=3 over 60 boundaries             PASS
  no stall for V=24 T=6 c=5 over 60 boundaries             PASS
  no stall for V=100 T=10 c=20 over 60 boundaries          PASS
  no stall for V=4 T=4 c=1 over 60 boundaries              PASS
  3c < V holds the floor with zero fills (V=12, c=3)       PASS
  3c = V does not (V=12, c=4)                              PASS
  T <= 3 admits no valid c at any V                        PASS
== 9. RF-008: the attrition horizon, computed rather than asserted ==
  V=12 contracts to k=4 in 4 boundaries                    PASS
  V=100 contracts to k=34 in 3 boundaries                  PASS
  V=60 contracts to k=20 in 3 boundaries                   PASS
  capture by attrition is NOT blocked below two thirds     PASS
== 10. RF-012: a shrinking term limit desynchronizes the stamps ==
  every T in the shrinking schedule is itself admissible   PASS
  shrinking T stalls with no adversary (boundary 12)       PASS
  the stall is not a shortage of candidates: fills were capped, not starved PASS
  monotonic T (constant) never stalls                      PASS
  monotonic T (growing) never stalls                       PASS
  every stamp collision has T2 < T1 (collisions iff T decreases) PASS
== 11. RF-015: the minimal admissible parameter instance ==
  V=3 admits nothing                                       PASS
  smallest admissible instance is V=4 T=4 c=1 m=1          PASS
  the published fixture V=12 T=4 c=3 m=1 is admissible     PASS
  c=1 would not exercise the entry cap                     PASS
  RF-016 counterexample V=7 T=4 c=2 m=1 is admissible      PASS
  RF-016 it exercises the cap (c > 1)                      PASS
  RF-016 so V=12 T=4 c=3 is not minimal among c>1 instances PASS

checks passed: 129   failed: 0
```

```text
broken anchors: 0
```

### Handoff status
- [x] Ready for Project Lead review