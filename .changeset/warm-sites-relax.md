---
"@biomejs/biome": patch
---

Fixed [#8573](https://github.com/biomejs/biome/issues/8573): own-line comments before binary operators stay above the operator when `javascript.formatter.operatorLinebreak` is `"before"`.

```diff
 foo
-  || // comment
-  bar;
+  // comment
+  || bar;
```
