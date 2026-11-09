---
"@biomejs/biome": patch
---

Added the nursery rule [`noYamlEmptyMapValues`](https://biomejs.dev/linter/rules/no-yaml-empty-map-values/), which reports keys in YAML maps that have no value.

```yaml
name:
```
