---
"@biomejs/biome": patch
---

Added the nursery rule [`noThisOutsideOfClass`](https://biomejs.dev/linter/rules/no-this-outside-of-class/). The rule reports `this` outside class members and TypeScript functions with an explicit `this` parameter.

```js
function Person(name) {
    this.name = name;
}
```

