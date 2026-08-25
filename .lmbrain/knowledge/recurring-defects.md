---
title: Difetti ricorrenti e le domande che li intercettano
updated: 2026-08-25
---

# Difetti ricorrenti e le domande che li intercettano

Pagina nata da difetti reali, contati e non ricordati a impressione. Vale per chiunque scriva specifiche, regole di protocollo o affermazioni di sicurezza in questo progetto — Lead compreso, e il Lead in particolare, perché tre delle occorrenze registrate qui sono sue.

Le famiglie hanno **un tratto comune** che è la cosa più utile di questa pagina: **in ogni caso il difetto era già scritto da qualche parte nel repository, e nessuno lo stava guardando.** Non sono stati errori di ragionamento. Sono stati errori di *dove si guardava*.

## Famiglia 1 — L'artefatto pubblicato che insegna una forma inammissibile

**Sei occorrenze in cinque spec.** L'elenco completo, con date e cause, è in [ADR-012], che la chiude con un meccanismo: una gate `before-submit` su ogni spec che introduca o modifichi una regola di validità, eseguita da uno strumento **versionato** e provata **in negativo**.

È l'unica delle tre famiglie meccanizzabile. È meccanizzata **per le coerenze fra copie**; per la correttezza semantica di una tabella lo è solo dove esiste un oracolo eseguibile, e allora appartiene alla suite di conformità. Le altre due famiglie non lo sono affatto.

**La quinta occorrenza è la prova che il meccanismo funziona, ed è per questo che è annotata qui.** L'esempio canonico di `challenge_evidence` in `ledger.md` portava un `request_hash` diverso dal proprio `challenge_id`, mentre `README.md` impone che siano uguali. L'ha trovata lo strumento di [SPEC-010] alla **prima esecuzione**, contro una regola che esisteva da [SPEC-001] — cioè un difetto che nessuna spec successiva avrebbe avuto ragione di cercare, e che quindi nessuna delle quattro occorrenze precedenti avrebbe fatto emergere. Le prime quattro sono state trovate dal caso; questa da una guardia. **È la differenza fra sapere di avere un problema e avere qualcosa che lo cerca.**

**La sesta occorrenza dice dove la guardia non arriva, e nega la frase «già meccanizzata» presa alla lettera.** La tabella degli esiti `ed25519-speccheck` in `README.md` dichiarava `accept` al vettore 8, contro la regola scritta due paragrafi sopra la tabella stessa. Lo strumento di [ADR-012] non poteva trovarla e lo dichiara nella propria intestazione: verifica forme e coerenze fra copie, mai la **correttezza semantica** di un valore. L'ha trovata [REVIEW-018] con un oracolo indipendente — cioè una review, non una guardia: **è la prima della famiglia trovata così.** La meccanizzazione esiste, ma per questa classe vive nella suite di conformità, dove `speccheck_conformance.rs` ora estrae la tabella dal documento invece di trascriverla.

**Il difetto ha una seconda metà che riguarda chi consegna.** L'evidenza di [SPEC-012] affermava un accordo perfetto con la tabella pubblicata mentre la fixture consegnata nello stesso commit diceva il contrario, e la causa meccanica era una costante *etichettata* come il documento ma trascritta dall'implementazione: il test confrontava l'implementazione con sé stessa attraverso due copie. **Una costante che dice di essere un documento senza esserne derivata non è una copia in più: è la copia che impedisce di accorgersi delle altre.** La domanda che lo intercetta è banale una volta scritta — *questa costante da dove è stata letta?*

**La domanda che la intercetta:** *quale artefatto pubblicato questa regola nuova rende inammissibile, fra quelli che non sto toccando?*

## Famiglia 2 — L'affermazione rimasta indietro rispetto alla regola

**Cinque occorrenze accertate**, tutte in documenti di protocollo, tutte trovate da review adversariale e mai da chi aveva scritto la modifica.

- `identity.md` affermava che un light client «può vedere» una transizione forzata da revoca, mentre `ledger.md` aveva già **ritrattato su sé stesso** esattamente quella frase. Il documento sorella conservava la versione riconosciuta falsa.
- Il residuo `(g)` nominava la censura **totale** mentre il vettore reale, stabilito due sezioni più in là nello stesso documento, era la **selettiva**.
- «La soglia effettiva torna a due terzi» — rivendicata, confutata dalla censura selettiva, rivendicata di nuovo dopo una regola nuova, e confutata **dalla stessa obiezione**.
- «Non esiste una regola onesta da scrivere qui»: un'**impossibilità dichiarata a torto**, la forma peggiore, perché dice al lettore successivo di smettere di cercare. La regola esisteva.
- Due affermazioni di **minimalità** — «la più piccola istanza che…» — entrambe false, la seconda introdotta *correggendo* la prima.

