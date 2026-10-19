---
"@biomejs/biome": patch
---

Fixed commands that run with `--use-server` reporting `cancelled because of pending write` for some files instead of checking them. Biome now sends a cancelled request again, so the command reports the same diagnostics as a run without `--use-server`.
