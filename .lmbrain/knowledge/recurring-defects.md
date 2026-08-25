---
title: Difetti ricorrenti e le domande che li intercettano
updated: 2026-08-25
---

# Difetti ricorrenti e le domande che li intercettano

Pagina nata da difetti reali, contati e non ricordati a impressione. Vale per chiunque scriva specifiche, regole di protocollo o affermazioni di sicurezza in questo progetto — Lead compreso, e il Lead in particolare, perché tre delle occorrenze registrate qui sono sue.

Le tre famiglie hanno **un tratto comune** che è la cosa più utile di questa pagina: **in ogni caso il difetto era già scritto da qualche parte nel repository, e nessuno lo stava guardando.** Non sono stati errori di ragionamento. Sono stati errori di *dove si guardava*.

## Famiglia 1 — L'artefatto pubblicato che insegna una forma inammissibile

**Cinque occorrenze in quattro spec.** L'elenco completo, con date e cause, è in [ADR-012], che la chiude con un meccanismo: una gate `before-submit` su ogni spec che introduca o modifichi una regola di validità, eseguita da uno strumento **versionato** e provata **in negativo**.

È l'unica delle tre famiglie meccanizzabile, ed è già meccanizzata. Le altre due no.

**La quinta occorrenza è la prova che il meccanismo funziona, ed è per questo che è annotata qui.** L'esempio canonico di `challenge_evidence` in `ledger.md` portava un `request_hash` diverso dal proprio `challenge_id`, mentre `README.md` impone che siano uguali. L'ha trovata lo strumento di [SPEC-010] alla **prima esecuzione**, contro una regola che esisteva da [SPEC-001] — cioè un difetto che nessuna spec successiva avrebbe avuto ragione di cercare, e che quindi nessuna delle quattro occorrenze precedenti avrebbe fatto emergere. Le prime quattro sono state trovate dal caso; questa da una guardia. **È la differenza fra sapere di avere un problema e avere qualcosa che lo cerca.**

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

## Perché la review adversariale non è facoltativa

Delle occorrenze registrate qui, **nessuna è stata trovata da chi aveva scritto la modifica**. Tutte da una revisione indipendente con mandato di attaccare, e in tre casi da una revisione che attaccava le superfici che l'autore stesso aveva indicato come rischiose — trovandole solide, e trovando i difetti **altrove**.

Ne discende una regola di dispatch: quando si segnalano a un revisore le superfici da guardare, **si dice esplicitamente che non sono il perimetro**. Sono i punti dove l'autore si aspetta di essere attaccato, e un revisore che guardi solo lì fa il gioco di chi ha scritto.

E una nota sul numero di giri. [SPEC-006] ne ha richiesti quattro, [SPEC-009] due. Non è un segnale di lavoro debole: **due dei quattro finding nuovi di ogni giro erano conseguenze delle correzioni del giro precedente**, cioè difetti che nessuno poteva vedere prima. È la forma normale di un invariante di consenso. La parte portante di entrambe le spec non è mai stata toccata da nessuno dei finding.

## Il Lead sbaglia, ed è previsto

Tre errori del Lead in questa sola sessione, tutti trovati dalla review e nessuno da lui: una gate `owner=agent` che richiedeva un'azione vietata a quel ruolo — vedi `review-lifecycle-discipline.md`; un'analisi dell'esistente che attribuiva al workspace un valore letto in un altro manifest; e la richiesta di riabilitazione descritta sopra.

**Ne segue un obbligo per gli specialisti, non una scusa per il Lead.** Il mandato di ogni profilo chiede di contestare assunzioni fragili, e quell'obbligo **vale anche quando l'assunzione arriva dal Lead**. AGENT-002 lo ha formulato meglio di così: *avevo scritto «prematura» perché mi era stato chiesto, e avrei dovuto contestarlo allora invece di eseguirlo*. Contestare una formulazione del Lead fa parte del lavoro e non è attrito.
