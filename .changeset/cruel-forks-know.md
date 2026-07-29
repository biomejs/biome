---
"@biomejs/biome": patch
---

Fixed [#11098](https://github.com/biomejs/biome/issues/11098): The HTML formatter now preserves the configured trailing newline when a file ends with a comment.

```diff
-<!-- trailing comment -->
\ No newline at end of file
+<!-- trailing comment -->
```
