---
id: REVIEW-042
# Note: Quote the title if it contains a colon
title: "GATE-SECREVIEW di SPEC-022: la banda che ho disegnato converte la censura in negazione"
status: changes-requested
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-022
reviewer: AGENT-007
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: [security, correctness, documentation]
# Managed append-only history. Use semantic review MCP verbs; do not edit events by hand.
review_events:
  - schema_version: "1"
    id: "REVIEW-042-EVENT-001"
    timestamp: "2026-08-27T03:10:00.000000+02:00"
    action: "submitted"
    from_status: "none"
    to_status: "pending"
    actor_role: "project-lead"
    reason: "review artifact created"
    implementation_agent: "AGENT-002"
  - schema_version: "1"
    id: "REVIEW-042-EVENT-002"
    timestamp: "2026-08-27T01:30:14.876732+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "changes-requested"
    actor_role: "project-lead"
    reason: "GATE-SECREVIEW non superata. Due high, tre medium, tre low, sul primo oggetto di questa gate che sia codice di consenso e non prosa.\n\nIl high che conta di piu' e' contro ADR-017, cioe' contro una decisione del Lead. La banda di key_compromise ammette altezze di inclusione [e-F-G, e-F], larghezza G+1. Il pavimento G >= 1 sta in check_relations; in ElectionBounds NON c'e' alcun _min per quel parametro, mentre il tetto c'e'. Verificato dal Lead. Un consensus_parameters con G = 1 passa ogni vincolo, e da quel momento un key_compromise va firmato a quorum e incluso entro una finestra di DUE BLOCCHI predetta prima che il giro di firma cominci: due blocchi di censura o una riorganizzazione di profondita' due lo invalidano.\n\nE il tetto e' nuovo di SPEC-022. La clausola 4 preesistente aveva pavimento e nessun tetto, quindi un ritardo poteva solo rimandare. ADR-017 aveva corretto REVIEW-036 RF-002 - l'uguaglianza regalava un veto a un proponente per un turno - sostituendola con una banda che regala lo stesso veto a chi censura G+1 blocchi. Il veto e' stato reso piu' caro, non tolto, e il prezzo lo fissa un parametro governato. Il reason con l'urgenza crittografica ha la finestra piu' stretta: l'ordinamento e' invertito rispetto all'urgenza.\n\nRF-002 high: il testo normativo della clausola 2 contraddice la riga 20 che REVIEW-039 RF-001 ha appena imposto, sul singoletto in cui le due letture divergono. Il preambolo dice \"against the finalized state that block builds on\" e \"facts about the block and its ancestors\", la clausola dice \"no finalized revoke_identity is included at a height at most h\". Una revoca inclusa A h non e' in nessuna delle tre cose: lettura letterale, a h=20 il burn e' valido, mentre tabella e codice dicono invalido. La probe pinna la frase nella forma ambigua, quindi la gate e' verde su di essa. Seconda faccia: la giustificazione scritta e' l'ordine di esecuzione intra-blocco, ma il predicato non consulta mai l'ordine - e' a granularita' di altezza, ed e' questa la ragione per cui e' sicuro.\n\nRF-003: i tre parametri hanno due dei tre strati di ADR-010. Manca il limite al tasso di variazione, quindi il cammino non e' un cammino ma un salto in un documento solo.\n\nRF-004: la clausola 3 non e' attuata. validate_effective_height non ha chiamanti fuori dai test, verificato dal Lead, e il test che porta il suo nome la chiama due volte con la stessa altezza: dimostra che la funzione legge il proprio argomento, non quale argomento un verificatore debba passare.\n\nRF-005: cercato il settimo artefatto della passata ADR-012, ne sono stati trovati tre, e uno - SECURITY.md - e' dentro il manifesto dello strumento, con la probe che pinna solo l'apertura della frase resa ambigua.\n\nGATE-TWO-ORACLES: soddisfatta per i digest, NON per AUTH-0. La seconda derivazione non e' mai stata fatta - la trascrizione contiene solo i due oracoli dei digest. La diagnosi: una derivazione fatta dalla regola produce le altezze di flip per esaurimento e trova 20 senza campionare; che la riga 20 mancasse da tabella e test, e che la prosa attribuisse la frontiera alla riga 21 che non separa, sono tre segni della stessa origine - la tabella nuova e' stata ottenuta ribaltando le righe della vecchia. Il Lead aveva dichiarato la gate non riprodotta e aveva scritto che, se non fosse stata indipendente, RF-001 sarebbe stato il sintomo e non la causa. E' il caso.\n\nNon rotto: la parte 1 contro un avversario di inclusione, e in forma chiusa - morso_nuovo(p) <= morso_vecchio(p) per ogni p, quindi e' monotona nella direzione giusta; la derivazione di AUTH-0 rifatta dalla regola come terzo oracolo, che conferma la tabella attuale; app-manifest.md, che non nomina alcuna altezza e non e' un settimo artefatto; TM-21; la regola locale del ricevente; e DEBT-035, che non e' sfruttabile perche' il predicato e' insensibile all'ordine intra-blocco - la terza opzione, non le due che la spec enumerava, e non segnalata al Lead come la spec richiedeva.\n\nRF-001 richiede una correzione ad ADR-017 e non solo alla spec: il pavimento di G va nell'ancora di genesi. E' un difetto di progetto del Lead e va corretto dove il progetto e' stato deciso."
    evidence_refs: ["SPEC-022", "ADR-017", "ADR-010", "REVIEW-036", "REVIEW-039", "DEBT-035"]
    implementation_agent: "AGENT-002"
    remediation_agent: "AGENT-002"
