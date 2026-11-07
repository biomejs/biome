---
"@biomejs/biome": patch
---

Fixed [#12413](https://github.com/biomejs/biome/issues/12413) and [#12414](https://github.com/biomejs/biome/issues/12414): [`useSortedProperties`](https://biomejs.dev/assist/actions/use-sorted-properties/) now sorts consecutive groups of properties independently, preserving the positions of nested rules, at-rules, and SCSS variable declarations.
