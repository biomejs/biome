---
"@biomejs/biome": patch
---

Fixed [#6759](https://github.com/biomejs/biome/issues/6759), a false positive for `noFocusedTests` that was triggered by calling any function with the name `fit` on any object.

The following code will now pass the `noFocusedTests` rule:
```js
import foo from 'foo';
foo.fit();
```
