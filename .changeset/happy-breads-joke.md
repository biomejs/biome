---
"@biomejs/biome": patch
---

Fixed [#12956](https://github.com/biomejs/biome/issues/12956): [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/) now respects Astro server and client scopes, `define:vars`, and type/value namespaces. [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/), [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/), and [`noUnusedFunctionParameters`](https://biomejs.dev/linter/rules/no-unused-function-parameters/) now follow resolved Astro symbols and treat write-only template references as unused.
