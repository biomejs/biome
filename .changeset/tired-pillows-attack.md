---
"@biomejs/biome": minor
---

Added `expand: "whenMultiple"` to format objects and arrays with more than one member on multiple lines, placing each member on its own line. Single-member collections stay on one line when they fit.

```diff
 const single = { enabled: true };
-const multiple = { enabled: true, retries: 3 };
+const multiple = {
+  enabled: true,
+  retries: 3,
+};
```
