---
id: SPEC-019
# Note: Quote the title if it contains a colon
title: "Cosa significa non revocata per autorizzare una spesa, e la riga che non lo chiede"
status: done
kind: bugfix
priority: high
area: ledger
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-002
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: [SPEC-018]
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-012, ADR-006]
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [security, identity, conformance]
activity:
  - date: 2026-08-26
    action: "transitioned backlog -> ready"
  - date: 2026-08-26
    action: "transitioned ready -> working"
  - date: 2026-08-26
    action: "transitioned working -> review"
  - date: 2026-08-26
    action: "attested verification GATE-SECREVIEW by lead"
  - date: 2026-08-26
    action: "transitioned review -> done"
verification_attestations:
  - actor: "AGENT-LEAD"
    actor_role: "lead"
    evidence_digest: "344396da025669a1b017bb594767644533d95ce742b38d44c5c0bd3b354abcb8"
    evidence_ref: "REVIEW-033"
    id: "SPEC-019-ATTEST-001"
    requirement_digest: "04dcbff39da9a9aa0b26546122cee37217c56971124aea3f4f5b41d19140c38d"
    requirement_id: "GATE-SECREVIEW"
    result: "passed"
    schema_version: "1"
    timestamp: "2026-08-26T13:57:37.447256500+02:00"
---
# Cosa significa non revocata per autorizzare una spesa, e la riga che non lo chiede

## Objective

Chiudere [DEBT-022]. La riga 407 di `docs/protocol/ledger.md` dice *«For a subscription, the key MUST derive `payer_node_id`»*, mentre le tre righe sorelle — 372, 458, 968 — dicono *«the **enrolled, unrevoked** key»*.

Ma la chiusura **non è l'allineamento**, e questa spec esiste per impedire che lo sia. La valutazione di AGENT-007 ha stabilito che **`unrevoked` non è definito da nessuna parte** per l'autorizzazione delle transazioni, e che il documento ne usa **due letture diverse**. Allineare la riga alle sorelle chiude l'asimmetria e **lascia il buco aperto**. La definizione viene prima.

## Context

**L'omissione è una svista e non una deroga, ed è accertato.** Il Lead aveva chiesto se esistesse una ragione per cui un'autorizzazione ricorrente debba sopravvivere alla revoca — per esempio perché l'addebito discende da un consenso passato. La risposta è no: **non esiste in questo protocollo alcun oggetto «abbonamento» con durata** da cui possa discendere alcunché. Ogni addebito è una transazione nuova, con firma fresca e nonce nuovo. Non c'è nulla da onorare.

**L'esito peggiore non è quello che il debito immaginava.** Non è l'addebito indebito né il drenaggio: è un **fork**. Chi legge la riga 407 alla lettera accetta il burn di una chiave revocata; chi generalizza dalle tre sorelle lo rifiuta; e il disaccordo è **sulla validità di un blocco**. Non richiede alcuna chiave rubata. È la famiglia 4 del censimento — clausola normativa che nessun oracolo esercita — nella sua forma peggiore, perché il costo non è un difetto ma una **divergenza di catena**.

Ne discendono due correzioni già registrate nel debito. La motivazione di severità è stata sostituita: fondarla sulla chiave rubata la faceva poggiare su un fallimento a monte, e una gravità che poggia su un fallimento altrui si lascia sempre declassare. E la condizione di chiusura è passata da *«prima che una devnet accumuli saldi reali»* a **«prima che esista una seconda implementazione»**: la grandezza da cui il pericolo dipende non è il valore in gioco, è il numero di lettori del documento.

**Il buco vero: `unrevoked` non ha una definizione.** Il documento usa due letture, e il divario fra loro non è teorico:

- `identity.md` la lega a una revoca **finalizzata**;
- `ledger.md` la lega a *«enrolled and not revoked **as of** una certa altezza»*, cioè a una revoca **efficace**;
- e `min_revocation_effective_delay_blocks` è **dichiaratamente scelto lungo** (`ledger.md` §*revoca*).

Fra «finalizzata» ed «efficace» c'è quindi un intervallo **deliberatamente ampio**, durante il quale le due letture divergono su ogni autorizzazione. Allineare la riga 407 alle sorelle sposta la riga dentro l'ambiguità invece di toglierla: **è un rimedio che sembra completo e non lo è.**

**Il drenaggio non dipende dalla finestra.** Il Lead aveva chiesto di stabilire *su quale finestra temporale* un attaccante operi. La domanda era mal posta: prezzo e periodo del burn sono scelti da chi attacca, e **una sola transazione può azzerare il saldo**. La finestra non è la grandezza da cui la perdita dipende — è la terza domanda della famiglia 3, posta e risposta.

## Scope

### Included

- La **definizione normativa** di ciò che `unrevoked` significa per autorizzare una transazione: quale delle due letture vale, e la stessa in tutti i punti.
- L'allineamento della riga 407, **dopo** la definizione.
- La **passata su tutte le regole di autorizzazione del protocollo**, non sulle quattro nominate: il difetto è nell'asimmetria, e un'asimmetria si censisce enumerando.
- La fixture di conformità che esercita il caso in cui le due letture divergono.
- La gate di [ADR-012], perché questa spec modifica una regola di validità.

### Excluded

- Qualunque modifica a `min_revocation_effective_delay_blocks`. È lungo per una ragione dichiarata, e accorciarlo per ridurre l'intervallo di ambiguità sarebbe **curare il sintomo cambiando un parametro invece di scrivere una regola** — [ADR-010], e la famiglia 3.
- La revoca stessa, la sua propagazione e la sua meccanica.
- [DEBT-017], che ha la propria spec.

## Existing-project analysis

Le quattro righe di `docs/protocol/ledger.md`: 372 (`payer_node_id`, con qualificazione), 407 (**abbonamento, senza**), 458 (`issuer_node_id`, con), 968 (`node_id`, con). Le righe si muoveranno con l'atterraggio di [SPEC-016]: sono citate anche per il testo della clausola.

Il termine `enrolled and not revoked as of ...` compare inoltre a 728 (set di validatori) e 1027 (eleggibilità), entrambi **ancorati a un'altezza**, che è la seconda lettura.

## Technical proposal

**Primo la definizione, poi l'allineamento, poi la passata.** L'ordine è la sostanza di questa spec. Una definizione scritta dopo l'allineamento verrebbe scritta per giustificarlo.

La definizione deve scegliere fra le due letture **e motivare la scelta sulla proprietà che si vuole**, non sulla comodità di implementazione. La domanda da cui dipende: un verificatore che rigioca la catena deve poter stabilire la validità di quella transazione **senza giudizio e senza stato esterno**, e delle due letture solo una ha questa proprietà a ogni altezza.

