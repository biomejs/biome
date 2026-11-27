---
"@biomejs/biome": patch
---

Fixed parsing of Markdown list items inside block quotes so a line without the quote marker starts a new block when required, as in this example where `- b` starts a list outside the quote.

```md
> - a
  - b
```
