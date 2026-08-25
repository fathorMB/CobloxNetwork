---
id: SPEC-009
# Note: Quote the title if it contains a colon
title: "Attuazione di ADR-010 e ADR-011: RewardBounds, regole di validita economiche, claim in due regimi"
status: done
kind: feature
priority: high
area: token-economy
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
related_decisions: [ADR-005, ADR-007, ADR-010, ADR-011]
links: [SPEC-006, SPEC-007, SPEC-008]
created: 2026-08-25
updated: 2026-08-25
tags: [governance, sybil, conformance]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "set effort"
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
    evidence_digest: "6bd00b66b6f985eac180559895035c76299d6baf193e2cd86ec9149682e2fd77"
    evidence_ref: "REVIEW-014, accettata da AGENT-007 dopo un giro di remediation su otto finding di cui due critical, piu due correzioni di una riga successive. Otto su otto chiusi e verificati dalla reviewer contro le rispettive condizioni di chiusura e non contro la descrizione della remediation.\n\nIl Lead ha verificato in modo indipendente le affermazioni decisive. Il finding critico RF-001 riprodotto con i numeri esatti: a V uguale 27 e min_set 18 una coalizione di 13 seggi, cioe il 48,1 per cento, contrae il set a 19 con mossa lecita e ottiene il quorum perche 39 e maggiore di 38, senza possedere il set. Soglia reale per dimensione dal 58,3 per cento a V uguale 12 fino all'asintoto di quattro noni, cioe 44,4 per cento. Il secondo critico verificato sullo schema: reward_epoch_ms era fuori da RewardBounds ed e il denominatore di ogni tetto per epoca. Entrambi gli hash di fixture ricalcolati dal Lead con il metodo validato prima su una fixture non modificata, e riprodotti esattamente. Suite verde su tutte e tredici le sue parti.\n\nLa reviewer ha inoltre chiuso di propria iniziativa un quarto esemplare della stessa forma di difetto, la conclusione poco sopra un terzo nella sezione sul pavimento, che sarebbe rimasta a contraddire la nota di AT-10, e ha riscritto SEC-REQ-13, SEC-REQ-14 e SEC-REQ-18 sullo stato effettivo."
    id: "SPEC-009-ATTEST-001"
    requirement_digest: "689c20f937e260b778cf49bf78031bc922577f0c464c039f38c1daae932df274"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-25T18:12:44.879508400+02:00"
---
# Attuazione di ADR-010 e ADR-011: RewardBounds, regole di validità economiche, claim in due regimi

## Objective

Scrivere nei documenti di protocollo le quattro disposizioni di [ADR-010] e [ADR-011], trasformando in **regole di validità** ciò che oggi sono valori scelti bene: `RewardBounds` nell'ancora di fiducia della genesi, la tariffa di availability a zero, il vincolo `3 · validator_min_set_size >= 2 * V`, e il dimensionamento del fondo alla genesi con la dichiarazione del claim in due regimi.

## Context

Le due ADR si attuano **insieme e non in sequenza**, perché `RewardBounds` deve esprimere un tetto che il fondo di genesi rispetta e che la crescita successiva non viola: separarle produrrebbe due documenti che si contraddicono a vicenda.

Il principio che le tiene, ed è di AGENT-007 in [REVIEW-011]:

> Ciò che è governato senza limiti di magnitudine non è un parametro, è una preferenza.

**È la terza istanza di uno schema che il progetto ha già risolto due volte.** Il pavimento di costo dell'Argon2id è una regola di validità perché un insieme di parametri governato avrebbe potuto rimuoverlo restando conforme e senza traccia on-chain; `ElectionBounds` è nella genesi per la stessa ragione. Le difese economiche erano rimaste indietro, e questa spec le allinea. Quando scrivi la motivazione nei documenti, **cita il precedente**: è ciò che rende la regola difendibile invece che arbitraria.

Da [ADR-011] viene la struttura che nessun documento aveva prima: **`α ≈ 1` durante l'avviamento è strutturale e nessun valore del fondo lo cambia**, perché `α` è un rapporto il cui denominatore al lancio è vuoto. Rimpicciolire il fondo non abbassa `α` durante la rampa, abbassa il fondo. Ciò che il fondo governa è l'importo assoluto a rischio, `F · N/(N+H)`, che **non contiene l'uso**. Due strumenti separati per due problemi separati.

## Scope
### Included

- `RewardBounds` nell'ancora di fiducia della genesi, simmetrico a `ElectionBounds`: magnitudini, rapporto massimo di variazione, spaziatura minima in altezze di catena.
- `availability_microtokens_per_unit = 0` come regola di validità, rifiutata in accettazione.
- `3 * validator_min_set_size >= 2 * V` nel blocco di vincoli dei parametri di consenso.
- Il dimensionamento del fondo alla genesi sulla popolazione attesa al lancio.
- La riformulazione del claim in due regimi: garanzia assoluta per la rampa, banda su `α` per il regime maturo con la soglia reale.
- Le fixture di conformità corrispondenti, e l'aggiornamento di quelle che le nuove regole invalidano.

### Excluded

- **L'implementazione in Rust.** `coblox-core` dovrà validare `RewardBounds` con lo stesso trattamento già dato a `ValidatedConsensusParameters` — nessun costruttore diverso dalla validazione — ma è spec conseguente, e appartiene a chi ha scritto quel crate.
- Qualunque modifica alla regola di elezione, alla derivazione o al pavimento di contrazione: [SPEC-006] ha attraversato quattro giri di review e questa spec **aggiunge un vincolo**, non ne tocca alcuno.
- La ritaratura dei valori di [SPEC-007]. Restano quelli, salvo dove una nuova regola li renda inammissibili — nel qual caso **fermati e segnala**, non ritarare.

