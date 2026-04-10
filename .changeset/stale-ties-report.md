---
"@biomejs/biome": patch
---

Added the nursery rule [`useReactAsyncServerFunction`](https://biomejs.dev/linter/rules/use-react-async-server-function), which requires React server actions to be async.

**Invalid:**

```js
function serverFunction() {
  'use server';
  // ...
}
```
