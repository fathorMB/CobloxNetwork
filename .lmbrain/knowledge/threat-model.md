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
| **A-02** emissione | TM-01, TM-02, TM-03, TM-05 | TM-08 | TM-14 | TM-20 | TM-22 | TM-37 | TM-35 |
| **A-03** consenso | TM-04 | TM-11 | TM-12, TM-16 | TM-17, TM-21 | n/a — nessun percorso dal manifest al voto di finalità | TM-31 | TM-19 |
| **A-04** set validatori | n/a — un singolo nodo egoista non altera la composizione del set | TM-09 | n/a — un validatore singolo non decide il set successivo, che richiede quorum | TM-18 | n/a — nessun percorso dalla pubblicazione all'elezione | n/a — nessun potere sulla composizione | TM-19 |
| **A-05** verità all'utente | n/a — un nodo egoista non serve prove: non ha vantaggio a falsificare ciò che l'utente vede | TM-10 | TM-12 | TM-21 | n/a — il manifest non alimenta la verifica del saldo | TM-31 | TM-36 |
| **A-06** challenge | TM-02, TM-05 | TM-08 | TM-13, TM-14 | TM-19 | n/a — il publisher non emette né valuta challenge | TM-30, TM-37 | TM-19 |
| **A-07** identità e chiavi | n/a — l'egoista abusa della propria identità, non ne attacca altre | TM-06, TM-07 | n/a — nessun percorso verso la chiave privata di un altro nodo: le chiavi non transitano mai (`identity.md` §"Key hierarchy") | TM-21 | n/a — nessun percorso verso chiavi altrui | TM-28 (legame identità ↔ persona, non furto di chiave) | TM-35 |
| **A-08** risorse dell'operatore | n/a — l'egoista risparmia le proprie risorse, non consuma quelle altrui | TM-11 | n/a — un validatore non assegna carico agli host: lo fa la politica di assegnazione | TM-19 | TM-23, TM-24, TM-25 | n/a — l'osservatore non impone carico oltre il traffico ordinario | TM-33 |
| **A-09** sandbox | n/a — l'egoista non pubblica moduli | n/a — moltiplicare identità non aggiunge alcun percorso verso il runtime | n/a — nessun percorso dal ruolo di validatore al runtime di un host | n/a — come sopra; il cartello agisce sul ledger, non sul runtime | TM-23, TM-27 | n/a — nessun percorso verso il runtime | n/a — l'insider agisce su parametri e liste, non sul runtime |
| **A-10** disponibilità | n/a — l'egoista vuole la rete viva, ci guadagna | TM-10, TM-11 | TM-15 | TM-17 | TM-24 | TM-31, TM-32 | TM-33 |
| **A-11** privacy | n/a — non ha accesso a dati altrui oltre quelli pubblici | TM-29 (con molte identità osservatrici) | TM-30 | TM-29 | TM-26 | TM-28, TM-29, TM-30, TM-37 | TM-34 |
| **A-12** accesso | n/a — nessun potere di esclusione | TM-09 | TM-13, TM-15 | TM-18, TM-19 | n/a — il publisher non esclude altri publisher | TM-31 | TM-33, TM-34, TM-35 |
| **A-13** catalogo e moduli | n/a — nessun percorso di scrittura sul catalogo | n/a — il contenuto è indirizzato per hash e firmato: moltiplicare identità non aiuta | n/a — un validatore singolo non altera un record di catalogo finalizzato | TM-20 | TM-25 | n/a — il trasporto non è fidato per costruzione (`app-manifest.md` §"Deterministic container") | TM-33, TM-36 |

Delle 91 celle, 60 hanno almeno uno scenario e 31 sono `n/a` con motivo. La cella `A-02` × `T-06` era fra le `n/a` fino a [REVIEW-021] — «l'emissione non dipende da dati osservabili sul filo» — e TM-37 la falsifica: chi detiene la chiave di trasporto di un nodo non osserva soltanto, ne occupa il posto e ne fa scadere le challenge, che è emissione mancata. È la famiglia 2: un'`n/a` scritta prima della regola che l'ha resa falsa. Quattro fra
queste ultime lo sono per una ragione che vale la pena isolare, perché sono
**proprietà di design conquistate** e non coincidenze: il
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

**Contromisura.** La scelta della primitiva e del contenimento è la decisione di §7;
`SEC-REQ-12` impone in ogni caso la dichiarazione del limite residuo. **Costo:**
dichiarato per opzione in §7.7.

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

**Asset:** A-04, A-12 · **Severità:** critica · **Stato:** **mitigato in specifica**
da [SPEC-006] (2026-08-25) · **Rif:** [ADR-001], [ADR-007], `ledger.md`
§"Validator-set continuity", §"Validator election and rotation"

> **Aggiornamento 2026-08-25.** Il vettore descritto sotto — uptime da datacenter che
> vince la classifica — è chiuso alla radice: nella regola scritta da [SPEC-006]
> l'evidenza di tipo `availability` contribuisce **zero** al `contribution_score`, e
> l'eleggibilità è una soglia binaria su storage e compute dimostrati, non una
> classifica. Un'infrastruttura con SLA non ottiene alcun vantaggio dal solo essere
> accesa. Resta il residuo di numerosità: `N` identità che forniscano ciascuna lavoro
> reale sopra la soglia hanno `N` biglietti, ed è il residuo governato da `alpha` in
> [ADR-007], limitato in velocità dal tetto di ingressi per epoca.
>
> **Residuo aggiunto dopo [REVIEW-010] ([RF-004]).** «Lavoro che un Sybil non può
> falsificare a costo nullo» è vero come tendenza e falso come assoluto, e la
> differenza è misurabile. Il `contribution_score` somma evidenze
> `challenge_evidence` con esito `passed`, e `ledger.md` §"Challenge evidence"
> dichiara già che un emittente colluso che consegni il proprio segreto impegnato
> consente a un proponente di enumerare i `timestamp_ms` legali — `10^3`–`10^6`
> valori, un SHA-256 ciascuno — finché il beacon accoppia quell'emittente al
> soggetto voluto e seleziona un frammento che il soggetto conserva davvero. La
> mitigazione dichiarata lì, la copertura a due emittenti di `wire.md`, **non
> trasferisce**: degradare a "superarne una su due" funziona per un *tasso di
> rilevamento*, dove il fallimento pesa, e non per una **somma di successi**, che
> non sottrae nulla. [SPEC-006] risponde con la condizione di eleggibilità 4 —
> almeno `validator_eligibility_min_issuers` emittenti distinti — che **alza il
> prezzo e non chiude il residuo**: il costo di falsificare l'eleggibilità è
> l'enrollment di `validator_eligibility_min_issuers` identità colluse per ogni
> candidato fabbricato, più la macinatura. È ancora un costo una tantum contro un
> flusso perpetuo, cioè di nuovo l'affermazione strutturale di [ADR-007].

**Attacco.** (1) L'attaccante mantiene N identità emulate su infrastruttura di
datacenter (TM-08). (2) L'elezione dei validatori pesa, secondo [ADR-001],
"reputazione e uptime dimostrato" — formulazione anteriore ad [ADR-007] e superata
su questo punto. (3) Un processo su un VPS con SLA al 99,99 % ha
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

**Contromisura.** `SEC-REQ-01`: una sola equazione di verifica dichiarata nei
documenti, con i 12 vettori di conformità come fixture (`AT-02`). **Costo:** nullo a
runtime — è una scelta di libreria più una tabella di vettori.

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

**Contromisura.** `SEC-REQ-17`: impegno dell'emittente pubblicato prima della
selezione del soggetto, rivelazione nell'evidenza, e ricalcolo della `randomness` in
validazione (`AT-12` parte A). **Costo:** reale — uno schema commit-reveal e un
impegno on-chain per emittente per round, cioè lavoro di M-03 sul motore delle
challenge; qui va fissato il **formato**, perché dopo sarebbe una modifica
incompatibile.

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

