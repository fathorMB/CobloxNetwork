---
id: SPEC-016
# Note: Quote the title if it contains a colon
title: "Gli orologi della catena: cadenza misurabile, epoca di ricompensa, e il legame di catena del set"
status: review
kind: feature
priority: high
area: consensus
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012, ADR-013, ADR-010]
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [consensus, governance, light-client]
activity:
  - date: 2026-08-26
    action: "transitioned backlog -> ready"
  - date: 2026-08-26
    action: "transitioned ready -> working"
  - date: 2026-08-26
    action: "transitioned working -> review"
---
# Gli orologi della catena: cadenza misurabile, epoca di ricompensa, e il legame di catena del set

## Objective

Chiudere [DEBT-013], [DEBT-019] e [DEBT-014], che sono **tre facce della stessa domanda**: quali grandezze della catena sono scritte dai validatori, e quali proprietà il protocollo promette poggiandovi sopra.

Le prime due si chiudono nella stessa forma perché hanno la stessa causa, stabilita da AGENT-007 valutando [DEBT-013] e scritta in [ADR-013] parte 3: **nessuna regola di validità interna alla catena può vincolare il tempo reale, perché ogni orologio della catena è scritto dai validatori.** La terza si chiude in due paragrafi ed è qui perché **lavora sullo stesso oggetto** — il checkpoint di soggettività debole — e chi scrive la prima deve conoscerne la conclusione per non riaprirla.

## Context

**[DEBT-013].** `docs/protocol/` non specifica né la selezione del proposer né la meccanica dei round: manca il livello di produzione dei blocchi per intero. Un **terzo bloccante** — non un quorum — può allungare la durata reale delle epoche con la catena viva e ogni blocco valido. Tre effetti, con gravità diverse: l'incumbency diventa **illimitata**; il ritardo effettivo di revoca è denominato in blocchi e `ledger.md` promette che la catena *si ferma* a `effective_height`, altezza che rallentando **non arriva mai**; e l'emissione **si muove verso il basso**, quindi il rallentamento ha un costo **esternalizzato** — si perde l'emissione di tutta la rete e si conserva il seggio del solo cartello.

**[DEBT-019].** Nessun documento deriva `reward_epoch` da alcunché. Il pavimento su `reward_epoch_ms` introdotto da [SPEC-009] vincola la **durata dichiarata in un documento firmato**, non la velocità con cui gli indici avanzano nei mint. È [REVIEW-014] un livello più sotto.

**[DEBT-014] — rifiuto motivato, già valutato.** `validator_set_hash` **non ha bisogno** del legame con `chain_id`: i byte di un `ValidatorSet` lo legano già tre volte, e il Lead ne ha verificate due (`election_seed` ed `election_ticket` contengono `chain_id_32`). Regge su tutte e tre le superfici. **Resta da scriverlo**, perché un'eccezione non dichiarata si legge come una dimenticanza.

## Scope

### Included

- La misura della cadenza reale lato light client, dal checkpoint di soggettività debole.
- La procedura di rilascio dei checkpoint e la riga di genesi che la banda richiede.
- La derivazione di `reward_epoch`, o la dimostrazione motivata che non è ottenibile dentro la catena.
- I due paragrafi dichiarativi di [DEBT-014] e l'allineamento del commento in `registry.rs`.

### Excluded

- **Specificare la produzione dei blocchi** — selezione del proposer, meccanica dei round. È lavoro proprio e più grande, e [ADR-013] lo nomina come la premessa che, se cadesse, imporrebbe di riesaminare la sua parte 3.
- **Una regola di validità sulla distanza fra `timestamp_ms` consecutivi.** È **respinta** da [ADR-013]: obbliga i validatori a *scrivere* timestamp vicini, non a *produrre* blocchi vicini, e darebbe una chiusura falsa al prezzo pieno di una passata di [ADR-012]. Non va reintrodotta da nessuna porta.
- La taratura dei valori di banda, se la spec conclude che servono: è decisione dell'operatore, come `α` e la popolazione al lancio.

## Existing-project analysis

**Verificato dal Lead il 2026-08-26.** `election_seed = H("coblox-election-seed-v0\0" || chain_id_32 || …)` ed `election_ticket` idem: il legame di catena dentro `ValidatorSet` c'è. `reward_epoch` compare diciannove volte fra `ledger.md` e `README.md` e **nessuna occorrenza lo deriva**; l'unico `MUST` che lo nomina riguarda i limiti su `reward_epoch_ms`. Il termine *proposer* compare solo come sostantivo incidentale in discussioni di minaccia.

Il checkpoint di soggettività debole porta `height`, `timestamp_ms` e `issued_at_ms` firmati da una chiave che **non appartiene a nessun validatore**: è l'unico orologio esterno del protocollo, ed è già normativo.

## Technical proposal

L'ordine è quello di forza stabilito da AGENT-007, e la prima è la sola imprescindibile.

**1. La misura, lato light client.** Il light client ricava blocchi per millisecondo reale dal checkpoint che già detiene più l'intestazione fidata, e **fallisce chiuso o segnala** fuori da una banda dichiarata alla genesi. È contenuto normativo nuovo, quindi **la gate di [ADR-012] si applica**.

