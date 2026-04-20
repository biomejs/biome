---
"@biomejs/biome": minor
---

Added the HTML lint rule [`useFocusableInteractive`](https://biomejs.dev/linter/rules/use-focusable-interactive/), which enforces elements with an interactive role and interaction handler to be focusable.

**Invalid**:

```html
<div role="button"></div>
```
