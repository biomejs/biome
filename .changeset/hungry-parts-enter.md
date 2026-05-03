---
"@biomejs/biome": patch
---

Fixed [#10185](https://github.com/biomejs/biome/issues/10185). [`organizeImports](https://biomejs.dev/assist/actions/organize-imports/) now errors when it encounters an unknown predefined group.

The following configuration is now reported as invalid because `:INEXISTENT:` is an unknown predefined group.

```json
{
  "assist": {
    "actions": {
      "source": {
        "organizeImports": { "options": { "groups": [":INEXISTENT:"] } }
      }
    }
  }
}
```
