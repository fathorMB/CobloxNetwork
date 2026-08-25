---
id: REVIEW-012
# Note: Quote the title if it contains a colon
title: "Review di SPEC-008 — Core del ledger in Rust"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-008
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-012-EVENT-001"
    timestamp: "2026-08-25T16:05:32.080162+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-012-EVENT-002"
    timestamp: "2026-08-25T16:06:38.557159600+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Undici criteri su undici, senza alcun finding a carico dell'implementazione. GATE-LEAD-REPRO eseguita dal Lead: 103 test verdi e provenienza degli attesi verificata per campione su cinque valori che il Lead aveva ricalcolato personalmente durante SPEC-006, tutti presenti come costanti letterali coincidenti col documento. GATE-CI-GREEN chiusa sulla pipeline reale, run 32856348095 sul commit 27187e7, verde su tutti e cinque i job inclusi la matrice Linux e il build Android che l'implementatore non poteva eseguire localmente. Sedici righe su sedici del registro di conformita riprodotte al primo tentativo, con le non coperte dichiarate con la ragione. I tre rilievi emersi sono due difetti di scrittura del Lead nella spec stessa e un difetto della specifica di protocollo, tutti trovati e segnalati dall'implementatore invece di essere aggirati; il terzo diventa DEBT-011."
    evidence_refs: ["SPEC-008", "SPEC-006"]
    implementation_agent: "AGENT-001"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
related_specs: [SPEC-008]
related_decisions: [ADR-003]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> accepted"
---
# Review di SPEC-008 — Core del ledger in Rust

## Outcome

**Accettata.** Undici criteri su undici. Nessun finding a carico dell'implementazione: i tre rilievi emersi sono **due difetti di scrittura del Lead in questa stessa spec** e **un difetto della specifica di protocollo**, tutti e tre trovati e correttamente segnalati dall'implementatore invece di essere aggirati.

È la prima implementazione reale del progetto, ed è arrivata con sedici fixture su sedici riprodotte al primo tentativo.

## Acceptance-criteria compliance

Tutti soddisfatti. Verifiche del Lead, eseguite e non riprese dall'evidenza:

- **`GATE-LEAD-REPRO`, che è la mia.** Suite rieseguita: 103 test verdi su sette file. Provenienza degli attesi verificata per campione su **cinque valori che avevo ricalcolato personalmente durante [SPEC-006]** — `policy_hash`, `consensus_parameters_hash`, `parameter_set_hash`, `candidate_root` ed `election_seed`: tutti presenti come costanti letterali coincidenti col documento. Il file dichiara in testa la propria regola di provenienza e **nomina la modalità di fallimento che la spec aveva previsto**, cioè il test che genera l'attesa dall'implementazione. La gate esisteva per intercettare quel difetto e il difetto non c'è.
- **`GATE-CI-GREEN`**, chiusa dal Lead per la ragione in *Review findings*: run 32856348095 sul commit `27187e7`, **verde su tutti e cinque i job**, inclusi la matrice Linux e il build Android che l'implementatore non poteva eseguire localmente.
- **Fixture**: 16 righe su 16 del registro, più `REVL-0`, la radice vuota, entrambe le grafie base64url, il `node_id` ricalcolato, `SMT-1`, i confini del quorum, le cinque righe del pavimento Argon2id, il confine del tetto di quota del creatore, l'esempio numerico dell'epoca 3 per intero e 17 serializzazioni canoniche byte per byte. Le non coperte sono **dichiarate con la ragione**, come la gate imponeva, e le ragioni sono buone: mancano gli oracoli, non l'impegno.

## Code observations

Tre convenzioni meritano di essere registrate, perché il resto di M-02 le eredita e sono **strutturali e non disciplinari** — cioè non dipendono dal fatto che il prossimo implementatore si ricordi di rispettarle.

**La canonicalizzazione è l'unica strada.** Il tipo `Json` non ha varianti `Number` e `Null`: tre restrizioni di canonicalizzazione diventano affermazioni su *quali programmi compilano* invece di controlli a runtime. Ne discende onestamente che la metà «formattazione numerica ES6» di RFC 8785 è irraggiungibile, e l'implementatore lo dichiara invece di lasciarlo credere implementato.

**La separazione di dominio è imposta dal tipo.** L'unico modo di iniziare una preimmagine scrive da sé il dominio e il byte zero, e non può essere convinto a non farlo. Era il difetto peggiore da diagnosticare — un errore di dominio produce un hash plausibile e sbagliato — ed è stato reso inesprimibile.

**Nessun valore di lancio nel crate.** `ValidatedConsensusParameters` non ha costruttore diverso dalla validazione. Il rischio che la spec nominava, le costanti compilate che avrebbero trasformato [SPEC-007] in una modifica al codice, è chiuso per costruzione.

