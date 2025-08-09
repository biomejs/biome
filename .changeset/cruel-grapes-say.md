---
"@biomejs/biome": patch
---

Added the rule [`useMaxParams`](https://biomejs.dev/linter/rules/use-max-params).

This rule enforces a maximum number of parameters for functions to improve code readability and maintainability. Functions with many parameters are difficult to read, understand, and maintain because they require memorizing parameter order and types.

```js
// Invalid - too many parameters (default max: 4)
function processData(name, age, email, phone, address, city, country, zipCode) {
  // ...
}

// Valid - within parameter limit
function processData(userData) {
  const { name, age, email, phone, address, city, country, zipCode } = userData;
  // ...
}

function calculateSum(a, b, c) {
  return a + b + c;
}
```
