---
"@biomejs/biome": patch
---

Fixed the formatter removing the parentheses around an instantiation expression before `+`, `-`, `<`, `>`, `>=`, `>>` and `>>>`. TypeScript reads `make<T> + y` as `make < T > +y`, and `make<T> >> y` doesn't parse.

```diff
- x = make<T> + y;
+ x = (make<T>) + y;
- x = make<T> >> y;
+ x = (make<T>) >> y;
```
