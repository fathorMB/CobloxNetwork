# Rapporto del simulatore economico — v1

Autrice: AGENT-002 (Sofia Consenso) · Spec di origine: [SPEC-007] · Data: 2026-08-25
Debito chiuso: [DEBT-007] · Decisioni collegate: [ADR-005], [ADR-006], [ADR-007], [ADR-008], [ADR-009]

**Revisione v1.1 (2026-08-25), remediation di [REVIEW-011].** Nessun valore
raccomandato è cambiato: AGENT-007 ha giudicato i valori difendibili e ha contestato
le **affermazioni** che li accompagnavano. Le correzioni di questa revisione sono
qualificazioni — la condizione d'uso su `X`, il limite di durata della proprietà di
`validator_min_set_size`, la distinzione fra valori e regole, gli intervalli leciti
sotto il limite 5/4, e la grandezza rivolta all'utente. Le voci fuori scope
(`RewardBounds` di genesi e il vincolo `3·min_set ≥ 2V`) sono modifiche di protocollo
e appartengono a un'ADR del Lead.

Il simulatore vive in `sim/`, è in Python 3.11 senza dipendenze, è deterministico
a seme fissato e si riesegue con `python -m coblox_sim` da `sim/`. Ogni cifra di
questo documento è un'uscita di quel comando: chi non è d'accordo con un numero
cambia `sim/coblox_sim/recommended.py` e riesegue, e il blocco di vincoli lo
ferma se la combinazione non regge.

---

## 1. La cosa che conta più di ogni numero: `α` non è un compromesso, è un'identità

La spec chiedeva una curva e non un punto, e temeva che il modello ottimizzasse
la sola difendibilità. Il modello ha trovato qualcosa di più stretto di un
compromesso.

Con il fondo a tetto ripartito uniformemente fra i presenti — che è ciò che
`ledger.md` **già impone** come regola di validità, `amount = F / E` — valgono
due identità esatte:

```text
quota catturata da una flotta di N contro H onesti  =  α · N/(N+H)
reddito di un nodo di sola availability / reddito medio di un nodo  =  α
```

La seconda non è un'approssimazione: il fondo è diviso in parti uguali e il
reddito medio è l'emissione totale sullo stesso numero di teste, quindi il
rapporto è `α` esattamente. Verificato su tutta la griglia dal test
`test_phone_share_of_average_equals_alpha`.

**Difendibilità e significato non sono due curve che si incontrano in un
ottimo: sono lo stesso numero letto due volte.** Una flotta è una flotta di
telefoni finti, quindi qualunque cosa paghi un telefono paga allo stesso modo un
membro della flotta. Non esiste un ginocchio nella curva: la cattura è lineare in
`α` su tutto l'intervallo, e **nessun valore di `α` è selezionato
dall'aritmetica**. La scelta è una decisione di prodotto; il simulatore la prezza,
non la prende.

### La curva

Canale di lavoro tenuto fermo all'uso di riferimento (90 000 cr per epoca di un
giorno, 10 000 nodi presenti, uno su cinque contribuente). `F` segue da `α`,
che è il verso causale corretto.

| `α` | `F` (cr/ep) | telefono (cr/ep) | contribuente (cr/ep) | telefono/medio | cattura H=100 | cattura H=10⁴ | cattura H=10⁵ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0,02 | 1 837 | 0,184 | 45,18 | 0,020 | 1,98 % | 1,00 % | 0,18 % |
| 0,05 | 4 737 | 0,474 | 45,47 | 0,050 | 4,95 % | 2,50 % | 0,45 % |
| 0,10 | 10 000 | 1,000 | 46,00 | 0,100 | 9,90 % | 5,00 % | 0,91 % |
| **0,15** | **15 882** | **1,588** | **46,59** | **0,150** | **14,85 %** | **7,50 %** | **1,36 %** |
| 0,20 | 22 500 | 2,250 | 47,25 | 0,200 | 19,80 % | 10,00 % | 1,82 % |
| 0,30 | 38 571 | 3,857 | 48,86 | 0,300 | 29,70 % | 15,00 % | 2,73 % |
| 0,50 | 90 000 | 9,000 | 54,00 | 0,500 | 49,50 % | 25,00 % | 4,55 % |
| 1,00 | 90 000 | 9,000 | 9,00 | 1,000 | 99,01 % | 50,00 % | 9,09 % |

### Ciò che abbassare `α` **non** compra

Un nodo onesto di sola availability guadagna `F/E`. Una flotta di `N` gonfia `E`,
quindi il nodo onesto conserva `H/(N+H)` di ciò che avrebbe guadagnato. **Quel
fattore non contiene `α`.**

| `N` | `H` | l'onesto conserva | dipende da `α` |
| --- | --- | --- | --- |
| 10 000 | 100 | 0,99 % | no |
| 10 000 | 1 000 | 9,09 % | no |
| 10 000 | 10 000 | 50,00 % | no |
| 10 000 | 100 000 | 90,91 % | no |

Abbassare `α` riduce la percentuale di cattura — un numero **sull'attaccante** —
e riduce il reddito del telefono onesto esattamente dello stesso fattore, attacco
o non attacco. Non riduce di una parte su mille il **rapporto di perdita**
dell'onesto sotto attacco.

[ADR-007] ha misurato la grandezza rivolta all'attaccante e su quella ha ragione.
Questa è la grandezza rivolta all'utente, ed è governata da `N/(N+H)` soltanto.
La conseguenza pratica è che il budget di difesa rende di più speso su ciò che
davvero separa un telefono da un membro di flotta — il pavimento Argon2id, la
soglia di availability, e i limiti di diversità di indirizzo aggregati per
prefisso instradato (`threat-model.md` §6.1, leva 3) — che su limature di `α`.
**Assunzione contestata**, portata in superficie e non sepolta, e rimessa a
`GATE-SECREVIEW`.

### `α` è osservata, non impostata — ed è massima quando la rete è più nuova

`ledger.md` lo dice per esteso: la frazione è «an observed ratio between
channels, not a knob». La manopola è `F`. Quindi `α = F/(F+W)` **deriva** con
l'uso, e alla genesi `W ≈ 0` e `α ≈ 1` qualunque sia `F`.

