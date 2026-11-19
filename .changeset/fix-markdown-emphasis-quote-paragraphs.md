---
"@biomejs/biome": patch
---

Fixed [#13268](https://github.com/biomejs/biome/issues/13268): the Markdown formatter no longer pairs emphasis markers across paragraphs of a blockquote.

```diff
-> a _**, b
+> a ___, b
 >
-> c 20**.
+> c 20__.
```
