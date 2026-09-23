---
"@biomejs/biome": patch
---

Added the nursery rule [`noMeaninglessVoidOperator`](https://biomejs.dev/linter/rules/no-meaningless-void-operator/), which reports unnecessary uses of `void`, such as `void log()` when `log` returns `void`. The rule allows discarded call results, thenables, `void 0`, and calls returning `never`.
