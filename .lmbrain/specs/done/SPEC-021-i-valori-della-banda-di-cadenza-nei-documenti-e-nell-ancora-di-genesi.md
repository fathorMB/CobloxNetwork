---
id: SPEC-021
# Note: Quote the title if it contains a colon
title: "I valori della banda di cadenza nei documenti e nell'ancora di genesi"
status: done
kind: feature
priority: high
area: consensus
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
capability_tier: sol
thinking_level: standard
effort_observations: []
depends_on: [SPEC-016]
dependency_events: []
parking_events: []
skills: [SKILL-001, SKILL-002, SKILL-003]
verification_gates: []
related_decisions: [ADR-012, ADR-013, ADR-016]
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [governance, light-client]
activity:
  - date: 2026-08-26
    action: "transitioned backlog -> ready"
  - date: 2026-08-26
    action: "transitioned ready -> working"
  - date: 2026-08-26
    action: "transitioned working -> review"
  - date: 2026-08-26
    action: "transitioned review -> done"
---
# I valori della banda di cadenza nei documenti e nell'ancora di genesi

## Objective

Scrivere nei documenti di protocollo e nell'ancora di fiducia di genesi i valori della `CadenceBand` che [ADR-016] ha deciso, e togliere la voce corrispondente dalla lista DRAFT dei parametri di lancio.

**È la spec più piccola della coda e quella con la superficie di errore più alta**, perché ciò che scrive non è un meccanismo ma **cinque numeri e ciò che quei numeri costano**. I numeri sono già decisi: il lavoro è la seconda metà.

## Context

[SPEC-016] ha chiuso [DEBT-013] rendendo la cadenza reale **misurabile e dichiarata**, non impedita. La tolleranza di quella misura era l'unica cosa rimasta aperta ed è una decisione dell'operatore, presa il 2026-08-26 e registrata in [ADR-016]:

| Campo | Valore |
| --- | --- |
| `block_interval_ms` | `5000` |
| `min_ms_per_block` | `2500` |
| `max_ms_per_block` | `20000` |
| `min_measured_blocks` | `720` |
| `max_external_clock_slack_ms` | `600000` |

I vincoli sono già regole scritte in `docs/protocol/README.md` e vanno verificati, non riscritti: `min ≤ block_interval_ms ≤ max`, tutti positivi, e `max_external_clock_slack_ms < min_measured_blocks × block_interval_ms`.

**La banda vive nella distribuzione firmata e in nessun altro canale.** Non è modificabile da alcun documento on-chain, non va appresa da un peer, da un'intestazione o da un documento di protocollo. La ragione è scritta ed è la parte che non va persa: *una banda che un quorum seduto potesse allargare sarebbe una tolleranza sotto l'unica misura che il protocollo ha del comportamento di quel quorum.*

**Le conseguenze dei due lati non si scambiano fra loro**, e [ADR-016] le dichiara:

- il **lato lento**, `4 × interval`, significa che un set attivo può **stirare le proprie epoche fino a quattro volte** prima che qualcosa lo dica. Le garanzie anti-cattura di [SPEC-006] restano vere in **epoche**; la loro traduzione in giorni dipende da chi le epoche le produce;
- il **lato veloce**, `interval / 2`, obietta a un **raddoppio dell'emissione reale**, ed è il lato su cui il client **fallisce chiuso**.

## Scope

### Included

- I cinque valori in `docs/protocol/README.md`, al posto della voce DRAFT.
- I cinque valori nell'ancora di fiducia di genesi.
- **Le conseguenze dei due lati scritte accanto ai valori**, nella forma dichiarata da [ADR-016] e non riassunta.
- La rimozione della **sola** voce della banda dalla lista DRAFT, con le altre tre intatte.
- Una fixture di frontiera sulla regola relazionale `slack < min_measured_blocks × block_interval_ms`.
- La passata di [ADR-012].

### Excluded

