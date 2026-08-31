---
"@biomejs/biome": patch
---

Fixed [#11512](https://github.com/biomejs/biome/issues/11512), where [`style/noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) missed lower-specificity selectors after a later higher-specificity selector with the same tail selector.
