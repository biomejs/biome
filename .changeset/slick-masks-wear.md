---
"@biomejs/biome": patch
---

Added new rule
[noReturnAssign](https://biomejs.dev/linter/rules/no-return-assign):

Disallows assignments inside return statements


Based on [no-return-assign](https://eslint.org/docs/latest/rules/no-return-assign)

Disallowed example:

```js
function f(a) {
    return a = 1;
}
```
