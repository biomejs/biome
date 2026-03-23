---
"@biomejs/biome": patch
---

Fixed [#7664](https://github.com/biomejs/biome/issues/7664): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports false positives for TypeScript namespace declarations that participate in declaration merging with an exported or used value declaration (`const`, `function`, or `class`) of the same name. The reverse direction is also handled: a value declaration merged with an exported namespace is no longer flagged.
