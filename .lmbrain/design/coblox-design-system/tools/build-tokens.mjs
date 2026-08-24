#!/usr/bin/env node
/**
 * Coblox design tokens: JSON source -> CSS custom properties.
 *
 * Usage:
 *   node design/tools/build-tokens.mjs           # write design/tokens/tokens.css
 *   node design/tools/build-tokens.mjs --check   # fail if tokens.css is out of date
 *
 * No dependencies: this must stay runnable from a bare Node install so the
 * design system does not acquire a build toolchain of its own.
 */

import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, resolve } from 'node:path';

const here = dirname(fileURLToPath(import.meta.url));
const SRC = resolve(here, '../tokens/tokens.json');
const OUT = resolve(here, '../tokens/tokens.css');

const tokens = JSON.parse(readFileSync(SRC, 'utf8'));
const prefix = tokens.$meta.cssPrefix;

/** Read a dotted path out of the `primitive` tree. */
function primitive(path) {
  const value = path.split('.').reduce((node, key) => (node == null ? node : node[key]), tokens.primitive);
  if (typeof value !== 'string') {
    throw new Error(`Unknown primitive reference: {${path}}`);
  }
  return value;
}

/** Resolve `{a.b.c}` references (possibly several per value) against the primitives. */
export function resolveValue(raw) {
  if (typeof raw !== 'string') throw new Error(`Token value must be a string, got ${typeof raw}`);
  return raw.replace(/\{([^}]+)\}/g, (_, path) => primitive(path));
}

/** `color.bg.surface-raised` -> `--cbx-color-bg-surface-raised` */
function cssName(key) {
  return `--${prefix}-${key.replace(/\./g, '-').replace(/([a-z0-9])([A-Z])/g, '$1-$2').toLowerCase()}`;
}

function flattenPrimitives(node, trail = []) {
  const out = [];
  for (const [key, value] of Object.entries(node)) {
    if (key.startsWith('_') || key.startsWith('$')) continue;
    if (value && typeof value === 'object') out.push(...flattenPrimitives(value, [...trail, key]));
    else out.push([[...trail, key].join('.'), value]);
  }
  return out;
}

function block(selector, entries, comment) {
  const lines = [];
  if (comment) lines.push(`/* ${comment} */`);
  lines.push(`${selector} {`);
  for (const [name, value] of entries) lines.push(`  ${name}: ${value};`);
  lines.push('}');
  return lines.join('\n');
}

/** Every semantic key must exist in every theme, or a theme switch silently breaks. */
function assertThemeParity(themes) {
  const names = Object.keys(themes).filter((k) => !k.startsWith('_'));
  const reference = Object.keys(themes[names[0]]).filter((k) => !k.startsWith('_'));
  for (const name of names.slice(1)) {
    const keys = new Set(Object.keys(themes[name]).filter((k) => !k.startsWith('_')));
    const missing = reference.filter((k) => !keys.has(k));
    const extra = [...keys].filter((k) => !reference.includes(k));
    if (missing.length || extra.length) {
      throw new Error(
        `Theme "${name}" is not in parity with "${names[0]}".` +
          (missing.length ? `\n  missing: ${missing.join(', ')}` : '') +
          (extra.length ? `\n  extra:   ${extra.join(', ')}` : '')
      );
    }
  }
}

function build() {
  assertThemeParity(tokens.semantic);

  const primitives = flattenPrimitives(tokens.primitive).map(([key, value]) => [cssName(key), value]);

  const aliases = Object.entries(tokens.alias)
    .filter(([key]) => !key.startsWith('_'))
    .map(([key, value]) => [cssName(key), resolveValue(value)]);

  const theme = (name) =>
    Object.entries(tokens.semantic[name])
      .filter(([key]) => !key.startsWith('_'))
      .map(([key, value]) => [cssName(key), resolveValue(value)]);

  const header = [
    '/*',
    ` * ${tokens.$meta.name} v${tokens.$meta.version} — GENERATED FILE, DO NOT EDIT.`,
    ` * Source: design/tokens/tokens.json  ·  Generator: design/tools/build-tokens.mjs`,
    ` * Owner: ${tokens.$meta.owner} · Spec: ${tokens.$meta.spec}`,
    ' *',
    ' * Dark is the default theme: bare :root carries the dark semantics.',
    ' * Themes are scoped by [data-theme], so a light region can be nested',
    ' * inside a dark app (and vice versa) without any script.',
    ' */',
    ''
  ].join('\n');

  const css = [
    header,
    block(':root', primitives, 'Primitives — never consumed directly by product surfaces.'),
    '',
    block(':root', aliases, 'Theme-independent semantics (type, space, radius, motion, layout).'),
    '',
    block(':root,\n[data-theme="dark"]', theme('dark'), 'Semantics — dark (default).'),
    '',
    block('[data-theme="light"]', theme('light'), 'Semantics — light (secondary, degradable).'),
    '',
    '/* Motion tokens collapse to zero when the user asks for less motion. */',
    '@media (prefers-reduced-motion: reduce) {',
    '  :root {',
    `    --${prefix}-motion-control: 0ms;`,
    `    --${prefix}-motion-surface: 0ms;`,
    `    --${prefix}-motion-liveness: 0ms;`,
    '  }',
    '}',
    ''
  ].join('\n');

  return css;
}

const css = build();
const check = process.argv.includes('--check');

if (check) {
  let current = '';
  try {
    current = readFileSync(OUT, 'utf8');
  } catch {
    /* missing file counts as drift */
  }
  if (current !== css) {
    console.error('DRIFT: design/tokens/tokens.css does not match tokens.json. Run: node design/tools/build-tokens.mjs');
    process.exit(1);
  }
  const counted = css.match(/^\s+--/gm)?.length ?? 0;
  console.log(`OK: tokens.css is in sync with tokens.json (${counted} custom properties).`);
} else {
  writeFileSync(OUT, css, 'utf8');
  const counted = css.match(/^\s+--/gm)?.length ?? 0;
  console.log(`Wrote ${OUT} (${counted} custom properties).`);
}
