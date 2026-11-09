---
"@biomejs/biome": patch
---

Fixed [#12994](https://github.com/biomejs/biome/issues/12994): the HTML formatter no longer moves a word onto its own line when it follows an inline element or `{expression}` that starts a line. Formatting such text is now stable after one pass.

```diff
 <span>
 	Showing the first {shown} of
-	{total}
-	references
+	{total} references
 </span>
```
