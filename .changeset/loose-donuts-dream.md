---
"@biomejs/biome": patch
---

Fixed [#12808](https://github.com/biomejs/biome/issues/12808): [`useMathMinMax`](https://biomejs.dev/linter/rules/use-math-min-max/) can now apply its unsafe fix when an operand is a global `Math.min()` or `Math.max()` call with simple arguments. This lets nested ternary fixes continue after the inner comparison has been rewritten.
