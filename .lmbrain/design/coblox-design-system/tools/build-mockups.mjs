#!/usr/bin/env node
/**
 * Generates design/mockups/*.html — three key desktop screens, each in five
 * artboards: the nominal state plus the four states SPEC-003 requires
 * (empty, loading, error, offline).
 *
 * As with the preview page, the OUTPUT is plain framework-free HTML+CSS that
 * opens straight from disk. The generator exists so the fifteen artboards share
 * one application shell and cannot drift apart.
 *
 *   node design/tools/build-mockups.mjs
 *   node design/tools/build-mockups.mjs --check
 */

import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const here = dirname(fileURLToPath(import.meta.url));
const DIR = resolve(here, '../mockups');
const tokens = JSON.parse(readFileSync(resolve(here, '../tokens/tokens.json'), 'utf8'));

/** Decided by ADR-009: the compact unit form, always posposed to the number
    (measure grammar — "50 kg", not currency grammar — "$50"). The visual gap
    is the existing `.cbx-unit` margin, matching how every other posposed unit
    in this system (GB, ms) is set. */
const UNIT = `<span class="cbx-unit">cr</span>`;

function sparkPoints(seed, count = 48, width = 320, height = 44) {
  const raw = [];
  let x = seed;
  for (let i = 0; i < count + 2; i += 1) {
    x = (x * 1103515245 + 12345) % 2147483648;
    raw.push(0.3 + (x / 2147483648) * 0.55);
  }
  const pts = [];
  for (let i = 0; i < count; i += 1) {
    const v = (raw[i] + raw[i + 1] + raw[i + 2]) / 3;
    pts.push(`${((i / (count - 1)) * width).toFixed(1)},${(height - v * height).toFixed(1)}`);
  }
  return pts.join(' ');
}

/* ----------------------------------------------------------------- fragments */

const conn = {
  live: `<span class="cbx-conn"><span class="cbx-dot cbx-dot--live" aria-hidden="true"></span>Live · updated 2 s ago</span>`,
  connecting: `<span class="cbx-conn cbx-conn--validating"><span class="cbx-dot cbx-dot--ring" aria-hidden="true"></span>Connecting</span>`,
  error: `<span class="cbx-conn cbx-conn--error"><span class="cbx-dot" aria-hidden="true"></span>No response from the network</span>`,
  offline: `<span class="cbx-conn cbx-conn--offline"><span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>Offline · last data 14:32</span>`
};

const nav = (current) =>
  [
    ['panoramica', '▤', 'Overview'],
    ['attivita', '↯', 'Activity'],
    ['nodi', '▣', 'My nodes'],
    ['app', '◆', 'Network apps'],
    ['impostazioni', '⚙', 'Settings']
  ]
    .map(
      ([key, glyph, label]) =>
        `<a class="cbx-nav__item" href="#"${key === current ? ' aria-current="page"' : ''}>` +
        `<span class="cbx-nav__glyph" aria-hidden="true">${glyph}</span>${label}</a>`
    )
    .join('\n            ');

const shell = ({ current, head, main }) => `
        <div class="cbx-app">
          <div class="cbx-app__brand"><span class="cbx-app__brand-mark" aria-hidden="true">◈</span>Coblox</div>
          <div class="cbx-app__topbar">
            ${head}
          </div>
          <nav class="cbx-app__nav" aria-label="Sections">
            ${nav(current)}
          </nav>
          <main class="cbx-app__main">
${main}
          </main>
        </div>`;

const topbar = (connState, right = '') =>
  `${connState}<span class="cbx-spacer"></span>${right}<span class="cbx-badge cbx-badge--online"><span class="cbx-dot cbx-dot--live" aria-hidden="true"></span>1 node active</span>`;

const pageHead = (title, sub, actions = '') => `
            <div class="cbx-page-head">
              <div class="cbx-stack cbx-stack--tight">
                <h2 class="cbx-title">${title}</h2>
                <p class="cbx-hint">${sub}</p>
              </div>
              <div class="cbx-row">${actions}</div>
            </div>`;

const stat = (label, value, meta, { accent = false, hero = true } = {}) => `
                <div class="cbx-card${accent ? ' cbx-card--accent' : ''}">
                  <div class="cbx-stat">
                    <span class="cbx-label">${label}</span>
                    <span class="cbx-stat__value${accent ? ' cbx-stat__value--accent' : ''} cbx-num ${hero ? 'cbx-num--hero' : 'cbx-num--lg'}">${value}</span>
                    <span class="cbx-stat__meta">${meta}</span>
                  </div>
                </div>`;

const skeletonStat = (label) => `
                <div class="cbx-card">
                  <div class="cbx-stat">
                    <span class="cbx-label">${label}</span>
                    <div class="cbx-skeleton cbx-skeleton--num" role="status" aria-label="Loading ${label}"></div>
                    <div class="cbx-skeleton cbx-skeleton--text" style="width: 62%"></div>
                  </div>
                </div>`;

/** A value the app genuinely does not know right now. Never a stale number
    dressed up as live, never a zero (zero is a fact, unknown is not). */
const unknown = (why) =>
  `<span class="cbx-num cbx-num--hero" aria-label="value unavailable">—</span>` +
  `<span class="cbx-stat__meta">${why}</span>`;

const sessionsTable = (rows) => `
              <div class="cbx-table__wrap">
                <table class="cbx-table">
                  <caption class="cbx-sr-only">Who is using your node right now</caption>
                  <thead>
                    <tr>
                      <th scope="col">Who</th>
                      <th scope="col">Resource</th>
                      <th scope="col">For</th>
                      <th scope="col">Status</th>
                      <th scope="col" class="cbx-cell--num">Rate<span class="cbx-sr-only"> of crediting, tokens per hour</span></th>
                    </tr>
                  </thead>
                  <tbody>
${rows}
                  </tbody>
                </table>
              </div>`;

const sessionRow = (who, what, since, badge, rate) => `
                    <tr>
                      <td class="cbx-id">${who}</td>
                      <td>${what}</td>
                      <td class="cbx-mono">${since}</td>
                      <td>${badge}</td>
                      <td class="cbx-cell--num cbx-num--mint">${rate}</td>
                    </tr>`;

const logLines = (lines) => `
              <div class="cbx-log" role="log" aria-label="Recent node events">
${lines
  .map(
    ([time, tag, text]) =>
      `                <div class="cbx-log__line"><span class="cbx-log__time">${time}</span><span class="cbx-log__tag cbx-log__tag--${tag}">${tag}</span><span>${text}</span></div>`
  )
  .join('\n')}
              </div>`;

const nodeCard = (body) => `
              <div class="cbx-card">
                <div class="cbx-card__head"><span class="cbx-label">Your node</span></div>
${body}
              </div>`;

