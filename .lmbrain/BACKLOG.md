---
title: Product and technical backlog
updated: 2026-08-25
---

# Backlog

This is a concise, prioritized index of opportunities and work areas. Implementation handoffs (specs) live under `specs/`.

## Now

- ~~[SPEC-001] Specifica del protocollo v0 — AGENT-001, M-01, high~~ → **done** il 2026-08-25
- ~~[SPEC-002] Scheletro workspace Rust + CI multipiattaforma — AGENT-008, M-01, high~~ → **done** il 2026-08-25
- [SPEC-003] Fondamenta del design system — AGENT-006, M-01, medium
- ~~[SPEC-004] Threat model iniziale — AGENT-007, M-01, high~~ → **done** il 2026-08-25

## Debiti aperti

- [DEBT-005] Il set di validatori è auto-perpetuante: manca la regola di elezione — **critical**, owner AGENT-002, M-02.
- [DEBT-001] La pipeline CI non è mai stata eseguita, `GATE-CI-GREEN` derogato — high, owner AGENT-008, M-01. Sbloccabile solo dalla fatturazione GitHub dell'operatore.
- [DEBT-006] La quota al creatore obbliga a pubblicare chi è abbonato a cosa — high, owner AGENT-LEAD, M-06.
- [DEBT-007] La forma del reddito di esistenza non è decisa e determina `α` — high, owner AGENT-002, M-02.
- [DEBT-008] Due imprecisioni residue nella specifica del protocollo v0 — low, owner AGENT-001, M-02.

## Next

- Decisione sul nome del token/unità (branding, coinvolge AGENT-006).
- Spec M-02: elezione/rotazione dei validatori (i vincoli emergeranno da SPEC-001 e SPEC-004).
- Configurazioni locali degli harness (`.codex/`, `.pi/`, `.mcp.json`, `opencode.json`): decidere se ignorarle o renderle portabili — contengono percorsi assoluti.
- Da [ADR-006]: entità "saldo dell'app" nel ledger con consumo per epoche (M-02, dominio AGENT-002).
- Da [ADR-006]: parametro economico della quota al creatore, da tarare nel simulatore (M-02).

## Later

- Mapping Compose dei design token (dopo SPEC-003).
- Da [ADR-006]: flusso di pubblicazione end-to-end e catalogo delle app (M-06).
- Da [ADR-006]: lista di rifiuto per nodo e lista di blocco di rete — funzionalità di prodotto, non dettagli implementativi (M-06/M-07).
- Simulatore economico (M-02, [ADR-005]).
- Tier "nodo certificato" con attestazione hardware (idea parcheggiata da [ADR-002]).

## Parking lot

- Demurrage anti-accumulo ([ADR-005], fase 2).
- Tier container per nodi headless potenti ([ADR-004], fase 2).
- iOS/macOS.
