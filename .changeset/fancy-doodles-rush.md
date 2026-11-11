---
"@biomejs/biome": patch
---

Fixed [#10607](https://github.com/biomejs/biome/issues/10607): [`noUndeclaredDependencies`](https://biomejs.dev/linter/rules/no-undeclared-dependencies/) no longer reports imports resolved through TypeScript's `compilerOptions.paths` mappings.
