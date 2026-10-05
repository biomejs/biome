---
"@biomejs/biome": patch
---

Fixed [#10283](https://github.com/biomejs/biome/issues/10283): [`noUnresolvedImports`](https://biomejs.dev/linter/rules/no-unresolved-imports/) now recognizes variable members of namespaces exported with `export =`. Merged namespace declarations retain their visible members, while private and nested members are not exposed as direct exports.
