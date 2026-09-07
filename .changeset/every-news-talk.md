---
"@biomejs/biome": patch
---

Fixed [#11653](https://github.com/biomejs/biome/issues/11653): Astro template suppression comments (`{/* biome-ignore lint: reason */}`) now suppress matching HTML lint diagnostics on the following line when full HTML support is enabled.
