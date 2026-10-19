---
"@biomejs/biome": patch
---

Improved the performance of commands that run with `--use-server`, such as `biome check --use-server`. The Biome daemon now processes the files of a command on all available CPU cores, instead of at most four files at a time.