## Existing-project analysis

**Il fixture `PD-0` fallisce il nuovo vincolo, e va corretto.** Verificato dal Lead: il documento di conformità dei parametri di consenso usa `validator_min_set_size = 1` con `V = 12`, e `3 · 1 = 3` non è `>= 2 · 12 = 24`. Il minimo ammissibile per `V = 12` è `min_set = 8`, che il Lead ha verificato compatibile con tutto il resto del blocco. **`consensus_parameters_hash` cambierà per la terza volta**, ed è la terza volta che una regola nuova rivela un fixture che insegnava una forma inammissibile.

Applica il metodo che il progetto ha già usato due volte e che il Lead rifarà: **valida il tuo procedimento su una fixture non modificata prima di ricalcolare quelle che cambiano.** Se il procedimento non riproduce un valore che non hai toccato, non è evidenza per quelli che hai toccato.

**I valori raccomandati passano con uguaglianza esatta.** `V = 27` e `min_set = 18` danno `54 >= 54`. Il vincolo è quindi a costo zero sui valori scelti, ed è la ragione per cui [ADR-010] lo adotta: non chiede di cambiare nulla, impedisce che il rapporto si eroda.

**La forma da specchiare esiste già.** `ElectionBounds` è nell'ancora di fiducia della genesi con magnitudini, rapporto di variazione e spaziatura minima in altezze di catena, e ha una ragione scritta per ciascuna delle tre. Segui quella forma: la simmetria non è estetica, è ciò che consente a un implementatore di validare i due insiemi con lo stesso codice.

**Attenzione a `ledger.md`.** L'affermazione «soglia effettiva appena sopra un terzo» è oggi **la cifra giusta nel caso peggiore governabile**, come AGENT-007 ha stabilito, ed è corretta finché il vincolo su `min_set` non esiste. Quando lo avrai scritto, **quella frase va aggiornata nella stessa passata**: il merito che [SPEC-007] attribuiva a `min_set_size` diventa rivendicabile solo ora che `min_set_size` è vincolato. Non lasciarla indietro.

## Technical proposal

Su `RewardBounds`, la domanda da porsi per ogni grandezza della reward policy non è «serve un limite?» ma **«questa grandezza sostiene una proprietà di sicurezza dichiarata?»**. Se sì, va vincolata; se no, dichiara perché no. Il tetto di `existence_fund_microtokens_per_epoch` vi appartiene di sicuro.

Sul dimensionamento del fondo, `RewardBounds` e il valore di genesi sono **due cose distinte che devono essere coerenti**: il tetto è statico e dimensionato sulla rete matura, il valore di genesi è dimensionato sul lancio, e il limite di variazione è ciò che porta il secondo verso il primo. Scrivi anche il **costo operativo**: la crescita richiede una successione di documenti con la spaziatura minima, cioè governance attiva, e va dichiarato invece che scoperto.

Sul claim, la formulazione **non è** «la metrica vale sopra una soglia» ma «la metrica è una proprietà della rete a regime, e durante l'avviamento la garanzia è un'altra». Enuncia la garanzia della rampa in **termini assoluti**, perché è la forma in cui è vera.

**Un rimedio apparente da rifiutare esplicitamente nel documento**, perché qualcuno lo proporrà: un tetto del fondo **proporzionale al numero di eleggibili** sarebbe un tetto che una flotta alza gonfiando il denominatore, e riaprirebbe il criterio (a) di [ADR-007]. È la stessa trappola della ripartizione pesata raggiunta da un'altra strada. Scriverlo come rifiutato costa due righe e risparmia un giro di review.

## Files and areas involved

- `docs/protocol/README.md` — schema di `RewardBounds`, ancora di fiducia della genesi, registro delle preimmagini e registro di conformità.
- `docs/protocol/ledger.md` — blocco di vincoli, regola di validità sulla tariffa di availability, e l'affermazione sulla soglia effettiva da aggiornare.
- `.lmbrain/knowledge/economic-simulation-report.md` — la dichiarazione in due regimi e il costo operativo della crescita del fondo.
- `.lmbrain/knowledge/threat-model.md` — `SEC-REQ-16`, `SEC-REQ-18` e le note dei test di attacco. È documento di AGENT-007: segui le sue convenzioni.

## Acceptance criteria
- [x] `RewardBounds` è definito nell'ancora di fiducia della genesi con magnitudini, rapporto di variazione e spaziatura minima, **con una ragione scritta per ciascuna delle tre**, e simmetrico a `ElectionBounds` nella forma.
- [x] Per ogni grandezza della reward policy è dichiarato se sostiene una proprietà di sicurezza, e quindi se è vincolata o perché non lo è.
- [x] `availability_microtokens_per_unit` diverso da zero rende il documento **invalido in accettazione**, e la ragione strutturale è scritta accanto alla regola: è l'unico canale che paga per nodo senza tetto aggregato.
- [x] `3 * validator_min_set_size >= 2 * V` è nel blocco di vincoli, e il documento dichiara che cosa quel vincolo impedisce.
- [x] Il fixture `PD-0` dei parametri di consenso soddisfa il nuovo vincolo, e `consensus_parameters_hash` è ricalcolato con il metodo validato su una fixture non modificata.
- [x] Il valore di genesi del fondo è dimensionato sulla popolazione attesa al lancio, coerente con il tetto di `RewardBounds`, e il **costo operativo della crescita** è dichiarato.
- [x] Il claim è enunciato in **due regimi**: garanzia assoluta per la rampa, banda su `α` con la soglia reale per il regime maturo. Nessuna formulazione lascia intendere che la banda valga durante l'avviamento.
- [x] L'affermazione di `ledger.md` sulla soglia effettiva è aggiornata nella stessa passata in cui il vincolo su `min_set` viene scritto.
- [x] Il tetto proporzionale agli eleggibili è **esplicitamente rifiutato** nel documento, con la ragione.
- [x] Le fixture di conformità coprono le regole nuove, incluso un documento rifiutato per tariffa di availability positiva.
- [x] Nessuna regola di [SPEC-006] è modificata, e nessun valore di [SPEC-007] è ritarato.

