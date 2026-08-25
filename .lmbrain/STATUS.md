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

**M-01 è completa.** [SPEC-003] è passata a `done` quando l'operatore ha attestato `GATE-OPERATOR-LOOK`: quattro spec su quattro chiuse. Il prossimo lavoro è M-02, che non ha ancora spec redatte e la cui priorità è dettata dai debiti — [DEBT-005], critico, veniva prima di tutto, ed è chiuso da [SPEC-006].

## Passata di chiusura delle decisioni di prodotto — 2026-08-25

L'operatore ha chiesto di risolvere in una sola passata tutti i punti aperti che dipendevano da lui. **Cinque decisioni**, le prime quattro censite dal Lead e la quinta emersa preparando la terza.

| # | Decisione | Esito | Registrata in |
| --- | --- | --- | --- |
| 1 | Intervallo di blocco | **5 s**, costante di genesi dichiarata e non parametro governato | [ADR-013] |
| 2 | Popolazione attesa al lancio | **~200 nodi** → `F` di genesi = **300 000 000 µt** contro un tetto di 15 882 352 941 | annotazione su [ADR-011] |
| 3 | Privacy degli abbonati | **Accettare e dichiarare**; prova aggregata come ricerca, mai come promessa | [ADR-014], chiude [DEBT-006] |
| 4 | Identità di trasporto | **Separare le chiavi in v0**, subordinate e ruotabili | [ADR-015] |
| 5 | [DEBT-010] | **Differito a M-07**, con la dimostrazione come criterio di una spec di M-02 | evento sul debito |

**Due cose che il pulse dava per aperte e non lo erano**, corrette in questa passata: il valore `X` di [ADR-007] è fissato a 20% da [SPEC-007], e la forma del fondo è decisa. Erano affermazioni rimaste indietro rispetto ai fatti, cioè la famiglia 2 applicata a questo stesso documento.

**Un errore del Lead corretto prima che entrasse in un artefatto.** L'opzione offerta all'operatore sulla decisione 4 diceva che la separazione delle chiavi *«chiude TM-28 alla radice»*. È falso: il legame `node_id → IP` non passa dalla chiave di trasporto ma dal certificato che `identity.md` impone di presentare prima di pubblicare gossip, e da `sender_node_id` in chiaro nell'envelope. L'avversario di TM-28 tiene sessioni Coblox, quindi il certificato lo riceve comunque. La decisione non cambia — la separazione resta necessaria e la sua finestra si chiude davvero all'enrollment — ma [ADR-015] è scritta per ciò che fa davvero: sposta il costo dell'attacco da **lettura gratuita e retroattiva del ledger** a **partecipazione attiva e contemporanea**, e dichiara il residuo invece di tacerlo.

**Conseguenza sul piano di M-02.** Due delle cinque decisioni cambiano artefatti pubblicati — il fondo di genesi cambia il `policy_hash`, la separazione delle chiavi cambia lo schema del certificato — quindi la gate di [ADR-012] si applica, e quella gate **oggi non è eseguibile** perché l'inventario su cui dovrebbe girare non esiste. L'inventario degli artefatti pubblicati passa da lavoro di smaltimento a **prerequisito**.

## Handoff attivo

Nessuno. [HANDOFF-001] è stato **consumato e archiviato** il 2026-08-25: tutte le sue azioni raccomandate sono state eseguite o superate dagli eventi. Le sue affermazioni sono state verificate prima di agire, come chiedeva, e due sono risultate da correggere — [DEBT-001] non era più bloccato dalla fatturazione, e la questione della licenza non era una decisione da prendere ma una contraddizione da sciogliere.

## Current milestone

**M-02 — Ledger vivo: federazione BFT su devnet.** M-01 è chiusa dal 2026-08-25, tutte e quattro le spec `done`.

M-02 non ha ancora spec redatte: è il primo lavoro del Lead. La priorità non è libera, la dettano i debiti aperti — [DEBT-005] (regola di elezione dei validatori, critico) prima di ogni altra cosa, perché **nessuna devnet deve accumulare storia conservabile prima che quella regola sia scritta**, e una devnet è precisamente ciò che M-02 produce.

