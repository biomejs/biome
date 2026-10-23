---
"@biomejs/biome": patch
---

Fixed [#12542](https://github.com/biomejs/biome/issues/12542): the Markdown formatter no longer removes the space before an inline element, such as a code span, when an earlier table cell ends with the same text. This affected table cells, links, emphasis, headings and block quotes. Table columns also no longer get an extra space of padding in similar cases or when a cell ends with an HTML comment, which made the formatter change its output on every run.

```diff
- | a        | b     |
- | -------- | ----- |
- | x        | `y` z |
- | `p` z`q` | r     |
+ | a         | b     |
+ | --------- | ----- |
+ | x         | `y` z |
+ | `p` z `q` | r     |
```
