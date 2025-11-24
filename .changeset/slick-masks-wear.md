---
"@biomejs/backend-jsonrpc": major
"@biomejs/biome": major
---

Ported
[no-return-assign](https://eslint.org/docs/latest/rules/no-return-assign):
Disallow assignments inside return statements

Return statements are often considered side-effect free.
Assignments inside return statements are often intended to be comparison operators (such as `==`).
Moreover, the use of assignments in a return statement is confusing.

Disallowed example:

```js
function f(a) {
    return a = 1;
}
```
