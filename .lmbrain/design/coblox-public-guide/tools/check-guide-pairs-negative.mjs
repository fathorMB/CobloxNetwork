#!/usr/bin/env node
/**
 * La prova in negativo di `check-guide-pairs.mjs`, e in particolare di G6.
 *
 * [SKILL-001]: una misura che non si e mai vista scattare e un calcolo, non
 * una guardia. G6 e la guardia che tiene le ancore della guida attaccate alle
 * frasi che dicono, ed e cambiata il 2026-08-26 — `claims` e diventato una
 * LISTA — quindi va riprovata, e la prova esige ora che **ogni affermazione sia
 * osservata fallire da sola**. Provare che UNA fallisce non dice nulla sulle
 * altre ottantacinque: un'ancora scritta contro un testo poi riscritto continua
 * a essere letta e a passare, ha solo smesso di ancorare qualcosa.
 *
 * Procedura, per ciascuna affermazione di ciascuna probe `guide-*`:
 *
 *   1. si copia l'albero minimo in una directory temporanea;
 *   2. si CANCELLA DALLA PAGINA la frase che quella affermazione dichiara di
 *      tenere — che e esattamente il difetto che G6 esiste per impedire: la
 *      frase editata o rimossa e l'ancora lasciata indietro;
 *   3. si esegue la gate sulla copia e si pretende uscita non-zero **che nomini
 *      quella probe**;
 *   4. si ripristina e si riverifica il verde.
 *
 * Piu quattro mutazioni sulla FORMA introdotta dal cambiamento di schema, che
 * la cancellazione di una frase non esercita: lista vuota, voce vuota, campo
 * assente, e il numero del colophon disallineato dal numero delle affermazioni.
 *
 * L'albero di lavoro non viene mai modificato.
 *
 *   node .lmbrain/design/coblox-public-guide/tools/check-guide-pairs-negative.mjs
 */

import { mkdtempSync, mkdirSync, copyFileSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { tmpdir } from 'node:os';
import { fileURLToPath } from 'node:url';
import { dirname, resolve, join } from 'node:path';

const here = dirname(fileURLToPath(import.meta.url));
const PKG = resolve(here, '..');
const REPO = resolve(PKG, '../../..');

const FILES = [
  '.lmbrain/design/coblox-public-guide/index.html',
  '.lmbrain/design/coblox-public-guide/guide.css',
  '.lmbrain/design/coblox-public-guide/used-pairs.json',
  '.lmbrain/design/coblox-public-guide/tools/check-guide-pairs.mjs',
  '.lmbrain/design/coblox-design-system/tokens/tokens.css',
  '.lmbrain/design/coblox-design-system/tokens/contrast-pairs.json',
  'sim/tools/published_artifacts.toml'
];

const MANIFEST_REL = 'sim/tools/published_artifacts.toml';
const HTML_REL = '.lmbrain/design/coblox-public-guide/index.html';
const GATE_REL = '.lmbrain/design/coblox-public-guide/tools/check-guide-pairs.mjs';

function makeCopy() {
  const root = mkdtempSync(join(tmpdir(), 'coblox-guide-negative-'));
  for (const rel of FILES) {
    const target = join(root, rel);
    mkdirSync(dirname(target), { recursive: true });
    copyFileSync(join(REPO, rel), target);
  }
  return root;
}

function runGate(root) {
  return spawnSync(process.execPath, [join(root, GATE_REL)], { encoding: 'utf8' });
}

/* Il lettore minimo, tenuto identico a quello della gate: se divergessero, la
   prova coprirebbe un insieme di probe diverso da quello che la gate legge. */
function readProbeTable(toml) {
  const rows = [];
  let current = null;
  const unescape = (s) => s.replace(/\\(.)/g, (_, c) => (c === 'n' ? '\n' : c));
  for (const line of toml.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (trimmed.startsWith('[[') || /^\[[A-Za-z_]/.test(trimmed)) {
      if (current) rows.push(current);
      current = trimmed === '[[probe]]' ? {} : null;
      continue;
    }
    if (!current) continue;
    const arr = /^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*\[(.*)\]\s*$/.exec(trimmed);
    if (arr) {
      current[arr[1]] = [...arr[2].matchAll(/"((?:[^"\\]|\\.)*)"/g)].map((m) => unescape(m[1]));
      continue;
    }
    const m = /^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"((?:[^"\\]|\\.)*)"\s*$/.exec(trimmed);
    if (m) current[m[1]] = m[1] === 'claims' ? [unescape(m[2])] : unescape(m[2]);
  }
  if (current) rows.push(current);
  return rows;
}