links: [DEBT-035, DEBT-036]
created: 2026-08-27
updated: 2026-08-27
tags: [security, review, ledger]
related_decisions: [ADR-017, ADR-010, ADR-012]
activity:
  - date: 2026-08-27
    action: "created"
  - date: 2026-08-27
    action: "transitioned pending -> changes-requested"
---
# Review

> **`GATE-SECREVIEW` di [SPEC-022]**, eseguita sull'albero a `f006695`. È il primo oggetto di questa gate che sia **codice di consenso** e non prosa.

## Outcome

**Non superata.** Due `high`, tre `medium`, tre `low`.

**Il `high` che conta di più è contro [ADR-017], cioè contro una decisione del Lead**, e dice che il rimedio con cui quell'ADR chiudeva un veto **lo ha reso più caro invece che toglierlo**, mettendone il prezzo in mano a un parametro governato.

E `GATE-TWO-ORACLES` **non era soddisfatta su `AUTH-0`**: la seconda derivazione non è mai stata fatta. Il Lead l'aveva dichiarata non riprodotta in [REVIEW-039] e aveva scritto che, se non fosse stata indipendente, la frontiera mancante sarebbe stata **il sintomo e non la causa**. È il caso.

## Tests and verification

La reviewer ha **eseguito**, non solo letto: baseline 191/0 contata, **dieci mutazioni applicate e ripristinate**, ognuna osservata far fallire almeno un test e in nove casi su dieci esattamente quello che quella regola esiste per tenere. Due file di test temporanei creati, misurati e rimossi; albero riverificato pulito.

**Il Lead ha riverificato i tre fatti portanti** — l'assenza del pavimento di `G` in genesi, il testo della clausola 2, l'assenza di chiamanti di `validate_effective_height` — e tutti e tre reggono.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

RF-001 | category=security | severity=high | criterion=ADR-017 parte 2, banda di `key_compromise` | remediation=`revocation_effective_grace_blocks_min` in `ElectionBounds`, imposto in `check_magnitudes`, con una mutazione che lo faccia fallire
**Il pavimento della banda è scritto dove il set seduto può portarlo a `1`, e il tetto è nuovo di questa consegna.**

Dato un corpo firmato con `effective_height = e`, le altezze di inclusione ammesse sono `[e−F−G, e−F]`: **larghezza `G+1`**. Misurato eseguendo: a `G=1` la finestra è **due blocchi**.

