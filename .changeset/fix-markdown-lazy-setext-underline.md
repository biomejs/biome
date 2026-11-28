---
"@biomejs/biome": patch
---

Fixed [#13397](https://github.com/biomejs/biome/issues/13397): the Markdown formatter no longer turns a paragraph into a heading when a lazy continuation line looks like a setext heading underline. The formatter adds the block quote prefix or list indentation to such a line, so it now escapes the line's first marker.

```diff
  > foo
- bar
- ===
+ > bar
+ > \===
```
