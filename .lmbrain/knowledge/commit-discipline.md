---
title: Disciplina di commit del Project Lead
updated: 2026-08-25
---

# Disciplina di commit del Project Lead

Pagina nata da un errore reale del Lead, non da una buona intenzione. Vale per chiunque ricopra il ruolo.

## Il fatto

Il 2026-08-25, durante la sessione autonoma notturna, il Lead ha eseguito due commit di chiusura mentre **AGENT-001 stava attivamente modificando `docs/protocol/`** per la remediation del Lotto B di [SPEC-001].

- `024f81f` ("SPEC-004 done") è stato creato con `git add .lmbrain docs`.
- `81cca93` ("SPEC-002 done") è stato creato con `git add -A`.

Entrambi hanno inglobato lavoro in corso non ancora consegnato: 93 righe di modifiche al protocollo nel primo, 473 nel secondo. Il contenuto è integro e verificato, ma **la storia git attribuisce la remediation del Lotto B a due commit che portano il titolo di altre spec**. L'implementatore ha rispettato i propri confini — non ha fatto né commit né push — e ha segnalato il problema nel rapporto di consegna. L'errore è interamente del Lead.

## Perché è successo

La strategia di branching dichiarata (`main-only`, commit del Lead al passaggio di una spec a `done`) presuppone implicitamente che l'albero contenga **solo** il lavoro di quella spec. Quel presupposto è falso quando più agenti lavorano in parallelo sullo stesso working tree, che è la modalità normale di questo progetto.

## Regola

Prima di ogni commit, il Lead:

1. **Verifica se ci sono agenti in lavorazione.** Se sì, non usa mai `git add -A` né percorsi larghi come `docs` o `.lmbrain`.
2. **Stage esplicito e ristretto** ai soli file che appartengono alla spec che si sta chiudendo. In caso di dubbio su un file, lo si lascia fuori: un file dimenticato si aggiunge al commit successivo, un file inglobato per errore resta nella storia.
3. **Rilegge `git diff --cached --stat` prima di committare** e si chiede, per ogni percorso, a quale spec appartenga. Se non sa rispondere, lo toglie.
4. **Preferisce aspettare.** Se un agente sta per consegnare, il commit può attendere qualche minuto. La fretta di committare non ha alcun valore; una storia sbagliata sì.

## Cosa non si fa per rimediare

Non si riscrive storia già spinta. Un `git push --force` cancellerebbe lavoro dal remote e non è mai una decisione del Lead: richiede l'operatore. La correzione corretta è **documentare l'attribuzione reale**, come fatto qui e nel messaggio del commit successivo.

## Attribuzione reale dei commit interessati

| Commit | Titolo | Contiene anche |
| --- | --- | --- |
| `024f81f` | SPEC-004 done | Prima parte della remediation del Lotto B di [SPEC-001] (README, identity, ledger, wire, app-manifest) |
| `81cca93` | SPEC-002 done | Parte principale della remediation del Lotto B di [SPEC-001] (473 righe sui cinque documenti di protocollo) |

Chi cerchi in futuro l'origine di una riga del protocollo deve sapere che `git log` su quei due commit è fuorviante, e che la fonte autorevole è [SPEC-001] con le sue review.
