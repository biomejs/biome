---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessSpread`](https://biomejs.dev/linter/rules/no-useless-spread/), which reports spread syntax that has no effect. For example, `[first, ...[second]]` can be written as `[first, second]`, and `new Set([...items])` can be written as `new Set(items)`.
