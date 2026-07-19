---
"@biomejs/biome": patch
---

Fixed [#10963](https://github.com/biomejs/biome/issues/10963): Biome no longer panics when a type-aware rule such as [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) checks a call to a function with multiple call signatures imported from another module.
