---
"@biomejs/biome": patch
---

Fixed [#11898](https://github.com/biomejs/biome/issues/11898): [`noUselessReturn`](https://biomejs.dev/linter/rules/no-useless-return/) no longer offers its safe fix for a `return` that is the unbraced body of an `if`, `else`, or labeled statement, and no longer drops the removed `return`'s leading comment.

Previously, the safe fix always deleted the `return` statement outright. For an unbraced consequent this could leave invalid syntax, and in every case it discarded any comment attached to the `return`.

```js
function f(aborted) {
    // no longer reported: removing `return` would leave `if (aborted)` without a body
    if (aborted) return;
}

function g() {
    doSomething();
    // this comment is now kept by the fix
    return;
}
```
