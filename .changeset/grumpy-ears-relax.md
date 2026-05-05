---
"@biomejs/biome": patch
---

Fixed [#10261](https://github.com/biomejs/biome/issues/10261): [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/) now keeps functional class values containing spaces, such as `border-oklch(0.922 0 0)`, as a single token when sorting utility classes.
