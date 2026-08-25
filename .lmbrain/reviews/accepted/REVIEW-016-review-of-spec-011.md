---
id: REVIEW-016
# Note: Quote the title if it contains a colon
title: "Review of SPEC-011"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-011
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-016-EVENT-001"
    timestamp: "2026-08-25T20:13:11.792564400+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Verificato dal Lead in modo indipendente e non letto dall'evidenza: 110 test workspace passati, clippy senza warning, fmt pulito, i quattro strumenti Python versionati tutti OK, 27 casi nominati che rispecchiano le tabelle pubblicate, e nessun hash pubblicato mosso. La verifica decisiva e una mutazione e non un conteggio di test verdi: rimosso dal codice il vincolo 3 min_set maggiore uguale 2V, tre test falliscono nominando la regola, e al ripristino la suite torna verde con params.rs identico. E l'evidenza che GATE-INVALID-REJECTED e GATE-DIRECTION vincolano invece di essere caselle spuntate. Nessun finding. GATE-SECREVIEW resta da attestare su una review di AGENT-007 prima di spec_done, perche le tre regole nascono da suoi finding critici."
    evidence_refs: ["SPEC-011", "ADR-010", "ADR-011", "ADR-012"]
    implementation_agent: "AGENT-001"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
activity:
  - date: 2026-08-25
    action: "transitioned pending -> accepted"
---
# Review

## Outcome

**Accettata dal Lead senza finding a carico dell'implementazione.** `GATE-SECREVIEW` resta da attestare: la spec la richiede `before-done` e le tre regole nascono da finding critici di AGENT-007, quindi chiuderle senza la sua verifica sarebbe incoerente con il modo in cui sono state aperte.

## Acceptance-criteria compliance

Tutti verificati dal Lead in modo indipendente.

`RewardBounds` esiste e **rispecchia `ElectionBounds`**, compresa la disciplina che conta: i limiti sono presi dai bound e mai dal documento in valutazione, e `ValidatedRewardPolicy` non ha costruttore diverso dalla validazione — la stessa forma di `ValidatedConsensusParameters`.

Le tre regole sono applicate: `availability_microtokens_per_unit == 0`, `3 * validator_min_set_size >= 2 * V` nel blocco relazionale, e i limiti di magnitudine, il rapporto di variazione e il gap di attivazione della reward policy.

Le tabelle di frontiera pubblicate sono coperte caso per caso: **27 casi nominati** in `constraint_block.rs`, che sono i 22 di accettazione più i 5 di variazione, con i nomi che rispecchiano quelli di `reward_rules.py`, e ciascuno confrontato con il proprio verdetto atteso invece che con un aggregato.

Nessun hash pubblicato si è mosso: `protocol_hashes.py` riporta MATCH su tutti e quattro i documenti governati, e in questa passata **tutti e quattro sono validazione di metodo** perché nessuno doveva cambiare.

## Code observations

**Il fondo di genesi è corretto dove appartiene.** `recommended.py` porta ora `existence_fund_microtokens_per_epoch = 300_000_000` con la ragione scritta accanto, contro i 15 882 352 941 — cioè `F_max` — che quel file chiamava `coblox-v0-genesis-candidate` in contraddizione con [ADR-011]. La fixture `PD-0` non è stata toccata, che è la cosa giusta: è una fixture di hashing e non il documento di genesi.

**La correzione ha fatto emergere un accoppiamento silenzioso**, ed è la parte più interessante del lavoro sul simulatore. Lo scenario `s11_at07_launch_regime` verificava una proprietà del **regime maturo** leggendo il fondo da `recommended.py`: cambiare quel valore avrebbe cambiato in silenzio ciò che lo scenario dimostrava. Il fondo è ora un parametro esplicito dello scenario, con il valore del regime maturo passato per nome. **Non è un test indebolito, è un test che ha smesso di ereditare implicitamente la grandezza su cui ragiona.**

## Tests and verification

Rieseguito dal Lead, non letto dall'evidenza: `cargo test --locked --workspace` **110 passati**, `cargo clippy --all-targets --locked` **zero warning**, `cargo fmt --check` pulito, e i quattro strumenti Python versionati tutti OK — inclusa la prova in negativo di [SPEC-010], che continua a fallire su tutte e dieci le classi.

**La verifica decisiva è una mutazione, non un conteggio di test verdi.** Un criterio come *ogni caso `invalid` dev'essere rifiutato* si soddisfa anche con test che non vincolano nulla, quindi il Lead ha rimosso dal codice il vincolo `3 * validator_min_set_size >= 2 * V` e rieseguito la suite. Esito:

```text
test every_relational_constraint_is_enforced_individually ... FAILED
test the_consensus_parameters_min_set_relational_rule_fixtures ... FAILED
test the_direction_of_danger_for_all_economic_limits ... FAILED
panicked: violating `3 * validator_min_set_size >= 2 * V` was accepted
```

Tre test falliscono **nominando la regola**. Ripristinato il file, la suite torna verde e `params.rs` è identico a come l'implementatore l'ha lasciato, con `fmt` e `clippy` puliti. È l'evidenza che `GATE-INVALID-REJECTED` e `GATE-DIRECTION` vincolano, invece di essere caselle spuntate.

## Production quality and documentation compliance

`sim/coblox_sim/scenarios.py` e `sim/tests/test_simulator.py` non erano in *Files and areas involved*. L'estensione è conseguenza necessaria della correzione del fondo, è di tredici righe, e non indebolisce alcuna asserzione — vedi *Code observations*. Accettata.

Nessun commit né push, come il contratto impone.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

**Nessun finding.**

**Un sospetto del Lead che si è rivelato infondato, registrato perché la verifica è parte del lavoro.** `RewardBounds` porta `network_id` e `chain_id` e li confronta con la catena configurata, il che sembrava un legame che `ElectionBounds` non ha — un'asimmetria della stessa famiglia di [DEBT-014], quindi da segnalare. **`ElectionBounds` li ha già entrambi**, alle righe 34 e 36 di `params.rs`. Il rispecchiamento è esatto e non c'è alcuna asimmetria. Il Lead ha letto la struttura in modo parziale prima di controllare, ed è la stessa forma dell'errore di conteggio di [REVIEW-015].

## Required follow-up

`GATE-SECREVIEW` va attestata su una review di AGENT-007 prima di `spec_done`. Nel dispatch va detto esplicitamente che **le superfici segnalate nella spec non sono il perimetro**, come la spec stessa istruisce: tre volte in questo progetto la reviewer le ha trovate solide e ha trovato i difetti altrove.

Il divario che questa spec chiude era di cucitura fra due spec — [SPEC-009] delegava i documenti, [SPEC-008] il crate, e la regola nuova è caduta in mezzo. Vale la pena chiedersi in review se altre superfici abbiano la stessa forma.

## Final decision

**Accettata**, con `GATE-SECREVIEW` in sospeso. La spec resta in `review` fino all'attestazione.
