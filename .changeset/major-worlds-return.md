---
"@biomejs/biome": patch
---

Fixed [#12726](https://github.com/biomejs/biome/issues/12726): [`useMathMinMax`](https://biomejs.dev/linter/rules/use-math-min-max/) now recognizes directly listed nonnumeric elements in array reductions, such as `["foo", "bar"].reduce((a, b) => a > b ? a : b)`. This also covers Date values and local constant arrays used only by reductions, including parentheses and TypeScript assertions. Spread elements and other unclassified inputs retain their existing behavior.
