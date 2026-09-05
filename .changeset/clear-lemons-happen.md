---
"@biomejs/biome": patch
---

Fixed [#11423](https://github.com/biomejs/biome/issues/11423): Multiline template interpolations now preserve the indentation of their closing brace when the source indentation is not a multiple of `tabWidth`.

```diff
 const value = `
      ${
        condition
          ? "yes"
          : "no"
-}
+     }
 `;
```
