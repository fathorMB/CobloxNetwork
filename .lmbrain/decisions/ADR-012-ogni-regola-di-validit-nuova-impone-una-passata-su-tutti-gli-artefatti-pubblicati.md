---
id: ADR-012
# Note: Quote the title if it contains a colon
title: "Ogni regola di validità nuova impone una passata su tutti gli artefatti pubblicati"
status: accepted
decision_date: 2026-08-25
decider: AGENT-LEAD
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: [ADR-010, ADR-011]
tags: [architecture, security]
created: 2026-08-25
updated: 2026-08-25
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned proposed -> accepted"
  - date: 2026-08-25
    action: "registered sixth occurrence (SPEC-012 / REVIEW-018): the ed25519-speccheck outcome table"
---
# Ogni regola di validità nuova impone una passata su tutti gli artefatti pubblicati

> Decisa il 2026-08-25 su mandato dell'operatore, dopo che lo stesso difetto si è ripetuto quattro volte in tre spec. AGENT-007 la sostiene in [REVIEW-014] e ne ha precisato la forma: **una gate, non una prassi**.

## Context

Sei volte, in cinque spec diverse, un artefatto **pubblicato** del protocollo ha insegnato una forma che le regole del protocollo rendono inammissibile. Ogni volta è emerso per caso, quando una regola nuova ha reso quell'artefatto verificabile, e mai perché qualcuno lo stesse cercando.

| Artefatto | Portava | Reso inammissibile da | Trovato in |
| --- | --- | --- | --- |
| Fixture `PD-0` dei parametri di consenso | `validator_max_consecutive_terms = 3` | La soddisfacibilità congiunta di [SPEC-006], che impone `T >= 4` a ogni dimensione del set | [SPEC-006], giro 3 |
| Esempio numerico dell'elezione | `T = 3`, stessa forma | Idem | [SPEC-006], verifica del giro 4 |
| Fixture `PD-0` dei parametri di consenso | `validator_min_set_size = 1` con `V = 12` | Il vincolo `3 * min_set >= 2 * V` di [ADR-010] | [SPEC-009], analisi del Lead |
| Fixture `PD-0` della reward policy | `availability_microtokens_per_unit = 1` | La regola di validità di [ADR-010], che rifiuta quella tariffa positiva | [SPEC-009], remediation |
| Esempio canonico di `challenge_evidence` in `ledger.md` | `request_hash` diverso da `challenge_id`, rispecchiato in `canonical_serialization.rs` | La regola di `README.md` §*Hash preimage registry*, che impone `challenge_id == request_hash` — **una regola che c'era già** | [SPEC-010], **prima esecuzione dello strumento** |
| Tabella degli esiti `ed25519-speccheck` in `README.md` §*Consensus-critical Ed25519 verification* | `accept` al vettore 8, rispecchiato in `tests/fixtures/README.md` | La regola dei quattro punti scritta **due paragrafi sopra la tabella stessa**, che impone l'hash di `k` sulle codifiche originali — **una regola che c'era già, nello stesso documento** | [SPEC-012], **[REVIEW-018]**, cioè una review |

**Le ultime due righe sono di natura diversa dalle prime quattro, e la differenza è il punto.** Le prime quattro emersero per caso, quando una regola nuova rese verificabile un artefatto che nessuno stava guardando. La quinta è stata **trovata dal meccanismo che questa ADR istituisce**, alla sua prima esecuzione, contro una regola che esisteva da [SPEC-001] — cioè un difetto che nessuna spec successiva avrebbe avuto motivo di cercare. È la prima evidenza che la gate fa ciò per cui è stata scritta, ed è registrata qui perché una ADR che elenca solo i propri fallimenti non dice se il rimedio funziona.

**La sesta riga dice invece dove il meccanismo non arriva, e va letta accanto alla quinta.** La tabella degli esiti `ed25519-speccheck` dichiarava `accept` al vettore 8, dove la regola scritta due paragrafi sopra di essa produce `reject`: il vettore verifica soltanto se `k` è calcolato su un `R` ridotto, che è esattamente ciò che la regola vieta. Era stata compilata a mano in [SPEC-001] e non era mai stata eseguita da nessuno. **Non l'ha trovata lo strumento**, e non poteva: `published_artifacts.py` verifica forme e coerenze fra copie, e dichiara nella propria intestazione di non verificare la correttezza semantica di alcun valore. L'ha trovata **[REVIEW-018]**, eseguendo un oracolo indipendente scritto da zero — cioè la review adversariale, non una guardia. È la prima occorrenza della famiglia trovata da una review.

La conseguenza pratica è che una tabella di esiti **con un oracolo eseguibile** è meccanizzabile, ma non da questo strumento: appartiene alla suite di conformità, che è il luogo a cui `published_artifacts.py` assegna esplicitamente la ricomputazione. La remediation di [SPEC-012] l'ha fatto — `speccheck_conformance.rs` **estrae la tabella dal documento a tempo di compilazione** invece di trascriverla, e fallisce se il documento e un'implementazione conforme divergono. La copia trascritta che quella remediation ha rimosso era la causa meccanica per cui la gate non poteva accorgersene: confrontava l'implementazione con sé stessa attraverso due copie.

