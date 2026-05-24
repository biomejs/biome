---
"@biomejs/biome": patch
---

Fixed [#10447](https://github.com/biomejs/biome/issues/10447): The `noProcessEnv` rule now detects environment variable access through `env` imported from `node:process` or `process`, and `Bun.env` usage.
