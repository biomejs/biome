---
"@biomejs/biome": patch
---

Fixed [#8488](https://github.com/biomejs/biome/issues/8488): Relative plugin paths are now resolved from the configuration file directory, including when configurations are merged (e.g. `extends: "//"`).
