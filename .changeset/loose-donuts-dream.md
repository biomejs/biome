---
"@biomejs/biome": patch
---

Improved nested ternary fixes ([#12808](https://github.com/biomejs/biome/issues/12808)) in [`useMathMinMax`](https://biomejs.dev/linter/rules/use-math-min-max/). The unsafe fix now accepts global `Math.min()` and `Math.max()` calls whose arguments are known to convert to numbers without side effects. Calls with objects or unknown arguments remain unfixed.
