---
"@biomejs/biome": patch
---

Fixed [#13038](https://github.com/biomejs/biome/issues/13038): Biome now aligns wrapped task-list paragraphs with the text following the checkbox.

```diff
 - [ ] Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc varius
-  malesuada eros, faucibus dapibus odio rhoncus in.
+      malesuada eros, faucibus dapibus odio rhoncus in.
```

