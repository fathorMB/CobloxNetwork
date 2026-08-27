---
title: Postura di sicurezza del repository pubblico
updated: 2026-08-27
tags: [security, repository, governance]
---

# Postura di sicurezza del repository pubblico

Stato verificato il 2026-08-25, giorno in cui `github.com/fathorMB/CobloxNetwork`
e' diventato pubblico. Estratto da `STATUS.md` il 2026-08-27: e' stato durevole,
non polso di sessione.

## Audit della storia dei commit

Pulita. Nessuna chiave, token o credenziale in nessun commit — scansionati i
pattern `ghp_`, `gho_`, `github_pat_`, `sk-`, `AKIA`, e le intestazioni di chiave
privata PEM. I file di configurazione degli harness, che contengono davvero
percorsi assoluti e nome utente, non sono mai entrati in un commit.

Unica esposizione residua, severita' bassa: le trascrizioni PowerShell delle
evidenze in [SPEC-002] mostrano `E:\Git\CobloxNetwork` e `F:/dev/android-sdk`.
Sono metadati di ambiente, senza username ne' email.

## Controlli attivati dal Lead, su autorizzazione esplicita dell'operatore

- **Secret scanning e push protection.** Push protection e' l'unico controllo che
  agisce in tempo: blocca un segreto prima che diventi pubblico anziche'
  segnalarlo dopo.
- **Dependabot alerts e security updates.** Hanno prodotto un risultato entro
  pochi minuti, che e' come [DEBT-009] e' stato scoperto.
- **Ruleset su `main`** che vieta force-push e cancellazione del branch.
  Deliberatamente **non** richiede pull request: la strategia main-only con push
  diretto del Lead resta intatta.
- **Pin a SHA di tutte le action di terze parti**, tredici occorrenze, con la
  versione leggibile in commento. Il caso peggiore era
  `dtolnay/rust-toolchain@1.96.0`, che non e' un tag ma un **branch**, quindi
  ripuntabile con un commit qualsiasi. Completato da `.github/dependabot.yml`,
  che propone il refresh in un solo pull request settimanale in batch: un pin non
  invecchia in un modo che GitHub segnali come vulnerabile, smette solo di
  ricevere le correzioni in silenzio, quindi il refresh e' la seconda meta' della
  difesa e non un extra. Il ciclo ha gia' girato una volta:
  [PR #1](https://github.com/fathorMB/CobloxNetwork/pull/1), quattro action con
  salti di major, verificata verde e mergiata su richiesta dell'operatore. Ha
  eliminato anche i warning di deprecazione Node 20.
- **Private vulnerability reporting** abilitato, documentato in `SECURITY.md`.
  Nessun indirizzo email esposto — e' un bersaglio di spam e un punto singolo di
  rottura, mentre il canale di GitHub da' una discussione privata tracciata. Il
  documento dichiara per iscritto i limiti noti invece di lasciarli scoprire:
  rete non resistente ai Sybil per via crittografica ([ADR-007]), set di
  validatori auto-perpetuante in v0 ([DEBT-005]), e advisory derogati con la loro
  condizione di riesame.

## `LICENSE` Apache-2.0

Su conferma dell'operatore. Non era una casella vuota ma una contraddizione: il
`Cargo.toml` del workspace dichiarava `license = "Apache-2.0"` dal bootstrap,
quindi il repository pubblicava crate che dichiaravano una licenza che nessun
file concedeva.

Il testo non e' stato trascritto ma copiato da una copia canonica del registry
Cargo locale, e il corpo verificato identico byte per byte contro una seconda
copia indipendente — su un documento legale la fedelta' conta piu' della
comodita'. Il segnaposto `[yyyy] [name of copyright owner]` nell'`APPENDIX` e'
parte del template di applicazione ai singoli file e va lasciato com'e'.

## Limiti dichiarati

Tutte le voci rilevate al passaggio a pubblico sono chiuse. Restano due limiti:

1. `secret_scanning_non_provider_patterns` richiede GitHub Advanced Security e
   resta disabilitato sul piano attuale. Vengono riconosciuti i formati di
   segreto dei provider noti, non quelli inventati dal progetto — rilevante se in
   futuro Coblox definisse un proprio formato di chiave.
2. I percorsi di macchina nelle trascrizioni di [SPEC-002], severita' bassa.
