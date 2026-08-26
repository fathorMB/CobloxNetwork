#!/usr/bin/env python3
"""Applica al Project Lead le regole che il Lead impone agli altri.

Nasce da un fatto, non da un'intenzione: nella notte del 2026-08-26 cinque
difetti su cinque erano in artefatti scritti dal Lead, e nessuno li aveva
attaccati perche' per cio' che scrive il Lead autore e revisore sono la stessa
persona. Gli agenti rivedono le implementazioni e il Lead rivede le loro; le
spec, le review, le ADR e i debiti del Lead non li rivede nessuno.

Due controlli, ciascuno preso da un difetto reale di quella notte.

  L1-ATTACK      Una review firmata dal Lead deve dire **cosa ha attaccato
                 senza riuscire a romperlo**. REVIEW-025 lodava l'asimmetria
                 fuori banda di SPEC-016 come "la parte migliore del lavoro"
                 e non l'aveva attaccata; REVIEW-027 vi ha trovato un finding
                 high. Cio' che il Lead loda e' precisamente cio' che smette
                 di verificare, quindi la parte che sembra migliore va
                 attaccata per prima. Una review senza questa sezione non e'
                 una review: e' una lettura.

  L2-SUPERLATIVE Un superlativo assoluto in un artefatto del Lead deve portare
                 accanto la traccia dell'enumerazione che lo sostiene.
                 DEBT-014 si intitolava "l'unica preimmagine a dominio separato
                 non legata a chain_id"; era falso, sei altre lo omettono, e il
                 Lead l'aveva ereditato da SPEC-010 senza contarle. Un
                 superlativo e' una affermazione universale: o e' enumerato, o
                 e' una congettura scritta come fatto.

Le deroghe sono dichiarate in DEROGATIONS con la loro ragione, mai silenziose:
uno strumento che nasce rosso viene disattivato, ed e' il modo in cui una
guardia muore.

Limite dichiarato, perche' una lista dichiarata invece che osservata e' il
difetto che questo progetto ha gia' pagato due volte: la paternita' del Lead e'
riconosciuta dal frontmatter (`reviewer`, `decider`, `actor`, `from_role`).
Un artefatto che il Lead scrive **senza** uno di quei campi - una pagina di
`knowledge/`, per esempio - non viene controllato. E' tracciato in DEBT-027
insieme all'arretrato e non e' silenzioso.

Uso:  python sim/tools/lead_claims_check.py
Exit: 0 se tutto passa, 1 al primo fallimento, con la classe nominata.
"""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path

REPO = Path(os.environ.get("COBLOX_REPO", Path(__file__).resolve().parents[2]))

LEAD = "AGENT-LEAD"

# La regola vincola in avanti. Gli artefatti anteriori non la violano: la regola
# non c'era. Il loro arretrato non e' pero' silenzioso - viene contato e stampato
# a ogni esecuzione, ed e' tracciato in DEBT-027. Uno strumento che nasce rosso
# viene disattivato, ed e' il modo in cui una guardia muore.
RULE_DATE = "2026-08-26"

CREATED = re.compile(r"^created:\s*\"?(\d{4}-\d{2}-\d{2})", re.M)


def in_scope(text: str) -> bool:
    m = CREATED.search(text)
    return bool(m) and m.group(1) >= RULE_DATE

# La sezione che una review del Lead deve portare. Il testo puo' variare;
# quello che non puo' mancare e' che dichiari un attacco non riuscito.
ATTACK_HEADING = re.compile(
    r"^##+\s+.*(attaccat|non si e' rott|non si è rott|senza romper).*$",
    re.IGNORECASE | re.MULTILINE,
)
ATTACK_MIN_CHARS = 120

# Superlativi assoluti: affermazioni universali travestite da descrizione.
SUPERLATIVES = re.compile(
    r"\b(l'unic[ao]|il solo|la sola|l'unica volta|nessun altr[ao]|nessuna altra"
    r"|in nessun altro|ogni altr[ao]|tutti gli altri|tutte le altre)\b",
    re.IGNORECASE,
)
# La traccia che rende un superlativo ammissibile: qualcuno le ha contate.
ENUMERATION = re.compile(
    r"(per esaurimento|enumera|verificat[oa] contando|contate una per una"
    r"|contando|elenco completo|controesemp)",
    re.IGNORECASE,
)

