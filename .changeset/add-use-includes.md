---
"@biomejs/biome": patch
---

Added a new nursery rule [`useIncludes`](https://biomejs.dev/linter/rules/use-includes/) that enforces the use of `includes()` over `indexOf()` comparisons when checking for the presence of a value in a string or array. Ported from the typescript-eslint [`prefer-includes`](https://typescript-eslint.io/rules/prefer-includes/) rule.

Invalid:

```js
str.indexOf("foo") !== -1;
arr.indexOf(item) === -1;
str.indexOf("foo") >= 0;
```
