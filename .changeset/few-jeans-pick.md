---
"@biomejs/biome": patch
---

Added a new nursery rule [`useDomQuerySelector`](https://biomejs.dev/linter/rules/use-dom-query-selector/) that prefers `querySelector()` and `querySelectorAll()` over older DOM query methods such as `getElementById()` and `getElementsByClassName()`.
