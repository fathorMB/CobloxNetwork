---
id: REVIEW-034
# Note: Quote the title if it contains a colon
title: "Review of SPEC-020"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-020
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-034-EVENT-001"
    timestamp: "2026-08-26T14:30:27.564469+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata senza finding a carico dell'implementazione. GATE-SECREVIEW resta da attestare.\n\nRiverificato dal Lead rieseguendo: 179 test da 177, 148 probe C10 da 146 ciascuna osservata fallire da sola, protocol_hashes senza valori mossi.\n\nLa consegna contraddice la spec su quale orologio usare, e la contraddizione e' corretta. La spec descriveva il minorante su timestamp_ms dell'ultimo blocco finalizzato; l'implementatore ha usato checkpoint.issued_at_ms, perche' GATE-ONE-CLOCK presa sul serio lo impone: l'orologio che SPEC-016 ha costruito e' issued_at_ms, e cadence.rs dice come divieto che timestamp_ms non e' ingresso di alcuna funzione del modulo. Un minorante su timestamp_ms sarebbe stato un secondo orologio, e proprio quello che quel modulo rifiuta.\n\nLa sostituzione e' strettamente migliore: azzera la leva di partizione che il Lead aveva aggiunto alla valutazione invece di misurarla, ed e' disponibile prima del sync invece che dopo, quindi non indebolisce la ragione 1 di identity.md.\n\nIl Lead ha attaccato l'affermazione piu' forte del rapporto - che un secondo orologio sia inesprimibile - e l'attacco e' fallito contro l'artefatto ma ha rivelato che la sintesi diceva piu' del codice. Il tipo non impone la provenienza, perche' with_checkpoint_floor prende due u64 grezzi, ma il commento lo dichiara meglio di come il Lead lo avrebbe chiesto: un checkpoint non verificato e' un numero scelto dall'attaccante e questo costruttore non puo' accorgersene, e timestamp_ms non e' questo parametro e non va passato come tale. Non e' un finding perche' il rapporto non e' un artefatto pubblicato.\n\nL'onestà che vale piu' della chiusura: l'implementatore aveva scritto che il terzo addendo diventa al piu' max_weak_subjectivity_age_ms e si e' accorto da se' che e' falso, verificando la misura invece che scrivendola. Il passo 1 dell'algoritmo light-client misura l'eta' del checkpoint sullo stesso orologio rotto, quindi nel caso peggiore il pavimento coincide con l'orologio locale. Il termine resta illimitato da ogni regola ed e' scritto cosi'. Cio' che il pavimento cambia e' la grandezza da cui il residuo dipende: un residuo piccolo e' ora ottenibile e non garantito.\n\nRegistrato un fatto nuovo: max_clock_drift_ms non e' fissato da alcun documento di genesi, e l'unico valore in albero e' un input di test. Va in coda alle decisioni di taratura."
    evidence_refs: ["SPEC-020", "DEBT-017", "SKILL-001"]
    implementation_agent: "AGENT-001"
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [review]
activity:
  - date: 2026-08-26
    action: "transitioned pending -> accepted"
---
# Review

## Outcome

**Accettata senza finding a carico dell'implementazione.** `GATE-SECREVIEW` resta da attestare.

La consegna **contraddice la spec su quale orologio usare**, e la contraddizione è corretta: la sorgente scelta azzera la leva che la spec chiedeva di misurare, invece di misurarla.

## La domanda che decideva tutto, e la sua risposta

`GATE-DRIFT-ANSWERED-FIRST` è soddisfatta nell'ordine e nella sostanza: la risposta è registrata **prima** di qualunque regola, con la baseline di 177 test accanto a fissare la sequenza, e istruita su **tre fonti indipendenti** — `ledger.md` §*Block format*, `README.md` §*Genesis constants*, e `params.rs:340-341`. Tutte e tre legano il controllo di drift **alla ricezione della proposta**.

**(a) Sì**, il controllo è un MUST applicato dagli onesti contro il proprio orologio: l'inflazione è limitata. **(b) No** per i blocchi storici in sync — e la confutazione è la parte migliore, perché è **auto-evidente invece che documentale**: riapplicato in sync, quel controllo rifiuterebbe ogni blocco più vecchio di `max_clock_drift_ms`, cioè l'intera storia da genesi. La circolarità non rientra.

## La contraddizione alla spec, che è la sostanza della consegna

La spec e la valutazione di [DEBT-017] descrivevano il minorante come `max(orologio locale, timestamp_ms dell'ultimo blocco finalizzato)`. L'implementatore ha usato **`checkpoint.issued_at_ms`**, e la ragione è `GATE-ONE-CLOCK` presa sul serio: l'orologio che [SPEC-016] ha costruito **è** `issued_at_ms`, e `cadence.rs` lo dice come divieto — *«`timestamp_ms` is not an input to any function in this module»*. Un minorante su `timestamp_ms` sarebbe stato **un secondo orologio, e proprio quello che quel modulo rifiuta**.

**La sostituzione è strettamente migliore**, non equivalente:

- **azzera** la leva di partizione che il Lead aveva aggiunto alla valutazione, invece di misurarla: `timestamp_ms` non è un ingresso, quindi un set che lo gonfia non muove il pavimento di un millisecondo;
- è disponibile **prima** del sync invece che dopo, quindi non indebolisce la ragione 1 di `identity.md`, che era il costo dichiarato dell'esito (A).

