---
"@biomejs/biome": major
---

Downgraded some code fixes to unsafe which were previously safe.

The following rules have now a unsafe fix:

- [`noFlatMapIdentity`](https://biomejs.dev/linter/rules/no-flat-map-identity)
- [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports)

If you want to keep applying these fixes automatically, [configure the rule fix](https://next.biomejs.dev/linter/#configure-the-code-fix) as safe:

```json
{
  "linter": {
    "rules": {
      "correctness": {
        "noFlatMapIdentity": {
          "level": "error",
          "fix": "safe"
        },
        "noUnusedImports": {
          "level": "error",
          "fix": "safe"
        }
      }
    }
  }
}
```