## Ready for handoff

Nessuna in `ready`. Quattro in `backlog` in attesa della tua approvazione: [SPEC-010], [SPEC-011], [SPEC-012], [SPEC-013].

## In progress

Nessuna. Le prime due spec di M-02 sono chiuse.

**Nessuna spec in lavorazione.** Le nove precedenti sono tutte `done`. Le quattro nuove — [SPEC-010]…[SPEC-013] — sono in `backlog` e aspettano la tua approvazione; vedi *Lavoro immediato di M-02*.

## Done

- [SPEC-010] Inventario degli artefatti pubblicati, codifica del `lifecycle`, precisione normativa → AGENT-001. **`done` il 2026-08-25**, accettata con [REVIEW-015] **senza finding a carico dell'implementazione**. Chiude [DEBT-012] e [DEBT-008]. Rende eseguibile la gate di [ADR-012] con uno strumento che **ri-deriva meccanicamente i candidati dai documenti** e fallisce in entrambe le direzioni su dieci classi di difetto, ciascuna **osservata fallire** su un albero mutato. **Il risultato che conta più della spec:** alla prima esecuzione lo strumento ha trovato una **quinta occorrenza della famiglia 1** — l'esempio canonico di `challenge_evidence` portava un `request_hash` diverso dal proprio `challenge_id`, contro una regola che esisteva da [SPEC-001]. Le prime quattro le aveva trovate il caso; questa una guardia. **Tre errori del Lead trovati dall'implementatore**, fra cui un conteggio sbagliato — «dieci valori di hash» quando sono sedici — **nella spec scritta per prevenire quella classe di errore**, con il conteggio corretto già presente nel file che il paragrafo citava. Su `lifecycle_u8` la scelta è migliore di quella attesa: `0x00` riservato e invalido, `active = 0x01`, perché il byte zero è ciò che un record troncato produce gratis e se significasse `active` l'incidente produrrebbe lo stato **permissivo**.
- [SPEC-009] Attuazione di [ADR-010] e [ADR-011] → AGENT-002. **`done` il 2026-08-25**, accettata con [REVIEW-013] e con `GATE-SECREVIEW` attestato su [REVIEW-014], dopo due giri di review e dieci finding di cui **tre `critical`**. Porta `RewardBounds` nella genesi, tre regole di validità nuove e il claim in due regimi. **La scoperta principale ribalta un invariante appena scritto:** il vincolo `3·min_set ≥ 2V` impedisce a una coalizione sotto i due terzi di *possedere* il set, non di ottenerne il **quorum** — a `V=27` bastano 13 seggi, il 48,1%, e la soglia reale ha asintoto `4V/9`. La sezione nuova di `ledger.md` si chiama *«Owning the set and controlling it are different thresholds»*. La diagnosi che tiene tutti i finding, di AGENT-007: **è stata vincolata la grandezza nominata dall'ADR, non quella da cui la proprietà dipende**.
- [SPEC-007] Simulatore economico e taratura di `α` → AGENT-002. **`done` il 2026-08-25**, accettata con [REVIEW-011] dopo un giro di review adversariale con sette finding, uno critico. Chiude [DEBT-007]. **Il risultato principale è un risultato negativo, riportato onestamente:** `α` non è una curva con un ottimo, è un'**identità** — la cattura vale `α·N/(N+H)` e il reddito di un nodo di sola availability rapportato al reddito medio vale `α` esattamente. Difendibilità e significato del reddito sono lo stesso numero letto due volte, la cattura è lineare senza ginocchio, e **il modello non poteva scegliere `α`**. Valori: `α = 0,15` banda `[0,10–0,20]`, `X = 20%`, tutti e ventidue i parametri fissati. Ha prodotto tre scoperte che il debito non chiedeva: una contestazione di [ADR-007], [ADR-010], e la disposizione sul dimensionamento del fondo di genesi.
- [SPEC-008] Core del ledger in Rust → AGENT-001. **`done` il 2026-08-25**, accettata con [REVIEW-012] senza finding a carico dell'implementazione. **Primo codice reale del progetto**: dodici moduli, 8.097 righe, 103 test, sedici fixture su sedici riprodotte al primo tentativo. Ha trovato [DEBT-012] e due errori di scrittura del Lead nella spec stessa.
- [SPEC-006] Regola di elezione dei validatori → AGENT-002. **`done` il 2026-08-25**, accettata con [REVIEW-009] e con `GATE-SECREVIEW` attestato su [REVIEW-010]. **Chiude [DEBT-005], l'unico debito `critical` del progetto.** Quattro giri di review adversariale con AGENT-007, tredici finding fra cui tre `critical`. **Due dei finding erano arresti certi della catena introdotti dalle correzioni precedenti**, e nessuno dei due era visibile prima che la correzione precedente esistesse: la genesi con mandati sincronizzati, e i timbri di scadenza che collidono se e solo se il limite di mandato decresce — quest'ultimo innescabile da un operatore onesto che accorci i mandati, senza alcun avversario. Scoperto per strada che **il fixture `PD-0` del progetto era esso stesso inammissibile** (`T=3`, mentre la soddisfacibilità congiunta impone `T ≥ 4`). L'architettura portante — due strati che falliscono in modo diverso, con l'invariante anti-cattura confinato in quello che un light client verifica — non è stata toccata da nessuno dei tredici finding. AGENT-007 chiude dichiarando il claim difendibile **senza dichiarazioni accanto**.
- [SPEC-005] Applicazione di [ADR-009] al design system → AGENT-006 (Lia Wireframe). **`done` il 2026-08-25**, accettata con [REVIEW-008] senza alcun finding. Nove criteri su nove, con ogni gate rieseguita dal Lead in modo indipendente invece che presa dall'evidenza: zero residui del segnaposto, zero virgole come separatore delle migliaia, i tre generatori in `--check` confermano che gli artefatti non sono stati modificati a mano, 130 coppie di contrasto su 130 conformi a WCAG AA. L'implementatrice ha distinto il glifo del marchio da quello del segnaposto, simili a vista, che una sostituzione frettolosa avrebbe confuso. Una sua segnalazione sui titoli italiani delle pagine di mockup è stata valutata e **respinta nel merito**: sono la cornice di documentazione attorno agli artboard, non superficie di prodotto.
- [SPEC-003] Design system → AGENT-006 (Lia Wireframe). **`done` il 2026-08-25**, ultima spec di M-01. Accettata tecnicamente con [REVIEW-004] — sette criteri su sette verificati, un solo finding di severità bassa non bloccante — ed è rimasta ferma sul solo `GATE-OPERATOR-LOOK` finché l'operatore non ha attestato di aver visto i mockup. Il gate ha funzionato come doveva: il sistema ha rifiutato `spec_done` al tentativo del Lead, e nessuno ha attestato al posto dell'operatore un giudizio estetico che spettava a lui. Il pacchetto vive in `.lmbrain/design/coblox-design-system/`.
- [SPEC-001] Protocollo v0 → AGENT-001 (Dario Meshnet). **`done` il 2026-08-25**, dopo tre giri di remediation di sicurezza. `GATE-SECREVIEW` attestato: AGENT-007 l'ha bocciato due volte ([REVIEW-002] con 18 finding, [REVIEW-006] con 4 gravi residui) e superato alla terza ([REVIEW-007]). I documenti sono passati da 1268 a 2607 righe. **Due contestazioni di AGENT-001 sono state confermate dalla reviewer come migliori della sua stessa condizione di chiusura**: il pavimento Argon2id imposto come area più memoria minima invece di `iterations ≥ 3`, che avrebbe rifiutato il profilo RFC 9106 più forte; e lo scudo di ammissione adattivo con validazione della sorgente invece di un puzzle fisso, che avrebbe reintrodotto il divario CPU/GPU per cui [ADR-007] esiste. Residui in [DEBT-008].
- [SPEC-002] Workspace Rust + CI → AGENT-008 (Remo Pipeline). **`done` il 2026-08-25**, accettata con [REVIEW-005]. Tre difetti chiusi più sei problemi ulteriori scoperti eseguendo davvero il runbook, fra cui un flag Tauri inesistente che il Lead stesso aveva citato come prova senza eseguirlo. `GATE-LOCAL-REPRO` soddisfatto e in parte rieseguito dal Lead; `GATE-CI-GREEN` era derogato e coperto da [DEBT-001]. Commit `81cca93`. **Deroga rientrata il 2026-08-25:** la run 32821923135 sul commit `6b9ad1f` è verde su tutti e cinque i job, con `cargo fmt`, `clippy -D warnings` e `cargo-deny` eseguiti come step distinti e riusciti; [DEBT-001] è risolto. Ne era però emerso [DEBT-009] — quel `cargo-deny` non copriva `apps/desktop/src-tauri`, escluso dal workspace — **anch'esso risolto lo stesso giorno** con la run 32833295352, che aggiunge il gate sul grafo della shell desktop.
- [SPEC-004] Threat model iniziale → AGENT-007 (Greta Threatmodel). **`done` il 2026-08-25**, prima spec chiusa del progetto. Accettata con [REVIEW-003], `GATE-LEAD-MAP` attestato dal Lead. Ha prodotto `.lmbrain/knowledge/threat-model.md` (1930 righe, 36 scenari, 24 `SEC-REQ`, 15 test di attacco) e ha istruito [ADR-007]. Commit `024f81f` spinto su `main`.

