---
title: Project pulse
status: active
milestone: M-02
updated: 2026-08-25
---

# Project Pulse

## Current focus

**Sessione autonoma del Lead conclusa, notte del 2026-08-25.** Su mandato dell'operatore ("completare tutte le spec aperte, salvare i debiti, occuparti di dispatch e review"), il Lead ha operato senza attendere conferme.

In quella sessione M-01 arrivò a tre spec su quattro `done` ([SPEC-001], [SPEC-002], [SPEC-004]), con la quarta ([SPEC-003]) accettata tecnicamente e ferma sul gate dell'operatore. Sette review prodotte, sette dispatch, [ADR-007] deciso su delega con la metrica di [[PROJECT]] riformulata, quattro debiti nuovi registrati, tre commit spinti su `main`.

Il Lead segnala all'operatore due cose in primo piano: la decisione di [ADR-007] tocca una promessa di prodotto ed è superabile se non concorda; e un proprio errore di processo è documentato in `.lmbrain/knowledge/commit-discipline.md`.

**Ripresa del 2026-08-25, sessione successiva: la CI è verde e il repository è pubblico.** L'operatore ha sbloccato la fatturazione GitHub e reso pubblico il repository. La pipeline ha eseguito per la prima volta e ora passa su tutti e cinque i job; [DEBT-001] è chiuso. Il risultato che conta oltre il colore: **le tre rotture emerse non erano problemi di toolchain cross-platform**, cioè il rischio n.1 di [ADR-003], ma difetti di confezionamento del repository — due bit di esecuzione persi perché i file erano stati committati da Windows, e un ordine di step sbagliato nel workflow. Rust, NDK, cargo-ndk, UniFFI e Tauri hanno funzionato al primo tentativo utile su Windows e su Linux.

Il passaggio a pubblico ha portato con sé il proprio lavoro di sicurezza; vedi la sezione *Postura di sicurezza del repository pubblico*. È chiuso: pin a SHA con refresh via Dependabot, canale di disclosure privato, `LICENSE` Apache-2.0, harness esclusi da regola, e [DEBT-009] risolto.

**M-01 è completa.** [SPEC-003] è passata a `done` quando l'operatore ha attestato `GATE-OPERATOR-LOOK`: quattro spec su quattro chiuse. Il prossimo lavoro è M-02, che non ha ancora spec redatte e la cui priorità è dettata dai debiti — [DEBT-005], critico, viene prima di tutto.

## Handoff attivo

Nessuno. [HANDOFF-001] è stato **consumato e archiviato** il 2026-08-25: tutte le sue azioni raccomandate sono state eseguite o superate dagli eventi. Le sue affermazioni sono state verificate prima di agire, come chiedeva, e due sono risultate da correggere — [DEBT-001] non era più bloccato dalla fatturazione, e la questione della licenza non era una decisione da prendere ma una contraddizione da sciogliere.

## Current milestone

**M-02 — Ledger vivo: federazione BFT su devnet.** M-01 è chiusa dal 2026-08-25, tutte e quattro le spec `done`.

M-02 non ha ancora spec redatte: è il primo lavoro del Lead. La priorità non è libera, la dettano i debiti aperti — [DEBT-005] (regola di elezione dei validatori, critico) prima di ogni altra cosa, perché **nessuna devnet deve accumulare storia conservabile prima che quella regola sia scritta**, e una devnet è precisamente ciò che M-02 produce.

## Ready for handoff

Nessuna. Non ci sono spec in `ready`: quelle di M-02 vanno ancora redatte.

## In progress

Nessuna. Le prime due spec di M-02 sono chiuse.

**Il prossimo lavoro va redatto:** simulatore economico con `α` ([DEBT-007]), da cui dipendono i valori di ogni parametro che [SPEC-006] ha lasciato simbolico e il valore `X` di [ADR-007]; poi devnet BFT, light client con prove Merkle e mint & burn.

## Done

