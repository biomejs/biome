---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessSplice`](https://biomejs.dev/linter/rules/no-useless-splice/), which reports `splice()` calls that have a simpler alternative. For example, `array.splice(0, 1)` can be written as `array.shift()`.