## Blockers and risks

- **Il claim di sicurezza, nella forma che AGENT-007 giudica difendibile.** La rete è robusta contro la falsificazione ma **non** resistente ai Sybil per via crittografica, e tre cose non sono garantite: la disponibilità dell'enrollment sotto attacco sostenuto (i dispositivi lenti soffrono per primi), la resistenza Sybil crittografica, e la verifica indipendente dell'eleggibilità a validatore prima di M-02. Parole della reviewer: *"il progetto non deve chiamare la rete super-sicura senza quelle tre frasi accanto; con quelle accanto il claim è più solido della media a questo stadio, e la parte migliore non è nessun singolo meccanismo ma il fatto che i limiti siano quantificati."*
- **BLOCCO 1 — sciolto con [ADR-007].** La metrica "zero accrediti a nodi emulati" era irraggiungibile per via crittografica. Il Lead ha adottato su delega l'opzione 4a di [SPEC-004]: difesa economica (fondo a tetto per il reddito di esistenza, frazione `α` sorvegliata, eleggibilità a validatore ancorata a lavoro difficile da falsificare) più Argon2id come pavimento d'ingresso. La metrica in [[PROJECT]] è stata riformulata di conseguenza. **Confermata dall'operatore il 2026-08-25**, dopo revisione congiunta: [ADR-007] resta `accepted`. Nella stessa sessione l'operatore ha però chiesto di riaprire le esclusioni permanenti che avevano collassato lo spazio delle alternative prima ancora che [SPEC-004] cominciasse a enumerarle — ne è nata [ADR-008]. ~~Resta aperto il valore `X`~~ — **chiuso il 2026-08-25 da [SPEC-007]**: `X = 20%`, pari al bordo superiore della banda di sorveglianza su `α`. Questa riga dava per aperto un punto chiuso ed è stata corretta nella passata di decisioni.
- **Attenzione, decisione delegata di rilievo:** il progetto ora dichiara di essere robusto contro la falsificazione ma **non** resistente ai Sybil per via crittografica. È una rinuncia esplicita a una promessa, resa in cambio di onestà verificabile.
- **BLOCCO 2 — chiuso.** La prima run CI (commit `4ea0db9`) era fallita in 6 secondi **senza eseguire alcun job**, per la fatturazione dell'account GitHub e non per il codice; l'operatore aveva concesso la deroga su `GATE-CI-GREEN`, registrata come [DEBT-001]. Il 2026-08-25 la fatturazione è stata sbloccata, la pipeline ha eseguito e, dopo due giri di remediation, la run 32821923135 è verde su tutti i job. [DEBT-001] è **risolto**. I criteri di [SPEC-002] marcati `[~] ... | waived=DEBT-001` non sono più coperti da una deroga ma da una run reale.
- ~~Nome del token e font monospace~~ — **decisi dall'operatore il 2026-08-25**, [ADR-009]. L'unità è `credit`/`credits`, forma compatta `cr` posposta al numero; il glifo `◇` e la classe `.cbx-unit--provisional` sono ritirati. Font: JetBrains Mono. **Resta lavoro di applicazione** nel pacchetto di design, che è di AGENT-006 e non del Lead.