- **Qualunque modifica alla misura, alla sua asimmetria o alla procedura di rilascio.** [SPEC-016] le ha chiuse e [REVIEW-027] le ha riviste dopo un finding `high`: qui si scrivono valori, non si tocca il meccanismo.
- **Qualunque modifica ai valori decisi.** Se un vincolo non fosse soddisfatto, è una decisione del Lead e dell'operatore, non una correzione da fare passando.
- Le altre tre voci della lista DRAFT.

## Existing-project analysis

`docs/protocol/README.md` porta lo schema di `CadenceBand`, le sue regole di validità, e la voce DRAFT che istruisce la scelta con il costo di ciascun ordine di grandezza **sui due lati**. Quella prosa istruttiva è ciò che ha permesso all'operatore di decidere, e **non va cancellata insieme alla voce**: va trasformata da istruzione a scelta in giustificazione della scelta fatta.

`core/coblox-core/src/params.rs` porta `CadenceBand` e la sua validazione, chiamata come primo atto da entrambe le misure di `cadence.rs`.

## Technical proposal

**Scrivere i valori è metà del lavoro. L'altra metà è ciò che i valori costano, e va scritta accanto a loro.**

Un lettore che trova una banda conclude che la cadenza sia **limitata**. Non lo è: la banda la rende **misurabile**, e [SPEC-016] ha già dovuto scrivere in `README.md` che *«No rule of this protocol prevents that, and none can»*. Quella frase è una probe della gate e va lasciata dov'è; ciò che questa spec aggiunge non deve contraddirla né attenuarla.

La conseguenza del lato lento — **quattro volte** — va scritta come numero e non come qualità. «Il set può allungare le proprie epoche» è vero e non dice quanto; `4 ×` sì, e traduce le nove epoche del mandato massimo in una grandezza che un lettore può confrontare con la propria attesa.

## Files and areas involved

- `docs/protocol/README.md` — i valori, le conseguenze, la voce DRAFT, la fixture.
- `core/coblox-core/` — l'ancora di genesi e la fixture di frontiera.
- `sim/tools/` — la gate di [ADR-012] e le probe nuove.

## Acceptance criteria

- [x] I cinque valori di [ADR-016] sono in `README.md` e nell'ancora di genesi, e coincidono con la ADR.
- [x] I tre vincoli relazionali sono **verificati sui valori scritti**, non riscritti.
- [x] **Le conseguenze dei due lati sono scritte accanto ai valori**, con il lato lento espresso come `4 ×` e il lato veloce come raddoppio dell'emissione.
- [x] La frase *«No rule of this protocol prevents that, and none can»* è **ancora là e non attenuata**.
- [x] La lista DRAFT ha perso **una sola** voce e ne conserva tre.
- [x] Una fixture esercita la frontiera di `slack < min_measured_blocks × block_interval_ms` **da entrambi i lati**.
- [x] Nessun valore pubblicato preesistente è cambiato, oppure ogni cambio è ricalcolato con il metodo validato prima su un valore non modificato.
- [x] La gate di [ADR-012] è eseguita e la trascrizione allegata.

## Implementation plan

