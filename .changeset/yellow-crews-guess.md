---
"@biomejs/biome": patch
---

Added the nursery rule [`noContinue`](https://biomejs.dev/linter/rules/no-continue/). Disallowing the usage of the `continue` statement, structured control flow statements such as `if` should be used instead.

**Invalid:**
```js
let sum = 0,
    i;

for(i = 0; i < 10; i++) {
    if(i >= 5) {
        continue;
    }

    sum += i;
}
```

**Valid:**
```js
let sum = 0,
    i;

for(i = 0; i < 10; i++) {
    if(i < 5) {
        sum += i;
    }
}
```