**Contromisura.** `SEC-REQ-02`: un'unica formula intera `3 × potere_firmatario >
2 × potere_totale`, identica nei tre punti in cui la soglia compare, con fixture al
confine (`AT-01`). **Costo:** nullo — è una riga di testo e un test.

### 5.4 `T-04` — Cartello di validatori

#### TM-17 — Fork deliberato o stallo con un terzo del potere di voto

**Asset:** A-01, A-03, A-10 · **Severità:** critica · **Stato:** aperto (dipende da
[RF-002]) · **Rif:** [RF-002], `ledger.md` §"What validators sign"

**Attacco.** (1) Un insieme coordinato accumula potere di voto. (2) Con ≥ 1/3 del
potere può **fermare** la rete rifiutando di votare: nessun quorum, nessuna finalità,
nessun accredito per nessuno. (3) Con ≥ 1/3 e la soglia difettosa di [RF-002] può
**equivocare**: far raggiungere il quorum a due blocchi in conflitto alla stessa
altezza, che risultano entrambi validi. (4) Due metà della rete finalizzano storie
diverse e i saldi divergono in modo permanente.

**Impatto.** Perdita simultanea di A-01 e A-03. La riparazione richiede un intervento
fuori protocollo, cioè esattamente il "coordinatore centrale" che [ADR-001] ha
scartato.

**Contromisura.** La soglia corretta ([RF-002], `SEC-REQ-02`) riporta la barra al
canonico 1/3, che è il massimo ottenibile in BFT — la resistenza a 1/3 non è
migliorabile, si può solo rendere costoso arrivarci (TM-18) e osservabile
l'equivocazione. Le firme di finalità sono su `"coblox-block-vote-v0\0" || … ||
u64be(height) || u64be(round) || block_id`: due firme dello stesso validatore su
altezza e round identici e `block_id` diversi sono una **prova crittografica di
equivocazione**, autocontenuta e verificabile da chiunque. Il protocollo v0 non la
raccoglie né la sanziona. `SEC-REQ-13` la introduce come transazione di evidenza.
**Costo:** una nuova classe di transazione più la regola di sanzione; il costo vero è
politico, perché lo slashing reputazionale di [ADR-001] deve avere una conseguenza
reale per non essere decorativo, e in una rete senza stake vincolato l'unica
conseguenza possibile è l'esclusione dal set — che è anche l'arma di TM-33.

#### TM-18 — Auto-perpetuazione del set di validatori

**Asset:** A-04, A-12 · **Severità:** critica · **Stato:** **mitigato in specifica**
da [SPEC-006] (2026-08-25), dopo la chiusura di [RF-001] e [RF-002] di [REVIEW-010];
resta aperta la parte di sanzione, vedi sotto · **Rif:**
`ledger.md` §"Validator-set continuity", §"Validator election and rotation",
[ADR-001], [ADR-007], [ADR-008], [DEBT-005]

> **Aggiornamento 2026-08-25.** Lo scenario descritto sotto era il comportamento del
> protocollo v0 *come scritto allora*. [SPEC-006] ha scritto la regola di elezione in
> `ledger.md` §"Validator election and rotation" e la frase citata al punto (1) non
> esiste più. Lo scenario è conservato integralmente perché resta la descrizione
> corretta di che cosa accade in assenza della regola, ed è il criterio con cui
> giudicare qualunque futura eccezione che riaprisse quello spazio. Che cosa è
> chiuso e che cosa no è nella contromisura in fondo alla scheda.

**Attacco.** Non serve alcun attacco: è il comportamento del protocollo v0 come
scritto. (1) `ledger.md` stabilisce che a ogni altezza il set attivo deve coincidere
con quello impegnato come `next_validator_set_hash` dall'altezza precedente, e chiude
la sezione con la frase decisiva: *"This continuity rule specifies safe
authentication but not how members are elected or rotated."* (2) Il set corrente è
quindi l'unico soggetto che scrive il set successivo, e **nessuna regola di protocollo
vincola cosa può scriverci**. (3) Un cartello che raggiunga il quorum in un qualsiasi
momento può impegnare un set successivo composto interamente da sé stesso. (4) Da
quel momento la rotazione promessa da [ADR-001] non avviene più, e nessun light
client può accorgersene: la continuità è formalmente valida a ogni passo.

**Impatto.** Il perimetro di fiducia della rete diventa **permanente e chiuso**, con
una transizione irreversibile e senza segnale. È il fallimento della promessa
centrale di [ADR-001] ("nessun validatore è permanente"), e la sua gravità è
aumentata dal fatto che sia una proprietà del design v0 e non un bug.

**Contromisura.** La regola di elezione non è un dettaglio di M-07: **è un invariante
di consenso e va scritta prima che una devnet abbia una storia**. Elementi minimi
verificabili: (a) l'insieme dei candidati eleggibili è calcolabile da chiunque a
partire dallo stato finalizzato; (b) la selezione è una funzione deterministica di
casualità finalizzata sui candidati eleggibili, non una scelta libera del set
uscente; (c) un tetto alla frazione di potere di voto sostituibile per rotazione;
(d) il light client verifica che il `next_validator_set_hash` sia **quello che la
funzione produce**, non semplicemente uno firmato dal quorum. `SEC-REQ-13`, `AT-10`.
**Costo:** (d) è il punto caro — obbliga il light client a ricalcolare la selezione, e
quindi a conoscere lo stato di eleggibilità, che oggi non è nel `BlockHeader`. La
versione economica è impegnare nel header l'hash dell'insieme dei candidati
eleggibili e del seme, così il ricalcolo resta possibile per chi lo vuole fare e
almeno la manipolazione è **dimostrabile a posteriori**. Raccomando la versione
economica per v0 e lo dichiaro come mitigazione parziale.

**Esito in [SPEC-006].** I quattro elementi minimi sono stati adottati, e il punto
(d) — quello caro — è stato risolto meglio della "versione economica" qui
raccomandata, ma non completamente. La regola è divisa in due strati:

- *strato 1, forma e ricambio*: limite di mandato (`seated_since_epoch` per voce e
  `validator_max_consecutive_terms`), potere di voto uniforme, tetto di ingressi per
  epoca, e il vincolo che `next_validator_set_hash` sia uguale a `validator_set_hash`
  a ogni altezza che non sia un confine di epoca o una transizione forzata da revoca.
  Tutto questo strato è verificabile da un light client **senza vedere transazioni**,
  perché è funzione dei soli documenti `ValidatorSet` che già scarica. È lo strato che
  rende l'auto-perpetuazione **impossibile** e non solo improbabile, e non dipende
  dalla qualità della casualità;
- *strato 2, composizione*: chi riempie i seggi liberati, derivato da
  `candidate_root` (albero di Merkle ordinato sugli `account_key` eleggibili) e da
  `election_seed`. Impegnato nel header **transitivamente**, perché `ElectionRecord`
  fa parte di `ValidatorSet` e quindi di `next_validator_set_hash`: nessun campo nuovo
  di `BlockHeader`, ricalcolo a posteriori possibile. Questo strato è verificabile per
  intero solo da chi rigioca le transazioni finalizzate.

**Le due porte che [REVIEW-010] ha trovato ancora aperte, e come sono state
chiuse.** La prima consegna di [SPEC-006] chiudeva l'ingresso e lasciava passare
`TM-18` da altre due strade, entrambe senza violare alcuna regola e con la catena
formalmente valida a ogni altezza. Sono registrate qui perché sono `TM-18` a tutti
gli effetti, non varianti minori:

- **porta del documento dei parametri.** Le grandezze dell'elezione vivono nel
  documento `consensus_parameters`, che è firmato dal quorum, cioè dal set in
  carica. Il blocco di vincoli originario legava i parametri **fra loro** e non
  nelle **magnitudini**: un `election_epoch_blocks` di `2^60` e un
  `validator_max_consecutive_terms` di `2^60` soddisfano ogni relazione,
  e da quel momento nessun confine arriva, nessun mandato scade, e la regola di
  confine impone al set di **non cambiare** a ogni altezza. Il light client non
  vedeva un'irregolarità: la applicava. *Chiusa* con l'oggetto `ElectionBounds`
  del trust anchor di genesi — tetti e pavimenti di magnitudine più una variazione
  massima per `sequence`, fuori dalla governance della catena — e con il passo 5
  dell'algoritmo del light client che dichiara la provenienza dei parametri e
  fallisce chiuso;
- **porta della contrazione.** Il tetto di rotazione limitava le **ammissioni** e
  nulla limitava le **uscite**. Una coalizione con `k > V/3` seggi, cioè sotto la
  soglia di safety BFT, poteva far finalizzare le proprie candidature e poi negare
  il quorum a ogni blocco che ne contenesse altre: al confine la derivazione
  onesta produce `R = C =` la coalizione, `fills = 0` — **sotto il tetto, non al
  tetto** — e la coalizione detiene il 100 % del potere senza aver ammesso
  nessuno, in **un solo confine e senza segnale**.
  *Chiusa nella parte che conta* con il **pavimento di contrazione**
  `3 * member_count(nuovo) > 2 * member_count(precedente)`, la stessa forma del
  predicato di quorum applicata ai seggi invece che al potere, valida sia ai
  confini sia nelle transizioni di sola rimozione. La coalizione a `k` appena
  sopra `V/3` ora produce un set **invalido** e la catena si ferma; contrarre fino
  a sé stessi in **un solo passo** resta possibile solo a chi detiene già più dei
  due terzi, dove la safety BFT è comunque già persa.

  **Che cosa il pavimento non compra, verificato da [REVIEW-010] al secondo giro
  ([RF-008]).** Il pavimento rifiuta la censura *totale* e non quella
  *selettiva*. Una coalizione a `k > V/3` lascia passare esattamente le
  candidature oneste che portano il set al minimo consentito e ripete:
  `V → 2V/3 → 4V/9 → … → k`, cioè `ceil(log(V/k)/log(3/2))` confini, che per `k`
  vicino a `V/3` sono **tre**. I nodi onesti firmano ognuno di quei blocchi
  perché ognuno è valido: la derivazione è deterministica e le candidature
  censurate non sono mai state finalizzate. **La soglia effettiva di cattura
  resta quindi poco sopra `1/3`** — del pavimento **preso da solo**, che è lo stato
  descritto qui. Il vincolo relazionale `3 · min_set >= 2 · V` di [ADR-010] la alza
  poi a circa **`4V/9`**, e non a due terzi: la correzione, con la misura e la
  ragione per cui la cifra dei due terzi è stata confutata tre volte, è nella nota
  di `AT-10` ([REVIEW-014] RF-001). Questa riga resta scritta al suo tempo e non va
  letta come lo stato corrente. Ciò che il pavimento compra, e va rivendicato
  per quello che è, è la conversione di un confine invisibile in **tre confini,
  ognuno dei quali pubblica la propria contrazione in un documento firmato che un
  light client sa confrontare** — lo stesso standard del tetto di ingressi, non
  uno più forte. Con un'asimmetria da dichiarare: l'orizzonte per **ammissione**
  è tarabile con `validator_min_capture_epochs`, quello per **attrito** è fisso e
  nessun parametro lo muove, e la sicurezza di una regola è quella del suo
  percorso più debole.

**Una terza interazione, trovata al secondo giro di [REVIEW-010] ([RF-007]) e
chiusa.** Limite di mandato, tetto di ingressi e pavimento di contrazione erano
congiuntamente insoddisfacibili su **ogni rete conforme**, non in un caso limite:
il set di genesi porta mandati sincroni, quindi al confine `e = T` scadono tutti
insieme, `R = ∅`, il nuovo set vale al più `c`, e il pavimento pretende `3c > 2V`
mentre il vincolo di cattura pretende `3cm <= V`, intervallo vuoto per ogni `V`.
Arresto certo all'altezza `T * election_epoch_blocks`. *Chiusa* alla causa e non
al sintomo: **scaglionamento dei mandati nel set di genesi** — al più
`validator_churn_cap_seats` voci per ogni `term_expiry_epoch`, valori in
`[1, T]` — più il vincolo `3c < V` nel blocco di validità, e la scadenza diventa
un campo `term_expiry_epoch` **timbrato all'insediamento** invece che ricalcolato,
il che chiude come effetto collaterale l'estensione retroattiva dei mandati in
carica da parte di un quorum che alzi `T`. L'esenzione del pavimento quando
`R = ∅` è stata **rifiutata**: è fabbricabile da chi censura, cioè la stessa
obiezione già usata contro la continuazione del set e contro la sospensione
dell'elezione.

**Corollario trovato al terzo giro ([RF-012]) e chiuso.** Lo scaglionamento si
automantiene solo a `T` costante o crescente: i timbri sono `e + T(e)`, quindi due
confini distinti collidono **se e solo se `T` diminuisce**, e una collisione mette
più coorti sullo stesso confine, che è esattamente ciò per cui `3c < V` non è
dimensionato. Accorciare i mandati su indicazione del simulatore — l'atto di
governance più ordinario che esista, **senza alcun avversario** — fermava la
catena, e un pool pieno non salvava, perché a limitare la ricostruzione è il tetto
di ingressi. *Chiuso* con il vincolo di **monotonia**: `T_new >= T_active` in
accettazione, quindi su catena viva il limite di mandato non decresce mai.
Alzarlo resta libero e gratuito proprio grazie al timbro. Il costo è dichiarato:
è una porta a senso unico su una grandezza rilevante per la sicurezza, e una rete
che parta con mandati troppo lunghi non può correggerli se non per la via fuori
banda riservata agli stalli.

**Che cosa resta aperto**, e va tenuto aperto: (i) un light client non può stabilire
che `candidate_root` contenga tutti gli eleggibili, né che ogni candidato impegnato
avesse davvero la soglia di contributo, né che i seggi siano andati ai biglietti più
bassi — falsificabile in modo compatto nei casi (a) e (c) di
`ledger.md` §"What a light client can establish about set composition", **non**
falsificabile in modo compatto nel caso (b), che richiede il replay; (ii) il seme è
derivato dagli ID di blocco della finestra di entropia ed è quindi *macinabile* da chi
propone l'ultimo blocco della finestra — il vantaggio è un best-of-`G`, ed è limitato
superiormente dal tetto di ingressi, non dalla qualità del seme; (iii) la sanzione
dell'equivocazione, seconda metà di `SEC-REQ-13`, non è toccata da [SPEC-006] e resta
lavoro di M-07; (iv) l'esclusione per **non finalizzazione** di una candidatura non è
rilevabile da nessun verificatore, full node compreso, perché una candidatura
censurata e una mai inviata sono la stessa assenza di transazione — è limitata nei
suoi effetti dal tetto e dal pavimento, non osservabile; (v) una contrazione lecita e
una cattura per attrito sono indistinguibili per un light client, che ne verifica il
pavimento ma non la causa. **Lo stato "mitigato in specifica" è condizionato
all'esistenza di `ElectionBounds` in ogni distribuzione firmata**: una rete che non
ne pubblichi non è conforme, e su di essa `TM-18` è integralmente aperto.

#### TM-19 — Governance dei parametri come strumento di vantaggio

**Asset:** A-03, A-04, A-06, A-08 · **Severità:** alta · **Stato:** aperto ·
**Attore:** `T-04` e `T-07` · **Rif:** [ADR-005], [ADR-006], `identity.md`
§"One-time anti-Sybil proof of work", `ledger.md` §"Burn", `app-manifest.md`
§"Pricing"

**Attacco.** I parametri firmati che il quorum controlla non hanno, tranne uno,
alcun limite normativo. (1) Il cartello abbassa `difficulty_bits` al minimo mentre
prepara le proprie identità, poi lo rialza: si è concesso un ingresso a sconto e ha
chiuso la porta dietro di sé. (2) Modifica la `policy_hash` per aumentare il compenso
delle categorie di lavoro che fornisce e ridurre le altre. (3) Modifica la rate card
di hosting per rendere antieconomico l'hosting sui nodi piccoli. (4) Modifica la
frequenza di campionamento delle challenge portandola a un livello che scarica i
telefoni degli avversari. Nessuno di questi passi viola una regola scritta.

**Impatto.** Controllo economico completo senza mai rompere il consenso. L'unico
limite oggi esistente è l'intervallo 18–40 di `difficulty_bits`, che è anche l'unico
parametro con un bound dichiarato — e §7 mostra che quel bound è legato alla
primitiva scelta e va rifatto se la primitiva cambia.

**Contromisura.** (a) Ogni parametro governabile ha un **intervallo ammissibile
firmato alla genesi** e una **variazione massima per epoca**; un valore fuori
intervallo invalida il blocco come qualunque altra regola. (b) Ogni modifica di
parametro è annunciata con un ritardo di attivazione (`timelock`) non inferiore a una
finestra dichiarata, così i partecipanti possono osservarla prima che abbia effetto.
(c) I parametri e i documenti firmati (`policy_hash`, rate card) sono pubblicamente
reperibili — che è anche il presupposto di auditabilità di [RF-008]. `SEC-REQ-14`,
`SEC-REQ-04`. **Costo:** il timelock rende la rete lenta a reagire a un problema
reale, e questo è un costo operativo autentico: se la difficoltà di enrollment va
alzata d'urgenza per un attacco in corso, il timelock è dalla parte dell'attaccante.
La composizione onesta è un timelock lungo per i parametri economici e una corsia
d'emergenza esplicita, dichiarata, e limitata a innalzare i costi mai ad abbassarli.

#### TM-20 — Emissione non auditabile fuori dal set di validatori

**Asset:** A-01, A-02, A-13 · **Severità:** alta · **Stato:** aperto · **Rif:**
[RF-008]

Coperto da [RF-008] per la parte di preimmagini non definite. **Aggiunta della vista
d'insieme:** il punto non è solo la divergenza fra implementazioni, è che
l'auditabilità dell'emissione — la promessa "verificabile in tempo reale" di
[[PROJECT]] — **si riduce a fiducia nei validatori** finché la preimmagine di
`policy_hash` non è definita e il documento firmato non è pubblicamente reperibile.
`ledger.md` §"Mint" fonda l'integrità dell'emissione su "validators recompute it": se
solo i validatori possono ricalcolare, un cartello può emettere qualunque importo e
nessun utente può dimostrare che sia sbagliato. È il complemento esatto di TM-18: uno
rende il potere permanente, l'altro lo rende inosservabile.

#### TM-21 — Cartello storico e falsificazione della storia

**Asset:** A-03, A-05, A-07 · **Severità:** alta · **Stato:** aperto · **Rif:**
[RF-003], `identity.md` §"Revocation and key replacement"

Coperto da [RF-003]. **Aggiunta della vista d'insieme:** l'attore non è il cartello
attuale ma quello *passato*, e questa è la ragione per cui l'attacco è a buon
mercato. In una rete senza stake vincolato, le chiavi di consenso di un set ormai
ruotato fuori non proteggono più nulla per il loro titolare: cederle non gli costa
niente, e `identity.md` garantisce esplicitamente che *"historical signatures remain
valid at heights before the effective height"*. Il progetto deve trattare le chiavi
di consenso dismesse come **materiale pericoloso da distruggere**, non come chiavi
scadute innocue — ed è un requisito di procedura operativa, non solo di protocollo.

**Contromisura.** `SEC-REQ-06` (ancoraggio di soggettività debole e non regressione,
verificati da `AT-03`) più l'obbligo procedurale di distruzione delle chiavi di
consenso all'uscita dal set. **Costo:** il canale di distribuzione degli ancoraggi,
con la superficie che esso stesso apre — vedi TM-36 e `SEC-REQ-23`.

### 5.5 `T-05` — Publisher ostile

#### TM-22 — Abbonati fittizi per lucrare la quota al creatore

**Asset:** A-02 · **Severità:** critica · **Stato:** aperto · **Rif:** [RF-007],
[ADR-006] §"Ricompensa al creatore", `ledger.md` §"Mint"

Il meccanismo di base è coperto da [RF-007], che propone il vincolo di consenso
`amount ≤ k × Σ(burn di abbonamento)` con `k < 1`. La spec [SPEC-004] chiede
esplicitamente l'analisi economica di questa superficie: è in §6.3, e mostra che il
vincolo `k` da solo **non chiude l'attacco**, perché il reddito di esistenza delle
identità fittizie finanzia la perdita. §6.3 quantifica la relazione fra prezzo di
abbonamento, quota al creatore e reddito di esistenza, e mostra che il vincolo che
rende l'attacco insostenibile rende anche l'abbonamento **inaccessibile all'utente
onesto con un solo dispositivo** — una contraddizione di prodotto che va decisa, non
tarata.

**Contromisura.** `SEC-REQ-15` (vincolo `k` imposto in consenso, `AT-11`) chiude la
stampa; per il residuo di reputazione, le tre risposte di §6.3 con i rispettivi
costi, e `SEC-REQ-16` che obbliga il simulatore a misurare esplicitamente il margine.
**Costo:** la ponderazione per contributo dimostrato — la risposta che raccomando — è
un campo in più nella foglia di abbonamento e una funzione di ricompensa più ricca,
cioè lavoro di consenso in M-02.

#### TM-23 — Auto-consenso alle capability sugli host assegnati d'ufficio

**Asset:** A-08, A-09 · **Severità:** alta · **Stato:** aperto · **Rif:** [RF-015],
[ADR-006] §"Consenso dell'ospite", `app-manifest.md` §"Installation and execution
verification" passo 7

Coperto integralmente da [RF-015]: l'assegnazione degli host da parte del protocollo
è incompatibile con un grant interattivo dell'operatore, perché su un daemon headless
o su un telefono in background non c'è nessuno davanti allo schermo, e l'unica
implementazione possibile diventa la concessione automatica.

#### TM-24 — Abuso dei tetti di risorsa dell'host

**Asset:** A-08, A-10 · **Severità:** media · **Stato:** aperto · **Rif:**
`app-manifest.md` §"Resource limits", §"Manifest schema"

**Attacco.** (1) Il publisher dichiara nel manifest tetti prossimi ai massimi di
protocollo — 4 GiB di memoria, 1 TiB di storage persistente, 1.024 invocazioni
concorrenti, 300 s di wall time — e `desired_replicas` fino a 1.024. (2) Il
protocollo assegna l'app a host idonei. (3) Gli host che avevano dichiarato una
capacità inferiore rifiutano, come `app-manifest.md` impone ("v0 hosts MUST
reject—not silently clamp"). (4) L'app resta `pending` e il protocollo continua a
cercare host, oppure trova i pochi nodi grandi e vi si concentra. (5) Su quei nodi il
carico è al massimo consentito e legittimo.

**Impatto.** Duplice. Sui nodi grandi, consumo massimo con costo interamente
sull'operatore. Sulla rete, una concentrazione sui pochi nodi capienti che riproduce
esattamente la centralizzazione che [ADR-006] ha evitato scartando la scelta degli
host da parte del publisher. Non è un bypass della sandbox: è la sandbox usata al
suo limite dichiarato.

**Contromisura.** (a) I massimi di protocollo di `app-manifest.md` sono ceiling
tecnici e vanno affiancati da **tetti di politica di deployment** molto più bassi,
firmati dalla governance e verificati alla pubblicazione — il documento stesso lo
suggerisce ("Deployment policy SHOULD set much lower defaults"), e va reso `MUST`.
(b) La politica di accettazione dell'host di [RF-015] deve essere anche una politica
di *capacità*, così il rifiuto è automatico e non lascia l'app in `pending`
indefinito. `SEC-REQ-19`. **Costo:** limita ciò che un'app legittima può chiedere e
imporrà a qualche caso d'uso reale di frammentarsi su più repliche; è il prezzo di
non concentrare la rete su pochi nodi.

#### TM-25 — Contenuto illecito ospitato su macchine di terzi

**Asset:** A-08, A-13 · **Severità:** alta · **Stato:** aperto · **Rif:** [ADR-006]
§"Consenso dell'ospite", `app-manifest.md` §"Overview"

**Attacco.** (1) Il publisher pubblica un modulo WASM il cui comportamento illecito
non è dichiarato in alcun campo del manifest: `name`, `version` e `description` sono
stringhe libere e nessun campo descrive **cosa l'app fa**. (2) Il modulo è
indirizzato per hash e opaco: l'host riceve byte, non semantica. (3) Il protocollo lo
assegna a host che non l'hanno scelto. (4) L'operatore del nodo ospita, sulla propria
macchina domestica e sotto la propria connessione, contenuto o servizi di cui non ha
conoscenza e per cui può rispondere legalmente.

**Impatto.** [ADR-006] è esplicito nel dire che la sandbox copre la sicurezza tecnica
"non quella legale o reputazionale di chi presta la propria macchina". Il threat
model conferma che il rischio è **interamente scaricato sull'hoster** e che gli
strumenti previsti per difenderlo — lista di rifiuto per nodo, lista di blocco di
rete — sono entrambi **reattivi**: agiscono dopo che il contenuto è già stato
ospitato. Per una rete di volontari con hardware domestico questo è, a mio giudizio,
il rischio di prodotto più sottovalutato dopo il Sybil.

**Contromisura.** Nessuna contromisura tecnica è possibile — il problema non è
tecnico. Le leve realistiche sono di prodotto: (a) l'hoster deve poter vedere e
dichiarare in anticipo **categorie** di ciò che accetta, e il manifest deve portare
una dichiarazione di categoria firmata dal publisher, non verificabile ma
**attribuibile** (una dichiarazione falsa è un abuso conclamato e quindi materia per
la lista di blocco); (b) trasparenza: l'operatore deve poter vedere, in ogni momento,
quali `app_id` ospita e con quali capability; (c) reversibilità immediata e senza
penalità del rifiuto. `SEC-REQ-19`, `SEC-REQ-21`. **Costo:** (a) è un campo in più e
una promessa che non si può far rispettare tecnicamente — va comunicata come tale;
(c) sposta il costo del rifiuto sul publisher, il che riapre TM-26.

#### TM-26 — Profilazione degli abbonati da parte del publisher

**Asset:** A-11 · **Severità:** media · **Stato:** aperto · **Rif:** `ledger.md`
§"Mint" (`active_subscription_root`), §"Burn"

**Attacco.** (1) Il meccanismo della quota al creatore richiede, per costruzione, che
i validatori raggruppino i burn di abbonamento **per `payer_node_id`**. (2) Quei burn
sono transazioni finalizzate e pubbliche. (3) Il publisher — e chiunque altro — legge
dal ledger l'elenco nominativo e permanente dei `node_id` abbonati a ciascuna app,
con le date. (4) Incrociando con TM-28 (legame `node_id` ↔ IP) ottiene un profilo di
consumo associabile a una persona e a una posizione.

**Impatto.** Non è un abuso del sistema: è il funzionamento del sistema. La
ricompensa al creatore **richiede** di contare gli abbonati per identità, e contare
per identità su un ledger pubblico significa pubblicare la lista. C'è un conflitto
strutturale fra la quota al creatore di [ADR-006] e la privacy dell'abbonato, e
finora non è stato dichiarato da nessuna parte.

**Contromisura.** Le opzioni non sono equivalenti e nessuna è gratuita.
(a) *Accettare e dichiarare*: gli abbonamenti sono pubblici, e il prodotto lo dice
chiaramente prima che l'utente si abboni. Costo: zero tecnico, alto reputazionale.
(b) *Chiave di spesa per app*: l'utente usa un'identità derivata distinta per ogni
app. Costo: rompe "un pagatore, un voto", perché l'unicità del pagatore è ciò che
impedisce il doppio conteggio — servirebbe una prova a divulgazione nulla di unicità,
sproporzionata per v0. (c) *Conteggio aggregato senza identità*: il publisher riceve
solo il numero, e il commitment usa un accumulatore che non espone i membri. Costo:
i validatori devono comunque conoscere i membri per verificarli, quindi sposta il
problema dal pubblico ai validatori senza eliminarlo. Raccomando (a) per v0 con la
dichiarazione esplicita, e (c) come lavoro di ricerca per la beta pubblica.
`SEC-REQ-22`.

#### TM-27 — App non deterministica che tenta la ricompensa di compute

**Asset:** A-02, A-09 · **Severità:** bassa · **Stato:** mitigato · **Rif:**
`app-manifest.md` §"Capabilities"

**Attacco.** Il publisher dichiara `runtime.deterministic: true` pur richiedendo
`http_fetch` o `storage_app` persistente, così i suoi risultati non sono
riproducibili e la verifica a campione fallisce sempre, accusando host onesti.

**Impatto.** Nullo, perché il documento chiude la combinazione: un'app con
`deterministic: true` **non può** richiedere quelle capability, e la violazione è un
rigetto del pacchetto. È un punto in cui il design ha già fatto la cosa giusta.
**Contromisura:** già presente; resta da assicurare che il fallimento di una verifica
a campione produca una contestazione arbitrabile e non una penalità automatica
all'host. **Costo:** trascurabile.

### 5.6 `T-06` — Osservatore di rete e avversario di percorso

Questa sezione è, insieme a §5.7, il contributo che nessuna review di una singola
spec poteva produrre: nessun ADR nomina la privacy, e i quattro documenti di
protocollo la trattano solo indirettamente. Non ci sono finding di [REVIEW-002] da
citare qui, perché [REVIEW-002] esaminava identità, enrollment e light client, e
questa superficie sta fra i documenti.

#### TM-28 — De-anonimizzazione: dal `node_id` alla persona

**Asset:** A-07, A-11 · **Severità:** alta · **Stato:** aperto · **Rif:**
`identity.md` §"Node identifier", `wire.md` §"Signed envelope", §"Gossip validation
and backpressure", §"Discovery"

**Attacco.** (1) L'avversario enrolla una manciata di identità legittime — costo
trascurabile — e si connette al maggior numero possibile di peer. (2) Ogni messaggio
che riceve porta `sender_node_id` in chiaro dentro l'envelope firmato, e la
connessione autenticata gli dà l'indirizzo IP. (3) `wire.md` vieta esplicitamente
l'uso della modalità autore anonimo di libp2p per gli oggetti applicativi, quindi
anche il gossip è attribuito. (4) mDNS su `_p2p._udp.local`, dove abilitato,
attribuisce il nodo a una **rete locale specifica**. (5) L'avversario costruisce una
mappa `node_id` → IP → geolocalizzazione, stabile nel tempo perché il `node_id` è
"stable for the life of the key". (6) Incrocia con il ledger pubblico (TM-26, TM-29)
e con i tempi delle challenge (TM-30).

**Impatto.** Chiunque, senza privilegi, può costruire l'elenco dei partecipanti alla
rete con la loro posizione approssimativa, i loro saldi, i loro abbonamenti e i loro
orari. Per un progetto che si presenta come "rete indipendente" e attrae per
definizione persone attente all'indipendenza, questo è un rischio reputazionale oltre
che personale. Va detto con chiarezza: **il design attuale è pseudonimo, non anonimo,
e la pseudonimia è stabile e quindi debole.**

**Contromisura.** 
(a) *Dichiarare*: la documentazione pubblica dice che partecipare espone IP e attività
a chiunque partecipi. Costo: zero, ed è **il minimo indispensabile e va fatto in ogni
caso**. 
(b) *Separazione della chiave di trasporto* (**adottata in ADR-015 e implementata in SPEC-013**):
la chiave di trasporto libp2p è distinta dalla chiave di identità permanente ed è
ruotabile; il legame tra chiave di trasporto e `node_id` avviene tramite `TransportKeyAttestation`
presentata in sessione e non è mai pubblicato sul ledger pubblico. Questo **toglie
all'osservatore offline che legge solo il ledger il legame già fatto**, e sposta il
costo dell'attacco da lettura gratuita e retroattiva a partecipazione attiva e
contemporanea. Non lo *elimina*, e la parola è stata corretta dopo [REVIEW-021]: la
misura tiene solo perché `identity.md` §"Key hierarchy" impone come **regola di
validità** che la chiave di trasporto non sia la chiave di identità, e perché
`TransportKeyAttestation::verify` rifiuta l'attestazione che le eguaglia. Finché
quella regola non esisteva, la contromisura era una preferenza dell'implementatore:
un nodo che riusasse la stessa chiave rendeva il Peer ID **ricalcolabile** dal solo
certificato pubblicato, il che ai fini di questa minaccia equivale a pubblicarlo.
La misura **non** è una rotazione: ciò che i documenti specificano è la *possibilità*
di ruotare e un intervallo **minimo** fra rotazioni (`wire.md` §"Transport rotation");
nessun pavimento, nessuna cadenza raccomandata. Un nodo che non ruota mai ha un Peer
ID stabile a vita, e una sola sessione con un peer ostile fissa la coppia per sempre.
(c) *Relay obbligatorio per i nodi domestici*: il Circuit Relay v2 già previsto da `wire.md`
nasconde l'IP di origine al peer remoto. Costo: latenza, carico sui relay, e un nuovo
insieme di nodi privilegiati che vedono tutto — sposta la fiducia invece di
eliminarla. `SEC-REQ-22`.

*Residuo aperto*: un interlocutore attivo che apre una connessione P2P con il nodo riceve
comunque l'attestazione e il certificato recante il `node_id` e vede `sender_node_id`
negli envelope firmati, associando l'indirizzo IP di trasporto all'identità (a meno di relay).
La minaccia resta pertanto **aperta** e la pseudonimia nei confronti dei peer attivi resta
soggetta a correlazione.

#### TM-29 — Il ledger pubblico come grafo permanente dei consumi

**Asset:** A-11 · **Severità:** alta · **Stato:** aperto · **Rif:** `ledger.md`
§"Burn", §"Sparse Merkle balance state"

**Attacco.** Non serve alcun attacco: è lettura. (1) Ogni `app_subscription` burn
contiene `payer_node_id`, `app_id`, l'importo e il periodo di servizio, in chiaro.
(2) Ogni saldo è in un albero pubblico indicizzato per `account_key = H(node_id)`, e
chiunque conosca il `node_id` — cioè chiunque abbia parlato con quel nodo — può
seguirne il saldo nel tempo. (3) La storia è finalizzata e permanente per costruzione.

**Impatto.** Chi si abbona a un'app di messaggistica riservata, a un servizio di
salute o a qualunque cosa dica qualcosa di sé, lo scrive per sempre in un registro
pubblico accanto al proprio identificatore stabile. L'assenza di valore monetario del
token **non riduce** questo rischio: lo rende meno interessante per un ladro e
altrettanto interessante per chiunque profili le persone.

**Contromisura.** Le stesse opzioni di TM-26, con la stessa conclusione: dichiarare
subito, ricercare per la beta pubblica. Il vincolo da tenere presente è che
qualunque soluzione deve preservare l'invariante "un pagatore, un voto" del
commitment di [ADR-006], che è ciò che impedisce a un proposer di inventare abbonati.
`SEC-REQ-22`. **Costo:** dichiarato in TM-26.

#### TM-30 — Pattern di vita ricavati dalle challenge

**Asset:** A-06, A-11 · **Severità:** media · **Stato:** aperto · **Rif:**
`ledger.md` §"Challenge evidence" (`completed_at_ms`, `outcome`)

**Attacco.** (1) Ogni evidenza di challenge finalizzata contiene `subject_node_id`,
`completed_at_ms` e `outcome`. (2) La serie storica delle evidenze di un nodo è la
serie storica di quando quel dispositivo era acceso e connesso. (3) Per un telefono,
questo è il calendario delle abitudini del proprietario: orari di sonno, giorni di
assenza, viaggi, cambi di fuso.

**Impatto.** Un effetto collaterale della trasparenza voluta ("verificabile in tempo
reale") che produce un dato personale sensibile senza che nessuno l'abbia deciso.

**Contromisura.** (a) Aggregare: il ledger registra il **conteggio** di challenge
superate per epoca invece di ogni singola evidenza con il suo istante. Costo:
riduce la granularità della dashboard "in tempo reale", che è una promessa di
prodotto — è un conflitto diretto e va deciso, non aggirato. (b) Arrotondare
`completed_at_ms` a una granularità grossa (l'epoca) nell'evidenza committata,
tenendo l'istante preciso solo nella verifica non persistita. Costo: minimo, ed è la
soluzione che raccomando perché preserva sia la verificabilità sia gran parte della
reattività percepita. `SEC-REQ-22`.

#### TM-31 — Censura di rete e isolamento da parte dell'ISP

**Asset:** A-03, A-05, A-10, A-12 · **Severità:** media · **Stato:** accettato ·
**Rif:** `wire.md` §"Network stack", §"Discovery"

**Attacco.** (1) Un ISP o un censore blocca UDP/QUIC, i multiaddr dei seed noti e le
porte usate. (2) I nodi in quella rete non raggiungono il consenso né i peer.

**Impatto.** Esclusione geografica dalla rete, con perdita di reddito per i
partecipanti coinvolti.

**Contromisura.** Parzialmente già presente e ben progettata: fallback TCP+Noise
obbligatorio, Circuit Relay v2 con ritenzione della connessione di relay quando
l'hole punching fallisce, seed da domini di guasto indipendenti. Resta scoperto il
caso del blocco per ispezione del protocollo, per cui l'unica risposta sarebbe un
trasporto offuscato — sproporzionato per v0. **Disposizione:** accettato per v0,
con `SEC-REQ-23` che chiede almeno la **diversità** dei canali di distribuzione dei
seed e degli ancoraggi. **Costo:** il relay obbligatorio è carico su volontari e un
punto di osservazione privilegiato (vedi TM-28c).

#### TM-32 — Intercettazione o sostituzione sul percorso

**Asset:** A-05, A-10 · **Severità:** bassa · **Stato:** mitigato · **Rif:**
`wire.md` §"Network stack", `ledger.md` §"Light-client balance verification"

**Attacco.** Un avversario sul percorso tenta di alterare blocchi, prove o risposte in
transito.

**Impatto.** Nullo. Il trasporto è autenticato (Noise/QUIC legati al Peer ID) e,
soprattutto, il design non si fida del trasporto: `ledger.md` dichiara che "TLS, a
signed peer envelope, or a proof from several servers cannot replace any step above"
e `wire.md` che "a response is never trusted because it arrived over an authenticated
peer connection". È una proprietà conquistata e va protetta da erosioni future.
**Contromisura:** già presente. **Costo:** nessuno. Nota: la frase di `ledger.md` va
comunque corretta come chiede [RF-003], perché oggi implica che i sette passi siano
*sufficienti*, mentre sono necessari.

#### TM-37 — Compromissione della chiave di trasporto: impersonificazione in sessione senza revoca

**Asset:** A-02, A-06, A-11 · **Severità:** media · **Stato:** aperto · **Rif:**
[ADR-015], `identity.md` §"Bounded validity in time", §"Anti-reuse property",
[REVIEW-021] RF-005

**Origine.** È una superficie **creata** da [ADR-015], e va registrata come tale: la
separazione che chiude metà di TM-28 introduce una chiave nuova, dichiarata "a basso
valore", il cui possesso vale in sessione il posto della vittima.

**Attacco.** (1) L'avversario ottiene la chiave privata di trasporto di un nodo —
esfiltrazione dal dispositivo, backup, chiave lasciata fuori dal credential store,
tutti casi che il documento stesso considera routine, perché è la ragione per cui la
chiave è dichiarata ephemera. (2) Intercetta o riceve la `TransportKeyAttestation`
corrispondente, che non è un segreto e non è legata a un destinatario. (3) Completa
l'handshake Noise/QUIC — che prova esattamente ciò che l'avversario ha — presenta
l'attestazione, ed è accettato come il nodo vittima da ogni peer che apre una
connessione diretta. (4) Non risponde. Un `challenge_request` è diretto a
`subject_node_id`: l'issuer che chiama la vittima raggiunge l'avversario, la challenge
scade, e l'evidenza entra nel ledger come `failed` o `late`.

**Impatto.** Perdita di `existence_income` e di eleggibilità a validatore per la
vittima: un attacco mirato con impatto economico misurabile. Ciò che l'avversario
**non** può fare è la parte solida del design e va detto: gli oggetti applicativi
restano firmati dalla chiave di identità, quindi non può forgiare un `SignedEnvelope`
né produrre un `subject_signature` valido. Il baratto rispetto a prima di [ADR-015] è
questo: allora la stessa cosa richiedeva la chiave di identità — compromissione
totale, ma **revocabile**; oggi è una chiave a basso valore e **non esiste alcuna
invalidazione anticipata di un'attestazione in circolazione**: nessun contatore di
epoca, nessun numero di serie, nessuna lista.

**Contromisura.** (a) *Finestra breve e limitata come regola*: il tetto
`max_transport_attestation_validity_ms` è un parametro di rete firmato dopo
[REVIEW-021], quindi l'esposizione è limitata dal protocollo e non dalla prudenza
dell'operatore. È l'unico limite che esista. **Con una riserva, e va detta qui
perché altrimenti questa frase promette più di quanto la regola tenga:** il tetto
vincola la *durata dichiarata* dall'attestazione, mentre la finestra in cui un
verificatore la accetta è quella durata **più** la tolleranza di orologio — e la
somma non è vincolata da nulla. Vedi [DEBT-017]. Finché quel debito è aperto,
l'esposizione è limitata dal protocollo *sul valore che il documento nomina*, non
sulla grandezza da cui la proprietà dipende. Costo: rotazioni più frequenti, cioè
verifiche di firma in più a ogni ristabilimento di sessione. (b) *Revoca
dell'identità*: funziona, ma distrugge identità, saldo e reputazione per una chiave
subordinata, ed è il rimedio sproporzionato che [ADR-015] esisteva per evitare. (c)
*Invalidazione anticipata dell'attestazione* — epoca o numero di serie pubblicati
sull'identità: **non adottata**, perché un contatore per identità osservabile in
sessione è un identificatore stabile in più, cioè ricrea in piccolo la correlazione
che [ADR-015] ha tolto. Va valutata come decisione propria se la finestra dovesse
essere allungata.

*Residuo aperto*: per tutta la durata della finestra la compromissione non è
revocabile, e la vittima non ha alcun segnale che la riveli se non l'esito delle
proprie challenge.

### 5.7 `T-07` — Insider di governance

Anche questa sezione è nuova rispetto a [REVIEW-002]. La spec [SPEC-004] chiede
esplicitamente l'analisi dell'abuso della lista di blocco di rete, e [ADR-006] la
elenca fra le proprie condizioni di revisione ("la governance della lista di blocco
di rete si dimostra un punto di potere problematico"). Il threat model conferma che
il problema è reale e ne dà la forma.

**Premessa di fatto.** La lista di blocco di rete **non esiste come oggetto di
protocollo**. [ADR-006] la istituisce in una frase — "esiste una lista di blocco di
rete, governata dai validatori, riservata agli abusi conclamati" — e l'unico
riferimento nei documenti è una menzione di passaggio in `app-manifest.md`
("Host refusal and network block policy may cause later reassignment"). Non esistono:
schema, autorità di proposta, soglia di quorum, requisito di evidenza, altezza di
efficacia, scadenza, procedura di appello, pubblicazione, né alcun modo per un light
client di verificare che un blocco sia avvenuto secondo le regole. Tutti gli scenari
seguenti discendono da questa assenza.

#### TM-33 — La lista di blocco come strumento di censura

**Asset:** A-08, A-10, A-12, A-13 · **Severità:** alta · **Stato:** aperto ·
**Rif:** [ADR-006] §"Consenso dell'ospite", `app-manifest.md` §"Manifest schema"

**Attacco.** (1) Un'app pubblica contenuti o servizi sgraditi a chi detiene il
quorum. (2) Il quorum la iscrive nella lista di blocco motivandola come "abuso
conclamato" — categoria che nessun documento definisce. (3) Gli host smettono di
ospitarla e la riassegnazione non trova nessuno. (4) L'app scompare dalla rete.
(5) Non esiste appello, non esiste scadenza, e non esiste alcun obbligo di pubblicare
l'evidenza: la decisione non è contestabile perché non è nemmeno ispezionabile.

**Impatto.** La rete acquisisce un interruttore di censura centralizzato,
esercitabile dallo stesso insieme che TM-18 mostra potersi rendere permanente. La
combinazione TM-18 + TM-33 è la trasformazione completa di una rete distribuita in un
servizio con un proprietario, e ciascuno dei due passi è invisibile preso da solo.

**Contromisura.** La lista di blocco deve diventare un oggetto di protocollo con
tutte le proprietà che oggi le mancano: (a) transazione firmata con quorum,
`effective_height`, e **motivo tra categorie chiuse**; (b) impegno all'evidenza,
pubblicamente reperibile; (c) **scadenza obbligatoria** con rinnovo esplicito — un
blocco che non scade è una condanna perpetua decisa senza processo; (d) visibilità
per il light client, cioè un impegno nel `BlockHeader` come chiede [RF-004](c) per le
revoche; (e) distinzione fra bloccare un `app_id` e bloccare l'identità del publisher,
che sono sanzioni di gravità incomparabile. `SEC-REQ-21`. **Costo:** è lavoro di
consenso reale su M-06 e una scelta politica esplicita: la scadenza obbligatoria
significa che la rete deve **riconfermare periodicamente** ogni blocco, il che è
lavoro di governance ricorrente. Lo raccomando comunque, perché l'alternativa è un
potere senza scadenza in un progetto che si definisce distribuito.

#### TM-34 — La lista di blocco come leva di pressione, e il blocco come confisca

**Asset:** A-11, A-12 · **Severità:** alta · **Stato:** aperto · **Rif:** [RF-017],
[ADR-006] §"Flusso di pubblicazione" punto 3, `ledger.md` §"Burn"

**Attacco.** (1) Un publisher ha un'app con molti abbonati e quindi un flusso di
`publisher_reward`. (2) L'insider fa sapere, formalmente o meno, che l'app è
"all'esame" per la lista di blocco. (3) Il publisher ha già bruciato token per il
periodo di hosting anticipato: [RF-017] mostra che il saldo dell'app previsto da
[ADR-006] non esiste nel ledger, e che l'`app_hosting` burn addebita l'intero periodo
in anticipo senza alcun percorso di rimborso. (4) Un blocco **distrugge** quei token e
azzera la quota al creatore. (5) Il publisher si adegua a ciò che gli viene chiesto.

**Impatto.** Estorsione senza denaro. L'assenza di convertibilità del token non
protegge da questo: ciò che è in gioco è il lavoro del publisher e l'accesso dei suoi
utenti al servizio, non un valore monetario. **Secondo effetto, sulla privacy:** per
poter accertare un "abuso conclamato" la rete deve costruire e conservare
l'attribuzione fra contenuto, publisher e host — cioè esattamente l'apparato di
sorveglianza che TM-28 e TM-29 rendono già facile. Un meccanismo di blocco crea
inevitabilmente il suo archivio.

**Contromisura.** (a) Chiudere [RF-017] introducendo il saldo dell'app previsto da
[ADR-006], così un blocco sospende il consumo invece di distruggere il residuo; se si
sceglie di non introdurlo in v0, la conseguenza va scritta. (b) Le proprietà di
TM-33: evidenza, scadenza, appello. (c) Regola esplicita: un blocco non è mai
retroattivo sui token già consumati né sui `publisher_reward` già emessi.
`SEC-REQ-21`. **Costo:** il saldo dell'app è una seconda classe di chiavi nell'albero
di stato e quindi lavoro di consenso significativo in M-02 — è il costo che [RF-017]
già dichiarava, e questo scenario è la ragione per cui vale la pena pagarlo.

#### TM-35 — La difficoltà di enrollment come gate silenzioso

**Asset:** A-02, A-07, A-12 · **Severità:** media · **Stato:** aperto · **Rif:**
`identity.md` §"One-time anti-Sybil proof of work", §"DRAFT: launch difficulty
policy"

**Attacco.** (1) `difficulty_bits` è un parametro del set firmato, con l'unico
vincolo di stare fra 18 e 40. (2) L'insider lo porta stabilmente verso 40. (3)
[RF-005] misura che a difficoltà 40 l'enrollment costa ~1,5 giorni su un core di
telefono Android e ~1 minuto su una GPU commodity. (4) I dispositivi mobili sono
esclusi di fatto dalla rete, mentre chi ha hardware da attacco entra comunque.
(5) Formalmente non è stato escluso nessuno.

**Impatto.** Il parametro pensato come difesa anti-Sybil funziona meglio come filtro
contro gli utenti onesti che come filtro contro l'attaccante — è la stessa asimmetria
di [RF-005] letta dal lato della governance. In più, la variante (b) della policy di
lancio ancora in `DRAFT` — aggiustamento automatico sul tasso di enrollment osservato
— è manipolabile: un attaccante che genera un picco di enrollment fa alzare la
difficoltà a tutti gli altri.

**Contromisura.** (a) Bound e variazione massima per epoca come in TM-19. (b) La
difficoltà va espressa e verificata rispetto a un **costo su un dispositivo di
riferimento dichiarato**, non a un numero di bit privo di significato indipendente
dalla primitiva. (c) Se si adotta l'aggiustamento automatico, l'input deve essere
resistente alla manipolazione da parte di chi genera gli enrollment. `SEC-REQ-14`,
`SEC-REQ-24`. **Costo:** (b) obbliga a mantenere un benchmark pubblico e a rifarlo
quando l'hardware cambia; è manutenzione ricorrente, ed è l'unico modo perché il
parametro voglia dire qualcosa.

#### TM-36 — Compromissione della distribuzione degli ancoraggi di fiducia

**Asset:** A-05, A-13 · **Severità:** alta · **Stato:** aperto · **Rif:** [RF-003],
`ledger.md` §"Light-client balance verification"

**Attacco.** (1) La chiusura di [RF-003] — che ho raccomandato io stessa — introduce
un **ancoraggio di soggettività debole**: un checkpoint `(height, block_id)` recente
distribuito insieme al pacchetto di rete. (2) Chi controlla quel canale di
distribuzione — la build, l'installer, lo store, il repository dei checkpoint —
controlla ciò che ogni nuovo client considera vero. (3) Un checkpoint falso in una
build compromessa produce esattamente l'attacco di lungo raggio che il checkpoint
doveva impedire, ma stavolta contro tutti i nuovi installati insieme.

**Impatto.** La contromisura sposta la fiducia dal protocollo alla catena di
distribuzione. Va detto, perché una difesa che nasconde la propria assunzione è
peggiore di un rischio dichiarato.

**Contromisura.** (a) Gli ancoraggi sono firmati da **più parti indipendenti** e il
client richiede una soglia di firme, non una sola. (b) Build riproducibili, così che
la corrispondenza fra sorgente e binario sia verificabile da terzi. (c) Il client
rifiuta un ancoraggio più recente di quello già persistito se non regredisce
coerentemente con la propria storia — la regola di non regressione di [RF-003] vale
anche verso i checkpoint, non solo verso i peer. `SEC-REQ-06`, `SEC-REQ-23`.
**Costo:** reale e a carico di AGENT-008: firma multipla significa più detentori di
chiavi e un processo di rilascio più lento; le build riproducibili sono un impegno di
ingegneria non banale su tre piattaforme. È il prezzo dell'unica difesa disponibile
contro il lungo raggio, e va messo a bilancio adesso, non alla beta.

## 6. Analisi quantitative

Tutti i numeri di questa sezione sono **ordini di grandezza**, dichiarati come tali.
Servono a distinguere fra "difficile" e "gratuito", non a tarare un parametro: la
taratura è compito del simulatore economico di M-02. Dove il numero deriva da una
misura pubblicata, la fonte è in §13.

### 6.1 Collusione e manipolazione dell'elezione dei validatori

**Il punto di partenza era che la regola di elezione non esisteva.** `ledger.md`
§"Validator-set continuity" chiudeva dicendo che la continuità "specifies safe
authentication but not how members are elected or rotated", e la sezione `DRAFT`
lasciava aperte due alternative: rotazione pesata su reputazione/uptime, oppure
lotteria su casualità finalizzata con soglia di eleggibilità. Le due alternative
hanno profili di attacco **opposti**, e questo è il primo risultato utile.

> **Aggiornamento 2026-08-25.** [SPEC-006] ha scelto, sulla base di questa analisi,
> la lotteria (b) con soglia di eleggibilità ancorata a storage e compute dimostrati,
> più le leve 1 e 2 dell'elenco in fondo alla sezione. La leva 3, diversificazione
> per posizione di rete, **non** è stata adottata: resta l'attrito evadibile descritto
> qui, e trasformerebbe la derivazione in una funzione di dati di rete non impegnati
> nel ledger, quindi non verificabile da un light client. L'analisi sotto resta valida
> come motivazione della scelta.

#### Quanto potere serve

Con la soglia BFT corretta serve **oltre 1/3** del potere di voto per rompere la
safety o fermare la liveness. Con la formulazione difettosa di `identity.md`
([RF-002]) serve meno: a `V=101` bastano 33 unità su 101, cioè il 32,7 %.

| Dimensione del set ([ADR-001]: 20–100) | Membri necessari a potere uguale |
| --- | --- |
| 20 | 7 |
| 50 | 17 |
| 100 | 34 |

#### Alternativa (a): elezione pesata su reputazione e uptime

L'attaccante non deve superare la rete onesta in numero: **deve solo che la sua
identità peggiore superi in classifica la migliore identità onesta.** Questo è
garantito per costruzione, non per fortuna:

| Tipo di nodo | Uptime realistico su una finestra di reputazione |
| --- | --- |
| Processo emulato su VPS con SLA 99,99 % | ~99,99 % (≈ 53 minuti di indisponibilità l'anno) |
| Desktop domestico acceso "sempre" | ~95–99 % (riavvii, aggiornamenti, blackout) |
| Telefono Android diligente | ~50–90 % (doze, rete assente, terminazione del processo da parte del sistema) |

Un'elezione che ordina per uptime **seleziona sistematicamente l'infrastruttura di
datacenter e scarta i dispositivi domestici**. È l'esatto rovescio dell'intenzione di
[[PROJECT]], che vuole includere telefoni e vecchi PC.

Costo per l'attaccante di 34 posizioni su 100: 34 identità enrollate (§6.2: costo
trascurabile) mantenute su infrastruttura con uptime da datacenter. Se l'unico
vincolo di diversità è per prefisso di rete, servono 34 istanze in prefissi distinti:
un ordine di grandezza di **poche centinaia di dollari al mese** di hosting presso
provider diversi. È il numero che conta: la soglia di safety della rete costa, oggi,
quanto un abbonamento aziendale a un servizio cloud di medie dimensioni.

#### Alternativa (b): lotteria su casualità finalizzata

Risolve il problema dell'uptime — la casualità non premia il datacenter — ma ne
introduce uno peggiore: **converte la numerosità direttamente in seggi.** Con `N`
identità dell'attaccante e `H` identità oneste eleggibili, la quota attesa di seggi è
`N/(N+H)`.

| `N` (Sybil) | `H` (oneste) | Quota attesa di seggi |
| --- | --- | --- |
| 10³ | 10³ | 50 % |
| 10⁴ | 10³ | 91 % |
| 10⁵ | 10³ | 99 % |

**Conseguenza decisiva: la selezione del comitato non è decidibile
indipendentemente dalla decisione anti-Sybil di §7.** Una lotteria su un insieme di
eleggibili che l'attaccante può popolare a piacere è peggiore di una classifica. La
lotteria è sicura solo se la **soglia di eleggibilità** è ancorata a qualcosa che un
Sybil non può falsificare a costo nullo — cioè a contributo di storage o compute
dimostrato, non a uptime e non alla semplice esistenza. Questo è lo stesso perno su
cui ruota §7.

#### Le tre leve, col loro costo

1. **Eleggibilità ancorata a lavoro Sybil-difficile.** Candidato validatore solo chi
   ha evidenza finalizzata di storage o compute forniti sopra una soglia, nelle
   ultime `k` epoche. *Costo:* esclude dal ruolo di validatore i nodi che offrono
   solo availability — cioè molti telefoni. È una scelta difendibile (fare il
   validatore su un telefono ha comunque poco senso), ma va detta.
2. **Tetto di rotazione per epoca (`churn cap`).** Al massimo una frazione `c` del
   potere di voto è sostituibile per rotazione. Un attaccante che domina la classifica
   impiega `⌈(1/3)/c⌉` epoche a raggiungere la soglia di safety:

   | `c` | Epoche per arrivare a 1/3 | Con epoca = 1 giorno |
   | --- | --- | --- |
   | nessun tetto | 1 | immediato |
   | 1/4 | 2 | 2 giorni |
   | 1/8 | 3 | 3 giorni |
   | 1/16 | 6 | 6 giorni |

   *Costo:* la rete diventa lenta a rimpiazzare validatori realmente guasti, e con
   `c` piccolo un guasto correlato (un provider che cade) resta scoperto per giorni.
   Il tetto non impedisce la cattura: **compra una finestra di osservazione**, ed è
   utile solo se qualcuno guarda. Va accompagnato dalla pubblicazione della
   composizione del set e della sua deriva.
3. **Diversificazione per posizione di rete.** Tetto di seggi per prefisso IPv4 /24 e
   IPv6 /48. *Costo:* penalizza nodi onesti co-locati (una famiglia, un campus, un
   ISP con CGNAT) ed è evadibile: distribuire 34 istanze su 34 prefissi presso
   provider diversi è una transazione di mercato ordinaria. Alza il costo da
   trascurabile a modesto; non è una difesa, è un attrito.

### 6.2 Economia dell'attacco Sybil contro il reddito di esistenza

#### L'affermazione strutturale

Il reddito di esistenza è un flusso **perpetuo**; il costo di enrollment è **una
tantum**. Il periodo di ammortamento è

```text
T = C_setup / (R_epoca − C_marginale)
```

e finché `R_epoca > C_marginale`, `T` è **finito qualunque sia `C_setup`**. Un costo
una tantum non può prezzare un flusso perpetuo. Questo vale per SHA-256, per
Argon2id, per qualunque prova di lavoro d'ingresso: cambia `T`, non l'esito.

Il parametro che decide davvero è quindi `C_marginale`: quanto costa **continuare** a
fingere, epoca dopo epoca. Oggi, per l'availability, è **una firma Ed25519** —
dell'ordine dei microsecondi (TM-08). Effettivamente zero.

#### Il vincolo che chiude la strada più ovvia

La risposta naturale sarebbe imporre un costo ricorrente per epoca. **[[PROJECT]]
lo vieta in modo permanente:** l'elenco delle esclusioni contiene "Mining/proof-of-work
continuo di qualsiasi tipo". La leva più efficace è quindi fuori discussione per
principio, e va tolta dal tavolo prima di ragionare sulle opzioni, non dopo.

Restano solo i costi marginali che derivano da **risorse realmente fornite**: tenere
i byte per superare una proof-of-retrievability, eseguire davvero un task per superare
la ri-esecuzione a campione. Che è precisamente la conclusione di §6.2.4.

#### Costo di produzione delle identità

Dai numeri misurati in [REVIEW-002] (Verifica 3) e dai profili di RFC 9106:

| Primitiva e difficoltà | Costo su un core di telefono (onesto, una tantum) | 10⁴ identità su una GPU commodity |
| --- | --- | --- |
| SHA-256, `d=22` (esempio di `identity.md`) | ~0,5 s | ~2 secondi |
| SHA-256, `d=40` (massimo consentito) | ~1,5 giorni — **inaccettabile per l'onboarding** | ~7 giorni |
| Argon2id `m=64 MiB, t=3, p=4`, ~16 valutazioni | ~3 s | ~4 minuti |

La riga Argon2id merita una precisazione, perché è una conseguenza che
[REVIEW-002] non aveva tratto: con una funzione memory-hard il costo sta nella
**singola valutazione**, non nel numero di tentativi, quindi la difficoltà va
ricalibrata a circa **2–6 bit** invece dei 18–40 di `identity.md`. L'intervallo 18–40
è una forma disegnata attorno a SHA-256. `identity.md` §"DRAFT: launch difficulty
policy" dichiara però che "the algorithm, verification rules, 18–40 safety bounds…
are not draft": **se la primitiva cambia, quel bound va riaperto insieme a essa**, e
il numero di bit deve essere espresso rispetto a un costo su dispositivo di
riferimento, non in assoluto (TM-35).

Il guadagno di Argon2id è di circa **due ordini di grandezza** — da 2 secondi a 4
minuti per 10⁴ identità. È reale e vale il suo prezzo, ma non è un cambiamento di
natura: non porta il costo dell'attacco da "trascurabile" a "proibitivo", lo porta da
"trascurabile" a "trascurabile-ma-noioso".

#### Il vero limite superiore a `N`

Non è l'hash: è la **presenza in rete**. Ogni identità deve essere raggiungibile per
ricevere `challenge_request`. Il vincolo pratico è lo stato di connessione e la
diversità degli indirizzi:

- un VPS ordinario (8 GiB) sostiene un ordine di 10³–10⁴ identità simultanee prima
  che lo stato per peer diventi il collo di bottiglia;
- un blocco IPv4 /24 offre 256 indirizzi distinti — noleggiabile sul mercato;
- una singola delega IPv6 /48 offre 2⁸⁰ indirizzi: **su IPv6 la diversità di
  indirizzo è gratuita**, il che rende i limiti per indirizzo IP inefficaci a meno di
  aggregare per prefisso instradato (§6.1, leva 3).

#### Il bottino, e la leva che lo controlla

Con `N` identità Sybil e `H` nodi onesti, e un reddito di esistenza **per nodo**,
l'attaccante cattura `N/(N+H)` dell'emissione di quel canale — e l'emissione totale
**cresce** con `N`, cioè l'attaccante stampa.

Se invece il reddito di esistenza è un **fondo a tetto fissato per epoca**, ripartito
fra i nodi con presenza dimostrata, allora:

- l'emissione totale **non cresce** con `N`: nessuna stampa, nessuna inflazione;
- l'attaccante ottiene una **quota**, non una quantità: l'attacco degrada da
  falsificazione a redistribuzione.

Chiamando `α` la frazione dell'emissione totale che passa dal canale
availability/esistenza, la quota di emissione totale catturata dall'attaccante è
`α · N/(N+H)`:

| `α` | `N=10⁴`, `H=10³` | `N=10³`, `H=10³` |
| --- | --- | --- |
| 1,0 (tutto il reddito è di esistenza) | 91 % | 50 % |
| 0,5 | 45 % | 25 % |
| 0,1 | 9 % | 5 % |

**Questa è la conclusione operativa di tutta §6.2, ed è la leva che il progetto non
sa di avere.** I canali storage e compute sono **intrinsecamente resistenti ai
Sybil**: falsificarli costa risorse reali, perché una proof-of-retrievability si
supera solo tenendo i byte e una ri-esecuzione deterministica si supera solo facendo
il lavoro. Il canale availability/esistenza è **intrinsecamente vulnerabile**, perché
falsificarlo costa una firma. Quindi:

> La domanda "quanto è resistente ai Sybil la rete Coblox" è, con precisione
> matematica, la domanda "quale frazione dell'emissione passa dal reddito di
> esistenza". Non è una domanda crittografica: è una domanda di design economico, e
> ha una risposta tarabile.

*Costo di questa leva, dichiarato:* un fondo a tetto fissato rende il reddito del
singolo **imprevedibile e decrescente con la crescita della rete** — "guadagni meno
man mano che arriva altra gente" è un messaggio di prodotto difficile e va deciso
consapevolmente. E dà all'attaccante un secondo obiettivo: **impoverire** gli onesti
diluendoli, invece di arricchirsi. Il danno è limitato in valore assoluto ma non è
nullo.

### 6.3 Superficie di [ADR-006]: abbonati fittizi e quota al creatore

#### Il conto

Per un periodo di abbonamento, con `S` = `microtokens_per_period`, `P` = quota al
creatore per abbonato attivo, ed `E_p` = reddito di esistenza maturato da un nodo
nello stesso periodo. L'attaccante controlla `N` identità e le abbona alla propria
app. Effetto marginale per identità per periodo:

```text
−S  (burn di abbonamento)  +  P  (quota al creatore)  =  −S + P
```

Con il vincolo di consenso proposto da [RF-007] — `P ≤ k·S` con `k < 1` — l'effetto è
`−S(1−k) < 0`: **strettamente negativo**. Il ciclo di stampa descritto in [RF-007] è
quindi chiuso, e vale la pena dirlo esplicitamente: quel vincolo fa il suo lavoro.
Un attaccante razionale sta meglio tenendosi il reddito di esistenza e non abbonando
nessuno.

#### Ciò che il vincolo `k` non chiude

Resta un guadagno che il conto sul ledger non cattura: **`active_subscriber_count` è
un numero pubblico e finalizzato**, ed è il segnale di popolarità dell'app nel
catalogo di scoperta. L'attaccante non compra token: compra **reputazione**, e la
paga `S(1−k)` per finto abbonato per periodo, con i token che il reddito di esistenza
gli regala.

L'attacco è sostenibile finché il reddito di esistenza copre la perdita:

```text
sostenibile  ⟺  E_p  ≥  S · (1 − k)
```

Usando i valori **di esempio** presenti nei documenti — `microtokens_per_period` =
300.000 per un periodo di 30 giorni (`app-manifest.md`), `amount_microtokens` =
250.000 per un mint di reddito di esistenza (`ledger.md`) — e ipotizzando un'epoca
giornaliera, `E_p` è dell'ordine di 7,5 · 10⁶ contro un costo di 1,5 · 10⁵ per finto
abbonato con `k = 0,5`: un margine di circa **50×**. Le cifre dei documenti sono
dichiaratamente illustrative e non tarate, quindi il numero non va preso come una
previsione — va preso come la dimostrazione che **il margine può facilmente essere di
uno o due ordini di grandezza, e che nessuno lo sta controllando**.

#### Le tre risposte, col loro costo

1. **Prezzare l'abbonamento sopra il reddito** (`S(1−k) > E_p`). *Costo: proibitivo,
   ed è la risposta sbagliata.* Renderebbe un abbonamento più caro di ciò che un nodo
   guadagna nello stesso periodo, cioè **inaccessibile all'utente onesto con un solo
   dispositivo** — che è esattamente l'utente descritto da [[PROJECT]]: "Utenti dei
   servizi: si abbonano alle app della rete spendendo i token del proprio reddito di
   esistenza". La segnalo perché è la leva che verrebbe naturale usare, ed è quella
   che rompe il prodotto.
2. **Pesare gli abbonati per contributo dimostrato.** La quota al creatore non conta
   teste ma somma un peso per abbonato derivato dal suo lavoro finalizzato
   (storage/compute) nell'epoca. Un'identità emulata ha peso ~0 e non produce né
   ricompensa né reputazione. *Costo:* un campo di peso nella foglia
   `subscription_leaf` e una funzione di reward più ricca, quindi lavoro di consenso
   in M-02; e un effetto collaterale da dichiarare — gli abbonati che contribuiscono
   poco "contano meno", il che va comunicato con attenzione. **È la risposta che
   riusa la sola proprietà Sybil-difficile che la rete possiede** (§6.2.4), ed è
   coerente con la leva di §6.2.
3. **Non esporre `active_subscriber_count` nel catalogo.** Il numero resta nel
   consenso per il calcolo, ma non è il segnale di popolarità mostrato agli utenti.
   *Costo:* nullo tecnicamente; toglie però un'informazione che gli utenti vogliono
   davvero, e la scoperta delle app deve reggersi su altro.

Le opzioni 2 e 3 sono componibili. La decisione spetta al Lead e ad AGENT-002 in sede
di taratura, ma il **vincolo va verificato dal simulatore in modo esplicito**
(`SEC-REQ-16`), perché oggi non è nell'elenco delle cose che il simulatore deve
controllare.

## 7. Quadro decisionale sull'anti-Sybil

Questa sezione **non sceglie**. Fornisce, per ciascuna opzione sul tavolo, cosa costa
all'attaccante, cosa costa all'utente onesto, quale rischio resta, e quale
formulazione della metrica di successo diventa allora verificabile. La scelta è
dell'operatore, e queste sono le sue quattro alternative scritte.

### 7.1 Il conflitto da risolvere

[[PROJECT]] §"Outcomes and success metrics" promette:

> *"Reddito di esistenza accreditato solo a nodi con presenza crittograficamente
> dimostrata (zero accrediti a nodi emulati nei test di attacco)."*

[RF-005] e TM-08 dimostrano che con il design v0 questo **non è raggiungibile per via
crittografica**. La `AvailabilityAssignment` prova che *una chiave risponde entro una
finestra*, non che *un dispositivo esiste*, e l'unico filtro d'ingresso — il proof of
work di enrollment — è battuto da hardware commodity di due-tre ordini di grandezza.

La metrica non è sbagliata come intenzione. È sbagliata come **formulazione**, perché
descrive un risultato che nessun meccanismo disponibile può produrre. Le §7.3–7.6
danno quattro riformulazioni candidate, una per opzione, tutte verificabili con un
test.

**Non modifico [[PROJECT]]**: è competenza dell'operatore. Propongo alternative
scritte perché la decisione sia una scelta fra opzioni note, non un'intuizione.

### 7.2 Due vincoli che valgono per tutte le opzioni

Prima di confrontare, due paletti che nessuna opzione può scavalcare — è utile
saperlo subito, perché eliminano le risposte più ovvie.

1. **Niente costo ricorrente.** [[PROJECT]] esclude in modo permanente "Mining/
   proof-of-work continuo di qualsiasi tipo". La leva più efficace contro un flusso
   perpetuo — un costo perpetuo — è fuori discussione per principio (§6.2.2).
2. **Niente valore monetario.** Nessuna contromisura può introdurre convertibilità,
   deposito cauzionale in valuta, o stake acquistabile. Questo esclude l'intera
   famiglia delle difese proof-of-stake, che è il modo in cui il resto del settore
   risolve il problema. §11 verifica che nessun `SEC-REQ` violi il vincolo.

Insieme, i due vincoli spiegano perché il problema è difficile qui e non altrove: le
due soluzioni standard sono entrambe escluse per ragioni fondative, non per
trascuratezza.

### 7.3 Opzione 1 — Onestà dichiarata più difesa economica

Il proof of work resta ciò che è, un costo d'ingresso; la difesa si sposta sulla
taratura di [ADR-005] con il vincolo che `N` identità costino più di quanto rendano.
§6.2 mostra che il vincolo, formulato così, **non è soddisfacibile**: un costo una
tantum non prezza un flusso perpetuo. La versione realizzabile dell'opzione 1 è
quindi più forte di come è enunciata, ed è quella che descrivo qui.

**Contenuto concreto.** (a) Il reddito di esistenza diventa un **fondo a tetto fissato
per epoca** ripartito fra i nodi con presenza dimostrata, invece di un importo per
nodo. (b) La frazione `α` dell'emissione totale che passa da quel canale è un
parametro esplicito, sorvegliato e pubblicato. (c) La resistenza reale viene dai
canali storage e compute, che sono Sybil-difficili per costruzione. (d) `identity.md`
dichiara il limite residuo.

**Costo per l'attaccante.** 10⁴ identità: ~2 secondi di una GPU. Il bottino non è più
"stampare": è `α · N/(N+H)` dell'emissione, senza aumentare l'emissione totale.
Con `α = 0,1`, `N = 10⁴`, `H = 10³` → 9 % dell'emissione.

**Costo per l'utente onesto.** Onboarding invariato (~0,5 s a `d=22`). Nessun
dispositivo escluso. Nessun costo di batteria aggiuntivo. Il costo è **di prodotto e
di comunicazione**: il reddito di esistenza diventa una quota variabile che scende
quando la rete cresce, e la rete deve dire chiaramente che una parte di quella quota
può essere catturata da nodi finti.

**Rischio residuo.** Un attaccante con 10⁵ identità cattura la quasi totalità del
canale di esistenza e, se l'eleggibilità dei validatori dipende da uptime o da
numerosità, anche il set (§6.1). L'opzione 1 **deve** quindi essere accompagnata da
un'eleggibilità ancorata a lavoro Sybil-difficile, altrimenti sposta il problema dal
portafoglio al consenso.

**Metrica candidata.**

> *Nei test di attacco, una flotta di `N ≥ 10.000` identità emulate su un singolo
> host: (a) non aumenta l'emissione totale dell'epoca di alcuna quantità;
> (b) non ottiene alcun accredito nelle categorie `storage` e `compute`; (c) non
> ottiene più del `X %` dell'emissione totale dell'epoca; (d) non ottiene alcun
> seggio di validatore.*

Verificabile con `AT-07` e `AT-10`. `X` è una decisione di prodotto: è la quota di
"perdita fisiologica" che il progetto dichiara di tollerare.

### 7.4 Opzione 2 — Proof of work memory-hard (Argon2id)

**Contenuto concreto.** Sostituire SHA-256 con Argon2id ([RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html)),
secondo profilo raccomandato (`m = 64 MiB`, `t = 3`, `p = 4`) come punto di partenza
da tarare a benchmark, e ricalibrare la difficoltà a ~2–6 bit invece di 18–40 (§6.2.3).

**Costo per l'attaccante.** 10⁴ identità passano da ~2 secondi a ~4 minuti di una
GPU: circa **due ordini di grandezza**, perché il costo per tentativo diventa
capacità di memoria (una GPU da 24 GiB regge ~375 tentativi concorrenti invece di
~16.000 shader). Reale, ma non un cambiamento di natura: un attaccante determinato
paga 4 minuti.

**Costo per l'utente onesto.** ~3 secondi e 64 MiB di picco, una tantum, su un
telefono di fascia media — sostenibile. Il primo profilo di RFC 9106 (2 GiB) **non**
lo è su Android e va escluso. I due costi meno evidenti:

- **DoS in verifica.** Con Argon2id verificare costa quanto generare (64 MiB e
  centinaia di ms per richiesta), mentre con SHA-256 costa un hash. È un vettore di
  denial-of-service contro i validatori che accettano gli enrollment. Mitigabile
  ordinando i controlli — schema, firma, `network_id`, timestamp, unicità, rate limit
  di connessione **prima** del passo memory-hard — ma la mitigazione va scritta e
  testata (`AT-14`), non assunta.
- **Il bound 18–40 va riaperto.** `identity.md` lo dichiara oggi "not draft"; è una
  forma disegnata attorno a SHA-256 e diventa priva di significato con una primitiva
  memory-hard. La difficoltà va espressa rispetto a un costo su dispositivo di
  riferimento dichiarato.

**Rischio residuo.** Tutto quello dell'opzione 1: la flotta emulata continua a
raccogliere il reddito di esistenza, solo con quattro minuti di preparazione in più.
**L'opzione 2 non è un'alternativa all'opzione 1: è un moltiplicatore che va sommato
a essa.** Da sola non consente alcuna riformulazione della metrica che contenga la
parola "zero".

**Metrica candidata.** Quella dell'opzione 1, più una clausola di pavimento:

> *Il costo di produzione di un'identità non scende sotto `C` secondi-core
> equivalenti su hardware d'attacco commodity, misurato con un benchmark pubblicato e
> ripetibile; l'onboarding di un dispositivo di riferimento dichiarato resta sotto
> `t` secondi.*

Verificabile: è un benchmark con due soglie. `AT-14` copre il lato DoS.

### 7.5 Opzione 3 — Attestazione hardware come tier "nodo certificato"

Questa è l'opzione parcheggiata in [ADR-002], che la descrive come "anti-Sybil più
forte ma esclude VM e vecchi dispositivi e crea dipendenza da Google/Microsoft". Ho
verificato la documentazione ufficiale corrente prima di scrivere questa sezione, e
**il quadro è sensibilmente peggiore di come [ADR-002] lo assume.** Le fonti sono in
§13; le incertezze residue sono dichiarate in fondo alla sezione.

**Google Play Integrity API.** Tre ostacoli, ciascuno sufficiente da solo:

1. **Non è verificabile da terzi.** Le richieste standard richiedono la decifratura
   del token **sui server di Google**; le richieste classiche consentono la
   decifratura locale ma con chiavi rilasciate dalla Play Console al titolare
   dell'app. In una rete peer-to-peer il verificatore è un validatore qualunque, che
   non ha né gli uni né le altre. Per usarla servirebbe un **servizio di verifica
   centrale gestito dal progetto** — cioè il coordinatore centrale che [ADR-001] ha
   scartato per principio.
2. **Quota.** Il limite predefinito è di **10.000 richieste al giorno per progetto
   Cloud**, condiviso fra tipi di richiesta; l'ammissibilità a un aumento richiede che
   l'app sia distribuita su Google Play. Una rete che punti a più di 10.000 nodi non
   può nemmeno attestarli una volta al giorno.
3. **Gli emulatori non sono esclusi, sono etichettati.** Un emulatore con Play
   services che supera i controlli riceve `MEETS_VIRTUAL_INTEGRITY`. Servono inoltre
   Play Store e Play services sul dispositivo, il che esclude Android de-googlizzato,
   ROM alternative e distribuzione via F-Droid — cioè, con precisione sgradevole, il
   pubblico che una "rete indipendente" attrae per primo.

**Android Key Attestation.** È l'unico meccanismo dell'insieme che un terzo può
verificare **offline** (catena fino alla radice di attestazione hardware di Google,
più la lista di revoca memorizzabile in cache). Ma prova una cosa diversa da quella
che serve: attesta che **una chiave** risiede in hardware ritenuto sicuro da Google
(`TrustedEnvironment` o `StrongBox`), non che il dispositivo sia distinto da un altro.
Un dispositivo può generare **un numero arbitrario di chiavi attestate**, e i campi
che identificano il dispositivo — `uniqueId`, e l'ID attestation con
`attestationIdSerial`/`attestationIdImei` — sono riservati alle app di sistema e ai
*device owner* tramite `DevicePolicyManager`: un'app P2P di consumo non vi ha accesso.

> **Conseguenza:** l'attestazione della chiave Android, da sola, fornisce **zero
> resistenza Sybil.** Prova la classe dell'hardware, non l'unicità. Un attaccante con
> dieci telefoni certificati veri produce diecimila identità attestate.

**TPM su Windows.** L'Endorsement Key è unico per TPM e "can identify it", ma:
Windows **evita deliberatamente** di esporlo, usando Attestation Identity Key proprio
"to prevent separate evaluators from collaborating to track the same device"; non
tutti i TPM hanno un certificato EK ("Not all TPMs have EKCert"); la lettura dell'EK
è un'operazione amministrativa; e il percorso documentato da Microsoft per
l'attestazione delle chiavi TPM è l'iscrizione presso una CA d'impresa, non un'API per
applicazioni desktop di terze parti.

**Linux e headless.** Non ho trovato alcun equivalente documentato, offline e a
livello applicativo. **Una delle tre piattaforme di nodo previste da [[PROJECT]] non
avrebbe alcun tier certificato.**

**Costo per l'utente onesto.** Esclusione di: macchine virtuali e server headless
(l'intera piattaforma), Android senza servizi Google, ROM alternative, dispositivi
con bootloader sbloccato, hardware vecchio senza TEE, PC senza certificato EK.
Dipendenza permanente da radici di fiducia Google e Microsoft, e da una loro politica
che può cambiare — Google, per esempio, inizia a firmare le catene di attestazione con
una nuova radice ECDSA dal 1° febbraio 2026, e i verificatori devono fidarsi di
entrambe.

**Costo per l'attaccante.** Nella forma realizzabile (attestazione della chiave
Android), **quasi nullo**: il prezzo di alcuni telefoni certificati veri, ammortizzato
su un numero illimitato di chiavi.

**Rischio residuo.** L'opzione non fa ciò per cui verrebbe scelta. Nessuna fonte
primaria avalla l'attestazione hardware come meccanismo di "un'identità per
dispositivo fisico" per un relying party decentralizzato: i due meccanismi
effettivamente unici per dispositivo (EK del TPM, ID hardware Android) sono
**entrambi deliberatamente protetti per ragioni di privacy**.

**Metrica candidata.** L'unica formulazione onesta e verificabile è molto più stretta
di quella originale:

> *Il reddito di esistenza è accreditato solo a nodi la cui chiave d'identità è
> attestata da hardware ritenuto affidabile secondo la radice di attestazione
> dichiarata; nei test di attacco, zero accrediti di reddito di esistenza vanno a
> chiavi non attestate.*

Verificabile, ma si noti cosa **non** dice: non dice "zero nodi emulati", perché una
chiave attestata non implica un dispositivo distinto. E si noti chi esclude: tutti i
nodi headless e tutte le VM.

**Incertezze dichiarate.** Tre punti che non ho potuto confermare da fonte primaria e
che vanno verificati sperimentalmente prima di qualunque impegno su questa opzione:
(i) se un vTPM di Hyper-V o Azure sia distinguibile da hardware fisico tramite la
catena del certificato EK; (ii) l'esistenza di un tipo di *claim* TPM documentato e
supportato per applicazioni desktop di terze parti; (iii) la frazione di PC di consumo
che possiede effettivamente un certificato EK, per cui non esiste alcun dato
ufficiale.

### 7.6 Opzione 4 — Combinazioni

Le opzioni non sono mutuamente esclusive e, alla luce di §7.5, la combinazione più
difendibile **non** è quella che si immaginerebbe. Presento le due che reggono
all'analisi.

**4a — Difesa economica più pavimento memory-hard (opzioni 1 + 2).**
Fondo a tetto fissato per il reddito di esistenza, `α` sorvegliata, eleggibilità dei
validatori ancorata a lavoro Sybil-difficile, Argon2id per alzare il pavimento
d'ingresso di due ordini di grandezza. Nessun dispositivo escluso, nessuna dipendenza
esterna, nessun servizio centrale. *Costo:* la metrica non conterrà mai la parola
"zero"; il reddito diventa una quota variabile; +3 s di onboarding e un vettore di
DoS in verifica da mitigare.
*Metrica:* quella di §7.3 con la clausola di pavimento di §7.4.

**4b — 4a più un tier certificato facoltativo e non escludente.**
Come 4a, con in più un'attestazione hardware **facoltativa** usata come *segnale di
qualità* per l'eleggibilità dei validatori o per una quota preferenziale, mai come
condizione per partecipare o per guadagnare. Un nodo non attestato resta un nodo di
prima classe per storage e compute. *Costo:* una superficie di attestazione da
mantenere per ogni piattaforma, una dipendenza da radici di fiducia esterne limitata
a un sottoinsieme di funzioni, e un rischio di **percezione a due velocità** nella
comunità. *Beneficio reale:* modesto, per le ragioni di §7.5 — l'attestazione dice
"hardware serio", non "dispositivo distinto".
*Metrica:* quella di 4a, più: *"nessuna funzione della rete è preclusa a un nodo non
attestato, e nessun canale di guadagno richiede attestazione."*

Sconsiglio esplicitamente la combinazione che rendesse l'attestazione **obbligatoria**
per il reddito di esistenza: è la sola che consentirebbe di scrivere "zero", ma
escluderebbe una delle tre piattaforme di prodotto e non fornirebbe comunque unicità
per dispositivo.

### 7.7 Confronto

| | Opzione 1 — economica | Opzione 2 — Argon2id | Opzione 3 — attestazione | Opzione 4a — 1+2 | Opzione 4b — 1+2+tier |
| --- | --- | --- | --- | --- | --- |
| **Costo attaccante, 10⁴ identità** | ~2 s di GPU | ~4 min di GPU | ~alcuni telefoni veri | ~4 min di GPU | ~4 min di GPU |
| **Onboarding onesto** | ~0,5 s | ~3 s, 64 MiB | dipende dalla piattaforma | ~3 s, 64 MiB | ~3 s, 64 MiB |
| **Dispositivi esclusi** | nessuno | nessuno | VM, headless, Android non-Google, hardware vecchio | nessuno | nessuno (tier facoltativo) |
| **Dipendenze esterne** | nessuna | nessuna | radici Google/Microsoft, quota Google, servizio centrale di verifica | nessuna | parziali e facoltative |
| **Emissione totale falsificabile** | no (fondo a tetto) | no | no | no | no |
| **Quota di emissione catturabile** | `α·N/(N+H)` | idem | ~0 nel tier certificato | `α·N/(N+H)` | `α·N/(N+H)` |
| **Consente di scrivere "zero"** | no | no | solo per "chiavi non attestate", non per "nodi emulati" | no | no |
| **Costo di implementazione** | consenso M-02 (fondo, `α`) | primitiva + ordine dei controlli | superficie per piattaforma + servizio | somma dei due | somma dei tre |

### 7.8 Cosa cambierebbe questa analisi

Tre informazioni che oggi non ho e che, se acquisite, sposterebbero il confronto:

1. **La `α` effettiva.** Il rapporto fra reddito di esistenza e compensi di lavoro
   nella taratura di M-02 decide la gravità di tutto il problema (§6.2.4). Se il
   simulatore mostrasse `α` naturalmente bassa, l'opzione 1 sarebbe quasi sufficiente
   da sola.
2. **Il benchmark reale di Argon2id sul parco dispositivi target.** I ~3 s e i ~4
   minuti di §6.2.3 sono ordini di grandezza. Se su un telefono di fascia bassa il
   profilo a 64 MiB costasse invece decine di secondi, l'opzione 2 perderebbe gran
   parte del suo valore.
3. **Le tre incertezze di §7.5.** In particolare, se risultasse che un vTPM è
   distinguibile da hardware fisico tramite la catena EK, il tier certificato
   guadagnerebbe una proprietà — l'esclusione delle VM di datacenter — che oggi non
   posso affermare.

## 8. Soglia di rischio accettato: devnet contro rete pubblica

[SPEC-004] registra come questione aperta la soglia di rischio accettabile per la
devnet rispetto alla rete pubblica. Ecco la proposta.

**Principio.** Sulla devnet i partecipanti sono invitati, identificabili e
consapevoli, e la storia della catena è sacrificabile: si può ripartire da una nuova
genesi. Sulla rete pubblica nessuna delle tre cose è vera, e in particolare **la
storia diventa irreversibile** — che è la ragione per cui i difetti che si manifestano
come spaccature di rete vanno chiusi *prima*, non *dopo*.

**Regola di taglio.** Un difetto può restare aperto in devnet se e solo se: (a) il suo
sfruttamento è osservabile dagli operatori; (b) la riparazione non richiede di
riscrivere una storia già finalizzata; (c) non produce dati personali permanenti.
Il criterio (c) è il motivo per cui gli scenari di privacy non seguono la stessa
scala degli altri: un ledger pubblico non si può de-pubblicare.

| Categoria | Devnet | Rete pubblica (M-08) |
| --- | --- | --- |
| Divergenze di consenso — TM-12, TM-16, TM-17 | **Da chiudere già in M-01.** Non sono rischi da accettare: sono difetti di specifica a costo di chiusura quasi nullo, e in devnet servono proprio per validare le fixture. | Chiusi e verificati su vettori |
| Falsificazione della storia — TM-10, TM-21, TM-36 | Accettabile finché i client di devnet nascono con un'ancora recente distribuita con la build | Chiuso: ancoraggio, non regressione, firma multipla |
| Cattura del set di validatori — TM-09, TM-18 | Accettabile **con dichiarazione esplicita** che i validatori di devnet sono seed del progetto e la rotazione non è ancora un meccanismo di sicurezza. La regola esiste da [SPEC-006], quindi la condizione di [DEBT-005] "nessuna devnet accumuli storia conservabile" è soddisfatta appena l'implementazione la applica | Chiuso: regola di elezione scritta, verificabile e con tetto di rotazione. **È il gate di M-07** |
| Sybil sul reddito di esistenza — TM-08, TM-22 | Accettabile: in devnet il reddito di esistenza può essere disattivato o simbolico, ed è la sede naturale per **eseguire** `AT-07` | Dipende dalla decisione di §7. Qualunque opzione, la metrica dichiarata deve essere verificata da `AT-07` prima dell'apertura |
| Abuso delle risorse dell'hoster — TM-23, TM-24, TM-25 | Accettabile: gli host di devnet sono del progetto o di volontari informati | Chiuso: politica di accettazione automatica, tetti di deployment, trasparenza su cosa si ospita. È un gate di M-06 |
| Censura e lista di blocco — TM-13, TM-15, TM-33, TM-34 | Accettabile: il set è del progetto e la questione è priva di senso pratico | Chiuso: la lista di blocco deve essere un oggetto di protocollo con evidenza, scadenza e visibilità **prima** che esistano publisher terzi |
| Privacy — TM-26, TM-28, TM-29, TM-30 | **Non accettabile in silenzio nemmeno in devnet.** La dichiarazione (`SEC-REQ-22`) va fatta al primo partecipante esterno, perché i dati che si producono sono permanenti | Dichiarazione più almeno le misure a costo basso (aggregazione dei tempi di challenge) |
| Esaurimento risorse — TM-11 | Accettabile con sorveglianza | Chiuso: limiti globali oltre che per peer |

Due categorie non ammettono la distinzione devnet/pubblico e vanno trattate come
gate assoluti: le **divergenze di consenso**, perché costano quasi nulla da chiudere
e moltissimo da correggere dopo; e la **dichiarazione di privacy**, perché produce
dati irreversibili dal primo partecipante.

## 9. Requisiti di sicurezza derivati

Ogni requisito è formulato perché un test o una review possa dire **pass/fail**, ed è
mappato a una milestone esistente della roadmap. Questa tabella è l'oggetto della
verifica `GATE-LEAD-MAP`.

| ID | Requisito | Come si verifica | Milestone | Owner | Copre |
| --- | --- | --- | --- | --- | --- |
| `SEC-REQ-01` | Una sola regola di verifica Ed25519 è dichiarata come equazione nei documenti di protocollo, con divieto esplicito delle alternative | I 12 vettori di `ed25519-speccheck` sono fixture di conformità con esito atteso dichiarato per ciascuno; `AT-02` | M-01 | AGENT-001 | TM-12, [RF-001] |
| `SEC-REQ-02` | Un'unica formula intera di quorum, identica nei tre punti in cui compare | `AT-01`: tabella di confine a `V=100/101/102` con esito atteso | M-01 | AGENT-001 | TM-16, TM-17, [RF-002] |
| `SEC-REQ-03` | Le preimmagini di firma di transazioni ed enrollment includono un `chain_id` derivato da `network_id` e `genesis_block_id` | `AT-05`: una transazione firmata su una rete con lo stesso `network_id` ma genesi diversa è rifiutata | M-01 | AGENT-001 | [RF-014] |
| `SEC-REQ-04` | Ogni campo hash usato come impegno ha dominio e serializzazione definiti; i documenti firmati (`policy_hash`, rate card, parameter set) sono pubblicamente reperibili | Una fixture per ciascuno degli otto campi; review che verifica la reperibilità dichiarata | M-01 | AGENT-001 | TM-19, TM-20, [RF-008] |
| `SEC-REQ-05` | Ogni voce del `ValidatorSet` porta una firma di legame emessa dalla chiave identitaria; una revoca finalizzata di un membro del set attivo impone una transizione di set | Fixture negativa: un set con una voce priva di legame valido è rifiutato. `AT-09` include un validatore revocato che continua a votare | M-01 (formato), M-02 (applicazione) | AGENT-001, AGENT-002 | [RF-004] |
| `SEC-REQ-06` | Il light client ha un ancoraggio di soggettività debole con età massima dichiarata, rifiuta la sincronizzazione da sola genesi oltre quell'età, e non regredisce mai sotto l'altezza fidata persistita | `AT-03`, `AT-04` | M-01 (regola), M-03 (client) | AGENT-001, AGENT-004 | TM-10, TM-21, TM-36, [RF-003], [RF-011] |
| `SEC-REQ-07` | La prova di enrollment lega un `block_id` finalizzato recente ed è rifiutata oltre una finestra parametrizzata; nessun `parameter_set_hash` è pubblicato prima dell'attivazione | `AT-08` | M-01 | AGENT-001 | TM-07, [RF-006] |
| `SEC-REQ-08` | Il bit della bitmap della prova Merkle è 1 se e solo se il sibling differisce dal default della sua profondità | `AT-06`: fixture negativa con sibling esplicito uguale al default | M-01 | AGENT-001 | [RF-016] |
| `SEC-REQ-09` | L'ordinamento delle transazioni in un blocco rispetta le dipendenze di nonce e di evidenza | `AT-15`: un blocco con due burn dello stesso account i cui `tx_id` sono in ordine inverso rispetto ai nonce è valido | M-01 | AGENT-001 | TM-04, [RF-009] |
| `SEC-REQ-10` | Una sola forma testuale canonica del Peer ID; ogni confronto avviene sulla multihash decodificata | Fixture: la stessa chiave nelle due forme testuali, con la non canonica rifiutata | M-01 | AGENT-001 | [RF-010] |
| `SEC-REQ-11` | Esiste un tetto alla validità di un envelope, e i limiti anti-abuso sono **globali** oltre che per peer, con comportamento definito in saturazione | `AT-13`: raffica a validità massima da molte identità distinte; la memoria resta limitata e il comportamento è quello dichiarato | M-01 (regola), M-02 (implementazione) | AGENT-001 | TM-11, [RF-012] |
| `SEC-REQ-12` | `identity.md` dichiara esplicitamente che il protocollo non distingue `N` nodi emulati su un host da `N` dispositivi reali | Review del Lead sul testo | M-01 | AGENT-001 | TM-08, [RF-005] |
| `SEC-REQ-13` | La regola di elezione dei validatori è scritta, deterministica a partire da casualità finalizzata su un insieme di eleggibili calcolabile da chiunque, con tetto di rotazione per epoca e con impegno nel header che ne consenta il ricalcolo a posteriori; l'equivocazione di un validatore è una transazione di evidenza sanzionabile | `AT-09`, `AT-10` | **M-02: coperto in specifica** da [SPEC-006] (`ledger.md` §"Validator election and rotation"), **in dipendenza da `SEC-REQ-14`**: la regola di elezione non è più forte dei limiti che vincolano i parametri che la definiscono, e finché quei parametri non hanno intervallo firmato alla genesi e variazione massima per epoca la copertura è condizionale. [SPEC-006] soddisfa quella dipendenza per i parametri di elezione tramite `ElectionBounds`, e [SPEC-009] la chiude per la reward policy tramite `RewardBounds`: `SEC-REQ-14` non è più aperto per i documenti che definiscono l'elezione e l'emissione, e la copertura di questa riga smette di essere condizionale su quel fronte. Resta condizionale sul **potere effettivo di una coalizione**: il vincolo `3 · min_set >= 2 · V` alza la soglia di cattura per attrito a circa `4V/9` e non a due terzi (`SEC-REQ-18`, `AT-10`). M-07 (rotazione automatica e sanzione dell'equivocazione) resta aperto | AGENT-002 | TM-09, TM-15, TM-17, TM-18 |
| `SEC-REQ-14` | Ogni parametro governabile ha un intervallo ammissibile firmato alla genesi e una variazione massima per epoca; le modifiche hanno un ritardo di attivazione dichiarato | Fixture: un blocco che attiva un parametro fuori intervallo o oltre la variazione massima è invalido | **M-02: coperto in specifica per i due documenti che portano proprietà di sicurezza dichiarate**, da [SPEC-006] (`ElectionBounds`) e [SPEC-009] (`RewardBounds`): magnitudini di genesi, vincoli relazionali, rapporto 5/4 e gap di attivazione, tutti fuori dalla governance on-chain e mai apprendibili da un peer. **Il criterio di completezza è stato corretto in [REVIEW-014] e va applicato così d'ora in poi**: non basta vincolare la grandezza che l'ADR nomina, perché una grandezza che sta al **denominatore** di una vincolata, o che ne **denomina l'unità**, porta la proprietà quanto quella nominata. La prima versione di `RewardBounds` vincolava `F` lasciando governata la durata dell'epoca (RF-002), e poneva un pavimento in unità di contributo lasciando governati i fattori che definiscono l'unità (RF-003); `RewardBounds` porta ora anche `reward_epoch_ms_min`/`_max`, i tetti dei due fattori di conversione, il tetto della finestra di eleggibilità e i pavimenti delle due tariffe di lavoro, e il rapporto 5/4 si applica **a ogni** grandezza di `RewardPolicyBody` senza eccezione. Le tredici grandezze sono coperte per enumerazione verificata. **Residuo dichiarato, e non è di [SPEC-009]:** `HostingRateCardBody` è un terzo documento governato senza alcun oggetto di limiti, con un proprio `billing_epoch_ms` che è il denominatore delle sue tariffe. Non tocca l'emissione — le voci di hosting sono **burn** e non mint, quindi non c'è superficie Sybil — ma è integrità di addebito sugli escrow delle app, ed è precisamente il caso che le *review conditions* di [ADR-010] chiedono di sorvegliare: una grandezza di sicurezza in un documento governato che nessuno dei due `Bounds` copre. Va valutata quando M-06 tocca l'hosting | AGENT-002 | TM-19, TM-35 |
| `SEC-REQ-15` | Per ogni `(app_id, reward_epoch)` vale `publisher_reward ≤ k × Σ(burn di abbonamento conteggiati nella radice)` con `k < 1` fissato nei parametri firmati, imposto dai validatori come regola di validità | `AT-11`: fixture al confine di `k` | M-02 | AGENT-002 | TM-22, [RF-007] |
| `SEC-REQ-16` | Il simulatore economico verifica ed espone nel proprio rapporto: (a) la frazione `α` dell'emissione che passa dal canale availability/esistenza; (b) la relazione `E_p` contro `S(1−k)` di §6.3; (c) la quota di emissione catturabile da `N` identità emulate | Il rapporto del simulatore contiene le tre grandezze con i valori tarati; review del Lead | **M-02: coperto** da [SPEC-007], rapporto in `knowledge/economic-simulation-report.md` §7, simulatore in `sim/`. (a) `α = 0,15`, banda [0,10–0,20], `X = 20 %`; (b) margine di reputazione **≈ 3 finti abbonati per nodo controllato per periodo** a 30 cr di abbonamento — non i 50× stimati da cifre illustrative — e **non chiudibile per taratura**: resta aperto sulle opzioni 2 e 3 di §6.3 sotto [ADR-006]; (c) 14,851 % a `N=10⁴`, `H=100`, sotto il tetto `X` per costruzione **nel regime maturo sopra la soglia d'uso reale (70,6 % del riferimento)** — sotto quella soglia `α → 1` e `X` non si applica, la garanzia valida lì è quella incondizionata `D = F · N/(N+H) < F`, vera a ogni livello d'uso ([ADR-011], [REVIEW-014] RF-008); **(d) aggiunta da [REVIEW-011] RF-005: la frazione di reddito che un nodo onesto di sola availability conserva sotto il banco di `AT-07` — `H/(N+H)`, cioè lo 0,99 % a `N=10⁴`, `H=100`, e non contiene `α`.** È la grandezza rivolta all'utente e va pubblicata accanto a `X`, **perché `X` da sola invita il lettore a concludere che il proprio reddito sia protetto entro un ordine di grandezza quando non è protetto affatto** (giustificazione ripristinata dopo [REVIEW-014] RF-006) | AGENT-002 | §6.2, §6.3, TM-08, TM-22 |
| `SEC-REQ-17` | La `randomness` di challenge e l'assegnazione emittente→soggetto derivano da casualità finalizzata con impegno pubblicato prima della selezione, sono ricalcolabili da chiunque, e ogni soggetto è coperto da almeno due emittenti indipendenti per epoca | `AT-12`; più un test statistico sul log di una devnet che verifichi l'imprevedibilità degli istanti per il soggetto | M-03 | AGENT-002, AGENT-001 | TM-05, TM-13, TM-14, [RF-013] |
| `SEC-REQ-18` | L'emissione del reddito di esistenza è ripartita da un fondo a tetto per epoca; la frazione `α` è un parametro pubblicato e sorvegliato; il valore atteso di un custode cresce strettamente con la frazione di dati realmente conservati | `AT-07`; per la seconda parte, `AT-12` parte B (simulazione del custode parziale con esito monotono) | **Fondo, regole di validità, due strumenti e claim: specificati** da [SPEC-007] e [SPEC-009] ([ADR-010], [ADR-011]), verificati in [REVIEW-014] dopo la correzione di otto rilievi. Ripartizione **uniforme** (`E > 0`, `amount = F/E`, resto mai emesso, somma per epoca `<= F`, insieme impegnato da `eligible_set_root`): nessuna via trovata per superare `F` in un'epoca né per gonfiare `E` senza che la radice lo mostri. `availability_microtokens_per_unit == 0` è **regola di validità in accettazione** con la ragione strutturale accanto — unico canale che paga per nodo senza tetto aggregato — ed è **chiusa senza residui**: cercate e non trovate vie alternative per reintrodurre una remunerazione per nodo da un altro canale, perché l'availability non ha altra tariffa, la sua evidenza alimenta solo la soglia di eleggibilità al fondo, e il fondo è `F/E`. `RewardBounds` nell'ancora di fiducia di genesi copre le tredici grandezze di `RewardPolicyBody` (vedi `SEC-REQ-14` per il criterio di completezza e il residuo dichiarato). **La forma del claim è una garanzia incondizionata più una proprietà aggiuntiva, non due fasi disgiunte** ([REVIEW-014] RF-008): l'importo massimo dirottabile per epoca è `D = F · N/(N+H) < F` **a ogni** livello d'uso, perché `D` non contiene `W`; sopra il 70,6 % dell'uso di riferimento vale **in più** la banda `α ∈ [0,10 – 0,20]` con `X = 20 %`, e sotto quella soglia la banda **non è sospesa, è falsa**, perché `α → 1` per costruzione. `F` di genesi è dimensionato sulla popolazione onesta attesa al lancio; la crescita verso `F_max` costa **18 documenti** al 5/4 con la spaziatura minima, ed è governance attiva dichiarata e non scoperta. Tetto proporzionale agli eleggibili `F = k · E` **rifiutato** con la motivazione corretta: sarebbe un tetto funzione di una grandezza che l'avversario controlla. **Ciò che resta vero solo per il valore e non per la regola:** il bordo inferiore `0,10` della banda è una scelta di prodotto e non una misura ([REVIEW-011] RF-005), e va pubblicato come tale. Custode parziale resta M-05 | AGENT-002 | TM-01, TM-02, TM-08 |
| `SEC-REQ-19` | Un host valuta automaticamente ogni assegnazione contro una politica dichiarata (capability, tetti, origini, capacità): dentro la politica è accettazione, fuori è **rifiuto**, mai concessione; l'operatore può vedere in ogni momento quali `app_id` ospita e con quali capability | Test: un'assegnazione fuori politica su un host headless produce rifiuto e non esecuzione | M-06 | AGENT-003 | TM-23, TM-24, TM-25, [RF-015] |
| `SEC-REQ-20` | `http_fetch` risolve il nome una sola volta, valida tutti gli indirizzi risultanti contro le classi vietate, si connette all'indirizzo validato senza ri-risolvere, applica la stessa validazione a ogni redirect, e imputa durata e byte ai limiti dichiarati | Vettore con un nome che cambia risoluzione fra validazione e connessione | M-06 | AGENT-003 | [RF-018] |
| `SEC-REQ-21` | La lista di blocco di rete è un oggetto di protocollo firmato con quorum, con motivo tra categorie chiuse, impegno all'evidenza, **scadenza obbligatoria**, distinzione fra blocco dell'app e blocco del publisher, e visibilità per il light client; un blocco non distrugge i token già versati né è retroattivo | Review del Lead sullo schema; test che un blocco senza scadenza è rifiutato | M-06 | AGENT-002 | TM-33, TM-34, [RF-017] |
| `SEC-REQ-22` | La documentazione pubblica dichiara che identificatore di nodo, indirizzo IP, saldo, abbonamenti e orari di attività sono pubblici e correlabili; gli istanti di challenge sono committati con granularità di epoca e non al millisecondo | Review sul testo pubblico; fixture sull'arrotondamento nell'evidenza | M-01 (dichiarazione), M-08 (misure) | AGENT-001, AGENT-LEAD | TM-26, TM-28, TM-29, TM-30 |
| `SEC-REQ-23` | Gli ancoraggi di fiducia e i binari sono firmati da più parti indipendenti con soglia richiesta dal client, distribuiti per canali diversificati, e le build sono riproducibili | Verifica indipendente che il binario rilasciato corrisponda alla sorgente; test che un ancoraggio sotto soglia di firme è rifiutato | M-04 (rilascio), M-08 (beta) | AGENT-008 | TM-31, TM-36 |
| `SEC-REQ-24` | Ogni nuovo canale di emissione introdotto dopo v0 è accompagnato, prima dell'attivazione, da una valutazione documentata del costo di falsificazione e della quota di emissione che vi transita | Review del Lead con il documento di valutazione allegato all'ADR o alla spec | M-02 e successive | AGENT-LEAD, AGENT-007 | TM-19, TM-35, §6.2 |

Tre requisiti sono, a mio giudizio, **irrinunciabili** e vanno chiusi prima di
qualunque altra cosa, perché hanno costo di chiusura quasi nullo e conseguenze
irreversibili se ignorati: `SEC-REQ-02` (soglia di quorum), `SEC-REQ-01` (regola di
verifica delle firme) e `SEC-REQ-13` (regola di elezione). I primi due sono righe di
testo; il terzo è l'unico che impedisce alla rete di diventare permanentemente chiusa
senza che nessuno se ne accorga.

## 10. Test di attacco per M-02 e M-03

Definizione, non esecuzione. Ogni test dichiara preparazione, procedura e **criterio
di superamento binario**, perché AGENT-001 e AGENT-002 possano implementarlo senza
reinterpretarlo. Dove il criterio dipende da un parametro non ancora tarato, il
parametro è nominato e il test verifica la *relazione*, non un valore.

> **Convenzione di scrittura dei criteri di superamento** (aggiunta 2026-08-25,
> AGENT-007, [REVIEW-011] RF-004). Un criterio di superamento deve essere una
> **disuguaglianza contro un parametro pubblicato** oppure una **proprietà imposta da
> una regola di validità nominata**. Un criterio che esprime un esito desiderato
> senza nominare la regola che lo produce non è un criterio: è un'aspirazione, e va
> marcato come tale finché non lo diventa.
>
> La convenzione è scritta perché la sua assenza è già costata due volte, entrambe su
> `AT-10`: «la coalizione non arriva mai al 100 % sotto i due terzi» e «l'attaccante
> non raggiunge 1/3 entro 50 epoche» sono entrambe affermazioni assolute su una
> grandezza *emergente*, formulate prima che esistesse la regola che quella grandezza
> produce, e nessuna delle due nomina una regola. Un criterio così non è falsificabile
> in fase di scrittura — lo è solo quando la simulazione lo smentisce, e a quel punto
> **lo smentisce addosso a chi implementa** invece che a chi ha scritto la specifica.
> Il modo di fallire era già nominato in questo documento e la nota da sola non è
> bastata.
>
> Seguito registrato: la convenzione va passata **una volta su tutti gli `AT-*`
> esistenti**, per scoprire eventuali terze occorrenze prima che sia una simulazione a
> farlo. Non è stato fatto in [SPEC-007], che aveva scope diverso.

| ID | Nome | Milestone | Owner |
| --- | --- | --- | --- |
| `AT-01` | Confine della soglia di quorum | M-02 | AGENT-002 |
| `AT-02` | Vettori di conformità Ed25519 | M-02 | AGENT-001 |
| `AT-03` | Catena alternativa di lungo raggio | M-02 | AGENT-002 |
| `AT-04` | Saldo obsoleto spacciato per corrente | M-02 | AGENT-002 |
| `AT-05` | Replay fra reti omonime | M-02 | AGENT-001 |
| `AT-06` | Malleabilità della prova Merkle sparsa | M-02 | AGENT-001 |
| `AT-07` | Flotta emulata contro il reddito di esistenza | M-03 | AGENT-002 |
| `AT-08` | Prove di enrollment precomputate | M-03 | AGENT-001 |
| `AT-09` | Equivocazione del cartello di validatori | M-02 | AGENT-002 |
| `AT-10` | Cattura dell'elezione dei validatori | M-02 | AGENT-002 |
| `AT-11` | Abbonati fittizi e quota al creatore | M-02 | AGENT-002 |
| `AT-12` | Collusione emittente/soggetto (parte A) e custode parziale (parte B) | M-03 (A), M-05 (B) | AGENT-002 |
| `AT-13` | Esaurimento della cache anti-replay | M-02 | AGENT-001 |
| `AT-14` | DoS in verifica dell'enrollment | M-03 | AGENT-001 |
| `AT-15` | Grinding del timestamp per invalidare blocchi | M-02 | AGENT-002 |

**`AT-01` — Confine della soglia di quorum.** *Preparazione:* set di validatori con
potere di voto totale `V` ∈ {100, 101, 102}. *Procedura:* costruire certificati di
quorum con potere firmatario 66, 67, 68 e 69 per ciascun `V`. *Superamento:* la
tabella di esito coincide esattamente con `3 × potere_firmatario > 2 × V` — in
particolare a `V=101` il certificato da 67 **deve fallire** e quello da 68 passare.
Il test si applica ai tre punti in cui la soglia compare: certificato di enrollment,
QC di blocco, QC di transazione.

**`AT-02` — Vettori di conformità Ed25519.** *Preparazione:* i 12 vettori di
`ed25519-speccheck`. *Procedura:* sottoporli al verificatore di firme del nodo.
*Superamento:* l'esito di ciascuno coincide con quello dichiarato nella specifica; il
test fallisce anche se un vettore è *accettato* dove la specifica dice di rifiutarlo.
Nessun vettore può avere esito "dipende dalla libreria".

**`AT-03` — Catena alternativa di lungo raggio.** *Preparazione:* una devnet con
storia fino all'altezza `H`; le chiavi di consenso di un set attivo a un'altezza
passata `h ≪ H`, ormai ruotato fuori. *Procedura:* dall'header genuino `h−1`
costruire una catena alternativa `h..H'` firmata con quelle chiavi, con set successivi
interamente controllati; servirla (a) a un client appena installato, (b) a un client
con un'altezza fidata persistita `> h`. *Superamento:* entrambi rifiutano. Il caso (a)
rifiuta per età dell'ancoraggio, il caso (b) per non regressione, e **alza un allarme
di fork invece di sovrascrivere lo stato**.

