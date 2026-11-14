---
"@biomejs/biome": patch
---

Fixed [#13125](https://github.com/biomejs/biome/issues/13125): [`noUselessThisAlias`](https://biomejs.dev/linter/rules/no-useless-this-alias/) no longer reports aliases used in instance and static class field initializers, where replacing the alias with `this` changes which object is accessed.
