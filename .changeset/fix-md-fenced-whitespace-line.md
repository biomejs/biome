---
"@biomejs/biome": patch
---

Fixed [#13466](https://github.com/biomejs/biome/issues/13466): The Markdown formatter no longer drops a whitespace-only line inside a fenced code block within a list item or misindents the following line.

```diff
  1. Item:

     ```shell
      echo a
-        echo b
+
+     echo b
     ```
```
