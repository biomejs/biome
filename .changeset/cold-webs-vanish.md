---
"@biomejs/biome": patch
---

Fixed a bug where logs were discarded (the kind from `--log-level=info` etc.). This is a regression introduced after an internal refactor that wasn't adequately tested.
