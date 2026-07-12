---
"@biomejs/biome": patch
---

Added a new nursery rule [`useConsistentReturn`](https://biomejs.dev/linter/rules/use-consistent-return/), which requires a function's `return` statements to either always or never specify a value. It reports a bare `return;` — or a code path that implicitly finishes without returning — when the function returns a value on another path.
