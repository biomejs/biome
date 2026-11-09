---
"@biomejs/biome": patch
---

Fixed [#13027](https://github.com/biomejs/biome/issues/13027): the Markdown formatter now preserves spaces around text after a code span that crosses block quote lines.

```diff
 > `a
-> b`c`d`
+> b` c `d`
```
