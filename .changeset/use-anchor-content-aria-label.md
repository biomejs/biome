---
"@biomejs/biome": patch
---

Fixed [#11913](https://github.com/biomejs/biome/issues/11913): [`useAnchorContent`](https://biomejs.dev/linter/rules/use-anchor-content/) no longer reports `a` elements that have a non-empty `aria-label`, because that attribute already provides an accessible name for the link. An `aria-label` that is empty or only whitespace is still reported.
