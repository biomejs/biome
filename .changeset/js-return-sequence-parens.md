---
"@biomejs/biome": patch
---

The JavaScript formatter now wraps a sequence expression returned from a `return` or `throw` statement in parentheses, matching Prettier.

```diff
-return a, b;
+return (a, b);
```
