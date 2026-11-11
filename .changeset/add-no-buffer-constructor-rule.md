---
"@biomejs/biome": patch
---

Added the nursery rule [`noBufferConstructor`](https://biomejs.dev/linter/rules/no-buffer-constructor/), which disallows the deprecated Node.js `Buffer()` and `new Buffer()` constructors. Use `Buffer.from()`, `Buffer.alloc()`, or `Buffer.allocUnsafe()` instead.

```js
new Buffer(5);
```
