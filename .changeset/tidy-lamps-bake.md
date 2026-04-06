---
"@biomejs/biome": patch
---

Added the nursery rule [`noBaseToString`](https://biomejs.dev/linter/rules/no-base-to-string/), which reports stringification sites that fall back to Object's default `"[object Object]"` formatting. The rule also supports the `ignoredTypeNames` option and `biome migrate eslint` now preserves that option from `@typescript-eslint/no-base-to-string`.
