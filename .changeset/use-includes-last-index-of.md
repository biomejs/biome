---
"@biomejs/biome": patch
---

Partially fixed [#10552](https://github.com/biomejs/biome/issues/10552): [`useIncludes`](https://biomejs.dev/linter/rules/use-includes/) now also detects `lastIndexOf()` comparisons against `-1`/`0`, suggesting `includes()` instead