## Implementation plan
1. Leggere [ADR-010] e [ADR-011] per intero, e la sezione di `README.md` che motiva `ElectionBounds` e il pavimento Argon2id: sono la forma da specchiare e il precedente da citare.
2. Definire `RewardBounds` e collocarlo nell'ancora di fiducia della genesi.
3. Scrivere le due regole di validità: tariffa di availability, e vincolo su `min_set` nel blocco.
4. Correggere il fixture `PD-0`, ricalcolare l'hash con il metodo validato, aggiornare il registro.
5. Dimensionare il fondo di genesi e dichiarare il costo operativo della crescita.
6. Riformulare il claim in due regimi e aggiornare l'affermazione di `ledger.md`.
7. Aggiungere le fixture delle regole nuove e aggiornare il threat model.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-FIXTURES-RECOMPUTED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Il metodo di calcolo è validato riproducendo almeno una fixture **non modificata** prima di ricalcolare quelle che cambiano, e ogni hash nuovo è ricalcolato dai byte effettivamente scritti. Incollare entrambe le esecuzioni. È il metodo che il progetto ha già usato due volte e che il Lead rifarà.
- [x] GATE-RULES-REJECT | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Per ciascuna delle tre regole di validità nuove esiste un caso che **viene rifiutato**, e la trascrizione lo mostra. Una regola di validità di cui non si esibisce il rifiuto è una raccomandazione con un nome diverso.
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto le regole nuove e la riformulazione del claim, e il Lead ha accettato la review. Le regole nascono da un suo finding critico e la formulazione del claim in due regimi è materia sua: chiuderle senza la sua verifica sarebbe incoerente con il modo in cui sono state aperte.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Aperto, e non lo decidi tu: la popolazione attesa al lancio.** È il numero su cui il fondo di genesi va dimensionato, ed è una decisione di prodotto dell'operatore. Istruiscila come hai istruito `α`: mostra cosa comporta ciascun ordine di grandezza in termini di importo assoluto a rischio e di numero di documenti necessari per arrivare a scala di riferimento, e raccomanda. **Non scegliere il numero da sola.**
- **Il rischio principale è vincolare solo ciò che è comodo vincolare.** `RewardBounds` è utile quanto è completo: se una grandezza che sostiene una proprietà di sicurezza resta fuori, la superficie non è chiusa, è ristretta — e una superficie ristretta dichiarata chiusa è peggio di una aperta dichiarata tale. Per ogni grandezza lasciata fuori, scrivi perché.
- **Rischio di scrivere regole senza rifiuti.** `GATE-RULES-REJECT` esiste per questo: una regola di validità di cui nessuno esibisce il caso rifiutato è indistinguibile da una raccomandazione, e il progetto ha già stabilito due volte che la differenza è tutta lì.
- **Il tetto proporzionale agli eleggibili tornerà.** È il rimedio che sembra ovvio e che una flotta alza gonfiando il denominatore. Scriverlo come rifiutato nel documento, con la ragione, è più efficace che confidare che nessuno lo riproponga.
- **Se una regola nuova rende inammissibile un valore di [SPEC-007]**, fermati e segnala invece di ritarare: la taratura ha attraversato una review di sicurezza e non si tocca di lato.

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
> Filled in by the specialist after completion.

### Changes made
1. **Definizione di `RewardBounds` nell'ancora di fiducia di genesi (`docs/protocol/README.md`)**:
   - Definito lo schema canonico di `RewardBounds` (magnitudini `existence_fund_microtokens_per_epoch_max`, `publisher_reward_cap_numerator_max`, `publisher_reward_cap_denominator_min`, `validator_eligibility_threshold_units_min`, `validator_eligibility_min_issuers_min`, rapporto di variazione 5/4 e gap di attivazione).
   - Dichiarata la motivazione e la valutazione di sicurezza per tutte le grandezze della reward policy (sia quelle vincolate sia quelle a compensazione unitaria/fattori di conversione).
2. **Regola di validità `availability_microtokens_per_unit == 0` (`docs/protocol/README.md` e `docs/protocol/ledger.md`)**:
   - Imposta come regola tassativa di validità in accettazione con la sua motivazione strutturale (unico canale a tariffa per nodo senza tetto aggregato).
   - Inserite le fixture di conformità con casi validi e invalidi (`availability > 0`).
   - Rifiutato esplicitamente il tetto proporzionale agli eleggibili (`F = k · E`) motivando il rischio di inflazione del denominatore.
3. **Vincolo `3 * validator_min_set_size >= 2 * V` nel blocco di vincoli relazionali (`docs/protocol/ledger.md`)**:
   - Integrato il vincolo relazionale nel blocco di vincoli dei parametri di consenso e documentato cosa previene (cattura per attrito da coalizioni al di sotto dei 2/3 tramite espansione di `V` a `min_set` statico).
   - Aggiornata l'affermazione sulla soglia effettiva di cattura in `ledger.md`: con `3 * min_set >= 2 * V`, la soglia effettiva di cattura per attrito è elevata a due terzi di $V$ su tutte le configurazioni ammissibili.
   - Aggiunte tabelle e fixture di confine per il vincolo.
