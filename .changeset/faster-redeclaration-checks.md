---
"@biomejs/biome": patch
---

Improved the performance of the rules [`noRedeclare`](https://biomejs.dev/linter/rules/no-redeclare/) and [`noUnsafeDeclarationMerging`](https://biomejs.dev/linter/rules/no-unsafe-declaration-merging/). `biome lint` with Biome's default rules is now about 3% to 5% faster on large projects, and up to 11% faster on some TypeScript projects.
