---
"@biomejs/biome": patch
---

Fixed the HTML formatter incorrectly applying native HTML element formatting to PascalCase component names such as `<Ul>` and `<Body>` in Vue, Svelte, and Astro files.

```diff
-<Body>
-  <div>content</div>
-</Body>
+<Body><div>content</div></Body>
```
