---
"@biomejs/biome": patch
---

Fixed [#11541](https://github.com/biomejs/biome/issues/11541): formatting a Svelte render tag followed by an HTML comment no longer duplicates the comment.

```diff
 <div>
   {@render children?.()}
   <!-- comment -->
-  <!-- comment -->
 </div>
```
