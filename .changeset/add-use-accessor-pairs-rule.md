---
"@biomejs/biome": patch
---

Added the nursery rule [`useAccessorPairs`](https://biomejs.dev/linter/rules/use-accessor-pairs/). The rule reports setters that have no matching getter in object literals, classes, and property descriptors.

```js
const user = {
    set name(value) {
        this._name = value;
    },
};
```
