---
title: Project pulse
status: active
milestone: M-01
updated: 2026-08-25
---

# Project Pulse

## Current focus

**Sessione autonoma del Lead conclusa, notte del 2026-08-25.** Su mandato dell'operatore ("completare tutte le spec aperte, salvare i debiti, occuparti di dispatch e review"), il Lead ha operato senza attendere conferme.

**M-01 è di fatto completata: tre spec su quattro sono `done`** ([SPEC-001], [SPEC-002], [SPEC-004]), e la quarta ([SPEC-003]) è accettata tecnicamente e attende solo che l'operatore guardi i mockup. Sette review prodotte, sette dispatch, [ADR-007] deciso su delega con la metrica di [[PROJECT]] riformulata, quattro debiti nuovi registrati, tre commit spinti su `main`.

Il Lead segnala all'operatore due cose in primo piano: la decisione di [ADR-007] tocca una promessa di prodotto ed è superabile se non concorda; e un proprio errore di processo è documentato in `.lmbrain/knowledge/commit-discipline.md`.

**Ripresa del 2026-08-25, sessione successiva: la CI è verde e il repository è pubblico.** L'operatore ha sbloccato la fatturazione GitHub e reso pubblico il repository. La pipeline ha eseguito per la prima volta e ora passa su tutti e cinque i job; [DEBT-001] è chiuso. Il risultato che conta oltre il colore: **le tre rotture emerse non erano problemi di toolchain cross-platform**, cioè il rischio n.1 di [ADR-003], ma difetti di confezionamento del repository — due bit di esecuzione persi perché i file erano stati committati da Windows, e un ordine di step sbagliato nel workflow. Rust, NDK, cargo-ndk, UniFFI e Tauri hanno funzionato al primo tentativo utile su Windows e su Linux.

Il passaggio a pubblico ha portato con sé il proprio lavoro di sicurezza, in parte fatto e in parte da decidere; vedi la sezione *Postura di sicurezza del repository pubblico*.

## Handoff attivo

[HANDOFF-001] — consegna della sessione del 2026-08-25. Il Lead entrante lo legga per primo: contiene stato, decisioni prese su delega, debiti e prossime azioni.

## Current milestone

M-01 — Fondamenta: protocollo su carta e scheletro del core.

## Ready for handoff

Nessuna: tutte e quattro le spec di M-01 sono avviate.

## In progress

Nessuna spec in lavorazione. L'unica non chiusa è [SPEC-003], ferma sul solo gate dell'operatore.
- [SPEC-003] Design system → AGENT-006 (Lia Wireframe). In `review`, **accettata tecnicamente** dal Lead con [REVIEW-004]: sette criteri su sette verificati meccanicamente, un solo finding di severità bassa che non blocca. Il pacchetto vive in `.lmbrain/design/coblox-design-system/`. **Bloccata dal solo `GATE-OPERATOR-LOOK`:** il sistema rifiuta `spec_done` finché l'operatore non attesta di aver visto e approvato la direzione estetica, e il Lead non può farlo al suo posto.

## Done

- [SPEC-001] Protocollo v0 → AGENT-001 (Dario Meshnet). **`done` il 2026-08-25**, dopo tre giri di remediation di sicurezza. `GATE-SECREVIEW` attestato: AGENT-007 l'ha bocciato due volte ([REVIEW-002] con 18 finding, [REVIEW-006] con 4 gravi residui) e superato alla terza ([REVIEW-007]). I documenti sono passati da 1268 a 2607 righe. **Due contestazioni di AGENT-001 sono state confermate dalla reviewer come migliori della sua stessa condizione di chiusura**: il pavimento Argon2id imposto come area più memoria minima invece di `iterations ≥ 3`, che avrebbe rifiutato il profilo RFC 9106 più forte; e lo scudo di ammissione adattivo con validazione della sorgente invece di un puzzle fisso, che avrebbe reintrodotto il divario CPU/GPU per cui [ADR-007] esiste. Residui in [DEBT-008].
- [SPEC-002] Workspace Rust + CI → AGENT-008 (Remo Pipeline). **`done` il 2026-08-25**, accettata con [REVIEW-005]. Tre difetti chiusi più sei problemi ulteriori scoperti eseguendo davvero il runbook, fra cui un flag Tauri inesistente che il Lead stesso aveva citato come prova senza eseguirlo. `GATE-LOCAL-REPRO` soddisfatto e in parte rieseguito dal Lead; `GATE-CI-GREEN` era derogato e coperto da [DEBT-001]. Commit `81cca93`. **Deroga rientrata il 2026-08-25:** la run 32821923135 sul commit `6b9ad1f` è verde su tutti e cinque i job, con `cargo fmt`, `clippy -D warnings` e `cargo-deny` eseguiti come step distinti e riusciti; [DEBT-001] è risolto. Ne è però emerso [DEBT-009]: quel `cargo-deny` non copre `apps/desktop/src-tauri`, escluso dal workspace.
- [SPEC-004] Threat model iniziale → AGENT-007 (Greta Threatmodel). **`done` il 2026-08-25**, prima spec chiusa del progetto. Accettata con [REVIEW-003], `GATE-LEAD-MAP` attestato dal Lead. Ha prodotto `.lmbrain/knowledge/threat-model.md` (1930 righe, 36 scenari, 24 `SEC-REQ`, 15 test di attacco) e ha istruito [ADR-007]. Commit `024f81f` spinto su `main`.

