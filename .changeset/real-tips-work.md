---
"@biomejs/biome": patch
---

Expanded [`useLogicalProperties`](https://biomejs.dev/linter/rules/use-logical-properties) to report multi-value physical shorthands and logical longhand pairs that can be combined. Dynamic shorthands such as `padding: var(--spacing) 2px` are also reported without an automatic fix because the variable may supply multiple values.

---