1. Verificare i tre vincoli sui valori decisi, **prima** di scrivere. Se uno non regge, fermarsi.
2. Scrivere valori e conseguenze insieme, mai i valori da soli.
3. L'ancora di genesi e la fixture di frontiera.
4. Togliere la voce DRAFT, conservando la prosa istruttiva come giustificazione.
5. Gate di [ADR-012] e probe nuove, ciascuna provata in negativo.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-COST-BESIDE-VALUE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Ogni valore scritto ha **accanto** ciò che costa, e il lato lento è espresso come `4 ×`. Una banda scritta senza il suo costo fa concludere a un lettore che la cadenza sia limitata, mentre è solo misurabile: è la famiglia 2, la pretesa avanti rispetto alla regola, su un documento che parla di ciò che il protocollo **non** impedisce.
- [x] GATE-RENUNCIATION-INTACT | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La frase *«No rule of this protocol prevents that, and none can»* è ancora nel documento, **non attenuata e non spostata sotto i valori nuovi**. È già una probe di [ADR-012]: la gate qui verifica che il testo attorno non la contraddica, cosa che una probe non può vedere.
- [x] GATE-DRAFT-MINUS-ONE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La lista DRAFT dei parametri di lancio ha perso **esattamente una** voce e ne conserva tre, contate prima e dopo. Una lista che si accorcia più del dovuto perde una decisione aperta senza che nessuno se ne accorga.
- [x] GATE-RELATION-BOUNDARY | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La frontiera di `slack < min_measured_blocks × block_interval_ms` è esercitata **da entrambi i lati**: il valore massimo ammesso passa, il primo valore oltre è rifiutato. Una regola relazionale provata da un lato solo non distingue `<` da `≤`.
- [x] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato e la trascrizione allegata, **anche se non trova nulla**. Ogni probe nuova entra nel conteggio individuale della prova in negativo.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è scrivere i numeri e non il loro costo.** È la metà facile e visibile; l'altra è quella per cui la spec esiste. Se al termine i cinque valori sono nel documento e un lettore ne conclude che la cadenza è limitata, questa spec ha fatto danno invece che lavoro.
- **Il rischio secondario è cancellare la prosa istruttiva insieme alla voce DRAFT.** Quella prosa è ciò che ha permesso la decisione, e diventa la sua giustificazione: si trasforma, non si toglie.
- **Un rischio terzo, e va nominato perché sembra un miglioramento:** rendere la banda leggibile da un documento di consenso «per comodità di configurazione». Sarebbe la cosa esatta che il divieto esiste per impedire, ed è la forma che un implementatore propone in buona fede guardando lo schema.
- `max_external_clock_slack_ms = 600000` è **una scelta sulla latenza attesa e non una misura**: [ADR-016] lo dichiara e va riverificato appena il processo di rilascio esiste. Scriverlo senza quella qualificazione lo trasformerebbe in un dato.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Fermarsi e riportare è un esito previsto**, e qui vale in particolare se uno dei tre vincoli relazionali non fosse soddisfatto dai valori decisi: è una decisione dell'operatore e non una correzione da fare passando.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

**I valori, e dove sono.** I cinque valori di [ADR-016] sono scritti in
`docs/protocol/README.md`, dentro `### Cadence band` — cioe' dentro la sezione
`## Trust anchors` — in una sottosezione nuova `#### The genesis band`. Quella
sezione **e'** l'ancora di fiducia di genesi: nel repo non esiste un artefatto
di ancora separato con valori (nessun JSON di distribuzione, e
`core/coblox-core` dichiara in `lib.rs` che «No launch value appears in this
crate»). Le due voci di scopo della spec — «in `README.md` al posto della voce
DRAFT» e «nell'ancora di fiducia di genesi» — sono quindi la partenza e
l'arrivo dello stesso trasferimento, non due file.

**Le conseguenze, accanto ai valori.** Sotto la tabella, e prima di qualunque
altra cosa, il documento dice che i cinque numeri **non limitano la cadenza** e
rimanda a `#genesis-constants`. Poi un paragrafo per lato:

- lato lento, `4 * block_interval_ms`: «An active validator set can stretch its
  own epochs to **four times** their declared real-time length before any
  measurement says so», con le garanzie anti-cattura dichiarate vere **in
  epoche** e la loro traduzione in giorni attribuita a chi le epoche le produce;
- lato veloce, `block_interval_ms / 2`: «admits a real issuance rate up to
  **twice** the intended one and refuses beyond it», ed e' il lato su cui il
  client fallisce chiuso.

