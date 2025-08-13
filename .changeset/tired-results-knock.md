---
"@biomejs/biome": major
---

Implemented [#7174](https://github.com/biomejs/biome/issues/7174). [`useConst`](https://biomejs.dev/linter/rules/use-const/) no longer reports variables read before to be written.

Previously, `useConst` reported uninitialised variables that were read in an inner function before being written, as shown in the following example::

```js
let v;
function f() {
    return v;
}
v = 0;
```

This can produce false positives in the case where `f` is called before `v` has be written as in the following code:

```js
let v;
function f() {
    return v;
}
console.log(f()); // print `undefined`
v = 0;
```

Although this is an expected behavior of the original implementation, we consider it problematic since the rule fix is marked as safe.
To avoid false positives like this, the rule now ignores the previous examples.
However, this has the disadvantage of resulting in false negatives, such as reporting the first code.
