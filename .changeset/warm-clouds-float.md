---
"@biomejs/biome": patch
---

Added `allowImplicit` option to [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/). When set to `true`, callbacks for methods like `map` and `filter` can use `return;` to implicitly return `undefined`. This matches ESLint's `allowImplicit` option for `array-callback-return`.
