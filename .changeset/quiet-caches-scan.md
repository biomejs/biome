---
"@biomejs/biome": patch
---

Fixed nested `.gitignore` files being ignored when the project lives inside a directory such as `.cache` that Biome's scanner skips by default. Those directory names now only apply inside the project.
