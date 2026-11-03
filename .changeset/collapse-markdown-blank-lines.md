---
"@biomejs/biome": patch
---

Fixed [#12738](https://github.com/biomejs/biome/issues/12738): the Markdown formatter now collapses consecutive blank lines between blocks into a single blank line.

```diff
 Some paragraph.

-
-
 ## Next section
```