| uso (frazione del riferimento) | `W` (cr/ep) | `α` con `F` fisso | `F` per tenere 0,15 |
| --- | --- | --- | --- |
| 0,00 | 0 | 1,0000 | 0 |
| 0,05 | 4 500 | 0,7792 | 794 |
| 0,25 | 22 500 | 0,4138 | 3 971 |
| 1,00 | 90 000 | 0,1500 | 15 882 |
| 5,00 | 450 000 | 0,0341 | 79 412 |

Tenere la banda dal primo blocco richiederebbe `F` vicino a zero: **nessun
reddito di esistenza al lancio**, proprio il giorno in cui la promessa deve
essere visibile. La banda vincola quindi **da una soglia d'uso dichiarata**, e
sotto quella soglia la rete pubblica l'**importo assoluto** dirottato invece del
rapporto — che lì è la grandezza onesta: il 91 % di un'emissione minuscola è
un'emissione minuscola. È una regola di governance del fondo, non di protocollo,
e non richiede alcun campo di schema.

### La raccomandazione, con la sua motivazione

> **`α` iniziale = 0,15 — intervallo di sorveglianza [0,10 – 0,20] — `X` = 20 %.**

- **Bordo inferiore 0,10.** Sotto, un dispositivo di sola availability riceve
  meno di un decimo del reddito di un nodo medio, e il reddito di esistenza
  smette di essere un pavimento e diventa una riga di arrotondamento su un
  cruscotto. È il modo di fallire contro cui la spec metteva in guardia, non
  compare in **nessun** numero di cattura, ed è esattamente per questo che va
  scritto come limite.
- **Bordo superiore 0,20.** Sopra, più di un quinto di tutta l'emissione passa
  dall'unico canale in cui si entra con una firma, e la tolleranza che il
  progetto deve pubblicare smette di essere difendibile in una frase.
- **0,15 dentro la banda.** Nulla nella simulazione la preferisce a 0,12 o a
  0,18. È il centro di un budget dichiarato, e la scelta è dell'operatore.
- **`X` = 20 %** perché la cattura è **strettamente inferiore ad `α`** per ogni
  `N` e ogni `H`: il bordo superiore della banda è quindi l'unico valore di `X`
  dimostrabile per costruzione invece che per il particolare `N/H` che capita di
  avere sul banco di prova. Un `X` più stretto — 10 %, per dire — sarebbe smentito
  dal banco di `AT-07` stesso, che prescrive `H ≥ 100` contro `N = 10 000` e
  quindi un rapporto `N/(N+H)` di 0,99.

> **`X` porta una condizione e non va pubblicata senza ([REVIEW-011] RF-002).**
> `X = 20 %` vincola **al di sopra della soglia d'uso dichiarata**. Sotto quella
> soglia `α` tende a 1 per costruzione e `X` **non è un'affermazione**: la
> grandezza che la rete pubblica lì è l'**importo assoluto dirottato per epoca**.
> Una `X` pubblicata senza quella condizione è violata di circa cinque volte per
> tutto l'avviamento — vedi §4 — e una tolleranza dichiarata e smentita è la forma
> di danno contro cui [ADR-007] ha già dovuto riformulare una metrica una volta.

**Il bordo inferiore è una scelta di prodotto travestita da misura, e va scritto
così** ([REVIEW-011] RF-005). Nessuna grandezza simulata seleziona 0,10: è un
giudizio sul significato, non un risultato. Vale la pena scriverlo come limite
proprio perché protegge una grandezza che nessun numero di sicurezza vede, e
sarebbe quindi il primo limite a sparire sotto pressione di taratura — ma non va
pubblicato come se il modello lo avesse prodotto.

**E la banda e `X` sono dichiarate in due mondi diversi**, il che è la parte che
il progetto pubblicherà comunque e deve quindi pubblicare intera. Il bordo
inferiore 0,10 protegge il significato del reddito **in assenza di avversario**: a
`α = 0,15` un telefono prende 0,15 del reddito di un nodo medio. `X = 20 %`
dichiara invece un avversario **presente e tollerato**, e in quel mondo il
telefono non prende 0,15 del medio: al banco di `AT-07` prende **0,0157 cr per
epoca invece di 1,588**, due ordini di grandezza in meno. Non è una contraddizione
aritmetica — misurano cose diverse — ma è un'incoerenza fra **promesse**, e la
banda non può essere pubblicata da sola.

**Sul punto dentro la banda.** Poiché `α` è osservata ed è massima quando la rete
è più nuova, **il verso di avvicinamento conta più del punto**: la rete
attraverserà comunque tutta la banda dall'alto durante l'avviamento. Un eventuale
margine va preso sul **bordo superiore**, che è quello duale a `X` e quello messo
alla prova per primo.

---

## 2. La forma del fondo

Il **criterio di ripartizione non è libero**: `ledger.md` lo fissa già come
regola di validità, `E > 0` e `amount_microtokens = F / E` a divisione intera con
il resto scartato e mai emesso. La ripartizione è quindi uniforme, e ogni
variante pesata è una modifica di protocollo e fuori dallo scope di questa spec.

Confermato e non solo ereditato, perché la spec chiedeva l'interazione fra
criterio e cattura per numerosità:

- una ripartizione **uniforme massimizza** la cattura per numerosità — una flotta
  di `N` prende `N/(N+H)` del fondo — ed è il costo della scelta, detto chiaro;
- una ripartizione **pesata per contributo dimostrato** la ridurrebbe, e
  distruggerebbe la cosa che si sta pagando. Gli unici pesi Sybil-difficili che
  la rete possiede sono storage e compute, che il canale di lavoro paga già a
  unità. Pesare il fondo con essi rende il reddito di esistenza una **seconda
  copia peggio denominata** della compensazione del lavoro, e un dispositivo di
  sola availability — il dispositivo caratteristico del progetto — riceve un peso
  di circa zero. La cattura scenderebbe quasi a nulla, e con essa la promessa in
  prima pagina di [[PROJECT]]. È la trappola contro cui la spec metteva in
  guardia, raggiunta per una strada che sembra prudenza;
- **non esiste una terza opzione**, ed è la parte onesta: un peso né uniforme né
  misura di contributo dovrebbe essere Sybil-difficile e non già pagato, e la
  rete non possiede una grandezza simile. Non ne è stata trovata nessuna.

