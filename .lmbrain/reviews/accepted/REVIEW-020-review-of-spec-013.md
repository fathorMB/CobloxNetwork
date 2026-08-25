---
id: REVIEW-020
# Note: Quote the title if it contains a colon
title: "Review of SPEC-013"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-013
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-001
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-020-EVENT-001"
    timestamp: "2026-08-25T22:26:04.585389900+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Nessun finding. Verificato dal Lead rieseguendo e non letto dall'evidenza: 125 test passati, clippy zero warning, fmt pulito, cinque strumenti versionati OK.\n\nGATE-NO-PUBLISHED-LINK soddisfatta nel senso forte: una ricerca su tutti i documenti di protocollo, su src e sui test restituisce una sola occorrenza di libp2p_peer_id, ed e la riga che dichiara che gli oggetti on-chain non lo contengono piu.\n\nER-0 e l'unico hash pubblicato che si muove, e il Lead l'ha ricalcolato con la validazione piu stringente disponibile: ricostruire il valore precedente dallo schema precedente, cioe lo stesso percorso su un bersaglio noto-buono. Entrambi riprodotti al primo tentativo, il che stabilisce anche una proprieta che nessuna trascrizione affermava, cioe che l'unica differenza fra i due oggetti e la rimozione di quel campo.\n\nGATE-NO-ATTESTATION-REJECTED provata in negativo dal Lead: rimosso dal codice il confronto fra chiave di trasporto attestata e chiave presentata, gate_no_attestation_rejected FAILED; ripristinato, 125 verdi e fmt pulito.\n\nLa conseguenza che ADR-015 indicava come piu probabile fonte di difetto e affrontata e non aggirata: attribuzione legata a sender_node_id e mai al Peer ID effimero, cache di replay indicizzata su sender_node_id e nonce, limitatori a livello di nodo che sopravvivono alla rotazione, e un tetto sulla frequenza di rotazione. Una chiave ruotabile non e piu una chiave che azzera lo stato per peer.\n\nTM-28 resta Stato aperto e in nessun punto risulta dichiarato chiuso, che e la condizione di revisione di ADR-015. Un sospetto del Lead sull'assenza di fixture per l'attestazione e stato verificato e risulta infondato: tutti e tredici i domini di firma sono trattati come domini e non come preimmagini, quindi la tassonomia e uniforme e non c'e deviazione.\n\nGATE-SECREVIEW resta da attestare su una review di AGENT-007 prima di spec_done."
    evidence_refs: ["SPEC-013", "ADR-015", "ADR-012"]
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

# Outcome

**Accettata dal Lead senza finding.** `GATE-SECREVIEW` resta da attestare: la spec la richiede `before-done`, e nel dispatch va detto — come la spec stessa istruisce — che **le superfici segnalate non sono il perimetro**.

## Acceptance-criteria compliance

**`GATE-NO-PUBLISHED-LINK` è soddisfatta nel senso forte.** Una ricerca su tutti i documenti di protocollo, su `coblox-core/src` e sui test restituisce **una sola occorrenza** di `libp2p_peer_id`, ed è la riga di `identity.md` che dichiara che gli oggetti on-chain non lo contengono più. Il legame fra identità di ledger e indirizzo di rete **non è più un fatto di dominio pubblico**, che era l'obiettivo di [ADR-015].

Il primo passo del piano — tracciare che cosa usasse `libp2p_peer_id`, che il Lead aveva dichiarato di non aver fatto — ha una risposta: compariva **solo** negli schemi di `EnrollmentRequest` ed `EnrollmentCertificate`, e nessun meccanismo di consenso, routing o transazione dipendeva dalla sua pubblicazione, perché la scoperta dei peer avviene per DHT, Identify e mDNS.

`identity.md` §*Key hierarchy* è **riscritta** e non annotata: tre chiavi con ruoli distinti, e la doppia derivazione obbligatoria non sopravvive in nessuna forma.

## Code observations

**La proprietà che la vecchia regola comprava è conservata come regola di validità**, non come raccomandazione: un peer privo di attestazione valida è rifiutato e disconnesso. L'attestazione ha una finestra temporale in millisecondi anziché in altezze di catena, con la motivazione corretta — è presentata in sessione a peer che possono non essere sincronizzati, e legarla all'altezza richiederebbe al verificatore di conoscere la catena.

