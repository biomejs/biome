---
cli: major
---

# The rule `useImportRestrictions` has been removed and replaced with an option in `noRestrictedImports`

To avoid confusion between the two similarly named rules, `useImportRestrictions` has been removed
and replaced with the option `restrictPackagePrivate` in the `noRestrictedImports` rule:

```jsonc
// biome.json
{
  "linter": {
    "rules": {
      "nursury": {
        "noRestrictedImports": {
          "level": "error",
          "options": {
            "restrictPackagePrivate": "all"
          }
        }
      }
    }
  }
}
```
