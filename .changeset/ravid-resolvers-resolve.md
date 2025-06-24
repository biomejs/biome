---
"@biomejs/biome": patch
---

Fixed [#6515](https://github.com/biomejs/biome/issues/6515). When using the
`extends` field to extend a configuration from an NPM package, we now accept the
_condition names_ `"biome"` and `"default"` for exporting the configuration in
the `package.json`.

This means that where previously your `package.json` had to contain an export
declaration similar to this:

```json
{
    "exports": {
        ".": "./biome.json"
    }
}
```

You may now use one of these as well:

```json
{
    "exports": {
        ".": {
            "biome": "./biome.json"
        }
    }
}
```

Or:

```json
{
    "exports": {
        ".": {
            "default": "./biome.json"
        }
    }
}
```