> **Decisione.** Ripartizione **uniforme**, come il protocollo già richiede, con
> tetto per epoca `F` scelto per tenere `α` dentro la banda. Valore di genesi:
> `existence_fund_microtokens_per_epoch = 15 882 352 941` per un'epoca di ricompensa
> di un giorno (15 882 cr), cioè `α = 0,15` all'uso di riferimento.

**Regola di governance di `F`**, da applicare per epoca e pubblicare:

1. osservare `α` = emissione di esistenza / emissione totale dell'ultima epoca;
2. se `α` è dentro la banda, lasciare `F` invariato;
3. se `α` esce dalla banda, muovere `F` verso il bersaglio di **al più il 25 % per
   documento**, la stessa disciplina 5/4 che i parametri di elezione già usano;
4. sotto la soglia d'uso — canale di lavoro sotto il 25 % del riferimento —
   sospendere la banda e pubblicare l'importo assoluto dirottato.

> **Che cosa NON è tutto questo, dichiarato per iscritto ([REVIEW-011] RF-001,
> parte 1).** Tre delle grandezze che questo studio fissa sono **valori e prassi,
> non regole**. Nessun documento di protocollo le impone e nessuna traccia
> on-chain distingue una reward policy che le abbandona da un normale atto di
> governance:
>
> - `availability_microtokens_per_unit = 0` è **un valore**. Nulla vieta un valore
>   positivo, e un valore positivo rompe il criterio (a) della metrica di
>   [ADR-007] — controesempio misurato in §4.
> - il tetto `F` è **un valore senza tetto e senza pavimento**. Un solo documento
>   lecito può portarlo da 15 882 cr a `2^60` microtoken.
> - la disciplina 5/4 su `F` del punto 3 è **una prassi**. I parametri di elezione
>   hanno un limite di variazione perché il progetto ha stabilito che serve; la
>   reward policy non ne ha nessuno, e la sua unica regola di validità è
>   `kn < kd` sul tetto della quota al creatore.
>
> Ne segue che **i criteri (a) e (c) della metrica di [ADR-007] sono veri a
> condizione che la reward policy attiva li rispetti**, e non come proprietà del
> protocollo. Uno zero scritto in un commento Python non è una difesa. Chiudere la
> superficie richiede un oggetto `RewardBounds` nel trust anchor di genesi, sul
> modello di `ElectionBounds`: modifica di protocollo, fuori dallo scope di questa
> spec, ADR del Lead.

---

## 3. I valori

Blocco assunto: **5 s per blocco**. È un'assunzione dichiarata, perché nessun
documento di protocollo fissa un intervallo di blocco; cambiandolo i parametri
espressi in blocchi vanno riscalati e il blocco di vincoli rieseguito.

### `consensus_parameters` — sottoinsieme di elezione

| Parametro | Valore | Perché |
| --- | --- | --- |
| `election_epoch_blocks` | 120 960 | 7 giorni |
| `candidacy_close_blocks` | 17 280 | 1 giorno prima del confine |
| `election_entropy_blocks` | 720 | 1 ora |
| `validator_min_set_size` | 18 | `2V/3` **a questo `V`**: chiude il percorso per attrito sotto i due terzi (§5). Il rapporto `min_set/V` non è preservato da alcuna regola |
| `validator_target_set_size` | 27 | `V` |
| `validator_max_set_size` | 45 | margine **nominale**: `V` è limitato a 36 per sempre dal `c` congelato — vedi sotto |
| `validator_churn_cap_seats` | 3 | `c = V/T` esatto; a `m = 3` nessun gioco. **CONGELATO** dal limite 5/4 |
| `validator_max_consecutive_terms` | 9 | `T = 3m`, il minimo che l'orizzonte ammette |
| `validator_cooldown_epochs` | 2 | corto: moltiplica la leva del censore e prosciuga il pool. **CONGELATO**: nessun documento lecito potrà cambiarlo |
| `validator_min_capture_epochs` | 3 | `m` = orizzonte per attrito; di più sarebbe autoinganno. **CONGELATO** |

### `reward_policy` — sottoinsieme di eleggibilità ed emissione

| Parametro | Valore | Perché |
| --- | --- | --- |
| `reward_epoch_ms` | 86 400 000 | 1 giorno |
| `existence_fund_microtokens_per_epoch` | 15 882 352 941 | `α = 0,15` all'uso di riferimento; governato **senza tetto e senza limite di variazione** (§2) |
| `availability_microtokens_per_unit` | **0** | un valore positivo rompe il criterio (a) di [ADR-007] (§4). È **un valore, non una regola** (§2) |
| `storage_units_per_contribution_unit` | 1 073 741 824 | 1 unità per GiB-epoca provato |
| `compute_units_per_contribution_unit` | 1 000 000 | 1 unità per milione di fuel rieseguito |
| `validator_eligibility_threshold_units` | 512 | circa 18 GiB sostenuti sulla finestra |
| `validator_eligibility_window_epochs` | 28 | 4 settimane |
| `validator_eligibility_min_issuers` | 3 | il prezzo di un candidato fabbricato è lineare in questo |
| `publisher_reward_cap_numerator` / `_denominator` | 1 / 2 | `k = 1/2`; vedi §6 sul perché non si tocca |

### `ElectionBounds` — ancoraggio di fiducia della genesi

| Parametro | Valore | Perché |
| --- | --- | --- |
| `election_epoch_blocks_max` | 241 920 | al più un raddoppio del periodo di confine |
| `validator_max_consecutive_terms_max` | **12** | il presidio residuo di [DEBT-010]; vedi §5 |
| `validator_max_set_size_max` | 81 | nominalmente `3V`, **irraggiungibile** — vedi sotto |
| `validator_min_set_size_min` | 18 | fissato al minimo scelto: non può mai essere abbassato |
| `validator_min_capture_epochs_min` | 3 | fissato all'orizzonte per attrito |
| `election_parameter_change_numerator` / `_denominator` | 5 / 4 | 25 % per documento |
| `election_parameter_min_activation_gap_blocks` | 120 960 | un'epoca di elezione intera |

Ventidue valori, più `α`, la sua banda, `X` e il tetto del fondo. Tutti verificati
contro il blocco di vincoli, riga per riga, in `GATE-CONSTRAINTS`.

### Che cosa la governance può ancora muovere, e che cosa non potrà mai disfare

