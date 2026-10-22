---
"@biomejs/biome": patch
---

Fixed [#12507](https://github.com/biomejs/biome/issues/12507): the Markdown formatter no longer drops the `>` markers after an empty quote line when `proseWrap` is `"always"` or `"never"`. Previously, the text after the empty line was moved out of the blockquote.

```diff
  > a
  >
- b
+ > b
```
