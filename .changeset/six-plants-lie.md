---
"@biomejs/biome": minor
---

Added the `extensionMappings` option to `useImportExtensions`. This allows users to specify custom file extensions for different module types.

For example, if you want to ban all `.ts` imports in favor of `.js` imports, you can now do so with this option:

```json
{
    "options": {
        "extensionMappings": {
            "ts": "js",
        }
    }
}
```
