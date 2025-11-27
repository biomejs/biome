---
"@biomejs/biome": patch
---

Fixed [#7806](https://github.com/biomejs/biome/issues/7806): Prefer breaking after the assignment operator for conditional types with generic parameters to match Prettier.

```diff
-type True = unknown extends Type<
-  "many",
-  "generic",
-  "parameters",
-  "one",
-  "two",
-  "three"
->
-  ? true
-  : false;
+type True =
+  unknown extends Type<"many", "generic", "parameters", "one", "two", "three">
+    ? true
+    : false;
```
