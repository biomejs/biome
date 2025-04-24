---
"@biomejs/biome": major
---

Biome doesn't ignore `node_modules` anymore *by default*. If the folder wasn't ignored already, run the command `biome migrate` and Biome will add it to the configuration.

The migration doesn't check if `node_modules` is already ignored in the `.gitignore` file. If the entry is already available, delete `**/node_modules/**` entry from the `includes` configuration.
