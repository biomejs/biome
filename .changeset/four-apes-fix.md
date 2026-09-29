---
"@biomejs/biome": patch
---

Fixed [#11951](https://github.com/biomejs/biome/issues/11951): the GritQL formatter no longer inserts a space after a `within` pattern without an `until` clause.

```diff
-$arg <: within `bar($_)` ,
+$arg <: within `bar($_)`,
```
