---
"@biomejs/biome": patch
---

Fixed [#6577](https://github.com/biomejs/biome/issues/6577): [`noUselessUndefined`](https://biomejs.dev/linter/rules/no-useless-undefined/) no longer reports `() => undefined` in arrow function expression bodies. Previously, the rule would flag this pattern and suggest replacing it with `() => {}`, which conflicts with the `noEmptyBlockStatements` rule.
