---
"@biomejs/biome": minor
---

Add `checkForEach` option to [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/) rule. This option allows checking `forEach` callbacks for unexpected return values. When `true`, the rule reports `forEach` callbacks that return a value since `forEach` ignores return values. Default: `false`, matching ESLint's `array-callback-return` rule behavior.
