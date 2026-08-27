---
id: DEBT-038
title: "Il beacon di casualita dedicato non ha un proprietario: ledger.md rimanda a un debito risolto"
status: planned
category: "security"
severity: "medium"
origin_severity: null
area: "consensus"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: null
origin_ref: null
related_specs: ["SPEC-023","SPEC-024"]
related_reviews: ["REVIEW-040"]
related_decisions: ["ADR-013"]
target_specs: ["SPEC-027","SPEC-024"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-27
updated: 2026-08-27
tags: ["security","consensus"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: Il primo dei due lavori e' fatto: l'operatore ha deciso il 2026-08-27 che il beacon di casualita' dedicato vive in M-03, ed e' scritto nella roadmap con il perimetro ristretto — la sola quantizzazione di `timestamp_ms` allo slot, perche' l'altra riduzione e' gia' presa dal seme dell'elezione. M-03 e' la milestone delle sfide di availability, cioe' il consumatore di quel beacon.\n\nResta da far puntare li' il rimando di `ledger.md`, che oggi indica DEBT-005, risolto e con un altro oggetto. E' una riga in un documento di protocollo dentro una passata che SPEC-027 esegue gia' su quel file: instradato li' e non su una spec propria.\n\nInstradato anche su SPEC-024 per la seconda meta', che il debito stesso propone e che il Lead condivide: la domanda giusta non e' se `docs/protocol/` citi DEBT-005, ma quanti rimandi a debiti chiusi contengano i documenti pubblicati, ed e' una domanda che si risponde enumerando. E' la stessa forma per cui SPEC-024 esiste — un riferimento che scade senza che nessuno se ne accorga — quindi vale estenderne il perimetro invece di aprire uno strumento nuovo. L'estensione va scritta in SPEC-024 prima che parta.\n\nNota di metodo, dal debito stesso: fu aperto applicando la regola \"verificare una citazione significa leggere il suo intorno\", e la regola si fermo' troppo presto — il Lead lesse l'intorno della frase e non cerco' un altro sito dello stesso documento che la superasse. E' lo stesso errore che lo stesso giorno ha prodotto DEBT-041."
debt_events:
  - schema_version: "1"
    id: "DEBT-038-EVENT-001"
    timestamp: "2026-08-27T00:25:03.119242200+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead mentre rimediava [REVIEW-040] NF-01, e non da chi ha scritto o rivisto l'analisi.\n\nVale la pena registrare come e' emerso, perche' e' una forma che questa sessione non aveva ancora visto. AGENT-007 aveva nominato le tre mitigazioni per dire che nessuna e' un tetto su `max_clock_drift_ms` — argomento corretto e sufficiente al suo scopo. Il Lead e' andato a verificarle **per non scriverle senza averle lette**, e la frase successiva a quelle tre era il rimando morto. **Il difetto non stava in cio' che la reviewer citava: stava nella riga dopo.**\n\nNe discende una regola pratica: verificare una citazione significa leggere il suo intorno, non solo la sua riga. Un difetto che nessuna delle due letture precedenti aveva visto stava a una frase di distanza da una che entrambe avevano guardato."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-038-EVENT-002"
    timestamp: "2026-08-27T15:17:00.994608200+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Il primo dei due lavori e' fatto: l'operatore ha deciso il 2026-08-27 che il beacon di casualita' dedicato vive in M-03, ed e' scritto nella roadmap con il perimetro ristretto — la sola quantizzazione di `timestamp_ms` allo slot, perche' l'altra riduzione e' gia' presa dal seme dell'elezione. M-03 e' la milestone delle sfide di availability, cioe' il consumatore di quel beacon.\n\nResta da far puntare li' il rimando di `ledger.md`, che oggi indica DEBT-005, risolto e con un altro oggetto. E' una riga in un documento di protocollo dentro una passata che SPEC-027 esegue gia' su quel file: instradato li' e non su una spec propria.\n\nInstradato anche su SPEC-024 per la seconda meta', che il debito stesso propone e che il Lead condivide: la domanda giusta non e' se `docs/protocol/` citi DEBT-005, ma quanti rimandi a debiti chiusi contengano i documenti pubblicati, ed e' una domanda che si risponde enumerando. E' la stessa forma per cui SPEC-024 esiste — un riferimento che scade senza che nessuno se ne accorga — quindi vale estenderne il perimetro invece di aprire uno strumento nuovo. L'estensione va scritta in SPEC-024 prima che parta.\n\nNota di metodo, dal debito stesso: fu aperto applicando la regola \"verificare una citazione significa leggere il suo intorno\", e la regola si fermo' troppo presto — il Lead lesse l'intorno della frase e non cerco' un altro sito dello stesso documento che la superasse. E' lo stesso errore che lo stesso giorno ha prodotto DEBT-041."
    evidence_refs: ["SPEC-027", "SPEC-024"]
---
# Il beacon di casualita dedicato non ha un proprietario: ledger.md rimanda a un debito risolto

## Statement

`docs/protocol/ledger.md`, §*"Challenge evidence"*, dichiara che la macinatura di `timestamp_ms` sul beacon di elezione ha **due riduzioni disponibili e non prese in v0** — quantizzare `timestamp_ms` allo slot di consenso, e derivare il materiale del beacon dai `block_id` di `K` blocchi consecutivi — e le colloca cosi':

> *"Both belong with the dedicated randomness beacon, which is M-02 work under [DEBT-005]."*

**[DEBT-005] e' risolto**, chiuso da [SPEC-006]. E il suo oggetto non era un beacon di casualita' dedicato: era *«il set di validatori e' auto-perpetuante, manca la regola di elezione»*.

Il lavoro a cui il documento rimanda **non e' tracciato da alcun debito aperto**. Un lettore che segua il rimando trova un debito chiuso e conclude che la questione sia chiusa con esso.

> **Portata ristretta a meta', [REVIEW-041] RF-004, 2026-08-27.** Delle due riduzioni, **una e' gia' presa**, e lo dice lo stesso `ledger.md` in un altro sito, §*"Validator election and rotation"*: *«Aggregating `election_entropy_blocks` consecutive blocks raises the cost of controlling the whole window to holding consecutive proposal slots — **the reduction this document deferred to "the dedicated randomness beacon" and takes here**»*. Verificato dal Lead.
>
> **Il lavoro senza proprietario e' quindi la sola quantizzazione di `timestamp_ms` allo slot di consenso**, non la coppia. Questo debito va letto con quel perimetro.
>
> **Come l'errore e' nato, perche' vale piu' della correzione.** Questo debito e' stato aperto applicando la regola *«verificare una citazione significa leggere il suo intorno»*, che il Lead aveva appena derivato trovando il rimando morto **nella frase successiva** a una citazione di AGENT-007. La regola ha funzionato e si e' fermata troppo presto: il Lead ha letto l'intorno **della frase**, e non ha cercato **un altro sito dello stesso documento** che la superasse. AGENT-007 ha applicato la stessa regola un livello sopra e ha trovato quello che mancava. **L'intorno di una citazione non e' solo cio' che le sta accanto: e' tutto cio' che nello stesso documento parla della stessa cosa.**

## Evidence and provenance

Trovato dal Lead il 2026-08-27 mentre rimediava [REVIEW-040] NF-01, leggendo la sezione per verificare le tre mitigazioni che AGENT-007 le attribuiva.

Le tre mitigazioni esistono verbatim e sono state confermate: la regola di copertura a due issuer, dichiarata *"the mitigation of this grinding"*; la quantizzazione allo slot; l'aggregazione su `K` blocchi consecutivi, *"so that grinding requires K consecutive proposals by the same attacker"*.

Il rimando a [DEBT-005] e' nella frase immediatamente successiva a quelle tre. Verificato che [DEBT-005] sia in `.lmbrain/debts/resolved/` e che il suo titolo riguardi la regola di elezione del set di validatori.

**Non verificato**: se esista altrove nel brain un artefatto che tracci il beacon di casualita' dedicato sotto un altro nome. La ricerca del Lead si e' fermata al rimando, e questo e' il perimetro della segnalazione.

## Impact and scope boundary

Il danno immediato non e' un difetto di protocollo: e' un **rimando morto in un documento pubblicato**, che indirizza chi legge verso un debito chiuso.

Il danno che conta e' a valle. [REVIEW-040] NF-01 ha stabilito che `max_clock_drift_ms` **non governa** la finestra di macinatura, il cui termine dominante e' la mediana degli undici blocchi finalizzati precedenti, denominata in `block_interval_ms`. La chiusura reale di quella superficie sta quindi nelle due riduzioni non prese piu' la regola a due issuer — cioe' **esattamente nel lavoro che questo rimando manda in un vicolo cieco**.

L'ADR che nascera' dall'analisi dei dieci parametri dovra' dire che il tetto su `max_clock_drift_ms` e' mitigazione di grado e non chiusura, e indicare dove sta la chiusura. **Se quel lavoro non e' tracciato, l'ADR indichera' il nulla**, e la superficie resta aperta senza che alcun artefatto la reclami.

`medium` e non `high` perche' la superficie e' gia' dichiarata con il proprio ordine di grandezza nel documento — che chiude la frase con *"the word 'bounded' without a number is not a bound"* — e la regola a due issuer, che e' la mitigazione attiva, esiste ed e' imposta. Cio' che manca e' il proprietario del residuo, non la conoscenza del residuo.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead mentre rimediava [REVIEW-040] NF-01, e non da chi ha scritto o rivisto l'analisi.

Vale la pena registrare come e' emerso, perche' e' una forma che questa sessione non aveva ancora visto. AGENT-007 aveva nominato le tre mitigazioni per dire che nessuna e' un tetto su `max_clock_drift_ms` — argomento corretto e sufficiente al suo scopo. Il Lead e' andato a verificarle **per non scriverle senza averle lette**, e la frase successiva a quelle tre era il rimando morto. **Il difetto non stava in cio' che la reviewer citava: stava nella riga dopo.**

Ne discende una regola pratica: verificare una citazione significa leggere il suo intorno, non solo la sua riga. Un difetto che nessuna delle due letture precedenti aveva visto stava a una frase di distanza da una che entrambe avevano guardato.

## Resolution criteria

Due lavori, e il primo e' quasi gratuito.

**1. Decidere dove vive il beacon di casualita' dedicato**, e far puntare il rimando li'. Le opzioni sono un debito proprio, una voce di roadmap M-02, o una spec. **Non e' ammissibile lasciare il rimando su [DEBT-005]**: un debito risolto non riapre, e la regola di forma di questo progetto vuole che cio' che e' superato sia sostituito e non lasciato a indicare il vuoto.

**2. Verificare che il residuo sia ancora quello dichiarato.** Il documento quantifica la finestra in `10^3-10^6` valori legali. [REVIEW-040] NF-01 ha stabilito che il termine dominante e' la mediana degli undici e non `max_clock_drift_ms`: **la stima resta valida, ma la sua attribuzione no**, e la sezione la presenta come una finestra governata anche dalla deriva. Va riletta con quella correzione in mano.

**Va inoltre deciso se questo rimando sia l'unico del suo genere.** La domanda giusta non e' se `docs/protocol/` citi [DEBT-005], ma **quanti rimandi a debiti chiusi contengano i documenti pubblicati** — ed e' una domanda che si risponde enumerando, non guardando. E' la stessa forma per cui [SPEC-024] esiste: un riferimento che scade senza che nessuno se ne accorga. **Vale la pena estendere il perimetro di [SPEC-024] a questa classe invece di aprire uno strumento nuovo.**

## Resolution evidence

