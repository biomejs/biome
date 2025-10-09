---
"@biomejs/biome": minor
---

The `formatWithErrors` option can now be set via CLI using the `--format-with-errors` flag.

This flag was previously only available in the configuration file. It allows formatting to proceed on files with syntax errors, which is useful during development when you want to auto-format code while fixing syntax issues.

#### Example

```shell
biome format --format-with-errors=true --write file.js
```

