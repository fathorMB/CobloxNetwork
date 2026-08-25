#!/usr/bin/env node
/**
 * GATE di forma della guida pubblica (SPEC-015).
 *
 * Il design system di [SPEC-005] prova che 130 coppie di contrasto su 130
 * passano WCAG AA. Quella prova non dice nulla su una pagina che inventa un
 * accostamento fuori elenco o che scrive un colore letterale: dice solo che le
 * coppie DICHIARATE tengono. Questo strumento chiude la distanza fra le due
 * cose per la sola guida pubblica, con sei verifiche meccaniche:
 *
 *   G1 NO-LITERAL-COLOUR   nessun colore letterale in guide.css o index.html
 *   G2 KNOWN-TOKEN         ogni --cbx-* citato esiste in tokens.css
 *   G3 DECLARED-PAIR       ogni coppia di used-pairs.json e dichiarata nel
 *                          design system, con lo stesso tipo
 *   G4 ADR-009             l'unita e posposta al numero, il separatore delle
 *                          migliaia e U+202F, e il glifo ritirato non compare
 *   G5 SELF-CONTAINED      la pagina non carica nulla dalla rete e non ha script
 *   G6 CLAIM-STILL-MADE    ogni probe `guide-*` di published_artifacts.toml
 *                          ancora una frase che la pagina dice ancora
 *
 * G6 e il verso che il manifesto delle probe non copre da solo: le probe
 * proteggono la guida dal cambiamento delle regole, questo controllo protegge
 * le probe dal cambiamento della guida. Un'ancora rimasta sola non e un
 * ancoraggio: e un commento.
 *
 * Non ricalcola i rapporti di contrasto: quello e il mestiere di
 * ../coblox-design-system/tools/check-contrast.mjs, che va eseguito insieme a
 * questo. Riscriverne la formula qui produrrebbe due copie che divergono.
 *
 *   node .lmbrain/design/coblox-public-guide/tools/check-guide-pairs.mjs
 *
 * Esce 0 se ogni verifica passa, 1 nominando la classe che ha fallito.
 */

import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const here = dirname(fileURLToPath(import.meta.url));
const PKG = resolve(here, '..');
const SYSTEM = resolve(PKG, '../coblox-design-system');

const GUIDE_CSS = resolve(PKG, 'guide.css');
const GUIDE_HTML = resolve(PKG, 'index.html');
const USED_PAIRS = resolve(PKG, 'used-pairs.json');
const TOKENS_CSS = resolve(SYSTEM, 'tokens/tokens.css');
const SYSTEM_PAIRS = resolve(SYSTEM, 'tokens/contrast-pairs.json');

const read = (p) => readFileSync(p, 'utf8');
const failures = [];
const fail = (code, message) => failures.push(`${code}: ${message}`);
const counts = {};
const note = (code, n) => (counts[code] = n);

const NAMED_ENTITIES = {
  amp: '&', lt: '<', gt: '>', quot: '"', apos: "'", nbsp: ' ',
  mdash: '—', ndash: '–', hellip: '…', middot: '·',
  ldquo: '“', rdquo: '”', lsquo: '‘', rsquo: '’',
  minus: '−', times: '×'
};

const css = read(GUIDE_CSS);
const html = read(GUIDE_HTML);
const tokensCss = read(TOKENS_CSS);
const used = JSON.parse(read(USED_PAIRS));
const systemPairs = JSON.parse(read(SYSTEM_PAIRS));

/* -------------------------------------------------- G1 no literal colours */

/* I commenti di guide.css contengono nomi di token e prosa, mai valori: la
   scansione li include di proposito, perche un colore letterale scritto in un
   commento e il primo passo verso lo stesso colore scritto in una regola. */
