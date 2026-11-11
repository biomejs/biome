---
"@biomejs/biome": patch
---

Fixed [`noDuplicateSelectors`](https://biomejs.dev/linter/rules/no-duplicate-selectors/) missing some duplicates. The rule now reports these selectors:

```css
.c:is(.a .b) {}
:is(.a .b).c {}

[data-progress="50%"] {}
[data-progress="50%"] {}
```
