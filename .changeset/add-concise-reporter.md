---
"@biomejs/biome": minor
---

Added a new reporter called `concise`. When `--reporter=concise` is passed the commands `format`, `lint`, `check` and `ci`, the diagnostics are printed in a compact manner:

```
! index.ts:2:10: lint/correctness/noUnusedImports: Several of these imports are unused.
! main.ts:9:7: lint/correctness/noUnusedVariables: This variable f is unused.
× index.ts:8:5: lint/suspicious/noImplicitAnyLet: This variable implicitly has the any type.
× main.ts:2:10: lint/suspicious/noRedeclare: Shouldn't redeclare 'z'. Consider to delete it or rename it.
```