const sparkCard = (label, seed, caption, aria) => `
              <div class="cbx-card">
                <span class="cbx-label">${label}</span>
                <svg class="cbx-sparkline" viewBox="0 0 320 44" preserveAspectRatio="none" role="img" aria-label="${aria}">
                  <line class="cbx-sparkline__grid" x1="0" y1="43.5" x2="320" y2="43.5" />
                  <polyline class="cbx-sparkline__line" points="${sparkPoints(seed)}" />
                </svg>
                <div class="cbx-sparkline__caption">${caption}</div>
              </div>`;

/* ================================================================ dashboard */

const dashboardReady = shell({
  current: 'panoramica',
  head: topbar(conn.live),
  main: `${pageHead(
    'Overview',
    'How much the network used you today, who is using you right now, and how your node is doing.',
    '<button type="button" class="cbx-btn cbx-btn--sm">Pause node</button>'
  )}
            <div class="cbx-grid cbx-grid--3">
${stat('Credited today', `128.40${UNIT}`, '24.00 of it for proven presence alone', { accent: true })}
${stat('Using you right now', '3', '2 storage sessions · 1 compute')}
${stat('Proofs passed (24 h)', `96<span class="cbx-unit">%</span>`, '142 of 148 challenges · your node is healthy')}
            </div>
            <div class="cbx-grid cbx-grid--sidebar">
              <div class="cbx-card">
                <div class="cbx-card__head">
                  <span class="cbx-label">Who is using your node right now</span>
                  <a href="#" class="cbx-hint">See all activity</a>
                </div>
${sessionsTable(
  [
    sessionRow(
      'app:photo-archive',
      'Storage · 1.2 GB',
      '42 min',
      '<span class="cbx-badge cbx-badge--online"><span class="cbx-dot cbx-dot--live" aria-hidden="true"></span>Active</span>',
      `+18.20 / h`
    ),
    sessionRow(
      'app:indexer',
      'Compute · 2 cores',
      '6 min',
      '<span class="cbx-badge cbx-badge--online"><span class="cbx-dot cbx-dot--live" aria-hidden="true"></span>Active</span>',
      `+7.85 / h`
    ),
    sessionRow(
      'app:open-maps',
      'Availability',
      '3 h 12 min',
      '<span class="cbx-badge cbx-badge--validating"><span class="cbx-dot cbx-dot--ring" aria-hidden="true"></span>Verifying</span>',
      `+1.10 / h`
    )
  ].join('\n')
)}
              </div>
              <div class="cbx-stack">
${nodeCard(`
                <dl class="cbx-kv">
                  <dt>Name</dt><dd>Loft laptop</dd>
                  <dt>Status</dt><dd><span class="cbx-badge cbx-badge--online"><span class="cbx-dot cbx-dot--live" aria-hidden="true"></span>Online</span></dd>
                  <dt>On network</dt><dd class="cbx-mono">4 d 06 h 12 min</dd>
                  <dt>Offering</dt><dd>Availability, storage, compute</dd>
                  <dt>Identifier</dt><dd class="cbx-id">cbx1q9f0…7ka2</dd>
                </dl>`)}
${sparkCard(
  'Activity — last 6 hours',
  11,
  '<span>−6 h</span><span>min 15 · max 22 challenges / 10 min</span><span>now</span>',
  'Activity over the last six hours: steady, between 15 and 22 challenges every ten minutes.'
)}
              </div>
            </div>
            <div class="cbx-card">
              <div class="cbx-card__head"><span class="cbx-label">Latest events</span></div>
${logLines([
  ['14:32:07', 'mint', '+12.40 · storage served to app:photo-archive (1.2 GB read)'],
  ['14:31:58', 'proof', 'retrievability challenge passed · block 0x8f21 · 340 ms'],
  ['14:28:51', 'burn', '−30.00 · monthly subscription to app:open-maps'],
  ['14:15:00', 'mint', '+2.00 · presence income · window 13:15–14:15'],
  ['14:14:12', 'proof', 'signed ping · nonce 0x3ac9 · replied in 88 ms']
])}
            </div>`
});

const dashboardEmpty = shell({
  current: 'panoramica',
  head: topbar(conn.live),
  main: `${pageHead(
    'Overview',
    'Your node joined the network 8 minutes ago.',
    '<button type="button" class="cbx-btn cbx-btn--sm">Pause node</button>'
  )}
            <div class="cbx-notice" role="note">
              <span class="cbx-notice__icon" aria-hidden="true">i</span>
              <span>Your node is already receiving presence income for proving it is here. Real work arrives once the network has seen that you stay reachable, usually within the first few hours.</span>
            </div>
            <div class="cbx-grid cbx-grid--3">
${stat('Credited today', `2.00${UNIT}`, 'all of it for proven presence alone', { accent: true })}
${stat('Using you right now', '0', 'no sessions yet — normal at the start')}
${stat('Proofs passed (24 h)', `100<span class="cbx-unit">%</span>`, '4 of 4 challenges · too few to judge yet')}
            </div>
            <div class="cbx-grid cbx-grid--sidebar">
              <div class="cbx-card">
                <div class="cbx-card__head"><span class="cbx-label">Who is using your node right now</span></div>
                <div class="cbx-empty">
                  <span class="cbx-empty__mark" aria-hidden="true">[ ]</span>
                  <span class="cbx-empty__title">Nobody is using your node yet</span>
                  <p class="cbx-empty__body">The network gives work to nodes it has seen answer reliably. Yours has passed 4 challenges out of 4. There is nothing to do: just leave it running.</p>
                  <button type="button" class="cbx-btn">How crediting works</button>
                </div>
              </div>
              <div class="cbx-stack">
${nodeCard(`
                <dl class="cbx-kv">
                  <dt>Name</dt><dd>Loft laptop</dd>
                  <dt>Status</dt><dd><span class="cbx-badge cbx-badge--online"><span class="cbx-dot cbx-dot--live" aria-hidden="true"></span>Online</span></dd>
                  <dt>On network</dt><dd class="cbx-mono">8 min</dd>
                  <dt>Offering</dt><dd>Availability, storage</dd>
                  <dt>Identifier</dt><dd class="cbx-id">cbx1q9f0…7ka2</dd>
                </dl>`)}
                <div class="cbx-card">
                  <span class="cbx-label">Activity — last 6 hours</span>
                  <p class="cbx-hint">There is not enough history to draw a trend yet. The chart appears after the first hour.</p>
                </div>
              </div>
            </div>
            <div class="cbx-card">
              <div class="cbx-card__head"><span class="cbx-label">Latest events</span></div>
${logLines([
  ['14:22:40', 'proof', 'signed ping · nonce 0x77c1 · replied in 91 ms'],
  ['14:19:05', 'proof', 'signed ping · nonce 0x2b8e · replied in 84 ms'],
  ['14:15:00', 'mint', '+2.00 · presence income · first window'],
  ['14:14:31', 'proof', 'node registered · identity verified']
])}
            </div>`
});

