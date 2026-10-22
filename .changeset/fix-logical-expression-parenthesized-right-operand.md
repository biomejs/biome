---
"@biomejs/biome": patch
---

Fixed [#ISSUE_NUMBER](https://github.com/biomejs/biome/issues/ISSUE_NUMBER): the formatter now flattens a parenthesized right operand with the same logical operator and more than two operands into the surrounding chain on the first pass. Previously, the operands of the removed group stayed together and the next format changed the output again.

```diff
  return (
    aaaaaaaaaaaaaaaaaaaa &&
    bbbbbbbbbbbbbbbbbbbbbbb &&
-   cccccccccccccccccccc && dddddddddddddddddddddd &&
+   cccccccccccccccccccc &&
+   dddddddddddddddddddddd &&
    eeeeeeeeeeeeeeeeeeeeeeeee
  );
```
