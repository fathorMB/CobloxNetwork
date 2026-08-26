---
id: DEBT-036
title: "Dieci parametri di consenso su venti non sono ne limitati in genesi ne dichiarati aperti"
status: open
category: "security"
severity: "high"
origin_severity: null
area: "governance"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-020","SPEC-022"]
related_reviews: []
related_decisions: ["ADR-017","ADR-010"]
target_specs: []
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-26
tags: ["security","governance","ledger"]
links: []
activity: []
debt_events:
  - schema_version: "1"
    id: "DEBT-036-EVENT-001"
    timestamp: "2026-08-26T22:27:07.119198500+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead perche' l'operatore stava per decidere tre parametri di taratura uno alla volta, ed erano tre sintomi della stessa lacuna.\n\nVale la pena registrare come e' stato trovato, perche' e' la sesta occorrenza della famiglia e la prima trovata **cercando la classe invece dell'occorrenza**. Le prime cinque - `SECURITY.md` fuori dall'inventario di [ADR-012], due liste in `published_artifacts.py` senza lato disco, `CadenceBand` assente dai portatori in `lib.rs`, `src/` fuori dallo scopo della passata, SKILL-004 assente dal registro - sono state trovate una per volta, ciascuna come un membro mancante da un elenco. Questa e' stata trovata perche' il Lead, dovendo portare la terza decisione di taratura in due giorni, si e' chiesto se le tre fossero tre o fossero una.\n\nLa lezione che vale oltre questo debito: **quando la stessa forma di questione si presenta tre volte, la terza volta la domanda giusta non e' la questione ma la classe.** Il progetto aveva registrato due decisioni pendenti in [HANDOFF-003] e ne stava aggiungendo una terza, e nessuna delle tre registrazioni aveva chiesto quante altre ce ne fossero."
    evidence_refs: []
---
# Dieci parametri di consenso su venti non sono ne limitati in genesi ne dichiarati aperti

## Statement

`ConsensusParametersBody` ha **venti campi**. I dieci di elezione — da `election_epoch_blocks` a `validator_min_capture_epochs` — hanno un limite di magnitudine preso dall'ancora di genesi in `ledger.md#magnitudes-not-only-relations`, **e** sono dichiarati aperti nella lista DRAFT dei parametri di lancio di `README.md`.

Gli altri dieci non hanno **ne' l'uno ne' l'altra**:

`max_clock_drift_ms`, `max_envelope_validity_ms`, `max_transport_attestation_validity_ms`, `max_transport_attestation_future_skew_ms`, `replay_cache_entries_per_peer`, `replay_cache_entries_global`, `max_weak_subjectivity_age_ms`, `max_current_balance_age_ms`, `app_suspension_notice_epochs`, `min_revocation_effective_delay_blocks`.

Tutti e dieci valgono `1` in albero, e `1` e' il valore delle fixture. **Nessun documento di genesi ne fissa uno, e nessun documento dichiara che siano aperti**, quindi non sono ne' decisi ne' registrati come da decidere.

Sono la meta' operativa e di sicurezza dei parametri: orologi, finestre di validita', cache anti-replay, freschezza dell'ancora di fiducia, ritardo della revoca. **Il quorum sedente li firma**, ed e' precisamente la ragione per cui il blocco dei vincoli di magnitudine esiste per l'altra meta'.

## Evidence and provenance

Trovato dal Lead il 2026-08-26 mentre raccoglieva i vincoli per portare all'operatore le decisioni di taratura rimaste aperte, sull'albero a `785ad67`.

Enumerazione eseguita, non dedotta: i venti campi sono letti da `docs/protocol/README.md:808-829`. Per ciascuno dei dieci operativi e' stato verificato il valore in `sim/tools/protocol_hashes.py` e la presenza nella sezione DRAFT di `README.md:1608-1632`. Esito: **dieci su dieci a `1`, dieci su dieci assenti dalla lista DRAFT**.

Il blocco dei vincoli di magnitudine e' stato letto per intero (`ledger.md:1985-2012`): contiene i parametri di elezione, le due unita' storage/compute, e `validator_eligibility_min_issuers`. **Nessuno dei dieci.**

**Tre di questi dieci erano gia' stati incontrati, uno alla volta e per caso**, senza che nessuno guardasse la classe: `max_clock_drift_ms` in [SPEC-020], confermato da [REVIEW-034] e [REVIEW-035]; `min_revocation_effective_delay_blocks` da [ADR-017]; `max_weak_subjectivity_age_ms` guardandolo adesso. Ogni volta e' stato registrato come una decisione di taratura pendente e mai come il sintomo di una lista incompleta.