const dashboardLoading = shell({
  current: 'panoramica',
  head: topbar(conn.connecting),
  main: `${pageHead('Overview', 'Reading the most recent state from the network.')}
            <div class="cbx-grid cbx-grid--3">
${skeletonStat('Credited today')}
${skeletonStat('Using you right now')}
${skeletonStat('Proofs passed (24 h)')}
            </div>
            <div class="cbx-grid cbx-grid--sidebar">
              <div class="cbx-card">
                <div class="cbx-card__head"><span class="cbx-label">Who is using your node right now</span></div>
                <div class="cbx-stack cbx-stack--tight" role="status" aria-label="Loading current sessions">
                  <div class="cbx-skeleton cbx-skeleton--text"></div>
                  <div class="cbx-skeleton cbx-skeleton--text" style="width: 92%"></div>
                  <div class="cbx-skeleton cbx-skeleton--text" style="width: 84%"></div>
                  <div class="cbx-skeleton cbx-skeleton--text" style="width: 88%"></div>
                </div>
              </div>
              <div class="cbx-stack">
                <div class="cbx-card">
                  <span class="cbx-label">Your node</span>
                  <div class="cbx-stack cbx-stack--tight">
                    <div class="cbx-skeleton cbx-skeleton--text" style="width: 70%"></div>
                    <div class="cbx-skeleton cbx-skeleton--text" style="width: 55%"></div>
                    <div class="cbx-skeleton cbx-skeleton--text" style="width: 64%"></div>
                  </div>
                </div>
                <div class="cbx-card">
                  <span class="cbx-label">Activity — last 6 hours</span>
                  <div class="cbx-skeleton cbx-skeleton--block"></div>
                </div>
              </div>
            </div>
            <div class="cbx-card">
              <div class="cbx-card__head"><span class="cbx-label">Latest events</span></div>
              <div class="cbx-stack cbx-stack--tight">
                <div class="cbx-skeleton cbx-skeleton--text"></div>
                <div class="cbx-skeleton cbx-skeleton--text" style="width: 90%"></div>
                <div class="cbx-skeleton cbx-skeleton--text" style="width: 95%"></div>
              </div>
            </div>`
});

const dashboardError = shell({
  current: 'panoramica',
  head: topbar(conn.error, '<button type="button" class="cbx-btn cbx-btn--sm">Retry</button>'),
  main: `${pageHead('Overview', 'Cannot read the network ledger.')}
            <div class="cbx-notice cbx-notice--danger" role="alert">
              <span class="cbx-notice__icon" aria-hidden="true">×</span>
              <span class="cbx-stack cbx-stack--tight">
                <strong>No response from the validators (5 attempts, last at 14:36).</strong>
                <span>Your node <strong>keeps working and keeps being credited</strong>. It is only this window that cannot read the totals. Nothing you have earned has been lost.</span>
              </span>
            </div>
            <div class="cbx-grid cbx-grid--3">
              <div class="cbx-card">
                <div class="cbx-stat"><span class="cbx-label">Credited today</span>${unknown('cannot be read right now')}</div>
              </div>
              <div class="cbx-card">
                <div class="cbx-stat"><span class="cbx-label">Using you right now</span>${unknown('cannot be read right now')}</div>
              </div>
              <div class="cbx-card">
                <div class="cbx-stat"><span class="cbx-label">Proofs passed (24 h)</span>${unknown('cannot be read right now')}</div>
              </div>
            </div>
            <div class="cbx-grid cbx-grid--sidebar">
              <div class="cbx-card">
                <div class="cbx-card__head"><span class="cbx-label">Who is using your node right now</span></div>
                <div class="cbx-empty">
                  <span class="cbx-empty__mark" aria-hidden="true">× ×</span>
                  <span class="cbx-empty__title">Could not read this</span>
                  <p class="cbx-empty__body">Retrying on its own every 30 seconds. If you want to see what is happening, the technical log below keeps updating from the node on this computer.</p>
                  <div class="cbx-row">
                    <button type="button" class="cbx-btn cbx-btn--primary">Retry now</button>
                    <button type="button" class="cbx-btn">Open technical log</button>
                  </div>
                </div>
              </div>
              <div class="cbx-stack">
${nodeCard(`
                <dl class="cbx-kv">
                  <dt>Name</dt><dd>Loft laptop</dd>
                  <dt>Local status</dt><dd><span class="cbx-badge cbx-badge--online"><span class="cbx-dot cbx-dot--live" aria-hidden="true"></span>Running</span></dd>
                  <dt>Ledger</dt><dd><span class="cbx-badge cbx-badge--error"><span class="cbx-dot" aria-hidden="true"></span>Unreachable</span></dd>
                  <dt>Identifier</dt><dd class="cbx-id">cbx1q9f0…7ka2</dd>
                </dl>`)}
              </div>
            </div>
            <div class="cbx-card">
              <div class="cbx-card__head"><span class="cbx-label">Local node log</span></div>
${logLines([
  ['14:36:02', 'proof', 'error: no response from validator v-07 (5 s timeout)'],
  ['14:35:31', 'proof', 'error: no response from validator v-02 (5 s timeout)'],
  ['14:34:58', 'proof', 'local challenge passed · awaiting ledger confirmation'],
  ['14:33:10', 'mint', 'pending: +12.40 not yet confirmed by the ledger']
])}
            </div>`
});

