---
"@biomejs/biome": patch
---

Fixed [#7677](https://github.com/biomejs/biome/issues/7677): `--linter-enabled=false` now applies to nested monorepo projects that use `"root": false` without `"extends": "//"`.