Un limite di variazione di 5/4 applicato a interi piccoli **non è un limite: è un
congelamento** ([REVIEW-011] RF-006). L'intervallo raggiungibile da un documento
lecito è `[ceil(x·4/5), floor(x·5/4)]`, che per `x` piccolo collassa in un punto.

| Parametro | Valore | Il prossimo documento può portarlo a | |
| --- | --- | --- | --- |
| `election_epoch_blocks` | 120 960 | [96 768, 151 200] | |
| `candidacy_close_blocks` | 17 280 | [13 824, 21 600] | |
| `election_entropy_blocks` | 720 | [576, 900] | |
| `validator_min_set_size` | 18 | [18, 22] | pavimentato da `min_set_min` |
| `validator_target_set_size` | 27 | [22, 33] | |
| `validator_max_set_size` | 45 | [36, 56] | |
| `validator_churn_cap_seats` | 3 | **[3, 3]** | **CONGELATO** |
| `validator_max_consecutive_terms` | 9 | [9, 11] | monotono, e sotto il tetto di genesi |
| `validator_cooldown_epochs` | 2 | **[2, 2]** | **CONGELATO** |
| `validator_min_capture_epochs` | 3 | **[3, 3]** | **CONGELATO** |

Conseguenze che ne discendono e che nessun altro punto del rapporto rendeva
visibili:

- **`V ≤ 36` per sempre.** `ceil(V/T) ≤ c` con `c` congelato a 3 e `T` limitato a
  12 dai bound di genesi dà `V ≤ c · T_max = 36`. Quindi
  `validator_max_set_size = 45` e `validator_max_set_size_max = 81` sono **margini
  che non si possono occupare**: i valori sono innocui ma le parole che li
  motivavano erano sbagliate — 45 come «margine di crescita» e 81 come `3V` — e
  sono corrette qui. Se il Lead volesse portare `validator_max_set_size` a 36 è un
  cambio di valore che non spetta a questa spec.
- **Il cooldown non ha un secondo tentativo.** §6 misura che a pool 33 un cooldown
  di 1 conserva tutti e 27 i seggi dove 2 si assesta a 24. Se la rete scoprisse
  quel pool in produzione, **la mossa correttiva non esiste**. 2 resta la scelta
  giusta — è il compromesso fra la leva del censore e il mordente del limite di
  mandato — ma è scelta **sapendo che è definitiva**.
- **Lo stesso vale per `c` e per `m`, e il ragionamento di questo studio non se
  n'era accorto.** L'argomento per `T_max = 12` è che un tetto di 9 non
  lascerebbe a una rete dal pool sottile alcuna mossa lecita. È giusto per `T`.
  Per `c` e per il cooldown **la mossa non esiste comunque**: alzare `c` o
  abbassare il cooldown sono entrambi rifiutati in accettazione. Restano solo
  alzare `T` — cricchetto irreversibile, due passi residui — o fermarsi.
- **`min_set/V` non è preservato da alcuna regola.** Vedi §5.

Chi tara `c`, `m` e il cooldown **li sceglie una volta sola**.

---

## 4. `AT-07` — verdetto numerico: **parzialmente coperto**

`H = 100` nodi onesti, `N = 10 000` identità emulate su un singolo host,
`α = 0,15`, `X = 20 %`.

| Criterio | Misura | Esito |
| --- | --- | --- |
| (a) emissione totale non aumentata | 105 882 352 900 → 105 882 351 000 µt | **PASS** |
| (b) quota della flotta ≤ `X` | 14,851 % ≤ 20 % | **PASS** |
| (c) accrediti `storage`/`compute` | 0 | **PASS** |
| (d) seggi di validatore | 0 | **PASS** |

Il criterio (d) tiene **per la regola e non per fortuna**: `contribution_score`
conta l'evidenza `availability` come zero, quindi una flotta che si limita a
firmare ha punteggio 0 e fallisce la condizione di eleggibilità 3 a qualunque
soglia positiva.

Il nodo onesto conserva lo **0,99 %** del proprio reddito sotto quell'attacco.
È la grandezza da leggere, ed è indipendente da `α` (§1).

### Il regime in cui il test verrà davvero eseguito

`AT-07` è schedulato in M-03 su devnet, e una devnet non ha uso: `W ≈ 0`, quindi
`α ≈ 1` qualunque sia `F` ([REVIEW-011] RF-002). Lo stesso banco lungo la rampa
d'uso:

| `W` (cr/ep) | `α` | quota della flotta | dirottato (cr/ep) | criterio (c) alla lettera |
| --- | --- | --- | --- | --- |
| 0 | 1,0000 | 99,01 % | 15 725 | **violato** |
| 4 500 | 0,7792 | 77,15 % | 15 725 | **violato** |
| 9 000 | 0,6383 | 63,20 % | 15 725 | **violato** |
| 22 500 | 0,4138 | 40,97 % | 15 725 | **violato** |
| 45 000 | 0,2609 | 25,83 % | 15 725 | **violato** |
| 90 000 | 0,1500 | 14,85 % | 15 725 | tenuto |

Il criterio (c) alla lettera è **violato di circa cinque volte per tutto
l'avviamento**, che è precisamente quando una flotta costa meno e la rete è più
esposta. Il danno non è il valore catturato: è la **smentita pubblica e
verificabile da chiunque di una tolleranza dichiarata**.

**Una correzione a ciò che questo studio aveva detto con leggerezza.** La quarta
colonna non si muove: `D = F · N/(N+H)` non contiene `W`. «Il 91 % di
un'emissione minuscola è un'emissione minuscola» era vero solo se anche `F` è
piccolo, e `F` è **una scelta di governance**, non una conseguenza del poco uso.
Con l'`F` di genesi dimensionato per 10 000 nodi, una flotta al lancio dirotta
circa **15 725 cr per epoca** — quasi l'intero fondo — mentre i cento onesti si
dividono il resto. Il criterio assoluto è quindi onesto **solo se `F` al lancio è
dimensionato sul numero di nodi onesti effettivamente presenti**. È prassi di
governance e non regola (§2), ed è la seconda cosa che `RewardBounds` chiuderebbe.

