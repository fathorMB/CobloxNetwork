---
id: DEBT-023
title: "GATE-SECREVIEW di SPEC-015 derogata: la guida pubblica non e' mai passata da una review di sicurezza"
status: resolved
category: "verification"
severity: "medium"
origin_severity: null
area: "design"
milestone: "M-08"
owner: "AGENT-007"
origin_artifact: "SPEC-015"
origin_ref: "GATE-SECREVIEW"
related_specs: ["SPEC-015"]
related_reviews: ["REVIEW-024"]
related_decisions: ["ADR-012"]
target_specs: []
blocked_by: []
resolution_refs: ["REVIEW-031","SPEC-015"]
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-26
tags: ["verification-gap","documentation","security"]
links: []
activity:
  - date: 2026-08-26
    action: "resolved: L'obbligo che questo debito portava e' discharged: la review c'e' stata prima della pubblicazione, e la guida non e' mai stata raggiungibile da un lettore esterno nel frattempo.\n\nUna clausola dei criteri e' pero' soddisfatta in sostanza e NON alla lettera, e va detto invece di lasciarlo dedurre. I criteri dicevano \"con la versione definitiva del protocollo sotto mano\". Il protocollo non e' definitivo: M-02 e' in corso e altre spec lo muoveranno. La review e' stata fatta contro il protocollo corrente, che e' il meglio disponibile e si e' rivelato sufficiente - ha trovato tre high che il 25 agosto non esistevano - ma non e' cio' che la clausola diceva.\n\nIl debito si chiude lo stesso perche' cio' che proteggeva era la pubblicazione, e quella e' protetta. Cio' che la clausola \"definitiva\" intercettava davvero e' un rischio diverso e permanente: la guida invecchia quando il protocollo si muove, e nessuna probe se ne accorge. E' passato su un debito proprio.\n\nVale la pena registrare che la scommessa dell'operatore ha pagato in modo misurabile. Rinviare la review invece di farla subito sembrava una deroga; si e' rivelato l'unico modo di farle trovare qualcosa. Una review il 25 agosto avrebbe trovato due low e avrebbe firmato una pagina che ventiquattro ore dopo conteneva tre affermazioni false."
debt_events:
  - schema_version: "1"
    id: "DEBT-023-EVENT-001"
    timestamp: "2026-08-26T00:46:58.108558900+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "L'operatore ha chiesto di chiudere SPEC-015. Chiuderla con la gate non soddisfatta e senza registrare nulla avrebbe fatto sparire l'obbligo insieme alla spec: e' esattamente la famiglia 2 del censimento — la pretesa («fatto») rimasta avanti rispetto alla regola («la sicurezza l'ha guardata») — commessa dal Lead nel gesto di chiusura.\n\nIl progetto ha gia' il precedente esatto e conviene seguirlo invece di inventare: DEBT-001 si intitolava «La pipeline CI non e' mai stata eseguita: GATE-CI-GREEN derogato», portava `origin_ref: GATE-CI-GREEN`, ed e' rimasto aperto finche' la CI non ha davvero girato. Una gate derogata diventa un debito che porta il nome della gate. Cosi' l'obbligo sopravvive alla chiusura della spec, ha un proprietario, e ha un innesco preciso invece di una buona intenzione.\n\nAlternativa scartata: lasciare SPEC-015 in `review` a tempo indeterminato. Sarebbe stato onesto ma attaccato all'oggetto sbagliato — una spec in `review` dice che qualcuno la sta rivedendo, e nessuno la sta rivedendo. Avrebbe inoltre tenuto acceso un segnale in ogni digest per mesi, e un segnale che resta acceso troppo a lungo smette di essere letto."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-023-EVENT-002"
    timestamp: "2026-08-26T13:02:34.278917300+02:00"
    action: "resolved"
    from_status: "open"
    to_status: "resolved"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "L'obbligo che questo debito portava e' discharged: la review c'e' stata prima della pubblicazione, e la guida non e' mai stata raggiungibile da un lettore esterno nel frattempo.\n\nUna clausola dei criteri e' pero' soddisfatta in sostanza e NON alla lettera, e va detto invece di lasciarlo dedurre. I criteri dicevano \"con la versione definitiva del protocollo sotto mano\". Il protocollo non e' definitivo: M-02 e' in corso e altre spec lo muoveranno. La review e' stata fatta contro il protocollo corrente, che e' il meglio disponibile e si e' rivelato sufficiente - ha trovato tre high che il 25 agosto non esistevano - ma non e' cio' che la clausola diceva.\n\nIl debito si chiude lo stesso perche' cio' che proteggeva era la pubblicazione, e quella e' protetta. Cio' che la clausola \"definitiva\" intercettava davvero e' un rischio diverso e permanente: la guida invecchia quando il protocollo si muove, e nessuna probe se ne accorge. E' passato su un debito proprio.\n\nVale la pena registrare che la scommessa dell'operatore ha pagato in modo misurabile. Rinviare la review invece di farla subito sembrava una deroga; si e' rivelato l'unico modo di farle trovare qualcosa. Una review il 25 agosto avrebbe trovato due low e avrebbe firmato una pagina che ventiquattro ore dopo conteneva tre affermazioni false."
    evidence_refs: ["REVIEW-031", "SPEC-015"]
