---
id: ADR-010
# Note: Quote the title if it contains a colon
title: "Le difese economiche hanno bisogno degli stessi tre strati dei parametri di elezione"
status: accepted
decision_date: 2026-08-25
decider: AGENT-LEAD
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: [ADR-005, ADR-006, ADR-007]
tags: [architecture, security]
created: 2026-08-25
updated: 2026-08-25
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned proposed -> accepted"
---
# Le difese economiche hanno bisogno degli stessi tre strati dei parametri di elezione

> Nata da [REVIEW-011], la revisione di sicurezza di [SPEC-007]. Il verdetto della reviewer sul lavoro di taratura: *«i valori sono difendibili, le affermazioni che li accompagnano no»* — e la ragione per cui non lo sono è che **nessuna regola li tiene**.

## Context

[SPEC-007] ha tarato `α`, la forma del fondo del reddito di esistenza e i parametri di elezione. Il lavoro numerico è corretto e non è in discussione. La revisione di sicurezza ha però posto al rapporto una domanda che il rapporto non si poneva: **questi numeri sono difesi da una regola, o soltanto scritti bene?** Tre volte su tre, la seconda.

La formulazione che riassume il difetto, ed è della reviewer:

> Ciò che è governato senza limiti di magnitudine non è un parametro, è una preferenza.

**La difesa anti-Sybil di [ADR-007] vive interamente nella reward policy**, che non ha nessuno dei tre strati che il progetto ha costruito per i parametri di elezione: né vincoli relazionali, né magnitudini fissate alla genesi, né limite al tasso di variazione. L'unica regola che governa quel documento è il tetto della quota al creatore, `kd > 0 and kn < kd`.

Ne discendono due superfici, entrambe verificate dal Lead.

**La prima.** Il set in carica firma un documento con `availability_microtokens_per_unit` positivo: `work_compensation` di tipo `availability` è un importo **per nodo senza tetto**, quindi la flotta stampa, il criterio (a) di [ADR-007] cade, e nessuna traccia on-chain distingue l'atto da un normale atto di governance. Fissare quella tariffa a zero è **necessario e non sufficiente**: il fondo `F` resta senza tetto e senza limite di variazione, e un solo documento lecito lo porta dal valore tarato a `2^60`. La disciplina 5/4 che il rapporto propone su `F` è prassi, non è imposta da nulla.

**La seconda.** Nessun vincolo lega `validator_min_set_size` a `V`. Il valore 18 è stato scelto perché è due terzi di `V = 27`, e a quel rapporto chiude la cattura per attrito sotto i due terzi. Ma `V` può crescere mentre `min_set` resta fermo: il Lead ha eseguito il percorso, `27 → 33 → 36` in **due documenti leciti**, dopo di che il blocco di vincoli accetta `V=36, T=12, min_set=18` riga per riga e la contrazione `36 → 25 → 18` consegna l'intero set a una coalizione di 18, che è il **50% di `V`**. La rassicurazione di `ledger.md` — oltre i due terzi la safety è comunque già caduta — viene distrutta, perché la cattura avviene sotto quella soglia.

**È la terza volta che questo schema compare, e le prime due il progetto lo ha risolto nello stesso modo.** Il pavimento di costo dell'Argon2id è una regola di validità e non una raccomandazione, motivato in `README.md` con il fatto che un insieme di parametri governato avrebbe potuto rimuoverlo restando conforme e senza traccia on-chain. `ElectionBounds` è nell'ancora di fiducia della genesi per la stessa ragione, decisa in [SPEC-006]. Le difese economiche sono la terza istanza, e sono rimaste indietro.

## Decision

**Le grandezze da cui dipende una proprietà di sicurezza dichiarata sono governate come tali, qualunque documento le porti.** In concreto, tre disposizioni.

**1. `RewardBounds` nell'ancora di fiducia della genesi.** Simmetrico a `ElectionBounds`: magnitudini minime e massime per le grandezze della reward policy, rapporto massimo di variazione per `sequence` consecutiva, e la stessa spaziatura minima in altezze di catena. Vi appartiene almeno il tetto di `existence_fund_microtokens_per_epoch`. Fuori dalla governance on-chain, mai apprendibile da un peer, spedito nella distribuzione firmata.