**La fixture deve esercitare la divergenza, non la regola.** Una fixture che mostra che una chiave revocata-e-efficace viene rifiutata non prova nulla: entrambe le letture la rifiutano. Il caso che conta è quello **fra le due**: revoca finalizzata ma non ancora efficace. È lì che due implementazioni conformi divergono, ed è l'unico caso la cui fixture chiude il fork.

## Files and areas involved

- `docs/protocol/ledger.md` — la definizione, la riga 407, la passata sulle regole di autorizzazione.
- `docs/protocol/identity.md` — solo se la definizione scelta contraddice ciò che lì è scritto; in tal caso è l'altro punto a muoversi.
- `docs/protocol/README.md` — la fixture pubblicata e gli hash che ne discendono.
- `core/coblox-core/` — la regola e la sua prova in negativo.
- `sim/tools/` — la gate di [ADR-012].

## Acceptance criteria

- [x] `unrevoked` ha **una** definizione normativa per l'autorizzazione delle transazioni, e la scelta fra le due letture è motivata sulla proprietà del verificatore che rigioca, non sulla comodità.
- [x] La riga 407 è allineata, **dopo** che la definizione esiste.
- [x] Esiste l'**elenco di tutte le regole di autorizzazione** del protocollo, con la qualificazione presente o assente segnata per ciascuna, anche se l'elenco non contiene altre omissioni.
- [x] Una fixture pubblicata esercita il caso **fra le due letture** — revoca finalizzata e non ancora efficace — e le due letture ora concordano.
- [x] Nessun parametro è stato mosso.
- [x] Ogni valore pubblicato che cambia è ricalcolato con il metodo validato prima su un valore non modificato.
- [x] La gate di [ADR-012] è eseguita e la trascrizione allegata.

## Implementation plan

1. Enumerare tutte le regole di autorizzazione e le due letture, prima di scrivere.
2. Scrivere la definizione, con la motivazione.
3. Allineare la riga 407 e ogni altra omissione che l'enumerazione abbia trovato.
4. La fixture del caso divergente, e la regola in `coblox-core` con la prova in negativo.
5. Ricalcolo degli hash pubblicati, e gate di [ADR-012].

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [x] GATE-DEFINITION-FIRST | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La definizione di `unrevoked` è scritta **prima** dell'allineamento della riga 407, e la trascrizione mostra l'ordine. Una definizione scritta dopo verrebbe scritta per giustificare l'allineamento, e l'allineamento da solo sposta la riga **dentro** l'ambiguità invece di toglierla.
- [x] GATE-DIVERGENT-CASE | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La fixture esercita il caso **fra le due letture** — revoca finalizzata e non ancora efficace — e non il caso che entrambe già rifiutano. Una fixture sul caso concorde è verde oggi e sarebbe stata verde anche col difetto aperto: sarebbe un calcolo, non una guardia.
- [x] GATE-ALL-AUTHORIZATION-RULES | kind=manual | owner=agent | phase=before-submit | evidence=transcript | L'elenco di **tutte** le regole di autorizzazione del protocollo è prodotto e allegato, con la qualificazione segnata presente o assente per ciascuna, **anche se non trova altre omissioni**. Il difetto è nell'asimmetria: correggere la sola riga nominata non dimostra che sia l'unica.
- [x] GATE-NO-PARAMETER-MOVED | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Una ricerca sul diff mostra che **nessun parametro è stato mosso**, e in particolare non `min_revocation_effective_delay_blocks`. Accorciarlo ridurrebbe l'intervallo di ambiguità senza scrivere alcuna regola: è il rimedio che sembra economico ed è la famiglia 3 commessa dentro il rimedio.
- [x] GATE-ADR012 | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La passata su tutti gli artefatti pubblicati è eseguita con lo strumento versionato e la trascrizione allegata, **anche se non trova nulla**.
- [x] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto la chiusura e il Lead ha accettato la review. Il debito nasce dalla sua valutazione, ed è lei ad aver stabilito che l'allineamento da solo non chiude: è la persona che può dire se la definizione scelta chiuda davvero.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **Il rischio dominante è la chiusura che sembra completa.** Allineare quattro righe è visibile, verificabile e sbagliato da solo. Se al termine di questa spec la riga 407 è allineata e `unrevoked` non ha una definizione, il fork è ancora lì e il documento **sembra** a posto — che è peggio di prima, perché nessuno tornerà a guardare.
- **Il rischio secondario è la scelta della lettura fatta per comodità.** La lettura «finalizzata» è più semplice da implementare e la «efficace» è più semplice da rigiocare. La scelta va motivata sulla seconda proprietà, e se le due confliggessero è una decisione del Lead.
- **Le citazioni di riga si muoveranno.** [SPEC-016] sta modificando `ledger.md`. Le righe qui citate sono da riverificare sul testo della clausola e non sul numero.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable work; do not ship placeholder or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- **Fermarsi e riportare è un esito previsto**, e qui vale in particolare se le due letture non fossero riconciliabili senza cambiare la meccanica della revoca: quella è un'altra spec e una decisione del Lead.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence

### Changes made

**1. La definizione, scritta per prima.** `docs/protocol/ledger.md` ha una
sezione nuova, `### What \`enrolled, unrevoked\` means, and as of which height`,
dentro `## Unsigned transaction and authorization`. Sceglie la lettura
**«efficace a una certa altezza»**, valutata all'altezza del blocco che include
la transazione:

> For a transaction included in the block at height `h`, a `node_id` is
> **enrolled, unrevoked** when both of the following hold against the finalized
> state that block builds on: (1) a finalized enrollment certificate names
> `node_id` and its `valid_from_height` is at most `h`; and (2) no finalized
> `revoke_identity` naming `node_id` carries an `effective_height` at most `h`.

**La motivazione e' sulla proprieta' del verificatore che rigioca, e scarta
l'altra lettura invece di preferirle questa.** La finalita' e' portata da un
`QuorumCertificate` su un blocco, e **nessun blocco ne porta uno**: un
`BlockHeader` impegna `previous_block_id`, `state_root` e `transactions_root` e
nulla che registri *quando* un blocco anteriore sia diventato finale. La catena
quindi **non contiene alcuna altezza a cui una revoca «e' diventata
finalizzata»**. Un verificatore che rigioca puo' stabilire che un
`revoke_identity` era *incluso* sotto `h`; che fosse *finale* sotto `h` lo puo'
stabilire solo dai certificati che possiede fuori dalla catena, e due
verificatori con certificati diversi danno verdetto opposto sullo stesso blocco.
`effective_height` ha la forma opposta: e' impegnato nel corpo della
transazione, deve essere posteriore al blocco che la propone, e ogni
verificatore lo legge dagli stessi byte. La lettura scelta e' quindi l'unica che
rende il predicato una funzione totale del blocco e dei suoi antenati, monotona
in `h` e identica per ogni verificatore a ogni testa successiva — **e vedi
RF-008 nella remediation: l'unicita' vale fra le due letture che il documento
usava, non fra tutte le letture concepibili** — che e' la
proprieta' che `ledger.md` gia' dichiara dell'eleggibilita', *«each of them a
fact a replaying verifier settles without judgement»*.

