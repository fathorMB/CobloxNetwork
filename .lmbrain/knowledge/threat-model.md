# Threat model della rete Coblox — v1

Autrice: AGENT-007 (Greta Threatmodel) · Spec di origine: [SPEC-004] · Data: 2026-08-25
Stato del documento: prima emissione, da rivedere a ogni ADR che tocchi identità,
emissione, consenso o pubblicazione delle app.

## 1. Cosa fa e cosa non fa questo documento

Questo è il threat model **della rete**, non la review di una specifica. La review
di sicurezza di [SPEC-001] esiste già ed è [REVIEW-002]: 18 finding (`RF-001`…`RF-018`)
con scenario d'attacco, condizione di chiusura e costo. **Quei finding non sono
ripetuti qui.** Dove uno scenario di questo documento coincide con un finding
esistente, lo scenario lo cita per ID e aggiunge solo ciò che la vista d'insieme
rende visibile e la review di una singola spec non poteva dare: la composizione fra
attacchi, gli attori che nessuna spec nomina, e le superfici che non appartengono a
nessun documento di protocollo perché stanno fra due decisioni.

Il documento aggiunge in proprio:

- l'inventario di **attori e asset** e la matrice di copertura (§4);
- gli scenari che nascono dalla **combinazione** di più superfici — per esempio
  Sybil di massa più elezione pesata sull'uptime (§6.1), o reddito di esistenza che
  finanzia gli abbonati fittizi (§6.3);
- due superfici che nessuna spec copre oggi: la **privacy dell'osservatore di rete**
  (§5.6) e l'**abuso della lista di blocco** introdotta da [ADR-006] (§5.7);
- il **quadro decisionale sull'anti-Sybil** (§7), che è il motivo per cui questo
  documento è stato commissionato prima della decisione, e non dopo;
- i requisiti `SEC-REQ-NN` (§9) e i test di attacco per M-02/M-03 (§10).

### Metodo

Struttura per scenari, non per componenti: ogni riga è un attacco raccontabile con
passi concreti e quindi contestabile da chi non è d'accordo. Ogni scenario dichiara
severità, contromisura e **stato**; ogni contromisura dichiara il proprio **costo**.
Un requisito di sicurezza che ignora il costo non viene implementato, e il documento
avrebbe fallito il suo scopo.

**Severità** — combinazione di impatto e praticabilità, non impatto puro:

