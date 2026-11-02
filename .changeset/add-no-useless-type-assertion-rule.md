---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUselessTypeAssertion`](https://biomejs.dev/linter/rules/no-useless-type-assertion/), which reports type assertions that don't change the type of an expression, such as a non-null assertion on a value that can't be `null` or `undefined`.

```ts
function double(value: number): number {
  return value! * 2;
}
```
