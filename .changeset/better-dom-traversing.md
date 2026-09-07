---
"@biomejs/biome": patch
---

Added the nursery rule [`useBetterDomTraversing`](https://biomejs.dev/linter/rules/use-better-dom-traversing), which prefers `.firstChild`, `.firstElementChild`, `.closest()`, and merged `.querySelector()` calls over positional DOM traversal. Inspired by [unicorn/better-dom-traversing](https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/better-dom-traversing.md).

```js
element.childNodes[0];
element.children[0];
element.parentElement.parentElement;
element.querySelector("a").querySelector("b");
```
