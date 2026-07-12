---
"@biomejs/biome": patch
---

Added a new nursery rule [`useConsistentErrorSubclassing`](https://biomejs.dev/linter/rules/use-consistent-error-subclassing/), which reports a class extending a built-in error that does not consistently set `this.name` or whose name does not end in `Error`.