Il quarto è il più istruttivo: la spec che *introduceva* quella regola ha corretto due fixture e ne ha lasciata una che la violava, e a trovarla è stato uno strumento versionato che un finding minore aveva imposto di scrivere. **Uno script non versionato non l'avrebbe mai trovata.**

**Perché il difetto è più grave di un valore sbagliato.** Un artefatto pubblicato del registro di conformità non è documentazione: è l'oracolo su cui un'implementazione indipendente si misura. Un caso di prova costruito su parametri inammissibili **asserisce un comportamento per uno stato che nessuna rete conforme può raggiungere** — chi lo implementa scrive codice per un caso impossibile, e chi lo verifica ottiene un verde che non significa nulla. È già normativo, dal 2026-08-25, che una suite validi i propri fixture di parametri prima di usarli; ciò che mancava è l'obbligo speculare su chi **cambia le regole**.

**Perché è ricorso.** Ogni spec ha corretto gli artefatti che *toccava*, ed è il comportamento naturale: si guarda dove si sta lavorando. Ma una regola di validità nuova non invalida gli artefatti che la spec tocca — invalida **quelli che nessuno sta guardando**.

## Decision

**Ogni spec che introduce o modifica una regola di validità dichiara una gate `before-submit` che impone una passata su tutti gli artefatti pubblicati, non solo su quelli che la spec tocca.**

Tre precisazioni che ne fanno un meccanismo e non un buon proposito.

**1. È una gate, non una prassi.** Dichiarata nella sezione *Required verification* della spec, con evidenza `transcript`, e la spec non passa a `submit` senza. È la forma che AGENT-007 ha indicato, e la ragione è che una prassi si dimentica esattamente nelle passate in cui l'attenzione è altrove — che sono quelle in cui il difetto è ricorso.

**2. La verifica è eseguita da uno strumento versionato nel repository**, mai da uno script temporaneo. Non è pedanteria: gli script `scratch/*.py` di [SPEC-009] non esistevano nell'albero, quindi la loro evidenza non era verificabile da nessuno, e la fixture sbagliata è sopravvissuta a quella passata. Lo strumento che l'ha trovata era versionato ed esegue in un secondo.

**3. Lo strumento deve saper fallire, e va verificato in negativo.** [SPEC-009] ha stabilito il precedente: la costante di confronto era invecchiata e lo strumento riportava una discordanza inesistente, cioè gridava al lupo su un allineamento corretto. Un falso positivo insegna a non fidarsi, e uno strumento in cui nessuno ha fiducia non viene eseguito — il che riporta al punto di partenza. La guardia va quindi provata reintroducendo il difetto e osservandola fallire: **una guardia che non sa fallire non è una guardia.**

**Il perimetro è «tutti gli artefatti pubblicati»**, e comprende il registro di conformità, gli esempi normativi e ogni valore che i documenti espongono come atteso. Non comprende gli artefatti interni al brain.

## Alternatives considered

- **Lasciarlo alla disciplina, registrandolo in una pagina di conoscenza.** È ciò che il progetto ha di fatto fatto finora, e ha prodotto quattro occorrenze. La disciplina funziona quando la si ricorda, e la si ricorda quando si sta già guardando nella direzione giusta — cioè mai in questo caso.
- **Una verifica di conformità continua in CI su tutti gli artefatti a ogni commit.** Sarebbe più forte e va valutata quando `coblox-core` avrà una copertura sufficiente dei documenti. Non è adottata adesso perché oggi coprirebbe solo ciò che il crate già implementa, quindi darebbe un verde più stretto del vero — e un verde più stretto del vero è il difetto che questa ADR esiste per chiudere.
- **Estendere l'obbligo esistente sulle suite di conformità** anziché crearne uno nuovo. Scartata perché i due obblighi hanno soggetti diversi: quello esistente vincola chi **usa** i fixture, questo vincola chi **cambia le regole**. Confonderli lascerebbe scoperto proprio il secondo, che è il caso che si è ripetuto.

## Consequences

- Ogni spec di questa classe porta una gate in più, con un costo di esecuzione di secondi e un costo di scrittura di due righe.
- Il progetto acquista un secondo strumento versionato per famiglia di regole, che è un bene di per sé: `sim/tools/protocol_hashes.py` e `sim/tools/reward_rules.py` esistono per questo motivo e hanno già trovato un difetto ciascuno.
- Chi scrive una spec deve sapere **quali** siano gli artefatti pubblicati, il che è un esercizio utile in sé: la quarta occorrenza esiste perché quell'inventario non era mai stato fatto.
- `HostingRateCardBody` è oggi un documento governato senza alcun oggetto di limiti, segnalato da AGENT-007 come residuo fuori ambito. Quando M-06 toccherà l'hosting, questa gate si applicherà anche lì.

## Review conditions

Rivedere se: una verifica continua in CI su tutti gli artefatti diventa praticabile, nel qual caso questa gate diventa ridondante e va ritirata invece che mantenuta per abitudine; oppure se il perimetro «tutti gli artefatti pubblicati» si rivelasse ambiguo su una classe di artefatti nuova, che è la forma in cui questa ADR fallirebbe. **Non rivedere** perché in una passata la gate non ha trovato nulla: è il caso previsto e non è evidenza che sia inutile.
