---
"@biomejs/biome": minor
---

Added the HTML lint rule [`noStaticElementInteractions`](https://biomejs.dev/linter/rules/no-static-element-interactions/), which enforces that static, visible elements (such as `<div>`) that have click handlers use the valid role attribute.

**Invalid**:

```html
<div onclick="myFunction()"></div>
```
