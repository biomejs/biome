---
"@biomejs/biome": minor
---

Added the `allowImplicit` option to [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/). When enabled, callbacks can use `return;` to implicitly return `undefined`, matching ESLint's `array-callback-return` rule. Fixed [#9445](https://github.com/biomejs/biome/issues/9445).