## Blockers and risks

- **Il claim di sicurezza, nella forma che AGENT-007 giudica difendibile.** La rete è robusta contro la falsificazione ma **non** resistente ai Sybil per via crittografica, e tre cose non sono garantite: la disponibilità dell'enrollment sotto attacco sostenuto (i dispositivi lenti soffrono per primi), la resistenza Sybil crittografica, e la verifica indipendente dell'eleggibilità a validatore prima di M-02. Parole della reviewer: *"il progetto non deve chiamare la rete super-sicura senza quelle tre frasi accanto; con quelle accanto il claim è più solido della media a questo stadio, e la parte migliore non è nessun singolo meccanismo ma il fatto che i limiti siano quantificati."*
- **BLOCCO 1 — sciolto con [ADR-007].** La metrica "zero accrediti a nodi emulati" era irraggiungibile per via crittografica. Il Lead ha adottato su delega l'opzione 4a di [SPEC-004]: difesa economica (fondo a tetto per il reddito di esistenza, frazione `α` sorvegliata, eleggibilità a validatore ancorata a lavoro difficile da falsificare) più Argon2id come pavimento d'ingresso. La metrica in [[PROJECT]] è stata riformulata di conseguenza. **Confermata dall'operatore il 2026-08-25**, dopo revisione congiunta: [ADR-007] resta `accepted`. Nella stessa sessione l'operatore ha però chiesto di riaprire le esclusioni permanenti che avevano collassato lo spazio delle alternative prima ancora che [SPEC-004] cominciasse a enumerarle — ne è nata [ADR-008]. Resta aperto il valore `X` della metrica riformulata, che dipende dal simulatore di M-02 e da [DEBT-007].
- **Attenzione, decisione delegata di rilievo:** il progetto ora dichiara di essere robusto contro la falsificazione ma **non** resistente ai Sybil per via crittografica. È una rinuncia esplicita a una promessa, resa in cambio di onestà verificabile.
- **BLOCCO 2 — chiuso.** La prima run CI (commit `4ea0db9`) era fallita in 6 secondi **senza eseguire alcun job**, per la fatturazione dell'account GitHub e non per il codice; l'operatore aveva concesso la deroga su `GATE-CI-GREEN`, registrata come [DEBT-001]. Il 2026-08-25 la fatturazione è stata sbloccata, la pipeline ha eseguito e, dopo due giri di remediation, la run 32821923135 è verde su tutti i job. [DEBT-001] è **risolto**. I criteri di [SPEC-002] marcati `[~] ... | waived=DEBT-001` non sono più coperti da una deroga ma da una run reale.
- Nome del token non ancora deciso (placeholder: `◇`). Lia propone JetBrains Mono come monospace: decisione dell'operatore.

## Debiti aperti