**2. La procedura di rilascio.** Il processo che emette i checkpoint non ne emette per una catena fuori banda. È procedura più una riga di genesi, non una regola di consenso.

**3. Il secondo limite in millisecondi di catena**, accanto a quello in blocchi, per le quantità che portano una promessa in tempo reale — il limite di mandato per primo. **Da sola è la stessa illusione** della regola respinta, perché i millisecondi di catena li scrivono i validatori: vale solo insieme al punto 1. Tocca `ElectionBounds` e la taratura di [SPEC-007].

**4. `reward_epoch`.** Derivarlo da una grandezza che i validatori non scrivono liberamente, oppure dimostrare che non è ottenibile dentro la catena — nel qual caso la chiusura ha **la stessa forma dei punti 1 e 2**: renderne l'avanzamento misurabile da fuori invece che vincolabile da dentro. Va valutato anche il verso opposto, un indice che non avanza affatto e congela l'emissione.

**5. [DEBT-014], due paragrafi.** In `README.md` accanto al registro delle preimmagini e in `ledger.md` accanto alla formula, più il commento in `registry.rs` cui manca la ragione. **Attenzione a un argomento falso:** la motivazione registrata nel debito — *«è una lista di chiavi, legarla impedirebbe di riusarla in una genesi nuova»* — **è sbagliata**, perché ogni `key_binding_signature` andrebbe riemessa comunque. Non va scritta: un'eccezione con la ragione sbagliata diventa un precedente.

## Files and areas involved

- `docs/protocol/ledger.md`, `docs/protocol/README.md` — la misura, la banda, `reward_epoch`, i due paragrafi.
- `core/coblox-core/src/light_client.rs`, `params.rs`, `registry.rs` — l'implementazione e il commento.
- `sim/tools/` — la gate di [ADR-012] e le eventuali probe.
- `sim/coblox_sim/` — solo se il punto 3 muove la taratura.

## Acceptance criteria

- [x] Un light client che detiene un checkpoint e un'intestazione fidata **misura la cadenza reale** e si comporta come la regola dichiara fuori banda.
- [x] La banda è dichiarata alla genesi e non scelta a runtime.
- [x] La procedura di rilascio dei checkpoint rifiuta una catena fuori banda.
- [x] `reward_epoch` è derivato da una grandezza che i validatori non scrivono liberamente, **oppure** la dimostrazione del contrario è scritta e la chiusura ha la forma dei punti 1 e 2. Entrambi i versi — indice troppo veloce e indice fermo — sono trattati.
- [x] I due paragrafi di [DEBT-014] sono scritti, e **l'argomento falso non compare**.
- [x] **Nessuna regola sulla distanza fra `timestamp_ms` è stata introdotta**, per nessuna via.
- [x] Ogni valore pubblicato che cambia è ricalcolato con il metodo validato prima su un valore non modificato.
- [x] La gate di [ADR-012] è eseguita e la trascrizione allegata.

## Implementation plan

1. Leggere la valutazione di AGENT-007 su [DEBT-013] e [DEBT-014] nel threat model: contiene la derivazione, non solo la conclusione.
2. Progettare la misura lato light client e la banda; stabilire il comportamento fuori banda **prima** di scrivere codice.
3. Affrontare `reward_epoch`, dichiarando quale dei due esiti si è raggiunto.
4. Scrivere i due paragrafi di [DEBT-014].
5. Eseguire la gate di [ADR-012] e le prove in negativo.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-MEASURE-BINDS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una catena simulata **fuori banda** produce l'esito che la regola dichiara, e una dentro banda no. La trascrizione mostra entrambi. Una misura che non si è mai vista scattare è un calcolo, non una guardia.
- [x] GATE-NO-TIMESTAMP-RULE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una ricerca su tutto il diff mostra che **nessuna regola sulla distanza fra `timestamp_ms` consecutivi è stata introdotta**. È respinta da [ADR-013] e la sua reintroduzione sarebbe la famiglia 3 commessa dentro il rimedio: la gate esiste perché è il rimedio che sembra ovvio.
- [x] GATE-BOTH-DIRECTIONS | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Per `reward_epoch` entrambi i versi sono trattati con un caso ciascuno: indice che avanza troppo in fretta e indice che non avanza. Il secondo congela l'emissione senza violare alcuna regola ed è il gemello del caso che `README.md` già dichiara invalido per `reward_epoch_ms` sopra il tetto.
- [x] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato e la trascrizione allegata, **anche se non trova nulla**.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la chiusura e il Lead ha accettato la review. Due dei tre debiti nascono da una sua valutazione, e il terzo da una sua osservazione adiacente: chiuderli senza la sua verifica sarebbe incoerente con il modo in cui sono stati aperti.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è dichiarare «chiuso» più di quanto sarà scritto.** Nessuno dei tre punti **impedisce** il rallentamento: lo rendono misurabile e dichiarato. Per un difetto la cui gravità è tutta nell'invisibilità è la parte che conta, ma la parola giusta va usata. AGENT-007 lo dice nella propria valutazione ed è il criterio con cui il Lead leggerà la consegna.
- **Il rischio secondario è il punto 3 da solo.** Un limite in millisecondi di catena senza la misura del punto 1 è la stessa illusione della regola respinta, con un'aria più tecnica.
- **La banda potrebbe richiedere un numero che non spetta all'implementatore.** Se emerge, va istruito come `α` e la popolazione al lancio — mostrando cosa comporta ciascun ordine di grandezza — e **portato all'operatore**, non scelto.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Contestare le formulazioni del Lead fa parte del mandato.** In questa spec l'ordine dei tre punti è di AGENT-007 e il Lead lo ha adottato: se l'implementazione mostra che è sbagliato, dillo.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence

