---
"@biomejs/biome": patch
---

Fixed the indentation of multiline Astro expressions, in templates and attribute values, when running `biome check --write`. Biome now formats Astro expressions with `biome format` too, and places them at the column of the surrounding markup.

```diff
 <div>
 	{items.map((item) => (
-	<span>{item}</span>
-))}
+		<span>{item}</span>
+	))}
 </div>
```