**2. `availability_microtokens_per_unit` deve valere zero, come regola di validità.** Un documento di reward policy con quella tariffa positiva è **rifiutato in accettazione**, non sconsigliato. La ragione è strutturale e va scritta accanto alla regola: è l'unico canale che paga **per nodo senza un tetto aggregato**, e un canale così è incompatibile con il criterio (a) di [ADR-007] per costruzione e non per taratura. Se un giorno l'availability dovrà essere remunerata, la strada è un canale con tetto aggregato, non una tariffa positiva.

**3. `3 * validator_min_set_size >= 2 * V` entra nel blocco di vincoli dei parametri di consenso.** Lega il pavimento assoluto alla dimensione del set invece di lasciarlo un valore ben scelto. Oggi è soddisfatto **con uguaglianza esatta**, `54 >= 54`, quindi il costo di adottarlo è zero.

Finché la regola 3 non esiste, **`ledger.md` non va corretto**: la sua affermazione «soglia effettiva appena sopra un terzo» è la cifra giusta nel caso peggiore *governabile*, ed è il rilievo della reviewer. Il merito che [SPEC-007] attribuiva a `min_set_size` va rivendicato solo quando `min_set_size` sarà vincolato.

## Alternatives considered

- **Lasciare le difese economiche come valori ben scelti e documentati.** È lo stato attuale. Scartata per la stessa ragione per cui fu scartata due volte prima: un insieme di parametri governato può rimuovere la protezione restando pienamente conforme e senza lasciare traccia distinguibile da un atto ordinario. Un progetto che ha già applicato questo argomento all'Argon2id e a `ElectionBounds` non può non applicarlo qui senza essere incoerente con sé stesso.
- **Fissare solo `availability = 0` e fermarsi.** È la correzione che il rapporto di [SPEC-007] indica ed è necessaria, ma la reviewer ha dimostrato che non basta: `F` senza tetto è la stessa superficie con un nome diverso. Chiudere metà di una superficie e dichiararla chiusa è peggio che lasciarla aperta e dichiararlo.
- **Un `departure_cap` o un parametro governato in più per legare `min_set` a `V`.** Scartata per l'argomento che [SPEC-006] ha già usato rifiutando la stessa forma: un parametro governato in più è un parametro in più da vincolare in un'ancora di genesi, cioè la stessa questione spostata di un livello. Il vincolo relazionale non aggiunge nulla da firmare.
- **Rendere `α` stessa un parametro governato con limiti.** Non è possibile e la confusione va evitata: `α` è una grandezza **osservata**, non impostata — `ledger.md` la chiama già «an observed ratio». Ciò che si governa sono le grandezze da cui emerge, ed è esattamente ciò che questa ADR vincola.

## Consequences

- Serve una spec di M-02 che scriva `RewardBounds`, la regola di validità sulla tariffa di availability e il vincolo relazionale su `min_set`, con le fixture di conformità corrispondenti. È lavoro di specifica di protocollo più una riga di blocco, non lavoro di taratura: non appartiene a [SPEC-007].
- `coblox-core` dovrà validare `RewardBounds` come già valida `ElectionBounds`, e il tipo che porta i parametri di reward dovrà avere lo stesso trattamento di `ValidatedConsensusParameters`, cioè nessun costruttore diverso dalla validazione. È lavoro conseguente a [SPEC-008].
- Il claim di resistenza ai Sybil diventa dichiarabile in una forma più forte di quella odierna. La reviewer lo formula oggi in quattro frasi, di cui **una sola ancorata a una regola**; chiuse queste tre disposizioni, la seconda — che una flotta non aumenta l'emissione totale — passa da vera-a-condizione a vera-per-regola.
- Il progetto acquista un criterio riusabile che vale oltre questa ADR: **prima di dichiarare una proprietà di sicurezza, chiedersi quale regola la tiene**. Se la risposta è «il valore è scelto bene», la proprietà è una preferenza e va o vincolata o dichiarata come tale.

## Review conditions

Rivedere se: emerge una grandezza di sicurezza in un documento governato che nessuno dei due `Bounds` copre, il che indicherebbe che il criterio va applicato per principio a ogni nuovo documento invece che per enumerazione; oppure se i limiti di genesi risultassero così stretti da impedire alla rete correzioni legittime, che è il costo simmetrico già registrato per il limite di mandato in [DEBT-010] e va sorvegliato con gli stessi occhi. **Non rivedere** per la ragione che i valori attuali sono ragionevoli: la ragionevolezza dei valori è precisamente ciò che questa ADR ritiene insufficiente.
