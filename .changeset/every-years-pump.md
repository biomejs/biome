---
"@biomejs/biome": patch
---

Fixed [#11304](https://github.com/biomejs/biome/issues/11304): Svelte attribute values now format their embedded JavaScript, including multiline event handlers.

```diff
-<button onclick={(e)=>{e.preventDefault(); submit()}}>
+<button onclick={(e) => {
+  e.preventDefault();
+  submit();
+}}>
```
