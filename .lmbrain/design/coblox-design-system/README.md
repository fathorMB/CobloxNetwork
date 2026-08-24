# Coblox — design system

Fondamenta del design system: token, componenti base, schermate chiave.
Prodotto sotto SPEC-003 da AGENT-006 (Lia Wireframe).

Superficie primaria: app desktop Tauri ([ADR-003]), quindi HTML/CSS. I nomi dei
token sono neutri rispetto alla piattaforma per poter essere ri-emessi per
Jetpack Compose in una spec dedicata.

## Da dove si comincia

| Se vuoi… | Apri |
| --- | --- |
| vedere i componenti, dark e light | [`preview/index.html`](./preview/index.html) |
| vedere le tre schermate e i loro stati | [`mockups/index.html`](./mockups/index.html) |
| capire le regole (monospace, numeri, copy, accessibilità) | [`PRINCIPLES.md`](./PRINCIPLES.md) |
| i valori dei token | [`tokens/tokens.json`](./tokens/tokens.json) |
| la verifica del contrasto | [`tokens/CONTRAST.md`](./tokens/CONTRAST.md) |

Tutte le pagine sono HTML e CSS statici, senza framework e senza script: si
aprono direttamente da disco, con doppio clic. Non serve nessun server, nessuna
build e nessuna dipendenza installata.

## Struttura

```text
design/
  PRINCIPLES.md            regole vincolanti del sistema
  tokens/
    tokens.json            SORGENTE dei token (unica cosa da modificare)
    tokens.css             GENERATO da tokens.json
    contrast-pairs.json    coppie colore dichiarate legittime + esenzioni motivate
    CONTRAST.md            GENERATO: tabella dei rapporti di contrasto
  css/
    base.css               reset, tipografia, formattazione dei dati, utility
    components.css         componenti (bottoni, card, tabella, badge, …)
    app-shell.css          telaio dell'app desktop + chrome dei mockup
  preview/
    index.html             GENERATO: galleria dei componenti, dark + light
    preview.css            stili della sola pagina di riferimento
  mockups/
    index.html             GENERATO: indice delle schermate
    dashboard.html         GENERATO: 5 artboard
    attivita.html          GENERATO: 5 artboard
    onboarding.html        GENERATO: 5 artboard
  tools/
    build-tokens.mjs       tokens.json  -> tokens.css
    build-preview.mjs      -> preview/index.html
    build-mockups.mjs      -> mockups/*.html
    check-contrast.mjs     verifica WCAG + CONTRAST.md
```

### Perché alcune pagine sono generate

La pagina dei componenti deve mostrare **la stessa galleria in due temi**, e le
schermate devono condividere **un solo telaio applicativo fra quindici artboard**.
Copiare quel markup a mano garantisce che le copie divergano. I generatori
eliminano la duplicazione; **l'output committato resta HTML/CSS puro, apribile da
disco senza build** — che è la proprietà richiesta dalla spec. I generatori non
sono una dipendenza di runtime: servono solo a chi modifica il design system.

**Non modificare a mano** i file marcati `GENERATO`: si modifica il generatore (o
`tokens.json`) e si ricostruisce.

## Rigenerare e verificare

Serve solo Node (nessun `npm install`, nessuna dipendenza esterna).

```bash
node design/tools/build-tokens.mjs          # tokens.json -> tokens.css
node design/tools/build-preview.mjs         # -> preview/index.html
node design/tools/build-mockups.mjs         # -> mockups/*.html
node design/tools/check-contrast.mjs --write # verifica WCAG + CONTRAST.md
```

Controllo di non-regressione (esce con codice ≠ 0 se qualcosa è disallineato o
se una coppia di colori non passa AA):

```bash
node design/tools/build-tokens.mjs --check
node design/tools/build-preview.mjs --check
node design/tools/build-mockups.mjs --check
node design/tools/check-contrast.mjs
```

## Lingua

**L'interfaccia del prodotto è in inglese**: ogni stringa visibile all'utente,
compresi `aria-label` e testi per screen reader. L'italiano resta solo nelle note
di lavoro interne per il team — le annotazioni attorno agli artboard, i commenti
nel codice, questi documenti. Le regole complete sono in
[`PRINCIPLES.md` §7](./PRINCIPLES.md).

## Regole d'uso per gli implementatori

1. Consuma **solo i token semantici** (`--cbx-color-*`, `--cbx-space-*`, …). Un
   colore letterale nel CSS di prodotto è un bug.
2. Il tema dark è il default su `:root`; il light si attiva con
   `data-theme="light"` su qualunque contenitore.
3. Ogni nuova coppia testo/sfondo va aggiunta a `tokens/contrast-pairs.json` e
   deve passare `check-contrast.mjs` prima di essere usata.
4. Le classi `mock-*` e quelle di `preview/preview.css` sono chrome delle pagine
   di riferimento: il codice di prodotto non deve dipenderne.
5. Prima di disegnare qualcosa di nuovo, leggi [`PRINCIPLES.md`](./PRINCIPLES.md):
   in particolare §1 (cosa non si disegna) e §6 (i quattro stati).