**Distinzione che va conservata e che smentisce una lettura troppo netta.** `max_weak_subjectivity_age_ms` ha una protezione parziale che gli altri nove non hanno: il checkpoint porta la propria copia, il client usa quella firmata dalla distribuzione, e MUST fallire chiuso se le due non concordano (`README.md:1599-1606`). Il canale di rilascio lo vincola quindi di fatto, anche se il documento di genesi no. Gli altri nove non hanno alcun secondo canale.

## Impact and scope boundary

Due danni distinti, e vanno tenuti separati perche' hanno rimedi diversi.

**Il primo e' che un quorum sedente puo' camminarli.** `ledger.md` dichiara che il blocco dei vincoli esiste per impedire a un set seduto di portare un parametro governato fino a un valore assurdo. Per questi dieci quella protezione non c'e'. Un `max_clock_drift_ms` enorme fa accettare blocchi da un futuro arbitrario; un `max_weak_subjectivity_age_ms` enorme fa accettare a un light client checkpoint antichi; `replay_cache_entries_*` a zero toglie la difesa anti-replay. **Sono le stesse conseguenze contro cui il blocco protegge l'altra meta'.**

**Il secondo e' che nessuno sa che vanno decisi.** La lista DRAFT e' la superficie su cui il progetto registra cio' che resta da scegliere prima di dichiararsi mainnet, e `README.md` dice che finche' i parametri firmati non selezionano valori un deployment e' una rete di sviluppo. Dieci parametri fuori da quella lista sono dieci decisioni che nessun documento reclama: alla devnet arriverebbero con il valore che qualcuno ha messo per far passare un test.

`high` perche' il secondo danno **rende invisibile il primo**: finche' non sono dichiarati aperti, il fatto che non siano nemmeno limitati non ha modo di farsi notare. E' la sesta occorrenza della famiglia dell'insieme dichiarato, ed e' la piu' grande: le prime cinque avevano un membro mancante ciascuna, questa ne ha dieci.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead perche' l'operatore stava per decidere tre parametri di taratura uno alla volta, ed erano tre sintomi della stessa lacuna.

Vale la pena registrare come e' stato trovato, perche' e' la sesta occorrenza della famiglia e la prima trovata **cercando la classe invece dell'occorrenza**. Le prime cinque - `SECURITY.md` fuori dall'inventario di [ADR-012], due liste in `published_artifacts.py` senza lato disco, `CadenceBand` assente dai portatori in `lib.rs`, `src/` fuori dallo scopo della passata, SKILL-004 assente dal registro - sono state trovate una per volta, ciascuna come un membro mancante da un elenco. Questa e' stata trovata perche' il Lead, dovendo portare la terza decisione di taratura in due giorni, si e' chiesto se le tre fossero tre o fossero una.

La lezione che vale oltre questo debito: **quando la stessa forma di questione si presenta tre volte, la terza volta la domanda giusta non e' la questione ma la classe.** Il progetto aveva registrato due decisioni pendenti in [HANDOFF-003] e ne stava aggiungendo una terza, e nessuna delle tre registrazioni aveva chiesto quante altre ce ne fossero.

## Resolution criteria

Due lavori distinti, e il secondo dipende dal primo.

**1. Portare i dieci nella lista DRAFT**, raggruppati per cosa governano, con dichiarata accanto la grandezza che li vincola. E' il lavoro economico e va fatto per primo, perche' rende visibile il resto.

**2. Decidere, per ciascuno, se serva un limite di magnitudine in genesi**, e questa **non e' una decisione uniforme**. La domanda giusta per ognuno e': *un quorum sedente che porta questo parametro al proprio estremo cosa ottiene?* Dove la risposta e' un guadagno per l'attaccante, il limite serve. Dove esiste gia' un secondo canale che lo vincola — il caso di `max_weak_subjectivity_age_ms`, che il checkpoint porta con se' — il limite in genesi va valutato ma potrebbe essere ridondante, **e la ridondanza va dichiarata invece che assunta**.

**Non fare la cosa ovvia**, che sarebbe aggiungere dieci righe di limite in blocco: sarebbe la famiglia 3, vincolare le grandezze nominate senza chiedersi da quale dipenda la proprieta'. Alcuni di questi dieci si vincolano meglio in relazione fra loro che con un tetto assoluto — `max_weak_subjectivity_age_ms` ha gia' un MUST che lo lega a `min_revocation_effective_delay_blocks`, ed e' un vincolo relazionale, non di magnitudine.

**Va inoltre deciso se la classe sia chiusa**: `ConsensusParametersBody` ha venti campi oggi, e nulla impedisce che il ventunesimo nasca fuori da entrambe le liste come questi dieci. Una gate che confronti i campi dello schema con l'unione di lista DRAFT e blocco dei vincoli, fallendo su un campo che non sta in nessuna delle due, e' la contromisura alla forma e non alla singola occorrenza.

Da chiudere **prima della devnet**, perche' e' li' che i valori smettono di essere fixture.

## Resolution evidence

