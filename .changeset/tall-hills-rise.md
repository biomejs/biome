---
"@biomejs/biome": patch
---

[useArrayLiterals](https://biomejs.dev/linter/rules/use-array-literals/) now reports all expressions using the `Array` constructors.

Previously, the rule reported only use of the `Array` constructor in expressions statements.

```js
// This was reported
new Array();
// This was not reported
const xs = new Array();
```

