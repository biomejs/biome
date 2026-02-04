---
"@biomejs/biome": minor
---

Added `ignore` option to the [`useHookAtTopLevel`](https://biomejs.dev/linter/rules/use-hook-at-top-level/) rule.

You can now specify function names that should not be treated as hooks, even if they follow the `use*` naming convention.

Example configuration:

```json
{
  "linter": {
    "rules": {
      "correctness": {
        "useHookAtTopLevel": {
          "options": {
            "ignore": ["useDebounce", "useCustomUtility"]
          }
        }
      }
    }
  }
}
```