### Changes made

**1. La misura, lato light client (punto 1).** Nuovo modulo `core/coblox-core/src/cadence.rs`.
Due misure, entrambe con **entrambi gli estremi fuori dalla catena**:

- `measure_cadence_from_checkpoint(chain_id, checkpoint_height, checkpoint_issued_at_ms, tip_height, now_ms, band)`
  — blocchi dal checkpoint alla testa autenticata, millisecondi da `issued_at_ms`
  (firmato dalla chiave di rilascio, che non appartiene a nessun validatore) all'orologio
  del client stesso, lo stesso che il passo 1 già usa per la freschezza;
- `measure_cadence_between_checkpoints(...)` — due checkpoint consecutivi, per il processo di rilascio.

Il confronto con la banda è **esatto in `u128`**: dividere prima farebbe rientrare in banda
una catena che ne è appena fuori. Il quoziente troncato è portato solo come diagnostica.

Il comportamento fuori banda è **asimmetrico e la ragione è la sola che lo giustifica**:

- `check_cadence_light_client` **fallisce chiuso** sopra la banda (troppo veloce) e **segnala**
  sotto. Un client non allineato conta meno blocchi di quanti la catena ne ha prodotti, quindi
  la sua misura è distorta **verso il basso e solo verso il basso**: una lettura lenta non è
  attribuibile alla catena da quella posizione, una veloce sì, perché il ritardo di sync non
  fabbrica blocchi. Fallire su una lettura che il ritardo del client stesso produce sarebbe una
  guardia che grida al lupo, che è il modo in cui [ADR-012] precisazione 3 registra la fine di
  una guardia;
- `check_cadence_release` **fallisce chiuso in entrambi i versi** e anche su `Inconclusive`:
  ha due checkpoint propri, nessun ritardo di sync, e può aspettare.

`CadenceVerdict::Inconclusive` sotto `min_measured_blocks` **non è un pass** ed è un esito
proprio: un rapporto su una manciata di blocchi è rumore.

**2. La banda alla genesi (punto 1 e 2).** `params::CadenceBand`, terzo oggetto di ancora
accanto a `ElectionBounds` e `RewardBounds`, con `block_interval_ms`, `min_ms_per_block`,
`max_ms_per_block`, `min_measured_blocks`. Regola relazionale
`min_ms_per_block <= block_interval_ms <= max_ms_per_block`: una banda che escludesse
l'intervallo dichiarato metterebbe fuori banda ogni catena conforme.
Le due misure **validano l'ancora come primo atto** — è il difetto che [REVIEW-017] RF-001
ha trovato sul lato reward, dove `RewardBounds::validate` esisteva e non era chiamato da
nessuna parte, e una banda con `min_ms_per_block = 0` ammetterebbe qualunque ritmo *senza
errore*.

**3. La procedura di rilascio (punto 2).** `README.md#weak-subjectivity-checkpoint`: il
processo misura contro l'ultimo checkpoint firmato per la stessa catena e **non emette** per
una catena fuori banda in nessuno dei due versi, né per un intervallo troppo corto. È
**procedura, non regola di validità**: trattenere un checkpoint non ferma una catena, le
toglie l'orologio esterno. Il primo checkpoint di una catena non ha predecessore ed è esente,
scritto invece che lasciato dedurre.

**4. `reward_epoch` (punto 4).** Esito raggiunto: **la derivazione esiste**. È derivato da
`height`, l'unica grandezza di questa catena che un validatore non scrive liberamente —
`height` è `previous + 1`, ricontrollabile da chiunque dalle sole intestazioni, per sempre.

```text
reward_epoch_blocks = ceil(reward_epoch_ms / block_interval_ms)
un mint che nomina reward_epoch e è valido solo a un'altezza h con
  (e + 1) * reward_epoch_blocks <= h
```

Il tetto e non il pavimento nell'arrotondamento, perché la grandezza è un **limite inferiore**
su quanta catena deve passare: arrotondare in giù allargherebbe il permesso.
Ciò che il limite dà, detto stretto quanto è vero: l'emissione di esistenza cumulativa fino
all'altezza `h` è al più `floor(h / reward_epoch_blocks) * F`. È un limite **per blocco**, non
per millisecondo reale — e il residuo è esattamente il residuo di [DEBT-013], che il punto 1
misura. Il pavimento `reward_epoch_ms_min` di [SPEC-009] passa così a mordere sull'indice e non
solo sulla durata dichiarata, che è il livello che [REVIEW-014] non guardava.