**La prosa istruttiva della voce DRAFT e' stata trasformata, non cancellata.**
Sono rientrati nella giustificazione: l'avvertimento su `2 × block_interval_ms`
che chiama fuori banda una rete in partizione ordinaria (con l'aggiunta della
conseguenza che [ADR-016] ne trae — il rilascio fallisce chiuso in entrambi i
versi, quindi quel bordo fermerebbe l'emissione dei checkpoint); i `20 ×` che
lasciano raddoppiare i mandati; l'esempio `block_interval_ms / 4` che permette
quattro volte l'emissione; il pavimento di rumore e il suo costo in latenza; e
per intero il paragrafo di dimensionamento di `max_external_clock_slack_ms`
(`L * min_ms_per_block / block_interval_ms`, il sotto-dimensionamento come
scelta dichiarata di fallire chiuso, il rifiuto sopra la finestra con
`min_measured_blocks` da alzare, e il costo). Aggiunta la qualificazione che
[ADR-016] esige: dieci minuti sono **una scelta sulla latenza attesa e non una
misura**, da riverificare quando il processo di rilascio esistera'.

**La banda resta fuori dalla portata di un quorum.** Non e' stato aggiunto alcun
campo, alcuno schema e alcuna regola che renda la banda leggibile da un
documento di consenso. L'ultimo paragrafo della sottosezione lo dice nella forma
utile a chi legge lo schema: «Narrower is a release; wider is not available».

**L'ancora nel crate.** `CadenceBand` in `core/coblox-core/src/params.rs` porta
ora i cinque valori nella sua documentazione, con la nota che arrivano come
configurazione e non sono compilati qui, e con la meta' che costa. Due commenti
che rimandavano a `README.md#draft-governance-selected-launch-parameters` — un
ancoraggio che dopo questa spec non descrive piu' la banda — puntano a
`README.md#the-genesis-band`.

**La fixture di frontiera.** Due test nuovi in
`core/coblox-core/tests/cadence_and_reward_epoch.rs`, su una `genesis_band()`
che porta i valori veri e non input di prova.

### Verification transcript

#### I tre vincoli relazionali, verificati sui valori scritti

Verificati prima di scrivere e poi di nuovo sui valori nel documento, dal test
`the_genesis_band_satisfies_the_rules_the_document_states`:

1. `min_ms_per_block <= block_interval_ms <= max_ms_per_block`:
   `2500 <= 5000 <= 20000`. **Vero.**
2. Tutti positivi: `5000`, `2500`, `20000`, `720`, `600000`. **Vero.**
3. `max_external_clock_slack_ms < min_measured_blocks * block_interval_ms`:
   `600000 < 720 * 5000 = 3600000`. **Vero.**

Nessuno dei tre e' stato riscritto: il test li applica alla banda di genesi e
asserisce anche i due multipli decisi (`interval / 2` e `4 * interval`).

#### GATE-COST-BESIDE-VALUE

`docs/protocol/README.md`, `#### The genesis band`. Ogni valore ha accanto la
sua colonna «Read against the declared interval» (`block_interval_ms / 2`,
`4 * block_interval_ms`, «one hour of chain», e la disuguaglianza scritta per
esteso), e sotto la tabella un paragrafo per ciascuno con il costo. Il lato
lento e' scritto come numero in due punti indipendenti — l'intestazione **The
slow side, `4 * block_interval_ms`, costs four** e la frase «stretch its own
epochs to **four times** their declared real-time length» — ed entrambi sono
probe C10 separate, perche' una riscrittura puo' conservare l'aritmetica e
perdere l'avvertimento.

#### GATE-RENUNCIATION-INTACT

```text
$ grep -n "No rule of this protocol prevents that, and none can" docs/protocol/README.md
114:**No rule of this protocol prevents that, and none can.** Every clock the chain

$ git diff -U0 docs/protocol/README.md | grep -E "^@@"
@@ -1378,6 +1378,80 @@ be measuring the validators with the validators' own clock.
@@ -1555,29 +1628,0 @@ not economic facts and remain open:
```

La frase e' alla riga 114, fuori da entrambi gli hunk: non e' stata toccata, ne'
attenuata, ne' spostata sotto i valori nuovi. Il testo aggiunto la rinforza
invece di contraddirla — «These five numbers do not limit the cadence, and
reading them as a limit is the misreading this section exists to prevent» — e
rimanda esplicitamente a `#genesis-constants`, dove la frase vive. Quella
ripetizione e' a sua volta una probe nuova
(`genesis-band-is-not-a-limit-on-the-cadence`), perche' la probe originale sta
1 200 righe prima e un lettore che arriva alla banda dalla sezione delle ancore
non ci passa mai.

#### GATE-DRAFT-MINUS-ONE

```text
$ python (conteggio delle voci di primo livello fra "## DRAFT: governance-selected
  launch parameters" e "## Reference sources", su HEAD e sull'albero)
BEFORE: 4
   - enrollment `difficulty_bits` and the Argon2id cost profile: benchmar
   - the per-epoch existence fund, work reward curves, hosting prices, an
   - the validator election parameters - epoch length, candidacy close, e
   - the [cadence band](#cadence-band): `min_ms_per_block`, `max_ms_per_b
AFTER: 3
   - enrollment `difficulty_bits` and the Argon2id cost profile: benchmar
   - the per-epoch existence fund, work reward curves, hosting prices, an
   - the validator election parameters - epoch length, candidacy close, e
delta: 1
```

Quattro prima, tre dopo, e le tre superstiti sono le stesse tre di prima. Il
paragrafo di coda della sezione (Project Lead / AGENT-007 / development
network) e' intatto.

#### GATE-RELATION-BOUNDARY, con entrambi i lati della frontiera

Test `the_genesis_window_admits_a_slack_one_millisecond_under_it_and_refuses_the_first_beyond`,
sulla finestra di genesi `720 * 5000 = 3 600 000 ms`:

| Caso | `max_external_clock_slack_ms` | Atteso | Osservato |
| --- | --- | --- | --- |
| valore massimo ammesso | `3 599 999` | accettato | `validate` ritorna `Ok` |
| primo valore oltre | `3 600 000` | rifiutato | `ParameterError::Bounds` |
| banda di genesi vera | `600 000` | accettato | `validate` ritorna `Ok` |
| finestra dimezzata, stesso slack | `3 599 999` con `min_measured_blocks = 360` | rifiutato | errore |

I primi due lati distinguono `<` da `<=`. Il terzo caso e' li' per la domanda
del passo 4 di [SKILL-001]: il test preesistente
`a_band_whose_slack_swallows_its_own_window_is_rejected` prova gia' i due lati,
ma **tiene costanti `min_measured_blocks` e `block_interval_ms`** e varia solo
lo slack, quindi da solo non distingue una regola relazionale da una soglia
costante. Il caso a finestra dimezzata e' la grandezza che mancava.

**Prova in negativo, su una copia dell'albero in
`%TEMP%/.../scratchpad/neg` (l'albero condiviso non e' stato mutato).**

```text
1. VERDE iniziale
   $ cargo test -p coblox-core --test cadence_and_reward_epoch
   test the_genesis_window_admits_a_slack_one_millisecond_under_it_and_refuses_the_first_beyond ... ok
   test the_genesis_band_satisfies_the_rules_the_document_states ... ok
   test result: ok. 19 passed; 0 failed

2. MUTAZIONE 1 - la regola relazionale allentata da `<` a `<=`
   (in params.rs: `>=` diventa `>`)
   test the_genesis_window_... ... FAILED
   panicked at tests/cadence_and_reward_epoch.rs:473:
     a slack equal to the measured window was accepted
   test result: FAILED. 17 passed; 2 failed

3. MUTAZIONE 2 - la finestra sostituita dalla costante 3_600_000, cosi'
   che la frontiera smetta di seguire i propri operandi
   test the_genesis_window_... ... FAILED
   panicked at tests/cadence_and_reward_epoch.rs:488:
     assertion failed: half_window.validate(&chain()).is_err()
   test result: FAILED. 17 passed; 2 failed

4. MUTAZIONE 3 - il lato lento di genesi riportato da 4x a 2x, cioe' la
   proposta che l'operatore ha rifiutato in [ADR-016]
   test the_genesis_band_satisfies_the_rules_the_document_states ... FAILED
   panicked at tests/cadence_and_reward_epoch.rs:439:
     assertion `left == right` failed
       left: 10000
      right: 20000
   test result: FAILED. 18 passed; 1 failed

5. RIPRISTINO e VERDE riverificato
   test result: ok. 19 passed; 0 failed
```

La mutazione 2 e' quella che il test preesistente non avrebbe colto: e' la
risposta al passo 4 di [SKILL-001], e la ragione per cui il terzo caso esiste.

#### GATE-ADR012

Conteggi prima e dopo, per classe:

| Classe | Prima | Dopo |
| --- | --- | --- |
| C1-DOMAIN | 40 | 40 |
| C2-TAG | 24 | 24 |
| C3-FIXTURE-ID | 19 | 19 |
| C4-VALUE | 60 | 60 |
| C5-MIRROR | 53 | 53 |
| C7-COVERAGE | 51 | 51 |
| C8-ENCODING | 1 | 1 |
| C9-EXAMPLE | 1 | 1 |
| C5-DISCOVERED | 67 | 67 |
| C10-PROBE | **116** | **126** |
| C11-CLAIMDOC | 8 | 8 |

Le dieci probe nuove sono tutte C10 e sono elencate qui perche' un conteggio che
sale senza che si sappia di cosa non e' una copertura: `genesis-band-*` per le
cinque righe di valore, per le due frasi del lato lento, per quella del lato
veloce, per la ripetizione della rinuncia, e per la qualificazione dei dieci
minuti come scelta e non misura.

```text
$ python sim/tools/published_artifacts.py
  C1-DOMAIN         40 candidate(s) checked
  C2-TAG            24 candidate(s) checked
  C3-FIXTURE-ID     19 candidate(s) checked
  C4-VALUE          60 candidate(s) checked
  C5-MIRROR         53 candidate(s) checked
  C7-COVERAGE       51 candidate(s) checked
  C8-ENCODING        1 candidate(s) checked
  C9-EXAMPLE         1 candidate(s) checked
  C5-DISCOVERED     67 candidate(s) checked
  C10-PROBE        126 candidate(s) checked
  C11-CLAIMDOC       8 candidate(s) checked

published-artifact inventory: PASS

$ python sim/tools/published_artifacts_negative.py
=== C10-PROBE, every probe individually ===
deleting each probe's own pinned passage from its own document, 126 case(s)
  every one of the 126 probes was observed failing

negative proof: PASS - 15 mutations across 11 defect classes, plus every
probe individually, each observed failing
```

Le dieci probe nuove entrano quindi nel conteggio individuale, non solo nel
totale.

#### Valori pubblicati: nessuno cambiato

Questa spec non ha ricalcolato alcun digest. La coppia *validazione su
invariato -> applicazione al variato* di [SKILL-002] non si applica perche' non
c'e' un variato; la passata e' stata eseguita lo stesso e lo dimostra:

```text
$ python sim/tools/protocol_hashes.py
  account_key (app) APP-0      MATCH
  app_leaf APP-0               MATCH
every published value reproduced: PASS
```

I valori pubblicati che questa spec **avrebbe potuto** toccare e non ha toccato:
tutti i digest di `README.md`, `consensus_parameters_hash`, gli `ER-*`, le
fixture PD-0 e le fixture Ed25519. I cinque numeri della banda non sono digest e
non entrano in alcun preimage: `CadenceBand` non e' un documento firmato e
nessuna regola di validita' della catena lo confronta con qualcosa.

#### Altre gate di repo

```text
$ python sim/tools/lead_claims_check.py        -> lead-claims: PASS
$ python sim/tools/non_consensus_containment.py -> ok
$ cargo clippy --workspace --all-targets -- -D warnings  -> pulito
$ cargo fmt --all --check                                 -> pulito
```

#### Byte di controllo nei documenti riscritti

L'avvertimento di [SPEC-017]: un `\0` interpretato invece che scritto e nessuna
gate lo vede.

```text
docs/protocol/README.md              NUL: 0  altri byte di controllo: 0  UTF-8 ok
sim/tools/published_artifacts.toml   NUL: 0  altri byte di controllo: 0  UTF-8 ok
core/coblox-core/src/params.rs       NUL: 0  altri byte di controllo: 0  UTF-8 ok
core/coblox-core/src/cadence.rs      NUL: 0  altri byte di controllo: 0  UTF-8 ok
core/.../tests/cadence_and_reward_epoch.rs  NUL: 0  altri: 0  UTF-8 ok
```

`README.md` e `published_artifacts.toml` restano a fine riga CRLF uniforme
(nessuna riga mista). Le nuove righe di `README.md` stanno tutte entro 80
colonne.

#### Test

```text
prima:  165 passati
dopo:   167 passati  (+2: the_genesis_band_satisfies_the_rules_the_document_states,
                      the_genesis_window_admits_a_slack_one_millisecond_under_it_and_refuses_the_first_beyond)
falliti: 0
```

### Files changed

- `docs/protocol/README.md` — i cinque valori e le loro conseguenze in
  `#### The genesis band` dentro la sezione `### Cadence band`; la voce della
  banda tolta dalla lista DRAFT (4 voci -> 3).
- `core/coblox-core/src/params.rs` — i cinque valori e la meta' che costano
  nella documentazione di `CadenceBand`.
- `core/coblox-core/src/cadence.rs` — il rimando all'ancoraggio DRAFT sostituito
  con `README.md#the-genesis-band`.
- `core/coblox-core/tests/cadence_and_reward_epoch.rs` — `genesis_band()` e i
  due test nuovi.
- `sim/tools/published_artifacts.toml` — dieci probe C10 nuove.

### Limiti noti e cose non fatte

- **Non e' stato toccato nulla della misura, della sua asimmetria o della
  procedura di rilascio.** `cadence.rs` cambia di un solo commento, e quel
  commento e' un rimando a un ancoraggio di documento che questa spec ha
  spostato.
- **Nessun valore decisto e' stato cambiato**, e i tre vincoli reggono, quindi
  non c'e' stato nulla da fermare e riportare su quel fronte.
- **La banda non e' stata resa leggibile da alcun documento di consenso.**
- `max_external_clock_slack_ms = 600000` resta una scelta e non una misura, ed
  e' scritto cosi'. La riverifica appartiene al momento in cui il processo di
  rilascio esistera'.

## Remediation di review — RF-001 (`low`)

**Il finding.** `core/coblox-core/src/lib.rs` dichiarava il principio «No launch
value appears in this crate» e poi **enumerava** cinque portatori:
`ConsensusParameters`, `ElectionBounds`, `RewardPolicy`, `RewardBounds`,
`EnrollmentParameters`. `CadenceBand` — il sesto, introdotto da [SPEC-016] e
quello i cui valori questa spec ha scritto — compariva zero volte. Terza
occorrenza stanotte della stessa forma: una lista dichiarata invece che
osservata, alla quale l'ultimo arrivato non è stato aggiunto.

**Perché non bastava aggiungerla.** Il paragrafo chiudeva con «in production
these values arrive inside a document a validator quorum signed». Per
`CadenceBand` è falso, e deliberatamente: [ADR-016] stabilisce che vive nella
distribuzione firmata e che nessun documento on-chain può cambiarla.
Aggiungerla all'elenco l'avrebbe portata sotto una frase che la descrive al
contrario — cioè avrebbe scambiato un difetto di omissione con un difetto di
asserzione, che è peggiore perché si legge come vero.

### Come ho classificato `CadenceBand`, e perché

Ho riscritto il paragrafo in **tre classi** invece di una lista, perché ciò che
distingue i portatori non è il fatto di essere configurazione ma **cosa
significa rifiutarli**:

1. **Governed documents** — `ConsensusParameters`, `RewardPolicy`,
   `EnrollmentParameters`. Arrivano dentro un documento firmato da un quorum di
   validatori. Il fallimento di validazione è un errore recuperabile perché
   rifiutare un tale documento è **operazione ordinaria di protocollo**.
2. **Genesis bounds** — `ElectionBounds`, `RewardBounds`. Viaggiano nella
   distribuzione firmata e in nessun altro canale, e **limitano ciò che un
   documento governato può portare**: un documento fuori da essi è rifiutato in
   accettazione.
3. **`CadenceBand`, che non è né l'uno né l'altro.** Viaggia nella distribuzione
   firmata come una genesis bound, **ma non limita nulla che un documento
   porti**: è la tolleranza applicata a una misura i cui due estremi sono fuori
   dalla catena, e nessuna regola di validità del protocollo confronta alcunché
   con essa.

La terza classe esiste perché la distinzione fra 2 e 3 è già scritta nel
protocollo — `README.md#cadence-band` dice *«The cadence band bounds **nothing
any document carries**»* — e perché è esattamente la clausola che rendeva la
lista falsa. Il testo nuovo la nomina invece di aggirarla: dice che la frase di
chiusura della prima classe **è falsa per `CadenceBand`, e deliberatamente**, e
ne dà la ragione di [ADR-016] — una banda che un quorum seduto potesse allargare
sarebbe una tolleranza sotto l'unica misura che il protocollo ha del
comportamento di quel quorum — chiudendo con «A new signed release may narrow
it; nothing on-chain may widen it».

**Ho corretto anche la seconda metà dello stesso difetto**, perché lasciarla
sarebbe stato consegnare un rimedio a metà. Il paragrafo «A trust anchor is
checked before it is trusted» enumerava **due** punti di ingresso composti
(`authenticate_consensus_parameters`, `authenticate_reward_policy`) e ometteva
lo stesso oggetto. `CadenceBand::validate` è già chiamata come primo atto di
entrambe le misure — è il rimedio che [SPEC-016] ha applicato alla forma di
[REVIEW-017] RF-001 — e il testo ora lo dice, nominando
`measure_cadence_from_checkpoint` e `measure_cadence_between_checkpoints`. Era
la stessa lista dichiarata e lo stesso omesso; la segnalo qui perché è una riga
oltre il perimetro letterale del finding.

**Nessun comportamento è cambiato.** La remediation tocca solo documentazione di
modulo: nessuna firma, nessuna regola, nessun valore.

### Verifica della remediation

```text
$ cargo test --workspace          -> 167 passati, 0 falliti (invariato)
$ cargo clippy --workspace --all-targets -- -D warnings  -> pulito
$ cargo fmt --all --check                                 -> pulito
$ cargo doc -p coblox-core --no-deps
  5 warning preesistenti (ElectionRecord, from_raw_bytes_non_consensus x2,
  curve25519-dalek, `derive`), nessuna delle quali riguarda i link nuovi:
  params::CadenceBand, cadence::measure_cadence_from_checkpoint e
  cadence::measure_cadence_between_checkpoints risolvono tutti.

$ python sim/tools/published_artifacts.py        -> PASS, C10-PROBE 126
$ python sim/tools/published_artifacts_negative.py
  every one of the 126 probes was observed failing
  negative proof: PASS
$ python sim/tools/protocol_hashes.py            -> every published value reproduced: PASS

core/coblox-core/src/lib.rs  NUL: 0  CRLF uniforme (186/186)
```

`lib.rs` non è letto da alcuna gate di [ADR-012] (la passata copia
`docs/protocol/`, `sim/tools/` e `core/coblox-core/tests/`), quindi il conteggio
delle probe resta 126 e non doveva salire.

### File cambiati dalla remediation

- `core/coblox-core/src/lib.rs` — le tre classi di portatori, la collocazione di
  `CadenceBand` fuori da entrambe le altre due, e i due punti di ingresso della
  cadenza aggiunti al paragrafo sugli anchor.