# Deroghe dichiarate. Ogni voce porta la propria ragione: una deroga senza
# ragione e' silenzio, e il silenzio e' cio' che questo strumento cerca.
DEROGATIONS = {
    "REVIEW-025": (
        {"L1-ATTACK", "L2-SUPERLATIVE"},
        "Non ha una sezione di attacco, ed e' precisamente il difetto che ha "
        "prodotto questo strumento: REVIEW-027 ha trovato un finding high dove "
        "questa review lodava. Riscriverla per far passare la guardia "
        "significherebbe cancellare la prova. Resta rossa e derogata. "
        "L2: lo strumento vi ha trovato tre superlativi non enumerati. Quello "
        "falso - height come unica grandezza non scritta liberamente - e' stato "
        "corretto in loco con l'enumerazione dei dodici campi di BlockHeader. "
        "Gli altri due sono imprecisioni e non falsita', e sono nell'arretrato "
        "di DEBT-027 insieme agli altri 36.",
    ),
    "REVIEW-024": (
        {"L1-ATTACK"},
        "Scritta prima che la regola esistesse. Contiene una verifica "
        "indipendente reale - il test del filo chiuso rifatto spogliando il DOM "
        "di ogni corpo apribile - ma non in una sezione che questo strumento "
        "possa riconoscere. Derogata per anteriorita', non per merito.",
    ),
    "REVIEW-026": (
        {"L1-ATTACK"},
        "Scritta prima che la regola esistesse. Contiene un attacco reale - le "
        "tre conferme piu' deboli sono state attaccate e una e' caduta - ma non "
        "in una sezione che questo strumento possa riconoscere. Derogata per "
        "anteriorita', non per merito.",
    ),
}


def fail(cls: str, msg: str) -> None:
    print(f"  FAIL {cls}: {msg}")
    sys.exit(1)


def body_of(text: str) -> str:
    """Il corpo, senza frontmatter.

    Il frontmatter porta i `review_events`, cioe' copie verbatim di ragioni gia'
    presenti nel corpo, serializzate su una riga. Sono un **registro di cio' che
    e' stato detto**, non un'affermazione che l'artefatto sta facendo adesso, e
    controllarle due volte farebbe fallire ogni review su una citazione di se
    stessa. Il restringimento e' dichiarato qui e non applicato in silenzio.
    """
    if text.startswith("---"):
        end = text.find("\n---", 3)
        if end != -1:
            return text[end + 4 :]
    return text


def lead_authored(text: str) -> bool:
    return bool(re.search(rf"^(reviewer|decider|actor|from_role):\s*\"?{LEAD}", text, re.M))


def check_reviews() -> int:
    checked = 0
    for path in sorted((REPO / ".lmbrain" / "reviews").rglob("REVIEW-*.md")):
        text = path.read_text(encoding="utf-8")
        if not lead_authored(text):
            continue
        if not in_scope(text):
            continue
        checked += 1
        rid = path.name.split("-review")[0].split("-security")[0]
        m = ATTACK_HEADING.search(text)
        body = ""
        if m:
            rest = text[m.end():]
            nxt = re.search(r"^##+\s", rest, re.MULTILINE)
            body = rest[: nxt.start()] if nxt else rest
        if not m or len(body.strip()) < ATTACK_MIN_CHARS:
            if "L1-ATTACK" in DEROGATIONS.get(rid, (set(),))[0]:
                continue
            fail(
                "L1-ATTACK",
                f"{path.relative_to(REPO)} e' firmata dal Lead e non dice cosa ha "
                f"attaccato senza riuscire a romperlo. Una review che loda senza "
                f"attaccare non ha verificato: e' il difetto che REVIEW-027 ha "
                f"trovato dentro REVIEW-025. Aggiungere una sezione che nomini "
                f"almeno un tentativo di rottura e il suo esito, oppure "
                f"dichiarare una deroga con la sua ragione.",
            )
    return checked


def check_superlatives(backlog) -> int:
    checked = 0
    roots = ["reviews", "specs", "decisions", "debts", "knowledge", "handoffs"]
    for root in roots:
        base = REPO / ".lmbrain" / root
        if not base.exists():
            continue
        for path in sorted(base.rglob("*.md")):
            text = path.read_text(encoding="utf-8")
            if not lead_authored(text):
                continue
            if not in_scope(text):
                backlog[0] += sum(
                    1
                    for para in re.split(r"\n\s*\n", body_of(text))
                    if SUPERLATIVES.search(para)
                    and not ENUMERATION.search(para)
                    and not re.search(r"(e' falso|è falso|era falso|~~)", para, re.I)
                )
                continue
            checked += 1
            for para in re.split(r"\n\s*\n", body_of(text)):
                if not SUPERLATIVES.search(para):
                    continue
                if ENUMERATION.search(para):
                    continue
                # Un superlativo citato per essere corretto non e' una pretesa.
                if re.search(r"(e' falso|è falso|era falso|~~|corretto dal Lead"
                             r"|non e' vero|non è vero)", para, re.IGNORECASE):
                    continue
                rid = re.match(r"([A-Z]+-\d+)", path.name)
                if rid and "L2-SUPERLATIVE" in DEROGATIONS.get(rid.group(1), (set(),))[0]:
                    break
                m = SUPERLATIVES.search(para)
                # Un superlativo **negato** non e' una pretesa universale: e' il
                # suo rifiuto, che e' esattamente cio' che questo strumento
                # vuole incoraggiare. "non si puo' presumere che sia l'unica"
                # non afferma nulla di universale.
                before = para[max(0, m.start() - 60) : m.start()]
                if re.search(r"\b(non|senza|smentit|escluso)\b", before, re.IGNORECASE):
                    continue
                hit = m.group(0)
                fail(
                    "L2-SUPERLATIVE",
                    f"{path.relative_to(REPO)} porta il superlativo \"{hit}\" senza "
                    f"la traccia di un'enumerazione. Un superlativo e' una "
                    f"affermazione universale: o e' contato, o e' una congettura "
                    f"scritta come fatto. E' cosi' che DEBT-014 ha ereditato da "
                    f"SPEC-010 un'affermazione falsa e l'ha portata avanti per tre "
                    f"stesure.\n    Paragrafo: {para.strip()[:220]}",
                )
    return checked