| ID | Severità | Owner | Questione |
| --- | --- | --- | --- |
| [DEBT-005] | critical | AGENT-002 | Il set di validatori è auto-perpetuante: manca la regola di elezione. Nessuna devnet deve accumulare storia conservabile prima che sia scritta. |
| [DEBT-009] | high | AGENT-008 | `cargo-deny` non vede il grafo di `apps/desktop/src-tauri`, escluso dal workspace: la CI riporta verde su dipendenze mai controllate. Già un advisory sfuggito (GHSA-wrw7-89jp-8q8g su `glib`). |
| [DEBT-006] | high | AGENT-LEAD | La quota al creatore di [ADR-006] obbliga a pubblicare chi è abbonato a cosa. È l'unica superficie priva di un ADR alle spalle. |
| [DEBT-007] | high | AGENT-002 | La forma del reddito di esistenza non è decisa e determina `α`, il parametro più importante dell'economia. |
| [DEBT-008] | low | AGENT-001 | Due frasi della specifica del protocollo promettono poco più di quanto le regole impongano. Una riga ciascuna, M-02. |

Risolti: [DEBT-001] il 2026-08-25, con la run CI verde 32821923135. È il primo debito chiuso del progetto.

## Next recommended actions

**Per l'operatore, al risveglio:**

1. ~~Rivedere [ADR-007]~~ — **fatto il 2026-08-25**: confermata, e dalla revisione è nata [ADR-008]. Resta da fissare `X`, ma dipende dal simulatore di M-02.
2. **Attestare `GATE-OPERATOR-LOOK`** guardando `.lmbrain/design/coblox-design-system/index.html`: è l'unico passo che separa SPEC-003 da `done`.
3. ~~Sbloccare la fatturazione GitHub~~ — **fatto il 2026-08-25**, [DEBT-001] chiuso.
4. Decidere nome del token/unità e font monospace (AGENT-006 propone JetBrains Mono).

5. **Decidere sui file di configurazione degli harness** (`.codex/`, `.pi/`, `.mcp.json`, `opencode.json`): il Lead li ha esclusi dai commit perché contengono percorsi assoluti della macchina e il nome utente. Vanno aggiunti al `.gitignore` oppure resi portabili. **Ora più urgente:** il repository è pubblico e questi file restano untracked solo per disciplina manuale, non per una regola.
6. **Decidere licenza e canale di disclosure** del repository pubblico: vedi la sezione seguente.

**Per il Lead, in autonomia:**

6. Alla consegna del Lotto B: verificare, poi chiedere ad AGENT-007 la ri-attestazione di `GATE-SECREVIEW` e chiudere SPEC-001. È l'ultima spec aperta di M-01.
7. A ogni spec che passa a `done`: commit e push su `main`.

## Postura di sicurezza del repository pubblico

Il repository `github.com/fathorMB/CobloxNetwork` è pubblico dal 2026-08-25. Quanto segue è lo stato verificato quel giorno.

**Audit della storia dei commit: pulita.** Nessuna chiave, token o credenziale in nessun commit — scansionati i pattern `ghp_`, `gho_`, `github_pat_`, `sk-`, `AKIA`, e le intestazioni di chiave privata PEM. I file di configurazione degli harness, che contengono davvero percorsi assoluti e nome utente, non sono mai entrati in un commit. Unica esposizione residua, di severità bassa: le trascrizioni PowerShell delle evidenze in [SPEC-002] mostrano `E:\Git\CobloxNetwork` e `F:/dev/android-sdk`. Sono metadati di ambiente, senza username né email.

**Attivato dal Lead il 2026-08-25, su autorizzazione esplicita dell'operatore:**

- Secret scanning e **push protection** — quest'ultima è l'unico controllo che agisce in tempo, perché blocca un segreto prima che diventi pubblico anziché segnalarlo dopo.
- Dependabot alerts e security updates. Hanno prodotto un risultato entro pochi minuti, che è come [DEBT-009] è stato scoperto.
- Ruleset su `main` che vieta force-push e cancellazione del branch. Deliberatamente **non** richiede pull request: la strategia main-only con push diretto del Lead resta intatta.

*Non* disponibile: `secret_scanning_non_provider_patterns` richiede GitHub Advanced Security e resta disabilitato sul piano attuale. In pratica significa che vengono riconosciuti i formati di segreto dei provider noti, non quelli inventati dal progetto — rilevante se in futuro Coblox definisse un proprio formato di chiave.

**Aperto, e in attesa dell'operatore:**

