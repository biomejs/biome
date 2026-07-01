---
"@biomejs/biome": patch
---

Fixed [#8816](https://github.com/biomejs/biome/issues/8816): [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) now reports a Promise that is coerced to a string through template literal interpolation or the `+` operator.

```ts
const promise = Promise.resolve("value");

// Both of these coerce the Promise to "[object Promise]".
const a = `wtf ${promise}`;
const b = "wtf " + promise;
```

Tagged templates are left untouched, since the tag receives the raw values instead of the coerced string.
