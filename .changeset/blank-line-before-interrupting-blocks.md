---
"@biomejs/biome": patch
---

Fixed [#12739](https://github.com/biomejs/biome/issues/12739), [#12741](https://github.com/biomejs/biome/issues/12741), and [#12742](https://github.com/biomejs/biome/issues/12742): the Markdown formatter now adds a blank line between a top-level paragraph and a header, fenced code block, list, or blockquote that directly follows it, matching Prettier.

```diff
 Some text:
+
 - bullet 1
 - bullet 2
```
