---
"@biomejs/biome": patch
---

Fixed [#6775](https://github.com/biomejs/biome/issues/6775): `useReadonlyClassProperties` now also captures mutations inside function arguments.

Example:

```ts
class Counter {
  private counter: number
  count() {
    console.log(this.counter++)
    const counterString = `${this.counter++}`
  }
}
```
