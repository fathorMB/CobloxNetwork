#!/usr/bin/env node
/**
 * Generates design/preview/index.html — the component reference page.
 *
 * Why generated: the page must show every component in BOTH themes. Hand-copying
 * the whole gallery twice guarantees the two halves drift apart within a week.
 * The OUTPUT is plain, framework-free, dependency-free HTML+CSS that opens in any
 * browser with no build step — that is the property SPEC-003 requires. This script
 * only removes the copy-paste, it is not a runtime dependency.
 *
 * LANGUAGE RULE: every string rendered INSIDE a component sample is English —
 * English is the interface language of the product. The explanatory notes AROUND
 * the samples are internal working notes for the team and stay in Italian.
 *
 *   node design/tools/build-preview.mjs           # write the page
 *   node design/tools/build-preview.mjs --check   # fail if the committed page is stale
 */

import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const here = dirname(fileURLToPath(import.meta.url));
const OUT = resolve(here, '../preview/index.html');
const tokens = JSON.parse(readFileSync(resolve(here, '../tokens/tokens.json'), 'utf8'));

/** Narrow no-break space: the group separator used everywhere in the system. */
const NNBSP = '&#8239;';
const UNIT = `<span class="cbx-unit cbx-unit--provisional" title="The name of the unit has not been decided yet: typographic placeholder">◇</span>`;

/** Deterministic sparkline path so the artefact is byte-stable across builds. */
function sparkPoints(seed, count = 40, width = 320, height = 44) {
  const raw = [];
  let x = seed;
  for (let i = 0; i < count + 2; i += 1) {
    x = (x * 1103515245 + 12345) % 2147483648;
    raw.push(0.35 + (x / 2147483648) * 0.5);
  }
  // Three-point moving average: real telemetry drifts, it does not vibrate.
  const pts = [];
  for (let i = 0; i < count; i += 1) {
    const v = (raw[i] + raw[i + 1] + raw[i + 2]) / 3;
    const px = (i / (count - 1)) * width;
    const py = height - v * height;
    pts.push(`${px.toFixed(1)},${py.toFixed(1)}`);
  }
  return pts.join(' ');
}

const SPARK_1 = sparkPoints(7);
const SPARK_2 = sparkPoints(23);

function swatches() {
  const names = Object.keys(tokens.semantic.dark).filter(
    (k) => k.startsWith('color.') && !k.includes('scrim')
  );
  return names
    .map(
      (n) => `
            <div class="swatch">
              <span class="swatch__chip" style="background: var(--cbx-${n.replace(/\./g, '-')})"></span>
              <code class="swatch__name">--cbx-${n.replace(/\./g, '-')}</code>
            </div>`
    )
    .join('');
}

const section = (id, title, note, body) => `
        <section class="gallery__section" id="${id}">
          <h3 class="cbx-section-title">${title}</h3>
          <p class="cbx-hint">${note}</p>
          <div class="gallery__demo">${body}
          </div>
        </section>`;

