---
"@biomejs/biome": minor
---

Introduces a new option `reactCompilerEnabled` to useExhaustiveDependencies rule.
Accepts a boolean value to enable or disable the use of the React compiler for checking dependencies in hooks.
Defaults to `false`.

Example configuration:
```json
{
  "rules": {
    "react-hooks/exhaustive-deps": [
      "warn",
      {
        "reactCompilerEnabled": true
      }
    ]
  }
}
```

