---
"@biomejs/biome": patch
---

[`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) no longer reports false positives when a union return type's `boolean` variant is covered by both `true` and `false` returns.
