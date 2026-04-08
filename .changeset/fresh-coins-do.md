---
"@biomejs/biome": patch
---

Added the new nursery rule [`useVarsOnTop`](https://biomejs.dev/linter/rules/use-vars-on-top/), which requires `var` declarations to appear at the top of their containing scope.

For example, the following code now triggers the rule:

```js
function f() {
    doSomething();
    var value = 1;
}
```
