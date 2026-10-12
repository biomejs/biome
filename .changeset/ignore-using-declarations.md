---
"@biomejs/biome": minor
---

Added the `ignoreUsingDeclarations` option to [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/). When enabled, the rule no longer reports unused variables declared with `using` or `await using`, because these declarations clean up their value automatically even when the code never reads it.

```json
{
  "linter": {
    "rules": {
      "correctness": {
        "noUnusedVariables": {
          "options": { "ignoreUsingDeclarations": true }
        }
      }
    }
  }
}
```
