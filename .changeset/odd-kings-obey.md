---
"@biomejs/biome": patch
---

Fixed [#8145](https://github.com/biomejs/biome/issues/8145): handling of large hex literals, which previously caused both false positives and false negatives.

This affects [`no-precision-loss`](https://biomejs.dev/linter/rules/no-precision-loss/) and [`no-constant-math-min-max-clamp`](https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp/).
