---
"@biomejs/biome": patch
---

The HTML formatter now preserves meaningful blank lines in HTML, including spacing after elements with trailing spaces and blank lines between comment groups.

```diff
 <div>
   <!-- first group -->
+
   <!-- second group -->
 </div>
```
