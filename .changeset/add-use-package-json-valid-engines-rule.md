---
"@biomejs/biome": patch
---

Added the nursery rule [`usePackageJsonValidEngines`](https://biomejs.dev/linter/rules/use-package-json-valid-engines/), which reports an invalid `engines` field in `package.json`. The field must be an object, and each of its values must be a non-empty string.

For example, the following `engines` field is reported because the version of `node` is a number:

```json
{
    "engines": {
        "node": 24
    }
}
```