**Il verso opposto** — indice che non avanza — **non è chiudibile da una regola e lo scrivo
così**: nessuna regola interna può obbligare un quorum a mintare; una regola rifiuta un atto,
non ne impone uno. È chiuso nella stessa forma dei punti 1 e 2: `settleable_reward_epoch` e
`reward_epoch_lag` rendono il ritardo **ricalcolabile** dalle intestazioni e dai mint
finalizzati. Una scadenza di liquidazione è stata valutata e **non adottata**: non
obbligherebbe comunque a mintare, trasformerebbe un'interruzione onesta in reddito perso
per sempre, e il limite cumulativo vale già anche se un arretrato viene liquidato in blocco.

**5. [DEBT-014], rifiuto motivato (punto 5).** Due paragrafi, in `README.md` accanto al
registro delle preimmagini e in `ledger.md` accanto alla formula, più il commento in
`registry.rs` e in `hash.rs`. La ragione scritta è quella corretta: i byte di un `ValidatorSet`
legano già la catena tre volte — `election_seed` ed `election_ticket` attraverso `chain_id_32`,
e ogni `key_binding_signature` attraverso la procedura di firma legata alla catena; il set di
genesi, l'unico senza `election`, porta comunque i key binding. **L'argomento registrato nel
debito — «è una lista di chiavi, legarla impedirebbe di riusarla in una genesi nuova» — non
compare da nessuna parte**, perché è falso: ogni `key_binding_signature` andrebbe riemessa
comunque.

**Il superlativo del debito era falso ed è stato corretto invece che copiato.** «L'unica
preimmagine a dominio separato non legata a `chain_id`» non regge: sei altre lo omettono —
`chain_id` stesso, `node_id`, le due derivazioni di `account_key`, `object_id`, `input_hash` e
`dht_namespace_key` — ciascuna per una ragione propria, e per `object_id` e `input_hash`
l'indipendenza dalla catena è **richiesta**, perché sono indirizzi di contenuto. Verificato per
esaurimento su tutte le formule di `docs/protocol/`. Il documento scrive la classe vera:
*una preimmagine su un oggetto di consenso specifico della catena che altri oggetti di consenso
nominano per hash*, e in quella classe `validator_set_hash` è l'unica eccezione.

**6. Punto 3 non eseguito.** Vedi *Deviations*.

### Files changed

- `core/coblox-core/src/cadence.rs` — **nuovo**. Le due misure, la banda applicata, la
  derivazione di `reward_epoch`, le due osservabili del verso opposto.
- `core/coblox-core/src/params.rs` — `CadenceBand` e `CadenceBand::validate`.
- `core/coblox-core/src/error.rs` — `Error::Cadence(CadenceError)` e le sue otto varianti.
- `core/coblox-core/src/lib.rs` — registrazione del modulo e riga nella tabella dei moduli.
- `core/coblox-core/src/registry.rs`, `src/hash.rs` — la ragione dell'eccezione di [DEBT-014].
- `core/coblox-core/tests/cadence_and_reward_epoch.rs` — **nuovo**, 13 test.
- `docs/protocol/README.md` — §Genesis constants riscritta; §Cadence band nuova;
  §Trust anchors nell'elenco di ciò che una distribuzione deve portare; procedura di rilascio
  in §Weak subjectivity checkpoint; paragrafo di [DEBT-014] nel registro delle preimmagini;
  voce della banda nella lista DRAFT.
- `docs/protocol/ledger.md` — §Block format riscritta; §`reward_epoch` is derived from height
  nuova; paragrafo di [DEBT-014] accanto alla formula; passo 4b dell'algoritmo del light client.
- `sim/tools/published_artifacts.toml` — probe `block-interval-debt-link` sostituita da
  `block-interval-no-rule-can-impose-it`, più sette probe nuove sui passaggi normativi nuovi.

### Verification performed

- `cargo test --workspace --all-features`: **147 passati**, 0 falliti (baseline 126, +21).
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: pulito.
- `cargo fmt --check`: pulito.
- `python sim/tools/published_artifacts.py`: PASS, 91 probe C10 (erano 84).
- `python sim/tools/published_artifacts_negative.py`: PASS, 10 classi ciascuna osservata fallire.
- `python sim/tools/protocol_hashes.py`: PASS, metodo validato prima su due valori non
  modificati. **Nessun valore pubblicato è cambiato in questa passata**: nessuna preimmagine
  nuova, nessuna fixture toccata, e `CadenceBand` non entra in alcun hash.
- `python sim/tools/reward_rules.py`, `python sim/tools/non_consensus_containment.py`: PASS.
- Quattro prove in negativo su albero copiato, mai sull'albero condiviso — vedi trascrizione.

### Verification transcript

