---
"@biomejs/biome": patch
---

Added the [`noProto`](https://biomejs.dev/linter/rules/no-proto/) rule, which disallows the use of the `__proto__` property for getting or setting the prototype of an object.

**Invalid**:

```js
obj.__proto__ = a;
const b = obj.__proto__;
```

**Valid**:

```js
const a = Object.getPrototypeOf(obj);
Object.setPrototypeOf(obj, b);
```
