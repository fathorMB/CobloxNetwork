---
id: REVIEW-026
# Note: Quote the title if it contains a colon
title: "Review of SPEC-018"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-018
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-007
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-026-EVENT-001"
    timestamp: "2026-08-26T01:41:40.487149800+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Accettata dopo un giro di remediation su due finding, entrambi risolti. Review del Lead e non di AGENT-007 perche' la spec era per costruzione un'autovalutazione.\n\nRiverificato dal Lead enumerando la tabella: 104 celle, 97 coperte, 7 n/a, contro 91, 60, 31 di prima. Riga A-09 a zero n/a. TM-43 ultimo scenario. threat_model_matrix_coherence.py verde.\n\nGATE-ALL-NA-RESUBMITTED ha pagato il proprio costo: imponeva trentuno celle invece delle venticinque selezionate, e due delle sei escluse sono cadute, fra cui proprio quella su cui la valutazione dichiarava di non pronunciarsi.\n\nRF-001 risolta facendo cadere la cella e non emendando §2, che e' il verso giusto per il merito prima che per la forma: tolto il qualificatore, la frase con cui la cella si difendeva diventa l'attacco, perche' la macchina di un T-01 che allenta il contenitore custodisce chunk di terzi, tiene la chiave di identita' del nodo ed e' un peer autenticato, e produce un T-08 senza che nessuno abbia pagato il costo di T-08. La cascata su A-09 x T-02 e A-09 x T-06 non era richiesta ed e' la parte migliore: l'argomento con cui aveva confermato A-09 x T-02 conteneva la propria confutazione, perche' se possedere l'host basta a rinunciare al confine, possederne molti e' l'attacco.\n\nRF-002 risolta dentro R-NA.1 senza toccare §2, piu' una seconda riga non richiesta che chiude il caso simmetrico di R-NA.3 sull'asset.\n\nEsito sul modello: A-09 non ha difese indipendenti dall'host, e sei celle dicevano il contrario.\n\nObbligo di sequenza registrato: SECURITY.md dichiara un conteggio ricalcolato dalla fonte da C11-CLAIMDOC, e la gate di ADR-012 resta rossa finche' quel numero non passa a 43. Lavoro del Lead perche' SECURITY.md era fuori scopo qui."
    evidence_refs: ["SPEC-018", "DEBT-018", "DEBT-024", "DEBT-025"]
    implementation_agent: "AGENT-007"
links: []
created: 2026-08-26
updated: 2026-08-26
tags: [review]
activity:
  - date: 2026-08-26
    action: "created"
  - date: 2026-08-26
    action: "transitioned pending -> accepted"
---
# Review

## Outcome

**Accettata dopo un giro di remediation su due finding.** La review è del Lead e non di AGENT-007 perché la spec era per costruzione un'**autovalutazione**: la matrice è sua, il difetto l'aveva trovato lei, e la passata rivedeva il suo stesso documento.

Il risultato che vale più del conteggio è una riga: **`A-09` non ha più alcuna `n/a`.** L'isolamento della sandbox non ha difese indipendenti dall'host, e sei celle dicevano il contrario.

## Acceptance-criteria compliance

Riverificato dal Lead enumerando la tabella, non letto dall'evidenza: **104 celle (13 × 8), 97 coperte, 7 `n/a`.** Prima: 91, 60, 31. La riga `A-09` ha otto celle e zero `n/a`. `TM-43` è l'ultimo scenario. `sim/tools/threat_model_matrix_coherence.py` è verde.

**La regola `R-NA` precede le modifiche** ed è in §4, come `GATE-METHOD-BEFORE-PASS` imponeva. Cinque condizioni, e quella che ha fatto più lavoro è la (1b) — *può causare una perdita su quell'asset?* — cioè la domanda che semplicemente **mancava**.

**`GATE-ALL-NA-RESUBMITTED` ha pagato il proprio costo.** Imponeva trentuno celle invece delle venticinque che la valutazione selezionava, e **due delle sei escluse sono cadute** — fra cui `A-08`×`T-06`, esattamente quella su cui la valutazione dichiarava di non pronunciarsi. La selezione era il giudizio da dimostrare, e non reggeva.

**`A-09` × `T-07` è stata chiusa falsificando la cella**, non restringendo `T-07`, e la terza ragione addotta chiude la questione senza dipendere dalla scelta del verso: la cella cade **anche** sotto la lettura ristretta, perché la politica di accettazione dell'host e i tetti di deployment *sono parametri firmati*.

## Review findings

**RF-001 — medium — `A-09` × `T-01` citava a §2 un qualificatore che §2 non contiene.** *Risolta.*

