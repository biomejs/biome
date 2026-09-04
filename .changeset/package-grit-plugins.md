---
"@biomejs/biome": minor
---

Added support for distributing named Grit rules, rule presets, and configurations through packages that have a manifest file.

The field `plugins` is used to export named Grit plugins, and `configs` is used to export named configurations.
Package entries must select a named export: `package/rule`, `package/presets/preset`, or `package/configs/config`. Bare package entries aren't supported.
Rules consumed from another package are exposed under the consuming package's name, while package authors continue to reference them by their source package inside the manifest.
Configurations consumed from another package are also exposed under the consuming package's name.

```json5
// biome-manifest.json
{
  "$schema": "./node_modules/@biomejs/biome/manifest_schema.json",
  "version": 1,
  "plugins": {
    "rules": [
      "@org/shared-rules/noDeprecatedApi",
      "@org/strict-rules/presets/recommended",
      { "useCompanyLogger": "./rules/useCompanyLogger.grit" }
    ],
    "presets": {
      "recommended": [
        "useCompanyLogger",
        "@org/shared-rules/noDeprecatedApi"
      ]
    }
  },
  "configs": [
    "@org/shared-configs/configs/recommended",
    { "base": "./configs/base.json" }
  ]
}
```

The package must publish the manifest and every referenced file, for example, through the `files` field in `package.json`.
Plugin authors can expose their manifest through the `biome` condition with `./biome-manifest.json` as its value.

```json5
// package.json
{
  "name": "@org/biome-plugin",
  "exports": {
    "biome": "./biome-manifest.json",
    "default": "./index.js"
  },
  "files": ["biome-manifest.json", "rules", "configs"]
}
```

And then, you can load the rules in your `biome.json`:

```json
// biome.json
{
  "plugins": [
    "@org/biome-plugin/useCompanyLogger",
    "@org/biome-plugin/presets/recommended"
  ]
}
```

For more information on how to create a plugin to distribute via npm, check the relative [guide](https://biomejs.dev/guides/publish-packages/).
