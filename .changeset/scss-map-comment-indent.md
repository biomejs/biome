---
"@biomejs/biome": patch
---

Fixed [#13072](https://github.com/biomejs/biome/issues/13072): a comment before a SCSS map entry whose value is a parenthesized group no longer indents that group one level too deep. This matches Prettier 3.10 behavior now.
```diff
  $map: (
    /* comment */
    key: (
-       a: 1,
-       b: 2,
-     ),
+     a: 1,
+     b: 2,
+   ),
  );
```