**Il costo della scelta e' dichiarato nel documento, non scoperto.** Fra la
finalizzazione di un `revoke_identity` e il suo `effective_height` la chiave
autorizza ancora, burn di abbonamento compreso. Il paragrafo *«The cost of this
reading, declared»* lo scrive, e dice perche' l'altra lettura non lo ridurrebbe:
non accorcia l'esposizione, la rende dipendente dal verificatore, e una finestra
dipendente dal verificatore e' peggio di una dichiarata.

**2. L'allineamento della riga, dopo.** La clausola dell'abbonamento — trovata
per il testo e non per il numero, perche' si era spostata da 407 a 542 — dice
ora *«For a subscription, the key MUST derive the enrolled, unrevoked
`payer_node_id`»*, identica nella formulazione alle tre sorelle. **Nessuna delle
quattro porta un link alla definizione**: darlo alla sola riga corretta avrebbe
creato una seconda asimmetria dove ne stavo togliendo una, e la sezione dichiara
da se' la propria portata.

**3. La passata su tutte le regole di autorizzazione** — vedi
`GATE-ALL-AUTHORIZATION-RULES` sotto. Nessuna seconda omissione. Due residui
riportati e non corretti.

**4. La regola e la fixture.** `core/coblox-core/src/authorization.rs` implementa
il predicato; `AUTH-0` in `ledger.md` pubblica la tabella dei casi;
`core/coblox-core/tests/authorization_unrevoked.rs` la esercita in nove test.
Cinque probe C10 nuove pinnano la sezione, che e' interamente prosa e quindi
invisibile alle classi di scoperta della passata.

### Files changed

- `docs/protocol/ledger.md` — la definizione, l'allineamento della clausola
  dell'abbonamento, la fixture `AUTH-0`.
- `core/coblox-core/src/authorization.rs` — **nuovo**, il predicato e la regola
  completa a chiave singola.
- `core/coblox-core/src/error.rs` — `AuthorizationError` e la variante `Error`.
- `core/coblox-core/src/lib.rs` — il modulo e la sua riga nella tabella.
- `core/coblox-core/tests/authorization_unrevoked.rs` — **nuovo**, `AUTH-0`.
- `sim/tools/published_artifacts.toml` — la fixture `AUTH-0`, cinque probe C10
  nuove, e l'aggiornamento della probe preesistente
  `guide-subscription-burn-needs-your-signature`, che pinnava alla lettera il
  testo della riga corretta.

**Non toccati:** `docs/protocol/identity.md` (la definizione scelta non lo
contraddice — vedi il residuo R2), `docs/protocol/README.md` (nessun valore
pubblicato cambia), `docs/protocol/app-manifest.md` (vedi il residuo R1),
`core/coblox-core/src/params.rs`.

### Verification transcript

#### GATE-DEFINITION-FIRST

L'ordine e' negli edit, non nel racconto. La definizione e' stata inserita per
prima e lo stato intermedio e' stato osservato **prima** di toccare la clausola
dell'abbonamento:

```
$ python - <<'PY'   # inserimento della sola sezione di definizione
inserted at 2026-08-26T13:11:06.301633
NUL bytes present: False | CRLF 2792 LF 2792
PY
$ grep -n "What .enrolled, unrevoked. means\|For a subscription, the key MUST derive" docs/protocol/ledger.md
86:### What `enrolled, unrevoked` means, and as of which height
543:For a subscription, the key MUST derive `payer_node_id`; the signature is
```

```
$ echo "--- STATE BETWEEN STEP 1 AND STEP 2 ---" && sed -n '543,545p' docs/protocol/ledger.md && git diff --stat
--- STATE BETWEEN STEP 1 AND STEP 2 ---
For a subscription, the key MUST derive `payer_node_id`; the signature is
required and the node balance is debited. The service period is half-open and
 docs/protocol/ledger.md    | 130 ++++++++++++++++++
 2 files changed, 130 insertions(+), 146 deletions(-)
```

La definizione esiste (riga 86) mentre la clausola e' ancora quella difettosa
(riga 543): 130 righe aggiunte, zero righe della clausola modificate. Solo dopo
questo stato la clausola e' stata allineata. Lo stato finale:

```
$ grep -n "enrolled, unrevoked" docs/protocol/ledger.md
86:### What `enrolled, unrevoked` means, and as of which height
507:The key MUST derive the enrolled, unrevoked `payer_node_id`.        (fund_app)
542:For a subscription, the key MUST derive the enrolled, unrevoked `payer_node_id`;
593:The key MUST derive the enrolled, unrevoked `issuer_node_id`.       (challenge_commitment)
1145:The authorization key MUST derive the enrolled, unrevoked `node_id`. (validator_candidacy)
```

#### GATE-ALL-AUTHORIZATION-RULES

**Da dove viene l'elenco** ([SKILL-001] passo 6). Non dalla memoria e non dalle
quattro righe nominate: dal **registro dei domini di firma** di
`published_artifacts.toml`, che `published_artifacts.py` rideriva
meccanicamente dai documenti e fa fallire in **entrambi i versi** — un dominio
nei documenti che manca dal manifesto e' C1, uno nel manifesto che non e' piu'
nei documenti e' C6. Ogni autorizzazione di questo protocollo passa per un
dominio di firma, quindi i tredici domini `kind = "signature-domain"` sono il
lato disco dell'enumerazione. Comando:

```
$ grep -n 'kind = "signature-domain"' -B3 -A2 sim/tools/published_artifacts.toml
```

**Le tredici superfici, con la qualificazione segnata.**

