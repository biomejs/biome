---
"@biomejs/biome": patch
---

Fixed [#13125](https://github.com/biomejs/biome/issues/13125): [`noUselessThisAlias`](https://biomejs.dev/linter/rules/no-useless-this-alias/) no longer reports `this` aliases that are used inside class field initializers, where `this` binds to the class instance instead of the captured value. Previously, the safe fix could change the meaning of the program.

```js
const store = {
    count: 42,
    makeWidget() {
        const self = this;
        class Widget {
            // `self` can't be replaced with `this` here:
            initial = self.count;
        }
        return new Widget();
    },
};
```
