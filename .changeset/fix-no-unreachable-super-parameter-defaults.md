---
"@biomejs/biome": patch
---

Fixed [`noUnreachableSuper`](https://biomejs.dev/linter/rules/no-unreachable-super/) ignoring the default values of constructor parameters. Using `this` in a default value is now reported, because it runs before `super()`, and calling `super()` in a default value is now recognized.

```js
class Child extends Parent {
	constructor(options = this.defaults()) { // reported
		super(options);
	}
}
```
