---
"@biomejs/biome": patch
---

Fixed [#11017](https://github.com/biomejs/biome/issues/11017): [`noUselessUndefined`](https://biomejs.dev/linter/rules/no-useless-undefined/) no longer reports `return undefined` when the enclosing function has a return type annotation other than `undefined` or `void`.
