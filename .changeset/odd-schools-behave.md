---
"@biomejs/biome": patch
---

Fixed [#4927](https://github.com/biomejs/biome/issues/4927), [#6407](https://github.com/biomejs/biome/issues/6407): The HTML formatter will now correctly break a block-like element if it has more than 2 children, and at least one of them is another block-like element.

```diff
-<div>a<div>b</div> c</div>
+<div>
+  a
+  <div>b</div>
+  c
+</div>
```
