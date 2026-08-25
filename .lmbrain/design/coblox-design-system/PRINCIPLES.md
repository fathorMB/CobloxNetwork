# Principi del design system Coblox

Riferimento vincolante per ogni superficie Coblox (desktop Tauri, Android, sito).
Sorgente dei valori: [`tokens/tokens.json`](./tokens/tokens.json). Componenti di
riferimento: [`preview/index.html`](./preview/index.html). Schermate:
[`mockups/`](./mockups/). Verifica del contrasto: [`tokens/CONTRAST.md`](./tokens/CONTRAST.md).

Owner: AGENT-006 (Lia Wireframe) · Spec di origine: SPEC-003.

---

## 1. Che cosa stiamo disegnando (e cosa non stiamo disegnando)

Coblox mostra **attività e contributo**, non un patrimonio.

Il token misura l'uso della rete e non è collegato al valore di alcuna valuta
reale o crypto ([[PROJECT]], [ADR-005]). Questo non è un disclaimer legale da
mettere a piè di pagina: è un vincolo di forma che decide come si disegna ogni
schermata.

**Vietato**, sempre e su ogni superficie:

- grafici a candele, order book, ticker, qualunque grammatica visiva da exchange;
- frecce verdi/rosse di "performance", variazioni percentuali presentate come
  guadagno o perdita, "+3.2% vs yesterday" in evidenza;
- la parola *saldo* usata come sinonimo di ricchezza, simboli di valuta,
  conversioni, prezzi in euro o in altra moneta;
- classifiche di accumulo tra utenti;
- qualunque metafora di portafoglio finanziario o di investimento.

**Al loro posto:**

- il tempo e il lavoro: *quanto la rete ti ha usato*, *chi ti sta usando adesso*,
  *da quanto*, *con quale esito delle prove*;
- emesso e bruciato come **due grandezze affiancate**, mai come un utile netto
  (vedi §5);
- proporzioni di un insieme (spazio impegnato, quota di una sessione), mai
  obiettivi da raggiungere o soglie da battere.

Il principio operativo: se una schermata funzionerebbe altrettanto bene in
un'app di trading, è disegnata male.

---

## 2. "Hacker ma usabile": dove sta l'estetica

L'identità è dark-first, densa, monospace sui dati, con accenti fosforescenti.
Quell'identità si gioca su **quattro leve, e solo quelle**:

1. **Densità** — righe da 36 px (28 px in modalità densa), spaziature strette,
   molta informazione per schermata.
2. **Monospace sui dati** — vedi §3.
3. **Micro-etichette maiuscole** in monospace con letter-spacing ampio, per le
   intestazioni di sezione e di colonna.
4. **Glow degli accenti** — bordi e ombre colorate su superfici, mai sul testo.

Non si gioca mai su: testo a basso contrasto, grigi "atmosferici" sul fondo
scuro, animazioni che disturbano la lettura, font display, testo in maiuscolo
per la prosa, effetti "matrix". Un prodotto che sembra un terminale ma che si
legge peggio di un terminale ha sbagliato entrambe le cose.

**Il tema dark è il primario** ed è il default su `:root`. Il tema light è una
variante secondaria completa (parità di token garantita dal generatore), attivabile
con `data-theme="light"` su qualunque contenitore.

---

## 3. Quando si usa il monospace

Il monospace non è decorazione: segnala **"questo valore è prodotto o riletto da
una macchina"**.

**Sempre monospace:**

- importi in token, quantità, percentuali, durate, dimensioni (GB, MB), latenze;
- identificativi: id di nodo, id di app, hash, nonce, codici di richiesta;
- date e orari;
- righe del registro eventi;
- intestazioni di colonna e micro-etichette (`.cbx-label`);
- campi di input il cui contenuto verrà riletto dalla macchina (`.cbx-input--data`):
  nomi di cartella, percorsi, importi, la frase di recupero.

**Mai monospace:**

- prosa, spiegazioni, messaggi di errore discorsivi, testi di onboarding;
- etichette dei bottoni e titoli di pagina;
- nomi scelti dall'utente (il nome del dispositivo è prosa, il suo id è dato).

Un errore ricorrente da evitare: mettere in monospace un intero paragrafo perché
contiene un numero. Va in monospace **il numero**, non il paragrafo.

