---
"@biomejs/biome": patch
---

Fixed the HTML formatter inserting whitespace between adjacent Svelte expressions when their combined length exceeds the line width.

```diff
 <span>
-  {head.median - base.median >= 0 ? "+" : "−"}
-  {formatMs(Math.abs(head.median - base.median))}
+  {head.median - base.median >= 0 ? "+" : "−"}{formatMs(Math.abs(head.median - base.median))}
 </span>
```