**Il tratto ricorrente:** la sostanza attorno all'affermazione era corretta ogni volta. A sbagliare era **la frase**, che prometteva una proprietà più forte di quella dimostrata, o che sopravviveva alla regola che l'aveva resa vera.

**Tre domande che la intercettano**, da porsi su ogni frase che afferma una proprietà:

1. **Quale regola la tiene?** Se la risposta è «il valore è scelto bene», non è una proprietà: è una preferenza. È la formulazione di AGENT-007 in [REVIEW-011], ed è diventata [ADR-010].
2. **È una proprietà o un superlativo?** *Minimo*, *massimo*, *l'unico*, *non esiste*: sono affermazioni su un intero spazio, e vanno **dimostrate per esaurimento o non fatte**. Tre superlativi su tre, in questo progetto, sono stati falsificati.
3. **Quale regola la renderà obsoleta?** Se una modifica futura può renderla falsa, quella frase va aggiornata *nella stessa passata* in cui la regola cambia, non dopo. È il criterio che ha prodotto la quinta occorrenza, perché una sola delle due frasi note fu aggiornata.

**Una regola di forma, imparata a caro prezzo.** Quando si ritratta un'affermazione, si ritratta — **non si riabilita**. Il Lead ha chiesto in [REVIEW-013] di riscrivere una ritrattazione come «era prematura, non sbagliata». Era sbagliata. La riabilitazione **rimuove il precedente che avrebbe impedito l'occorrenza successiva**, e l'occorrenza successiva è arrivata nella stessa spec. La traccia storica si conserva per intero, con l'errore chiamato errore.

## Famiglia 3 — La grandezza vincolata non è quella da cui la proprietà dipende

**Tre occorrenze in una sola spec**, [SPEC-009], trovate tutte da AGENT-007 in [REVIEW-014] che ne ha dato la diagnosi:

> È stata vincolata la grandezza nominata dall'ADR, non la grandezza da cui la proprietà dipende.

- Vincolato il **possesso** del set sotto i due terzi; ciò che conta è il **quorum**, che una coalizione ottiene al 48%.
- Vincolato il **tetto per epoca** del fondo; il denominatore è la **durata dell'epoca**, lasciata libera — accorciarla moltiplica l'emissione reale senza violare il tetto.
- Vincolato il **pavimento di eleggibilità**; è denominato in un'**unità** che la governance definiva liberamente.

**È la famiglia 2 vista dal lato della regola invece che dal lato della frase**, ed è il livello più profondo dello stesso principio: non basta chiedersi *quale regola tiene questa proprietà*, bisogna chiedersi **da quali grandezze la proprietà dipende davvero**, e vincolare quelle.

**Due domande che la intercettano:**

1. **Qual è il denominatore?** Ogni tetto *per qualcosa* ha un denominatore, e il denominatore è una grandezza quanto il tetto.
2. **In quale unità è espressa?** Un pavimento denominato in un'unità che qualcun altro definisce non è un pavimento.

E una terza che AGENT-007 giudica il punto in cui l'errore sarebbe **facile e invisibile**: **in quale direzione sta il pericolo?** Tetti dove il pericolo è verso l'alto, **pavimenti** dove è verso il basso. In [SPEC-009] tre dei sette limiti nuovi vanno nella direzione opposta a quella che l'intuizione suggerisce.

## Famiglia 4 — La clausola normativa che nessun oracolo esercita

**Una occorrenza accertata**, trovata da AGENT-007 in [REVIEW-019] mentre la spec che l'ha introdotta, [SPEC-012], era già passata per una review e ne era uscita corretta.

La regola 1 di `README.md` §*Consensus-critical Ed25519 verification* prescriveva due comportamenti di decodifica. I dodici vettori `ed25519-speccheck` — l'oracolo della spec, la sua gate, la ragione stessa per cui la spec esisteva — ne esercitavano **uno**. Delle ventiquattro codifiche di punto della fixture, **nessuna** aveva `y` mascherata `>= 2^255-19`, che è la metà che la regola nomina per prima. La metà esercitata dai vettori 8–11 — `x = 0` con bit di segno 1 — la regola non la nominava affatto, e la prosa la descriveva come una `y` non canonica quando `p-1` è canonica.

