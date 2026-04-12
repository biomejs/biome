---
"@biomejs/biome": patch
---

Fixed [#9744](https://github.com/biomejs/biome/issues/9744): [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) no longer reports a false positive when a variable is declared via computed property destructuring (e.g. `const { [KEY]: value } = obj`).
