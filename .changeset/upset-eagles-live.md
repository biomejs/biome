---
"@biomejs/biome": patch
---

Fixed [#11278](https://github.com/biomejs/biome/issues/11278): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) no longer incorrectly reports optional chaining on `RegExp.exec()` results, including patterns created with `new RegExp()`. Biome now infers the nullable `RegExpExecArray | null` return type, preserving necessary checks such as `new RegExp(pattern).exec(input)?.[1]`.
