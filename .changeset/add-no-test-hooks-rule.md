---
"@biomejs/biome": patch
---

Added the nursery rule [`noTestHooks`](https://biomejs.dev/linter/rules/no-test-hooks/), which reports setup and teardown hooks in tests, such as `beforeEach` and `afterAll`.

```js
beforeEach(() => { // reported
	cart = createCart();
});
```