**`AT-04` — Saldo obsoleto.** *Preparazione:* un account con saldo 10⁶ all'altezza
4000 e 0 all'altezza 5000, dopo un burn. *Procedura:* un server risponde a
`balance_proof_request` per `at_height = 5000` con header, certificato e prova
genuini dell'altezza 4000. *Superamento:* il client rifiuta perché
`header.height ≠ at_height`; e se `at_height` fosse inferiore alla massima altezza già
fidata, rifiuta la richiesta stessa.

**`AT-05` — Replay fra reti omonime.** *Preparazione:* due catene con lo stesso
`network_id` e genesi diverse. *Procedura:* far firmare alla vittima un burn di
abbonamento sulla catena A e ripresentare i byte identici sulla catena B.
*Superamento:* la catena B rifiuta la transazione per `chain_id` non corrispondente.

**`AT-06` — Malleabilità della prova.** *Preparazione:* una `BalanceProof` valida in
cui almeno un sibling coincide con il default della sua profondità. *Procedura:*
trasmetterla nelle due codifiche (bit a 1 con valore esplicito, bit a 0 con default).
*Superamento:* la codifica con sibling esplicito uguale al default è **rifiutata**;
esiste una sola codifica valida per ogni prova.

**`AT-07` — Flotta emulata contro il reddito di esistenza.** *Il test che sostituisce
la metrica attuale di [[PROJECT]]; la sua formulazione dipende dalla decisione di §7.*
*Preparazione:* una devnet con `H` nodi onesti reali (`H ≥ 100`, con almeno un
Android e un headless) e un singolo host che esegue `N = 10.000` identità enrollate
con la difficoltà di produzione scelta. *Procedura:* far girare la rete per almeno 10
epoche di ricompensa con la flotta che risponde a tutte le challenge di availability.
*Misure obbligatorie da riportare:* (a) emissione totale per epoca, con e senza la
flotta; (b) quota dell'emissione totale ottenuta dalla flotta; (c) accrediti ottenuti
dalla flotta nelle categorie `storage` e `compute`; (d) seggi di validatore ottenuti
dalla flotta; (e) tempo e costo hardware impiegati per produrre le `N` identità.
*Superamento:* (a) invariata entro l'errore di misura; (c) esattamente **zero**;
(d) esattamente **zero**; (b) non superiore alla soglia dichiarata nella metrica
scelta. Il test **non** può essere superato dichiarando che (b) è zero: la misura va
riportata comunque, ed è il numero che il progetto deve pubblicare.

