---
id: REVIEW-032
# Note: Quote the title if it contains a colon
title: "Review of SPEC-019"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-019
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-002
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-032-EVENT-001"
    timestamp: "2026-08-26T13:26:33.205663300+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata senza finding a carico dell'implementazione. GATE-SECREVIEW resta da attestare ed e' di AGENT-007, che e' anche la persona di cui questa consegna contraddice l'esito raccomandato: e' l'unica sede giusta per decidere.\n\nRiverificato dal Lead rieseguendo: 176 test da 167, 142 probe C10 da 137 ciascuna osservata fallire da sola, protocol_hashes senza valori mossi.\n\nLa contraddizione e' il contenuto della consegna. L'esito (A) di DEBT-022 raccomandava la lettura \"finalizzata\" con un argomento di margine di sicurezza; la spec chiedeva di motivare sulla proprieta' del verificatore che rigioca, e applicata la proprieta' la scelta si rovescia. Verificato dal Lead enumerando i dodici campi di BlockHeader: nessuno riguarda la finalita'. Un verificatore che rigioca puo' stabilire che una revoca era inclusa sotto h, non che fosse finale sotto h - quello lo puo' solo dai certificati che possiede fuori dalla catena, cioe' dallo stato esterno che la spec vieta. Due verificatori con certificati diversi darebbero verdetto opposto sullo stesso blocco: la lettura \"finalizzata\" non e' una regola stretta con margine largo, e' un fork con una specifica. Adottarla avrebbe sostituito una finestra dichiarata con un fork, reintroducendo l'esito peggiore che ha giustificato la severita' high.\n\nIl Lead ha attaccato tre cose e nessuna si e' rotta. Che la finalita' fosse ricostruibile dalla catena, che e' il fondamento della scelta: dodici campi, nessuno la riguarda. Che effective_height potesse essere retroattivo, che e' il modo in cui questa regola si romperebbe: ledger.md:963 impone che stia almeno min_revocation_effective_delay_blocks sopra il blocco che propone la revoca, strettamente in avanti. E che l'elenco delle regole di autorizzazione fosse compilato a memoria: e' derivato dal registro dei domini di firma, che la gate rideriva nei due versi.\n\nGATE-DIVERGENT-CASE esercita il caso giusto: le righe 21 e 49 sono le uniche divergenti, e la mutazione che reintroduce la lettura sbagliata fa fallire quelle due e solo quelle, mentre le tre concordi restano verdi - che e' la dimostrazione di cosa la fixture misura.\n\nTre difetti sono della spec e quindi del Lead. La condizione su identity.md era binaria contraddizione-o-niente e non copriva il caso che ha prodotto il difetto, cioe' una seconda lettura non contraddittoria che convive. Spec e debito si contraddicono sui numeri di riga fra loro, non solo con l'albero: chi li rincorre trova tre risposte. E il piano prevedeva un ricalcolo di hash che non serviva."
    evidence_refs: ["SPEC-019", "DEBT-022"]
    implementation_agent: "AGENT-002"
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

**Accettata senza finding a carico dell'implementazione.** `GATE-SECREVIEW` resta da attestare, ed è di AGENT-007 — che è anche la persona di cui questa consegna **contraddice l'esito raccomandato**, il che rende la sua review la sede giusta per decidere. **Correzione del 2026-08-26:** questa frase diceva «l'unica sede giusta», e non e' vero — il Lead poteva decidere da se' o portarlo all'operatore. E' la sede **migliore**, perche' e' l'unica persona che possa dire se alla contraddizione sfugga qualcosa che la sua valutazione vedeva; ma «unica» era enfasi scritta come fatto, ed e' stata trovata da `lead_claims_check.py`.

## La contraddizione, che è il contenuto di questa consegna

[DEBT-022] è di AGENT-007, e il suo esito (A) raccomandava l'allineamento **con la lettura «finalizzata»**, motivato così: *per l'autorizzazione di una spesa il pericolo sta verso l'alto sulla durata dell'esposizione, e la scelta sicura è quella che chiude prima.*

L'implementatrice ha scelto l'altra, e l'argomento regge alla verifica.

**La finalità non è ricostruibile dalla catena.** `BlockHeader` ha **dodici campi** — verificato dal Lead enumerandoli — e **nessuno** riguarda la finalità: non c'è alcun campo che registri quando un blocco anteriore sia diventato finale. Un verificatore che rigioca può stabilire che un `revoke_identity` era **incluso** sotto `h`; che fosse **finale** sotto `h` lo può stabilire solo dai certificati che possiede fuori dalla catena — cioè dallo stato esterno che la spec vieta.

Ne discende che due verificatori con certificati diversi darebbero verdetto **opposto sullo stesso blocco**. La lettura «finalizzata» non è una regola stretta con un margine largo: **è un fork con una specifica.** Adottarla per chiudere la finestra avrebbe sostituito una finestra **dichiarata** con un fork — cioè avrebbe reintrodotto esattamente l'esito peggiore che ha giustificato la severità `high` di quel debito.

L'argomento di AGENT-007 era di **margine di sicurezza**; la spec chiedeva di motivare sulla **proprietà**. Applicata la proprietà, la scelta si rovescia. È la settima volta su questo progetto che chi implementa ha ragione contro chi ha scritto l'istruzione, e la prima in cui la contraddizione è fra due specialisti invece che contro il Lead.

