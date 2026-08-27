---
id: REVIEW-046
# Note: Quote the title if it contains a colon
title: "Critica avversariale della BOZZA v2 di ADR-017 parte 2: il clamp non e' iniettivo, e nessuna delle regole che dice di sostituire e' stata abrogata"
status: pending
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-022
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-LEAD
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security-boundary, correctness, robustness, documentation, verification-integrity]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events: []
links: [DEBT-040, DEBT-033, DEBT-045]
created: 2026-08-27
updated: 2026-08-27
tags: [security, review, ledger, consensus]
related_decisions: [ADR-017, ADR-018, ADR-010, ADR-012, ADR-013]
activity:
  - date: 2026-08-27
    action: "created"
---
# Review

> **Critica avversariale di una BOZZA**, richiesta dall'operatore prima che il testo diventi
> normativo. Oggetto: il riquadro *«SECONDA CORREZIONE, del 2026-08-27 — BOZZA v2»* di [ADR-017]
> parte 2, i suoi cinque punti, e le tre cose che dichiara di non stabilire.
> Terza critica della stessa parte 2, dopo [REVIEW-036] (prima stesura), [REVIEW-044] e
> [REVIEW-045] (v1). Eseguita sull'albero a `7c95267`, `git status` pulito all'inizio e alla fine.
>
> **La regola centrale della v2 e' mia.** `e_eff = min(max(e, p + F), p + F + G)` e' la «alternativa
> terza» che ho proposto in [REVIEW-045] ed e' entrata nella bozza senza che nessuno l'attaccasse.
> E' la condizione esatta in cui sono nate le tre cadute precedenti di questa parte, e questa review
> la attacca per prima.

## Outcome

**La v2 non regge, ma cade in modo diverso dalla v1.** Sei `high`, tre `medium`, due `low`.

**La v1 aveva una premessa falsa. La v2 ha le premesse giuste e una regola incompleta.** Le due
verifiche che porta sono corrette e le ho rifatte: l'aritmetica di `F=10, G=5, e=100` e' trascritta
fedelmente (estremo superiore `p = 90` identico con e senza tetto, `p = 91` invalido in entrambi i
casi per il pavimento), il ritiro della rimozione del tetto e' giusto, e la gamba superstite regge.
**Cio' che non regge e' il punto 3.**

**Il colpo principale e' che il clamp non e' iniettivo, e questo lo rende un'arma.** Sotto `e_eff`
un lotto di `key_compromise` che il quorum onesto ha **deliberatamente diluito** su altezze distinte
— che e' il rimedio dichiarato a `ledger.md:1986` — **collassa su una sola altezza di efficacia** non
appena l'inclusione e' ritardata abbastanza. A quel punto nessun set valido esiste e la catena si
ferma per sempre. Sotto la banda dura le stesse revoche sarebbero state **invalide**, cioe' un
fallimento rumoroso e recuperabile. **La v2 converte un rifiuto in un arresto permanente, e chi
sceglie il momento e' un avversario a un terzo bloccante.** E' [REVIEW-044] RF-002 — che la v2 non
nomina — reso raggiungibile dall'avversario invece che dall'errore del difensore.

**Il secondo colpo e' che nessuna regola e' stata abrogata.** Il punto 3 dichiara che *«un ritardo
sposta l'efficacia invece di invalidare la transazione»*. Perche' quella frase sia vera devono
cadere **tre MUST** — `ledger.md:795`, la clausola 4 di `ledger.md:1076-1080`, e la tabella
normativa di `ledger.md:803-804` — piu' la funzione spedita
`identity.rs::validate_effective_height`. La v2 non ne nomina **nessuna**, e la tabella della banda
resta scritta tre schermate sopra, nello stesso ADR, nella forma di vincolo di validita'. Con quelle
righe in piedi il clamp non produce l'effetto che la v2 gli attribuisce: la censura distrugge
esattamente come prima.

**Il terzo e' che il pavimento di genesi su `G` cambia segno.** Sotto la banda dura `G` era la
larghezza della finestra difensiva e un pavimento la proteggeva. Sotto il clamp `G` e' **la
latitudine dell'efficacia che il protocollo concede a chi firma**, e il pavimento la rende
**obbligatoria**: sui soli bounds tarati del repository il protocollo garantirebbe al firmatario
almeno **17 blocchi** di ritardo discrezionale sull'efficacia di una revoca per compromissione. Il
punto 4 della v2 afferma invece — citandomi — che `G_min` *«non e' orfano ma ha cambiato mestiere»*.
**Quella affermazione l'ho scritta io nel ramo in cui il tetto CADE.** La v2 ritira quel ramo e
trapianta la conclusione.

**Cio' che regge, e va detto.** Il punto 1 (ritiro dell'argomento della rotazione sulla sola gamba
della soglia) e il punto 2 (ritiro della rimozione del tetto) sono **corretti nella conclusione e
nell'aritmetica**. Il punto 5 e' la forma verificabile che avevo chiesto, ed e' migliore della
regola che sostituisce. La scelta di non derivare a mano l'enumerazione di [ADR-012] e' **giusta in
linea di principio** — ed e' lo strumento nominato a non poterla produrre (RF-006).

## Cosa ho ESEGUITO, e cosa ho solo LETTO