> **Valutazione contro i valori tarati di [SPEC-007] (2026-08-25, AGENT-002; aggiornata
> dopo [REVIEW-011]).** Esito: **parzialmente coperto** — superato su tutti e quattro i
> criteri **al regime d'uso di riferimento**, non valutabile con il criterio (c) alla
> lettera al regime di lancio, dove il criterio applicabile è quello assoluto. Le
> misure di riferimento, con `α = 0,15`, `X` dichiarato
> pari al **20 %** e `H = 100` contro `N = 10.000`. Misure: (a) emissione totale
> 105.882.352.900 µt senza flotta contro 105.882.351.000 µt con la flotta — non
> aumentata, e leggermente **minore** per il resto della divisione intera che il fondo
> a tetto scarta; (b) quota della flotta **14,851 %**, sotto `X`; (c) accrediti in
> `storage` e `compute` **zero**; (d) seggi di validatore **zero**. Il criterio (d)
> tiene per la regola e non per fortuna: `contribution_score` conta l'evidenza
> `availability` come zero, quindi una flotta che si limita a firmare ha punteggio 0 e
> fallisce la condizione di eleggibilità 3 a qualunque soglia positiva.
>
> *Il numero che il progetto deve pubblicare, e che questo test rende obbligatorio:*
> sotto quell'attacco il nodo onesto di sola availability conserva lo **0,99 %** del
> proprio reddito. Quel rapporto è `H/(N+H)` e **non contiene `α`**: abbassare `α`
> riduce la percentuale catturata dall'attaccante e riduce il reddito dell'onesto dello
> stesso fattore, quindi non migliora di nulla la perdita dell'utente. È la
> qualificazione che mancava alla leva di §6.2 ed è portata qui perché la misura (b),
> presa da sola, la nasconde.
>
> *Il regime di lancio, che è quello in cui il test verrà eseguito* ([REVIEW-011]
> RF-002). Lo stesso banco lungo la rampa d'uso, misurato dal simulatore:
>
> | `W` (cr/ep) | `α` | quota della flotta | dirottato (cr/ep) | criterio (c) alla lettera |
> | --- | --- | --- | --- | --- |
> | 0 | 1,0000 | 99,01 % | 15 725 | **violato** |
> | 4 500 | 0,7792 | 77,15 % | 15 725 | **violato** |
> | 22 500 | 0,4138 | 40,97 % | 15 725 | **violato** |
> | 90 000 | 0,1500 | 14,85 % | 15 725 | tenuto |
>
> Il criterio (c) alla lettera è violato di circa cinque volte per tutto
> l'avviamento se valutato contro la banda di `α`.
>
> **La forma corretta del claim è una garanzia incondizionata più una proprietà
> aggiuntiva, non due fasi disgiunte** ([ADR-011], [SPEC-009], corretta da
> [REVIEW-014] RF-008). *Una flotta non può dirottare più di
> `D = F · N/(N+H) < F` per epoca, **a ogni** livello d'uso, perché `D` non
> contiene `W`: la copertura è totale e non ha buchi.* Al di sopra del 70,6 %
> dell'uso di riferimento vale **in più** la banda `α ∈ [0,10 – 0,20]` con
> `X = 20 %`; al di sotto quella banda **non è sospesa, è falsa**, perché
> `α → 1` per costruzione. La formulazione precedente elencava due fasi, e un
> lettore al 40 % dell'uso non trovava la propria in nessuna delle due.
>
> *Una correzione a ciò che [SPEC-007] aveva detto con leggerezza, ripristinata
> qui dopo che [REVIEW-014] RF-006 ha rilevato che era stata cancellata fuori
> ambito.* La quarta colonna della tabella **non si muove**, perché
> `D = F · N/(N+H)` non contiene `W`. «Il 91 % di un'emissione minuscola è
> un'emissione minuscola» vale solo se anche `F` è piccolo, e `F` è una scelta di
> governance: con un `F` dimensionato per 10 000 nodi una flotta al lancio
> dirotta circa 15 725 cr per epoca, quasi l'intero fondo. È esattamente la
> ragione per cui [ADR-011] impone di dimensionare `F` di genesi sulla
> popolazione onesta attesa al lancio, e non è una prassi facoltativa.
>
> *Le tre regole di validità chiuse da [ADR-010] e [SPEC-009], con ciò che
> ciascuna non chiude:*
> 1. `availability_microtokens_per_unit == 0` è una **regola di validità** in
>    accettazione: un documento con tariffa positiva è rifiutato, il che
>    garantisce il criterio (a) di [ADR-007] per costruzione. **Chiusa senza
>    residui**, verificata da AGENT-007 cercando vie alternative e non
>    trovandone.
> 2. `RewardBounds` nell'ancora di fiducia di genesi fissa il tetto di `F`, il
>    pavimento e il tetto della durata dell'epoca, i pavimenti di eleggibilità,
>    i tetti dei fattori di conversione, i pavimenti delle tariffe di lavoro, il
>    rapporto 5/4 e il gap di attivazione. La prima versione vincolava solo le
>    grandezze nominate dalle ADR e lasciava governati i loro **denominatori e
>    le loro unità** ([REVIEW-014] RF-002, RF-003, RF-004): la durata dell'epoca
>    è il denominatore di ogni tetto per epoca, i fattori di conversione
>    denominano l'unità del pavimento di eleggibilità, e le tariffe di lavoro
>    sono il denominatore di `α`.
> 3. Il tetto proporzionale `F = k · E` è **esplicitamente rifiutato** per
>    evitare l'inflazione del denominatore da parte della flotta stessa.
>
> `AT-07` resta **parzialmente coperto**: superato al regime d'uso di
> riferimento, e coperto al regime di lancio dalla sola garanzia assoluta, che è
> la garanzia corretta lì.

