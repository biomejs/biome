---
"@biomejs/biome": patch
---

The HTML formatter now lays out the `srcset` attribute of `<img>` and `<source>` as the list of candidates it is. Runs of whitespace between candidates collapse, and once the list no longer fits on one line each candidate goes on its own line with the descriptors aligned:

```diff
- <img srcset="/visual@0.5.png  400w, /visual.png 805w, /visual@2x.png 1610w, /visual@3x.png 2415w" />
+ <img
+   srcset="
+     /visual@0.5.png  400w,
+     /visual.png      805w,
+     /visual@2x.png  1610w,
+     /visual@3x.png  2415w
+   "
+ />
```

The attribute is left untouched when it holds no candidates, when a descriptor is malformed, or when the descriptors mix widths with pixel densities.
