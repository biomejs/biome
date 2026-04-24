---
"@biomejs/biome": patch
---

Added the new nursery rule [`useDomNodeTextContent`](https://biomejs.dev/linter/rules/use-dom-node-text-content/), which prefers `textContent` over `innerText` for DOM node text access and destructuring.

For example, the following snippet triggers the rule:

```js
const foo = node.innerText;
```
