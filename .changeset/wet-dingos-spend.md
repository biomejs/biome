---
"@biomejs/biome": patch
---

Fixed [#9300](https://github.com/biomejs/biome/issues/9300): Lowercase component member expressions like `<form.Field>` in Svelte and Astro files are now correctly formatted.

```diff
-<form .Field></form.Field>
+<form.Field></form.Field>
```
