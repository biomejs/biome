---
"@biomejs/biome": minor
---

Adds a new `resolvePath` option for object-syntax plugin entries.

This allows shared Biome configs in monorepos to load local Grit plugins from the package that declares them instead of resolving plugin paths from the consuming project.