| # | Dominio di firma | Oggetto / regola | Autorizzatore | Qualificazione |
|---|---|---|---|---|
| 1a | `coblox-ledger-transaction-v0` | `FundAppAuthorization`, `ledger.md:507` | chiave singola | **presente** |
| 1b | `coblox-ledger-transaction-v0` | `SubscriptionBurnAuthorization`, `ledger.md:542` | chiave singola | **era assente — questa spec** |
| 1c | `coblox-ledger-transaction-v0` | `ChallengeCommitmentAuthorization`, `ledger.md:593` | chiave singola | **presente** |
| 1d | `coblox-ledger-transaction-v0` | `ValidatorCandidacyAuthorization`, `ledger.md:1145` | chiave singola | **presente** |
| 1e | `coblox-ledger-transaction-v0` | `MintAuthorization` | quorum | n/a — via `ledger.md:890` (*«Validators are sorted by ID, unique, enrolled and unrevoked»*), *«enrolled and unrevoked»* sul set |
| 1f | `coblox-ledger-transaction-v0` | `HostingBurnAuthorization` | quorum | n/a — idem |
| 1g | `coblox-ledger-transaction-v0` | `ChallengeEvidenceAuthorization` | quorum | n/a — idem |
| 1h | `coblox-ledger-transaction-v0` | `RevokeIdentityAuthorization` | quorum | n/a — idem |
| 2 | `coblox-block-vote-v0` | voto di finalita' | chiave di consenso di un seggio | **presente**, indiretta: `ledger.md:890` (*«Validators are sorted by ID, unique, enrolled and unrevoked»*) |
| 3 | `coblox-consensus-key-binding-v0` | `key_binding_signature` | chiave di consenso | **presente**, indiretta: la candidacy che la porta e' 1d |
| 4 | `coblox-challenge-evidence-v0` | firme degli auditor | validatori | **presente**, indiretta: `ledger.md:890` (*«Validators are sorted by ID, unique, enrolled and unrevoked»*) |
| 5 | `coblox-challenge-request-v0` | `issuer_signature` | emittente | assente nella regola di `wire.md`; **coperta sul percorso di consenso** — l'evidenza e' a quorum e richiede un `challenge_commitment` finalizzato per quell'emittente, che e' 1c |
| 6 | `coblox-challenge-response-v0` | `subject_signature` | soggetto | assente; **coperta** come 5, e la risposta non autorizza nulla da sola |
| 7 | `coblox-enrollment-request-v0` | richiesta di iscrizione | chiave non ancora iscritta | **n/a per costruzione**, e la forma negativa e' presente: `identity.md:235` esige che la chiave **non** sia gia' iscritta ne' revocata |
| 8 | `coblox-enrollment-certificate-v0` | certificato di iscrizione | quorum | n/a |
| 9 | `coblox-transport-key-attestation-v0` | attestazione della chiave di trasporto | chiave d'identita' iscritta | **presente**, ma nell'**altra lettura** — `identity.md:614`, *«at the receiver's finalized height»*. Vedi il residuo R2 |
| 10 | `coblox-app-manifest-v0` | firma del publisher, `app-manifest.md:64-65` | chiave singola | **presente** (*«a finalized, unrevoked enrollment certificate»*), **senza altezza**. Vedi il residuo R1 |
| 11 | `coblox-protocol-document-v0` | documento governato | quorum | n/a |
| 12 | `coblox-weak-subjectivity-signature-v0` | checkpoint | chiave di rilascio, fuori catena | n/a — non e' un'identita' iscritta |
| 13 | `coblox-wire-envelope-v0` | `SignedEnvelope` | `sender_node_id` | n/a — autenticita' di trasporto, non validita' di un blocco; la revoca di un peer e' la regola di connessione di `identity.md` |

**Esito: nessuna seconda omissione.** L'unica regola priva della qualificazione
in tutto `docs/protocol/` era quella dell'abbonamento, e la valutazione di
AGENT-007 su questo punto e' confermata da un'enumerazione indipendente e da una
lista con lato disco. **L'elenco vale comunque**, ed e' il motivo per cui e'
richiesto anche vuoto: ha prodotto due residui che l'allineamento della sola
riga non avrebbe mai fatto emergere.

**R1 — `app-manifest.md:64-65` porta la qualificazione e non porta l'altezza.**
La chiave del publisher deve avere *«a finalized, unrevoked enrollment
certificate»*, ma il documento non dice **rispetto a quale altezza** si valuti,
ed e' la stessa sotto-specificazione che [DEBT-022] aveva sulla riga
dell'abbonamento — in forma minore, perche' la qualificazione c'e'. Non l'ho
corretto: `app-manifest.md` non e' fra i file di questa spec, e correggerlo qui
scavalcherebbe la gate che quella correzione merita ([SKILL-003]). La sezione
nuova di `ledger.md` dichiara pero' di governare *«every occurrence of the words
enrolled, unrevoked in the v0 protocol documents, including the publisher key
rule of app-manifest.md, where `h` is the height of the block finalizing the
catalog record»*, quindi il buco di lettura e' chiuso dal lato della
definizione; resta aperto il fatto che `app-manifest.md` non rimanda indietro.
**Raccomando un debito.**

**R2 — `identity.md:614` e' l'altra lettura, e resta.** Non e' una
contraddizione e per questo non ho toccato il file, come la spec prescrive: e'
una **regola di accettazione locale del ricevente** su una connessione, non una
regola di validita' su un blocco. Nessuno la rigioca, due riceventi non devono
concordare, ed e' legittimamente ancorata alla propria vista e legittimamente
piu' stretta. Ma la sua formulazione **e'** la seconda lettura, ed e' cosi' che
le due sono arrivate a coesistere: un lettore che incontra prima quella la
prende per la definizione. Il paragrafo *«One rule this definition does not
govern»* la nomina e dice perche' non lo e', ed e' pinnato da una probe.

#### GATE-DIVERGENT-CASE

**La fixture pubblicata, `AUTH-0`** (`ledger.md`): un `revoke_identity` su
`cblx1revokedfixture` con `effective_height` `50`, finalizzato nel blocco ad
altezza `20`. Le righe `21` e `49` sono **le uniche** su cui la lettura
«finalizzata» dice *invalid* e la definizione in vigore dice *valid*. Le righe
`19`, `50`, `51` sono concordi fra le due letture e sarebbero state verdi col
difetto aperto: sono nella tabella con scritto accanto che non provano nulla.

La prova in negativo e' su una **copia dell'albero**
(`.../scratchpad/neg`), non sull'albero condiviso.

Verde iniziale nella copia:

```
$ cargo test -p coblox-core --test authorization_unrevoked
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Mutazione A — la lettura «finalizzata».** Il difetto reintrodotto e'
*esattamente* quello che la gate esiste per impedire: la revoca morde
dall'altezza a cui e' stata finalizzata (`20`) invece che dal proprio
`effective_height`.

```
- .find(|record| record.effective_height <= including_height)
+ .find(|_record| 20_u64 <= including_height)
```

```
MUTATION A applied: the finalized reading