> **Verdetto `AT-07`: parzialmente coperto.**
> *Regime d'uso di riferimento:* **superato** su tutti e quattro i criteri.
> *Regime di lancio:* il criterio (c) alla lettera **fallisce**; il criterio
> applicabile lì è quello **assoluto**, perché `X` non è un'affermazione sotto la
> soglia d'uso. La matrice riporta `AT-07` come **parzialmente coperto** finché
> `X` non porta la sua condizione d'uso e il verdetto di regime di lancio non è
> emesso contro il criterio assoluto.

### Il controesempio che fissa un valore a zero

Con un `availability_microtokens_per_unit` positivo, la stessa prova dà
un'emissione totale che passa da 205 882 352 900 a 10 205 882 351 000 µt: **il
criterio (a) fallisce e la flotta stampa.**

`work_compensation` con `work_kind = "availability"` è un importo **per nodo senza
tetto**. `RewardPolicyBody` porta la tariffa e nessuna regola vieta un valore
positivo, quindi `N` identità emulate aumenterebbero l'emissione totale e il
criterio (a) della metrica di [ADR-007] fallirebbe alla prima epoca. **Non è un
difetto della regola e non serve un'ADR:** il campo di schema resta e il *valore*
deve essere zero. È registrato qui perché uno zero non scritto è uno zero che
qualcuno più tardi alza «solo un po'».

---

## 5. `AT-10` — verdetto numerico, criterio per criterio

`V = 27`, `c = 3`, `T = 9`, `m = 3`, `validator_min_set_size = 18`.
Orizzonte per **ammissione** `ceil((V/3)/c)` = 3 confini (tarabile).
Orizzonte per **attrito** `ceil(log(V/k)/log(3/2))` = 3 confini (fisso).

### Configurazione 1 — macinatura del seme

L'attaccante fornisce storage e compute reali appena sopra la soglia (la
correzione che il threat model stesso impone: l'uptime contribuisce zero) e
macina solo ai confini in cui detiene lo slot di proposta, perché solo un
proposer può macinare. `G = 128` ricampionamenti dell'ultimo blocco di entropia.

| `N/H` | cand. avv. | cand. onesti | riempimenti `G=1` | riempimenti `G=128` | epoche a 1/3 `G=1` | epoche a 1/3 macinando | seggi finali |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 0,1 | 7 | 74 | 12 | 13 | mai | mai | 2/27 |
| 1 | 74 | 74 | 32 | 36 | 6 | 6 | 27/27 |
| 10 | 740 | 74 | 31 | 27 | 4 | 4 | 27/27 |

**Superata su ciò che la regola afferma:** la macinatura produce bias e mai
scelta, e 128 ricampionamenti muovono il conteggio dei riempimenti di meno del
tetto di rotazione di un singolo confine su 50 confini.

**Fallita sul criterio di superamento che `AT-10` oggi enuncia** — «l'attaccante
non raggiunge 1/3 entro 50 epoche» — a `N/H = 1` e `N/H = 10`. Non è un
fallimento di taratura e nessuna combinazione di parametri lo ripara: il criterio
letterale equivale a `m ≥ 50`, che per `T ≥ 3m` forza `T ≥ 150` e `c ≤ V/150`.

**Che cosa hanno in comune le due occorrenze**, che è più utile di entrambe le
correzioni ([REVIEW-011] RF-004, adottato). «La coalizione non arriva mai al 100 %
sotto i due terzi» e «l'attaccante non raggiunge 1/3 entro 50 epoche» sono
entrambe **affermazioni assolute su una grandezza emergente**, formulate prima che
esistesse la regola che quella grandezza produce, e **nessuna delle due nomina una
regola**. Un criterio di quella forma non è verificabile quando lo si scrive: lo è
solo quando una simulazione lo smentisce, e a quel punto lo smentisce addosso a chi
ha implementato. **Il difetto è nel modo in cui il test è scritto, non nei singoli
criteri.** La formulazione sostitutiva proposta non ha quella forma — ogni clausola
è una disuguaglianza contro un parametro pubblicato e nomina la regola che la
impone. AGENT-007 ha adottato la correzione e ha aggiunto la convenzione di
scrittura corrispondente al proprio documento. La sostituzione **riduce
un'affermazione pubblica** e va quindi all'operatore: registrata, non applicata.

### Configurazione 2a — censura totale

Coalizione a 10 seggi su 27 (37,0 %). La catena **si ferma al primo confine**
(`3 · 8 ≤ 2 · 27`, pavimento di contrazione). La coalizione ottiene un arresto,
mai il set. **Superata.**

### Configurazione 2b — censura selettiva, il vettore reale

| `k` | % di `V` | dimensioni per confine | esito |
| --- | --- | --- | --- |
| 10 | 37,0 % | 27 → 19 → 18 | **bloccata a 18**: detiene 10 su 18 (55,6 %) e non ottiene mai l'intero set |
| 17 | 63,0 % | 27 → 19 → 18 | **bloccata a 18**: detiene 17 su 18 (94,4 %) |
| 18 | 66,7 % | 27 → 19 → 18 | ottiene l'intero set in **2 confini** |
| 19 | 70,4 % | 27 → 19 | ottiene l'intero set in **1 confine** |

**Il risultato che merita un nome.** Con `validator_min_set_size = 18` il percorso
per attrito è **chiuso per ogni coalizione più piccola di 18**. `ledger.md`
attribuisce il merito al solo pavimento di contrazione e conclude che la soglia
effettiva di cattura è «appena sopra un terzo»: è esattamente vero del pavimento
preso da solo, e portare il minimo del set a `2V/3` la alza a **due terzi**, punto
oltre il quale l'assunzione di safety BFT è già caduta e nessuna regola di
composizione del set stava più promettendo niente. Il pavimento fa ciò che il
documento dice; il **minimo fa più di quanto il documento gli attribuisca**.

È un'affermazione di sicurezza prodotta da una spec di taratura ed è la voce
singola più importante per AGENT-007 a `GATE-SECREVIEW`.

*Il suo prezzo è liveness*, ed è pagato nella tabella di §6: il set non può
contrarsi sotto 18, quindi tre confini consecutivi con pool di riempimento vuoto
fermano la catena invece di cinque.