**Verificato dal Lead:** `revocation_effective_grace_blocks_max` esiste ed è ancorato in genesi; **`G >= 1` sta in `check_relations`**, e in `ElectionBounds` **non c'è alcun `_min`** per quel parametro. Un `consensus_parameters` con `G = 1` passa ogni vincolo ed è indistinguibile da governance ordinaria.

**Scenario.** Una coalizione che ha compromesso una chiave e vuole sopravvivere alla propria revoca pubblica `G = 1`. Da quel momento un `key_compromise` va **firmato a quorum e incluso entro una finestra di due blocchi, predetta prima che il giro di firma cominci**. Due blocchi di censura, o una riorganizzazione di profondità due, rendono la transazione **invalida** e obbligano a rifare il quorum. La cadenza non è imposta, quindi la finestra resta due blocchi comunque si muova il tempo reale. **Il `reason` con l'urgenza crittografica è quello con la finestra più stretta: l'ordinamento è invertito rispetto all'urgenza.**

**E questa non è la clausola 4 preesistente.** Quella aveva un pavimento e **nessun tetto**: un ritardo di inclusione poteva solo rimandare. **Il tetto è nuovo di [SPEC-022]**, e con esso la possibilità che un ritardo di inclusione **distrugga** una revoca. [ADR-017] aveva corretto [REVIEW-036] RF-002 — l'uguaglianza regalava un veto a **un** proponente per **un** turno — sostituendola con una banda che regala lo stesso veto a chi censura `G+1` blocchi. **Il veto è stato reso più caro, non tolto, e il prezzo lo fissa un parametro governato.**

RF-002 | category=correctness | severity=high | criterion=clausola 2 di *unrevoked* | remediation=riscrivere clausola, preambolo e la giustificazione sull'ordine, e ripuntare la probe
**Il testo normativo contraddice la riga `20` che [REVIEW-039] RF-001 ha appena imposto, esattamente sul singoletto in cui le due letture divergono.**

Il preambolo dice che entrambe le clausole valgono *«against the finalized state that block builds on»* e che sono *«facts about the block being validated and its ancestors»*. La clausola 2 dice *«no **finalized** `revoke_identity` … is included at a height at most `h`»*. **Verificato dal Lead.**

Una revoca inclusa **a** `h` non è nello stato su cui `h` si costruisce, non è in un antenato, e non è finalizzata mentre `h` viene validato. **Lettura letterale: a `h = 20` la clausola è soddisfatta e il burn è valido.** Tabella e codice dicono invalido.

RF-001 di [REVIEW-039] ha pinnato la frontiera nel codice, nella tabella e in una probe. **La frase che la definisce è rimasta con due letture**, e la probe la tiene ferma nella forma ambigua: **la gate è verde su di essa.**

**Seconda faccia.** La giustificazione scritta del morso è l'ordine di esecuzione intra-blocco — *«class 0 … executes before any spending transaction in the same block»*. **Il predicato non consulta mai l'ordine**: è a granularità di altezza, quindi insensibile all'ordine, ed è questa la ragione per cui è sicuro. Un implementatore che segua la giustificazione scritta implementa un predicato che legge le revoche *al momento in cui la transazione esegue* — e dentro la classe 0, ordinata per raw transaction ID, l'esito dipenderebbe da `created_at_ms` macinabile.

**Scenario.** Non serve un avversario: serve una seconda implementazione. Due nodi conformi a ogni test, verdetti opposti sulla validità del blocco `20`. **Terza occorrenza della forma di [REVIEW-033] RF-004, stavolta un livello sopra il codice: nella frase.**

RF-003 | category=security | severity=medium | criterion=ADR-010, tre strati | remediation=aggiungere i tre nomi al rapporto di variazione, o dichiarare l'esclusione e perché
**I tre parametri della revoca hanno due dei tre strati di [ADR-010].** Vincoli relazionali sì, magnitudini di genesi sì, **limite al tasso di variazione no**: non sono nell'elenco dei parametri soggetti al rapporto. Resta la sola spaziatura, quindi il cammino non è un cammino — **è un salto in un documento solo**. Alzare `F` in un colpo **autorizza** ad allungare la finestra di obsolescenza del light client, che è la saldatura che [ADR-017] dichiara di aver rotto con il limite di genesi.

