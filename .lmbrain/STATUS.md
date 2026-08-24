---
title: Project pulse
status: active
milestone: M-01
updated: 2026-08-25
---

# Project Pulse

## Current focus

**Decisioni dell'operatore necessarie per sbloccare M-01.** La review di sicurezza di SPEC-001 ([REVIEW-002]) non ha superato `GATE-SECREVIEW`: 18 finding, 8 gravi, tra cui una violazione di safety BFT verificata indipendentemente dal Lead e la dimostrazione quantitativa che il proof of work SHA-256 non regge l'anti-Sybil su hardware commodity. Quest'ultima **contraddice una metrica di successo dichiarata in [[PROJECT]]** ("zero accrediti a nodi emulati nei test di attacco"): serve una scelta di prodotto, non una correzione tecnica. SPEC-003 è consegnata e attende review del Lead più il gate estetico dell'operatore. SPEC-002 è ferma su un blocco reale (nessun push, quindi CI mai eseguita) più due difetti trovati dal Lead.

## Current milestone

M-01 — Fondamenta: protocollo su carta e scheletro del core.

## Ready for handoff

Nessuna: tutte e quattro le spec di M-01 sono avviate.

## In progress

- [SPEC-001] Protocollo v0 → AGENT-001 (Dario Meshnet). In `review`. [REVIEW-001] chiusa (remediation verificata), ma [REVIEW-002] (sicurezza) è **changes-requested**: `GATE-SECREVIEW` non superato, 18 finding di cui 8 gravi. Serve una seconda remediation, subordinata alle decisioni di prodotto qui sotto.
- [SPEC-002] Workspace Rust + CI → AGENT-008 (Remo Pipeline). In `working`, non consegnata: l'implementatore ha correttamente rifiutato `spec_submit` con i gate non verificati. Vedi la "Nota del Lead sul blocco dichiarato" nella spec: il blocco NDK era inesatto (il Lead ha eseguito la cross-build con successo), ma restano due difetti (script `build-android.sh` rotto, wrapper Gradle mancante) e un blocco vero su `GATE-CI-GREEN`.
- [SPEC-003] Design system → AGENT-006 (Lia Wireframe). **Consegnata**, ora in `review`. `GATE-CONTRAST` chiuso con 130/130 coppie AA; interfaccia interamente in inglese come da correzione del Lead. Attende la review del Lead e il gate `GATE-OPERATOR-LOOK` dell'operatore.
- [SPEC-004] Threat model → AGENT-007 (Greta Threatmodel). Dispacciata dal Lead il 2026-08-25, modello Opus, su decisione dell'operatore di stabilire la posizione anti-Sybil **dopo** il threat model. Il documento dovrà istruire quella decisione con numeri e conseguenze per ciascuna opzione, incluse le riformulazioni candidate della metrica di [[PROJECT]]. **In corso.**

## Blockers and risks

- **BLOCCO 1 — conflitto tra sicurezza e metrica di prodotto.** [REVIEW-002] RF-005 dimostra numericamente (rapporto telefono/GPU ~2.750×) che nessun valore di `difficulty_bits` nell'intervallo 18–40 è insieme tollerabile su Android e costoso per un attaccante. Ne segue che la metrica di successo di [[PROJECT]] "zero accrediti a nodi emulati nei test di attacco" **non è raggiungibile per via crittografica** con il design attuale. *Decisione dell'operatore del 2026-08-25: si sceglie dopo il threat model. [SPEC-004] è stata avviata proprio per istruire questa scelta; PROJECT.md resta invariato fino ad allora.*
- **BLOCCO 2 — `GATE-CI-GREEN` di SPEC-002, ora un cortocircuito.** La pipeline non ha mai eseguito perché nulla è stato spinto su `origin` (github.com/fathorMB/CobloxNetwork). Con la strategia di branching dichiarata il 2026-08-25 la dipendenza diventa circolare: il gate esige una run verde, la pipeline parte solo su push a `main`, il push avviene a `spec_done`, e `spec_done` esige il gate. Analisi e due vie d'uscita nella sezione dedicata di [SPEC-002]; il Lead raccomanda un push di bootstrap una tantum. Decisione dell'operatore.
- Violazione di safety BFT ([REVIEW-002] RF-002) verificata indipendentemente dal Lead: con potere di voto totale 101 la regola `2f+1` dà 67 e la regola "due terzi più uno" dà 68; due certificati di quorum in conflitto possono essere entrambi validi. Correzione a costo zero, ma va fatta prima che due implementazioni divergano.
- Nome del token non ancora deciso (placeholder: `◇`). Lia propone JetBrains Mono come monospace: decisione dell'operatore.

