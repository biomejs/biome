---
"@biomejs/biome": patch
---

[#13384](https://github.com/biomejs/biome/issues/13384): Fixed linting of whole Svelte components so that template writes, store updates, directive names, and type references count against the declaration they actually use. [`noConstAssign`](https://biomejs.dev/linter/rules/no-const-assign/) now reports assignments in template expressions to constants declared in a script.
