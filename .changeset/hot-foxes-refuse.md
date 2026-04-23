---
"@biomejs/biome": patch
---

Added the new nursery rule [`noLoopFunc`](https://biomejs.dev/linter/rules/no-loop-func/). When enabled, it warns when a function declared inside a loop captures outer variables that can change across iterations.
