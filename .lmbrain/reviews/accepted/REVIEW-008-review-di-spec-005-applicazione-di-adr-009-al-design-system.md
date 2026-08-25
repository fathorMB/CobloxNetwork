---
id: REVIEW-008
# Note: Quote the title if it contains a colon
title: "Review di SPEC-005 — Applicazione di ADR-009 al design system"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-005
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-006
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-008-EVENT-001"
    timestamp: "2026-08-25T12:43:43.005956800+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-006"
  - schema_version: "1"
    id: "REVIEW-008-EVENT-002"
    timestamp: "2026-08-25T12:44:56.611438200+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Nove criteri su nove verificati, con ogni gate rieseguita dal Lead in modo indipendente e non presa dall'evidenza dell'implementatrice: zero residui del segnaposto, zero virgole come separatore delle migliaia, i tre generatori in --check confermano che gli artefatti non sono stati modificati a mano, e 130 coppie di contrasto su 130 restano conformi a WCAG AA. Nessun finding. La segnalazione sui titoli italiani delle pagine di mockup e stata valutata e respinta nel merito, con la motivazione scritta nella review perche resti trovabile."
    evidence_refs: ["SPEC-005", "ADR-009"]
    implementation_agent: "AGENT-006"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
related_specs: [SPEC-005]
related_decisions: [ADR-009]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> accepted"
---
# Review di SPEC-005 — Applicazione di ADR-009 al design system

## Outcome

**Accettata.** Nove criteri su nove verificati, tutti rieseguiti dal Lead in modo indipendente e non presi dall'evidenza dell'implementatrice. Nessun finding bloccante, nessun finding non bloccante. Una segnalazione dell'implementatrice è stata valutata e respinta nel merito, con motivazione scritta qui perché non venga risollevata a ogni passata.

## Acceptance-criteria compliance

Tutti i criteri sono soddisfatti. Verifiche rieseguite dal Lead:

- **Residui del segnaposto.** Ricerca ricorsiva sull'intero pacchetto per `U+25C7` e per `cbx-unit--provisional`: zero risultati. L'implementatrice ha inoltre distinto il glifo del marchio `U+25C8`, simile a vista e non correlato, e lo ha lasciato intatto — distinzione corretta, che una sostituzione frettolosa avrebbe mancato.
- **Formato numerico.** Ricerca per il pattern di virgola come separatore delle migliaia: zero risultati. Il vincolo di `PRINCIPLES.md` §4.2 regge.
- **Rigenerazione e idempotenza.** Il Lead ha rieseguito i tre generatori in modalità `--check`: `tokens.css is in sync with tokens.json (247 custom properties)`, `design/preview/index.html matches its generator`, `all 4 mockup pages match their generator`. Gli HTML non sono stati modificati a mano: se lo fossero stati, il controllo avrebbe rilevato la deriva. `GATE-REGENERATED` è soddisfatto su evidenza indipendente.
- **Contrasto.** `check-contrast.mjs` rieseguito dal Lead: `all 130 declared pairs meet WCAG AA`. Nessuna regressione.
- **`PRINCIPLES.md` §4.1.** Riscritta con nome, plurale, forma compatta **e la ragione della posposizione**, che era il punto su cui la spec insisteva. Dichiara esplicitamente che la regola non è nuova e che finisce l'unica eccezione a §7.3.
- **`tokens.json`.** Il blocco `$meta.unit` sostituisce il segnaposto e porta con sé la nota sul perché l'unità è posposta, non solo il valore.

## Code observations

La scelta di riusare il margine già presente su `.cbx-unit` come separatore, invece di introdurre un carattere di spazio letterale nel markup generato, è corretta e migliore dell'ovvio: il separatore resta una proprietà del design system anziché diventare un carattere sparso nel contenuto, e non può andare a capo separandosi dal numero.

## Tests and verification

Entrambe le gate dichiarate sono soddisfatte, e sono state verificate due volte: dall'implementatrice nell'evidenza, e dal Lead in modo indipendente con gli stessi strumenti. `GATE-REGENERATED` merita una nota di merito di processo — è la gate che ha *dimostrato* l'assenza dell'errore più probabile di questa spec, cioè la modifica manuale degli artefatti generati, invece di limitarsi a vietarlo a parole.

## Production quality and documentation compliance

Conforme. La documentazione delegata dalla spec è stata aggiornata per intero, e l'implementatrice ha esteso `PRINCIPLES.md` §3 e la tabella §10 per dichiarare deciso il font, propagando anche la correzione di [ADR-009] sulla licenza SIL OFL 1.1. Non è scope creep: è la stessa questione, chiusa dove il pacchetto la dichiarava aperta.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

Nessun finding.

## Required follow-up

**Segnalazione dell'implementatrice, valutata e respinta nel merito.** Ha rilevato che i `<title>` e gli `<h1>` delle pagine di mockup sono in italiano — «Dashboard del nodo», «Dettaglio attività» — e ha chiesto al Lead di giudicare se violino il vincolo di lingua di [[PROJECT]]. **Non lo violano.** Quei titoli sono la *cornice* delle pagine di mockup, cioè la documentazione di design che circonda gli artboard, non superficie di prodotto: gli artboard al loro interno sono in inglese. Il vincolo copre ciò che vede l'utente finale, e nessun utente finale vedrà mai la pagina indice dei mockup. La segnalazione era però corretta come domanda: il confine fra artefatto interno e superficie di prodotto è genuinamente sottile in un pacchetto che li mette nello stesso file, e chiedere invece di decidere da sola è stato il comportamento giusto. Registrata qui perché la risposta resti trovabile e la domanda non si ripeta.

**Scelta di scope dell'implementatrice, confermata.** Non ha incrementato `$meta.version` in `tokens.json`, per tenere il diff nel perimetro dichiarato, segnalandolo come modifica di una riga a disposizione del Lead. Confermata la scelta di non incrementarlo: la versione dei token descrive la forma del vocabolario dei token, che non è cambiata; cambiare versione per una decisione di naming la renderebbe un registro di eventi anziché un numero di compatibilità.

**Giudizio di design esercitato dove la spec lo lasciava aperto**, come previsto: `cr` su ogni superficie di prodotto che riporti una cifra, `credits` per la prosa discorsiva e la documentazione. La motivazione è scritta in `PRINCIPLES.md` §4.1 e ancorata alla leva della densità da terminale e alla condizione di revisione di [ADR-009]. È la scelta che il Lead avrebbe preso, ed è stata scritta come regola invece che lasciata implicita nei mockup — che era la richiesta esplicita della spec.

## Final decision

Accettata senza remediation. [SPEC-005] passa a `done`.