```text
$ cargo test --workspace --all-features   # baseline, prima di ogni modifica
TOTAL PASSED: 126

=========================== GATE-MEASURE-BINDS ===========================
$ cargo run -p coblox-core --example gate_transcript
band: interval 5000 ms/block, accepted 2500..=10000 ms/block, floor 100 blocks

  IN BAND     1000 blocks in 5 000 000 ms  (5 000 ms/block)
     verdict        : WithinBand { blocks: 1000, elapsed_ms: 5000000, observed_ms_per_block: 5000 }
     light client   : Ok("proceeds")
     release process: Ok("issues checkpoint")
  OUT (fast)  1000 blocks in 1 000 000 ms  (1 000 ms/block)
     verdict        : FasterThanBand { blocks: 1000, elapsed_ms: 1000000, observed_ms_per_block: 1000 }
     light client   : Err(Cadence(FasterThanBand { blocks: 1000, elapsed_ms: 1000000, observed_ms_per_block: 1000 }))
     release process: Err(Cadence(FasterThanBand { blocks: 1000, elapsed_ms: 1000000, observed_ms_per_block: 1000 }))
  OUT (slow)  1000 blocks in 40 000 000 ms (40 000 ms/block)
     verdict        : SlowerThanBand { blocks: 1000, elapsed_ms: 40000000, observed_ms_per_block: 40000 }
     light client   : Ok("proceeds")
     release process: Err(Cadence(SlowerThanBand { blocks: 1000, elapsed_ms: 40000000, observed_ms_per_block: 40000 }))
  UNMEASURED    50 blocks in   250 000 ms
     verdict        : Inconclusive { blocks: 50, min_measured_blocks: 100 }
     light client   : Ok("proceeds")
     release process: Err(Cadence(Inconclusive { blocks: 50, min_measured_blocks: 100 }))

--- la stessa misura provata in negativo, su albero copiato ---
DEFECT A: il confronto con la banda divide prima di confrontare
$ cargo test -p coblox-core --lib cadence
---- cadence::tests::the_band_comparison_does_not_divide_first stdout ----
thread '...' panicked at core\coblox-core\src\cadence.rs:455:9:
assertion failed: matches!(measure(100, 1_000_099, &b), CadenceVerdict::SlowerThanBand { .. })

DEFECT B: il light client smette di fallire chiuso sul lato veloce
$ cargo test -p coblox-core --test cadence_and_reward_epoch
test a_chain_faster_than_its_band_fails_closed_for_both_parties ... FAILED
test result: FAILED. 11 passed; 1 failed

=========================== GATE-BOTH-DIRECTIONS ===========================
(stessa esecuzione dell'esempio; epoca di un giorno = 17 280 blocchi)

  VERSO 1 - indice troppo veloce: un incremento per blocco,
            mint(reward_epoch=42) all'altezza 42
     -> Err(Cadence(RewardEpochAhead { reward_epoch: 42, height: 42 }))
  al pavimento: mint(reward_epoch=17) all'altezza 311 040
     -> Ok("valid")
  un blocco sotto: mint(reward_epoch=17) all'altezza 311 039
     -> Err(Cadence(RewardEpochAhead { reward_epoch: 17, height: 311039 }))

  VERSO 2 - indice fermo: catena all'altezza 3 456 000, ultima epoca liquidata 3
     settleable now : Some(199)
     lag            : 196 epoche non liquidate oltre il loro pavimento
     never minted   : lag 200 epoche
     (nessuna regola lo rifiuta: una regola rifiuta un atto, non ne impone uno)

--- provata in negativo, su albero copiato ---
DEFECT C: il pavimento di liquidazione smette di rifiutare un indice in anticipo
$ cargo test -p coblox-core --test cadence_and_reward_epoch
test an_index_that_advances_too_fast_is_invalid ... FAILED
test shortening_the_declared_epoch_shortens_the_floor_and_the_reward_bounds_floor_holds_it ... FAILED
test result: FAILED. 10 passed; 2 failed

========================= GATE-NO-TIMESTAMP-RULE =========================
$ git diff -- docs core sim | grep "^+.*timestamp_ms"    # righe AGGIUNTE
+/// is about `timestamp_ms`, and that omission is the substance of [ADR-013]
+which is why a rule on the distance between consecutive `timestamp_ms` values is
+`timestamp_ms` is not an input to either measurement, and MUST NOT become one.
+a rule on the distance between consecutive `timestamp_ms` values is **rejected**
+   `timestamp_ms` is **not** an input to this step and MUST NOT be used in it,
+   on the distance between consecutive `timestamp_ms` values is rejected
+pattern = '`timestamp_ms` is not an input to either measurement, and MUST NOT become one\.'
+why = "[ADR-013] rejects a validity rule on the distance between consecutive timestamp_ms ..."
+pattern = '`timestamp_ms` is \*\*not\*\* an input to this step and MUST NOT be used in it'

$ git diff -- docs core sim | grep "^-.*timestamp_ms"    # righe RIMOSSE
(nessuna)

$ grep -n "timestamp_ms" core/coblox-core/src/cadence.rs \
      core/coblox-core/tests/cadence_and_reward_epoch.rs   # file nuovi
cadence.rs:8,10,13,114,159            -> tutte in commenti di documentazione
cadence_and_reward_epoch.rs:10,11,15,332,333,350,351 -> il divieto e il suo test

$ git diff -- docs | grep "^+.*MUST"    # ogni MUST nuovo, riletto uno per uno
... nessuno riguarda la distanza fra timestamp consecutivi ...

DEFECT D: l'orologio della catena entra nella misura (senza cambiare la firma)
$ cargo test -p coblox-core --test cadence_and_reward_epoch
---- the_cadence_module_never_reads_a_chain_written_clock stdout ----
thread '...' panicked at core\coblox-core\tests\cadence_and_reward_epoch.rs:349:5:
`timestamp_ms` appeared in executable code of src/cadence.rs. It is written by the
validators whose cadence this module measures; using it would be [ADR-013]'s rejected
rule reintroduced through the measurement instead of through a validity rule.
test result: FAILED. 11 passed; 1 failed
$ # ripristinato
test result: ok. 12 passed; 0 failed

=============================== GATE-ADR012 ===============================
$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     16 candidate(s) checked
  C4-VALUE          51 candidate(s) checked
  C5-MIRROR         42 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C10-PROBE         91 candidate(s) checked
published-artifact inventory: PASS

  Prima esecuzione, PRIMA di aggiornare il manifesto - la gate ha trovato
  qualcosa e non e passata a vuoto:
  FAIL C10-PROBE: probe 'block-interval-debt-link' expected 1 match(es) of
  '\[DEBT-013\]' in README.md, found 0. The unenforced cadence must stay linked
  to the debt that owns it; SPEC-010 was not permitted to close it.
  published-artifact inventory: FAIL (1 finding(s))

$ python sim/tools/published_artifacts_negative.py
negative proof: PASS - 10 defect classes, each observed failing

$ python sim/tools/protocol_hashes.py
  empty revocation_root H(0x33) MATCH      <- metodo validato su valori
  revocation_leaf REVL-0       MATCH          non modificati da questa passata
  account_key (app) APP-0      MATCH
  app_leaf APP-0               MATCH
every published value reproduced: PASS
(nessun valore pubblicato e cambiato in questa passata)

$ python sim/tools/reward_rules.py
cases: 58, mismatches: 0
GATE-RULES-REJECT: PASS

$ python sim/tools/non_consensus_containment.py
ok  `from_raw_bytes_non_consensus` is named only in ... PASS

============================== suite finale ==============================
$ cargo fmt --check
FMT OK
$ cargo clippy --workspace --all-targets --all-features -- -D warnings
    Finished `dev` profile
$ cargo test --workspace --all-features
TOTAL PASSED: 147   (failing suites: 0)
```

