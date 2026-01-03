---
"@biomejs/biome": minor
---

Fixed [#8024](https://github.com/biomejs/biome/issues/8024). The rule [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/) now supports a `checkForEach` option. When set to `false`, the rule will skip checking for `forEach()` callbacks for returning values.