**`AT-08` — Prove precomputate.** *Preparazione:* un parameter set futuro non ancora
attivo. *Procedura:* (i) minare prove per il set futuro e presentarle all'attivazione;
(ii) minare prove valide, attendere oltre la finestra di freschezza e presentarle
rifirmando la richiesta con timestamp corrente. *Superamento:* entrambe rifiutate —
la (i) perché l'hash del set futuro non è disponibile prima dell'attivazione, la (ii)
perché il `block_id` legato alla prova è fuori finestra.

**`AT-09` — Equivocazione del cartello.** *Preparazione:* una devnet in cui il test
controlla una frazione parametrizzata del potere di voto. *Procedura:* con il 33 % e
poi con il 34 %, tentare di far finalizzare due blocchi in conflitto alla stessa
altezza; includere una variante con un validatore revocato che continua a votare.
*Superamento:* al 33 % nessuna coppia di certificati in conflitto è accettata da alcun
nodo o light client; al 34 % la rete si ferma (perdita di liveness) e **non** produce
due storie finalizzate; il voto del validatore revocato non è conteggiato a partire
dalla sua `effective_height`; l'equivocazione produce una transazione di evidenza
verificabile da chiunque.

> **Valutazione contro la regola di [SPEC-006] (2026-08-25, AGENT-002).** Esito:
> **parzialmente superato in specifica, non superabile per intero**. I primi tre
> criteri non sono toccati dalla regola di elezione: la soglia di safety resta 1/3
> come da `SEC-REQ-02`, e il non conteggio del validatore revocato resta la regola di
> `ledger.md` §"Revocation forces a validator set transition", che [SPEC-006] non
> modifica. La regola aggiunge però due fatti rilevanti per la variante con
> validatore revocato: una transizione fuori dal confine di epoca è valida **solo se
> rimuove**, con `validators` sottoinsieme stretto del set precedente e ogni voce
> rimossa coperta da una `revoke_identity` finalizzata, quindi la variante non può
> essere usata per insediare un sostituto; e i seggi liberati dalla revoca sono
> ripresi al confine successivo **sotto il tetto ordinario**, quindi una revoca di
> massa non si converte in un'ammissione di massa. Quella frase, da sola, era vera e
> insufficiente ([RF-002] di [REVIEW-010]): una revoca di massa non si converte in
> ammissione ma **si convertiva in concentrazione**, che per un avversario già dentro
> è equivalente e costa meno, perché rimuovere gli altri *è* scegliere il set. Il
> **pavimento di contrazione** `3 * member_count(nuovo) > 2 * member_count(vecchio)`
> vale ora anche per le transizioni di sola rimozione, quindi la variante con
> validatore revocato non produce né ammissione né concentrazione: oltre il pavimento
> non esiste set valido e la catena si ferma. Il quarto criterio — "l'equivocazione
> produce una transazione di evidenza verificabile da chiunque" — **non è coperto**:
> è la seconda metà di `SEC-REQ-13`, esplicitamente fuori dallo scope di [SPEC-006]
> (`Excluded`: slashing reputazionale, M-07). `AT-09` non può quindi essere dichiarato
> superato finché quella transazione non esiste, e la parte mancante è nominata qui
> perché non sia scambiata per coperta.

