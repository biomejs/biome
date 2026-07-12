---
"@biomejs/biome": patch
---

Added a new nursery rule [`noUselessLengthCheck`](https://biomejs.dev/linter/rules/no-useless-length-check/), which reports a `.length` check made redundant by an adjacent `.every()` or `.some()` call (e.g. `array.length === 0 || array.every(fn)`).
