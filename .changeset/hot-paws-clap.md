---
"@biomejs/biome": patch
---

Fixed [#4013](https://github.com/biomejs/biome/issues/4013), where comments in member chains caused unnecessary line breaks.

```js
// Before
aFunction
  .b()
  .c.d()

// After
aFunction.b().c.d()
```
