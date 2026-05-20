---
"@biomejs/biome": patch
---

Added the new nursery rule [`noTypeofUndefined`](https://biomejs.dev/linter/rules/no-typeof-undefined/), which disallows comparing `typeof` results to `"undefined"` and prefers direct comparison with `undefined`.

The rule also supports an option to check unresolved or global identifiers when you want to forbid `typeof missingGlobal === "undefined"` style checks.
