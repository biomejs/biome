---
"@biomejs/biome": patch
---

Fixed a parse error in HTML and Svelte files when an attribute named `?` came right before the end of a tag. `<div a ?>` now has the attributes `a` and `?`, as it already did in Astro and Vue files.
