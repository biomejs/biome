---
"@biomejs/biome": patch
---

Removed the `attributes` and `functions` options from the nursery rule [`noTailwindArbitraryValue`](https://biomejs.dev/linter/rules/no-tailwind-arbitrary-value/). The rule now uses the same Tailwind detection as [`useTailwindShorthandClasses`](https://biomejs.dev/linter/rules/use-tailwind-shorthand-classes/).
