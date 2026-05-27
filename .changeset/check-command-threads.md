---
"@biomejs/biome": minor
---

Added `--threads` and `BIOME_THREADS` support to `biome check`. This lets local workflows such as pre-commit hooks limit Biome's worker count while still using `check --write` to apply safe fixes.
