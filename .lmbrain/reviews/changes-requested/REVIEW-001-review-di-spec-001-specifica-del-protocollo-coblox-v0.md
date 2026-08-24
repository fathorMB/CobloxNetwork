---
id: REVIEW-001
# Note: Quote the title if it contains a colon
title: "Review di SPEC-001 — Specifica del protocollo Coblox v0"
status: changes-requested
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-001
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [requirements-completeness, correctness]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-001-EVENT-001"
    timestamp: "2026-08-25T00:45:00.167955300+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-XXX"
  - schema_version: "1"
    id: "REVIEW-001-EVENT-002"
    timestamp: "2026-08-25T00:46:09.231957800+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "I sei criteri di accettazione sono soddisfatti e la qualita e alta; GATE-COHERENCE verificato indipendentemente dal Lead (16 esempi canonici, 10 link interni risolti, 12 messaggi con schema). Le modifiche richieste riguardano esclusivamente l'allineamento con ADR-006, accettato dopo l'avvio del lavoro: manca la causale di mint per l'emissione al creatore (RF-001), manca il campo repliche nel manifest (RF-002), e il prezzo di hosting e dichiarato dal publisher invece che dal listino di protocollo (RF-003). GATE-SECREVIEW resta da eseguire ed e di competenza del Lead."
    evidence_refs: ["SPEC-001", "ADR-006", "docs/protocol/ledger.md", "docs/protocol/app-manifest.md"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-001-EVENT-003"
    timestamp: "2026-08-25T00:49:52.559688200+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "Remediated all requested ADR-006 alignment findings: RF-001 adds publisher_reward mint eligibility committed to finalized active subscriptions with uniqueness and validation rules; RF-002 adds signed desired_replicas with bounds and protocol-owned host assignment; RF-003 removes publisher-set hosting unit prices and binds app_hosting burns to the active signed protocol rate card. Revalidated all 17 canonical examples, document sections, and targeted findings."
    evidence_refs: ["SPEC-001", "ADR-006", "docs/protocol/ledger.md", "docs/protocol/app-manifest.md"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-001-EVENT-004"
    timestamp: "2026-08-25T00:51:45.965211400+02:00"
    action: "remediation"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "implementation-specialist"
    reason: "AGENT-001 ha rimediato RF-001, RF-002 e RF-003 nei documenti di protocollo: aggiunta la causale di mint publisher_reward con commitment Merkle degli abbonati attivi, aggiunto deployment.desired_replicas al manifest, e spostata la tariffa di hosting sul rate card di protocollo (hosting.rate_source = protocol). Remediation segnalata come completata dall'operatore."
    evidence_refs: ["docs/protocol/ledger.md", "docs/protocol/app-manifest.md"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
  - schema_version: "1"
    id: "REVIEW-001-EVENT-005"
    timestamp: "2026-08-25T00:51:55.933839500+02:00"
    action: "remediation-verification"
    from_status: "changes-requested"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "Verifica indipendente del Lead sui documenti aggiornati. RF-001 chiuso: MintBody ammette publisher_reward con app_id, active_subscriber_count e active_subscription_root; i validatori ricalcolano conteggio e radice dai burn finalizzati, deduplicando per payer, escludendo il node ID del publisher stesso e ammettendo al massimo un mint per (app_id, reward_epoch). RF-002 chiuso: deployment.desired_replicas obbligatorio 1-1024, con nota esplicita che la selezione degli host spetta al protocollo. RF-003 chiuso: hosting.rate_source vincolato al literal protocol, il publisher non puo dichiarare ne abbassare una tariffa di hosting, e il pricing_hash del burn di hosting punta al rate card firmato; il prezzo di abbonamento resta del publisher. Riesecuzione delle verifiche automatiche: 17 esempi JSON tutti canonici, 11 link interni tutti risolti."
    evidence_refs: ["docs/protocol/ledger.md", "docs/protocol/app-manifest.md", "ADR-006"]
    implementation_agent: "AGENT-001"
    remediation_agent: "AGENT-001"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned pending -> changes-requested"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "recorded review remediation"
  - date: 2026-08-25
    action: "recorded review remediation-verification"
---
# Review

## Outcome

**Changes requested**, per un solo motivo: allineamento con [ADR-006], accettato *dopo* l'avvio del lavoro. I sei criteri di accettazione originali sono tutti soddisfatti e la qualità del lavoro è alta. I finding non sono difetti di esecuzione ma scope aggiunto da una decisione successiva.

## Acceptance-criteria compliance

| Criterio | Esito | Evidenza |
| --- | --- | --- |
| I cinque documenti esistono e coprono lo scope, senza sezioni vuote non marcate `DRAFT` | Pass | 5 file in `docs/protocol/`, 1268 righe totali; sezioni `DRAFT` presenti solo in README, identity, ledger, app-manifest e tutte con alternative e owner |
| Ogni formato ha almeno un esempio concreto serializzato | Pass | 16 esempi, verificati indipendentemente dal Lead (vedi sotto) |
| Enrollment anti-Sybil specificato: costo, verifica, e perché non aggirabile | Pass | `identity.md` §"One-time anti-Sybil proof of work": input di PoW che lega network ID, chiave pubblica e parameter set; costo atteso `N × 2^difficulty_bits`; dichiara esplicitamente il limite ("proof of work is a cost, not proof of personhood") |
| Le transazioni mint/burn coprono i flussi di [ADR-005] senza trasferimenti utente→utente | Pass | `ledger.md` invariante 3: nessuna transazione ha sorgente *e* destinazione controllate dall'utente, quindi il trasferimento diretto è **strutturalmente non rappresentabile**, non solo vietato a parole |
| Percorso di verifica del light client descritto passo-passo | Pass | `ledger.md` §"Light-client balance verification", 7 passi da trust anchor di genesi a saldo autenticato |
| Le sezioni `DRAFT` elencano alternative e chi decide | Pass | 4 sezioni `DRAFT`, ciascuna con alternative delimitate e owner nominato |

## Code observations

Lavoro di livello produttivo, con scelte progettuali che vanno oltre il compitino:

- **La proibizione del trasferimento è strutturale.** L'invariante 3 di `ledger.md` rende il trasferimento utente→utente non esprimibile nello schema. Un vincolo di prodotto è stato tradotto in una proprietà del formato: è il modo giusto di difendere l'esclusione permanente di [[PROJECT]].
- **Separazione di dominio sistematica.** Ogni firma ha il proprio dominio ASCII con terminatore zero, quindi una firma non è riutilizzabile in un contesto diverso.
- **Canonicalizzazione motivata.** JCS (RFC 8785) con Protobuf e CBOR scartati con motivazione esplicita; gli interi `u64` viaggiano come stringhe per evitare le differenze di precisione tra linguaggi ospite — dettaglio che evita una classe intera di bug di interoperabilità.
- **Onestà sui limiti.** Il documento dichiara che il proof of work è un costo e non una prova di persona, e che i rate limit non vanno contati come garanzia crittografica. È esattamente il tipo di franchezza che serve in una specifica di sicurezza.
- **Chiave di consenso distinta da quella di trasporto** per i validatori: la compromissione dell'identità libp2p non falsifica immediatamente i voti.
- **AutoNAT v1 come baseline** con v2 trattata come opzionale perché la sua specifica ufficiale è ancora in bozza: verifica sulle fonti correnti, come richiesto dalla spec.

## Tests and verification

`GATE-COHERENCE` (owner: agent, before-submit): **verificato in modo indipendente dal Lead**, non accettato sulla fiducia del transcript.

```text
$ python - # ricanonicalizza ogni esempio ```json e confronta con il testo
examples=16 non_canonical_or_failed=0

$ python - # risolve link e ancore interne tra i documenti
internal_links=10 broken=0
```

Verificato inoltre a mano che l'enum dei messaggi di `wire.md` contiene esattamente i 12 tipi dichiarati e che ognuno ha il proprio schema di payload nel catalogo.

`GATE-SECREVIEW` (owner: lead, before-done): **non ancora eseguito**. Richiede la review di sicurezza di AGENT-007, il cui dispatch necessita autorizzazione esplicita dell'operatore. Correttamente l'implementatore non l'ha spuntato.

## Production quality and documentation compliance

Conforme a [[QUALITY]]. Nessuna eccezione di policy usata, nessun contenuto segnaposto spacciato per definitivo: i valori non decisi sono marcati `DRAFT` con alternative e owner, e il README dichiara che un deployment senza parametri firmati non può chiamarsi mainnet. La stima di effort è stata registrata con osservazione motivata (`sol` osservato = `sol` previsto).

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

- RF-001 | category=requirements-completeness | severity=medium | criterion=allineamento con [ADR-006] | remediation=`ledger.md` non prevede l'emissione al creatore. `MintBody.reason` ammette solo `existence_income` e `work_compensation`; [ADR-006] introduce una quota emessa al publisher proporzionale agli abbonati attivi. Aggiungere la terza causale con la propria evidenza di eleggibilità (prova degli abbonamenti attivi nell'epoca), coerente con l'invariante "ogni mint è legato a evidenza verificabile". Aggiornare di conseguenza la frase di `app-manifest.md` §Pricing che oggi cita solo i mint agli host.
- RF-002 | category=requirements-completeness | severity=medium | criterion=allineamento con [ADR-006] | remediation=Il manifest non ha un campo per il **numero di repliche desiderate**, richiesto dal passo 1 del flusso di pubblicazione di [ADR-006]. Aggiungerlo a `AppManifest` con i propri limiti, oppure — se si conclude che le repliche appartengono all'ordine di hosting e non al manifest — documentare esplicitamente quella separazione e dove vive il campo.
- RF-003 | category=correctness | severity=medium | criterion=allineamento con [ADR-006] | remediation=Contraddizione sui prezzi di hosting. `app-manifest.md` §Pricing fa dichiarare al publisher `hosting.microtokens_per_gib_hour`, ma [ADR-006] stabilisce che il costo dell'hosting segue un **listino fissato dal protocollo**, non dal publisher (che è il pagatore, non il venditore). Così com'è, un publisher potrebbe dichiarare un prezzo di hosting arbitrariamente basso. Il prezzo di abbonamento resta invece legittimamente del publisher. Risolvere spostando la tariffa di hosting nei parametri di rete firmati e lasciando nel manifest il solo riferimento, oppure motivare perché la lettura corrente è compatibile.

## Required follow-up

1. ~~L'implementatore rimedia RF-001, RF-002 e RF-003.~~ **Fatto e verificato dal Lead il 2026-08-25** (vedi sotto).
2. Il Lead ottiene la review di sicurezza di AGENT-007 su identità, enrollment e verifica light client (`GATE-SECREVIEW`). Dispatch autorizzato dall'operatore il 2026-08-25 ed eseguito; **in attesa dell'esito**.
3. Solo dopo l'esito di AGENT-007 la spec può passare a `done`.

## Esito della remediation

Tutti e tre i finding sono chiusi, con verifica indipendente del Lead sui documenti aggiornati.

- **RF-001 chiuso.** `MintBody` ammette ora `publisher_reward` con `app_id`, `active_subscriber_count` e `active_subscription_root`. La soluzione va oltre il minimo richiesto: i validatori ricalcolano conteggio e radice dai burn di abbonamento finalizzati, raggruppando per payer (un abbonato conta una volta sola), escludendo il node ID del publisher stesso, e ammettendo al massimo un mint per `(app_id, reward_epoch)`. Il commitment Merkle impedisce a un proponente di inventare abbonati. Restano correttamente rimandati al threat model gli abbonati Sybil posseduti dal publisher.
- **RF-002 chiuso.** `deployment.desired_replicas` è obbligatorio nell'intervallo 1–1.024, con la precisazione esplicita che la selezione degli host spetta al protocollo e non al publisher, e che il rifiuto di un host o il blocco di rete provocano riassegnazione senza modificare il manifest — coerente con il consenso dell'ospite previsto da [ADR-006].
- **RF-003 chiuso.** `hosting.rate_source` è vincolato al letterale `protocol`: il publisher non può dichiarare né abbassare una tariffa di hosting, e il `pricing_hash` del burn di hosting identifica il rate card firmato attivo per l'epoca fatturata. Il prezzo di abbonamento resta legittimamente del publisher. La correzione è coerente su entrambi i documenti.

Verifiche automatiche rieseguite sui documenti aggiornati:

```text
json_examples=17 non_canonical_or_failed=0
internal_links=11 broken=0
```

## Final decision

Changes requested. Il lavoro è sostanzialmente accettato nel merito; le tre modifiche richieste servono ad allineare la specifica a [ADR-006], accettato dopo l'avvio dell'implementazione, prima che i documenti diventino il contratto su cui M-02 costruisce.
