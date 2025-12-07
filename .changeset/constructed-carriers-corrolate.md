---
"@biomejs/biome": patch
---

Fixed [#8292](https://github.com/biomejs/biome/issues/8292): Implement tracking
of types of TypeScript constructor parameter properties.

This resolves certain false negatives in `noFloatingPromises` and other typed
rules.

#### Example

```ts
class AsyncClass {
    async returnsPromise() {
        return 'value';
    }
}

class ShouldBeReported {
    constructor(public field: AsyncClass) { }
    //          ^^^^^^^^^^^^----------------- Parameter property declaration

    async shouldBeReported() {
        // `noFloatingPromises` will now report the following usage:
        this.field.returnsPromise();
    }
}
```
