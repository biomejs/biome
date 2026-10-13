---
"@biomejs/biome": patch
---

Added the new nursery rule [`usePackageJsonValidCpu`](https://biomejs.dev/linter/rules/use-package-json-valid-cpu/), which reports a `cpu` field in `package.json` that isn't an array of known processor types. The `cpu` field lists the processor types that the package supports.

For example, the rule reports the following `package.json` file because `x86` isn't a processor type that package managers recognize:

```json
{
  "name": "my-package",
  "cpu": ["x86"]
}
```
