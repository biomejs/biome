---
"@biomejs/biome": patch
---

Added the nursery rule [`noForIn`](https://biomejs.dev/linter/rules/no-for-in/). Disallow iterating using a for-in loop.

**Invalid:**

```js
for (const i in array) {
  console.log(array[i]);
}
```

**Valid:**

```js
for (const value of array) {
  console.log(value);
}

for (let i = 0; i < array.length; i += 1) {
  console.log(i, array[i]);
}

array.forEach((value, i) => {
  console.log(i, value);
});

for (const [i, value] of array.entries()) {
  console.log(i, value);
}
```
