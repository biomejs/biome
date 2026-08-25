---
"@biomejs/biome": minor
---

Adds a new `resolutionKind` option for object-syntax plugin entries.

This allows shared Biome configs in monorepos to load local Grit plugins from the package that declares them instead of resolving plugin paths from the consuming project.

```json
{
  "plugins": [
    { "path": "./grit/no-debugger.grit", "resolutionKind": "config" },
    { "path": "local-plugin.grit", "resolutionKind": "project" },
    { "path": "local-plugin.grit" }
  ]
}
```

Use `resolutionKind: "config"` to resolve the plugin from the configuration file that declares it. Use `resolutionKind: "project"` to resolve the plugin from the consuming project. When omitted, `resolutionKind` defaults to `project`. This only affects plugin resolution; `includes` still apply to end-user project files as usual.
