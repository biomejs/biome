---
"@biomejs/biome": patch
---

Added nursery rule [`noUnusedExpressions`](https://biomejs.dev/linter/rules/no-unused-expressions/) to flag expressions used as a statement that is neither an assignment nor a function call.

#### Invalid examples

```js
f // intended to call `f()` instead
```

```js
function foo() {
    0 // intended to `return 0` instead
}
```

#### Valid examples

```js
f()
```

```js
function foo() {
    return 0;
}
```
