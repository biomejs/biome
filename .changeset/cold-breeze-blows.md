---
"@biomejs/biome": minor
---

Fixed [#3401](https://github.com/biomejs/biome/issues/3401): `noUnusedImports` now keeps comments separated from the import with a blank line.

For example:

```diff
  // Orphan comment

- // Header comment
- import {} from "mod";
```
