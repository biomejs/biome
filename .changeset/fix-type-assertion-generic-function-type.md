---
"@biomejs/biome": patch
---

Fixed the formatter removing the parentheses around a generic function type in a type assertion, which produced code that doesn't parse because it starts with `<<`.

```diff
- var a = <<T>(x: T) => T>f;
+ var a = <(<T>(x: T) => T)>f;
```
