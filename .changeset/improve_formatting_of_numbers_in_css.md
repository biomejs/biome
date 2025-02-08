---
"@biomejs/biome": patch
---

Fix [#5031](https://github.com/biomejs/biome/issues/5031). Improve CSS formatting for numbers:

```diff
.class {
-	padding: .5em;
-	marding: 1.0;
+	padding: 0.5em;	
+	marding: 1;
}
```
