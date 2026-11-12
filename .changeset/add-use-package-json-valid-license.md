---
"@biomejs/biome": patch
---

Added the new nursery rule [`usePackageJsonValidLicense`](https://biomejs.dev/linter/rules/use-package-json-valid-license/), which reports a `license` field in `package.json` that isn't a valid license. The rule accepts SPDX license identifiers and expressions, `UNLICENSED`, and references to a license file such as `SEE LICENSE IN LICENSE.md`.

For example, the rule reports the following `package.json` file because `GPL3` isn't an SPDX license identifier:

```json
{
  "name": "my-package",
  "license": "GPL3"
}
```
