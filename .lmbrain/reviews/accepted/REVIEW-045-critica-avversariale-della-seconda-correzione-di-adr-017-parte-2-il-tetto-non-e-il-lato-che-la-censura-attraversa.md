---
id: REVIEW-045
# Note: Quote the title if it contains a colon
title: "Critica avversariale della BOZZA di seconda correzione di ADR-017 parte 2: il tetto non e' il lato che la censura attraversa"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-022
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-LEAD
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, correctness, documentation, verification-integrity]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-045-EVENT-001"
    timestamp: "2026-08-27T14:54:24.380030800+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata su richiesta dell'operatore del 2026-08-27. Non e' il verdetto su una consegna ma una critica avversariale di una bozza, chiesta prima della decisione: stesso tipo e stesso esito di REVIEW-036, che e' accepted. Ha finito il proprio lavoro.\n\nHa fatto esattamente cio' per cui e' stata autorizzata: ha demolito la bozza v1 con sei high PRIMA che diventasse testo normativo. Le due correzioni precedenti a questo ADR erano state attaccate solo dopo essere entrate in ledger.md, in README.md e in params.rs, e dopo essere state pinnate da probe verdi. Questa volta il testo falso non e' entrato da nessuna parte.\n\nIl Lead ha verificato eseguendo il rilievo portante: con F=10, G=5, e=100 l'estremo superiore della finestra di inclusione e' p=90 identico con e senza tetto, e a p=91 la revoca e' invalida in entrambi i casi per il pavimento e >= p+F. Il tetto non e' il lato che la censura attraversa, e la premessa della decisione dell'operatore era falsa. La review individua inoltre l'origine dell'errore in REVIEW-042 RF-001, cioe' in un proprio rilievo precedente, e lo dichiara.\n\nEsiti consumati nella bozza v2, commit d8389f1: rimozione del tetto ritirata, efficacia derivata all'inclusione su proposta di questa review, ritiro della rotazione ridotto alla sola gamba che regge. Esiti non consumati e dichiarati aperti nella bozza: la sorte di revocation_effective_grace_blocks_min, il tetto sui reason pianificati, e l'enumerazione degli artefatti resi falsi, che va prodotta eseguendo la passata di ADR-012 e non derivata a mano.\n\nRF-008 resta come lavoro di tooling proprio: la regola sulla provenienza degli argomenti nelle probe e' la forma verificabile che sostituisce una regola di metodo che il Lead aveva scritto e che non era decidibile."
    evidence_refs: ["ADR-017", "SPEC-022", "REVIEW-044", "REVIEW-042", "REVIEW-036"]
    implementation_agent: "AGENT-LEAD"
links: [ADR-017, ADR-018, DEBT-040, DEBT-033]
created: 2026-08-27
updated: 2026-08-27
tags: [security, review, consensus, ledger]
related_decisions: [ADR-017, ADR-018, ADR-010, ADR-012, ADR-013, ADR-016]
activity:
  - date: 2026-08-27
    action: "transitioned pending -> accepted"
---
# Review

> **Critica avversariale di una BOZZA**, richiesta dall'operatore prima che il testo diventi
> normativo. Oggetto: il riquadro *«SECONDA CORREZIONE, del 2026-08-27 — BOZZA»* di [ADR-017]
> parte 2, la decisione dell'operatore che vi e' dentro, e le due questioni che dichiara aperte.
> Stessa forma di [REVIEW-036], che attacco' la prima stesura dello stesso ADR.
> Eseguita sull'albero a `7c95267`.

## Outcome

**La bozza non regge.** Sei `high`, due `medium`, due `low`.

**Il colpo principale e' aritmetico e ribalta la premessa della decisione.** La bozza motiva la
caduta del tetto cosi': *«Su quel `reason` un ritardo di inclusione torna a poter solo **rimandare**
e mai **distruggere**, com'era nella clausola 4 preesistente.»* **E' falso, e l'ho eseguito.** Il
vincolo che una censura attraversa non e' il tetto: e' il **pavimento** `e >= p + F`, cioe' la
clausola 4 stessa. Il tetto e' il lato che un'inclusione **troppo presto** viola. Togliere il tetto
allarga la finestra di inclusione **verso il basso**, dove non c'e' avversario, e lascia la
scadenza `e − F` esattamente dov'era.

