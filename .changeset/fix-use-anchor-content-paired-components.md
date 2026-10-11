---
"@biomejs/biome": patch
---

Fixed [#11872](https://github.com/biomejs/biome/issues/11872): anchors in Vue, Svelte, and Astro files no longer report PascalCase components written with an explicit closing tag as empty. The rule now treats both spellings the same, so an anchor whose only child is a paired component, for example `<a><Icon></Icon></a>`, is not reported anymore. Anchors in plain HTML files still require accessible content.
