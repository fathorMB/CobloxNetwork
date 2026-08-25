---
id: REVIEW-015
# Note: Quote the title if it contains a colon
title: "Review of SPEC-010"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-010
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-015-EVENT-001"
    timestamp: "2026-08-25T19:37:56.384160700+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Dodici criteri su dodici e quattro gate verificati dal Lead in modo indipendente e non presi dall'evidenza: 105 test Rust passati, l'inventario PASS, la prova in negativo PASS su dieci classi ciascuna osservata fallire, e il ricalcolo da zero dei due hash nuovi con il metodo validato prima su due fixture non modificate. Nessun finding a carico dell'implementazione. Tre errori del Lead trovati dall'implementatore e registrati, fra cui un conteggio sbagliato nella spec stessa che esisteva per prevenire quella classe di errore. L'inventario ha trovato alla prima esecuzione una quinta occorrenza della famiglia 1, che e la prima evidenza che la gate di ADR-012 funziona."
    evidence_refs: ["SPEC-010", "DEBT-012", "DEBT-008", "ADR-012", "ADR-013"]
    implementation_agent: "AGENT-001"
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [review]
activity:
  - date: 2026-08-25
    action: "transitioned pending -> accepted"
---
# Review

## Outcome

**Accettata senza finding a carico dell'implementazione.** Dodici criteri su dodici soddisfatti, quattro gate verificate, e **tre errori del Lead trovati dall'implementatore** — di cui uno nella spec stessa che esisteva per prevenire quella classe di errore.

## Acceptance-criteria compliance

Tutti e dodici verificati dal Lead in modo indipendente, non presi dall'evidenza.

L'inventario esiste in due metà come la spec chiedeva, e la seconda — quella che conta — è reale: `published_artifacts.py` **ri-deriva meccanicamente** l'insieme dei candidati dai documenti e fallisce quando manifesto e documenti divergono **in entrambe le direzioni**, su dieci classi di difetto. Il manifesto non può quindi invecchiare in silenzio, che era il requisito portante.

La sede scelta è `sim/tools/` con una motivazione che il Lead non aveva previsto e che accetta: sotto `docs/` l'inventario **diventerebbe un quinto artefatto pubblicato** che asserisce valori attesi, quindi capace di invecchiare, quindi bisognoso di una guardia propria. È l'argomento corretto.

La risposta alla domanda generale di [DEBT-012] è un **elenco e non una rassicurazione**, come il criterio chiedeva: 51 preimmagini, 18 nel registro, 8 pubblicate altrove, **25 senza alcun valore pubblicato**, ciascuna con la ragione scritta.

## Code observations

**La scelta su `lifecycle_u8` è migliore di quella che il Lead si aspettava, e per la ragione giusta.** `0x00` è **riservato e invalido**, `active = 0x01`. Non l'ordine di elencazione. Il ragionamento è la domanda della famiglia 3 di `recurring-defects.md` applicata correttamente — *in quale direzione sta il pericolo?* — e la risposta è che il byte zero è ciò che un record troncato o azzerato produce gratis in ogni linguaggio: se significasse `active`, l'incidente produrrebbe lo stato **permissivo** e una foglia che nulla a valle può contraddire. Con `0x00` riservato lo stesso incidente è un rifiuto nel punto in cui accade.

Coerente con la scelta, la fixture `APP-0` è in stato **`suspended`** e non `active`: *una fixture nello stato il cui byte un'implementazione indovinerebbe non prova nulla sulla codifica*, che è la lacuna per cui la fixture esiste.

## Tests and verification

Rieseguito dal Lead in modo indipendente, non letto dall'evidenza.

- `cargo test --locked --workspace` — **105 test passati**, 13 binari, zero falliti.
- `published_artifacts.py` — PASS su nove classi attive, 51 digest e 39 domini classificati.
- `published_artifacts_negative.py` — **PASS su dieci classi, ciascuna osservata fallire** e poi passare a difetto rimosso.
- **Ricalcolo indipendente dei due hash nuovi**, con il metodo validato prima su due fixture **non modificate** (`object_id` e `input_hash` su `00 01 02`, entrambe riprodotte). `account_key` (app) `a881e2e0…` e `app_leaf` `2eac8b0a…` riprodotti entrambi al primo tentativo da preimmagini ricostruite dai documenti.
- Il rifiuto di un `lifecycle_u8` non assegnato è implementato in `merkle.rs` e coperto da un test che nomina la proprietà.

## Production quality and documentation compliance

