---
"@biomejs/biome": patch
---

Added the recommended nursery rule [`noArgumentsCallerOrCallee`](https://biomejs.dev/linter/rules/no-arguments-caller-or-callee/), which disallows `arguments.caller` and `arguments.callee`. For example, `arguments.callee()` now produces a diagnostic; refer to the function by name for recursion.
