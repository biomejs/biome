---
"@biomejs/biome": patch
---

Fixed [#6003](https://github.com/biomejs/biome/issues/6003): `noUselessUndefinedInitialization` no longer reports exported variables initialized to `undefined`. In Svelte, this pattern is used to declare optional component props.