const dashboardOffline = shell({
  current: 'panoramica',
  head: topbar(conn.offline, '<button type="button" class="cbx-btn cbx-btn--sm">Reconnect</button>'),
  main: `${pageHead('Overview', 'This computer is not connected to the internet.')}
            <div class="cbx-stale-rule">
              <span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>
              <span><strong>You are seeing the last known data, from 14:32 (26 minutes ago).</strong> No figure on this page is updating.</span>
            </div>
            <div class="cbx-notice cbx-notice--warning" role="note">
              <span class="cbx-notice__icon" aria-hidden="true">!</span>
              <span>While the node is offline it <strong>cannot prove that it is present</strong>, so presence income is paused. It resumes on its own as soon as the connection is back. Nothing already credited is lost.</span>
            </div>
            <div class="cbx-grid cbx-grid--3">
${stat('Credited today <span class="cbx-badge cbx-badge--offline">at 14:32</span>', `128.40${UNIT}`, 'frozen: last value confirmed by the ledger')}
${stat('Was using you <span class="cbx-badge cbx-badge--offline">at 14:32</span>', '3', 'these sessions ended when you went offline')}
${stat('Proofs passed (24 h) <span class="cbx-badge cbx-badge--offline">at 14:32</span>', `96<span class="cbx-unit">%</span>`, '142 of 148 challenges')}
            </div>
            <div class="cbx-grid cbx-grid--sidebar">
              <div class="cbx-card">
                <div class="cbx-card__head"><span class="cbx-label">Who was using your node</span></div>
${sessionsTable(
  [
    sessionRow(
      'app:photo-archive',
      'Storage · 1.2 GB',
      '42 min',
      '<span class="cbx-badge cbx-badge--offline"><span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>Ended</span>',
      '—'
    ),
    sessionRow(
      'app:indexer',
      'Compute · 2 cores',
      '6 min',
      '<span class="cbx-badge cbx-badge--offline"><span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>Ended</span>',
      '—'
    ),
    sessionRow(
      'app:open-maps',
      'Availability',
      '3 h 12 min',
      '<span class="cbx-badge cbx-badge--offline"><span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>Ended</span>',
      '—'
    )
  ].join('\n')
)}
              </div>
              <div class="cbx-stack">
${nodeCard(`
                <dl class="cbx-kv">
                  <dt>Name</dt><dd>Loft laptop</dd>
                  <dt>Status</dt><dd><span class="cbx-badge cbx-badge--offline"><span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>Offline</span></dd>
                  <dt>Last contact</dt><dd class="cbx-mono">14:32:07</dd>
                  <dt>Identifier</dt><dd class="cbx-id">cbx1q9f0…7ka2</dd>
                </dl>
                <button type="button" class="cbx-btn cbx-btn--primary">Try to reconnect</button>`)}
              </div>
            </div>
            <div class="cbx-card">
              <div class="cbx-card__head"><span class="cbx-label">Last events received</span></div>
${logLines([
  ['14:32:07', 'mint', '+12.40 · storage served to app:photo-archive'],
  ['14:31:58', 'proof', 'retrievability challenge passed · block 0x8f21'],
  ['14:32:44', 'proof', 'connection lost · waiting for network']
])}
            </div>`
});

/* ================================================================= activity */

/* `scope` keeps the control ids unique: the five artboards live in ONE document,
   and a repeated id silently breaks every label[for] after the first. */
const filters = (scope, disabled = false) => `
            <div class="cbx-row cbx-row--wrap">
              <div class="cbx-field" style="max-width: 200px">
                <label class="cbx-label" for="${scope}-period">Period</label>
                <select class="cbx-input" id="${scope}-period"${disabled ? ' disabled' : ''}>
                  <option>Today</option><option>Last 7 days</option><option>Last 30 days</option>
                </select>
              </div>
              <div class="cbx-field" style="max-width: 200px">
                <label class="cbx-label" for="${scope}-kind">Movement type</label>
                <select class="cbx-input" id="${scope}-kind"${disabled ? ' disabled' : ''}>
                  <option>All</option><option>Minted only</option><option>Burned only</option>
                </select>
              </div>
            </div>`;

const ledgerTable = (rows) => `
              <div class="cbx-table__wrap">
                <table class="cbx-table cbx-table--dense">
                  <caption class="cbx-sr-only">Token movements in the selected period</caption>
                  <thead>
                    <tr>
                      <th scope="col">Time</th>
                      <th scope="col">Reason</th>
                      <th scope="col">Counterparty</th>
                      <th scope="col">Direction</th>
                      <th scope="col" class="cbx-cell--num">Amount</th>
                    </tr>
                  </thead>
                  <tbody>
${rows
  .map(
    ([time, why, who, dir, amount]) => `                    <tr>
                      <td class="cbx-mono">${time}</td>
                      <td>${why}</td>
                      <td class="cbx-id">${who}</td>
                      <td><span class="cbx-badge cbx-badge--${dir === 'mint' ? 'mint">Minted' : 'burn">Burned'}</span></td>
                      <td class="cbx-cell--num cbx-num--${dir}">${amount}</td>
                    </tr>`
  )
  .join('\n')}
                  </tbody>
                </table>
              </div>`;

const activityReady = shell({
  current: 'attivita',
  head: topbar(conn.live),
  main: `${pageHead(
    'Activity',
    'Who used your node, what it returned, and where the tokens you spent went.'
  )}
${filters('att-nom')}
            <div class="cbx-grid cbx-grid--3">
${stat('Minted to you — today', `128.40${UNIT}`, '31 credits · 24.00 for presence', { accent: true })}
${stat('Burned by you — today', `30.00${UNIT}`, '1 subscription to a service')}
              <div class="cbx-card">
                <div class="cbx-meter">
                  <span class="cbx-label">Movement split</span>
                  <div class="cbx-meter__track cbx-meter__track--split" role="img" aria-label="Today: 128.40 minted to you, 30.00 burned by you.">
                    <div class="cbx-meter__segment cbx-meter__segment--mint" style="width: 81%"></div>
                    <div class="cbx-meter__segment cbx-meter__segment--burn" style="width: 19%"></div>
                  </div>
                  <div class="cbx-meter__legend">
                    <span><span class="cbx-badge cbx-badge--mint">Minted</span> <span class="cbx-num cbx-num--mint">128.40</span></span>
                    <span><span class="cbx-badge cbx-badge--burn">Burned</span> <span class="cbx-num cbx-num--burn">30.00</span></span>
                  </div>
                  <p class="cbx-hint">What you spend is destroyed; what you receive is created by the protocol. This is not a profit-and-loss balance.</p>
                </div>
              </div>
            </div>
            <div class="cbx-card">
              <div class="cbx-card__head"><span class="cbx-label">Sessions in progress</span></div>
${sessionsTable(
  [
    sessionRow(
      'app:photo-archive',
      'Storage · 1.2 GB · 340 reads',
      '42 min',
      '<span class="cbx-badge cbx-badge--online"><span class="cbx-dot cbx-dot--live" aria-hidden="true"></span>Active</span>',
      '+18.20 / h'
    ),
    sessionRow(
      'app:indexer',
      'Compute · 2 cores · 88 tasks',
      '6 min',
      '<span class="cbx-badge cbx-badge--online"><span class="cbx-dot cbx-dot--live" aria-hidden="true"></span>Active</span>',
      '+7.85 / h'
    ),
    sessionRow(
      'app:open-maps',
      'Availability',
      '3 h 12 min',
      '<span class="cbx-badge cbx-badge--validating"><span class="cbx-dot cbx-dot--ring" aria-hidden="true"></span>Verifying</span>',
      '+1.10 / h'
    )
  ].join('\n')
)}
            </div>
            <div class="cbx-grid cbx-grid--sidebar">
              <div class="cbx-card">
                <div class="cbx-card__head"><span class="cbx-label">Movements — today</span><span class="cbx-hint">31 rows</span></div>
${ledgerTable([
  ['14:32:07', 'Storage served', 'app:photo-archive', 'mint', '+12.40'],
  ['14:28:51', 'Monthly subscription', 'app:open-maps', 'burn', '−30.00'],
  ['14:15:00', 'Presence income', 'protocol', 'mint', '+2.00'],
  ['14:02:33', 'Compute run', 'app:indexer', 'mint', '+7.85'],
  ['13:47:12', 'Storage served', 'app:photo-archive', 'mint', '+11.05'],
  ['13:15:00', 'Presence income', 'protocol', 'mint', '+2.00'],
  ['12:58:40', 'Availability', 'app:open-maps', 'mint', '+1.10']
])}
              </div>
${sparkCard(
  'Credits — last 24 hours',
  29,
  '<span>−24 h</span><span>min 1.10 · max 18.20 per hour</span><span>now</span>',
  'Credits over the last twenty-four hours: between 1.10 and 18.20 tokens per hour, peaking in the afternoon.'
)}
            </div>`
});

