---
"@biomejs/biome": patch
---

Improved the rule `noBiomeFirstException`. The rule can now inspect if extended configurations already contain the catch-all `**` inside `files.includes` and, if so, the rule suggests removing `**` from the user configuration.
