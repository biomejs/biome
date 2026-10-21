---
"@biomejs/biome": patch
---

Fixed [#12524](https://github.com/biomejs/biome/issues/12524): the Markdown formatter no longer removes the blank line between a paragraph and the list that follows it. Removing it could change the document, because an ordered list that doesn't start at `1` can't interrupt a paragraph and became part of the paragraph text.

```md
Steps:

3. three
4. four
```
