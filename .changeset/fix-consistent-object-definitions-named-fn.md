---
"@biomejs/biome": patch
---

Fixed [#10212](https://github.com/biomejs/biome/issues/10212): [`useConsistentObjectDefinitions`](https://biomejs.dev/linter/rules/use-consistent-object-definitions/) no longer converts named function expressions to shorthand methods.

Shorthanding a named function expression renames it after the property key, which changes the function's `name`:

```js
const obj = {
  b: function c() {}, // obj.b.name === "c", but a shorthand `b() {}` would make it "b"
};
```

Anonymous function expressions are still converted, since they already take their name from the property key.