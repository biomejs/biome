---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessQualifier`](https://biomejs.dev/linter/rules/no-useless-qualifier/), which reports namespace and enum names that are repeated before a member inside that same namespace or enum. Biome provides a fix that removes the repeated name.

```ts
namespace Shapes {
  export type Point = { x: number; y: number };
  const origin: Shapes.Point = { x: 0, y: 0 };
}
```
