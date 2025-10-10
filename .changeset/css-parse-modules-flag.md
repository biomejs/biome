---
"@biomejs/biome": minor
---

Added `--css-parse-css-modules` CLI flag to control whether CSS Modules syntax is enabled.

You can now enable or disable CSS Modules parsing directly from the command line:

```shell
biome check --css-parse-css-modules=true file.module.css
biome format --css-parse-css-modules=true file.module.css
biome lint --css-parse-css-modules=true file.module.css
biome ci --css-parse-css-modules=true file.module.css
```
