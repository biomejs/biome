---
"@biomejs/biome": patch
---

Fixed [#8774](https://github.com/biomejs/biome/issues/8774): Type aliases with generic parameters that have `extends` constraints now properly indent comments after the equals sign.

Previously, comments after the `=` in type aliases with `extends` constraints were not indented:

```diff
-type A<B, C extends D> = // Some comment
-undefined;
+type A<B, C extends D> =
+    // Some comment
+    undefined;
```