> **Ed è una proprietà di questa combinazione, non delle regole ([REVIEW-011]
> RF-003).** La proprietà dipende dal **rapporto** `min_set/V`, e **nessuna regola
> lo preserva**: il blocco di vincoli richiede soltanto `0 < min_set <= V`.
> `validator_min_set_size_min` impedisce di abbassare il minimo; nulla impedisce
> di alzare `V`. Percorso misurato, ogni passo dentro il 5/4 e dentro la
> monotonia di `T`, ognuno **accettato dal blocco di vincoli**:
>
> | documento | `V` | `T` | `min_set` | `min_set/V` | blocco di vincoli |
> | --- | --- | --- | --- | --- | --- |
> | genesi | 27 | 9 | 18 | 0,667 | accettato |
> | 1 | 33 | 11 | 18 | 0,545 | accettato |
> | 2 | 36 | 12 | 18 | **0,500** | accettato |
>
> Due documenti distanziati da un'epoca di elezione, cioè circa **quattordici
> giorni**. E a `V = 36` con `min_set = 18`, la censura selettiva dà:
>
> | `k` | % di `V` | dimensioni | esito |
> | --- | --- | --- | --- |
> | 13 | 36,1 % | 36 → 25 → 18 | bloccata |
> | **18** | **50,0 %** | 36 → 25 → 18 | **intero set in 2 confini** |
> | 23 | 63,9 % | 36 → 25 → 23 | intero set in 2 confini |
>
> **La soglia di cattura per attrito scende da due terzi a esattamente una metà**,
> dove la safety BFT **non** è caduta e la rete crede ancora di avere una
> garanzia. La proprietà non si degrada gradualmente: si sposta sotto il confine
> che la rendeva innocua. L'affermazione va quindi enunciata così: *chiusa sotto
> `2V/3` **alla combinazione raccomandata**; in generale chiusa solo sotto
> `validator_min_set_size`, grandezza che la governance può lasciare indietro
> mentre `V` cresce.* La conclusione di `ledger.md` — «soglia effettiva appena
> sopra un terzo» — **non va cambiata**: è la cifra corretta nel caso peggiore
> governabile, e alzarla a due terzi sarebbe scrivere una garanzia più forte di
> quella che le regole impongono. La regola che la renderebbe vera —
> `3 · min_set ≥ 2V`, soddisfatta con uguaglianza alla combinazione raccomandata —
> è modifica di protocollo, fuori scope, ADR del Lead.

*Correzione discreta.* La formula continua predice meno confini della simulazione
perché il pavimento è **stretto**: un set di 27 può scendere a 19, non a 18. La
cifra misurata è quella da citare e non è mai inferiore a quella della formula.

### Configurazione 3 — evasione del cooldown

`validator_cooldown_epochs = 2`. Assenza dopo scadenza del mandato: **2 epoche**.
Assenza dopo uscita volontaria un'epoca prima: **2 epoche**. **Superata** — la
condizione di eleggibilità 5 nella forma «uscita per qualunque ragione» rende le
due misure uguali, che è il senso del test; con la formulazione precedente
l'uscita volontaria avrebbe misurato **una** epoca.

### Riassunto

| Criterio | Esito |
| --- | --- |
| macinatura limitata dal tetto di rotazione e non dal seme | **PASS** |
| censura totale → arresto, mai un set scelto dalla coalizione | **PASS** |
| censura selettiva bloccata a `validator_min_set_size` sotto `2V/3` | **PASS alla combinazione raccomandata**; a `V = 36` la soglia scende a `V/2` |
| cooldown non evadibile uscendo un'epoca prima | **PASS** |
| deriva della composizione calcolabile da un light client a ogni confine | **PASS** (per costruzione) |
| «non raggiunge 1/3 entro 50 epoche», tutti e tre gli `N/H` | **FAIL** a `N/H = 1` e `N/H = 10` |

---

## 6. I tre accoppiamenti insieme, e dove la rete si ferma **senza avversario**

### Accoppiamento 1 — cooldown, soglia di eleggibilità, pool

Alla soglia raccomandata di 512 unità la barriera di eleggibilità **non è** il
vincolo che morde: lo è la **disponibilità a candidarsi**. Al riferimento
(2 000 contribuenti) 1 286 nodi superano la soglia; ne servono 36.

| soglia (unità) | eleggibili | pool necessario | disponibilità necessaria |
| --- | --- | --- | --- |
| 512 | 1 286 | 36 | 2,80 % |
| 1 024 | 945 | 36 | 3,81 % |
| 4 096 | 305 | 36 | 11,80 % |
| 16 384 | 55 | 36 | 65,45 % |
| 65 536 | 3 | 36 | **insoddisfacibile** |

Minimo aritmetico: `V` seduti + `ceil(V/T) = 3` in cooldown per `cooldown = 2`
confini + 3 liberi per il riempimento = **36**.

**La siccità.** Nessun avversario: solo un pool finito di nodi disposti a
candidarsi, che rifilano la candidatura ogni epoca che il cooldown consente.

| pool stabile | confini sopravvissuti | dimensione finale | motivo dell'arresto |
| --- | --- | --- | --- |
| 0 | 3 | 18 | sotto `validator_min_set_size`: 15 < 18 |
| 6 | 5 | 18 | sotto `validator_min_set_size` |
| 12 | 7 | 18 | sotto `validator_min_set_size` |
| 24 | 11 | 18 | sotto `validator_min_set_size` |
| 30 | 30 | 21 | nessun arresto |
| 33 | 30 | 24 | nessun arresto |
| **36** | 30 | **27** | nessun arresto |

> **Dove la rete si ferma senza avversario, come numero:** sotto un pool stabile
> di **30** la catena si arresta — in **3 confini** con pool zero, in **11** con
> pool 24. Fra 30 e 35 sopravvive ma si assesta sotto la dimensione bersaglio. A
> **36**, il minimo aritmetico, tiene tutti e 27 i seggi. È una **soglia di
> partecipazione**, non un valore di parametro, ed è la grandezza da sorvegliare:
> alla rete di riferimento significa che circa il **3 % dei contribuenti** deve
> essere disposto a candidarsi.

**Sensibilità al cooldown** a pool 33, appena sotto il minimo aritmetico, che è
dove il parametro può ancora cambiare la risposta:

| cooldown | confini sopravvissuti | dimensione finale | esito |
| --- | --- | --- | --- |
| 1 | 30 | 27 | nessun arresto |
| 2 | 30 | 24 | nessun arresto |
| 3 | 30 | 21 | nessun arresto |
| 5 | 14 | 18 | **arresto** |
| 9 | 14 | 18 | **arresto** |

