---
"@biomejs/biome": minor
---

The rule [`noEmptySource`](https://biomejs.dev/linter/rules/no-empty-source/) now supports Markdown files. It reports files that contain only blank lines and HTML comments. For example, the following file is now reported:

```md
<!-- TODO -->
```
