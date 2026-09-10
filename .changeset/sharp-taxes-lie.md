---
"@biomejs/biome": patch
---

Fixed [#11722](https://github.com/biomejs/biome/issues/11722): the JavaScript formatter inserts a newline before the closing angle bracket when a leading comment forces type arguments onto multiple lines.

```diff
 type Foo = Record<
   // comment
   string,
-  number>;
+  number
+>;
```
