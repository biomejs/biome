---
"@biomejs/biome": patch
---

Fixed [#10265](https://github.com/biomejs/biome/issues/10265): Svelte function bindings such as `bind:value={get, set}` are now parsed more precisely, so [`noCommaOperator`](https://biomejs.dev/linter/rules/no-comma-operator/) won't emit false positives for that syntax anymore.