**Eseguito** (Windows 11, `python 3`, albero pulito prima e dopo, nessun file dell'albero toccato):

- `python sim/tools/published_artifacts.py` → **PASS**, `C1-DOMAIN 40`, `C2-TAG 24`,
  `C3-FIXTURE-ID 20`, `C4-VALUE 60`, `C5-MIRROR 53`, `C7-COVERAGE 51`, `C8-ENCODING 1`,
  `C9-EXAMPLE 1`, `C5-DISCOVERED 67`, `C10-PROBE 172`, `C11-CLAIMDOC 8`, exit `0`. **E' la prova di
  RF-006**: la passata che il punto 4 incarica di produrre l'enumerazione e' verde oggi e non
  enumera nulla.
- **Un controesempio aritmetico eseguibile** che trascrive
  `core/coblox-core/src/identity.rs::validate_effective_height` (il predicato **spedito**, non la
  prosa), il clamp `e_eff` del punto 3, e il pavimento di contrazione `3 * member_count(new) >
  2 * member_count(old)` (regola 10, `ledger.md:1153`). Calcola: (a) la finestra di inclusione con e
  senza tetto; (b) l'immagine di un lotto diluito sotto il clamp al crescere di `p`; (c) il numero
  di transizioni di contrazione necessarie; (d) la latitudine che `G` concede sotto il clamp; (e) il
  tetto che il clamp produce per un `reason` pianificato. Uscite riportate dentro i finding. Lo
  script vive nella mia scratchpad e non tocca l'albero.

**Letto e non eseguito:** [ADR-017] per intero; [ADR-018] (fatti 1–8, Decision §§1–5, *Consequences*,
*Review conditions*); [REVIEW-036], [REVIEW-042], [REVIEW-044], [REVIEW-045] per intero; [DEBT-040];
`docs/protocol/ledger.md` (§*Identity revocation* `:795-835`, §*Revocation forces a validator set
transition* `:1058-1160`, §*Magnitudes, not only relations* `:2075-2130`, `:836` la fixture canonica
di `revoke_identity`); `docs/protocol/README.md` (§*weak subjectivity checkpoint* `:1503-1543`);
`core/coblox-core/src/params.rs` (`ElectionBounds`, `check_revocation_grace_floor`,
`check_relations`, `check_magnitudes`, `ELECTION_PARAMETERS`);
`core/coblox-core/src/identity.rs` (`validate_effective_height`,
`validate_effective_height_in_block`); `core/coblox-core/src/light_client.rs:212-290`;
`core/coblox-core/src/merkle.rs:273-277`; `core/coblox-core/tests/common/mod.rs`
(PD-0 `:238-255`, `permissive_bounds()` `:349-366`); `sim/coblox_sim/recommended.py:49-58`;
`sim/tools/published_artifacts.py` (le undici classi e la definizione di C10).

**Non guardato, dichiarato:** `cargo test` non eseguito in questa passata — nessuna affermazione qui
dipende dall'esito della suite, e tutte le trascrizioni di codice sono letture dirette del sorgente;
`GATE-CI-GREEN`; `apps/`, `dist/`, FFI; la superficie di [DEBT-034]; `sim/coblox_sim/` oltre a
`recommended.py`; [SPEC-025] oltre a quanto [REVIEW-045] ne riporta.

**Perimetro di ogni affermazione sul proponente e sulle soglie:** riguarda la regola **decisa** in
[ADR-018] e il predicato di quorum **gia' spedito**. Un motore di consenso non esiste in `core/`.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security-boundary | severity=high | criterion=ADR-017 parte 2, punto 3 della BOZZA v2; ADR-012 (una regola di validita' che cambia va scritta dove la regola vive) | remediation=nominare nel punto 3 le tre regole di validita' che il clamp sostituisce e dichiararle abrogate, oppure dichiarare che restano — nel qual caso il punto 3 non produce l'effetto che gli e' attribuito e va riscritto

**Il punto 3 dichiara un effetto che nessuna regola produce. `e_eff` e' introdotto senza che
nessuna delle regole che invalidano una revoca ritardata sia toccata.**

Testo attaccato, spazi normalizzati perche' attraversa due a capo ([ADR-017], riquadro BOZZA v2,
punto 3):

> *«Un ritardo **sposta l'efficacia** invece di invalidare la transazione, quindi la censura torna a
> poter solo rimandare — questa volta sul lato giusto — e il tetto continua a limitare la discrezione
> del quorum.»*

Perche' quella frase sia vera devono cadere quattro cose, e la v2 non ne nomina nessuna:

1. **`ledger.md:795`** — *«The effective height MUST be later than the block proposing the
   revocation.»* E' uno dei **tre MUST** che [ADR-017] stesso enumera come la superficie che tocca
   piu' da vicino.
2. **La clausola 4, `ledger.md:1076-1080`** — *«`effective_height` MUST be at least
   `min_revocation_effective_delay_blocks` above the height of the block proposing the
   revocation»*. E' il lato che la censura attraversa, come il punto 2 della v2 stessa ha appena
   accertato eseguendo.
3. **La tabella normativa, `ledger.md:803-804`** — `key_compromise | p + F <= effective_height <=
   p + F + G`, introdotta da *«`effective_height` MUST lie within the reason-dependent band»*.
4. **`core/coblox-core/src/identity.rs:460-512`**, `validate_effective_height`, che quella tabella
   la **implementa gia'**, restituendo `EffectiveHeightBelowFloor` e `EffectiveHeightAboveCeiling`.
   E' codice spedito, non prosa.

**E la contraddizione e' dentro l'ADR, non fuori.** La tabella a due righe della parte 2 —
`key_compromise | p + F <= effective_height <= p + F + G` — sta **tre schermate sopra** il riquadro,
nella forma di **vincolo di validita'**, e la v2 non la tocca. Un implementatore che legge la
tabella scrive il predicato che c'e' gia'; uno che legge il punto 3 scrive un clamp. Sono due
protocolli diversi. **E' [REVIEW-045] RF-007 non chiuso**, trasportato dalla v1 alla v2 con l'oggetto
cambiato: la' la tabella contraddiceva la caduta del tetto, qui contraddice la derivazione.

**Scenario, eseguito.** `F = 10`, `G = 5`, corpo firmato con `e = 100`. Una coalizione a **oltre un
terzo** ritarda l'inclusione fino a `p = 91`:

```text
capped   inclusion heights ammesse: 85 .. 90
uncapped inclusion heights ammesse:  0 .. 90
p=91  capped: (False, 'floor e >= p+F')   |  uncapped: (False, 'floor e >= p+F')
```

Con le quattro righe sopra in piedi, a `p = 91` la transazione e' **invalida** e nessun clamp viene
mai calcolato, perche' il verificatore la rifiuta prima. **La v2 non ha cambiato lo scenario che
dice di avere chiuso.** Il costo dell'ambiguita' non e' teorico: la prima correzione di questa parte
e' diventata testo normativo in tre posti in un giorno.

**Condizione di chiusura verificabile.** Il punto 3 di [ADR-017] enumera per file e riga le regole
che il clamp sostituisce, e per ciascuna dice se cade o resta; e in [ADR-017] non esistono due
affermazioni in conflitto su cosa renda invalida una `revoke_identity` la cui `effective_height` sia
fuori banda all'inclusione.

RF-002 | category=security-boundary | severity=high | criterion=REVIEW-044 RF-002; GATE-SECREVIEW; QUALITY.md §*Required engineering standard* (percorsi di fallimento) | remediation=dichiarare che `e_eff` non e' iniettivo, che il collasso di un lotto diluito e' raggiungibile da un avversario a un terzo bloccante, e che l'esito e' un arresto permanente — oppure scegliere una forma del clamp che preservi le distinzioni

**Il clamp non e' iniettivo. Sotto ritardo collassa un lotto diluito su una sola altezza di
efficacia, e la catena si ferma per sempre. La v2 non nomina [REVIEW-044] RF-002, ed e' il finding
che `e_eff` peggiora invece di risolvere.**

