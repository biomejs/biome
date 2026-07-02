---
"@biomejs/biome": patch
---

Implemented [#10552](https://github.com/biomejs/biome/issues/10552): the rule [`useIncludes`](https://biomejs.dev/linter/rules/use-includes/) now also reports `lastIndexOf()` comparisons against `-1` and `some()` calls with a strict-equality callback, e.g. `arr.lastIndexOf(x) !== -1` and `arr.some(item => item === x)`. Both are fixed to `arr.includes(x)`.
