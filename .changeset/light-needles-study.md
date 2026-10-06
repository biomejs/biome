---
"@biomejs/biome": patch
---

Fixed [#9749](https://github.com/biomejs/biome/issues/9749): in GritQL plugins and `biome search`, a `contains` followed by further conditions now tries every node it matches, instead of failing when the first match doesn't satisfy the remaining conditions.