| Livello | Significato |
| --- | --- |
| `critica` | Rompe un invariante fondativo (safety del consenso, non falsificabilità dell'emissione) ed è praticabile da un attaccante con risorse ordinarie. |
| `alta` | Danno strutturale all'economia, alla verità mostrata all'utente o all'operatore di nodo; praticabile con risorse ordinarie o con la collusione di pochi. |
| `media` | Danno reale ma circoscritto, oppure grave ma che richiede una posizione privilegiata. |
| `bassa` | Fastidio, degrado, o perdita di proprietà desiderabili ma non fondative. |

**Stato** — disposizione decisa, mai lasciata implicita:

| Stato | Significato |
| --- | --- |
| `mitigato` | Una contromisura è già scritta in un ADR o in un documento di protocollo e regge all'analisi. |
| `aperto` | Nessuna contromisura sufficiente esiste oggi; il documento ne propone una e ne dichiara il costo. |
| `accettato` | Il rischio resta, consapevolmente, con la ragione dichiarata e la condizione che ne imporrebbe la revisione. |
| `n/a` | La combinazione attore × asset non è realizzabile, con il motivo. |

Il perimetro temporale è la rete v0 come definita da [ADR-001], [ADR-002],
[ADR-004], [ADR-005], [ADR-006] e dai documenti di `docs/protocol/`. Le app di
terze parti come sistema (SDK, marketplace) restano fuori: arrivano con M-06.

### Una nota di onestà preliminare

Il claim di prodotto "super-sicura" si regge, oggi, su due gambe di lunghezza molto
diversa. La rete v0 è **robusta contro la falsificazione**: saldi, firme, doppia
spesa e prove Merkle hanno un impianto crittografico valido, e i difetti trovati in
[REVIEW-002] si chiudono tutti aggiungendo campi e regole senza toccare
l'architettura. La rete v0 **non è resistente ai Sybil per via crittografica**, e non
può diventarlo con il design attuale. §7 esiste per rendere questa seconda frase una
decisione informata invece di una scoperta tardiva.

## 2. Asset da proteggere

Ordinati per gravità della perdita, non per probabilità.

| ID | Asset | Perché conta | Perdita significa |
| --- | --- | --- | --- |
| `A-01` | **Integrità del ledger** — saldi, nonce, unicità delle transazioni | È il registro di tutto ciò che la rete misura | Saldi falsi, doppia spesa, storia riscrivibile |
| `A-02` | **Integrità dell'emissione** — ogni `mint` ancorato a evidenza verificabile | È la promessa "prove crittografiche per ogni accredito" di [[PROJECT]] | Token stampabili: l'economia perde ogni significato |
| `A-03` | **Safety e liveness del consenso** | Senza finalità non esiste "in tempo reale" né saldo affidabile | Fork permanenti o rete ferma |
| `A-04` | **Integrità del set di validatori** — chi ne fa parte e come ci entra | Il set è il perimetro di fiducia dichiarato da [ADR-001] | Cattura permanente della rete da parte di pochi |
| `A-05` | **Verità mostrata all'utente** — light client e dashboard | È l'unica cosa che l'utente vede davvero | L'utente prende decisioni su dati falsi o vecchi |
| `A-06` | **Integrità del sistema di challenge** — emissione, casualità, evidenze | È l'unico ponte fra "risorsa fornita" e "token accreditato" | L'emissione si scollega dal lavoro reale |
| `A-07` | **Identità e chiavi dei nodi** | Identità = conto, reputazione, potere di voto | Impersonificazione, furto del reddito accumulato |
| `A-08` | **Risorse e incolumità dell'operatore di nodo** — CPU, RAM, disco, batteria, banda, reputazione dell'IP, esposizione legale | Sono macchine personali di volontari, non capacità cloud | La gente spegne i nodi; la rete muore per abbandono |
| `A-09` | **Isolamento della sandbox** | I nodi eseguono codice di sconosciuti ([ADR-004]) | Compromissione della macchina di un partecipante |
| `A-10` | **Disponibilità e connettività della rete** | Un nodo isolato non guadagna e non verifica | Esclusione mirata, eclipse, rete inutilizzabile |
| `A-11` | **Privacy dei partecipanti** — legame `node_id` ↔ IP ↔ persona, e pattern di uso | Nessun ADR la nomina; è l'asset più esposto e meno difeso | Profilazione di chi usa cosa e quando, permanente e pubblica |
| `A-12` | **Accesso non discriminatorio** — enrollment, pubblicazione, hosting | È ciò che distingue una rete distribuita da un servizio | Censura, gatekeeping, pressione su chi pubblica |
| `A-13` | **Integrità del catalogo e della distribuzione dei moduli** | Il codice eseguito dai nodi arriva da lì | Esecuzione di moduli sostituiti o non voluti |

`A-11` merita una riga in più, perché è l'asset che questo documento aggiunge e che
non compare in nessun ADR: nel design attuale il `node_id` è **stabile per la vita
della chiave** (`identity.md` §"Node identifier"), ogni envelope è firmato con
`sender_node_id` in chiaro (`wire.md` §"Signed envelope"), l'uso della modalità
autore anonimo di libp2p è **vietato** (`wire.md` §"Gossip validation and
backpressure") e ogni `app_subscription` burn nomina in chiaro `payer_node_id` e
`app_id` su un ledger finalizzato per sempre (`ledger.md` §"Burn"). Il risultato non
è un difetto di implementazione: è una proprietà del design.

## 3. Attori ostili

Ogni attore è descritto per **capacità e budget**, non per intenzione: la difesa si
progetta sul primo, non sulla seconda.

| ID | Attore | Capacità e budget | Motivazione |
| --- | --- | --- | --- |
| `T-01` | **Nodo egoista** (free-rider enrollato) | Un'identità legittima, hardware ordinario, nessuna collusione. Modifica il proprio client. | Massimizzare gli accrediti minimizzando le risorse davvero fornite |
| `T-02` | **Fattoria di identità / botnet Sybil** | Da migliaia a milioni di identità; una GPU commodity, uno o più VPS, un blocco di indirizzi IP. Nessun accesso privilegiato. | Catturare una quota dominante dell'emissione, e per suo tramite del potere di voto |
| `T-03` | **Validatore malevolo singolo** | Potere di voto entro `f`; emette challenge, propone blocchi, firma certificati. | Favorire sé stesso, danneggiare un concorrente, censurare un utente |
| `T-04` | **Cartello di validatori** | Potere di voto coordinato: soglia rilevante ≥ 1/3 per rompere la safety o la liveness, > 2/3 per il controllo pieno. Include il cartello *storico* (chiavi ormai ruotate fuori). | Controllo della rete, dei parametri e dell'emissione |
| `T-05` | **Publisher ostile** | Pubblica moduli WASM, controlla identità enrollate, sceglie prezzi di abbonamento e tetti di risorsa nel manifest. | Lucrare la quota al creatore, usare le macchine altrui, scaricare rischio legale sugli hoster |
| `T-06` | **Osservatore di rete / avversario di percorso** | Osservazione passiva del traffico; oppure un peer enrollato che si connette a molti; oppure un ISP/censore che filtra. Nessun potere di voto. | De-anonimizzare i partecipanti, profilarne il consumo, isolarne alcuni |
| `T-07` | **Insider di governance** | Chi controlla ciò che i validatori firmano fuori dal ledger: parameter set, `policy_hash`, rate card, lista di blocco di rete, seed di bootstrap, distribuzione dei trust anchor e dei binari. | Potere discrezionale sulla rete senza mai violare una regola scritta |

`T-07` non è nell'elenco dello scope di [SPEC-004]. Lo aggiungo perché lo scope
richiede esplicitamente l'analisi dell'**abuso della lista di blocco**, e quella
superficie non ha un attore fra i sei nominati: non è un validatore malevolo (agisce
col quorum, non contro di esso) né un publisher. Senza `T-07` la richiesta della
spec non sarebbe rappresentabile nella matrice.

## 4. Matrice attori × asset

Ogni cella contiene gli scenari che la coprono, oppure `n/a` con il motivo. Nessuna
cella è vuota. Questa matrice è l'evidenza richiesta da `GATE-COVERAGE`.

| | `T-01` egoista | `T-02` Sybil | `T-03` validatore | `T-04` cartello | `T-05` publisher | `T-06` osservatore | `T-07` insider |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **A-01** ledger | TM-04 | n/a — nessuna identità, per quanto numerosa, altera saldi altrui: ogni burn è firmato dal debitato e ogni mint richiede quorum | TM-16 | TM-17, TM-20 | n/a — il publisher non ha percorso verso saldi altrui: `BurnBody` esige la firma del pagatore | n/a — attore senza potere di scrittura sul ledger | TM-20 |
| **A-02** emissione | TM-01, TM-02, TM-03, TM-05 | TM-08 | TM-14 | TM-20 | TM-22 | n/a — l'emissione non dipende da dati osservabili sul filo | TM-35 |
| **A-03** consenso | TM-04 | TM-11 | TM-12, TM-16 | TM-17, TM-21 | n/a — nessun percorso dal manifest al voto di finalità | TM-31 | TM-19 |
| **A-04** set validatori | n/a — un singolo nodo egoista non altera la composizione del set | TM-09 | n/a — un validatore singolo non decide il set successivo, che richiede quorum | TM-18 | n/a — nessun percorso dalla pubblicazione all'elezione | n/a — nessun potere sulla composizione | TM-19 |
| **A-05** verità all'utente | n/a — un nodo egoista non serve prove: non ha vantaggio a falsificare ciò che l'utente vede | TM-10 | TM-12 | TM-21 | n/a — il manifest non alimenta la verifica del saldo | TM-31 | TM-36 |
| **A-06** challenge | TM-02, TM-05 | TM-08 | TM-13, TM-14 | TM-19 | n/a — il publisher non emette né valuta challenge | TM-30 | TM-19 |
| **A-07** identità e chiavi | n/a — l'egoista abusa della propria identità, non ne attacca altre | TM-06, TM-07 | n/a — nessun percorso verso la chiave privata di un altro nodo: le chiavi non transitano mai (`identity.md` §"Key hierarchy") | TM-21 | n/a — nessun percorso verso chiavi altrui | TM-28 (legame identità ↔ persona, non furto di chiave) | TM-35 |
| **A-08** risorse dell'operatore | n/a — l'egoista risparmia le proprie risorse, non consuma quelle altrui | TM-11 | n/a — un validatore non assegna carico agli host: lo fa la politica di assegnazione | TM-19 | TM-23, TM-24, TM-25 | n/a — l'osservatore non impone carico oltre il traffico ordinario | TM-33 |
| **A-09** sandbox | n/a — l'egoista non pubblica moduli | n/a — moltiplicare identità non aggiunge alcun percorso verso il runtime | n/a — nessun percorso dal ruolo di validatore al runtime di un host | n/a — come sopra; il cartello agisce sul ledger, non sul runtime | TM-23, TM-27 | n/a — nessun percorso verso il runtime | n/a — l'insider agisce su parametri e liste, non sul runtime |
| **A-10** disponibilità | n/a — l'egoista vuole la rete viva, ci guadagna | TM-10, TM-11 | TM-15 | TM-17 | TM-24 | TM-31, TM-32 | TM-33 |
| **A-11** privacy | n/a — non ha accesso a dati altrui oltre quelli pubblici | TM-29 (con molte identità osservatrici) | TM-30 | TM-29 | TM-26 | TM-28, TM-29, TM-30 | TM-34 |
| **A-12** accesso | n/a — nessun potere di esclusione | TM-09 | TM-13, TM-15 | TM-18, TM-19 | n/a — il publisher non esclude altri publisher | TM-31 | TM-33, TM-34, TM-35 |
| **A-13** catalogo e moduli | n/a — nessun percorso di scrittura sul catalogo | n/a — il contenuto è indirizzato per hash e firmato: moltiplicare identità non aiuta | n/a — un validatore singolo non altera un record di catalogo finalizzato | TM-20 | TM-25 | n/a — il trasporto non è fidato per costruzione (`app-manifest.md` §"Deterministic container") | TM-33, TM-36 |

Sedici celle sono `n/a`. Quattro di esse lo sono per una ragione che vale la pena
isolare, perché sono **proprietà di design conquistate** e non coincidenze: il
divieto strutturale di trasferimento (`A-01` × `T-02`/`T-05`), il fatto che le chiavi
private non transitano mai (`A-07` × `T-03`/`T-04` parziale), e l'indirizzamento per
contenuto dei moduli (`A-13` × `T-06`). Sono i punti in cui il design ha già chiuso
una classe intera di attacchi, ed è utile sapere quali regole non si possono
rilassare senza riaprirla.

## 5. Scenari

Formato: attore, asset colpiti, severità, stato, riferimenti. Poi l'attacco in passi
concreti, l'impatto, e la contromisura **col suo costo**.

### 5.1 `T-01` — Nodo egoista

#### TM-01 — Presenza dimostrata senza risorse fornite

**Asset:** A-02, A-06 · **Severità:** alta · **Stato:** accettato · **Rif:** [RF-005],
`wire.md` §`challenge_request`, `identity.md` §"One-time anti-Sybil proof of work"

**Attacco.** (1) L'operatore avvia un processo che detiene la chiave e nient'altro:
niente disco offerto, niente CPU disponibile. (2) Riceve `challenge_request` di tipo
`availability`, il cui `AvailabilityAssignment` chiede solo `{"response_bytes": N}`.
(3) Firma la risposta entro la finestra. (4) L'evidenza entra nel ledger e sblocca
`existence_income`. Nessuna risorsa è stata offerta a nessuno.

**Impatto.** Il "reddito di esistenza" misura la presenza di una *chiave online*, non
di un *dispositivo utile*. Preso da solo non è un attacco: è la definizione voluta —
[ADR-002] dice esplicitamente che il reddito di esistenza è "reddito di presenza
dimostrata". Diventa un attacco quando lo si moltiplica (TM-08).

**Contromisura.** Nessuna, ed è una scelta corretta: rendere costosa la presenza
significherebbe proof of work continuo, che [[PROJECT]] esclude in modo permanente.
La disposizione è **accettare** il rischio a livello di singolo nodo e contenerlo a
livello di aggregato (§7). **Condizione di revisione:** se la quota di emissione che
passa dal canale `existence_income` supera la soglia fissata da `SEC-REQ-18`.
**Costo della contromisura:** zero, perché non c'è contromisura — il costo è
interamente nel contenimento aggregato, quantificato in §7.

#### TM-02 — Storage scartato con scommessa sul campionamento

**Asset:** A-02, A-06 · **Severità:** alta · **Stato:** aperto · **Rif:** [ADR-002],
`wire.md` §`challenge_request` (`StorageAssignment`)

**Attacco.** (1) Il nodo accetta la custodia di 1.000 chunk e ne conserva 10.
(2) Continua a dichiararsi custode. (3) Quando arriva una proof-of-retrievability su
un chunk casuale, risponde correttamente con probabilità 1 % e fallisce con
probabilità 99 %. (4) Se la penalità per un fallimento è inferiore al risparmio di
990 chunk non custoditi per l'intero periodo, l'attacco è profittevole anche
fallendo quasi sempre.

**Impatto.** I compensi storage si scollegano dallo storage reale, e i dati degli
utenti non sono realmente replicati: il danno peggiore non è economico ma la perdita
silenziosa di dati quando la riparazione automatica (M-05) si fida di custodi
fantasma.

**Contromisura.** La profittabilità dipende da tre parametri che oggi non esistono:
frequenza di campionamento `q`, penalità reputazionale `p` per fallimento, e
compenso `c`. Il vincolo verificabile è che il valore atteso di un nodo che conserva
la frazione `x` dei chunk sia strettamente crescente in `x` su tutto `[0,1]` — cioè
che non esista alcuna frazione parziale conveniente. `SEC-REQ-18` e `AT-12`.
**Costo:** il campionamento è traffico e I/O ricorrenti su ogni custode, incluse le
schede SD dei telefoni; ogni aumento di `q` è una tassa su batteria e usura del
supporto. È il costo che [ADR-002] segnala già come condizione di revisione.

#### TM-03 — Compute non eseguito

**Asset:** A-02, A-06 · **Severità:** media · **Stato:** mitigato · **Rif:** [ADR-004],
`app-manifest.md` §"Capabilities"

**Attacco.** (1) Il nodo accetta task di compute e restituisce output inventati.
(2) Incassa `work_compensation` con `work_kind: "compute"`.

**Impatto.** Sarebbe grave, ma il design lo chiude: il determinismo obbligatorio
([ADR-004]) rende la ri-esecuzione a campione un confronto di byte, e
`app-manifest.md` stabilisce che le app non deterministiche **non possono** generare
ricompense di compute in v0. Un output falso è rilevabile con certezza da un solo
verificatore onesto.

**Contromisura.** Già presente. Resta da fissare la frequenza di ri-esecuzione con
lo stesso criterio di TM-02. **Costo:** la ri-esecuzione è lavoro duplicato pagato
dalla rete; è il prezzo esplicito della verificabilità, ed è la ragione per cui il
tier non deterministico esiste.

#### TM-04 — Grinding del timestamp per invalidare blocchi altrui

**Asset:** A-01, A-03 · **Severità:** media · **Stato:** aperto · **Rif:** [RF-009]

Coperto integralmente da [RF-009]: l'ordinamento obbligatorio per `tx_id` contraddice
la consecutività dei nonce, e un utente può macinare `created_at_ms` finché il
proprio `tx_id` cade dal lato che invalida il blocco. Non lo riscrivo. **Aggiunta
della vista d'insieme:** il costo per l'attaccante è quasi nullo perché in v0 non
esistono fee (`ledger.md` §"State transition order": "there are no partially applied
transactions or fees in v0"). L'assenza di fee è una scelta corretta per un token non
monetario, ma toglie l'unico deterrente automatico contro la spam di transazioni
valide-e-dannose. Va compensata con limiti di ammissione per account, non con un
prezzo. **Costo:** un contatore per account nel mempool.

#### TM-05 — Presenza a finestra su nodo mobile

**Asset:** A-02, A-06 · **Severità:** media · **Stato:** aperto · **Rif:** [ADR-002],
[RF-013]

**Attacco.** (1) L'operatore osserva che le challenge di availability arrivano con
una periodicità o da un insieme di issuer prevedibile. (2) Configura il nodo perché
sia online solo nelle finestre attese e spento il resto del tempo. (3) Matura reddito
di esistenza pieno con una frazione dell'uptime — e, su Android, senza il costo di
batteria che il progetto dice di voler rispettare.

**Impatto.** La misura di presenza cessa di misurare la presenza. Peggiora se la
selezione dell'issuer è deterministica e pubblica: diventa un calendario.

**Contromisura.** Gli intervalli devono essere imprevedibili *a priori* e non
anticipabili dal soggetto: è la stessa proprietà che [RF-013] chiede per la
`randomness`, estesa alla **schedulazione**. La formulazione verificabile è che la
distribuzione degli istanti di challenge sia indistinguibile, per il soggetto, da un
processo di Poisson di intensità nota — verificabile con un test statistico sul log
delle challenge di una devnet. `SEC-REQ-17`. **Costo:** l'imprevedibilità impedisce a
un nodo Android di programmare le proprie finestre di sveglia, e quindi costa
batteria per davvero: è un conflitto diretto con il vincolo "i nodi Android devono
rispettare batteria/dati" di [[PROJECT]]. La composizione ragionevole è una finestra
di risposta ampia (minuti, non secondi) con istante di emissione imprevedibile, così
il dispositivo può accorpare i risvegli senza poterli anticipare.

### 5.2 `T-02` — Fattoria di identità / botnet Sybil

#### TM-06 — Produzione industriale di identità

**Asset:** A-07 · **Severità:** alta · **Stato:** aperto · **Rif:** [RF-005]

Coperto da [RF-005], che quantifica il rapporto telefono/GPU a ~2.750× e mostra che
nessun valore di `difficulty_bits` nell'intervallo 18–40 imposto da `identity.md` è
insieme tollerabile su Android e costoso per l'attaccante. Non lo riscrivo. La
quantificazione completa dell'economia che ne segue è in §6.2, e le opzioni di
risposta in §7.

#### TM-07 — Precomputazione e rilascio in blocco

**Asset:** A-07 · **Severità:** alta · **Stato:** aperto · **Rif:** [RF-006]

Coperto da [RF-006]. **Aggiunta della vista d'insieme:** l'effetto peggiore non è
l'enrollment in sé ma la composizione con TM-09. Un rilascio simultaneo di decine di
migliaia di identità che iniziano tutte ad accumulare uptime nello stesso istante
produce, dopo una finestra di reputazione, un blocco compatto di candidati validatori
con uptime identico e perfetto. Se l'elezione ordina per uptime, l'attaccante non
occupa posizioni sparse: occupa **le prime N**, tutte insieme.

#### TM-08 — Flotta emulata che raccoglie il reddito di esistenza

**Asset:** A-02, A-03 (per composizione) · **Severità:** critica · **Stato:** aperto
· **Rif:** [RF-005], `wire.md` §`challenge_request`, [[PROJECT]] §"Outcomes and
success metrics"

**Attacco.** (1) L'attaccante genera N identità enrollate (TM-06). (2) Un singolo
processo su un VPS detiene tutte le N chiavi. (3) Alla ricezione di una
`challenge_request` di availability per una qualsiasi delle N, firma la risposta con
la chiave corrispondente: il costo marginale è **una firma Ed25519**, ordine dei
microsecondi. (4) Ogni identità matura `existence_income` a ogni epoca, per sempre.
(5) L'attaccante non ha mai fornito storage né compute e non è distinguibile da N
dispositivi reali con nessun controllo previsto dal protocollo v0.

**Impatto.** È il conflitto centrale del progetto. [[PROJECT]] promette *"zero
accrediti a nodi emulati nei test di attacco"*; [RF-005] dimostra che con il design
v0 questo non è ottenibile per via crittografica. Non è un difetto di
implementazione: discende da [ADR-002], che ha scartato l'attestazione hardware per
ragioni legittime. La quantificazione della quota di emissione catturabile è in §6.2;
le quattro opzioni di risposta, con i loro costi e le riformulazioni candidate della
metrica, sono in §7. **Questo scenario è il motivo per cui §7 esiste.**

**Contromisura.** Nessuna disponibile al livello crittografico. Le opzioni sono
tutte in §7 e la scelta spetta all'operatore. **Costo:** dichiarato per ciascuna
opzione in §7 — è precisamente il contenuto che la decisione richiede.

#### TM-09 — Dalla massa di identità alla cattura dell'eleggibilità

**Asset:** A-04, A-12 · **Severità:** critica · **Stato:** aperto · **Rif:**
[ADR-001], `ledger.md` §"Validator-set continuity", §"DRAFT: committee selection"

**Attacco.** (1) L'attaccante mantiene N identità emulate su infrastruttura di
datacenter (TM-08). (2) L'elezione dei validatori pesa, secondo [ADR-001],
"reputazione e uptime dimostrato". (3) Un processo su un VPS con SLA al 99,99 % ha
un uptime **strutturalmente superiore** a qualunque telefono Android reale, che si
spegne, perde rete, entra in doze e viene ucciso dal sistema operativo. (4) Ordinando
i candidati per uptime, le identità dell'attaccante occupano le prime posizioni non
per fortuna ma per costruzione. (5) A ogni rotazione l'attaccante conquista una
quota crescente del set, fino alla soglia di 1/3 che rompe la safety.

**Impatto.** È lo scenario più grave del documento, perché non richiede alcuna
violazione: l'attaccante vince la competizione con le regole scritte. **Un'elezione
pesata sull'uptime seleziona sistematicamente contro i dispositivi domestici per cui
il progetto esiste, e a favore dell'infrastruttura di datacenter che l'attaccante
usa.** L'incentivo è perverso in modo esatto, non approssimativo. Analisi
quantitativa in §6.1.

**Contromisura.** Tre leve, con costi molto diversi, dettagliate in §6.1: lotteria
su casualità finalizzata con soglia di eleggibilità (l'alternativa già prevista come
opzione in `ledger.md` §"DRAFT"); tetto di rotazione per epoca (`churn cap`);
diversificazione per posizione di rete. `SEC-REQ-13`, `AT-10`.

#### TM-10 — Eclipse di un nodo tramite popolamento della DHT

**Asset:** A-05, A-10 · **Severità:** alta · **Stato:** aperto · **Rif:** `wire.md`
§"Discovery", [RF-003]

**Attacco.** (1) L'attaccante dispone di migliaia di identità enrollate, quindi
legittimate a partecipare a tutti i protocolli Coblox (`wire.md`: "all other Coblox
protocols require a valid enrollment certificate" — l'enrollment è l'unico filtro).
(2) Popola la tabella di routing Kademlia della vittima con i propri peer, sfruttando
il fatto che i `node_id` sono generabili a piacere e quindi collocabili vicino a
qualunque punto dello spazio delle chiavi. (3) La vittima finisce circondata: tutte
le risposte `ledger_status_response` e `ledger_range_response` che riceve provengono
dall'attaccante. (4) Combinato con [RF-003], l'attaccante serve alla vittima una
storia alternativa che supera tutti e sette i passi del light client.

**Impatto.** La vittima vede un saldo, uno stato e un insieme di validatori scelti
dall'attaccante, senza alcun segnale di anomalia. È la composizione che rende
[RF-003] praticabile su bersagli scelti anziché solo su installazioni fresche.

**Contromisura.** (a) L'ancoraggio di soggettività debole e la non regressione
richiesti da [RF-003] rendono l'eclipse insufficiente da solo. (b) Diversificazione
obbligatoria dei peer per prefisso di rete nella tabella di routing, non solo per
distanza nello spazio delle chiavi. (c) Interrogazione di seed indipendenti al
riavvio, già prevista da `wire.md` ("at least three seed multiaddresses from
independent failure domains"), estesa a un controllo periodico e non solo al
bootstrap. `SEC-REQ-06`, `SEC-REQ-13`, `AT-03`. **Costo:** (b) riduce l'efficienza
del routing DHT e complica la vita ai nodi dietro CGNAT che condividono un prefisso
con estranei — proprio i nodi domestici; (c) è traffico periodico modesto. Nessuno
dei tre è gratuito ma nessuno è oneroso.

#### TM-11 — Esaurimento delle risorse dei nodi onesti

**Asset:** A-03, A-08, A-10 · **Severità:** media · **Stato:** aperto · **Rif:**
[RF-012]

Coperto da [RF-012] per il vettore della cache anti-replay. **Aggiunta della vista
d'insieme:** [RF-012] considera un nodo enrollato; con N identità l'attacco si
moltiplica per N e cambia natura, perché i limiti *per peer* che `wire.md` §"Gossip
validation and backpressure" prescrive non offrono alcuna protezione contro N peer
tutti formalmente distinti e tutti legittimamente enrollati. La difesa per peer è
strutturalmente cieca a un attaccante Sybil. Il bersaglio elettivo sono i nodi
Android, dove l'esaurimento di memoria porta alla terminazione del processo da parte
del sistema e quindi alla **perdita del reddito** del partecipante onesto: l'attacco
paga due volte, degradando i concorrenti e liberando quota di emissione.
**Contromisura:** i limiti dichiarati devono essere anche **globali** (memoria totale
della cache, connessioni totali), non solo per peer, con comportamento definito in
saturazione. `SEC-REQ-11`. **Costo:** trascurabile in calcolo; richiede però di
scegliere *chi* sacrificare in saturazione, che è una decisione di progetto da
scrivere, non da lasciare all'implementatore.

### 5.3 `T-03` — Validatore malevolo singolo

#### TM-12 — Voto con componente di torsione per spaccare la rete

**Asset:** A-03, A-05 · **Severità:** alta · **Stato:** aperto · **Rif:** [RF-001]

Coperto integralmente da [RF-001]. È il finding a costo di chiusura più basso e
conseguenza più grave dell'intera review: una riga di testo normativo evita una
spaccatura permanente della rete fra implementazioni entrambe conformi.

#### TM-13 — Censura del reddito tramite mancata emissione di challenge

**Asset:** A-06, A-12 · **Severità:** alta · **Stato:** aperto · **Rif:** `wire.md`
§`challenge_request` ("Only an assigned validator or ledger-selected auditor may
issue a challenge"), [ADR-002]

**Attacco.** (1) Il protocollo non specifica **come** un validatore venga assegnato a
un soggetto: `wire.md` dice chi *può* emettere, mai chi *deve*. (2) Un validatore
malevolo, assegnato a un insieme di soggetti, semplicemente non emette challenge
verso una vittima scelta — oppure le emette con `deadline_ms` così stretto da
renderle irraggiungibili su una connessione mobile. (3) La vittima non produce
evidenza. (4) Senza evidenza non c'è `existence_income`: `ledger.md` §"Mint" impone
che l'evidenza "MUST establish the configured availability threshold for that node
and epoch". (5) La vittima perde il reddito senza che nulla di anomalo sia
registrato: l'assenza di una challenge non lascia traccia.

**Impatto.** Un singolo validatore può azzerare il reddito di partecipanti scelti, in
modo invisibile e non contestabile. È censura economica con costo zero e rischio
zero, ed è più facile di qualunque attacco al consenso.

**Contromisura.** (a) L'assegnazione issuer→soggetto deriva dalla casualità
finalizzata ed è **ricalcolabile da chiunque**, così l'assenza di una challenge dovuta
diventa un fatto osservabile e non un non-evento. (b) Ogni soggetto è coperto da
almeno due issuer indipendenti per epoca, così un singolo censore non basta. (c) Un
soggetto può registrare una challenge dovuta e non ricevuta (`missing_challenge`) come
evidenza contestabile. `SEC-REQ-17`, `AT-12`. **Costo:** (a) è gratuito se la
casualità finalizzata esiste già per [RF-013]; (b) raddoppia il traffico di
challenge, e quindi la batteria sui nodi mobili — è il costo vero; (c) aggiunge una
classe di evidenza e la logica per arbitrarla, cioè lavoro di consenso reale in M-03.

#### TM-14 — Collusione fra emittente e soggetto della challenge

**Asset:** A-02, A-06 · **Severità:** alta · **Stato:** aperto · **Rif:** [RF-013]

Coperto integralmente da [RF-013]. È la condizione di revisione già scritta in
[ADR-002] ("emergono attacchi di collusione tra sfidanti e sfidati"): il threat model
la conferma come realizzata, non ipotetica.

#### TM-15 — Censura delle transazioni da parte del proposer

**Asset:** A-10, A-12 · **Severità:** media · **Stato:** aperto · **Rif:**
`ledger.md` §"Block format", §"State transition order"

**Attacco.** (1) Un validatore, nel proprio turno di proposta, omette
sistematicamente le transazioni di un account bersaglio. (2) Non esistono fee, quindi
la vittima non può "pagare di più" per essere inclusa; non esiste alcun meccanismo di
inclusione forzata né alcun timeout normativo. (3) La vittima non può bruciare token
per abbonarsi o pubblicare, e le sue evidenze non entrano nel ledger.

**Impatto.** Esclusione di fatto dalla rete, limitata alla frazione di turni
controllata dall'attaccante. Con un set di 20 e un attaccante che ne controlla uno,
il 5 % delle proposte è censurato — fastidio; con un cartello (TM-17) diventa
esclusione totale.

**Contromisura.** Regola di inclusione: una transazione valida, vista da un validatore
onesto e non ancora scaduta, **deve** essere inclusa entro K blocchi o il proposer
successivo la include d'ufficio; la violazione ripetuta è evidenza per lo slashing
reputazionale previsto da [ADR-001]. `SEC-REQ-13`. **Costo:** richiede che i
validatori concordino su cosa "hanno visto", il che è un problema noto e non banale
(la censura è indistinguibile dal ritardo di rete). La versione onesta e a basso
costo è più debole: rendere **osservabile** la statistica di inclusione per proposer
e pubblicarla, lasciando la sanzione alla reputazione. Raccomando la seconda per v0 e
lo dichiaro come mitigazione parziale, non come soluzione.

#### TM-16 — Certificato di quorum sotto la soglia di safety

**Asset:** A-01, A-03 · **Severità:** alta · **Stato:** aperto · **Rif:** [RF-002]

Coperto integralmente da [RF-002], con il controesempio numerico a `V=101`. Nella
vista d'insieme conta perché **abbassa la soglia di TM-17**: con la formulazione di
`identity.md` un cartello non ha bisogno di 1/3 del potere di voto, gli basta la
frazione che la formula sbagliata concede.

<!-- CONT -->

