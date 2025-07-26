---
"@biomejs/biome": patch
---

Adds tests for [#6994](https://github.com/biomejs/biome/issues/6994)
`useReadonlyClassProperties` more tests to cover ??=, ||=, &&=.

Example:

```typescript
export class Test {
  private field: number;

  someMethod() {
    this.field &&= 1;
  }
}
```