### Deviations from the specification

**1. Il punto 3 della proposta tecnica non è stato eseguito, ed è un disaccordo motivato.**
La spec chiede «il secondo limite in millisecondi di catena, accanto a quello in blocchi, per
le quantità che portano una promessa in tempo reale — il limite di mandato per primo».

I millisecondi di catena sono `timestamp_ms`, e `timestamp_ms` è scritto dai validatori con il
solo vincolo della mediana degli undici, che impone monotonia e non passo. Un limite di mandato
in millisecondi di catena si evade scrivendo incrementi di timestamp piccoli a piacere — cioè
esattamente la manovra che la regola respinta da [ADR-013] non impedisce. Vincolerebbe la
grandezza **nominata** (millisecondi di catena) e non quella da cui la proprietà dipende
(tempo reale trascorso): è la famiglia 3 di `recurring-defects.md`, commessa dentro il rimedio
che esiste per non commetterla. La spec stessa lo registra come rischio secondario; la mia
valutazione è che non sia un rischio ma la conclusione.

L'intento del punto 3 — dare al limite di mandato un significato in tempo reale — **è
soddisfatto dal punto 1**, che è già denominato in tempo reale e che due checkpoint traducono
in giorni. Non toccare `ElectionBounds` ha inoltre il beneficio di non muovere la taratura di
[SPEC-007] e di non aprire una superficie di ricalcolo.

**2. Una conseguenza nuova che il debito e la ADR non prevedevano, e che ha cambiato una
scelta di progetto.** [DEBT-013] e [ADR-013] dichiarano entrambi che la direzione del pericolo
è il **rallentamento**, e che l'accelerazione «accorcia tutto e favorisce il ricambio», cioè è
benigna. Con `reward_epoch` derivato da `height` **non lo è più**: accelerare la produzione
moltiplica l'emissione reale. Per questo la banda è **a due lati** e il lato veloce è quello
che fallisce chiuso lato client. Se il Lead ritiene che questo modifichi la valutazione di
AGENT-007, va detto a lei prima della review.

**3. I valori della banda non sono stati scelti.** `min_ms_per_block`, `max_ms_per_block` e
`min_measured_blocks` sono istruiti nella lista DRAFT di `README.md` con cosa comporta ciascun
ordine di grandezza sui due lati, e **portati all'operatore** come `α` e la popolazione al
lancio. I numeri che compaiono nei test sono etichettati come input di prova.

**4. `sim/coblox_sim/recommended.py` non è stato toccato.** `BLOCK_INTERVAL_SECONDS = 5
# assumption` è dichiarato una conseguenza di [ADR-013], ma la spec limita `sim/coblox_sim/`
al caso in cui il punto 3 muova la taratura, e il punto 3 non è stato eseguito. Resta aperto e
va segnalato al Lead.

**5. Le due liste chiuse di `ledger.md#what-a-light-client-can-establish-about-set-composition`
non sono state toccate.** La cadenza misurata è una grandezza che il client calcola e non un
fatto di composizione che stabilisce; il passo 4b lo dice in prosa e si accosta alle otto voci
di `CANNOT_ESTABLISH` senza entrarci. Aggiungere una voce a una lista che la specifica tratta
come chiusa non è lavoro di questa spec.

---

## Remediation, giro 1 (2026-08-26)

### RF-001 — `SECURITY.md` fuori dall'inventario, e la pretesa che ne era rimasta indietro