`e_eff = min(max(e, p + F), p + F + G)` e' costante su `(−∞, p + F]` e su `[p + F + G, +∞)`. Due
`effective_height` distinti firmati insieme hanno la **stessa immagine** appena l'inclusione e'
abbastanza tardiva. Il rimedio che `ledger.md:1986` dichiara — *«over several transitions it can»* —
richiede **altezze di efficacia distinte**, perche' la regola 8 (`ledger.md:1149`) fa di un
`effective_height` una scadenza e non un calendario.

**Eseguito**, con il pavimento di contrazione della regola 10 e i bounds tarati
(`recommended.py:54`, `validator_min_set_size_min = 18` → `G >= 17`), `F = 10`, `V = 45`:

```text
V=45: at most 14 removable per transition; k=15 needs 2 transitions -> 2 DISTINCT effective heights
  included at p=100: hard band -> all valid? True ; v2 clamp -> 15 distinct e_eff [110,111,112,113]... | halt? False
  included at p=118: hard band -> all valid? False; v2 clamp ->  1 distinct e_eff [128]              | halt? True
  included at p=190: hard band -> all valid? False; v2 clamp ->  1 distinct e_eff [200]              | halt? True
```

**Scenario d'attacco, e la mossa che ferma la catena e' quella corretta.** Un avversario compromette
`k = 15` chiavi di consenso su un set di `45`, uno sopra il limite di contrazione in un passo. Il
quorum onesto fa **esattamente la cosa giusta**: firma il lotto **diluito**, `e = 110 … 124`, tutte
dentro la banda per l'altezza di inclusione prevista `p = 100` — sotto la banda dura sono tutte e
quindici **valide**, verificato sopra. L'avversario, che ha gia' un terzo bloccante fra le chiavi che
ha rubato, ritarda l'inclusione fino a `p = 118`:

- **sotto la banda dura**: tutte e quindici sono invalide, il giro di firma va rifatto. Fallimento
  **rumoroso**, danno = una rifirma, catena viva;
- **sotto il clamp della v2**: tutte e quindici sono **valide** e collassano su `e_eff = 128`. La
  regola 8 vieta di rimuoverne alcuna prima di `128`; la regola 2 rende invalido ogni blocco a
  `>= 128` il cui set le contenga; la regola 10 vieta la transizione che le toglie tutte insieme.
  **Non esiste alcun set valido a `128`, quindi non esiste alcun blocco valido a nessuna altezza
  `>= 128`. La catena si ferma e non riparte**, e le revoche sono finalizzate e non si disfano.

**Su PD-0 (`V = 12`, `F = G = 1`) bastano `k = 4` e un ritardo di dieci blocchi.** Il costo per
l'avversario e' il ritardo che ha gia' pagato per censurare.

**Cio' che questo finding aggiunge a [REVIEW-044] RF-002.** Quello descriveva un lotto con **un solo
`effective_height`**, cioe' un errore del difensore. Questo descrive un lotto **correttamente
diluito** che l'avversario **rende** coincidente. La v2 non nomina RF-002 in nessun punto: la
domanda posta era se `e_eff` lo risolva. **Non lo risolve: ne rimuove la protezione — la banda dura
rifiutava il lotto collassato — e ne sposta l'innesco dal difensore all'attaccante.**

**Condizione di chiusura verificabile.** `ledger.md` §*Revocation forces a validator set transition*
dichiara che sotto la derivazione due `effective_height` distinti possono avere la stessa efficacia,
nomina il pavimento di contrazione fra le cause dello stallo, e dichiara chi sceglie il momento del
collasso; oppure il clamp e' sostituito da una forma che preservi l'ordine stretto (per esempio uno
spostamento `e_eff = e + max(0, p + F − e)` limitato, che non appiattisce), e un test di conformita'
sul lotto diluito sotto ritardo esiste ed e' rosso sulla forma attuale.

RF-003 | category=security-boundary | severity=high | criterion=ADR-017 parte 2, punto 4 secondo trattino; ADR-010 (tre strati su ogni grandezza governata); REVIEW-042 RF-001 | remediation=riderivare il pavimento di `G` sotto il clamp, o toglierlo e tenere il solo tetto — e in ogni caso dichiarare quanti blocchi di ritardo discrezionale il protocollo garantisce a chi firma

**Sotto il clamp il pavimento di genesi su `G` cambia segno: non protegge piu' una finestra
difensiva, garantisce una latitudine minima all'avversario. E l'affermazione della v2 che `G_min`
«ha cambiato mestiere» e' trapiantata dal ramo che la v2 ha appena ritirato.**

Testo attaccato ([ADR-017], riquadro BOZZA v2, punto 4):

> *«**La sorte di `revocation_effective_grace_blocks_min`.** [REVIEW-045] accerta che **non e'
> orfano**: ha cambiato mestiere e impone ora la larghezza minima della banda pianificata via
> `P >= F + G`.»*

**L'ho scritto io, e l'ho scritto sotto un'ipotesi che la v2 non tiene piu'.** [REVIEW-045],
Questione 1, apre quel ragionamento con *«se il tetto cade, `G` non compare piu' in quella riga»* e
lo chiude con *«**Se il tetto cade** (la via della bozza)»*. **Il punto 2 della v2 ritira la caduta
del tetto.** Nel ramo che la v2 ha scelto, `G` compare eccome nella riga `key_compromise`: e' il
termine di larghezza del clamp. La conclusione e' stata trasportata attraverso il ramo che la
produceva. E' la stessa forma della gamba (1) che il punto 1 ritira: **un fatto vero in un contesto,
citato in un contesto che lo ha cancellato.**

**Che mestiere fa `G` sotto il clamp, e perche' il pavimento e' dalla parte sbagliata.** Sotto la
banda dura, `G + 1` era la **larghezza della finestra di inclusione**: piu' largo, piu' facile
sopravvivere a un ritardo. Un pavimento la difendeva, ed era [REVIEW-042] RF-001. Sotto il clamp la
finestra di inclusione **non esiste piu'** — ogni `p` e' ammissibile — e `G` diventa una cosa sola:
**di quanto chi firma puo' spingere avanti l'efficacia rispetto al minimo `p + F`**. Eseguito:

```text
PD-0 (F=1, G=1)                        : e_eff in [p+1,  p+2]      -> fino a 2 blocchi
bounds tarati (F=10, G>=G_min=17)      : e_eff in [p+10, p+27]     -> fino a 27 blocchi
G al tetto di genesi (1e6, permissive) : e_eff in [p+10, p+1000010]-> fino a 1.000.010 blocchi
```