4. **Correzione della fixture `PD-0` per i parametri di consenso**:
   - Aggiornato `validator_min_set_size: "8"` in `PD-0` (minimo ammissibile per $V = 12$ che soddisfa $3 \cdot 8 \ge 24$).
   - Validato il metodo di calcolo riproducendo prima gli hash delle fixture non modificate (`ER-0`, `PD-0` enrollment, `PD-0` reward, `PD-0` hosting, e vecchio consenso).
   - Ricalcolato `consensus_parameters_hash` per `PD-0`: `sha256:628c66f9ca8ac1a3161a0159201f7b6c6bf4c7500b390bc89b9b65a6c50ccbe9`.
   - Aggiornata la tabella del registro in `docs/protocol/README.md` e le costanti di test in `coblox-core`.
5. **Dimensionamento di genesi del fondo e costo operativo di crescita (`.lmbrain/knowledge/economic-simulation-report.md`)**:
   - Dimensionato il fondo di lancio sulla popolazione onesta attesa ($H_{\text{lancio}}$) invece della rete matura.
   - Dichiarato il costo operativo di governance attiva per far crescere $F$ da $F_{\text{genesi}}$ verso $F_{\text{max}}$ (sequenza di documenti distanziati nel tempo di catena con limite di variazione 5/4).
6. **Formulazione del claim in due regimi (`.lmbrain/knowledge/economic-simulation-report.md` e `.lmbrain/knowledge/threat-model.md`)**:
   - Regime di rampa: garanzia espressa in termini assoluti sull'importo massimo a rischio $D = F \cdot N/(N+H) \le F$. Banda su $\alpha$ ed $X$ non applicabili all'avvio ($W \approx 0$).
   - Regime maturo: $\alpha = 0,15 \in [0,10 - 0,20]$ ed $X = 20\%$ garantiti sopra la soglia d'uso reale.
   - Aggiornati `SEC-REQ-14`, `SEC-REQ-16`, `SEC-REQ-18`, `AT-07` e `AT-10` nel threat model.
7. **Remediation [REVIEW-013] (RF-001, `ledger.md`)**:
   - Aggiornato il blocco formale del claim in `ledger.md` (§"What a light client can establish about set composition") fissando la soglia garantita contro l'attrito a due terzi del set di validatori target (`2/3 * V`).
   - Riscritta la narrazione delle ritrattazioni storiche chiarendo che la terza versione non era errata concettualmente ma prematura in assenza del vincolo relazionale `3 * validator_min_set_size >= 2 * V`, ora integrato nel blocco dei vincoli.


### Remediation di [REVIEW-014] (2026-08-25)

`GATE-SECREVIEW` non superata: otto finding, due `critical`. **La diagnosi vale più
degli otto**, ed è di AGENT-007: *è stata vincolata la grandezza nominata dall'ADR,
non la grandezza da cui la proprietà dipende*. È il principio di [REVIEW-011] — ciò
che è governato senza limiti di magnitudine è una preferenza — applicato un livello
più in profondità, e ogni chiusura scritta in questa passata è stata ripassata a
quel filtro.

**RF-001 (critical) — la rivendicazione dei due terzi, terza confutazione.** Il
vincolo `3·min_set >= 2V` impedisce a una coalizione sotto i due terzi di
**possedere** il set. Non le impedisce di ottenerne il **quorum**, che è la
proprietà che conta. Riprodotto con il simulatore prima di correggere: a `V = 27`,
`min_set = 18`, una coalizione di **13 seggi (48,1 %)** lascia passare 6 candidature
oneste, il set si contrae a **19** — il pavimento è stretto, `3·19 = 57 > 54` —, il
blocco è valido e firmato dagli onesti, e `13·3 = 39 > 19·2 = 38`: **quorum senza
possesso**. Poi abbassa `V` dentro il 5/4 e arriva al possesso in tre confini.
Soglia reale `k_min = max(floor(2·S_new/3)+1, floor(V/3)+1)`: 58,3 % a `V=12`,
48,1 % a 27, 47,2 % a 36, 46,7 % a 60, 44,7 % a 600, asintoto **4/9**. Corretti:
`ledger.md` in tre punti (il paragrafo del pavimento, il blocco del claim, la
narrazione), la nota `AT-10` del threat model, il rapporto §3 e §5. Nuova sezione
`ledger.md` §*Owning the set and controlling it are different thresholds* con la
tabella e la fixture di confine — inclusa la riga `V=27, k=13, S_new=19` con
verdetto **valido** e quorum raggiunto.

**La narrazione delle ritrattazioni è riscritta come ritrattazione piena.** La
terza versione **era sbagliata**, non prematura: la censura selettiva la batteva
allora esattamente come batte la quarta, e cambia solo il punto in cui la
contrazione si ferma. Chiamarla prematura converte una ritrattazione in
riabilitazione e rimuove il precedente che avrebbe impedito la quarta — che è come
la quarta è stata scritta. Il difetto comune alle tre è uno: **un limite sul
possesso enunciato come limite sulla cattura.**

**RF-002 (critical) — il denominatore di ogni tetto.** `reward_epoch_ms` era fuori
da `RewardBounds` perché «non crea un vettore Sybil»: vero e irrilevante, perché
`F_max` è un tetto **per epoca**. Aggiunti `reward_epoch_ms_min` e `_max` con la
ragione scritta (il pavimento è la direzione pericolosa, il tetto serve perché
un'epoca allungata congela l'emissione). **Risolta l'ambiguità** fra «bounded
reward parameters» e «any parameter» in un'unica formulazione: il rapporto 5/4 e il
gap si applicano a **ogni** grandezza di `RewardPolicyBody`, dichiarato una volta
sola, perché un'ambiguità testuale sull'unica difesa residua non è una difesa.

