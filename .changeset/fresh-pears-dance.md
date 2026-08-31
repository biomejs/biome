---
"@biomejs/biome": patch
---

Fixed [`#11537`](https://github.com/biomejs/biome/issues/11537): [`noShorthandPropertyOverrides`](https://biomejs.dev/linter/rules/no-shorthand-property-overrides/) now compares declarations only within the same block. The rule no longer reports `@supports` feature queries and correctly checks nested, `@keyframes`, and `@page` blocks.
