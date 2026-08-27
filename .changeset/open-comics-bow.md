---
"@biomejs/biome": patch
---

Fixed [#11512](https://github.com/biomejs/biome/issues/11512): [`noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) now compares a selector against the highest specificity seen among preceding selectors that share its tail selector, rather than only the first one seen. A descending pair is now reported when a lower-specificity selector follows a higher-specificity one that was not the first occurrence of that tail.
