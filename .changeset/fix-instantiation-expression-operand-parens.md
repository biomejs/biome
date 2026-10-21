---
"@biomejs/biome": patch
---

Fixed the formatter removing the parentheses around binary, logical, type-cast and instantiation operands of an instantiation expression. The type arguments then applied to the last operand only, and a nested instantiation expression didn't parse.

```diff
- const x = a + b<T>;
+ const x = (a + b)<T>;
- const y = a as B<T>;
+ const y = (a as B)<T>;
- const z = f<A><B>;
+ const z = (f<A>)<B>;
```
