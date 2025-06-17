---
"@biomejs/biome": minor
---

Fixed [#3574](https://github.com/biomejs/biome/issues/3574): `noUnusedImports` now reports empty named imports and suggests their removal.

The rule now suggests the removal of empty named imports such as:

```diff
- import {} from "mod";
```
