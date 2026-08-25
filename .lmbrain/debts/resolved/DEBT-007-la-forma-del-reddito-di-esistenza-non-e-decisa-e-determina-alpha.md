---
id: DEBT-007
title: "La forma del reddito di esistenza non e decisa e determina alpha"
status: resolved
category: "design"
severity: "high"
origin_severity: "high"
area: "core"
milestone: "M-02"
owner: "AGENT-002"
origin_artifact: "SPEC-004"
origin_ref: "TM-08"
related_specs: ["SPEC-004"]
related_reviews: ["REVIEW-003"]
related_decisions: ["ADR-005","ADR-007"]
target_specs: []
blocked_by: []
resolution_refs: ["SPEC-007","REVIEW-011","ADR-007","ADR-010"]
superseded_by: null
revisit_condition: null
created: 2026-08-25
updated: 2026-08-25
tags: ["economy","simulation","sybil"]
links: []
activity:
  - date: 2026-08-25
    action: "resolved: Chiuso con SPEC-007, accettata dopo un giro di review adversariale di sicurezza con sette finding. Il debito chiedeva di fissare valori e li ha ottenuti, ma il risultato piu importante e stato un risultato negativo riportato onestamente: alpha non e una curva con un ottimo, e un'identita, e il modello non poteva sceglierlo. Difendibilita e significato del reddito sono lo stesso numero letto due volte, la cattura e lineare senza ginocchio, e nessun valore e selezionato dall'aritmetica. Il valore resta quindi una scelta dell'operatore su un budget dichiarato, e la spec lo scrive invece di nasconderlo dietro una raccomandazione.\n\nIl debito lascia dietro di se piu di quanto chiudesse, ed e il segno che l'istruttoria ha funzionato: una ADR nuova, ADR-010, e due decisioni di prodotto ancora aperte che nessuno sapeva di dover prendere."
debt_events:
  - schema_version: "1"
    id: "DEBT-007-EVENT-001"
    timestamp: "2026-08-25T01:51:01.942934400+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Promosso a debito dal Lead durante la review di SPEC-004 e alla luce di ADR-007, su mandato dell'operatore di salvare tutti i debiti emergenti. E il ponte fra la decisione anti-Sybil appena presa e la taratura economica di M-02."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-007-EVENT-002"
    timestamp: "2026-08-25T16:32:34.040878900+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "project-lead"
    rationale: "Chiuso con SPEC-007, accettata dopo un giro di review adversariale di sicurezza con sette finding. Il debito chiedeva di fissare valori e li ha ottenuti, ma il risultato piu importante e stato un risultato negativo riportato onestamente: alpha non e una curva con un ottimo, e un'identita, e il modello non poteva sceglierlo. Difendibilita e significato del reddito sono lo stesso numero letto due volte, la cattura e lineare senza ginocchio, e nessun valore e selezionato dall'aritmetica. Il valore resta quindi una scelta dell'operatore su un budget dichiarato, e la spec lo scrive invece di nasconderlo dietro una raccomandazione.\n\nIl debito lascia dietro di se piu di quanto chiudesse, ed e il segno che l'istruttoria ha funzionato: una ADR nuova, ADR-010, e due decisioni di prodotto ancora aperte che nessuno sapeva di dover prendere."
    evidence_refs: ["SPEC-007", "REVIEW-011", "ADR-007", "ADR-010"]
---
# La forma del reddito di esistenza non e decisa e determina alpha

## Statement

