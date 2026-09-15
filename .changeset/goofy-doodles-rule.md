---
"@biomejs/biome": patch
---

Fixed [#11786](https://github.com/biomejs/biome/issues/11786): [`useAnchorContent`](https://biomejs.dev/linter/rules/use-anchor-content/) now reports anchors without accessible content in HTML, Astro, Vue, and Svelte even when they have an `aria-label`, `aria-labelledby`, or `title` attribute, matching JSX behavior.
