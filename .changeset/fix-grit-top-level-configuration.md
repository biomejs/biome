---
"@biomejs/biome": patch
---

Fixed [#12862](https://github.com/biomejs/biome/issues/12862): Biome now applies the options under the top-level `grit` configuration key. Previously, options such as `grit.formatter.lineWidth` and `grit.formatter.enabled` were ignored, and Grit files were formatted with the global `formatter` settings.