const COLOUR_LITERALS = [
  [/#[0-9a-fA-F]{3,8}\b/g, 'hex colour'],
  [/\brgba?\s*\(/g, 'rgb()/rgba()'],
  [/\bhsla?\s*\(/g, 'hsl()/hsla()'],
  [/\b(?:color-mix|oklch|oklab|lab|lch)\s*\(/g, 'modern colour function'],
  [/(?:^|[\s:;,{(])(?:red|green|blue|black|white|grey|gray|orange|purple|yellow|cyan|magenta)(?=[\s;,)}]|$)/g, 'CSS named colour']
];

{
  let checked = 0;
  for (const [source, text] of [['guide.css', css], ['index.html', stripHtmlComments(html)]]) {
    for (const [re, label] of COLOUR_LITERALS) {
      for (const m of text.matchAll(re)) {
        /* Le entita HTML numeriche (&#8239;) e i riferimenti a frammenti
           (url(#ah-mint), href="#q-what") non sono colori. */
        if (label === 'hex colour') {
          const before = text[m.index - 1];
          if (before === '&' || before === '(' || before === '"' || before === "'") continue;
          if (/^#[a-zA-Z_-]/.test(m[0])) continue;
        }
        fail('G1-NO-LITERAL-COLOUR', `${source} carries a ${label}: ${JSON.stringify(m[0].trim())}`);
      }
      checked += 1;
    }
  }
  note('G1-NO-LITERAL-COLOUR', checked);
}

/* ---------------------------------------------------- G2 tokens must exist */

{
  const declared = new Set([...tokensCss.matchAll(/(--cbx-[a-z0-9-]+)\s*:/g)].map((m) => m[1]));
  const referenced = new Set([...`${css}\n${html}`.matchAll(/var\(\s*(--cbx-[a-z0-9-]+)/g)].map((m) => m[1]));
  for (const name of [...referenced].sort()) {
    if (!declared.has(name)) {
      fail('G2-KNOWN-TOKEN', `${name} is used by the guide but is not emitted by tokens.css`);
    }
  }
  note('G2-KNOWN-TOKEN', referenced.size);
}

/* ------------------------------------------------- G3 pairs must be declared */

{
  const key = (p) => `${p.fg} on ${p.bg}`;
  const declared = new Map(systemPairs.pairs.map((p) => [key(p), p.type]));
  for (const pair of used.pairs) {
    const type = declared.get(key(pair));
    if (type === undefined) {
      fail(
        'G3-DECLARED-PAIR',
        `the guide uses ${key(pair)} but the design system does not declare it as legitimate. ` +
          `A pairing that is not in contrast-pairs.json has never been contrast-checked.`
      );
    } else if (type !== pair.type) {
      fail(
        'G3-DECLARED-PAIR',
        `the guide records ${key(pair)} as "${pair.type}" but the design system declares it as "${type}". ` +
          `The two thresholds are not the same and the weaker reading would be the one that passes.`
      );
    }
    if (!String(pair.where || '').trim()) {
      fail('G3-DECLARED-PAIR', `${key(pair)} is declared used but does not say where`);
    }
  }
  note('G3-DECLARED-PAIR', used.pairs.length);
}

/* ------------------------------------------------------------- G4 ADR-009 */

{
  let checked = 0;

  /* Il glifo ritirato, ovunque nel pacchetto. */
  checked += 1;
  for (const [source, text] of [['guide.css', css], ['index.html', html], ['used-pairs.json', read(USED_PAIRS)]]) {
    if (text.includes('◇')) {
      fail('G4-ADR-009', `${source} contains U+25C7, the retired token glyph. ADR-009 withdrew it.`);
    }
  }

  /* L'unita e posposta: ogni .cbx-unit segue immediatamente un .cbx-num. */
  checked += 1;
  const unitTotal = [...html.matchAll(/class="cbx-unit"/g)].length;
  const unitPosposed = [...html.matchAll(/<span class="cbx-num">[^<]*<\/span\s*\n?\s*><span class="cbx-unit">/g)].length;
  if (unitTotal !== unitPosposed) {
    fail(
      'G4-ADR-009',
      `${unitTotal} unit span(s) in index.html but ${unitPosposed} immediately follow a cbx-num span. ` +
        `An abbreviation that PRECEDES the number is the grammar of money; ADR-009 requires it to follow.`
    );
  }

  /* Il separatore delle migliaia e U+202F: nella prosa visibile non deve
     comparire ne la virgola ne il punto fra gruppi di tre cifre, e nessun
     numero di quattro cifre puo restare senza separatore. */
  checked += 1;
  const prose = visibleProse(html);
  for (const m of prose.matchAll(/\d[.,]\d{3}(?!\d)/g)) {
    fail(
      'G4-ADR-009',
      `index.html writes ${JSON.stringify(m[0])} in visible prose. The thousands separator is ` +
        `U+202F (narrow no-break space), never a comma or a full stop.`
    );
  }
  checked += 1;
  for (const m of prose.matchAll(/(?<![\d ])\d{4,}(?![\d ])/g)) {
    fail(
      'G4-ADR-009',
      `index.html writes the unseparated number ${JSON.stringify(m[0])} in visible prose. ` +
        `Groups of three digits are separated by U+202F.`
    );
  }
  note('G4-ADR-009', checked);
}

/* ---------------------------------------------- G6 probes still have a claim */

/* La direzione che il manifesto delle probe da solo non copre. Una probe
   ancora la pagina alla regola: se la REGOLA cambia, published_artifacts.py
   diventa rosso. Se cambia la PAGINA, invece, non se ne accorge nessuno, e la
   probe resta a difendere una frase che non e piu scritta da nessuna parte —
   che e la stessa cosa di un commento.
   Questo controllo chiude il verso mancante: ogni probe `guide-*` porta la
   frase della guida che sostiene, e quella frase deve trovarsi ancora nella
   pagina. */
{
  const MANIFEST = resolve(PKG, '../../../sim/tools/published_artifacts.toml');
  const probes = readProbeTable(read(MANIFEST));
  const prose = visibleText(html);
  let checked = 0;
  for (const probe of probes) {
    if (!probe.id?.startsWith('guide-')) continue;
    checked += 1;
    const claim = (probe.claims || '').replace(/\s+/g, ' ').trim();
    if (!claim) {
      fail('G6-CLAIM-STILL-MADE', `probe ${probe.id} pins a rule for the guide but does not record which sentence it holds`);
      continue;
    }
    if (!prose.includes(claim)) {
      fail(
        'G6-CLAIM-STILL-MADE',
        `probe ${probe.id} claims to anchor ${JSON.stringify(claim)}, which the guide no longer says. ` +
          `Either the sentence was edited and its anchor was left behind, or the sentence was dropped ` +
          `and the probe now defends nothing.`
      );
    }
  }
  if (checked === 0) {
    fail('G6-CLAIM-STILL-MADE', 'no guide-* probe exists in published_artifacts.toml; the guide is unanchored');
  }
  note('G6-CLAIM-STILL-MADE', checked);
}

/* ------------------------------------------------------- G5 self-contained */

{
  let checked = 0;
  for (const m of html.matchAll(/\b(?:href|src)\s*=\s*"([^"]*)"/g)) {
    checked += 1;
    const value = m[1];
    if (/^(?:[a-z][a-z0-9+.-]*:)?\/\//i.test(value)) {
      fail('G5-SELF-CONTAINED', `index.html loads ${JSON.stringify(value)} from the network. The guide must open from disk.`);
    }
  }
  for (const m of css.matchAll(/url\(\s*['"]?([^'")]+)/g)) {
    checked += 1;
    if (/^(?:[a-z][a-z0-9+.-]*:)?\/\//i.test(m[1])) {
      fail('G5-SELF-CONTAINED', `guide.css loads ${JSON.stringify(m[1])} from the network.`);
    }
  }
  if (/<script\b/i.test(html)) {
    fail('G5-SELF-CONTAINED', 'index.html carries a <script> element; the guide is script-free.');
  }
  note('G5-SELF-CONTAINED', checked);
}

/* ------------------------------------------------------------------ report */

function stripHtmlComments(text) {
  return text.replace(/<!--[\s\S]*?-->/g, ' ');
}

/** Tutto il testo che un lettore vede, comprese le etichette dentro i
    diagrammi: un'affermazione disegnata e un'affermazione come le altre. Le
    entita sono risolte, cosi che &#8239; e &mdash; siano i caratteri che sono,
    e le virgolette curve sono raddrizzate perche la frase registrata nella
    probe e scritta in ASCII. */
function visibleText(text) {
  return decode(stripHtmlComments(text).replace(/<[^>]+>/g, ' '));
}

/** La sola prosa: i diagrammi sono esclusi perche le loro coordinate sono
    numeri che nessuno legge, e la verifica sui separatori delle migliaia li
    prenderebbe per importi. */
function visibleProse(text) {
  return decode(
    stripHtmlComments(text)
      .replace(/<svg[\s\S]*?<\/svg>/gi, ' ')
      .replace(/<[^>]+>/g, ' ')
  );
}

function decode(text) {
  return text
    .replace(/&#(\d+);/g, (_, d) => String.fromCodePoint(Number(d)))
    .replace(/&#x([0-9a-f]+);/gi, (_, h) => String.fromCodePoint(parseInt(h, 16)))
    .replace(/&([a-zA-Z]+);/g, (m, name) => NAMED_ENTITIES[name] ?? m)
    .replace(/[“”]/g, '"')
    .replace(/[‘’]/g, "'")
    .replace(/\s+/g, ' ')
    .trim();
}

/** Lettore volutamente minimo di published_artifacts.toml: legge le sole
    tabelle [[probe]] e le sole chiavi con valore stringa su una riga, che e la
    forma che quel file usa. Non e un parser TOML e non pretende di esserlo;
    una voce scritta in un'altra forma verrebbe ignorata, quindi il file resta
    la fonte e questo strumento il consumatore. */
function readProbeTable(toml) {
  const rows = [];
  let current = null;
  for (const line of toml.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (trimmed.startsWith('[')) {
      if (current) rows.push(current);
      current = trimmed === '[[probe]]' ? {} : null;
      continue;
    }
    if (!current) continue;
    const m = /^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"((?:[^"\\]|\\.)*)"\s*$/.exec(trimmed);
    if (m) current[m[1]] = m[2].replace(/\\(.)/g, (_, c) => (c === 'n' ? '\n' : c));
  }
  if (current) rows.push(current);
  return rows;
}

for (const code of ['G1-NO-LITERAL-COLOUR', 'G2-KNOWN-TOKEN', 'G3-DECLARED-PAIR', 'G4-ADR-009', 'G5-SELF-CONTAINED', 'G6-CLAIM-STILL-MADE']) {
  console.log(`  ${code.padEnd(22)} ${String(counts[code] ?? 0).padStart(4)} candidate(s) checked`);
}
console.log('');

if (failures.length === 0) {
  console.log('public-guide form check: PASS');
  process.exit(0);
}
for (const line of failures) console.log(`FAIL ${line}`);
console.log('');
console.log(`public-guide form check: FAIL (${failures.length} finding(s))`);
process.exit(1);
