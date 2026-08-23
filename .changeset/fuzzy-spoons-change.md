---
"@biomejs/biome": patch
---

Fixed [#11465](https://github.com/biomejs/biome/issues/11465): blockquoted fenced code blocks no longer gain indentation on every formatting pass.

```diff
 > ```js
-  > var re = /../g;
+> var re = /../g;
 > ```
```
