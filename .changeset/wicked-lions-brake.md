---
"@biomejs/biome": minor
---

Added the HTML lint rule [`noNoninteractiveElementInteractions`](https://biomejs.dev/linter/rules/no-noninteractive-element-interactions/), which disallows use event handlers on non-interactive elements.

**Invalid**:

```html
<div onclick="myFunction()">button</div>
```
