---
"@biomejs/biome": patch
---

Fixed the formatter removing the parentheses around a class or function expression at the start of a statement when it's followed by type arguments, which produced code that doesn't parse.

```diff
- class {}<string>;
+ (class {})<string>;
- function () {}<string>;
+ (function () {})<string>;
```