**`AT-10` — Cattura dell'elezione.** *Preparazione:* simulazione con `H` candidati
onesti dal profilo di uptime realistico di §6.1 e `N` candidati d'attaccante con
uptime da datacenter, per `N/H` ∈ {0,1, 1, 10}. *Procedura:* eseguire la regola di
elezione scelta per 50 epoche, con e senza tetto di rotazione, e con e senza soglia di
eleggibilità ancorata a lavoro dimostrato. *Misure:* epoche fino al raggiungimento di
1/3 del potere di voto da parte dell'attaccante, per ciascuna configurazione.
*Superamento:* con la configurazione scelta per la produzione, l'attaccante non
raggiunge 1/3 entro 50 epoche in nessuno dei tre rapporti `N/H`; e la deriva della
composizione del set è osservabile a ogni epoca da un light client.

> **Valutazione contro la regola di [SPEC-006] (2026-08-25, AGENT-002).** Esito:
> **eseguibile, e il secondo criterio è già soddisfatto; il primo non è decidibile
> qui.**
>
> *Secondo criterio, osservabilità della deriva: soddisfatto per costruzione.* Il
> light client possiede in chiaro entrambi i documenti `ValidatorSet` a ogni confine e
> ne calcola la differenza; verifica inoltre `filled_count` contro
> `validator_churn_cap_seats`, il limite di mandato da `seated_since_epoch`, e che
> `next_validator_set_hash` non cambi fuori dai confini. La deriva non è una statistica
> pubblicata da un operatore, è una quantità che il client ricava dai dati che già
> scarica.
>
> *Primo criterio: la regola lo rende raggiungibile ma il verdetto dipende dai
> parametri.* Il tempo di cattura non è più una proprietà emergente della classifica
> ma una disuguaglianza: con `V = validator_target_set_size` e
> `c = validator_churn_cap_seats`, un attaccante che domini il pool di riempimento
> impiega almeno `ceil((V/3)/c)` confini a raggiungere 1/3 **per ammissione**, e il
> documento dei parametri di consenso è **rifiutato in accettazione** se `3*c*m > V`,
> dove `m` è l'orizzonte di cattura dichiarato `validator_min_capture_epochs`. La
> qualificazione «per ammissione» è necessaria e la sua assenza era un difetto
> ([RF-002] di [REVIEW-010]): la cattura **per attrito** non passa dall'ammissione e
> non è misurata da quella disuguaglianza. Con il pavimento di contrazione ha una
> propria disuguaglianza, `ceil(log(V/k) / log(3/2))` confini per contrarre da `V` a
> `k` — **tre** per `k` vicino a `V/3` — e non è tarabile da alcun parametro. La
> coalizione sotto i due terzi ci arriva comunque, con censura selettiva invece che
> totale; ciò che il pavimento le toglie è di arrivarci in un confine solo e senza
> pubblicare nulla. Il criterio di superamento di `AT-10` ("non raggiunge 1/3 entro 50 epoche")
> è esprimibile come una scelta di `m`, ma i valori di `V`, `c`, `m` e della soglia
> di eleggibilità vengono dal simulatore economico di M-02 ([DEBT-007]): **il
> verdetto numerico di `AT-10` resta rinviato alla simulazione, ed è corretto che lo
> sia.**
>
> *Nota sulla preparazione del test.* Il profilo "candidati d'attaccante con uptime da
> datacenter" non è più il vettore giusto: l'uptime non concorre all'eleggibilità e
> l'evidenza `availability` contribuisce **zero** al `contribution_score`. La
> preparazione va aggiornata a candidati d'attaccante che forniscono storage e compute
> reali sopra la soglia, che è un costo diverso e va misurato come tale. Servono
> inoltre tre configurazioni che la preparazione originaria non prevedeva:
>
> 1. **macinatura del seme.** Un attaccante che proponga l'ultimo blocco della
>    finestra di entropia e cerchi il seme migliore, per verificare che il guadagno
>    resti limitato dal tetto e non dal seme.
> 2. **cattura per attrito.** Una coalizione a `k > V/3` che censura le candidature
>    altrui, misurando i confini necessari ad arrivare al 100 % del potere. Vanno
>    eseguite **entrambe** le varianti, perché danno risultati diversi e solo la
>    seconda è il vettore reale: (2a) censura **totale**, che il pavimento di
>    contrazione rifiuta — misura attesa: la catena si ferma al confine, la
>    coalizione non ottiene il set; (2b) censura **selettiva**, in cui la
>    coalizione lascia passare esattamente le candidature oneste che portano il set
>    al minimo consentito dal pavimento — misura attesa:
>    `ceil(log(V/k)/log(3/2))` confini, cioè **tre** per `k` vicino a `V/3`, con
>    ogni confine pubblicato. Un criterio di superamento che dicesse "la coalizione
>    non arriva mai al 100 % sotto i due terzi" sarebbe **sbagliato** e verrebbe
>    smentito dalla simulazione: era la formulazione precedente di questa riga ed è
>    corretta qui, perché un criterio di test errato viene attribuito
>    all'implementazione invece che alla specifica.
> 3. **evasione del cooldown.** Incumbent che escono volontariamente un'epoca prima
>    della scadenza del mandato, misurando l'assenza effettiva contro
>    `validator_cooldown_epochs`. Con la condizione di eleggibilità 5 nella forma
>    «uscita per qualunque ragione» la misura attesa è `validator_cooldown_epochs`;
>    con la formulazione precedente, limitata alla scadenza del mandato, era **una**
>    epoca, e il confronto fra le due misure è ciò che rende il test utile.

