---
"@biomejs/biome": minor
---

Added support for loading Grit plugins from npm packages.

Plugin authors can distribute their rules by including a mandatory `biome-manifest.json` or `biome-manifest.jsonc` in their library, which will look like this:

```json
{
  "$schema": "./node_modules/@biomejs/biome/manifest_schema.json",
  "version": 1,
  "rules": ["rules/noDeprecatedApi.grit", "rules/useCompanyLogger.grit"]
}
```

The package **must** publish the manifest and the rule files, for example, with the `files` field in `package.json`:

```json
{
  "name": "@scope/biome-plugin",
  "files": ["biome-manifest.json", "rules"]
}
```

After installing the package, users can enable every rule from its manifest by adding the package name to `plugins`:

```json
{
  "plugins": ["@scope/biome-plugin"]
}
```

Package rules use the package name and Grit file name as their plugin identity. You can suppress an individual rule with package-qualified plugin suppressions:

```js
// biome-ignore lint/plugin/@scope/biome-plugin/noDeprecatedApi: uses the deprecated API for compatibility
deprecatedApi();
```

For an unscoped package named `biome-plugin`, the same rule is addressed as `lint/plugin/biome-plugin/noDeprecatedApi`.