Font: la famiglia è dichiarata in `font.family.mono` con una catena di fallback
di sistema. **Il monospace definitivo è deciso: JetBrains Mono** ([ADR-009]) —
vedi §10. Resta aperta solo l'incorporazione dei file del font nel bundle
Tauri, che è lavoro di un'altra spec.

---

## 4. Come si scrivono i numeri di token

### 4.1 Unità

Deciso da [ADR-009]. Il nome dell'unità è **`credit`**, al plurale **`credits`**;
forma compatta **`cr`**. Resa: classe `.cbx-unit`, senza modificatori.

**L'unità è sempre posposta al numero, mai anteposta, e non è mai resa come
glifo o simbolo isolato.** Non è un dettaglio tipografico: è il portatore del
vincolo permanente di [[PROJECT]] per cui il token non deve poter acquisire
valore monetario neanche di fatto. Un segno che **precede** il numero — `$50`,
`€50` — è la grammatica del denaro. Un'abbreviazione che lo **segue** — `50 kg`,
`340 ms`, `128.40 cr` — è la grammatica della misura. Ogni schermata che scrive
un importo ripete quel vincolo tipograficamente, invece di contraddirlo.

**Questa non è una regola nuova: è la fine dell'unica eccezione.** §7.3 impone
già lo spazio unificatore fra numero e unità per ogni altra unità del prodotto
(`512 GB`, `340 ms`) — il token, reso finora con un glifo segnaposto, era
l'unico valore trattato diversamente. Con [ADR-009] non fa più eccezione: si
scrive `credit`/`credits` per esteso o `cr` in forma compatta, sempre dopo il
numero, esattamente come ogni altra unità del sistema.

