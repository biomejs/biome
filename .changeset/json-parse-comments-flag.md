---
"@biomejs/biome": minor
---

Added `--json-parse-allow-comments` CLI flag to control whether comments are allowed in JSON files.

You can now enable or disable comment parsing in JSON files directly from the command line:

```shell
biome check --json-parse-allow-comments=true file.json
biome format --json-parse-allow-comments=true file.json
biome lint --json-parse-allow-comments=true file.json
biome ci --json-parse-allow-comments=true file.json
```