**Una estensione di perimetro, accettata.** `.github/workflows/ci.yml` non era in *Files and areas involved*, e l'implementatore vi ha aggiunto un job che esegue i quattro strumenti Python. La motivazione scritta accanto è quella giusta: *erano versionati ma non eseguiti da nulla se non da qualcuno che si ricordava*, il che è il modo di fallire che [ADR-012] esiste per chiudere. Una guardia che nessuno esegue è una guardia di cui nessuno si fida. L'action è pinnata a SHA, coerente con la postura del repository.

Nessun commit né push, come il contratto impone.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

**Nessun finding a carico dell'implementazione.**

Vanno invece registrati **tre errori del Lead trovati dall'implementatore**, perché è la traccia che serve e perché il progetto ha stabilito che si registrano.

- **RF-201 | category=correctness | severity=medium | a carico del Lead.** L'analisi dell'esistente di [SPEC-010] dice *«Il registro di conformità in `README.md` elenca **dieci** valori di hash attesi»*. Sono **sedici**, e `conformance_registry.rs` portava già `REGISTRY_ROW_COUNT = 16` nel file che lo stesso paragrafo cita due punti più in là. Un inventario dimensionato su dieci si sarebbe dichiarato completo **su poco più del 60% della tabella** — cioè esattamente il verde più stretto del vero per cui [ADR-012] esiste. **Il Lead aveva l'output del conteggio davanti mentre scriveva la frase.** Corretto dall'implementatore.
- **RF-202 | category=correctness | severity=low | a carico del Lead.** La spec attribuisce la frase di RF-109 a `identity.md`; è in `README.md`. Verificato.
- **RF-203 | category=design | severity=low | a carico del Lead.** L'osservazione sulla triplice sede degli hash era giusta nel sintomo e sbagliata nella forma: le tre sedi **non erano equivalenti**. Il README è l'oracolo; `protocol_hashes.py` **ricalcola**, quindi la sua copia era pura duplicazione ed è stata rimossa — ed è la copia che era già invecchiata una volta, in [SPEC-009], producendo il falso positivo che [ADR-012] cita come precedente. La trascrizione in Rust è deliberata e resta, ora confrontata a ogni esecuzione dalla verifica C5.

**Una quarta contestazione, e non è un errore del Lead ma un miglioramento della decisione.** Su RF-110 la spec offriva una scelta *«riformulare la frase oppure cambiare la regola»*. L'implementatore ha stabilito che l'**«oppure» è sbagliato**: la sola riformulazione lascerebbe il primo passo dello scudo di ammissione a costare all'attaccante un giro, contraddicendo l'argomento della sezione stessa. Ha fatto entrambe — regola *e* frase — e con un tetto `k` sui nonce in sospeso il costo è un indirizzo ogni **`k`** slot, non ogni slot. È il tipo di contestazione che il mandato chiede.

## Required follow-up

**Una quinta occorrenza della famiglia 1, trovata dall'inventario alla sua prima esecuzione.** `README.md` impone che `challenge_id` **debba essere uguale** a `request_hash`; l'esempio canonico di `challenge_evidence` in `ledger.md` portava due valori diversi, rispecchiati in `canonical_serialization.rs`. Un esempio pubblicato che asseriva uno stato che nessuna rete conforme può raggiungere. **Verificato dal Lead nel diff.** Corretto, dichiarato in entrambi i documenti, e ora sorvegliato dalla verifica C9. È l'unico letterale di hash pubblicato che questa passata ha cambiato, ed è un segnaposto illustrativo: nessuna riga del registro si è mossa.

L'occorrenza va aggiunta alla tabella di [ADR-012] e al conteggio di `recurring-defects.md`. **Il fatto che l'abbia trovata lo strumento e non una persona è la prima evidenza che la gate funziona**, e va registrato come tale.

**Tre segnalazioni fuori ambito, da valutare e non da chiudere qui:**

1. `validator_set_hash` è, **solo fra le preimmagini a dominio separato, non legato a `chain_id`**. Va portato ad AGENT-007.
2. L'albero Merkle delle transazioni, quello degli abbonamenti e quello dell'insieme eleggibile **non hanno un esempio lavorato pubblicato**; quello dei candidati sì.
3. Le lacune più chiare fra le 25 preimmagini scoperte sono `enrollment_pow_salt` e `node_leaf`.

## Final decision

**Accettata.** `GATE-INVENTORY-ANSWER` è attestata dal Lead su verifica indipendente: la risposta a [DEBT-012] è enumerata, e due hash nuovi su due sono stati ricalcolati da zero con il metodo validato prima su fixture non modificate.

Le tre caselle `owner=agent` sono spuntate dal Lead in sede di review, non dall'implementatore, perché `CONTRACT.md` gli riserva la sola evidenza di implementazione — ed è il comportamento corretto: la stessa restrizione, in [SPEC-008], aveva prodotto una gate strutturalmente insoddisfacibile che il Lead aveva scritto male.
