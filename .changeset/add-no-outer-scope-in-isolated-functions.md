---
"@biomejs/biome": patch
---

Added the nursery rule [`noOuterScopeInIsolatedFunctions`](https://biomejs.dev/linter/rules/no-outer-scope-in-isolated-functions/), which reports uses of variables, `this`, and `super` from outside a function that runs apart from the code around it, such as a function passed to `makeSynchronous()` or marked with an `@isolated` comment.

```js
const url = "https://example.com";

makeSynchronous(async () => {
	const response = await fetch(url); // `url` doesn't exist where the function runs
	return response.text();
});
```
