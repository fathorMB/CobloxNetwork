---
title: Roadmap
updated: 2026-08-27
---

# Roadmap

> Roadmap di massima. Ogni milestone verrà scomposta in SPEC solo quando la precedente è vicina alla chiusura. Le decisioni portanti sono in ADR-001…ADR-005.

## Milestones

### M-01 — Fondamenta: protocollo su carta e scheletro del core

- `status`: completed
- `nota`: **Completata il 2026-08-25**: tutte e quattro le spec sono `done`. SPEC-001, SPEC-002 e SPEC-004 chiuse nella sessione autonoma; SPEC-003 chiusa quando l'operatore ha attestato `GATE-OPERATOR-LOOK`. Due debiti nati qui sono già risolti — DEBT-001 (la CI ha eseguito e passa su tutti e cinque i job) e DEBT-009 (`cargo-deny` esteso al grafo della shell desktop) — e restano aperti DEBT-005, DEBT-006, DEBT-007, DEBT-008. Il rischio `toolchain Android/UniFFI` è **sbancato**: la prima esecuzione reale della CI ha rivelato solo difetti di confezionamento del repository, nessun problema di toolchain. Il secondo rischio, `sotto-specificare l'elezione dei validatori`, si è invece materializzato ed è DEBT-005, critico, prima voce di M-02.
- `outcome`: Le specifiche del protocollo (identità dei nodi, formato ledger, messaggi P2P, manifest delle app) esistono come documenti versionati; il workspace Rust `coblox-core` compila su Win/Linux/Android con CI multipiattaforma; il design system "hacker" ha i suoi token di base (palette, tipografia, componenti chiave); il threat model iniziale è redatto.
- `specs`: [SPEC-001, SPEC-002, SPEC-003, SPEC-004]
- `risks`: [toolchain Android/UniFFI, sotto-specificare l'elezione dei validatori]

### M-02 — Ledger vivo: federazione BFT su devnet

- `status`: active
- `outcome`: Una devnet di validatori seed raggiunge consenso BFT; i light client verificano saldi con prove Merkle; mint & burn implementati a livello di transazioni; primo simulatore economico per la taratura dei parametri.
- `specs`: [SPEC-005, SPEC-006, SPEC-007, SPEC-008, SPEC-009, SPEC-010, SPEC-011, SPEC-012, SPEC-013, SPEC-014, SPEC-015, SPEC-016, SPEC-017, SPEC-018, SPEC-019, SPEC-020, SPEC-021, SPEC-022, SPEC-023, SPEC-024, SPEC-025, SPEC-026, SPEC-027]
- `risks`: [complessità del consenso, taratura curve emissione/burn]
- `nota`: **Ventuno spec `done`, quattro aperte.** [SPEC-022] e [SPEC-023] sono in `review` con remediation; [SPEC-024] e [SPEC-025] in `backlog`. Aggiunte all'elenco il 2026-08-27: dichiaravano `M-02` senza essere elencate qui. Il lavoro che doveva precedere la devnet e' completo — regole di consenso scritte, applicate e verificate; verificatore di firme misurato contro un oracolo upstream; legame fra identita e indirizzo di rete ne pubblicato ne ricalcolabile; [ADR-018] fissa il protocollo di consenso. Resta la parte che la milestone nomina: devnet BFT, light client con prove Merkle, mint & burn. [SPEC-025] e' il primo passo e attua [ADR-018]. [SPEC-015] non appartiene all'esito di questa milestone ed e assegnata qui perche e la corrente: e la guida pubblica al funzionamento, artefatto vivo che conviene cominciare ora invece che a M-08.
- `nota storica`: Cinque spec `done`. Le quattro nuove sono in `backlog` dal 2026-08-25 e precedono tutte la devnet, ciascuna per una ragione propria: [SPEC-010] rende eseguibile la gate di [ADR-012]; [SPEC-011] chiude il divario fra regole scritte e regole applicate; [SPEC-012] dà alla rete il verificatore di firme che non ha; [SPEC-013] deve atterrare prima del primo certificato di enrollment. Il rischio *taratura curve emissione/burn* è **rientrato** con [SPEC-007] e [SPEC-009].

### M-03 — Presenza dimostrata: challenge di availability e reddito di esistenza

- `status`: proposed
- `outcome`: I nodi rispondono a ping firmati con nonce; il reddito di esistenza viene accreditato solo su presenza dimostrata; test di attacco Sybil documentati; prima dashboard in tempo reale (desktop Tauri) di guadagni ed eventi.
- `specs`: []
- `risks`: [campionamento aggirabile, costo batteria/dati su mobile]
- `nota`: **Il beacon di casualita' dedicato vive qui.** Assegna emittente e soggetto delle sfide, quindi il suo consumatore e' l'esito di questa milestone. Il lavoro senza proprietario e' la **quantizzazione di `timestamp_ms` allo slot di consenso**: l'altra riduzione che `ledger.md` elencava come non presa — derivare il materiale dai `block_id` di `K` blocchi consecutivi — **e' gia' presa** dal seme dell'elezione via `election_entropy_blocks`. Collocato qui il 2026-08-27 su decisione dell'operatore, per chiudere [DEBT-038]: il rimando di `ledger.md` puntava a [DEBT-005], che e' risolto e non aveva quell'oggetto, quindi indicava il vuoto.

### M-04 — Nodi ovunque: shell desktop, Android e headless complete

- `status`: proposed
- `outcome`: App desktop Tauri con il design system completo; app Android (Compose + foreground service) che partecipa rispettando batteria/dati; daemon headless con CLI e API locale; onboarding di un nuovo nodo in < 5 minuti.
- `specs`: []
- `risks`: [restrizioni background Android, UX onboarding chiavi]

### M-05 — Storage distribuito: custodia e proof-of-retrievability

- `status`: proposed
- `outcome`: I nodi custodiscono blocchi cifrati con replica; le proof-of-retrievability a campione condizionano i compensi storage; riparazione automatica quando un nodo sparisce.
- `specs`: []
- `risks`: [churn dei nodi mobili, dimensionamento replica]

### M-06 — Compute e app: runtime WASM, SDK e marketplace

- `status`: proposed
- `outcome`: I nodi eseguono moduli WASM con metering e capability; verifica a campione dei risultati; SDK per sviluppatori; pubblicare un'app costa token (burn) e hostarla li fa guadagnare; catalogo/abbonamenti ai servizi delle app.
- `specs`: []
- `risks`: [limiti WASI, equità del metering, prima app dimostrativa convincente]

### M-07 — Rotazione e resilienza: validatori eletti e hardening

- `status`: proposed
- `outcome`: I validatori ruotano per reputazione/uptime senza intervento manuale; slashing reputazionale; audit di sicurezza interno completo; la rete sopravvive a partizioni e a validatori malevoli nei test.
- `specs`: []
- `risks`: [collusione nell'elezione, governance dei parametri]

### M-08 — Beta pubblica

- `status`: proposed
- `outcome`: Rete aperta a partecipanti esterni; installer/store per le tre piattaforme; documentazione pubblica; telemetria di salute della rete; app dimostrative di terzi.
- `specs`: []
- `risks`: [scalabilità reale, supporto utenti, aspettative "guadagno" da comunicare bene]