## Debiti aperti

| ID | Severità | Owner | Questione |
| --- | --- | --- | --- |
| [DEBT-013] | medium | AGENT-007 | Nessuna regola impone il passo di produzione dei blocchi: il set attivo decide la durata reale delle proprie epoche, quindi la propria incumbency. Aperto con [ADR-013]. |
| [DEBT-014] | medium | AGENT-007 | `validator_set_hash` è **l'unica preimmagine a dominio separato non legata a `chain_id`**: un set identico su due catene produce lo stesso hash. Trovato da AGENT-001 costruendo l'inventario di [SPEC-010]. |

**Nessun debito `critical` aperto**, e **nessun `high`**: [DEBT-012] e [DEBT-008] sono chiusi da [SPEC-010]. Entrambi i debiti aperti hanno owner AGENT-007 e la stessa forma — un'osservazione che chi l'ha fatta non deve valutare da sé.

**Differito:** [DEBT-010] a M-07, il 2026-08-25. Non chiuso come rischio accettato benché i numeri lo suggeriscano — con il blocco a 5 s una spinta irreversibile porta l'incumbency massima da 63 a 84 giorni e il pavimento di ricambio non si muove, perché `ceil(27/9)` e `ceil(27/12)` valgono entrambi 3. **Ma è aritmetica del Lead, non la dimostrazione che il debito pone come condizione**, e accettare un rischio su un'affermazione non dimostrata è la famiglia 2 di `recurring-defects.md`. La dimostrazione è ora un criterio della spec di M-02 che tocca i parametri di consenso.

