---
"@biomejs/biome": minor
---

Added the rule [`useAnchorContent`](https://biomejs.dev/linter/rules/use-anchor-content/) for HTML to enforce that anchor elements have accessible content for screen readers. The rule flags empty anchors, anchors with only whitespace, and anchors where all content is hidden with `aria-hidden`. Anchors with `aria-label` or `title` attributes providing a non-empty accessible name are considered valid.
