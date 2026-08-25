---
id: HANDOFF-001
title: "Handoff di sessione — chiusura di M-01 e sessione autonoma notturna"
status: ready
from_role: AGENT-LEAD
to_role: AGENT-LEAD
created: 2026-08-25
updated: 2026-08-25
related_specs: [SPEC-001, SPEC-002, SPEC-003, SPEC-004]
related_reviews: [REVIEW-001, REVIEW-002, REVIEW-003, REVIEW-004, REVIEW-005, REVIEW-006, REVIEW-007]
related_decisions: [ADR-001, ADR-002, ADR-003, ADR-004, ADR-005, ADR-006, ADR-007]
links: [DEBT-001, DEBT-005, DEBT-006, DEBT-007, DEBT-008]
tags: [session-handoff]
activity:
  - date: 2026-08-25
    action: "created"
---
# Project Lead session handoff

## Purpose of this handoff

Consegnare lo stato del progetto Coblox Network dopo la sessione di bootstrap e la sessione autonoma notturna del 2026-08-25, in cui l'operatore ha delegato al Lead dispatch e review con il mandato di completare tutte le spec aperte e registrare ogni debito emergente.

Il Lead entrante deve poter riprendere senza rileggere la conversazione. **Verificare le affermazioni di questo documento prima di agire**: sono uno scatto al momento della scrittura.

## Executive project state

Coblox Network è una rete P2P dove gli utenti offrono i propri dispositivi (Android, desktop Windows/Linux, server headless) come nodi che forniscono availability, storage e compute, in cambio di un token interno **mai convertibile in denaro** (esclusione permanente e di principio). Vedi [[PROJECT]].

**M-01 è di fatto completata.** Tre spec su quattro sono `done`; la quarta è accettata tecnicamente e attende un solo gate dell'operatore.

| Spec | Stato | Nota |
| --- | --- | --- |
| [SPEC-001] Protocollo v0 | `done` | Tre giri di remediation di sicurezza; `GATE-SECREVIEW` attestato |
| [SPEC-002] Workspace Rust + CI | `done` | `GATE-CI-GREEN` derogato, coperto da [DEBT-001] |
| [SPEC-003] Design system | `review` | **Bloccata solo da `GATE-OPERATOR-LOOK`** |
| [SPEC-004] Threat model | `done` | Ha istruito [ADR-007] |

Sette ADR accettati, sette review, cinque debiti aperti. Repository su `github.com/fathorMB/CobloxNetwork`, branch unico `main`, ultimo commit `301f5b5`.

## Work completed in this session

**Bootstrap (prima parte).** Popolato il brain da zero: [[PROJECT]], [[ROADMAP]] con otto milestone, nove profili agente attivati, ADR-001…ADR-005 (federazione BFT, challenge crittografici, core Rust con shell native, sandbox WASM, economia mint & burn), quattro spec di M-01 redatte e approvate.

**[ADR-006]** — pubblicazione delle app e ricompensa al creatore. Nato da una domanda dell'operatore sul flusso di pubblicazione; ha scoperto una lacuna strutturale in [ADR-005]: il publisher non guadagnava nulla dagli abbonamenti, quindi pubblicare sarebbe stato puro costo crescente col successo.

**[ADR-007]** — posizione anti-Sybil, **decisa dal Lead su delega esplicita dell'operatore**. È l'unica decisione di prodotto presa in sua assenza e va portata alla sua attenzione.

**Sessione autonoma notturna.** Sette dispatch, sette review, quattro debiti nuovi, quattro commit spinti su `main`.

## Active work and current position

**Nessun agente in lavorazione.** Tutti e sette i dispatch sono conclusi.

L'unico lavoro aperto è [SPEC-003], in `review`, accettata tecnicamente con [REVIEW-004] e ferma sul solo `GATE-OPERATOR-LOOK`. Il sistema rifiuta correttamente `spec_done` finché quel gate non è attestato: il Lead ha provato e ha ricevuto `invariant failed`. **Non aggirarlo**: è un giudizio estetico che spetta all'operatore, e `spec_attest_operator_delegated` non va usato per attestare che qualcuno ha visto qualcosa che non ha visto.

## Ready for manual handoff

Nulla. Non ci sono spec in `ready` in attesa di dispatch. Il prossimo lavoro è M-02, che non ha ancora spec redatte.

## Pending review or evidence to inspect

- `.lmbrain/design/coblox-design-system/index.html` — punto d'ingresso del pacchetto di design che l'operatore deve guardare per attestare il gate. Da lì si raggiungono la galleria dei componenti e i tre mockup.
- `.lmbrain/knowledge/threat-model.md` — 1930 righe, 36 scenari, 24 `SEC-REQ` mappati a milestone, 15 test di attacco. È il documento più denso del progetto e la base di M-02 e M-03.
- `.lmbrain/knowledge/commit-discipline.md` — errore di processo del Lead, con l'attribuzione reale di due commit fuorviante.

## Decisions, assumptions, and constraints

**Vincoli permanenti**, da non violare senza un ADR che li superi: il token non deve mai poter acquisire valore monetario, neanche di fatto; nessun trasferimento diretto utente→utente in v0 (reso *strutturalmente inesprimibile* nel formato del ledger, non solo vietato a parole); lingua del prodotto inglese per tutto ciò che vede l'utente finale, italiano per il lavoro interno e il brain.

