---
"@biomejs/biome": patch
---

Fixed [#12555](https://github.com/biomejs/biome/issues/12555): the default reporter now prints the command summary, the `--max-diagnostics` warning, and the skipped-fixes notice to stderr instead of stdout. Diagnostics already go to stderr, so the summary can no longer get mixed into the middle of a diagnostic when a tool reads the two streams separately.