> **Verdetto numerico di [SPEC-007] (2026-08-25, AGENT-002).** Il verdetto rinviato
> dalla valutazione precedente è emesso qui, con `V = 27`, `c = 3`, `T = 9`, `m = 3` e
> `validator_min_set_size = 18`. Esito: **superato su cinque criteri su sei; il sesto
> non è soddisfacibile da alcuna rete operabile e va corretto, non ritarato.**
>
> *Configurazione 1, macinatura del seme — superata.* Attaccante che fornisce storage e
> compute reali appena sopra la soglia, con `G = 128` ricampionamenti dell'ultimo blocco
> della finestra, e **solo ai confini in cui detiene lo slot di proposta**, perché solo
> un proposer può macinare. Riempimenti ottenuti su 50 confini, senza e con macinatura:
> 12 → 13 a `N/H = 0,1`; 32 → 36 a `N/H = 1`; 31 → 27 a `N/H = 10`. Il guadagno resta
> **sotto il tetto di rotazione di un singolo confine su 50 confini**: la macinatura
> produce bias e mai scelta, che è esattamente ciò che `ledger.md` afferma.
>
> *Configurazione 2a, censura totale — superata.* Coalizione a 10 seggi su 27 (37,0 %):
> la catena **si ferma al primo confine** sul pavimento di contrazione (`3·8 ≤ 2·27`).
> La coalizione ottiene un arresto, mai il set.
>
> *Configurazione 2b, censura selettiva — superata, e con un risultato che il documento
> non prevedeva.* Le misure: `k = 10` (37,0 %) e `k = 17` (63,0 %) contraggono il set
> `27 → 19 → 18` e poi **si bloccano**, perché il pavimento e
> `validator_min_set_size = 18` vietano un set più piccolo; la coalizione detiene
> rispettivamente il 55,6 % e il 94,4 % del set e **non ottiene mai l'intero set**.
> Solo da `k = 18 = 2V/3` in su la cattura si completa, in **2 confini** a `k = 18` e in
> **1** a `k = 19`. Ne segue che `validator_min_set_size` **sta facendo lavoro
> anti-cattura che `ledger.md` attribuisce al solo pavimento di contrazione**: la
> conclusione «la soglia effettiva di cattura resta appena sopra un terzo» è esatta del
> pavimento preso da solo, e portare il minimo del set a `2V/3` la alza a due terzi,
> punto oltre il quale la safety BFT è già caduta e nessuna regola di composizione
> stava più promettendo niente.
> **Attuazione della regola in [SPEC-009] ([ADR-010]), e la sua correzione in
> [REVIEW-014] RF-001.** Il vincolo relazionale
> `3 · validator_min_set_size >= 2 · V` è ora nel blocco di vincoli dei parametri
> di consenso, quindi `min_set >= ceil(2V/3)` in ogni documento valido. **Ciò che
> il vincolo garantisce è il possesso, non il controllo, e la differenza è
> grande.** Una coalizione sotto i due terzi non può arrivare a **detenere ogni
> seggio**; può però ottenere il **quorum** sul set contratto, che è la proprietà
> che conta, e per farlo le basta molto meno.
>
> Misurato sui valori raccomandati `V = 27`, `min_set = 18`: una coalizione di
> **13 seggi, il 48,1 %**, censura selettivamente lasciando passare 6 candidature
> oneste; il set si contrae a **19** — il pavimento è **stretto**, quindi 27
> scende a 19 e non a 18, `3·19 = 57 > 54`, e `18 ≤ 19` — e il blocco è valido,
> firmato dagli onesti perché nulla lo distingue da uno onesto. A quel punto
> `13·3 = 39 > 19·2 = 38`: **quorum ottenuto senza possedere il set**. Con il
> quorum la coalizione abbassa `V` e `min_set` insieme dentro il rapporto 5/4 e
> arriva al possesso integrale in **tre confini**.
>
> | `V` | `min_set` | `S_new` | `k_min` per il quorum | frazione di `V` |
> | --- | --- | --- | --- | --- |
> | 12 | 8 | 9 | 7 | 58,3 % |
> | 27 | 18 | 19 | 13 | 48,1 % |
> | 36 | 24 | 25 | 17 | 47,2 % |
> | 60 | 40 | 41 | 28 | 46,7 % |
> | 600 | 400 | 401 | 268 | 44,7 % |
>
> con `k_min = max(floor(2·S_new/3)+1, floor(V/3)+1)` e
> `S_new = max(floor(2V/3)+1, min_set)`, decrescente in `V` e con asintoto
> **`4/9` = 44,4 %** dall'alto. **Il guadagno del vincolo è reale e va rivendicato
> per quello che è: da «appena sopra un terzo» a circa quattro noni. Non a due
> terzi.** L'argomento «sopra i due terzi la safety BFT è comunque già caduta»
> vale per il possesso e **non** per il quorum, perché a `4V/9` la safety non è
> caduta affatto: è lo stesso passo logico che era sbagliato nelle due
> ritrattazioni precedenti, ed è la terza volta che questa affermazione viene
> confutata. Il numero 19 era peraltro già scritto in questa stessa nota, nella
> nota metodologica sotto.
>
> Nota metodologica: la formula continua predice meno confini della simulazione
> perché il pavimento è **stretto** — un set di 27 scende a 19, non a 18 — e la
> cifra misurata, mai inferiore a quella della formula, è quella da citare.
>
> **Il prezzo in liveness, rimisurato sui parametri raccomandati** e non
> ereditato da una passata che assumeva un minimo diverso ([REVIEW-014] RF-005):
> con `V = 27`, `T = 9`, `c = 3`, `cooldown = 2`, `min_set = 18`, una rete con
> pool di candidati vuoto perde 3 seggi per confine e sopravvive **tre confini**;
> al quarto il successore sarebbe 15, sotto `min_set`, e la catena si ferma. E il
> margine di contrazione è **speso una volta sola**: dopo la contrazione massima
> `27 → 19` il successore lecito più piccolo è 18, cioè il pavimento stesso, e il
> confine seguente non tollera più alcuna uscita non rimpiazzata. Il costo è
> dichiarato accanto alla regola in `ledger.md`.
>
> *Configurazione 3, evasione del cooldown — superata.* Con
> `validator_cooldown_epochs = 2`, l'assenza misurata è di **2 epoche** sia dopo la
> scadenza del mandato sia dopo un'uscita volontaria un'epoca prima. Con la
> formulazione precedente della condizione 5 l'uscita volontaria avrebbe misurato
> **una** epoca: il confronto fra le due misure conferma che la riformulazione «per
> qualunque ragione» fa il lavoro per cui è stata scritta.
>
> *Osservabilità della deriva — superata per costruzione*, come già stabilito.
>
> **Il criterio che fallisce, e perché va corretto invece che ritarato.** «L'attaccante
> non raggiunge 1/3 entro 50 epoche» è **fallito** a `N/H = 1` (6 confini) e a
> `N/H = 10` (4 confini). Non è un fallimento di taratura e **nessuna combinazione di
> parametri lo ripara.** Il tempo per raggiungere un terzo **per ammissione** è
> `ceil((V/3)/c)` confini nel caso migliore per l'attaccante, e il blocco di vincoli
> impone `3·c·m ≤ V`, cioè `(V/3)/c ≥ m`. Il criterio letterale è quindi la richiesta
> `m ≥ 50`, che per `T ≥ 3m` forza `T ≥ 150` e `c ≤ V/150`: un set di almeno 150
> validatori che ruota **un** seggio per confine con mandati di 150 confini, cioè circa
> tre anni a confini settimanali. Non è una rete operabile, ed è l'esatto contrario
> dell'istruzione di [DEBT-010] di tenere il limite di mandato stretto quanto la rete
> tollera. A ciò si aggiunge che l'orizzonte per **attrito** è fisso a tre confini e non
> si allunga con alcun parametro, quindi anche un `m` enorme non comprerebbe il tempo
> che il criterio chiede: lo comprerebbe su uno solo dei due percorsi.
>
> Questa è la **seconda** volta che un criterio di superamento di `AT-10` risulta
> sbagliato invece che non soddisfatto — la prima era «la coalizione non arriva mai al
> 100 % sotto i due terzi» — e la ragione per correggerlo è la stessa già scritta qui
> sopra: *un criterio di test errato viene attribuito all'implementazione invece che
> alla specifica*. La formulazione che la regola garantisce davvero, e che un light
> client può verificare, è:
>
> > *Il tempo per raggiungere un terzo del potere di voto per ammissione non è
> > inferiore ai `validator_min_capture_epochs` confini dichiarati; il tempo per
> > raggiungerlo per attrito non è inferiore a `ceil(log(V/k)/log(3/2))` confini; la
> > cattura per attrito è impossibile per una coalizione più piccola di
> > `validator_min_set_size`; e la deriva della composizione è calcolabile da un light
> > client a ogni confine.*
>
> **La correzione non è applicata qui.** Questo documento è di AGENT-007 e il criterio è
> una scelta di prodotto sulla perdita dichiarata: [SPEC-007] emette il verdetto e
> registra la proposta, e la decisione spetta al Lead e all'operatore, eventualmente con
> un'ADR. Nessuna regola di protocollo cambia in nessuno dei due casi.