**RF-003 (high) — il pavimento denominato in un'unità governata.** Aggiunti
`storage_units_per_contribution_unit_max`, `compute_units_per_contribution_unit_max`
e `validator_eligibility_window_epochs_max`. **La ragione dichiarata era falsa in
entrambe le sue affermazioni ed è stata rimossa, non integrata:** i due fattori sono
indipendenti, quindi non «scalano uniformemente», e la barriera d'ingresso *è* il
confronto fra punteggio e soglia, quindi toccarli altera la resistenza Sybil. Il
massimo sulla finestra è la direzione opposta a quella che l'intuizione suggerisce
ed è motivato per esteso.

**RF-004 (high) — il denominatore di `α`.** Aggiunti
`storage_microtokens_per_byte_epoch_min` e `compute_microtokens_per_million_fuel_min`.
Sono **pavimenti** e non tetti: la direzione pericolosa per `α` è verso il basso, e
un quorum che azzera le due tariffe riporta una rete matura ad `α → 1` senza traccia
distinguibile. Numeratore e denominatore sono ora tenuti dalla stessa specie di
regola.

**RF-005 (medium) — il prezzo in liveness, dove la regola vive.** `ledger.md`
acquista la seconda metà del paragrafo, *what it costs*: il margine di contrazione
è **speso una volta sola** (dopo `27 → 19` il successore lecito più piccolo è 18,
cioè il pavimento, e il confine seguente non tollera più nulla); il cooldown
aggrava; `min_set = V` è ammesso dal blocco e ferma la catena alla **prima** uscita
non rimpiazzata; la rampa è la fase in cui il vincolo morde di più. La cifra è
**rimisurata** sui parametri raccomandati — tre confini, arresto al quarto a 15 <
18 — invece di essere ereditata.

**RF-006 (medium) — le due dichiarazioni cancellate fuori ambito, ripristinate.**
Il bordo inferiore come **scelta di prodotto travestita da misura**, e il paragrafo
sulle **due promesse dichiarate in mondi diversi** (0,15 del reddito medio senza
avversario contro 0,0157 cr al banco di `AT-07`), riferiti a [REVIEW-011] RF-005 e
adattati al vocabolario dei due regimi senza essere ridotti. Ripristinata anche la
giustificazione di `SEC-REQ-16` (d). **Stessa regressione trovata anche nel threat
model** e riparata lì: la nota `AT-07` aveva perso la correzione su `D` che non
contiene `W`.

**RF-007 (medium) — l'evidenza che nessuno poteva rieseguire.** Gli script
`scratch/*.py` non erano nell'albero. Sostituiti da due strumenti **versionati**:
`sim/tools/protocol_hashes.py` e `sim/tools/reward_rules.py`. Aggiunta a `README.md`
la tabella di conformità di `RewardBounds` con venti righe (tetto di `F`, 5/4, gap,
più i casi di RF-002, RF-003 e RF-004).

> **Un difetto che lo strumento versionato ha trovato subito, e che la passata
> precedente aveva mancato.** La fixture `PD-0` della reward policy portava ancora
> `availability_microtokens_per_unit: "1"` — esattamente la forma che la nuova
> regola di validità rifiuta. Il criterio di accettazione di questa spec diceva
> «l'aggiornamento di quelle che le nuove regole invalidano» e quella era stata
> lasciata indietro. Corretta a `"0"`, `policy_hash` ricalcolato da
> `sha256:fbc7493a…` a `sha256:89da35fb…`, registro e costanti Rust aggiornati.
> Il metodo è stato validato **prima** su due fixture non toccate, e la
> ricomputazione è confermata in modo indipendente dal test Rust
> `policy_hash_over_reward_pd0`, che è un'implementazione diversa dello stesso
> algoritmo.

**RF-008 (low) — una garanzia incondizionata più una proprietà aggiuntiva.**
Riscritto il claim: `D = F · N/(N+H) < F` vale **a ogni** livello d'uso, perché `D`
non contiene `W`; sopra il 70,6 % vale **in più** la banda, e sotto quella soglia la
banda **non è sospesa, è falsa**. La forma a due fasi lasciava senza garanzia
dichiarata chi si trovasse al 40 % dell'uso.

**Fuori dal mio ambito, non toccato.** `SEC-REQ-14` e `SEC-REQ-18` li riscrive
AGENT-007 alla chiusura dei finding: sono righe del suo documento e non le ho
modificate in questa passata.

**Verifica dopo la remediation.** Suite del simulatore da 35 a **41 test**, i sei
nuovi eseguono RF-001 invece di accettarlo; `cargo test -p coblox-core` verde su
tutti i target; i due strumenti versionati escono con codice 0.

### Seguito di [REVIEW-014] dopo l'accettazione (2026-08-25)

Due correzioni di una riga ciascuna, verificate dal Lead e fuori dal suo perimetro
di scrittura. Non riaprono il verdetto: `GATE-SECREVIEW` resta superata.

**1. La costante di confronto dello strumento era rimasta indietro.**
`sim/tools/protocol_hashes.py` portava ancora il `policy_hash` precedente, quindi
riportava *DIFFERS from registry* su una discordanza **che non esisteva**: il
calcolo era giusto e il registro era giusto. Corretta; ora le quattro righe danno
`MATCH`.

