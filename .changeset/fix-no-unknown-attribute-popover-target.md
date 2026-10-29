---
"@biomejs/biome": patch
---

Fixed a false positive in `noUnknownAttribute` for `popoverTarget` and `popoverTargetAction` when no package manifest is available. The rule now accepts these exact camelCase spellings in that case.