RF-004 | category=correctness | severity=medium | criterion=clausola 3 di ADR-017 | remediation=esporre un selettore legato a `consensus_parameters_hash`, oppure dichiarare che in v0 è documentale e rinominare il test
**La clausola 3 non è attuata, e il test che porta il suo nome è vacuo.** `validate_effective_height` riceve i parametri **come argomento**, e **non ha chiamanti fuori dai test** — verificato dal Lead. Il test la chiama due volte con parametri diversi e **la stessa altezza**: dimostra che la funzione legge il proprio argomento, non quale argomento un verificatore debba passare, che è tutto il contenuto della clausola.

Il criterio di accettazione è quindi **soddisfatto alla lettera e vuoto nella sostanza**. La clausola è ben definita — `BlockHeader` porta `consensus_parameters_hash`, quindi «i parametri in vigore a `p`» è un fatto del blocco — ma il buco sta fra la definizione e ciò che esiste.

RF-005 | category=documentation | severity=medium | criterion=passata di ADR-012 | remediation=riscrivere le tre righe distinguendo i due percorsi, e allargare la probe su `SECURITY.md`
**Cercato il settimo artefatto che il Lead ha dichiarato di non aver cercato: ne ha trovati tre, e uno è dentro il perimetro dichiarato dello strumento.**

- **`SECURITY.md`** dice che il rallentamento allunga *«the effective delay of a revocation»*. Dopo la parte 1 sono **due** grandezze: sul percorso di spesa il ritardo in blocchi è zero. **`SECURITY.md` è nel manifesto della passata**, e la probe pinna solo l'apertura della frase e non l'elenco: **lo strumento è verde su una frase che questa consegna ha reso ambigua.**
- **`threat-model.md`**, cella della chiave di identità rubata, attribuisce il ritardo a `effective_height`, che su quel percorso non governa più. Sovrastima il rischio, ed è falsa.
- **`published_artifacts.toml`**, la voce `AUTH-0`, descrive ancora *«the interval in which a revocation is finalized but not yet effective»* — la finestra che la parte 1 ha abolito sul percorso di spesa, **dentro lo strumento della passata**.

RF-006 | category=documentation | severity=low | criterion=ledger.md | remediation=`bites`, e una probe sulla frase corretta
**Una parola italiana in prosa normativa inglese, sulla frase che enuncia la parte 1**: *«revocation **morde** at the height of the block that includes it»*. Unica occorrenza contata su `docs/` + `core/` + `sim/`. `published_artifacts.py` è `PASS`.

RF-007 | category=correctness | severity=low | criterion=authorization.rs | remediation=`min_by_key`, più un test con le due permutazioni
**Il valore `included_height` riportato nell'errore dipende dall'ordine della slice.** Dimostrato eseguendo: ordine `[20,30]` riporta `20`, ordine `[30,20]` riporta `30`, verdetto identico. **Il verdetto è indipendente dall'ordine e non è un fork oggi**, ma il campo è **nuovo di questa consegna**, e se un artefatto di consenso lo serializzasse due verificatori scriverebbero valori diversi.

RF-008 | category=design | severity=low | criterion=ADR-017 parte 2 | remediation=`P > F + G` stretto, o un pavimento sullo scarto, o una frase che lo dichiari
**`P = F + G` è lecito e rende `reason` letto ma privo di effetto.** Misurato: con `F=10, G=5, P=15`, su sessanta altezze **zero** in cui i due `reason` divergono. Nessun vincolo e nessun test distingue quello stato da quello in cui `reason` conta, ed è raggiungibile con un documento solo.

## Cosa la reviewer ha attaccato senza riuscire a romperlo

