---
"@biomejs/biome": patch
---

Fixed the formatter removing the parentheses around an instantiation expression before a non-null assertion or type arguments of a class heritage, which produced code that doesn't parse.

```diff
- const a = make<T>!;
+ const a = (make<T>)!;
- class D extends B<T><T> {}
+ class D extends (B<T>)<T> {}
```
