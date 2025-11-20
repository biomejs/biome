---
"@biomejs/biome": patch
---

Fixed [#8145](https://github.com/biomejs/biome/issues/8145): handling of large hex literals, which previously caused both false positives and false negatives.

This affects [`noPrecisionLoss`](https://biomejs.dev/linter/rules/no-precision-loss/) and [`noConstantMathMinMaxClamp`](https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp/).
