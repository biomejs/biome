---
"@biomejs/biome": minor
---

Added the new JavaScript rule [`useConsistentObjectDefinition`](https://biomejs.dev/linter/rules/use-consistent-object-definition/) rule. The rule enforces a consistent style for the definition of objects:

By default, the rule enforces a shorthand style:

```js
const validShorthand = {
  // Property shorthand
  foo,

  // Method shorthand
  method() {
    return "method";
  },
}
```

Alternatively, the rule can be configured to enforce an explicit style:

```js
const invalidExplicit = {
  // Basic property shorthand violations
  foo: foo,

  // Method shorthand violations
  method: function () {
    return "method";
  },
}
```