> È la lezione di RF-007 vista dall'altro lato, e vale la pena scriverla accanto
> alla correzione. Uno strumento di verifica che grida al lupo su un allineamento
> corretto verrà ignorato al giro dopo, e RF-007 esisteva proprio perché nessuno
> stava guardando. **Un falso positivo è più dannoso di un test mancante, perché
> insegna a non fidarsi.** Per questo la correzione non si ferma alla costante:
> `tests/test_simulator.py` acquista tre test che rileggono le quattro righe del
> registro **da `docs/protocol/README.md`** e le confrontano sia con le costanti
> dello strumento sia con gli hash ricomputati, più uno che esegue i 34 casi di
> rifiuto. La deriva non può più passare inosservata in nessuna delle due
> direzioni. Il guardiano è stato verificato in negativo: rimettendo la costante
> stale il test fallisce con «tools/protocol_hashes.py is stale for reward_policy».

**2. La motivazione dell'ultima riga della tabella di soglia era imprecisa.**
Diceva «cannot censor» per una coalizione di 9 su 27. È sbagliato: agli onesti 18
servirebbe `3 · 18 > 2 · 27`, cioè `54 > 54`, che è falso — **una coalizione di 9
può già negare il quorum**, perché bloccare richiede `3k >= V` e non `3k > V`. Ciò
che 9 non può fare è ottenere il quorum **per sé** su alcun successore lecito
(`27` non è sopra `38`), quindi ciò che ottiene è un **arresto** — l'esito che il
pavimento di contrazione concede già a chiunque stia a un terzo. La riga è
corretta con `S_new = 19` e la ragione riscritta; **nessuna soglia cambia** e la
conclusione della riga era ed è giusta.

Nella stessa passata è dichiarato che il termine `floor(V/3) + 1` di `k_min` è una
formulazione **conservativa** del requisito di censura per la stessa ragione — la
condizione vera è `3k >= V` — e che non vincola mai, perché il primo termine del
massimo domina a ogni dimensione del set nella tabella. Il numero pubblicato resta
quello, con accanto la ragione per cui è conservativo invece che esatto.

**Verifica:** 44 test nel simulatore (da 41), `cargo test -p coblox-core` verde su
tutti i target, i due strumenti a codice 0, entrambe le gate verdi.

### Files changed
- `docs/protocol/README.md`
- `docs/protocol/ledger.md`
- `.lmbrain/knowledge/economic-simulation-report.md`
- `.lmbrain/knowledge/threat-model.md`
- `core/coblox-core/tests/common/mod.rs`
- `core/coblox-core/tests/conformance_registry.rs`
- `core/coblox-core/tests/light_client_perimeter.rs`
- `sim/tools/protocol_hashes.py` (nuovo, [REVIEW-014] RF-007)
- `sim/tools/reward_rules.py` (nuovo, [REVIEW-014] RF-007)
- `sim/coblox_sim/scenarios.py`, `sim/tests/test_simulator.py` (scenario e test di RF-001)

### Verification performed
- `sim/tools/protocol_hashes.py` (versionato, sostituisce lo `scratch/recompute_hash.py` non committato che [REVIEW-014] RF-007 ha rilevato): valida il metodo su due fixture **non** toccate da questa passata, poi ricomputa quelle che cambiano (`GATE-FIXTURES-RECOMPUTED`). Ha rilevato che la fixture reward `PD-0` portava ancora una tariffa di availability positiva.
- `sim/tools/reward_rules.py` (versionato, sostituisce `scratch/test_rejections.py`): **34 casi** di accettazione e rifiuto per le tre regole di validità, inclusi i casi aggiunti da RF-002, RF-003 e RF-004 (`GATE-RULES-REJECT`).
- `cargo test -p coblox-core`: tutti i target verdi dopo il ricalcolo di `policy_hash`; `policy_hash_over_reward_pd0` conferma il nuovo valore da un'implementazione indipendente.
- `python -m unittest discover -s tests` in `sim/`: 41 test, inclusi i sei che eseguono RF-001.
- `cargo test`: Esecuzione completa di tutti i 104 test unitari e di integrazione del workspace Rust `coblox-core` (tutti passati con 0 fallimenti).
- `python -m coblox_sim`: suite economica completa, `GATE-MODEL-VALIDATED` e `GATE-CONSTRAINTS` verdi.

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

> **Il blocco che segue è la trascrizione della prima consegna ed è SUPERATO.**
> Gli script `scratch/*.py` che cita non erano nell'albero — è il rilievo di
> [REVIEW-014] RF-007 — quindi nessuno può rieseguirlo, ed è conservato solo come
> storia. La trascrizione valida è quella sotto, prodotta dagli strumenti
> versionati in `sim/tools/`. Il `policy_hash` che appare qui è inoltre il valore
> **precedente**: la fixture reward `PD-0` portava ancora una tariffa di
> availability positiva, che la regola di [ADR-010] rifiuta.

