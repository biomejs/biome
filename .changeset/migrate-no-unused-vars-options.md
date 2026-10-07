---
"@biomejs/biome": minor
---

Added migration of the `ignoreRestSiblings` and `ignoreUsingDeclarations` options of `no-unused-vars`, `@typescript-eslint/no-unused-vars`, and `unused-imports/no-unused-vars` to [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) in `biome migrate eslint`. Options that the ESLint configuration omits are set to ESLint's defaults, so the migrated rule reports the same variables. For example, `"no-unused-vars": "error"` now migrates `ignoreRestSiblings` as `false`.
