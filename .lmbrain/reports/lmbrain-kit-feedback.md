---
id: LMBRAIN-KIT-FEEDBACK
schema_version: 1
updated: 2026-08-26
lmbrain_version: 5.0.0
notes:
  - schema_version: "1"
    id: "KIT-NOTE-001"
    timestamp: "2026-08-26T21:01:17.585811400+02:00"
    lmbrain_version: "5.0.3"
    category: "bug"
    severity: "medium"
    summary: "skill_activate sposta la skill in active/ e timbra l'attivita, ma non aggiunge la riga al registro dichiarato"
    observed_behavior: "SKILL-004 si trova in `.lmbrain/skills/active/SKILL-004-far-convergere-due-derivazioni-indipendenti.md` con `status: active` nel frontmatter e con l'evento `transitioned proposed -> active` datato 2026-08-26 nel blocco `activity`, cioe' con tutte le tracce di un'attivazione andata a buon fine. La tabella di `.lmbrain/skills/registry.md` conteneva pero' solo SKILL-001, SKILL-002 e SKILL-003: la riga di SKILL-004 non e' mai stata scritta. SKILL-003 e SKILL-004 sono state attivate lo stesso giorno e solo la prima e' finita nel registro."
    expected_behavior: "`skill_activate` dovrebbe essere atomico sui due lati che rendono una skill trovabile: il file in `active/` e la riga nella tabella di `registry.md`. Un'attivazione che aggiorna solo il primo lascia una skill attiva e invisibile a chi legge il registro, che e' la superficie che gli agenti e il Lead consultano per sapere quali procedure esistono."
    impact: "Una skill attiva e non registrata non viene applicata: nessun dispatch la cita, perche' il Lead compone i prompt leggendo il registro e non la cartella. SKILL-004 codifica la procedura che dimostra l'accordo fra due derivazioni indipendenti invece di costruirlo - e' la contromisura a una classe di difetto che questo progetto ha gia' subito con DEBT-012 - ed e' rimasta inapplicabile per tutta la durata in cui e' stata attiva. Il difetto e' inoltre silenzioso: nessuna gate confronta il contenuto di `active/` con le righe del registro, quindi la divergenza non ha alcun modo di farsi notare."
    evidence: "Osservato dal Lead il 2026-08-26 sull'albero a `3f1bef7`, mentre leggeva il registro per comporre un dispatch. `ls .lmbrain/skills/active/` restituisce quattro file (SKILL-001, SKILL-002, SKILL-003, SKILL-004); `grep -n \"SKILL-00\" .lmbrain/skills/registry.md` restituisce tre righe (10, 11, 12), tutte anteriori a SKILL-004. Il frontmatter di SKILL-004 riporta `status: active` e l'evento `transitioned proposed -> active` con data 2026-08-26, identico per forma a quello di SKILL-003 che invece la riga ce l'ha. La riga mancante e' stata aggiunta a mano dal Lead."
    workaround: "Aggiungere la riga a mano dopo ogni `skill_activate`, e verificare la corrispondenza confrontando `ls skills/active/` con `grep SKILL- registry.md`."
    suggested_improvement: "Rendere `skill_activate` transazionale sui due lati, e aggiungere a `lmbrain_validate` una diagnostica che confronti l'insieme dei file in `skills/active/` con l'insieme delle righe `active` del registro, fallendo in entrambe le direzioni. La direzione che conta di piu' e' quella osservata qui: un file presente e una riga assente. E' la stessa forma di difetto che il progetto ha censito quattro volte sotto il nome di \"una gate misura l'insieme dichiarato, non quello osservato\", e ogni volta il membro mancante era l'ultimo arrivato - come qui, dove SKILL-004 e' la piu' recente delle quattro."
    related_note: null
    actor: "AGENT-LEAD"
---
# LMBrain kit feedback

This append-only report records evidence-backed observations about LMBrain itself. It is not project backlog, a `DEBT-*` registry, or lifecycle authority.

The Project Lead maintains it autonomously through `lmbrain_feedback_record`. Share this file with the LMBrain product team when requesting kit improvements or fixes.
