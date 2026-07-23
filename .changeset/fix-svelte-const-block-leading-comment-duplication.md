---
"@biomejs/biome": patch
---

Fixed an issue where the HTML formatter would duplicate a comment placed directly before a Svelte `{@const ...}` or `{@debug ...}` block. The duplication compounded on every subsequent `--write`, causing the file to grow exponentially.
