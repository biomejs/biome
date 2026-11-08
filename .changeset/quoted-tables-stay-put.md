---
"@biomejs/biome": patch
---

Fixed [#13026](https://github.com/biomejs/biome/issues/13026): the Markdown formatter no longer indents the rows of a table inside a blockquote when the table follows a paragraph of two or more lines. Previously, every run of the formatter indented the rows further until they were joined onto the header line.

```diff
  > First line of a paragraph,
  > second line of it.
  >
  > | A   | B   |
-   > | --- | --- |
-   > | x   | y   |
+ > | --- | --- |
+ > | x   | y   |
```