---- a_finalized_but_not_yet_effective_revocation_still_authorizes_at_21 stdout ----
thread '...at_21' panicked at tests\authorization_unrevoked.rs:74:5:
assertion failed: qualification_at(REVOKED, 21).is_ok()

---- a_finalized_but_not_yet_effective_revocation_still_authorizes_at_49 stdout ----
thread '...at_49' panicked at tests\authorization_unrevoked.rs:80:5:
assertion failed: qualification_at(REVOKED, 49).is_ok()

failures:
    a_finalized_but_not_yet_effective_revocation_still_authorizes_at_21
    a_finalized_but_not_yet_effective_revocation_still_authorizes_at_49

test result: FAILED. 7 passed; 2 failed
```

**Falliscono le due righe divergenti e solo quelle.** Le tre righe concordi
restano verdi sotto la lettura sbagliata, che e' la dimostrazione di cosa
misurano: nulla. La guardia e' nelle due, e le nomina.

Ripristino e verde riverificato:

```
$ cargo test -p coblox-core --test authorization_unrevoked
test result: ok. 9 passed; 0 failed
```

**Mutazione B — [DEBT-022] stesso.** L'autorizzazione controlla solo che la
chiave derivi il node ID, che e' alla lettera cio' che la riga diceva.

```
- enrolled_unrevoked(node_id, including_height, enrollments, revocations)
+ let _ = (including_height, enrollments, revocations);
+ Ok(())
```

```
MUTATION B applied: the key MUST derive payer_node_id, and nothing more

---- the_complete_rule_checks_the_derivation_and_the_qualification stdout ----
thread '...' panicked at tests\authorization_unrevoked.rs:157:5:
assertion failed: matches!(authorize_single_key(&public_key, derived.as_str(), 21,
    &enrollments, &revocations),
    Err(Error::Authorization(AuthorizationError::Revoked { .. })))

test result: FAILED. 8 passed; 1 failed
```

Ripristino, verde riverificato, e copia identica all'albero:

```
$ cargo test -p coblox-core --test authorization_unrevoked
test result: ok. 9 passed; 0 failed
$ diff E:/Git/CobloxNetwork/core/coblox-core/src/authorization.rs $SC/core/.../authorization.rs
COPY IDENTICAL TO TREE
```

**[SKILL-001] passo 4 — quale grandezza e' costante in tutti i casi.** Nelle
cinque righe sull'identita' revocata e' costante *l'esistenza di una revoca*:
un'implementazione che rispondesse *invalid* a ogni chiave le passerebbe tutte
tranne le due divergenti, e un'implementazione che leggesse la clausola 1 come
la clausola 2 non sarebbe distinta. Ho quindi aggiunto **la sesta riga**
(`cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka`, che nessuna revoca
nomina, alla **stessa altezza** `51` di una riga invalida) e due casi sulla
clausola 1 — identita' che nessun certificato nomina, e altezza sotto
`valid_from_height` — che falliscono con `NotEnrolled` e non con `Revoked`.
Seconda grandezza che sarebbe rimasta costante: `effective_height` e l'altezza
di finalizzazione. Sono **deliberatamente diverse** (`50` contro `20`) e la
mutazione A e' precisamente ciò che quella differenza rende osservabile; se
fossero uguali la mutazione A sarebbe passata verde. E' scritto sia nel
documento sia accanto alla fixture, perche' una riga che sembra un doppione
viene cancellata dal primo che riordina ([SKILL-004] passo 7).

#### GATE-NO-PARAMETER-MOVED

```
$ git diff -U0 -- docs sim core | grep -E "^[+-]" | grep -v "^[+-][+-][+-]" \
    | grep -E "_blocks|_ms|_seconds|_units|_bits|_seats|_size|:\"[0-9]+\"|= [0-9]+"
+included. That interval is at least `min_revocation_effective_delay_blocks`
+count = 1
+count = 1
+count = 1
+count = 1
+count = 1
```

Le uniche righe di forma parametrica nel diff sono: **una menzione in prosa** di
`min_revocation_effective_delay_blocks` nel paragrafo che dichiara il costo
della lettura scelta — il parametro e' **nominato, non mosso** — e i cinque
`count = 1` delle probe C10 nuove, che sono metadati del manifesto e non
parametri di protocollo. `core/coblox-core/src/params.rs` non e' fra i file
modificati.

#### GATE-ADR012

Stato di partenza annotato prima del lavoro ([SKILL-002] precondizione):
**137 probe C10, PASS**.

```
$ python sim/tools/published_artifacts.py        # PRIMA
  C1-DOMAIN 40 / C2-TAG 24 / C3-FIXTURE-ID 19 / C4-VALUE 60 / C5-MIRROR 53
  C7-COVERAGE 51 / C8-ENCODING 1 / C9-EXAMPLE 1 / C5-DISCOVERED 67
  C10-PROBE 137 / C11-CLAIMDOC 8
published-artifact inventory: PASS
```

La passata **ha morso durante il lavoro**, ed e' l'unica ragione per cui una
probe preesistente non e' rimasta a pinnare un testo riscritto:

```
FAIL C10-PROBE: probe 'guide-subscription-burn-needs-your-signature' expected 1
match(es) of 'the key MUST derive `payer_node_id`; the signature is' in
ledger.md, found 0.
```

Quella probe pinnava alla lettera la riga che questa spec corregge, a garanzia
di una frase della guida (*«nobody can take credits away from you»*). L'ho
aggiornata al testo nuovo **allargandola invece di restringerla**: ora pinna
anche la qualificazione, perche' una firma di una chiave che la rete ha revocato
non e' una firma che l'utente ha dato, e senza la qualificazione quella frase
della guida sarebbe falsa.

Un secondo intoppo, riportato perche' e' esattamente la famiglia contro cui il
dispatch mi metteva in guardia: la prima stesura del pattern usava `\s+` dentro
una stringa TOML **base**, dove `\s` non e' un escape valido. `tomllib` ha
fallito il parse e l'ho riscritto come stringa **letterale**. Verificato che il
byte sia scritto e non interpretato:

```
$ python -c "import tomllib; ..."
guide-subscription-burn-needs-your-signature -> 'the key MUST derive the enrolled, unrevoked `payer_node_id`;\\s+the signature is'
```

Stato finale:

```
$ python sim/tools/published_artifacts.py        # DOPO
  C1-DOMAIN 40 / C2-TAG 24 / C3-FIXTURE-ID 20 / C4-VALUE 60 / C5-MIRROR 53
  C7-COVERAGE 51 / C8-ENCODING 1 / C9-EXAMPLE 1 / C5-DISCOVERED 67
  C10-PROBE 142 / C11-CLAIMDOC 8
