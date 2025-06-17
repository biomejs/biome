---
"@biomejs/biome": minor
---

The CLI options `--only` and `--skip` now accept rule and action names without prefixing the group name.

Previously `--only=noDebugger` was rejected.
You had to add the group name: `--only=suspicious/noDebugger`.
