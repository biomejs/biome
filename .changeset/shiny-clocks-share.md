---
"@biomejs/biome": patch
---

Fixed [#11867](https://github.com/biomejs/biome/issues/11867): [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/) incorrectly reported Vue slot props declared with `v-slot` or its `#` shorthand, including destructured props.