---
# GATE-SECREVIEW di SPEC-015 derogata: la guida pubblica non e' mai passata da una review di sicurezza

## Statement

La guida pubblica al funzionamento di Coblox, in `.lmbrain/design/coblox-public-guide/`, e' stata chiusa con `GATE-SECREVIEW` non soddisfatta. AGENT-007 non l'ha mai letta. La gate non e' stata saltata: e' stata **rinviata dall'operatore il 2026-08-25 con una condizione dichiarata** — la review si fa prima della pubblicazione, con la versione definitiva del protocollo sotto mano. Questo debito e' il portatore di quell'obbligo, perche' SPEC-015 non lo e' piu'.

## Evidence and provenance

`.lmbrain/specs/*/SPEC-015-*.md`: `GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact` risulta non spuntata mentre le altre quattro gate lo sono, i nove criteri di accettazione sono tutti spuntati, REVIEW-024 e' accettata senza alcun finding a carico dell'implementazione, e `GATE-OPERATOR-LOOK` e' attestata dall'operatore il 2026-08-26. La deroga e la sua condizione sono registrate in `STATUS.md` al commit `94fde8d`. La chiusura di SPEC-015 e' avvenuta con `spec_done` forzato, e la ragione del forzamento nomina questo debito.

## Impact and scope boundary

La guida e' l'artefatto che piu' di ogni altro **insegna** il sistema: e' scritta per l'on-boarding e per la trasparenza, quindi cio' che afferma verra' creduto e ripetuto. Una descrizione puo' fare due danni che il codice non fa. Puo' **insegnare una forma inammissibile** — famiglia 1 del censimento, sette occorrenze, la piu' numerosa del progetto, e nasce sempre da un artefatto pubblicato. E puo' **dire piu' di quanto la regola garantisca** — famiglia 2: su una pagina che spiega le difese, una pretesa in eccesso e' una falsa assicurazione data a chi non ha modo di verificarla.

Il rischio residuo e' pero' piu' stretto di quanto la severita' suggerirebbe, ed e' la ragione per cui questo debito e' `medium` e non `high`. La pagina **non e' pubblicata**: vive in `.lmbrain/design/` e nessun lettore esterno la raggiunge. Ogni affermazione di proprieta' del filo principale ha gia' una probe in `published_artifacts.toml`, quindi la classe «afferma cio' che nessuna regola sostiene» e' gia' sbarrata meccanicamente. Cio' che manca e' l'occhio che nessuno strumento sostituisce: se le tre cose scomode siano dette abbastanza presto, se un'omissione sia una semplificazione o una reticenza, e se la pagina nel suo insieme lasci un'idea della sicurezza della rete piu' generosa del vero.

## Decision log

Created by AGENT-LEAD: L'operatore ha chiesto di chiudere SPEC-015. Chiuderla con la gate non soddisfatta e senza registrare nulla avrebbe fatto sparire l'obbligo insieme alla spec: e' esattamente la famiglia 2 del censimento — la pretesa («fatto») rimasta avanti rispetto alla regola («la sicurezza l'ha guardata») — commessa dal Lead nel gesto di chiusura.

Il progetto ha gia' il precedente esatto e conviene seguirlo invece di inventare: DEBT-001 si intitolava «La pipeline CI non e' mai stata eseguita: GATE-CI-GREEN derogato», portava `origin_ref: GATE-CI-GREEN`, ed e' rimasto aperto finche' la CI non ha davvero girato. Una gate derogata diventa un debito che porta il nome della gate. Cosi' l'obbligo sopravvive alla chiusura della spec, ha un proprietario, e ha un innesco preciso invece di una buona intenzione.