**La causa, prima del rimedio.** `sim/tools/published_artifacts.py` costruiva il proprio
insieme di documenti da `docs/protocol/*.md` e da nient'altro. `SECURITY.md` — che GitHub
pubblica dalla scheda *Security*, cioè il primo artefatto che un ricercatore esterno legge —
non è mai stato guardato da questa gate. La passata è quindi passata verde su 91 probe mentre
un artefatto pubblicato portava una dichiarazione di limite diventata incompleta. **La gate
misurava l'insieme sbagliato**: è la famiglia 3 rivolta alla gate stessa.

**1. `SECURITY.md` entra nell'inventario, come classe propria.** Non fra `meta.documents` ma
come `meta.claim_documents`, ed è una distinzione di sostanza e non di comodo: il documento
porta **pretese e non artefatti** — nessun digest, nessuna stringa di dominio, nessun tag byte,
nessun identificatore di fixture — quindi le cinque classi di scoperta meccanica non hanno
nulla da trovarvi. Farvele girare sopra produrrebbe falsi positivi immediati, perché
`` `ADR-007` `` e `` `DEBT-013` `` corrispondono alla regex degli identificatori di fixture, e
un falso positivo è il modo in cui [ADR-012] precisazione 3 registra la fine di una guardia.
È quindi spazzato per le probe C10 e per i conteggi derivati.

**2. `C11-CLAIMDOC`, perché quella decisione non decada.** Una classe nuova che fallisce se un
documento di pretese **acquista** un digest, una stringa di dominio o un tag byte. Il giorno in
cui accade, il trattamento più stretto è esattamente il difetto di RF-001 di nuovo, e il tool lo
dice invece di continuare a misurare l'insieme più piccolo. La stessa classe verifica che
l'insieme dei documenti di pretese su disco coincida con quello dichiarato, nei due versi, come
già fa `C6-ORPHAN` per i documenti di protocollo.

**3. Il paragrafo sulla cadenza, riscritto.** Ora nomina **entrambe le direzioni** e non le
scambia: *stretching* allunga in tempo reale tutto ciò che il protocollo denomina in blocchi —
incumbency e ritardo effettivo di revoca; *compressing* moltiplica l'emissione reale, perché
l'indice di epoca è derivato da `height`. Dice cosa v0 **ha** — la misura contro una banda a due
lati fissata nella distribuzione di genesi, e il fallimento chiuso dove il punto di osservazione
lo rende solido — e chiude con «The manoeuvre is not prevented», che è la frase che impedisce al
paragrafo di promettere più di quanto il codice contiene. `DEBT-013` resta citato come la sede
dell'analisi, non come un lavoro pendente.

**4. Il resto di `SECURITY.md`, guardato riga per riga. Ho trovato altre due imprecisioni, e non
erano nel paragrafo della cadenza.** L'ultima sezione dichiarava che il threat model porta
«36 scenari, 24 requisiti di sicurezza numerati e 15 test di attacco». Contati alla fonte:
**39** scenari (`TM-01`…`TM-39`), **26** requisiti (`SEC-REQ-01`…`SEC-REQ-26`), 15 test — quindi
due numeri su tre erano fermi a una versione precedente del documento che citano. Corretti.

Ma pinnarli come probe avrebbe comprato una sola modifica di tregua, perché è la trascrizione a
mano il difetto e non il numero: è la forma che [SPEC-012] ha chiuso facendo **estrarre** la
tabella dal documento invece di trascriverla. I tre conteggi sono quindi **ricalcolati dalla
fonte** a ogni esecuzione (`[[claim_count]]`), compreso quello che oggi è giusto — un dato
corretto e non provato è a una modifica di distanza dall'essere il prossimo dato fermo.

**5. Due osservazioni che ho lasciato al Lead invece di agire.** (a) Il paragrafo anti-Sybil è
oggi **più debole di quanto le regole sostengano**: [ADR-010] registra che, chiuse le tre
disposizioni di [SPEC-009], l'affermazione «una flotta non aumenta l'emissione totale» passa da
vera-a-condizione a vera-per-regola, e le tre disposizioni sono in albero (`RewardBounds`,
tariffa di availability zero, `3 * min_set >= 2 * V`, quest'ultima verificata da
`reward_rules.py`). Non l'ho rafforzato: rendere **più forte** una pretesa di sicurezza in
`SECURITY.md` non è lavoro che un implementatore debba fare da solo, ed è la superficie su cui
questo progetto ha già sbagliato cinque volte. Va ad AGENT-007. (b) «Milestone M-01 covers the
protocol on paper» si legge come stato corrente mentre il lavoro è in M-02; non è
un'imprecisione causata da una regola, quindi non l'ho toccata.

**6. La prova in negativo, e non solo per le probe nuove.** [ADR-012] chiede che ogni guardia sia
provata in negativo, e la prova esistente copriva **una** probe su 91: dimostrava che *una* C10
può fallire e nulla sulle altre novanta. Una probe il cui pattern è stato scritto contro un testo
poi riscritto continua a essere valida, a eseguire e a passare — ha solo smesso di pinnare
qualcosa. `prove_every_probe` cancella quindi da ciascun documento il passaggio che la **sua**
probe dichiara di pinnare, ed esige che il tool fallisca **nominandola per id**. Tutte e 98 sono
state osservate fallire, le sette nuove comprese. Più due mutazioni nuove per `C11-CLAIMDOC`.

