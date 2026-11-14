---
"@biomejs/biome": patch
---

Fixed [#13108](https://github.com/biomejs/biome/issues/13108): [`noUnusedExpressions`](https://biomejs.dev/linter/rules/no-unused-expressions/) no longer reports `satisfies` expressions used as statements, such as `shape satisfies never;` for exhaustiveness checks.