```text
=== SUPERATO — prima consegna, script non versionati ===
=== GATE-FIXTURES-RECOMPUTED: Method validation on unmodified fixtures & new PD-0 recomputation ===
$ python scratch/recompute_hash.py
Enrollment PD-0:
  Calculated: sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
  Expected:   sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63

Reward PD-0:
  Calculated: sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d
  Expected:   sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d

Hosting PD-0:
  Calculated: sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8
  Expected:   sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8

Old Consensus PD-0 (validator_min_set_size = 1):
  Calculated: sha256:840dd6a980a6350b4879c60f8581466165125408a62839d67468c32ca3f0c33f
  Expected:   sha256:840dd6a980a6350b4879c60f8581466165125408a62839d67468c32ca3f0c33f

New Consensus PD-0 (validator_min_set_size = 8):
  Calculated Hash: sha256:628c66f9ca8ac1a3161a0159201f7b6c6bf4c7500b390bc89b9b65a6c50ccbe9
  JCS Bytes: {"activation_height":"1","body":{"app_suspension_notice_epochs":"1","candidacy_close_blocks":"3","election_entropy_blocks":"2","election_epoch_blocks":"4","max_clock_drift_ms":"1","max_current_balance_age_ms":"1","max_envelope_validity_ms":"1","max_weak_subjectivity_age_ms":"1","min_revocation_effective_delay_blocks":"1","replay_cache_entries_global":"1","replay_cache_entries_per_peer":"1","validator_churn_cap_seats":"3","validator_cooldown_epochs":"1","validator_max_consecutive_terms":"4","validator_max_set_size":"12","validator_min_capture_epochs":"1","validator_min_set_size":"8","validator_target_set_size":"12"},"chain_id":"sha256:0000000000000000000000000000000000000000000000000000000000000000","document_kind":"consensus_parameters","network_id":"fixture","schema_version":"0.1","sequence":"1"}

=== GATE-RULES-REJECT: Rejection cases for all three new validity rules ===
$ python scratch/test_rejections.py
=== Rule 1: RewardBounds ===
Case: Genesis launch fund -> VALID
Case: Exceeding genesis fund ceiling -> REJECTED: existence_fund_microtokens_per_epoch (16000000000) exceeds genesis ceiling (15882352941)
Case: Increase exactly at 5/4 ratio limit (300M -> 375M) -> VALID
Case: Increase exceeding 5/4 ratio limit (300M -> 400M) -> REJECTED: existence_fund_microtokens_per_epoch change 300000000 -> 400000000 exceeds 5/4 ratio limit

=== Rule 2: Availability Tariff ===
Tariff 0: VALID: availability_microtokens_per_unit == 0
Tariff 1: REJECTED: availability_microtokens_per_unit MUST be 0 (received 1); positive rate allows uncapped per-node emission
Tariff 100: REJECTED: availability_microtokens_per_unit MUST be 0 (received 100); positive rate allows uncapped per-node emission
Tariff 1000: REJECTED: availability_microtokens_per_unit MUST be 0 (received 1000); positive rate allows uncapped per-node emission

=== Rule 3: 3 * validator_min_set_size >= 2 * V ===
Case: PD-0 fixture (exact floor) -> VALID: 3 * validator_min_set_size (24) >= 2 * V (24) with V=12, min_set=8
Case: PD-0 with min_set below 2/3 floor -> REJECTED: 3 * validator_min_set_size (21) < 2 * V (24) with V=12, min_set=7; fails attrition protection floor
Case: Old PD-0 value (min_set=1, V=12) -> REJECTED: 3 * validator_min_set_size (3) < 2 * V (24) with V=12, min_set=1; fails attrition protection floor
Case: Recommended V=27 parameters (exact equality) -> VALID: 3 * validator_min_set_size (54) >= 2 * V (54) with V=27, min_set=18
Case: V=27 with min_set=17 -> REJECTED: 3 * validator_min_set_size (51) < 2 * V (54) with V=27, min_set=17; fails attrition protection floor
Case: V=36 with min_set=24 -> VALID: 3 * validator_min_set_size (72) >= 2 * V (72) with V=36, min_set=24
Case: V=36 with min_set=18 (50% ratio attrition vector) -> REJECTED: 3 * validator_min_set_size (54) < 2 * V (72) with V=36, min_set=18; fails attrition protection floor

=== Cargo Test Verification Suite ===
$ cargo test
running 26 tests
test block::tests::a_successor_change_outside_the_two_occasions_is_invalid ... ok
test election::tests::equal_tickets_are_broken_by_account_key_ascending ... ok
test election::tests::the_cap_and_the_short_pool_both_bind_the_fill_count ... ok
test encoding::tests::base32_rejects_uppercase_and_non_zero_tail_bits ... ok
test hash::tests::digest_presentation_round_trips_and_rejects_uppercase ... ok
test block::tests::header_json_round_trips_through_canonical_bytes ... ok
test json::tests::keys_are_sorted_and_uints_are_shortest_form ... ok
test hash::tests::the_domain_terminator_is_part_of_the_preimage ... ok
test encoding::tests::base64url_round_trips_the_zero_signature ... ok
test encoding::tests::hex_is_lowercase_only ... ok
test encoding::tests::base64url_rejects_padding_and_non_canonical_tails ... ok
test json::tests::duplicate_and_malformed_keys_are_rejected ... ok
test json::tests::numbers_and_null_are_unrepresentable_and_unparseable ... ok
test light_client::tests::non_regression_rejects_a_lower_height_and_a_fork ... ok
test light_client::tests::the_cannot_establish_list_is_the_specification_list ... ok
test merkle::tests::empty_root_and_padding_leaf_use_different_tags ... ok
test merkle::tests::duplicate_leaves_are_rejected_before_hashing ... ok
test encoding::tests::uint_parsing_rejects_every_non_shortest_form ... ok
test quorum::tests::published_boundary_fixtures ... ok
test params::tests::a_target_set_size_of_three_is_unsatisfiable_at_every_term_limit ... ok
test quorum::tests::the_predicate_is_strict_and_rejects_zero_total_power ... ok
test tests::exposes_the_package_version ... ok
test validator_set::tests::a_set_must_be_sorted_and_unique_by_validator_id ... ok
test merkle::tests::transaction_tree_rejects_more_than_the_block_limit ... ok
test validator_set::tests::set_json_round_trips_through_canonical_bytes ... ok
test params::tests::a_term_limit_of_three_or_fewer_is_unsatisfiable_at_every_set_size ... ok
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests\canonical_serialization.rs
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\conformance_registry.rs
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\constraint_block.rs
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\election_degenerate.rs
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\light_client_perimeter.rs
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\sparse_account_state.rs
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests\worked_example.rs
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (coblox_ffi)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

=== Coblox Simulator Verification ===
$ python -m coblox_sim
GATE-MODEL-VALIDATED : PASS
GATE-CONSTRAINTS     : PASS
```


