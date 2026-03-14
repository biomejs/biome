---
"@biomejs/biome": patch
---

Fixed [#6316](https://github.com/biomejs/biome/issues/6316): Biome now resolves Svelte `$store` references to the underlying `store` binding in semantic analysis, preventing false `noUndeclaredVariables` diagnostics when the store is declared.
