---
id: ADR-007
# Note: Quote the title if it contains a colon
title: "Posizione anti-Sybil: difesa economica e pavimento memory-hard"
status: accepted
decision_date: 2026-08-25
decider: AGENT-LEAD
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: [ADR-002, ADR-005, ADR-006]
tags: [architecture, security]
created: 2026-08-25
updated: 2026-08-25
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned proposed -> accepted"
---
# Posizione anti-Sybil: difesa economica e pavimento memory-hard

> **Decisione presa dal Lead su delega esplicita dell'operatore** (2026-08-25, "occupatene tu"). Tocca una promessa di prodotto in [[PROJECT]]: se l'operatore non concorda, questa ADR va superata, non modificata in silenzio.

## Context

[REVIEW-002] RF-005 ha dimostrato con misure che il proof of work SHA-256 dell'enrollment non fornisce resistenza Sybil su hardware commodity: il rapporto telefono/GPU è di circa 2.750 a 1, e nessun valore di `difficulty_bits` nell'intervallo 18–40 dichiarato in `identity.md` è insieme tollerabile su Android e costoso per un attaccante. [SPEC-004] ha poi istruito la scelta con quattro opzioni, costi e metriche candidate.

Due esclusioni permanenti di [[PROJECT]] eliminano le risposte standard del settore: niente proof-of-work continuo, che è la difesa più efficace contro un flusso perpetuo di identità; e niente valore monetario, che elimina l'intera famiglia proof-of-stake.

L'affermazione strutturale che governa tutto il resto: **il reddito di esistenza è perpetuo, il costo di enrollment è una tantum, e un costo una tantum non può prezzare un flusso perpetuo.** Vale per SHA-256, per Argon2id e per qualunque prova d'ingresso concepibile.

Il threat model ha però individuato una leva che il progetto non sapeva di avere (§6.2.4). Storage e compute sono intrinsecamente difficili da falsificare, perché costano risorse reali; l'availability è intrinsecamente facile, perché costa una firma. Quindi la domanda "quanto è resistente ai Sybil la rete" coincide con precisione matematica con la domanda "quale frazione `α` dell'emissione passa dal canale del reddito di esistenza". Il Lead ha verificato il calcolo in modo indipendente: con `α` pari a 1 una flotta di 10.000 identità emulate contro 1.000 nodi onesti cattura il 90,9% dell'emissione; con `α` pari a 0,1 ne cattura il 9,1%.

## Decision

Si adotta l'**opzione 4a** di [SPEC-004] §7.6: **difesa economica più pavimento memory-hard.**

1. **La resistenza Sybil è un parametro economico, non una garanzia crittografica.** Il reddito di esistenza è erogato da un **fondo a tetto per epoca** e ripartito, non come importo fisso per nodo. La frazione `α` di emissione che vi transita è un parametro pubblicato e sorvegliato. Ne consegue che una flotta di identità emulate non può aumentare l'emissione totale: può solo diluire la propria quota, e quella quota è limitata da `α`.
2. **L'eleggibilità a validatore è ancorata a lavoro difficile da falsificare** (storage e compute dimostrati), mai al solo uptime, che una VPS con SLA batte per costruzione contro qualunque telefono reale.
3. **Argon2id sostituisce SHA-256 nell'enrollment**, come pavimento d'ingresso: alza il costo per 10.000 identità da circa due secondi di GPU a circa quattro minuti. Due ordini di grandezza reali, non un cambiamento di natura. La difficoltà va ricalibrata sull'ordine di 2–6 bit, non i 18–40 che `identity.md` dichiara oggi non-draft. Poiché con una funzione memory-hard verificare costa quanto generare, l'ordine dei controlli di validazione deve mettere il proof of work **per ultimo**, dopo firma, schema e rate limit, per non trasformarlo in un vettore di denial-of-service sui validatori.
4. **Il protocollo dichiara apertamente ciò che non fa.** `identity.md` deve affermare esplicitamente che il protocollo v0 non distingue `N` nodi emulati su un host da `N` dispositivi reali (`SEC-REQ-12`).
5. **L'attestazione hardware resta esclusa**, anche come tier facoltativo, per questa versione. Riapribile solo se cade una delle incertezze di [SPEC-004] §7.5.

