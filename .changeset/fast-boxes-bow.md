---
"@biomejs/biome": patch
---

Fixed [#11280](https://github.com/biomejs/biome/issues/11280): CSS formatting keeps comments inside functional pseudo-classes and pseudo-elements instead of moving them before the function name.

```diff
-:/* comment */ where(div) {}
+:where(/* comment */ div) {}
```
