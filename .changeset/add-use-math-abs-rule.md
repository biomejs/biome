---
"@biomejs/biome": patch
---

Added the nursery rule [`useMathAbs`](https://biomejs.dev/linter/rules/use-math-abs/), which reports manual absolute value calculations and range checks that can use `Math.abs()`. For example, `value < 0 ? -value : value` can be written as `Math.abs(value)`, and `value > limit || value < -limit` can be written as `Math.abs(value) > limit`.
