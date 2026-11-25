---
"@biomejs/biome": patch
---

Added the nursery rule [`useUnknownInCatchCallbackVariable`](https://biomejs.dev/linter/rules/use-unknown-in-catch-callback-variable/). It requires the error variable of a function passed to `.catch()`, or as the second argument of `.then()`, to be typed as `unknown`. The rule needs type information and belongs to the `types` domain.

```ts
Promise.reject(new Error("Request failed")).catch((error) => {
  console.log(error.message);
});
```