function gallery(theme) {
  return `
      <div class="gallery" data-theme="${theme}">
        <header class="gallery__head">
          <h2 class="cbx-title">Tema ${theme}</h2>
          <p class="cbx-hint">${
            theme === 'dark'
              ? 'Tema primario. È il default su <code>:root</code>: nessun attributo richiesto in produzione.'
              : 'Variante secondaria, degradabile. Attivata da <code>data-theme="light"</code> su qualunque contenitore.'
          }</p>
        </header>
${section(
  `${theme}-buttons`,
  'Bottoni',
  'Un solo bottone primario per schermata. Le azioni distruttive sono piene e dichiarano la conseguenza nell\'etichetta: il colore non è mai l\'unico segnale.',
  `
            <div class="cbx-row cbx-row--wrap">
              <button type="button" class="cbx-btn cbx-btn--primary">Start node</button>
              <button type="button" class="cbx-btn">Settings</button>
              <button type="button" class="cbx-btn cbx-btn--ghost">Cancel</button>
              <button type="button" class="cbx-btn cbx-btn--danger">Delete identity</button>
              <button type="button" class="cbx-btn cbx-btn--sm cbx-btn--mono">--verbose</button>
              <button type="button" class="cbx-btn" disabled>Unavailable offline</button>
            </div>`
)}
${section(
  `${theme}-inputs`,
  'Campi di input',
  'Tutto ciò che la macchina rileggerà — identificatori, chiavi, importi — è monospace. Gli errori sono testo, non solo un bordo rosso.',
  `
            <div class="cbx-grid cbx-grid--2">
              <div class="cbx-field">
                <label class="cbx-label" for="${theme}-i1">Node name</label>
                <input class="cbx-input" id="${theme}-i1" type="text" value="Loft laptop" />
                <span class="cbx-hint">For you only. The network never sees this name.</span>
              </div>
              <div class="cbx-field">
                <label class="cbx-label" for="${theme}-i2">Node identifier</label>
                <input class="cbx-input cbx-input--data" id="${theme}-i2" type="text" value="cbx1q9f0…7ka2" readonly />
                <span class="cbx-hint">Derived from your key. Cannot be changed.</span>
              </div>
              <div class="cbx-field">
                <label class="cbx-label" for="${theme}-i3">Storage to offer</label>
                <input class="cbx-input cbx-input--data" id="${theme}-i3" type="text" value="512 GB" aria-invalid="true" aria-describedby="${theme}-e3" />
                <span class="cbx-field__error" id="${theme}-e3"><span class="cbx-notice__icon" aria-hidden="true">!</span>This disk has 240 GB free. Lower the amount you offer.</span>
              </div>
              <div class="cbx-field">
                <label class="cbx-label" for="${theme}-i4">Participation profile</label>
                <select class="cbx-input" id="${theme}-i4">
                  <option>Always on</option>
                  <option>Only while charging</option>
                  <option>Only on Wi-Fi</option>
                </select>
                <span class="cbx-hint">You can change this at any time.</span>
              </div>
            </div>`
)}
${section(
  `${theme}-cards`,
  'Card e blocchi metrica',
  'La cifra è l\'elemento più grande del blocco: la domanda primaria è sempre “quanto mi ha usato la rete”. L\'unità è un segnaposto finché il nome non è deciso.',
  `
            <div class="cbx-grid cbx-grid--3">
              <div class="cbx-card cbx-card--accent">
                <div class="cbx-stat">
                  <span class="cbx-label">Credited today</span>
                  <span class="cbx-stat__value cbx-stat__value--accent cbx-num cbx-num--hero">128.40${UNIT}</span>
                  <span class="cbx-stat__meta">24.00 of it for proven presence alone</span>
                </div>
              </div>
              <div class="cbx-card">
                <div class="cbx-stat">
                  <span class="cbx-label">Using you right now</span>
                  <span class="cbx-stat__value cbx-num cbx-num--hero">3</span>
                  <span class="cbx-stat__meta">2 storage · 1 compute</span>
                </div>
              </div>
              <div class="cbx-card cbx-card--inset">
                <div class="cbx-stat">
                  <span class="cbx-label">Proofs passed (24 h)</span>
                  <span class="cbx-stat__value cbx-num cbx-num--hero">96<span class="cbx-unit">%</span></span>
                  <span class="cbx-stat__meta">142 of 148 challenges</span>
                </div>
              </div>
            </div>`
)}
${section(
  `${theme}-badges`,
  'Badge di stato',
  'Colore, forma del punto e parola scritta dicono la stessa cosa: lo stato resta leggibile in bianco e nero e per chi non distingue i colori. “Offline” è neutro, non è un errore.',
  `
            <div class="cbx-row cbx-row--wrap">
              <span class="cbx-badge cbx-badge--online"><span class="cbx-dot cbx-dot--live" aria-hidden="true"></span>Online</span>
              <span class="cbx-badge cbx-badge--validating"><span class="cbx-dot cbx-dot--ring" aria-hidden="true"></span>Verifying</span>
              <span class="cbx-badge cbx-badge--degraded"><span class="cbx-dot" aria-hidden="true"></span>Degraded</span>
              <span class="cbx-badge cbx-badge--offline"><span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>Offline</span>
              <span class="cbx-badge cbx-badge--error"><span class="cbx-dot" aria-hidden="true"></span>Error</span>
              <span class="cbx-badge cbx-badge--mint">Minted</span>
              <span class="cbx-badge cbx-badge--burn">Burned</span>
            </div>`
)}
${section(
  `${theme}-table`,
  'Tabella dati',
  'Densità da terminale: righe da 36px, intestazioni monospace, valori allineati a destra con cifre tabellari così che le colonne non ballino quando un valore si aggiorna.',
  `
            <div class="cbx-table__wrap">
              <table class="cbx-table">
                <caption class="cbx-sr-only">Recent movements on this node</caption>
                <thead>
                  <tr>
                    <th scope="col">Time</th>
                    <th scope="col">Event</th>
                    <th scope="col">Counterparty</th>
                    <th scope="col">Direction</th>
                    <th scope="col" class="cbx-cell--num">Amount</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td class="cbx-mono">14:32:07</td>
                    <td>Storage served</td>
                    <td class="cbx-id">app:photo-archive</td>
                    <td><span class="cbx-badge cbx-badge--mint">Minted</span></td>
                    <td class="cbx-cell--num cbx-num--mint">+12.40</td>
                  </tr>
                  <tr>
                    <td class="cbx-mono">14:28:51</td>
                    <td>Monthly subscription</td>
                    <td class="cbx-id">app:open-maps</td>
                    <td><span class="cbx-badge cbx-badge--burn">Burned</span></td>
                    <td class="cbx-cell--num cbx-num--burn">−30.00</td>
                  </tr>
                  <tr>
                    <td class="cbx-mono">14:15:00</td>
                    <td>Presence income</td>
                    <td class="cbx-id">protocol</td>
                    <td><span class="cbx-badge cbx-badge--mint">Minted</span></td>
                    <td class="cbx-cell--num cbx-num--mint">+2.00</td>
                  </tr>
                  <tr>
                    <td class="cbx-mono">14:02:33</td>
                    <td>Compute run</td>
                    <td class="cbx-id">app:indexer</td>
                    <td><span class="cbx-badge cbx-badge--mint">Minted</span></td>
                    <td class="cbx-cell--num cbx-num--mint">+7.85</td>
                  </tr>
                </tbody>
              </table>
            </div>`
)}
${section(
  `${theme}-tooltip`,
  'Tooltip',
  'Spiega un termine di protocollo senza portare via l\'utente. Si apre con hover e con il focus da tastiera; qui il secondo è mostrato già aperto per la revisione statica.',
  `
            <div class="cbx-row cbx-row--wrap" style="padding-top: 96px">
              <span class="cbx-tooltip">
                <button type="button" class="cbx-btn cbx-btn--sm">What is a challenge?</button>
                <span class="cbx-tooltip__bubble" role="tooltip">The network asks your node to prove, with a signature, that it really is online and really holds the data it claims to hold.</span>
              </span>
              <span class="cbx-tooltip">
                <button type="button" class="cbx-btn cbx-btn--sm">Open state (static)</button>
                <span class="cbx-tooltip__bubble cbx-tooltip__bubble--open" role="tooltip">A tooltip never holds the only copy of something you need in order to act.</span>
              </span>
            </div>`
)}
${section(
  `${theme}-toast`,
  'Notifiche e avvisi',
  'I toast riportano un fatto e, se serve, l\'azione per rimediare. Gli avvisi persistenti restano nel flusso della pagina.',
  `
            <div class="cbx-stack">
              <div class="cbx-toast" role="status">
                <span class="cbx-notice__icon" aria-hidden="true">✓</span>
                <span class="cbx-stack cbx-stack--tight">
                  <span class="cbx-toast__title">Node registered</span>
                  <span class="cbx-toast__body">“Loft laptop” is now part of the network. The first challenges arrive within a few minutes.</span>
                </span>
              </div>
              <div class="cbx-toast cbx-toast--warning" role="status">
                <span class="cbx-notice__icon" aria-hidden="true">!</span>
                <span class="cbx-stack cbx-stack--tight">
                  <span class="cbx-toast__title">You are on a metered connection</span>
                  <span class="cbx-toast__body">The node paused storage so it does not use up your data allowance. Presence is still being credited.</span>
                </span>
              </div>
              <div class="cbx-notice" role="note">
                <span class="cbx-notice__icon" aria-hidden="true">i</span>
                <span>The Coblox token measures use of the network. It cannot be converted into money and has no market price.</span>
              </div>
              <div class="cbx-notice cbx-notice--warning" role="note">
                <span class="cbx-notice__icon" aria-hidden="true">!</span>
                <span>Three challenges went unanswered in the last hour. If this continues, presence crediting stops.</span>
              </div>
              <div class="cbx-notice cbx-notice--danger" role="alert">
                <span class="cbx-notice__icon" aria-hidden="true">×</span>
                <span>Cannot read the storage folder. The node stays online but is no longer serving data.</span>
              </div>
            </div>`
)}
${section(
  `${theme}-spark`,
  'Sparkline (attività nel tempo)',
  'Mostra <em>quanta attività</em> c\'è stata, non un valore di mercato: niente candele, niente frecce di performance, nessun asse di prezzo. La scala è dichiarata a parole sotto il grafico.',
  `
            <div class="cbx-grid cbx-grid--2">
              <div class="cbx-card">
                <span class="cbx-label">Challenges passed — last 6 hours</span>
                <svg class="cbx-sparkline" viewBox="0 0 320 44" preserveAspectRatio="none" role="img" aria-label="Challenges passed over the last six hours: steady, between 15 and 22 per ten-minute interval.">
                  <line class="cbx-sparkline__grid" x1="0" y1="43.5" x2="320" y2="43.5" />
                  <polyline class="cbx-sparkline__line" points="${SPARK_1}" />
                </svg>
                <div class="cbx-sparkline__caption"><span>−6 h</span><span>min 15 · max 22 per 10 min</span><span>now</span></div>
              </div>
              <div class="cbx-card">
                <span class="cbx-label">Traffic served — last 6 hours</span>
                <svg class="cbx-sparkline" viewBox="0 0 320 44" preserveAspectRatio="none" role="img" aria-label="Traffic served over the last six hours: variable, between 40 and 180 megabytes per ten-minute interval.">
                  <line class="cbx-sparkline__grid" x1="0" y1="43.5" x2="320" y2="43.5" />
                  <polyline class="cbx-sparkline__line cbx-sparkline__line--series-2" points="${SPARK_2}" />
                </svg>
                <div class="cbx-sparkline__caption"><span>−6 h</span><span>min 40 MB · max 180 MB per 10 min</span><span>now</span></div>
              </div>
            </div>`
)}
${section(
  `${theme}-meter`,
  'Barre di proporzione',
  'Quote di un insieme, mai un prezzo o un obiettivo da raggiungere. La barra emesso/bruciato ha sempre una legenda scritta: la lunghezza da sola non basta.',
  `
            <div class="cbx-grid cbx-grid--2">
              <div class="cbx-card">
                <div class="cbx-meter">
                  <span class="cbx-label">Storage committed by the network</span>
                  <div class="cbx-meter__track" role="img" aria-label="188 gigabytes committed of the 240 you offered, 78 percent.">
                    <div class="cbx-meter__fill" style="width: 78%"></div>
                  </div>
                  <div class="cbx-meter__legend"><span class="cbx-mono">188 GB of 240 GB</span><span class="cbx-mono">78%</span></div>
                </div>
              </div>
              <div class="cbx-card">
                <div class="cbx-meter">
                  <span class="cbx-label">Token movement — today</span>
                  <div class="cbx-meter__track cbx-meter__track--split" role="img" aria-label="Today: 128.40 minted to you, 30.00 burned by you.">
                    <div class="cbx-meter__segment cbx-meter__segment--mint" style="width: 81%"></div>
                    <div class="cbx-meter__segment cbx-meter__segment--burn" style="width: 19%"></div>
                  </div>
                  <div class="cbx-meter__legend">
                    <span><span class="cbx-badge cbx-badge--mint">Minted</span> <span class="cbx-num cbx-num--mint">128.40</span></span>
                    <span><span class="cbx-badge cbx-badge--burn">Burned</span> <span class="cbx-num cbx-num--burn">30.00</span></span>
                  </div>
                </div>
              </div>
            </div>`
)}
${section(
  `${theme}-loading`,
  'Segnaposto di caricamento',
  'Gli scheletri hanno la forma e la dimensione del contenuto atteso, così il layout non salta quando i dati arrivano. Con <code>prefers-reduced-motion</code> lo scintillio è disattivato.',
  `
            <div class="cbx-grid cbx-grid--3">
              <div class="cbx-card">
                <span class="cbx-label">Credited today</span>
                <div class="cbx-skeleton cbx-skeleton--num" role="status" aria-label="Loading the credited total"></div>
                <div class="cbx-skeleton cbx-skeleton--text" style="width: 60%"></div>
              </div>
              <div class="cbx-card">
                <span class="cbx-label">Activity</span>
                <div class="cbx-skeleton cbx-skeleton--block" role="status" aria-label="Loading the chart"></div>
              </div>
              <div class="cbx-card">
                <span class="cbx-label">Rows</span>
                <div class="cbx-stack cbx-stack--tight">
                  <div class="cbx-skeleton cbx-skeleton--text"></div>
                  <div class="cbx-skeleton cbx-skeleton--text" style="width: 82%"></div>
                  <div class="cbx-skeleton cbx-skeleton--text" style="width: 91%"></div>
                </div>
              </div>
            </div>`
)}
${section(
  `${theme}-empty`,
  'Stato vuoto',
  'Dice perché è vuoto, se è normale, e qual è l\'unica azione sensata adesso. Non usa mai un tono di colpa.',
  `
            <div class="cbx-empty">
              <span class="cbx-empty__mark" aria-hidden="true">[ ]</span>
              <span class="cbx-empty__title">Nobody is using your node yet</span>
              <p class="cbx-empty__body">That is normal in the first few hours. The network hands out work as your node proves it stays reachable. Presence income is already being credited in the meantime.</p>
              <button type="button" class="cbx-btn cbx-btn--primary">See how crediting works</button>
            </div>`
)}
${section(
  `${theme}-log`,
  'Flusso eventi',
  'L\'unico punto in cui l\'estetica “terminale” è letterale, perché il contenuto è davvero un registro macchina. Ogni riga resta comunque comprensibile a parole.',
  `
            <div class="cbx-log" role="log">
              <div class="cbx-log__line"><span class="cbx-log__time">14:32:07</span><span class="cbx-log__tag cbx-log__tag--mint">mint</span><span>+12.40 · storage served to app:photo-archive (1.2 GB read)</span></div>
              <div class="cbx-log__line"><span class="cbx-log__time">14:31:58</span><span class="cbx-log__tag cbx-log__tag--proof">proof</span><span>retrievability challenge passed · block 0x8f21 · 340 ms</span></div>
              <div class="cbx-log__line"><span class="cbx-log__time">14:28:51</span><span class="cbx-log__tag cbx-log__tag--burn">burn</span><span>−30.00 · monthly subscription to app:open-maps</span></div>
              <div class="cbx-log__line"><span class="cbx-log__time">14:15:00</span><span class="cbx-log__tag cbx-log__tag--mint">mint</span><span>+2.00 · presence income · window 13:15–14:15</span></div>
              <div class="cbx-log__line"><span class="cbx-log__time">14:14:12</span><span class="cbx-log__tag cbx-log__tag--proof">proof</span><span>signed ping · nonce 0x3ac9 · replied in 88 ms</span></div>
            </div>`
)}
${section(
  `${theme}-kv`,
  'Liste chiave/valore',
  'Per le schede di dettaglio: etichetta monospace maiuscola a sinistra, valore leggibile a destra.',
  `
            <div class="cbx-card">
              <dl class="cbx-kv">
                <dt>Node</dt><dd>Loft laptop</dd>
                <dt>Identifier</dt><dd class="cbx-id">cbx1q9f0x8m3k7ka2</dd>
                <dt>Platform</dt><dd>Windows 11 · desktop</dd>
                <dt>On the network</dt><dd class="cbx-mono">4 d 06 h 12 min</dd>
                <dt>Offering</dt><dd>Availability and storage</dd>
              </dl>
            </div>`
)}
${section(
  `${theme}-type`,
  'Tipografia e formato dei numeri',
  'Sans per la prosa, monospace per ogni valore prodotto dalla macchina. Separatore delle migliaia: spazio stretto unificatore, mai punto o virgola, così la cifra non cambia senso cambiando lingua.',
  `
            <div class="cbx-stack">
              <p class="cbx-prose">Prose explains and reassures; it never winks at the reader. This is the voice a person hears when they need to understand a decision the software made on their behalf.</p>
              <div class="cbx-row cbx-row--wrap">
                <span class="cbx-num cbx-num--hero">1${NNBSP}284.50${UNIT}</span>
                <span class="cbx-num cbx-num--lg">128.40${UNIT}</span>
                <span class="cbx-num cbx-num--md">12.40${UNIT}</span>
                <span class="cbx-num">2.00${UNIT}</span>
              </div>
              <div class="cbx-row cbx-row--wrap">
                <span class="cbx-num cbx-num--mint">+12.40${UNIT}</span>
                <span class="cbx-num cbx-num--burn">−30.00${UNIT}</span>
                <span class="cbx-id">cbx1q9f0x8m3k7ka2</span>
                <span class="cbx-mono">2026-08-25 14:32:07</span>
              </div>
            </div>`
)}
${section(
  `${theme}-steps`,
  'Passi di configurazione',
  'Per le procedure brevi e delicate. La posizione è scritta a parole (“Step 2 of 3”), non solo disegnata.',
  `
            <ol class="cbx-steps">
              <li class="cbx-steps__item" data-state="done">
                <span class="cbx-steps__marker">Step 1 of 3 · done</span>
                <span class="cbx-steps__label">Create your identity</span>
              </li>
              <li class="cbx-steps__item" data-state="current" aria-current="step">
                <span class="cbx-steps__marker">Step 2 of 3 · in progress</span>
                <span class="cbx-steps__label">Write down your recovery phrase</span>
              </li>
              <li class="cbx-steps__item" data-state="todo">
                <span class="cbx-steps__marker">Step 3 of 3</span>
                <span class="cbx-steps__label">Choose what to offer</span>
              </li>
            </ol>`
)}
${section(
  `${theme}-words`,
  'Frase di recupero',
  'Parole numerate e monospace: si trascrivono su carta senza perdere il segno e senza confondere 1 e l, 0 e O.',
  `
            <ol class="cbx-wordgrid">
              ${['anchor', 'birch', 'canyon', 'dolomite', 'ember', 'fresco', 'granite', 'harbour', 'lantern', 'melody', 'nocturne', 'umbrella']
                .map(
                  (w, i) =>
                    `<li class="cbx-wordgrid__item"><span class="cbx-wordgrid__index">${i + 1}</span>${w}</li>`
                )
                .join('\n              ')}
            </ol>`
)}
${section(
  `${theme}-swatches`,
  'Token di colore semantici',
  'Riferimento per gli implementatori. Nessuna superficie di prodotto usa i primitivi: si consumano solo questi nomi di ruolo.',
  `
            <div class="swatches">${swatches()}
            </div>`
)}
      </div>`;
}

