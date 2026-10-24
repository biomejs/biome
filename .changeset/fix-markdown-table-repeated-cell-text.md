---
"@biomejs/biome": patch
---

Fixed [#12542](https://github.com/biomejs/biome/issues/12542): the Markdown formatter no longer removes a space from a table cell, or adds a space of padding that changed on every run, when the same text appears in the middle of one cell and at the end of another. A cell that ends with an HTML comment also no longer widens its column on every run.

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
