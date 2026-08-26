---
title: Disciplina delle affermazioni del Project Lead
updated: 2026-08-26
---

# Disciplina delle affermazioni del Project Lead

Pagina nata da cinque errori reali in una notte, non da una buona intenzione. Vale per chiunque ricopra il ruolo.

## Il fatto

Nella notte fra il 25 e il 26 agosto 2026 il progetto ha registrato cinque difetti in artefatti scritti dal Lead:

1. Il superlativo di [DEBT-014] — *«l'unica preimmagine a dominio separato non legata a `chain_id`»* — **falso**, ereditato dall'inventario di [SPEC-010] e mai contato. Sei altre lo omettono.
2. Il «punto 3» della proposta tecnica di [SPEC-016]: un limite di mandato in millisecondi di catena, cioè la famiglia 3 **scritta dentro la spec che esisteva per non commetterla**. Rifiutato con argomento dall'implementatrice.
3. [REVIEW-025] che lodava l'asimmetria fuori banda come *«la parte migliore del lavoro»* senza attaccarla. [REVIEW-027] vi ha trovato un finding **high**.
4. `GATE-MEASURE-BINDS` accettata con tre casi di prova **tutti a latenza zero**.
5. Una banda di cadenza proposta all'operatore senza aver letto il paragrafo di `README.md` che ne governava un lato — il documento avverte esplicitamente contro il valore proposto.

E, appena lo strumento di questa pagina è esistito, altri due nella stessa notte: il superlativo su `height` in [REVIEW-025] (**falso**, verificato enumerando i dodici campi di `BlockHeader`) e due volte *«l'unica cosa che aspetta l'operatore»* in [HANDOFF-002], falsa mentre la scrivevo.

## Perché è successo

**Per tutto ciò che scrive il Lead, autore e revisore sono la stessa persona.**

Gli agenti rivedono le implementazioni; il Lead rivede le loro. Le spec, le review, le ADR, i debiti e gli handoff del Lead **non li rivede nessuno**. Ne segue che ciò che il Lead scrive è, per costruzione, la superficie non attaccata — ed è la ragione per cui il finding `high` era esattamente dove la review diceva «la parte migliore».

C'è un secondo strato, ed è quello che rende il primo pericoloso invece che soltanto sfortunato: **ogni regola che il Lead impone agli agenti è una regola che il Lead non applicava a sé.** Prova in negativo obbligatoria, ma la prosa di review non ne aveva nessuna. L'affermazione deve citare la regola che la tiene, ma il superlativo non citava niente. L'eccezione va dimostrata per esaurimento, ma era ereditata. Il modo di sbagliare va nominato, ma mai per ciò che scriveva il Lead.

**Aggiungere un revisore non lo risolve**, sposta il confine di un passo: chi rivede il revisore? La soluzione è applicare al Lead le sue stesse regole, e su questo progetto una regola che nessuno esercita è la famiglia 4 del censimento.

## Regola

**1. Una review che loda senza attaccare non ha verificato.** Ogni review firmata dal Lead deve contenere una sezione che dica **cosa ha attaccato senza riuscire a romperlo**: un tentativo concreto e il suo esito, non un elenco di comandi rieseguiti. *Rieseguire un comando non è verificare un argomento.*

**Corollario, ed è la parte contro-intuitiva:** la parte che sembra migliore va attaccata **per prima**. Ciò che il Lead loda è precisamente ciò che smette di verificare.

**2. Un superlativo è un'affermazione universale.** *«L'unica»*, *«il solo»*, *«nessun altro»*: o è enumerato in quella sessione con la traccia scritta accanto, o va riformulato come congettura. Un superlativo ereditato da un altro artefatto **non è enumerato**: è ereditato, e [DEBT-014] lo ha portato avanti per tre stesure.

**3. Se tutti i casi di una gate condividono un valore su una grandezza che non è quella sotto test, quella grandezza è la prossima da variare.** È la forma generale del difetto di `GATE-MEASURE-BINDS`.

**4. Non proporre un valore prima di aver letto l'istruzione che lo governa.** Rispondere in fretta a una domanda dell'operatore non è un valore; una domanda ben posta un minuto dopo lo è.

**5. Non scrivere una proposta tecnica in un dominio che non si implementa senza citare la fonte letta.** Il «punto 3» esisteva perché il Lead aveva inventato un meccanismo.

## Applicazione

Le regole 1 e 2 sono **eseguibili**: `sim/tools/lead_claims_check.py`, con `--prove-negative` per la prova su sei casi, tre per classe, ciascuno osservato nel verso dichiarato.

Vincola **in avanti**, dal 2026-08-26: gli artefatti anteriori non violano una regola che non esisteva. Il loro arretrato non è però silenzioso — **36 superlativi non enumerati** vengono contati e stampati a ogni esecuzione, e sono tracciati in [DEBT-027]. Tre review portano una deroga dichiarata con la sua ragione; quella di [REVIEW-025] dice perché non va riscritta: **riscriverla per far passare la guardia cancellerebbe la prova.**

Le regole 3, 4 e 5 non sono meccanizzabili e restano giudizio. Sono scritte qui perché la 3 in particolare è la domanda che nessuno pone da sé.

## Cosa questa pagina non pretende

Non pretende che il Lead smetta di sbagliare. Pretende che gli errori di questa forma **falliscano rumorosamente** invece di essere accettati da chi li ha scritti — che è tutto ciò che una guardia può fare, ed è esattamente ciò che `C11-CLAIMDOC` ha fatto per il conteggio di `SECURITY.md` la stessa notte, entro pochi minuti dall'esistere.

Vedi anche [[commit-discipline]], [[review-lifecycle-discipline]] e `recurring-defects.md`.
