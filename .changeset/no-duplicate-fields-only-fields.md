---
"@biomejs/biome": patch
---

Fixed `noDuplicateFields` reporting duplicate arguments and duplicate variable definitions, which `noDuplicateArgumentNames` and `noDuplicateVariableNames` already report. The rule now only checks duplicate fields in a selection set.
