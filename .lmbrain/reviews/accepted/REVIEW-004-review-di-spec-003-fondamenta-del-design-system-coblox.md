---
id: REVIEW-004
# Note: Quote the title if it contains a colon
title: "Review di SPEC-003 — Fondamenta del design system Coblox"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-003
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-006
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-004-EVENT-001"
    timestamp: "2026-08-25T01:54:26.113526800+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-XXX"
  - schema_version: "1"
    id: "REVIEW-004-EVENT-002"
    timestamp: "2026-08-25T01:55:24.367002200+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Tutti e sette i criteri di accettazione verificati meccanicamente dal Lead: zero colori hard-coded fuori dai token, temi dark e light nella stessa pagina demo, 5 artboard per ciascuna delle tre schermate con tutti e quattro gli stati richiesti, GATE-CONTRAST rieseguito dal Lead con 130 coppie su 130 in AA, PRINCIPLES.md completo sui quattro temi, inglese dentro gli artboard con annotazioni italiane per il team secondo convenzione dichiarata, e il criterio di comprensione risolto in modo diretto. Un solo finding di severita bassa (RF-D01, percorsi documentati obsoleti dopo lo spostamento richiesto dall'operatore) che non blocca la chiusura. La spec resta in review perche GATE-OPERATOR-LOOK e di competenza dell'operatore e riguarda un giudizio estetico che il Lead non puo dare al suo posto."
    evidence_refs: ["SPEC-003", ".lmbrain/design/coblox-design-system/tokens/CONTRAST.md", ".lmbrain/design/coblox-design-system/PRINCIPLES.md"]
    implementation_agent: "AGENT-006"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> accepted"
---
# Review

## Outcome

**Accettata sul piano tecnico.** Tutti e sette i criteri di accettazione sono soddisfatti, verificati meccanicamente dal Lead. La spec **non può passare a `done`** perché `GATE-OPERATOR-LOOK` è di competenza dell'operatore e riguarda un giudizio — l'approvazione della direzione estetica — che il Lead non può dare al suo posto, nemmeno sotto delega generale sul dispatch e sulle review.

## Acceptance-criteria compliance

| Criterio | Esito | Verifica del Lead |
| --- | --- | --- |
| Token in JSON + CSS con nomi semantici, nessun colore hard-coded fuori dai token | Pass | 0 valori esadecimali nei tre CSS di componenti, base e shell. I 2 apparenti riscontri negli HTML erano falsi positivi del mio controllo: `&#8239;`, l'entità dello spazio stretto, non un colore |
| Pagina demo con tutti i componenti, in dark e light | Pass | `data-theme` presente su entrambi i temi nello stesso documento, senza JavaScript |
| Tre schermate con i quattro stati ciascuna | Pass | 5 artboard per ciascuna delle tre schermate: `stato-nominale`, `stato-vuoto`, `stato-caricamento`, `stato-errore`, `stato-offline` |
| Contrasto WCAG AA su ogni coppia dichiarata, con strumento e valori | Pass | Verificatore rieseguito dal Lead dopo lo spostamento del pacchetto: `RESULT: all 130 declared pairs meet WCAG AA` |
| PRINCIPLES.md copre monospace, numeri di token, lingua e tono, accessibilità | Pass | §3 monospace, §4 numeri, §7 lingua e tono, §8 accessibilità |
| Tutto il testo visibile all'utente in inglese | Pass | Vedi la nota sulla lingua qui sotto |
| Un non-designer capisce dalla dashboard: quanto ho guadagnato, chi mi usa, se il nodo è sano | Pass | Le tre risposte sono esplicite e affiancate nell'artboard nominale |

### Sulla lingua, che era il rilievo dell'operatore

Il criterio è soddisfatto e la separazione è deliberata, non un residuo. Il testo **dentro** gli artboard è inglese; l'italiano compare solo nelle annotazioni **attorno** agli artboard (`mock-artboard__head`, `mock-artboard__note`) e negli identificatori di ancora. La pagina lo dichiara in testa: *"Tutto il testo dentro gli artboard è in inglese, perché è la lingua del prodotto. Le annotazioni attorno agli artboard sono note di lavoro interne per il team e restano in italiano."*