const activityEmpty = shell({
  current: 'attivita',
  head: topbar(conn.live),
  main: `${pageHead('Activity', 'Who used your node and what it returned.')}
${filters('att-empty')}
            <div class="cbx-grid cbx-grid--3">
${stat('Minted to you — today', `0.00${UNIT}`, 'no credits in the selected period')}
${stat('Burned by you — today', `0.00${UNIT}`, 'nothing spent in the selected period')}
              <div class="cbx-card">
                <div class="cbx-meter">
                  <span class="cbx-label">Movement split</span>
                  <p class="cbx-hint">Nothing to compare: there are no movements in the selected period.</p>
                </div>
              </div>
            </div>
            <div class="cbx-empty">
              <span class="cbx-empty__mark" aria-hidden="true">[ ]</span>
              <span class="cbx-empty__title">No movements today</span>
              <p class="cbx-empty__body">Your node was switched off until a short while ago, so today it neither received nor spent anything. Try widening the period to see earlier days.</p>
              <div class="cbx-row">
                <button type="button" class="cbx-btn cbx-btn--primary">Look at the last 7 days</button>
                <button type="button" class="cbx-btn">How you earn on the network</button>
              </div>
            </div>`
});

const activityLoading = shell({
  current: 'attivita',
  head: topbar(conn.connecting),
  main: `${pageHead('Activity', 'Fetching movements from the ledger.')}
${filters('att-load', true)}
            <div class="cbx-grid cbx-grid--3">
${skeletonStat('Minted to you — today')}
${skeletonStat('Burned by you — today')}
              <div class="cbx-card">
                <span class="cbx-label">Movement split</span>
                <div class="cbx-skeleton" style="height: 8px"></div>
                <div class="cbx-skeleton cbx-skeleton--text" style="width: 70%"></div>
              </div>
            </div>
            <div class="cbx-card">
              <div class="cbx-card__head"><span class="cbx-label">Sessions in progress</span></div>
              <div class="cbx-stack cbx-stack--tight" role="status" aria-label="Loading sessions">
                <div class="cbx-skeleton cbx-skeleton--text"></div>
                <div class="cbx-skeleton cbx-skeleton--text" style="width: 93%"></div>
                <div class="cbx-skeleton cbx-skeleton--text" style="width: 87%"></div>
              </div>
            </div>
            <div class="cbx-grid cbx-grid--sidebar">
              <div class="cbx-card">
                <div class="cbx-card__head"><span class="cbx-label">Movements — today</span></div>
                <div class="cbx-stack cbx-stack--tight" role="status" aria-label="Loading movements">
                  ${Array.from({ length: 7 }, (_, i) => `<div class="cbx-skeleton cbx-skeleton--text" style="width: ${100 - i * 4}%"></div>`).join('\n                  ')}
                </div>
              </div>
              <div class="cbx-card">
                <span class="cbx-label">Credits — last 24 hours</span>
                <div class="cbx-skeleton cbx-skeleton--block"></div>
              </div>
            </div>`
});

const activityError = shell({
  current: 'attivita',
  head: topbar(conn.error, '<button type="button" class="cbx-btn cbx-btn--sm">Retry</button>'),
  main: `${pageHead('Activity', 'Cannot fetch the movements.')}
${filters('att-err', true)}
            <div class="cbx-notice cbx-notice--danger" role="alert">
              <span class="cbx-notice__icon" aria-hidden="true">×</span>
              <span class="cbx-stack cbx-stack--tight">
                <strong>The ledger replied with an error (request 0x91af, 14:36).</strong>
                <span>No partial figures are shown: an incomplete list of movements would be worse than no list at all. Your credits are untouched by this error.</span>
              </span>
            </div>
            <div class="cbx-empty">
              <span class="cbx-empty__mark" aria-hidden="true">× ×</span>
              <span class="cbx-empty__title">Movements cannot be fetched right now</span>
              <p class="cbx-empty__body">Retrying automatically every 30 seconds. If the error persists, you can copy the request code and attach it to a report.</p>
              <div class="cbx-row">
                <button type="button" class="cbx-btn cbx-btn--primary">Retry now</button>
                <button type="button" class="cbx-btn cbx-btn--mono">Copy 0x91af</button>
              </div>
            </div>`
});

const activityOffline = shell({
  current: 'attivita',
  head: topbar(conn.offline, '<button type="button" class="cbx-btn cbx-btn--sm">Reconnect</button>'),
  main: `${pageHead('Activity', 'This computer is not connected to the internet.')}
            <div class="cbx-stale-rule">
              <span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>
              <span><strong>Movements frozen at 14:32 (26 minutes ago).</strong> Whatever happened on the network since then has not reached this computer yet.</span>
            </div>
${filters('att-off', true)}
            <div class="cbx-grid cbx-grid--3">
${stat('Minted to you <span class="cbx-badge cbx-badge--offline">at 14:32</span>', `128.40${UNIT}`, 'last total confirmed by the ledger')}
${stat('Burned by you <span class="cbx-badge cbx-badge--offline">at 14:32</span>', `30.00${UNIT}`, 'last total confirmed by the ledger')}
              <div class="cbx-card">
                <div class="cbx-meter">
                  <span class="cbx-label">Movement split</span>
                  <div class="cbx-meter__track cbx-meter__track--split" role="img" aria-label="At 14:32: 128.40 minted to you, 30.00 burned by you.">
                    <div class="cbx-meter__segment cbx-meter__segment--mint" style="width: 81%"></div>
                    <div class="cbx-meter__segment cbx-meter__segment--burn" style="width: 19%"></div>
                  </div>
                  <div class="cbx-meter__legend"><span class="cbx-mono">at 14:32</span><span class="cbx-mono">not updating</span></div>
                </div>
              </div>
            </div>
            <div class="cbx-card">
              <div class="cbx-card__head">
                <span class="cbx-label">Movements — up to 14:32</span>
                <span class="cbx-badge cbx-badge--offline"><span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>Not updating</span>
              </div>
${ledgerTable([
  ['14:32:07', 'Storage served', 'app:photo-archive', 'mint', '+12.40'],
  ['14:28:51', 'Monthly subscription', 'app:open-maps', 'burn', '−30.00'],
  ['14:15:00', 'Presence income', 'protocol', 'mint', '+2.00'],
  ['14:02:33', 'Compute run', 'app:indexer', 'mint', '+7.85']
])}
              <p class="cbx-hint">As soon as you are back online the list realigns on its own: the missing movements are fetched from the ledger, not reconstructed locally.</p>
            </div>`
});

