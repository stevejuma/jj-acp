---
"@lapismd/jj-acp": patch
---

Release a completed serve turn's worker slot before reporting its record, so an
immediate follow-up submission cannot become stranded after an agent crash.
