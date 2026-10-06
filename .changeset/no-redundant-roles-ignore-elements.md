---
"@biomejs/biome": minor
---

Added the `ignoreElements` option to [`noRedundantRoles`](https://biomejs.dev/linter/rules/no-redundant-roles/). Elements listed in this option are not checked by the rule.

```json
{
  "linter": {
    "rules": {
      "a11y": {
        "noRedundantRoles": {
          "options": {
            "ignoreElements": ["button"]
          }
        }
      }
    }
  }
}
```
