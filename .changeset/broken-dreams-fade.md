---
"@biomejs/biome": minor
---

`noUnusedImports` now reports empty named imports and suggests its removal ([#3574](https://github.com/biomejs/biome/issues/3574)).

The rule now suggests the removal of empty named imports such as:

```diff
- import {} from "mod";
```