**[ADR-007], da rivedere con l'operatore.** Adotta l'opzione 4a del threat model: difesa economica (fondo a tetto per il reddito di esistenza, frazione `α` sorvegliata, eleggibilità a validatore ancorata a lavoro difficile da falsificare) più Argon2id come pavimento d'ingresso. Ha **riformulato una metrica di successo di [[PROJECT]]**: "zero accrediti a nodi emulati" era irraggiungibile e non lo si poteva scoprire senza il threat model. Se l'operatore non concorda, va superata con una nuova ADR e non modificata a mano.

L'affermazione strutturale che la governa: *il reddito di esistenza è perpetuo, il costo di enrollment è una tantum, e un costo una tantum non può prezzare un flusso perpetuo.* Vale per qualunque prova d'ingresso. La leva reale è `α`, verificata dal Lead: con `α`=1 una flotta di 10.000 identità emulate contro 1.000 nodi onesti cattura il 90,9% dell'emissione; con `α`=0,1 ne cattura il 9,1%.

**Strategia di branching** (`.lmbrain/BRANCHING.json`): main-only, gli specialisti non fanno mai commit né push, il Lead committa e pusha al passaggio di una spec a `done`. Nessun installer o release lato GitHub per ora.

## Risks, blockers, and unresolved questions

**Debiti aperti**, in ordine di gravità:

| ID | Severità | Questione |
| --- | --- | --- |
| [DEBT-005] | critical | Il set di validatori è **auto-perpetuante per costruzione**: il protocollo autentica la transizione ma non vincola chi possa entrare nel set successivo. Un quorum raggiunto una volta può impegnare sé stesso all'infinito e il light client non se ne accorge. **Nessuna devnet deve accumulare storia conservabile prima che la regola di elezione sia scritta.** |
| [DEBT-001] | high | La pipeline CI **non ha mai eseguito una sola riga**: la fatturazione GitHub blocca l'avvio dei job. Il rischio n.1 di [ADR-003], l'attrito della toolchain cross-platform, resta non sbancato. Solo l'operatore può sbloccarlo. |
| [DEBT-006] | high | La quota al creatore di [ADR-006] obbliga a pubblicare per sempre chi è abbonato a cosa. È l'unica superficie del threat model priva di un ADR alle spalle. |
| [DEBT-007] | high | La forma del reddito di esistenza non è decisa e determina `α`. Senza, il simulatore di M-02 non ha un modello da simulare. |
| [DEBT-008] | low | Due frasi della specifica promettono poco più di quanto le regole impongano. Una riga ciascuna. |

**Decisioni aperte dell'operatore:** nome del token e dell'unità (oggi un segnaposto `◇`); font monospace (AGENT-006 propone JetBrains Mono con motivazione tecnica); valore `X` della metrica riformulata; sorte dei file di configurazione degli harness (`.codex/`, `.pi/`, `.mcp.json`, `opencode.json`), esclusi dai commit perché contengono percorsi assoluti e nome utente.

**Il claim di sicurezza.** AGENT-007 lo giudica difendibile solo in questa forma: la rete è robusta contro la falsificazione ma **non** resistente ai Sybil per via crittografica, e tre cose non sono garantite — disponibilità dell'enrollment sotto attacco sostenuto, resistenza Sybil crittografica, verifica indipendente dell'eleggibilità prima di M-02. Sue parole: *"il progetto non deve chiamare la rete super-sicura senza quelle tre frasi accanto."*

## Documentation updated

`PROJECT.md` (metrica riformulata, vincolo di lingua, claim di sicurezza preciso), `ROADMAP.md` (M-01 `active` con le sue spec), `STATUS.md`, `BACKLOG.md`, `BRANCHING.json`, `agents/registry.md` e il profilo AGENT-006, `knowledge/threat-model.md`, `knowledge/build-toolchain.md`, `knowledge/commit-discipline.md`, i cinque documenti in `docs/protocol/` (da 1268 a 2607 righe), il pacchetto di design in `.lmbrain/design/coblox-design-system/`.

## Recommended next actions

1. **Attendere l'operatore su [ADR-007]**: è l'unica decisione di prodotto presa in sua assenza.
2. **Chiudere [SPEC-003]** appena l'operatore attesta `GATE-OPERATOR-LOOK`, poi commit e push. Chiude M-01.
3. **Redigere le spec di M-02**, dove la priorità è dettata dai debiti: la regola di elezione dei validatori ([DEBT-005], critico) viene prima di tutto, poi il simulatore economico con `α` ([DEBT-007]) da cui dipendono i parametri, poi un ADR sulla privacy ([DEBT-006]).
4. **Non dimenticare [DEBT-001]**: alla ripresa della fatturazione serve una run verde e la ri-attestazione di `GATE-CI-GREEN`. Fino ad allora nessuna affermazione sulla tenuta cross-platform è dimostrata.
5. **Applicare la disciplina di commit**: mai `git add -A` o percorsi larghi con agenti in lavorazione.

## Receiving Project Lead checklist
- [ ] Read this handoff and linked artifacts.
- [ ] Read `STATUS.md` and relevant current specs.
- [ ] Inspect the repository/Git state where relevant.
- [ ] Validate claims before changing project status or making recommendations.
- [ ] Update `STATUS.md` if the validated state differs from this snapshot.

## Handoff outcome
> Filled by the receiving Project Lead.

- [ ] Context consumed
- [ ] Handoff superseded or archived