/* =============================================================== onboarding */

const onboarding = ({ steps, panel }) => `
        <div class="cbx-onboarding">
          <div class="cbx-onboarding__bar"><span class="cbx-app__brand-mark" aria-hidden="true">◈</span>Coblox · first run</div>
          <div class="cbx-onboarding__body">
            <div class="cbx-onboarding__panel">
              <ol class="cbx-steps">
${['Create your identity', 'Write down your recovery phrase', 'Choose what to offer']
  .map((label, i) => {
    const state = steps[i];
    const suffix = state === 'done' ? ' · done' : state === 'current' ? ' · in progress' : '';
    return `                <li class="cbx-steps__item" data-state="${state}"${state === 'current' ? ' aria-current="step"' : ''}>
                  <span class="cbx-steps__marker">Step ${i + 1} of 3${suffix}</span>
                  <span class="cbx-steps__label">${label}</span>
                </li>`;
  })
  .join('\n')}
              </ol>
${panel}
            </div>
          </div>
        </div>`;

const onboardingEmpty = onboarding({
  steps: ['current', 'todo', 'todo'],
  panel: `
              <div class="cbx-stack">
                <h2 class="cbx-title">Create your identity</h2>
                <p class="cbx-prose">
                  Coblox keeps no register of accounts. Your identity is a key that is created here and stays on
                  this computer. Nobody, ourselves included, can reset it for you — which is why the next step
                  asks you to write down a recovery phrase.
                </p>
                <div class="cbx-field">
                  <label class="cbx-label" for="ob-name">What do you want to call this device?</label>
                  <input class="cbx-input" id="ob-name" type="text" placeholder="For example: loft laptop" />
                  <span class="cbx-hint">This is only so you can recognise it. The network never sees this name.</span>
                </div>
                <div class="cbx-notice" role="note">
                  <span class="cbx-notice__icon" aria-hidden="true">i</span>
                  <span>Your identity is created on this computer and needs no internet connection. You will not be asked for an email address or any personal detail.</span>
                </div>
                <div class="cbx-onboarding__foot">
                  <button type="button" class="cbx-btn cbx-btn--primary">Create identity</button>
                  <button type="button" class="cbx-btn cbx-btn--ghost">What does this mean?</button>
                </div>
              </div>`
});

const onboardingLoading = onboarding({
  steps: ['current', 'todo', 'todo'],
  panel: `
              <div class="cbx-stack">
                <h2 class="cbx-title">Creating your keys</h2>
                <p class="cbx-prose">This takes a few seconds. Please do not close the window: if you stop now, you will have to start again from the beginning.</p>
                <div class="cbx-card">
                  <div class="cbx-stack cbx-stack--tight" role="status" aria-live="polite" aria-label="Creating your identity">
                    <span class="cbx-label">Step in progress</span>
                    <div class="cbx-skeleton cbx-skeleton--text" style="width: 55%"></div>
                    <div class="cbx-skeleton cbx-skeleton--text" style="width: 78%"></div>
                    <div class="cbx-skeleton cbx-skeleton--text" style="width: 40%"></div>
                  </div>
                </div>
                <div class="cbx-onboarding__foot">
                  <button type="button" class="cbx-btn cbx-btn--primary" disabled>Creating…</button>
                  <span class="cbx-hint">Generating the key pair on this device.</span>
                </div>
              </div>`
});

const onboardingReady = onboarding({
  steps: ['done', 'current', 'todo'],
  panel: `
              <div class="cbx-stack">
                <h2 class="cbx-title">Write down your recovery phrase</h2>
                <p class="cbx-prose">
                  These twelve words <strong>are</strong> your identity. Write them by hand on paper, in the order
                  shown, and keep them away from this computer. If you lose the device, they are the only way back
                  to what you have built up.
                </p>
                <ol class="cbx-wordgrid">
${['anchor', 'birch', 'canyon', 'dolomite', 'ember', 'fresco', 'granite', 'harbour', 'lantern', 'melody', 'nocturne', 'umbrella']
  .map(
    (w, i) =>
      `                  <li class="cbx-wordgrid__item"><span class="cbx-wordgrid__index">${i + 1}</span>${w}</li>`
  )
  .join('\n')}
                </ol>
                <div class="cbx-notice cbx-notice--warning" role="note">
                  <span class="cbx-notice__icon" aria-hidden="true">!</span>
                  <span>Do not photograph them and do not save them in a file: anyone who reads them can use your identity. Nobody from Coblox will ever ask you for them.</span>
                </div>
                <label class="cbx-check">
                  <input type="checkbox" />
                  <span>I have written them on paper and put them somewhere safe. I understand there is no other way to recover them.</span>
                </label>
                <div class="cbx-onboarding__foot">
                  <button type="button" class="cbx-btn cbx-btn--primary">Continue</button>
                  <button type="button" class="cbx-btn">Show them again later</button>
                </div>
              </div>`
});

const onboardingError = onboarding({
  steps: ['done', 'done', 'current'],
  panel: `
              <div class="cbx-stack">
                <h2 class="cbx-title">Choose what to offer</h2>
                <p class="cbx-prose">You can change any of this later. Offering more does not expose you more: the network never sees what is on your disk.</p>
                <div class="cbx-card">
                  <div class="cbx-stack">
                    <div class="cbx-field">
                      <label class="cbx-label" for="ob-folder">Folder for network data</label>
                      <input class="cbx-input cbx-input--data" id="ob-folder" type="text" value="D:\\Coblox\\storage" aria-invalid="true" aria-describedby="ob-folder-err" />
                      <span class="cbx-field__error" id="ob-folder-err">
                        <span class="cbx-notice__icon" aria-hidden="true">!</span>
                        There is no permission to write in this folder.
                      </span>
                    </div>
                    <div class="cbx-notice cbx-notice--danger" role="alert">
                      <span class="cbx-notice__icon" aria-hidden="true">×</span>
                      <span class="cbx-stack cbx-stack--tight">
                        <strong>D:\\Coblox\\storage cannot be used.</strong>
                        <span>The folder exists but is read-only. Pick a different one, or allow writing and try again. You can also carry on now by offering availability only: presence income is still credited.</span>
                      </span>
                    </div>
                    <div class="cbx-row cbx-row--wrap">
                      <button type="button" class="cbx-btn">Choose another folder</button>
                      <button type="button" class="cbx-btn">Try again</button>
                    </div>
                  </div>
                </div>
                <div class="cbx-onboarding__foot">
                  <button type="button" class="cbx-btn cbx-btn--primary" disabled>Join the network</button>
                  <button type="button" class="cbx-btn cbx-btn--ghost">Offer availability only</button>
                </div>
              </div>`
});

