---
title: Agent registry
updated: 2026-08-27
---

# Agent Registry

| ID | Name | Role | Status | Activation | Definition | Domains |
| --- | --- | --- | --- | --- | --- | --- |
| AGENT-LEAD | Ada Checklist | Project Lead | active | manual | [[project-lead]] | project-management |
| AGENT-001 | Dario Meshnet | Rust Core & P2P Specialist | active | manual | [[AGENT-001-rust-core-p2p-specialist]] | rust, libp2p, networking, crypto |
| AGENT-002 | Sofia Consenso | Ledger, Consenso & Token Economy | active | manual | [[AGENT-002-ledger-consenso-token-economy-specialist]] | consensus, bft, ledger, token-economy, simulation, governance |
| AGENT-003 | Elio Sandbox | WASM Runtime & Compute Specialist | active | manual | [[AGENT-003-wasm-runtime-compute-specialist]] | wasm, wasi, sandbox, compute, sdk |
| AGENT-004 | Marta Pixelperfetta | Desktop UI (Tauri) Specialist | active | manual | [[AGENT-004-desktop-ui-tauri-specialist]] | tauri, frontend, desktop |
| AGENT-005 | Nadia Composita | Android Node Specialist | active | manual | [[AGENT-005-android-node-specialist]] | android, kotlin, compose, uniffi |
| AGENT-006 | Lia Wireframe | Design System & UX Specialist | active | manual | [[AGENT-006-design-system-ux-specialist]] | design, ui-ux, accessibility |
| AGENT-007 | Greta Threatmodel | Security & Threat Model Reviewer | active | manual | [[AGENT-007-security-threat-model-reviewer]] | security, threat-modeling, review |
| AGENT-008 | Remo Pipeline | DevOps, CI & Release Specialist | active | manual | [[AGENT-008-devops-ci-release-specialist]] | ci, build, release, packaging, devnet, tooling |

Add specialist profiles only when a real project need justifies them. Keep profiles in `profiles/` and proposals in `proposals/`.

**Activation guard:** Profiles with `status: proposed` are not ready for implementation handoff. The Project Lead must ask the operator to approve and activate a proposed profile (set `status: active`) before recommending it for a spec. The operator activates profiles by updating the frontmatter `status` field.

## V3 controlled improvement loop

Improvement proposals follow the same lifecycle as new-profile proposals but use `proposal_type: improvement` and specify a `target_profile`. The Project Lead may create improvement proposals from accepted reviews, repeated remediation findings, implementation evidence, diagnostics, or operator feedback. Operator approval is required before any behavior-affecting profile change becomes active.
