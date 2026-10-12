---
"@biomejs/biome": patch
---

Added the new nursery rule [`noYamlBooleanKeys`](https://biomejs.dev/linter/rules/no-yaml-boolean-keys/), which disallows boolean values as keys in YAML. For example, the following key is a boolean instead of text, so it is reported:

```yaml
true: enabled
```
