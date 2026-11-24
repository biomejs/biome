---
"@biomejs/biome": patch
---

Fixed [#10547](https://github.com/biomejs/biome/issues/10547): Astro files can now place `<!doctype html>` after top-level elements such as inline scripts without parse errors, and formatting preserves the source order.

```diff
-<script is:inline>console.log("ready");</script>
+<script is:inline>
+	console.log("ready");
+</script>
 <!doctype html>
```

