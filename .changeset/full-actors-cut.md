---
"@biomejs/biome": patch
---

Updated `biome migrate eslint` to migrate `tailwindcss/enforces-shorthand` to [`useTailwindShorthandClasses`](https://biomejs.dev/linter/rules/use-tailwind-shorthand-classes/) when nursery rules are included, and `@html-eslint/no-target-blank` to [`noBlankTarget`](https://biomejs.dev/linter/rules/no-blank-target/). Corrected the reported reasons for unsupported legacy, deprecated, formatter-covered, and compiler-dependent rules.

