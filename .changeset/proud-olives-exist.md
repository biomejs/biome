---
"@biomejs/biome": minor
---

Added the new rule `noBiomeFirstException`. This rule prevents the incorrect usage of patterns inside `files.includes`.

This rule catches if the first element of the array contains `!`. This mistake will cause Biome to analyze no files:

```json5
// biome.json
{
  "files": {
    "includes": ["!dist/**"] // this is an error
  }
}
```
