---
"@biomejs/biome": patch
---

Fixed the GritQL formatter treating a single line break between list items as a blank line. Formatting a broken list no longer adds blank lines between its items, and blank lines from the source are still kept.

```diff
  $x <: [
-   `1`,
-
-   `2`,
+   `1`, `2`,

    `3`
  ]
```
