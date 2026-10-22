---
"@biomejs/biome": patch
---

Fixed [`noUselessAssignment`](https://biomejs.dev/linter/rules/no-useless-assignment/) ignoring variables that are used in the iterated value of a `for...of` or `for...in` loop, in the default values of a destructuring declaration or of parameters, or in the body of an arrow function written as a single expression. These variables are now checked too.

```js
function process(first, second) {
	let list = first; // reported: this value is never read
	list = second;
	for (const item of list) {
		handle(item);
	}
}
```
