---
id: ADR-006
# Note: Quote the title if it contains a colon
title: "Pubblicazione delle app e ricompensa al creatore"
status: accepted
decision_date: 2026-08-25
decider: AGENT-LEAD
# References use IDs only (e.g. [ADR-001]); use [[wikilinks]] in prose
# Both sides are written together by `adr_supersede` once this ADR is accepted.
# Declaring `supersedes` while still proposed records the intent; it takes
# effect at acceptance. Do not edit either side by hand.
supersedes: []
superseded_by: []
links: [ADR-002, ADR-004, ADR-005]
tags: [architecture]
created: 2026-08-25
updated: 2026-08-25
activity:
  - date: 2026-08-25
    action: "created"
  - date: 2026-08-25
    action: "transitioned proposed -> accepted"
---
# Pubblicazione delle app e ricompensa al creatore

## Context

[ADR-004] stabilisce che le app sono moduli WASM eseguiti dai nodi, e [ADR-005] che le spese si bruciano mentre i fornitori di risorse ricevono nuova emissione. Mancava però il percorso concreto con cui un utente pubblica un'app: come si finanzia l'hosting, chi sceglie i nodi ospiti, a che prezzo, e se chi ospita può rifiutare.

Analizzando il flusso è emersa una lacuna in [ADR-005]: applicato agli abbonamenti, l'utente brucia token, i nodi ospitanti guadagnano, e **il publisher non guadagna nulla**. Pubblicare resterebbe puro costo, crescente col successo dell'app: nessuno pubblicherebbe mai un servizio popolare. Il lato domanda dell'economia non si chiude.

## Decision

### Flusso di pubblicazione

1. **Build e manifest** — il publisher compila il modulo WASM e dichiara nel manifest capability, tetti di risorse, numero di repliche desiderate ed eventuale prezzo di abbonamento.
2. **Indirizzamento e firma** — il modulo è indirizzato per hash del contenuto e firmato con l'identità di nodo del publisher. Immutabilità e provenienza derivano dall'indirizzamento; una nuova versione è un nuovo hash.
3. **Finanziamento a consumo** — l'app possiede un proprio saldo che si consuma (burn) per epoche di hosting, non un pagamento unico anticipato. Esaurito il saldo l'app è sospesa con preavviso, mai cancellata.
4. **Assegnazione dei nodi da parte del protocollo** — il publisher dichiara i requisiti; il protocollo seleziona i nodi ospiti idonei pesando reputazione e uptime ([ADR-002]). Il publisher non sceglie i propri fornitori.
5. **Distribuzione e registrazione** — il modulo viaggia sul livello storage della rete e l'app entra nel catalogo di scoperta.
6. **Ciclo di vita** — monitoraggio, aggiornamento (nuovo hash, con migrazione dello stato), ritiro.

### Prezzi fissati dal protocollo, niente aste

Il costo dell'hosting segue un listino per unità di risorsa, fissato dalla governance dei validatori e tarato in simulazione. Non esiste alcun meccanismo d'asta o di offerta competitiva tra nodi.

### Consenso dell'ospite

Ogni operatore di nodo può dichiarare cosa è disposto a ospitare e mantenere una propria lista di rifiuto. In aggiunta esiste una lista di blocco di rete, governata dai validatori, riservata agli abusi conclamati. La sandbox WASM copre la sicurezza tecnica, non quella legale o reputazionale di chi presta la propria macchina.

### Ricompensa al creatore

Gli abbonamenti degli utenti sono bruciati come previsto da [ADR-005]; il protocollo emette in aggiunta una **quota al publisher** proporzionale agli abbonati attivi, accanto a quella destinata ai nodi ospitanti. Questa ADR estende [ADR-005] introducendo una nuova categoria di emissione; non ne sostituisce alcuna decisione.

## Alternatives considered

- **Asta al ribasso tra provider (modello Akash):** matura ed efficiente, ma produce un prezzo di mercato — il primo passo perché il token si comporti come denaro, in contrasto diretto col vincolo fondativo del progetto.
- **Publisher che sceglie i propri nodi ospiti:** massimo controllo, ma concentra le app sui pochi nodi più grandi e favorisce la nascita di cartelli.
- **Pagamento unico anticipato con deposito a garanzia:** più semplice da implementare, ma rende opaco il costo reale e produce un'esperienza a scatti invece del saldo leggibile "l'app ha copertura per N giorni".
- **Nessuna ricompensa al creatore (pubblicare è puro costo, modello Urbit/software libero):** coerentissimo col rifiuto della logica monetaria, ma realisticamente popola la rete di sole app-giocattolo.
- **Solo sconto sull'hosting per il publisher:** più sobrio, un'app di successo si autofinanzia senza mai fruttare; scartato perché non ripaga il lavoro di chi scrive servizi realmente usati.
- **Rinviare la decisione ai dati del simulatore (M-02):** valutato e scartato: la lacuna è strutturale, non parametrica, e vincola il formato del manifest già in M-01.

## Consequences

- Il manifest delle app deve prevedere i campi per repliche, tetti di risorse e prezzo di abbonamento: vincola il lavoro in corso su [SPEC-001].
- Serve un'entità "saldo dell'app" nel ledger, distinta dal saldo dell'utente, con la sua meccanica di consumo per epoche.
- La quota al creatore è un nuovo parametro economico da tarare col simulatore in M-02, e va sorvegliata: se troppo generosa, incentiva abbonati fittizi controllati dal publisher stesso (nuova superficie Sybil da coprire nel threat model, [SPEC-004]).
- La lista di rifiuto per nodo e la lista di blocco di rete sono funzionalità di prodotto da specificare, non dettagli di implementazione.
- Nessun gatekeeper è necessario per la pubblicazione: poiché pubblicare brucia token e i token si guadagnano contribuendo, per pubblicare occorre aver prima contribuito — antispam naturale.

## Review conditions

Rivedere se: le simulazioni di M-02 mostrano che la quota al creatore destabilizza l'offerta o è economicamente sfruttabile con abbonati fittizi; il listino fissato dal protocollo si rivela troppo rigido per risorse molto eterogenee; la governance della lista di blocco di rete si dimostra un punto di potere problematico.