Con cooldown 5 o 9 la catena si ferma dopo 14 confini **senza avversario da
nessuna parte**, solo perché chi esce non rientra abbastanza in fretta da
ripopolare i seggi che il limite di mandato continua a svuotare. Il cooldown è
anche l'unica grandezza di elezione il cui **aumento aiuta un avversario**:
censurare una candidatura per un'epoca rimuove quel nodo per `1 + cooldown`
epoche. I due argomenti puntano nella stessa direzione, ed è per questo che la
raccomandazione è **2** e non il massimo che il blocco di vincoli consentirebbe.

### Accoppiamento 2 — `validator_max_consecutive_terms_max` ([DEBT-010])

| `T` | seggi liberati per confine | pool minimo | `c` ammissibili |
| --- | --- | --- | --- |
| 9 | 3,00 | 36 | (3) |
| 12 | 2,25 | 36 | (3) |
| 15 | 1,80 | 33 | (2, 3) |
| 18 | 1,50 | 33 | (2, 3) |
| 27 | 1,00 | 30 | (1, 2, 3) |

[DEBT-010] rende il limite di mandato un cricchetto spingibile e non tirabile:
una volta che un quorum ai due terzi alza `T`, è alzato per sempre, e questo
tetto è l'unico freno residuo sulla velocità di rotazione. Va scelto stretto
quanto la rete tollera, e la tabella prezza il «tollera».

> **`validator_max_consecutive_terms_max` = 12.** A `T = 9` la rete deve
> ripopolare 3,00 seggi per confine; portando `T` al tetto scende a 2,25, cioè un
> **25 % di valvola di sfogo** sulla fornitura di candidati, pagata con una
> rotazione forzata di un terzo più lenta. Un tetto di 9 non lascerebbe **alcuna**
> valvola e il limite non è più abbassabile, quindi una rete che si scoprisse il
> pool troppo sottile non avrebbe più alcuna mossa lecita. Un tetto di 27
> comprerebbe il 67 % di sfogo e farebbe durare la rotazione forzata completa
> mezzo anno a confini settimanali, cioè quasi tutto ciò per cui il limite di
> mandato esiste. **12 è il valore più stretto che lascia ancora una valvola
> usabile**, ed è raggiungibile solo con due documenti firmati al 5/4 ciascuno,
> distanziati da un'epoca di elezione intera: un processo che qualcuno può
> guardare, non un evento.

Verificato in `GATE-CONSTRAINTS`: con `T` spinto al tetto **l'intera combinazione
soddisfa ancora il blocco di vincoli**, quindi il cricchetto non può portare la
rete in uno stato in cui nessun documento valido esiste.

### Accoppiamento 3 — `α` e la forma del fondo

È il soggetto di §1 e §2. I tre sono riportati insieme perché interagiscono: la
soglia di eleggibilità decide **chi può sedere**, `α` decide **quanto guadagna
chi non siederà mai** per il solo essere presente, e una soglia abbastanza alta
da proteggere il consenso è esattamente la soglia che rende il reddito di
esistenza l'unico reddito che la maggior parte dei dispositivi vedrà mai.

---

## 7. `SEC-REQ-16` — le grandezze obbligatorie (tre, più una)

**(a) `α`.** Bersaglio 0,15; banda di sorveglianza [0,10 – 0,20]; tolleranza
dichiarata `X` = 20 %, pari al bordo superiore della banda, che è un tetto duro
sull'intero canale e quindi dimostrabile per ogni `N` e `H` — **al di sopra della
soglia d'uso dichiarata**. Sotto quella soglia `α` tende a 1 per costruzione e `X`
non è un'affermazione: la grandezza pubblicata è l'importo assoluto dirottato per
epoca. E `α` è tenuta sotto 0,20 **per prassi, non per regola** (§2).

**(b) `E_p` contro `S(1−k)`** — il margine di acquisto di reputazione di
`threat-model.md` §6.3, con `k = 1/2` e `E_p = 47,6 cr` per periodo di 30 giorni:

| prezzo abbonamento | costo netto per finto abbonato | margine (finti abbonati) | sostenibile |
| --- | --- | --- | --- |
| 0,3 cr | 0,1 cr | 317,6 | sì |
| 3,0 cr | 1,5 cr | 31,8 | sì |
| 30,0 cr | 15,0 cr | 3,2 | sì |
| 60,0 cr | 30,0 cr | 1,6 | sì |
| 100,0 cr | 50,0 cr | 1,0 | no |

Ai valori tarati il margine è di circa **3 finti abbonati per nodo controllato per
periodo di 30 giorni** contro un abbonamento da 30 cr — non i 50× che il threat
model stimava dalle cifre illustrative dei documenti, perché quelle cifre non
erano mai state tarate una contro l'altra. **Resta un attacco:** una flotta di
10 000 nodi finanzia dell'ordine di 30 000 finti abbonamenti per periodo, e ciò
che compra è reputazione.

**Il margine non è chiudibile per taratura.** Prezzare un abbonamento sopra il
reddito di esistenza di un nodo è l'opzione 1 di §6.3, che quel documento nomina
come la risposta sbagliata perché esclude dal prodotto l'utente onesto con un
solo dispositivo. Abbassare `k` muove il margine di un fattore al più due. Le
risposte sono le opzioni 2 e 3 — pesare gli abbonati per contributo dimostrato e
non esporre `active_subscriber_count` nella scoperta — entrambe lavoro di consenso
e di catalogo sotto [ADR-006] ed entrambe **escluse** da questa spec.
**Riportato, non chiuso.**

**(c) Quota catturabile da `N` identità emulate**, a `α = 0,15`:

| `N` | `H` | quota | tetto `X` |
| --- | --- | --- | --- |
| 10 000 | 100 | 14,851 % | 20 % |
| 10 000 | 1 000 | 13,636 % | 20 % |
| 10 000 | 10 000 | 7,500 % | 20 % |
| 100 000 | 10 000 | 13,636 % | 20 % |

Ogni voce è strettamente sotto `α`, e `α` è tenuta sotto il bordo superiore della
banda, quindi `X` limita la colonna per costruzione e non per fortuna — sopra la
soglia d'uso, e per prassi anziché per regola.

