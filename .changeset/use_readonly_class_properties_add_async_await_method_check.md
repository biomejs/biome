---
"@biomejs/biome": patch
---

Fixed [#6919](https://github.com/biomejs/biome/issues/6919) and [#6920](https://github.com/biomejs/biome/issues/6920):
`useReadonlyClassProperties` now does checks for mutations in async class methods.

Example:

```typescript
class Counter3 {
  private counter: number;
  async count() {
    this.counter = 1;
    const counterString = `${this.counter++}`;
  }
}
```
