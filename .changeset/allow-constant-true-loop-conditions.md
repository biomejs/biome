---
"@biomejs/biome": patch
---

Fixed [#13320](https://github.com/biomejs/biome/issues/13320): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) no longer reports loops whose condition is the literal `true`, such as `while (true) {}`, `do {} while (true)`, and `for (; true;) {}`.
