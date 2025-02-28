---
"@biomejs/biome": major
---

Removed `--apply` and `--apply-unsafe`.

The CLI options `--apply` and `--apply-unasfe` aren't accepted anymore. Use `--write` and `--write --unafe` instead:

```diff
-biome check --apply-unsafe
+biome check --write --unsafe
```

```diff
-biome check --apply
+biome check --write
```