**È diversa dalla famiglia 1 e va tenuta separata.** La famiglia 1 è un artefatto pubblicato che dice una cosa *falsa*; qui ogni parola era vera. Il difetto è che la gate portava evidenza per una metà della regola e nessuna evidenza per l'altra, e nulla nel verde lo diceva. Due implementazioni che passano tutti e dodici i vettori — una che riduce `y >= p`, una che lo rifiuta come impone RFC 8032 §5.1.3 — danno verdetto opposto su una firma che qualunque possessore di chiave costruisce in tempo costante. Il costo dell'attacco è nullo e il bersaglio è un voto di finalità.

**La classe pericolosa non è quella che ci si aspetta**, ed è la precisazione del Lead che rende il finding più forte: l'implementazione *pienamente* stretta secondo RFC 8032 fallisce il vettore 9, quindi i dodici la escludono già. Quella che passa tutti e dodici ed è identica a Coblox fino all'input costruito è l'**intermedia**: ZIP-215 sul bit di segno, RFC sulla canonicità di `y` — cioè la lettura più diligente, non la più sciatta.

**La chiusura ha la forma della famiglia 1 rovesciata.** Non correggere un artefatto, ma **costruire l'oracolo che mancava**: sette vettori di estensione Coblox in un file separato dalla fixture upstream, che resta identica byte per byte, con gli esiti derivati dalla regola e confermati dall'oracolo versionato, e la prova in negativo eseguita a ogni esecuzione invece che trascritta una volta (`ed25519_speccheck_oracle.py --decoder strict_y`, e l'asserzione Rust che i dodici concordano e i nuovi divergono).

**Le domande che la intercettano**, ed è la coppia, perché una sola delle due lascia scoperto l'altro verso:

1. **Per ogni clausola della regola, quale vettore la esercita?** Se la risposta è «nessuno», la gate è verde su metà di ciò che dichiara.
2. **Per ogni vettore, quale clausola della regola lo nomina?** Se la risposta è «nessuna», la regola tace su un comportamento che l'oracolo ha già reso normativo.

E una terza, che è il criterio con cui giudicare se un vettore nuovo vale qualcosa: **quale seconda implementazione plausibile questo vettore esclude, che i vettori esistenti non escludono già?** Un vettore che nessuna implementazione ragionevole fallisce è copertura di decodifica, non evidenza — e va pubblicato dicendolo.

## Perché la review adversariale non è facoltativa

Delle occorrenze registrate qui, **nessuna è stata trovata da chi aveva scritto la modifica**. Tutte da una revisione indipendente con mandato di attaccare, e in tre casi da una revisione che attaccava le superfici che l'autore stesso aveva indicato come rischiose — trovandole solide, e trovando i difetti **altrove**.

Ne discende una regola di dispatch: quando si segnalano a un revisore le superfici da guardare, **si dice esplicitamente che non sono il perimetro**. Sono i punti dove l'autore si aspetta di essere attaccato, e un revisore che guardi solo lì fa il gioco di chi ha scritto.

E una nota sul numero di giri. [SPEC-006] ne ha richiesti quattro, [SPEC-009] due. Non è un segnale di lavoro debole: **due dei quattro finding nuovi di ogni giro erano conseguenze delle correzioni del giro precedente**, cioè difetti che nessuno poteva vedere prima. È la forma normale di un invariante di consenso. La parte portante di entrambe le spec non è mai stata toccata da nessuno dei finding.

## Il Lead sbaglia, ed è previsto

Tre errori del Lead in questa sola sessione, tutti trovati dalla review e nessuno da lui: una gate `owner=agent` che richiedeva un'azione vietata a quel ruolo — vedi `review-lifecycle-discipline.md`; un'analisi dell'esistente che attribuiva al workspace un valore letto in un altro manifest; e la richiesta di riabilitazione descritta sopra.

**Ne segue un obbligo per gli specialisti, non una scusa per il Lead.** Il mandato di ogni profilo chiede di contestare assunzioni fragili, e quell'obbligo **vale anche quando l'assunzione arriva dal Lead**. AGENT-002 lo ha formulato meglio di così: *avevo scritto «prematura» perché mi era stato chiesto, e avrei dovuto contestarlo allora invece di eseguirlo*. Contestare una formulazione del Lead fa parte del lavoro e non è attrito.
