---
"@biomejs/biome": patch
---

Fixed the CSS formatter to preserve comments on the correct side of selector combinators and before declaration blocks.

```diff
-.before > /* comment */ .after {}
+.before /* comment */ > .after {}
```
