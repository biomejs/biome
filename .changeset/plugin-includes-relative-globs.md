---
"@biomejs/biome": minor
---

Fixed [#11082](https://github.com/biomejs/biome/issues/11082): glob patterns in the `includes` option of a plugin are now matched against file paths relative to the directory of the configuration file, the same as `files.includes` and `overrides[].includes`.

Previously, the patterns were matched against the absolute file path, so a pattern such as `packages/**/src/**` matched no files and the plugin never ran.

```json
{
  "plugins": [
    { "path": "./my-plugin.grit", "includes": ["packages/**/src/**"] }
  ]
}
```

Patterns that start with `**/` keep working. Patterns that contain the absolute path of the project no longer match, and must be rewritten as relative patterns.
