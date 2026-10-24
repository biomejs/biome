---
"@biomejs/biome": patch
---

Fixed the formatter removing the parentheses around the callee of a `new` expression when it contains a dynamic `import()` call, which produced code that doesn't parse.

```diff
- new import("foo").bar();
+ new (import("foo").bar)();
```
