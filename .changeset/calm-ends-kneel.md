---
"@biomejs/biome": patch
---

The `biome format` command now correctly handles the `--skip-errors` option, allowing it to skip files with syntax errors and continue formatting the remaining valid files.
When this option is used, skipped syntax errors are reported as information, since the user is already aware of them.