È esattamente la distinzione registrata come vincolo in [[PROJECT]]. Avevo inizialmente segnato come rilievo il nome del file `attivita.html`: **lo ritiro**, perché è coerente con la stessa convenzione — l'impalcatura della pagina di mockup è materiale di lavoro del team, non superficie di prodotto. Segnalare come difetto una scelta dichiarata e coerente sarebbe stato ingiusto.

## Code observations

Il criterio di comprensione era il più soggetto a interpretazione ed è quello meglio risolto. L'artboard nominale della dashboard risponde alle tre domande in modo diretto e affiancato: *Credited today 128.40*, con la precisazione *24.00 of it for proven presence alone*; *Using you right now: 2 storage sessions · 1 compute*; *Proofs passed (24 h): 142 of 148 challenges · your node is healthy*.

Notevole la seconda riga: separare la quota che deriva dalla sola presenza dimostrata dal totale è onestà economica messa in interfaccia. Con [ADR-007] quella distinzione diventa ancora più importante di quanto lo fosse quando la schermata è stata disegnata, perché il reddito di esistenza diventa una quota variabile: la dashboard è già predisposta a raccontarlo.

Scelte di design non ovvie e ben motivate dall'implementatrice: il burn è violetto e non rosso, perché spendere non è una perdita; nessun "netto" da nessuna parte, con emesso e bruciato affiancati e legenda esplicita; cifre tabulari ovunque, così i numeri in diretta non ballano; il valore sconosciuto è un trattino e mai uno zero o un numero vecchio non etichettato.

## Tests and verification

`GATE-CONTRAST` (owner: agent, before-submit): **rieseguito dal Lead**, non accettato sulla parola.

```text
node tools/check-contrast.mjs -> RESULT: all 130 declared pairs meet WCAG AA
node tools/build-tokens.mjs   -> 247 custom properties, rigenerazione identica
link relativi del pacchetto   -> file=11 riferimenti=31 rotti=0
```

`GATE-OPERATOR-LOOK` (owner: operator, before-done): **aperto**. Punti d'ingresso per l'operatore: `.lmbrain/design/coblox-design-system/index.html`, oppure direttamente `preview/index.html` e `mockups/index.html`.

## Production quality and documentation compliance

Conforme a [[QUALITY]]. Degno di nota: le verifiche dell'implementatrice hanno trovato tre difetti reali che ha **corretto invece di aggirare** — una rottura di parità fra temi intercettata dal generatore, una coppia a 4,44:1 scoperta con uno sweep sul DOM renderizzato e non coperta dalla tabella dei token (risolta alzando il token, non abbassando la soglia), e `id` duplicati che rompevano `label[for]` in quattro artboard. Chi trova difetti nel proprio lavoro e li dichiara sta lavorando come si deve.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

- RF-D01 | category=documentation | severity=low | criterion=coerenza dei percorsi documentati | remediation=Restano circa 54 occorrenze testuali del vecchio percorso `design/...` nel pacchetto, conseguenza dello spostamento in `.lmbrain/design/` richiesto dall'operatore dopo la consegna. Non rompono nulla, ma istruiscono a eseguire comandi come `node design/tools/build-tokens.mjs`, che oggi falliscono. La correzione va fatta nei generatori e poi rigenerando, non a mano sugli output. Condizione di chiusura: nessuna occorrenza del vecchio percorso e comandi documentati eseguibili come scritti.

## Required follow-up

1. **Operatore:** guardare le pagine e attestare `GATE-OPERATOR-LOOK`. È l'unico passo che separa SPEC-003 da `done`.
2. **AGENT-006:** chiudere RF-D01 nei generatori. Non blocca la chiusura della spec: è manutenzione conseguente a una decisione presa dopo la consegna.
3. **Operatore:** due decisioni che l'implementatrice ha posto invece di darle per scontate — il font monospace (propone JetBrains Mono: zero barrato, `1/l/I` e `0/O` distinti, decisivi su hash e identificatori) e il nome dell'unità, oggi un segnaposto. Nessuna delle due blocca la spec.

## Final decision

Accettata sul piano tecnico, con un finding di severità bassa che non blocca. La chiusura resta sospesa al solo gate dell'operatore.
