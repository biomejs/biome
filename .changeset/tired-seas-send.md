---
"@biomejs/biome": patch
---

Fixed [#13053](https://github.com/biomejs/biome/issues/13053): Biome now reports missing CLI paths as file-system errors without the `INTERNAL` tag or advice that the path was ignored by configuration. File-system diagnostics now use specific `fs/*` categories, with underlying I/O errors categorized as `fs/io`.