- [SPEC-006] Regola di elezione dei validatori → AGENT-002. **`done` il 2026-08-25**, accettata con [REVIEW-009] e con `GATE-SECREVIEW` attestato su [REVIEW-010]. **Chiude [DEBT-005], l'unico debito `critical` del progetto.** Quattro giri di review adversariale con AGENT-007, tredici finding fra cui tre `critical`. **Due dei finding erano arresti certi della catena introdotti dalle correzioni precedenti**, e nessuno dei due era visibile prima che la correzione precedente esistesse: la genesi con mandati sincronizzati, e i timbri di scadenza che collidono se e solo se il limite di mandato decresce — quest'ultimo innescabile da un operatore onesto che accorci i mandati, senza alcun avversario. Scoperto per strada che **il fixture `PD-0` del progetto era esso stesso inammissibile** (`T=3`, mentre la soddisfacibilità congiunta impone `T ≥ 4`). L'architettura portante — due strati che falliscono in modo diverso, con l'invariante anti-cattura confinato in quello che un light client verifica — non è stata toccata da nessuno dei tredici finding. AGENT-007 chiude dichiarando il claim difendibile **senza dichiarazioni accanto**.
- [SPEC-005] Applicazione di [ADR-009] al design system → AGENT-006 (Lia Wireframe). **`done` il 2026-08-25**, accettata con [REVIEW-008] senza alcun finding. Nove criteri su nove, con ogni gate rieseguita dal Lead in modo indipendente invece che presa dall'evidenza: zero residui del segnaposto, zero virgole come separatore delle migliaia, i tre generatori in `--check` confermano che gli artefatti non sono stati modificati a mano, 130 coppie di contrasto su 130 conformi a WCAG AA. L'implementatrice ha distinto il glifo del marchio da quello del segnaposto, simili a vista, che una sostituzione frettolosa avrebbe confuso. Una sua segnalazione sui titoli italiani delle pagine di mockup è stata valutata e **respinta nel merito**: sono la cornice di documentazione attorno agli artboard, non superficie di prodotto.
- [SPEC-003] Design system → AGENT-006 (Lia Wireframe). **`done` il 2026-08-25**, ultima spec di M-01. Accettata tecnicamente con [REVIEW-004] — sette criteri su sette verificati, un solo finding di severità bassa non bloccante — ed è rimasta ferma sul solo `GATE-OPERATOR-LOOK` finché l'operatore non ha attestato di aver visto i mockup. Il gate ha funzionato come doveva: il sistema ha rifiutato `spec_done` al tentativo del Lead, e nessuno ha attestato al posto dell'operatore un giudizio estetico che spettava a lui. Il pacchetto vive in `.lmbrain/design/coblox-design-system/`.
- [SPEC-001] Protocollo v0 → AGENT-001 (Dario Meshnet). **`done` il 2026-08-25**, dopo tre giri di remediation di sicurezza. `GATE-SECREVIEW` attestato: AGENT-007 l'ha bocciato due volte ([REVIEW-002] con 18 finding, [REVIEW-006] con 4 gravi residui) e superato alla terza ([REVIEW-007]). I documenti sono passati da 1268 a 2607 righe. **Due contestazioni di AGENT-001 sono state confermate dalla reviewer come migliori della sua stessa condizione di chiusura**: il pavimento Argon2id imposto come area più memoria minima invece di `iterations ≥ 3`, che avrebbe rifiutato il profilo RFC 9106 più forte; e lo scudo di ammissione adattivo con validazione della sorgente invece di un puzzle fisso, che avrebbe reintrodotto il divario CPU/GPU per cui [ADR-007] esiste. Residui in [DEBT-008].
- [SPEC-002] Workspace Rust + CI → AGENT-008 (Remo Pipeline). **`done` il 2026-08-25**, accettata con [REVIEW-005]. Tre difetti chiusi più sei problemi ulteriori scoperti eseguendo davvero il runbook, fra cui un flag Tauri inesistente che il Lead stesso aveva citato come prova senza eseguirlo. `GATE-LOCAL-REPRO` soddisfatto e in parte rieseguito dal Lead; `GATE-CI-GREEN` era derogato e coperto da [DEBT-001]. Commit `81cca93`. **Deroga rientrata il 2026-08-25:** la run 32821923135 sul commit `6b9ad1f` è verde su tutti e cinque i job, con `cargo fmt`, `clippy -D warnings` e `cargo-deny` eseguiti come step distinti e riusciti; [DEBT-001] è risolto. Ne era però emerso [DEBT-009] — quel `cargo-deny` non copriva `apps/desktop/src-tauri`, escluso dal workspace — **anch'esso risolto lo stesso giorno** con la run 32833295352, che aggiunge il gate sul grafo della shell desktop.
- [SPEC-004] Threat model iniziale → AGENT-007 (Greta Threatmodel). **`done` il 2026-08-25**, prima spec chiusa del progetto. Accettata con [REVIEW-003], `GATE-LEAD-MAP` attestato dal Lead. Ha prodotto `.lmbrain/knowledge/threat-model.md` (1930 righe, 36 scenari, 24 `SEC-REQ`, 15 test di attacco) e ha istruito [ADR-007]. Commit `024f81f` spinto su `main`.

