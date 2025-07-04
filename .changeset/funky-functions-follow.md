---
"@biomejs/biome": patch
---

Type inference can now infer the return types of functions and methods without
annotations.

## Example

```ts
const sneakyObject = {
	doSomething() {
		return Promise.resolve("This is a floating promise!");
	},
};

// We can now detect that `doSomething()` returns a `Promise`.
sneakyObject.doSomething();
```