**(d) La grandezza rivolta all'utente**, aggiunta perché (a), (b) e (c) guardano
tutte all'attaccante o al sistema e nessuna guarda alla persona per cui la rete
esiste ([REVIEW-011] RF-005): **la frazione del proprio reddito che un nodo onesto
di sola availability conserva sotto il banco di `AT-07`**.

| `N` | `H` | l'onesto conserva |
| --- | --- | --- |
| 10 000 | 100 | **0,99 %** |
| 10 000 | 1 000 | 9,09 % |
| 10 000 | 10 000 | 50,00 % |
| 10 000 | 100 000 | 90,91 % |

Al banco che il progetto dichiara di tollerare è lo **0,99 %**. Il fattore è
`H/(N+H)` e non contiene `α`, quindi nessuna scelta di `α` lo migliora. È il numero
da pubblicare **accanto** a `X`, perché `X` da sola invita il lettore a concludere
che il proprio reddito sia protetto entro un ordine di grandezza, quando non è
protetto affatto.

---

## 8. Assunzioni contestate e questioni aperte

Stato dopo [REVIEW-011]: le voci 1–7 sotto sono le auto-segnalazioni della prima
stesura, quattro delle quali la review ha promosso a finding. Le qualificazioni
richieste sono ora **applicate** nelle sezioni sopra; quel che resta aperto sono
le tre voci che richiedono una regola nuova e quindi un'ADR del Lead:

- **`RewardBounds` di genesi** — tetto e limite di variazione su `F`, e
  `availability_microtokens_per_unit == 0` come regola di validità (voce 4 sotto,
  [REVIEW-011] RF-001). È la sola voce che **chiuda** una superficie invece di
  descriverla.
- **`3 · validator_min_set_size ≥ 2 · V`** nel blocco di vincoli, soddisfatta con
  uguaglianza alla combinazione raccomandata ([REVIEW-011] RF-003).
- **Annotazione di [ADR-007]** con la grandezza rivolta all'utente e conferma
  esplicita dell'operatore (voce 2 sotto, [REVIEW-011] RF-005).


1. **Il criterio di superamento di `AT-10` non è soddisfacibile da alcuna rete
   operabile.** Vedi la nota di valutazione in `threat-model.md`. Va deciso dal
   Lead e dall'operatore, eventualmente con un'ADR; il simulatore emette il
   verdetto e non riscrive il criterio.
2. **`α` non protegge il telefono onesto dalla diluizione Sybil** (§1). [ADR-007]
   resta corretta su ciò che misura; la conseguenza rivolta all'utente non era
   stata scritta e va rivista a `GATE-SECREVIEW`.
3. **`validator_min_set_size` sta facendo lavoro anti-cattura che `ledger.md`
   non gli attribuisce** (§5). Se la revisione conferma, la conclusione «soglia
   effettiva appena sopra un terzo» andrebbe qualificata in una futura passata di
   `ledger.md` — passata che **non** appartiene a questa spec.
4. **`availability_microtokens_per_unit` deve essere zero** (§4). Nessuna regola
   lo impone: è una scelta di valore che regge un criterio della metrica. Se il
   progetto volesse renderla non violabile per governance, sarebbe una regola di
   validità nuova e quindi un'ADR.
5. **L'intervallo di blocco (5 s) è assunto**, non specificato da alcun documento.
6. **La banda di `α` non può valere al lancio** (§1). La soglia d'uso proposta —
   25 % dell'uso di riferimento — è una scelta di governance che l'operatore deve
   confermare.
7. **La scala di emissione è arbitraria per costruzione**: il credit non ha
   ancoraggio esterno ([ADR-009]). Il riferimento scelto è il canale di lavoro a
   90 000 cr per epoca; è la grandezza esogena, non una previsione.

---

## 9. Formulazione di prodotto (inglese, pronta per l'interfaccia)

Il reddito di esistenza è una **quota variabile** e non un importo garantito. La
lingua di prodotto è l'inglese ([[PROJECT]]).

> **Primary line (dashboard, beside the figure)**
>
> Existence income — your share of this epoch's network fund

> **Supporting sentence (first run, and the help panel)**
>
> Every epoch the network issues a fixed fund and splits it equally among all
> nodes that proved they were there. Your income is a share of that fund, not a
> fixed amount: it goes down when more nodes are present and up when fewer are.
> The fund is capped, so nobody can make it bigger by adding devices.

> **The one-line answer to "why did my income drop?"**
>
> The fund did not shrink — it was shared with more nodes this epoch.

> **The honest note the network owes its users (help panel, not the dashboard)**
>
> Some of the nodes sharing the fund are not real people. The protocol cannot
> tell a phone from a program pretending to be one, and it does not claim to.
> What it does guarantee is that no amount of pretending creates new credits: a
> fake node can only take a slice, never bake a bigger cake.
>
> But it takes that slice instead of you. Fake nodes do not reduce the fund —
> they share it — so every one of them makes your share smaller, and the network
> cannot stop that. It is the cost of letting anyone join with a device they
> already own, and it is a cost you pay directly.
>
> Once the network is carrying real work, we publish the share of all issuance
> that flows through this fund every epoch, and we commit to keeping it under
> 20 %. Before then that share is close to everything, because there is almost
> nothing else being issued yet, so we publish the amount instead of the share —
> it is the honest number while the network is starting up.

Le due aggiunte sono le condizioni di chiusura di [REVIEW-011] RF-005 parte 2 e
RF-002 punto 3: la prima dice senza attenuazioni che la fetta la prende **al posto
dell'utente**, la seconda toglie il «under 20 %» incondizionato.

**Parole da evitare, con la ragione.** *"guaranteed"*: non lo è, la quota si muove
ogni epoca. *"basic income"*: importa l'aspettativa di un pavimento fisso
denominato in denaro, che è l'unica cosa che un credit non è. *"reward"*: questo
fondo paga la presenza, non il lavoro, e i canali di lavoro sono nominati a parte
e pagati a unità. *`$` o qualunque glifo prima del numero*: [ADR-009], l'unità si
scrive dopo il numero (`1 240 cr`), perché quella è la grammatica di una misura e
non di una valuta. ***"protected"***: l'impegno del 20 % limita ciò che una flotta
prende dell'**emissione**, non ciò che il singolo utente perde, che dipende da
quanti nodi sono presenti; le due cose non vanno mai lasciate leggere come la
stessa promessa.
