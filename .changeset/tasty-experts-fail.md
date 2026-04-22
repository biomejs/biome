---
"@biomejs/biome": patch
---

Fixed [#9482](https://github.com/biomejs/biome/issues/9482): [`noShadow`](https://biomejs.dev/linter/rules/no-shadow/) no longer flags parameter names inside function type annotations as shadowing outer variables. Parameters in constructs like `(options: unknown) => void` only exist within the type scope and do not shadow runtime bindings.
