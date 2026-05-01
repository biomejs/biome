---
"@biomejs/biome": minor
---

Added the `allowConditional` option to [`noSkippedTests`](https://biomejs.dev/linter/rules/no-skipped-tests/). When enabled, conditional skip patterns are allowed:

- `test.skip()` / `test.fixme()` calls inside an `if` statement.
- `test.skip(condition)` / `test.skip(condition, "reason")` with a non-string first argument.

```json
{
    "linter": {
        "rules": {
            "suspicious": {
                "noSkippedTests": {
                    "level": "error",
                    "options": {
                        "allowConditional": true
                    }
                }
            }
        }
    }
}
```
