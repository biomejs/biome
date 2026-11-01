---
"@biomejs/biome": patch
---

Fixed [#12375](https://github.com/biomejs/biome/issues/12375): [`noSetterReturn`](https://biomejs.dev/linter/rules/no-setter-return/) now also reports setters in property descriptors passed to `Object.defineProperty()`, `Reflect.defineProperty()`, `Object.defineProperties()` and `Object.create()`, like ESLint's `no-setter-return`.

```js
Object.defineProperty(obj, "value", {
  set: (newValue) => (value = newValue), // now reported
});
```