1. **La parte 1 contro un avversario di inclusione, e la tenuta è in forma chiusa.** Per ogni `p`: morso nuovo `= p`, morso vecchio `= max(p, e)` con `e > p` imposto. Quindi **`morso_nuovo(p) <= morso_vecchio(p)` per ogni `p`**: nessun ritardo, nessun turno di proposta, nessuna scelta di altezza può rendere la parte 1 peggiore della regola che sostituisce. **È monotona nella direzione giusta.**
2. **La derivazione di `AUTH-0`, rifatta dalla regola come terzo oracolo.** Verdetti coincidenti; tre righe divergenti sulle otto, trenta altezze divergenti nell'intervallo. **La tabella oggi è corretta.**
3. **`app-manifest.md`**, che nessuno aveva riletto: dice *«finalized, unrevoked enrollment certificate»* e **non nomina alcuna altezza**, quindi non è un settimo artefatto.
4. **TM-21 del threat model**, che cita la non retroattività: è la metà del percorso del **set**, che `effective_height` ha conservato. Regge.
5. **La regola locale del ricevente**, inclusa la riga sulla rivalutazione delle connessioni aperte: intatta.
6. **[DEBT-035] non è diventato sfruttabile**, e per una ragione che il documento non dice: il codice valuta **a granularità di altezza**, quindi il predicato è **insensibile all'ordine intra-blocco** e macinare `created_at_ms` non sposta alcun verdetto. **È la terza opzione, non le due che [SPEC-022] enumerava** — e non è stata segnalata al Lead come la spec richiedeva.

## Giudizio su `GATE-TWO-ORACLES`

**Soddisfatta per i digest. Non soddisfatta per `AUTH-0`: la seconda derivazione non è mai stata fatta.**

La sezione della trascrizione contiene **soltanto** i due oracoli dei digest. Per `AUTH-0` non c'è traccia di una seconda strada, e la gate esige che la trascrizione **dichiari cosa è stato letto** per costruirla.

**La diagnosi della reviewer.** Una derivazione fatta davvero dalla regola produce le altezze di flip **per esaurimento**: la clausola 2 flippa dove `included_height == h`, cioè a `20`, e nessun campionamento serve. Che la riga `20` mancasse dalla tabella **e** dal test, e che la prosa attribuisse la frontiera alla riga `21` — la sola che non le separa — sono **i tre segni della stessa origine**: la tabella nuova è stata ottenuta **ribaltando le righe della vecchia**, che era costruita attorno a `effective_height = 50` e non aveva ragione di contenere `20`.

**Il ribaltamento riproduce l'insieme di righe della lettura precedente, non quello che la lettura nuova richiede.** La remediation di RF-001 ha aggiunto la riga trovata dal Lead; **non ha rifatto la derivazione**.

## Cosa la reviewer non ha guardato

- **`GATE-CI-GREEN`**: nessuna pipeline eseguita. Locale su un ambiente solo, Windows.
- **Il costo reale della rifirma sotto censura.** La severità `high` di RF-001 poggia sulla **larghezza della finestra, misurata**, non sul costo del giro di quorum, **non misurato** — ed è lo stesso numero mancante che [ADR-017] nomina nelle proprie condizioni di revisione.
- **La passata fuori da `docs/protocol/`, `SECURITY.md`, `sim/tools/`, `.lmbrain/knowledge/`**: `apps/`, `dist/` e i binding FFI non ispezionati.
- **La composizione fra la banda e la regola di contrazione del set**, in particolare se il pavimento di contrazione interagisca con una revoca di massa le cui efficacie cadano in blocchi diversi. **Superficie reale, dichiarata e non raggiunta.**
- **La taratura**: nessun valore proposto per `F`, `G`, `P`. RF-001 e RF-008 dicono che **il pavimento di `G` e lo scarto `P−(F+G)` sono grandezze di sicurezza**, non di comodità.

## Required follow-up

- **RF-001 richiede una correzione a [ADR-017]**, non solo alla spec: il pavimento di `G` va nell'ancora di genesi. È un difetto di progetto del Lead, e va corretto dove il progetto è stato deciso.
- **RF-002 prima di ogni altra cosa sul documento**: è la frase che definisce la regola che tutto il resto attua.
- **RF-003, RF-004, RF-005** nello stesso giro.
- **`GATE-TWO-ORACLES` va rieseguita e trascritta su `AUTH-0`**, oppure la sua evidenza va corretta per dire che copre gli otto digest e non la fixture.