Risolti, tutti il 2026-08-25 — **cinque su dieci**: [DEBT-001] con la run CI verde 32821923135, primo debito chiuso del progetto; [DEBT-009] con la run 32833295352, che esegue `cargo-deny` anche sul grafo della shell desktop; **[DEBT-005] con [SPEC-006]**, dopo quattro giri di review adversariale e tredici finding; e **[DEBT-007] con [SPEC-007]**. Eseguire quel controllo ha rivelato che la stima del debito era per difetto — fallivano `advisories`, `bans` e `licenses` — e ha scoperto due errori nostri che non erano derogabili: `coblox-desktop` senza `license` né `publish`, e il campo `repository` del workspace che puntava a un repository inesistente.

## Lavoro immediato di M-02

**Il divario che il pulse non diceva, verificato il 2026-08-25.** [SPEC-009] ha cambiato le regole **nei documenti e non nel codice**: il commit `eadba2d` tocca `core/coblox-core/tests/` e non tocca `core/coblox-core/src/` in nessuna riga. Tre conseguenze verificate una per una:

- `RewardPolicyConstraints::from_body` non legge affatto `availability_microtokens_per_unit`, quindi il crate **accetta la tariffa positiva** che `ledger.md` dichiara rifiutata in accettazione;
- `check_relations` contiene `3c < V` e `3cm ≤ V` ma **non** `3·min_set ≥ 2V`, cioè il vincolo su cui poggia l'intera sezione *«Owning the set and controlling it are different thresholds»*;
- **`RewardBounds` non esiste come tipo**: c'è `ElectionBounds`, non il suo gemello.

Quattro fixture di frontiera pubblicate in `docs/protocol/README.md` dichiarano `invalid` casi che **oggi nessuna implementazione rifiuta**.

### Tre spec in `ready`, dispacciabili

[SPEC-010] è `done`; le tre che seguono erano bloccate dalla sua dipendenza e si sono sbloccate alla sua chiusura. Tutte ad AGENT-001, `sol`.

