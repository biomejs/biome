---
"@biomejs/biome": patch
---

Improved the performance of CSS linting. Rules that compare selectors, such as [`noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) and [`noDuplicateSelectors`](https://biomejs.dev/linter/rules/no-duplicate-selectors/), now make fewer memory allocations.