Alternativa scartata: lasciare SPEC-015 in `review` a tempo indeterminato. Sarebbe stato onesto ma attaccato all'oggetto sbagliato — una spec in `review` dice che qualcuno la sta rivedendo, e nessuno la sta rivedendo. Avrebbe inoltre tenuto acceso un segnale in ogni digest per mesi, e un segnale che resta acceso troppo a lungo smette di essere letto.

## Resolution criteria

AGENT-007 rivede la guida **con la versione definitiva del protocollo sotto mano**, e la review e' accettata dal Lead, **prima** che la guida sia pubblicata in qualunque forma raggiungibile da un lettore esterno.

La review deve trattare esplicitamente tre cose, che sono quelle che nessuna probe puo' misurare:
1. Se una qualunque affermazione della pagina, letta da chi non conosce il protocollo, **insegni una forma che il protocollo non ammette**.
2. Se le tre cose scomode dichiarate in *Context* siano leggibili **a blocchi chiusi** e non solo dentro i dettagli apribili — il test meccanico verifica che ci siano, non che arrivino in tempo.
3. Se la pagina lasci intendere garanzie di sicurezza piu' forti di quelle che `SECURITY.md` dichiara, con attenzione ai due punti dove il progetto ha gia' sbagliato per eccesso: la resistenza Sybil, che e' **economica e non crittografica**, e la soglia di controllo del set, che e' **circa quattro noni** e non i due terzi che l'intuizione suggerisce.

**La condizione di sblocco e' la pubblicazione, non una data.** Se la guida resta interna, il debito resta aperto senza urgenza. Se qualcuno propone di pubblicarla, questo debito e' il blocco.

## Resolution evidence

Chiuso da REVIEW-031, security review di AGENT-007, accettata dal Lead dopo un giro di remediation di AGENT-006.

Verdetto: pubblicabile dopo correzioni. Tre high, quattro medium, due low; i tre high chiusi e riverificati dal Lead sui file - "whatever anybody intends" zero occorrenze, "four ninths" due, la definizione di periodo in blocchi presente.

I tre criteri del debito sono soddisfatti. Criterio 1, forme inammissibili insegnate: tre trovate e corrette, piu' quattro medium. Criterio 2, le cose scomode leggibili a blocchi chiusi: verificato che nessuno degli otto details porti open, che nessun blocco plainly sia annidato in un apribile, e che quattro affermazioni scomode stiano nel filo; il residuo e' RF-007, che resta una decisione. Criterio 3, garanzie piu' forti di SECURITY.md: i due punti nominati per nome dal debito - Sybil economica e non crittografica, e la soglia dei quattro noni - sono entrambi corretti nella pagina.

Strumenti: published_artifacts.py PASS con 137 probe C10 da 126; prova in negativo PASS con ogni probe osservata fallire da sola; check-guide-pairs.mjs PASS con 76 claims da 65; check-contrast.mjs 130 su 130. Le tre mutazioni che reintroducono i bloccanti verbatim ora fanno rosso, e prima non facevano fallire nulla.</resolution_evidence>
</invoke>

## Nota del Lead — 2026-08-26, dopo la chiusura

**Come va letto il criterio 2, deciso dall'operatore.** Il criterio chiedeva che le cose scomode fossero leggibili «a blocchi chiusi» entro tre sezioni. [REVIEW-031] RF-007 ha rilevato che la quarta — la permanenza pubblica e correlabile degli abbonamenti — arriva alla **quarta** sezione, e AGENT-006 si e' fermata invece di assorbire la differenza in silenzio, che era la cosa giusta.

**Decisione: resta dov'e'.** La ragione non e' il costo — sono sei parole — ma il fatto che spostarla peggiorerebbe cio' che il criterio protegge. Un fatto scomodo su una funzione che il lettore **non ha ancora incontrato** e' un fatto senza referente: dire in §03 che gli abbonamenti sono pubblici per sempre, prima che §04 spieghi cos'e' un abbonamento, non mette in guardia — confonde. E §03 dice gia' che il registro e' pubblico, che e' la **categoria** del disagio; la permanenza degli abbonamenti ne e' un caso, non una sorpresa di genere nuovo.

**Il criterio va quindi letto come «entro la sezione che introduce la cosa», e non «entro le prime tre».** Lo scrivo qui perche' un criterio scritto una volta viene riletto alla lettera da chi non c'era, e la lettera qui produrrebbe una pagina peggiore.

**Cio' che la decisione non concede:** non e' una licenza a rimandare le cose scomode alla sezione che fa comodo. Il vincolo che resta e' che una cosa scomoda arrivi **insieme** alla cosa di cui e' scomoda, mai dopo, e mai dietro un blocco apribile.