> **Revisione di sicurezza di [SPEC-007] (2026-08-25, AGENT-007, [REVIEW-011]).** Due
> qualificazioni alle misure sopra, entrambe verificate rieseguendo il simulatore del
> progetto con parametri diversi da quelli raccomandati.
>
> *La chiusura della cattura per attrito sotto `2V/3` è una proprietà della
> combinazione raccomandata, non una regola* ([REVIEW-011] RF-003). Il blocco di
> vincoli impone `0 < validator_min_set_size <= V` e **nessuna regola lega il minimo a
> `V`**, mentre la proprietà dipende dal loro rapporto. `validator_min_set_size_min = 18`
> impedisce di abbassare il minimo, ma non di alzare `V`: con `c` e `m` congelati a 3
> dal limite 5/4, `ceil(V/T) <= c` e `T <= 12` danno `V <= 36`, e **`V = 36` è
> raggiungibile in due documenti leciti** (`V: 27 → 33 → 36`, `T: 9 → 11 → 12`),
> distanziati da un'epoca di elezione, cioè circa 14 giorni. `check_constraint_block`
> accetta lo stato finale riga per riga. Lì `2V/3 = 24 > 18`, e la simulazione di
> censura selettiva dà cattura completa dell'intero set in 2 confini già a `k = 18`,
> cioè al **50,0 % di `V`**. La soglia scende quindi da due terzi a una metà, e con
> essa cade la rassicurazione che la reggeva: a `V = 27` la cattura si completava solo
> oltre il punto in cui la safety BFT è già persa; a `V = 36` si completa dove non lo è
> ancora. La chiusura proposta è un vincolo di accettazione nuovo,
> `3 * validator_min_set_size >= 2 * V`, soddisfatto con uguaglianza dalla combinazione
> raccomandata (`54 >= 54`) e quindi a costo zero oggi. È lavoro di protocollo e
> un'ADR, fuori dallo scope di [SPEC-007].
>
> *`AT-07` è superato al regime d'uso di riferimento e non al regime di lancio*
> ([REVIEW-011] RF-002). `α = F/(F+W)` è **osservata**, e al lancio `W ≈ 0` implica
> `α ≈ 1` qualunque sia `F`: una flotta di 10 000 identità contro 100 nodi onesti
> cattura allora circa il 99 % dell'emissione, contro il `X = 20 %` dichiarato dal
> criterio (c) di [ADR-007]. `AT-07` è schedulato in M-03 su devnet, cioè proprio in
> quel regime. Finché la soglia d'uso non è scritta dentro la formulazione di `X`, la
> riga `AT-07` della matrice va letta come **parzialmente coperta**: superata sopra la
> soglia d'uso dichiarata, non valutata sotto, dove la grandezza onesta è l'importo
> assoluto dirottato e non il rapporto.

### Soglia di partecipazione: dove la rete si ferma senza avversario

*Aggiunta 2026-08-25, AGENT-007, [REVIEW-011] RF-007. Misure da
`knowledge/economic-simulation-report.md` §6.*

È la sola condizione nota in cui la rete perde liveness **senza che nessuno l'abbia
attaccata**, e una perdita di disponibilità è dentro il perimetro di questo documento
quanto una cattura. Alla combinazione raccomandata (`V = 27`, `T = 9`, `c = 3`,
`cooldown = 2`, `validator_min_set_size = 18`):

| pool stabile di candidati | esito |
| --- | --- |
| 0 | arresto in **3** confini, sotto `validator_min_set_size` |
| 24 | arresto in **11** confini |
| **30** | nessun arresto, il set si assesta a 21 seggi |
| **36** (minimo aritmetico) | nessun arresto, tutti e 27 i seggi |

**Grandezza da sorvegliare:** un pool stabile `>= 30` per non arrestarsi e `>= 36`
per tenere `V` seggi — alla rete di riferimento, circa il **3 % dei contribuenti**
disposto a candidarsi. Il minimo aritmetico 36 è `V` seduti più `ceil(V/T) = 3` in
cooldown più 3 liberi per il riempimento.

Due note che rendono il numero più fragile di come appare:

- **il cooldown moltiplica l'effetto di una censura sul pool.** Censurare una
  candidatura per una sola epoca rimuove quel nodo per `1 + validator_cooldown_epochs`
  epoche, quindi il pool *efficace* sotto pressione avversariale è più piccolo di
  quello nominale, proprio nella direzione in cui non c'è margine;
- **la manopola correttiva non esiste.** `validator_churn_cap_seats = 3` e
  `validator_cooldown_epochs = 2` sono **congelati per sempre** dal limite di
  variazione 5/4 applicato a interi piccoli (intervallo lecito `[3,3]` e `[2,2]`): una
  rete che si scoprisse il pool troppo sottile non può né alzare il tetto di
  riempimento né accorciare il cooldown. La sola mossa residua è alzare `T`, che è il
  cricchetto irreversibile di [DEBT-010].

La decisione se promuovere questa soglia a `SEC-REQ` con un test d'attacco dedicato
spetta al Lead; qui è registrata perché finora esisteva solo in un rapporto di
taratura, e nessuno l'avrebbe osservata.

**`AT-11` — Abbonati fittizi.** *Preparazione:* un publisher che controlla `N`
identità enrollate e un'app con `microtokens_per_period = S`. *Procedura:* abbonare
tutte le `N` identità e far girare la rete per almeno 5 epoche di ricompensa.
*Misure:* saldo netto complessivo dell'attaccante prima e dopo; `publisher_reward`
emesso; `active_subscriber_count` pubblicato. *Superamento:* il saldo netto
complessivo è **strettamente decrescente** (il ciclo è in perdita); il vincolo
`publisher_reward ≤ k × Σ burn` è imposto in consenso e una fixture al confine di `k`
è rifiutata quando lo supera. *Misura da riportare senza criterio di superamento:*
il numero di epoche per cui il reddito di esistenza delle `N` identità sostiene la
perdita — è la grandezza di §6.3 e il progetto deve conoscerla.

**`AT-12` — Collusione e custode parziale.** *Parte A:* un emittente colluso sceglie
la `randomness` in modo da selezionare l'unico chunk che il soggetto ha conservato.
*Superamento A:* l'evidenza è rifiutata perché la `randomness` non si ricalcola dalla
sorgente finalizzata più il segreto rivelato, il cui impegno era pubblicato prima
della selezione del soggetto. *Parte B:* un custode conserva la frazione `x` dei
chunk, per `x` ∈ {0,01, 0,1, 0,5, 0,9, 1}. *Superamento B:* il compenso atteso è
**strettamente crescente** in `x` su tutti i valori misurati; nessuna frazione
parziale è più conveniente della custodia completa a parità di risorse.

**`AT-13` — Esaurimento della cache anti-replay.** *Preparazione:* 100 identità
enrollate distinte. *Procedura:* ciascuna invia envelope validi e canonici con
`expires_at_ms` al massimo consentito e nonce casuali, al ritmo massimo accettato.
*Superamento:* la memoria del ricevente resta sotto il limite dichiarato; il
comportamento in saturazione è quello scritto nella specifica (rifiuto esplicito, non
accettazione silenziosa); un nodo Android sotto lo stesso carico non viene terminato
dal sistema operativo.

**`AT-14` — DoS in verifica dell'enrollment.** *Applicabile solo se si adotta
l'opzione 2 o 4 di §7.* *Procedura:* inviare una raffica di `EnrollmentRequest`
sintatticamente valide ma con proof of work errato, e una seconda raffica con firma
errata. *Superamento:* il consumo di CPU e memoria del validatore resta lineare nel
numero di richieste **con costante piccola**, perché la funzione memory-hard è
eseguita solo dopo schema, firma, `network_id`, timestamp, unicità e rate limit; il
nodo continua a servire il consenso senza degrado misurabile.

**`AT-15` — Grinding del timestamp.** *Preparazione:* un account con due burn
consecutivi, nonce `n` e `n+1`. *Procedura:* (i) verificare che il blocco sia valido
qualunque sia l'ordine relativo dei `tx_id`; (ii) tentare di macinare `created_at_ms`
della seconda transazione per invalidare il blocco di un proposer onesto.
*Superamento:* (i) il blocco è valido in entrambi gli ordini; (ii) nessun valore di
timestamp entro la finestra consentita produce un blocco invalido.

## 11. Conformità alle esclusioni permanenti di [[PROJECT]]

Verifica esplicita, richiesta dai criteri di accettazione di [SPEC-004].

| Esclusione permanente | Verifica |
| --- | --- |
| Convertibilità del token in denaro, exchange, ponti verso crypto | **Nessun `SEC-REQ` la introduce.** Nessuna contromisura di questo documento prevede depositi cauzionali in valuta, stake acquistabile, prezzi di mercato o aste. `SEC-REQ-15` e `SEC-REQ-18` **vincolano** l'emissione, non ne creano il valore. §7.2 registra che l'intera famiglia proof-of-stake è esclusa proprio da questo vincolo, e ne prende atto invece di aggirarlo |
| Trasferimenti diretti utente→utente | Nessun `SEC-REQ` li richiede né li presuppone. L'invariante 3 di `ledger.md` — nessuna transazione ha insieme sorgente e destinazione controllate dall'utente — è preservato da ogni contromisura proposta, inclusa la ponderazione degli abbonati di §6.3, che modifica una funzione di ricompensa e non introduce alcun percorso di trasferimento |
| iOS/macOS | Nessun `SEC-REQ` li riguarda. §7.5 tratta esclusivamente Android, Windows e Linux |
| Container OCI come runtime | Nessun `SEC-REQ` li introduce. `SEC-REQ-19` e `SEC-REQ-20` operano interamente dentro il modello a capability di [ADR-004] |
| Mining / proof-of-work continuo di qualsiasi tipo | **Verifica sostanziale, non formale.** §6.2.2 identifica il costo marginale ricorrente come la difesa anti-Sybil più efficace e la **scarta esplicitamente** per questa esclusione. Nessuna opzione di §7 la reintroduce: l'opzione 2 è una prova una tantum all'enrollment, non ricorrente; `SEC-REQ-18` ripartisce un fondo, non impone lavoro. `SEC-REQ-17` aumenta la frequenza delle challenge, e va sorvegliato perché è il requisito più vicino al confine: una challenge è una firma su un nonce, non una ricerca di preimmagine, ma se in futuro qualcuno proponesse di rendere *costosa* la risposta a una challenge, quella proposta ricadrebbe nell'esclusione |

Segnalo una tensione che **non** è una violazione di un'esclusione ma un conflitto con
lo scope dichiarato, e che quindi va decisa e non risolta da me: l'opzione 3 di §7
escluderebbe di fatto la piattaforma headless, che [[PROJECT]] §"Scope" include
esplicitamente fra le shell previste.

## 12. Raccomandazioni al Project Lead

Non modifico ADR, roadmap né [[PROJECT]]. Queste sono le decisioni che, a mio
giudizio, il threat model rende necessarie, in ordine di urgenza.

1. **La regola di elezione dei validatori va scritta in M-02, non in M-07.** È la
   raccomandazione più importante del documento. `ledger.md` dice esplicitamente che
   la continuità "does not specify how members are elected", e TM-18 mostra che in
   quello spazio vuoto il set corrente può rendersi permanente con una transizione
   valida e invisibile. Una devnet che accumuli storia sotto questa regola mancante
   non è un esperimento innocuo: è un precedente che sarà costoso correggere. La
   roadmap colloca la rotazione in M-07, il che è ragionevole per l'*automazione* ma
   non per l'*invariante*.
2. **Decidere §7 prima di tarare [ADR-005].** La scelta anti-Sybil determina il
   parametro `α`, e `α` determina l'esito del simulatore economico. Tarare prima di
   decidere significa tarare due volte.
3. **[ADR-002] merita una revisione sul punto dell'attestazione hardware.** Il testo
   la descrive come "anti-Sybil più forte", e §7.5 mostra che, alla documentazione
   ufficiale corrente, nella forma accessibile a un'applicazione P2P di consumo essa
   non fornisce alcuna unicità per dispositivo, e nel caso di Play Integrity non è
   nemmeno verificabile da un terzo. L'ADR ha preso la decisione giusta per le
   ragioni giuste, ma la caratterizzazione dell'alternativa scartata non regge, e
   lasciarla in piedi significa tenere sul tavolo un'opzione che non esiste.
4. **Serve una decisione esplicita sulla forma del reddito di esistenza:** importo per
   nodo, oppure fondo a tetto ripartito. §6.2.4 mostra che è la scelta che decide la
   gravità dell'intero problema Sybil, e oggi non è presa da nessuna parte —
   [ADR-005] parla di "emissione base contenuta" senza dire quale delle due forme sia.
   È materia da ADR, non da taratura.
5. **La privacy non ha un ADR.** È l'unica superficie di questo documento che non ha
   una decisione alle spalle, e §5.6 mostra che è già determinata dal design: la rete
   è pseudonima con pseudonimo stabile, e il ledger pubblica in chiaro gli abbonamenti
   di ciascuno. Raccomando un ADR dedicato **prima** che partecipi il primo utente
   esterno, perché i dati che si producono sono irreversibili.
6. **[ADR-006] va integrato su due punti:** la lista di blocco di rete deve diventare
   un oggetto di protocollo con evidenza, scadenza e appello (`SEC-REQ-21`); e la
   quota al creatore andrebbe valutata nella forma ponderata per contributo dimostrato
   (§6.3), che è l'unica che neutralizza gli abbonati fittizi senza rendere gli
   abbonamenti inaccessibili all'utente onesto.
7. **Il bound 18–40 di `identity.md` è legato alla primitiva** e la sezione `DRAFT` lo
   dichiara oggi non-draft. Se si adotta l'opzione 2 di §7, quel bound va riaperto
   insieme alla primitiva, e la difficoltà va espressa rispetto a un costo su
   dispositivo di riferimento.
8. **La metrica di [[PROJECT]] va riformulata**, e le quattro candidate sono in §7.
   È una decisione dell'operatore. Segnalo solo che nessuna delle quattro opzioni
   consente di mantenere la formulazione attuale, e che l'opzione che più le si
   avvicina è anche quella che escluderebbe una piattaforma di prodotto.

## 13. Fonti consultate

Le verifiche crittografiche e i benchmark di enrollment sono quelli condotti per
[REVIEW-002] su documentazione primaria corrente e non sono stati rifatti: RFC 8032
§5.1.7, *Taming the many EdDSAs* ([ePrint 2020/1244](https://eprint.iacr.org/2020/1244)),
[ed25519-speccheck](https://github.com/novifinancial/ed25519-speccheck),
[ZIP-215](https://zips.z.cash/zip-0215),
[RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html) per i profili Argon2id,
[le specifiche libp2p](https://github.com/libp2p/specs), e i benchmark pubblicati di
hashcat e dei core ARMv8 con estensioni SHA-2.

Per §7.5 ho consultato la documentazione ufficiale corrente, perché il comportamento
di queste piattaforme cambia nel tempo e non andava ricostruito a memoria:

- Play Integrity API — [panoramica](https://developer.android.com/google/play/integrity/overview),
  [verdetti](https://developer.android.com/google/play/integrity/verdicts),
  [configurazione e quote](https://developer.android.com/google/play/integrity/setup),
  [richieste classiche](https://developer.android.com/google/play/integrity/classic),
  [richieste standard](https://developer.android.com/google/play/integrity/standard),
  [device recall](https://developer.android.com/google/play/integrity/device-recall);
- Android key attestation — [guida per sviluppatori](https://developer.android.com/privacy-and-security/security-key-attestation),
  [AOSP key e ID attestation](https://source.android.com/docs/security/features/keystore/attestation),
  [attestazione degli ID in ambito enterprise](https://developer.android.com/work/versions/android-9.0);
- TPM su Windows — [TPM key attestation](https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/manage/component-updates/tpm-key-attestation),
  [come Windows usa il TPM](https://learn.microsoft.com/en-us/windows/security/hardware-security/tpm/how-windows-uses-the-tpm),
  [requisiti di Windows 11](https://learn.microsoft.com/en-us/windows/whats-new/windows-11-requirements),
  [Azure Attestation](https://learn.microsoft.com/en-us/azure/attestation/tpm-attestation-concepts).

Le tre affermazioni che **non** ho potuto confermare da fonte primaria sono elencate
in fondo a §7.5 e vanno verificate sperimentalmente prima di qualunque impegno
sull'opzione 3.

## 14. Manutenzione di questo documento

Va rivisto quando: cambia una delle decisioni di §12; viene accettato un ADR che
introduce un nuovo canale di emissione (`SEC-REQ-24`); un test di §10 fallisce in modo
che cambi la valutazione di uno scenario; o quando [REVIEW-002] passa da
`changes-requested` ad accettata, momento in cui gli scenari che vi si appoggiano
vanno riletti sui documenti corretti anziché su quelli attuali.






