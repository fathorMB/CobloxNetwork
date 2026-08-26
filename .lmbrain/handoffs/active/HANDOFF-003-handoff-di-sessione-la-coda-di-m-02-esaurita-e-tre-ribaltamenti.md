---
id: HANDOFF-003
title: "Handoff di sessione — la coda di M-02 esaurita, e tre ribaltamenti"
status: ready
from_role: AGENT-LEAD
to_role: AGENT-LEAD
created: 2026-08-26
updated: 2026-08-26
related_specs: [SPEC-015, SPEC-016, SPEC-017, SPEC-018, SPEC-019, SPEC-020, SPEC-021]
related_reviews: [REVIEW-025, REVIEW-026, REVIEW-027, REVIEW-028, REVIEW-029, REVIEW-030, REVIEW-031, REVIEW-032, REVIEW-033, REVIEW-034, REVIEW-035]
related_decisions: [ADR-012, ADR-013, ADR-016]
links: [DEBT-024, DEBT-025, DEBT-027, DEBT-028, DEBT-029, DEBT-031, DEBT-032, DEBT-033, DEBT-034]
tags: [session-handoff]
activity:
  - date: 2026-08-26
    action: "created"
---
# Project Lead session handoff

## Purpose of this handoff

Consegnare lo stato dopo la sessione del 26 agosto 2026, che ha **esaurito la coda delle spec di M-02**. Supera [HANDOFF-002], scritto quando la coda ne contava ancora tre.

**Verificare le affermazioni di questo documento prima di agire.** Questa sessione ha prodotto tre casi in cui un'affermazione scritta con cura da uno specialista era falsa, e in un caso la falsità ha attraversato tre artefatti prima che qualcuno la guardasse.

## Executive project state

**Ventuno spec, tutte `done`.** `backlog`, `ready`, `working` e `review` sono **vuote**. Nessun agente in esecuzione. L'albero è pulito e allineato a `origin/main`.

**181 test**, `clippy -D warnings` e `fmt --check` puliti. Gate di [ADR-012] `PASS` con **158 probe C10**; prova in negativo con **17 mutazioni su 11 classi più ogni probe osservata fallire da sola**. La guida pubblica ha **86 affermazioni ancorate su 84 probe**, con la propria prova in negativo. Quattro skill attive.

**Dieci debiti aperti**, di cui **tre `high`**, e due deferred.

## Le tre cose che il Lead entrante deve leggere prima delle altre

**1. Tre volte una dimostrazione valida è stata letta come conclusiva oltre il perimetro su cui era fatta.**

- AGENT-007, valutando [DEBT-022], raccomandò la lettura «finalizzata» motivando **sul margine dove la proprietà era disponibile**. AGENT-002 la contraddisse e aveva ragione: `BlockHeader` ha dodici campi e nessuno registra la finalità di un antenato, quindi quella lettura non e' una regola stretta con margine largo, **è un fork con una specifica**.
- AGENT-002, nella stessa spec, dimostrò che `identity.md:614` non è una regola di validità su un blocco e concluse che **quindi** non produce due verdetti. La seconda cosa non segue dalla prima: un verdetto locale può rientrare in catena **attraverso una firma di quorum**.
- AGENT-007, valutando [DEBT-017], stabilì che il minorante è **fail-closed** guardando **una delle due metà** della regola 5, quella della scadenza. Sull'altra metà, alzare `now_ms` **ammette**. Questo errore ha attraversato la valutazione, la review del Lead che la citava come accertamento, e `identity.md` che l'aveva ereditata.

**La lezione non è che qualcuno abbia sbagliato: è che una dimostrazione va consegnata insieme al perimetro su cui vale**, altrimenti chi la cita ne eredita la conclusione e non il confine.

**2. Il Lead ora ha una guardia su di sé, e ha già morso.** `sim/tools/lead_claims_check.py` impone due regole agli artefatti del Lead: una review firmata dal Lead deve dire **cosa ha attaccato senza riuscire a romperlo**, e un superlativo assoluto deve portare la traccia di un'enumerazione. Scrivendola ha trovato **due affermazioni false scritte dal Lead quella notte**. Vincola dal 2026-08-26 in avanti; l'arretrato è **36 superlativi non enumerati**, contati a ogni esecuzione e tracciati in [DEBT-027].

**3. Una gate misura l'insieme dichiarato, non quello osservato — quattro volte.** `SECURITY.md` fuori dall'inventario di [ADR-012]; due liste in `published_artifacts.py` senza lato disco; `CadenceBand` assente dall'elenco dei portatori in `lib.rs`; e `src/` fuori dallo scopo della passata ([DEBT-031]). **Ogni volta il membro mancante era l'ultimo arrivato**, perché un insieme dichiarato non ha modo di accorgersi di un nuovo membro e chi lo aggiunge non sa che esiste una lista.

## Work completed in this session

Ventitré commit, da `dfe44fe` a `2d3c6d7`.

**Sei spec chiuse**: [SPEC-015] (con deroga trasferita), [SPEC-016], [SPEC-017], [SPEC-018], [SPEC-019], [SPEC-020], [SPEC-021]. **Nove debiti risolti**: [DEBT-013], [DEBT-014], [DEBT-017], [DEBT-018], [DEBT-019], [DEBT-020], [DEBT-021], [DEBT-022], [DEBT-023].

**Recuperata l'implementazione di [SPEC-014]**, accettata il giorno prima e mai committata: sette file di `core/` erano nell'albero e in nessun commit.

**[ADR-016]** decisa dall'operatore: la banda di cadenza di genesi, larga sul lato lento e stretta dove sta l'emissione. La proposta del Lead conteneva un errore sul lato lento, corretto prima di scriverla.