1. **[SPEC-011] — `RewardBounds` e le regole di validità economiche in `coblox-core`.** Chiude il divario fra documenti e codice lasciato da [SPEC-009]. Criterio portante: **ogni caso dichiarato `invalid` dev'essere rifiutato**, perché una suite di soli casi validi la passa anche un validatore che accetta tutto. Con una gate dedicata alla **direzione** del limite, che [REVIEW-014] indica come il punto in cui l'errore sarebbe facile e invisibile.
2. **[SPEC-012] — Verificatore Ed25519 con i vettori speccheck come oracolo.** Isolata, dispatchabile in parallelo, **prima di qualunque devnet**. Il suo rischio dichiarato è anche il suo possibile risultato migliore: **la tabella pubblicata dei dodici esiti non è mai stata eseguita da nessuno**, e se un esito diverge non è un fallimento della spec ma la ragione per cui esiste.
3. **[SPEC-013] — Separazione della chiave di trasporto dalla chiave di identità.** Attua [ADR-015]. Scadenza dura: **prima che la devnet emetta il primo certificato**. Il primo passo del piano è tracciare cosa usava `libp2p_peer_id`, perché il Lead non l'ha fatto e lo dichiara.

Poi devnet BFT, light client con prove Merkle e mint & burn, che dipendono dalla forma delle API fissate da [SPEC-008]. Costruirli prima significherebbe poggiare il consenso su uno strato di firme che nessuno ha verificato e su un validatore di parametri che accetta genesi inammissibili.

## Next recommended actions

**Per l'operatore, al risveglio:**

1. ~~Rivedere [ADR-007]~~ — **fatto il 2026-08-25**: confermata, e dalla revisione è nata [ADR-008]. Resta da fissare `X`, ma dipende dal simulatore di M-02.
2. ~~Attestare `GATE-OPERATOR-LOOK`~~ — **fatto il 2026-08-25**: [SPEC-003] è `done` e **M-01 è chiusa**.
3. ~~Sbloccare la fatturazione GitHub~~ — **fatto il 2026-08-25**, [DEBT-001] chiuso.
4. ~~Decidere nome del token/unità e font monospace~~ — **fatto il 2026-08-25**, [ADR-009].

5. **Decidere sui file di configurazione degli harness** (`.codex/`, `.pi/`, `.mcp.json`, `opencode.json`): il Lead li ha esclusi dai commit perché contengono percorsi assoluti della macchina e il nome utente. ~~Vanno aggiunti al `.gitignore` oppure resi portabili.~~ **Fatto il 2026-08-25:** aggiunti al `.gitignore`, quindi esclusi da una regola e non più dalla disciplina manuale.
6. ~~Decidere sulla `LICENSE`~~ — **fatto il 2026-08-25**: `LICENSE` Apache-2.0 in radice, allineato a quanto i manifest già dichiaravano. Anche `SECURITY.md` è in piedi.
7. ~~Redigere le spec di M-02~~ — **in corso**: cinque redatte e chiuse ([SPEC-005]…[SPEC-009]), quattro da redigere nell'ordine rivisto in *Lavoro immediato di M-02*.
8. ~~Prendere le decisioni di prodotto aperte~~ — **fatto il 2026-08-25**, tutte e cinque; vedi *Passata di chiusura delle decisioni di prodotto*.
9. **Leggere [ADR-015] e accettarla o rimandarla.** È l'unica ADR lasciata deliberatamente in `proposed`: supera una regola già accettata, e il Lead si era impegnato a portarla in lettura prima dell'accettazione.

**Per il Lead, in autonomia:**

- A ogni spec che passa a `done`: commit e push su `main`.
- Prima di ogni `spec_done`: il controllo sullo stato delle review, che `spec_done` non fa e `lmbrain_validate` non segnala — vedi `.lmbrain/knowledge/review-lifecycle-discipline.md`.

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

## Difetti ricorrenti

`.lmbrain/knowledge/recurring-defects.md` raccoglie le **tre famiglie** che si sono ripetute, con le occorrenze contate e le domande che le intercettano. Il tratto comune: **in ogni caso il difetto era gia scritto da qualche parte nel repository e nessuno lo stava guardando** — non errori di ragionamento, errori di dove si guardava. La prima famiglia e chiusa da [ADR-012] con una gate; le altre due no, e vivono nelle domande da porsi.

