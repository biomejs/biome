---
"@biomejs/biome": minor
---

Added `--json-parse-allow-trailing-commas` CLI flag to control whether trailing commas are allowed in JSON files.

You can now enable or disable trailing comma parsing in JSON files directly from the command line:

```shell
biome check --json-parse-allow-trailing-commas=true file.json
biome format --json-parse-allow-trailing-commas=true file.json
biome lint --json-parse-allow-trailing-commas=true file.json
biome ci --json-parse-allow-trailing-commas=true file.json
```
