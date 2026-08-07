---
"@biomejs/biome": patch
---

Added the nursery rule [`useSingleTopLevelHeading`](https://biomejs.dev/linter/rules/use-single-top-level-heading/), a port of markdownlint's [`MD025`](https://github.com/DavidAnson/markdownlint/blob/main/doc/md025.md), that enforces a single top-level heading per Markdown document.

For example, the following snippet triggers the rule:

```md
# Title

# Another top-level heading
```