La cella scriveva *«la perdita che §2 assegna ad `A-09` è la compromissione della macchina di un partecipante **che non ha scelto il codice**»*. §2 dice: *«Compromissione della macchina di un partecipante»*. Il qualificatore era introdotto nella cella e attribuito a §2 — la stessa forma del difetto che questa spec esiste per chiudere, commessa dentro il rimedio e da chi aveva scritto la regola.

L'implementatrice ha scelto **di far cadere la cella**, che è il verso giusto, e per il merito prima ancora che per la forma: tolto il qualificatore, la frase con cui la cella si difendeva — *chi disattiva il proprio confine ha scelto* — **diventa l'attacco**. La macchina di un `T-01` che allenta il contenitore custodisce chunk di terzi, tiene la chiave di identità del nodo ed è un peer autenticato. La compromissione non è confinata a chi ha scelto, e **produce un `T-08` senza che nessuno abbia pagato il costo di `T-08`**. La conformità al contenitore è verificabile sugli output e non sul modo dell'esecuzione, quindi nulla la rileva.

**La cascata non era richiesta ed è la parte migliore.** `R-NA.4` — la monotonia — impone la stessa conclusione a `A-09`×`T-02` e `A-09`×`T-06`, e l'implementatrice l'ha applicata **contro se stessa**. L'argomento con cui aveva confermato `A-09`×`T-02` — *moltiplicare identità moltiplica gli host che l'attaccante possiede* — **conteneva la propria confutazione**: se possedere l'host basta a rinunciare al confine, possederne molti è l'attacco. Il difetto era già scritto nella cella, che è la forma sotto cui questo progetto lo trova ogni volta.

**RF-002 — low — la colonna *Perdita significa* non è uniforme.** *Risolta.*

Per `A-01` porta un elenco di forme, per `A-04` un caso peggiore singolo: `R-NA.1(b)` era quindi più permissiva verso l'`n/a` sugli asset scritti come caso peggiore, senza che nessuno l'avesse deciso. Chiusa dentro `R-NA.1` senza toccare §2: quando la colonna porta un caso peggiore, la domanda si risolve contro la **classe** di perdita che quel caso esemplifica, e la cella deve dichiarare quale classe sta negando. Le due `n/a` superstiti di `A-04` la dichiarano.

**Una seconda riga non richiesta, e necessaria:** il caso simmetrico di `R-NA.3` sull'**asset**. Una cella non può citare §2 aggiungendo né togliendo un qualificatore. RF-001 è la prova che serviva, e senza quella riga la regola restava aperta proprio dove era stata aggirata.

## Tests and verification

Le cinque gate sono soddisfatte. `GATE-NO-SELF-CONFIRMATION` porta una tabella cella per cella con argomento originale e argomento nuovo, e la sua utilità è dimostrata al contrario: le conferme sopravvissute sono scese da dieci a sette proprio perché gli argomenti nuovi non reggevano.

Lo strumento di coerenza è stato **provato in negativo** invece che trascritto, reintroducendo due occorrenze reali e verificando che la guardia le trovi entrambe distinguendo le cause.

## Required follow-up

Due debiti aperti dal Lead su segnalazione dell'implementatrice, che ha rispettato il perimetro invece di correggere fuori scopo: **[DEBT-024]** su `ComputeAssignment`, e **[DEBT-025]** sulla coerenza matrice-scenari, di cui `threat_model_matrix_coherence.py` è il punto di partenza e non la chiusura — non è cablato in CI e gli otto disallineamenti vanno sanati prima, altrimenti nascerebbe rosso e verrebbe disattivato.

**Obbligo di sequenza:** `SECURITY.md` dichiara un conteggio di scenari ricalcolato dalla fonte da `C11-CLAIMDOC`. Questa spec ha portato gli scenari a `TM-43`, quindi la gate di [ADR-012] resta rossa finché quel numero non è aggiornato. È lavoro del Lead perché `SECURITY.md` era fuori scopo qui.

## Final decision

**Accettata.** Nessun finding residuo.

Va registrato che la spec conteneva un rischio scritto a metà, e l'implementatrice l'ha nominato: *«se la passata non falsifica nessuna cella, sospetta la regola»* copre una direzione sola, e ventiquattro falsificazioni su trentuno vanno guardate dall'altro lato. Guardate: le cause si raggruppano in quattro e non in ventiquattro giudizi indipendenti, e la dominante è la domanda che mancava. Non è un finding, ma il gemello del rischio va nella prossima spec di questa forma.

E una cosa che il Lead si aspettava diversa: **nessuna cella `n/a` poggiava sull'assunzione dell'accelerazione benigna**, verificato enumerando. La griglia ha tenuto; è la **prosa** di `TM-38` che non ha tenuto. È la famiglia 2 alla sesta occorrenza, e conferma che la matrice è più robusta del testo che la circonda — il che è un argomento per meccanizzare la coerenza, cioè per [DEBT-025].