**La seconda faccia e' che la decisione non toglie un tetto a una riga: toglie la banda.** `reason`
lo sceglie chi firma e non lo verifica nessuno ([REVIEW-036] RF-005, gia' dichiarato in [ADR-017]).
Con la riga `key_compromise` senza tetto, l'insieme delle altezze ammesse per quel `reason` e' un
**soprainsieme stretto** di quello dei `reason` pianificati: `P` vincola solo un quorum che accetti
di essere vincolato. Il risultato e' lo stato che [ADR-017] aveva rifiutato fra le alternative con
la propria frase — *«Non fare nulla e dichiarare la finestra. E' lo stato attuale, ed e' onesto.
Rifiutata perche' la dichiarazione descrive un danno di cui l'ampiezza la sceglie l'avversario»* —
riottenuto per intero sul solo `reason` per cui la parte 2 era stata scritta.

**La terza e' che l'aritmetica del rimedio che la decisione dice di riottenere non e' stata fatta.**
Il tetto toglieva la diluizione di un lotto su piu' transizioni **solo** quando `G + 1` e' minore
del numero di contrazioni necessarie. Sui soli bounds tarati del repository (`G_min = 17`) quel
numero e' `10` contro `18` disponibili: **il tetto non toglieva il rimedio**. Lo toglieva sotto
`permissive_bounds()`, cioe' sotto `G = 1`, che e' lo stato che il pavimento di genesi ha appena
proibito. La decisione e' quindi una risposta sproporzionata a [REVIEW-044] RF-002 — che e' mio, e
che era **quantitativamente sbagliato** nel dichiarare il rimedio tolto: lo correggo qui.

**Cio' che regge.** La gamba (3) dell'argomento ritirato — censura a **oltre un terzo**, quorum a
**oltre due terzi** — e' l'unica delle tre che sia permanente e sia descritta correttamente, ed e'
da sola sufficiente a ritirare l'argomento della rotazione. Il ritiro e' giusto nella conclusione;
sono le ragioni scritte che non tengono.

## Cosa ho ESEGUITO, e cosa ho solo LETTO

**Eseguito** (Windows 11, `python 3`, albero pulito salvo la rimozione gia' presente di
`reviews/pending/REVIEW-044…` prima che cominciassi — non l'ho prodotta io e non l'ho toccata):

- `python sim/tools/published_artifacts.py` → **PASS**, `C10-PROBE 172`, `C11-CLAIMDOC 8`. Serve a
  RF-006: le probe che pinnano le frasi che la decisione rende false sono **verdi adesso**.
- **Un controesempio aritmetico eseguibile** che trascrive
  `identity.rs::validate_effective_height` (il predicato implementato, non la prosa) e le regole
  1, 2, 8 e 10 di `ledger.md`, e calcola: (a) quale lato della banda una censura attraversa, con e
  senza tetto; (b) se la riga senza tetto discrimini ancora su `reason`; (c) quante transizioni di
  contrazione servono e quante altezze di efficacia distinte una singola altezza di inclusione
  ammette; (d) che cosa vincoli `effective_height` con il tetto caduto. Uscite riportate dentro i
  finding. Lo script vive nella mia scratchpad e non tocca l'albero.

**Letto e non eseguito:** [ADR-017] per intero, [ADR-018] (punti 1–8, §3, *Review conditions*),
[REVIEW-036], [REVIEW-042], [REVIEW-044], [DEBT-040], `docs/protocol/ledger.md`
(§*Identity revocation* `:783-834`, §*Revocation forces a validator set transition* `:1058-1160`,
§*Magnitudes, not only relations* `:2040-2130`, §*Why inclusion height on the spending path*
`:150-200`, §*cadence* `:345-360` e `:875-910`), `core/coblox-core/src/params.rs`
(`ElectionBounds`, `check_relations`, `check_magnitudes`), `core/coblox-core/src/identity.rs`
(`validate_effective_height`, `validate_effective_height_in_block`),
`sim/tools/published_artifacts.toml` (le sette probe della revoca e le due `claims` della guida),
`SECURITY.md:105-125`, [SPEC-025], [ADR-013].

**Non guardato, dichiarato:** `cargo test` non e' stato eseguito in questa passata — nessuna
affermazione qui dipende dall'esito della suite; `GATE-CI-GREEN`; `apps/`, `dist/`, FFI;
`sim/coblox_sim/` oltre a `recommended.py:54-56` letto in [REVIEW-044]; la superficie di
[DEBT-034].

**Perimetro di ogni affermazione sul proponente:** riguarda la regola **decisa** in [ADR-018] e la
spec **non consegnata** [SPEC-025]. Un motore di consenso non esiste in `core/`.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security-boundary | severity=high | criterion=ADR-017 parte 2, seconda correzione; QUALITY.md §*Verification standard* | remediation=ritirare la frase «un ritardo di inclusione torna a poter solo rimandare e mai distruggere», e riscrivere la motivazione della decisione contro il lato che la censura attraversa davvero — il pavimento `e >= p + F` — dichiarando il prezzo della sola relief che il tetto caduto produce

**Il tetto non e' il lato che una censura attraversa. E' il pavimento, cioe' la clausola 4 che la
bozza cita come stato innocuo.**

Testo attaccato, spazi normalizzati perche' attraversa due a capo ([ADR-017], riquadro BOZZA):

> *«Su quel `reason` un ritardo di inclusione torna a poter solo **rimandare** e mai **distruggere**,
> com'era nella clausola 4 preesistente.»*

Il corpo firmato porta `e` fisso; l'inclusione avviene a `p`, che la censura fa **crescere**.
`ledger.md:2083-2085` scrive l'aritmetica: *«given a body signed with `effective_height = e`, the
admissible inclusion heights are `[e − F − G, e − F]`»*. L'estremo **superiore** `e − F` viene dal
**pavimento**; l'estremo inferiore `e − F − G` viene dal tetto. Un ritardo attraversa l'estremo
superiore. **Con `F = 10, G = 5, e = 100`, eseguito:**

```text
capped    inclusion heights ammesse: [85, 90]   larghezza 6
capped    a p = 91: valid=False, rotto='floor e >= p+F'
uncapped  inclusion heights ammesse: [0, 90]    larghezza 91
uncapped  a p = 91: valid=False, rotto='floor e >= p+F'
```

**Con e senza tetto la revoca muore alla stessa altezza e per lo stesso vincolo.** Il tetto e'
il lato che punisce un'inclusione **troppo presto**, cioe' una sovrastima di `p` da parte del
quorum; il pavimento e' il lato che punisce una sottostima, ed e' quello che un avversario muove.

**La clausola 4 preesistente distruggeva.** *«`effective_height` MUST be at least
`min_revocation_effective_delay_blocks` above the height of the block proposing the revocation»*
(`ledger.md:1076-1080`) e' `e >= p + F`: firmata `e = 100` con `F = 10`, un'inclusione a `p = 91`
la rende invalida e il giro di firma va rifatto contro una transazione che porta `expires_at_ms`.
**La frase «senza tetto un ritardo poteva solo rimandare» e' falsa, e non nasce in questa bozza:
nasce in [REVIEW-042] RF-001, cioe' e' mia.** Da li' e' passata in `ledger.md:2100-2103`
(*«the pre-existing clause 4 had a floor and no ceiling, so a delayed inclusion could only postpone
a revocation and can now destroy it»*), nella probe `revocation-band-ceiling-is-new-and-inverted`,
nella *Statement* di [DEBT-040], in [REVIEW-044] RF-002 e ora nella premessa di una decisione
dell'operatore. **E' la quarta occorrenza della forma che questa spec ha gia' subito tre volte, e
questa volta l'errore e' del reviewer invece che del Lead.**

**La sola relief che il tetto caduto produce davvero, e il suo prezzo.** Senza tetto il quorum non
deve piu' predire `p` da **due** lati: gli basta scegliere `e` abbastanza avanti perche' la
scadenza `e − F` cada oltre qualunque censura sopportabile. E' una relief reale — la finestra
diventa `(−∞, e − F]` — ma **si compra rimandando l'efficacia della revoca esattamente di quanto
margine si vuole**, ed e' la frase che la bozza deve scrivere al posto di quella attuale, perche' e'
la frase da cui discendono RF-002 e RF-003.

**Scenario d'attacco (invariato dalla decisione).** Una coalizione con **oltre un terzo** del potere
— la soglia di censura di [ADR-018], non quella di quorum — trattiene i precommit su ogni round il
cui proponente includa la `revoke_identity`, e lascia finalizzare i round in cui il proponente e'
suo: la catena avanza, `p` cresce, la revoca non entra. Superata l'altezza `e − F` la transazione
e' morta e il quorum onesto deve rifirmare. **Il tetto caduto non tocca questo scenario di un
blocco.**

**Condizione di chiusura verificabile.** Nessuna fra [ADR-017], `docs/protocol/ledger.md`,
`README.md`, `SECURITY.md`, `core/coblox-core/src/params.rs` e `sim/tools/published_artifacts.toml`
afferma che, senza tetto, un ritardo di inclusione possa solo rimandare una revoca. La frase
sostitutiva nomina il pavimento come il lato che la censura attraversa, e un controesempio
numerico — `F`, `e`, `p = e − F + 1` — sta accanto ad essa o in un test.

RF-002 | category=security-boundary | severity=high | criterion=ADR-017 parte 2; ADR-010 (tre strati su ogni grandezza governata); REVIEW-036 RF-005 | remediation=dichiarare che con una riga senza tetto la banda ha una riga sola, oppure conservare un tetto su `key_compromise` in una forma che la censura non possa muovere

**Con il tetto caduto la tabella di `reason` non discrimina piu': la riga larga e' disponibile a
chiunque firmi, e `reason` non lo verifica nessuno.**

Eseguito, con `F = 10, G = 5, P = 40, p = 50`, su `0..4000`:

```text
key_compromise ammette 3940 altezze su 4000; i reason pianificati ne ammettono 31
pianificati ⊆ key_compromise: True   strettamente piu' larga: True
```

`ledger.md:2113-2120` dichiara gia' il caso degenere nell'altro verso — *«`P = F + G` … the two rows
of the band table coincide … `reason` is read but changes nothing»* — e lo dichiara **come stato
raggiungibile e accettato**. La decisione lo rende **lo stato normale, nel verso peggiore**: non
due righe che coincidono, ma una riga senza limite che contiene l'altra. [ADR-017] scrive gia' che
*«`reason` **non e' verificabile da nessuno**»* e che l'incentivo a misdichiarare *«e' proporzionale
a `P − (F + G)`»*. **Quella grandezza, dopo la decisione, non e' definita**: la latitudine della riga
urgente e' infinita, quindi l'incentivo cambia di segno e di oggetto — non piu' dichiarare
`operator_request` per avere latitudine, ma dichiarare **`key_compromise`** per non averne alcun
limite. `P` e i suoi tre strati di [ADR-010] restano scritti e vincolano **solo un quorum che
scelga di essere vincolato**.

**Scenario d'attacco.** Un quorum vuole tenere in carica un validatore uscente oltre il proprio
`term_expiry_epoch`, o vuole latitudine su un'uscita che una regola pianificata limiterebbe a
`p + P`. Firma la revoca con `reason = "key_compromise"` e `effective_height` a piacere. Nessuna
regola confronta `reason` con un fatto: `reason` e' inerte per costruzione, come [ADR-017]
enumera. Il documento resta valido, la passata di [ADR-012] resta verde, e `P` non ha mai morso.
**Costo per l'avversario: una stringa.**

**Condizione di chiusura verificabile.** O `ledger.md` §*Identity revocation* dichiara in chiaro,
accanto alla tabella, che la riga `key_compromise` non ha tetto e che percio' **il tetto dei
`reason` pianificati e' rinunciabile da chi firma**, con una probe che pinna la frase; oppure la
riga `key_compromise` conserva un tetto, e allora RF-001 impone che il tetto non sia il lato
sbagliato.

RF-003 | category=security-boundary | severity=high | criterion=ADR-017 parte 2, il suo scopo dichiarato; REVIEW-036 RF-001 | remediation=dichiarare, con il numero, per quanti blocchi una chiave compromessa conserva il pieno potere di voto sotto la decisione, e chi sceglie quel numero

**Con il tetto caduto, cio' che vincola ancora `effective_height` su `key_compromise` e' soltanto
`e >= p + F`. La durata per cui una chiave compromessa conserva il pieno potere di voto la sceglie
chi firma, senza limite. E' [REVIEW-036] RF-001 rientrato dalla porta, sul percorso del set.**

Eseguito: `e = 2^40` a `p = 50` e' **valido** sotto la regola senza tetto.

E non e' inerte, perche' tre regole si compongono e **inchiodano la transizione di rimozione
esattamente a `e`**:

- **regola 8** (`ledger.md:1149-1150`): una transizione puo' rimuovere un `node_id` solo se la sua
  revoca ha `effective_height` **al piu'** quell'`activation_height` → non si puo' togliere **prima**
  di `e`;
- **regole 1 e 2** (`ledger.md:1069-1073`): ogni set con `activation_height >= e` che lo contenga e'
  invalido, e ogni blocco ad altezza `>= e` il cui set attivo lo contenga e' invalido → deve essere
  fuori **da** `e`;
- quindi `activation_height = e` esattamente, e **per `e − p` blocchi la chiave compromessa resta
  nel set, con il pieno peso contato nel quorum**, come `ledger.md:222-227` gia' dichiara.

Questa e' precisamente la buca che la sezione esiste per chiudere, e la sezione la scrive da se'
(`ledger.md:1060-1064`): *«The compromised consensus key would keep voting, with its full weight
counted toward quorum, until some later transition that no rule obliges anyone to make»*. Dopo la
decisione la transizione **e'** obbligata — ma a un'altezza che sceglie il firmatario, senza
pavimento, senza tetto e senza i tre strati che [ADR-010] impone a ogni grandezza governata.
`SECURITY.md:113-118` chiama gia' quella finestra *«the window `effective_height` governs, in which
a compromised validator keeps its voting power»*, e dichiara che allungarla richiede **soltanto un
terzo bloccante**. La decisione aggiunge a quel terzo una leva che nessun numero limita.

**Aggravante sul light client.** Un light client applica le regole 1 e 2 con `revoked_validators`
del checkpoint (`ledger.md:1105-1112`): con `e` lontano, **accetta correttamente** blocchi firmati
da un set che contiene una chiave che sa revocata, per `e − p` blocchi. Il MUST di
`ledger.md:1119-1126` lega `max_weak_subjectivity_age_ms` alla durata reale di **`F`**, cioe' alla
grandezza sbagliata: dopo la decisione l'intervallo *«finalizzata ma non ancora efficace»* non e'
piu' `F` ne' `F + G`, e' `e − p`, illimitato. **La riga che lega i due parametri resta scritta e
smette di legare cio' che dice di legare.**

**Scenario d'attacco.** Non serve un quorum ostile. Serve un quorum **onesto e prudente** che, dopo
RF-001, scopra che l'unico modo di sopravvivere alla censura e' scegliere `e` molto avanti: sceglie
`e = p + 20 000`, e da quel momento la chiave compromessa vota per ventimila blocchi, in un incidente
che la sezione dichiara essere quello per cui la regola esiste. **Il danno non e' l'abuso: e' che il
protocollo non ha piu' un'opinione su quel numero.**

**Condizione di chiusura verificabile.** `ledger.md` §*Revocation forces a validator set transition*
porta una frase che dice quanto a lungo, al massimo, una chiave revocata per `key_compromise`
conserva il potere di voto, e nomina chi sceglie quel numero; se la risposta e' *«senza limite, e lo
sceglie il quorum che firma»*, la frase lo dice con quelle parole ed e' pinnata da una probe. In piu'
il MUST su `max_weak_subjectivity_age_ms` nomina la grandezza giusta.

RF-004 | category=correctness | severity=high | criterion=REVIEW-044 RF-002; ADR-017, motivazione della decisione | remediation=rifare l'aritmetica della diluizione e correggere sia [REVIEW-044] RF-002 sia la motivazione della decisione, oppure dichiarare che la decisione poggia su altro

**Il tetto non toglieva il rimedio della diluizione, se non con `G` piccolo. La giustificazione
della decisione e' quantitativamente falsa — e correggo qui il mio [REVIEW-044] RF-002, che
l'aveva prodotta.**

La bozza scrive: *«[REVIEW-044] RF-002 riottiene il rimedio che il tetto aveva tolto: diluire un
lotto di `key_compromise` su piu' transizioni con `effective_height` distinti.»* Il conto e' questo,
eseguito contro il pavimento di contrazione `3 * new > 2 * old` (`ledger.md:1153`):

```text
V=12   peggior caso: 5 transizioni      V=45  : 9      V=81 : 10
V=100  peggior caso: 11 transizioni     V=1000: 16
G=1  -> 2 effective_height distinti ammessi da UNA sola altezza di inclusione
G=5  -> 6      G=17 -> 18      G=80 -> 81
```

**Un'unica altezza di inclusione `p` ammette `G + 1` valori distinti di `effective_height`**, perche'
la banda e' `[p + F, p + F + G]`. Il rimedio richiede tante altezze di efficacia distinte quante le
transizioni di contrazione, cioe' al piu' `⌈log_{3/2}(V)⌉`. **Quindi il tetto toglie il rimedio se e
solo se `G + 1 < numero di transizioni`.** Sui soli bounds tarati del repository
(`recommended.py:54`, `validator_min_set_size_min = 18`, da cui `G >= 17`): **18 disponibili contro
10 necessarie a `V = 81`** — il rimedio c'era. Sotto `permissive_bounds()` (`G = 1`): 2 contro 16 a
`V = 1000` — il rimedio non c'era. **Il controesempio vive nell'albero, come in [REVIEW-044], ma
punta nella direzione opposta a quella che ho scritto allora**: la condizione che rompeva la
diluizione era `G` piccolo, cioe' lo stato che il pavimento di genesi della **prima** correzione ha
appena proibito.

Ne segue una cosa che l'operatore deve sapere prima di confermare: **la rimozione del tetto e' una
risposta sproporzionata a RF-002.** La risposta proporzionata e' un pavimento su `G` **derivato dal
numero di contrazioni**, che e' esattamente il denominatore che [REVIEW-044] RF-002 nominava
(*«il suo denominatore e' `⌈log(V/k)/log(3/2)⌉`, non la dimensione del set»*) e che nessuno ha poi
usato. Il dettaglio: quella relazione e' ancorabile a `validator_max_set_size_max`, che e' una
costante di genesi, cioe' proprio l'ancora che [REVIEW-044] (d) chiedeva al posto di
`validator_min_set_size_min`.

**Caveat che dichiaro invece di nascondere:** il piano diluito con una sola inclusione richiede una
transizione di set **per blocco** per `n` blocchi consecutivi. Se una transizione per blocco non e'
producibile, `G` deve essere piu' largo di `n`. Nessuna misura di questo repository dice quante
transizioni per blocco un set possa impegnare — e' la stessa misura mancante che [ADR-017] dichiara
nella propria sezione *Revisit*.

**Condizione di chiusura verificabile.** [REVIEW-044] RF-002 porta una nota che ne limita
l'affermazione a `G + 1 < ⌈log_{3/2}(V)⌉`, e la motivazione della decisione in [ADR-017] o cita
quella condizione o non cita la diluizione.

RF-005 | category=documentation | severity=high | criterion=ADR-017, riquadro BOZZA; ADR-018 punto 7 e §3; lead_claims_check L2-SUPERLATIVE | remediation=riscrivere le quattro gambe: datare la (1) e dichiararne la scadenza, correggere «sorteggio» in «round-robin pesato», tenere la (3) come l'unica ragione permanente, e dire su che cosa la (4) morda dopo la decisione

**Le tre gambe dichiarate cadute non sono descritte correttamente, e due di esse si contraddicono a
vicenda dentro la stessa bozza.**

- **Gamba (1), *«Nessuna regola assegna un proponente»*, e' un fatto con una scadenza gia'
  programmata, presentato come permanente.** La citazione e' il **punto 7 dei fatti di contesto** di
  [ADR-018], cioe' la descrizione dello stato di `docs/protocol/` **prima** di quella decisione.
  La **Decision §3 della stessa ADR** assegna un proponente, e [SPEC-025] — gia' scritta, riga 64 —
  lo porta in `docs/protocol/`: *«La regola di chi propone: round-robin deterministico sul set
  attivo, indicizzato da `(height, round)` e pesato per potere di voto»*, con un criterio di
  accettazione dedicato alla riga 112. **La bozza cita l'ADR che crea il proponente come prova che
  il proponente non esiste**, e ritira per sempre un argomento su un fatto che una spec consegnata
  cancellera'.
- **Gamba (2) contraddice la gamba (1) e sbaglia il nome del meccanismo.** «La sola regola
  **esistente**» e «nessuna regola» non possono stare entrambe come fatti nello stesso elenco.
  E [ADR-018] §3 non decide un **sorteggio**: decide un *«round-robin deterministico … pesato per
  potere di voto»*, che e' una **rotazione**, deterministica, con periodo pari alla somma dei pesi
  invece che al numero dei membri. **L'errore di nome e' mio**: la bozza trascrive fedelmente
  [REVIEW-044] (b). La **sostanza** della gamba regge — pesi non uniformi sul set di genesi, indice
  `(height, round)` e non `height` — e va tenuta con il nome giusto.
- **Gamba (3) e' l'unica permanente, ed e' descritta correttamente.** Censura a **oltre un terzo**,
  quorum a **oltre due terzi**: nessuna larghezza di finestra difende sopra la soglia di censura.
  Sopravvive a [SPEC-025], sopravvive a qualunque regola di proposta, e da sola basta a ritirare
  l'argomento della rotazione. **Se la bozza tenesse solo questa gamba, il ritiro sarebbe
  inattaccabile.** Aggiungo il meccanismo, che rende lo scenario piu' preciso di come l'ho scritto
  in [REVIEW-044]: per far **crescere `p`** — che e' cio' che serve alla censura — la coalizione non
  puo' limitarsi a fermare i round, perche' un'altezza che non finalizza non fa crescere `p`. Deve
  **lasciare finalizzare** i round proposti da se stessa e far fallire gli altri. Con un terzo
  bloccante e slot di proposta proporzionali al peso, entrambe le cose le riescono: la catena
  avanza, piu' lenta, e la revoca non entra.
- **Gamba (4), dichiarata reggente, cambia oggetto e la bozza non lo dice.** `G >= G_min` resta
  imposto da `check_magnitudes`, quindi la relazione lega ancora il caso peggiore — ma **dopo la
  caduta del tetto `G` non compare piu' in alcun vincolo su `key_compromise`**. Cio' che
  `G_min` lega non e' piu' la finestra del caso urgente: e' il **pavimento di `P`** attraverso
  `P >= F + G`, cioe' la larghezza **minima** della banda pianificata. La frase *«regge»* e' vera e,
  scritta senza il suo nuovo oggetto, e' fuorviante — ed e' la premessa della prima delle due
  questioni aperte.

**Condizione di chiusura verificabile.** Il riquadro non contiene due affermazioni di esistenza in
conflitto sulla regola del proponente; la gamba (1) porta la data e la scadenza ([SPEC-025]); la
parola «sorteggio» non compare riferita a [ADR-018] §3 in alcun artefatto; la gamba (4) nomina la
grandezza che lega dopo la decisione.

RF-006 | category=verification-integrity | severity=high | criterion=ADR-012 (passata su tutti gli artefatti pubblicati); ADR-017 §*Consequences*, che quella enumerazione l'ha inaugurata | remediation=enumerare nella bozza gli artefatti pubblicati che la decisione rende falsi, prima che la decisione sia approvata

**La bozza non nomina un solo artefatto pubblicato che la propria decisione rende falso. E' la
famiglia 1, ed e' la stessa omissione che [ADR-017] rimprovera alla propria prima stesura.**

[ADR-017] §*Consequences* scrive: *«Quattro artefatti pubblicati diventano falsi, e la prima
stesura non li nominava — famiglia 1, la classe che questo progetto ha gia' subito sette volte.»*
La seconda correzione ne rende falsi almeno **dodici** e non ne nomina nessuno. Enumerazione, con
il perimetro `docs/` + `core/` + `sim/` + `README.md` + `SECURITY.md` + `.lmbrain/`:

1. `ledger.md:801-804`, la **riga `key_compromise` della tabella normativa**: `p + F <= e <= p + F + G`.
2. `ledger.md:822-830`: *«The upper bound is the side that carries the cost … the window admissible
   for a given signed `effective_height` is `G + 1` blocks wide for `key_compromise`»*, e
   *«`revocation_effective_grace_blocks` has a genesis floor **as well as a ceiling for that
   reason**»*.
3. `ledger.md:2082-2085`: *«`G` is the width term of the `key_compromise` band … the admissible
   inclusion heights are `[e − F − G, e − F]`»*.
4. `ledger.md:2100-2106`: il paragrafo *«What this floor does not fix»*, che dice che il tetto e'
   nuovo, che distrugge, e che `key_compromise` conserva la finestra piu' stretta — **tutte e tre
   le affermazioni cambiano stato**, la prima per RF-001 e le altre due per la decisione.
5. `ledger.md:2113-2125`: `P = F + G` fa coincidere le righe — falso quando una riga non ha tetto —
   e l'incentivo *«proporzionale a `P − (F + G)`»*, che diventa indefinito (RF-002).
6. `ledger.md:1119-1126`: il MUST che lega `max_weak_subjectivity_age_ms` a `F` (RF-003).
7. `README.md` §`ElectionBounds`, le righe che [REVIEW-044] localizza a `1057-1060`.
8. `core/coblox-core/src/params.rs:41-62`: il commento di
   `revocation_effective_grace_blocks_min`, che descrive `G` come *«the width term of the
   `key_compromise` band»* con la finestra `[e − F − G, e − F]`.
9. `core/coblox-core/src/identity.rs:455-495`: `validate_effective_height`, il suo blocco
   `# Errors` e il ramo `RevocationReason::KeyCompromise` che calcola il tetto; e con essi
   `RevocationError::EffectiveHeightAboveCeiling` sul ramo urgente.
10. **Sei probe** di `published_artifacts.toml`, tutte **verdi oggi** — l'ho eseguito:
    `unrevoked-effective-height-reason-band` (il cui campo `why` dice testualmente *«capping
    key_compromise by p + F + G»*), `revocation-grace-floor-is-a-genesis-bound`,
    `revocation-grace-floor-is-one-rotation-of-the-minimum-set` (gia' condannata da [REVIEW-044]
    RF-001), `revocation-band-ceiling-is-new-and-inverted` (il cui `why` porta l'errore di RF-001),
    `revocation-planned-delay-may-equal-floor-plus-grace`, `guide-revocation-height-has-no-ceiling`.
11. **[DEBT-040] si ribalta.** La sua *Statement* dice che `key_compromise` riceve la finestra piu'
    stretta; dopo la decisione riceve la piu' larga, e la piu' larga possibile. Il debito e'
    **risolto per decisione** attraverso la propria alternativa (*«e' dimostrato che l'ordinamento
    attuale e' corretto»* — qui: l'ordinamento e' invertito nell'altro senso), oppure va riscritto.
    La bozza invece lo cita ancora come aperto e con la premessa vecchia.
12. **`SECURITY.md:105-125`**, che descrive la finestra del percorso del set come una grandezza che
    solo la cadenza allunga; dopo la decisione la allunga anche il firmatario.

**E c'e' una probe la cui ironia va detta**: `guide-revocation-height-has-no-ceiling` pinna la
claim della guida *«No rule caps how far ahead that height may be set»* con `why = «[ADR-017]
bounds effective_height with reason-dependent ceilings»`. **Dopo la decisione la claim che la probe
esiste per smentire torna vera** sul `reason` che conta. Nella stessa area,
`guide-revocation-protection-is-chosen-by-the-revoker` e' ancorata alla frase di `ledger.md:174`
*«This **completely eliminates** the vulnerability window…»* — un universale con un superlativo, che
`lead_claims_check` L2 chiede di enumerare, e che regge solo sul percorso di spesa mentre la claim
che pinna (*«how much a revocation protects is chosen by the quorum that revokes»*) torna vera sul
percorso del set.

**Condizione di chiusura verificabile.** Il riquadro della seconda correzione contiene
un'enumerazione degli artefatti che la decisione rende falsi, con file e riga, nella stessa forma
che [ADR-017] §*Consequences* usa per la parte 1; e ogni probe della lista e' ripuntata prima che
`published_artifacts.py` torni verde.

RF-007 | category=documentation | severity=medium | criterion=ADR-017, coerenza interna del testo normativo | remediation=riscrivere la riga `key_compromise` della tabella e il blocco dei vincoli dentro l'ADR, e togliere il paragrafo che contraddice la decisione

**La bozza decide che il tetto cade e lascia in piedi, nello stesso documento, la tabella che lo
impone e un paragrafo che dice il contrario.**

- La **tabella normativa** della parte 2, tre schermate sopra la decisione, porta ancora
  `key_compromise | p + F <= effective_height <= p + F + G`. Un implementatore legge la tabella.
- Il paragrafo **«Cosa questa correzione non chiude»**, che sta **dentro** il riquadro della seconda
  correzione e dopo la decisione, dice: *«Il tetto della banda e' nuovo di [SPEC-022] … un ritardo
  di inclusione poteva solo rimandare una revoca, e ora puo' distruggerla … `key_compromise` resta
  il caso con il margine minore. E' [DEBT-040], aperto.»* **Tutte e tre le affermazioni sono
  contraddette dalla decisione che le precede di due paragrafi** (e la prima e' falsa comunque, per
  RF-001).
- Il **blocco dei vincoli** non e' toccato: restano `revocation_effective_grace_blocks >= …_min`,
  `<= …_max`, `P >= F + G` e la relazione della rotazione — che [REVIEW-044] RF-001 ha gia'
  condannato e che la bozza dichiara di ritirare **nell'argomento** senza toccarla **nel vincolo**.

**Perche' e' `medium` e non cosmetico.** [ADR-017] e' l'artefatto da cui la spec attuativa copia. La
prima correzione e' diventata testo normativo in tre posti in un giorno; una seconda correzione che
si contraddice produrra' due implementazioni diverse a seconda del paragrafo letto, e la gate di
[ADR-012] non distingue fra i due perche' pinna frasi, non coerenza.

**Condizione di chiusura verificabile.** Nel testo di [ADR-017] non esistono due affermazioni in
conflitto sull'esistenza del tetto su `key_compromise`; la tabella e il blocco dei vincoli portano
la forma decisa.

RF-008 | category=verification-integrity | severity=medium | criterion=la disciplina che la bozza stessa impone; CONTRACT.md §*verification*; QUALITY.md §*Verification standard* | remediation=dare alla regola una gate con un proprietario, oppure declassarla a preferenza dichiarata dell'autore

**«Nessun argomento diventa normativo prima di essere stato attaccato» non e' verificabile, non ha
un proprietario, e nessuna gate la impone.**

Testo attaccato: *«Nessun argomento di questa correzione diventa normativo prima di essere stato
attaccato da AGENT-007 come artefatto a se', invece che dopo, quando una gate lo tiene gia' fermo.»*
Attacco la regola:

1. **Non e' decidibile.** Nulla definisce che cosa faccia di una frase un «argomento», ne' quando un
   attacco sia avvenuto. `published_artifacts.py` verifica che una frase **esista**;
   `lead_claims_check.py` verifica la **forma** di una claim; nessuno dei due sa se una frase sia
   stata attaccata. Le tre passate di [SPEC-023] e le due di [SPEC-022] mostrano che una frase falsa
   passa entrambi.
2. **Non ha un proprietario.** E' scritta in un ADR. Gli ADR non sono letti da alcuno strumento, non
   compaiono nei `verification_gates` di alcuna spec, e non hanno una gate `before-ready`. Una
   regola di metodo che vive solo nel documento che la enuncia e' una **preferenza dell'autore**,
   ed e' esattamente il difetto che [DEBT-031] censisce per la documentazione di modulo del crate.
3. **E' scritta dalla parte che deve rispettarla.** [REVIEW-044] RF-001 cita `lead_claims_check.py`
   come esistente *«proprio perche' per cio' che scrive il Lead autore e revisore sono la stessa
   persona»*. La disciplina nuova ripete la forma che quello strumento esiste per non ripetere: una
   promessa del Lead sul lavoro del Lead.
4. **La sua prima applicazione e' gia' fuori sequenza, e la bozza non lo dice.** La decisione
   dell'operatore sul tetto e' **gia' presa** dentro il riquadro (*«Decisione dell'operatore: il
   tetto della banda cade»*), e le sue tre motivazioni sono argomenti nuovi che nessuno aveva
   attaccato — questa review e' il primo attacco, e ne rompe due. La regola ha funzionato in questa
   occasione **perche' l'operatore l'ha chiesto**, non perche' qualcosa l'abbia imposta.

**Forma verificabile che propongo** (una qualunque delle due, entrambe implementabili oggi):

- una **gate** nella spec attuativa, `GATE-ADVERSARIAL-PRE | kind=manual | owner=agent |
  phase=before-ready`, il cui criterio e': ogni frase che la spec introdurra' in `docs/protocol/`
  come argomento di sicurezza del Lead e' nominata in una REVIEW **precedente** alla spec; oppure
- un **campo obbligatorio** nelle probe: ogni `[[probe]]` il cui `why` porti un argomento di
  sicurezza nomina l'ID della REVIEW che l'ha attaccato, e `published_artifacts.py` fallisce se
  manca. **E' un controllo di dieci righe, e sarebbe stato rosso su
  `revocation-grace-floor-is-one-rotation-of-the-minimum-set`** il giorno in cui e' entrata.

**Condizione di chiusura verificabile.** Esiste uno strumento o una gate versionata che fallisce
quando la regola e' violata, e la sua prima esecuzione e' registrata; oppure [ADR-017] declassa la
frase a *«e' la disciplina che il Lead si impone e che nessuno strumento verifica»*, che e' onesto e
non costa nulla.

RF-009 | category=documentation | severity=low | criterion=ADR-013; ledger.md coerenza interna | remediation=riconciliare le due frasi, o dichiarare quale delle due vale per i parametri di revoca

**`ledger.md` si contraddice sulla convertibilita' dei blocchi in tempo reale, e la contraddizione
e' proprio la premessa che ha imposto la forma «relazione invece che numero» al pavimento di `G`.**

- `ledger.md:2094-2098`: *«The floor is anchored as a relation between genesis constants and not as
  a value, because **no rule of this protocol imposes a cadence** … so `G + 1` blocks do not convert
  into real time, so the floor cannot be justified in seconds.»*
- `ledger.md:875-885`, sulla costante di genesi `block_interval_ms` di [ADR-013]: *«It is what gives
  a **real-time meaning to every quantity this protocol denominates in blocks** —
  `election_epoch_blocks`, `candidacy_close_blocks`, `election_entropy_blocks`,
  **`min_revocation_effective_delay_blocks`** and `election_parameter_min_activation_gap_blocks` —
  and that meaning is the whole of its normative content.»*

Le due frasi sono conciliabili — la seconda parla di significato dichiarato, la prima di regola di
validita' — ma **non sono conciliate in nessun punto**, e l'enumerazione della seconda **contiene
gia' un ritardo di revoca** e non contiene i due parametri nuovi. Conseguenza pratica per la
questione B1: la premessa *«il pavimento non puo' essere giustificato in secondi»* e' piu' debole di
come e' scritta, perche' lo strumento della conversione esiste ([ADR-013]) e la sua tolleranza pure
(la banda di cadenza di [ADR-016], misurata allo step 4b contro `issued_at_ms` di un checkpoint,
che e' l'unico orologio che nessun validatore scrive).

**Condizione di chiusura verificabile.** Il paragrafo del pavimento di `G` cita `block_interval_ms`
e dice perche' la conversione non basta **qui**, oppure la usa; e l'enumerazione di `:878-882`
nomina i tre parametri di revoca o dichiara perche' ne nomina uno solo.

RF-010 | category=documentation | severity=low | criterion=nessuno; nota non bloccante | remediation=dichiarare, dove la banda e' definita, che nessuna regola e nessun documento dice al quorum quanto avanti mettere `effective_height`

**Con il tetto caduto, la scelta di `effective_height` su `key_compromise` diventa una convenzione
operativa non scritta, e un verificatore non puo' distinguere una scelta prudente da una avventata.**

Prima della decisione la grandezza era governata e aveva i tre strati di [ADR-010]. Dopo, la
grandezza che decide sia la resistenza alla censura (RF-001) sia la durata del potere di voto
compromesso (RF-003) e' un numero libero scelto per transazione, e non esiste ne' una regola, ne' un
parametro, ne' una riga di guida che dica al quorum come sceglierlo. **Il rischio non e' un attacco:
e' che la prima revoca reale della devnet lo scelga a caso.**

## Le due questioni aperte: raccomandazione, alternativa, costo

**Non decido al posto dell'operatore. Ogni via qui sotto porta cio' che resta non difeso.**

### Questione 1 — la sorte di `revocation_effective_grace_blocks_min`

**Premessa che cambia la domanda.** La bozza la pone come *«tenerlo senza giustificazione, oppure
toglierlo e restare con il solo `G >= 1` di `check_relations`, cioe' lo stato che [REVIEW-042]
RF-001 aveva dichiarato attaccabile»*. **La seconda meta' e' imprecisa.** Lo scenario di
[REVIEW-042] RF-001 vive sulla riga `key_compromise`; se il tetto cade, `G` **non compare piu'** in
quella riga, e lo scenario non e' riproducibile li'. Cio' che `G_min` continua a fare, attraverso
`P >= F + G`, e' **imporre una larghezza minima alla banda dei `reason` pianificati** — e su quella
banda lo scenario di [REVIEW-042] RF-001 e' riproducibile alla lettera (vedi Questione 2). `G_min`
non e' orfano: ha cambiato mestiere senza che nessuno lo dicesse.

**Raccomandazione: tenerlo, ricollocarlo e riderivarlo — e la derivazione esiste.**

- **Se il tetto su `key_compromise` resta** (la via che raccomando, per RF-001 e RF-004): il
  pavimento di `G` va riderivato contro il **numero di transizioni di contrazione** che una revoca
  di massa richiede, cioe' `revocation_effective_grace_blocks_min + 1 >= C(validator_max_set_size_max)`
  con `C(V)` il peggior caso del pavimento `3 * new > 2 * old` — **16 per `V = 1000`, 10 per
  `V = 81`**, calcolati sopra. E' ancorato a costanti di genesi, non presuppone alcun proponente,
  non presuppone alcuna cadenza, e sopravvive a [SPEC-025] e a [ADR-018]. **E' anche il tetto del
  set attivo al posto del pavimento del pavimento**, che e' cio' che [REVIEW-044] (d) chiedeva.
  *Costo:* il numero e' una funzione, non una costante, e va o tabulato o calcolato in
  `ElectionBounds::validate`; `permissive_bounds()` va rifatto perche' oggi non lo soddisfa.
  *Cosa resta non difeso:* la censura sopra un terzo, che **nessuna larghezza difende** — va scritto
  accanto al pavimento invece che dedotto.
- **Se il tetto cade** (la via della bozza): `G` sopravvive solo dentro `P >= F + G`, quindi il
  pavimento va **rinominato per cio' che fa** — la larghezza minima della banda pianificata — oppure
  fuso in un pavimento diretto su `P` (`P − F + 1 >= …`), che e' la stessa cosa detta senza un
  parametro intermedio. *Costo:* una rinomina tocca `params.rs`, `README.md`, `ledger.md`, tre
  probe e il documento PD-0. *Cosa resta non difeso:* tutto cio' che RF-002 e RF-003 descrivono, che
  nessun pavimento su `G` puo' piu' toccare.

**Alternativa terza, e la porto perche' e' l'unica che chiude RF-001 senza rinunciare al limite:**
**derivare l'altezza di efficacia all'inclusione invece di prenderla dal corpo** —
`e_eff = min(max(e, p + F), p + F + G)`. Ne' un ritardo ne' un anticipo distruggono piu' nulla
(RF-001 chiuso), la discrezione resta limitata da `G` (RF-003 chiuso), e non e' un fork: `p` ed `e`
sono entrambi fatti del blocco includente, letti dagli stessi byte da ogni verificatore — e'
**testualmente l'argomento** con cui [ADR-017] ha stabilito che il `min(...)` sul percorso di spesa
non era un fork, applicato al percorso dove nessuno l'ha provato. *Costo, dichiarato:* `e` nel corpo
diventa **indicativo**, e la grandezza che il checkpoint impegna in `revoked_validators` deve
diventare quella derivata, non quella firmata; una revoca rimasta in mempool a lungo entrerebbe in
vigore a partire dalla sua inclusione tardiva, e l'unico limite a quel ritardo e' `expires_at_ms`
della busta. *Cosa resta non difeso:* la censura sopra un terzo — di nuovo, e per sempre.

### Questione 2 — il tetto sui `reason` pianificati

**Risposta: no, non e' lo stesso difetto a severita' minore, e «chi revoca sceglie il momento» non
e' una difesa.** Costruisco lo scenario invece di ragionarci sopra.

- **Chi censura:** il bersaglio stesso della revoca. Un `validator_misconduct` nomina un validatore
  **in carica**, che ha potere di voto per costruzione — a differenza di `key_compromise`, dove
  l'avversario ha rubato una chiave e potrebbe non averne altre.
- **Con che potere:** un **terzo bloccante**, la stessa soglia di [ADR-018] e di `SECURITY.md:117`.
  Non serve il quorum: serve far fallire i round altrui e lasciar finalizzare i propri, cosi' che
  `p` cresca senza la transazione dentro.
- **Che cosa distrugge:** la finestra di inclusione della banda pianificata e'
  `p ∈ [e − P, e − F]`, larga `P − F + 1`. Superata l'altezza `e − F` la revoca e' invalida e il
  quorum deve **rifirmare**, cioe' rifare un giro a oltre due terzi contro una busta con
  `expires_at_ms`. Il bersaglio ripete la mossa a ogni giro: **una sanzione per cattiva condotta
  diventa un livelock**, e chi lo produce e' precisamente chi la sanzione colpisce.
- **Perche' «chi revoca sceglie il momento» non difende:** la scelta del momento sposta la
  **posizione** della finestra, non la sua **larghezza**. La larghezza e' `P − F + 1`, ed e'
  governata: un set seduto con `P = F + G` e `G` al pavimento la porta al minimo consentito, che e'
  esattamente lo scenario di [REVIEW-042] RF-001 trapiantato sulla riga pianificata — dove nessuno
  l'ha ancora attaccato.

**Raccomandazione:** trattare il tetto pianificato come il tetto urgente, non diversamente. Se la
via scelta e' il clamp (alternativa terza sopra), si applica a entrambe le righe e la questione si
chiude da sola. Se il tetto pianificato resta secco, allora `P` ha bisogno di un pavimento espresso
come **larghezza** (`P − F + 1 >= C`, con la stessa `C` della Questione 1) e non del solo
`P >= F + G`. *Costo:* un vincolo di genesi in piu' e una passata di [ADR-012]. *Cosa resta non
difeso in ogni caso:* la censura sopra un terzo, che sposta `p` finche' vuole; contro di essa
l'unico rimedio strutturale non e' una larghezza ma il clamp, perche' toglie del tutto la scadenza.

## Verdetto raccomandato

**Non approvare la bozza nella forma attuale.**

La conclusione che la bozza raggiunge — l'argomento della rotazione va ritirato — e' **giusta**, e
la gamba (3) la sostiene da sola. Ma la **decisione** che la bozza porta poggia su tre affermazioni
di cui due sono false (RF-001, RF-004) e la terza incompleta (RF-005 gamba 4), e produce tre
proprieta' che nessuno ha scelto (RF-002, RF-003, RF-010).

**In una riga: la bozza toglie il lato della banda che nessun avversario attraversa, e lascia in
piedi quello che l'avversario attraversa davvero.**

Se l'operatore conferma comunque la caduta del tetto — e' una sua prerogativa, e la relief di
RF-001 e' reale ancorche' diversa da come e' scritta — allora RF-002, RF-003, RF-006 e RF-007 vanno
chiusi **nello stesso giro**, perche' descrivono cio' che la decisione lascia indifeso e cio' che
rende falso.

## Required follow-up

- **RF-001 per primo**: e' un errore mio, propagato in `ledger.md`, in una probe, in [DEBT-040] e
  in [REVIEW-044] RF-002, e oggi e' la premessa di una decisione. Va corretto in tutti e cinque i
  posti, non solo nella bozza.
- **RF-004 insieme**: la nota che limita [REVIEW-044] RF-002 va scritta nella review, che e' in
  `changes-requested` e verra' riletta dall'implementatrice.
- **RF-006 prima dell'approvazione**: l'enumerazione degli artefatti che la decisione rende falsi e'
  lavoro del Lead e non dell'implementatrice, e [ADR-012] la impone comunque alla spec attuativa.
- **[DEBT-040] va deciso**, non lasciato citato con la premessa vecchia: risolto per decisione,
  riscritto al contrario, o superseduto.
- **RF-008 va portato all'operatore come voce a se'**: e' l'unica delle dieci che riguarda il
  processo, ed e' quella che decide se ci sara' una quarta occorrenza.
