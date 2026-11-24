---
"@biomejs/biome": patch
---

[#13384](https://github.com/biomejs/biome/issues/13384): Fixed name resolution when linting whole Svelte components. Each items and indices, await branch bindings, snippet parameters, and template declarations now shadow outer variables only within their own scopes.
