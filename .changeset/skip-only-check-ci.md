---
"@biomejs/biome": minor
---

Added `--only` and `--skip` options to `biome check` and `biome ci`, covering both lint diagnostics and assist actions. Biome now lets you run or exclude specific lint rules, assist actions, group or rules and actions, or domains when running these commands.

Examples:

```shell
biome check --only=suspicious/noDebugger src/**/*.js
biome ci --skip=project src/**
```
