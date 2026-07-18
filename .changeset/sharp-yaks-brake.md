---
"@biomejs/biome": patch
---

Added the rule [`noNegationInEqualityCheck`](https://biomejs.dev/linter/rules/no-negation-in-equality-check/). The rule flags negated expressions on the left side of strict equality checks like `!foo === bar` — due to operator precedence this evaluates as `(!foo) === bar` which is almost always a mistake for `foo !== bar`.

The rule provides an unsafe fix that flips the operator.

```js
// Invalid
!foo === bar;
!foo !== bar;

// Valid
foo !== bar;
foo === bar;
```