**L'argomento anti-riuso è scritto nel documento**, non lasciato dedurre: chi presenta l'attestazione deve completare l'handshake Noise o QUIC con la chiave che l'attestazione nomina, quindi dimostrarne il possesso.

**La conseguenza che [ADR-015] indicava come più probabile fonte di difetto è affrontata e non aggirata.** `wire.md` acquista una sezione su rotazione, attribuzione e limiti, con tre punti che tengono: l'attribuzione del gossip è legata a `sender_node_id` e mai al Peer ID effimero; la cache di replay è indicizzata su `(sender_node_id, nonce)`, quindi riconnettersi sotto una chiave nuova non azzera nulla; e i limitatori a livello di nodo sopravvivono alla rotazione, con un tetto sulla frequenza di rotazione. **Una chiave ruotabile non è più una chiave che azzera lo stato per peer.**

## Tests and verification

Rieseguito dal Lead: **125 test passati**, clippy zero warning, `fmt` pulito, tutti e cinque gli strumenti versionati OK.

**`ER-0` è l'unico hash pubblicato che si muove, e il Lead l'ha ricalcolato con il metodo validato prima su un valore noto.** La validazione scelta è la più stringente disponibile: ricostruire il valore **precedente** dallo schema precedente, cioè lo stesso percorso di codice su un bersaglio noto-buono.

```text
metodo validato sul valore precedente:
  calcolato : sha256:cb1245f681d732aba57064face8872cd2104a185916ff1f0ac2d2e0651e7fb7f
  pubblicato: sha256:cb1245f681d732aba57064face8872cd2104a185916ff1f0ac2d2e0651e7fb7f
valore nuovo (senza libp2p_peer_id):
  calcolato : sha256:52118f65908736ec7fd837a4d6c1b8c2b3ba28e2f0127cea6e282b311e401e58
  pubblicato: sha256:52118f65908736ec7fd837a4d6c1b8c2b3ba28e2f0127cea6e282b311e401e58
```

Entrambi riprodotti. Ne discende anche una proprietà che nessuna trascrizione affermava: **l'unica differenza fra i due oggetti è la rimozione di quel campo**, perché lo stesso costruttore produce entrambi i valori pubblicati.

**`GATE-NO-ATTESTATION-REJECTED` provata in negativo dal Lead**, non presa dall'evidenza: rimosso dal codice il confronto fra chiave di trasporto attestata e chiave presentata, `gate_no_attestation_rejected` **FAILED**; ripristinato, 125 verdi e `fmt` pulito.

## Production quality and documentation compliance

**TM-28 è aggiornato e resta `Stato: aperto`.** Il documento distingue ciò che questa spec chiude — l'osservatore passivo e fuori sessione — da ciò che resta, cioè l'interlocutore attivo che apre una connessione. **In nessun punto risulta dichiarato chiuso**, che è la condizione di revisione di [ADR-015] e la cosa che il Lead ha controllato per prima.

**Un sospetto del Lead, verificato e infondato.** L'attestazione è registrata nell'inventario come `signature-domain` e **non** come preimmagine, il che sembrava lasciarla senza fixture pubblicata né ragione dichiarata per non averla — la classe di [DEBT-012]. Il controllo mostra che la tassonomia è uniforme: **tutti e tredici** i domini di firma sono trattati così, compresi `coblox-enrollment-certificate-v0` e `coblox-consensus-key-binding-v0`, che sono gli analoghi più stretti. L'attestazione riceve lo stesso trattamento degli altri oggetti firmati — un esempio canonico che fissa la serializzazione, senza digest pubblicato. Nessuna deviazione.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

**Nessun finding.**

## Required follow-up

`GATE-SECREVIEW` su una review di AGENT-007. Le superfici che il Lead giudica probabili, **e che non sono il perimetro**: la finestra temporale dell'attestazione e cosa accade ai suoi bordi con orologi divergenti; l'interazione fra rotazione e i limiti dello stream di enrollment, che è l'unico ad accettare peer di trasporto non autenticati; e se la revoca continui davvero a mordere quando l'identità è presentata in sessione anziché legata al trasporto.

## Final decision

**Accettata**, con `GATE-SECREVIEW` in sospeso. La spec resta in `review` fino all'attestazione.
