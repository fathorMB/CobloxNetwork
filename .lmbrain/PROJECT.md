---
title: Project overview
updated: 2026-08-25
---

# Coblox Network

## Vision

Una rete distribuita super-sicura in cui chiunque può offrire i propri dispositivi (telefoni Android, desktop Windows/Linux, server headless) come nodi che forniscono **availability, storage e compute**. In cambio i partecipanti guadagnano un token di scambio interno alla rete, **per progetto mai collegato al valore di alcuna valuta reale o crypto**: il token misura l'uso della rete, non la ricchezza.

Ogni utente riceve un **reddito di esistenza** di base per la sola presenza dimostrata dei propri nodi, più compensi proporzionali quando i nodi vengono effettivamente usati — con la possibilità di verificare in tempo reale chi usa cosa e quanto frutta. Il token si spende per hostare app sulla rete o per abbonarsi ai servizi che le app offrono. In sintesi: un "Ethereum del fare", completamente scollegato dal denaro.

## Users and problems

- **Fornitori di nodi** ("hoster"): persone con hardware inutilizzato (un vecchio PC, un telefono, un server) che vogliono contribuire a una rete indipendente ed essere riconosciuti per questo — senza speculazione finanziaria.
- **Sviluppatori di app**: vogliono pubblicare servizi distribuiti (moduli WASM) senza pagare cloud in denaro, spendendo token guadagnati contribuendo alla rete.
- **Utenti dei servizi**: si abbonano alle app della rete spendendo i token del proprio reddito di esistenza.
- **Problema di fondo**: le reti d'incentivo esistenti (crypto) sono dominate dalla speculazione; le reti volontarie pure (BOINC, ecc.) non hanno un'economia interna che colleghi domanda e offerta. Coblox sta nel mezzo.

## Outcomes and success metrics

- Una devnet con ≥ 3 piattaforme di nodo funzionanti (desktop, Android, headless) e ledger BFT stabile.
- Reddito di esistenza accreditato solo a nodi con presenza crittograficamente dimostrata. **Resistenza Sybil** (riformulata da [ADR-007] dopo [SPEC-004]; la formulazione precedente, "zero accrediti a nodi emulati", è stata dimostrata irraggiungibile): nei test di attacco una flotta di `N ≥ 10.000` identità emulate su un singolo host (a) non aumenta di nulla l'emissione totale dell'epoca, (b) non ottiene alcun accredito in `storage` o `compute`, (c) non ottiene più di una quota `X` dichiarata dell'emissione, (d) non ottiene alcun seggio di validatore. Il valore di `X` è aperto in [DEBT-007].
- Dashboard in tempo reale: latenza percepita tra evento sulla rete e visualizzazione < pochi secondi.
- Almeno un'app dimostrativa end-to-end: pubblicata spendendo token, hostata da nodi terzi, con abbonamenti attivi.
- Economia stabile in simulazione (niente spirali inflattive/deflattive nei modelli agent-based).

## Scope

### Included
- Core del nodo in Rust ([ADR-003]): P2P (libp2p), light client del ledger, storage engine, runtime WASM ([ADR-004]), motore dei challenge ([ADR-002]).
- Ledger a federazione BFT con validatori a rotazione ([ADR-001]).
- Economia mint & burn con simulatore per la taratura ([ADR-005]).
- Shell: Tauri (desktop Win/Linux), Kotlin/Compose (Android), daemon+CLI (headless Win/Linux).
- Design system "hacker ma usabile" condiviso tra le superfici.
- SDK per sviluppatori di app WASM + app dimostrativa.

### Explicitly excluded
- Qualsiasi convertibilità del token in denaro, exchange, o ponte verso crypto (esclusione permanente, di principio).
- Trasferimenti diretti di token utente→utente (non previsti; riaprire solo con ADR dedicato).
- iOS/macOS (valutabili in futuro, non in scope ora).
- Container OCI come runtime app (eventuale tier futuro, [ADR-004]).
- **Mining, cioè lavoro il cui costo è il meccanismo** ([ADR-008]). L'esclusione colpisce il lavoro in cui bruciare risorse *è* l'evidenza: chi partecipa razionalmente scala la spesa senza limite, e il budget di sicurezza della rete finisce per coincidere con la sua dissipazione. Non colpisce il lavoro il cui *prodotto* è il meccanismo — la risorsa sarebbe stata spesa comunque per servire la rete, e la challenge si limita a campionarla. Le challenge di [ADR-002] e l'ancoraggio dell'eleggibilità a validatore a storage e compute di [ADR-007] stanno da questa parte e sono ammessi.

  **Il test.** Una proposta ricade nell'esclusione se soddisfa anche una sola delle prime due condizioni, e non è ammissibile su nodi mobili se non soddisfa la terza. Ogni ADR o spec che introduca una nuova forma di lavoro remunerato deve dichiarare l'esito dei tre punti.

  1. **Limite.** Se un nodo può guadagnare di più spendendo di più *senza che la rete abbia bisogno di più*, è mining. I blocchi da custodire e i task da eseguire hanno un tetto dato dalla domanda; gli hash da calcolare no.
  2. **Spreco.** Se quel lavoro smettesse di essere svolto, un servizio visibile all'utente si degraderebbe? Se no, il lavoro era il fine e non il mezzo.
  3. **Batteria.** Il lavoro è campionabile entro una finestra di risposta ampia, che consenta a un dispositivo mobile di accorpare i risvegli senza poterne anticipare l'istante di emissione (`SEC-REQ-17`)? Un lavoro che pretende presenza continua a bassa latenza non è proponibile su nodi mobili, comunque vadano i punti 1 e 2.

## Constraints

- **Lingua del prodotto: inglese.** Tutto ciò che vede l'utente finale — interfacce (desktop, Android, headless), documentazione pubblica, specifiche di protocollo, SDK — è in inglese. L'italiano resta la lingua di lavoro tra operatore e team di agenti, e degli artefatti interni in `.lmbrain/`. La localizzazione non è in scope, ma le scelte tecniche non devono precluderla.
- Il token non deve mai poter acquisire valore monetario neanche di fatto: ogni scelta di design va vagliata anche sotto questo profilo. Questo include il **naming e la resa tipografica** ([ADR-009]): l'unità si chiama `credit`/`credits`, forma compatta `cr` **posposta** al numero (`1,240 cr`), mai un glifo anteposto. Un'abbreviazione dopo il numero è la grammatica della misura (`50 kg`), un simbolo prima è la grammatica del denaro (`$50`); e un nome generico è molto più difficile da feticizzare di un nome proprio coniabile. Il vincolo spinge quindi verso un nome **poco brandizzabile**, non solo verso uno non-monetario.
- I nodi Android devono rispettare batteria/dati: partecipazione utile senza degradare il dispositivo.
- "Super-sicura" è un requisito di prodotto: memoria safe (Rust), sandbox forte (WASM), prove crittografiche per ogni accredito. **Va dichiarato in modo preciso**: la rete è robusta contro la *falsificazione* — saldi, firme, doppia spesa — ma non è resistente ai Sybil per via crittografica ([ADR-007]). La resistenza Sybil è una proprietà economica, governata dalla frazione `α` di emissione che passa dal reddito di esistenza.
- Team di sviluppo basato su agenti coordinati via LMBrain; il Lead scrive solo in `.lmbrain/`.

## Stakeholders

- **Operatore/fondatore:** Moreno Bruschi (visione, decisioni finali, attivazione agenti).
- **Project Lead:** Ada Checklist (AGENT-LEAD) — specifiche, roadmap, review.
- **Specialisti:** vedi [[agents/registry]].
