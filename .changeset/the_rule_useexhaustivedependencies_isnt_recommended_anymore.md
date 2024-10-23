---
cli: major
---

# The rule `useExhaustiveDependencies` isn't recommended anymore

The rule `useExhaustiveDependencies` is not recommended anymore. If your codebase uses `react` and relies on that rule, you have to enable it:


```jsonc
// biome.json
{
  "linter": {
    "rules": {
      "correctness": {
        "useExhaustiveDependencies": "error"
      }
    }
  }
}
```
