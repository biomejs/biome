---
"@biomejs/biome": minor
---

Biome migrate eslint outputs a better overriding behavior.

A Biome rule can have multiple ESLint equivalent rules.
For example, [useLiteralKeys](https://biomejs.dev/linter/rules/use-literal-keys/) has two ESLint equivalent rules: [dot-notation](https://eslint.org/docs/latest/rules/dot-notation) and [@typescript-eslint/dot-notation](https://typescript-eslint.io/rules/dot-notation/).

Previously, Biome wouldn't always enable a Biome rule even if one of its equivalent rules was enabled.
Now Biome uses the higher severity level of all the equivalent ESLint rules to set the severity level of the Biome rule.

The following ESLint configuration...

```json
{
  "rules": {
    "@typescript-eslint/dot-notation": "error",
    "dot-notation": "off"
  }
}
```

...is now migrated to...

```json
{
  "linter": {
    "rules": {
      "complexity": {
        "useLiteralKeys": "error"
      }
    }
  }
}
```

...because `error` is higher than `off`.
