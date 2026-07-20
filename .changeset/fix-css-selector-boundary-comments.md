---
"@biomejs/biome": patch
---

Fixed the CSS formatter to preserve comments on the correct side of selector combinators and before declaration blocks.

```diff
-.before > /* comment */ .after {}
+.before /* comment */ > .after {}
```

It now also keeps selectors with escaped newlines in attribute values inline when they fit.

```diff
-div
-  span[foo="bar\
+div span[foo="bar\
 value"] {}
```
