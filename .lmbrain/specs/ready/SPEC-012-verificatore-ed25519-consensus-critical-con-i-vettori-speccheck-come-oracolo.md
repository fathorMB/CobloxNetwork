---
id: SPEC-012
# Note: Quote the title if it contains a colon
title: "Verificatore Ed25519 consensus-critical con i vettori speccheck come oracolo"
status: ready
kind: feature
priority: high
area: core
milestone: M-02
# References use IDs only (e.g. [SPEC-001]); use [[wikilinks]] in prose
recommended_agent: AGENT-001
capability_tier: sol
thinking_level: extended
effort_observations: []
depends_on: []
dependency_events: []
parking_events: []
skills: []
verification_gates: []
related_decisions: [ADR-003]
links: []
created: 2026-08-25
updated: 2026-08-25
tags: [conformance, rust, security]
activity:
  - date: 2026-08-25
    action: "transitioned backlog -> ready"
---
# Verificatore Ed25519 consensus-critical con i vettori speccheck come oracolo

## Objective

Dare a Coblox il verificatore di firme che oggi non ha, con la tabella dei vettori `ed25519-speccheck` come proprio oracolo, **prima di qualunque devnet**.

Oggi nessuna firma è verificata da nulla. `coblox-core` dichiara la mancanza come limite esplicito e ne dà la ragione, che è anche la ragione per cui questa spec ha la priorità che ha: *un verificatore non validato sui casi limite è **indistinguibile da uno corretto fino a una divisione della catena**.*

## Context

Raccomandazione di AGENT-001 alla chiusura di [SPEC-008], condivisa dal Lead. Il crate spedisce deliberatamente le **preimmagini** di firma, che sono deterministiche e testate, e la cucitura `SignatureVerifier`, e **non** spedisce un verificatore: spedirne uno senza i vettori come oracolo sarebbe esattamente il comportamento non validato che la specifica vieta.

La regola è già scritta e non va inventata. `docs/protocol/README.md` §*Consensus-critical Ed25519 verification* impone una regola derivata da ZIP-215 in quattro punti, con una quinta condizione propria di Coblox — `[8]A != identity`, che rifiuta le chiavi di ordine piccolo — e pubblica la tabella degli esiti attesi per i vettori 0–11.

## Scope

### Included

- L'implementazione della regola pubblicata dietro la cucitura `SignatureVerifier`.
- I dodici vettori di `novifinancial/ed25519-speccheck` versionati nel repository come dati di prova, con la loro provenienza.
- La tabella di conformità eseguita, vettore per vettore.
- Il verdetto su quale strada seguire fra libreria vagliata e aritmetica propria, motivato.

### Excluded

- **Il cablaggio del verificatore nei percorsi di consenso.** Non esistono ancora: blocchi, certificati di quorum e transazioni sono lavoro successivo. Questa spec fornisce l'implementazione della cucitura, non i suoi chiamanti.
- Qualunque modifica a `docs/protocol/README.md` §*Consensus-critical Ed25519 verification*, **con l'eccezione dichiarata in Risks**: se la tabella pubblicata risultasse sbagliata, correggerla è nel perimetro e diventa il risultato principale.
- Chiavi di consenso dei validatori, prove di possesso e rotazione: sono di altre spec.

## Existing-project analysis

**Verificato dal Lead il 2026-08-25:**

- `core/coblox-core/src/lib.rs` definisce il tratto `SignatureVerifier` e documenta il limite; nessuna implementazione esiste nel crate.
- `registry.rs` produce la **preimmagine** di firma e documenta che il valore è il messaggio e non un digest, perché Ed25519 lo digerisce internamente. È il contratto che il verificatore deve rispettare.
- `hash.rs` deriva `node_id` da una chiave pubblica Ed25519 a 32 byte.
- La regola pubblicata comprende quattro punti più la condizione sulle chiavi di ordine piccolo, vieta l'equazione senza cofattore `[S]B = R + [k]A`, e precisa che l'hash per `k` usa **le codifiche originali**, non i punti ricodificati.
- La tabella pubblicata degli esiti per i vettori 0–11 è: `reject, reject, accept, accept, accept, accept, reject, reject, accept, accept, reject, reject`.

