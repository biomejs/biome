---
"@biomejs/biome": patch
---

Added new lint rule [`noImpliedEval`](https://biomejs.dev/linter/rules/no-implied-eval/) to nursery.

The rule detects implied `eval()` usage through functions like `setTimeout`, `setInterval`, and `setImmediate` when called with string arguments.

```js
// Invalid
setTimeout("alert('Hello');", 100);

// Valid
setTimeout(() => alert('Hello'), 100);
```
