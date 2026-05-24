---
"@biomejs/biome": patch
---

Fixed [#10411](https://github.com/biomejs/biome/issues/10411): [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) no longer causes a stack overflow when a nested function returns an object with shorthand properties that shadow destructured variables from an outer scope.