const onboardingOffline = onboarding({
  steps: ['done', 'done', 'current'],
  panel: `
              <div class="cbx-stack">
                <h2 class="cbx-title">Choose what to offer</h2>
                <div class="cbx-stale-rule">
                  <span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>
                  <span><strong>This computer is not connected to the internet.</strong> Your identity is already created and safe: only joining the network is left.</span>
                </div>
                <p class="cbx-prose">
                  You can finish setting up now. Your choices are saved on this computer and the node joins the
                  network by itself as soon as the connection is back. You will not have to redo any of this.
                </p>
                <div class="cbx-card">
                  <dl class="cbx-kv">
                    <dt>Identity</dt><dd><span class="cbx-badge cbx-badge--online">Created and saved</span></dd>
                    <dt>Phrase</dt><dd><span class="cbx-badge cbx-badge--online">Confirmed</span></dd>
                    <dt>Joining</dt><dd><span class="cbx-badge cbx-badge--offline"><span class="cbx-dot cbx-dot--hollow" aria-hidden="true"></span>Waiting for network</span></dd>
                  </dl>
                </div>
                <div class="cbx-notice" role="note">
                  <span class="cbx-notice__icon" aria-hidden="true">i</span>
                  <span>Until the node joins the network it cannot prove that it is present, so nothing is being credited yet.</span>
                </div>
                <div class="cbx-onboarding__foot">
                  <button type="button" class="cbx-btn cbx-btn--primary">Save and wait for the connection</button>
                  <button type="button" class="cbx-btn">Try to reconnect</button>
                </div>
              </div>`
});

/* ==================================================================== pages */

function page(title, intro, artboards) {
  const toc = artboards
    .map((a) => `<a href="#${a.id}">${a.state}</a>`)
    .join('\n          ');
  return `<!doctype html>
<html lang="it" data-theme="dark">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Coblox — ${title}</title>
    <link rel="stylesheet" href="../tokens/tokens.css" />
    <link rel="stylesheet" href="../css/base.css" />
    <link rel="stylesheet" href="../css/components.css" />
    <link rel="stylesheet" href="../css/app-shell.css" />
    <!--
      GENERATED by design/tools/build-mockups.mjs — do not edit by hand.
      Framework-free and script-free: open directly in a browser.
    -->
  </head>
  <body>
    <div class="mock-page">
      <header class="cbx-stack">
        <p class="cbx-label">Coblox · mockup · ${tokens.$meta.spec}</p>
        <h1 class="cbx-title" style="font-size: var(--cbx-font-size-3xl)">${title}</h1>
        <p class="cbx-prose">${intro}</p>
        <div class="cbx-notice" role="note">
          <span class="cbx-notice__icon" aria-hidden="true">EN</span>
          <span>
            <strong>Lingua dell'interfaccia: inglese.</strong> Tutto il testo <em>dentro</em> gli artboard è in
            inglese, perché è la lingua del prodotto. Le annotazioni <em>attorno</em> agli artboard sono note di
            lavoro interne per il team e restano in italiano.
          </span>
        </div>
        <nav class="mock-toc" aria-label="Stati di questa schermata">
          ${toc}
        </nav>
      </header>
${artboards
  .map(
    (a) => `      <section class="mock-artboard" id="${a.id}">
        <div class="mock-artboard__head">
          <span class="mock-artboard__id">${a.state}</span>
          <span class="cbx-section-title">${a.title}</span>
        </div>
        <p class="mock-artboard__note">${a.note}</p>
${a.html}
      </section>`
  )
  .join('\n')}
      <footer>
        <p class="cbx-hint">Token: <code class="cbx-mono">design/tokens/</code> · Componenti: <code class="cbx-mono">design/preview/index.html</code> · Principi: <code class="cbx-mono">design/PRINCIPLES.md</code></p>
      </footer>
    </div>
  </body>
</html>
`;
}

