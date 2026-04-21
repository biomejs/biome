---
"@biomejs/biome": minor
---

Added `ignore` option to the [noUnusedVariables](https://biomejs.dev/linter/rules/no-unused-variables/) rule. The option allows excluding identifiers by providing a list of ignored names. It also allows excluding kinds of identifiers from this rule entirely, which may be useful when loading classes dynamically.

For example, unused classes as well as all unused variables, functions, etc. called "unused" may be ignored entirely with the following configuration:

```json
{
  "ignore": {
    "*": ["unused"],
    "class": ["*"]
  }
}
```
