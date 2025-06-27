---
"@biomejs/biome": patch
---

Fixed [#6529](https://github.com/biomejs/biome/issues/6529), where the Biome Language Server would emit an error when the user would open a file that isn't part of its workspace (`node_modules` or external files).
Now the language server doesn't emit any errors and it exits gracefully.
