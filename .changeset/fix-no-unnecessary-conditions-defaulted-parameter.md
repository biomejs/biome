---
"@biomejs/biome": patch
---

Fixed [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) missing unnecessary checks on a parameter whose default value can't be `undefined`, such as `parameter ?? "y"` for `parameter: string | undefined = "x"`.
