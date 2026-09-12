---
"@biomejs/biome": patch
---

Fixed [#11304](https://github.com/biomejs/biome/issues/11304): Svelte attribute values such as `onclick={...}` are now formatted as JavaScript instead of keeping their original layout.

```diff
-<form onsubmit={(event)=>{event.preventDefault();submitForm();  trackSubmission();}}></form>
+<form
+	onsubmit={(event) => {
+		event.preventDefault();
+		submitForm();
+		trackSubmission();
+	}}
+></form>
```
