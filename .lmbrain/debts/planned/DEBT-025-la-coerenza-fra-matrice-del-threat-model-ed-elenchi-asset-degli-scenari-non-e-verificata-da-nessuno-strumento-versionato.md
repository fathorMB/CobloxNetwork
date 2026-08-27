---
id: DEBT-025
title: "La coerenza fra matrice del threat model ed elenchi asset degli scenari non e' verificata da nessuno strumento versionato"
status: planned
category: "verification"
severity: "medium"
origin_severity: null
area: "security"
milestone: "M-02"
owner: "AGENT-007"
origin_artifact: "SPEC-018"
origin_ref: "controllo di coerenza, otto disallineamenti"
related_specs: ["SPEC-018"]
related_reviews: []
related_decisions: ["ADR-012"]
target_specs: ["SPEC-026"]
blocked_by: []
resolution_refs: []
superseded_by: null
revisit_condition: null
created: 2026-08-26
updated: 2026-08-27
tags: ["verification-gap","threat-model","documentation"]
links: []
activity:
  - date: 2026-08-27
    action: "planned: Il debito e' gia' soddisfatto in parte e il residuo coincide con lo scope di SPEC-026. Verificato dal Lead il 2026-08-27: `sim/tools/threat_model_matrix_coherence.py` esiste versionato, gira, e riporta \"matrice e scenari coerenti\" con 104 celle, 97 coperte, 7 n/a e 43 scenari — quindi gli otto disallineamenti che il debito voleva sanati prima del cablaggio sono sanati.\n\nRestano esattamente due cose, ed entrambe sono cio' che SPEC-026 fa per la propria famiglia di controlli: la prova in negativo — oggi `--negative` e' ignorato e produce lo stesso output del comando nudo, quindi lo strumento non ha mai dimostrato di vedere — e il cablaggio in CI, oggi assente. Il debito avverte che uno strumento cablato mentre e' rosso viene disattivato, \"il modo in cui una guardia muore\": qui e' verde, quindi il cablaggio e' sicuro."
debt_events:
  - schema_version: "1"
    id: "DEBT-025-EVENT-001"
    timestamp: "2026-08-26T01:35:40.548136900+02:00"
    action: "created"
    from_status: "none"
    to_status: "open"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Aperto dal Lead su raccomandazione esplicita di AGENT-007, che ha stabilito la distinzione giusta e si e' fermata alla decisione invece di prenderla: la **correttezza semantica** di una cella non e' meccanizzabile e resta giudizio, la **coerenza fra matrice ed elenchi** lo e'. Meccanizzare la seconda non pretende di aver meccanizzato la prima, ed e' la ragione per cui questo debito e' scrivibile senza scivolare in una promessa piu' grande.\n\nNon chiuso dentro [SPEC-018] perche' quella spec, per costruzione, non ha la gate di [ADR-012] e non tocca la strumentazione: aggiungerle uno strumento versionato le avrebbe fatto acquistare il perimetro che le era stato deliberatamente tolto."
    evidence_refs: []
  - schema_version: "1"
    id: "DEBT-025-EVENT-002"
    timestamp: "2026-08-27T15:00:55.259007400+02:00"
    action: "planned"
    from_status: "open"
    to_status: "planned"
    actor_role: "project-lead"
    actor: "AGENT-LEAD"
    rationale: "Il debito e' gia' soddisfatto in parte e il residuo coincide con lo scope di SPEC-026. Verificato dal Lead il 2026-08-27: `sim/tools/threat_model_matrix_coherence.py` esiste versionato, gira, e riporta \"matrice e scenari coerenti\" con 104 celle, 97 coperte, 7 n/a e 43 scenari — quindi gli otto disallineamenti che il debito voleva sanati prima del cablaggio sono sanati.\n\nRestano esattamente due cose, ed entrambe sono cio' che SPEC-026 fa per la propria famiglia di controlli: la prova in negativo — oggi `--negative` e' ignorato e produce lo stesso output del comando nudo, quindi lo strumento non ha mai dimostrato di vedere — e il cablaggio in CI, oggi assente. Il debito avverte che uno strumento cablato mentre e' rosso viene disattivato, \"il modo in cui una guardia muore\": qui e' verde, quindi il cablaggio e' sicuro."
    evidence_refs: ["SPEC-026"]
