---
"@biomejs/biome": minor
---

Added the lint rule [`useFocusableInteractive`](https://biomejs.dev/linter/rules/use-focusable-interactive/) to HTML, which enforces elements with an interactive role and interaction handler to be focusable.

**Invalid**:

```html
<div role="button"></div>
```