Nessun documento del progetto dichiara se il reddito di esistenza sia un importo fisso per nodo oppure un fondo a tetto per epoca ripartito fra i nodi presenti. ADR-007 adotta la seconda forma come conseguenza della decisione anti-Sybil, ma i parametri concreti (tetto del fondo, frazione alpha dell'emissione che vi transita, criterio di ripartizione) restano da fissare, e alpha determina direttamente quale quota di emissione una flotta di identita emulate puo catturare.

## Evidence and provenance

threat-model.md §6.2.4 e §7, con la raccomandazione esplicita di AGENT-007 di prendere questa decisione prima di tarare ADR-005. Calcolo verificato indipendentemente dal Lead: con alpha=1 una flotta di 10.000 identita emulate contro 1.000 nodi onesti cattura il 90,9% dell'emissione, con alpha=0,1 ne cattura il 9,1%. Requisiti derivati SEC-REQ-16 e SEC-REQ-18.

## Impact and scope boundary

Alpha e il parametro piu importante dell'economia della rete e oggi non esiste da nessuna parte. Senza una decisione esplicita il simulatore economico di M-02 non ha un modello da simulare, e la metrica di successo riformulata da ADR-007 non ha un valore di X da verificare. C'e inoltre una conseguenza di prodotto da comunicare: con il fondo a tetto il reddito di esistenza diventa una quota variabile e non un importo garantito, il che contraddice l'intuizione comune della parola reddito.

## Decision log

Created by project-lead: Promosso a debito dal Lead durante la review di SPEC-004 e alla luce di ADR-007, su mandato dell'operatore di salvare tutti i debiti emergenti. E il ponte fra la decisione anti-Sybil appena presa e la taratura economica di M-02.

## Resolution criteria

Fissati e documentati: forma del reddito di esistenza (adottato il fondo a tetto in ADR-007), tetto per epoca, criterio di ripartizione, valore iniziale di alpha e il suo intervallo di sorveglianza, e il valore X della metrica riformulata. Il rapporto del simulatore di M-02 espone le tre grandezze richieste da SEC-REQ-16. La comunicazione all'utente del fatto che il reddito e una quota variabile e presente nel design del prodotto.

## Resolution evidence

Tutti i criteri di risoluzione sono soddisfatti e documentati in .lmbrain/knowledge/economic-simulation-report.md, con il simulatore in sim/ eseguibile e deterministico a seme fissato, 35 test verdi.

Forma del reddito di esistenza: fondo a tetto per epoca con ripartizione uniforme, che ledger.md gia impone come regola di validita. Tetto di genesi fissato. Criterio di ripartizione motivato: una variante pesata userebbe come pesi storage e compute, cioe renderebbe il reddito un doppione del canale di lavoro e azzererebbe il telefono, che e il dispositivo per cui quel canale esiste.

Alpha fissata a 0,15 con banda di sorveglianza da 0,10 a 0,20, e il valore X della metrica riformulata di ADR-007 fissato al 20 per cento, con l'argomento di dimostrabilita per costruzione che la reviewer ha verificato e dichiarato corretto: la cattura e strettamente inferiore ad alpha per ogni popolazione, quindi il bordo alto della banda e l'unico X dimostrabile.

Tutti e ventidue i parametri di elezione e di reward policy hanno un valore, e la combinazione passa il blocco di vincoli, verificata riga per riga due volte, al documento di genesi e con il limite di mandato spinto al tetto che DEBT-010 rende irreversibile.

Le tre grandezze richieste da SEC-REQ-16 sono nel rapporto, piu una quarta aggiunta in review: la frazione che un nodo onesto conserva sotto l'attacco tollerato, lo 0,99 per cento, che non contiene alpha.

Verdetti dei test di attacco: AT-07 parzialmente coperto, con la qualificazione della soglia d'uso portata in tutti i punti in cui il criterio compare; AT-10 superato su cinque criteri su sei, con il sesto dichiarato non soddisfacibile e non non-soddisfatto.

La comunicazione all'utente del reddito come quota variabile esiste in inglese, e dopo la review dice anche cio che le mancava: che la fetta il nodo finto la prende al posto dell'utente e che la rete non puo impedirlo.

Verifiche indipendenti del Lead. L'identita alpha per N su N piu H riprodotta e coincidente con ADR-007 a alpha uno e a alpha zero virgola uno. La diluizione H su N piu H verificata su tre popolazioni, e non contiene alpha. Il blocco di vincoli sui parametri scelti, verificato anche col limite di mandato al tetto. La cattura per attrito con min_set a 18, che si ferma a 27-19-18. Il congelamento degli interi piccoli sotto il rapporto cinque quarti, che collassa c, m e cooldown in un punto. L'erosione del rapporto min_set su V, che porta a una cattura al cinquanta per cento in due confini dopo due documenti leciti. L'invarianza dell'importo assoluto dirottato rispetto all'uso, 15725 crediti identici. E la soglia d'uso reale a cui la banda diventa tenibile, il 70,6 per cento e non il 25 dichiarato.

Tre scoperte che il debito non chiedeva e che sono confluite altrove. La prima ha contestato un'assunzione di ADR-007: abbassare alpha non protegge il dispositivo onesto, la cui perdita non contiene alpha, perche ADR-007 misurava la grandezza rivolta all'attaccante e quella rivolta all'utente non era scritta. La seconda ha prodotto ADR-010, perche la difesa anti-Sybil viveva interamente in un documento governato privo di limiti di magnitudine. La terza e emersa in remediation ed e un'autocorrezione dell'implementatrice: l'importo dirottato non dipende dall'uso, quindi il criterio assoluto e onesto solo se il fondo di genesi e dimensionato sui nodi onesti presenti al lancio, e questa e una disposizione ulteriore rispetto alle tre di ADR-010 perche vincola cio che la genesi deve contenere invece di cio che la governance puo fare.