published-artifact inventory: PASS
```

**Le differenze, spiegate.** C3-FIXTURE-ID 19 → **20**: `AUTH-0`, registrata
come fixture comportamentale accanto ad `ADM-1` e `ORDER-1`. C10-PROBE 137 →
**142**: cinque probe nuove sulla sezione di definizione, che e' **interamente
prosa** — nessun digest, nessun dominio, nessun tag byte — e che le cinque classi
di scoperta quindi non vedono. Sono le uniche cose che la tengono al suo posto:
`unrevoked-anchored-to-the-including-height` (la clausola 2, cioe' la scelta
stessa), `unrevoked-no-block-carries-a-quorum-certificate` (la ragione che non
dipende dalla comodita'), `unrevoked-declared-window-cost` (il residuo
dichiarato), `unrevoked-receiver-local-rule-is-not-the-definition` (R2), e
`auth0-divergent-rows-are-the-fixture` (quali righe sono la guardia). Tutte le
altre classi invariate.

**La prova in negativo della passata**, con ogni probe verificata
individualmente ([SKILL-002] passo 3):

```
$ python sim/tools/published_artifacts_negative.py
=== C10-PROBE, every probe individually ===
  every one of the 142 probes was observed failing
negative proof: PASS - 15 mutations across 11 defect classes, plus every probe
individually, each observed failing
```

Le cinque probe nuove **sono nel conteggio individuale**: 142, non 137.

**Valori pubblicati: nessuno e' cambiato, e la passata si e' eseguita lo
stesso.** Questa spec non introduce alcuna preimmagine e non tocca alcuna
fixture di digest: `AUTH-0` e' comportamentale, come `ORDER-1`, e la definizione
e' una regola su altezze gia' impegnate. Non c'era quindi alcun valore su cui
applicare la coppia *validazione su invariato → applicazione al variato*. Lo
strumento e' stato eseguito comunque, ed e' il caso che dimostra di averlo
fatto:

```
$ python sim/tools/protocol_hashes.py
  revocation_leaf REVL-0       MATCH
    published sha256:7fb1f4024627c413cbf70b49a390b6d31778e667e86042864c4bed107cd52497
    computed  sha256:7fb1f4024627c413cbf70b49a390b6d31778e667e86042864c4bed107cd52497
  account_key (app) APP-0      MATCH
  app_leaf APP-0               MATCH
every published value reproduced: PASS
```

`REVL-0` e' il valore pertinente e vale la pena dirlo: e' la foglia di revoca
per `cblx1revokedfixture` a `effective_height` 50, cioe' la **stessa identita' e
la stessa altezza** che `AUTH-0` usa. Il suo digest e' invariato — la spec
cambia cio' che `effective_height` significa per autorizzare, non cio' che la
foglia impegna.

#### Suite e lint

```
$ cargo test --workspace | grep "^test result" | somma dei passati
PRIMA:  TOTAL PASSED: 167
DOPO:   TOTAL PASSED: 176        (+9, tutti in tests/authorization_unrevoked.rs)

$ cargo fmt --all -- --check      (nessun diff)
$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

Controllo del byte NUL e delle interruzioni di riga sui file di testo toccati,
per l'avvertimento del dispatch:

```
toml   CRLF 0    LF 2616  NUL False      (il file era ed e' a LF)
ledger CRLF 2791 LF 2791  NUL False      (il file era ed e' a CRLF)
core/coblox-core/src/authorization.rs       NUL False
core/coblox-core/tests/authorization_unrevoked.rs  NUL False
```

### Limiti noti e cio' che non e' stato fatto

- **La finestra fra finalizzazione ed efficacia resta aperta, per scelta, ed e'
  scritta.** La lettura scelta la dichiara invece di chiuderla, e chiuderla
  richiederebbe di toccare la meccanica della revoca — chi sceglie
  `effective_height` e con quale vincolo per `reason = "key_compromise"` — che
  questa spec esclude. AGENT-007 raccomandava l'esito (A) con la lettura
  **«finalizzata»** proprio per chiudere questa finestra: **non l'ho seguita, e
  il disaccordo e' nominato sotto.**
- **`min_revocation_effective_delay_blocks` non e' stato mosso** e non andava
  mosso.
- **R1 e R2 riportati, non corretti.** Vedi `GATE-ALL-AUTHORIZATION-RULES`.
- **La regola di unicita' su `(payer_node_id, app_id, service_period)`**, che
  AGENT-007 lascia esplicitamente come domanda aperta a chi chiude il debito:
  **non l'ho cercata**, perche' e' una domanda separata da `unrevoked` e
  risolverla qui per comodita' sarebbe la stessa scorciatoia che la spec
  vieta. Resta aperta.
- **Nessun `git add`, `git commit`, `git push`.** Il lavoro e' nell'albero.

## Remediation of [REVIEW-033] — 2026-08-26 (AGENT-002)

La spec resta in `review`. Quattro voci chiuse dentro il perimetro, cinque
riportate. **RF-001 chiuso solo dal lato del testo**, e la ragione e' sotto.

### RF-004 — la frontiera della clausola 1, chiusa, e la mutazione osservata rossa

Il finding e' giusto e la sua diagnosi lo e' di piu': fra le due clausole della
definizione, quella sotto test era la seconda e **la prima era la grandezza
costante**. E' il passo 4 di [SKILL-001] applicato dentro la definizione invece
che dentro la fixture, ed e' il livello a cui non l'avevo applicato.

`AUTH-0` ha ora una colonna `enrolled by h` e **due righe nuove**: `h = 4`
(sotto `valid_from_height`, invalid) e **`h = 5`**, che e' la frontiera. La
sezione dice esplicitamente che le due frontiere sono una riga ciascuna e che
entrambe le clausole sono `<=` e non `<`. Il test
`enrollment_authorizes_exactly_at_its_valid_from_height` la esercita, e
`authorization.rs` ha ora accanto alla clausola 1 lo stesso commento che aveva
solo la clausola 2.

**Mutazione C, la stessa che [REVIEW-033] ha riprodotto**, su copia dell'albero
in `.../scratchpad/neg2` — che stavolta include `docs/`, perche'
`speccheck_conformance.rs` fa `include_str!` su `docs/protocol/README.md` e
senza quella cartella il workspace non compila.

Verde di partenza nella copia: **177 passati**.

```
- record.node_id == node_id && record.valid_from_height <= including_height
+ record.node_id == node_id && record.valid_from_height <  including_height
```

