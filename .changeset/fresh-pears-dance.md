---
"@biomejs/biome": patch
---

Fixed [`noShorthandPropertyOverrides`](https://biomejs.dev/linter/rules/no-shorthand-property-overrides/) to compare declarations only within the same block. The rule no longer reports `@supports` feature queries and correctly checks nested, `@keyframes`, and `@page` blocks.