`GATE-LEVER-MEASURED` è comunque soddisfatta: la misura è stata fatta **sulla formulazione rifiutata** — `max_clock_drift_ms + ε` — con la derivazione dal quorum, e con l'osservazione che non è un cricchetto perché la mediana degli undici impone monotonia e non accumulo.

## Cosa ho attaccato senza riuscire a romperlo

**Che un secondo orologio fosse davvero inesprimibile**, che è l'affermazione più forte del rapporto. Se il tipo si potesse costruire da un numero qualsiasi, la garanzia sarebbe nominale — la forma di [REVIEW-022] RF-001 e di [DEBT-029].

Il tipo **non impone la provenienza**: `with_checkpoint_floor` prende due `u64` grezzi, e nulla obbliga il secondo a venire da un checkpoint. **Ma il codice lo dichiara da sé**, e lo dichiara meglio di come lo avrei chiesto: *«An unverified checkpoint is an attacker-chosen number and this constructor cannot tell»*, e in grassetto *«`timestamp_ms` is not this parameter and must not be passed as it»*, con la ragione — i due campi sono a una riga di distanza nello schema.

**L'attacco è quindi fallito contro l'artefatto.** Ciò che è emerso è che **la frase del rapporto era più forte del codice**: *«non è vietato per convenzione: è inesprimibile»* dice più di quanto il tipo imponga, mentre il commento dice esattamente il vero. Non è un finding — il rapporto non è un artefatto pubblicato — e lo registro perché è la forma che questa sessione ha censito sette volte, qui presente **solo** nella sintesi e non nella cosa consegnata.

**Che i campi fossero pubblici.** `now_ms` e `local_clock_ms` sono privati, i costruttori due. **Non si è rotto.**

## Acceptance-criteria compliance

Riverificato dal Lead rieseguendo: **179 test** da 177. `published_artifacts.py` `PASS` con **148 candidati C10** da 146. `published_artifacts_negative.py` `PASS`, ogni probe osservata fallire da sola. `protocol_hashes.py` `PASS`, nessun valore pubblicato cambiato.

**`GATE-BOOTSTRAP-UNCHANGED` è resa strutturale prima che asserita**: `local_only` ha `floor_ms() == 0` per costruzione, e l'altro costruttore richiede un dato che il nodo in bootstrap non possiede. La prova che conta è la seconda: un caso che **riproduce** il limite dichiarato invece di assumerlo — stesso ricevente, attestazione scaduta, **accettata senza checkpoint e rifiutata con uno**.

**Nessuno dei quattro rimedi esclusi è stato adottato**: `params.rs` non è toccato, la struct dell'attestazione è invariata.

## L'onestà che vale più della chiusura

L'implementatore aveva scritto che il terzo addendo diventa *«al più `max_weak_subjectivity_age_ms`»*, **e si è accorto da sé che è falso** — verificando la misura, non scrivendola. Il passo 1 dell'algoritmo light-client misura l'età del checkpoint come `orologio locale − issued_at_ms`, cioè **sullo stesso orologio rotto**: un ricevente indietro di `b` accetta un checkpoint di età vera fino a `W + b`, e nel caso peggiore il pavimento coincide con l'orologio locale.

**Il termine resta quindi illimitato da ogni regola del protocollo, ed è così che è scritto** — in `identity.md`, in TM-37 e nella documentazione del modulo. Ciò che il pavimento cambia è la **grandezza da cui il residuo dipende**: prima l'errore dell'orologio, non osservabile e non correggibile senza un riferimento esterno; ora l'età di un artefatto che l'operatore ottiene fuori banda **senza possedere un orologio giusto**. Un residuo piccolo è ora **ottenibile**, non **garantito**.

Scrivere «chiude» sarebbe stata la famiglia 2 dentro una spec che esiste per non commetterla. L'ha evitata correggendo sé stesso.

## Review findings

Nessuno a carico dell'implementazione.

**Un fatto nuovo è emerso ed è registrato:** `max_clock_drift_ms` **non è fissato da alcun documento di genesi** — l'unico valore nell'albero è `1` ed è un input di test. La misura della leva della formulazione rifiutata dipende quindi da un parametro che **nessuno ha scelto**. Non blocca questa spec, perché la formulazione che vi poggiava è stata scartata; va in coda alle decisioni di taratura.

**Tre affermazioni preesistenti di `identity.md` sono state corrette** — dicevano *«bounded only by the length of the window»*. Non è correzione fuori scopo: erano affermazioni che la regola nuova rendeva stantie **nella stessa passata**, cioè la famiglia 2 che questa spec esiste per non commettere.

## Final decision

**Accettata.** `GATE-SECREVIEW` resta da attestare, e l'implementatore ha nominato i due punti su cui la reviewer deve pronunciarsi, entrambi giusti: che il pavimento **non chiude** il termine, e che la sorgente `issued_at_ms` **sposta la fiducia sulla chiave di rilascio invece che sul set di validatori**. Il secondo è la questione vera: la leva è azzerata per i validatori e non per chi firma i checkpoint, e va detto quanto quella capacità sia già posseduta.
