---
"@biomejs/biome": patch
---

Fixed `noAccumulatingSpread` not reporting calls to `Object.assign`. The following code will now be reported:

```js
let a = [{ a: 1 }, { b: 2 }];
a.reduce((acc, val) => Object.assign(acc, val), []);
```
