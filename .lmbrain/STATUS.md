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

- [SPEC-001] Protocollo v0 → AGENT-001 (Dario Meshnet). In `review`. [REVIEW-001] chiusa. Su [REVIEW-002] il **Lotto A è rimediato e verificato dal Lead** (14 finding su 18, inclusi tutti i gravi tranne RF-005 e RF-007): documenti passati da 1327 a 1709 righe, 18 esempi JSON canonici, 16 link risolti, e le fixture di identità ora derivano davvero il node ID dalla chiave. **Restano due condizioni per `done`:** (1) il Lotto B, subordinato alla decisione anti-Sybil; (2) la ri-attestazione di `GATE-SECREVIEW` da parte di AGENT-007, oggi impegnata su [SPEC-004].
- [SPEC-002] Workspace Rust + CI → AGENT-008 (Remo Pipeline). In `working`. **Remediation dispacciata dal Lead il 2026-08-25 su autorizzazione dell'operatore, modello Sonnet**: chiusura dei tre difetti trovati dal Lead (script `build-android.sh` rotto per `-p` invece di `--platform`, wrapper Gradle mancante, `.gitignore` di root assente) più le verifiche locali, il cui standard si alza perché con la CI ferma `GATE-LOCAL-REPRO` è l'unica verifica reale che la spec produce. **In corso.**
- [SPEC-003] Design system → AGENT-006 (Lia Wireframe). **Consegnata**, ora in `review`. `GATE-CONTRAST` chiuso con 130/130 coppie AA; interfaccia interamente in inglese come da correzione del Lead. Su richiesta dell'operatore il pacchetto è stato spostato in **`.lmbrain/design/coblox-design-system/`** secondo la convenzione del brain, così da essere visibile nell'app; generatori e link verificati dopo lo spostamento. Attende la review del Lead e il gate `GATE-OPERATOR-LOOK` dell'operatore.
- [SPEC-004] Threat model → AGENT-007 (Greta Threatmodel). Dispacciata dal Lead il 2026-08-25, modello Opus, su decisione dell'operatore di stabilire la posizione anti-Sybil **dopo** il threat model. Il documento dovrà istruire quella decisione con numeri e conseguenze per ciascuna opzione, incluse le riformulazioni candidate della metrica di [[PROJECT]]. **In corso.**

## Blockers and risks

- **BLOCCO 1 — conflitto tra sicurezza e metrica di prodotto.** [REVIEW-002] RF-005 dimostra numericamente (rapporto telefono/GPU ~2.750×) che nessun valore di `difficulty_bits` nell'intervallo 18–40 è insieme tollerabile su Android e costoso per un attaccante. Ne segue che la metrica di successo di [[PROJECT]] "zero accrediti a nodi emulati nei test di attacco" **non è raggiungibile per via crittografica** con il design attuale. *Decisione dell'operatore del 2026-08-25: si sceglie dopo il threat model. [SPEC-004] è stata avviata proprio per istruire questa scelta; PROJECT.md resta invariato fino ad allora.*
- **BLOCCO 2 — risolto per ora con una deroga.** La prima run CI (commit `4ea0db9`) è fallita in 6 secondi **senza eseguire alcun job**: `The job was not started because recent account payments have failed or your spending limit needs to be increased`. Problema dell'account GitHub, non del codice. L'operatore ha concesso il 2026-08-25 la deroga su `GATE-CI-GREEN`, registrata come **[DEBT-001]** (open, owner AGENT-008, severità high). I criteri che richiedevano la verifica *in CI* sono marcati `[~] ... | waived=DEBT-001`; la verifica locale equivalente **non** è derogata. Alla ripresa della fatturazione, DEBT-001 impone una run verde e la ri-attestazione del gate.
- Violazione di safety BFT ([REVIEW-002] RF-002) verificata indipendentemente dal Lead: con potere di voto totale 101 la regola `2f+1` dà 67 e la regola "due terzi più uno" dà 68; due certificati di quorum in conflitto possono essere entrambi validi. Correzione a costo zero, ma va fatta prima che due implementazioni divergano.
- Nome del token non ancora deciso (placeholder: `◇`). Lia propone JetBrains Mono come monospace: decisione dell'operatore.

## Next recommended actions

1. Alla consegna di [SPEC-004]: decisione anti-Sybil dell'operatore, poi remediation del **Lotto B** e ri-attestazione di `GATE-SECREVIEW` da parte di AGENT-007. Sono le ultime due condizioni per chiudere SPEC-001.
2. Operatore: sbloccare la fatturazione GitHub per chiudere [DEBT-001]; nel frattempo AGENT-008 sta correggendo i tre difetti trovati dal Lead.
3. Lead: recensire SPEC-003 (consegnata); l'operatore attesta poi `GATE-OPERATOR-LOOK`.
4. Operatore: decidere nome del token/unità e font monospace (AGENT-006 propone JetBrains Mono).
5. Alla decisione anti-Sybil: riformulare la metrica di successo in [[PROJECT]].
6. **Alla prima spec che passa a `done`:** il Lead committa e pusha, includendo lo spostamento del pacchetto di design in `.lmbrain/design/` (19 rinomine oggi non ancora committate, per volontà dell'operatore).

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
