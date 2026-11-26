---
"@biomejs/biome": patch
---

Fixed [#13466](https://github.com/biomejs/biome/issues/13466): the Markdown formatter no longer merges a whitespace-only line in a fenced code block inside a list item with the next line. The line is now printed as an empty line, and the next line keeps its indentation.

````diff
  1. Item:

     ```shell
      echo a
-        echo b
+
+     echo b
     ```
````