/* L'affermazione e registrata in ASCII e confrontata dalla gate contro la prosa
   NORMALIZZATA: entita risolte, virgolette raddrizzate, spazi collassati, tag
   rimossi. Per cancellarla dal sorgente serve quindi un'espressione che tolleri
   quelle tre differenze — in particolare i tag, perche una frase della guida
   attraversa i suoi <span> («V divided by T» ne contiene tre). */
const ENTITY_OF = { '&': 'amp', '<': 'lt', '>': 'gt', '—': 'mdash', '·': 'middot', '–': 'ndash', '…': 'hellip' };
const GAP = '(?:\\s|<[^>]+>|&nbsp;|&#8239;|&#160;)+';

function claimPattern(claim) {
  let out = '';
  for (const ch of claim.replace(/\s+/g, ' ')) {
    if (ch === ' ') {
      out += GAP;
      continue;
    }
    const esc = ch.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    if (ch === '"') out += '(?:"|&quot;|&ldquo;|&rdquo;|“|”)';
    else if (ch === "'") out += "(?:'|&apos;|&lsquo;|&rsquo;|‘|’)";
    else if (ENTITY_OF[ch]) out += `(?:${esc}|&${ENTITY_OF[ch]};)`;
    else out += esc;
  }
  return new RegExp(out);
}

const failures = [];

function expectFail(root, label, probeId) {
  const result = runGate(root);
  const named = result.stdout.includes(`probe ${probeId}`);
  if (result.status === 0 || !named) {
    failures.push(
      `${label}: the gate did not fail naming ${probeId} (exit=${result.status})`
    );
    return false;
  }
  return true;
}