- **Nessuna `LICENSE`.** Un repository pubblico senza licenza è, per default legale, *tutti i diritti riservati*: nessuno può forkare, modificare o contribuire legalmente. Per un progetto che si fonda su una rete di nodi volontari è una contraddizione da sciogliere presto.
- **Nessun `SECURITY.md`.** Non esiste un canale per segnalare una vulnerabilità in privato. Il progetto dichiara la sicurezza come proprietà portante e ora lo fa in pubblico, dove qualcuno può trovare qualcosa davvero.
- **Le action di terze parti non sono pinnate a SHA.** Il workflow ne usa cinque per tag mutabile (`dtolnay/rust-toolchain`, `Swatinem/rust-cache`, `EmbarkStudios/cargo-deny-action`, `android-actions/setup-android`, più le `actions/*` ufficiali). Un tag ripuntato dal manutentore, o un suo account compromesso, esegue codice arbitrario nel runner. Mitigazione già in essere: il `GITHUB_TOKEN` è di default in sola lettura e la CI non usa segreti, quindi oggi non c'è nulla da esfiltrare — la posta in gioco sale il giorno in cui la pipeline firmerà o pubblicherà artefatti.

## Strategia di branching

Dichiarata dall'operatore il 2026-08-25 e registrata in `.lmbrain/BRANCHING.json` via `branching_strategy_set`: topologia **main-only**, nessun branch di feature. Gli specialisti lavorano sul working tree e **non fanno mai commit né push**; il Project Lead è l'unico autorizzato, e committa e pusha su `main` **al passaggio di una spec a `done`**. Vincolo aggiuntivo: **nessuna produzione di installer o release lato GitHub per ora** — la CI di [SPEC-002] è già conforme (Tauri con `--bundles none`, nessun job di release).

## Recent scope clarifications

- 2026-08-25 — **Lingua del prodotto: inglese** per tutto ciò che vede l'utente finale (rilievo dell'operatore sulle anteprime di SPEC-003). Registrata come vincolo in [[PROJECT]]. SPEC-003 corretta in corso d'opera e AGENT-006 avvisata mentre era ancora in lavorazione; la formulazione originale della spec ("tono del copy (it/en)") era ambigua per responsabilità del Lead, non dell'implementatrice. Da applicare d'ora in poi a ogni spec con superficie utente (SPEC-002 espone solo una schermata di versione, impatto trascurabile ma da verificare in review).

## Recent profile changes

- 2026-08-25 — AGENT-007: `can_implement` portato a true con vincolo "solo deliverable documentali di sicurezza, mai codice" (approvato dall'operatore per SPEC-004); sui propri documenti la review spetta al Lead.

## Recent decisions

- ADR-008 — Il divieto di proof-of-work continuo colpisce il lavoro sprecato, non il lavoro campionato (accepted, 2026-08-25). Nata dalla revisione di [ADR-007] con l'operatore. Non abroga l'esclusione: la sostituisce con un principio più un test in tre punti. **Ha sanato una contraddizione che nessuno aveva messo alla prova:** `PROJECT.md` escludeva il proof-of-work continuo «di qualsiasi tipo» mentre [ADR-002] prescrive proof-of-retrievability continuo e ri-esecuzione WASM, quindi il protocollo violava già una propria esclusione dichiarata. **Impatto su lavoro futuro:** ogni ADR o spec che introduca lavoro remunerato deve dichiarare l'esito dei tre punti del test, ed è materia di review verificarlo; il punto 1 vincola la specifica di elezione dei validatori di M-02, già gravata da [DEBT-005].
- ADR-006 — Pubblicazione delle app e ricompensa al creatore (accepted, 2026-08-25). Estende ADR-005 con una nuova categoria di emissione. **Impatto su lavoro in corso:** vincola i campi del manifest in SPEC-001 (repliche, tetti di risorse, prezzo di abbonamento) — da comunicare all'implementatore o da verificare in review.
- ADR-001 — Ledger su federazione BFT con validatori a rotazione (accepted, 2026-08-25)
- ADR-002 — Proof of contribution tramite challenge crittografici (accepted, 2026-08-25)
- ADR-003 — Core del nodo in Rust con shell native (accepted, 2026-08-25)
- ADR-004 — Runtime delle app in sandbox WASM/WASI (accepted, 2026-08-25)
- ADR-005 — Economia del token a mint & burn (accepted, 2026-08-25)