## Blockers and risks

- **Il claim di sicurezza, nella forma che AGENT-007 giudica difendibile.** La rete è robusta contro la falsificazione ma **non** resistente ai Sybil per via crittografica, e tre cose non sono garantite: la disponibilità dell'enrollment sotto attacco sostenuto (i dispositivi lenti soffrono per primi), la resistenza Sybil crittografica, e la verifica indipendente dell'eleggibilità a validatore prima di M-02. Parole della reviewer: *"il progetto non deve chiamare la rete super-sicura senza quelle tre frasi accanto; con quelle accanto il claim è più solido della media a questo stadio, e la parte migliore non è nessun singolo meccanismo ma il fatto che i limiti siano quantificati."*
- **BLOCCO 1 — sciolto con [ADR-007].** La metrica "zero accrediti a nodi emulati" era irraggiungibile per via crittografica. Il Lead ha adottato su delega l'opzione 4a di [SPEC-004]: difesa economica (fondo a tetto per il reddito di esistenza, frazione `α` sorvegliata, eleggibilità a validatore ancorata a lavoro difficile da falsificare) più Argon2id come pavimento d'ingresso. La metrica in [[PROJECT]] è stata riformulata di conseguenza. **Confermata dall'operatore il 2026-08-25**, dopo revisione congiunta: [ADR-007] resta `accepted`. Nella stessa sessione l'operatore ha però chiesto di riaprire le esclusioni permanenti che avevano collassato lo spazio delle alternative prima ancora che [SPEC-004] cominciasse a enumerarle — ne è nata [ADR-008]. Resta aperto il valore `X` della metrica riformulata, che dipende dal simulatore di M-02 e da [DEBT-007].
- **Attenzione, decisione delegata di rilievo:** il progetto ora dichiara di essere robusto contro la falsificazione ma **non** resistente ai Sybil per via crittografica. È una rinuncia esplicita a una promessa, resa in cambio di onestà verificabile.
- **BLOCCO 2 — chiuso.** La prima run CI (commit `4ea0db9`) era fallita in 6 secondi **senza eseguire alcun job**, per la fatturazione dell'account GitHub e non per il codice; l'operatore aveva concesso la deroga su `GATE-CI-GREEN`, registrata come [DEBT-001]. Il 2026-08-25 la fatturazione è stata sbloccata, la pipeline ha eseguito e, dopo due giri di remediation, la run 32821923135 è verde su tutti i job. [DEBT-001] è **risolto**. I criteri di [SPEC-002] marcati `[~] ... | waived=DEBT-001` non sono più coperti da una deroga ma da una run reale.
- ~~Nome del token e font monospace~~ — **decisi dall'operatore il 2026-08-25**, [ADR-009]. L'unità è `credit`/`credits`, forma compatta `cr` posposta al numero; il glifo `◇` e la classe `.cbx-unit--provisional` sono ritirati. Font: JetBrains Mono. **Resta lavoro di applicazione** nel pacchetto di design, che è di AGENT-006 e non del Lead.

## Debiti aperti

| ID | Severità | Owner | Questione |
| --- | --- | --- | --- |
| [DEBT-010] | medium | AGENT-002 | La monotonicità del limite di mandato è un cricchetto spingibile da un avversario e non tirabile indietro da nessuno. Sorveglianza, non lavoro: il presidio è il tetto di genesi, che M-02 deve tarare stretto. |
| [DEBT-006] | high | AGENT-LEAD | La quota al creatore di [ADR-006] obbliga a pubblicare chi è abbonato a cosa. È l'unica superficie priva di un ADR alle spalle. |
| [DEBT-007] | high | AGENT-002 | La forma del reddito di esistenza non è decisa e determina `α`, il parametro più importante dell'economia. |
| [DEBT-008] | low | AGENT-001 | Due frasi della specifica del protocollo promettono poco più di quanto le regole impongano. Una riga ciascuna, M-02. |

**Nessun debito `critical` aperto.** [DEBT-005] era l'unico ed è chiuso.

Risolti, tutti il 2026-08-25: [DEBT-001] con la run CI verde 32821923135, primo debito chiuso del progetto; [DEBT-009] con la run 32833295352, che esegue `cargo-deny` anche sul grafo della shell desktop; e **[DEBT-005] con [SPEC-006]**, dopo quattro giri di review adversariale di sicurezza e tredici finding. Eseguire quel controllo ha rivelato che la stima del debito era per difetto — fallivano `advisories`, `bans` e `licenses` — e ha scoperto due errori nostri che non erano derogabili: `coblox-desktop` senza `license` né `publish`, e il campo `repository` del workspace che puntava a un repository inesistente.

