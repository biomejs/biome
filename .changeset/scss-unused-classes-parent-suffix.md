---
"@biomejs/biome": patch
---

Fixed [`noUnusedClasses`](https://biomejs.dev/linter/rules/no-unused-classes/) missing classes built from an SCSS parent selector suffix. The rule now checks `&__element` in `.block { &__element {} }` as the class `block__element`.