```
MUTATION C applied: clause 1 comparison weakened from <= to <

---- enrollment_authorizes_exactly_at_its_valid_from_height stdout ----
thread '...' panicked at tests\authorization_unrevoked.rs:154:5:
assertion failed: qualification_at(REVOKED, VALID_FROM_HEIGHT).is_ok()

failures:
    enrollment_authorizes_exactly_at_its_valid_from_height

test result: FAILED. 9 passed; 1 failed
```

**La mutazione che sopravviveva a 176 test ora fallisce, e il fallimento nomina
il caso.** Ripristino, verde riverificato (10 su 10), `diff` con l'albero vuoto.

**Riverificata anche la mutazione A sulla fixture allargata**, perche' aggiungere
righe a una fixture e' il modo di indebolirla senza accorgersene:

```
MUTATION A re-applied on the enlarged fixture
---- a_finalized_but_not_yet_effective_revocation_still_authorizes_at_21 ---
---- a_finalized_but_not_yet_effective_revocation_still_authorizes_at_49 ---
test result: FAILED. 8 passed; 2 failed
```

Continuano a fallire **quelle due e solo quelle**. Ripristino, verde, copia
byte-identica all'albero.

### RF-003 — la motivazione della probe, riscritta su cio' che il protocollo consegna

Il finding e' corretto e la forma dell'errore e' peggiore di come l'avevo
scritta: avevo allargato la probe *credendo di rafforzarla*, e la motivazione
nuova affermava una garanzia — *«a signature from a key the network has revoked
is not a signature you gave»* — che **dentro la finestra e' falsa**, nel campo
che questo progetto usa per legare una promessa pubblica a una riga di
protocollo. Il `why` ora dice che la revoca ferma la spesa **da
`effective_height`** e non dalla finalizzazione, che dentro quella finestra un
addebito firmato con una chiave rubata riduce ancora il saldo, e rimanda a
RF-001. Il pattern resta allargato: la qualificazione **e'** la parte della
frase che il protocollo impone.

### RF-005 — la portata dichiarata, e il residuo R3

Il finding e' corretto due volte. La portata era dichiarata come **stringa
letterale** (*«every occurrence of the words `enrolled, unrevoked`»*) mentre le
formulazioni reali sono tre e **nessuna e' quella**:

```
$ grep -n "enrolled, unrevoked\|enrolled and unrevoked\|enrolled and not revoked\|finalized, unrevoked" docs/protocol/*.md
app-manifest.md:65:  ... finalized, unrevoked enrollment certificate.
ledger.md:928:      ... enrolled and unrevoked; voting power is
ledger.md:1242:  1. it is enrolled and not revoked as of `candidacy_close_height(e)`;
```

La portata e' ora dichiarata **come perimetro e non come grafia**: governa la
qualificazione ovunque autorizzi una *transazione*, in tutte e tre le
formulazioni, e `h` e' sempre l'altezza del blocco che la include.

**R3 — `ledger.md:928` porta la qualificazione e non porta l'altezza, e non e'
raggiunta dalla definizione.** E' il gemello di R1 dentro il file che questa
spec poteva toccare, ed e' peggiore di R1 per un motivo che il finding nomina:
un `ValidatorSet` **non e' una transazione**, quindi l'ancoraggio scelto — che
e' all'altezza del blocco includente — non la raggiunge *per costruzione*, e sei
delle tredici celle della mia tabella erano segnate coperte via quella riga.
Non ho supplito l'altezza in silenzio: `activation_height` e' l'ancoraggio
plausibile, ma sceglierlo e' una decisione sulla continuita' del set e non su
cosa significhi la qualificazione — e questa spec esclude la meccanica del set
tanto quanto quella della revoca. La sezione ora **dichiara il limite** invece
di lasciare la cella verde. **Raccomando un debito**, accanto a quello di R1.

### RF-001 — chiuso dal lato del testo; il recinto e' fuori perimetro, e ho valutato quale grandezza vincolare

**Il finding e' giusto e la sua diagnosi e' quella che mi mancava**: la mia
dichiarazione del costo raccontava il pavimento e taceva che il soffitto non
c'e', ed e' [DEBT-022] spostato dalla clausola **al campo su cui la clausola ora
poggia**. Riverificato da me:

```
$ grep -n "effective_height" docs/protocol/*.md core/coblox-core/src/*.rs
```

Due soli MUST lo nominano — `ledger.md:711` (posteriore al blocco proponente) e
`ledger.md:963` (il pavimento). La `1037` e' una condizione sulla validita' di
una **contrazione**, non un tetto: una revoca con `effective_height` assurdo non
puo' giustificare una contrazione e **resta una revoca valida**.
`light_client.rs:289` confronta `header.height >= effective_height` senza alcun
vincolo sul valore. **Nessun tetto, confermato.**

Il paragrafo del costo ora dice che l'intervallo **non ha limite superiore**,
che una revoca con `effective_height` assurdo soddisfa ogni MUST ed e'
cosmetica, e che **finche' il campo non e' recintato, quanto una revoca protegga
un saldo lo sceglie il quorum che revoca** — con la frase che dice che e' la
definizione ad aver dato al campo quella portata.

**Perche' non ho recintato il campo, avendo valutato prima quale grandezza
vincolare.** Il tetto ovvio — un `max_revocation_effective_delay_blocks` — e'
**la famiglia 3 un'altra volta**: un valore scelto bene al posto di una
proprieta', e per giunta inefficace, perche' un quorum ostile sceglierebbe
comunque il massimo. Le due alternative che *non* sono famiglia 3 **tolgono la
discrezione invece di limitarla**, e sono entrambe meccanica della revoca:

1. far mordere la revoca, **sul solo percorso di spesa**, a
   `min(effective_height, altezza_proponente + min_revocation_effective_delay_blocks)`.
   E' replayable — l'altezza proponente e' un fatto degli antenati — ma
   reintrodurrebbe **due significati di *revocata* alla stessa altezza**, uno per
   spendere e uno per validare, che e' precisamente cio' contro cui ho
   argomentato per scartare la lettura «finalizzata»;
2. legare il ritardo a `reason`: per `key_compromise` nessuna discrezione, il
   ritardo **e'** il pavimento.

La seconda e' la piu' promettente e ha un fatto a favore che vale la pena
scrivere perche' rende il debito azionabile: **`reason` esiste gia' e non e'
letto da nulla.**

```
$ grep -n "key_compromise\|validator_misconduct\|operator_request" docs/protocol/*.md core/coblox-core/src/*.rs
docs/protocol/ledger.md:778:  "reason":"key_compromise"|"validator_misconduct"|"operator_request",
```

Un'unica occorrenza: la dichiarazione. Il campo che porterebbe la distinzione e'
gia' in catena, gia' impegnato nell'ID della transazione, e **inerte**. Cambiare
questo significa cambiare quando una revoca morde, che e' meccanica della revoca
ed e' esclusa da questa spec. **Mi fermo e riporto**, come il dispatch prevede.

