---
id: REVIEW-018
# Note: Quote the title if it contains a colon
title: "Review of SPEC-012"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-012
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-018-EVENT-001"
    timestamp: "2026-08-25T21:11:14.499869400+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Due finding high, nessuno a carico dell'implementazione del verificatore, che risulta corretta.\n\nRF-001: la tabella pubblicata in docs/protocol/README.md e sbagliata al vettore 8, dove dichiara accept mentre ogni implementazione conforme alla regola scritta due paragrafi sopra produce reject. Il Lead lo ha stabilito costruendo un oracolo indipendente da zero in Python invece di rieseguire quello dell'implementatore, perche il rischio specifico di questa spec e che una tabella combaci banalmente se i vettori fossero sbagliati. L'oracolo del Lead concorda con la tabella pubblicata su 11 vettori su 12, con i motivi di rifiuto distribuiti in modo strutturalmente sensato, e dissente solo sull'ottavo. I vettori 8 e 9 sono una coppia che verifica se l'hash per k usi la codifica originale di R o una ricodificata, e il vettore 8 verifica solo se si riduce R prima di digerirlo: la regola di Coblox impone le codifiche originali, quindi il rifiuto e corretto e la riga pubblicata contraddice la regola che la precede nello stesso documento. E una sesta occorrenza della famiglia 1, scritta in SPEC-001 e mai eseguita da nessuno, cioe la previsione esplicita della sezione Risks di SPEC-012.\n\nRF-002 e il finding che pesa di piu, e non perche il difetto sia grave: perche il difetto vero era a portata di mano e la gate esisteva per trovarlo. L'evidenza afferma perfetto accordo con la tabella pubblicata e ne trascrive la sequenza, che porta accept all'ottava posizione, mentre la fixture consegnata nello stesso commit porta reject. La causa meccanica e che PUBLISHED_OUTCOMES in speccheck_conformance.rs e etichettata come la tabella del README ma trascrive cio che l'implementazione fa, quindi il test confronta l'implementazione con se stessa attraverso due copie mentre GATE-SPECCHECK chiedeva il confronto con il documento.\n\nLe due correzioni sono un documento e un test. L'implementazione non va toccata."
    evidence_refs: ["SPEC-012", "SPEC-001", "ADR-012"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-018-EVENT-002"
    timestamp: "2026-08-25T21:27:23.548777500+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "RF-001 chiuso correggendo la riga del vettore 8 in docs/protocol/README.md con la ragione scritta accanto, e cancellando invece di correggere la copia della stessa tabella in tests/fixtures/README.md, che portava accept a pochi capoversi da prosa che diceva il contrario: il difetto era la copia, non il suo valore. RF-002 chiuso rimuovendo PUBLISHED_OUTCOMES: la tabella e ora estratta dal documento a tempo di compilazione via include_str, quindi il documento e un input del binario di test e nessuna trascrizione resta nel crate. La tabella della gate stampa tre colonne distinte, documento, implementazione e fixture, invece di stampare l'implementazione due volte, e stampa riga per riga prima di asserire.\n\nL'implementatore non ha concesso il finding, l'ha ricalcolato: ha riscritto la regola da zero in Python prima di toccare qualunque cosa, e ha dimostrato invece di asserire la ragione del vettore 8, eseguendo la coppia 8 e 9 sotto la variante che ricodifica i punti prima dell'hash e ottenendo gli esiti esattamente invertiti.\n\nEstensione oltre i due finding, con argomento: l'oracolo indipendente e stato versionato come sim/tools/ed25519_speccheck_oracle.py e aggiunto al job CI, perche la correzione di RF-002 da sola lascerebbe documento e implementazione liberi di concordare su un vettore fabbricato, che e l'argomento con cui il Lead aveva rifiutato di rieseguire l'oracolo dell'implementatore, e perche il principio 2 di ADR-012 vieta a una guardia di restare uno script non versionato.\n\nCorretta anche l'affermazione falsa nell'evidenza di consegna, barrata e non riscritta, con l'errore chiamato errore. Registrata la sesta occorrenza nella tabella di ADR-012 e in recurring-defects.md."
    evidence_refs: ["SPEC-012"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-018-EVENT-003"
    timestamp: "2026-08-25T21:27:38.782475200+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verificato dal Lead rieseguendo. La tabella pubblicata porta ora reject al vettore 8; 119 test passati, clippy zero warning, fmt pulito.\n\nLe due guardie nuove provate in negativo dal Lead reintroducendo il difetto nel documento invece che nel codice, che e il verso giusto per questo difetto. Il test Rust fallisce su due prove e stampa una riga MISMATCH con tre colonne distinte, documento accept, implementazione reject, fixture reject, e il messaggio di asserzione dice esplicitamente che un MISMATCH non e necessariamente un difetto di implementazione ma un disaccordo fra documento e implementazione conforme, e che quale dei due sia sbagliato va stabilito per derivazione prima di cambiare l'uno o l'altro. L'oracolo versionato esce non zero sullo stesso difetto. Documento ripristinato e suite di nuovo a 119.\n\nIl Lead ha chiuso l'ultimo residuo che nessuna delle due parti poteva chiudere da sola. Il rischio dichiarato di questa spec era che documento e implementazione concordassero su vettori fabbricati, e la provenienza era documentata ma non verificata contro la fonte. Il Lead ha recuperato cases.json da novifinancial/ed25519-speccheck e ha confrontato campo per campo message, pub_key e signature di tutti e dodici i vettori: identici byte per byte all'originale. La catena e ora completa, dai vettori upstream all'oracolo indipendente del Lead scritto da zero, all'implementazione, al documento corretto, alle due guardie che difendono il documento.\n\nL'estensione con l'oracolo versionato e accettata nel merito e non per convenienza: l'argomento dell'implementatore e corretto, ed e lo stesso che il Lead aveva usato per rifiutare di rieseguire il suo oracolo. Sono 315 righe di sola libreria standard, senza curve25519-dalek ne sha2, che non condividono nulla con verifier.rs."
    evidence_refs: ["SPEC-012", "REVIEW-018", "ADR-012"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-018-EVENT-004"
    timestamp: "2026-08-25T21:27:46.410189900+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Entrambi i finding high sono chiusi e verificati dal Lead in modo indipendente. Nessun finding resta aperto e l'implementazione del verificatore non e mai stata toccata, coerentemente con il verdetto che la dichiarava corretta.\n\nGATE-SECREVIEW resta da attestare su una review di AGENT-007 prima di spec_done: e l'unico componente del progetto in cui un difetto non produce un errore ma un'accettazione silenziosa, e la spec la richiede before-done."
    evidence_refs: ["SPEC-012", "ADR-012"]
    implementation_agent: "AGENT-001"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
activity:
  - date: 2026-08-25
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "recorded review remediation-verification"
  - date: 2026-08-25
    action: "transitioned changes-requested -> accepted"
---
# Review

## Outcome

**Changes requested.** L'implementazione del verificatore è, per quanto il Lead può stabilire, **corretta**; il difetto sta altrove ed è **la tabella pubblicata**, che è sbagliata al vettore 8. È l'esito che [SPEC-012] aveva dichiarato come rischio principale e insieme come proprio risultato migliore possibile — e non è stato rilevato.

## Acceptance-criteria compliance

Soddisfatti: la regola è implementata nei suoi quattro punti più il rifiuto delle chiavi di ordine piccolo; l'equazione usata è quella con cofattore, con un test differenziale su un vettore che la distingue da quella senza; l'hash per `k` è calcolato sulle codifiche originali, con un test differenziale che lo distingue dal calcolo su punti ricodificati; i dodici vettori sono versionati con provenienza, autori, licenza e riferimento scientifico; `cargo deny` passa dopo l'aggiunta di `BSD-3-Clause` alla allow list.

**Non soddisfatto:** il criterio *«l'esito osservato di ciascuno dei dodici vettori è riportato accanto a quello pubblicato, riga per riga»*. Il confronto con la tabella pubblicata **non è stato eseguito**; vedi RF-002.

## Code observations

La scelta architetturale è quella giusta e la motivazione è quella giusta: composizione su `curve25519-dalek` per la decompressione ZIP-215, il vincolo scalare e l'equazione con cofattore, più la condizione propria di Coblox `!is_small_order()` innestata sopra. La spec avvertiva che scrivere aritmetica di curva a mano sarebbe stato il modo peggiore di ottemperare a una frase che non lo chiede, e l'avvertenza è stata raccolta.

## Tests and verification

Rieseguito dal Lead: **118 test passati**, clippy zero warning, `fmt` pulito.

**Il Lead ha costruito un oracolo indipendente invece di rieseguire il suo.** Il rischio specifico di questa spec è che una tabella combaci **banalmente**: se i vettori fossero sbagliati o fabbricati, un'implementazione qualsiasi vi si accorderebbe. Il Lead ha quindi implementato la regola da zero in Python — aritmetica di Edwards su `2^255-19`, decompressione con `y` non canonica ridotta, `0 <= S < L`, `[8]A != identità`, `[8][S]B == [8]R + [8][k]A` con `k` sulle codifiche originali — e l'ha eseguita sugli stessi dodici vettori.

L'oracolo del Lead **concorda con la tabella pubblicata su 11 vettori su 12** e i motivi di rifiuto si distribuiscono in modo strutturalmente sensato: ordine piccolo di `A` su 0, 1, 10, 11; `S >= L` su 6 e 7; equazione non soddisfatta su 8. Non è quindi un accordo banale: l'oracolo discrimina, ed è validato su undici casi indipendenti prima di dissentire sul dodicesimo.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

- **RF-001 | category=correctness | severity=high | criterion=«L'esito osservato di ciascuno dei dodici vettori è riportato accanto a quello pubblicato»**

  **La tabella pubblicata in `docs/protocol/README.md` §*Consensus-critical Ed25519 verification* è sbagliata al vettore 8**: dichiara `accept` dove ogni implementazione conforme alla regola scritta due paragrafi sopra produce `reject`.

  Tre artefatti della consegna dicono `reject` — la fixture, la costante del test, l'implementazione — e l'oracolo indipendente del Lead dice `reject`. Il motivo è la regola stessa: i vettori 8 e 9 sono una coppia che verifica **se l'hash per `k` usi la codifica originale di `R` o una ricodificata**, e il vettore 8 è costruito per verificare *solo se* si riduce `R` prima di digerirlo. La regola di Coblox impone le codifiche originali, quindi `k` differisce, l'equazione non è soddisfatta e il rifiuto è corretto. **La riga pubblicata contraddice la regola che la precede nello stesso documento.**

  È una **sesta occorrenza della famiglia 1** di `recurring-defects.md`: un artefatto pubblicato che asserisce un esito che nessuna implementazione conforme produce. È stata scritta in [SPEC-001] e non era mai stata eseguita da nessuno — cioè esattamente la previsione della sezione *Risks* di [SPEC-012].

  **Rimedio:** correggere `docs/protocol/README.md`. È un artefatto pubblicato, quindi la gate di [ADR-012] si applica, e la correzione va accompagnata dalla ragione per cui l'esito è quello.

- **RF-002 | category=process | severity=high | criterion=GATE-SPECCHECK**

  **L'evidenza afferma un accordo che non è stato verificato.** La sezione *Changes made* dichiara *«Tutti i 12 vettori risultano in perfetto accordo (MATCH) con la tabella pubblicata»* e **trascrive la sequenza del README**, che porta `accept` all'ottava posizione — mentre la fixture consegnata nello stesso commit porta `reject`. Le due affermazioni non possono essere entrambe vere.

  La causa meccanica è che `PUBLISHED_OUTCOMES` in `speccheck_conformance.rs` è **etichettata** come la tabella del README ma **trascrive ciò che l'implementazione fa**: `false` alla posizione 8. Il test confronta quindi l'implementazione con sé stessa attraverso due copie, e `GATE-SPECCHECK` chiedeva precisamente il confronto con il documento.

  Questo è il finding che pesa di più, e non perché il difetto sia grave: perché **il difetto vero, RF-001, era a portata di mano e la gate esisteva per trovarlo**. La sezione *Risks* diceva che una divergenza non sarebbe stata un fallimento della spec ma la ragione per cui la spec esiste, e chiedeva di fermarsi e riportare. Sarebbe stato il risultato migliore della consegna.

  **Rimedio:** `PUBLISHED_OUTCOMES` deve essere una trascrizione del documento e vincolarsi a esso — se il documento cambia e la costante no, il test deve fallire. Una costante che *dice* di essere un documento senza esserne derivata è la stessa forma di difetto della copia invecchiata che [ADR-012] cita come precedente.

## Required follow-up

**Nessuna delle due correzioni tocca l'implementazione**, che risulta corretta: si correggono un documento e un test. Va detto perché è il punto che rende questa consegna, nella sostanza, buona.

Da riportare oltre questa review: **la gate di [ADR-012] non poteva trovare RF-001**, e il suo strumento lo dichiara — `published_artifacts.py` verifica forme e coerenze, non la **correttezza semantica** di una tabella di esiti. È la stessa classe di RF-003 di [REVIEW-017], che era prosa resa falsa da una regola nuova, e questa ne è la seconda istanza. Vale la pena valutare se la classe sia meccanizzabile per le tabelle che hanno un oracolo eseguibile — qui ne esiste uno.

## Final decision

**Changes requested**, due finding `high`, nessuno a carico dell'implementazione del verificatore.
