---
"@biomejs/biome": minor
---

Added the HTML lint rule [`noNoninteractiveElementToInteractiveRole`](https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role/), which enforces that interactive ARIA roles are not assigned to non-interactive HTML elements.

**Invalid**:

```html
<h1 role="checkbox"></h1>
```
