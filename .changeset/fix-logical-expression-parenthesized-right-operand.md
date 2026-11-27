---
"@biomejs/biome": patch
---

Fixed the formatter output changing on a second run for logical chains. The formatter now flattens a parenthesized right operand with the same logical operator and more than two operands into the surrounding chain on the first pass. Previously, the operands of the removed group stayed together and the next format changed the output again.

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
