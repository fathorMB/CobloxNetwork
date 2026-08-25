---
id: REVIEW-024
# Note: Quote the title if it contains a colon
title: "Review of SPEC-015"
status: accepted
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
spec: SPEC-015
reviewer: AGENT-LEAD
review_requested_by: user
implementation_agent: AGENT-006
# Taxonomy v1 canonical values are listed in CONTRACT.md. New reviews use canonical values only.
finding_taxonomy_version: 1
finding_categories: []
review_events:
  - schema_version: "1"
    id: "REVIEW-024-EVENT-001"
    timestamp: "2026-08-26T00:26:21.418965500+02:00"
    action: "verdict"
    from_status: "pending"
    to_status: "accepted"
    actor_role: "operator"
    reason: "Nessun finding a carico dell'implementazione. Verificato dal Lead rieseguendo: published_artifacts.py e la sua prova in negativo OK, check-guide-pairs.mjs PASS su sei controlli, aree in sola lettura intatte, aggiunta al manifesto puramente additiva con 533 aggiunte e zero rimozioni. Test del filo chiuso rifatto in modo indipendente spogliando il DOM di ogni corpo apribile: le tre cose scomode sono presenti.\n\nL'implementatrice ha chiuso una direzione che il Lead non aveva chiesto e la cui assenza avrebbe reso l'ancoraggio inutile a meta: ogni probe porta il testo della frase che tiene, e il suo strumento fallisce se quella frase non e piu sulla pagina, altrimenti una probe sopravvive a una riscrittura e diventa un commento. E ha contestato con argomento una lettura letterale della gate, tenendo l'ordine delle sette domande ma non la collocazione delle tre cose scomode, perche in sezione sei sarebbero arrivate dopo quattro sezioni di persuasione.\n\nIl risultato che vale piu della pagina e l'elenco delle undici affermazioni tolte perche nessuna regola le teneva, fra cui la non convertibilita dei credits, che era una frase scritta dal Lead nella spec.\n\nRegistrato in review un terzo errore del Lead dello stesso tipo in un giorno: il suo primo test del filo chiuso ha dichiarato assente il fatto sui Sybil cercando la parola Sybil, che e il gergo che il lettore di dieci anni non conosce, quindi la sua assenza e corretta e voluta. Il test misurava la parola e non la proprieta.\n\nGATE-OPERATOR-LOOK e attestata dall'operatore. GATE-SECREVIEW resta da attestare su una review di AGENT-007, perche una posizione di sicurezza detta in parole semplici e la forma in cui e piu facile prometterla piu forte di com'e."
    evidence_refs: ["SPEC-015", "ADR-012", "ADR-014"]
    implementation_agent: "AGENT-006"
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

**Accettata senza finding a carico dell'implementazione.** `GATE-OPERATOR-LOOK` è attestata dall'operatore; `GATE-SECREVIEW` resta da attestare.

Il risultato che conta non è la pagina: sono **cinque questioni che scrivere la pagina ha fatto emergere**, di cui una è un difetto del protocollo e una è una divergenza su un documento pubblico.

## Acceptance-criteria compliance

Il pacchetto è autoconsistente, in inglese, e carica token e CSS del design system **per percorso relativo senza copiarne né alterarne un byte** — verificato: `docs/protocol/`, `.lmbrain/decisions/` e `coblox-design-system/` non hanno una sola modifica.

L'aggiunta a `published_artifacts.toml` è **puramente additiva** come il dispatch imponeva: `533 aggiunte, 0 rimozioni`, nessun riordino. Le probe passano da 19 a **84**.

## Code observations

**Ha chiuso una direzione che il Lead non aveva chiesto, e la sua assenza avrebbe reso l'ancoraggio inutile a metà.** La spec imponeva che ogni affermazione avesse una probe verso la regola che la tiene. Manca il verso opposto: **una probe sopravvive a una riscrittura della pagina e diventa un commento.** Ogni probe porta ora il testo della frase che tiene, e `check-guide-pairs.mjs` fallisce se quella frase non è più sulla pagina.

