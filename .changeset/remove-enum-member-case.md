---
"@biomejs/biome": major
---

Remove the option `enumMemberCase` from the lint rule `useNamingConvention`.

`enumMemberCase` is an option that allows to customize the enforced case for TypeScript's enum members.
The option was introduced prior to the `conventions` option that allows to do the same thing.

The following configuration...

```json
{
  "linter": {
    "rules": {
      "style": {
        "useNamingConvention": {
          "level": "on",
          "options": {
            "enumMemberCase": "PascalCase"
          }
        }
      }
    }
  }
}
```

...must be rewritten as:

```json
{
  "linter": {
    "rules": {
      "style": {
        "useNamingConvention": {
          "level": "on",
          "options": {
            "conventions": [{
                "selector": { "kind": "enumMember" },
                "formats": ["PascalCase"]
            }]
          }
        }
      }
    }
  }
}
```

Run `biome migrate --write` to turn `enumMemberCase` into `conventions` automatically.
