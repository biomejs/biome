---
"@biomejs/biome": patch
---

Added the new nursery rule [`usePackageJsonCpu`](https://biomejs.dev/linter/rules/use-package-json-cpu/), which reports `package.json` files that have no `cpu` field. The `cpu` field lists the processor types that the package supports.

The following `package.json` file satisfies the rule:

```json
{
  "name": "my-package",
  "version": "1.0.0",
  "cpu": ["x64", "arm64"]
}
```
