---
"@biomejs/biome": patch
---

Fixed HTML formatting that inserted rendered whitespace between an element and touching text when the line wrapped.

```diff
  <div>
-   before<meter value=".5"></meter>
-   after
+   before<meter value=".5"></meter
+   >after
  </div>
```
