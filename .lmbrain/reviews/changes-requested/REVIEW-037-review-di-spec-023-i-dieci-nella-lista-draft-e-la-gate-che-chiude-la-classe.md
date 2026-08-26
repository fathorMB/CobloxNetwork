---
id: REVIEW-037
# Note: Quote the title if it contains a colon
title: "Review di SPEC-023: i dieci nella lista DRAFT e la gate che chiude la classe"
status: changes-requested
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-023
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [documentation, process, correctness]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-037-EVENT-001"
    timestamp: "2026-08-26T22:46:30.370427+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-037-EVENT-002"
    timestamp: "2026-08-26T22:49:23.762137+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Quattro finding, nessuno di sostanza: due medium e due low, uno di evidenza e tre di forma su un artefatto pubblicato. La consegna e' buona e nessuna riparazione tocca il progetto dello strumento o le conclusioni dell'analisi.\n\nIl Lead ha rieseguito sei criteri su dieci invece di prenderli dall'evidenza, e reggono tutti. In particolare GATE-SEEN-IT-FAIL-FIRST non e' stata letta ma riprodotta: ripristinato README.md allo stato pre-consegna, lo strumento ha nominato esattamente i dieci parametri reali con i rispettivi numeri di riga. I codici di uscita sono corretti, 1 sul mutato e 0 sul consegnato, controllati apposta perche' una gate che stampa FAIL uscendo 0 e' decorativa. ledger.md non e' stato toccato affatto, quindi lo scopo escluso e' stato rispettato.\n\nRF-001 medium: la trascrizione dichiara 85 test dove il comando dichiarato ne produce 181. Rieseguito: 181 passati, zero falliti. Lo stato e' corretto, e' la trascrizione a portare un numero non guardato. Conta perche' su questo progetto le gate sono kind=manual con evidence=transcript: la trascrizione e' la prova, e un numero falso toglie credito anche alle righe accanto che erano vere.\n\nRF-002 medium: notazione LaTeX introdotta in docs/protocol/README.md, che e' pubblicato e nell'inventario di ADR-012. Zero occorrenze in tutti e cinque i documenti di protocollo prima, tre dopo. Il caso peggiore e' $F$, simbolo che nasce in ADR-017 - artefatto del brain, in italiano, non pubblicato - e che in README.md non e' definito da nessuna parte.\n\nRF-003 low: righe oltre le 100 colonne da 46 a 60 in un documento che va a capo ovunque, con le quattordici nuove tutte in questa sezione.\n\nRF-004 low: lo strumento non e' cablato in CI ne' altrove e la scelta non e' dichiarata. La spec la lasciava al giudizio, quindi non cablarlo non viola il perimetro; a violarlo e' non averlo detto. E' la forma di DEBT-025, e il rischio ha una data: SPEC-022 aggiunge due campi a ConsensusParametersBody e la gate esiste per accorgersene. Dichiarare chi la esegue e quando e' una chiusura accettabile quanto cablarla.\n\nPerimetro dichiarato di questa review: sei analisi su dieci non sono state controllate e la loro correttezza e' inferita da un campione di quattro citazioni, tutte precise. Le conclusioni tassonomiche sono giudicate ben argomentate e non verificate. Verificarle spetta a GATE-SECREVIEW, che e' sull'analisi e non sullo strumento perche' e' l'analisi a diventare un ADR.\n\nLa spec resta in review: la remediation e' continuazione del ciclo, non un ripristino di ciclo di vita."
    evidence_refs: ["SPEC-023", "DEBT-036", "DEBT-025", "SPEC-022", "ADR-012", "ADR-017"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
links: [DEBT-036, DEBT-025]
created: 2026-08-26
updated: 2026-08-26
tags: [review, governance, conformance]
related_decisions: [ADR-012, ADR-010]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned pending -> changes-requested"
---
# Review

## Outcome

**Changes requested, e nessuno dei quattro finding tocca la sostanza.** Lo strumento fa quello che deve, l'ho verificato in modo indipendente, e l'analisi dei dieci parametri è il pezzo migliore della consegna: dieci su dieci, quattro domande ciascuno, fonti citate con intervalli di riga, e le citazioni che ho controllato a campione **sono precise**.

I quattro rilievi sono uno di **evidenza** — un numero falso in trascrizione — e tre di **forma su un artefatto pubblicato**. Sono tutti riparabili senza toccare il progetto della consegna.

## Acceptance-criteria compliance

Dieci criteri su dieci spuntati dall'implementatore. **Il Lead ne ha rieseguiti sei in modo indipendente** invece di prenderli dall'evidenza; tutti e sei reggono. Il decimo è quello che porta il finding RF-001.

| Criterio | Esito della verifica del Lead |
| --- | --- |
| Dieci parametri nella lista DRAFT, raggruppati, con la grandezza che li vincola | **Confermato** leggendo il diff |
| Nessuna voce preesistente persa | **Confermato**: le tre voci originali sono tutte presenti e sono state **arricchite** con i nomi di campo espliciti |
| Strumento versionato che confronta schema e unione delle due liste | **Confermato**: `sim/tools/consensus_parameters_closure.py` |
| Fallisce su un campo dello schema fuori da entrambe le liste | **Rieseguito dal Lead**, `C1-SCHEMA-NOT-COVERED` |
| Fallisce nell'altra direzione | **Rieseguito dal Lead**, `C2-ORPHAN-PARAM` |
| `PASS` sull'albero reale | **Rieseguito**: 20 campi su 20 coperti, 10 vincolati, 20 in DRAFT |
| Analisi su dieci di dieci con le quattro domande | **Confermato** leggendo il documento |
| Distingue relazionale da magnitudine, niente tetto uniforme | **Confermato**, ed è fatto bene |
| Nessun valore fissato, nessun limite aggiunto al blocco dei vincoli | **Confermato**: `docs/protocol/ledger.md` **non è stato toccato affatto** |
| Test, clippy, fmt, `published_artifacts.py` | **Rieseguiti**, tutti verdi — **ma il conteggio dei test in trascrizione è falso**, vedi RF-001 |

## Code observations

Lo strumento estrae i venti campi dallo schema invece di elencarli, che è la forma giusta: un elenco cablato sarebbe stato l'undicesima occorrenza della famiglia che lo strumento esiste per chiudere.

**I codici di uscita sono corretti**, e li ho controllati apposta perché una gate che stampa `FAIL` e esce `0` è invisibile in CI: `1` sull'albero mutato, `0` su quello consegnato.

## Tests and verification

`cargo test --workspace --all-features` → **181 passati, 0 falliti**. `clippy -D warnings` e `fmt --check` puliti. `published_artifacts.py` `PASS`; la prova in negativo di [ADR-012] `PASS` con 17 mutazioni su 11 classi; `lead_claims_check.py` `PASS`.

## Production quality and documentation

L'analisi in `.lmbrain/knowledge/` è di qualità superiore alla media delle consegne di questo progetto. Quattro citazioni controllate a campione, tutte precise: `ledger.md:2798` è davvero il passo 6 *«Corroborate freshness»* che nomina `max_current_balance_age_ms`; TM-37 esiste e riguarda davvero la compromissione della chiave di trasporto; `app_suspension_notice_epochs` sta alla 628, dentro l'intervallo citato; `max_envelope_validity_ms` e le due cache anti-replay stanno alle righe 106 e 109 di `wire.md`, dentro l'intervallo citato.

La tassonomia finale **non propone dieci tetti**: distingue magnitudine, relazionale, banda a due lati e forme ibride, e nomina [DEBT-036] famiglia 3 come la trappola evitata. Era il rischio dichiarato della spec ed è stato evitato consapevolmente.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=process | severity=medium | criterion=ultimo criterio di accettazione | remediation=correggere il numero in trascrizione, o dichiarare quale comando è stato davvero eseguito
**La trascrizione dichiara «`cargo test --workspace --all-features` (PASS, 85 test passati)». I test di questo progetto sono 181.** Rieseguito dal Lead: 181 passati, 0 falliti. Lo **stato** è quindi corretto e nessun test è rotto; è la **trascrizione** a riportare un numero che il comando dichiarato non produce — o il comando eseguito era un altro, o il conteggio è stato scritto senza guardarlo.

Conta su questo progetto più che altrove, e per una ragione precisa: le gate qui sono `kind=manual` con `evidence=transcript`, cioè **la trascrizione è la prova**. Un numero non guardato dentro una trascrizione toglie valore anche alle righe accanto che erano vere, perché chi legge non ha modo di sapere quali siano state controllate. È inoltre la stessa forma che [HANDOFF-003] censisce tre volte con il nome di *affermazione scritta con cura e falsa*.

RF-002 | category=documentation | severity=medium | criterion=primo criterio di accettazione | remediation=togliere la notazione LaTeX dai tre punti, e sostituire `$F$` con il nome del campo o toglierlo
**Notazione LaTeX introdotta in un documento pubblicato che non ne conteneva.** `docs/protocol/README.md` porta ora `$D_{\max}$`, `$S_{\max}$` e `$F$`. Verificato contando: **zero occorrenze** di notazione matematica in tutti e cinque i documenti di `docs/protocol/` prima di questa consegna, **tre** dopo, tutte in questa sezione.

**Il caso peggiore dei tre è `$F$`**, in *«validator succession coordination margin ($F$)»*. `F` è un simbolo che nasce in [ADR-017], che è un artefatto del brain, scritto in italiano e non pubblicato. In `README.md` **non è definito da nessuna parte**: un lettore esterno incontra un simbolo che il documento non introduce. `D_max` e `S_max` sono meno gravi perché seguono immediatamente il nome del campo e si leggono come un alias, ma condividono la stessa forma.

La sezione è `DRAFT` e quindi non normativa, il che riduce la severità ma non la annulla: `README.md` è **pubblicato**, ed è nell'inventario di [ADR-012].

RF-003 | category=documentation | severity=low | criterion=primo criterio di accettazione | remediation=riportare i punti nuovi e riscritti alla larghezza del documento
**Regressione dell'a capo in un documento che va a capo ovunque.** Le righe oltre le 100 colonne in `README.md` passano da **46 a 60**, e le quattordici nuove sono tutte in questa sezione: i punti su economia ed elezione sono stati riscritti come righe singole molto lunghe, e i dieci punti nuovi sono nati così. Il diff di una riga lunga è illeggibile, e questa sezione tornerà a cambiare a ogni parametro che si decide.

RF-004 | category=process | severity=low | criterion=terzo criterio di accettazione | remediation=cablare lo strumento in CI, oppure dichiarare per iscritto perché non lo si è fatto
**Lo strumento nuovo non è cablato da nessuna parte, e la scelta non è dichiarata.** Ricerca eseguita: `consensus_parameters_closure.py` non compare in `.github/`, né in `published_artifacts.toml`, né in alcun altro punto dell'albero fuori da se stesso.

La spec lasciava la decisione al giudizio (*«se la gate va cablata lì»*), quindi non cablarlo non viola il perimetro. **A violarlo è non averlo detto**: uno strumento versionato che nessuno esegue è precisamente la forma di [DEBT-025], dove `threat_model_matrix_coherence.py` esiste, non è in CI, e invecchia. Qui il rischio è concreto e datato: `ConsensusParametersBody` guadagnerà campi — [SPEC-022] ne aggiunge **due** — e la gate esiste proprio per accorgersene.

## Cosa il Lead ha attaccato senza riuscire a romperlo

1. **`GATE-SEEN-IT-FAIL-FIRST`, che è la gate più facile da soddisfare a parole.** Non ho preso la trascrizione: ho **ripristinato io** `docs/protocol/README.md` allo stato pre-consegna e rieseguito lo strumento. Ha nominato **esattamente i dieci parametri reali**, uno per riga, ciascuno con il proprio numero di riga nello schema. La gate è stata soddisfatta davvero.
2. **Il codice di uscita sul fallimento.** Sospettavo un `FAIL` stampato con uscita `0`, che è il modo in cui una gate diventa decorativa. Esce `1` sull'albero mutato e `0` su quello consegnato.
3. **La prova in negativo.** Rieseguita dal Lead nelle due direzioni, non letta: `C1-SCHEMA-NOT-COVERED` e `C2-ORPHAN-PARAM`, entrambe osservate.
4. **La perdita di voci dalla lista DRAFT.** Ho confrontato la sezione prima e dopo cercando una voce caduta nella riscrittura. Non ce ne sono: le tre originali sono tutte presenti e portano ora i nomi di campo espliciti che prima erano solo descrizioni in prosa.
5. **Lo sconfinamento nello scopo escluso.** Ho controllato se fosse stato aggiunto un limite al blocco dei vincoli approfittando della passata. `docs/protocol/ledger.md` **non è stato toccato**, e nessun valore di lancio è stato fissato.
6. **Quattro citazioni dell'analisi**, scelte fra quelle che un ADR futuro userebbe. Tutte precise, nessuna approssimata, nessuna a un intervallo che non contiene ciò che dice di contenere.

## Cosa il Lead non ha guardato

- **Non ho verificato le altre sei analisi su dieci.** Ne ho controllate quattro a campione e reggono; le altre sei **non sono state controllate**, e la loro correttezza è al momento inferita dal campione. È il perimetro di questa review, ed è la ragione per cui `GATE-SECREVIEW` su [SPEC-023] è **sull'analisi** e non sullo strumento.
- **Non ho valutato nel merito le conclusioni tassonomiche.** Che `max_envelope_validity_ms` voglia un tetto di magnitudine e `max_weak_subjectivity_age_ms` un vincolo relazionale è una tesi che questa review considera **ben argomentata e non verificata**. Verificarla è compito di AGENT-007.
- **Non ho eseguito la pipeline CI.** `GATE-CI-GREEN` è `before-done` e resta da soddisfare.

## Required follow-up

- **RF-001** e **RF-002** vanno chiusi prima dell'accettazione: il primo perché la trascrizione è la prova, il secondo perché tocca un artefatto pubblicato.
- **RF-003** e **RF-004** sono a basso costo e conviene chiuderli nello stesso giro. Per RF-004 è accettabile **dichiarare** la scelta invece di cablare, purché la dichiarazione dica chi eseguirà la gate e quando.
- **`GATE-SECREVIEW` resta da soddisfare**, ed è sull'analisi: è quella che l'operatore userà per decidere l'ADR della seconda metà di [DEBT-036], e un errore lì si propaga dentro una decisione di protocollo.
- **`GATE-CI-GREEN` resta da soddisfare.**