function main() {
  const root = makeCopy();
  try {
    /* Il verde iniziale. Provare in negativo una gate rossa non dimostra nulla. */
    const control = runGate(root);
    console.log('=== control: the unmutated copy ===');
    console.log(`  exit=${control.status}  ${control.stdout.trim().split('\n').pop()}`);
    if (control.status !== 0) {
      failures.push('control run failed; a guard that rejects a correct tree proves nothing');
      report();
      return;
    }
    console.log('');

    const htmlPath = join(root, HTML_REL);
    const original = readFileSync(htmlPath, 'utf8');
    const probes = readProbeTable(readFileSync(join(root, MANIFEST_REL), 'utf8')).filter((p) =>
      p.id?.startsWith('guide-')
    );
    const claimTotal = probes.reduce((n, p) => n + (p.claims?.length ?? 0), 0);

    console.log('=== G6-CLAIM-STILL-MADE, every statement individually ===');
    console.log(
      `deleting each statement from the page it is claimed to hold, ` +
        `${claimTotal} case(s) across ${probes.length} probe(s)`
    );
    const unreachable = [];
    for (const probe of probes) {
      for (const claim of probe.claims ?? []) {
        const re = claimPattern(claim);
        if (!re.test(original)) {
          unreachable.push(
            `${probe.id}: its statement ${JSON.stringify(claim)} cannot be located in the ` +
              `page source, so deleting it cannot be used to prove the anchor`
          );
          continue;
        }
        writeFileSync(htmlPath, original.replace(re, ' '), 'utf8');
        try {
          if (!expectFail(root, 'deleting a claimed statement', probe.id)) {
            unreachable.push(
              `${probe.id}: deleting ${JSON.stringify(claim)} from the page did not make ` +
                `the gate fail naming it`
            );
          }
        } finally {
          writeFileSync(htmlPath, original, 'utf8');
        }
      }
    }
    if (unreachable.length) {
      for (const line of unreachable) console.log(`  UNREACHABLE ${line}`);
      failures.push(...unreachable);
    } else {
      console.log(`  every one of the ${claimTotal} statements was observed failing`);
    }
    console.log('');

    /* Il ripristino, riverificato: senza questo il verde iniziale resta un
       artefatto e ogni caso sopra e senza controllo. */
    const restored = runGate(root);
    console.log('=== restored: the copy after every mutation was reverted ===');
    console.log(`  exit=${restored.status}  ${restored.stdout.trim().split('\n').pop()}`);
    if (restored.status !== 0) {
      failures.push('the restored copy does not pass; the per-claim cases were not isolated');
    }
    console.log('');

    /* Le mutazioni di FORMA. La cancellazione di una frase esercita il
       CONTENUTO della lista e non la sua forma, e la forma e cio che il
       cambiamento di schema ha introdotto: il passo 4 di [SKILL-001] chiede
       quale grandezza sia costante in tutti i casi sopra, e la risposta e che
       in tutti la lista e ben formata e lunga almeno uno. Ecco i casi che la
       variano. */
    const manifestPath = join(root, MANIFEST_REL);
    const manifest = readFileSync(manifestPath, 'utf8');
    const victim = probes[0];
    const idAt = manifest.indexOf(`id = "${victim.id}"`);
    const claimsAt = manifest.indexOf('\nclaims = ', idAt);
    const victimLine = manifest.slice(claimsAt + 1, manifest.indexOf('\n', claimsAt + 1));
    if (idAt < 0 || claimsAt < 0) {
      failures.push('cannot locate a claims line to mutate; the shape cases were not run');
      report();
      return;
    }
    const shapeCases = [
      ['an empty claims list', 'claims = []'],
      ['a claims list holding one empty string', 'claims = [""]'],
      ['no claims field at all', '']
    ];
    console.log('=== G6-CLAIM-STILL-MADE, the shape of the list itself ===');
    for (const [label, replacement] of shapeCases) {
      writeFileSync(manifestPath, manifest.replace(victimLine, replacement), 'utf8');
      try {
        const ok = expectFail(root, label, victim.id);
        console.log(`  ${label}: gate fails naming ${victim.id}: ${ok}`);
      } finally {
        writeFileSync(manifestPath, manifest, 'utf8');
      }
    }

    /* Il colophon: il numero pubblicato al lettore, che da quando `claims` e
       una lista non e piu il numero delle probe. Se questa mutazione non
       mordesse, la pagina potrebbe dichiarare un numero qualsiasi. */
    const stated = /There are (\d+) statements of property on this page/.exec(original);
    const drifted = original.replace(stated[0], `There are ${Number(stated[1]) + 1} statements of property on this page`);
    writeFileSync(htmlPath, drifted, 'utf8');
    let colophonCaught = false;
    try {
      const result = runGate(root);
      colophonCaught = result.status !== 0 && result.stdout.includes('the colophon tells the reader');
      console.log(`  a colophon count one too high: gate fails naming the count: ${colophonCaught}`);
      if (!colophonCaught) {
        failures.push('the colophon count can drift from the number of statements without the gate biting');
      }
    } finally {
      writeFileSync(htmlPath, original, 'utf8');
    }
    console.log('');
    report();
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}

function report() {
  if (failures.length === 0) {
    console.log('public-guide form check, negative proof: PASS');
    process.exitCode = 0;
    return;
  }
  for (const line of failures) console.log(`FAIL ${line}`);
  console.log('');
  console.log(`public-guide form check, negative proof: FAIL (${failures.length} finding(s))`);
  process.exitCode = 1;
}

main();
