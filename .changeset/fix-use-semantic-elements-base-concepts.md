---
"@biomejs/biome": patch
---

Fixed [#9245](https://github.com/biomejs/biome/issues/9245): the [`useSemanticElements`](https://biomejs.dev/linter/rules/use-semantic-elements/) rule no longer suggests incorrect HTML elements for roles that only have `relatedConcepts` in the ARIA spec. The rule now only suggests replacements for roles with `baseConcepts`, which are direct semantic equivalents. For example, `<div role="status">` no longer incorrectly suggests `<output>`.
