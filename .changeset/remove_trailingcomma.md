---
"@biomejs/biome": major
---

Removed the option `trailingComma` from the configuration and the CLI. Use the option `trailingCommas` instead:

```diff
{
  "javascript": {
    "formatter": {
-      "trailingComma": "es5"
+      "trailingCommas": "es5"
    }
  }
}
```

```diff
-biome format --trailing-comma=es5
+biome format --trailing-commas=es5
```