**Un punto che l'implementatore deve leggere con attenzione, perché il Lead lo giudica frainendibile.** La specifica vieta di *sostituire* `verify_strict`, le modalità di compatibilità legacy o un default di libreria **la cui accettazione sui casi limite non sia stata mostrata equivalente** alle regole. Il divieto è sulla sostituzione non dimostrata, **non sull'uso di una libreria**. Scrivere aritmetica di curva a mano per rispettare alla lettera una frase che non lo chiede sarebbe il modo peggiore di ottemperare: è il genere di codice in cui i difetti sono catastrofici e invisibili. La scelta va fatta nel merito e motivata.

## Technical proposal

### 1. La strada, motivata prima di essere presa

Due strade sono ammissibili e vanno confrontate esplicitamente nell'evidenza:

- **Comporre su una libreria vagliata**, aggiungendo le condizioni che la libreria non applica — tipicamente il rifiuto delle chiavi di ordine piccolo — e **dimostrando l'equivalenza** sui dodici vettori. È la strada che il Lead si aspetta, perché sposta il rischio dall'aritmetica alla composizione, che è verificabile.
- **Implementare l'equazione con cofattore direttamente** sulle primitive di curva, se la libreria disponibile non espone la forma richiesta.

In entrambi i casi valgono i vincoli della specifica: equazione **con** cofattore, codifiche y non canoniche accettate e ridotte, `0 <= S < L`, e l'hash per `k` calcolato sulle codifiche originali.

### 2. I vettori sono dati versionati, non una dipendenza di rete

I dodici vettori vanno **incorporati nel repository** come dati di prova, con la loro provenienza scritta accanto: origine, revisione, e come sono stati ottenuti. Scaricarli in fase di test renderebbe la conformità dipendente da una rete e da un repository di terzi, il che è un problema di riproducibilità e di catena di fornitura insieme.

### 3. La tabella si esegue, non si asserisce

Il criterio di accettazione è che l'esito **osservato** per ciascun vettore sia riportato accanto a quello pubblicato, uno per uno. Non un test aggregato che passa: dodici righe, con il verdetto di ciascuna.

## Files and areas involved

- `core/coblox-core/src/` — l'implementazione del verificatore, in un modulo proprio.
- `core/coblox-core/tests/` — la tabella di conformità eseguita.
- Sede dei vettori, da proporre e motivare.
- `core/coblox-core/Cargo.toml` e il lockfile, se si aggiunge una dipendenza; e `deny.toml` se ne deriva un advisory o una licenza da vagliare.
- `core/coblox-core/src/lib.rs` — il limite dichiarato va aggiornato quando smette di essere vero.

## Acceptance criteria

- [ ] Esiste un'implementazione di `SignatureVerifier` che applica i quattro punti della regola pubblicata più il rifiuto delle chiavi di ordine piccolo.
- [ ] L'equazione usata è quella **con cofattore**; l'assenza della forma senza cofattore è dimostrata e non asserita.
- [ ] L'hash per `k` è calcolato sulle codifiche originali, e un test lo distingue dal calcolo su punti ricodificati.
- [ ] I dodici vettori sono versionati nel repository con la loro provenienza.
- [ ] L'esito **osservato** di ciascuno dei dodici vettori è riportato accanto a quello pubblicato, riga per riga.
- [ ] La scelta fra libreria vagliata e aritmetica propria è motivata nel merito, e se si usa una libreria l'equivalenza sui casi limite è **mostrata**, non affermata.
- [ ] Il verificatore rispetta il contratto di `registry::signing_preimage`: riceve il messaggio, non un suo digest.
- [ ] Il limite dichiarato nella documentazione di `coblox-core` è aggiornato: oggi dice che il crate non spedisce un verificatore.
- [ ] Se è stata aggiunta una dipendenza, `cargo-deny` passa e la scelta è giustificata.

## Implementation plan

1. Ottenere i dodici vettori, incorporarli con la provenienza, e verificarne l'integrità rispetto alla fonte.
2. Confrontare le due strade e sceglierne una motivando.
3. Implementare, con particolare attenzione ai tre punti in cui la regola diverge dai default: cofattore, codifiche non canoniche, `[8]A != identity`.
4. Eseguire la tabella e riportare gli esiti osservati riga per riga.
5. Se un esito diverge da quello pubblicato, **fermarsi e riportare** prima di modificare qualunque cosa.
6. Aggiornare il limite dichiarato nel crate.

## Required verification

