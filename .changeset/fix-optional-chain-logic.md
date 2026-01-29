---
"@biomejs/biome": patch
---

Fixed [#8577](https://github.com/biomejs/biome/issues/8577): `useSimplifiedLogicExpression` no longer incorrectly suggests removing `|| false` when the left-hand side contains optional chaining. Previously, the rule would suggest simplifying `account?.test || false` to `account?.test`, which changes the semantics (returning `undefined` instead of `false` when `account` is `undefined`).