`G >= revocation_effective_grace_blocks_min` e' imposto da `check_magnitudes`
(`params.rs:674`) e `revocation_effective_grace_blocks_min + 1 >= validator_min_set_size_min` da
`check_revocation_grace_floor` (`params.rs:130-137`). Sui soli bounds tarati che il repository
possieda, **il protocollo obbliga la genesi a concedere al firmatario almeno diciassette blocchi di
ritardo discrezionale sull'efficacia di una revoca per compromissione**, e li obbliga con un
argomento — la rotazione del set minimo — che il punto 1 della stessa v2 dichiara ritirato.

**Scenario, ed e' [REVIEW-036] RF-001 sul percorso del set.** Un quorum vuole tenere una chiave
compromessa dentro il set il piu' a lungo possibile senza violare nulla: firma `e` alto, il clamp lo
riporta a `p + F + G`, e per `F + G` blocchi quella chiave **conta a piena potenza di voto**, come
`ledger.md:222-227` e `ledger.md:1060-1064` gia' dichiarano. La differenza rispetto alla v1 e' che
il numero e' **limitato**; la differenza rispetto alla banda dura e' che ora e' **garantito dal
basso dalla genesi**, e nessuno dei tre strati di [ADR-010] ha mai valutato quel numero in questo
mestiere. `revocation_effective_grace_blocks_max` **non ha alcun valore tarato**: `recommended.py`
non porta nessuno dei tre parametri di revoca (e' [DEBT-045] e [REVIEW-044] RF-003), quindi l'unica
magnitudine presente nell'albero e' `1_000_000` di `permissive_bounds()`.

**Condizione di chiusura verificabile.** [ADR-017] dichiara, con il numero, per quanti blocchi al
massimo una chiave revocata per `key_compromise` conserva il potere di voto sotto la derivazione — la
risposta e' `F + G` — e dice che il pavimento di genesi su `G` ne fissa il **minimo garantito**, non
il massimo difeso; e la derivazione del pavimento e' rifatta sotto il clamp o dichiarata ritirata.
**Sotto il clamp `G = 0` e' la scelta che toglie ogni discrezione**, e va nominata fra le
alternative invece che esclusa da un pavimento ereditato.

RF-004 | category=correctness | severity=high | criterion=ADR-017 parte 2, la tabella a due righe; REVIEW-042 RF-008 (decisione dell'operatore in sospeso); REVIEW-045 Questione 2 | remediation=dire a quali `reason` si applica il clamp e con quale tetto, e riconciliare il punto 3 con il terzo trattino del punto 4

**La formula normativa della v2 non nomina ne' `reason` ne' `P`. La v2 adotta il clamp per una riga
di una tabella a due righe, e nello stesso riquadro dichiara aperta la questione che il clamp
chiudeva.**

La formula scritta e' `e_eff = min(max(e, p + F), p + F + G)`. `F` e `G` sono i termini della riga
`key_compromise`. `P = max_planned_revocation_delay_blocks`, che governa l'altra riga, **non compare**.
Eseguito, con `F = 10, G = 5, P = 40, p = 100`:

```text
key_compromise      : tetto banda dura = 115 | tetto v2 clamp = 115
validator_misconduct: tetto banda dura = 140 | tetto v2 clamp = 115
```

Restano tre letture, e la v2 non sceglie:

1. **il clamp vale per tutti i `reason` con `F` e `G`** → `P` diventa **inerte sempre**, non solo
   nello stato `P = F + G` che `ledger.md:2113-2125` dichiara raggiungibile. Questo **cambia**
   [REVIEW-042] RF-008, che e' una decisione dell'operatore ancora in sospeso, e la cambia senza
   dirlo. *Non la risolvo: la segnalo, come il mio incarico impone.*
2. **il clamp vale solo su `key_compromise`, e i `reason` pianificati tengono il tetto duro
   `p + P`** → il protocollo ha **due meccaniche diverse** sulla stessa grandezza, selezionate da un
   campo che [ADR-017] dichiara *«non verificabile da nessuno»*: su una riga un ritardo sposta, su
   l'altra distrugge. E chi sceglie quale, scegliendo `reason`, e' chi firma.
3. **il clamp vale su entrambe con tetti diversi** — la lettura piu' probabilmente intesa, e la sola
   che il testo **non** scrive.

**E il terzo trattino del punto 4 contraddice il punto 3.** La v2 scrive che *«il tetto sui `reason`
pianificati … resta aperto»*, citandomi. Ma [REVIEW-045], Questione 2, chiude quella raccomandazione
cosi': *«Se la via scelta e' il clamp … si applica a entrambe le righe e la questione si chiude da
sola.»* **La v2 adotta il clamp e dichiara aperta la questione che l'adozione del clamp chiudeva.**
E' o un'incoerenza, o la lettura (2) non dichiarata — nel qual caso il livelock che [REVIEW-045]
costruisce sulla riga pianificata (il bersaglio di `validator_misconduct` censura la propria
sanzione con un terzo bloccante, all'infinito) **resta aperto per scelta**, e va scritto che lo e'.

**Condizione di chiusura verificabile.** Il punto 3 di [ADR-017] porta la forma completa della
derivazione per **entrambe** le righe della tabella, con il tetto di ciascuna; e il testo non
contiene insieme l'adozione del clamp e la dichiarazione che la questione del tetto pianificato
resta aperta senza dire perche' il clamp non la tocchi.

RF-005 | category=security-boundary | severity=high | criterion=ledger.md regole 1, 2, 6 e 8; README.md §*weak subjectivity checkpoint*; REVIEW-045, i costi dichiarati dell'alternativa terza | remediation=dire quale grandezza leggono le quattro regole del percorso del set e cosa impegna il checkpoint sotto la derivazione, e dichiarare che quella grandezza non e' piu' impegnata da alcuna firma

**Quattro regole e un impegno crittografico leggono `effective_height`. La v2 introduce una seconda
grandezza e non dice quale delle due leggano. Per il light client la risposta non puo' essere
`e_eff`, perche' non puo' calcolarla.**

Le regole che leggono il campo, tutte in `ledger.md` §*Revocation forces a validator set transition*:

- **regola 1** (`:1069`) — un `ValidatorSet` con `activation_height >= effective_height` che contenga
  il `node_id` e' invalido;
- **regola 2** (`:1071`) — un blocco ad altezza `>= effective_height` il cui set attivo lo contenga
  e' invalido;
- **regola 6** (`:1110-1112`) — il **light client** MUST applicare le regole 1 e 2 *«with the data the
  checkpoint gives it»*;
- **regola 8** (`:1149`) — una transizione puo' rimuovere un `node_id` solo se la sua revoca ha
  `effective_height` al piu' quell'`activation_height`.

**Il light client non puo' calcolare `e_eff`.** Lo dice il documento stesso, e lo dice come premessa
di una correzione: *«A light client … **never sees transactions**, so it does not know that a
revocation exists or whom it names»* (`ledger.md:1083-1090`). `e_eff` e' funzione di `p`, cioe'
dell'altezza del blocco che include la transazione. Un client che non vede la transazione non vede
`p`. **La sola via e' che il checkpoint impegni `e_eff` invece di `e`** — e il checkpoint porta oggi
`revoked_validators:[{node_id, effective_height}]` (`README.md:1503`) con `revocation_root` calcolato
su quelle coppie (`merkle.rs:273-277`, `light_client.rs:288`).

**Ed e' qui la perdita.** Oggi `e` e' nel **corpo firmato** e impegnato nell'ID della transazione:
chiunque abbia la transazione puo' provare che quella coppia e' giusta, e il `revocation_root`
impegna una grandezza che una firma sostiene. Sotto la derivazione la grandezza operativa **non e'
impegnata da alcuna firma**: e' un fatto di posizione in catena, e il checkpoint — emesso dallo
stesso quorum — e' l'unico canale che la porta al light client, che non ha modo di ricalcolarla.
`revocation_root` smette di impegnare cio' che conta, e la difesa che `ledger.md:1096-1102` ha
costruito proprio perche' *«adding a header field … would still be authenticated by the very set
that is compromised»* torna a poggiare su un'asserzione del set.

**Aggravante, e vale anche fra nodi pieni.** `ledger.md:1119-1126` lega
`max_weak_subjectivity_age_ms` alla durata reale di **`F`**. Sotto il clamp l'intervallo
*«finalizzata ma non ancora efficace»* e' `e_eff − p ∈ [F, F + G]`, quindi la riga nomina il termine
minimo e ignora `G`. E' la stessa riga che [REVIEW-045] RF-003 ha gia' colpito nell'altro ramo: **la
v2 cambia il ramo e non tocca la riga.**

**E [REVIEW-045] questo costo lo aveva dichiarato**, testualmente: *«`e` nel corpo diventa
indicativo, e la grandezza che il checkpoint impegna in `revoked_validators` deve diventare quella
derivata, non quella firmata»*. **La v2 adotta la proposta e lascia fuori tutti e tre i costi che la
proposta portava con se'.** Chi ha scelto ha letto i costi; chi implementera' leggera' l'ADR.

**Condizione di chiusura verificabile.** [ADR-017] o `ledger.md` dice, per ciascuna delle regole 1,
2, 6 e 8, se legge `e` o `e_eff`; `README.md` §*weak subjectivity checkpoint* dice quale delle due
`revoked_validators` porta; e se e' `e_eff`, una frase dichiara che il light client accetta quella
grandezza sulla parola del quorum che emette il checkpoint, con una probe che la pinna.

RF-006 | category=verification-integrity | severity=high | criterion=ADR-012; ADR-017 §*Consequences*, che l'enumerazione l'ha inaugurata; REVIEW-045 RF-006 | remediation=enumerare a mano gli artefatti che la derivazione rende falsi, perche' lo strumento nominato non e' in grado di produrli, e usare la passata per cio' che sa fare

**Il punto 4 assegna l'enumerazione degli artefatti resi falsi allo strumento della passata di
[ADR-012]. Quello strumento non puo' produrla: verifica che una frase **esista**, non che sia vera.
L'ho eseguito, ed e' verde.**

Testo attaccato ([ADR-017], riquadro BOZZA v2, punto 4, primo trattino):

> *«L'insieme di questa correzione va prodotto **eseguendo la passata di [ADR-012]** con lo strumento
> versionato, non derivato a mano.»*

`sim/tools/published_artifacts.py` documenta le proprie undici classi. La sola che tocchi la prosa
normativa e' **C10-PROBE**, definita cosi' nel file: *«a normative passage the manifest pins is no
longer there»*. Rileva una frase **spostata o cancellata**, mai una frase **diventata falsa**. Le
altre dieci classi sono su domini, tag, ID di fixture, digest, trascrizioni, orfani, copertura,
codifica, esempi e classificazione dei markdown. **Nessuna legge un argomento.** Eseguito adesso:

```text
C10-PROBE 172 candidate(s) checked ... published-artifact inventory: PASS   (exit 0)
```

**La passata, eseguita oggi contro la v2, enumera l'insieme vuoto.** Assegnarle il lavoro non e' un
modo rigoroso di farlo: e' il modo di non farlo, e produce un artefatto verde su un documento
falso — che e' esattamente cio' che [REVIEW-044] RF-001 ha censito
(`revocation-grace-floor-is-one-rotation-of-the-minimum-set` pinnata e verde su una frase falsa) e
che [REVIEW-042] RF-002 e RF-005 avevano gia' censito due volte prima.

**E nemmeno il punto 5 lo coglie.** La disciplina nuova — *«ogni `[[probe]]` il cui `why` porti un
argomento di sicurezza nomina l'ID della review che lo ha attaccato»* — e' migliore della regola che
sostituisce e la sostengo. Ma verifica che un ID **sia scritto**, non che l'argomento regga ancora
dopo che l'ADR e' cambiato: una probe con `why = «[REVIEW-045]»` resta verde mentre la frase che
pinna diventa falsa per il punto 3.

**Quello che la derivazione rende falso, e che nomino perche' la v2 non lo fa** — quattro dei
dodici di [REVIEW-045] RF-006 tornano veri (il tetto resta), questi no:

1. `ledger.md:2082-2085` — *«given a body signed with `effective_height = e`, the admissible
   inclusion heights are `[e − F − G, e − F]`, so the window is `G + 1` blocks wide and has to be
   predicted **before** the quorum round begins»*. Sotto il clamp ogni altezza e' ammissibile e non
   c'e' nulla da predire.
2. `ledger.md:803-804` e `:818-828` — la tabella e *«The upper bound is the side that carries the
   cost … the window admissible for a given signed `effective_height` is `G + 1` blocks wide»*.
3. `core/coblox-core/src/params.rs:41-62`, il commento di `revocation_effective_grace_blocks_min`,
   che descrive `G` come termine di larghezza della finestra di inclusione e giustifica il pavimento
   con la rotazione — **due frasi, entrambe ritirate**, una dal punto 1 e una dal punto 3.
4. `core/coblox-core/src/identity.rs:446-512` — la documentazione e il corpo di
   `validate_effective_height`, e `RevocationError::EffectiveHeightBelowFloor`, che sotto il clamp
   non e' piu' raggiungibile.
5. le probe `unrevoked-effective-height-reason-band`,
   `revocation-grace-floor-is-one-rotation-of-the-minimum-set`,
   `revocation-band-ceiling-is-new-and-inverted`, `guide-revocation-height-has-no-ceiling` —
   **verdi adesso**, e il loro `why` porta gli argomenti ritirati.
6. **[DEBT-040] non e' solo falso nella premessa: perde l'oggetto sulla riga urgente.** La sua
   *Statement* parla di larghezze di finestre di inclusione fra `reason`; sotto il clamp
   `key_compromise` non ha piu' una finestra di inclusione. Il debito sopravvive solo per i `reason`
   pianificati, ed e' un debito diverso. La v2 lo cita fra i quattro «gia' noti e falsi comunque
   vada» senza dire che cambia oggetto.

**Condizione di chiusura verificabile.** Il riquadro contiene un'enumerazione per file e riga nella
stessa forma che [ADR-017] §*Consequences* usa per la parte 1; e la frase che assegna quel lavoro a
`published_artifacts.py` e' tolta o riscritta per cio' che lo strumento sa fare (ripuntare le probe
dopo che l'enumerazione umana e' stata fatta).

RF-007 | category=documentation | severity=medium | criterion=ADR-017, coerenza interna del testo normativo; REVIEW-045 RF-007 | remediation=togliere dal riquadro il paragrafo che ripete la frase che il punto 2 dichiara falsa, e allineare la tabella e il blocco dei vincoli

**Il riquadro della v2 contiene, due paragrafi dopo il punto che la smentisce eseguendo, la frase
falsa che ha causato tre artefatti sbagliati.**

Il punto 2 accerta: *«E' falso, e il Lead lo ha verificato eseguendo»*, e lo dice della frase per cui
un ritardo di inclusione, senza tetto, potrebbe solo rimandare. Il paragrafo **«Cosa questa
correzione non chiude»**, che sta nello **stesso riquadro**, dice:

> *«Il tetto della banda e' nuovo di [SPEC-022]: la clausola 4 preesistente aveva pavimento e nessun
> tetto, quindi un ritardo di inclusione poteva solo rimandare una revoca, e ora puo' distruggerla.»*

**E' la quinta occorrenza della stessa frase** — [REVIEW-042] RF-001, `ledger.md:2100-2103`, la probe
`revocation-band-ceiling-is-new-and-inverted`, la *Statement* di [DEBT-040], e qui — ed e' l'unica
delle cinque a stare nello stesso riquadro della propria smentita. E' [REVIEW-045] RF-007 non chiuso.

Nella stessa famiglia, e non chiusi dalla v2: la **tabella normativa** della parte 2 (RF-001) e il
**blocco dei vincoli**, che porta ancora `revocation_effective_grace_blocks_min + 1 >=
validator_min_set_size_min` — cioe' la relazione la cui **giustificazione** il punto 1 ritira e il cui
**mestiere** il punto 3 cambia, senza che il vincolo sia toccato in nessuno dei due sensi.

**Condizione di chiusura verificabile.** In [ADR-017] non esiste alcuna occorrenza dell'affermazione
che senza tetto un ritardo di inclusione possa solo rimandare; e il blocco dei vincoli o porta la
forma decisa o dichiara accanto a ogni riga ritirata che la sua giustificazione e' caduta.

RF-008 | category=documentation | severity=medium | criterion=ADR-017 parte 2, punto 1; la stessa regola con cui il punto 1 scarta la gamba (1) | remediation=ancorare la gamba superstite al predicato di quorum gia' spedito invece che al protocollo a due fasi non ancora implementato

**La gamba superstite regge — l'ho verificata — ma e' ancorata alla piu' debole delle due ancore
disponibili, che e' il difetto per cui la v2 scarta la gamba (1).**

**Cio' che ho verificato e che tiene.** Il predicato di quorum e' `signed_power * 3 > total_power * 2`
([ADR-018], fatto **7 dei fatti di contesto** non c'entra: e' il fatto **3**, e a differenza del 7 e'
lo stato **spedito** del protocollo, non una lacuna che [SPEC-025] colmera'). Da quel solo predicato
segue che una potenza `>= 1/3` che si astenga impedisce ogni quorum, mentre firmarne uno ne richiede
`> 2/3`. Le due soglie sono diverse, e **nessuna larghezza di finestra difende sopra la piu' bassa**.
La gamba e' quindi:

- **corretta nei numeri** — «oltre un terzo» e' anzi conservativo, perche' il predicato e' stretto e
  una potenza pari a esattamente un terzo gia' blocca;
- **sufficiente da sola** a ritirare l'argomento della rotazione: quell'argomento affermava che *«una
  coalizione capace di censurare un'intera rotazione avrebbe gia' il quorum»*, e la soglia di censura
  strettamente minore di quella di quorum lo falsifica direttamente. **Risposta all'incarico B: si',
  e' sufficiente da sola.**

**Cio' che non tiene e' l'ancora.** Il testo scrive *«Sotto il protocollo a due fasi di [ADR-018]»*.
Il protocollo a due fasi e' **deciso e non implementato**: `wire.md` non ha messaggi di consenso
([ADR-018] fatto 6), `core/` non ha un ciclo di voto, [SPEC-025] non e' consegnata. Il punto 1
scarta la gamba (1) proprio perche' citava *«il punto 7 **di contesto**»* — cioe' uno stato
transitorio — per un ritiro permanente. **La gamba (3) cita lo stesso ADR per un ritiro permanente,
quando le serve solo il predicato di quorum**, che e' in `ledger.md`, in `params.rs` e in ogni
fixture da mesi, e che nessuna spec futura cambiera'. Riancorarla costa una riga e la rende
insensibile a [SPEC-025] e a qualunque revisione della regola di blocco che [ADR-018]
§*Review conditions* lascia aperta.

**Precisazione sul meccanismo, perche' la frase «fa fallire ogni round» da sola non basta.** Per far
crescere `p` — che e' cio' che serve alla censura — la coalizione non puo' limitarsi a far fallire i
round: un'altezza che non finalizza non fa crescere `p`. Deve **lasciare finalizzare** i round
proposti da se stessa, e nessuna regola obbliga un blocco a includere una transazione della mempool,
quindi il quorum onesto firma quei blocchi senza violare nulla. La catena avanza piu' lenta, e la
revoca non entra.

**Condizione di chiusura verificabile.** La gamba superstite cita `signed_power * 3 > total_power * 2`
e la sua sede in `docs/protocol/`, e non condiziona la propria validita' a [ADR-018] o a [SPEC-025].

RF-009 | category=correctness | severity=medium | criterion=ADR-017 clausola 3; REVIEW-036, che dichiara la clausola 3 letta e non attaccata | remediation=dire che `F` e `G` di `e_eff` sono quelli del blocco che include, che il firmatario non li conosce, e che un cambio di parametri fra firma e inclusione sposta l'efficacia in silenzio

**`e_eff` dipende da due parametri governati e la v2 non dice quale versione ne governi il calcolo.
La risposta della clausola 3 e' coerente, ma il suo effetto sotto il clamp e' nuovo e non e'
dichiarato.**

La clausola 3 di [ADR-017] dice che *«ogni vincolo … si valuta contro i **parametri di consenso in
vigore all'altezza del blocco che include** la `revoke_identity`»*, e `ledger.md:806-814` ne nomina
il selettore: il `consensus_parameters_hash` dell'header a `p`. Applicata a `e_eff`, la risposta e'
**i parametri a `p`**, non quelli in vigore alla firma. Non e' un fork — `p` ed `e` sono entrambi
fatti del blocco includente, letti dagli stessi byte da ogni verificatore — e su questo la
derivazione regge.

**Cio' che cambia e non e' scritto.** Sotto la banda dura, un cambio di `F` o `G` fra la firma e
l'inclusione **invalidava** la transazione: fallimento rumoroso, il quorum se ne accorgeva e
rifirmava. Sotto il clamp **sposta l'efficacia in silenzio**, di una quantita' che il firmatario non
poteva conoscere. `F` e' governato, e un aumento di `F` sposta in avanti l'efficacia di **ogni**
revoca pendente in un colpo solo. E' la **famiglia 3** che [ADR-017] nomina gia' — *«vincolata la
grandezza nominata, non quella da cui la proprieta' dipende»* — riapplicata alla grandezza che il
clamp introduce. Il limite e'
`min_revocation_effective_delay_blocks_max + revocation_effective_grace_blocks_max`, e i soli valori
presenti nell'albero sono quelli di `permissive_bounds()`: **`1_000_000 + 1_000_000`**. Nessun valore
tarato esiste, perche' `recommended.py` non porta alcun parametro di revoca ([DEBT-045]).

**E c'e' la ragione di metodo.** [ADR-017] dichiara di se': *«la clausola 3 … resta la parte meno
provata di questa decisione»* e *«rivedere la clausola 3 per prima in qualunque revisione futura»*.
**Questa e' una revisione futura**, e introduce una regola che dipende dalla clausola 3 senza
rileggerla. Le *Review conditions* dell'ADR sono state disattese dal riquadro dentro l'ADR stesso.

**Condizione di chiusura verificabile.** Il punto 3 dice che `F` e `G` di `e_eff` sono presi dal
documento che hasha al `consensus_parameters_hash` del blocco a `p`, e dichiara che un cambio di
quei parametri fra la firma e l'inclusione sposta l'efficacia senza invalidare nulla, con il limite
`F_max + G_max` nominato.

RF-010 | category=documentation | severity=low | criterion=nessuno; nota non bloccante | remediation=dichiarare che `e` diventa indicativo e che il modo di fallire passa da rumoroso a silenzioso, e nominare l'unica grandezza che limita lo scostamento

**Che cosa firma l'autore, e perche' il silenzio conta piu' della promessa.**

`e` resta nel corpo, quindi nella preimmagine di firma e nell'**ID della transazione**, e resta nella
fixture canonica pubblicata (`ledger.md:836`, `"effective_height":"50"`). Sotto la derivazione nessuna
regola lo obbliga. **La v2 non dice che diventa indicativo**, e [REVIEW-045] lo dichiarava fra i
costi della proposta.

Il rilievo non e' che una promessa non sia mantenuta — un valore indicativo firmato non e' di per se'
un difetto, ed e' una scelta legittima. E' che **il modo di fallire cambia specie senza che nessuno
lo dica**. Prima: la revoca ritardata era **invalida**, il quorum lo vedeva, rifirmava. Dopo: la
revoca entra e diventa efficace **piu' tardi di quanto chi l'ha firmata intendesse**, e nulla
segnala lo scostamento — non un errore, non un campo, non una riga di guida. Un osservatore che
vede la revoca in catena la considera fatta.

**E l'unico limite allo scostamento e' in millisecondi.** Una revoca ferma in mempool entra in vigore
dalla propria inclusione tardiva, e cio' che ne limita la permanenza e' `expires_at_ms` della busta
(la fixture di `revoke_identity` porta ventiquattro ore: `1787654550000` → `1787740950000`), piu'
`max_envelope_lifetime_ms` (`params.rs:399`). Ma `ledger.md:2094-2098` argomenta, ed e' la premessa
che ha imposto la forma del pavimento di `G`, che *«no rule of this protocol imposes a cadence …
`G + 1` blocks do not convert into real time»*. **Il solo limite allo scostamento e' quindi espresso
in una unita' che il documento dichiara non convertibile in quella dello scostamento.**

**Condizione di chiusura verificabile.** [ADR-017] o `ledger.md` §*Identity revocation* dichiara che
`e` e' un'indicazione dell'autore che il protocollo puo' non onorare, nomina `expires_at_ms` come
l'unico limite al ritardo, e dice che quel limite e' in millisecondi mentre lo scostamento e' in
blocchi.

RF-011 | category=documentation | severity=low | criterion=nessuno; nota non bloccante | remediation=nominare, dove il clamp e' definito, chi controlla il momento dell'efficacia

**Sotto la derivazione, chi controlla il momento dell'inclusione controlla il momento
dell'efficacia, e nessun documento lo dice.**

E' la risposta diretta alla domanda dell'incarico *«chi guadagna dalla derivazione»*, e vale la pena
scriverla perche' non e' univoca. Guadagna il **difensore** in un caso: la sua transazione non muore
piu' per un ritardo, e questa e' la relief vera del clamp. Guadagna l'**avversario** in due: sposta
l'efficacia in avanti di quanto riesce a censurare (RF-003 ne da' il tetto, `F + G`, che e' finito e
questo e' il merito della v2 rispetto alla v1), e — piu' seriamente — **sceglie il momento del
collasso** di RF-002. Prima della derivazione la censura aveva un solo effetto: uccidere la
transazione. Dopo ne ha due, e il secondo non e' dichiarato in nessun punto.

**Condizione di chiusura verificabile.** Accanto alla definizione del clamp c'e' una frase che dice
che l'altezza di efficacia e' scelta congiuntamente da chi firma (sceglie `e`) e da chi include
(sceglie `p`), e che il secondo puo' essere un avversario.

## Le tre cose che la bozza dichiara di non stabilire: risposta secca

Come richiesto, ognuna con la ragione e il costo, e con cio' che resterebbe non difeso. **Non decido
al posto dell'operatore.**

**C1 — l'enumerazione degli artefatti va prodotta eseguendo la passata di [ADR-012]?** **No, non
cosi'.** La scelta di non derivarla a mano *e' giusta in linea di principio* — a mano e' proprio come
sono nate le omissioni di famiglia 1 — ma **lo strumento nominato non e' in grado**: e' RF-006, e
l'ho eseguito. La forma corretta e' in due tempi: l'enumerazione umana, che e' lavoro del Lead e che
RF-006 avvia, e **poi** la passata per ripuntare le probe e verificare che nessuna resti su una frase
tolta. *Costo:* un'ora di lettura contro un artefatto verde su un documento falso. *Cosa resta non
difeso se si esegue e basta:* tutto, con la firma dello strumento sopra.

**C2 — `revocation_effective_grace_blocks_min` non e' orfano ma ha cambiato mestiere?** **Non sotto
la v2.** L'affermazione e' mia e vale nel ramo in cui il tetto cade. La v2 tiene il tetto, quindi
`G` torna nella riga `key_compromise` — ma con il **segno rovesciato**: e' RF-003. Le vie sono tre.
(i) **Riderivare il pavimento sotto il clamp** contro il numero di transizioni di contrazione, che e'
la derivazione che nessuno ha ancora attaccato e che RF-002 rende ora doppiamente pertinente. *Costo:*
una funzione invece che una costante in `ElectionBounds::validate`, e `permissive_bounds()` da
rifare. *Non difeso:* la censura sopra un terzo, sempre. (ii) **Togliere il pavimento e tenere il solo
tetto**, che sotto il clamp e' il lato che difende. *Costo:* una passata di [ADR-012] su tre probe e
`params.rs`. *Non difeso:* nulla che il pavimento difendesse — perche' sotto il clamp non difende. (iii)
**Tenerlo com'e' e dichiarare che garantisce una latitudine minima.** *Costo:* zero. *Non difeso:*
diciassette blocchi di potere di voto compromesso, obbligatori per genesi, senza che nessuno li abbia
scelti in quel mestiere.

**C3 — il tetto sui `reason` pianificati resta aperto: coerente con l'aver ritirato la rimozione?**
**No, e' un'incoerenza — o una lettura non dichiarata.** E' RF-004. Se il clamp vale su entrambe le
righe, la questione si chiude e la v2 dichiara aperto cio' che ha chiuso. Se vale su una sola, il
protocollo ha due meccaniche selezionate da un campo non verificabile, e il livelock sulla riga
pianificata resta aperto **per scelta** e va scritto. *Costo della prima via:* `P` diventa inerte
sempre, il che tocca [REVIEW-042] RF-008 — **decisione dell'operatore, che non risolvo**. *Costo
della seconda:* due regole invece di una, e un implementatore in piu' da convincere. *Non difeso
nella seconda:* la sanzione per cattiva condotta censurabile all'infinito dal proprio bersaglio con
un terzo bloccante.

## Verdetto raccomandato

**Non approvare la v2 nella forma attuale. Approvarne i punti 1, 2 e 5 subito, e rifare il punto 3.**

Non e' la stessa raccomandazione della v1 e va detto perche'. **La v1 aveva una premessa falsa: era
da buttare.** La v2 ha le premesse giuste, la verifica eseguita, il ritiro corretto, e la disciplina
nel punto 5 e' migliore di quella che sostituisce. **I punti 1, 2 e 5 li ho attaccati e reggono** —
RF-008 e' una riga di riancoraggio, non un difetto di sostanza. Se il riquadro contenesse solo quei
tre punti, lo raccomanderei per l'approvazione oggi.

**Il punto 3 e' incompleto in un modo che ha una sola forma di risposta: scriverlo per intero.** Non
chiedo di ritirare il clamp — chiedo che il testo dica **quali regole abroga** (RF-001), **quali
grandezze leggono le quattro regole del percorso del set e il checkpoint** (RF-005), **a quali
`reason` si applica** (RF-004), e **che `G` ha cambiato segno** (RF-003). Sono quattro paragrafi.

**Uno solo dei sei `high` non e' un'omissione di testo, ed e' RF-002.** Il clamp appiattisce, e un
lotto diluito che un avversario ritarda ferma la catena per sempre dove la banda dura si sarebbe
limitata a rifiutarlo. Quello va **deciso**, non scritto: o si dichiara il rischio dove lo stallo e'
gia' dichiarato, o si sceglie una forma della derivazione che preservi le distinzioni. **E' la parte
della mia stessa proposta che non avevo guardato quando l'ho proposta.**

**In una riga: la v2 ha ragione su cio' che ritira e non ha ancora scritto cio' che adotta — e cio'
che adotta ha un bordo che nessuno aveva guardato, nemmeno chi l'ha proposto.**

## Required follow-up

- **RF-002 per primo, e va all'operatore**: e' l'unico che chiede una scelta e non una riscrittura, e
  l'esito e' un arresto permanente della catena innescato da un avversario a un terzo bloccante.
  Va portato insieme a [REVIEW-044] RF-002, che e' in `changes-requested` e la cui premessa questo
  finding cambia: la' era un errore del difensore, qui e' una leva dell'attaccante.
- **RF-001, RF-004, RF-005 insieme**: sono la stessa lacuna vista da tre lati — la derivazione e'
  dichiarata e non scritta. Vanno chiusi nello stesso giro o produrranno tre implementazioni diverse.
- **RF-003 prima che `G` riceva un valore di lancio**: il parametro sta per essere tarato insieme a
  `F`, `max_clock_drift_ms` e `D_max`/`S_max`, e sotto il clamp il suo pavimento significa
  l'opposto di cio' che il testo dice.
- **RF-006 prima dell'approvazione**: [ADR-012] impone comunque l'enumerazione alla spec attuativa, e
  la passata verde non la produrra'.
- **[DEBT-040] va deciso**, ed e' la seconda review consecutiva che lo chiede: sotto la v2 non ha piu'
  lo stesso oggetto sulla riga urgente.
- **La correzione di `revocation_effective_grace_blocks_min + 1 >= validator_min_set_size_min` nel
  blocco dei vincoli**: e' l'unica riga che tre review consecutive hanno condannato — [REVIEW-044]
  RF-001 nella giustificazione, il punto 1 della v2 nell'argomento, RF-003 qui nel mestiere — e non
  e' stata toccata da nessuna delle tre.