## Igiene del brain

- 2026-08-25 — **Tre review erano ferme in `changes-requested` su una spec già `done`** ([SPEC-001]): rilievo dell'operatore dalla board. Nessuna era sbagliata nel merito; mancava solo il verdetto finale, perché a ogni giro di remediation veniva creata una review nuova invece di ri-esprimere il verdetto su quella esistente. Disposizione caso per caso e non in blocco: [REVIEW-001] **accettata**, perché i suoi tre finding risultano chiusi e verificati; [REVIEW-002] e [REVIEW-006] **superate**, perché rimpiazzate da review successive sullo stesso gate senza mai arrivare all'accettazione — registrarle come accettate avrebbe cancellato proprio l'informazione che rende leggibile la catena. **Il difetto era invisibile agli strumenti**: `spec_done` verifica i gate e non lo stato delle review, e `lmbrain_validate` non lo segnala. La regola e il controllo da fare prima di `spec_done` sono in `.lmbrain/knowledge/review-lifecycle-discipline.md`.

## Strategia di branching

Dichiarata dall'operatore il 2026-08-25 e registrata in `.lmbrain/BRANCHING.json` via `branching_strategy_set`: topologia **main-only**, nessun branch di feature. Gli specialisti lavorano sul working tree e **non fanno mai commit né push**; il Project Lead è l'unico autorizzato, e committa e pusha su `main` **al passaggio di una spec a `done`**. Vincolo aggiuntivo: **nessuna produzione di installer o release lato GitHub per ora** — la CI di [SPEC-002] è già conforme (Tauri con `--bundles none`, nessun job di release).

## Recent scope clarifications

- 2026-08-25 — **Lingua del prodotto: inglese** per tutto ciò che vede l'utente finale (rilievo dell'operatore sulle anteprime di SPEC-003). Registrata come vincolo in [[PROJECT]]. SPEC-003 corretta in corso d'opera e AGENT-006 avvisata mentre era ancora in lavorazione; la formulazione originale della spec ("tono del copy (it/en)") era ambigua per responsabilità del Lead, non dell'implementatrice. Da applicare d'ora in poi a ogni spec con superficie utente (SPEC-002 espone solo una schermata di versione, impatto trascurabile ma da verificare in review).

## Recent profile changes

