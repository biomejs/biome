---
"@biomejs/biome": patch
---

Added the nursery rule [`useCustomErrorDefinition`](https://biomejs.dev/linter/rules/use-custom-error-definition/), which enforces a consistent way of defining custom error classes. It reports error classes whose name doesn't start with an uppercase letter and end with `Error`, that don't set their `name` property to the class name, that assign `this.message` instead of passing the message to `super()`, or that don't forward an `options` parameter to a built-in error.

```js
class FooError extends Error {
    constructor(message) {
        super();
        this.message = message;
    }
}
```
