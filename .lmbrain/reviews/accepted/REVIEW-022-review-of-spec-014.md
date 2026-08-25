---
id: REVIEW-022
# Note: Quote the title if it contains a colon
title: "Review of SPEC-014"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-014
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-022-EVENT-001"
    timestamp: "2026-08-25T23:25:37.338304300+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata con un solo finding low, non bloccante e rinviato al giro di remediation della review di sicurezza invece di aprirne uno proprio.\n\nLa prova portante e stata rifatta dal Lead compilando e non letta dall'evidenza: una sonda temporanea che passa i byte di un Digest32 e un letterale di byte grezzi a verify_consensus_ed25519 produce due errori E0308 con expected SigningPreimage, e il tipo e imposto su entrambi i punti d'ingresso e non su uno solo, che era il modo di sbagliare nominato nella spec.\n\nLa via non-consensus e nominata bene ed e il criterio che la spec temeva di piu: from_raw_bytes_non_consensus porta la propria natura nel nome, e una ricerca in albero mostra che i suoi unici utilizzatori sono in speccheck_conformance.rs, otto occorrenze, tutte nella suite di conformita. Nessun percorso di consenso la tocca.\n\nLa riscrittura dei tre chiamanti e migliore dell'originale: passano ora per la validazione completa contro bound permissivi, e la riga 525 asserisce che la policy base e ammissibile sotto quegli stessi bound, il che produce un argomento differenziale che l'originale non aveva. Nulla di perduto e qualcosa di guadagnato.\n\n126 test passati, identici al conteggio precedente, clippy zero warning, fmt pulito, cinque strumenti versionati OK, nessun valore pubblicato mosso.\n\nRF-001 low: il campo del tipo e pub(crate), quindi qualunque modulo del crate puo costruire una SigningPreimage da byte arbitrari. Il criterio e soddisfatto verso l'esterno e nominale all'interno. Verificato che nessuno lo fa oggi e che l'ampiezza non serve, perche il verificatore legge tramite as_bytes e tutti i costruttori vivono nello stesso modulo della definizione. E la stessa forma del difetto che questa spec chiude, un livello piu in dentro, e proprio dentro il confine in cui verra scritto il codice dei chiamanti di consenso.\n\nGATE-SECREVIEW resta da attestare, e il dispatch e in coda dietro la valutazione di DEBT-014 e DEBT-013."
    evidence_refs: ["SPEC-014", "DEBT-015", "DEBT-016", "REVIEW-019"]
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

**Accettata con un finding `low`**, non bloccante e deliberatamente rinviato al giro di remediation della review di sicurezza invece di aprirne uno proprio. `GATE-SECREVIEW` resta da attestare.

## Acceptance-criteria compliance

**La prova portante è stata rifatta dal Lead compilando**, non letta dall'evidenza. Una sonda temporanea che passa i byte di un `Digest32` e un letterale di byte grezzi a `verify_consensus_ed25519`:

```text
error[E0308]: mismatched types
  --> tests\_lead_compile_probe.rs:6:50
   |  expected `&SigningPreimage`, found `&[u8; 32]`
error[E0308]: mismatched types
  --> tests\_lead_compile_probe.rs:7:50
   |  expected `&SigningPreimage`, found `&[u8; 3]`
error: could not compile `coblox-core` due to 2 previous errors
```

Il tipo è imposto su **entrambi** i punti d'ingresso — `SignatureVerifier::verify` a `lib.rs:143` e `verify_consensus_ed25519` a `verifier.rs:78` — e non su uno solo, che era il modo di sbagliare nominato nella spec.

`RewardPolicy::check_internal` e `check_magnitudes` sono ora privati (`params.rs:720` e `767`), esattamente come i gemelli del lato consenso. La locuzione di `verifier.rs` non dice più «audited primitive crate» e rimanda a `Cargo.toml` per la provenienza degli audit, che è dove RF-005 di [REVIEW-019] l'aveva resa esatta.

## Code observations

**La via non-consensus è nominata bene, ed è il criterio che la spec temeva di più.** `SigningPreimage::from_raw_bytes_non_consensus` porta la propria natura nel nome, è documentata come tale, e una ricerca in albero mostra che **i suoi unici utilizzatori sono in `speccheck_conformance.rs`** — otto occorrenze, tutte nella suite di conformità. Nessun percorso di consenso la tocca. Non è un costruttore generico senza nome, che era il modo in cui questa spec poteva sembrare chiusa senza esserlo.

**La riscrittura dei tre chiamanti è migliore dell'originale**, e va detto. I casi passano ora per `validate_reward`, cioè per la validazione completa contro bound permissivi, e la riga 525 asserisce che **la policy base è ammissibile** sotto quegli stessi bound. Ne discende un argomento differenziale che l'originale non aveva: la base passa, ogni caso mutato differisce per un solo campo e viene rifiutato, quindi **il rifiuto viene dal campo mutato** e non da un controllo che scatta prima. `check_internal().is_err()` non nominava la variante di errore, quindi non c'è nulla di perduto e c'è qualcosa di guadagnato.

Il commento sulla precedenza fra controlli — che la spec chiedeva di leggere prima di toccare i test — è conservato e resta accurato.

## Tests and verification

Rieseguito dal Lead: **126 test passati**, identici al conteggio prima della passata, clippy zero warning, `fmt` pulito, tutti e cinque gli strumenti versionati OK. **Nessun valore pubblicato si è mosso**, che è la cosa che questa spec doveva *non* fare.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

- **RF-001 | category=design | severity=low | criterion=«Il tipo non è costruibile da byte arbitrari se non attraverso una via nominata e documentata come non-consensus»**

  Il campo del tipo è dichiarato `pub(crate)` (`registry.rs:332`), quindi **qualunque modulo di `coblox-core` può costruire una `SigningPreimage` da byte arbitrari** senza passare né per `signing_preimage` né per la via nominata. Il criterio è soddisfatto verso l'esterno del crate e **nominale al suo interno**.

  Verificato che oggi **nessuno lo fa**: nessuna costruzione diretta esiste fuori da `registry.rs`. Verificato anche che l'ampiezza **non serve**: il verificatore legge i byte tramite l'accessore `as_bytes()` (`verifier.rs:115`), e tutti i costruttori vivono nello stesso modulo della definizione, quindi un campo privato basterebbe. La correzione è una parola chiave.

  Non è bloccante e non merita un giro di remediation proprio. Merita però di essere registrato, perché è **la stessa forma del difetto che questa spec chiude** — una garanzia tenuta dalla convenzione invece che dal tipo — un livello più in dentro, e proprio dentro il confine in cui verrà scritto il codice dei chiamanti di consenso che questa spec esiste per preparare.

  **Rimedio:** restringere la visibilità del campo, nello stesso giro di remediation di ciò che emergerà da `GATE-SECREVIEW`.

## Required follow-up

`GATE-SECREVIEW` su una review di AGENT-007. Nel dispatch va incluso RF-001, perché la forma della via non-consensus è materia sua, e va detto — come sempre — che **le superfici segnalate non sono il perimetro**.

Il dispatch è **in coda** dietro la valutazione di [DEBT-014] e [DEBT-013], che AGENT-007 sta svolgendo: sono lavoro sostanziale con una finestra che si chiude alla devnet, e frammentarlo per anticipare una review su una superficie piccola sarebbe il compromesso sbagliato.

## Final decision

**Accettata**, con RF-001 `low` registrato e rinviato. La spec resta in `review` fino all'attestazione di `GATE-SECREVIEW`.