### Riformulazione della metrica di successo

La metrica di [[PROJECT]] "zero accrediti a nodi emulati nei test di attacco" **non è raggiungibile** e viene sostituita da:

> Nei test di attacco, una flotta di `N ≥ 10.000` identità emulate su un singolo host: (a) non aumenta l'emissione totale dell'epoca di alcuna quantità; (b) non ottiene alcun accredito nelle categorie `storage` e `compute`; (c) non ottiene più di una quota `X` dichiarata dell'emissione totale dell'epoca; (d) non ottiene alcun seggio di validatore.

Verificabile con i test `AT-07` e `AT-10` di [SPEC-004]. Il valore di `X` è una decisione di prodotto ancora aperta: è la "perdita fisiologica" che il progetto dichiara di tollerare, e va fissata con i dati del simulatore in M-02.

## Alternatives considered

- **Opzione 1, sola difesa economica:** stessa filosofia ma senza pavimento d'ingresso, 10.000 identità al costo di due secondi di GPU. Scartata perché lasciare l'ingresso praticamente gratuito rende ogni altra difesa più fragile del necessario, a fronte di un costo di adozione di Argon2id modesto.
- **Opzione 2, solo Argon2id:** alza il pavimento ma non tocca la leva che conta. Da sola non impedisce a una flotta di catturare una quota alta dell'emissione se `α` è alta.
- **Opzione 3, attestazione hardware come tier:** verificata su documentazione ufficiale corrente in [SPEC-004] §7.5 e risulta **peggiore** di come [ADR-002] l'aveva assunta. Play Integrity non è verificabile da un terzo, quindi in una rete P2P richiederebbe il servizio centrale che [ADR-001] ha scartato, e ha una quota giornaliera predefinita; gli emulatori non sono esclusi ma etichettati. L'attestazione della chiave Android è verificabile offline ma attesta una chiave, non un dispositivo, e un dispositivo può generarne quante ne vuole: resistenza Sybil nulla. Su Windows l'EK è unico ma il sistema lo evita deliberatamente via AIK proprio per impedire la correlazione. Su Linux e headless non esiste alcun percorso documentato: una delle tre piattaforme di prodotto resterebbe senza tier. Scartata.
- **Opzione 4b, 4a più tier certificato facoltativo:** beneficio reale modesto per le stesse ragioni, a fronte di una superficie di attestazione da mantenere per piattaforma e di un rischio di percezione "a due velocità" nella comunità. Rimandata, non esclusa per sempre.
- **Attestazione obbligatoria per il reddito di esistenza:** è l'unica via che consentirebbe di scrivere "zero" nella metrica, ma escluderebbe una piattaforma di prodotto e comunque non fornirebbe unicità per dispositivo. Sconsigliata esplicitamente da [SPEC-004] e qui respinta.

## Consequences

- `identity.md` cambia primitiva di enrollment: Argon2id al posto di SHA-256, nuovo intervallo di difficoltà, ordine dei controlli vincolato. È lavoro del Lotto B su [SPEC-001].
- Il reddito di esistenza diventa una **quota variabile** e non un importo garantito: va comunicato con cura agli utenti, perché contraddice l'intuizione di "reddito" fissa. La forma esatta del fondo è aperta in [DEBT-007].
- `α` diventa il parametro più importante dell'economia e deve comparire nel rapporto del simulatore di M-02 (`SEC-REQ-16`, `SEC-REQ-18`).
- L'eleggibilità a validatore non può più essere definita sul solo uptime: vincola la specifica di elezione di M-02.
- Il progetto rinuncia formalmente a dichiararsi resistente ai Sybil per via crittografica. Resta difendibile, e va dichiarato in questi termini, che la rete è **robusta contro la falsificazione** — saldi, firme, doppia spesa — anche quando non è resistente ai Sybil.

