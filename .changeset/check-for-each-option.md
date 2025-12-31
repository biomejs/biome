---
"@biomejs/biome": patch
---

Added `checkForEach` option to the [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/) rule. When set to `true`, the rule will also check that `forEach` callbacks do not return a value. This matches the behavior of ESLint's `array-callback-return` rule with the same option. The default is `false`, which means `forEach` callbacks are not checked by default.
