---
"@biomejs/biome": patch
---

Fixed [#13466](https://github.com/biomejs/biome/issues/13466): the Markdown formatter no longer removes a whitespace-only line from a fenced code block that is inside a list item or indented, and no longer adds indentation to the line after it.

```diff
 1. Item:
 
    ```shell
     echo a
-       echo b
+
+    echo b
    ```
```
