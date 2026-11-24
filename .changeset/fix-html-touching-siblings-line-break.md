---
"@biomejs/biome": patch
---

Fixed [#13299](https://github.com/biomejs/biome/issues/13299): the HTML formatter no longer adds a line break between whitespace-sensitive siblings that touch in the source, such as an element next to a self-closing element, a Svelte `{@render}` tag, or a text expression. The line break rendered as a space that wasn't in the source.

```diff
 <p>
-	<span class="label">label</span><img src="image.png">
-	<span class="hint">hint</span>
+	<span class="label">label</span><img src="image.png"><span class="hint"
+		>hint</span
+	>
 </p>
```
