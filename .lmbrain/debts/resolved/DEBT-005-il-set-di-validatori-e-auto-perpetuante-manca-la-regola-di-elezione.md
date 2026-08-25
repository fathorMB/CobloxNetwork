---
id: DEBT-005
title: "Il set di validatori e auto-perpetuante: manca la regola di elezione"
status: resolved
category: "security"
severity: "critical"
origin_severity: "critical"
area: "core"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: "SPEC-004"
origin_ref: "TM-18"
related_specs: ["SPEC-004","SPEC-001"]
related_reviews: ["REVIEW-003"]
related_decisions: ["ADR-001","ADR-007"]
target_specs: []
blocked_by: []
resolution_refs: ["SPEC-006","REVIEW-010","ADR-007"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["consensus","governance","sybil"]
links: []
activity:
  - date: 2026-08-25
    action: "resolved: Chiuso con SPEC-006, che soddisfa tutti i criteri di risoluzione. La regola non e stata accettata al primo tentativo: ha attraversato quattro giri di review adversariale con AGENT-007 e tredici finding, tre dei quali critical. Due di quei finding erano arresti certi della catena introdotti dalle correzioni dei giri precedenti, e nessuno dei due sarebbe stato visibile prima che la correzione precedente esistesse. E la forma normale di un invariante di consenso, non un sintomo di lavoro debole: la parte portante, l'architettura a due strati e la derivazione, non e stata toccata da nessuno dei tredici finding.\n\nIl Lead ha verificato in modo indipendente a ogni giro invece di accettare l'evidenza, ed e stato corretto due volte dagli agenti che eseguivano invece di fidarsi. La piu significativa: uno scenario di risincronizzazione che il Lead aveva passato come verificato non produce arresto su genesi scaglionata, e AGENT-002 ha eseguito la successione, trovato che non reggeva, e pubblicato quella verificata."
debt_events:
  - schema_version: "1"
    id: "DEBT-005-EVENT-001"
    timestamp: "2026-08-25T01:50:03.498792200+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Promosso a debito dal Lead durante la review di SPEC-004: e una questione che sopravvive alla spec che l'ha scoperta e che nessuna spec attualmente aperta copre. Registrato su mandato dell'operatore di salvare tutti i debiti emergenti."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-005-EVENT-002"
    timestamp: "2026-08-25T14:48:14.432365500+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Chiuso con SPEC-006, che soddisfa tutti i criteri di risoluzione. La regola non e stata accettata al primo tentativo: ha attraversato quattro giri di review adversariale con AGENT-007 e tredici finding, tre dei quali critical. Due di quei finding erano arresti certi della catena introdotti dalle correzioni dei giri precedenti, e nessuno dei due sarebbe stato visibile prima che la correzione precedente esistesse. E la forma normale di un invariante di consenso, non un sintomo di lavoro debole: la parte portante, l'architettura a due strati e la derivazione, non e stata toccata da nessuno dei tredici finding.\n\nIl Lead ha verificato in modo indipendente a ogni giro invece di accettare l'evidenza, ed e stato corretto due volte dagli agenti che eseguivano invece di fidarsi. La piu significativa: uno scenario di risincronizzazione che il Lead aveva passato come verificato non produce arresto su genesi scaglionata, e AGENT-002 ha eseguito la successione, trovato che non reggeva, e pubblicato quella verificata."
    evidence_refs: ["SPEC-006", "REVIEW-010", "ADR-007"]
---
# Il set di validatori e auto-perpetuante: manca la regola di elezione

## Statement

La regola di continuita del validator set in ledger.md autentica in modo sicuro la transizione da un set al successivo, ma non vincola in alcun modo CHI possa finire nel set successivo: il documento dichiara esplicitamente che non specifica come i membri siano eletti o ruotati. In quello spazio vuoto il set corrente e l'unico soggetto che scrive il set successivo, quindi un quorum raggiunto una sola volta puo impegnare un successore composto interamente da se stesso, all'infinito. Il light client non puo accorgersene perche la continuita e formalmente valida a ogni passo.

## Evidence and provenance

SPEC-004 scenario TM-18, con l'analisi quantitativa di threat-model.md §6.1. Citazione diretta da docs/protocol/ledger.md: "This continuity rule specifies safe authentication but not how members are elected or rotated". Requisito derivato SEC-REQ-13, indicato da AGENT-007 come uno dei tre irrinunciabili.

## Impact and scope boundary

Una rete che accumuli storia sotto questa regola mancante puo diventare permanentemente chiusa senza che nessuno se ne accorga, e la chiusura non e reversibile a posteriori perche il set insediato controlla ogni transizione futura. La roadmap colloca la rotazione automatica in M-07, il che e ragionevole per l'automazione ma non per l'invariante: l'invariante serve prima che esista storia. E inoltre intrecciato con ADR-007, che vincola l'eleggibilita a lavoro difficile da falsificare e non al solo uptime.

## Decision log

Created by project-lead: Promosso a debito dal Lead durante la review di SPEC-004: e una questione che sopravvive alla spec che l'ha scoperta e che nessuna spec attualmente aperta copre. Registrato su mandato dell'operatore di salvare tutti i debiti emergenti.

## Resolution criteria

La regola di elezione e scritta nei documenti di protocollo: deterministica a partire da casualita finalizzata, su un insieme di eleggibili calcolabile da chiunque, con tetto di rotazione per epoca e impegno nel header del blocco che ne consenta il ricalcolo a posteriori. I test di attacco AT-09 e AT-10 di SPEC-004 passano. Fino ad allora nessuna devnet deve accumulare storia che si intenda conservare.

## Resolution evidence

La regola e scritta in docs/protocol/ledger.md, sezione Validator election and rotation, con le sottosezioni su epoche e confine, candidatura esplicita per epoca, eleggibilita, insieme candidato impegnato, seme, derivazione, tetto e pavimento, casi degeneri, test di ADR-008, perimetro del light client ed esempio numerico. Aggiornati anche identity.md, README.md e knowledge/threat-model.md.

I quattro criteri di risoluzione sono soddisfatti nella forma richiesta.

Deterministica a partire da casualita finalizzata: election_ticket = H(dominio || chain_id || election_seed || account_key) in ordine crescente. Il Lead ha riprodotto l'intero esempio numerico a ogni cambio di formula, l'ultima volta con il seme che dipende dalla sola finestra di entropia: diciotto valori su diciotto.

Insieme degli eleggibili calcolabile da chiunque: candidate_root come albero di Merkle su insieme ordinato bytewise, riusando lo schema gia presente per eligible_set_root dell'existence income. Eleggibilita come soglia binaria su contribution_score costruito dai soli passed di storage e compute, con l'availability che contribuisce zero, come ADR-007 impone, e con diversita di emittente minima per chiudere la coppia collusa.

Tetto di rotazione per epoca: validator_churn_cap_seats, accompagnato da un pavimento di contrazione 3*member_count(new) > 2*member_count(old), che e l'aritmetica del predicato di quorum applicata ai seggi.

Impegno che consente il ricalcolo a posteriori: ElectionRecord dentro ValidatorSet, quindi impegnato da validator_set_hash, che l'altezza precedente impegna gia come next_validator_set_hash nel BlockHeader. Il requisito e soddisfatto transitivamente, senza un campo di header nuovo per blocco.

Difetti trovati e chiusi durante i quattro giri, oltre a quelli che DEBT-005 nominava.

La regola di confine che nessuno aveva chiesto: senza l'obbligo che a ogni altezza non di confine e non di revoca next_validator_set_hash uguagli validator_set_hash, l'intera elezione era aggirabile cambiando set a un'altezza arbitraria.

I limiti di magnitudine dei parametri di elezione: il blocco di vincoli governava le relazioni e non le grandezze, e quei parametri li firma il quorum in carica, quindi un set poteva congelarsi per sempre restando conforme. Chiuso spostando i limiti in ElectionBounds nell'ancora di fiducia della genesi, fuori dalla governance on-chain, con un tetto al tasso di variazione e uno spaziamento minimo in altezze di catena.

Il pavimento di contrazione, senza il quale una coalizione poco sopra un terzo poteva restare sola censurando le candidature altrui e ottenere il cento per cento del potere in un solo confine.

Due arresti certi della catena, entrambi introdotti dalle correzioni precedenti. Il primo: il set di genesi con mandati sincronizzati li faceva scadere tutti insieme al confine T, rendendo tetto di ingressi e pavimento di contrazione congiuntamente insoddisfacibili a ogni V. Il secondo: i timbri di scadenza collidono se e solo se il limite di mandato decresce, quindi un operatore onesto che accorciasse i mandati su indicazione del simulatore fermava la catena senza alcun avversario e senza alcun avvertimento. Chiusi rispettivamente con la genesi obbligatoriamente scaglionata piu il vincolo 3c < V, e con la monotonicita non decrescente del limite di mandato.

Scoperto inoltre che il fixture PD-0 del progetto era esso stesso inammissibile: la soddisfacibilita congiunta impone T >= 4 e il fixture usava T = 3. Il Lead ha verificato per forza bruta che T <= 3 non ammette alcun c valido a nessuna dimensione del set fino a V = 399. Il fixture e stato sostituito e l'obbligo per le suite di conformita di validare i propri fixture di parametri contro il blocco di vincoli e ora normativo.

Tre affermazioni di minimalita o di impossibilita sono state falsificate nel corso dei giri, ognuna con la sostanza attorno intatta: la soglia effettiva di cattura torna a due terzi, non esiste una regola onesta da scrivere contro la contrazione selettiva, e l'istanza minima ammissibile. Tutte e tre corrette nel documento accanto a cio che hanno sostituito.

Verifiche indipendenti del Lead, oltre alle 129 dell'implementatrice: i tre hash delle fixture PD-0 ricalcolati a ogni giro che li toccava, con il metodo validato prima su una fixture non modificata; l'esempio numerico riprodotto per intero a ogni cambio di formula; l'aritmetica delle coppie di vincoli T >= 3m e T >= 4; la simulazione della successione dei set su 30 e 60 confini, che riproduce entrambi gli arresti e ne conferma la chiusura; l'enumerazione di 36300 coppie di timbri, le cui 1155 collisioni hanno tutte il limite di mandato decrescente, confermando che la condizione e necessaria e sufficiente; e il controesempio all'istanza minima.

AT-09 resta parzialmente coperto, con la parte sull'equivocazione come transazione di evidenza verificabile dichiarata non coperta e rimandata a M-07, e SEC-REQ-13 registra questa divisione. AT-10 e ora eseguibile, con la disuguaglianza qualificata come tempo di cattura per ammissione e due configurazioni nuove; il verdetto numerico dipende dal simulatore economico e resta a DEBT-007.
