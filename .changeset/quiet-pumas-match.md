---
"@biomejs/biome": patch
---

Fixed CSS metavariables in compound selectors, complex selectors, and property names in GritQL plugins and `biome search`. Patterns such as `` `$tag:hover { color: red; }` ``, `` `$parent .child {}` `` and `` `$property: red` `` now match the intended rules and declarations, whereas they previously failed to parse or matched the wrong nodes.
