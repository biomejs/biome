---
"@biomejs/biome": patch
---

Added the nursery rule [`noJsRestrictedProperties`](https://biomejs.dev/linter/rules/no-js-restricted-properties/), which ports ESLint's `no-restricted-properties` rule. Biome now flags restricted member access and object destructuring, and `biome migrate eslint` preserves the rule's options.
