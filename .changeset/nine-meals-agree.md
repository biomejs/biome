---
'@biomejs/biome': patch
---

Added the new nursery rule [`useDestructuring`](https://biomejs.dev/linter/rules/use-destructuring). This rule helps to encourage destructuring from arrays and objects.

For example, the following code triggers because the variable name `x` matches the property `foo.x`, making it ideal for object destructuring syntax.

```js
var x = foo.x;
```