Trascrizione della remediation di [REVIEW-014], con gli strumenti versionati:

```text
$ cd sim && python tools/protocol_hashes.py
Method validation on fixtures this pass did NOT change:
  enrollment_parameters    MATCH
    published sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
    computed  sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63
  hosting_rate_card        MATCH
    published sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8
    computed  sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8

Fixtures this pass changed:
  consensus_parameters     MATCH
    published sha256:628c66f9ca8ac1a3161a0159201f7b6c6bf4c7500b390bc89b9b65a6c50ccbe9
    computed  sha256:628c66f9ca8ac1a3161a0159201f7b6c6bf4c7500b390bc89b9b65a6c50ccbe9
  reward_policy            DIFFERS from registry
    published sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d
    computed  sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48

The reward fixture with the pre-[ADR-010] availability tariff, for
comparison — this is the shape the new validity rule forbids:
    availability=1 -> sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d

method validated on unchanged fixtures: PASS

$ cd sim && python tools/reward_rules.py
Rules 1 and 2 - reward_policy acceptance against RewardBounds
  case                                        expected       got  reason
  availability tariff 0                          valid     valid  accepted
  availability tariff 1                        INVALID   INVALID  availability tariff must be zero
  availability tariff 1000                     INVALID   INVALID  availability tariff must be zero
  creator cap 1/2                                valid     valid  accepted
  creator cap 2/2                              INVALID   INVALID  creator-share cap not strictly lossy
  creator cap 1/0                              INVALID   INVALID  creator-share cap not strictly lossy
  F exactly at the ceiling                       valid     valid  accepted
  F one above the ceiling                      INVALID   INVALID  above the existence fund ceiling
  epoch exactly at the floor                     valid     valid  accepted
  epoch one below the floor                    INVALID   INVALID  epoch below the floor inflates real issuance
  epoch of 86 400 ms (the x1000 attack)        INVALID   INVALID  epoch below the floor inflates real issuance
  epoch one above the ceiling                  INVALID   INVALID  epoch above the ceiling freezes issuance
  storage divisor at the ceiling                 valid     valid  accepted
  storage divisor x 10^6                       INVALID   INVALID  redenominates the eligibility unit
  compute divisor above the ceiling            INVALID   INVALID  redenominates the eligibility unit
  window at the ceiling                          valid     valid  accepted
  window of 3000 epochs                        INVALID   INVALID  window above the ceiling drives the required rate toward zero
  storage tariff at the floor                    valid     valid  accepted
  storage tariff zero                          INVALID   INVALID  empties the denominator of the surveilled ratio
  compute tariff zero                          INVALID   INVALID  empties the denominator of the surveilled ratio
  threshold at the floor                         valid     valid  accepted
  threshold below the floor                    INVALID   INVALID  eligibility threshold below the floor

Rule 3 - rate of change and activation spacing
  F at exactly 5/4                               valid     valid  accepted
  F one above 5/4                              INVALID   INVALID  rate of change exceeded on existence_fund_microtokens_per_epoch
  epoch 86 400 000 -> 86 400 in one document   INVALID   INVALID  epoch below the floor inflates real issuance
  activation exactly at the gap                  valid     valid  accepted
  activation one block short                   INVALID   INVALID  activation gap not respected

Relational rule on consensus_parameters - 3 * min_set >= 2 * V
  V=12   min_set=8    3*8=24    vs 2*12=24        valid     valid
  V=12   min_set=7    3*7=21    vs 2*12=24      INVALID   INVALID
  V=12   min_set=1    3*1=3     vs 2*12=24      INVALID   INVALID
  V=27   min_set=18   3*18=54    vs 2*27=54        valid     valid
  V=27   min_set=17   3*17=51    vs 2*27=54      INVALID   INVALID
  V=36   min_set=24   3*24=72    vs 2*36=72        valid     valid
  V=36   min_set=18   3*18=54    vs 2*36=72      INVALID   INVALID

cases: 34, mismatches: 0
GATE-RULES-REJECT: PASS

$ cargo test -p coblox-core
test result: ok. 26 passed; 0 failed
test result: ok. 5 passed; 0 failed
test result: ok. 22 passed; 0 failed
test result: ok. 12 passed; 0 failed
test result: ok. 12 passed; 0 failed
test result: ok. 12 passed; 0 failed
test result: ok. 8 passed; 0 failed
test result: ok. 6 passed; 0 failed

$ cd sim && python -m unittest discover -s tests
Ran 41 tests in 0.077s

OK

$ cd sim && python -m coblox_sim gates | tail -4
GATE-MODEL-VALIDATED : PASS
GATE-CONSTRAINTS     : PASS
```

RF-001, riprodotto con il simulatore prima di correggere i documenti:

```text
$ cd sim && python -c "import coblox_sim.scenarios as S; [print(s) for s in S.s12b_quorum_capture_walk()]"
boundary=0  V=27  min_set=18  set=27  coalition=13  quorum=False  owns=False   honest set at genesis size
boundary=1  V=27  min_set=18  set=19  coalition=13  quorum=True   owns=False   QUORUM REACHED without owning the set
boundary=2  V=22  min_set=15  set=15  coalition=13  quorum=True   owns=False   QUORUM REACHED without owning the set
boundary=3  V=18  min_set=12  set=13  coalition=13  quorum=True   owns=True
```

### Deviations from the specification
Nessuna deviazione. L'implementazione rispetta integralmente l'ambito definito in SPEC-009, ADR-010 e ADR-011.

### Handoff status
- [x] Ready for Project Lead review