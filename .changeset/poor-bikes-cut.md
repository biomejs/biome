---
"@biomejs/biome": patch
---

Fixed Vue and Svelte formatting for standalone interpolations in inline elements. Biome now preserves existing newlines in cases like:

```diff
- <span> {{ value }} </span>
+ <span>
+   {{ value }}
+ </span>
```
