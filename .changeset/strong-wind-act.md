---
"@biomejs/biome": patch
---

Added a new nursery rule, `block-scoped-var`, that enforces the use of variables within the scope they are defined.

This rule generates warnings when variables are used outside of the block in which they were defined, helping avoid difficult bugs with variable hoisting.

The following code is deemed incorrect:

```js
function doIf() {
  if (true) {
    var build = true;
  }

  console.log(build);
}
```
