# Coblox — guida pubblica

La pagina che spiega come funziona Coblox a chi non ha mai letto una specifica
di protocollo. Prodotta sotto [SPEC-015] da AGENT-006 (Lia Wireframe).

Serve due scopi insieme, ed è la loro tensione a definire la pagina:
**onboarding** — chi accende un nodo deve capire cosa sta facendo — e
**trasparenza** — chi entra deve sapere cosa il sistema espone di lui *prima* di
esporlo.

| Se vuoi… | Apri |
| --- | --- |
| leggere la guida | [`index.html`](./index.html) |
| gli accostamenti di colore che la pagina impegna | [`used-pairs.json`](./used-pairs.json) |
| le regole del design system | [`../coblox-design-system/PRINCIPLES.md`](../coblox-design-system/PRINCIPLES.md) |

La pagina **è in inglese**, perché `PROJECT.md` include la documentazione
pubblica fra ciò che vede l'utente finale. Questi appunti di lavoro restano in
italiano e non compaiono da nessuna parte dentro la pagina.

## La regola di forma, che è tutta la spec in una riga

> Se la versione semplice è più forte di quella esatta, la versione semplice è
> sbagliata.

«I tuoi credits sono al sicuro» è più semplice e più falso di «nessuno può
toglierteli, ma chi ti guarda vede quanto ne hai».

Da questa discendono le due regole operative della pagina:

1. **Il punto scomodo sta nel filo principale, in una frase semplice.
   L'apribile porta l'esattezza, mai la notizia.** Il criterio è verificabile e
   va verificato a ogni modifica: *leggendo il filo con tutti i blocchi chiusi,
   le tre cose scomode devono essere già state dette.* Se una compare solo
   aprendo, l'apribile è diventato il posto dove si nasconde ciò che imbarazza,
   ed è il modo in cui questa pagina fallisce sembrando riuscita.
2. **Ogni affermazione di proprietà porta una probe** verso la regola che la
   tiene. Se un'affermazione non è ancorabile perché nessuna regola la tiene,
   non è una semplificazione: è un'invenzione, e va tolta o riscritta.

Le tre cose scomode, che devono restare nel filo:

- la rete è robusta contro la falsificazione ma **non è resistente ai Sybil per
  via crittografica** ([ADR-007]) — §01;
- gli abbonamenti sono scritti in un **registro pubblico e permanente** accanto
  a un identificatore stabile ([ADR-014]) — §04;
- il design è **pseudonimo, non anonimo**, e la pseudonimia è stabile quindi
  debole (TM-28) — §03.

Il §06 le raccoglie tutte e tre, ma non è dove vengono dette per la prima volta.
Se un giorno lo diventasse, la pagina sarebbe da rifare.

## Struttura

```text
coblox-public-guide/
  index.html      la guida (inglese, senza framework, senza script, senza rete)
  guide.css       il solo strato di pagina; nessun token ridefinito
  used-pairs.json gli accostamenti di colore impegnati, sottoinsieme dichiarato
                  di ../coblox-design-system/tokens/contrast-pairs.json
  tools/
    check-guide-pairs.mjs          sei verifiche di forma, vedi sotto
    check-guide-pairs-negative.mjs la prova in negativo di G6: ogni
                                   affermazione osservata fallire da sola
```

Il pacchetto **non contiene copie** di token, CSS o strumenti del design system:
li carica per percorso relativo dalla cartella sorella, che resta in sola
lettura. Nessuna risorsa arriva dalla rete: la pagina si apre da disco con un
doppio clic.

## Le due direzioni dell'ancoraggio

L'ancoraggio è una cerniera fra la pagina e le regole, e va tenuto in tutte e
due le direzioni. Ciascuna è coperta da uno strumento diverso.

**Regola → pagina.** `sim/tools/published_artifacts.toml` porta 84 probe con
prefisso `guide-`, che tengono 86 affermazioni di proprietà della pagina. Se la
regola cambia o sparisce, `sim/tools/published_artifacts.py` esce diverso da
zero **nominando la frase della guida** che restava indietro. Gira in CI a ogni
push, nel job *Protocol document guards*.

```bash
python sim/tools/published_artifacts.py
```

**Pagina → regola.** Il verso che il manifesto da solo non copre: se la *frase*
cambia, la probe resta a difendere qualcosa che non è più scritto. Ogni probe
`guide-*` porta perciò il campo `claims` con le frasi che sostiene, e questo
controllo fallisce se una di quelle frasi non è più nella pagina.