**La disposizione delle tre cose scomode è una contestazione motivata di una lettura letterale della gate**, ed è corretta. Metterle in §06 avrebbe soddisfatto il criterio e *«tradita, perché arriverebbero dopo quattro sezioni di persuasione»*. Stanno in §01, §03 e §04; §06 le raccoglie **e dice che le sta raccogliendo**.

**Il `1,240` del Lead è diventato un controllo meccanico** che fallisce su quella stringa esatta, provato in negativo con essa. La segnalazione nell'analisi dell'esistente era un avvertimento; è diventata una guardia.

## Tests and verification

Rieseguito dal Lead: `published_artifacts.py` **OK**, la sua prova in negativo **OK** su dieci classi, `check-guide-pairs.mjs` **PASS** su sei controlli e 137 candidati.

**Il test del filo chiuso rifatto in modo indipendente dal Lead**, spogliando il DOM di ogni `.detail__body`: 11 571 caratteri residui, e le tre cose scomode presenti.

**Un errore del Lead nel proprio strumento di verifica, registrato perché è la terza occorrenza dello stesso tipo in un giorno.** Il primo test del Lead ha dichiarato **assente** il fatto sui Sybil. Era presente: il Lead cercava la parola *Sybil*, che è il gergo che un lettore di dieci anni non conosce, quindi la sua assenza dal filo è **corretta e voluta**. Il fatto è detto in §01 così:

> *«It cannot tell a thousand devices apart from one computer pretending to be a thousand devices. That is a deliberate and permanent property of this version, not a gap waiting for a patch.»*

**Il test misurava la parola, non la proprietà** — la stessa forma censita in `recurring-defects.md` un'ora prima, e applicata di nuovo a uno strumento di verifica del Lead.

## Review findings

<!-- Stable form: RF-001 | category=... | severity=... | criterion=... | remediation=... -->

**Nessun finding a carico dell'implementazione.**

Le cinque questioni che l'implementatrice ha riportato invece di aggirare scrivendo bene sono registrate in *Required follow-up*: **è il comportamento che la spec chiedeva**, ed è ciò che ha prodotto il risultato migliore della passata.

## Required follow-up

**F4, verificato dal Lead e promosso a debito.** `ledger.md:347`, autorizzazione del burn di abbonamento: *«the key MUST derive `payer_node_id`»*, e basta. Le regole sorelle dicono tutte *«the enrolled, **unrevoked**»* — righe 312, 398, 871. **Il Lead ha verificato che la 312 non sia una regola generale che copre anche questa**: appartiene a `FundAppAuthorization`, un'altra transazione con un'altra struttura di autorizzazione. Una chiave revocata per compromissione può quindi autorizzare addebiti sul saldo. Trovato provando a scrivere *«cosa succede quando una chiave viene revocata»*, e di conseguenza **la guida non afferma che la revoca fermi la spesa**.

**F2, correzione del Lead.** `SECURITY.md` dichiara ancora che il set di validatori è auto-perpetuante e la regola di elezione non scritta, con [DEBT-005] aperto. È chiuso da [SPEC-006]. Due copie pubbliche divergenti — famiglia 2 — **sulla pagina che un ricercatore di sicurezza legge per prima**. È artefatto di radice del repository e la sua manutenzione è del Lead, non dell'implementatrice.

**F3 e F1, da valutare.** La non convertibilità dei credits è tenuta solo da `PROJECT.md` e [ADR-005], **fuori dal perimetro di [ADR-012]**: l'affermazione più centrale del prodotto non ha una probe disponibile, ed è la ragione per cui è nell'elenco delle undici tolte. E il testo pubblico di [ADR-014] non esiste: la guida **dice che è dovuto e non pubblicato** invece di rimandare al nulla.

**F5.** Le guardie di design non sono in CI, mentre `published_artifacts.py` sì — quindi il verso che conta per l'ancoraggio è coperto e quello sulla forma no. La CI non era delegata a questa spec.

## Final decision

**Accettata.** `GATE-OPERATOR-LOOK` attestata dall'operatore il 2026-08-26; la spec resta in `review` fino a `GATE-SECREVIEW`.
