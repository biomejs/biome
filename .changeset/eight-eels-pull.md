---
"@biomejs/biome": minor
---

Fixed [#6438](https://github.com/biomejs/biome/issues/6438) and [#3682](https://github.com/biomejs/biome/issues/3682): Biome now respects `jsxFactory` and `jsxFragmentFactory` settings from `tsconfig.json` when using the classic JSX runtime, preventing false positive [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) errors for custom JSX libraries like Preact.
