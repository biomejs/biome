---
"@biomejs/biome": minor
---

The rule [`noEmptySource`](https://biomejs.dev/linter/rules/no-empty-source/) now supports YAML files. It reports files with no documents and each document that has no content. For example, the second document in the following code is now reported:

```yaml
name: foo
---
```
