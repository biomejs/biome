---
"@biomejs/biome": minor
---

Added the HTML lint rule [`noInteractiveElementToNoninteractiveRole`](https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role/), which enforces that non-interactive ARIA roles are not assigned to interactive HTML elements.

**Invalid**:

```html
<input role="img" />
```
