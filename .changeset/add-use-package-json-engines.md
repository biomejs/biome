---
"@biomejs/biome": patch
---

Added the nursery rule [`usePackageJsonEngines`](https://biomejs.dev/linter/rules/use-package-json-engines/), which reports `package.json` files that have no `engines` field.

A `package.json` passes the rule once it declares the field:

```json
{
  "name": "my-package",
  "engines": {
    "node": ">=20"
  }
}
```
