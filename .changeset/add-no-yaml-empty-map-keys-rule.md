---
"@biomejs/biome": patch
---

Added the nursery rule [`noYamlEmptyMapKeys`](https://biomejs.dev/linter/rules/no-yaml-empty-map-keys/), which reports YAML map entries whose key is missing, or has only a tag or an anchor. The rule is recommended.

```yaml
: value
```
