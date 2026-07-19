---
"@biomejs/biome": patch
---

Improved the performance of [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) by skipping type inference for expressions that can't trigger the rule.
