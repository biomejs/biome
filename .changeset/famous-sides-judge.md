---
"@biomejs/biome": patch
---

Fixed [#12374](https://github.com/biomejs/biome/issues/12374): `biome migrate eslint` now preserves `eqeqeq`'s null comparison options when configuring [`noDoubleEquals`](https://biomejs.dev/linter/rules/no-double-equals/). Migrating the default ESLint configuration now sets `ignoreNull: false`, so comparisons such as `x == null` remain reported.
