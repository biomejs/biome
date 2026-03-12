---
"@biomejs/biome": patch
---

Fixed [#9210](https://github.com/biomejs/biome/issues/9210): [`useAnchorContent`](https://biomejs.dev/linter/rules/use-anchor-content/) no longer reports an accessibility error for Astro `Image` components inside links when they provide non-empty `alt` text.