**`claims` è una lista, e le probe restano una per regola.** La pagina afferma
alcune cose due volte — la promessa anti-confisca sta in §03 e in §07 — e con una
stringa sola la seconda occorrenza restava indifesa comunque la si scegliesse.
L'alternativa era due probe sulla stessa regola: scartata perché la lista
mantiene la corrispondenza **regola → probe uno-a-uno**, che è ciò che servirà a
[DEBT-032] per camminare le regole e verificare che non si siano mosse. Una
stringa nuda resta valida e si legge come lista di uno; nel file non ne resta
nessuna. `published_artifacts.py` verifica la **forma** del campo e rifiuta due
probe che rivendichino la stessa frase, perché il lettore minimo di
`check-guide-pairs.mjs` salterebbe in silenzio una voce malformata, e un'ancora
saltata è indistinguibile da un'ancora che tiene.

```bash
node .lmbrain/design/coblox-public-guide/tools/check-guide-pairs.mjs
```

Il numero non va tenuto a mente: il colophon della pagina lo dichiara al
lettore e `check-guide-pairs.mjs` fallisce se i due divergono. Prima di
[REVIEW-031] il colophon prometteva invece *«every sentence»*, che è un
superlativo universale non enumerato: il meccanismo è un elenco di ancore
scelte a mano, e dirlo con il numero è insieme più vero e più impressionante.

**La `claims` finisce dove finisce la frase, punto fermo compreso.** È la lezione
di [REVIEW-031], e non è un dettaglio di stile. Una `claims` che si ferma un
carattere prima del punto lascia la frase libera di crescere una virgola e una
clausola nuova senza che nulla se ne accorga: è esattamente così che
*«whatever anybody intends»* è sopravvissuta accanto a una probe verde. La
domanda da fare a ogni probe è una sola — *la frase che pinno si ferma prima
della clausola che porta il rischio?* — e la ragione per cui non è mai
automatizzabile del tutto è che **la clausola che eccede non è ancorabile
proprio in quanto eccede**: non c'è una regola a cui agganciarla, quindi cade
fuori dall'ancoraggio per costruzione. Lo strumento può misurare se una `claims`
copre la sua frase fino in fondo; non può decidere se la frase dica il vero.

**La prova in negativo di G6, e perché non basta un caso solo.** Provare che
*una* affermazione fallisce non dice nulla sulle altre ottantacinque: un'ancora
scritta contro un testo poi riscritto continua a essere letta e a passare, ha
solo smesso di ancorare qualcosa ([SKILL-001]). `check-guide-pairs-negative.mjs`
cancella dalla pagina, una alla volta, ogni frase rivendicata da una probe e
pretende che la gate esca non-zero **nominando quella probe**; poi ripristina e
riverifica il verde. Aggiunge quattro casi sulla *forma* introdotta dallo
schema — lista vuota, voce vuota, campo assente, e il numero del colophon
disallineato — che la cancellazione di una frase non esercita.

```bash
node .lmbrain/design/coblox-public-guide/tools/check-guide-pairs-negative.mjs
```

Le due prove in negativo coprono **una direzione ciascuna**, ed è la ragione per
cui servono entrambe: `sim/tools/published_artifacts_negative.py` cancella la
*regola* dal documento e pretende che C10 nomini la probe; questa cancella la
*frase* dalla pagina e pretende che G6 nomini la probe. Nessuna delle due
sorveglia la direzione di [DEBT-032] — la regola che si sposta restando
presente — e quel debito resta aperto.

Lo stesso strumento verifica altre cinque cose che nessun altro guarda: nessun
colore letterale, nessun token inventato, nessun accostamento fuori dall'elenco
verificato di [SPEC-005], le regole tipografiche di [ADR-009] (unità posposta,
separatore U+202F, glifo `◇` assente), e l'assenza di risorse di rete e di
script. Le sei classi sono state provate in negativo una per una.

Il rapporto di contrasto in sé non è ricalcolato qui: lo prova
`../coblox-design-system/tools/check-contrast.mjs`, che va eseguito insieme.
Riscriverne la formula produrrebbe due copie destinate a divergere.

```bash
node .lmbrain/design/coblox-design-system/tools/check-contrast.mjs
```

## Cosa non c'è in questa pagina, e perché

- **Il testo pubblico che attua [ADR-014]** — la dichiarazione formale
  `SEC-REQ-22` su cosa è pubblico e correlabile. È un documento di sicurezza
  proprio, con un proprio `GATE-SECREVIEW`, e **non esiste ancora**. La guida
  dice il fatto e vi rimanda invece di sostituirlo: due copie di una
  dichiarazione di sicurezza divergono. Il §04 dice esplicitamente che la
  dichiarazione manca, invece di rimandare a un documento che il lettore non
  troverebbe.
- **La localizzazione.** Fuori scope, ma nulla qui la preclude: nessuna stringa
  concatenata, nessun testo dentro le immagini, maiuscolo via CSS, numeri di
  sezione resi da `::before` e non scritti nella stringa.
