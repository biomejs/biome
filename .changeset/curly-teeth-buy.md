---
"@biomejs/biome": patch
---

Fixed [#12040](https://github.com/biomejs/biome/issues/12040): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) no longer reports a condition on the result of a generic call that passes `undefined` or `null` as the default value, such as Vue's `inject(key, undefined)`.