<!-- Canonical form: ID | kind=executable|manual|operator | owner=agent|kit|lead|operator | phase=before-submit|before-done | evidence=transcript|observation|artifact | requirement -->
- [ ] GATE-SPECCHECK | kind=manual | owner=agent | phase=before-submit | evidence=transcript | La tabella dei dodici vettori è eseguita e la trascrizione riporta l'esito **osservato** accanto a quello pubblicato, riga per riga. Un test aggregato che passa non soddisfa questa gate: il valore sta nel confronto vettore per vettore, ed è l'unica evidenza che distingue un verificatore corretto da uno che lo sembra.
- [ ] GATE-COFACTOR | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Esiste almeno un caso in cui l'equazione con cofattore e quella senza danno esiti **diversi**, e la trascrizione mostra che l'implementazione segue quella con cofattore. È la differenza che la specifica vieta di sbagliare, e un test che non la esercita non dice nulla.
- [ ] GATE-DEPENDENCY | kind=manual | owner=agent | phase=before-submit | evidence=transcript | Se è stata introdotta una dipendenza crittografica, `cargo-deny` è eseguito e passa, e la scelta della libreria è motivata con la sua provenienza. Una dipendenza crittografica nuova in un repository pubblico è una superficie di catena di fornitura, non una riga di `Cargo.toml`.
- [ ] GATE-SECREVIEW | kind=manual | owner=lead | phase=before-done | evidence=artifact | AGENT-007 ha rivisto l'implementazione e il Lead ha accettato la review. È l'unico componente del progetto in cui un difetto non produce un errore ma un'accettazione silenziosa.

## Production quality and documentation
- Follow [[QUALITY]]; this is production work, not a prototype.
- Identify and update all relevant technical LMBrain knowledge pages delegated by this spec.
- Report any quality-policy exception explicitly; do not silently accept shortcuts.

## Risks and open decisions

- **La tabella pubblicata potrebbe essere sbagliata, e scoprirlo sarebbe il risultato più prezioso di questa spec.** È stata scritta durante [SPEC-001] e **non è mai stata eseguita da nessuno**: dodici esiti di casi limite su una regola con una condizione non standard, compilati a mano. Se un esito osservato diverge da quello pubblicato, non è un fallimento della spec — è la ragione per cui la spec esiste. In quel caso: fermarsi, riportare la divergenza con la derivazione che la spiega, e non correggere né il codice né la tabella prima che il Lead abbia verificato in modo indipendente da che parte sta l'errore. **La tabella è un artefatto pubblicato**, e correggerla ricade sotto [ADR-012].
- **Il rischio secondario è ottemperare alla lettera peggiorando la sostanza**, scrivendo aritmetica di curva a mano per evitare una libreria che la specifica non vieta. Vedi *Existing-project analysis*.
- **`[8]A != identity` non è ZIP-215**, è una condizione propria di Coblox. Una libreria che implementa ZIP-215 correttamente non la applica, e l'omissione non si vede su nessun vettore che non contenga una chiave di ordine piccolo.
- Il Lead non ha eseguito i vettori e non ha verificato la tabella: qui la sua analisi si ferma, ed è detto invece di essere lasciato intendere.

## Instructions for the assigned specialist
- If this spec is in `ready`, run `spec_start` as your first implementation action and `spec_submit` when the implementation is complete. If this spec is already in `review` for remediation, do not move it back to `working`; update evidence and report completion for re-review.
- Implement only the stated scope.
- Report changed files, tests run, and known limitations.
- Produce production-grade, maintainable code; do not ship placeholder, POC, or knowingly incomplete behaviour.
- Update only the technical documentation explicitly delegated by this spec, plus implementation evidence.
- Challenge flawed or fragile technical assumptions and propose the clean alternative; consult current official documentation when material behavior is uncertain or changeable.
- Do not adopt shortcuts without the explicit operator-approved exception required by [[QUALITY]].
- Do not change product scope, roadmap, or ADRs.
- **V3 context-economy:** Read mandatory policy files (`QUALITY.md`, `CONTRACT.md`, `AGENT.md`) first. Use `lmbrain_spec_context` for a compact handoff context. Expand to full artifacts and source code only when the context pack points to them or verification requires it. Record evidence when you expand scope beyond the context pack.

## Implementation evidence
> Filled in by the specialist after completion.

### Changes made

### Files changed

### Verification performed

### Verification transcript

<!-- Required before spec_submit. Paste actual command/result output in a fenced block, or use approved `spec_verify` gates. Predictions and summaries are not execution evidence. -->

```text

```

### Deviations from the specification

### Handoff status
- [ ] Ready for Project Lead review
