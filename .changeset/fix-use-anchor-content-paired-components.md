---
"@biomejs/biome": patch
---

Fixed [#11872](https://github.com/biomejs/biome/issues/11872): [`useAnchorContent`](https://biomejs.dev/linter/rules/use-anchor-content/) no longer reports PascalCase components written with an explicit closing tag as empty anchors in Vue, Svelte, and Astro files. The anchor is not reported when its only child is a paired component, for example `<a><Icon></Icon></a>`, matching the self-closing spelling. Anchors in plain HTML files still require accessible content.