def main() -> int:
    print("lead-claims check")
    r = check_reviews()
    print(f"  L1-ATTACK        {r} review del Lead controllate")
    backlog = [0]
    s = check_superlatives(backlog)
    print(f"  L2-SUPERLATIVE   {s} artefatti del Lead controllati")
    print(
        f"  arretrato        {backlog[0]} superlativi non enumerati in artefatti "
        f"anteriori al {RULE_DATE}, fuori scopo per data e tracciati in DEBT-027"
    )
    if DEROGATIONS:
        print(f"  deroghe dichiarate: {len(DEROGATIONS)}")
        for rid, (cls, why) in DEROGATIONS.items():
            print(f"    {rid} [{','.join(sorted(cls))}] {why.splitlines()[0][:88]}")
    print("\nlead-claims: PASS")
    return 0




# --- prova in negativo -------------------------------------------------------
# Una guardia che non si e' mai vista scattare e' un calcolo. Ogni classe viene
# qui reintrodotta ad arte e osservata fallire, senza toccare l'albero.

_SAMPLES = [
    (
        "L1-ATTACK",
        "una review del Lead senza sezione di attacco",
        "---\nreviewer: AGENT-LEAD\ncreated: 2026-09-01\n---\n"
        "# Review\n\n## Outcome\n\nTutto verde, accettata.\n",
        False,
    ),
    (
        "L1-ATTACK",
        "la stessa review con una sezione di attacco troppo corta per dire qualcosa",
        "---\nreviewer: AGENT-LEAD\ncreated: 2026-09-01\n---\n"
        "# Review\n\n## Cosa ho attaccato senza romperlo\n\nNiente.\n",
        False,
    ),
    (
        "L1-ATTACK",
        "e con un attacco reale, che deve passare",
        "---\nreviewer: AGENT-LEAD\ncreated: 2026-09-01\n---\n"
        "# Review\n\n## Cosa ho attaccato senza romperlo\n\n"
        "Ho provato a costruire una preimmagine per il dominio sbagliato e a "
        "passarla al verificatore: non compila, e l'errore e' E0308. Ho poi "
        "provato a far passare la gate cancellando la frase pinnata: fallisce.\n",
        True,
    ),
]

_PARAS = [
    ("un superlativo nudo", "`height` e' l'unica grandezza che nessuno scrive.", False),
    (
        "lo stesso con la traccia dell'enumerazione",
        "`height` e' l'unica grandezza che nessuno scrive, verificato enumerando "
        "i dodici campi di BlockHeader.",
        True,
    ),
    ("un superlativo negato, che non e' una pretesa", "Non si puo' presumere che sia l'unica.", True),
]


def _l1_ok(text: str) -> bool:
    m = ATTACK_HEADING.search(text)
    if not m:
        return False
    rest = text[m.end():]
    nxt = re.search(r"^##+\s", rest, re.MULTILINE)
    body = rest[: nxt.start()] if nxt else rest
    return len(body.strip()) >= ATTACK_MIN_CHARS


def _l2_ok(para: str) -> bool:
    m = SUPERLATIVES.search(para)
    if not m or ENUMERATION.search(para):
        return True
    before = para[max(0, m.start() - 60) : m.start()]
    return bool(re.search(r"\b(non|senza|smentit|escluso)\b", before, re.IGNORECASE))


def prove_negative() -> int:
    print("prova in negativo di lead-claims\n")
    bad = 0
    print("=== L1-ATTACK ===")
    for _, label, text, expected in _SAMPLES:
        got = _l1_ok(text)
        mark = "ok" if got == expected else "NON OSSERVATO"
        if got != expected:
            bad += 1
        print(f"  {mark:14} {label}: atteso {'passa' if expected else 'fallisce'}")
    print("\n=== L2-SUPERLATIVE ===")
    for label, para, expected in _PARAS:
        got = _l2_ok(para)
        mark = "ok" if got == expected else "NON OSSERVATO"
        if got != expected:
            bad += 1
        print(f"  {mark:14} {label}: atteso {'passa' if expected else 'fallisce'}")
    if bad:
        print(f"\nprova in negativo: FAIL - {bad} caso/i non si e' comportato come dichiarato")
        return 1
    print(f"\nprova in negativo: PASS - {len(_SAMPLES) + len(_PARAS)} casi, "
          f"ciascuno osservato nel verso dichiarato")
    return 0


if __name__ == "__main__":
    if "--prove-negative" in sys.argv:
        sys.exit(prove_negative())
    sys.exit(main())