## Acceptance-criteria compliance

Riverificato dal Lead rieseguendo: **176 test** da 167. `published_artifacts.py` `PASS` con **142 candidati C10** da 137. `published_artifacts_negative.py` `PASS`, ogni probe osservata fallire da sola. `protocol_hashes.py` `PASS`, nessun valore pubblicato cambiato — coerente, perché `AUTH-0` è una fixture **comportamentale** e la spec non introduce preimmagini.

**`GATE-DEFINITION-FIRST` è soddisfatta nella sostanza e non solo nell'ordine:** la definizione esiste come sezione propria di `ledger.md`, con l'altezza di valutazione nel titolo, e l'allineamento della riga sull'abbonamento ne discende invece di precederla.

**`GATE-DIVERGENT-CASE` esercita il caso giusto.** Le righe 21 e 49 di `AUTH-0` sono le **uniche** su cui le due letture divergono, e la mutazione che reintroduce la lettura «finalizzata» fa fallire **quelle due e solo quelle**: le tre righe concordi restano verdi sotto la lettura sbagliata, che è la dimostrazione di cosa la fixture misura. Una fixture sul caso concorde sarebbe stata verde oggi e verde anche col difetto aperto.

**`GATE-NO-PARAMETER-MOVED`**: `min_revocation_effective_delay_blocks` compare nel diff **una sola volta, in prosa**, nel paragrafo che dichiara il costo. `params.rs` non è fra i file modificati.

## Cosa ho attaccato senza riuscire a romperlo

**Che la finalità fosse ricostruibile dalla catena**, che è il fondamento dell'intera scelta. Se un solo campo dell'intestazione registrasse la finalità di un antenato, l'argomento cadrebbe e con esso la contraddizione ad AGENT-007. Enumerati i dodici campi di `BlockHeader`: nessuno la riguarda, e una ricerca di `final` nella regione dà zero. **Non si è rotto.**

**Che `effective_height` potesse essere retroattivo**, che è il modo in cui questa regola si romperebbe se dovesse rompersi. Una revoca con `effective_height` nel passato invaliderebbe transazioni già accettate, e la monotonia del predicato — la proprietà su cui la scelta poggia — sarebbe falsa. `ledger.md:963` impone che `effective_height` stia **almeno `min_revocation_effective_delay_blocks` sopra** il blocco che propone la revoca. Strettamente in avanti. **Non si è rotto.**

**Che l'elenco delle regole di autorizzazione fosse compilato a memoria.** Non lo è: è derivato dal **registro dei domini di firma** del manifesto, che `published_artifacts.py` rideriva dai documenti e fa fallire nei due versi. Tredici domini, tredici superfici. **Non si è rotto.**

## Review findings

Nessuno a carico dell'implementazione.

**Due residui riportati e non corretti**, entrambi correttamente fuori scopo.

**R1** — `app-manifest.md:64-65` porta la qualificazione e **non porta l'altezza**: è la stessa sotto-specificazione di [DEBT-022] in forma minore. La definizione nuova dichiara di governarla e nomina l'altezza, ma quel documento non rimanda indietro. Debito del Lead.

**R2** — `identity.md:614` **è** la seconda lettura e resta. Non è una contraddizione: è una regola di accettazione **locale del ricevente** su una connessione, nessuno la rigioca, ed è legittimamente più stretta e ancorata alla propria vista. La sezione nuova la nomina in un paragrafo *«One rule this definition does not govern»*, pinnato da una probe.

## Tre difetti della spec, quindi del Lead

**La condizione su `identity.md` era troppo stretta.** Autorizzava a toccarlo «solo se la definizione scelta contraddice ciò che lì è scritto». **Non lo contraddice — e proprio per questo il problema esisteva**: è una seconda lettura *non contraddittoria* che convive, ed è così che [DEBT-022] è nato. Un criterio binario contraddizione-o-niente non copre il caso che ha prodotto il difetto.

**Spec e debito si contraddicono sui numeri di riga**, e non solo con l'albero: la spec cita 372/407/458/968, il debito 312/347/398/871, l'albero oggi ha 507/542/593/1145. Chiunque li rincorra trova **tre risposte**. Il testo della clausola regge dove i numeri di riga non reggono, e questa e' la forma esatta: non e' il solo riferimento stabile — anche gli ancoraggi di sezione lo sono — ma e' quello che sopravvive a una riscrittura che sposta le righe senza cambiare la regola.

**Il piano prevedeva un ricalcolo di hash che non serviva.** Nessun valore pubblicato cambia. L'implementatrice ha eseguito la passata lo stesso e scritto la frase che dice che nulla è cambiato e perché, come [SKILL-002] impone — la clausola che ho aggiunto a quella skill stamattina su segnalazione sua.

## Final decision

**Accettata.** `GATE-SECREVIEW` resta da attestare.

Va registrata un'osservazione dell'implementatrice sulla gate che ho scritto: **`GATE-ALL-AUTHORIZATION-RULES` può solo produrre segnalazioni fuori da `ledger.md`**, perché chiede la passata su tutto il protocollo mentre i file ammessi sono tre. È coerente con [SKILL-003] e va detto lo stesso: quella gate è **per costruzione un generatore di debiti, non di correzioni**. Non è un difetto — è ciò che la separazione fra perimetro e passata produce, e conviene saperlo scrivendo la prossima.
