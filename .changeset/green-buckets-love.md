---
"@biomejs/biome": patch
---

Fixed [#9556](https://github.com/biomejs/biome/issues/9556): [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) no longer reports false positives for variables obtained via object destructuring with computed keys, e.g. `const { [KEY]: key1 } = props`.
