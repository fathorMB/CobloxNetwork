---
title: Project pulse
status: active
milestone: M-01
updated: 2026-08-25
---

# Project Pulse

## Current focus

**Sessione autonoma del Lead, notte del 2026-08-25.** Su mandato dell'operatore ("completare tutte le spec aperte, salvare i debiti, occuparti di dispatch e review"), il Lead opera senza attendere conferme. Risultati finora: **[SPEC-004] chiusa** (prima spec `done` del progetto), **[ADR-007] accettato** con la decisione anti-Sybil presa su delega, metrica di successo di [[PROJECT]] riformulata, **[SPEC-003] accettata tecnicamente** ([REVIEW-004]) ma bloccata dal gate dell'operatore, tre debiti nuovi registrati, primo commit di chiusura spinto su `main`. In lavorazione: Lotto B su SPEC-001 e remediation su SPEC-002.

## Current milestone

M-01 — Fondamenta: protocollo su carta e scheletro del core.

## Ready for handoff

Nessuna: tutte e quattro le spec di M-01 sono avviate.

## In progress

- [SPEC-001] Protocollo v0 → AGENT-001 (Dario Meshnet). In `review`. Lotto A rimediato e verificato. **Lotto B dispacciato dal Lead nella notte** (modello Opus), ora sbloccato da [ADR-007]: Argon2id al posto di SHA-256 con controlli riordinati contro il DoS, vincolo `k < 1` sulla quota al creatore, revoca che forza la transizione del validator set, casualità delle challenge verificabile, consenso dell'host valutabile automaticamente. Poi resta la sola ri-attestazione di `GATE-SECREVIEW` da parte di AGENT-007, ora libera. **In corso.**
- [SPEC-002] Workspace Rust + CI → AGENT-008 (Remo Pipeline). In `working`. **Remediation dispacciata dal Lead il 2026-08-25 su autorizzazione dell'operatore, modello Sonnet**: chiusura dei tre difetti trovati dal Lead (script `build-android.sh` rotto per `-p` invece di `--platform`, wrapper Gradle mancante, `.gitignore` di root assente) più le verifiche locali, il cui standard si alza perché con la CI ferma `GATE-LOCAL-REPRO` è l'unica verifica reale che la spec produce. **In corso.**
- [SPEC-003] Design system → AGENT-006 (Lia Wireframe). In `review`, **accettata tecnicamente** dal Lead con [REVIEW-004]: sette criteri su sette verificati meccanicamente, un solo finding di severità bassa che non blocca. Il pacchetto vive in `.lmbrain/design/coblox-design-system/`. **Bloccata dal solo `GATE-OPERATOR-LOOK`:** il sistema rifiuta `spec_done` finché l'operatore non attesta di aver visto e approvato la direzione estetica, e il Lead non può farlo al suo posto.

## Done

- [SPEC-004] Threat model iniziale → AGENT-007 (Greta Threatmodel). **`done` il 2026-08-25**, prima spec chiusa del progetto. Accettata con [REVIEW-003], `GATE-LEAD-MAP` attestato dal Lead. Ha prodotto `.lmbrain/knowledge/threat-model.md` (1930 righe, 36 scenari, 24 `SEC-REQ`, 15 test di attacco) e ha istruito [ADR-007]. Commit `024f81f` spinto su `main`.

## Blockers and risks

- **BLOCCO 1 — sciolto con [ADR-007].** La metrica "zero accrediti a nodi emulati" era irraggiungibile per via crittografica. Il Lead ha adottato su delega l'opzione 4a di [SPEC-004]: difesa economica (fondo a tetto per il reddito di esistenza, frazione `α` sorvegliata, eleggibilità a validatore ancorata a lavoro difficile da falsificare) più Argon2id come pavimento d'ingresso. La metrica in [[PROJECT]] è stata riformulata di conseguenza. **Da rivedere con l'operatore al risveglio:** è una decisione di prodotto presa in sua assenza e, se non concorda, va superata con una nuova ADR e non modificata in silenzio.
- **Attenzione, decisione delegata di rilievo:** il progetto ora dichiara di essere robusto contro la falsificazione ma **non** resistente ai Sybil per via crittografica. È una rinuncia esplicita a una promessa, resa in cambio di onestà verificabile.
- **BLOCCO 2 — risolto per ora con una deroga.** La prima run CI (commit `4ea0db9`) è fallita in 6 secondi **senza eseguire alcun job**: `The job was not started because recent account payments have failed or your spending limit needs to be increased`. Problema dell'account GitHub, non del codice. L'operatore ha concesso il 2026-08-25 la deroga su `GATE-CI-GREEN`, registrata come **[DEBT-001]** (open, owner AGENT-008, severità high). I criteri che richiedevano la verifica *in CI* sono marcati `[~] ... | waived=DEBT-001`; la verifica locale equivalente **non** è derogata. Alla ripresa della fatturazione, DEBT-001 impone una run verde e la ri-attestazione del gate.
- Nome del token non ancora deciso (placeholder: `◇`). Lia propone JetBrains Mono come monospace: decisione dell'operatore.

## Debiti aperti

| ID | Severità | Owner | Questione |
| --- | --- | --- | --- |
| [DEBT-005] | critical | AGENT-002 | Il set di validatori è auto-perpetuante: manca la regola di elezione. Nessuna devnet deve accumulare storia conservabile prima che sia scritta. |
| [DEBT-001] | high | AGENT-008 | La pipeline CI non è mai stata eseguita, `GATE-CI-GREEN` derogato. Sbloccabile solo dalla fatturazione GitHub dell'operatore. |
| [DEBT-006] | high | AGENT-LEAD | La quota al creatore di [ADR-006] obbliga a pubblicare chi è abbonato a cosa. È l'unica superficie priva di un ADR alle spalle. |
| [DEBT-007] | high | AGENT-002 | La forma del reddito di esistenza non è decisa e determina `α`, il parametro più importante dell'economia. |

## Next recommended actions

**Per l'operatore, al risveglio:**

1. **Rivedere [ADR-007]**, la decisione anti-Sybil presa su delega: riformula una promessa di prodotto. Se non concordi va superata con una nuova ADR, non modificata a mano.
2. **Attestare `GATE-OPERATOR-LOOK`** guardando `.lmbrain/design/coblox-design-system/index.html`: è l'unico passo che separa SPEC-003 da `done`.
3. **Sbloccare la fatturazione GitHub** per chiudere [DEBT-001] e far girare finalmente la pipeline.
4. Decidere nome del token/unità e font monospace (AGENT-006 propone JetBrains Mono).

**Per il Lead, in autonomia:**

5. Alla consegna del Lotto B: verificare, poi chiedere ad AGENT-007 la ri-attestazione di `GATE-SECREVIEW` e chiudere SPEC-001.
6. Alla consegna di SPEC-002: recensire; `GATE-CI-GREEN` è derogato, quindi la chiusura dipende solo da `GATE-LOCAL-REPRO` e dai tre difetti.
7. A ogni spec che passa a `done`: commit e push su `main`.

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
