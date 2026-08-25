---
id: REVIEW-009
# Note: Quote the title if it contains a colon
title: "Review di SPEC-006 — Regola di elezione e rotazione del set di validatori"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-006
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-009-EVENT-001"
    timestamp: "2026-08-25T13:12:49.777042600+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-009-EVENT-002"
    timestamp: "2026-08-25T13:14:10.280415300+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Undici criteri su undici soddisfatti e lavoro di qualita alta, con ogni verifica numerica rieseguita dal Lead in modo indipendente: i tre hash delle fixture PD-0 ricalcolati, con il metodo validato prima su una fixture non modificata, e i diciotto valori dell'esempio dell'epoca 3 riprodotti tutti. Un solo finding, RF-001, di severita media e di una riga. identity.md riga 398 afferma che in caso di revoca un light client puo vedere la transizione invece di doversi fidare di una affermazione al riguardo, mentre ledger.md ha ritrattato esattamente questa affermazione su se stesso scrivendo che un light client osserva una transizione se avviene ma non puo stabilire che fosse dovuta. Il documento sorella conserva la versione gia riconosciuta come falsa. E la classe di difetto che il progetto dichiara peggiore di una affermazione mancante, quindi non viene rimandata a debito. Il finding e stato individuato e segnalato dall'implementatrice stessa, che ha correttamente evitato di correggerlo perche la spec escludeva la regola di revoca: la valutazione del Lead e che la correzione non tocchi quella regola, perche cambia una descrizione di cosa un light client possa stabilire, che e materia di questa spec."
    evidence_refs: ["SPEC-006", "DEBT-005"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-009-EVENT-003"
    timestamp: "2026-08-25T13:16:02.774171200+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "RF-001 corretto. docs/protocol/identity.md, paragrafo sulla revoca di un validatore in carica: la frase \"a light client can see that transition rather than having to trust a claim about it\" e sostituita da una formulazione che separa i due verificatori e non sovrastima. Un full node vede la transazione revoke_identity e applica la regola per intero; un light client non vede transazioni, osserva UNA transizione se avviene e puo verificare che sia di sola rimozione, ma NON puo stabilire che fosse dovuta, e per la parte coperta si affida alla lista revoked_validators del proprio weak subjectivity checkpoint, con il limite gia dichiarato in ledger.md. La regola di revoca non e stata toccata: nessuna modifica a ledger.md ne ai suoi punti 1-9, diff confinato a quel solo paragrafo di identity.md (16 righe inserite, 4 rimosse). Spec rimasta in review per tutta la remediation, spec_start non chiamato. Verifica: 0 ancore interne rotte sui quattro documenti di docs/protocol; transcript numerico rieseguito, 42 controlli superati e 0 falliti. Evidenza aggiornata nella sezione \"Remediation RF-001\" della spec. Nessun commit e nessun push."
    evidence_refs: ["SPEC-006", "DEBT-005"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-009-EVENT-004"
    timestamp: "2026-08-25T14:47:15.392450+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "RF-001 rimediato e verificato dal Lead: la frase di identity.md riga 398 separa ora full node e light client e riusa la formulazione della ritrattazione gia fatta da ledger.md invece di inventarne una parallela, dichiarando anche la dipendenza dal checkpoint di weak subjectivity e il suo limite. La regola di revoca non e stata toccata e l'anchor nuovo risolve. Il lavoro complessivo e stato poi sottoposto a GATE-SECREVIEW con AGENT-007, che dopo quattro giri e tredici finding ha accettato con REVIEW-010."
    evidence_refs: ["SPEC-006", "REVIEW-010", "DEBT-005"]
    implementation_agent: "AGENT-002"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
related_specs: [SPEC-006]
related_decisions: [ADR-001, ADR-007, ADR-008]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "transitioned changes-requested -> accepted"
---
# Review di SPEC-006 — Regola di elezione e rotazione del set di validatori

## Outcome

**Changes requested, per un singolo finding di una riga.** Undici criteri su undici sono soddisfatti e il lavoro è di qualità alta: il Lead ha rieseguito in modo indipendente ogni verifica numerica invece di accettare l'evidenza, e tutto torna. L'unico finding non riguarda la regola scritta ma una contraddizione documentale che l'implementatrice stessa ha individuato e correttamente segnalato invece di correggere di propria iniziativa, essendo fuori dallo scope dichiarato. Poiché è precisamente la classe di difetto che il progetto dichiara peggiore di un'affermazione mancante, non viene rimandato a debito.

La spec resta in `review`; la remediation è continuazione del ciclo, non azzeramento.

## Acceptance-criteria compliance

Tutti gli undici criteri sono soddisfatti. Verifiche **rieseguite dal Lead**, non riprese dall'evidenza:

- **Fixture di conformità pubblicate.** È la modifica a più alto rischio della consegna, perché due hash già pubblicati sono stati sostituiti. Il Lead ha ricalcolato in modo indipendente tutti e tre gli hash dei documenti `PD-0`, validando prima il proprio metodo su una fixture **non** modificata: `parameter_set_hash` riproduce esattamente il valore preesistente, il che dimostra che l'interpretazione di JCS, dominio e `chain_id` è corretta. Applicato lo stesso metodo, `policy_hash` e `consensus_parameters_hash` riproducono esattamente i nuovi valori. La sostituzione è dovuta all'estensione dei corpi governati ed è **corretta**: lasciare i vecchi valori avrebbe pubblicato due fixture di conformità sbagliate.
- **`GATE-DETERMINISM`.** Il Lead ha ricalcolato l'intero esempio dell'epoca 3: cinque foglie candidate, la foglia vuota, sei nodi interni, `candidate_root`, `election_entropy`, `election_seed` e i tre `election_ticket`. **Diciotto valori su diciotto coincidono.** Verificati anche l'ordinamento crescente per ticket e la formula `fills = min(max(0, 5-2), 2, 3) = 2`, che fa vincolare il cap ed esclude il nodo `08`. La gate è soddisfatta nel merito e non a parole: l'esempio è davvero rifacibile senza scrivere codice.
- **Accoppiamento dei parametri.** L'affermazione che `ceil(V/T) <= c` e `3*c*m <= V` siano congiuntamente soddisfacibili solo per `T >= 3*m` è stata verificata algebricamente dal Lead e regge: dalla seconda `c <= V/(3m)`, dalla prima `V/T <= c`, quindi `V/T <= V/(3m)` e `T >= 3m`. È una **scoperta reale e non ovvia**, non una riformulazione: una rete che voglia che la cattura richieda almeno `m` confini non può contemporaneamente volere mandati brevi. Averla resa una regola di validità sul documento dei parametri, anziché una raccomandazione, impedisce che la coppia venga scelta in modo indipendente e si riveli contraddittoria su una catena viva.
- **`GATE-LIGHTCLIENT`.** Soddisfatta oltre la richiesta. Due liste chiuse, nove capacità e sei incapacità, e una dichiarazione finale deliberatamente più stretta di *"un light client verifica l'elezione"*: stabilisce che il set è **lawfully shaped and lawfully rotating**, non che sia **corretto**.
- **Vincolo di [ADR-007].** L'eleggibilità è una **soglia binaria** su `contribution_score` costruito solo da evidenze `storage` e `compute`; l'evidenza `availability` contribuisce **zero**. Il vincolo "mai al solo uptime" non è solo rispettato, è reso strutturalmente inesprimibile.
- **Test di [ADR-008].** I tre punti sono dichiarati con esito e motivazione. Il punto 1 è argomentato bene: la soglia binaria e il potere di voto uniforme esistono *perché* una graduatoria per lavoro contribuito fallirebbe quel punto, e l'alternativa è stata scartata per quel motivo. Il residuo di numerosità è dichiarato e ricondotto ad `α`, non nascosto.
- **Nessun parametro numerico inventato.** Rispettato.

## Code observations

Nessun codice: il deliverable è specifica, come da scope.

Tre scelte di progetto meritano di essere registrate come migliori dell'ovvio.

**La regola che è stata trovata lavorando, non chiesta dalla spec.** L'obbligo che a ogni altezza che non sia un confine di elezione o una transizione forzata da revoca `next_validator_set_hash` debba uguagliare `validator_set_hash`. Senza di essa l'intera regola di elezione era **aggirabile cambiando set a un'altezza arbitraria**: si sarebbe scritta una regola elegante su un cancello lasciato aperto di fianco. È il finding più importante della consegna e non compariva in [DEBT-005].

**L'impegno a costo zero.** `ElectionRecord` vive dentro `ValidatorSet`, quindi è impegnato da `validator_set_hash`, che l'altezza precedente già impegna come `next_validator_set_hash`, campo dell'header. Il requisito di [DEBT-005] — impegno nell'header che consenta il ricalcolo a posteriori — è soddisfatto **transitivamente ed esattamente**, senza un campo nuovo per blocco. L'alternativa scartata, un campo dedicato, sarebbe costata ogni blocco per sempre pur essendo autenticata dallo stesso quorum.

**La necessità di `validator_candidacy` è stata dimostrata, non assunta.** `key_binding_signature` è firmata su una specifica `activation_height`: senza dichiarazione anticipata il set uscente dovrebbe inventare le chiavi di consenso altrui, che è il difetto stesso che si stava chiudendo. Ne discende anche che cessare di candidarsi *è* l'uscita volontaria, senza bisogno di un meccanismo separato.

## Tests and verification

Le due gate `before-submit` sono soddisfatte e verificate due volte: dall'implementatrice e dal Lead in modo indipendente. `GATE-SECREVIEW` è `before-done` e resta da soddisfare.

Da segnalare come merito di metodo: la **casualità è stata attaccata per prima**, come la spec chiedeva, e l'esito è stato riportato onestamente invece di essere risolto a parole. Un beacon da ID di blocco *è* grindable dal proponente, l'aggregazione su più blocchi riduce ma non elimina il best-of-`G`, e in v0 senza VDF né firme a soglia non è eliminabile. La risposta di progetto è la sola difendibile: **rendere l'invariante indipendente dal seme**, così che il grinding produca bias e mai scelta, con il guadagno comunque limitato dal cap e con la scadenza del mandato che non è funzione del seme. Il residuo è dichiarato con la sua forma quantitativa.

## Production quality and documentation compliance

Conforme. `threat-model.md` aggiornato con `TM-18` e `TM-09` come *mitigated in specification* e gli scenari storici conservati; `SEC-REQ-13` diviso fra la parte coperta in M-02 e quella aperta in M-07; §6.1 annotata anche con la leva **non** adottata e il perché.

Apprezzabile in particolare il rifiuto di rendere `AT-09` "passato" per intero: la parte sull'equivocazione come transazione di evidenza verificabile è fuori scope e viene **nominata come non coperta**, invece di essere lasciata credere coperta. Ed è stato segnalato che la preparazione di `AT-10` è ora **sbagliata**, perché "uptime da datacenter" non è più il vettore dal momento che l'availability vale zero — un test che nessuno avrebbe rieseguito accorgendosene.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=documentation | severity=medium | criterion=Nessuna dichiarazione di sicurezza sovrastimata nei documenti di protocollo | remediation=Correggere la frase di `identity.md`

`docs/protocol/identity.md`, riga 398, afferma che in caso di revoca di un validatore in carica *"a light client can see that transition rather than having to trust a claim about it"*. `ledger.md` ha **ritrattato esattamente questa affermazione su sé stesso**, scrivendo che una dichiarazione di sicurezza sbagliata è peggio di una mancante e che un light client *"observes a transition if one happens; it cannot establish that a transition was due"*. Il documento sorella conserva quindi la versione già riconosciuta come falsa, e chi legga solo `identity.md` ne trae una garanzia che il protocollo non offre.

Il finding è stato **individuato e segnalato dall'implementatrice**, che ha correttamente scelto di non correggerlo perché la spec escludeva la regola di revoca. La valutazione del Lead è che la correzione **non tocca la regola**: cambia una descrizione di ciò che un light client può stabilire, che è materia di questa spec, e la allinea alla posizione che `ledger.md` ha già preso. Non viene quindi rimandato a debito.

## Required follow-up

**Remediation richiesta, una frase.** Allineare `identity.md` riga 398 alla posizione di `ledger.md`: un light client osserva una transizione se avviene e verifica che sia di sola rimozione, ma non può stabilire che fosse dovuta, e per la parte coperta si affida al proprio checkpoint. Non modificare la regola di revoca.

**Dopo la remediation:** `GATE-SECREVIEW`. AGENT-007 rivede la regola come superficie di sicurezza. Il debito chiuso è `critical` e la review di sicurezza non è facoltativa.

## Final decision

Changes requested. La spec resta in `review`.
