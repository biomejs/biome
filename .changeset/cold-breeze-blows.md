---
"@biomejs/biome": minor
---

`noUnusedImports` now keeps comments separated from the import with a blank line ([#3401](https://github.com/biomejs/biome/issues/3401)).

Here is an example:

```diff
  // Orphan comment

- // Header comment
- import {} from "mod";
```