## Next recommended actions

**Per l'operatore, al risveglio:**

1. ~~Rivedere [ADR-007]~~ — **fatto il 2026-08-25**: confermata, e dalla revisione è nata [ADR-008]. Resta da fissare `X`, ma dipende dal simulatore di M-02.
2. ~~Attestare `GATE-OPERATOR-LOOK`~~ — **fatto il 2026-08-25**: [SPEC-003] è `done` e **M-01 è chiusa**.
3. ~~Sbloccare la fatturazione GitHub~~ — **fatto il 2026-08-25**, [DEBT-001] chiuso.
4. ~~Decidere nome del token/unità e font monospace~~ — **fatto il 2026-08-25**, [ADR-009].

5. **Decidere sui file di configurazione degli harness** (`.codex/`, `.pi/`, `.mcp.json`, `opencode.json`): il Lead li ha esclusi dai commit perché contengono percorsi assoluti della macchina e il nome utente. ~~Vanno aggiunti al `.gitignore` oppure resi portabili.~~ **Fatto il 2026-08-25:** aggiunti al `.gitignore`, quindi esclusi da una regola e non più dalla disciplina manuale.
6. ~~Decidere sulla `LICENSE`~~ — **fatto il 2026-08-25**: `LICENSE` Apache-2.0 in radice, allineato a quanto i manifest già dichiaravano. Anche `SECURITY.md` è in piedi.
7. **Redigere le spec di M-02.** È il prossimo lavoro reale e non ne esiste ancora una. L'ordine è dettato dai debiti: la regola di elezione dei validatori ([DEBT-005], critico) viene prima di tutto, poi il simulatore economico che fissa `α` ([DEBT-007]) e da cui dipende il valore `X` di [ADR-007], poi un ADR sulla privacy ([DEBT-006]).

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
- **Pin a SHA di tutte le action di terze parti**, tredici occorrenze, con la versione leggibile in commento. Il caso peggiore era `dtolnay/rust-toolchain@1.96.0`, che non è un tag ma un **branch**, quindi ripuntabile con un commit qualsiasi. Completato da `.github/dependabot.yml`, che propone il refresh in un solo pull request settimanale in batch: un pin non invecchia in un modo che GitHub segnali come vulnerabile, smette solo di ricevere le correzioni in silenzio, quindi il refresh è la seconda metà della difesa e non un extra. Il ciclo ha già girato una volta: [PR #1](https://github.com/fathorMB/CobloxNetwork/pull/1), quattro action con salti di major, verificata verde e mergiata su richiesta dell'operatore. Ha eliminato anche i warning di deprecazione Node 20.
- **Private vulnerability reporting** abilitato, e `SECURITY.md` che lo documenta. Nessun indirizzo email esposto — è un bersaglio di spam e un punto singolo di rottura, mentre il canale di GitHub dà una discussione privata tracciata. Il documento dichiara per iscritto i limiti noti invece di lasciarli scoprire: rete non resistente ai Sybil per via crittografica ([ADR-007]), set di validatori auto-perpetuante in v0 ([DEBT-005]), e advisory derogati con la loro condizione di riesame.

*Non* disponibile: `secret_scanning_non_provider_patterns` richiede GitHub Advanced Security e resta disabilitato sul piano attuale. In pratica significa che vengono riconosciuti i formati di segreto dei provider noti, non quelli inventati dal progetto — rilevante se in futuro Coblox definisse un proprio formato di chiave.

- **`LICENSE` Apache-2.0**, su conferma dell'operatore. Non era una casella vuota ma una contraddizione: il `Cargo.toml` del workspace dichiarava `license = "Apache-2.0"` dal bootstrap, quindi il repository pubblicava crate che dichiaravano una licenza che nessun file concedeva. Il testo non è stato trascritto ma copiato da una copia canonica del registry Cargo locale, e il corpo verificato identico byte per byte contro una seconda copia indipendente — su un documento legale la fedeltà conta più della comodità. Il segnaposto `[yyyy] [name of copyright owner]` nell'`APPENDIX` è parte del template di applicazione ai singoli file e va lasciato com'è.

**Tutte le voci rilevate al passaggio a pubblico sono chiuse.** Restano solo due limiti dichiarati: `secret_scanning_non_provider_patterns` non disponibile sul piano attuale, e i percorsi di macchina nelle trascrizioni di [SPEC-002], severità bassa.

## Strategia di branching

Dichiarata dall'operatore il 2026-08-25 e registrata in `.lmbrain/BRANCHING.json` via `branching_strategy_set`: topologia **main-only**, nessun branch di feature. Gli specialisti lavorano sul working tree e **non fanno mai commit né push**; il Project Lead è l'unico autorizzato, e committa e pusha su `main` **al passaggio di una spec a `done`**. Vincolo aggiuntivo: **nessuna produzione di installer o release lato GitHub per ora** — la CI di [SPEC-002] è già conforme (Tauri con `--bundles none`, nessun job di release).

## Recent scope clarifications

- 2026-08-25 — **Lingua del prodotto: inglese** per tutto ciò che vede l'utente finale (rilievo dell'operatore sulle anteprime di SPEC-003). Registrata come vincolo in [[PROJECT]]. SPEC-003 corretta in corso d'opera e AGENT-006 avvisata mentre era ancora in lavorazione; la formulazione originale della spec ("tono del copy (it/en)") era ambigua per responsabilità del Lead, non dell'implementatrice. Da applicare d'ora in poi a ogni spec con superficie utente (SPEC-002 espone solo una schermata di versione, impatto trascurabile ma da verificare in review).

