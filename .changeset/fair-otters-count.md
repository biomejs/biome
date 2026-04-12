---
"@biomejs/biome": patch
---

Fixed [#9918](https://github.com/biomejs/biome/issues/9918): [`useConsistentTestIt`](https://biomejs.dev/linter/rules/use-consistent-test-it/) no longer panics when applying fixes to chained calls such as `test.for([])("x", () => {});`.