Da segnalare anche il rifiuto di spedire un verificatore Ed25519 senza i vettori speccheck come oracolo. Il ragionamento è quello giusto e vale la pena citarlo: senza oracolo un verificatore è **indistinguibile da uno corretto fino a una divisione della catena**. Ha spedito le preimmagini di firma, deterministiche e testate, più il punto di innesto con il contratto scritto.

## Tests and verification

103 test. Notevoli due scelte che vanno oltre il richiesto: la lista chiusa delle **incapacità** del light client è portata nel codice come costante, e due test **asseriscono l'indistinguibilità dichiarata invece di negarla** — se un cambiamento futuro li facesse fallire rifiutando la transizione censurata, starebbe rivendicando una garanzia che il protocollo non dà. È il modo giusto di rendere eseguibile una lista di ciò che non si può fare.

## Production quality and documentation compliance

Conforme. Una sola dipendenza, `sha2`, con licenza già ammessa. `docs/protocol/` non toccato.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=specification | severity=high | criterion=Nessuna ambiguità che produca divisioni della catena | remediation=Debito separato, fuori dallo scope di questa spec

**`lifecycle_u8` non è definito da nessun documento.** La preimmagine di `app_leaf` in `ledger.md` lo commette, ma i valori del ciclo di vita compaiono ovunque solo come stringhe `active`, `grace`, `suspended`: nessun documento assegna loro un valore numerico. Verificato dal Lead. Due implementazioni conformi calcolano quindi `app_leaf` diverse per lo stesso stato, cioè `state_root` diverse, cioè **una divisione della catena sul primo conto di app che non sia `active`**. Il registro di conformità non copre `app_leaf`, quindi le fixture non lo avrebbero intercettato.

Trovato scrivendo il codice, che è l'unico modo in cui poteva emergere. `coblox-core` usa una codifica provvisoria documentata come tale sul tipo e bloccata da un test che **dichiara di non essere una prova di correttezza** — la disposizione corretta, dato che `docs/protocol/` è sola lettura per questa spec. Diventa [DEBT-012].

RF-002 | category=process | severity=medium | criterion=Le gate devono essere soddisfacibili da chi deve soddisfarle | remediation=Nessuna sull'implementazione; correzione di metodo del Lead

**`GATE-CI-GREEN` era strutturalmente non soddisfacibile.** Il Lead l'ha scritta chiedendo la pipeline verde «sul commit consegnato» a un agente cui il dispatch vieta commit e push: non esiste un commit consegnato. L'implementatore ha rifiutato di spuntarla, ha forzato `spec_submit` registrandone la ragione, e ha eseguito localmente **ogni comando dei cinque job che la sua macchina poteva eseguire**, dichiarando i tre che non poteva. È la condotta corretta: spuntarla sarebbe stato falso.

La gate è stata chiusa dal Lead sulla pipeline reale, che è l'unico soggetto che poteva farlo. **Il difetto di metodo resta del Lead** e la sua lezione è generale: una gate `owner=agent` non può richiedere un'azione che il ruolo di quell'agente ha vietato.

RF-003 | category=specification | severity=low | criterion=L'analisi dell'esistente deve essere verificata | remediation=Nessuna; corretta dall'implementatore

**L'analisi dell'esistente della spec affermava che `unsafe_code = "forbid"` fosse già attivo su `coblox-core`. Non lo era.** Il Lead aveva letto il valore di `apps/desktop/src-tauri/Cargo.toml` attribuendolo al workspace, che invece dichiara `allow` per via di UniFFI. Verificato. L'implementatore l'ha rilevato e ha aggiunto l'attributo a livello di crate, senza toccare `coblox-ffi`: rimedio corretto e minimo.

## Required follow-up

**[DEBT-012]** per `lifecycle_u8`, severità high, con la codifica provvisoria del crate da sostituire quando la specifica assegnerà i valori.

**Una spec dedicata al verificatore Ed25519**, con la tabella speccheck 0–11 come proprio gate, **prima di qualunque devnet**. Raccomandazione dell'implementatore, condivisa dal Lead: è la stessa forma di argomento che regge il registro di conformità, cioè che senza oracolo un'implementazione crittografica non è verificabile.

**Nota sul valore dichiarato inventato**: l'esempio numerico non pubblica `validator_max_set_size`, che il blocco di vincoli richiede; il fixture lo pone al minimo che ammette il set dell'esempio e non entra in alcuna asserzione. Dichiararlo era la cosa giusta.

## Final decision

Accettata. [SPEC-008] passa a `done`.
