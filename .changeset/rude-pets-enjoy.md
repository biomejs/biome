---
"@biomejs/biome": patch
---

Fixed [#7516](https://github.com/biomejs/biome/issues/7516): `noUnusedImports` no longer reports a false positive when a local variable shadows an imported type namespace that is still used in a type annotation.
