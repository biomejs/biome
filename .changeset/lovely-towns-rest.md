---
"@biomejs/biome": patch
---

Improved the performance of [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) by skipping type inference for assignment statements, which are always considered handled.
