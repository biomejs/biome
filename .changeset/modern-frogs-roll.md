---
"@biomejs/biome": minor
---

Rule's `options` is now optional in the Biome configuration files for rules with a `fix` kind.

Previously, configuring a rule's `fix` required `options` to be set.
Now, `options` is optional.
The following configuration is now valid:

```json
{
  "linter": {
    "rules": {
      "correctness": {
        "noUnusedImports": {
          "level": "on",
          "fix": "safe"
        }
      }
    }
  }
}
```
