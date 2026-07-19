---
"@biomejs/biome": patch
---

Improved the performance of [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/). The rule queries every expression in a file, then ran full type inference on each one before checking whether it was even in a syntactic position the rule cares about (a conditional/loop test, a spread element, or a call/new argument). That cheap, purely syntactic check now runs first, skipping type inference entirely for expressions that cannot match regardless of their type. On a large real-world project, this rule alone accounted for the majority of total lint time; with this change its own cost dropped roughly 4-5x, with no change in reported diagnostics.
