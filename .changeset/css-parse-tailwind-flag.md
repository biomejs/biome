---
"@biomejs/biome": minor
---

Added `--css-parse-tailwind-directives` CLI flag to control whether Tailwind CSS 4.0 directives and functions are enabled.

You can now enable or disable Tailwind CSS 4.0 directive parsing directly from the command line:

```shell
biome check --css-parse-tailwind-directives=true file.css
biome format --css-parse-tailwind-directives=true file.css
biome lint --css-parse-tailwind-directives=true file.css
biome ci --css-parse-tailwind-directives=true file.css
```