const files = {
  'dashboard.html': page(
    'Dashboard del nodo',
    "La schermata che risponde a tre domande, in quest'ordine: quanto mi ha accreditato la rete oggi, chi mi sta usando adesso, il mio nodo è in salute.",
    [
      {
        id: 'stato-nominale',
        state: 'Nominale',
        title: 'Nodo attivo, rete in diretta',
        note: "Le tre domande hanno tre risposte in cima alla pagina. Il totale del giorno è l'elemento più grande, e dichiara subito quanta parte viene dalla sola presenza: è la promessa del prodotto, non un dettaglio contabile.",
        html: dashboardReady
      },
      {
        id: 'stato-vuoto',
        state: 'Vuoto',
        title: 'Nodo appena entrato in rete',
        note: "Vuoto di sessioni, non vuoto di accrediti: il reddito di esistenza (<em>presence income</em>) sta già maturando. Lo stato vuoto spiega perché è vuoto e che non c'è nulla da fare, evitando che l'utente pensi di aver sbagliato la configurazione.",
        html: dashboardEmpty
      },
      {
        id: 'stato-caricamento',
        state: 'Caricamento',
        title: 'Prima lettura dello stato',
        note: 'Gli scheletri hanno la forma esatta del contenuto atteso, così il layout non salta. Nessuno spinner al centro dello schermo: la struttura della pagina è già informazione.',
        html: dashboardLoading
      },
      {
        id: 'stato-errore',
        state: 'Errore',
        title: 'Registro della rete irraggiungibile',
        note: "L'errore distingue ciò che è rotto (la lettura) da ciò che funziona (il nodo continua a lavorare e a essere accreditato). I valori non leggibili sono un trattino con spiegazione, mai uno zero e mai un numero vecchio spacciato per attuale.",
        html: dashboardError
      },
      {
        id: 'stato-offline',
        state: 'Offline',
        title: 'Nessuna connessione',
        note: "Ogni cifra porta l'etichetta dell'ora a cui si riferisce, e la pagina dichiara la conseguenza economica reale: senza presenza dimostrata il reddito di esistenza è sospeso ([ADR-002]).",
        html: dashboardOffline
      }
    ]
  ),
  'attivita.html': page(
    'Dettaglio attività',
    'Chi sta usando il nodo, quanto frutta ogni sessione, e lo storico dei movimenti emessi e bruciati.',
    [
      {
        id: 'stato-nominale',
        state: 'Nominale',
        title: 'Sessioni in corso e movimenti del giorno',
        note: "Emesso e bruciato sono due grandezze affiancate, non un utile netto: la barra di proporzione ha sempre una legenda scritta e una nota che ricorda il modello mint & burn ([ADR-005]). Nessun grafico di prezzo, nessuna freccia di performance.",
        html: activityReady
      },
      {
        id: 'stato-vuoto',
        state: 'Vuoto',
        title: 'Nessun movimento nel periodo scelto',
        note: 'Zero è un fatto e viene mostrato come zero (<em>0.00</em>), con la causa (il nodo è stato spento) e la via d’uscita più probabile (allargare il periodo).',
        html: activityEmpty
      },
      {
        id: 'stato-caricamento',
        state: 'Caricamento',
        title: 'Recupero dei movimenti',
        note: 'I filtri restano visibili ma disabilitati: la loro posizione non cambia quando i dati arrivano, così il puntatore non insegue i controlli.',
        html: activityLoading
      },
      {
        id: 'stato-errore',
        state: 'Errore',
        title: 'Il registro risponde con un errore',
        note: "Su un elenco contabile un risultato parziale è peggio di nessun risultato: qui si sceglie di non mostrare righe e di dare all'utente il codice della richiesta da allegare a una segnalazione.",
        html: activityError
      },
      {
        id: 'stato-offline',
        state: 'Offline',
        title: 'Elenco fermo all’ultimo dato ricevuto',
        note: "L'elenco resta consultabile ma è marcato come non aggiornato in tre punti: banda superiore, badge sulla card e legenda della barra. Dichiara anche come si riallineerà (dal registro, non ricostruendo in locale).",
        html: activityOffline
      }
    ]
  ),
  'onboarding.html': page(
    'Onboarding in tre passi',
    "Creazione dell'identità per una persona che non sa cosa sia una chiave crittografica. Niente gergo non spiegato, niente scelte irreversibili senza un avviso esplicito.",
    [
      {
        id: 'stato-nominale',
        state: 'Nominale',
        title: 'Passo 2 — frase di recupero',
        note: "Il passo più delicato del prodotto. Le parole sono numerate e monospace per la trascrizione a mano; la conferma è una spunta esplicita che nomina la conseguenza, non un semplice “Continue”.",
        html: onboardingReady
      },
      {
        id: 'stato-vuoto',
        state: 'Vuoto',
        title: 'Passo 1 — nulla inserito',
        note: "Primo avvio senza navigazione: c'è una cosa sola da fare. Il testo dichiara subito che non servono email né dati personali e che nessuno può reimpostare l'identità al posto tuo.",
        html: onboardingEmpty
      },
      {
        id: 'stato-caricamento',
        state: 'Caricamento',
        title: 'Passo 1 — generazione delle chiavi',
        note: "Un'attesa breve ma non interrompibile: il bottone è disabilitato e il testo dice che cosa succede se si chiude la finestra. La regione è annunciata con aria-live.",
        html: onboardingLoading
      },
      {
        id: 'stato-errore',
        state: 'Errore',
        title: 'Passo 3 — cartella non scrivibile',
        note: "L'errore dice qual è il problema, dove si trova, e offre due vie d'uscita reali — fra cui proseguire offrendo solo la disponibilità, che è comunque un contributo accreditato. Nessun vicolo cieco.",
        html: onboardingError
      },
      {
        id: 'stato-offline',
        state: 'Offline',
        title: 'Passo 3 — senza connessione',
        note: "L'identità si crea offline: la schermata lo dice e permette di finire la configurazione, rimandando il solo ingresso in rete. Dichiara però che senza rete non maturano accrediti.",
        html: onboardingOffline
      }
    ]
  ),
  'index.html': `<!doctype html>
<html lang="it" data-theme="dark">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Coblox — Mockup delle schermate chiave</title>
    <link rel="stylesheet" href="../tokens/tokens.css" />
    <link rel="stylesheet" href="../css/base.css" />
    <link rel="stylesheet" href="../css/components.css" />
    <link rel="stylesheet" href="../css/app-shell.css" />
    <!-- GENERATED by design/tools/build-mockups.mjs — do not edit by hand. -->
  </head>
  <body>
    <div class="mock-page">
      <header class="cbx-stack">
        <p class="cbx-label">Coblox · mockup · ${tokens.$meta.spec}</p>
        <h1 class="cbx-title" style="font-size: var(--cbx-font-size-3xl)">Schermate chiave</h1>
        <p class="cbx-prose">
          Tre schermate del client desktop, ciascuna in cinque artboard: lo stato nominale più i quattro
          stati richiesti (vuoto, caricamento, errore, offline). Tutto è HTML e CSS statici che consumano
          i token di <code class="cbx-mono">design/tokens/</code>.
        </p>
      </header>
      <div class="cbx-grid cbx-grid--3">
        <a class="cbx-card" href="./dashboard.html" style="text-decoration: none">
          <span class="cbx-label">Schermata 1</span>
          <span class="cbx-section-title">Dashboard del nodo</span>
          <span class="cbx-hint">Quanto ho ricevuto oggi, chi mi sta usando ora, il nodo è in salute.</span>
        </a>
        <a class="cbx-card" href="./attivita.html" style="text-decoration: none">
          <span class="cbx-label">Schermata 2</span>
          <span class="cbx-section-title">Dettaglio attività</span>
          <span class="cbx-hint">Sessioni in corso e storico dei movimenti emessi e bruciati.</span>
        </a>
        <a class="cbx-card" href="./onboarding.html" style="text-decoration: none">
          <span class="cbx-label">Schermata 3</span>
          <span class="cbx-section-title">Onboarding in tre passi</span>
          <span class="cbx-hint">Creazione dell'identità per chi non è tecnico.</span>
        </a>
      </div>
      <footer>
        <p class="cbx-hint">Componenti: <code class="cbx-mono">../preview/index.html</code> · Principi: <code class="cbx-mono">../PRINCIPLES.md</code> · Contrasto: <code class="cbx-mono">../tokens/CONTRAST.md</code></p>
      </footer>
    </div>
  </body>
</html>
`
};

const check = process.argv.includes('--check');
let drift = 0;
for (const [name, content] of Object.entries(files)) {
  const path = resolve(DIR, name);
  if (check) {
    let current = '';
    try {
      current = readFileSync(path, 'utf8');
    } catch {
      /* missing counts as drift */
    }
    if (current !== content) {
      console.error(`DRIFT: design/mockups/${name} is stale.`);
      drift += 1;
    }
  } else {
    writeFileSync(path, content, 'utf8');
    console.log(`Wrote design/mockups/${name} (${content.length} bytes).`);
  }
}
if (check) {
  if (drift) {
    console.error('Run: node design/tools/build-mockups.mjs');
    process.exit(1);
  }
  console.log(`OK: all ${Object.keys(files).length} mockup pages match their generator.`);
}
