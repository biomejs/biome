---
"@biomejs/biome": patch
---

Fixed [#8574](https://github.com/biomejs/biome/issues/8574): the JavaScript formatter sometimes added extra parentheses and moved comments when formatting multiline expressions after operators such as `!`. Comments now stay beside the values they describe, without an extra pair of parentheses.

```diff
 !(
-  (
-    cond1 || // force this to be multi line
-    cond3
-  ) // comment
+  cond1 || // force this to be multi line
+  cond3 // comment
 );
```
