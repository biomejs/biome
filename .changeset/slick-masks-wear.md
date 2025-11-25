---
"@biomejs/biome": patch
---

Added new rule
[noReturnAssign](https://biomejs.dev/linter/rules/no-return-assign):

Disallows assignments inside return statements

Closes [#8199](https://github.com/biomejs/biome/issues/8199)

Based on [no-return-assign](https://eslint.org/docs/latest/rules/no-return-assign)

Disallowed example:

```js
function f(a) {
    return a = 1;
}
```
