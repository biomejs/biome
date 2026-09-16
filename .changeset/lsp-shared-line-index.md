---
"@biomejs/biome": patch
---

Fixed [#11767](https://github.com/biomejs/biome/issues/11767): The LSP no longer returns corrupted edits when the same file is open in two clients with different contents. Positions are now computed from the content the workspace holds, instead of from a line index cached per client.
