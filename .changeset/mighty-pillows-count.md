---
"@biomejs/biome": patch
---

Fixed assist diagnostics being invisible when using `--diagnostic-level=error`. Enforced assist violations (e.g. `useSortedKeys`) were filtered out before being promoted to errors, causing `biome check` to incorrectly return success.
