---
"@biomejs/biome": patch
---

[#13384](https://github.com/biomejs/biome/issues/13384): Fixed false lint diagnostics and unsafe suggestions in whole Svelte components with unsupported scripts or malformed expressions. Biome still reports problems in scopes that the unreadable code cannot affect.
