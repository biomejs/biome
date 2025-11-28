---
"@biomejs/biome": patch
---

Improved the rule `noBiomeFirstExpection`. The rule is now able to inspect if extended configurations already contain the catch-all `**` inside `files.includes` and, if so, the rule suggests to remove `**` from the user configuration.
