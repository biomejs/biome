---
"@biomejs/biome": patch
---

Added the nursery rule [`usePromiseRejectErrors`](https://biomejs.dev/linter/rules/use-promise-reject-errors/), which requires Error objects as Promise rejection reasons.

```js
Promise.reject("Request failed");
new Promise((resolve, reject) => reject(42));
```