### RF-002 — la riga di [DEBT-013] resa falsa

Non toccata, su istruzione del Lead. `.lmbrain/debts/open/DEBT-013-*.md` dice che «blocchi più
veloci accorciano tutto e favoriscono il ricambio»: con `reward_epoch` derivato da `height`
quella frase è **falsa** e non incompleta, perché comprimere moltiplica l'emissione reale. La
correzione la porta il Lead e la conferma AGENT-007 in `GATE-SECREVIEW`. Registrata qui perché
resti nell'evidenza di questa spec.

### File toccati dalla remediation

- `SECURITY.md` — paragrafo sulla cadenza riscritto nei due versi; due conteggi corretti.
- `sim/tools/published_artifacts.py` — `CLAIM_DOCS`, `claim_documents()`,
  `check_claim_documents()` con la classe `C11-CLAIMDOC` e i conteggi derivati; docstring e
  elenco di ciò che il tool non copre.
- `sim/tools/published_artifacts.toml` — `meta.claim_documents`, sette probe su `SECURITY.md`,
  tre `[[claim_count]]`.
- `sim/tools/published_artifacts_negative.py` — `COPIED_FILES`, `prove_every_probe`, due
  mutazioni `C11-CLAIMDOC`, docstring, e la riga di riepilogo che ora distingue le mutazioni
  dalle classi invece di confonderle.

### Trascrizione della remediation

```text
--- lo stato PRIMA del rimedio, che e il finding ---
$ grep -n "SECURITY" sim/tools/published_artifacts.toml sim/tools/published_artifacts.py
(nessun risultato)
$ grep -oE "\bTM-[0-9]+" .lmbrain/knowledge/threat-model.md | sort -u | wc -l
39                      # SECURITY.md dichiarava 36
$ grep -oE "SEC-REQ-[0-9]+" .lmbrain/knowledge/threat-model.md | sort -u | wc -l
26                      # SECURITY.md dichiarava 24
$ grep -oE "\bAT-[0-9]+" .lmbrain/knowledge/threat-model.md | sort -u | wc -l
15                      # SECURITY.md dichiarava 15 - corretto, e ora provato

--- la passata in avanti, dopo ---
$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     16 candidate(s) checked
  C4-VALUE          51 candidate(s) checked
  C5-MIRROR         42 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C10-PROBE         98 candidate(s) checked     <- 91 prima, +7 su SECURITY.md
  C11-CLAIMDOC       4 candidate(s) checked     <- classe nuova
published-artifact inventory: PASS

--- ogni probe provata singolarmente, non solo la classe ---
$ python sim/tools/published_artifacts_negative.py
=== control: the unmutated copy ===
published-artifact inventory: PASS

=== C10-PROBE, every probe individually ===
deleting each probe's own pinned passage from its own document, 98 case(s)
  every one of the 98 probes was observed failing

=== C11-CLAIMDOC ===
defect reintroduced: SECURITY.md grows a digest literal, so the probe-only treatment it
is given has stopped being the right one and the sweep would otherwise keep measuring the
smaller set - [SPEC-016] RF-001 exactly
  FAIL C11-CLAIMDOC: SECURITY.md now carries a digest literal
  ('sha256:993b24bf6115fbf5651d615ca57a1baa825baf304b1dcc4d52debbc7fa3bd6d8'). It is swept
  for C10 probes only, which was right while it carried claims and no artifacts. Promote it
  to meta.documents and to the five discovery classes, or remove the artifact.
  exit=1 names C11-CLAIMDOC: True

=== C11-CLAIMDOC ===
defect reintroduced: a count SECURITY.md transcribes from the threat model drifts away
from it, which is how it came to claim 36 scenarios against 39
  FAIL C11-CLAIMDOC: claim count 'threat-model-scenarios': SECURITY.md claims 39 but
  .lmbrain/knowledge/threat-model.md carries 40 distinct '\bTM-\d+\b'. [...]
  exit=1 names C11-CLAIMDOC: True

negative proof: PASS - 12 mutations across 11 defect classes, plus every probe
individually, each observed failing

--- le sette probe nuove, ciascuna osservata fallire dentro la passata sopra ---
security-sybil-not-cryptographic              cancellata la sua frase -> FAIL nominandola
security-sybil-is-economic-not-cryptographic  idem
security-quorum-four-ninths                   idem
security-owning-is-not-controlling            idem
security-cadence-is-measured-not-enforced     idem
security-cadence-both-directions              idem
security-cadence-not-prevented                idem

--- il resto, invariato ---
$ python sim/tools/protocol_hashes.py
every published value reproduced: PASS
$ python sim/tools/reward_rules.py
cases: 58, mismatches: 0 / GATE-RULES-REJECT: PASS
$ cargo fmt --check
FMT OK
$ cargo clippy --workspace --all-targets --all-features -- -D warnings
    Finished `dev` profile
$ cargo test --workspace --all-features
TOTAL PASSED: 147   (invariato: la remediation non tocca il crate)
```

### Handoff status
- [x] Ready for Project Lead review
