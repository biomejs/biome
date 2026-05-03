---
"@biomejs/biome": patch
---

Fixed [`useAnchorContent`](https://biomejs.dev/linter/rules/use-anchor-content/) false positive for `<a>` elements used as render prop values (e.g. `render={<a href="..." />}`), a pattern where the receiving component renders its children inside the anchor element.