---
# La coerenza fra matrice del threat model ed elenchi asset degli scenari non e' verificata da nessuno strumento versionato

## Statement

Uno scenario del threat model dichiara nel proprio campo *Asset* quali asset colpisce, e la matrice dichiara quali scenari coprono ogni cella. **Le due dichiarazioni non sono confrontate da nulla.** Uno scenario che dichiara di colpire `A-06` e non compare nella riga `A-06` e' un buco nell'evidenza di `GATE-COVERAGE`: la copertura risulta dimostrata e non lo e'.

## Evidence and provenance

AGENT-007 ha scritto uno script ad hoc durante [SPEC-018] e ha trovato **otto disallineamenti**: sei **preesistenti**, piu' `TM-39` mai collocato in matrice da quando e' stato scritto, e — il dato che rende il debito necessario — **due introdotti dalla passata stessa**, cioe' dall'agente che stava scrivendo il controllo. Lo script vive nello scratchpad e non e' versionato; le e' stato chiesto di salvarlo sotto `sim/tools/` senza cablarlo in CI, come punto di partenza e non come chiusura.

## Impact and scope boundary

Il difetto non e' in una cella ma nella **prova di copertura**. `GATE-COVERAGE` afferma che ogni coppia asset-attore e' stata considerata; se un asset dichiarato da uno scenario non compare nella riga corrispondente, quella riga sembra coperta da meno di quanto la sostiene, o l'asset sembra colpito da uno scenario che la matrice non conosce. In entrambi i versi **la matrice dice al lettore successivo qualcosa di falso su dove guardare**, che e' la stessa forma di danno di una `n/a` sbagliata.

Il fatto che due degli otto siano nati durante la passata che li cercava e' la misura giusta della frequenza: non e' un difetto storico da sanare una volta, e' una deriva che si rigenera a ogni modifica.

Questa e' inoltre la classe di difetto che **ne' [DEBT-018] ne' [SPEC-018] nominavano**, ed e' emersa perche' qualcuno ha scritto un controllo invece di rileggere.

## Decision log

Created by AGENT-LEAD: Aperto dal Lead su raccomandazione esplicita di AGENT-007, che ha stabilito la distinzione giusta e si e' fermata alla decisione invece di prenderla: la **correttezza semantica** di una cella non e' meccanizzabile e resta giudizio, la **coerenza fra matrice ed elenchi** lo e'. Meccanizzare la seconda non pretende di aver meccanizzato la prima, ed e' la ragione per cui questo debito e' scrivibile senza scivolare in una promessa piu' grande.

Non chiuso dentro [SPEC-018] perche' quella spec, per costruzione, non ha la gate di [ADR-012] e non tocca la strumentazione: aggiungerle uno strumento versionato le avrebbe fatto acquistare il perimetro che le era stato deliberatamente tolto.

## Resolution criteria

Il controllo diventa uno **strumento versionato** sotto `sim/tools/`, eseguito e con la trascrizione allegata come gli altri della famiglia [ADR-012], e **provato in negativo**: un disallineamento introdotto ad arte deve farlo fallire nominando lo scenario e l'asset.

Deve verificare **nei due versi**, come `C6-ORPHAN` gia' fa per i documenti di protocollo: ogni asset dichiarato da uno scenario compare nella riga di quell'asset, e ogni scenario citato in una cella dichiara quell'asset.

Gli otto disallineamenti noti vanno sanati **prima** che lo strumento sia cablato, altrimenti nasce rosso e viene disattivato, che e' il modo in cui una guardia muore.

**Il rimedio apparente da non adottare:** correggere gli otto e basta. Sarebbero corretti oggi e riderivati alla prossima modifica — due degli otto sono nati durante la passata che li cercava. Il difetto e' l'assenza del controllo, non lo stato corrente delle otto righe.

## Resolution evidence

