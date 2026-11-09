---
"@biomejs/biome": patch
---

Added the new nursery rule [`noYamlEmptySequenceEntries`](https://biomejs.dev/linter/rules/no-yaml-empty-sequence-entries/), which reports empty items in YAML lists.

```yaml
fruits:
  - apple
  -
  - banana
```
