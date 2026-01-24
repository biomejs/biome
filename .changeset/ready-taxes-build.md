---
"@biomejs/biome": patch
---

**Major Breaking Changes to the HTML formatter only**

The HTML formatter (which is still experimental) has been completely overhauled from the ground up to more closely resemble Prettier's formatting. If you have opted in to the HTML formatter, you may see large formatting diffs for your HTML, Vue, Svelte, and Astro files.

This overhaul fixes several issues ([#5150](https://github.com/biomejs/biome/issues/5150), [#6625](https://github.com/biomejs/biome/issues/6623), [#8437](https://github.com/biomejs/biome/issues/8437)) around whitespace sensitivity that were causing incorrect formatting in certain scenarios that were difficult or impossible to fully address before.
