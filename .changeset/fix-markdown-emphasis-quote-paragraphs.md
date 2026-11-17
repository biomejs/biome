---
"@biomejs/biome": patch
---

Fixed [#13268](https://github.com/biomejs/biome/issues/13268): the Markdown formatter no longer pairs emphasis markers across two paragraphs of a blockquote. Previously, `___` in one paragraph and `__` in the next were rewritten as strong emphasis, which changed the rendered text.

```diff
-> a _**, b
+> a ___, b
 >
-> c 20**.
+> c 20__.
```