## Next recommended actions

1. Operatore: autorizzare la seconda remediation di SPEC-001 ad AGENT-001 sul **Lotto A** dei finding di [REVIEW-002] (triage completo nella spec). Il Lotto B attende [SPEC-004].
2. Operatore: sciogliere BLOCCO 2 (autorizzare il primo push, o accettare una deroga documentata sul gate CI di SPEC-002); e far correggere a AGENT-008 i due difetti trovati dal Lead.
3. Lead: recensire SPEC-003 (consegnata); l'operatore attesta poi `GATE-OPERATOR-LOOK`.
4. Operatore: decidere nome del token/unità e font monospace (AGENT-006 propone JetBrains Mono).
5. Alla consegna di [SPEC-004]: decisione anti-Sybil, riformulazione della metrica in [[PROJECT]], e sblocco del Lotto B.
6. Definire la strategia di branching (`branching_strategy_set`) insieme a SPEC-002.

## Strategia di branching

Dichiarata dall'operatore il 2026-08-25 e registrata in `.lmbrain/BRANCHING.json` via `branching_strategy_set`: topologia **main-only**, nessun branch di feature. Gli specialisti lavorano sul working tree e **non fanno mai commit né push**; il Project Lead è l'unico autorizzato, e committa e pusha su `main` **al passaggio di una spec a `done`**. Vincolo aggiuntivo: **nessuna produzione di installer o release lato GitHub per ora** — la CI di [SPEC-002] è già conforme (Tauri con `--bundles none`, nessun job di release).

## Recent scope clarifications

- 2026-08-25 — **Lingua del prodotto: inglese** per tutto ciò che vede l'utente finale (rilievo dell'operatore sulle anteprime di SPEC-003). Registrata come vincolo in [[PROJECT]]. SPEC-003 corretta in corso d'opera e AGENT-006 avvisata mentre era ancora in lavorazione; la formulazione originale della spec ("tono del copy (it/en)") era ambigua per responsabilità del Lead, non dell'implementatrice. Da applicare d'ora in poi a ogni spec con superficie utente (SPEC-002 espone solo una schermata di versione, impatto trascurabile ma da verificare in review).

## Recent profile changes

- 2026-08-25 — AGENT-007: `can_implement` portato a true con vincolo "solo deliverable documentali di sicurezza, mai codice" (approvato dall'operatore per SPEC-004); sui propri documenti la review spetta al Lead.

## Recent decisions

- ADR-006 — Pubblicazione delle app e ricompensa al creatore (accepted, 2026-08-25). Estende ADR-005 con una nuova categoria di emissione. **Impatto su lavoro in corso:** vincola i campi del manifest in SPEC-001 (repliche, tetti di risorse, prezzo di abbonamento) — da comunicare all'implementatore o da verificare in review.
- ADR-001 — Ledger su federazione BFT con validatori a rotazione (accepted, 2026-08-25)
- ADR-002 — Proof of contribution tramite challenge crittografici (accepted, 2026-08-25)
- ADR-003 — Core del nodo in Rust con shell native (accepted, 2026-08-25)
- ADR-004 — Runtime delle app in sandbox WASM/WASI (accepted, 2026-08-25)
- ADR-005 — Economia del token a mint & burn (accepted, 2026-08-25)