## Annotazione del 2026-08-25 — ciò che [SPEC-007] ha stabilito

> Aggiunta su conferma esplicita dell'operatore, dopo [REVIEW-011]. **La decisione non è superata e nessuno dei cinque punti è falsificato**: AGENT-007 ha stabilito che l'annotazione è la disposizione corretta e non la supersessione. Ciò che segue è ciò che il lavoro di taratura ha reso noto e che questa ADR, quando fu scritta, non poteva sapere.

**`α` non è una manopola: è un'identità, e la scelta non è selezionata dall'aritmetica.** Con il fondo a tetto ripartito uniformemente, la quota catturata da una flotta di `N` identità emulate contro `H` nodi onesti vale esattamente `α · N/(N+H)`, e la cattura è **lineare in `α` su tutto l'intervallo, senza ginocchio**. Il simulatore ha riprodotto i due valori citati sopra e non ha trovato alcun punto preferito: il valore adottato, `α = 0,15` con banda `[0,10 – 0,20]`, è il centro di un budget dichiarato dall'operatore, non un ottimo misurato.

**La grandezza rivolta all'utente non era scritta, ed è la correzione principale.** Questa ADR misura ciò che l'attaccante ottiene, e su quello ha ragione. Ma il reddito di un nodo di sola availability rapportato al reddito medio vale **`α` esattamente**, quindi difendibilità e significato del reddito sono lo stesso numero letto due volte; e la perdita di quel nodo sotto attacco vale **`H/(N+H)` e non contiene `α`**. Ne segue che **abbassare `α` non protegge il dispositivo onesto**: rimpicciolisce il canale per tutti, non la quota dell'onesto dentro il canale. Al banco di prova tollerato da questa ADR — 10.000 emulati contro 100 onesti — il telefono conserva lo 0,99% del proprio reddito qualunque sia `α`. Verificato in modo indipendente dal Lead.

**Il criterio (a) è vero per regola solo a condizione che nessun canale paghi per nodo senza tetto aggregato**, condizione che al momento della stesura non era imposta da nulla. La difesa che questa ADR descrive viveva interamente nella reward policy, un documento governato privo di limiti di magnitudine. [ADR-010] chiude quella superficie.

**Il criterio (c) non è raggiunto durante l'avviamento.** `α` è una grandezza **osservata** e non impostata: al lancio non c'è lavoro che la diluisca, quindi vale circa 1 e la quota catturata è vicina al 99%, contro il `X = 20%` che questa ADR fissa come tollerato. La metrica riformulata è quindi vera **sopra una soglia d'uso dichiarata**, e la determinazione di quella soglia e del dimensionamento del fondo alla genesi è decisione aperta dell'operatore.

**Ciò che il progetto non può dichiarare, e va detto qui perché questa è l'ADR che governa il claim:** che la resistenza ai Sybil sia una proprietà *stabile*. Questa ADR dice correttamente che è un parametro economico e non una garanzia crittografica; la seconda metà è che **ciò che è governato senza limiti di magnitudine non è un parametro, è una preferenza** — formulazione di AGENT-007 in [REVIEW-011], adottata da [ADR-010].

## Review conditions

Rivedere se: il simulatore di M-02 mostra che `α` non può essere tenuta abbastanza bassa senza svuotare di senso il reddito di esistenza; il benchmark reale di Argon2id sul parco dispositivi target risulta molto peggiore degli ordini di grandezza stimati; cade una delle tre incertezze di [SPEC-004] §7.5 sull'attestazione, in particolare se un vTPM risultasse distinguibile da hardware fisico; oppure se l'operatore non concorda con questa decisione presa su delega.
