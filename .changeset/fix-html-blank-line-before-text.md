---
"@biomejs/biome": patch
---

Fixed the HTML formatter collapsing the blank line between an element and the text that follows it. A blank line before text is now kept, the way one before another element already was:

```diff
  <div>foo</div>
-
  text
```
