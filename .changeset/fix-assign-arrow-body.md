---
"@biomejs/biome": patch
---

Fixed [#8790](https://github.com/biomejs/biome/issues/8790): The [`noAssignInExpressions`](https://biomejs.dev/linter/rules/no-assign-in-expressions/) rule no longer reports a false positive when an assignment is used as the expression body of an arrow function (e.g., `const f = b => a += b`).