## Recent profile changes

- 2026-08-25 — AGENT-007: `can_implement` portato a true con vincolo "solo deliverable documentali di sicurezza, mai codice" (approvato dall'operatore per SPEC-004); sui propri documenti la review spetta al Lead.

## Recent decisions

- ADR-009 — L'unità del token si chiama credit e si scrive come una misura, non come una valuta (accepted, 2026-08-25). Decisa dall'operatore. Il ragionamento che la tiene insieme: il vincolo di non convertibilità spinge verso un nome **poco brandizzabile**, non solo non-monetario, perché la speculazione ha bisogno di un brand su cui aggrapparsi; e la posizione dell'unità porta significato, perché `1 240 cr` è la grammatica della misura mentre `◇1 240` è quella del denaro. Ha corretto un dato di [SPEC-003]: JetBrains Mono è sotto SIL OFL 1.1, non Apache-2.0 — quest'ultima copre il codice sorgente del repository, non il carattere — e poiché anche gli altri due candidati sono OFL 1.1, l'argomento della licenza non li distingueva. **Impatto su lavoro futuro:** aggiornamento del pacchetto di design e di `PRINCIPLES.md`, lavoro di AGENT-006; e con OFL 1.1 la licenza del font andrà inclusa accanto all'Apache-2.0 quando il font sarà incorporato nel bundle Tauri.
- ADR-008 — Il divieto di proof-of-work continuo colpisce il lavoro sprecato, non il lavoro campionato (accepted, 2026-08-25). Nata dalla revisione di [ADR-007] con l'operatore. Non abroga l'esclusione: la sostituisce con un principio più un test in tre punti. **Ha sanato una contraddizione che nessuno aveva messo alla prova:** `PROJECT.md` escludeva il proof-of-work continuo «di qualsiasi tipo» mentre [ADR-002] prescrive proof-of-retrievability continuo e ri-esecuzione WASM, quindi il protocollo violava già una propria esclusione dichiarata. **Impatto su lavoro futuro:** ogni ADR o spec che introduca lavoro remunerato deve dichiarare l'esito dei tre punti del test, ed è materia di review verificarlo; il punto 1 vincola la specifica di elezione dei validatori di M-02, già gravata da [DEBT-005].
- ADR-006 — Pubblicazione delle app e ricompensa al creatore (accepted, 2026-08-25). Estende ADR-005 con una nuova categoria di emissione. **Impatto su lavoro in corso:** vincola i campi del manifest in SPEC-001 (repliche, tetti di risorse, prezzo di abbonamento) — da comunicare all'implementatore o da verificare in review.
- ADR-001 — Ledger su federazione BFT con validatori a rotazione (accepted, 2026-08-25)
- ADR-002 — Proof of contribution tramite challenge crittografici (accepted, 2026-08-25)
- ADR-003 — Core del nodo in Rust con shell native (accepted, 2026-08-25)
- ADR-004 — Runtime delle app in sandbox WASM/WASI (accepted, 2026-08-25)
- ADR-005 — Economia del token a mint & burn (accepted, 2026-08-25)
