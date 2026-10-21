---
"@biomejs/biome": patch
---

Fixed the formatter removing the parentheses around binary, logical and type-cast operands of an instantiation expression. Formatting `(a + b)<T>` printed `a + b<T>`, which applies the type arguments to `b` only, and `(f<A>)<B>` printed `f<A><B>`, which doesn't parse.