**Forma estesa o compatta.** La densità da terminale (§2, leva 1) è una delle
quattro leve dell'identità visiva, e la forma compatta `cr` la serve meglio
della forma estesa in ogni superficie di prodotto — card, tabelle, log,
didascalie: si usa **`cr`** ovunque compare un valore numerico di token. La
forma estesa **`credits`** resta per la prosa discorsiva (frasi che non
riportano una cifra accanto all'unità) e per la documentazione. Se test con
utenti reali mostrassero che `cr` posposto non viene compreso come unità, la
forma estesa va preferita anche a costo di densità ([ADR-009], *Review
conditions*) — non per ragioni di gusto o di brand.

### 4.2 Formato

- **Cifre tabellari obbligatorie** (`font-variant-numeric: tabular-nums`). I
  valori si aggiornano in tempo reale: senza cifre a larghezza fissa la riga
  "balla" mentre la si sta leggendo. Vale anche per i numeri fuori tabella.
- **Separatore delle migliaia: spazio stretto unificatore** (U+202F), mai punto
  né virgola. `1 284.50` resta leggibile e non cambia significato se un giorno il
  prodotto verrà tradotto in lingue dove punto e virgola si scambiano di ruolo.
- **Separatore decimale: il punto**, perché la lingua dell'interfaccia è
  l'inglese (§7.1). Se in futuro si aggiungeranno altre lingue, è l'unico
  carattere del formato che cambierà: il separatore delle migliaia no.
- **Due decimali** per gli importi e per i ritmi (`+18.20 / h`). Non troncare a
  zero decimali negli elenchi contabili: la somma delle righe deve tornare.
- **Mai abbreviare** (`1.2k`) in un elenco di movimenti o in un totale. È
  ammesso solo nelle didascalie di un grafico, dove la precisione non è il punto.
- **Segno esplicito** per i movimenti: `+12.40` per l'emissione, `−30.00` per il
  burn, con il segno meno tipografico U+2212 (non il trattino).
- **Zero è un fatto**: `0.00` si scrive `0.00`. Non si sostituisce con un
  trattino, né si nasconde la riga.
- **Sconosciuto non è zero**: quando l'app non riesce a leggere un valore mostra
  `—` con l'`aria-label` `value unavailable` e una riga che spiega perché.
  Non mostrare mai un numero vecchio senza etichettarlo come vecchio (§6).

### 4.3 Colore dei movimenti

- `color.flow.mint` (verde) = **emesso dal protocollo verso di te**.
- `color.flow.burn` (violetto) = **bruciato da te**.

Il burn **non è rosso**: spendere non è un errore né una perdita, e il rosso in
questo sistema significa "qualcosa non funziona". Il verde qui non significa
"bene": significa "emissione". Ogni importo colorato porta comunque accanto un
badge scritto (**Minted** / **Burned**): il colore non è mai l'unico canale.

---

## 5. Emesso e bruciato: come si rappresenta l'economia

Il modello è mint & burn ([ADR-005]): la spesa distrugge token, il compenso ne
crea di nuovi. Le conseguenze di design:

- **Non esiste un "netto"** in evidenza. Emesso e bruciato sono due totali
  affiancati. La barra `.cbx-meter__track--split` mostra la loro proporzione e
  ha **sempre** una legenda scritta con entrambi i valori.
- Ogni movimento dichiara **la causa** (storage servito, reddito di esistenza,
  abbonamento) e **la controparte** (`app:photo-archive`, `protocol`). Un
  importo senza causa non si mostra.
- Il **reddito di esistenza** va reso visibile come componente a sé del totale
  giornaliero ("24.00 of it for proven presence alone"). È la promessa
  centrale del prodotto e non deve annegare in un totale unico.
- La presenza è **dimostrata**, non dichiarata ([ADR-002]): dove si mostra lo
  stato di salute del nodo si mostrano le prove superate (`142 of 148 challenges`),
  non un semaforo generico.

---

## 6. I quattro stati, più quello nominale

Ogni schermata esiste in cinque stati e nessuno di essi è un ripiego. Sono
specificati e disegnati in `mockups/`.

**Vuoto.** Dice *perché* è vuoto, se è normale, e qual è l'unica azione sensata
adesso. Mai un tono di colpa, mai una schermata bianca. Attenzione: su Coblox
"nessuna sessione" non significa "nessun accredito" — il reddito di esistenza
matura comunque, e lo stato vuoto deve dirlo.

**Caricamento.** Scheletri della **forma e dimensione del contenuto atteso**, così
il layout non salta all'arrivo dei dati. Niente spinner al centro dello schermo:
la struttura della pagina è già informazione. I controlli restano visibili ma
disabilitati, per non far inseguire i bottoni al puntatore.

**Errore.** Distingue **ciò che è rotto da ciò che funziona**. Se il nodo continua
a lavorare mentre la finestra non riesce a leggere i totali, dirlo è la parte più
importante del messaggio. L'errore contiene: che cosa è successo, quali
conseguenze ha (spesso: nessuna sull'accredito), che cosa fa l'app da sola
(riprova ogni 30 s), che cosa può fare l'utente. Su un elenco contabile è
preferibile non mostrare righe piuttosto che mostrarne una parte.

**Offline.** Ogni cifra porta l'etichetta dell'ora a cui si riferisce
(`at 14:32`), e la pagina lo dichiara in banda alta con `.cbx-stale-rule`.
Non si segnala la staleness attenuando il testo: sarebbe una violazione di
contrasto. Va dichiarata anche la conseguenza reale: **offline il nodo non può
dimostrare la propria presenza, quindi il reddito di esistenza è sospeso**
([ADR-002]).

**Nominale.** Lo stato pieno, con dati verosimili — mai `Lorem ipsum`, mai
`123456`: i valori d'esempio devono essere plausibili per la rete reale.

---

## 7. Lingua e tono del copy

### 7.1 La lingua dell'interfaccia è l'inglese

**Tutto il testo che l'utente vede è in inglese**: etichette, bottoni, titoli,
messaggi di errore, stati vuoti, onboarding, microcopy, testi alternativi per
screen reader e `aria-label`.

L'italiano è ammesso **solo nelle note di lavoro interne rivolte al team** — le
annotazioni attorno agli artboard nei mockup, i commenti nel codice, questo
documento. Nulla che veda l'utente. Le pagine di riferimento marcano
esplicitamente questa separazione con un avviso in testa.

> Requisito corretto dal Project Lead in corso d'opera. La formulazione
> precedente della spec ("tono del copy (it/en)") era ambigua.

### 7.2 Registro

La voce è quella di un tecnico onesto che parla a una persona non tecnica senza
farla sentire stupida.

- **Diretta, in seconda persona, al presente.** "Your node is already receiving
  presence income", non "The system will proceed with crediting".
- **Il gergo si usa o si spiega, mai si ammicca.** I termini di protocollo
  (*challenge*, *proof*, *minted*, *burned*, *node*) sono il vocabolario del
  prodotto: si usano con costanza e si spiegano al primo incontro, in un tooltip
  o in una riga di aiuto. Vietato inventare sinonimi rassicuranti che poi non
  compaiono da nessun'altra parte.
- **Niente marketing e niente ammiccamenti hacker.** Nessun "🚀", nessun
  "boom!", nessun "you're in the Matrix".
- **Gli errori non colpevolizzano** e non usano l'imperativo secco. "There is no
  permission to write in this folder" descrive il fatto; "You picked the wrong
  folder" accusa.
- **Le conseguenze irreversibili si nominano.** Nell'onboarding la conferma non è
  "Next": è una spunta che dice che cosa succede se si perdono le parole.
- **Mai promettere valore economico.** Non *earnings*, non *your capital*, non
  *balance*: usare *credited to you*, *minted to you*, *the network used you*.

### 7.3 Convenzioni inglesi

- **Maiuscole: sentence case.** Titoli, bottoni, etichette di campo e voci di
  menu hanno la maiuscola solo sulla prima parola e sui nomi propri
  ("Pause node", "Movement type", "Write down your recovery phrase"). **Mai
  Title Case.** Il maiuscolo integrale è riservato alle micro-etichette monospace
  e ai badge, dove è reso via CSS (`text-transform`) e non scritto nella stringa:
  così la stringa resta traducibile e leggibile dagli screen reader.
- **Punteggiatura**: niente punto finale in etichette, titoli e bottoni; punto
  finale nelle frasi di aiuto, nella prosa e negli errori.
- **Numeri**: separatore decimale **punto** (`128.40`), separatore delle migliaia
  spazio stretto unificatore (§4.2).
- **Date e ore**: orario su 24 ore (`14:32:07`) e date in forma ISO
  (`2026-08-25`) ovunque compaiano accanto a dati di protocollo. È una scelta
  deliberata contro il formato ambiguo `MM/DD` vs `DD/MM`, e coerente con il
  fatto che questi valori sono monospace e di origine macchina (§3).
- **Unità**: spazio unificatore fra numero e unità (`512 GB`, `340 ms`,
  `+18.20 / h`), così non vanno a capo separandosi.
- **Inglese britannico o americano**: scegliere **britannico** e restare
  coerenti (*organisation*, *behaviour*). Un solo dizionario per tutto il
  prodotto.

### 7.4 Localizzazione futura

Non si implementa adesso: si evita solo di ostacolarla.

- **Nessuna stringa concatenata a mano** da frammenti ("You have " + n + "
  sessions"). Una frase è un'unità: la sua struttura cambia da lingua a lingua.
- **Niente testo dentro le immagini**, e nessun significato affidato all'ordine
  delle parole in un layout.
- **Respiro nei layout**: molte lingue sono più lunghe dell'inglese (tedesco e
  italiano arrivano facilmente al +30%). Bottoni ed etichette non hanno larghezze
  fisse, le griglie usano `auto-fit`/`minmax`, e nessun componente si rompe se la
  stringa cresce. Il maiuscolo via CSS anziché in stringa va nella stessa
  direzione.
- **Glossario unico**: un termine di protocollo ha **una** traduzione, sempre la
  stessa, in ogni lingua che verrà aggiunta.

---

## 8. Regole di accessibilità

Non sono un livello di rifinitura: sono un criterio di accettazione.

1. **Contrasto.** Ogni coppia testo/sfondo dichiarata legittima raggiunge almeno
   **4,5:1** (WCAG 1.4.3 AA); componenti e grafica portatrice di significato
   almeno **3:1** (WCAG 1.4.11). Il sistema tiene **anche il testo "muted" a
   4,5:1**, rinunciando volutamente alla deroga per il testo grande: il testo
   tenue su fondo scuro è il modo tipico in cui le UI "da terminale" diventano
   illeggibili. Le coppie ammesse sono elencate in
   [`tokens/contrast-pairs.json`](./tokens/contrast-pairs.json) e verificate da
   `node design/tools/check-contrast.mjs`. **Una combinazione non elencata non è
   autorizzata**: i componenti non inventano accostamenti.
2. **Il colore non è mai l'unico canale.** Ogni stato ha colore + forma del punto
   (pieno / anello / vuoto) + parola scritta. Le schermate restano leggibili in
   scala di grigi e per chi non distingue i colori.
3. **Focus sempre visibile.** `outline` di 2 px in `color.focus.ring` con
   `outline-offset: 2px`, così l'anello poggia sempre sullo sfondo della pagina e
   non sul riempimento del controllo. Non rimuovere mai l'outline senza
   sostituirlo con un indicatore altrettanto contrastato.
4. **Bersagli**: almeno 32 px di altezza per i controlli
   (`layout.hit-target-min`); i bottoni densi da 26 px sono ammessi solo per
   azioni secondarie affiancate a un'alternativa piena.
5. **Movimento.** Una sola animazione ambientale (il "respiro" dell'indicatore
   live), a bassa ampiezza e mai dietro al testo. Tutte le animazioni si
   annullano sotto `prefers-reduced-motion: reduce`, comprese quelle degli
   scheletri di caricamento.
6. **Struttura e alternative testuali.** Ogni tabella ha una `<caption>` (anche
   solo per screen reader), ogni campo la sua `<label>`, ogni grafico un
   `role="img"` con `aria-label` che ne descrive **l'andamento e la scala** — non
   "grafico". Le regioni che cambiano da sole usano `role="status"` o
   `role="log"`; gli errori bloccanti `role="alert"`.
7. **Nessuna informazione solo nel tooltip.** Il tooltip spiega, non contiene mai
   l'unica copia di un dato necessario all'azione.
8. **Zoom e ridimensionamento.** Tipografia in `rem`, griglie con
   `auto-fit`/`minmax` e **container query** sul pannello dei contenuti: il
   layout risponde allo spazio che ha davvero, non a un'ipotesi sulla finestra.

---

## 9. Regole per i token

- **Due livelli.** Primitivi (`color.green.400`, `space.4`) → semantici
  (`color.flow.mint`, `space.card-padding`). **Le superfici di prodotto usano solo
  i semantici.** Un colore letterale in un foglio di stile di prodotto è un bug.
- **Nomi di ruolo, mai di schermata.** `color.bg.surface`, non
  `color-dashboard-card`. I nomi sono neutri rispetto alla piattaforma, così da
  poter essere ri-emessi per Jetpack Compose (`color.bg.app` → `ColorBgApp`)
  senza rinominare nulla.
- **Parità fra i temi.** Ogni chiave semantica esiste in tutti i temi; il
  generatore fallisce se manca (`assertThemeParity`).
- **`tokens.css` è generato.** Si modifica `tokens.json` e si ricostruisce; la
  verifica `--check` fallisce se il CSS committato è disallineato.
- **Aggiungere un colore comporta aggiungere le sue coppie di contrasto** in
  `contrast-pairs.json`. Un token di colore non verificato non entra nel sistema.

---

## 10. Decisioni aperte

| Tema | Stato | Nota |
| --- | --- | --- |
| Nome dell'unità di conto | **Deciso** ([ADR-009]) | `credit`/`credits`, forma compatta `cr`, sempre posposta al numero. Reso da `$meta.unit` e dalla classe `.cbx-unit` (senza modificatori). |
| Font monospace definitivo | **Deciso: JetBrains Mono** ([ADR-009]) | Licenza **SIL OFL 1.1** per il carattere (non Apache-2.0: quella copre solo il codice sorgente del repository, dato corretto da [ADR-009]). Zero barrato, altezza-x generosa, ottima distinzione `1/l/I` e `0/O` — decisive su hash e id. Alternative scartate: IBM Plex Mono (SIL OFL, più sobrio), Fira Code (SIL OFL, ma le legature vanno disattivate: falsano la lettura dei dati). |
| Font sans definitivo | **Proposta: Inter** (SIL OFL) | Fallback di sistema già dichiarato; oggi la pagina non scarica nulla dalla rete. |
| Incorporare i font nell'app | **Aperto** | Oggi le famiglie sono dichiarate con fallback di sistema, quindi le proporzioni variano fra macchine. Per un rendering identico su Windows/Linux i file vanno incorporati nel bundle Tauri: è lavoro di un'altra spec. |
| Mapping Compose per Android | **Fuori scope** | Spec dedicata. La nomenclatura semantica è già predisposta. |
