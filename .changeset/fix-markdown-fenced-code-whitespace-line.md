---
"@biomejs/biome": patch
---

Fixed [#13466](https://github.com/biomejs/biome/issues/13466): the Markdown formatter now preserves whitespace-only lines inside fenced code blocks in list items without shifting the indentation of the following line.

````diff
 1. Item:

    ```shell
     echo a
-       echo b
+
+    echo b
    ```
````
