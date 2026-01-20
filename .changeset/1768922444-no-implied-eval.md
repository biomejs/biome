---
"@biomejs/biome": minor
---

Added new lint rule `noImpliedEval` to the nursery group.

The rule detects implied `eval()` usage through functions like `setTimeout`, `setInterval`, and `setImmediate` when called with string arguments.

```js
// Invalid - will be flagged
setTimeout("alert('Hello');", 100);

// Valid - use a function instead
setTimeout(() => alert('Hello'), 100);
```
