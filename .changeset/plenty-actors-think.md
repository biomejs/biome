---
"@biomejs/biome": patch
---

Fixed [#4298](https://github.com/biomejs/biome/issues/4298). Biome now correctly formats CSS declarations when it contains one single value:

```diff
.bar {
-  --123456789012345678901234567890: var(--1234567890123456789012345678901234567);
+  --123456789012345678901234567890: var(
+    --1234567890123456789012345678901234567
+  );
}
```