**Quattro skill attive**, nate perché la disciplina di verifica viveva solo nei prompt di dispatch scritti a mano. Al primo impiego [SKILL-001] ha prodotto un caso di prova che senza di essa non sarebbe esistito, e le altre due hanno ricevuto due critiche di taratura, entrambe applicate.

**La guida pubblica è stata rivista, corretta e dichiarata pubblicabile**, ma **l'operatore ha deciso di non pubblicarla** su un canale permanente finché [DEBT-032] è aperto.

## Active work and current position

**Nessuna.** Coda esaurita, nessun agente, albero pulito.

## Ready for manual handoff

**Nessuna spec redatta.** Il prossimo lavoro va deciso, e i candidati sono di due nature.

**I tre debiti `high`**, tutti M-02 e tutti chiudibili prima della devnet:

- **[DEBT-033]** — `effective_height` non ha tetto, e il campo `reason` che porterebbe la distinzione **esiste già ed è inerte**: due occorrenze in tutto il protocollo, nessuna regola lo legge. Il tetto ovvio è famiglia 3 **e inefficace**, perché un quorum ostile sceglierebbe il massimo. AGENT-002.
- **[DEBT-034]** — un verdetto locale del ricevente può entrare in catena attraverso una firma di quorum, sul percorso della sfida. Si compone con [DEBT-033]: la finestra che entrambi sfruttano è la stessa. AGENT-007.
- **[DEBT-028]** — `election_epoch` dipende da un parametro governato senza che il documento dica quale versione valga. Terza porta sulla stessa famiglia dopo [DEBT-012] e [DEBT-020]. AGENT-002.

**L'esito residuo di M-02 che nessuna di queste tocca**: devnet BFT, light client con prove Merkle, mint & burn. È il lavoro che la milestone nomina, e non è mai stato cominciato.

## Pending review or evidence to inspect

Nulla in attesa. Tutte le review sono `accepted`.

**Due decisioni di taratura aspettano l'operatore**, entrambe emerse verificando e nessuna bloccante: `max_clock_drift_ms` non è fissato da alcun documento di genesi — l'unico valore in albero è un input di test — e `D_max`/`S_max` restano non fissati.

## Decisions, assumptions, and constraints

**[ADR-013] annotata**: il pericolo della cadenza ha ora **due direzioni** che non si scambiano fra loro, e le due soglie non sono la stessa — rallentare basta a un **terzo bloccante**, accelerare richiede un **quorum**. Il movente dominante resta il rallentamento, **ed è il lato su cui il protocollo si limita a segnalare**: il progetto fallisce chiuso sul lato più caro da attaccare. È la scelta giusta e va saputa.

**Vincoli di processo confermati:** gli specialisti non committano né pushano; mai staging ampio mentre un agente lavora sullo stesso albero; testo di prodotto in inglese, artefatti del brain in italiano; le convenzioni sull'unità sono in [ADR-009] e **non** ripetute nelle skill.

## Risks, blockers, and unresolved questions

**Nessun blocker.**

**Il rischio dominante è la composizione.** Due volte in questa sessione un difetto non stava in nessuna delle due metà ma **nel fatto di comporle**: in [SPEC-017] fra [DEBT-020] e [DEBT-021], e in [SPEC-019] dove chiudere l'asimmetria ha spostato il peso su una grandezza non vincolata. Nessuna gate cerca questa forma, perché **le grandezze non cambiano — cambia ciò che vi poggia sopra**.

**[DEBT-032] è il debito da guardare per primo fra i `medium`**, perché il suo costo è già **misurato** e non stimato: la guida è invecchiata **due volte in due giorni** con tutte le probe verdi, e la seconda volta l'ha trovata una passata deliberata e non uno strumento.

**[DEBT-025] va chiuso prima che la sua strumentazione invecchi**: `threat_model_matrix_coherence.py` esiste, non è cablato in CI, e gli otto disallineamenti noti vanno sanati prima del cablaggio, altrimenti nasce rosso e viene disattivato.

## Documentation updated

`SECURITY.md`, `docs/protocol/README.md`, `ledger.md`, `identity.md`, `.lmbrain/knowledge/threat-model.md`, `recurring-defects.md`, `lead-claims-discipline.md` (nuova), `derivazioni-non-univoche.md` (nuova), `.lmbrain/STATUS.md`, `.lmbrain/ROADMAP.md`, [ADR-013], la guida pubblica e i suoi strumenti.

## Recommended next actions

1. **Decidere la natura del prossimo blocco**: chiudere i tre `high` prima della devnet, oppure cominciare l'esito di M-02 che nessuno ha ancora toccato. Sono lavori diversi e non si sostituiscono.
2. Se i debiti: **[DEBT-033] e [DEBT-034] insieme**, perché la finestra che sfruttano è la stessa.
3. **[DEBT-032]** appena possibile: è quello che ha già dimostrato di costare.
4. Portare all'operatore le due decisioni di taratura.

## Receiving Project Lead checklist

- [ ] `cargo test --workspace --all-features` → attendersi **181**.
- [ ] `python sim/tools/published_artifacts.py` → **PASS**, 158 C10.
- [ ] `python sim/tools/published_artifacts_negative.py` → **17 mutazioni, 11 classi, ogni probe individualmente**.
- [ ] `python sim/tools/lead_claims_check.py` → **PASS**. Leggerlo prima di scrivere la prima review.
- [ ] `node .lmbrain/design/coblox-public-guide/tools/check-guide-pairs-negative.mjs` → **PASS**.
- [ ] Leggere le tre cose in cima a questo documento **prima** di accettare qualunque cosa.

## Handoff outcome

> Compilato dal Lead entrante.
