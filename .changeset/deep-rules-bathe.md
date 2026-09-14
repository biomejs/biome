---
"@biomejs/biome": patch
---

Fixed [#11782](https://github.com/biomejs/biome/issues/11782): [`noUndeclaredCustomProperties`](https://biomejs.dev/linter/rules/no-undeclared-custom-properties/) could hang while checking stylesheets imported by JavaScript modules with many shared dependencies.
