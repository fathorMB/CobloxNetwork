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
    check-guide-pairs.mjs   sei verifiche di forma, vedi sotto
```

Il pacchetto **non contiene copie** di token, CSS o strumenti del design system:
li carica per percorso relativo dalla cartella sorella, che resta in sola
lettura. Nessuna risorsa arriva dalla rete: la pagina si apre da disco con un
doppio clic.

## Le due direzioni dell'ancoraggio

L'ancoraggio è una cerniera fra la pagina e le regole, e va tenuto in tutte e
due le direzioni. Ciascuna è coperta da uno strumento diverso.

**Regola → pagina.** `sim/tools/published_artifacts.toml` porta 65 probe con
prefisso `guide-`, una per ogni affermazione di proprietà della pagina. Se la
regola cambia o sparisce, `sim/tools/published_artifacts.py` esce diverso da
zero **nominando la frase della guida** che restava indietro. Gira in CI a ogni
push, nel job *Protocol document guards*.

```bash
python sim/tools/published_artifacts.py
```

**Pagina → regola.** Il verso che il manifesto da solo non copre: se la *frase*
cambia, la probe resta a difendere qualcosa che non è più scritto. Ogni probe
`guide-*` porta perciò il campo `claims` con la frase che sostiene, e questo
controllo fallisce se la frase non è più nella pagina.

```bash
node .lmbrain/design/coblox-public-guide/tools/check-guide-pairs.mjs
```

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