- 2026-08-25 — AGENT-007: `can_implement` portato a true con vincolo "solo deliverable documentali di sicurezza, mai codice" (approvato dall'operatore per SPEC-004); sui propri documenti la review spetta al Lead.

## Recent decisions

- ADR-015 — L'identità di trasporto è subordinata e ruotabile, non è la chiave di identità (**proposed**, 2026-08-25). Decisa dall'operatore, in attesa della sua lettura prima dell'accettazione perché **supera una regola già accettata**: `identity.md` §*Key hierarchy* impone oggi la chiave unica, e quella frase diventa falsa nel momento in cui la ADR è attuata. Affronta TM-28, severità alta e finora senza alcun ADR né debito alle spalle. **Impatto su lavoro futuro:** tocca `identity.md`, `wire.md`, lo schema di richiesta e certificato di enrollment e le preimmagini che ne discendono; deve atterrare **prima che la devnet emetta il primo certificato**, dopo di che non è più una decisione ma una migrazione. La conseguenza che il Lead giudica più probabile fonte di difetto, e la segnala come tale a chi farà la review: una chiave di trasporto ruotabile azzera lo stato per peer, quindi tocca code e backpressure di `wire.md` e va confrontata con lo scudo di ammissione di [ADR-007].
- ADR-014 — Gli abbonamenti sono pubblici e correlabili, e il progetto lo dichiara prima che lo siano (accepted, 2026-08-25). Decisa dall'operatore, coincide con la raccomandazione di AGENT-007 per v0. Chiude [DEBT-006]. Il ragionamento che la tiene insieme non è la proporzionalità ma **dove sta davvero la fuga**: non nel conteggio degli abbonati ma nel **burn**, che nomina `payer_node_id` perché è la firma del pagatore ad autorizzare l'addebito — quindi togliere la quota al creatore toglie la ragione di contare e **lascia la lista intatta**. La grandezza da cui la proprietà dipende è l'invariante *un pagatore, un voto*. **Impatto su lavoro futuro:** il testo pubblico va scritto **una volta sola** e citato, ed è deliverable di una spec con `GATE-SECREVIEW`, con scadenza al primo partecipante esterno e non al lancio.
- ADR-013 — L'intervallo di blocco è una costante di genesi dichiarata, non un parametro governato (accepted, 2026-08-25). Decisa dall'operatore. **Ha reso normativo un numero che esisteva in un solo punto del repository come assunzione** (`sim/coblox_sim/recommended.py:21`), e su cui poggiava la taratura dei ventidue parametri di [SPEC-007]: `election_epoch_blocks = 120 960` significa «7 giorni» solo se un blocco dura 5 secondi. È la domanda di [REVIEW-014] — *qual è il denominatore* — applicata alla metà che quella review non guardava: l'emissione è denominata in millisecondi e il suo denominatore fu chiuso da [SPEC-009], l'elezione è denominata in blocchi e il suo no. **Impatto su lavoro futuro:** contenuto normativo nuovo nei documenti di protocollo, quindi la gate di [ADR-012] si applica; e la parte 3 della decisione — v0 dichiara la cadenza e non la impone — ha aperto [DEBT-013].
- ADR-009 — L'unità del token si chiama credit e si scrive come una misura, non come una valuta (accepted, 2026-08-25). Decisa dall'operatore. Il ragionamento che la tiene insieme: il vincolo di non convertibilità spinge verso un nome **poco brandizzabile**, non solo non-monetario, perché la speculazione ha bisogno di un brand su cui aggrapparsi; e la posizione dell'unità porta significato, perché `1 240 cr` è la grammatica della misura mentre `◇1 240` è quella del denaro. Ha corretto un dato di [SPEC-003]: JetBrains Mono è sotto SIL OFL 1.1, non Apache-2.0 — quest'ultima copre il codice sorgente del repository, non il carattere — e poiché anche gli altri due candidati sono OFL 1.1, l'argomento della licenza non li distingueva. **Impatto su lavoro futuro:** aggiornamento del pacchetto di design e di `PRINCIPLES.md`, lavoro di AGENT-006; e con OFL 1.1 la licenza del font andrà inclusa accanto all'Apache-2.0 quando il font sarà incorporato nel bundle Tauri.
- ADR-008 — Il divieto di proof-of-work continuo colpisce il lavoro sprecato, non il lavoro campionato (accepted, 2026-08-25). Nata dalla revisione di [ADR-007] con l'operatore. Non abroga l'esclusione: la sostituisce con un principio più un test in tre punti. **Ha sanato una contraddizione che nessuno aveva messo alla prova:** `PROJECT.md` escludeva il proof-of-work continuo «di qualsiasi tipo» mentre [ADR-002] prescrive proof-of-retrievability continuo e ri-esecuzione WASM, quindi il protocollo violava già una propria esclusione dichiarata. **Impatto su lavoro futuro:** ogni ADR o spec che introduca lavoro remunerato deve dichiarare l'esito dei tre punti del test, ed è materia di review verificarlo; il punto 1 vincola la specifica di elezione dei validatori di M-02, già gravata da [DEBT-005].
- ADR-006 — Pubblicazione delle app e ricompensa al creatore (accepted, 2026-08-25). Estende ADR-005 con una nuova categoria di emissione. **Impatto su lavoro in corso:** vincola i campi del manifest in SPEC-001 (repliche, tetti di risorse, prezzo di abbonamento) — da comunicare all'implementatore o da verificare in review.
- ADR-001 — Ledger su federazione BFT con validatori a rotazione (accepted, 2026-08-25)
- ADR-002 — Proof of contribution tramite challenge crittografici (accepted, 2026-08-25)
- ADR-003 — Core del nodo in Rust con shell native (accepted, 2026-08-25)
- ADR-004 — Runtime delle app in sandbox WASM/WASI (accepted, 2026-08-25)
- ADR-005 — Economia del token a mint & burn (accepted, 2026-08-25)
