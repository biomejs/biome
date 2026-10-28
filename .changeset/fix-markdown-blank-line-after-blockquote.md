---
"@biomejs/biome": patch
---

Fixed [#12645](https://github.com/biomejs/biome/issues/12645): the Markdown formatter no longer removes the blank line between a blockquote and the list that follows it, matching Prettier.

```md
> A note.

- one
- two
```
