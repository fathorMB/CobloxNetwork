---
id: REVIEW-028
# Note: Quote the title if it contains a colon
title: "Review of SPEC-017"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-017
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-028-EVENT-001"
    timestamp: "2026-08-26T10:43:05.304133600+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Un finding medium, dentro GATE-TWO-DERIVATIONS, cioe' la gate portante della spec. Il resto e' forte: 163 test da 151, gate di ADR-012 verde con prova in negativo su 15 mutazioni e 111 probe individualmente, nessun valore pubblicato mosso.\n\nIl Lead ha attaccato invece di rieseguire soltanto, come la regola nuova di lead-claims-discipline impone. Quattro attacchi non si sono rotti: l'indipendenza della strada Python (nessun import di coblox_core, e il punto di contatto CONSENSUS_BODY e' fra due strumenti Python mentre il Rust costruisce il proprio oggetto); che gli attesi siano letti dalla tabella di README e non cablati; GATE-VERIFIER-UNCHANGED, verificato con git diff --numstat su verifier.rs che da' 32 aggiunte e zero rimozioni invece di fidarsi del conteggio dei test; e che nessun valore pubblicato si fosse mosso.\n\nRF-001: per la prima genesi entrambe le strade si confrontano con la tabella pubblicata di README, cioe' con un terzo che nessuna ha scritto. Per la seconda no: il test stampa i quattro valori e asserisce solo assert_ne contro la prima. L'argomento dell'implementatore per non asserire e' corretto - asserire una costante presa dalla strada Python farebbe confermare quella strada invece di incontrarla - ma la conseguenza e' che l'accordo e' stato osservato una volta a occhio e non e' asserito da nulla. Il test dimostra che i valori si muovono, non che si muovono insieme. E la seconda genesi esiste proprio perche' la prima lascerebbe costante la lunghezza in byte del network_id: il caso della varianza c'e', ma la sua gate e' un calcolo e non una guardia.\n\nRegistrato che l'implementatore ha contraddetto la spec su un punto e aveva ragione: la circolarita' non passa solo per validator_set_hash, gli ingressi circolari sono tre, e una regola scritta sull'elenco della spec sarebbe stata troppo stretta senza che alcuna gate lo rilevasse. E che ha dichiarato una famiglia 4 invece di nasconderla sulla clausola key_binding_signature.\n\nDue cose sono del Lead: il debito su election_epoch, e l'off-by-one nella riga di riepilogo di published_artifacts_negative.py."
    evidence_refs: ["SPEC-017", "DEBT-020", "DEBT-021", "SKILL-001"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-028-EVENT-002"
    timestamp: "2026-08-26T10:50:25.891460100+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "RF-001 chiuso pubblicando GEN-1 come fixture nel registro di README.md, cosi' che entrambe le strade si confrontino con il documento e nessuna con l'altra. Da verificare dal Lead."
    evidence_refs: ["SPEC-017"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-028-EVENT-003"
    timestamp: "2026-08-26T10:50:43.365086500+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificata dal Lead rieseguendo: 164 test, published_artifacts.py PASS con 112 candidati C10, prova in negativo PASS con 15 mutazioni su 11 classi piu' tutte e 112 le probe individualmente, protocol_hashes.py PASS senza valori preesistenti mossi, genesi_chain_id.py verde.\n\nIl metodo con cui ha fatto convergere le due strade e' migliore di quello che la review suggeriva, e va registrato. Il Lead aveva proposto di pubblicare i valori e far confrontare entrambe le strade col documento; l'implementatore ha fatto quello e in piu' ha rifiutato di spuntare un accordo osservato a occhio. Ha pubblicato le righe **a zero**, ha fatto fallire prima una strada e poi l'altra, e ha guardato quale valore ciascuna nominasse contro il documento sbagliato: 0x6ba582b4... compare in entrambe le trascrizioni, prodotto prima che il documento lo contenesse e senza che nessuna delle due potesse copiarlo dall'altra. Poi ha riempito le righe. E' una prova dell'accordo, non una sua asserzione per costruzione.\n\nHa inoltre conservato gen1_moves_every_derived_value_away_from_gen0 come asserzione propria, con la ragione: due tabelle di digest che per caso coincidessero soddisferebbero il confronto con il documento e non direbbero nulla sul nome di rete che entra nella derivazione. La ragione per cui GEN-1 esiste e' ora asserita invece che implicita.\n\nSul costo che la review gli aveva chiesto di valutare, ha risposto e la risposta regge: GEN-1 e' GEN-0 con un solo campo diverso e non insegna alcuna configurazione che GEN-0 non mostri gia', quindi non e' famiglia 1; e il file terzo non pubblicato sarebbe stato peggio, perche' un oracolo fuori dal registro e' un oracolo che nessuna implementazione indipendente riceve, cioe' esattamente cio' che questa spec esiste per non fare. Una probe C10 nuova tiene GEN-1 al suo posto, perche' una fixture che differisce per un campo verrebbe cancellata dal primo lettore che riordina.\n\nDue note di igiene che il Lead registra come corrette: ha annotato invece di riscrivere l'affermazione della consegna diventata falsa con la remediation, perche' la traccia di cio' che RF-001 ha trovato vale piu' della frase pulita; e ha aggiornato il censimento delle derivazioni non univoche perche' citi DEBT-028 invece di dire che va aperto."
    evidence_refs: ["SPEC-017", "REVIEW-028"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-028-EVENT-004"
    timestamp: "2026-08-26T10:50:59.247499100+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata dopo un giro di remediation. RF-001 chiuso e verificato dal Lead rieseguendo: 164 test, 112 probe C10, prova in negativo su ciascuna individualmente, nessun valore preesistente mosso.\n\nGATE-SECREVIEW resta da attestare ed e' before-done: la separazione di dominio e' la difesa che impedisce a una firma di valere in due contesti, ed e' materia di AGENT-007.\n\nIl valore di questa spec non e' la fixture ma l'elenco che chiedeva anche se vuoto: cinquantuno preimmagini classificate, cinque ambiguita' chiuse dentro la spec, e una lasciata aperta che e' diventata DEBT-028 - election_epoch dipende da un parametro governato senza che il documento dica quale versione valga, terza porta sulla stessa famiglia dopo DEBT-012 e DEBT-020.\n\nRegistrato che l'implementatore ha contraddetto la spec e aveva ragione: gli ingressi circolari dell'intestazione di genesi sono tre e non uno, e la regola formulata sul criterio invece che sull'elenco li copre tutti. Una regola scritta sull'elenco della spec sarebbe stata troppo stretta senza che alcuna gate lo rilevasse."
    evidence_refs: ["SPEC-017", "DEBT-020", "DEBT-021", "DEBT-028"]
    implementation_agent: "AGENT-001"
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [review]
activity:
  - date: 2026-08-26
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-26
    action: "recorded review remediation"
  - date: 2026-08-26
    action: "recorded review remediation-verification"
  - date: 2026-08-26
    action: "transitioned changes-requested -> accepted"
---
# Review

## Outcome

**Changes requested, un finding medium**, e sta esattamente dentro `GATE-TWO-DERIVATIONS`, cioè la gate portante di questa spec.

Il resto del lavoro è forte, e la parte che vale più della fixture è quella che la spec non chiedeva: l'elenco delle derivazioni non univoche non è vuoto, contiene **cinquantuno** preimmagini classificate, e ne lascia **una aperta** che è un difetto reale del protocollo.

## Acceptance-criteria compliance

Riverificato dal Lead rieseguendo: **163 test** da una baseline di 151. `published_artifacts.py` `PASS`; `published_artifacts_negative.py` `PASS` con 15 mutazioni su 11 classi **più ogni probe individualmente**; `protocol_hashes.py` `PASS` con ogni valore pubblicato riprodotto.

**Nessuna preimmagine già pubblicata è cambiata**, che era il caso in cui l'implementatore doveva fermarsi e riportare. Il segnaposto a 32 byte zero è compatibile con tutto ciò che era in albero.

**La regola è formulata sul criterio invece che sull'elenco**, ed è la scelta giusta per una ragione che l'implementatore ha trovato contraddicendo la spec: l'*Existing-project analysis* diceva che la circolarità passa per `validator_set_hash`. È vero e **non esaurisce il caso**: gli ingressi circolari dell'intestazione di genesi sono tre — il `chain_id_32` della preimmagine di `block_id`, il documento `consensus_parameters`, e il set attraverso `key_binding_signature`. Una regola scritta sull'elenco della spec sarebbe stata **troppo stretta**, e nessuna gate l'avrebbe rilevato.

## Cosa ho attaccato senza riuscire a romperlo

Questa sezione esiste perché [REVIEW-025] non ce l'aveva e ha lasciato passare un finding `high` esattamente dove lodava. La regola è in `.lmbrain/knowledge/lead-claims-discipline.md` e la guardia in `sim/tools/lead_claims_check.py`.

**L'indipendenza della strada Python, che è ciò su cui l'intera gate poggia.** Se il tool Python leggesse in qualunque modo il Rust, le due strade sarebbero una. Cercati import, riferimenti a `coblox_core`, invocazioni di `cargo` o `subprocess` in `sim/tools/genesis_chain_id.py`: **non ce ne sono**. Le sole dipendenze sono `hashlib`, `json`, `pathlib`, `re`, `sys` e `CONSENSUS_BODY` da `protocol_hashes.py`.

Ho attaccato anche quest'ultima, che è il punto di contatto meno ovvio: `CONSENSUS_BODY` è condiviso fra **due strumenti Python**, e il Rust costruisce il proprio oggetto per conto suo. Le due *derivazioni* restano quindi disgiunte; ciò che è condiviso è un ingresso fra Python e Python, non fra le due strade. **Non si è rotto.**

**Che gli attesi siano letti e non cablati.** Un oracolo che porta dentro di sé la risposta non è un oracolo. `genesis_chain_id.py` estrae i valori attesi dalla tabella di `docs/protocol/README.md` con una regex sulle righe del registro, e fallisce esplicitamente se la riga non c'è. **Non si è rotto.**

**Che la logica di verifica delle firme sia davvero invariata**, che è `GATE-VERIFIER-UNCHANGED`. Non mi sono fidato del conteggio dei test: `git diff --numstat core/coblox-core/src/verifier.rs` dà **`32 0`** — trentadue aggiunte, **zero rimozioni**. `verify_consensus_ed25519` non è toccata, e nessuna riga esistente è stata modificata. **Non si è rotto.**

**Che qualche valore pubblicato si fosse mosso in silenzio.** `protocol_hashes.py` riproduce tutto. **Non si è rotto.**

Ciò che si è rotto è il quinto attacco, ed è sotto.

## Review findings

**RF-001 — medium — la seconda genesi non è confrontata fra le due strade, ed è proprio il caso che porta la lezione della varianza.**

`GATE-TWO-DERIVATIONS` chiede che due strade senza codice in comune producano lo stesso `chain_id`. Per la **prima** genesi questo è mechanizzato bene, e per la via giusta: entrambe le strade si confrontano con la **tabella pubblicata** di `README.md`, cioè con un terzo che nessuna delle due ha scritto.

Per la **seconda** genesi no. `core/coblox-core/tests/genesis_derivation.rs`, in `the_second_genesis_moves_every_derived_value`, **stampa** i quattro valori e asserisce soltanto `assert_ne!` contro quelli della prima. Il commento dichiara la ragione, ed è un buon argomento: *«asserire una costante presa dalla strada Python farebbe confermare quella strada invece di incontrarla»*. È corretto — copiare la risposta dell'altra strada è esattamente ciò che la gate vieta.

**Ma la conseguenza è che l'accordo sulla seconda genesi non è verificato da nulla.** È stato osservato una volta, a occhio, confrontando due output. Se una delle due strade derivasse domani, nessun test fallirebbe. Quel test dimostra che i valori **si muovono** quando cambia il nome della rete; non dimostra che si muovono **insieme**.

E la seconda genesi esiste precisamente perché la prima da sola lascerebbe la lunghezza in byte del `network_id` costante in tutti i casi — la lezione di `GATE-MEASURE-BINDS`, che l'implementatore cita e applica. Il caso della varianza c'è, ma **la sua gate è un calcolo e non una guardia**: la stessa forma di difetto che [SKILL-001] è stata scritta per intercettare, un livello più in dentro.

**La via d'uscita esiste già in questa spec ed è quella usata per la prima genesi:** pubblicare i valori della seconda come fixture, e far confrontare entrambe le strade con il documento. Nessuna strada copia l'altra, e l'accordo diventa asserito invece che osservato.

## Tests and verification

`GATE-WRONG-CONTEXT-REJECTED` è soddisfatta nel ramo «è rifiutata», con una matrice 4×4 e la prova in negativo — reso `binds` sempre `true`, tre test falliscono. L'implementatore ha lasciato **vuota** la casella del criterio di compilazione invece di spuntarla a vuoto, ed è la scelta giusta: quel criterio non si applica alla forma scelta, e spuntarlo sarebbe stata una pretesa.

**Il rifiuto del parametro di tipo sul dominio è motivato meglio di quanto la spec chiedesse.** Sposterebbe a compilazione **metà** del controllo — `chain_id` è un valore, non un tipo — coprendo l'errore facile (dominio sbagliato in una funzione che nomina il dominio) e non quello difficile (dominio giusto, catena altrui). E renderebbe `SignatureVerifier` generico, quindi non più `dyn`, obbligando chi tiene preimmagini di domini diversi in una collezione a introdurre un enum. È il criterio dell'ergonomia applicato con un caso concreto invece che a parole.

## Required follow-up

RF-001 all'implementatore come remediation dentro il giro di review.

**Due cose sono del Lead e non sue.**

`election_epoch` va aperto come debito proprio, ed è il ritrovamento che vale più della fixture: `election_boundary_height(e) = e * L` con `L` preso *«dai parametri di consenso attivi»*, e il documento non dice attivi a quale altezza. `L` è governato, quindi la stessa altezza cade in epoche diverse sotto documenti diversi, e `election_epoch` entra in `election_entropy`, `election_seed` ed `election_ticket`. [SPEC-016] ha chiuso la stessa forma per `reward_epoch` nominando il documento; **nessun oggetto dell'elezione porta quella cucitura.** L'implementatore si è fermato correttamente: sono tre regole di validità nuove e aprono la propria passata.

Il difetto minore che ha segnalato senza correggere — `published_artifacts_negative.py` stampa «110 probes» mentre i casi eseguiti sono 111 — è del Lead per la ragione che dà: correggerlo dentro questa spec imporrebbe di rieseguire la prova in negativo di uno strumento fuori mandato, per una stringa.

## Final decision

**Changes requested su RF-001.**

Va registrato che l'implementatore ha contraddetto la spec su un punto e aveva ragione — la circolarità non passa solo per `validator_set_hash` — e che ha dichiarato una famiglia 4 invece di nasconderla: la clausola `key_binding_signature` non ha fixture, e la ragione scritta è che pubblicare un `ValidatorSet` di genesi significa pubblicare una coorte che il blocco di vincoli governa. Preferire una famiglia 4 dichiarata a una famiglia 1 nascosta è il compromesso giusto, e averlo scritto lo rende una decisione invece che un'omissione.
