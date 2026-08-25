---
id: REVIEW-013
# Note: Quote the title if it contains a colon
title: "Review di SPEC-009 — RewardBounds, regole di validità economiche, claim in due regimi"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-009
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-013-EVENT-001"
    timestamp: "2026-08-25T17:19:10.914974+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-013-EVENT-002"
    timestamp: "2026-08-25T17:20:15.550927+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Dieci criteri su undici soddisfatti e le quattro disposizioni di ADR-010 e ADR-011 sono scritte bene, con RewardBounds nella genesi, la tariffa di availability come regola di validita, il vincolo relazionale nel blocco e il tetto proporzionale agli eleggibili rifiutato esplicitamente. Il Lead ha rieseguito la gate delle fixture in modo indipendente, validando il metodo su una fixture non modificata e riproducendo esattamente il nuovo consensus_parameters_hash, e la suite e verde su 104 test.\n\nUn solo finding, di severita high, ed e una contraddizione interna del documento su una soglia di sicurezza. La riga 1444 di ledger.md afferma che con il vincolo nuovo la soglia effettiva di cattura per attrito e due terzi; la riga 1907, nel blocco del claim, afferma ancora che resta appena sopra un terzo, e la riga 1911 dichiara confutata proprio la pretesa dei due terzi che la riga 1444 adesso sostiene. Il Lead ha verificato quale delle due sia vera: con min_set almeno due terzi di V la contrazione per attrito si ferma esattamente a due terzi su ogni dimensione provata, quindi la riga 1444 e corretta e il blocco del claim e rimasto indietro. E la stessa forma di difetto gia corretta due volte in questo progetto, e il criterio di accettazione imponeva esplicitamente l'aggiornamento nella stessa passata."
    evidence_refs: ["SPEC-009", "ADR-010", "ADR-011"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-013-EVENT-003"
    timestamp: "2026-08-25T17:23:38.045029700+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Risolto il finding RF-001: aggiornato il blocco del claim in ledger.md (§What a light client can establish about set composition) per dichiarare la soglia di cattura per attrito a due terzi (garantita da 3*validator_min_set_size >= 2*V) e riscritta la narrazione delle ritrattazioni storiche spiegando che la versione precedente era prematura in assenza del vincolo relazionale, ora introdotto come regola di validita da ADR-010 e SPEC-009."
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-013-EVENT-004"
    timestamp: "2026-08-25T17:25:13.791727600+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "RF-001 chiuso e verificato dal Lead. Il blocco del claim di ledger.md porta ora la soglia dei due terzi attribuita all'azione congiunta del pavimento di contrazione e del vincolo relazionale, con la conseguenza scritta per esteso: una coalizione sotto i due terzi non puo contrarre il set su di se, e sopra i due terzi la safety BFT e gia caduta.\n\nLa narrazione delle ritrattazioni e stata riscritta nella forma richiesta e dice il vero: la terza versione non era sbagliata in se, era prematura, perche la censura selettiva poteva contrarre il set al cinquanta per cento quando V cresceva lasciando min_set statico. Ora che il vincolo e imposto come regola di validita relazionale, min_set e ancorato ad almeno due terzi di V e la soglia diventa un invariante su tutti gli insiemi di parametri conformi. La traccia storica e conservata e non cancellata, come il documento fa gia altrove.\n\nVerificato inoltre che non restino altre occorrenze della vecchia cifra come affermazione viva. L'unica rimasta e alla riga 1414 ed e corretta perche circoscritta al pavimento preso da solo, che effettivamente non porta ai due terzi: e la riga 1444 ad attribuire il risultato all'azione congiunta delle due regole, e la sezione distingue le due cose invece di confonderle. Gli hash delle fixture sono invariati rispetto alla verifica precedente del Lead e la suite resta verde su tutte e tredici le sue parti."
    evidence_refs: ["SPEC-009", "ADR-010"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-013-EVENT-005"
    timestamp: "2026-08-25T17:25:27.512255900+02:00"
    action: "verdict"
    from_status: "changes-requested"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Undici criteri su undici, dopo la chiusura verificata di RF-001. Le quattro disposizioni di ADR-010 e ADR-011 sono scritte: RewardBounds nell'ancora di fiducia della genesi con una valutazione di sicurezza dichiarata per ogni grandezza della reward policy, incluse quelle lasciate fuori; la tariffa di availability a zero come regola di validita con due casi rifiutati nel registro; il vincolo relazionale nel blocco di vincoli; il dimensionamento del fondo alla genesi e il claim in due regimi. Il tetto proporzionale agli eleggibili e rifiutato esplicitamente in una sezione propria, come la spec chiedeva di fare prima che qualcuno lo proponesse.\n\nIl Lead ha rieseguito la gate delle fixture in modo indipendente, validando il metodo su una fixture non modificata prima di ricalcolare quella cambiata, e ha riprodotto esattamente il nuovo consensus_parameters_hash. Ha inoltre verificato per conto proprio che il vincolo faccia davvero cio che il documento gli attribuisce: con min_set ancorato a due terzi di V la contrazione per attrito si ferma esattamente a due terzi su ogni dimensione provata. La suite resta verde su 104 test.\n\nLe tre modifiche ai test di coblox-core sono il minimo necessario e non espansione di scope: il valore della fixture e la costante dell'hash, senza cui la suite fallirebbe su un valore che il documento ha cambiato."
    evidence_refs: ["SPEC-009", "ADR-010", "ADR-011"]
    implementation_agent: "AGENT-002"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
related_specs: [SPEC-009]
related_decisions: [ADR-010, ADR-011]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "recorded review remediation-verification"
  - date: 2026-08-25
    action: "transitioned changes-requested -> accepted"
---
# Review di SPEC-009 — RewardBounds, regole di validità economiche, claim in due regimi

## Outcome

**Changes requested, per un solo finding — ma è una contraddizione interna del documento su una soglia di sicurezza, e non si può accettare.** Dieci criteri su undici sono soddisfatti e le quattro disposizioni di [ADR-010] e [ADR-011] sono scritte bene. L'undicesimo — l'aggiornamento dell'affermazione di `ledger.md` sulla soglia effettiva, che la spec imponeva **nella stessa passata** — è soddisfatto in un punto e non nell'altro, e i due punti ora si contraddicono.

## Acceptance-criteria compliance

Soddisfatti, verificati dal Lead:

- **`RewardBounds` esiste** nell'ancora di fiducia della genesi con magnitudini, rapporto di variazione 5/4 e spaziatura di attivazione, dichiarato *configuration, not chain state*. Per ogni grandezza della reward policy è dichiarato se sostenga una proprietà di sicurezza — inclusi i fattori di conversione lasciati fuori, con la ragione. Era il criterio su cui la spec avvertiva del rischio di vincolare solo ciò che è comodo: non è successo.
- **La tariffa di availability a zero è regola di validità**, `ledger.md` la formula con `MUST` e la ragione strutturale accanto, e il registro porta due casi **invalidi** con tariffa positiva.
- **Il vincolo `3 * validator_min_set_size >= 2 * V` è nel blocco**, con il commento che dice cosa impedisce.
- **`GATE-FIXTURES-RECOMPUTED`, rieseguita dal Lead.** Metodo validato su `parameter_set_hash`, non modificata, che riproduce il valore preesistente; con lo stesso metodo il nuovo `consensus_parameters_hash` — `628c66f9…` — è **riprodotto esattamente**. Il fixture `PD-0` porta ora `validator_min_set_size = 8`, e `README.md` spiega perché non può valere `1`.
- **`GATE-RULES-REJECT`**: il registro contiene i casi rifiutati per la tariffa di availability e per il tetto della quota al creatore, con verdetto e ragione per riga.
- **Il tetto proporzionale agli eleggibili è rifiutato esplicitamente** in una sezione propria di `README.md`, con la motivazione dell'inflazione del denominatore. Era il rimedio apparente che la spec chiedeva di nominare prima che qualcuno lo proponesse.
- **Suite verde**: 104 test, rieseguiti dal Lead.

## Code observations

Le tre modifiche a `core/coblox-core/tests/` sono il **minimo necessario e non espansione di scope**: il valore del fixture da `1` a `8` e la costante dell'hash in due file. Senza, la suite fallirebbe su un valore che il documento ha cambiato. La spec escludeva l'implementazione in Rust intesa come lavoro nuovo, non la coerenza di test già ancorati a una fixture. Giudicata corretta.

## Tests and verification

Entrambe le gate `before-submit` sono soddisfatte e la prima è stata rieseguita dal Lead in modo indipendente, con la validazione preventiva del metodo su una fixture non toccata — che è il protocollo che il progetto ha ormai usato quattro volte.

## Production quality and documentation compliance

Conforme, con l'eccezione che è il finding. Il claim in due regimi è scritto in `economic-simulation-report.md` e nel threat model: rampa con garanzia in **termini assoluti** su `D = F · N/(N+H) <= F`, regime maturo con la banda su `α` e `X` sopra la soglia reale, e la banda dichiarata **non applicabile** all'avviamento. È la forma che [ADR-011] chiedeva.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=specification | severity=high | criterion=L'affermazione di `ledger.md` sulla soglia effettiva è aggiornata nella stessa passata | remediation=Aggiornare il blocco del claim e la narrazione delle ritrattazioni

**`ledger.md` ora si contraddice su una soglia di sicurezza.**

Alla riga 1444, con la regola nuova: *«The effective capture threshold against attrition of this network is therefore **two thirds**, guaranteed by the joint enforcement of the contraction floor and `3 * validator_min_set_size >= 2 * V`.»*

Alla riga 1907, nel blocco del claim, invariato: *«the effective threshold remains **just above one third**, and what the rules buy is that reaching it takes several transitions».*

E alla riga 1911 la narrazione delle ritrattazioni afferma tuttora che una versione precedente *«claimed that closing the second gap moved the effective capture threshold to two thirds; selective censorship refutes that in three boundaries»* — cioè **dichiara confutata proprio l'affermazione che la riga 1444 adesso sostiene**.

**Il Lead ha verificato quale delle due è vera, e la riga 1444 è corretta.** Con `min_set >= 2V/3` la contrazione per attrito si ferma esattamente a `2V/3` su ogni dimensione del set provata — `V=12 → 8`, `V=27 → 18`, `V=36 → 24`, `V=60 → 40`, sempre il 66,7% — quindi una coalizione può possedere l'intero set solo a partire da due terzi. Il vincolo nuovo **fa davvero** ciò che la riga 1444 gli attribuisce. La censura selettiva confutava la pretesa dei due terzi quando `V` poteva crescere lasciando `min_set` fermo, ed è precisamente la porta che questa spec ha chiuso.

Il blocco del claim è quindi rimasto indietro rispetto alla regola che lo rende obsoleto — **la stessa forma di difetto già corretta due volte in questo progetto**, su `identity.md` in [SPEC-006] e sul residuo (g) nella stessa spec. Chi legga il blocco del claim, che è la sezione scritta apposta perché sia citata da chi vuole sapere cosa il protocollo promette, ne trae una garanzia **più debole del vero** e una spiegazione che dichiara sbagliata la garanzia corrente.

Non è un difetto della regola: è un'affermazione lasciata indietro. Ma è nella sezione che il progetto ha designato come luogo del claim, e il criterio di accettazione lo imponeva esplicitamente.

## Required follow-up

Aggiornare il blocco del claim di `ledger.md` alla soglia dei due terzi, e riscrivere la narrazione delle ritrattazioni in modo che dica il vero: la terza versione **non era sbagliata in sé**, era prematura, perché rivendicava una proprietà che nessuna regola sosteneva ancora. Ora la regola esiste. Conservare la traccia storica come il documento fa già altrove, senza cancellarla.

Verificare nella stessa passata che non esistano **altre** occorrenze della vecchia cifra: due erano note, e la ragione per cui questo finding esiste è che una sola delle due è stata aggiornata.

Poi `GATE-SECREVIEW` con AGENT-007, che è `before-done`. Le regole nascono da un suo finding critico e la formulazione del claim è materia sua.

## Final decision

Changes requested. La spec resta in `review`.