const html = `<!doctype html>
<html lang="it" data-theme="dark">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Coblox — Design system, componenti base</title>
    <link rel="stylesheet" href="../tokens/tokens.css" />
    <link rel="stylesheet" href="../css/base.css" />
    <link rel="stylesheet" href="../css/components.css" />
    <link rel="stylesheet" href="../css/app-shell.css" />
    <link rel="stylesheet" href="./preview.css" />
    <!--
      GENERATED by design/tools/build-preview.mjs — do not edit by hand.
      The generated output is intentionally framework-free and script-free:
      open this file directly in a browser, no build and no server required.
    -->
  </head>
  <body>
    <main class="preview">
      <header class="preview__head">
        <p class="cbx-label">Coblox · design system v${tokens.$meta.version} · ${tokens.$meta.spec}</p>
        <h1 class="cbx-title preview__h1">Componenti base</h1>
        <p class="cbx-prose">
          Pagina di riferimento dei componenti, in tema dark (primario) e light (secondario).
          Nessun framework e nessuno script: quello che vedi è HTML e CSS che consumano solo i token
          di <code>design/tokens/</code>. Ogni coppia testo/sfondo qui presente è verificata in
          <code>design/tokens/CONTRAST.md</code>.
        </p>
        <div class="cbx-notice" role="note">
          <span class="cbx-notice__icon" aria-hidden="true">EN</span>
          <span>
            <strong>Lingua dell'interfaccia: inglese.</strong> Tutto il testo <em>dentro</em> i componenti è
            in inglese, perché è la lingua del prodotto. Le note esplicative <em>attorno</em> ai componenti
            sono note di lavoro interne per il team e restano in italiano.
          </span>
        </div>
        <div class="cbx-notice" role="note">
          <span class="cbx-notice__icon" aria-hidden="true">◇</span>
          <span>
            Il nome dell'unità di conto <strong>non è ancora deciso</strong>. Ovunque compaia il segno
            <span class="cbx-unit cbx-unit--provisional">◇</span> si legge “token Coblox”: è un segnaposto
            tipografico, non un simbolo definitivo.
          </span>
        </div>
      </header>
${gallery('dark')}
${gallery('light')}
      <footer class="preview__foot">
        <p class="cbx-hint">
          Sorgente dei token: <code>design/tokens/tokens.json</code> · CSS generato:
          <code>design/tokens/tokens.css</code> · Verifica contrasto:
          <code>node design/tools/check-contrast.mjs</code>
        </p>
      </footer>
    </main>
  </body>
</html>
`;

if (process.argv.includes('--check')) {
  let current = '';
  try {
    current = readFileSync(OUT, 'utf8');
  } catch {
    /* missing counts as drift */
  }
  if (current !== html) {
    console.error('DRIFT: design/preview/index.html is stale. Run: node design/tools/build-preview.mjs');
    process.exit(1);
  }
  console.log('OK: design/preview/index.html matches its generator.');
} else {
  writeFileSync(OUT, html, 'utf8');
  console.log(`Wrote ${OUT} (${html.length} bytes).`);
}
