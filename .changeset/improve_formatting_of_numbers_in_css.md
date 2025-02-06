---
"@biomejs/biome": patch
---

Fixes [#5031](https://github.com/biomejs/biome/issues/5031). Improve CSS numeric formatting with two changes:

- Adds leading zeros to value with unit `.5em` => `0.5em`
- Removes trailing zeros. `1.0` => `1`