**RF-001 e RF-002 sono la stessa leva da due lati** e vanno probabilmente in un
debito solo, o in due legati: RF-002 dice che il pavimento e' tarato da una
ragione di continuita' del set che su un nodo senza seggio non esiste, e la
proposta 2 qui sopra e' esattamente il rimedio che ne discende.

### RF-006 — il paragrafo non fa piu' sembrare chiusa la convivenza

Il finding e' corretto e la mia classificazione era **giusta per il perimetro
che avevo guardato**: ho dimostrato che `identity.md:614` non e' una regola di
validita' su un blocco, e ho concluso che quindi non produce due verdetti. La
seconda cosa non segue dalla prima. Il paragrafo ora dice che la regola locale
governa la **raggiungibilita'** e non la validita', e che dentro la finestra due
auditor ad altezze finalizzate diverse registrano esiti opposti sulla stessa
sfida — `no_response` contro `passed` — e che quell'esito entra in un
`challenge_evidence` firmato a quorum, da cui raggiunge `contribution_score`,
l'eleggibilita' e i mint `work_compensation`: **la lettura locale raggiunge la
catena attraverso il contenuto di un oggetto invece che attraverso la validita'
di un blocco.** Il paragrafo nomina anche lo stato contraddittorio dentro la
finestra — autorizzato a spendere, contato a pieno peso di voto,
obbligatoriamente irraggiungibile — e chiude dicendo **che non dichiara la
combinazione risolta**. Il resto e' fuori perimetro: **raccomando il debito**,
nella passata di [DEBT-018] come chiede la review, perche' l'attore non ha
ancora una cella.

### RF-007 e RF-008 — le due frasi che affermavano piu' del vero

**RF-007.** *«Le righe 21 e 49 sono le uniche»* era vera della tabella e falsa
delle altezze. Ora: sono le uniche righe **di questa tabella**, e l'intervallo
divergente e' **`[20, 49]`**, di cui la tabella campiona il primo interno e
l'ultimo — detto in `ledger.md` e nel commento di modulo del file di test, che
portava la stessa frase.

**RF-008.** L'unicita' era rivendicata nell'**evidenza** e non nel documento, e
la review ha ragione a chiamarla falsa: la terza lettura — *nessun
`revoke_identity` incluso ad altezza `<= h`* — ha tutte le proprieta' che il mio
argomento invoca **e chiude la finestra**. Non e' adottabile senza contraddire
`identity.md:638`, cioe' senza ridefinire `effective_height`. Il paragrafo del
costo lo scrive ora: **il baratto non e' «finestra dichiarata contro fork», e'
«finestra dichiarata contro ridefinire `effective_height`»**, e il secondo
termine e' lavoro che nessuno ha fatto, non una cosa impossibile. **Ritiro la
frase dell'evidenza precedente**: la lettura scelta e' l'unica delle due
candidate del documento con quella proprieta', non l'unica concepibile.

### RF-009 e la nota sui chiamanti

**RF-009**, la regola di unicita' su `(payer_node_id, app_id, service_period)`:
la reviewer conferma che non cercarla era corretto, l'ha cercata e **non
esiste**, e quattro oggetti sorelle portano un *«at most one per…»* che il burn
di abbonamento non porta. Non e' lo stesso difetto ed e' **la stessa mano**:
seconda volta che quella clausola risulta non chiedere qualcosa che le sorelle
chiedono. Non la chiudo qui — resta separata da `unrevoked` per le tre ragioni
della Domanda 6 — e **raccomando il debito**.

**La nota sui chiamanti e' esatta e la lascio scritta**: `authorize_single_key`
e `enrolled_unrevoked` non hanno chiamanti fuori dai test, perche' non esiste
alcun percorso di esecuzione delle transazioni in questo crate.
`election::CandidateFacts` ha la stessa forma per la stessa ragione. Il modulo e'
la regola con la sua prova in negativo, non una guardia su un percorso di spesa,
e oggi nulla lo cabla.

### Passata e suite dopo la remediation

```
$ python sim/tools/published_artifacts.py
  C3-FIXTURE-ID 20 / C10-PROBE 146 / tutte le altre classi invariate
published-artifact inventory: PASS

$ python sim/tools/published_artifacts_negative.py
=== C10-PROBE, every probe individually ===
deleting each probe's own pinned passage from its own document, 146 case(s)
  every one of the 146 probes was observed failing
negative proof: PASS - 15 mutations across 11 defect classes

$ python sim/tools/protocol_hashes.py
every published value reproduced: PASS        (nessun valore pubblicato cambia)
```

**C10-PROBE 142 → 146.** Una probe **riscritta** —
`auth0-divergent-rows-are-the-fixture`, perche' RF-007 ha cambiato la frase che
pinnava — e **quattro nuove**, tutte su prosa che la remediation ha reso
portante e che le classi di scoperta non vedono:
`unrevoked-effective-height-has-no-upper-bound` (RF-001, ed e' la frase che
questa consegna aveva taciuto),
`unrevoked-window-is-the-sample-not-the-enumeration` (RF-007),
`unrevoked-clause-one-boundary-row` (RF-004, perche' la riga `5` verra' letta
come doppione della `19` dal primo che riordina) e
`unrevoked-local-rule-reaches-the-chain-through-evidence` (RF-006).

```
$ cargo test --workspace     176 → 177     (+1: la frontiera della clausola 1)
$ cargo fmt --all -- --check  FMT CLEAN
$ cargo clippy --workspace --all-targets -- -D warnings   Finished
```

Byte NUL e interruzioni di riga riverificati su tutti i file toccati:
`ledger.md` resta CRLF (2861), il `.toml` e i due file Rust restano LF, nessun
NUL.

### Debiti raccomandati al Lead

Cinque, tutti fuori perimetro, e concordo con le severita' della review:
**`effective_height` senza tetto** (RF-001, `high`), **il pavimento del ritardo
e `reason`** (RF-002, `medium`, e vedi sopra: e' la stessa leva di RF-001),
**isolamento di trasporto contro potere di voto dentro la finestra** (RF-006,
`medium`, nella passata di [DEBT-018]), **unicita' su `(payer_node_id, app_id,
service_period)`** (RF-009, `low`), **la conseguenza non scritta sul conteggio
degli abbonati** (`ledger.md:391`, `low`). **Piu' due miei**: **R1**
(`app-manifest.md:65`, qualificazione senza altezza) e **R3** (`ledger.md:928`,
idem su un oggetto che non e' una transazione, e su cui sei celle della tabella
poggiano).
