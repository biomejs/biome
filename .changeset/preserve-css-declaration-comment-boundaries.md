---
"@biomejs/biome": patch
---

CSS declarations with comments before `:` or after `!important` now preserve Prettier-compatible spaces before `:` and `;`.

```diff
 .selector {
-  padding/* name */: 1px;
-  color: red !important /* note */;
+  padding/* name */ : 1px;
+  color: red !important /* note */ ;
 }
```
