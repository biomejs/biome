---
"@biomejs/biome": patch
---

Fixed [#6419](https://github.com/biomejs/biome/issues/6419), a regression where stdin mode would create a temporary new file instead of using the one provided by the user. This was an intended regression.

Now Biome will use the file path passed via `--std-file-path`, and apply the configuration that matches it.
