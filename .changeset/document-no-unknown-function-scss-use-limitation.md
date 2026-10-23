---
"@biomejs/biome": patch
---

Documented the `SCSS limitations` of [`noUnknownFunction`](https://biomejs.dev/linter/rules/no-unknown-function/): functions made available from other files, whether by legacy SCSS `@import` or by `@use`, are not resolved, and the `ignore` option is the way to handle them. Tracked in [#10800](https://github.com/biomejs/biome/issues/10800).
